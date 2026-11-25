/** 🔬️ Where the union of the filed prism polygons breaks: `bun union-diagnose.ts <union-repro.json> <scratch dir>` copies the polygon module with its internals exported into the scratch folder and reports, per failing input, the area and closure of the edge-splitting union, of the snapped one and of the slab union on the polygons as given, snapped and turned by a few angles. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const [input, scratch] = process.argv.slice(2);
mkdirSync(scratch!, { recursive: true });
const source = readFileSync(join(S, "🔨️modules", "🔺️polygon", "🟦️.ts"), "utf8")
  .replace('"../🔗️contracts/🟦️.ts"', JSON.stringify(join(S, "🔨️modules", "🔗️contracts", "🟦️.ts")))
  .replace('"../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts"', JSON.stringify(join(S, "../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts")))
  .replace(/^function (snapped|primaryUnion|slabUnion|chainPieces)/gmu, "export function $1");
writeFileSync(join(scratch!, "polygon-open.ts"), source);
const open = await import(join(scratch!, "polygon-open.ts"));
const area = (ring: { x: number; y: number }[]): number => ring.reduce((sum, p, i) => sum + (p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y), 0) / 2;
const total = (rings: { x: number; y: number }[][]): number => rings.reduce((sum, ring) => sum + area(ring), 0);
for (const failing of JSON.parse(readFileSync(input!, "utf8")) as { asset: string; subject: string; polygons: number[][][] }[]) {
  const polygons = failing.polygons.map((ring) => ring.map(([x, y]) => ({ x: x!, y: y! })));
  console.log(`[DEBUG] ${failing.asset} ${failing.subject}: ${polygons.length} polygons, areas ${polygons.map((ring) => area(ring).toExponential(3)).join(" ")}`);
  const fast = open.primaryUnion(polygons);
  const near = open.snapped(polygons);
  const joined = open.primaryUnion(near);
  console.log(`[DEBUG]   edge-splitting: ${fast.length} rings area ${total(fast).toExponential(6)}; snapped (${near.length} polygons): ${joined.length} rings area ${total(joined).toExponential(6)}`);
  for (const [name, candidate] of [["given", polygons], ["snapped", near]] as const) {
    const exact = open.slabUnion(candidate);
    console.log(`[DEBUG]   slab ${name}: ${exact.rings.length} rings area ${total(exact.rings).toExponential(6)} against exact ${exact.area.toExponential(6)}`);
    for (const angle of [0.3, 0.7, 1.1, 1.9]) {
      const [c, s] = [Math.cos(angle), Math.sin(angle)];
      const turned = open.slabUnion(candidate.map((ring: { x: number; y: number }[]) => ring.map((p) => ({ x: c * p.x - s * p.y, y: s * p.x + c * p.y }))));
      console.log(`[DEBUG]     turned ${angle}: ${turned.rings.length} rings area ${total(turned.rings).toExponential(6)} against exact ${turned.area.toExponential(6)}`);
    }
  }
  const edges = polygons.flatMap((ring, owner) => ring.map((a, i) => ({ owner, a, b: ring[(i + 1) % ring.length]! })));
  const steep = edges.filter((edge) => Math.abs(edge.b.x - edge.a.x) > 1e-12 && Math.abs(edge.b.x - edge.a.x) < 1e-7);
  console.log(`[DEBUG]   edges ${edges.length}; nearly vertical (1e-12 < |dx| < 1e-7): ${steep.map((edge) => `${edge.owner}: dx ${(edge.b.x - edge.a.x).toExponential(2)} dy ${(edge.b.y - edge.a.y).toExponential(2)}`).join("; ")}`);
}
