/** 🔁️ The one registered all-plugin rebuild: every component built ONCE for both its descriptor and its dev staging, the
 * generated registry and its check, the restaged `s` guests and their convergence proof, then the trusted catalog of every
 * package, from one tree, in the order `🔣️.json` declares. The whole span runs inside ONE queued exclusive `wasm-build`
 * lease, so no other all-plugin wasm build lands between a descriptor and the staging it describes. Each step is an
 * existing product verb run through nx; a failing step stops the chain. `--from <step>` resumes, `--to <step>` stops. */

import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, getWorkspaceRoot, orchestratorBudgetOpts, runCmd } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { developmentRuntimeRoot, pluginModulesRoot, readActivationReceipt } from "../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { moduleDirectoryName } from "../📦️deployment/🟦️.ts";

/** 🧱️ Where a step sits in the dependency order: descriptors feed the registry, the registry feeds the guests, the guests feed the catalog. */
export const REBUILD_STAGES = ["descriptors", "registry", "guests", "catalog"] as const;

/** 🚦️ The repository resource every all-plugin wasm build serializes on (`lease exclusive wasm-build …` for shell callers). */
export const WASM_BUILD_LEASE = "wasm-build";

/** 🪜️ One declared step of the chain. */
export type RebuildStepV1 = Readonly<{ id: string; stage: (typeof REBUILD_STAGES)[number]; command: readonly string[] }>;

/** 📜️ Reads and validates the declared chain: known schema, unique ids, nx commands, stages in dependency order. */
export function readRebuildChain(path = join(dirname(fileURLToPath(import.meta.url)), "🔣️.json")): readonly RebuildStepV1[] {
  const document = JSON.parse(readFileSync(path, "utf8")) as { schema?: unknown; steps?: unknown };
  if (document.schema !== "semio.plugin-registry.rebuild-chain/v1" || !Array.isArray(document.steps) || document.steps.length === 0) throw new Error("rebuild chain is not a semio.plugin-registry.rebuild-chain/v1 document");
  const ids = new Set<string>();
  let stage = 0;
  return Object.freeze(
    document.steps.map((value: any) => {
      const next = REBUILD_STAGES.indexOf(value?.stage);
      if (typeof value?.id !== "string" || ids.has(value.id) || next < stage || !Array.isArray(value.command) || value.command[0] !== "nx" || value.command.some((part: unknown) => typeof part !== "string"))
        throw new Error(`rebuild chain step ${String(value?.id)} is duplicated, out of stage order, or not an nx command`);
      ids.add(value.id);
      stage = next;
      return Object.freeze({ id: value.id, stage: value.stage, command: Object.freeze([...value.command]) }) as RebuildStepV1;
    }),
  );
}

/** 🧭️ The contiguous step range `--from`/`--to` select, refusing an unknown or inverted bound. */
export function selectRebuildSteps(chain: readonly RebuildStepV1[], segments: readonly string[]): readonly RebuildStepV1[] {
  const bound = (flag: string, fallback: number): number => {
    const index = segments.indexOf(flag);
    if (index === -1) return fallback;
    const position = chain.findIndex((step) => step.id === segments[index + 1]);
    if (position === -1) throw new Error(`${flag} names no step (${chain.map((step) => step.id).join("|")})`);
    return position;
  };
  const flags = segments.filter((_, index) => index % 2 === 0);
  if (segments.length % 2 !== 0 || flags.some((flag) => flag !== "--from" && flag !== "--to") || new Set(flags).size !== flags.length) throw new Error("usage: rebuild-all [--from <step>] [--to <step>]");
  const from = bound("--from", 0);
  const to = bound("--to", chain.length - 1);
  if (from > to) throw new Error("rebuild-all --from comes after --to");
  return chain.slice(from, to + 1);
}

/** 🏁️ `rebuild-all [--from <step>] [--to <step>]`: runs the selected steps in order under one queued exclusive
 * `wasm-build` lease, with a numbered progress line per step; cancellation (SIGINT/SIGTERM) releases the lease. */
