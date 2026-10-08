#!/usr/bin/env bun
/**
 * 🧬️ Single source of the `s.bim.model@1` snapshot vocabulary. Run `bun r3-f1-gen-model.ts` to (re)write, from ONE
 * model below: the value-type and entity Rust files, the patch Rust file, and the json/ts/graphql/proto facets of
 * the artifact, snapshot and diff schema families. Hand-written Rust (snapshot struct, diff algebra, mutations)
 * lives beside the generated files and is never touched.
 */
import { mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const em = (...points: number[]) => String.fromCodePoint(...points) + "️";
const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
let subset = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
subset = sub(sub(subset, "plugins"), "bim");
subset = sub(sub(subset, "artifacts"), "model");
subset = join(sub(sub(sub(subset, "standards"), "1"), "subsets"), readdirSync(sub(sub(sub(subset, "standards"), "1"), "subsets")).find((n) => n.endsWith("any"))!);
const schema = sub(subset, "schema");
const dirOf = (parent: string, name: string) => {
  const found = readdirSync(parent).find((n) => n.endsWith(name));
  if (found) return join(parent, found);
  throw new Error(`missing ${name} under ${parent}`);
};
const snapshotDir = dirOf(schema, "snapshot");
const diffDir = dirOf(schema, "diff");
const values = join(snapshotDir, em(0x1f4a0) + "values");
const entities = join(snapshotDir, em(0x1f9f1) + "entities");
const patches = join(diffDir, em(0x1fa79) + "patches");
for (const dir of [values, entities, patches]) mkdirSync(dir, { recursive: true });
const RS = em(0x1f980) + ".rs";
const JSONF = em(0x1f523) + ".json";
const TSF = em(0x1f7e6) + ".ts";
const GQLF = em(0x1f517) + ".graphql";
const PROTOF = em(0x1f6f0) + ".proto";
const file = (dir: string, leaf: string) => join(dir, leaf);

type Field = { name: string; type: string; doc?: string };
type Struct = { name: string; doc: string; fields: Field[]; entity?: { collection: string; plural: string } ; copy?: boolean; singleton?: boolean };
type UnitEnum = { name: string; doc: string; variants: string[] };
type DataEnum = { name: string; doc: string; variants: { name: string; fields: Field[] }[]; intrinsic?: boolean };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":"); const name = part.slice(0, at).trim(); const type = part.slice(at + 1).trim();
    return { name, type };
  });

const unitEnums: UnitEnum[] = [
  { name: "MaterialCategory", doc: "🧱️ Coarse material family.", variants: ["Concrete", "Masonry", "Wood", "Metal", "Glass", "Insulation", "Finish", "Membrane", "Other"] },
  { name: "LayerFunction", doc: "🍰️ Role of one wall, slab or roof layer.", variants: ["Structure", "Substrate", "Insulation", "Finish", "Membrane", "Core"] },
  { name: "LocationLine", doc: "📏️ Which face or line of a wall its axis denotes.", variants: ["Center", "Interior", "Exterior", "CoreCenter"] },
  { name: "Phase", doc: "🕰️ Construction phase of an element.", variants: ["Existing", "New", "Demolished", "Temporary"] },
  { name: "DoorLeaves", doc: "🚪️ Leaf count of a door type.", variants: ["Single", "Double"] },
  { name: "Swing", doc: "↪️ Swing side of a door.", variants: ["Left", "Right"] },
  { name: "Turn", doc: "↩️ Turning direction of a stair flight.", variants: ["Left", "Right"] },
  { name: "StringerKind", doc: "🪵️ Stringer construction of a stair: none (cantilevered treads), closed (side boards housing the steps), open (cut stringers carrying the treads) or mono (one central spine).", variants: ["None", "Closed", "Open", "Mono"] },
  { name: "RiserKind", doc: "🪜️ Whether a stair has riser boards between its treads.", variants: ["Open", "Closed"] },
];

