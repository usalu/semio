/** 🔬️ Extent of the authored shapes per view and of the volume in a pose: `bun silhouette-extent.ts <slug> [--pose id] [--scale 50]`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, project, worldSolids, VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const pose = opt("--pose") ?? asset.presentation.defaultPose;
const scale = Number(opt("--scale") ?? 50);
const state = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const e = extentOf(worldSolids(asset, state.bones, scale));
console.log("dimensions", JSON.stringify(asset.dimensions));
console.log("volume x", e.min[0].toFixed(3), e.max[0].toFixed(3), "y", e.min[1].toFixed(3), e.max[1].toFixed(3), "z", e.min[2].toFixed(3), e.max[2].toFixed(3));
for (const v of VIEWPOINTS) {
  const b = { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
  const who: Record<string, string> = {};
  for (const s of state.shapes[v] as any[]) { if (!["body", "detail", "glazing"].includes(s.shape.role)) continue; for (const r of s.rings) for (const p of r) { if (p.x < b.x0) { b.x0 = p.x; who.x0 = s.shape.id; } if (p.x > b.x1) { b.x1 = p.x; who.x1 = s.shape.id; } if (p.y < b.y0) { b.y0 = p.y; who.y0 = s.shape.id; } if (p.y > b.y1) { b.y1 = p.y; who.y1 = s.shape.id; } } }
  console.log(v, "x", b.x0.toFixed(3), b.x1.toFixed(3), "y", b.y0.toFixed(3), b.y1.toFixed(3), JSON.stringify(who));
}
