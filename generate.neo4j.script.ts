#!/usr/bin/env bun
/**
 * 🛂 Writes `.repo/🛂/<technology>.cypher` **exclusively** from the live Neo4j database via APOC `apoc.export.cypher.all` (no YAML, GraphQL, or hand-appended Cypher).
 *
 * Usage:
 *   `bun ./generate.neo4j.script.ts [technology]`
 *   `technology` defaults to `NEO4J_DATABASE` or `semio`. Allowed: `semio` | `elements` | `coda` | `reuse`.
 *
 * Multi-database: run `bun run generate` to refresh all four files, or e.g. `NEO4J_DATABASE=semio bun ./generate.neo4j.script.ts semio`.
 *
 * Requires: APOC `apoc.export.cypher.all`, `apoc.export.file.enabled=true`, and import path writable (see setup scripts / `apoc.import.file.use_neo4j_config=false`).
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const NEO4J_VERSION = "5.26.26";
const ALLOWED = new Set(["semio", "elements", "coda", "reuse"]);
//#endregion 🧭Constants

//#region 📝CypherShell
function resolveCypherShell(): string | null {
  const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
  const cachedShell = join(REPO_ROOT, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
  const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));

  for (const candidate of candidates) {
    if (candidate.includes("/") || candidate.includes("\\")) {
      if (existsSync(candidate)) {
        return candidate;
      }
      continue;
    }

    const probe = spawnSync(candidate, ["--version"], { stdio: "ignore" });
    if (probe.status === 0) {
      return candidate;
    }
  }

  return null;
}

function buildCypherEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  if (process.platform === "win32") {
    const javaHome = "C:\\Program Files\\Microsoft\\jdk-21.0.11.10-hotspot";
    const javaExecutable = join(javaHome, "bin", "java.exe");
    if (existsSync(javaExecutable)) {
      env.JAVA_HOME = javaHome;
      env.Path = `${join(javaHome, "bin")};${env.Path || ""}`;
    }
  }
  return env;
}

function runCypher(database: string, cypher: string): { ok: boolean; stdout: string; stderr: string } {
  const shell = resolveCypherShell();
  if (!shell) {
    return { ok: false, stdout: "", stderr: "cypher-shell not found (install Neo4j tools or set NEO4J_CYPHER_SHELL)." };
  }

  const queryDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(queryDir, { recursive: true });
  const queryPath = join(queryDir, `neo4j-generate-query-${process.pid}-${Date.now()}.cypher`);
  writeFileSync(queryPath, `${cypher.trim()}\n`, "utf8");

  try {
    const result = spawnSync(
      shell,
      [
        "-a",
        process.env.NEO4J_URI || "bolt://localhost:7687",
        "-u",
        process.env.NEO4J_USERNAME || "neo4j",
        "-p",
        process.env.NEO4J_PASSWORD || "password",
        "-d",
        database,
        "--format",
        "plain",
        "-f",
        queryPath,
      ],
      {
        cwd: REPO_ROOT,
        encoding: "utf8",
        env: buildCypherEnv(),
      },
    );

    return {
      ok: result.status === 0,
      stdout: typeof result.stdout === "string" ? result.stdout : result.stdout?.toString() ?? "",
      stderr: typeof result.stderr === "string" ? result.stderr : result.stderr?.toString() ?? "",
    };
  } finally {
    try {
      unlinkSync(queryPath);
    } catch {
      /* temp query cleanup best-effort */
    }
  }
}
//#endregion 📝CypherShell