const dataEnums: DataEnum[] = [
  { name: "Axis", doc: "〰️ Wall axis: a line, or an arc whose bulge is tan(sweep / 4); a curved wall is an axis whose bulge is set.", variants: [{ name: "Line", fields: f("start:Point2, end:Point2") }, { name: "Arc", fields: f("start:Point2, end:Point2, bulge:f64") }] },
  { name: "TopConstraint", doc: "🔝️ How the top of an element is resolved: the parametric hook between storeys and elements.", variants: [{ name: "Unconnected", fields: f("height:f64") }, { name: "StoreyTop", fields: f("offset:f64") }, { name: "Storey", fields: f("storey:string, offset:f64") }] },
  { name: "Profile", doc: "▭️ Cross-section of a linear element.", variants: [{ name: "Rectangle", fields: f("width:f64, depth:f64") }, { name: "Circle", fields: f("diameter:f64") }, { name: "IShape", fields: f("width:f64, depth:f64, web:f64, flange:f64") }, { name: "Custom", fields: f("outline:vec:Vertex") }] },
  { name: "RoofShape", doc: "🏠️ Parametric roof form.", variants: [{ name: "Flat", fields: [] }, { name: "Shed", fields: f("pitch:f64, direction:f64") }, { name: "Gable", fields: f("pitch:f64, ridge_direction:f64") }, { name: "Hip", fields: f("pitch:f64") }, { name: "Mansard", fields: f("lower_pitch:f64, upper_pitch:f64, break_height:f64") }] },
  { name: "OpeningKind", doc: "🪟️ What an opening is: a window, a door or a plain void.", variants: [{ name: "Window", fields: f("window_type:string") }, { name: "Door", fields: f("door_type:string") }, { name: "Void", fields: f("width:f64, height:f64") }] },
  { name: "StairFlight", doc: "🪜️ Flight layout of a stair.", variants: [{ name: "Straight", fields: [] }, { name: "LTurn", fields: f("split:f64, turn:Turn") }, { name: "UTurn", fields: f("gap:f64") }, { name: "Spiral", fields: f("radius:f64, sweep:f64") }] },
  { name: "PropertyValue", intrinsic: true, doc: "🏷️ Typed property value: exactly one measure of the kind the variant names (text, real, integer, boolean, length in metres, area in square metres, volume in cubic metres, angle in radians).", variants: [{ name: "Text", fields: f("value:string") }, { name: "Real", fields: f("value:f64") }, { name: "Integer", fields: f("value:i32") }, { name: "Boolean", fields: f("value:bool") }, { name: "Length", fields: f("value:f64") }, { name: "Area", fields: f("value:f64") }, { name: "Volume", fields: f("value:f64") }, { name: "Angle", fields: f("value:f64") }] },
  { name: "Infill", doc: "🪟️ Fill between the posts of a railing: nothing, a glass pane or a solid panel of the given thickness in metres.", variants: [{ name: "None", fields: [] }, { name: "Glass", fields: f("thickness:f64") }, { name: "Panel", fields: f("thickness:f64") }] },
  { name: "SpaceBoundary", doc: "🏠️ How a space outline is given: inferred from the walls around a seed, or explicit.", variants: [{ name: "Bounded", fields: f("seed:Point2") }, { name: "Explicit", fields: f("outline:vec:Vertex") }] },
];

