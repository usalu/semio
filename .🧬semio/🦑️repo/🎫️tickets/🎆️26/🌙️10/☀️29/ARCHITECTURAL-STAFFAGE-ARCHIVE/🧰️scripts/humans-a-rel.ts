// [DEBUG] anchor world x minus root x over time
import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, id, clipId, anchor, t0, n] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).assets.find((a: any) => a.id === id);
let prev = 0;
for (let k = 0; k < Number(n ?? 12); k++) { const t = Number(t0) + k / 240; const st = evaluate(asset, { kind: "clip", clip: clipId!, timeS: t }); const a = st.anchors[anchor!].at; const rx = st.bones[asset.rig.root].origin.x; const rel = a.x - rx; console.log(t.toFixed(4), "rel", rel.toFixed(5), "d", (rel - prev).toFixed(5), "ay", a.y.toFixed(4), "rootx", rx.toFixed(5)); prev = rel; }
