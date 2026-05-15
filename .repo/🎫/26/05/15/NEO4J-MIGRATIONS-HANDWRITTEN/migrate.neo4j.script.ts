#!/usr/bin/env bun
/**
 * 🧩 Runs ticket migrations.cypher via cypher-shell; copies batch to .repo/cache for Windows -f paths. NEO4J_DATABASE defaults to semio; not used by generate.script.ts.
 */
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧭Constants
const TICKET_DIR = import.meta.dir;
const NEO4J_VERSION = "5.26.26";
const DATABASE = process.env.NEO4J_DATABASE || "semio";
const MIGRATIONS_CYPHER = join(TICKET_DIR, "migrations.cypher");
//#endregion 🧭Constants

//#region 🧭ResolveRepoRoot
/**
 * 📁 Finds workspace root (directory containing generate.neo4j.script.ts) for Neo4j tool cache paths.
 */
function resolveRepoRoot(start: string): string {
  let dir = start;
  for (let i = 0; i < 24; i++) {
    if (existsSync(join(dir, "generate.neo4j.script.ts"))) {
      return dir;
    }
    const parent = dirname(dir);
    if (parent === dir) {
      break;
    }
    dir = parent;
  }
  throw new Error("[migrate:neo4j] could not locate repo root (generate.neo4j.script.ts missing in parents).");
}
//#endregion 🧭ResolveRepoRoot

//#region 📝CypherShell
const REPO_ROOT = resolveRepoRoot(TICKET_DIR);

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

