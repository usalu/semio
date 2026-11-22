import { loadDoc, evaluate } from "./lib.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const c = a.clips.find((x: any) => x.id === "uphill-walk");
const i = 30; const k = c.keys[i];
const s = evaluate(a, { kind: "clip", clip: c.id, timeS: k.t });
for (const b of ["arm-r", "forearm-r", "pole-r", "torso", "thigh-r", "shank-r"]) console.log(b, JSON.stringify(k.bones[b]), JSON.stringify(s.bones[b].offset));
