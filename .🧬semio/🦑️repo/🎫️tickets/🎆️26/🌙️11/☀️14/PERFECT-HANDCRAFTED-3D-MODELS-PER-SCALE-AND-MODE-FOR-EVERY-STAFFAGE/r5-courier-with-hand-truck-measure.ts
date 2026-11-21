/** 📏 Round 5 measurements of a human staffage asset against its reference table: joint heights and segment lengths of the rig in the rest pose, and the extents of the body of one scale (catalog with the author source merged) in horizontal slices (breadth, depth, girth by the polygon of the slice) and per solid group. `bun r5-courier-with-hand-truck-measure.ts humans/<slug> <scale> <json of {label: [solid ids, slice height | null]}>` prints one line per entry.
 * @see ./r5-brief.md */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, loadSource, localGeometry, merged, qualityOfAsset, resolveList } from "./volumes-lib.ts";

const [spec, scaleText, groupsText] = process.argv.slice(2) as [string, string, string];
const { entry, asset: catalog } = entryOf(spec);
const asset: any = merged(catalog, loadSource(entry) ?? {});
const state: any = evaluate({ ...asset, poses: [{ ...asset.poses[0], id: "rest", bones: {}, ground: "authored" }] } as never, { kind: "pose", pose: "rest" } as never);
const r3 = (v: number): string => v.toFixed(3);
for (const b of asset.rig.bones) {
  const f = state.bones[b.id].frame;
  console.log(`bone ${b.id}: joint y ${r3(f.o.y)} x ${r3(f.o.x)} z ${r3(f.o.z)} length ${r3(b.lengthM)}`);
}
const q = qualityOfAsset(asset);
const body: any[] = resolveList(asset.volumes[scaleText], "body");
const points = (ids: string[]): [number, number, number][][] => {
  const out: [number, number, number][][] = [];
  for (const s of body.filter((x) => ids.includes(x.id))) {
    const g = localGeometry(s, q);
    const f = state.bones[s.bone].frame;
    const w = (i: number, k: "x" | "y" | "z"): number => f.o[k] + g.positions[3 * i]! * f.u[k] + g.positions[3 * i + 1]! * f.v[k] + g.positions[3 * i + 2]! * f.w[k];
    for (let i = 0; i < g.indices.length; i += 3) out.push([0, 1, 2].map((j) => { const n = g.indices[i + j]!; return [w(n, "x"), w(n, "y"), w(n, "z")] as [number, number, number]; }));
  }
  return out;
};
for (const [label, [ids, at]] of Object.entries<[string[], number | null]>(JSON.parse(groupsText))) {
  const tris = points(ids);
  const all = tris.flat();
  const ext = (k: 0 | 1 | 2): [number, number] => [Math.min(...all.map((p) => p[k])), Math.max(...all.map((p) => p[k]))];
  if (at === null) {
    const [x, y, z] = [ext(0), ext(1), ext(2)];
    console.log(`${label}: x ${r3(x[0])}..${r3(x[1])} y ${r3(y[0])}..${r3(y[1])} z ${r3(z[0])}..${r3(z[1])}`);
    continue;
  }
  const cut: [number, number][] = [];
  for (const t of tris)
    for (let j = 0; j < 3; j++) {
      const [a, b] = [t[j]!, t[(j + 1) % 3]!];
      if ((a[1] - at) * (b[1] - at) < 0) {
        const s = (at - a[1]) / (b[1] - a[1]);
        cut.push([a[0] + s * (b[0] - a[0]), a[2] + s * (b[2] - a[2])]);
      }
    }
  const cx = cut.reduce((n, p) => n + p[0], 0) / cut.length;
  const cz = cut.reduce((n, p) => n + p[1], 0) / cut.length;
  const ring = [...cut].sort((p, r) => Math.atan2(p[1] - cz, p[0] - cx) - Math.atan2(r[1] - cz, r[0] - cx));
  const girth = ring.reduce((n, p, i) => n + Math.hypot(p[0] - ring[(i + 1) % ring.length]![0], p[1] - ring[(i + 1) % ring.length]![1]), 0);
  const xs = cut.map((p) => p[0]);
  const zs = cut.map((p) => p[1]);
  console.log(`${label} at y ${at}: depth ${r3(Math.max(...xs) - Math.min(...xs))} breadth ${r3(Math.max(...zs) - Math.min(...zs))} girth ${r3(girth)}`);
}
