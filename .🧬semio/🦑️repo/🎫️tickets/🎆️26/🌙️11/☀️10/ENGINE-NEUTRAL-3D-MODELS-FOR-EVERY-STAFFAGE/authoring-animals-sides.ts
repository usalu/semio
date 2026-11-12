import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { QUALITY_FOR, frames, loadAsset, resolveAsset, toWorld } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const q = QUALITY_FOR(maxDim);
const tess = asset.volume.solids.map((s: any) => ({ s, g: tessellateSolid({ ...s, node: s.bone } as never, q) }));
const side = (bone: string, id: string): number => (/-(n|hn|fn)$/.test(bone) || /^near-|-near$|-n$/.test(id) ? -1 : /-(f|hf|ff)$/.test(bone) || /^far-|-far$|-f$/.test(id) ? 1 : 0);
const queries: any[] = [{ label: "rest", q: null }, ...asset.poses.map((p: any) => ({ label: p.id, q: { kind: "pose", pose: p.id } }))];
const rest = frames(asset);
let bad = 0;
for (const { label, q: qq } of queries) {
  let fr: (b: string) => any = (b) => rest[b];
  if (qq) { const st: any = evaluate(asset, qq, { kind: "scale", scale: { denominator: 50 } } as any); fr = (b) => { const f = st.bones[b].frame; return { o: [f.o.x, f.o.y, f.o.z], u: [f.u.x, f.u.y, f.u.z], v: [f.v.x, f.v.y, f.v.z], w: [f.w.x, f.w.y, f.w.z] }; }; }
  for (const { s, g } of tess) {
    const sd = side(s.bone, s.id);
    if (sd === 0) continue;
    const f = fr(s.bone);
    let sum = 0, n = 0;
    for (let i = 0; i < g.positions.length; i += 3) { sum += toWorld(f, [g.positions[i], g.positions[i + 1], g.positions[i + 2]])[2]!; n++; }
    if (sd * (sum / n) < -0.001) { bad++; console.log(`${label}: ${s.id} (${s.bone}) mean z ${(sum / n).toFixed(3)} on the wrong side`); }
  }
}
console.log(asset.id, "poses", asset.poses.length, "wrongSide", bad);
