import { entryOf, loadSource, merged, featureM, poseOf } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, project } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [spec, scaleArg, view, pat] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const state: any = evaluate(asset, { kind: "pose", pose: poseOf(asset) }, { kind: "scale", scale: { denominator: scale } });
const re = new RegExp(pat ?? ".");
const bb = (pts: any[]) => { const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y); return [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)].map((v) => +v.toFixed(3)); };
for (const s of state.shapes[view] as any[]) if (re.test(s.shape.id) || re.test(s.shape.bone ?? "")) console.log("2D", s.shape.id, s.shape.role, s.shape.bone, s.closed ? "closed" : "open", s.shape.filled, bb(s.rings.flat()));
const solids = worldSolids(asset, state.bones, scale);
for (const w of solids) if (re.test(w.id)) {
  const pts: any[] = [];
  for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
  console.log("3D", w.id, bb(pts));
}