//#region 🚀Entry
function main(): void {
  if (!existsSync(MIGRATIONS_CYPHER)) {
    console.error(`[migrate:neo4j] missing ${MIGRATIONS_CYPHER}`);
    process.exit(1);
  }

  const cacheDir = join(REPO_ROOT, ".repo", "cache");
  mkdirSync(cacheDir, { recursive: true });
  const batchPath = join(cacheDir, `neo4j-migrate-handwritten-${process.pid}.cypher`);
  const body = readFileSync(MIGRATIONS_CYPHER, "utf8");
  writeFileSync(batchPath, `${body.trim()}\n`, "utf8");

  const { ok, stderr } = runCypherFile(batchPath);
  try {
    unlinkSync(batchPath);
  } catch {
    /* best-effort */
  }

  if (!ok) {
    console.error(`[migrate:neo4j] migration failed:\n${stderr}`);
    process.exit(1);
  }

  const shell = resolveCypherShell();
  if (!shell) {
    console.error("[migrate:neo4j] cypher-shell missing; cannot verify.");
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
    console.error(`[migrate:neo4j] verify query failed: ${probe.stderr}`);
    process.exit(1);
  }

  const tail = String(probe.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .filter(Boolean);
  const last = tail[tail.length - 1] ?? "";
  const fieldNodes = Number.parseInt(last.trim(), 10);
  if (!Number.isFinite(fieldNodes) || fieldNodes !== 0) {
    console.error(`[migrate:neo4j] expected zero :Field nodes after migration; verify output:\n${probe.stdout}`);
    process.exit(1);
  }

  const probeFk = spawnSync(
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
      "OPTIONAL MATCH (m:Module {name:'FieldKind'}) WITH count(m) AS fieldKindModules OPTIONAL MATCH (e:Enum) RETURN fieldKindModules, count(e) AS enumNodes;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeFk.status !== 0) {
    console.error(`[migrate:neo4j] FieldKind/Enum verify failed: ${probeFk.stderr}`);
    process.exit(1);
  }

  const fkTail = String(probeFk.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const fkLast = fkTail[fkTail.length - 1] ?? "";
  const nums = fkLast.match(/\d+/g)?.map((d) => Number.parseInt(d, 10)) ?? [];
  if (nums.length < 2 || nums[0] !== 0 || nums[1] !== 0) {
    console.error(`[migrate:neo4j] expected zero :Module FieldKind and zero :Enum after migration; verify output:\n${probeFk.stdout}`);
    process.exit(1);
  }

  const probeEntityMod = spawnSync(
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
      "MATCH (m:Module {name:'Entity'}) RETURN count(m) AS entityModules;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeEntityMod.status !== 0) {
    console.error(`[migrate:neo4j] Entity module verify failed: ${probeEntityMod.stderr}`);
    process.exit(1);
  }

  const entTail = String(probeEntityMod.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const entLast = entTail[entTail.length - 1] ?? "";
  const entityModCount = Number.parseInt(entLast.trim(), 10);
  if (!Number.isFinite(entityModCount) || entityModCount !== 1) {
    console.error(`[migrate:neo4j] expected exactly one :Module Entity after migration; verify output:\n${probeEntityMod.stdout}`);
    process.exit(1);
  }

  const probeEntityLadder = spawnSync(
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
      "MATCH (:Module {name:'Entity'})-[:OWNS]->(w:Module {name:'WeakEntity'}) RETURN count(w) AS weakUnderEntity;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeEntityLadder.status !== 0) {
    console.error(`[migrate:neo4j] Entity ladder verify failed: ${probeEntityLadder.stderr}`);
    process.exit(1);
  }

  const elTail = String(probeEntityLadder.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const elLast = elTail[elTail.length - 1] ?? "";
  const weakUnderEntity = Number.parseInt(elLast.trim(), 10);
  if (!Number.isFinite(weakUnderEntity) || weakUnderEntity < 1) {
    console.error(`[migrate:neo4j] expected Entity to OWNS WeakEntity; verify output:\n${probeEntityLadder.stdout}`);
    process.exit(1);
  }

  const probeDomainKit = spawnSync(
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
      "MATCH (:Module {name:'Domain'})-[:OWNS]->(k:Module {name:'Kit'}) RETURN count(k) AS domainOwnsKit;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeDomainKit.status !== 0) {
    console.error(`[migrate:neo4j] Domain Kit verify failed: ${probeDomainKit.stderr}`);
    process.exit(1);
  }

  const dkTail = String(probeDomainKit.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const dkLast = dkTail[dkTail.length - 1] ?? "";
  const domainOwnsKit = Number.parseInt(dkLast.trim(), 10);
  if (!Number.isFinite(domainOwnsKit) || domainOwnsKit < 1) {
    console.error(`[migrate:neo4j] expected Domain to OWNS Kit; verify output:\n${probeDomainKit.stdout}`);
    process.exit(1);
  }

  const probeDomainVcs = spawnSync(
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
      "MATCH (:Module {name:'Domain'})-[:OWNS]->(v:Module {name:'VCS'}) RETURN count(v) AS domainOwnsVcs;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeDomainVcs.status !== 0) {
    console.error(`[migrate:neo4j] Domain VCS verify failed: ${probeDomainVcs.stderr}`);
    process.exit(1);
  }

  const dvTail = String(probeDomainVcs.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const dvLast = dvTail[dvTail.length - 1] ?? "";
  const domainOwnsVcs = Number.parseInt(dvLast.trim(), 10);
  if (!Number.isFinite(domainOwnsVcs) || domainOwnsVcs < 1) {
    console.error(`[migrate:neo4j] expected Domain to OWNS VCS; verify output:\n${probeDomainVcs.stdout}`);
    process.exit(1);
  }

  const probeLegacyHas = spawnSync(
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
      "MATCH ()-[r:HAS]->() RETURN count(r) AS hasRelCount;",
    ],
    { encoding: "utf8", cwd: REPO_ROOT, env: buildCypherEnv() },
  );

  if (probeLegacyHas.status !== 0) {
    console.error(`[migrate:neo4j] legacy HAS rel verify failed: ${probeLegacyHas.stderr}`);
    process.exit(1);
  }

  const lhTail = String(probeLegacyHas.stdout ?? "")
    .trim()
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  const lhLast = lhTail[lhTail.length - 1] ?? "";
  const hasRelCount = Number.parseInt(lhLast.trim(), 10);
  if (!Number.isFinite(hasRelCount) || hasRelCount !== 0) {
    console.error(`[migrate:neo4j] expected zero :HAS relationships after migration; verify output:\n${probeLegacyHas.stdout}`);
    process.exit(1);
  }

  console.log("[migrate:neo4j] handwritten migrations ok.");
}

main();
//#endregion 🚀Entry
