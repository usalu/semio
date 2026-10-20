/**
 * 🏗️ Rebuilds `batch-delta.json` from a verified scratch mirror: the SDK mirror keys and glue (computed by sdk-mirror.ts), the sdkfix importer rewrites and the hand-made pieces of `batch-delta.base.json`.
 *
 *   SEMIO_ROOT=<mirror> bun build-delta.ts <plan-sdk.json>
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";

const W = process.env.SEMIO_ROOT!;
const planPath = process.argv[2]!;
const batch = JSON.parse(readFileSync(resolve(import.meta.dir, "../../exports-stage-tools/batch.json"), "utf8"));
const base = JSON.parse(readFileSync(resolve(import.meta.dir, "batch-delta.base.json"), "utf8"));
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const SDK_PJ = "🧰️framework/📦️packages/🟦️typescript/package.json";
const REAL = resolve(import.meta.dir, "../../../../../../../../..");
const sdkNow = JSON.parse(readFileSync(resolve(W, SDK_PJ), "utf8")).exports as Record<string, string>;
const sdkReal = JSON.parse(readFileSync(resolve(REAL, SDK_PJ), "utf8")).exports as Record<string, string>;
const batchAdds = new Map<string, any>(batch.entries.filter((e: any) => e.packageJson === SDK_PJ && e.op === "add" && e.field === "exports" && e.apply).map((e: any) => [e.key, e]));
const entries = [...base.entries];
const glue = [...base.glue];
const idOf = new Map<string, string>();
let n = 0;
for (const [key, value] of Object.entries(sdkNow)) {
  if (key in sdkReal || batchAdds.has(key)) continue;
  const id = `L17-M${String(++n).padStart(3, "0")}`;
  const gid = `L17-GM${String(n).padStart(3, "0")}`;
  const gluePath = relative(resolve(W), resolve(W, SDK_PJ, "..", value)).split("\\").join("/");
  const content = readFileSync(resolve(W, gluePath), "utf8");
  entries.push({ id, op: "add", packageJson: SDK_PJ, package: "@semio-tech/framework", field: "exports", key, value, tier: "export", apply: true, justification: { sources: ["lay17: SDK mirror of a public owner entry (sdk-mirror.json); the glue re-exports every member explicitly from the owner's bare key"] }, glueRef: [gid] });
  glue.push({ id: gid, kind: "create", path: gluePath, content, entry: id, targets: [] });
  idOf.set(key, id);
}
const entryIdOfKey = (key: string): string => idOf.get(key) ?? batchAdds.get(key)?.id ?? (() => { throw Error("no entry for SDK key " + key); })();
const seen = new Set<string>();
const rewrites = [...base.rewrites];
for (const e of plan.edits as { file: string; before: string; after: string; pkg: string; sub: string }[]) {
  const k = e.file + "\0" + e.before;
  if (seen.has(k)) continue;
  seen.add(k);
  rewrites.push({ file: e.file, from: e.before, to: e.after, entry: entryIdOfKey(e.sub) });
}
const caller = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts";
const callerText = readFileSync(resolve(REAL, caller), "utf8");
const callerFrom = /import "([^"]*🎟️caller\/🧪️tests\/🟦️\.ts)";/u.exec(callerText)?.[1];
if (callerFrom) rewrites.push({ file: caller, from: callerFrom, to: "@semio-tech/framework/job/context/testing/caller/testing", entry: entryIdOfKey("./job/context/testing/caller/testing") });
const out = { ...base, entries, glue, rewrites };
writeFileSync(resolve(import.meta.dir, "batch-delta.json"), JSON.stringify(out, null, 1) + "\n");
console.log(`delta: ${entries.length} entries (${n} mirror keys), ${glue.length} glue, ${rewrites.length} rewrites, ${(out.patchGlue ?? []).length} glue patches`);
void existsSync;
