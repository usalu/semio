/** ✂️ Removes the drawing ops of the automatic stages (50 and above) of one person that break the three-views guard (🧪️tests/🧊️three-views: every pose and clip instant draws, and the views keep one extent): per scale with findings the ops of that scale are all left out and put back one at a time in their order; an op stays out when the findings rise with it. Rewrites the ops files without them.
 *  bun prune.ts <slug> <generator-output.json> */
import { Glob } from "bun";
import { join } from "node:path";
import { authoredScales, consistency, sweep } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const [slug, basePath] = process.argv.slice(2) as [string, string];
const dir = join(import.meta.dir, "ops");
const files = [...new Glob(`${slug}.*.json`).scanSync(dir)].sort();
const all: { file: string; index: number; op: any; auto: boolean }[] = [];
for (const file of files) for (const [index, op] of (JSON.parse(await Bun.file(join(dir, file)).text()) as any[]).entries()) all.push({ file, index, op, auto: Number(file.slice(slug.length + 1).split("-")[0]) >= 50 });
const base = await Bun.file(basePath).text();
const pick = (node: any, seg: string) => (Array.isArray(node) ? (seg.startsWith("id=") ? node.findIndex((x) => x?.id === seg.slice(3)) : Number(seg)) : seg);
const apply = (a: any, o: any): boolean => {
  if (o.op === "set") { let node = a; for (const seg of o.path.slice(0, -1)) node = node[pick(node, seg)]; node[pick(node, o.path[o.path.length - 1])] = o.value; return true; }
  if (o.op.startsWith("shape")) {
    const g: any[] = a.representations[o.scale].geometry[o.view];
    const i = g.findIndex((x) => x.id === (o.op === "shape-add" ? o.after : o.id));
    if (i < 0 && !(o.op === "shape-add" && !o.after)) return false;
    if (o.op === "shape-add") g.splice(o.after ? i + 1 : g.length, 0, structuredClone(o.shape));
    else if (o.op === "shape") g[i] = structuredClone(o.shape);
    else if (o.op === "shape-set") Object.assign(g[i], structuredClone(o.set));
    else a.representations[o.scale].geometry[o.view] = g.filter((x) => x.id !== o.id);
    return true;
  }
  const l: any[] = a.volumes[o.scale][o.list];
  if (o.op === "solid-radii") { for (const x of l) if (new RegExp(`^(?:${o.match})$`).test(x.id)) for (const k of ["r0", "r1", "radius"]) if (k in x) x[k] = +(x[k] * o.factor).toFixed(5); return true; }
  const i = l.findIndex((x) => x.id === (o.op === "solid-add" ? o.after : o.id));
  if (o.op === "solid-add") l.splice(o.after ? i + 1 : l.length, 0, structuredClone(o.solid));
  else if (i < 0) return false;
  else if (o.op === "solid-set") for (const [k, v] of Object.entries(o.set)) { if (v === null) delete l[i][k]; else l[i][k] = structuredClone(v); }
  else l.splice(i, 1);
  return true;
};
const out = new Set<number>();
const dropped = new Set<number>();
const build = (): any => {
  const asset = JSON.parse(base).asset;
  dropped.clear();
  for (const [k, e] of all.entries()) if (!out.has(k) && !apply(asset, e.op)) dropped.add(k);
  return asset;
};
const count = (asset: any, scale: number): number => sweep(asset, scale).length + consistency(asset, scale).length;
let asset = build();
for (const scale of authoredScales(asset)) {
  let now = count(asset, scale);
  if (now === 0) continue;
  console.log(`[DEBUG] ${slug} 1:${scale}: ${now} findings`);
  const mine = all.map((e, k) => k).filter((k) => all[k]!.auto && String(all[k]!.op.scale) === String(scale) && all[k]!.op.op.startsWith("shape") && !out.has(k));
  for (const k of mine) out.add(k);
  const floor = count(build(), scale);
  for (const k of mine) {
    out.delete(k);
    const e = all[k]!;
    if (count(build(), scale) > floor || dropped.has(k)) { out.add(k); console.log(`[DEBUG] ${slug} 1:${scale}: ${e.op.op} ${e.op.view} ${e.op.id ?? e.op.shape.id} (${e.file}) left out`); }
  }
  now = floor;
  asset = build();
}
build();
for (const file of files) {
  const keep = all.filter((e, k) => e.file === file && !out.has(k) && !dropped.has(k)).map((e) => e.op);
  if (all.some((e, k) => e.file === file && (out.has(k) || dropped.has(k)))) await Bun.write(join(dir, file), JSON.stringify(keep, null, file.includes("-auto") ? 0 : 1));
}
console.log(`${slug}: ${out.size} ops removed, ${dropped.size} dependent ops dropped, findings left ${authoredScales(asset).reduce((n: number, s: number) => n + count(asset, s), 0)}`);
