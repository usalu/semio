// 📸️ Baseline snapshot before the first edit (write it OUTSIDE the ticket tree, e.g. a scratch dir: copies of tracked sources exceed the path limit): manifests of all listed crates plus the compiled source closure (all labels, all features, lib and tests) and small embedded inputs of every crate with planned use-line edits. argv: manifests list, plan json, out dir.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {copyFileSync, existsSync, mkdirSync, readFileSync, statSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {LABELS, ROOT, cfgList, loadNode, readFile, targetsOf} from "./rtd-lib.ts";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const [listFile, planFile, outDir] = process.argv.slice(2) as [string, string, string];
const tolerant = async (p: string) => (await readFile(p)) ?? new Uint8Array(0);
const plan = JSON.parse(readFileSync(planFile, "utf8")) as any[];
const want = new Set(plan.filter(c => c.edits.length > 0).map(c => c.manifest));
const copy = (rel: string, base: string) => {
  const src = join(ROOT, rel);
  if (!existsSync(src)) return false;
  const dst = join(outDir, base, rel);
  mkdirSync(dirname(dst), {recursive: true});
  copyFileSync(src, dst);
  return true;
};
const index: Record<string, string[]> = {};
for (const m of readFileSync(listFile, "utf8").split("\n").filter(Boolean)) {
  copy(m, "manifests");
  if (!want.has(m)) continue;
  const dir = dirname(join(ROOT, m));
  const node = loadNode(dir)!;
  const files = new Set<string>();
  const feats = new Set<string>([...Object.keys(node.table), "default"]);
  const closure = async (entry: string, cfg: string[]) => {
    try { for (const i of await readRustSourceClosureV1([entry], ROOT, cfg, tolerant, {manifestDirectory: node.dir, generated: () => undefined}) as any[]) { let small = false; try { small = statSync(join(ROOT, i.path)).size < 300000; } catch { small = false; } if (i.kind === "source" || small) files.add(i.path); } } catch { /* unmeasurable unit */ }
  };
  for (const label of LABELS) {
    await closure(node.libEntry!, cfgList(label, feats, false));
    for (const t of targetsOf(node, feats)) if (t.kind !== "example") await closure(t.entry, cfgList(label, feats, t.test));
  }
  const rel = [...files].map(f => f.startsWith("/") ? f.replace(ROOT + "/", "") : f);
  for (const f of rel) copy(f, `crates/${node.name}`);
  index[node.name] = rel;
}
writeFileSync(join(outDir, "index.json"), JSON.stringify(index, null, 1));
console.log(Object.keys(index).length, "crates snapshotted;", Object.values(index).reduce((s, v) => s + v.length, 0), "files");
