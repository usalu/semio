/**
 * 🎬 Re-stages the whole batch from the live tree (about 6 minutes under load): policy dump, scan, violations, moves, groups,
 * candidates, dependency gaps, export health, assembly, validation, simulation, reconciliation.
 *
 *   bun stage.ts [--refresh]    --refresh drops the cached file list and scan first
 */
import { existsSync, mkdirSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { GENERATED } from "./lib.ts";

mkdirSync(GENERATED, { recursive: true });
if (process.argv.includes("--refresh")) for (const f of ["all-files.txt", "scan.json", "ls-_package_json.txt", "policy.json"]) rmSync(resolve(GENERATED, f), { force: true });
const here = import.meta.dir;
const run = (cmd: string, ...args: string[]): void => {
  const t = Date.now();
  const r = spawnSync(cmd, args, { stdio: "inherit", cwd: resolve(here, "../../../../../../../.."), env: process.env });
  console.log(`[stage] ${args[0]!.split("/").pop()} rc=${r.status} ${Math.round((Date.now() - t) / 1000)}s`);
  if (r.status !== 0) process.exit(r.status ?? 1);
};
if (!existsSync(resolve(GENERATED, "policy.json"))) run("node", resolve(here, "policy-dump.cjs"), resolve(GENERATED, "policy.json"));
for (const step of ["scan", "ncpr-now", "moves", "groups", "build-candidates", "deps", "health", "assemble", "validate", "simulate", "reconcile", "missing-keys", "import-names"]) run("bun", resolve(here, `${step}.ts`));
