import { loadDoc, evaluate } from "./lib.ts";
import { stateOf } from "./ik.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const c = a.clips.find((x: any) => x.id === "uphill-walk");
for (const i of [14, 16, 18, 30, 31]) {
  const k = c.keys[i];
  const s = evaluate(a, { kind: "clip", clip: c.id, timeS: k.t });
  const p = stateOf(a, k.bones, s.contacts as string[]);
  console.log(i, k.t, "contacts", s.contacts.join(","), "root", s.root.x.toFixed(4), s.root.y.toFixed(4), "tipR clip", s.bones["pole-r"].tip.x.toFixed(4), s.bones["pole-r"].tip.y.toFixed(4), "pose", (p.bones["pole-r"].tip.x + s.root.x).toFixed(4), p.bones["pole-r"].tip.y.toFixed(4), "tipL", s.bones["pole-l"].tip.y.toFixed(4), "feet", ["heel-l","ball-l","heel-r","ball-r"].map((id) => s.anchors[id].at.y.toFixed(3)).join(" "));
}
