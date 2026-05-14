#!/usr/bin/env bun
/**
 * 🧹 Cleans every node in the active Neo4j graph (`NEO4J_DATABASE`, default `semio`):
 * - Removes legacy / introspection props (`entityKind`, `schemaTag`, …) and denormalized **owner names / ids / kinds**
 *   (`ownerName`, `owner`, `ownerId`, `ownerKind`, …) so ownership is not duplicated as scalars.
 * - Rewrites legacy ownership **relationships** `OWNS`, `OWNED_BY`, `OWNER` into canonical **`HAS`** (same semantics:
 *   `OWNS`/`OWNER`: `(owner)-[:HAS]->(child)`; `OWNED_BY`: `(parent)-[:HAS]->(child)`).
 *
 * Usage: `bun ./prune.neo4j.script.ts`
 *
 * Uses APOC `apoc.periodic.iterate` when available so large graphs do not single-transaction OOM; otherwise one `MATCH (n) REMOVE …`.
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const NEO4J_VERSION = "5.26.26";
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
    return { ok: false, stdout: "", stderr: "cypher-shell not found." };
  }

  const queryDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(queryDir, { recursive: true });
  const queryPath = join(queryDir, `neo4j-prune-query-${process.pid}-${Date.now()}.cypher`);
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
//#endregion 📝CypherShell

//#region 🧹Prune
/** @emoji 🧹 Keys stripped from every node; missing keys are ignored by Cypher REMOVE. */
const STRIPPED_NODE_KEYS = [
  "entityKind",
  "schemaTag",
  "schemaSource",
  "id",
  "emoji",
  "moduleName",
  "domain",
  "ownerName",
  "owner_name",
  "ownerNames",
  "ownedByName",
  "owned_by_name",
  "ownerLabel",
  "ownerId",
  "owner_id",
  "ownerKind",
  "owner_kind",
  "parentName",
  "parent_name",
  "owner",
] as const;

const REMOVE_NODE_PROPS = `n.${STRIPPED_NODE_KEYS.join(", n.")}`;

const PRUNE_BATCHED = [
  "CALL apoc.periodic.iterate(",
  `  'MATCH (n) RETURN n',`,
  `  'WITH n REMOVE ${REMOVE_NODE_PROPS}',`,
  "  { batchSize: 5000 }",
  ") YIELD batches, total RETURN batches, total;",
].join("\n");

const PRUNE_SIMPLE = `MATCH (n) REMOVE ${REMOVE_NODE_PROPS} RETURN count(n) AS nodes_updated;`;

/** @emoji 🔗 Rewrites non-HAS ownership rel kinds into `HAS` (idempotent MERGE per pair). */
const OWNERSHIP_TO_HAS: readonly { label: string; cypher: string }[] = [
  {
    label: "OWNS→HAS",
    cypher: `MATCH (a)-[r:OWNS]->(b)
MERGE (a)-[:HAS]->(b)
DELETE r`,
  },
  {
    label: "OWNED_BY→HAS",
    cypher: `MATCH (child)-[r:OWNED_BY]->(parent)
MERGE (parent)-[:HAS]->(child)
DELETE r`,
  },
  {
    label: "OWNER→HAS",
    cypher: `MATCH (a)-[r:OWNER]->(b)
MERGE (a)-[:HAS]->(b)
DELETE r`,
  },
];

function hasApocPeriodic(database: string): boolean {
  const q =
    "SHOW PROCEDURES YIELD name WHERE name STARTS WITH 'apoc.periodic.iterate' RETURN count(*) AS c;";
  const r = runCypher(database, q);
  return r.ok && /\b[1-9]\d*\b/.test(r.stdout);
}

function main(): void {
  const database = process.env.NEO4J_DATABASE || "semio";
  const probe = runCypher(database, "RETURN 1 AS ok;");
  if (!probe.ok) {
    console.error(`[prune:neo4j] cannot reach database ${JSON.stringify(database)}:\n${probe.stderr || probe.stdout}`);
    process.exit(1);
  }

  const useBatch = hasApocPeriodic(database);
  console.log(`[prune:neo4j] stripping legacy props on all nodes in ${JSON.stringify(database)} (${useBatch ? "batched APOC" : "single statement"})…`);

  const result = runCypher(database, useBatch ? PRUNE_BATCHED : PRUNE_SIMPLE);
  if (!result.ok) {
    if (useBatch) {
      console.log("[prune:neo4j] batched prune failed; retrying single-statement REMOVE…");
      const retry = runCypher(database, PRUNE_SIMPLE);
      if (!retry.ok) {
        console.error(`[prune:neo4j] failed:\n${retry.stderr || retry.stdout}`);
        process.exit(1);
      }
      console.log(retry.stdout.trim());
    } else {
      console.error(`[prune:neo4j] failed:\n${result.stderr || result.stdout}`);
      process.exit(1);
    }
  } else {
    console.log(result.stdout.trim());
  }

  console.log("[prune:neo4j] migrating legacy ownership rels to HAS…");
  for (const step of OWNERSHIP_TO_HAS) {
    const mig = runCypher(database, step.cypher);
    if (!mig.ok) {
      console.warn(`[prune:neo4j] ${step.label} skipped or failed:\n${mig.stderr || mig.stdout}`);
    }
  }

  console.log(
    `[prune:neo4j] done (stripped ${STRIPPED_NODE_KEYS.length} optional node keys incl. owner*; OWNS/OWNED_BY/OWNER → HAS). Re-export with \`bun run export:neo4j\` if you want \`.repo/🛂/*.cypher\` to match.`,
  );
}

main();
//#endregion 🧹Prune
