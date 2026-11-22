/** 🧱️ Fast check of every polyhedron of a candidate with the kernel's own `polyhedronFault` (closed, oriented, one shell, no self-intersection), before the slow validator.
 * Usage: `repo tools exec bun -- pw10-solids.ts <asset document json>...` */
import { readFileSync } from "node:fs";
import { polyhedronFault } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

for (const file of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  let bad = 0;
  for (const [scale, v] of Object.entries(asset.volumes as Record<string, { body: { id: string; kind: string; vertices: number[][]; triangles: number[][] }[] }>))
    for (const s of v.body) if (s.kind === "polyhedron") {
      const fault = polyhedronFault(s.vertices, s.triangles);
      if (fault !== null) { bad++; console.log(`[DEBUG] ${asset.id} 1:${scale} ${s.id} ${fault}`); }
    }
  console.log(`[DEBUG] ${asset.id}: ${bad} faulty polyhedra`);
}
