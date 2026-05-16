/**
 * 🛂 Neo4j → `.repo/🛂/<graph-database>.cypher` export (pure module; invoked from root `script.ts`). Graph id = `joinNeo4jGraphDatabaseName(parts)` (e.g. `["semio","metabolism"]` → `semio-metabolism`); MCP uses `… mcp neo4j … <parts…>` the same way.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const NEO4J_VERSION = "5.26.26";

/** 🗄️Canonical graph specs: each row is argv segments after `neo4j` / `generate neo4j`, joined with `-` for Bolt `NEO4J_DATABASE` and export filename. */
export const NEO4J_GRAPH_DATABASE_SPECS = [
  ["semio"],
  ["elements"],
  ["coda"],
  ["reuse"],
  ["semio", "metabolism"],
] as const;

/** 🔗Bolt user graph name from MCP/generate name segments (hyphen join). */
export function joinNeo4jGraphDatabaseName(parts: readonly string[]): string {
  return parts.join("-");
}

/** 🔀Leading argv tokens until the first `-` flag become the graph name segments; remainder is passed through to `uvx` (MCP only). */
export function partitionNeo4jGraphCliArgv(segments: string[]): { nameParts: string[]; passthrough: string[] } {
  const nameParts: string[] = [];
  let i = 0;
  while (i < segments.length && !segments[i]!.startsWith("-")) {
    nameParts.push(segments[i]!);
    i += 1;
  }
  return { nameParts, passthrough: segments.slice(i) };
}

export const NEO4J_GRAPH_DATABASE_NAMES = NEO4J_GRAPH_DATABASE_SPECS.map((s) => joinNeo4jGraphDatabaseName(s));

export type Neo4jGraphDatabaseName = (typeof NEO4J_GRAPH_DATABASE_NAMES)[number];

const NEO4J_GRAPH_DATABASE_LOOKUP = new Set<string>(NEO4J_GRAPH_DATABASE_NAMES);

export class Neo4jCypherExport {
  constructor(private readonly repoRoot: string) {}

  resolveCypherShell(): string | null {
    const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
    const cachedShell = join(this.repoRoot, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
    const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));

    for (const candidate of candidates) {
      if (candidate.includes("/") || candidate.includes("\\")) {
        if (existsSync(candidate)) return candidate;
        continue;
      }
      const probe = spawnSync(candidate, ["--version"], { stdio: "ignore" });
      if (probe.status === 0) return candidate;
    }
    return null;
  }

  buildCypherEnv(): NodeJS.ProcessEnv {
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

  runCypher(database: string, cypher: string): { ok: boolean; stdout: string; stderr: string } {
    const shell = this.resolveCypherShell();
    if (!shell) {
      return { ok: false, stdout: "", stderr: "cypher-shell not found (install Neo4j tools or set NEO4J_CYPHER_SHELL)." };
    }

    const queryDir = join(this.repoRoot, ".repo", "cache");
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
          cwd: this.repoRoot,
          encoding: "utf8",
          env: this.buildCypherEnv(),
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
        /* temp query cleanup */
      }
    }
  }

  apocExportCypherAllToAbsoluteFile(database: string, absoluteFile: string): { ok: boolean; message: string } {
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

    const { ok, stdout, stderr } = this.runCypher(database, cypher);
    if (!ok) {
      return {
        ok: false,
        message: `${stderr || stdout || "unknown error"}\n` +
          "Ensure APOC is installed, apoc.export.file.enabled=true, and Neo4j may write this absolute path (set apoc.import.file.use_neo4j_config=false on Desktop — see setup scripts).",
      };
    }
    return { ok: true, message: stdout.trim() };
  }

  writeGeneratedCypherBundle(technology: string, database: string, body: string, finalPath: string): void {
    const stamp = new Date().toISOString();
    const header = [
      "// SPDX-License-Identifier: AGPL-3.0-only",
      "// Generated exclusively from the live Neo4j database — do not edit this file by hand.",
      "// Refresh: `bun run generate` (root `script.ts`).",
      `// technology: ${technology} | database: ${database} | generated: ${stamp}`,
      "//",
      "",
    ].join("\n");

    writeFileSync(finalPath, `${header}${body.trim()}\n`, "utf8");
  }

  tryExportFromArgv(argv: string[]): boolean {
    const { nameParts, passthrough } = partitionNeo4jGraphCliArgv(argv);
    if (passthrough.length > 0) {
      console.error(`[generate:neo4j] unexpected extra arguments (use only graph name segments before any -flags): ${JSON.stringify(passthrough)}`);
      return false;
    }
    const joined =
      nameParts.length > 0 ? joinNeo4jGraphDatabaseName(nameParts) : (process.env.NEO4J_DATABASE ?? "semio");
    if (!NEO4J_GRAPH_DATABASE_LOOKUP.has(joined)) {
      console.error(
        `[generate:neo4j] graph database must be one of: ${NEO4J_GRAPH_DATABASE_NAMES.join(", ")} (got ${JSON.stringify(joined)}; argv segments ${JSON.stringify(nameParts)})`,
      );
      return false;
    }

    const technology = joined;
    const database = process.env.NEO4J_DATABASE ?? joined;
    const outDir = join(this.repoRoot, ".repo", "🛂");
    mkdirSync(outDir, { recursive: true });

    const finalAbs = join(outDir, `${technology}.cypher`);
    const cacheDir = join(this.repoRoot, ".repo", "cache");
    mkdirSync(cacheDir, { recursive: true });
    const tmpAbs = join(cacheDir, `.generate-${technology}-${process.pid}.tmp.cypher`);

    const probe = this.runCypher(database, "RETURN 1 AS ok;");
    if (!probe.ok) {
      console.error(`[generate:neo4j] cannot reach database ${JSON.stringify(database)}:\n${probe.stderr || probe.stdout}`);
      return false;
    }

    if (existsSync(tmpAbs)) unlinkSync(tmpAbs);

    const result = this.apocExportCypherAllToAbsoluteFile(database, tmpAbs);
    if (!result.ok) {
      console.error(`[generate:neo4j] apoc.export.cypher.all failed:\n${result.message}`);
      return false;
    }

    if (!existsSync(tmpAbs)) {
      console.error(`[generate:neo4j] expected export file missing at ${tmpAbs} after APOC call.`);
      return false;
    }

    const body = readFileSync(tmpAbs, "utf8");
    unlinkSync(tmpAbs);
    this.writeGeneratedCypherBundle(technology, database, body, finalAbs);

    console.log(`[generate:neo4j] wrote ${finalAbs} (database ${database}).`);
    if (result.message) console.log(result.message);
    return true;
  }

  runFromArgv(argv: string[]): void {
    if (!this.tryExportFromArgv(argv)) process.exit(1);
  }
}
