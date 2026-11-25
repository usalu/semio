/** 🔺️ Unites recorded union inputs (`[{name, polygons}]`, also the `cases` and `slivers` of the polygon-union fixture) with the module of `ROOT` and compares area and ring count with polygon-clipping.
 * usage: ROOT=<mirror> bun union-inputs.ts <inputs.json> [...]
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { REAL, mod } from "./lib.ts";

const polygonClipping = (await import(join(REAL, "../../node_modules/polygon-clipping/dist/polygon-clipping.esm.js"))).default;
const { unionPolygons, ringArea } = await mod("🔨️modules/🔺️polygon/🟦️.ts");
let [built, thrown, worst] = [0, 0, 0];
for (const file of process.argv.slice(2)) {
  const document = JSON.parse(readFileSync(file, "utf8"));
  const cases = Array.isArray(document) ? document : [...(document.cases ?? []), ...(document.slivers ?? [])];
  for (const c of cases) {
    const reference = polygonClipping.union(...c.polygons.map((ring: number[][]) => [[...ring, ring[0]]]));
    const expected = reference.reduce((sum: number, polygon: number[][][]) => sum + polygon.reduce((inner, ring, k) => inner + (k === 0 ? 1 : -1) * Math.abs(ringArea(ring.slice(0, -1).map(([x, y]) => ({ x, y })))), 0), 0);
    const started = performance.now();
    try {
      const union = unionPolygons(c.polygons.map((ring: number[][]) => ring.map(([x, y]) => ({ x, y }))));
      const area = union.reduce((sum: number, ring: any) => sum + ringArea(ring), 0);
      const relative = expected > 0 ? Math.abs(area - expected) / expected : Math.abs(area);
      built++;
      worst = Math.max(worst, relative);
      if (relative > 1e-6 || process.env.VERBOSE) console.log(`${relative > 1e-6 ? "AREA" : "ok"} ${c.name.slice(0, 90)}: ${union.length} ring(s) [${union.map((ring: any) => ring.length).join(",")}], area ${area} against ${expected} (relative ${relative.toExponential(2)}), reference rings ${reference.reduce((n: number, polygon: any) => n + polygon.length, 0)}, ${(performance.now() - started).toFixed(0)} ms`);
    } catch (error: any) {
      thrown++;
      console.log(`THROWS ${c.name.slice(0, 90)}: ${error.message.slice(0, 120)}`);
    }
  }
}
console.log(`${built} built, ${thrown} thrown, largest relative area difference ${worst.toExponential(2)}`);
