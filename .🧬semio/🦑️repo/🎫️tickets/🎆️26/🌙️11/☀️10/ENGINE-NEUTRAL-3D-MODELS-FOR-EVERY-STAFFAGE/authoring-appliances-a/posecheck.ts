/** 🦴️ Evaluates poses of an asset and checks the volume's solids stay together: `bun posecheck.ts <slug> [pose...]` (default: all poses) */
import { STAFFAGE, frames, loadAsset, resolveAsset, meshes, extentOf, QUALITY_FOR } from "../volume-lib.ts";

const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { carry } = await import(`${STAFFAGE}/🔨️modules/🦴️rig/🟦️.ts`);
const slug = process.argv[2]!;
const asset = loadAsset(resolveAsset(slug).path);
const synthetic = (process.argv.slice(3)).filter((a) => a.includes(":"));
for (const spec of synthetic) {
  const bones: Record<string, any> = {};
  for (const part of spec.split(",")) { const [b, k, v] = part.split(":"); bones[b!] = { ...(bones[b!] ?? {}), [k === undefined || v === undefined ? "angleDeg" : k]: Number(v ?? k) }; }
  asset.poses = [...asset.poses, { ...asset.poses[0], id: spec, bones }];
}
const solids = asset.volume.solids as any[];
const d = asset.dimensions;
const rest = frames(asset);
const list = meshes(asset, solids, Math.max(d.widthM, d.heightM, d.depthM));
type Box = { min: number[]; max: number[] };
const boxOf = (w: Float64Array): Box => {
  const min = [1e9, 1e9, 1e9], max = [-1e9, -1e9, -1e9];
  for (let i = 0; i < w.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, w[i + k]!); max[k] = Math.max(max[k]!, w[i + k]!); }
  return { min, max };
};
const gap = (a: Box, b: Box): number => {
  let s = 0;
  for (let k = 0; k < 3; k++) { const g = Math.max(a.min[k]! - b.max[k]!, b.min[k]! - a.max[k]!, 0); s += g * g; }
  return Math.sqrt(s);
};
const components = (boxes: Box[], tol: number): number[] => {
  const comp = boxes.map((_, i) => i);
  const find = (i: number): number => (comp[i] === i ? i : (comp[i] = find(comp[i]!)));
  for (let i = 0; i < boxes.length; i++) for (let j = i + 1; j < boxes.length; j++) if (gap(boxes[i]!, boxes[j]!) <= tol) comp[find(i)] = find(j);
  return boxes.map((_, i) => find(i));
};
const restBoxes = list.map((m) => boxOf(m.world));
const restComp = components(restBoxes, 0.003);
const nRest = new Set(restComp).size;
console.log(`${slug}: ${solids.length} solids, rest components ${nRest}`);
const named = process.argv.slice(3);
const poses = named.length > 0 ? named : asset.poses.map((p: any) => p.id);
for (const id of poses) {
  const state = evaluate(asset, { kind: "pose", pose: id }, { kind: "scale", scale: { denominator: 50 } });
  const now: Record<string, any> = {};
  for (const [b, e] of Object.entries<any>(state.bones)) now[b] = e.frame;
  const arr = (v: any) => (Array.isArray(v) ? v : [v.x, v.y, v.z]);
  const fr = (f: any) => ({ o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) });
  const boxes = list.map((m, i) => {
    const s = solids[i];
    const w = new Float64Array(m.world.length);
    for (let k = 0; k < w.length; k += 3) {
      const p = carry(rest[s.bone] as any, fr(now[s.bone]) as any, [m.world[k]!, m.world[k + 1]!, m.world[k + 2]!]);
      w[k] = p[0]; w[k + 1] = p[1]; w[k + 2] = p[2];
    }
    return boxOf(w);
  });
  const comp = components(boxes, 0.003);
  const n = new Set(comp).size;
  const moved: string[] = [];
  for (let i = 0; i < boxes.length; i++) for (let j = i + 1; j < boxes.length; j++) {
    const g0 = gap(restBoxes[i]!, restBoxes[j]!), g1 = gap(boxes[i]!, boxes[j]!);
    if (g0 <= 0.003 && g1 > 0.006) moved.push(`${solids[i].id}|${solids[j].id}:${g1.toFixed(3)}`);
  }
  const all = { min: [1e9, 1e9, 1e9], max: [-1e9, -1e9, -1e9] };
  for (const b of boxes) for (let k = 0; k < 3; k++) { all.min[k] = Math.min(all.min[k]!, b.min[k]!); all.max[k] = Math.max(all.max[k]!, b.max[k]!); }
  console.log(` pose ${id}: components ${n} (rest ${nRest}) bbox ${all.max.map((x, k) => (x - all.min[k]!).toFixed(3)).join("x")} separated-pairs ${moved.length}${moved.length ? " " + moved.slice(0, 12).join(" ") : ""}`);
}