const structs: Struct[] = [
  { name: "Point2", doc: "📍️ Planar point in metres.", copy: true, fields: f("x:f64, y:f64") },
  { name: "Rgb", doc: "🎨️ Linear colour, components in 0..1.", copy: true, fields: f("r:f64, g:f64, b:f64") },
  { name: "Vertex", doc: "🔷️ Loop vertex; bulge shapes the segment to the next vertex.", copy: true, fields: f("point:Point2, bulge:f64") },
  { name: "Slope", doc: "📐️ Slope of a plane: fall direction and angle in radians.", copy: true, fields: f("direction:f64, angle:f64") },
  { name: "StairStringer", doc: "🪵️ A stair stringer: its construction kind, the width (thickness across the stair) and depth (height of the board seen from the side) of each stringer in metres; width and depth are ignored by the kind none.", copy: true, fields: f("kind:StringerKind, width:f64, depth:f64") },
  { name: "Baluster", doc: "🏛️ A baluster row of a railing: the section of one baluster and the clear axis-to-axis spacing along the path in metres.", fields: f("profile:Profile, spacing:f64") },
  { name: "Layer", doc: "🍰️ One layer of a layered type.", fields: f("material:string, thickness:f64, function:LayerFunction") },
  { name: "Project", doc: "🏗️ Project metadata.", singleton: true, fields: f("name:string, description:string, author:string, organization:string, phase_names:vec:string") },
  { name: "Material", doc: "🧱️ A material of the project library.", entity: { collection: "materials", plural: "Materials" }, fields: f("name:string, category:MaterialCategory, color:Rgb, density:f64, conductivity:f64, specific_heat:f64") },
  { name: "WallType", doc: "🧱️ Layered wall build-up.", entity: { collection: "wall_types", plural: "WallTypes" }, fields: f("name:string, layers:vec:Layer") },
  { name: "SlabType", doc: "🧱️ Layered slab build-up.", entity: { collection: "slab_types", plural: "SlabTypes" }, fields: f("name:string, layers:vec:Layer") },
  { name: "RoofType", doc: "🧱️ Layered roof build-up.", entity: { collection: "roof_types", plural: "RoofTypes" }, fields: f("name:string, layers:vec:Layer") },
  { name: "ColumnType", doc: "🏛️ Column profile and material.", entity: { collection: "column_types", plural: "ColumnTypes" }, fields: f("name:string, profile:Profile, material:string") },
  { name: "BeamType", doc: "🏛️ Beam profile and material.", entity: { collection: "beam_types", plural: "BeamTypes" }, fields: f("name:string, profile:Profile, material:string") },
  { name: "WindowType", doc: "🪟️ Window family parameters.", entity: { collection: "window_types", plural: "WindowTypes" }, fields: f("name:string, width:f64, height:f64, sill:f64, frame_width:f64, frame_depth:f64, panes:u32, material:string") },
  { name: "DoorType", doc: "🚪️ Door family parameters.", entity: { collection: "door_types", plural: "DoorTypes" }, fields: f("name:string, width:f64, height:f64, frame_width:f64, frame_depth:f64, leaves:DoorLeaves, swing:Swing, material:string") },
  { name: "Site", doc: "🌍️ A site: geographic anchor and boundary.", entity: { collection: "sites", plural: "Sites" }, fields: f("name:string, latitude:f64, longitude:f64, elevation:f64, true_north:f64, boundary:vec:Point2") },
  { name: "Building", doc: "🏢️ A building on a site.", entity: { collection: "buildings", plural: "Buildings" }, fields: f("site:string, name:string, origin:Point2, rotation:f64, elevation:f64") },
  { name: "Storey", doc: "🪜️ A storey: authored level index, height and optional plan cut height; its elevation is inferred.", entity: { collection: "storeys", plural: "Storeys" }, fields: f("building:string, name:string, level:i32, height:f64, cut_height:opt:f64") },
  { name: "GridLine", doc: "📏️ A labelled grid line of a building.", entity: { collection: "grids", plural: "Grids" }, fields: f("building:string, label:string, start:Point2, end:Point2") },
  { name: "Wall", doc: "🧱️ A wall: axis, type, location line and base/top constraints; never its height.", entity: { collection: "walls", plural: "Walls" }, fields: f("storey:string, wall_type:string, axis:Axis, location:LocationLine, base_offset:f64, top:TopConstraint, phase:Phase, name:string") },
  { name: "CurtainWall", doc: "🪟️ A curtain wall: axis plus grid spacings and mullion profile.", entity: { collection: "curtain_walls", plural: "CurtainWalls" }, fields: f("storey:string, axis:Axis, base_offset:f64, top:TopConstraint, u_spacing:f64, v_spacing:f64, mullion:Profile, panel_material:string, mullion_material:string, name:string") },
  { name: "Column", doc: "🏛️ A column placed on a storey.", entity: { collection: "columns", plural: "Columns" }, fields: f("storey:string, column_type:string, position:Point2, rotation:f64, base_offset:f64, top:TopConstraint, name:string") },
  { name: "Beam", doc: "➖️ A beam: its top lies `top_offset` metres above (positive) or below (negative) the storey top; zero is flush with it.", entity: { collection: "beams", plural: "Beams" }, fields: f("storey:string, beam_type:string, start:Point2, end:Point2, top_offset:f64, name:string") },
  { name: "Slab", doc: "⬜️ A slab: boundary loop, holes, offset and optional slope.", entity: { collection: "slabs", plural: "Slabs" }, fields: f("storey:string, slab_type:string, boundary:vec:Vertex, holes:vec:vec:Vertex, offset:f64, slope:opt:Slope, name:string") },
  { name: "Roof", doc: "🏠️ A roof: footprint loop and shape.", entity: { collection: "roofs", plural: "Roofs" }, fields: f("storey:string, roof_type:string, footprint:vec:Vertex, shape:RoofShape, overhang:f64, base_offset:f64, name:string") },
  { name: "Opening", doc: "🪟️ A window, door or void hosted by a wall or curtain wall; `sill_override` replaces the sill of its type (zero for doors and voids).", entity: { collection: "openings", plural: "Openings" }, fields: f("host:string, kind:OpeningKind, offset:f64, sill_override:opt:f64, width:opt:f64, height:opt:f64, flip_hand:bool, flip_facing:bool, name:string") },
  { name: "Stair", doc: "🪜️ A stair run: placement, flight, limits and construction (stringer, nosing, tread thickness, risers, landing depth).", entity: { collection: "stairs", plural: "Stairs" }, fields: f("storey:string, start:Point2, direction:f64, width:f64, flight:StairFlight, top:TopConstraint, max_riser:f64, min_tread:f64, stringer:StairStringer, nosing:f64, tread_thickness:f64, riser:RiserKind, landing_depth:f64, name:string") },
  { name: "Railing", doc: "🛤️ A railing along a path: heights, rail and post sections, optional balusters and an infill.", entity: { collection: "railings", plural: "Railings" }, fields: f("storey:string, path:vec:Point2, height:f64, post_spacing:f64, profile:Profile, post_profile:Profile, baluster:opt:Baluster, infill:Infill, material:string, base_offset:f64, name:string") },
  { name: "Space", doc: "🏠️ A room.", entity: { collection: "spaces", plural: "Spaces" }, fields: f("storey:string, number:string, name:string, boundary:SpaceBoundary, usage:string") },
  { name: "Classification", doc: "🗂️ A classification reference of an element.", entity: { collection: "classifications", plural: "Classifications" }, fields: f("system:string, code:string, title:string") },
];

const fieldDocs: Record<string, string> = {
  "Beam.top_offset": "Signed vertical offset in metres of the top of the beam from the top of its storey: positive lifts it above the storey top, negative hangs it below, zero is flush.",
  "Storey.cut_height": "Plan view convention: height in metres above the storey elevation at which the plan cuts through the storey; absent means the default of 1.2 m.",
  "Opening.sill_override": "Sill height in metres above the host base that replaces the sill of the window type; absent means the type sill for windows and zero for doors and voids.",
  "Stair.stringer": "Stringer construction: kind, plus width and depth of each stringer in metres.",
  "Stair.nosing": "Overhang in metres of a tread beyond the riser below it; zero is flush.",
  "Stair.tread_thickness": "Vertical thickness in metres of each tread slab.",
  "Stair.riser": "Whether riser boards close the gap between treads.",
  "Stair.landing_depth": "Depth in metres of the landing of a turning flight, measured along the arriving flight.",
  "Railing.profile": "Section of the top rail.",
  "Railing.post_profile": "Section of a post.",
  "Railing.baluster": "Optional baluster row between the posts: section and spacing.",
  "Railing.infill": "Fill between the posts: none, glass or a panel with a thickness in metres.",
};
for (const s of structs) for (const x of s.fields) x.doc = fieldDocs[`${s.name}.${x.name}`];
const entityList = structs.filter((s) => s.entity);
const unitNames = new Set(unitEnums.map((e) => e.name));
const dataNames = new Set(dataEnums.map((e) => e.name));
const structNames = new Set(structs.map((s) => s.name));

