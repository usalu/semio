import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { meshIntegrity } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const asset = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
const ev: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const ws = worldSolids(asset, ev.bones, 50, "body");
let neg = 0, pos = 0;
for (const w of ws) { const m = meshIntegrity(w.positions, w.indices); if (!m.closed) continue; if (m.signedVolume < 0) neg++; else pos++; }
const dets = new Set(ws.map((w) => { const f: any = w.frame; return Math.sign(f.u.x * (f.v.y * f.w.z - f.v.z * f.w.y) - f.u.y * (f.v.x * f.w.z - f.v.z * f.w.x) + f.u.z * (f.v.x * f.w.y - f.v.y * f.w.x)); }));
console.log("closed solids with negative signed volume in the world:", neg, "positive:", pos, "frame det signs", [...dets]);