export class RebuildAllScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const chain = readRebuildChain();
    const steps = selectRebuildSteps(chain, segments);
    const repoRoot = getWorkspaceRoot();
    const controller = new AbortController(), abort = () => controller.abort();
    process.once("SIGINT", abort);
    process.once("SIGTERM", abort);
    const waited = Date.now();
    const lease = await acquireQueuedResourceLease({ directory: repoCacheDirectory(repoRoot, "agents", "resource-leases"), resource: WASM_BUILD_LEASE, mode: "exclusive", owner: "rebuild-all", signal: controller.signal });
    console.log(`rebuild-all holds ${WASM_BUILD_LEASE} after ${Math.round((Date.now() - waited) / 1000)} s`);
    try {
      for (const step of steps) {
        const index = chain.indexOf(step);
        const started = Date.now();
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} (${step.stage}) started`);
        runCmd("bun", [...step.command], { cwd: repoRoot, ...orchestratorBudgetOpts() });
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} done in ${Math.round((Date.now() - started) / 1000)} s`);
      }
    } finally {
      lease.release();
    }
  }
}

/** 🧾️ One component's convergence row: its committed descriptor, its `dist/component-dev` deliverable, its dev staging
 * and the activation receipt must name ONE build. */
export type StagedConvergenceRowV1 = Readonly<{ pluginId: string; committed: string; dist: string; staged: string; activated: boolean; missing: readonly string[]; ok: boolean }>;

/** 🔍️ Proves `committed == dist == staged` and full staging for every registry component of one dev variant. */
export function stagedConvergence(repoRoot: string, variant: string): Readonly<{ rows: readonly StagedConvergenceRowV1[]; receiptPlugins: number }> {
  const sha = (path: string): string => (existsSync(path) ? createHash("sha256").update(readFileSync(path)).digest("hex") : "-");
  const registry = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"), "utf8")) as readonly { pluginId: string; cratePath: string; wasmOut: string }[];
  const receipt = readActivationReceipt(join(developmentRuntimeRoot(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, "dev", "react"), "activation"));
  const activated = new Set(receipt.plugins.map((row) => row.pluginId));
  const modules = pluginModulesRoot("dev");
  const rows = registry.map((entry) => {
    const descriptor = join(repoRoot, entry.cratePath, "..", "..", "🔣️.json");
    const committed = existsSync(descriptor) ? String((JSON.parse(readFileSync(descriptor, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256 ?? "-") : "-";
    const dist = sha(join(repoRoot, entry.cratePath, "dist", "component-dev", entry.wasmOut));
    const moduleRoot = join(modules, moduleDirectoryName(entry.pluginId));
    const stagedDescriptor = join(moduleRoot, "🔣️.json");
    const staged = existsSync(stagedDescriptor) ? String((JSON.parse(readFileSync(stagedDescriptor, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256 ?? "-") : "-";
    const manifest = join(moduleRoot, ".nx-artifact.json");
    const files = existsSync(manifest) ? (JSON.parse(readFileSync(manifest, "utf8")) as { files: string[] }).files : [];
    const missing = [...files.filter((file) => !existsSync(join(moduleRoot, file))), ...(files.some((file) => file.endsWith("_component.core.wasm")) ? [] : ["*_component.core.wasm"])];
    const ok = committed !== "-" && committed === dist && dist === staged && activated.has(entry.pluginId) && missing.length === 0;
    return Object.freeze({ pluginId: entry.pluginId, committed, dist, staged, activated: activated.has(entry.pluginId), missing: Object.freeze(missing), ok });
  });
  return Object.freeze({ rows: Object.freeze(rows), receiptPlugins: receipt.plugins.length });
}

/** ✅️ `verify-staged --variant <variant>`: one row per component, the summary line, exit 1 when any component diverged. */
export class VerifyStagedScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 2 || segments[0] !== "--variant") throw new Error("usage: verify-staged --variant <variant>");
    const { rows, receiptPlugins } = stagedConvergence(getWorkspaceRoot(), segments[1]!);
    for (const row of rows) console.log(`${row.ok ? "OK  " : "DIFF"} ${row.pluginId.padEnd(34)} committed=${row.committed.slice(0, 12)} dist=${row.dist.slice(0, 12)} staged=${row.staged.slice(0, 12)} activated=${row.activated}${row.missing.length ? ` missing=${row.missing.slice(0, 3).join(",")}` : ""}`);
    const diverged = rows.filter((row) => !row.ok).length;
    console.log(`components=${rows.length} consistent=${rows.length - diverged} diverged=${diverged} receiptPlugins=${receiptPlugins}`);
    if (diverged > 0) process.exitCode = 1;
  }
}
