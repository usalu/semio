#!/usr/bin/env bun
/**
 * 🧩 One-shot Neo4j migrations before/after `generate:neo4j`. Idempotent where safe: re-run only applies steps that still match.
 */
import { existsSync, mkdirSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const NEO4J_VERSION = "5.26.26";
const DATABASE = process.env.NEO4J_DATABASE || "semio";
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

function runCypherFile(filePath: string): { ok: boolean; stderr: string } {
  const shell = resolveCypherShell();
  if (!shell) {
    return { ok: false, stderr: "cypher-shell not found" };
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
      DATABASE,
      "--format",
      "plain",
      "-f",
      filePath,
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  const stderr = typeof result.stderr === "string" ? result.stderr : String(result.stderr ?? "");
  return { ok: result.status === 0, stderr };
}
//#endregion 📝CypherShell

//#region 🚀MigrateFieldToKitMembers
const MIGRATION = `
//#region 🏷️RelabelFieldNodes
MATCH (f:Field {kind: 'EMBEDDED'})
SET f:Data
REMOVE f:Field, f.kind;

MATCH (f:Field)
WHERE f.kind IN ['COMPUTED', 'CACHED']
WITH f, f.kind AS k
SET f:Computation, f.cached = (k = 'CACHED')
REMOVE f:Field, f.kind;

MATCH (f:Field {kind: 'REFERENCE'})
SET f:Reference
REMOVE f:Field, f.kind;
//#endregion 🏷️RelabelFieldNodes

//#region 🔗MaterializeTransitiveIsForKitMembers
MATCH (n:Data|Computation|Reference)-[:IS]->(i:Interface)
MATCH (i)-[:IS*1..25]->(b:Interface)
WHERE n <> b
MERGE (n)-[:IS]->(b);
MATCH (n:Data|Computation|Reference)-[:IS]->(c:Class)
MATCH (c)-[:IS*1..25]->(b:Interface)
MERGE (n)-[:IS]->(b);
//#endregion 🔗MaterializeTransitiveIsForKitMembers

//#region 🔍ReplaceFieldIndexes
DROP INDEX index_field_name IF EXISTS;
CREATE RANGE INDEX index_data_name IF NOT EXISTS FOR (n:Data) ON (n.name);
CREATE RANGE INDEX index_computation_name IF NOT EXISTS FOR (n:Computation) ON (n.name);
CREATE RANGE INDEX index_reference_name IF NOT EXISTS FOR (n:Reference) ON (n.name);
DROP INDEX semio_name_fulltext IF EXISTS;
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Constraint|Data|Computation|Reference|Interface|Module|Scalar|Enum) ON EACH [n.name];
//#endregion 🔍ReplaceFieldIndexes
`.trim();
//#endregion 🚀MigrateFieldToKitMembers

function main(): void {
  const cacheDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(cacheDir, { recursive: true });
  const path = join(cacheDir, `neo4j-migrate-field-kit-${process.pid}.cypher`);
  writeFileSync(path, `${MIGRATION}\n`, "utf8");

  const { ok, stderr } = runCypherFile(path);
  try {
    unlinkSync(path);
  } catch {
    /* best-effort */
  }

  if (!ok) {
    console.error(`[generate.neo4j.migrate] migration failed:\n${stderr}`);
    process.exit(1);
  }

  const shell = resolveCypherShell();
  if (!shell) {
    console.error("[generate.neo4j.migrate] cypher-shell missing; cannot verify.");
    process.exit(1);
  }

  const probe = spawnSync(
    shell,
    [
      "-a",
      process.env.NEO4J_URI || "bolt://localhost:7687",
      "-u",
      process.env.NEO4J_USERNAME || "neo4j",
      "-p",
      process.env.NEO4J_PASSWORD || "password",
      "-d",
      DATABASE,
      "--format",
      "plain",
      "MATCH (f:Field) RETURN count(f) AS fieldNodes;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probe.status !== 0) {
    console.error(`[generate.neo4j.migrate] verify query failed: ${probe.stderr}`);
    process.exit(1);
  }

  const tail = String(probe.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .filter(Boolean);
  const last = tail[tail.length - 1] ?? "";
  const fieldNodes = Number.parseInt(last.trim(), 10);
  if (!Number.isFinite(fieldNodes) || fieldNodes !== 0) {
    console.error(`[generate.neo4j.migrate] expected zero :Field nodes after migration; verify output:\n${probe.stdout}`);
    process.exit(1);
  }

  console.log("[generate.neo4j.migrate] Field→Data|Computation|Reference migration + indexes ok.");
}

main();
