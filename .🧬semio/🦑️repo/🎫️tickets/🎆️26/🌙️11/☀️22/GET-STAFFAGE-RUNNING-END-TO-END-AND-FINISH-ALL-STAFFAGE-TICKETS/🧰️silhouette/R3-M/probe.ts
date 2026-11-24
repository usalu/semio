/** 🔎️ The drawn shapes and the solids that cover points of a view, without measuring: bun probe.ts <id|file.json> <scale> <view> "x,y;x,y…" */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

type P = { x: number; y: number };
const [name, sc, view, points] = process.argv.slice(2) as [string, string, string, string];
const scale = Number(sc);
void featureOf;
const { asset } = await Bun.file(fileOf(name)).json();
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const worlds = worldSolids(asset, state.bones as never, scale);
const inRing = (p: P, ring: P[]): boolean => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > p.y) !== (ring[j]!.y > p.y) && p.x < ((ring[j]!.x - ring[i]!.x) * (p.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c; return c; };
for (const q of points.split(";")) {
  const [x, y] = q.split(",").map(Number) as [number, number];
  const p = { x, y };
  const d = state.shapes[view].filter((s: any) => s.closed && s.rings.filter((r: P[]) => inRing(p, r)).length % 2 === 1).map((s: any) => `${s.shape.id}[${s.shape.bone},${s.shape.role}${s.shape.filled === false ? ",outline" : ""},${s.shape.kind}${s.shape.solid ? "/" + s.shape.solid.kind : ""}]`);
  const v: string[] = [];
  for (const w of worlds) for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }); if (inRing(p, t)) { v.push(`${w.id}[${w.bone}${w.slender ? ",slender" : ""}${w.texture ? ",texture" : ""}]`); break; } }
  console.log(`(${x}, ${y}) drawn: ${d.join(" ")} | vol: ${v.join(" ")}`);
}
