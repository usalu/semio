import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, pose, scaleArg = "50"] = process.argv;
const asset: any = (await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.find((a: any) => a.id === id)!;
const s: any = evaluate(asset, { kind: "pose", pose } as any, { kind: "scale", scale: { denominator: Number(scaleArg) } } as any);
const f = (n: number) => n.toFixed(3);
for (const v of ["elevation", "front", "plan"] as const) for (const e of s.shapes[v]) {
  if (e.shape.role === "ground") continue;
  const pts = e.rings.flat();
  if (!pts.length) continue;
  console.log("[DEBUG]", v, e.shape.id, e.shape.bone, "x", f(Math.min(...pts.map((p: any) => p.x))), f(Math.max(...pts.map((p: any) => p.x))), "y", f(Math.min(...pts.map((p: any) => p.y))), f(Math.max(...pts.map((p: any) => p.y))));
}

for (const b of (process.env.QA5_BONES ?? "hand-l,flag-pole,flag").split(",")) console.log("[DEBUG] bone", b, "o", f(s.bones[b].origin.x), f(s.bones[b].origin.y), "t", f(s.bones[b].tip.x), f(s.bones[b].tip.y));