//#region 🔖️TypeParsing
type Ty = { kind: "scalar"; name: string } | { kind: "ref"; name: string } | { kind: "opt" | "vec"; inner: Ty };
const parseTy = (text: string): Ty => {
  if (text.startsWith("opt:")) return { kind: "opt", inner: parseTy(text.slice(4)) };
  if (text.startsWith("vec:")) return { kind: "vec", inner: parseTy(text.slice(4)) };
  return ["f64", "i32", "u32", "bool", "string"].includes(text) ? { kind: "scalar", name: text } : { kind: "ref", name: text };
};
const rust = (t: Ty): string => (t.kind === "scalar" ? (t.name === "string" ? "String" : t.name) : t.kind === "ref" ? t.name : t.kind === "opt" ? `Option<${rust(t.inner)}>` : `Vec<${rust(t.inner)}>`);
const isRecordRef = (t: Ty) => t.kind === "ref" && structNames.has(t.name);
const isDataRef = (t: Ty) => t.kind === "ref" && dataNames.has(t.name);
const deepInner = (t: Ty): Ty => (t.kind === "opt" || t.kind === "vec" ? deepInner(t.inner) : t);
//#endregion 🔖️TypeParsing

const bodyDefault = (t: Ty) => t.kind === "vec" || (t.kind === "opt");
const fieldAttrs = (field: Field, inEnum = false): string => {
  const t = parseTy(field.type);
  const out: string[] = [];
  if (t.kind === "vec" && t.inner.kind === "ref" && structNames.has(t.inner.name)) out.push("#[dsl(table)]");
  else if (isDataRef(t)) out.push("#[dsl(statements, block)]");
  else if (isRecordRef(t) || (t.kind === "opt" && isRecordRef(t.inner))) out.push("#[dsl(block)]");
  if (inEnum) {} else if (t.kind === "opt") out.push('#[value(default, skip_serializing_if = "Option::is_none")]');
  else if (t.kind === "vec") out.push("#[value(default)]");
  return out.join(" ");
};

//#region 🔖️RustValues
const header = (what: string) => `//! 🤖️ Generated by \`.🧬semio/…/BIM-PLUGIN/r3-f1-gen-model.ts\` — ${what}. Edit the model there, not here.\n\n`;
let valuesRs = header("value types of `s.bim.model@1`");
for (const e of unitEnums) {
  valuesRs += `/// ${e.doc}\n#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]\npub enum ${e.name} {\n${e.variants.map((v) => `    ${v},`).join("\n")}\n}\n\n`;
}
valuesRs += `/// 🔷️ A closed loop of bulged vertices (counter-clockwise).\npub type Loop = Vec<Vertex>;\n\n`;
const variantFields = (v: { name: string; fields: Field[] }) =>
  v.fields.length === 0 ? `    ${v.name},` : `    ${v.name} {\n${v.fields.map((x) => `        ${fieldAttrs(x, true)} ${x.name}: ${rust(parseTy(x.type))},`.replace(/^ {8} /, "        ")).join("\n")}\n    },`;
const intrinsicField = (name: string) => `/// 🧾️ The value travels through the text language as its schema value: the externally tagged object of the JSON form.
impl semio_framework_dsl_record::DslField for ${name} {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Value)
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        <Self as semio_framework_value::ToValue>::to_value_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Value)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => <Self as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error| error.to_string()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => <Self as semio_framework_value::FromValue>::from_value_controlled(value, control),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an intrinsic Value field")),
        }
    }
}

/// 🫳️ The borrowed text schema of an intrinsic value field.
impl semio_framework_dsl_record::BorrowedDslField for ${name} {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Value;
}

`;
for (const e of dataEnums) {
  const derives = `Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue${e.intrinsic ? "" : ", semio_framework_dsl_record_derive::DslEnum"}`;
  valuesRs += `/// ${e.doc}\n#[derive(${derives})]\npub enum ${e.name} {\n${e.variants.map(variantFields).join("\n")}\n}\n\n${e.intrinsic ? intrinsicField(e.name) : ""}`;
}
const structRs = (s: Struct) => {
  const derives = `Clone, ${s.copy ? "Copy, " : ""}Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord`;
  return `/// ${s.doc}\n#[derive(${derives})]\npub struct ${s.name} {\n${s.fields.map((x) => `${x.doc ? `    /// ${x.doc}\n` : ""}    ${fieldAttrs(x)} pub ${x.name}: ${rust(parseTy(x.type))},`.replace(/\n {4} pub/, "\n    pub").replace(/^ {4} pub/, "    pub")).join("\n")}\n}\n\n`;
};
for (const s of structs.filter((x) => !x.entity)) valuesRs += structRs(s);
valuesRs += `/// 🏷️ All property sets of one element: property set name → property name → value.\npub type PropertySet = std::collections::BTreeMap<String, std::collections::BTreeMap<String, PropertyValue>>;\n`;
let entitiesRs = header("entity records of `s.bim.model@1`") + "use super::values::*;\n\n";
for (const s of entityList) entitiesRs += structRs(s);
//#endregion 🔖️RustValues

