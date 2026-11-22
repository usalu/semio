/** 🔎️ Extents of named solids (world, default pose) against the drawn shapes of the same id per view: bun part.ts <slug> <scale> <id,...> */
import { join } from "node:path";
import { S, CATALOG } from "./lib.ts";
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { worldSolids, extentOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const [slug, scale, ids] = process.argv.slice(2);
const a = (await Bun.file(process.env.FILE ?? join(CATALOG, "👤️humans", slug!, "🔣️.json")).json()).asset;
const st = evaluate(a, { kind: "pose", pose: process.env.POSE ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
const ws = worldSolids(a, st.bones, Number(scale));
const f = (x: number) => x.toFixed(3);
for (const id of ids!.split(",")) {
  const w = ws.filter((s: any) => s.id === id || s.id.startsWith(id + "#"));
  if (w.length) { const e = extentOf(w); console.log(id, "solid x", f(e.min[0]), f(e.max[0]), "y", f(e.min[1]), f(e.max[1]), "z", f(e.min[2]), f(e.max[2])); }
  for (const v of ["elevation", "front", "plan"]) for (const sh of st.shapes[v].filter((x: any) => x.shape.id === id)) { const p = sh.rings.flat(); console.log("  ", v, "x", f(Math.min(...p.map((q: any) => q.x))), f(Math.max(...p.map((q: any) => q.x))), "y", f(Math.min(...p.map((q: any) => q.y))), f(Math.max(...p.map((q: any) => q.y)))); }
}
