/** 🕳️ Diagnostic: the see-through holes of each scale's body silhouette per view (union of the projected parts, no hole dropped), with their count, area and perimeter against the outer perimeter; holes are what the cross-scale boundary distance cannot match between scales.
 * Usage: `repo tools exec bun -- holes.ts <species>`.
 * @see ./assemble.ts */
import { lobeOutline, outline, tubePolygons, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";
import { build } from "./assemble.ts";
import { SCALES, type Part } from "./tree-kit.ts";

const sp = (await import(`./${process.argv[2]}.ts`)).species;
const { parts } = build(sp) as unknown as { parts: Record<number, Part[]> };
const perim = (r: { x: number; y: number }[]): number => r.reduce((s, p, i) => s + Math.hypot(r[(i + 1) % r.length]!.x - p.x, r[(i + 1) % r.length]!.y - p.y), 0);
const area = (r: { x: number; y: number }[]): number => r.reduce((s, p, i) => s + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;
for (const s of SCALES) for (const v of ["elevation", "front", "plan"] as View[]) {
  const polys = parts[s]!.flatMap((p) => (p.kind === "lobe" ? [lobeOutline(v, p, 72)] : tubePolygons(v, p)));
  const rings = outline(polys, 0.002, 0);
  const holes = rings.filter((r) => area(r) < 0);
  const outer = rings.filter((r) => area(r) > 0);
  console.log(`[DEBUG] 1:${s} ${v}: outer ${outer.length} perimeter ${outer.reduce((a, r) => a + perim(r), 0).toFixed(1)} m, holes ${holes.length} area ${(-holes.reduce((a, r) => a + area(r), 0)).toFixed(2)} m2 perimeter ${holes.reduce((a, r) => a + perim(r), 0).toFixed(1)} m`);
}
