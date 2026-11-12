/** 🦴️ Evaluates every pose and four samples of every clip of an asset, carries the tessellated solids by the posed bone frames and reports solids that detach from neighbours they touch at rest (bounding boxes, tolerance 1 cm at rest, 5 cm or 3 % of the largest dimension posed) and connected-component changes.
 * Usage: bun authoring-machinery-poses.ts <slug> [--volume file] */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, QUALITY_FOR, frames, loadAsset, resolveAsset, toWorld, type V3 } from "./volume-lib.ts";

const argv = process.argv.slice(2);
const slug = argv[0]!;
const vi = argv.indexOf("--volume");
const asset = loadAsset(resolveAsset(slug).path);
const vol = vi >= 0 ? (() => { const p = JSON.parse(readFileSync(argv[vi + 1]!, "utf8")); return p.volume ?? p; })() : asset.volume;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
const q = QUALITY_FOR(Math.max(...dims));
const local = vol.solids.map((s: any) => ({ s, g: tessellateSolid({ ...s, node: s.bone } as never, q) }));
type Box = { min: V3; max: V3 };
const framesAt = (query: any) => {
  const st: any = evaluate(asset, query, { kind: "scale", scale: { denominator: 50 } } as any);
  const out: Record<string, any> = {};
  for (const [id, b] of Object.entries<any>(st.bones)) out[id] = { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] };
  return out;
};
const boxes = (fr: Record<string, any>): Box[] => local.map(({ s, g }: any) => {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  const f = fr[s.bone];
  for (let i = 0; i < g.positions.length; i += 3) {
    const p = toWorld(f, [g.positions[i], g.positions[i + 1], g.positions[i + 2]]);
    for (let k = 0; k < 3; k++) { if (p[k]! < min[k]!) min[k] = p[k]!; if (p[k]! > max[k]!) max[k] = p[k]!; }
  }
  return { min, max };
});
const gap = (a: Box, b: Box): number => Math.max(...[0, 1, 2].map((k) => Math.max(a.min[k]! - b.max[k]!, b.min[k]! - a.max[k]!, 0)));
const rest = boxes(frames(asset));
const n = rest.length;
const parentOf = new Map<string, string | null>(asset.rig.bones.map((b: any) => [b.id, b.parent]));
const restFrames = frames(asset);
const near = (box: Box, p: V3, r: number): boolean => [0, 1, 2].every((k) => p[k]! >= box.min[k]! - r && p[k]! <= box.max[k]! + r);
type Joint = { bone: string; parents: number[]; children: number[] };
const joints: Joint[] = [];
for (const b of asset.rig.bones) {
  if (b.parent === null) continue;
  const o = restFrames[b.id]!.o;
  const children = vol.solids.map((_: any, i: number) => i).filter((i: number) => vol.solids[i].bone === b.id && near(rest[i]!, o, 0.03));
  const parents = vol.solids.map((_: any, i: number) => i).filter((i: number) => vol.solids[i].bone === b.parent && near(rest[i]!, o, 0.03));
  if (children.length > 0 && parents.length > 0) joints.push({ bone: b.id, parents, children });
}
const touching: [number, number][] = [];
const components = (bs: Box[], tol: number): number => {
  const parent = bs.map((_, i) => i);
  const find = (x: number): number => (parent[x] === x ? x : (parent[x] = find(parent[x]!)));
  for (let i = 0; i < bs.length; i++) for (let j = i + 1; j < bs.length; j++) if (gap(bs[i]!, bs[j]!) <= tol) parent[find(i)] = find(j);
  return new Set(bs.map((_, i) => find(i))).size;
};
const allTouching = (bs: Box[], tol: number): number => components(bs, tol);
const restComponents = components(rest, 0.01);
const tolerance = Math.max(0.05, 0.03 * Math.max(...dims));
const queries: { name: string; query: any }[] = [];
for (const p of asset.poses) queries.push({ name: `pose ${p.id}`, query: { kind: "pose", pose: p.id } });
for (const c of asset.clips) {
  const d = c.durationS ?? 1;
  for (const f of [0, 0.25, 0.5, 0.75]) queries.push({ name: `clip ${c.id} @${(f * d).toFixed(2)}s`, query: { kind: "clip", clip: c.id, timeS: f * d } });
}
const lines: string[] = [`# ${asset.id}: ${n} solids, ${joints.length} joints with contact solids at rest, ${restComponents} component(s) at rest, tolerance ${tolerance.toFixed(3)} m`];
let problems = 0;
for (const { name, query } of queries) {
  let fr: Record<string, any>;
  try { fr = framesAt(query); } catch (e) { lines.push(`${name}: evaluate failed: ${(e as Error).message}`); problems++; continue; }
  const bs = boxes(fr);
  const detached = joints.map((j) => ({ j, g: Math.min(...j.parents.flatMap((p) => j.children.map((c) => gap(bs[p]!, bs[c]!)))) })).filter((x) => x.g > tolerance).map((x) => `joint ${x.j.bone} ${x.g.toFixed(3)}`);
  const comps = components(bs, tolerance);
  const ext = { min: [0, 1, 2].map((k) => Math.min(...bs.map((b) => b.min[k]!))), max: [0, 1, 2].map((k) => Math.max(...bs.map((b) => b.max[k]!))) };
  const flag = detached.length > 0 || comps > restComponents;
  if (flag) problems++;
  lines.push(`${name}: components ${comps}, detached ${detached.length}${detached.length ? " [" + detached.slice(0, 12).join("; ") + "]" : ""}, extent ${ext.min.map((v) => v.toFixed(2))} .. ${ext.max.map((v) => v.toFixed(2))}${flag ? "  <<<" : ""}`);
}
const out = lines.join("\n");
writeFileSync(join(GENERATED, "authoring-machinery", `${asset.id}.poses.txt`), out + "\n");
console.log(out);
console.log(problems === 0 ? "POSES OK" : `POSES: ${problems} sample(s) flagged`);
