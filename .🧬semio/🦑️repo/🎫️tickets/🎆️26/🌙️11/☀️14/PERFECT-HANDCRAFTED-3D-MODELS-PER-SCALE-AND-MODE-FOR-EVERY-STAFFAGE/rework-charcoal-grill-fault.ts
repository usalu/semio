import { polyhedronFault } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/✅️validation/🪨️polyhedron/🟦️.ts";
const doc = await Bun.file(process.argv[2]!).json();
for (const [scale, v] of Object.entries<any>(doc.asset.volumes)) for (const list of ["body", "thermal"]) for (const s of v[list]) if (s.kind === "polyhedron" && s.bone === "lid") console.log("[DEBUG]", scale, list, s.id, polyhedronFault(s.vertices, s.triangles));
