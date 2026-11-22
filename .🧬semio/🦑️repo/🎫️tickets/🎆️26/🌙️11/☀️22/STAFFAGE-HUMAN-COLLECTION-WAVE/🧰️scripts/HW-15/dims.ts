/** 📏️ Drawn extents of the default pose per scale and view and the volume bounding box, the inputs of `dimensions`: bun dims.ts <slug> [pose] */
import { join } from "node:path";
import { S, CATALOG } from "./lib.ts";
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { viewExtent } = await import(join(S, "🔨️modules/🗂️catalog/🟦️.ts"));
const { worldSolids, bodyDimensions } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const slug = process.argv[2]!;
const a = (await Bun.file(process.env.FILE ?? join(CATALOG, "👤️humans", slug, "🔣️.json")).json()).asset;
const pose = process.argv[3] ?? a.presentation.defaultPose;
const f = (x: number) => x.toFixed(3);
console.log("declared w h d", a.dimensions.widthM, a.dimensions.heightM, a.dimensions.depthM);
for (const scale of [50, 100, 200, 500]) {
  const s = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
  const fr = viewExtent(s, "front"), pl = viewExtent(s, "plan");
  const vb = bodyDimensions(worldSolids(a, s.bones, scale));
  console.log(`1:${scale} elev w ${f(s.bounds.maxX - s.bounds.minX)} h ${f(s.bounds.maxY - s.bounds.minY)} [${f(s.bounds.minX)}..${f(s.bounds.maxX)}] | front d ${f(fr.horizontal)} h ${f(fr.vertical)} | plan w ${f(pl.horizontal)} d ${f(pl.vertical)} | volume ${vb.map(f).join(" ")}`);
  if (process.env.WHO) for (const v of ["elevation", "plan", "front"]) { const all = s.shapes[v].filter((x: any) => x.shape.role !== "ground").map((x: any) => ({ id: x.shape.id, b: x.shape.bone, max: Math.max(...x.rings.flat().map((p: any) => p.x)), min: Math.min(...x.rings.flat().map((p: any) => p.x)), maxy: Math.max(...x.rings.flat().map((p: any) => p.y)), miny: Math.min(...x.rings.flat().map((p: any) => p.y)) })); all.sort((p: any, q: any) => q.max - p.max); console.log("  ", v, "max x", all.slice(0, 3).map((x: any) => `${x.id}@${x.b} ${f(x.max)}`).join(", "), "| min x", [...all].sort((p: any, q: any) => p.min - q.min).slice(0, 2).map((x: any) => `${x.id} ${f(x.min)}`).join(", "), "| y", [...all].sort((p: any, q: any) => q.maxy - p.maxy).slice(0, 2).map((x: any) => `${x.id} ${f(x.maxy)}`).join(", "), [...all].sort((p: any, q: any) => p.miny - q.miny).slice(0, 2).map((x: any) => `${x.id} ${f(x.miny)}`).join(", ")); }
}