//#region 🔖️RustPatches
let patchRs = header("sparse patches of `s.bim.model@1` entities") + "use super::*;\n\n";
const patchables = structs.filter((s) => s.entity || s.singleton);
const slotTy = (t: Ty) => (t.kind === "opt" ? `Option<Assigned<${rust(t)}>>` : `Option<${rust(t)}>`);
for (const s of patchables) {
  const fields = s.fields.map((x) => ({ ...x, t: parseTy(x.type) }));
  const opt = (x: { t: Ty }) => x.t.kind === "opt";
  patchRs += `/// 🩹 Sparse patch of [\`${s.name}\`]: an absent field is untouched.\n#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n#[value(default)]\npub struct ${s.name}Patch {\n${fields.map((x) => `    #[value(skip_serializing_if = "Option::is_none")]\n    pub ${x.name}: ${slotTy(x.t)},`).join("\n")}\n}\n\n`;
  patchRs += `impl Patch<${s.name}> for ${s.name}Patch {\n`;
  patchRs += `    fn write(&self, base: &${s.name}) -> ${s.name} {\n        ${s.name} { ${fields.map((x) => `${x.name}: ${opt(x) ? "take_assigned" : "take"}(&self.${x.name}, &base.${x.name})`).join(", ")} }\n    }\n`;
  patchRs += `    fn negate(&self, base: &${s.name}) -> Self {\n        Self { ${fields.map((x) => `${x.name}: ${opt(x) ? "restore_assigned" : "restore"}(&self.${x.name}, &base.${x.name})`).join(", ")} }\n    }\n`;
  patchRs += `    fn merge(&mut self, later: Self) {\n${fields.map((x) => `        merge_slot(&mut self.${x.name}, later.${x.name});`).join("\n")}\n    }\n`;
  patchRs += `    fn between(from: &${s.name}, to: &${s.name}) -> Self {\n        Self { ${fields.map((x) => `${x.name}: ${opt(x) ? "differs_assigned" : "differs"}(&from.${x.name}, &to.${x.name})`).join(", ")} }\n    }\n`;
  patchRs += `    fn minimal(&self, base: &${s.name}) -> Self {
        Self { ${fields.map((x) => `${x.name}: ${opt(x) ? "changed_assigned" : "changed"}(&self.${x.name}, &base.${x.name})`).join(", ")} }
    }
`;
  patchRs += `    fn touched(&self) -> Vec<String> {\n        let mut out = Vec::new();\n${fields.map((x) => `        if self.${x.name}.is_some() {\n            out.push("${x.name}".to_string());\n        }`).join("\n")}\n        out\n    }\n`;
  patchRs += `    fn is_empty(&self) -> bool {\n        self == &Self::default()\n    }\n}\n\n`;
}
//#endregion 🔖️RustPatches

writeFileSync(file(values, RS), valuesRs);
writeFileSync(file(entities, RS), entitiesRs);
writeFileSync(file(patches, RS), patchRs);

//#region 🔖️JsonSchema
const NS = "https://json.schemas.assets.semio-tech.com/s/bim/model";
const jsonOf = (t: Ty, root: string): any => {
  switch (t.kind) {
    case "scalar":
      return t.name === "string" ? { type: "string" } : t.name === "bool" ? { type: "boolean" } : t.name === "f64" ? { type: "number" } : { type: "integer" };
    case "ref":
      return { $ref: `${root}#/$defs/${t.name}` };
    case "opt":
      return jsonOf(t.inner, root);
    case "vec":
      return { type: "array", items: jsonOf(t.inner, root) };
  }
};
const objectSchema = (fields: Field[], root: string, extra: Record<string, unknown> = {}) => {
  const parsed = fields.map((x) => ({ ...x, t: parseTy(x.type) }));
  return {
    type: "object",
    additionalProperties: false,
    required: parsed.filter((x) => x.t.kind !== "opt" && x.t.kind !== "vec").map((x) => x.name),
    properties: Object.fromEntries(parsed.map((x) => [x.name, x.doc ? { ...jsonOf(x.t, root), description: x.doc } : jsonOf(x.t, root)])),
    ...extra,
  };
};
const defs = (root: string): Record<string, any> => {
  const out: Record<string, any> = {};
  for (const e of unitEnums) out[e.name] = { enum: e.variants };
  for (const e of dataEnums)
    out[e.name] = {
      oneOf: e.variants.map((v) =>
        v.fields.length === 0 ? { const: v.name } : { type: "object", additionalProperties: false, required: [v.name], properties: { [v.name]: objectSchema(v.fields, root) } },
      ),
    };
  for (const s of structs) out[s.name] = objectSchema(s.fields, root);
  out.PropertySet = { type: "object", additionalProperties: { type: "object", additionalProperties: { $ref: `${root}#/$defs/PropertyValue` } } };
  return out;
};
const collections: { field: string; entity: string; patch: string; schemaTy: any }[] = [
  ...entityList.map((s) => ({ field: s.entity!.collection, entity: s.name, patch: `${s.name}Patch`, schemaTy: null })),
];
collections.splice(
  collections.findIndex((c) => c.field === "classifications"),
  0,
  { field: "properties", entity: "PropertySet", patch: "PropertySetPatch", schemaTy: null },
);
// Reorder so the on-wire / Rust order is materials..spaces, properties, classifications.
const order = ["materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "properties", "classifications"];
collections.sort((a, b) => order.indexOf(a.field) - order.indexOf(b.field));
const snapshotProps = (root: string) => ({
  schema: { type: "string", "x-semio-state": "artifact" },
  project: { $ref: `${root}#/$defs/Project`, "x-semio-state": "artifact" },
  ...Object.fromEntries(collections.map((c) => [c.field, { type: "object", additionalProperties: { $ref: `${root}#/$defs/${c.entity}` }, "x-semio-state": "artifact" }])),
});
const artifactJson = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: `${NS}/artifact.json`,
  title: "ModelArtifact",
  type: "object",
  additionalProperties: false,
  required: ["schema", "project"],
  properties: snapshotProps(""),
  $defs: defs(""),
};
const snapshotJson = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: `${NS}/snapshot.json`,
  title: "ModelSnapshot",
  type: "object",
  additionalProperties: false,
  required: ["schema", "project"],
  properties: snapshotProps(`${NS}/artifact.json`),
};
const A = `${NS}/artifact.json`;
const entryDef = (entityRef: string, patchRef: string) => ({
  oneOf: [
    { type: "object", required: ["entry"], properties: { entry: { const: "Created" } }, allOf: [{ $ref: entityRef }] },
    { type: "object", additionalProperties: false, required: ["entry"], properties: { entry: { const: "Deleted" } } },
    { type: "object", required: ["entry"], properties: { entry: { const: "Replaced" } }, allOf: [{ $ref: entityRef }] },
    { type: "object", required: ["entry"], properties: { entry: { const: "Patched" } }, allOf: [{ $ref: patchRef }] },
  ],
});
const patchDefs = () => {
  const out: Record<string, any> = {};
  for (const s of patchables)
    out[`${s.name}Patch`] = { type: "object", additionalProperties: false, properties: Object.fromEntries(s.fields.map((x) => [x.name, jsonOf(parseTy(x.type), A)])) };
  out.PropertySetPatch = { type: "object", additionalProperties: false, properties: { assigned: { type: "object", additionalProperties: { type: "object", additionalProperties: { oneOf: [{ $ref: `${A}#/$defs/PropertyValue` }, { type: "null" }] } } } } };
  for (const c of collections) out[`${c.entity}Entry`] = entryDef(`${A}#/$defs/${c.entity}`, `#/$defs/${c.patch}`);
  return out;
};
const diffJson = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: `${NS}/diff.json`,
  title: "ModelDiff",
  type: "object",
  additionalProperties: false,
  properties: {
    project: { $ref: "#/$defs/ProjectPatch" },
    ...Object.fromEntries(collections.map((c) => [c.field, { type: "object", additionalProperties: { $ref: `#/$defs/${c.entity}Entry` } }])),
  },
  $defs: patchDefs(),
};
const writeJson = (dir: string, doc: unknown) => writeFileSync(join(dir, JSONF), JSON.stringify(doc, null, 2) + "\n");
writeJson(schema, artifactJson);
writeJson(snapshotDir, snapshotJson);
writeJson(diffDir, diffJson);
//#endregion 🔖️JsonSchema

