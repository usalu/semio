import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..", "..", "..", "..", "..", "..", "..", "..");
const brep = readFileSync(join(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs"), "utf8");
const mesh = readFileSync(join(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs"), "utf8");
const mapping = JSON.parse(readFileSync(join(here, "..", "🗑️generated", "lane-c", "mapping.json"), "utf8"));
const newOf = new Map(mapping.filter((entry) => entry.was).map((entry) => [entry.was, entry.id]));

const nodeTable = brep.slice(brep.indexOf("const NODE_KERNEL_METHOD"), brep.indexOf("];", brep.indexOf("const NODE_KERNEL_METHOD")));
const kernelCoverage = [...nodeTable.matchAll(/\("(brep\.[A-Za-z0-9.]+)", "([a-z_]+)"\)/g)].map(([, id, method]) => ({ method, kind: newOf.get(id) }));
const definitions = mesh.slice(mesh.indexOf("let definitions"), mesh.indexOf("];", mesh.indexOf("let definitions")));
const meshCoverage = [...definitions.matchAll(/\("([A-Za-z]+)", "[^"]+", "Mesh/g)].map(([, operation]) => ({ operation, kind: newOf.get(`brep.mesh.${operation}`) }));
if (kernelCoverage.length !== 93 || meshCoverage.length !== 42 || [...kernelCoverage, ...meshCoverage].some((entry) => !entry.kind)) throw new Error("coverage extraction failed");

const text = (en, de) => ({ en, de });
const port = (name, en, de, den, dde, extra) => ({ name, label: text(en, de), description: text(den, dde), ...extra });
const option = (value, en, de) => ({ value, label: text(en, de) });
const base = {
  category: { id: "brep.demo", emoji: "🧪️", order: 1, label: text("Demo", "Demo"), description: text("Demonstration category.", "Demonstrationskategorie.") },
  kinds: [
    {
      id: "brep.demo.cut", category: "brep.demo", emoji: "✂️", label: text("Cut", "Schneiden"), description: text("Cuts edges of a shape.", "Schneidet Kanten einer Form."),
      inputs: [
        port("shape", "Shape", "Form", "The shape to cut.", "Die zu schneidende Form.", { type: "shape", shapeKinds: ["solid"] }),
        port("edges", "Edges", "Kanten", "The edges to cut.", "Die zu schneidenden Kanten.", { type: "selection", selection: { component: "edge", source: "shape", multiple: true }, default: [], minItems: 1 }),
        port("size", "Size", "Größe", "The size of the cut.", "Die Größe des Schnitts.", { type: "length", default: 1, min: 0, exclusiveMin: true, max: 10, step: 0.1 }),
        port("count", "Count", "Anzahl", "How many cuts.", "Wie viele Schnitte.", { type: "integer", default: 3, min: 1, max: 9, step: 1 }),
        port("mode", "Mode", "Modus", "Which axis to cut along.", "Entlang welcher Achse geschnitten wird.", { type: "enum", default: "x", options: [option("x", "X axis", "X-Achse"), option("y", "Y axis", "Y-Achse")] }),
        port("offset", "Offset", "Versatz", "Displacement of the cut.", "Verschiebung des Schnitts.", { type: "vector", default: [0, 0, 1] }),
        port("angle", "Angle", "Winkel", "Tilt of the cut.", "Neigung des Schnitts.", { type: "angle", default: 0.5, step: 0.017453292519943295, unit: "rad" }),
        port("points", "Points", "Punkte", "Guide points.", "Führungspunkte.", { type: "point", list: true, minItems: 2, default: [[0, 0, 0], [1, 0, 0]] }),
        port("plane", "Plane", "Ebene", "Cutting plane.", "Schnittebene.", { type: "plane", default: { origin: [0, 0, 0], normal: [0, 0, 1] } }),
        port("flag", "Flag", "Schalter", "Switches the cut on.", "Schaltet den Schnitt ein.", { type: "boolean", default: true }),
        port("note", "Note", "Notiz", "A free note.", "Eine freie Notiz.", { type: "text", default: "" }),
      ],
      outputs: [port("shape", "Shape", "Form", "The cut shape.", "Die geschnittene Form.", { type: "shape", shapeKinds: ["solid"] })],
      quality: "exact-analytic",
      interaction: {
        pick: [{ component: "shape", port: "shape" }, { component: "edge", port: "edges" }],
        gumball: [{ motion: "translate", port: "offset" }, { motion: "rotate", port: "angle", axisPort: "offset" }, { motion: "translate", port: "size", along: "normal" }],
      },
      preview: true,
    },
    {
      id: "brep.demo.copy", category: "brep.demo", emoji: "📋️", label: text("Copy", "Kopieren"), description: text("Copies a shape.", "Kopiert eine Form."),
      inputs: [port("shape", "Shape", "Form", "The shape to copy.", "Die zu kopierende Form.", { type: "shape" })],
      outputs: [port("shape", "Shape", "Form", "The copy.", "Die Kopie.", { type: "shape" })],
      quality: "exact-analytic",
      preview: true,
    },
  ],
};

const set = (path, value) => ({ path, value });
const drop = (path) => ({ path, delete: true });
const cut = ["kinds", 0];
const input = (index, ...rest) => [...cut, "inputs", index, ...rest];
const expected = (code, owner, portName) => ({ code, owner, ...(portName ? { port: portName } : {}) });
const second = [set(["category", "label", "en"], "Demo two"), set(["kinds", 0, "id"], "brep.demo.cut2"), set(["kinds", 0, "emoji"], "🔪"), set(["kinds", 1, "id"], "brep.demo.copy2"), set(["kinds", 1, "emoji"], "🧾")];

const cases = [
  { name: "valid base has no findings", patches: [], expected: [] },
  { name: "duplicate kind id", patches: [set(["kinds", 1, "id"], "brep.demo.cut")], expected: [expected("duplicate-kind-id", "brep.demo.cut")] },
  { name: "duplicate category id across files", patches: [], appendFile: second, expected: [expected("duplicate-category-id", "brep.demo")] },
  { name: "id outside the category", patches: [set(["kinds", 1, "id"], "brep.other.copy")], expected: [expected("kind-id-malformed", "brep.other.copy")] },
  { name: "kind category differs from the file", patches: [set(["kinds", 1, "category"], "mesh.demo")], expected: [expected("kind-category-mismatch", "brep.demo.copy")] },
  { name: "emoji reused inside a category", patches: [set(["kinds", 1, "emoji"], "✂️")], expected: [expected("emoji-duplicate", "brep.demo.copy")] },
  { name: "kind label without German", patches: [set([...cut, "label", "de"], "")], expected: [expected("text-missing", "brep.demo.cut")] },
  { name: "kind description copied across languages", patches: [set([...cut, "description", "de"], "Cuts edges of a shape.")], expected: [expected("text-identical", "brep.demo.cut")] },
  { name: "port description without English", patches: [set(input(2, "description", "en"), " ")], expected: [expected("text-missing", "brep.demo.cut", "size")] },
  { name: "duplicate input name", patches: [set(input(3, "name"), "size")], expected: [expected("duplicate-port-name", "brep.demo.cut", "size")] },
  { name: "output without ports", patches: [set(["kinds", 1, "outputs"], [])], expected: [expected("no-outputs", "brep.demo.copy")] },
  { name: "default below an exclusive minimum", patches: [set(input(2, "default"), 0)], expected: [expected("default-out-of-range", "brep.demo.cut", "size")] },
  { name: "default above the maximum", patches: [set(input(3, "default"), 12)], expected: [expected("default-out-of-range", "brep.demo.cut", "count")] },
  { name: "fraction as integer default", patches: [set(input(3, "default"), 2.5)], expected: [expected("default-type-mismatch", "brep.demo.cut", "count")] },
  { name: "vector default with two components", patches: [set(input(5, "default"), [0, 1])], expected: [expected("default-type-mismatch", "brep.demo.cut", "offset")] },
  { name: "list default shorter than the minimum", patches: [set(input(7, "default"), [[0, 0, 0]])], expected: [expected("default-length-out-of-range", "brep.demo.cut", "points")] },
  { name: "input without default", patches: [drop(input(6, "default"))], expected: [expected("default-missing", "brep.demo.cut", "angle")] },
  { name: "default on an output", patches: [set([...cut, "outputs", 0, "default"], 1)], expected: [expected("default-misplaced", "brep.demo.cut", "shape")] },
  { name: "enum default is not an option", patches: [set(input(4, "default"), "z")], expected: [expected("enum-default-not-option", "brep.demo.cut", "mode")] },
  { name: "enum with duplicate option values", patches: [set(input(4, "options", 1, "value"), "x")], expected: [expected("enum-options-invalid", "brep.demo.cut", "mode")] },
  { name: "bounds on a vector", patches: [set(input(5, "min"), 0)], expected: [expected("constraint-misplaced", "brep.demo.cut", "offset")] },
  { name: "zero step", patches: [set(input(2, "step"), 0)], expected: [expected("bounds-inverted", "brep.demo.cut", "size")] },
  { name: "list flag on a shape", patches: [set(input(0, "list"), true)], expected: [expected("list-misplaced", "brep.demo.cut", "shape")] },
  { name: "shape kinds on a number", patches: [set(input(2, "shapeKinds"), ["solid"])], expected: [expected("shape-kinds-misplaced", "brep.demo.cut", "size")] },
  { name: "item bounds on text", patches: [set(input(10, "minItems"), 1)], expected: [expected("item-bounds-misplaced", "brep.demo.cut", "note")] },
  { name: "selection from a missing source", patches: [set(input(1, "selection", "source"), "missing")], expected: [expected("selection-source-invalid", "brep.demo.cut", "edges")] },
  { name: "selection mode without a mode port", patches: [set(input(1, "selection", "component"), "mode")], expected: [expected("selection-mode-invalid", "brep.demo.cut", "edges"), expected("pick-component-mismatch", "brep.demo.cut", "edges")] },
  { name: "single selection without a single-item cap", patches: [set(input(1, "selection", "multiple"), false)], expected: [expected("selection-multiplicity-mismatch", "brep.demo.cut", "edges")] },
  { name: "selection on an output", patches: [set([...cut, "outputs", 0, "selection"], { component: "edge", source: "shape", multiple: true })], expected: [expected("selection-on-output", "brep.demo.cut", "shape")] },
  { name: "pick of a missing port", patches: [set([...cut, "interaction", "pick", 0, "port"], "nowhere")], expected: [expected("pick-port-missing", "brep.demo.cut", "nowhere")] },
  { name: "pick of the wrong component", patches: [set([...cut, "interaction", "pick", 1, "component"], "face")], expected: [expected("pick-component-mismatch", "brep.demo.cut", "edges")] },
  { name: "gumball on a missing port", patches: [set([...cut, "interaction", "gumball", 0, "port"], "nowhere")], expected: [expected("gumball-port-missing", "brep.demo.cut", "nowhere")] },
  { name: "gumball on an enum", patches: [set([...cut, "interaction", "gumball", 0, "port"], "mode")], expected: [expected("gumball-type-mismatch", "brep.demo.cut", "mode")] },
  { name: "scalar translate without a direction", patches: [drop([...cut, "interaction", "gumball", 2, "along"])], expected: [expected("gumball-along-invalid", "brep.demo.cut", "size")] },
  { name: "rotate with a non-vector axis", patches: [set([...cut, "interaction", "gumball", 1, "axisPort"], "size")], expected: [expected("gumball-axis-invalid", "brep.demo.cut", "angle")] },
];

const schemaRejections = [
  { name: "unknown property", patches: [set(["kinds", 0, "surprise"], true)] },
  { name: "kind id outside the namespaces", patches: [set(["kinds", 0, "id"], "solid.demo.cut")] },
  { name: "missing German label", patches: [drop(["kinds", 0, "label", "de"])] },
  { name: "unknown port type", patches: [set(["kinds", 0, "inputs", 2, "type"], "decimal")] },
  { name: "selection port without descriptor", patches: [drop(["kinds", 0, "inputs", 1, "selection"])] },
  { name: "enum port without options", patches: [drop(["kinds", 0, "inputs", 4, "options"])] },
  { name: "shape kinds on a number port", patches: [set(["kinds", 0, "inputs", 2, "shapeKinds"], ["solid"])] },
  { name: "unknown quality", patches: [set(["kinds", 0, "quality"], "perfect")] },
  { name: "empty kind list", patches: [set(["kinds"], [])] },
];

const lookups = [
  { kind: "brep.primitive.box", label: text("Box", "Quader"), quality: "exact-analytic", inputs: ["width", "depth", "height"], outputs: ["shape"], defaults: { width: 1, depth: 1, height: 1 } },
  { kind: "brep.feature.filletEdges", label: text("Fillet edges", "Kanten verrunden"), quality: "exact-analytic", inputs: ["shape", "edges", "radius"], outputs: ["shape"], defaults: { radius: 0.1 } },
  { kind: "mesh.edit.loopCut", label: text("Cut edge loops", "Kantenschleifen schneiden"), quality: "polygon-mesh", inputs: ["mesh", "edges", "cuts"], outputs: ["mesh"], defaults: { cuts: 1 } },
  { kind: "analysis.massProperties", label: text("Mass properties", "Masseneigenschaften"), quality: "exact-numerical", inputs: ["solid", "density"], outputs: ["mass", "volume", "centroid", "inertia", "principalMoments", "principalAxes"], defaults: { density: 1 } },
  { kind: "math.add", label: text("Add", "Addieren"), quality: "exact-analytic", inputs: ["a", "b"], outputs: ["result"], defaults: { a: 0, b: 0 } },
];

const fixture = {
  description: "Language-agnostic laws of the generation3d geometry widget catalogue. Rust and TypeScript run the same cases: coverage tables, lookups, semantic findings of mutated copies of base, and (TypeScript, via a third-party JSON Schema validator) schema rejections.",
  unexposedKernelMethods: ["scale", "kind", "tessellate", "dispose", "retain", "registry_len", "export_gltf"],
  kernelCoverage,
  meshCoverage,
  minimumKinds: 190,
  lookups,
  base,
  cases,
  schemaRejections,
};

const target = join(root, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/🧫️fixtures");
mkdirSync(target, { recursive: true });
writeFileSync(join(target, "🔣️.json"), `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`fixture: ${kernelCoverage.length} kernel, ${meshCoverage.length} mesh, ${cases.length} cases, ${schemaRejections.length} schema rejections`);
