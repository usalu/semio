/** 🐚 Author 12: names every polyhedron of a candidate document that is not one closed, outward, simple shell (the product's own `polyhedronFault`), in seconds: `bun rework-12-shells.ts <asset document>`. */
import { readFileSync } from "node:fs";
import { polyhedronFault } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
let bad = 0;
for (const scale of ["50", "100", "200", "500"]) for (const list of ["body", "thermal", "structure"]) for (const solid of asset.volumes[scale][list]) {
  if (solid.kind !== "polyhedron") continue;
  const fault = polyhedronFault(solid.vertices, solid.triangles);
  if (fault === null) continue;
  bad++;
  process.stdout.write(`[DEBUG] 1:${scale} ${list} ${solid.id}: ${fault}\n`);
}
process.stdout.write(`${bad} faulty shells\n`);
