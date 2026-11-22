/** 🧭️ Shared helpers of HW-10: the library barrel, loading an asset document and evaluating bone frames and anchors. */
export * from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { evaluate, rigBounds, type Asset, type EvaluationQuery } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

/** 📂️ Reads `{ asset }` from an asset document path. */
export const loadAsset = async (path: string): Promise<Asset> => (await Bun.file(path).json()).asset as Asset;

/** 🧾️ The query for a pose id or `clip@time`. */
export const queryOf = (spec: string): EvaluationQuery => (spec.includes("@") ? { kind: "clip", clip: spec.split("@")[0]!, timeS: Number(spec.split("@")[1]) } : { kind: "pose", pose: spec });

/** 🦴️ Frames, tips and anchors of one evaluated instant, rounded to 0.1 mm. */
export function framesOf(asset: Asset, spec: string) {
  const state = evaluate(asset, queryOf(spec));
  const r = (v: { x: number; y: number; z: number }) => [v.x, v.y, v.z].map((n) => Math.round(n * 1e4) / 1e4);
  const bones = Object.fromEntries(Object.values(state.bones).map((b) => [b.id, { o: r(b.frame.o), u: r(b.frame.u), v: r(b.frame.v), w: r(b.frame.w), tip: r(b.tip) }]));
  const anchors = Object.fromEntries(Object.entries(state.anchors).map(([id, a]) => [id, [a.at.x, a.at.y, a.z].map((n) => Math.round(n * 1e4) / 1e4)]));
  return { bones, anchors, root: state.root, rigBounds: rigBounds(state), contacts: state.contacts };
}