//#region 🛂ApocExport
function apocExportCypherAllToAbsoluteFile(database: string, absoluteFile: string): { ok: boolean; message: string } {
  const neoPath = absoluteFile.replace(/\\/g, "/");
  const apocTarget = /^[A-Za-z]:\//.test(neoPath) ? `file:${neoPath}` : neoPath.startsWith("/") ? `file://${neoPath}` : neoPath;
  const pathLiteral = JSON.stringify(apocTarget);
  const cypher = [
    `CALL apoc.export.cypher.all(${pathLiteral}, {`,
    `  format: "cypher-shell",`,
    `  writeNodeProperties: true,`,
    `  ifNotExists: true,`,
    `  useOptimizations: { type: "UNWIND_BATCH", unwindBatchSize: 100 }`,
    `})`,
    `YIELD file, batches, source, format, nodes, relationships, properties, time, rows, batchSize`,
    `RETURN file, batches, source, format, nodes, relationships, properties, time, rows, batchSize;`,
  ].join("\n");

  const { ok, stdout, stderr } = runCypher(database, cypher);
  if (!ok) {
    return {
      ok: false,
      message: `${stderr || stdout || "unknown error"}\n` +
        "Ensure APOC is installed, apoc.export.file.enabled=true, and Neo4j may write this absolute path (set apoc.import.file.use_neo4j_config=false on Desktop — see setup scripts).",
    };
  }
  return { ok: true, message: stdout.trim() };
}

function writeGeneratedCypherBundle(technology: string, database: string, body: string, finalPath: string): void {
  const stamp = new Date().toISOString();
  const header = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Generated exclusively from the live Neo4j database — do not edit this file by hand.",
    "// Refresh: `bun run generate` (see generate.neo4j.script.ts).",
    `// technology: ${technology} | database: ${database} | generated: ${stamp}`,
    "//",
    "",
  ].join("\n");

  writeFileSync(finalPath, `${header}${body.trim()}\n`, "utf8");
}
//#endregion 🛂ApocExport

//#region 🚀Entry
/**
 * 🗄️ Dumps one allowed technology graph from the live DBMS to `.repo/🛂/<technology>.cypher`.
 */
export function runGenerateNeo4jFromLiveDatabase(argv: string[] = process.argv.slice(2)): void {
  const arg = argv.find((a) => ALLOWED.has(a));
  const technology = arg ?? process.env.NEO4J_DATABASE ?? "semio";
  if (!ALLOWED.has(technology)) {
    console.error(`[generate:neo4j] technology must be one of: ${[...ALLOWED].join(", ")} (got ${JSON.stringify(technology)})`);
    process.exit(1);
  }

  const database = process.env.NEO4J_DATABASE ?? technology;
  const outDir = join(REPO_ROOT, ".repo", "🛂");
  mkdirSync(outDir, { recursive: true });

  const finalName = `${technology}.cypher`;
  const finalAbs = join(outDir, finalName);
  const cacheDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(cacheDir, { recursive: true });
  const tmpFileName = `.generate-${technology}-${process.pid}.tmp.cypher`;
  const tmpAbs = join(cacheDir, tmpFileName);

  const probe = runCypher(database, "RETURN 1 AS ok;");
  if (!probe.ok) {
    console.error(`[generate:neo4j] cannot reach database ${JSON.stringify(database)}:\n${probe.stderr || probe.stdout}`);
    process.exit(1);
  }

  if (existsSync(tmpAbs)) {
    unlinkSync(tmpAbs);
  }

  const result = apocExportCypherAllToAbsoluteFile(database, tmpAbs);
  if (!result.ok) {
    console.error(`[generate:neo4j] apoc.export.cypher.all failed:\n${result.message}`);
    process.exit(1);
  }

  if (!existsSync(tmpAbs)) {
    console.error(`[generate:neo4j] expected export file missing at ${tmpAbs} after APOC call.`);
    process.exit(1);
  }

  const body = readFileSync(tmpAbs, "utf8");
  unlinkSync(tmpAbs);
  writeGeneratedCypherBundle(technology, database, body, finalAbs);

  console.log(`[generate:neo4j] wrote ${finalAbs} (database ${database}).`);
  if (result.message) {
    console.log(result.message);
  }
}

if (import.meta.main) {
  runGenerateNeo4jFromLiveDatabase();
}
//#endregion 🚀Entry
