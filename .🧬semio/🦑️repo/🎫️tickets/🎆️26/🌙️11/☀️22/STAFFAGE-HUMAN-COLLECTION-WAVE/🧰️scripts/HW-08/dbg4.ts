import { loadDoc, evaluate, worldSolids, measureAsset, featureOf } from "./lib.ts";
import { drawnOf, followedWidth, project, strokesOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 500 } } as any);
const drawn = drawnOf(st.shapes.elevation as any, 500, a.representations["500"].pens);
const ws = worldSolids(a, st.bones as any, 500).filter((w: any) => w.bone === "pole-r");
for (const w of ws) { console.log(w.id, w.slender, w.material); const s = strokesOf("elevation", w, drawn); console.log(JSON.stringify(s[0]?.map((p: any) => [p.x.toFixed(3), p.y.toFixed(3)]))); }
console.log(drawn.filter((d: any) => d.bone === "pole-r").map((d: any) => [d.material, d.closed, d.filled, d.rings[0].length]));
const d = drawn.filter((d: any) => d.bone === "pole-r")[0]!;
console.log(JSON.stringify(d.rings[0].map((p: any) => [p.x.toFixed(3), p.y.toFixed(3)])), d.penM);
const centre = [{ x: 0.376, y: 1.1 }, { x: 0.37, y: 0.5 }, { x: 0.36, y: 0 }];
console.log("follow", followedWidth("pole-r", centre, drawn, "pole-aluminium", 0.016));
import { centreLineOf, clusterPoints } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts";
const w = ws[0]!; const per = w.copyVertices ?? w.positions.length / 3;
const line = centreLineOf(w.positions, w.positions.subarray(0, per * 3), w.indices.subarray(0, w.indices.length));
console.log("line", line ? line.edges.length : "none");
if (line) { const c = clusterPoints(line, w.positions, 0).map((q: any) => project("elevation", q[0], q[1], q[2])); console.log(c.length, JSON.stringify(c.slice(0, 5)), JSON.stringify(c.slice(-3))); console.log("follow2", followedWidth("pole-r", c, drawn, "pole-aluminium", 0.016)); }
console.log("bone", w.bone, "mat", w.material, "copy", w.copyVertices, w.positions.length / 3, "slender", w.slender, Object.keys(w));
