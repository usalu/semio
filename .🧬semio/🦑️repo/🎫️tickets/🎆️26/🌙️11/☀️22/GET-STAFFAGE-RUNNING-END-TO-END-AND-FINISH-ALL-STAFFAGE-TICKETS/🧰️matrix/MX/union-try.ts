/** 🧪️ Candidate cures of the prism unions that throw: `bun union-try.ts <union-repro.json> <scratch dir>` joins the nearly touching vertices of the filed polygons with growing tolerances (0.3, 1, 3 and 10 micrometres, the last being the distance below which the union already counts two boundaries as one) and reports for each whether the edge-splitting or the slab union closes with a coherent area, and how far the result lies from the largest input polygon. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const [input, scratch] = process.argv.slice(2);
mkdirSync(scratch!, { recursive: true });
const source = readFileSync(join(S, "🔨️modules", "🔺️polygon", "🟦️.ts"), "utf8")
  .replace('"../🔗️contracts/🟦️.ts"', JSON.stringify(join(S, "🔨️modules", "🔗️contracts", "🟦️.ts")))
  .replace('"../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts"', JSON.stringify(join(S, "../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts")))
  .replace(/^function (primaryUnion|slabUnion|chainPieces)/gmu, "export function $1")
  .replace("function snapped(polygons: readonly (readonly Point[])[]): Point[][] {", "export function snapped(polygons: readonly (readonly Point[])[], tolerance = SAME_M): Point[][] {")
  .replace("let nearest = SAME_M;", "let nearest = tolerance;");
writeFileSync(join(scratch!, "polygon-try.ts"), source);
const open = await import(join(scratch!, "polygon-try.ts"));
type P = { x: number; y: number };
const area = (ring: P[]): number => ring.reduce((sum, p, i) => sum + (p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y), 0) / 2;
const total = (rings: P[][]): number => rings.reduce((sum, ring) => sum + area(ring), 0);
for (const failing of JSON.parse(readFileSync(input!, "utf8")) as { asset: string; subject: string; polygons: number[][][] }[]) {
  const polygons: P[][] = failing.polygons.map((ring) => ring.map(([x, y]) => ({ x: x!, y: y! })));
  const largest = Math.max(...polygons.map(area));
  const upper = total(polygons);
  console.log(`[DEBUG] ${failing.asset} ${failing.subject}: largest ${largest.toExponential(6)} sum ${upper.toExponential(6)}`);
  const perimeter = (ring: P[]): number => ring.reduce((sum, p, i) => sum + Math.hypot(p.x - ring[(i + 1) % ring.length]!.x, p.y - ring[(i + 1) % ring.length]!.y), 0);
  const broad = polygons.filter((ring) => (2 * Math.abs(area(ring))) / perimeter(ring) >= 1e-5);
  console.log(`[DEBUG]   mean widths ${polygons.map((ring) => ((2 * Math.abs(area(ring))) / perimeter(ring)).toExponential(2)).join(" ")}; ${polygons.length - broad.length} thinner than 10 micrometres`);
  for (const [name, candidate] of [["all", polygons], ["without slivers", broad]] as const)
  for (const tolerance of [0, 3e-7, 1e-6, 3e-6, 1e-5]) {
    console.log(`[DEBUG]   ${name}:`);
    const near: P[][] = tolerance === 0 ? candidate : open.snapped(candidate, tolerance);
    const joined: P[][] = open.primaryUnion(near);
    const exact = open.slabUnion(near) as { rings: P[][]; area: number };
    const coherent = (rings: P[][]): boolean => rings.length > 0 && total(rings) >= largest * (1 - 1e-6) - 1e-12 && total(rings) <= upper * (1 + 1e-6) + 1e-12;
    console.log(`[DEBUG]   join ${tolerance}: ${near.length} polygons; edge-splitting ${joined.length} rings area ${total(joined).toExponential(6)} ${coherent(joined) ? "coherent" : "no"}; slab ${exact.rings.length} rings area ${total(exact.rings).toExponential(6)} exact ${exact.area.toExponential(6)} ${exact.rings.length > 0 && Math.abs(total(exact.rings) - exact.area) <= 1e-4 * exact.area + 1e-9 ? "closes" : "no"}`);
  }
}
