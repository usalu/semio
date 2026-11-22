import { loadDoc, solidify, measureAsset, featureOf, validateAsset } from "./lib.ts";
const a = loadDoc("standing-adult").asset;
const before = Object.fromEntries([50, 500].map((s) => [s, measureAsset(a, s, undefined, undefined, featureOf(s as never))]));
const n = solidify(a, new Set(["torso", "neck", "head", "thigh-l", "thigh-r"]), undefined, ["elevation", "plan"]);
console.log("solidified", n);
for (const s of [50, 500]) { const m: any = measureAsset(a, s, undefined, undefined, featureOf(s as never)); console.log(s, ["elevation", "front", "plan"].map((v) => `${(before[s] as any).views[v].iou.toFixed(3)}->${m.views[v].iou.toFixed(3)} ${((before[s] as any).views[v].hausdorffM * 1000).toFixed(0)}->${(m.views[v].hausdorffM * 1000).toFixed(0)}`).join(" | ")); }
const is = validateAsset(a); console.log(is.length, is.slice(0, 10).map((i: any) => i.code + " " + i.message.en.slice(0, 200)));
