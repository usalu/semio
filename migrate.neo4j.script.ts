#!/usr/bin/env bun
/**
 * 🧭 Copies the live graph between **named** graph databases on the same DBMS (e.g. `elements` → `semio`).
 * Uses APOC `apoc.export.cypher.all` + cypher-shell replay — no YAML or GraphQL.
 *
 * Usage (both env vars required — no default to the removed stock `neo4j` database):
 *   `NEO4J_MIGRATE_FROM=elements NEO4J_MIGRATE_TO=semio bun ./migrate.neo4j.script.ts`
 *
 * Optional: `--drop-legacy` drops `NEO4J_MIGRATE_FROM` after a successful copy (set default to `NEO4J_MIGRATE_TO` first).
 *
 * Env: `NEO4J_URI`, `NEO4J_USERNAME`, `NEO4J_PASSWORD` (same as MCP).
 *
 * Neo4j Community (single user database): this script cannot create a second database — set `initial.dbms.default_database=semio`
 * and reset the store once (see `.devcontainer/post-start.sh` migration) instead.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const NEO4J_VERSION = "5.26.26";
const ALLOWED_DB = new Set(["semio", "elements", "coda", "reuse"]);
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
  const queryPath = join(queryDir, `neo4j-migrate-query-${process.pid}-${Date.now()}.cypher`);
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
      /* best-effort */
    }
  }
}

function runCypherFile(database: string, cypherFilePath: string): { ok: boolean; stdout: string; stderr: string } {
  const shell = resolveCypherShell();
  if (!shell) {
    return { ok: false, stdout: "", stderr: "cypher-shell not found." };
  }

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
      cypherFilePath,
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
    return { ok: false, message: stderr || stdout || "unknown error" };
  }
  return { ok: true, message: stdout.trim() };
}
//#endregion 🛂ApocExport

function assertDbName(name: string, label: string): void {
  if (!ALLOWED_DB.has(name)) {
    console.error(`[migrate:neo4j] invalid ${label} ${JSON.stringify(name)}`);
    process.exit(1);
  }
}

function main(): void {
  const dropLegacy = process.argv.includes("--drop-legacy");
  const fromDb = process.env.NEO4J_MIGRATE_FROM?.trim();
  const toDb = (process.env.NEO4J_MIGRATE_TO || "semio").trim();
  if (!fromDb) {
    console.error("[migrate:neo4j] set NEO4J_MIGRATE_FROM (and usually NEO4J_MIGRATE_TO=semio). Example: NEO4J_MIGRATE_FROM=elements NEO4J_MIGRATE_TO=semio");
    process.exit(1);
  }
  assertDbName(fromDb, "NEO4J_MIGRATE_FROM");
  assertDbName(toDb, "NEO4J_MIGRATE_TO");

  if (fromDb === toDb) {
    console.error("[migrate:neo4j] NEO4J_MIGRATE_FROM and NEO4J_MIGRATE_TO must differ.");
    process.exit(1);
  }

  const cacheDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(cacheDir, { recursive: true });
  const dumpAbs = join(cacheDir, `.migrate-${fromDb}-to-${toDb}-${process.pid}.cypher`);

  const probeFrom = runCypher(fromDb, "RETURN 1 AS ok;");
  if (!probeFrom.ok) {
    console.error(
      `[migrate:neo4j] cannot open source database ${JSON.stringify(fromDb)}:\n${probeFrom.stderr || probeFrom.stdout}\n` +
        "Neo4j Community has only one user graph — use `initial.dbms.default_database=semio` and a one-time data reset instead of this migrator.",
    );
    process.exit(1);
  }

  console.log(`[migrate:neo4j] ensuring target database ${JSON.stringify(toDb)} exists…`);
  const create = runCypher("system", `CREATE DATABASE \`${toDb}\` IF NOT EXISTS WAIT;`);
  if (!create.ok) {
    console.error(
      `[migrate:neo4j] CREATE DATABASE failed (Enterprise feature?):\n${create.stderr || create.stdout}\n` +
        "On Neo4j Community you cannot add a second database; configure the sole database name as `semio` and replay `.repo/🛂/*.cypher` after reset.",
    );
    process.exit(1);
  }

  const start = runCypher("system", `START DATABASE \`${toDb}\` WAIT;`);
  if (!start.ok) {
    console.log(`[migrate:neo4j] START DATABASE note: ${(start.stderr || start.stdout).trim()}`);
  }

  if (existsSync(dumpAbs)) {
    unlinkSync(dumpAbs);
  }

  console.log(`[migrate:neo4j] exporting from ${fromDb} → ${dumpAbs}…`);
  const ex = apocExportCypherAllToAbsoluteFile(fromDb, dumpAbs);
  if (!ex.ok) {
    console.error(`[migrate:neo4j] export failed:\n${ex.message}`);
    process.exit(1);
  }
  if (!existsSync(dumpAbs)) {
    console.error(`[migrate:neo4j] dump file missing at ${dumpAbs}`);
    process.exit(1);
  }

  console.log(`[migrate:neo4j] wiping ${toDb}, then importing dump…`);
  const wipe = runCypher(toDb, "MATCH (n) DETACH DELETE n;");
  if (!wipe.ok) {
    console.error(`[migrate:neo4j] wipe ${toDb} failed:\n${wipe.stderr || wipe.stdout}`);
    process.exit(1);
  }

  const imp = runCypherFile(toDb, dumpAbs);
  if (!imp.ok) {
    console.error(`[migrate:neo4j] import into ${toDb} failed:\n${imp.stderr || imp.stdout}`);
    process.exit(1);
  }

  unlinkSync(dumpAbs);

  console.log(`[migrate:neo4j] graph copied ${fromDb} → ${toDb}. Point clients at database ${JSON.stringify(toDb)} (e.g. NEO4J_DATABASE=${toDb}).`);

  if (dropLegacy) {
    console.log(`[migrate:neo4j] setting default database to ${JSON.stringify(toDb)} and dropping ${JSON.stringify(fromDb)}…`);
    const def = runCypher("system", `CALL dbms.setDefaultDatabase(${JSON.stringify(toDb)});`);
    if (!def.ok) {
      console.error(`[migrate:neo4j] setDefaultDatabase failed:\n${def.stderr || def.stdout}`);
      process.exit(1);
    }
    const drop = runCypher("system", `DROP DATABASE \`${fromDb}\` IF EXISTS CASCADE ALIASES WAIT;`);
    if (!drop.ok) {
      console.error(`[migrate:neo4j] DROP DATABASE ${fromDb} failed:\n${drop.stderr || drop.stdout}`);
      process.exit(1);
    }
    console.log(`[migrate:neo4j] dropped database ${fromDb}. Default is now ${toDb}.`);
  }
}

main();