//#region 🔖️TsGraphqlProto
const tsTy = (t: Ty): string => (t.kind === "scalar" ? (t.name === "string" ? "string" : t.name === "bool" ? "boolean" : "number") : t.kind === "ref" ? t.name : t.kind === "opt" ? `${tsTy(t.inner)} | null` : `${tsTy(t.inner)}[]`);
const gqlTy = (t: Ty): string => (t.kind === "scalar" ? (t.name === "string" ? "String" : t.name === "bool" ? "Boolean" : t.name === "f64" ? "Float" : "Int") : t.kind === "ref" ? t.name : t.kind === "opt" ? gqlTy(t.inner) : `[${gqlTy(t.inner)}!]`);
const gqlField = (x: Field) => {
  const t = parseTy(x.type);
  return `  ${x.name}: ${gqlTy(t)}${t.kind === "opt" ? "" : "!"}`;
};
const protoTy = (t: Ty): string => (t.kind === "scalar" ? ({ string: "string", bool: "bool", f64: "double", i32: "int32", u32: "uint32" } as any)[t.name] : t.kind === "ref" ? t.name : protoTy(t.inner));
const listName = (t: Ty): string => (t.kind === "vec" ? `${listName(t.inner)}List` : t.kind === "opt" ? listName(t.inner) : t.kind === "scalar" ? ({ string: "String", bool: "Bool", f64: "Double", i32: "Int", u32: "UInt" } as any)[t.name] : t.name);
const lists = new Map<string, Ty>();
const noteList = (t: Ty) => { if (t.kind === "vec") { lists.set(listName(t), t); noteList(t.inner); } else if (t.kind === "opt") noteList(t.inner); };
const protoField = (x: Field, n: number, patch = false) => {
  const t = parseTy(x.type);
  noteList(t);
  const bare = t.kind === "opt" ? t.inner : t;
  if (patch) return `  optional ${bare.kind === "vec" ? listName(bare) : protoTy(bare)} ${x.name} = ${n};`;
  if (bare.kind === "vec" && bare.inner.kind === "vec") return `  ${t.kind === "opt" ? "optional " : "repeated "}${listName(bare.inner)} ${x.name} = ${n};`;
  return `  ${bare.kind === "vec" ? "repeated " : t.kind === "opt" ? "optional " : ""}${protoTy(bare)} ${x.name} = ${n};`;
};
const listMessages = () => [...lists.entries()].map(([name, t]) => `message ${name} {\n  repeated ${t.kind === "vec" ? (t.inner.kind === "vec" ? listName(t.inner) : protoTy(t.inner)) : ""} items = 1;\n}\n`).join("\n");
const tsDefs = (): string => {
  let out = "";
  for (const e of unitEnums) out += `export type ${e.name} = ${e.variants.map((v) => `"${v}"`).join(" | ")};\n\n`;
  for (const e of dataEnums) out += `export type ${e.name} =\n${e.variants.map((v) => (v.fields.length ? `  | { ${v.name}: { ${v.fields.map((x) => `${x.name}: ${tsTy(parseTy(x.type))}`).join("; ")} } }` : `  | "${v.name}"`)).join("\n")};\n\n`;
  for (const s of structs) out += `export interface ${s.name} {\n${s.fields.map((x) => `  ${x.name}${parseTy(x.type).kind === "opt" ? "?" : ""}: ${tsTy(parseTy(x.type))};`).join("\n")}\n}\n\n`;
  out += `export type PropertySet = Record<string, Record<string, PropertyValue>>;\n\n`;
  return out;
};
const stateFields = [{ name: "schema", type: "string" }, { name: "project", type: "Project" }];
const snapshotTs = (name: string) =>
  `export interface ${name} {\n${stateFields.map((x) => `  /** @state artifact */\n  ${x.name}: ${tsTy(parseTy(x.type))};`).join("\n")}\n${collections.map((c) => `  /** @state artifact */\n  ${c.field}?: Record<string, ${c.entity}>;`).join("\n")}\n}\n\n`;
