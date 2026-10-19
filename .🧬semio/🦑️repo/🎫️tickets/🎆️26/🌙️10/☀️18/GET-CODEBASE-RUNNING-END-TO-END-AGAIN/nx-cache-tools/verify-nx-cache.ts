#!/usr/bin/env bun
/**
 * 🔬️ Proves that the Nx local task cache is populated and restores results.
 *
 * Usage (run from anywhere, after dependencies sync and tools setup are green):
 *   bun <ticket>/nx-cache-tools/verify-nx-cache.ts [<project:target>]
 *
 * Default target: @teaching/architecture-course:test (cache: true, no dependsOn).
 * Steps: 1) preconditions, 2) `repo cache status`, 3) run with --skip-cache (forced miss that still stores),
 * 4) `repo cache status` must show entries, 5) identical run must be served from the cache, 6) `repo cache status` unchanged.
 * Logs: <ticket>/🗑️generated/nx-cache-tools/*.log (delete after reading). Exit code 0 only when every assertion holds.
 */
import { existsSync, mkdirSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const root = resolve(import.meta.dir, "../../../../../../..", "..").replace(/[\\/]+$/u, "");
const ticket = resolve(import.meta.dir, "..");
const target = process.argv[2] ?? "@teaching/architecture-course:test";
const logs = join(ticket, "🗑️generated", "nx-cache-tools");
mkdirSync(logs, { recursive: true });

const fail = (message: string): never => { console.error(`[verify-nx-cache] FAIL: ${message}`); process.exit(1); };
const processes = spawnSync("ps", ["-Ao", "command"], { encoding: "utf8" }).stdout.split("\n").filter(line => /bun repo (dependencies|show)/u.test(line) && !line.includes("verify-nx-cache"));
if (processes.length) fail(`dependency or graph processes are running, wait for them:\n${processes.join("\n")}`);
try { realpathSync(join(root, "node_modules")); } catch { fail("node_modules does not resolve; run `bun repo dependencies sync --all` first"); }

const run = (name: string, args: string[]) => {
  const started = performance.now();
  const result = spawnSync("bun", ["repo", ...args], { cwd: root, encoding: "utf8", maxBuffer: 512 * 1024 * 1024, timeout: 25 * 60_000, env: { ...process.env, NO_COLOR: "1", FORCE_COLOR: "0" } });
  const text = `${result.stdout}\n${result.stderr}`;
  writeFileSync(join(logs, `${name}.log`), text);
  console.log(`[verify-nx-cache] ${name}: exit ${result.status} in ${Math.round(performance.now() - started)} ms (log ${join(logs, `${name}.log`)})`);
  return { status: result.status, text, stdout: result.stdout };
};
const status = (name: string) => {
  const result = run(name, ["cache", "status", "--tool=nx"]);
  if (result.status !== 0) fail(`${name} exited ${result.status}`);
  const start = result.stdout.lastIndexOf('{\n  "version"');
  if (start < 0) fail(`${name} printed no CacheStatus; inspect ${join(logs, name + ".log")}`);
  return JSON.parse(result.stdout.slice(start, result.stdout.lastIndexOf("}") + 1)) as { available: boolean; directory: string; entries: number; bytes: number };
};

const before = status("status-before");
const miss = run("run-forced-miss", ["run", target, "--skip-cache", "--output=static"]);
if (miss.status !== 0) fail(`forced run exited ${miss.status}; inspect ${join(logs, "run-forced-miss.log")}`);
const stored = status("status-after-store");
if (!stored.available || stored.entries <= before.entries && stored.bytes <= before.bytes) fail(`the cache did not grow after a stored run: ${JSON.stringify({ before, stored })}`);
const hit = run("run-expect-hit", ["run", target, "--output=static"]);
if (hit.status !== 0) fail(`second run exited ${hit.status}; inspect ${join(logs, "run-expect-hit.log")}`);
if (!/read the output from the cache|\[local cache\]|\[existing outputs match the cache\]/u.test(hit.text)) fail(`the second run was not served from the cache; inspect ${join(logs, "run-expect-hit.log")}`);
const after = status("status-after-hit");
console.log(JSON.stringify({ verdict: "PASS", target, cacheDirectory: stored.directory, before, stored, after, expectedDirectory: existsSync(stored.directory) }, null, 2));
