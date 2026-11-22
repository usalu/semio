import { loadHuman, stateOf, pointOf } from "./kin.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const { asset } = await loadHuman("traveller-with-rolling-suitcase");
const c = asset.clips[0];
for (const i of [0, 30, 45]) {
  const k = c.keys[i];
  const s = evaluate(asset, { kind: "clip", clip: c.id, timeS: k.t });
  const p = s.anchors["case-wheel-patch"], g = s.anchors["grip-l"], h = s.anchors["case-handle"];
  console.log("[DEBUG]", i, k.t, "root", JSON.stringify(s.root), "patch", p.at.y.toFixed(3), "grip", g.at.x.toFixed(3), g.at.y.toFixed(3), g.z.toFixed(3), "handle", h.at.x.toFixed(3), h.at.y.toFixed(3), h.z.toFixed(3), "lift", JSON.stringify(s.bones["case-lift"].offset));
}
