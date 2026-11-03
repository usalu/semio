import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// usage: maxy.ts <asset> <scale> <pose>   prints the shapes that set the y extent of elevation and front
const [id, scale, pose] = process.argv.slice(2);
const asset = catalog().find((a: any) => a.id === id)!;
const st: any = evaluate(asset, (pose.startsWith("clip:") ? { kind: "clip", clip: pose.split(":")[1], timeS: Number(pose.split(":")[2]) } : { kind: "pose", pose }) as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
const grep = process.argv[5];
for (const vp of ["elevation", "front"]) {
  const rows = (st.shapes[vp] as any[]).filter((s) => s.shape.role !== "ground").map((s) => ({ id: s.shape.id, max: Math.max(...s.rings.flat().map((p: any) => p.y)), min: Math.min(...s.rings.flat().map((p: any) => p.y)) }));
  console.log(vp, "BOTTOM", [...rows].sort((a, b) => a.min - b.min).slice(0, 4).map((r) => `${r.id}:${r.min.toFixed(3)}`).join(" "));
  if (grep) console.log(vp, rows.filter((r) => r.id.includes(grep)).map((r) => `${r.id}:${r.min.toFixed(3)}..${r.max.toFixed(3)}`).join(" "));
  console.log(vp, "TOP", [...rows].sort((a, b) => b.max - a.max).slice(0, 4).map((r) => `${r.id}:${r.max.toFixed(3)}`).join(" "));
}
