import fs from "node:fs";
const T = process.env.T;
const file = "🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts";
const see = (list) => list.map(([p, d]) => ` * @see ${p} — ${d}`).join("\n");
const doc = (emoji, text, refs = []) => `/** ${emoji} ${text}\n *\n${see([...refs, ["../🟦️.ts", "the native grammar run that calls these compilations"]])}\n */`;
const spec = {
  file,
  label: "native-chart-grammar",
  childRoot: "🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar",
  deleteParent: false,
  children: [
    { folder: "🧰️grammar-kit", doc: doc("🧰️", "Shared kit of the native grammar compilations: the equality check every compilation uses on records it compares."), stmts: ["385"] },
    { folder: "🧱️basics", doc: doc("🧱️", "Native grammar compilations, basics: unknown and missing values, directed graphs, font metrics, legends, spatial data, geo palettes, nice scales and the vector paint, base and flow admissions.", [["../🧰️grammar-kit/🟦️.ts", "the equality check"]]), stmts: ["345-384", "390-739", "1954-2003"] },
    { folder: "🎛️family-and-scientific", doc: doc("🎛️", "Native grammar compilations, family customization and scientific controls: the family customization and its pdf shapes, the scientific defaults, control effects, canvas and mixed controls, mode defaults and the admissions of records, relations, scalars, modes and specialized placement.", [["../🧰️grammar-kit/🟦️.ts", "the equality check"]]), stmts: ["742-1175", "1662-1803"] },
    { folder: "🗺️geo-planar", doc: doc("🗺️", "Native grammar compilations, geo planar: the planar geo grammar with its admission, guard and pdf paths, and the contour boundary.", [["../🧰️grammar-kit/🟦️.ts", "the equality check"]]), stmts: ["1179-1309"] },
    { folder: "🏭️domain-effects", doc: doc("🏭️", "Native grammar compilations, domain control effects: process, thermodynamic, list seating, primitive hull, timeline, density, diagram, spatial group, planning, logo radar, caption, biofabric, engineering and UpSet controls.", [["../🧰️grammar-kit/🟦️.ts", "the equality check"]]), stmts: ["1312-1660"] },
    { folder: "🧠️neural-and-domain", doc: doc("🧠️", "Native grammar compilations, neural and domain controls: neural architecture, topology, incidence and references, graph and domain references, critical paths, token publication, actual marks and the hive, self loop, genome, music, meteogram, domain scalar, physics and surface window controls.", [["../🧰️grammar-kit/🟦️.ts", "the equality check"]]), stmts: ["1805-1952", "2007-2209"] },
  ],
};
fs.writeFileSync(T + "/spec-grammar.json", JSON.stringify(spec, null, 1));
