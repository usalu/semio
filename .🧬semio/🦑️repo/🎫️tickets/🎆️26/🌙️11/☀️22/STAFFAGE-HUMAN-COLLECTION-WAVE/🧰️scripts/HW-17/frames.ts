/** 🦴️ Prints the world frames (origin, tip, u, v, w) of every bone of an asset file at rest and in the given poses, as JSON, for the generators.
 *  bun frames.ts <asset.json> [pose | clip@seconds ...] */
import { evaluate, rigBounds } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const [file, ...poses] = process.argv.slice(2);
const doc = await Bun.file(file!).json();
const asset = doc.asset ?? doc;
const r = (n: number): number => Math.round(n * 1e5) / 1e5;
const v = (a: readonly number[] | { readonly x: number; readonly y: number; readonly z: number }): number[] => (Array.isArray(a) ? a : [(a as { x: number }).x, (a as { y: number }).y, (a as { z: number }).z]).map(r);
const rest = Object.fromEntries(Object.entries(restFrames(asset.rig)).map(([id, f]) => [id, { o: v(f.frame.o), u: v(f.frame.u), v: v(f.frame.v), w: v(f.frame.w) }]));
const out: Record<string, unknown> = { rest };
for (const pose of poses) {
  const [clip, at] = pose.split("@");
  const state = at === undefined ? evaluate(asset, { kind: "pose", pose }) : evaluate(asset, { kind: "clip", clip: clip!, timeS: Number(at) });
  out[pose] = {
    bounds: state.bounds, rigBounds: rigBounds(state),
    bones: Object.fromEntries(Object.entries(state.bones).map(([id, b]) => [id, { o: v(b.frame.o), tip: v(b.tip), u: v(b.frame.u), v: v(b.frame.v), w: v(b.frame.w) }])),
    anchors: Object.fromEntries(Object.entries(state.anchors).map(([id, a]) => [id, v([a.at.x, a.at.y, a.z])])),
  };
}
console.log(JSON.stringify(out));
