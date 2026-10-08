import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { vs } from "./🐍️lib.mjs";
import "./🐍️ports.mjs";
import primitive from "./🐍️brep-primitive.mjs";
import curve from "./🐍️brep-curve.mjs";
import surface from "./🐍️brep-surface.mjs";
import solid from "./🐍️brep-solid.mjs";
import boolean from "./🐍️brep-boolean.mjs";
import feature from "./🐍️brep-feature.mjs";
import transform from "./🐍️brep-transform.mjs";
import intersect from "./🐍️brep-intersect.mjs";
import evaluate from "./🐍️brep-evaluate.mjs";
import topology from "./🐍️brep-topology.mjs";
import interchange from "./🐍️brep-interchange.mjs";
import meshPrimitive from "./🐍️mesh-primitive.mjs";
import meshConvert from "./🐍️mesh-convert.mjs";
import { transform as meshTransform, component as meshComponent } from "./🐍️mesh-transform.mjs";
import { edit as meshEdit, repair as meshRepair } from "./🐍️mesh-edit.mjs";
import { inspect as meshInspect, interchange as meshInterchange, shading as meshShading, uv as meshUv } from "./🐍️mesh-other.mjs";
import { measure, check } from "./🐍️analysis.mjs";
import { values, arithmetic, vector, list } from "./🐍️math.mjs";

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..", "..", "..", "..", "..", "..", "..", "..");
const target = join(root, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue");
const generated = join(here, "..", "🗑️generated", "lane-c");

const categories = [primitive, curve, surface, solid, boolean, feature, transform, intersect, evaluate, topology, interchange, meshPrimitive, meshConvert, meshTransform, meshComponent, meshEdit, meshRepair, meshInspect, meshInterchange, meshShading, meshUv, measure, check, values, arithmetic, vector, list];

const seen = new Set();
const problems = [];
const mapping = [];
let total = 0;
for (const file of categories) {
  const emojis = new Set();
  for (const { kind, was, ren, note } of file.kinds) {
    total++;
    if (seen.has(kind.id)) problems.push(`duplicate id ${kind.id}`);
    seen.add(kind.id);
    if (emojis.has(kind.emoji)) problems.push(`duplicate emoji ${kind.emoji} in ${file.category.id} (${kind.id})`);
    emojis.add(kind.emoji);
    mapping.push({ was: was ?? null, id: kind.id, category: file.category.id, ren, note });
    const names = new Set();
    for (const port of kind.inputs) { if (names.has(port.name)) problems.push(`duplicate input ${kind.id}.${port.name}`); names.add(port.name); }
    const outs = new Set();
    for (const port of kind.outputs) { if (outs.has(port.name)) problems.push(`duplicate output ${kind.id}.${port.name}`); outs.add(port.name); }
  }
}
if (problems.length) { console.error(problems.join("\n")); process.exit(1); }

mkdirSync(target, { recursive: true });
mkdirSync(generated, { recursive: true });
const files = [];
for (const file of categories.slice().sort((a, b) => a.category.order - b.category.order)) {
  const slug = file.category.id.replace(".", "-");
  const body = { category: { id: file.category.id, emoji: vs(file.category.emoji), order: file.category.order, label: file.category.label, description: file.category.description }, kinds: file.kinds.map((entry) => entry.kind) };
  writeFileSync(join(target, `🔣️${slug}.json`), `${JSON.stringify(body, null, 2)}\n`);
  files.push({ slug, id: file.category.id, kinds: body.kinds.length });
}
writeFileSync(join(generated, "mapping.json"), `${JSON.stringify(mapping, null, 2)}\n`);
writeFileSync(join(generated, "files.json"), `${JSON.stringify(files, null, 2)}\n`);
console.log(`wrote ${files.length} category files, ${total} kinds`);
