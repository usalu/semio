/** 🎧️ Head-local lateral profile of the solids on one bone per scale: for every u bin the largest |w| of the vertices within a v slab, as JSON. bun headprofile.ts <asset.json> <bone> <v0> <v1> [step] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, bone, a, b, s] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
const f = (state.bones as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>)[bone!]!.frame;
const g = (x: unknown): number[] => (Array.isArray(x) ? (x as number[]) : [(x as { x: number }).x, (x as { y: number }).y, (x as { z: number }).z]);
const [o, u, v, w] = [g(f.o), g(f.u), g(f.v), g(f.w)];
const step = Number(s ?? 0.005);
const out: Record<string, Record<string, number>> = {};
for (const scale of [50, 100, 200, 500]) {
  const bins: Record<string, number> = {};
  for (const solid of worldSolids(asset, state.bones as never, scale)) {
    if (solid.bone !== bone) continue;
    const p = solid.positions;
    for (let i = 0; i < p.length; i += 3) {
      const d = [p[i]! - o[0]!, p[i + 1]! - o[1]!, p[i + 2]! - o[2]!];
      const lu = d[0]! * u[0]! + d[1]! * u[1]! + d[2]! * u[2]!, lv = d[0]! * v[0]! + d[1]! * v[1]! + d[2]! * v[2]!, lw = d[0]! * w[0]! + d[1]! * w[1]! + d[2]! * w[2]!;
      if (lv < Number(a) || lv > Number(b)) continue;
      const k = (Math.round(lu / step) * step).toFixed(3);
      bins[k] = Math.max(bins[k] ?? 0, Math.abs(lw));
    }
  }
  out[scale] = bins;
}
console.log(JSON.stringify(out));
