import { loadHuman } from "./kin.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const { asset } = await loadHuman("traveller-with-rolling-suitcase");
const FREE = new Set(["arm-l", "forearm-l", "hand-l", "case-lift", "case-slide", "suitcase", "case-wheels"]);
const c = asset.clips[0];
const bare = { ...c, id: "bare", keys: c.keys.map((k: any) => ({ ...k, bones: Object.fromEntries(Object.entries(k.bones).filter(([b]) => !FREE.has(b))) })) };
const a2 = { ...asset, clips: [...asset.clips, bare] };
for (const t of [0, 0.75, 1.125]) console.log("[DEBUG]", t, evaluate(a2, { kind: "clip", clip: "pull-walk", timeS: t }).root.y.toFixed(4), evaluate(a2, { kind: "clip", clip: "bare", timeS: t }).root.y.toFixed(4));
const p = asset.poses.find((x: any) => x.id === "pulling-suitcase");
const s = evaluate(asset, { kind: "pose", pose: "pulling-suitcase" });
console.log("[DEBUG] pose root", s.root.y.toFixed(4), "patch", s.anchors["case-wheel-patch"].at.y.toFixed(4), JSON.stringify(s.contacts));
import { stateOf } from "./kin.ts";
const k = c.keys[0];
const merged: any = { ...asset.poses.find((x: any) => x.id === k.pose).bones };
for (const [b, o] of Object.entries<any>(k.bones)) merged[b] = { ...(merged[b] ?? {}), ...o };
for (const y of [0, -0.0511]) { const st = stateOf(asset, merged, { y }); console.log("[DEBUG] probe y", y, "patch", st.anchors["case-wheel-patch"].at.y.toFixed(4), "heel-l", st.anchors["heel-l"].at.y.toFixed(4)); }
const cs = evaluate(asset, { kind: "clip", clip: "pull-walk", timeS: 0 });
console.log("[DEBUG] clip heel-l", cs.anchors["heel-l"].at.y.toFixed(4), "ball-r", cs.anchors["ball-r"].at.y.toFixed(4));
