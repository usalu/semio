/**
 * 🧩 Merges `batch-delta.json` into the exports-stage batch (`exports-stage-tools/batch.json`).
 *
 *   bun merge-delta.ts            dry run: reports what would change
 *   bun merge-delta.ts --write    writes batch.json (baselines of touched manifests are taken from the current tree)
 *   --batch=<file> --root=<dir>   merge into another batch copy and read baselines from a scratch mirror (verification)
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const flag = (name: string): string | undefined => process.argv.find(a => a.startsWith(`--${name}=`))?.slice(name.length + 3);
const ROOT = flag("root") ? resolve(flag("root")!) : resolve(import.meta.dir, "../../../../../../../../..");
const batchPath = flag("batch") ? resolve(flag("batch")!) : resolve(import.meta.dir, "../../exports-stage-tools/batch.json");
const batch = JSON.parse(readFileSync(batchPath, "utf8"));
const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "batch-delta.json"), "utf8"));
const write = process.argv.includes("--write");
const problems: string[] = [];
const ids = new Set<string>(batch.entries.map((e: { id: string }) => e.id).concat(batch.glue.map((g: { id: string }) => g.id)));
for (const row of [...delta.entries, ...delta.glue]) if (ids.has(row.id)) problems.push(`id already merged: ${row.id}`);
for (const e of delta.entries) {
  const clash = batch.entries.find((x: any) => x.packageJson === e.packageJson && x.field === e.field && x.key === e.key && x.op === e.op);
  if (clash) problems.push(`batch already carries ${e.package} ${e.field}[${e.key}] (${clash.id})`);
  if (!batch.baseline[e.packageJson]) {
    const bytes = readFileSync(resolve(ROOT, e.packageJson));
    batch.baseline[e.packageJson] = { sha256: createHash("sha256").update(bytes).digest("hex"), bytes: bytes.length };
  }
}
const dropped = delta.dropRewrites.filter((d: any) => batch.rewrites.some((r: any) => r.file === d.file && r.from === d.from));
batch.rewrites = batch.rewrites.filter((r: any) => !delta.dropRewrites.some((d: any) => d.file === r.file && d.from === r.from));
for (const patch of delta.patchGlue ?? []) {
  const hit = batch.glue.find((g: any) => g.path === patch.path && g.kind === "create");
  if (!hit) { problems.push(`glue to patch is not in the batch: ${patch.path}`); continue; }
  hit.content = patch.content;
}
batch.entries.push(...delta.entries);
batch.glue.push(...delta.glue);
batch.rewrites.push(...delta.rewrites);
console.log(`delta: ${(delta.patchGlue ?? []).length} glue patched, +${delta.entries.length} entries, +${delta.glue.length} glue, +${delta.rewrites.length} rewrites, -${dropped.length} stale rewrites`);
if (problems.length) { console.error("REFUSED:\n- " + problems.join("\n- ")); process.exit(3); }
if (write) { writeFileSync(batchPath, JSON.stringify(batch, null, 1) + "\n"); console.log("written " + batchPath); } else console.log("dry run: nothing written");
