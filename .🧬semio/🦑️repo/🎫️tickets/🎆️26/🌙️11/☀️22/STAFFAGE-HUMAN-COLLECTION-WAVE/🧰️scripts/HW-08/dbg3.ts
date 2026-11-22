import { loadDoc, evaluate } from "./lib.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const c = a.clips.find((x: any) => x.id === "uphill-walk");
for (let i = 0; i <= 36; i++) { const k = c.keys[i]; const s = evaluate(a, { kind: "clip", clip: c.id, timeS: Math.min(k.t, c.durationS - 1e-4) });
  console.log(i, k.t.toFixed(3), s.contacts.join(","), "root", s.root.x.toFixed(3), "L", s.bones["pole-l"].tip.x.toFixed(3), s.bones["pole-l"].tip.y.toFixed(3), "R", s.bones["pole-r"].tip.x.toFixed(3), s.bones["pole-r"].tip.y.toFixed(3), "angL", k.bones["pole-l"].angleDeg, k.bones["arm-l"].angleDeg, k.bones["forearm-l"].angleDeg); }