const artifactTs = `/** 🧬️ BIM model artifact schema — every field with its state class. */\n\n${snapshotTs("ModelArtifact")}${tsDefs()}`;
const snapshotTsDoc = `/** 🧬️ BIM model snapshot schema — authored parameters only; nothing derivable is stored. */\n\nimport type { ${[...unitEnums.map((e) => e.name), ...dataEnums.map((e) => e.name), ...structs.map((s) => s.name), "PropertySet"].join(", ")} } from "../${TSF}";\n\n${snapshotTs("ModelSnapshot")}`;
const patchTs = () => {
  let out = `export interface Assigned<T> {\n  value: T;\n}\n\n`;
  for (const s of patchables) out += `export interface ${s.name}Patch {\n${s.fields.map((x) => `  ${x.name}?: ${parseTy(x.type).kind === "opt" ? `Assigned<${tsTy(parseTy(x.type))}>` : tsTy(parseTy(x.type))};`).join("\n")}\n}\n\n`;
  out += `export interface PropertySetPatch {\n  assigned?: Record<string, Record<string, PropertyValue | null>>;\n}\n\n`;
  for (const c of collections) out += `export type ${c.entity}Entry = ({ entry: "Created" } & ${c.entity}) | { entry: "Deleted" } | ({ entry: "Replaced" } & ${c.entity}) | ({ entry: "Patched" } & ${c.patch});\n`;
  return out;
};
const diffTs = `/** 🔺️ BIM model diff schema — a sparse per-collection keyed delta; absent collections are untouched. */\n\nimport type { ${[...structs.filter((s) => !s.entity && !s.singleton).map((s) => s.name), ...dataEnums.map((e) => e.name), ...unitEnums.map((e) => e.name), ...entityList.map((s) => s.name), "Project", "PropertySet"].join(", ")} } from "../${TSF}";\n\n${patchTs()}\nexport interface ModelDiff {\n  project?: ProjectPatch;\n${collections.map((c) => `  ${c.field}?: Record<string, ${c.entity}Entry>;`).join("\n")}\n}\n`;
const gqlDefs = (): string => {
  let out = "";
  for (const e of unitEnums) out += `enum ${e.name} {\n${e.variants.map((v) => `  ${v}`).join("\n")}\n}\n\n`;
  for (const e of dataEnums) out += `# externally tagged: exactly one member is present\ntype ${e.name} {\n${e.variants.map((v) => `  ${v}: ${v.fields.length ? `${e.name}${v.name}` : "Boolean"}`).join("\n")}\n}\n\n${e.variants.filter((v) => v.fields.length).map((v) => `type ${e.name}${v.name} {\n${v.fields.map(gqlField).join("\n")}\n}\n`).join("\n")}\n`;
  for (const s of structs) out += `type ${s.name} {\n${s.fields.map(gqlField).join("\n")}\n}\n\n`;
  return out;
};
const gqlMaps = (): string => collections.map((c) => `type ${c.entity}Row {\n  id: String!\n  value: ${c.entity === "PropertySet" ? "PropertySetRows" : c.entity}!\n}\n`).join("\n") + `\ntype PropertySetRows {\n  rows: [PropertyRow!]!\n}\n\ntype PropertyRow {\n  set: String!\n  name: String!\n  value: PropertyValue!\n}\n\n`;
const gqlSnapshot = (name: string) => `type ${name} {\n  schema: String! @state(class: ARTIFACT)\n  project: Project! @state(class: ARTIFACT)\n${collections.map((c) => `  ${c.field}: [${c.entity}Row!]! @state(class: ARTIFACT)`).join("\n")}\n}\n\n`;
const artifactGql = `# 🧬️ BIM model artifact schema — every field with its state class.\n\n${gqlSnapshot("ModelArtifact")}${gqlDefs()}${gqlMaps()}`;
const snapshotGql = `# 🧬️ BIM model snapshot schema — authored parameters only.\n\n${gqlSnapshot("ModelSnapshot")}`;
const gqlPatches = (): string => {
  let out = `type AssignedValue {\n  present: Boolean!\n}\n\n`;
  for (const s of patchables) out += `type ${s.name}Patch {\n${s.fields.map((x) => `  ${x.name}: ${gqlTy(parseTy(x.type))}`).join("\n")}\n}\n\n`;
  out += `type PropertySetPatch {\n  assigned: [PropertyRow!]\n}\n\n`;
  out += `enum EntryKind {\n  Created\n  Deleted\n  Replaced\n  Patched\n}\n\n`;
  for (const c of collections) out += `type ${c.entity}Entry {\n  id: String!\n  entry: EntryKind!\n  created: ${c.entity === "PropertySet" ? "PropertySetRows" : c.entity}\n  patch: ${c.patch}\n}\n\n`;
  return out;
};
const diffGql = `# 🔺️ BIM model diff schema — a sparse per-collection keyed delta.\n\ntype ModelDiff {\n  project: ProjectPatch\n${collections.map((c) => `  ${c.field}: [${c.entity}Entry!]`).join("\n")}\n}\n\n${gqlPatches()}`;
const protoDefs = (): string => {
  let out = "";
  for (const e of unitEnums) out += `enum ${e.name} {\n  ${e.name.toUpperCase()}_UNSPECIFIED = 0;\n${e.variants.map((v, i) => `  ${e.name.toUpperCase()}_${v.toUpperCase()} = ${i + 1};`).join("\n")}\n}\n\n`;
  for (const e of dataEnums) {
    out += e.variants.filter((v) => v.fields.length).map((v) => `message ${e.name}${v.name} {\n${v.fields.map((x, i) => protoField(x, i + 1)).join("\n")}\n}\n`).join("\n");
    out += `\nmessage ${e.name} {\n  oneof variant {\n${e.variants.map((v, i) => `    ${v.fields.length ? `${e.name}${v.name}` : "bool"} ${v.name.toLowerCase()} = ${i + 1};`).join("\n")}\n  }\n}\n\n`;
  }
  const rows = new Set<string>();
  for (const s of structs) {
    for (const x of s.fields) { const t = parseTy(x.type); if (t.kind === "vec" && t.inner.kind === "vec") rows.add(protoTy(t)); }
    out += `message ${s.name} {\n${s.fields.map((x, i) => protoField(x, i + 1)).join("\n")}\n}\n\n`;
  }
  for (const r of rows) out += `message ${r}Row {\n  repeated ${r} items = 1;\n}\n\n`;
  out += `message PropertySetRows {\n  map<string, PropertyGroup> sets = 1;\n}\n\nmessage PropertyGroup {\n  map<string, PropertyValue> properties = 1;\n}\n\n`;
  return out;
};
const protoSnapshot = (name: string) => `message ${name} {\n  string schema = 1;\n  Project project = 2;\n${collections.map((c, i) => `  map<string, ${c.entity === "PropertySet" ? "PropertySetRows" : c.entity}> ${c.field} = ${i + 3};`).join("\n")}\n}\n\n`;
const artifactProto = `syntax = "proto3";\npackage semio.s.bim.model.artifact;\n\n// 🧬️ BIM model artifact schema — every field with its state class.\n\n${protoSnapshot("ModelArtifact")}${protoDefs()}${listMessages()}`;
const snapshotProto = `syntax = "proto3";\npackage semio.s.bim.model.snapshot;\nimport "../${PROTOF}";\n\n// 🧬️ BIM model snapshot schema — authored parameters only.\n\n${protoSnapshot("ModelSnapshot").replaceAll("map<string, ", "map<string, artifact.")}`;
const protoPatches = (): string => {
  let out = `message Assigned {\n  bool present = 1;\n}\n\n`;
  for (const s of patchables) out += `message ${s.name}Patch {\n${s.fields.map((x, i) => protoField(x, i + 1, true)).join("\n")}\n}\n\n`;
  out += `message PropertySetPatch {\n  map<string, PropertyGroupPatch> assigned = 1;\n}\n\nmessage PropertyGroupPatch {\n  map<string, PropertyValue> set = 1;\n  repeated string removed = 2;\n}\n\n`;
  out += `enum EntryKind {\n  ENTRY_UNSPECIFIED = 0;\n  ENTRY_CREATED = 1;\n  ENTRY_DELETED = 2;\n  ENTRY_REPLACED = 3;\n  ENTRY_PATCHED = 4;\n}\n\n`;
  for (const c of collections) out += `message ${c.entity}Entry {\n  EntryKind entry = 1;\n  optional ${c.entity === "PropertySet" ? "PropertySetRows" : c.entity} record = 2;\n  optional ${c.patch} patch = 3;\n}\n\n`;
  return out;
};
const diffProto = `syntax = "proto3";\npackage semio.s.bim.model.diff;\n\n// 🔺️ BIM model diff schema — a sparse per-collection keyed delta.\n\nmessage ModelDiff {\n  optional ProjectPatch project = 1;\n${collections.map((c, i) => `  map<string, ${c.entity}Entry> ${c.field} = ${i + 2};`).join("\n")}\n}\n\n${protoPatches()}${listMessages()}`;
const w = (dir: string, leaf: string, body: string) => writeFileSync(join(dir, leaf), body);
w(schema, TSF, artifactTs);
w(snapshotDir, TSF, snapshotTsDoc);
w(diffDir, TSF, diffTs);
w(schema, GQLF, artifactGql);
w(snapshotDir, GQLF, snapshotGql);
w(diffDir, GQLF, diffGql);
w(schema, PROTOF, artifactProto);
w(snapshotDir, PROTOF, snapshotProto);
w(diffDir, PROTOF, diffProto);
//#endregion 🔖️TsGraphqlProto

console.log(`wrote model: ${structs.length} structs, ${unitEnums.length + dataEnums.length} enums, ${collections.length} collections`);
