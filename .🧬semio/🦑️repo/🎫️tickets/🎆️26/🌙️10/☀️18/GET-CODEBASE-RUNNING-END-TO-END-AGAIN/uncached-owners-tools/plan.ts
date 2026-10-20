/** 🧾 Turns the classification into the manifest operations `apply-manifests.ts` writes (data only, keyed by manifest and target so a re-plan right before the write survives concurrent edits).
 *
 *   bun plan.ts   # reads classification.json and the manifests, writes manifest-ops.json next to this file
 *
 * An operation states the end state of one target: `cache`, `continuous`, `outputs`, `inputs` and the `metadata.semio.cache` decision. A target that is already in that state is skipped at apply time.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { load } from "../cache-fix-tools/overlay.ts";
import type { Decision, Row } from "./decide.ts";

const WS = "/Users/ueli/Documents/semio", G = join(import.meta.dir, "../🗑️generated/uncached-owners");
export type Op = { manifest: string; target: string; id: string; cache: boolean; continuous?: true; outputs?: string[]; inputs?: unknown[]; decision?: { version: 1 } & Decision };

const TEST_ENV = [{ env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }, { env: "SEMIO_BUILD_BUDGET_MS" }];
const COURSE = "{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/**/*";
const EXTRA_INPUTS: Record<string, unknown[]> = {
  "@teaching/proctor:check": [COURSE, { env: "PROCTOR_COURSE" }],
  "@teaching/architecture-course:check": ["{workspaceRoot}/🎓️teaching/🛂️proctor/**/*", "{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/rust-toolchain.toml", { env: "PROCTOR_COURSE" }],
};
const key = (value: unknown) => JSON.stringify(value);
const union = (...lists: unknown[][]) => { const seen = new Set<string>(), out: unknown[] = []; for (const item of lists.flat()) if (!seen.has(key(item))) { seen.add(key(item)); out.push(item); } return out; };

const staged = await load("staged");
const rows: Row[] = JSON.parse(readFileSync(join(import.meta.dir, "classification.json"), "utf8"));
const ops: Op[] = [], manifests = new Map<string, any>();
for (const row of rows) {
  const json = manifests.get(row.manifest) ?? manifests.set(row.manifest, JSON.parse(readFileSync(join(WS, row.manifest), "utf8"))).get(row.manifest);
  const target = json.targets[row.target], base: Op = { manifest: row.manifest, target: row.target, id: row.id, cache: row.disposition === "cached" };
  if (row.disposition !== "cached") { ops.push({ ...base, ...(row.disposition === "interactive" ? { continuous: true as const } : {}), decision: { version: 1, ...row.decision } }); continue; }
  const sibling = json.targets.test, bare = row.target.replace(/^[^A-Za-z0-9]+/u, "");
  const explicit: unknown[] | undefined = target.inputs ?? (sibling && sibling.cache !== false && Array.isArray(sibling.inputs) ? sibling.inputs : undefined);
  const extra = EXTRA_INPUTS[row.id] ?? [];
  const named = (explicit ?? []).includes("testEnv");
  const inputs = explicit === undefined && !extra.length && bare.startsWith("test") ? undefined : union(explicit ?? ["default", "^default"], named ? [] : TEST_ENV, extra);
  const klass = staged.module.targetClassV1(bare, staged.POLICY.targetClasses);
  const declared = klass?.class === "deterministic" && klass.reason === row.decision.reason ? undefined : { version: 1 as const, ...row.decision };
  ops.push({ ...base, ...(target.outputs === undefined ? { outputs: [] } : {}), ...(inputs !== undefined && key(inputs) !== key(target.inputs) ? { inputs } : {}), ...(declared ? { decision: declared } : {}) });
}
writeFileSync(join(import.meta.dir, "manifest-ops.json"), JSON.stringify(ops, null, 1) + "\n");
console.log(ops.length, "operations in", manifests.size, "manifests;", ops.filter((o) => o.cache).length, "cached,", ops.filter((o) => o.continuous).length, "interactive,", ops.filter((o) => !o.cache && !o.continuous).length, "uncached;", ops.filter((o) => o.cache && o.decision).length, "cached with an explicit decision");
