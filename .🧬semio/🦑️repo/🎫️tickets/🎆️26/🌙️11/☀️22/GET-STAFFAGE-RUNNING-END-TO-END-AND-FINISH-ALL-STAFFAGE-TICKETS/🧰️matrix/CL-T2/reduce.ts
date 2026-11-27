/** 🔬️ The reduced cases behind the three kinds of findings of lane CL-T2, measured with the oracle of `🧊️model-catalog` and the kernel: a box repeated at a pitch equal to its size against one with a gap and a single box (watertight), an open revolve profile whose ends stay off the axis at the same axial position against the same profile closed as a loop (self-intersection), and a marker of one feature in diameter as ellipsoid and as block at the quality of every scale for assets of several sizes (triangles that grow). reduce.ts */
import { tessellateSolid, type Solid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { ringFault } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🔺️polygon/🟦️.ts";
import { revolveOutline } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🌀️revolved/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { solidReport } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";

const asset = (largest: number): any => ({ dimensions: { heightM: largest, widthM: largest / 2, depthM: largest / 2 } });
const base = { node: "n", material: "m" };
const box = (id: string, repeat?: { count: number; translation: [number, number, number] }): Solid => ({ ...base, id, kind: "box", size: [0.4, 0.1, 1], ...(repeat === undefined ? {} : { repeat }) }) as Solid;
for (const solid of [box("one"), box("two-touching", { count: 2, translation: [0, 0.1, 0] }), box("two-apart", { count: 2, translation: [0, 0.104, 0] }), box("six-touching", { count: 6, translation: [0, 0.1, 0] })]) {
  const report = solidReport(solid, tessellateSolid(solid, qualityOf(asset(1), 200)));
  console.log(`box ${solid.id}: ${report.triangles} triangles, ${report.boundary} boundary and ${report.overlapped} overlapped edges`);
}
const fault = (profile: [number, number][]): boolean => {
  const { points, loop } = revolveOutline(profile);
  return points.length < 2 || ringFault(points, loop, (q) => q[1] <= 1e-9);
};
const ring: [number, number][] = [[0.011, 0.027], [0.0128, 0.0275], [0.0128, 0.0305], [0.011, 0.031]];
const cup: [number, number][] = [[-0.14, 0.0365], [-0.14, 0.115], [0.043, 0.115], [0.043, 0], [0.01, 0], [0.01, 0.0365]];
for (const [name, profile] of [["ring", ring], ["cup", cup]] as const) console.log(`revolve ${name}: open ${fault(profile) ? "self-intersection" : "simple"} (outline ${JSON.stringify(revolveOutline(profile).points)}), first point repeated ${fault([...profile, profile[0]!]) ? "self-intersection" : "simple"}`);
for (const largest of [0.6, 0.9, 1.7, 2.5, 10]) {
  const row = [50, 100, 200, 500].map((scale) => {
    const r = (0.00015 * scale) / 2, quality = qualityOf(asset(largest), scale);
    const ball = tessellateSolid({ ...base, id: "ball", kind: "ellipsoid", radii: [r, r, r] } as Solid, quality).indices.length / 3;
    const block = tessellateSolid({ ...base, id: "block", kind: "box", size: [2 * r, 2 * r, 2 * r] } as Solid, quality).indices.length / 3;
    return `1:${scale} chord ${(quality.chordM * 1000).toFixed(3)} mm ellipsoid ${ball} block ${block}`;
  });
  console.log(`marker of one feature, asset of ${largest} m: ${row.join("; ")}`);
}
