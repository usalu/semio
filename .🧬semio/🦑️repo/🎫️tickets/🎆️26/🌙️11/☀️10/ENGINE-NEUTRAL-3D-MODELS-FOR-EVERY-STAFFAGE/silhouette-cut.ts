/** 🔬️ Cut faces of every solid by the declared section of a view: `bun silhouette-cut.ts <slug> <view> [--plane p] [--pose id]`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { CUT, cutLoops, pieces, project, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const view = a[1] as "elevation" | "front" | "plan";
const state = evaluate(asset, { kind: "pose", pose: opt("--pose") ?? asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as any;
const plane = Number(opt("--plane") ?? asset.representations["50"].sections?.[view]?.plane);
const c = CUT[view];
for (const whole of worldSolids(asset, state.bones, 50)) for (const [n, s] of pieces(whole).entries()) {
  let low = false, high = false;
  for (let t = 0; t < s.indices.length; t++) (s.positions[s.indices[t]! * 3 + c.axis]! < plane ? (low = true) : (high = true));
  const side = low && high ? "CUT" : (high ? "high" : "low");
  if (side !== "CUT") continue;
  const loops = cutLoops(s, c.axis, plane).map((l) => l.map((q) => project(view, q[0]!, q[1]!, q[2]!)));
  const bb = loops.flat().reduce((m, p) => ({ x0: Math.min(m.x0, p.x), x1: Math.max(m.x1, p.x), y0: Math.min(m.y0, p.y), y1: Math.max(m.y1, p.y) }), { x0: 9e9, x1: -9e9, y0: 9e9, y1: -9e9 });
  console.log(whole.id.padEnd(20), "piece", n, "loops", loops.length, `x ${bb.x0.toFixed(3)}..${bb.x1.toFixed(3)} y ${bb.y0.toFixed(3)}..${bb.y1.toFixed(3)}`);
}
