/**
 * 🧪 Builds a scratch mirror of every file the batch reads or writes (manifests, glue, re-export targets, importers) so that
 * `apply.ts apply --root=<dir> --freeze-lifted` can be exercised end to end without touching the repository.
 *
 *   bun scratch.ts <dir>
 */
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../../..");
const out = process.argv[2];
if (!out) throw Error("usage: bun scratch.ts <dir>");
const batch = JSON.parse(readFileSync(resolve(import.meta.dir, "batch.json"), "utf8"));
rmSync(out, { recursive: true, force: true });
const files = new Set<string>(Object.keys(batch.baseline));
for (const g of batch.glue) { if (g.kind !== "create") files.add(g.path); for (const t of g.targets ?? []) files.add(t.path); }
for (const r of batch.rewrites) files.add(r.file);
for (const e of batch.entries) if (e.field === "exports" && e.op === "add") files.add(resolve(dirname(resolve(ROOT, e.packageJson)), e.value).slice(ROOT.length + 1));
let n = 0;
for (const f of files) { const src = resolve(ROOT, f); if (!existsSync(src)) continue; const dst = resolve(out, f); mkdirSync(dirname(dst), { recursive: true }); copyFileSync(src, dst); n++; }
console.log(`${n} files mirrored into ${out}`);
