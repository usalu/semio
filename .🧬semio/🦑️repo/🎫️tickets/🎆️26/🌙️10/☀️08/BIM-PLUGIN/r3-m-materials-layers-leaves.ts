#!/usr/bin/env bun
/**
 * 🧪️ Wave M slice 1 (`m-materials-layers`, binary tags 100..199): create/delete/set of materials and of the layered wall, slab and roof
 * types. `bun r3-m-materials-layers-leaves.ts` rewrites the boilerplate and fixtures through `emitLeaf`, fixes the optional `set-*` fields
 * of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`, never overwritten) and prints the mount lines.
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
type Label = { en: string; de: string };
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const ref = (entity: string, role: "target" | "identity", label: Label, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Prop => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const optional = (prop: Prop): Prop => ({ ...prop, rust: `Option<${prop.rust}>` });
const textProp = (name: string, label: Label, order: number): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const numProp = (name: string, label: Label, order: number): Prop => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const categoryProp = (order: number): Prop => ({ name: "category", rust: "MaterialCategory", schema: record("MaterialCategory"), ui: { widget: "select", role: "value", label: { en: "Category", de: "Kategorie" }, group: "value", order } });
const colorProp = (order: number): Prop => ({ name: "color", rust: "Rgb", schema: record("Rgb"), ui: { widget: "record", role: "value", label: { en: "Colour", de: "Farbe" }, group: "value", order } });
const layersProp = (order: number): Prop => ({ name: "layers", rust: "Vec<Layer>", schema: { type: "array", items: record("Layer") }, ui: { widget: "list", role: "value", label: { en: "Layers", de: "Schichten" }, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const color = (r: number, g: number, b: number) => ({ r, g, b });
const mat = (name: string, category: string, density: number, conductivity: number, specificHeat: number, rgb = color(0.7, 0.35, 0.25)) => ({ name, category, color: rgb, density, conductivity, specific_heat: specificHeat });
const layer = F.layer;
const square = [[0, 0], [6, 0], [6, 5], [0, 5]].map(([x, y]) => ({ point: F.P(x, y), bulge: 0 }));
const slab = { storey: "st-ground", slab_type: "sl-200", boundary: square, holes: [] as unknown[], offset: 0, name: "Ground slab" };
const roof = { storey: "st-first", roof_type: "rf-tile", footprint: square, shape: "Flat", overhang: 0.3, base_offset: 0, name: "Main roof" };

const library = (extra: Record<string, unknown> = {}) => ({
  ...F.scene(),
  materials: { "m-brick": F.material("Brick"), "m-wool": mat("Mineral Wool", "Insulation", 30, 0.035, 840, color(0.9, 0.85, 0.4)), "m-screed": mat("Screed", "Concrete", 2000, 1.4, 1000, color(0.6, 0.6, 0.6)) },
  slab_types: { "sl-200": { name: "Concrete 200", layers: [layer("m-screed", 0.2)] } },
  roof_types: { "rf-tile": { name: "Tiled", layers: [layer("m-wool", 0.2, "Insulation"), layer("m-brick", 0.03, "Finish")] } },
  ...extra,
});

type Family = { type: string; pascal: string; snake: string; plural: string; kebab: string; human: string; humanCap: string; humanDe: string; users: string; usersField: string; usersDe: string; emoji: [number, number, number]; tag: number; userCollection: string; sample: { id: string; layers: unknown[]; renamed: string; stack: unknown[]; unused: string; unusedLayers: unknown[] }; used: { id: string; before: () => unknown } };

const wall: Family = {
  type: "wall_type", pascal: "WallType", snake: "wall_type", plural: "wall_types", kebab: "wall-type", human: "wall type", humanCap: "Wall type", humanDe: "Wandtyp", users: "walls", usersField: "wall_type", usersDe: "Wänden", emoji: [0x1fab5, 0x1f6a7, 0x1f528], tag: 103, userCollection: "walls",
  sample: { id: "wt-timber", layers: [layer("m-wool", 0.16, "Insulation"), layer("m-brick", 0.12)], renamed: "Brick 300 (renamed)", stack: [layer("m-brick", 0.24), layer("m-wool", 0.12, "Insulation")], unused: "wt-spare", unusedLayers: [layer("m-brick", 0.2)] },
  used: { id: "wt-300", before: () => library() },
};
const slabs: Family = {
  type: "slab_type", pascal: "SlabType", snake: "slab_type", plural: "slab_types", kebab: "slab-type", human: "slab type", humanCap: "Slab type", humanDe: "Deckentyp", users: "slabs", usersField: "slab_type", usersDe: "Decken", emoji: [0x1f7eb, 0x1f7e5, 0x1f7e7], tag: 106, userCollection: "slabs",
  sample: { id: "sl-300", layers: [layer("m-wool", 0.1, "Insulation"), layer("m-screed", 0.2)], renamed: "Concrete 200 (renamed)", stack: [layer("m-screed", 0.24), layer("m-wool", 0.08, "Insulation")], unused: "sl-spare", unusedLayers: [layer("m-screed", 0.18)] },
  used: { id: "sl-200", before: () => library({ slabs: { "sl-ground": slab } }) },
};
const roofs: Family = {
  type: "roof_type", pascal: "RoofType", snake: "roof_type", plural: "roof_types", kebab: "roof-type", human: "roof type", humanCap: "Roof type", humanDe: "Dachtyp", users: "roofs", usersField: "roof_type", usersDe: "Dächern", emoji: [0x1f6d6, 0x26fa, 0x1f3d5], tag: 109, userCollection: "roofs",
  sample: { id: "rf-metal", layers: [layer("m-wool", 0.24, "Insulation"), layer("m-screed", 0.01, "Finish")], renamed: "Tiled (renamed)", stack: [layer("m-wool", 0.3, "Insulation"), layer("m-brick", 0.03, "Finish")], unused: "rf-spare", unusedLayers: [layer("m-wool", 0.1, "Insulation")] },
  used: { id: "rf-tile", before: () => library({ roofs: { "r-main": roof } }) },
};

const withSpare = (family: Family) => {
  const base = library() as Record<string, any>;
  return { ...base, [family.plural]: { ...base[family.plural], [family.sample.unused]: { name: "Spare", layers: family.sample.unusedLayers } } };
};

const family = (f: Family): Leaf[] => {
  const idLabel = { en: `${f.humanCap} id`, de: `${f.humanDe}-Id` };
  const targetLabel = { en: f.humanCap, de: f.humanDe };
  const recordName = f.snake;
  const asRecord = { name: "Glass Wool 300", layers: f.sample.layers };
  const unknownMaterial = { name: "Ghost", layers: [layer("m-ghost", 0.2)] };
  const kind = (verb: string) => `${verb}-${f.kebab}`;
  const [eCreate, eDelete, eSet] = f.emoji;
  const path = (...tail: string[]) => [recordName, "layers", ...tail];
  return [
    {
      kind: kind("create"), emoji: eCreate, variant: `Create${f.pascal}`, verb: "create", entity: f.snake.replace("_", "-"), doc: `Brings a new layered ${f.human} into the library; every layer names an existing material and has a positive thickness.`, displayName: `Create ${f.humanCap}`, binaryTag: f.tag,
      props: [ref(f.kebab, "identity", idLabel), recordProp(recordName, f.pascal, targetLabel)],
      label: { en: `format!("Create ${f.human} \\"{}\\"", self.${recordName}.name)`, de: `format!("${f.humanDe} \\"{}\\" anlegen", self.${recordName}.name)` },
      target,
      cases: [
        { name: "adds", emoji: 0x2705, before: library(), mutation: { id: f.sample.id, [recordName]: asRecord }, outcome: ok },
        { name: "duplicate", emoji: 0x1f6ab, before: library(), mutation: { id: f.used.id, [recordName]: asRecord }, outcome: reject("mutation.duplicate-id", [f.used.id]) },
        { name: "material-missing", emoji: 0x26d4, before: library(), mutation: { id: f.sample.id, [recordName]: unknownMaterial }, outcome: reject("mutation.target-missing", path("material")) },
        { name: "empty-layers", emoji: 0x1f573, before: library(), mutation: { id: f.sample.id, [recordName]: { name: "Hollow", layers: [] } }, outcome: reject("mutation.invariant", path()) },
        { name: "thin-layer", emoji: 0x1f4c9, before: library(), mutation: { id: f.sample.id, [recordName]: { name: "Flat", layers: [layer("m-brick", 0)] } }, outcome: reject("mutation.invariant", path("thickness")) },
      ],
    },
    {
      kind: kind("delete"), emoji: eDelete, variant: `Delete${f.pascal}`, verb: "delete", entity: f.snake.replace("_", "-"), doc: `Removes a ${f.human} that no ${f.users.slice(0, -1)} uses.`, displayName: `Delete ${f.humanCap}`, binaryTag: f.tag + 1,
      props: [ref(f.kebab, "target", targetLabel)],
      label: { en: `format!("Delete ${f.human} \\"{}\\"", self.id)`, de: `format!("${f.humanDe} \\"{}\\" löschen", self.id)` },
      target,
      cases: [
        { name: "removes", emoji: 0x2705, before: withSpare(f), mutation: { id: f.sample.unused }, outcome: ok },
        { name: `used-by-${f.users}`, emoji: 0x1f6ab, before: f.used.before(), mutation: { id: f.used.id }, outcome: reject("mutation.target-referenced", [f.used.id]) },
        { name: "missing", emoji: 0x26d4, before: library(), mutation: { id: "gone" }, outcome: reject("mutation.target-missing", ["gone"]) },
      ],
    },
    {
      kind: kind("set"), emoji: eSet, variant: `Set${f.pascal}`, verb: "set", entity: f.snake.replace("_", "-"), doc: `Patches exactly the provided fields of a ${f.human}; the layer stack is one field and replaces the whole stack.`, displayName: `Set ${f.humanCap}`, binaryTag: f.tag + 2,
      props: [ref(f.kebab, "target", targetLabel), optional(textProp("name", { en: "Name", de: "Name" }, 20)), optional(layersProp(30))],
      label: { en: `format!("Edit ${f.human} \\"{}\\"", self.id)`, de: `format!("${f.humanDe} \\"{}\\" ändern", self.id)` },
      target,
      cases: [
        { name: "renames", emoji: 0x2705, before: library(), mutation: { id: f.used.id, name: f.sample.renamed }, outcome: ok },
        { name: "restacks", emoji: 0x1f9f2, before: library(), mutation: { id: f.used.id, layers: f.sample.stack }, outcome: ok },
        { name: "both-fields", emoji: 0x1f9e9, before: library(), mutation: { id: f.used.id, name: f.sample.renamed, layers: f.sample.stack }, outcome: ok },
        { name: "empty-patch", emoji: 0x1f4ed, before: library(), mutation: { id: f.used.id }, outcome: reject("mutation.no-op", [f.used.id]) },
        { name: "empty-layers", emoji: 0x1f573, before: library(), mutation: { id: f.used.id, layers: [] }, outcome: reject("mutation.invariant", ["layers"]) },
        { name: "thin-layer", emoji: 0x1f4c9, before: library(), mutation: { id: f.used.id, layers: [layer("m-brick", -0.1)] }, outcome: reject("mutation.invariant", ["layers", "thickness"]) },
        { name: "material-missing", emoji: 0x1f47b, before: library(), mutation: { id: f.used.id, layers: [layer("m-ghost", 0.2)] }, outcome: reject("mutation.target-missing", ["layers", "material"]) },
        { name: "missing", emoji: 0x26d4, before: library(), mutation: { id: "gone", name: "Ghost" }, outcome: reject("mutation.target-missing", ["gone"]) },
      ],
    },
  ];
};

const pine = mat("Pine", "Wood", 500, 0.13, 1600, color(0.8, 0.6, 0.35));
const steel = mat("Steel", "Metal", 7850, 50, 470, color(0.55, 0.57, 0.6));
const column = (materialId: string) => ({ name: "Steel column", profile: { Rectangle: { width: 0.3, depth: 0.3 } }, material: materialId });
const railing = (materialId: string) => ({ storey: "st-ground", path: [F.P(0, 0), F.P(4, 0)], height: 1.0, post_spacing: 1.2, material: materialId, base_offset: 0, name: "Balustrade" });
const withMaterials = (extra: Record<string, unknown> = {}) => ({ ...F.scene(), materials: { "m-brick": F.material("Brick"), "m-spare": mat("Spare", "Other", 1000, 1, 1000), "m-steel": steel }, ...extra });

const materials: Leaf[] = [
  {
    kind: "create-material", emoji: 0x1faa8, variant: "CreateMaterial", verb: "create", entity: "material", doc: "Brings a new material into the library; physical values are finite and non-negative and every colour component lies in 0..1.", displayName: "Create Material", binaryTag: 100,
    props: [ref("material", "identity", { en: "Material id", de: "Material-Id" }), recordProp("material", "Material", { en: "Material", de: "Material" })],
    label: { en: 'format!("Create material \\"{}\\"", self.material.name)', de: 'format!("Material \\"{}\\" anlegen", self.material.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: F.scene(), mutation: { id: "m-pine", material: pine }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: F.scene(), mutation: { id: "m-brick", material: pine }, outcome: reject("mutation.duplicate-id", ["m-brick"]) },
      { name: "negative-density", emoji: 0x26d4, before: F.scene(), mutation: { id: "m-pine", material: { ...pine, density: -1 } }, outcome: reject("mutation.invariant", ["material", "density"]) },
      { name: "colour-out-of-range", emoji: 0x1f308, before: F.scene(), mutation: { id: "m-pine", material: { ...pine, color: color(1.2, 0.5, 0.5) } }, outcome: reject("mutation.invariant", ["material", "color"]) },
    ],
  },
  {
    kind: "delete-material", emoji: 0x1f5d1, variant: "DeleteMaterial", verb: "delete", entity: "material", doc: "Removes a material that no layer, profile type, railing or curtain wall uses.", displayName: "Delete Material", binaryTag: 101,
    props: [ref("material", "target", { en: "Material", de: "Material" })],
    label: { en: 'format!("Delete material \\"{}\\"", self.id)', de: 'format!("Material \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: withMaterials(), mutation: { id: "m-spare" }, outcome: ok },
      { name: "used-by-a-layer", emoji: 0x1f6ab, before: library(), mutation: { id: "m-brick" }, outcome: reject("mutation.target-referenced", ["m-brick"]) },
      { name: "used-by-a-column-type", emoji: 0x1f3db, before: withMaterials({ column_types: { "ct-steel": column("m-steel") } }), mutation: { id: "m-steel" }, outcome: reject("mutation.target-referenced", ["m-steel"]) },
      { name: "used-by-a-railing", emoji: 0x1f6e4, before: withMaterials({ railings: { "rl-1": railing("m-steel") } }), mutation: { id: "m-steel" }, outcome: reject("mutation.target-referenced", ["m-steel"]) },
      { name: "missing", emoji: 0x26d4, before: withMaterials(), mutation: { id: "m-gone" }, outcome: reject("mutation.target-missing", ["m-gone"]) },
    ],
  },
  {
    kind: "set-material", emoji: 0x1f58c, variant: "SetMaterial", verb: "set", entity: "material", doc: "Patches exactly the provided fields of a material; layouts, solids and quantities that use it follow by inference.", displayName: "Set Material", binaryTag: 102,
    props: [
      ref("material", "target", { en: "Material", de: "Material" }),
      optional(textProp("name", { en: "Name", de: "Name" }, 20)),
      optional(categoryProp(30)),
      optional(colorProp(40)),
      optional(numProp("density", { en: "Density (kg/m3)", de: "Dichte (kg/m3)" }, 50)),
      optional(numProp("conductivity", { en: "Conductivity (W/mK)", de: "Wärmeleitfähigkeit (W/mK)" }, 60)),
      optional(numProp("specific_heat", { en: "Specific heat (J/kgK)", de: "Spezifische Wärme (J/kgK)" }, 70)),
    ],
    label: { en: 'format!("Edit material \\"{}\\"", self.id)', de: 'format!("Material \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "recolours-and-renames", emoji: 0x2705, before: F.scene(), mutation: { id: "m-brick", name: "Red Brick", color: color(0.8, 0.2, 0.15) }, outcome: ok },
      { name: "retunes-the-physics", emoji: 0x1f321, before: F.scene(), mutation: { id: "m-brick", density: 1900, conductivity: 0.9, specific_heat: 880 }, outcome: ok },
      { name: "recategorises", emoji: 0x1f9f2, before: F.scene(), mutation: { id: "m-brick", category: "Finish" }, outcome: ok },
      { name: "empty-patch", emoji: 0x1f4ed, before: F.scene(), mutation: { id: "m-brick" }, outcome: reject("mutation.no-op", ["m-brick"]) },
      { name: "same-values", emoji: 0x267b, before: F.scene(), mutation: { id: "m-brick", name: "Brick", density: 1800 }, outcome: reject("mutation.no-op", ["m-brick"]) },
      { name: "negative-conductivity", emoji: 0x1f6ab, before: F.scene(), mutation: { id: "m-brick", conductivity: -0.5 }, outcome: reject("mutation.invariant", ["conductivity"]) },
      { name: "colour-out-of-range", emoji: 0x1f308, before: F.scene(), mutation: { id: "m-brick", color: color(0.5, -0.1, 0.5) }, outcome: reject("mutation.invariant", ["color"]) },
      { name: "missing", emoji: 0x26d4, before: F.scene(), mutation: { id: "m-gone", name: "Ghost" }, outcome: reject("mutation.target-missing", ["m-gone"]) },
    ],
  },
];

export const leaves: Leaf[] = [...materials, ...family(wall), ...family(slabs), ...family(roofs)];

const OPTIONALS: Record<string, string[]> = Object.fromEntries(leaves.map((leaf) => [leaf.kind, leaf.props.filter((prop) => prop.rust.startsWith("Option<")).map((prop) => prop.name)]));

function fixLeaf(leaf: Leaf) {
  const optionals = OPTIONALS[leaf.kind];
  const dir = join(mutations, em(leaf.emoji) + leaf.kind);
  const file = join(dir, em(0x1f9a0) + "mutation", RS);
  const used = new Set(leaf.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  let text = readFileSync(file, "utf8");
  text = text.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) text = text.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  writeFileSync(file, text);
  if (optionals.length > 0) {
    const schemaFile = join(dir, em(0x1f9ec) + "schema", JSONF);
    const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
    schema.required = schema.required.filter((name: string) => !optionals.includes(name));
    writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  }
}

const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const refuse = (code: string, message: string, path: string) => `return MutationOutcome::refuse(OutcomeCode::${code}, ${message}, ${path});`;

const materialHand: Record<string, { diff: string; inverse: string }> = {
  "create-material": {
    diff: `//! ${DIFF} Diff constructor for \`CreateMaterial\`: one created material entry. Density, conductivity and specific heat are finite and non-negative; every colour channel lies in 0..1.

use super::CreateMaterial;
use crate::{Entry, ModelDiff, ModelSnapshot, Rgb};
use protocol::{MutationOutcome, OutcomeCode};

fn unit(color: &Rgb) -> bool {
    [color.r, color.g, color.b].into_iter().all(|channel| channel.is_finite() && (0.0..=1.0).contains(&channel))
}

pub fn diff(payload: &CreateMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let material = &payload.material;
    if base.materials.contains_key(&payload.id) {
        ${refuse("DuplicateId", 'format!("Material \\"{}\\" already exists.", payload.id)', "[payload.id.clone()]")}
    }
    let physical = [("density", material.density), ("conductivity", material.conductivity), ("specific_heat", material.specific_heat)];
    if let Some((field, _)) = physical.into_iter().find(|(_, value)| !(value.is_finite() && *value >= 0.0)) {
        ${refuse("Invariant", 'format!("Material {field} must be a finite, non-negative number.")', '["material", field]')}
    }
    if !unit(&material.color) {
        ${refuse("Invariant", '"Every colour channel must lie between 0 and 1."', '["material", "color"]')}
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Created(material.clone())))
}
`,
    inverse: `//! ${INVERSE} Inverse of \`CreateMaterial\`: the concrete \`DeleteMaterial\` of the id it created, none when the id was already taken.

use super::super::delete_material::DeleteMaterial;
use super::CreateMaterial;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.materials.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteMaterial(DeleteMaterial { id: payload.id.clone() })]
}
`,
  },
  "delete-material": {
    diff: `//! ${DIFF} Diff constructor for \`DeleteMaterial\`: one deleted material entry. A material that any layer, profile type, railing or curtain
//! wall still names is refused as \`mutation.target-referenced\`.

use super::DeleteMaterial;
use crate::{Entry, Layer, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn layered(layers: &[Layer], id: &str) -> bool {
    layers.iter().any(|layer| layer.material == id)
}

fn user(base: &ModelSnapshot, id: &str) -> Option<&'static str> {
    [
        ("a wall type", base.wall_types.values().any(|row| layered(&row.layers, id))),
        ("a slab type", base.slab_types.values().any(|row| layered(&row.layers, id))),
        ("a roof type", base.roof_types.values().any(|row| layered(&row.layers, id))),
        ("a column type", base.column_types.values().any(|row| row.material == id)),
        ("a beam type", base.beam_types.values().any(|row| row.material == id)),
        ("a window type", base.window_types.values().any(|row| row.material == id)),
        ("a door type", base.door_types.values().any(|row| row.material == id)),
        ("a curtain wall", base.curtain_walls.values().any(|row| row.panel_material == id || row.mullion_material == id)),
        ("a railing", base.railings.values().any(|row| row.material == id)),
    ]
    .into_iter()
    .find_map(|(kind, present)| present.then_some(kind))
}

pub fn diff(payload: &DeleteMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.materials.contains_key(&payload.id) {
        ${refuse("TargetMissing", 'format!("Material \\"{}\\" does not exist.", payload.id)', "[payload.id.clone()]")}
    }
    if let Some(kind) = user(base, &payload.id) {
        ${refuse("TargetReferenced", 'format!("Material \\"{}\\" is still used by {kind}.", payload.id)', "[payload.id.clone()]")}
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Deleted))
}
`,
    inverse: `//! ${INVERSE} Inverse of \`DeleteMaterial\`: the concrete \`CreateMaterial\` carrying the full removed record, none when the material was absent.

use super::super::create_material::CreateMaterial;
use super::DeleteMaterial;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.materials.get(&payload.id) {
        Some(material) => vec![ModelMutation::CreateMaterial(CreateMaterial { id: payload.id.clone(), material: material.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-material": {
    diff: `//! ${DIFF} Diff constructor for \`SetMaterial\`: a sparse material patch of exactly the provided fields. Physical values stay finite and
//! non-negative, colour channels in 0..1; a patch that changes nothing is a no-op. Everything derived from the material follows by inference.

use super::SetMaterial;
use crate::{Entry, MaterialPatch, ModelDiff, ModelSnapshot, Patch, Rgb};
use protocol::{MutationOutcome, OutcomeCode};

pub fn patch(payload: &SetMaterial) -> MaterialPatch {
    MaterialPatch { name: payload.name.clone(), category: payload.category, color: payload.color, density: payload.density, conductivity: payload.conductivity, specific_heat: payload.specific_heat }
}

fn unit(color: &Rgb) -> bool {
    [color.r, color.g, color.b].into_iter().all(|channel| channel.is_finite() && (0.0..=1.0).contains(&channel))
}

fn fault(change: &MaterialPatch) -> Option<&'static str> {
    [("density", change.density), ("conductivity", change.conductivity), ("specific_heat", change.specific_heat)]
        .into_iter()
        .find_map(|(field, value)| value.filter(|value| !(value.is_finite() && *value >= 0.0)).map(|_| field))
        .or_else(|| change.color.filter(|color| !unit(color)).map(|_| "color"))
}

pub fn diff(payload: &SetMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(material) = base.materials.get(&payload.id) else {
        ${refuse("TargetMissing", 'format!("Material \\"{}\\" does not exist.", payload.id)', "[payload.id.clone()]")}
    };
    let change = patch(payload);
    if let Some(field) = fault(&change) {
        ${refuse("Invariant", 'format!("Material {field} is out of range.")', "[field]")}
    }
    if change.write(material) == *material {
        ${refuse("NoOp", 'format!("Material \\"{}\\" already has these values.", payload.id)', "[payload.id.clone()]")}
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: `//! ${INVERSE} Inverse of \`SetMaterial\`: an absolute \`SetMaterial\` restoring the base values of exactly the provided fields, none when the material is absent.

use super::SetMaterial;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetMaterial, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.materials.get(&payload.id) {
        Some(material) => {
            let restored = super::diff::patch(payload).negate(material);
            vec![ModelMutation::SetMaterial(SetMaterial { id: payload.id.clone(), name: restored.name, category: restored.category, color: restored.color, density: restored.density, conductivity: restored.conductivity, specific_heat: restored.specific_heat })]
        }
        None => Vec::new(),
    }
}
`,
  },
};

const layeredHand = (f: Family, verb: "create" | "delete" | "set") => {
  const P = f.pascal;
  const field = f.snake;
  const tag = f.humanCap;
  if (verb === "create") {
    return {
      diff: `//! ${DIFF} Diff constructor for \`Create${P}\`: one created ${f.human} entry. Every layer names an existing material and has a positive finite
//! thickness; a ${f.human} has at least one layer.

use super::Create${P};
use crate::{Entry, Layer, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \\"{}\\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A ${f.human} needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}

pub fn diff(payload: &Create${P}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if base.${f.plural}.contains_key(&payload.id) {
        ${refuse("DuplicateId", `format!("${tag} \\"{}\\" already exists.", payload.id)`, "[payload.id.clone()]")}
    }
    if let Some((code, message, field)) = fault(&payload.${field}.layers, base) {
        return MutationOutcome::refuse(code, message, ["${field}", "layers"].into_iter().chain(field));
    }
    MutationOutcome::new(ModelDiff::${f.plural}(payload.id.clone(), Entry::Created(payload.${field}.clone())))
}
`,
      inverse: `//! ${INVERSE} Inverse of \`Create${P}\`: the concrete \`Delete${P}\` of the id it created, none when the id was already taken.

use super::super::delete_${f.snake}::Delete${P};
use super::Create${P};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Create${P}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${f.plural}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::Delete${P}(Delete${P} { id: payload.id.clone() })]
}
`,
    };
  }
  if (verb === "delete") {
    return {
      diff: `//! ${DIFF} Diff constructor for \`Delete${P}\`: one deleted ${f.human} entry; refused while a ${f.users.slice(0, -1)} still uses the type.

use super::Delete${P};
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Delete${P}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${f.plural}.contains_key(&payload.id) {
        ${refuse("TargetMissing", `format!("${tag} \\"{}\\" does not exist.", payload.id)`, "[payload.id.clone()]")}
    }
    if base.${f.users}.values().any(|row| row.${field} == payload.id) {
        ${refuse("TargetReferenced", `format!("${tag} \\"{}\\" is still used by ${f.users}.", payload.id)`, "[payload.id.clone()]")}
    }
    MutationOutcome::new(ModelDiff::${f.plural}(payload.id.clone(), Entry::Deleted))
}
`,
      inverse: `//! ${INVERSE} Inverse of \`Delete${P}\`: the concrete \`Create${P}\` carrying the full removed record, none when the ${f.human} was absent.

use super::super::create_${f.snake}::Create${P};
use super::Delete${P};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Delete${P}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.${f.plural}.get(&payload.id) {
        Some(${field}) => vec![ModelMutation::Create${P}(Create${P} { id: payload.id.clone(), ${field}: ${field}.clone() })],
        None => Vec::new(),
    }
}
`,
    };
  }
  return {
    diff: `//! ${DIFF} Diff constructor for \`Set${P}\`: a sparse ${f.human} patch of exactly the provided fields. A provided layer stack replaces the whole
//! stack: every layer names an existing material and has a positive finite thickness, and the stack is not empty. A patch that changes nothing is a no-op.

use super::Set${P};
use crate::{Entry, Layer, ModelDiff, ModelSnapshot, Patch, ${P}Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn patch(payload: &Set${P}) -> ${P}Patch {
    ${P}Patch { name: payload.name.clone(), layers: payload.layers.clone() }
}

fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \\"{}\\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A ${f.human} needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}

pub fn diff(payload: &Set${P}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(current) = base.${f.plural}.get(&payload.id) else {
        ${refuse("TargetMissing", `format!("${tag} \\"{}\\" does not exist.", payload.id)`, "[payload.id.clone()]")}
    };
    if let Some((code, message, field)) = payload.layers.as_deref().and_then(|layers| fault(layers, base)) {
        return MutationOutcome::refuse(code, message, ["layers"].into_iter().chain(field));
    }
    let change = patch(payload);
    if change.write(current) == *current {
        ${refuse("NoOp", `format!("${tag} \\"{}\\" already has these values.", payload.id)`, "[payload.id.clone()]")}
    }
    MutationOutcome::new(ModelDiff::${f.plural}(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: `//! ${INVERSE} Inverse of \`Set${P}\`: an absolute \`Set${P}\` restoring the base values of exactly the provided fields, none when the ${f.human} is absent.

use super::Set${P};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &Set${P}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.${f.plural}.get(&payload.id) {
        Some(current) => {
            let restored = super::diff::patch(payload).negate(current);
            vec![ModelMutation::Set${P}(Set${P} { id: payload.id.clone(), name: restored.name, layers: restored.layers })]
        }
        None => Vec::new(),
    }
}
`,
  };
};

const hand = new Map<string, { diff: string; inverse: string }>(Object.entries(materialHand));
for (const f of [wall, slabs, roofs]) for (const verb of ["create", "delete", "set"] as const) hand.set(`${verb}-${f.kebab}`, layeredHand(f, verb));

const once = (file: string, body: string) => {
  mkdirSync(join(file, ".."), { recursive: true });
  if (!existsSync(file)) writeFileSync(file, body);
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "m-materials-layers");
  mkdirSync(out, { recursive: true });
  let all = "";
  for (const leaf of leaves) {
    all += emitLeaf(leaf);
    fixLeaf(leaf);
    const dir = join(mutations, em(leaf.emoji) + leaf.kind);
    const files = hand.get(leaf.kind)!;
    once(join(dir, DIFF + "diff", RS), files.diff);
    once(join(dir, INVERSE + "inverse", RS), files.inverse);
  }
  writeFileSync(join(out, "mounts.txt"), all);
  console.log(`emitted ${leaves.length} leaves`);
  if (process.argv.includes("--mount")) {
    const root = join(artifact, RS);
    const source = readFileSync(root, "utf8");
    const anchor = "//#endregion " + em(0x1f516) + "Leaves";
    const modOf = (leaf: Leaf) => `pub mod ${leaf.kind.replaceAll("-", "_")} {`;
    const fresh = leaves.filter((leaf) => !source.includes(modOf(leaf)));
    if (fresh.length > 0) {
      const lead = '                        #[path = "."]\n';
      const blocks = all.split(lead).filter(Boolean).map((block) => lead + block);
      const text = fresh.map((leaf) => blocks.find((block) => block.includes(modOf(leaf)))!).join("");
      const lineStart = source.lastIndexOf("\n", source.indexOf(anchor)) + 1;
      writeFileSync(root, source.slice(0, lineStart) + text + source.slice(lineStart));
    }
    console.log(`mounted ${fresh.length} leaves`);
  }
}
