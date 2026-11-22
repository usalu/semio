import { loadHuman } from "./kin.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const { asset } = await loadHuman("traveller-with-rolling-suitcase");
for (const id of ["pulling-suitcase", "passing-right", "contact-left"]) { const s = evaluate(asset, { kind: "pose", pose: id }); console.log("[DEBUG]", id, JSON.stringify(s.root), "patch", s.anchors["case-wheel-patch"].at.y.toFixed(3), "heel-r", s.anchors["heel-r"].at.y.toFixed(3), "ball-r", s.anchors["ball-r"].at.y.toFixed(3), "grip", s.anchors["grip-l"].at.y.toFixed(3), s.anchors["case-handle"].at.y.toFixed(3), JSON.stringify(asset.poses.find((p: any) => p.id === id).bones.suitcase)); }
