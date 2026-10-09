#!/usr/bin/env bun
/**
 * 🔲️ Wave W1 `w06-ceilings` (binary tags 6000..6006): the seven ceiling leaves of `s.bim.model@1`: create/set/delete of the layered ceiling
 * type and create/set/set-boundary/delete of the ceiling. `bun r10-w06-ceilings-leaves.ts` rewrites the boilerplate and fixtures through
 * `emitLeaf`, fixes the optional `set-*` fields of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`,
 * never overwritten) and mounts the leaves once in the artifact root. `after`/`diff` fixtures of applied cases are never overwritten: bless
 * them with `BIM_BLESS=1 cargo test`.
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
type Mine = Prop & { optional?: true };
type MineLeaf = Omit<Leaf, "props"> & { props: Mine[] };

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Mine => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const sparse = (prop: Mine, description: Label): Mine => ({ ...prop, optional: true, rust: `Option<${prop.rust}>`, ui: { ...prop.ui, description } });
const keep: Label = { en: "Leave empty to keep the current value.", de: "Leer lassen, um den aktuellen Wert zu behalten." };
const reference = (name: string, kind: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group: "type", order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"] });
const text = (name: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const layersProp = (order: number): Mine => ({ name: "layers", rust: "Vec<Layer>", schema: { type: "array", items: record("Layer") }, ui: { widget: "list", role: "value", label: { en: "Layers", de: "Schichten" }, group: "value", order } });
const loopProp = (name: string, label: Label, order: number): Mine => ({ name, rust: "Vec<Vertex>", schema: { type: "array", items: record("Vertex") }, ui: { widget: "record", role: "value", label, group: "outline", order } });
const holesProp = (order: number): Mine => ({ name: "holes", rust: "Vec<Vec<Vertex>>", schema: { type: "array", items: { type: "array", items: record("Vertex") } }, ui: { widget: "record", role: "value", label: { en: "Holes", de: "Öffnungen" }, group: "outline", order } });
const slopeProp = (order: number): Mine => ({
  name: "slope",
  rust: "Assigned<Option<Slope>>",
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [{ type: "null" }, record("Slope")] } } },
  ui: { widget: "record", role: "value", label: { en: "Slope", de: "Neigung" }, group: "value", order },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Leaf["cases"][number]["outcome"] };
const cases = (rows: Row[]): Leaf["cases"] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

const V = (x: number, y: number, bulge = 0) => ({ point: F.P(x, y), bulge });
const ring = (list: [number, number][]) => list.map(([x, y]) => V(x, y));
const rect = (x0: number, y0: number, x1: number, y1: number) => ring([[x0, y0], [x1, y0], [x1, y1], [x0, y1]]);
const bowTie = ring([[0, 0], [6, 4], [6, 0], [0, 4]]);
const clockwise = ring([[0, 0], [0, 6], [8, 6], [8, 0]]);
const ceiling = (storey: string, type: string, boundary: unknown, holes: unknown[], offset: number, label: string, slope?: { direction: number; angle: number }) => ({ storey, ceiling_type: type, boundary, holes, offset, ...(slope ? { slope } : {}), name: label });
const layer = F.layer;

const library = (extra: { ceilings?: Record<string, unknown>; ceiling_types?: Record<string, unknown>; extra?: Record<string, unknown> } = {}) => {
  const base: Record<string, unknown> = {
    ...F.scene(),
    materials: { "m-brick": F.material("Brick"), "m-board": F.material("Plasterboard"), "m-spare": F.material("Spare") },
    ceiling_types: extra.ceiling_types ?? { "ct-board": { name: "Plasterboard 12.5", layers: [layer("m-board", 0.0125, "Finish")] }, "ct-tile": { name: "Acoustic tile", layers: [layer("m-board", 0.02, "Finish")] } },
    ...(extra.extra ?? {}),
  };
  return extra.ceilings && Object.keys(extra.ceilings).length > 0 ? { ...base, ceilings: extra.ceilings } : base;
};
const LIVING = () => ceiling("st-ground", "ct-board", rect(0, 0, 8, 6), [rect(3, 2, 4, 3)], 0.3, "Living ceiling");
const SLOPED = () => ceiling("st-ground", "ct-board", rect(0, 0, 8, 6), [rect(3, 2, 4, 3)], 0.3, "Living ceiling", { direction: 0, angle: 0.05 });
const withCeiling = () => library({ ceilings: { "ce-living": LIVING() } });
const withSloped = () => library({ ceilings: { "ce-living": SLOPED() } });
const withData = () => ({ ...withCeiling(), properties: { "ce-living": { Pset_CoveringCommon: { FireRating: { Text: { value: "REI 30" } } } } }, classifications: { "ce-living": { system: "Uniclass", code: "Ss_32_10", title: "Suspended ceilings" } } });
const withSpareType = () => library({ ceiling_types: { "ct-board": { name: "Plasterboard 12.5", layers: [layer("m-board", 0.0125, "Finish")] }, "ct-tile": { name: "Acoustic tile", layers: [layer("m-board", 0.02, "Finish")] }, "ct-spare": { name: "Spare", layers: [layer("m-spare", 0.01)] } } });

type Extra = { optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf): MineLeaf => (extras.set(spec.kind, { optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const typeLabel = { en: "Ceiling type", de: "Unterdeckentyp" };
const kindsOfTypes: MineLeaf[] = [
  leaf({
    kind: "create-ceiling-type", emoji: 0x1f391, variant: "CreateCeilingType", verb: "create", entity: "ceiling-type", binaryTag: 6000, displayName: "Create Ceiling Type",
    doc: "Brings a new layered ceiling type into the library; every layer names an existing material and has a positive thickness.",
    props: [id("ceiling-type", "identity", { en: "Ceiling type id", de: "Unterdeckentyp-Id" }, 10), recordProp("ceiling_type", "CeilingType", typeLabel)],
    label: { en: 'format!("Create ceiling type \\"{}\\"", self.ceiling_type.name)', de: 'format!("Unterdeckentyp \\"{}\\" anlegen", self.ceiling_type.name)' },
    target,
    cases: cases([
      { name: "adds", before: library(), mutation: { id: "ct-mineral", ceiling_type: { name: "Mineral fibre", layers: [layer("m-board", 0.015, "Finish"), layer("m-brick", 0.05, "Insulation")] } }, outcome: ok },
      { name: "duplicate", before: library(), mutation: { id: "ct-board", ceiling_type: { name: "Again", layers: [layer("m-board", 0.0125)] } }, outcome: reject("mutation.duplicate-id", ["ct-board"]) },
      { name: "material-missing", before: library(), mutation: { id: "ct-mineral", ceiling_type: { name: "Ghost", layers: [layer("m-ghost", 0.02)] } }, outcome: reject("mutation.target-missing", ["ceiling_type", "layers", "material"]) },
      { name: "empty-layers", before: library(), mutation: { id: "ct-mineral", ceiling_type: { name: "Hollow", layers: [] } }, outcome: reject("mutation.invariant", ["ceiling_type", "layers"]) },
      { name: "thin-layer", before: library(), mutation: { id: "ct-mineral", ceiling_type: { name: "Flat", layers: [layer("m-board", 0)] } }, outcome: reject("mutation.invariant", ["ceiling_type", "layers", "thickness"]) },
    ]),
  }),
  leaf({
    kind: "delete-ceiling-type", emoji: 0x1f38f, variant: "DeleteCeilingType", verb: "delete", entity: "ceiling-type", binaryTag: 6001, displayName: "Delete Ceiling Type",
    doc: "Removes a ceiling type that no ceiling uses.",
    props: [id("ceiling-type", "target", typeLabel)],
    label: { en: 'format!("Delete ceiling type \\"{}\\"", self.id)', de: 'format!("Unterdeckentyp \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: withSpareType(), mutation: { id: "ct-spare" }, outcome: ok },
      { name: "used-by-ceilings", before: withCeiling(), mutation: { id: "ct-board" }, outcome: reject("mutation.target-referenced", ["ct-board"]) },
      { name: "missing", before: library(), mutation: { id: "ct-gone" }, outcome: reject("mutation.target-missing", ["ct-gone"]) },
    ]),
  }),
  leaf({
    kind: "set-ceiling-type", emoji: 0x1f390, variant: "SetCeilingType", verb: "set", entity: "ceiling-type", binaryTag: 6002, displayName: "Set Ceiling Type",
    doc: "Patches exactly the provided fields of a ceiling type; the layer stack is one field and replaces the whole stack.",
    props: [id("ceiling-type", "target", typeLabel), sparse(text("name", { en: "Name", de: "Name" }, 20), keep), sparse(layersProp(30), keep)],
    label: { en: 'format!("Edit ceiling type \\"{}\\"", self.id)', de: 'format!("Unterdeckentyp \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: library(), mutation: { id: "ct-board", name: "Plasterboard 12.5 (renamed)" }, outcome: ok },
      { name: "restacks", before: library(), mutation: { id: "ct-board", layers: [layer("m-board", 0.025, "Finish"), layer("m-brick", 0.04, "Insulation")] }, outcome: ok },
      { name: "both-fields", before: library(), mutation: { id: "ct-board", name: "Double board", layers: [layer("m-board", 0.025, "Finish")] }, outcome: ok },
      { name: "restates-an-unchanged-field", before: library(), mutation: { id: "ct-board", name: "Plasterboard 12.5", layers: [layer("m-board", 0.025, "Finish")] }, outcome: ok },
      { name: "empty-patch", before: library(), mutation: { id: "ct-board" }, outcome: reject("mutation.no-op", ["ct-board"]) },
      { name: "empty-layers", before: library(), mutation: { id: "ct-board", layers: [] }, outcome: reject("mutation.invariant", ["layers"]) },
      { name: "thin-layer", before: library(), mutation: { id: "ct-board", layers: [layer("m-board", -0.1)] }, outcome: reject("mutation.invariant", ["layers", "thickness"]) },
      { name: "material-missing", before: library(), mutation: { id: "ct-board", layers: [layer("m-ghost", 0.02)] }, outcome: reject("mutation.target-missing", ["layers", "material"]) },
      { name: "missing", before: library(), mutation: { id: "ct-gone", name: "Ghost" }, outcome: reject("mutation.target-missing", ["ct-gone"]) },
    ]),
  }),
];

const ceilingLabel = { en: "Ceiling", de: "Unterdecke" };
const kindsOfCeilings: MineLeaf[] = [
  leaf({
    kind: "create-ceiling", emoji: 0x1f3de, variant: "CreateCeiling", verb: "create", entity: "ceiling", binaryTag: 6003, displayName: "Create Ceiling",
    doc: "Brings a new ceiling onto a storey; its boundary and holes are closed counter-clockwise loops, area, solid and the clear height of the rooms below are inferred.",
    props: [id("ceiling", "identity", { en: "Ceiling id", de: "Unterdecken-Id" }, 10), recordProp("ceiling", "Ceiling", ceilingLabel)],
    label: { en: 'format!("Create ceiling \\"{}\\"", self.ceiling.name)', de: 'format!("Unterdecke \\"{}\\" anlegen", self.ceiling.name)' },
    target,
    cases: cases([
      { name: "adds-a-ceiling-with-a-hole", before: library(), mutation: { id: "ce-living", ceiling: LIVING() }, outcome: ok },
      { name: "adds-a-sloped-curved-ceiling", before: library(), mutation: { id: "ce-attic", ceiling: ceiling("st-first", "ct-tile", [V(0, 0), V(6, 0, 0.4), V(6, 6), V(0, 6)], [], 0.1, "Attic ceiling", { direction: 0, angle: 0.05 }) }, outcome: ok },
      { name: "duplicate", before: withCeiling(), mutation: { id: "ce-living", ceiling: LIVING() }, outcome: reject("mutation.duplicate-id", ["ce-living"]) },
      { name: "storey-missing", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-attic", "ct-board", rect(0, 0, 8, 6), [], 0.3, "Living ceiling") }, outcome: reject("mutation.target-missing", ["ceiling", "storey"]) },
      { name: "type-missing", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-missing", rect(0, 0, 8, 6), [], 0.3, "Living ceiling") }, outcome: reject("mutation.target-missing", ["ceiling", "ceiling_type"]) },
      { name: "too-few-vertices", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-board", ring([[0, 0], [8, 0]]), [], 0.3, "Living ceiling") }, outcome: reject("mutation.invariant", ["ceiling", "boundary"]) },
      { name: "zero-area", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-board", ring([[0, 0], [2, 0], [4, 0]]), [], 0.3, "Living ceiling") }, outcome: reject("mutation.invariant", ["ceiling", "boundary"]) },
      { name: "self-intersecting", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-board", bowTie, [], 0.3, "Living ceiling") }, outcome: reject("mutation.invariant", ["ceiling", "boundary"]) },
      { name: "clockwise", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-board", clockwise, [], 0.3, "Living ceiling") }, outcome: reject("mutation.invariant", ["ceiling", "boundary"]) },
      { name: "hole-outside", before: library(), mutation: { id: "ce-living", ceiling: ceiling("st-ground", "ct-board", rect(0, 0, 8, 6), [rect(7, 2, 9, 3)], 0.3, "Living ceiling") }, outcome: reject("mutation.invariant", ["ceiling", "holes"]) },
    ]),
  }),
  leaf({
    kind: "delete-ceiling", emoji: 0x1f306, variant: "DeleteCeiling", verb: "delete", entity: "ceiling", binaryTag: 6004, displayName: "Delete Ceiling",
    doc: "Removes a ceiling together with its properties and classifications.",
    props: [id("ceiling", "target", ceilingLabel)],
    label: { en: 'format!("Delete ceiling \\"{}\\"", self.id)', de: 'format!("Unterdecke \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withCeiling(), mutation: { id: "ce-living" }, outcome: ok },
      { name: "removes-its-data", before: withData(), mutation: { id: "ce-living" }, outcome: ok },
      { name: "missing", before: withCeiling(), mutation: { id: "ce-attic" }, outcome: reject("mutation.target-missing", ["ce-attic"]) },
    ]),
  }),
  leaf({
    kind: "set-ceiling-boundary", emoji: 0x1f307, variant: "SetCeilingBoundary", verb: "set", entity: "ceiling", binaryTag: 6005, displayName: "Set Ceiling Boundary",
    doc: "Replaces the outline of a ceiling as one semantic unit: its boundary loop and its hole loops, validated together.",
    props: [id("ceiling", "target", ceilingLabel), loopProp("boundary", { en: "Boundary", de: "Umriss" }, 20), holesProp(30)],
    label: { en: 'format!("Reshape the outline of ceiling \\"{}\\"", self.id)', de: 'format!("Umriss der Unterdecke \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "reshapes", before: withCeiling(), mutation: { id: "ce-living", boundary: rect(0, 0, 9, 6), holes: [rect(3, 2, 4, 3)] }, outcome: ok },
      { name: "drops-the-hole", before: withCeiling(), mutation: { id: "ce-living", boundary: rect(0, 0, 8, 6), holes: [] }, outcome: ok },
      { name: "unchanged", before: withCeiling(), mutation: { id: "ce-living", boundary: rect(0, 0, 8, 6), holes: [rect(3, 2, 4, 3)] }, outcome: reject("mutation.no-op", ["ce-living"]) },
      { name: "hole-outside", before: withCeiling(), mutation: { id: "ce-living", boundary: rect(0, 0, 8, 6), holes: [rect(7, 2, 9, 3)] }, outcome: reject("mutation.invariant", ["holes"]) },
      { name: "overlapping-holes", before: withCeiling(), mutation: { id: "ce-living", boundary: rect(0, 0, 8, 6), holes: [rect(3, 2, 4, 3), rect(3.5, 2.5, 4.5, 3.5)] }, outcome: reject("mutation.invariant", ["holes"]) },
      { name: "self-intersecting", before: withCeiling(), mutation: { id: "ce-living", boundary: bowTie, holes: [] }, outcome: reject("mutation.invariant", ["boundary"]) },
      { name: "missing", before: withCeiling(), mutation: { id: "ce-attic", boundary: rect(0, 0, 8, 6), holes: [] }, outcome: reject("mutation.target-missing", ["ce-attic"]) },
    ]),
  }),
  leaf({
    kind: "set-ceiling", emoji: 0x1f304, variant: "SetCeiling", verb: "set", entity: "ceiling", binaryTag: 6006, displayName: "Set Ceiling",
    doc: "Sets exactly the provided fields of a ceiling: type, drop below the storey top, slope (set or cleared) and name; absent fields stay untouched.",
    props: [
      id("ceiling", "target", ceilingLabel),
      sparse(reference("ceiling_type", "ceiling-type", typeLabel, 20), keep),
      sparse(length("offset", { en: "Drop (m)", de: "Abhängung (m)" }, 30), keep),
      sparse(slopeProp(40), keep),
      sparse(text("name", { en: "Name", de: "Name" }, 50), keep),
    ],
    label: { en: 'format!("Change ceiling \\"{}\\"", self.id)', de: 'format!("Unterdecke \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "retypes-and-drops", before: withCeiling(), mutation: { id: "ce-living", ceiling_type: "ct-tile", offset: 0.5 }, outcome: ok },
      { name: "slopes", before: withCeiling(), mutation: { id: "ce-living", slope: { value: { direction: 1.57, angle: 0.05 } } }, outcome: ok },
      { name: "clears-the-slope", before: withSloped(), mutation: { id: "ce-living", slope: { value: null } }, outcome: ok },
      { name: "renames", before: withCeiling(), mutation: { id: "ce-living", name: "Lounge ceiling" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withCeiling(), mutation: { id: "ce-living", offset: 0.5, name: "Living ceiling" }, outcome: ok },
      { name: "unchanged", before: withCeiling(), mutation: { id: "ce-living", name: "Living ceiling", offset: 0.3 }, outcome: reject("mutation.no-op", ["ce-living"]) },
      { name: "names-no-field", before: withCeiling(), mutation: { id: "ce-living" }, outcome: reject("mutation.no-op", ["ce-living"]) },
      { name: "type-missing", before: withCeiling(), mutation: { id: "ce-living", ceiling_type: "ct-missing" }, outcome: reject("mutation.target-missing", ["ceiling_type"]) },
      { name: "slope-too-steep", before: withCeiling(), mutation: { id: "ce-living", slope: { value: { direction: 0, angle: 1.6 } } }, outcome: reject("mutation.invariant", ["slope"]) },
      { name: "missing", before: withCeiling(), mutation: { id: "ce-attic", name: "Attic ceiling" }, outcome: reject("mutation.target-missing", ["ce-attic"]) },
    ]),
  }),
];

export const leaves: MineLeaf[] = [...kindsOfTypes, ...kindsOfCeilings];

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const SCHEMA = em(0x1f9ec);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...values: string[]) =>
  strings.raw
    .reduce((out, chunk, index) => out + chunk + (values[index] ?? ""), "")
    .replace(/\\u\{([0-9a-fA-F]+)\}/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/\\u([0-9a-fA-F]{4})/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replaceAll("¶", "`")
    .replaceAll('\\\\"', '\\"');

const TYPE_FAULT = `fn fault(layers: &[Layer], base: &ModelSnapshot) -> Option<(OutcomeCode, String, Option<&'static str>)> {
    if let Some(layer) = layers.iter().find(|layer| !base.materials.contains_key(&layer.material)) {
        return Some((OutcomeCode::TargetMissing, format!("Material \\"{}\\" does not exist.", layer.material), Some("material")));
    }
    if layers.is_empty() {
        return Some((OutcomeCode::Invariant, "A ceiling type needs at least one layer.".to_string(), None));
    }
    if layers.iter().any(|layer| !(layer.thickness.is_finite() && layer.thickness > 0.0)) {
        return Some((OutcomeCode::Invariant, "Every layer needs a positive thickness.".to_string(), Some("thickness")));
    }
    None
}`;

const HAND: Record<string, { diff: string; inverse: string }> = {
  "create-ceiling-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateCeilingType¶: one created ceiling type entry. Every layer names an existing material and has a positive finite
//! thickness; a ceiling type has at least one layer.

use super::super::elements;
use super::CreateCeilingType;
use crate::{Entry, Layer, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

${TYPE_FAULT}

pub fn diff(payload: &CreateCeilingType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some((code, message, field)) = fault(&payload.ceiling_type.layers, base) {
        return MutationOutcome::refuse(code, message, ["ceiling_type", "layers"].into_iter().chain(field));
    }
    MutationOutcome::new(ModelDiff::ceiling_types(payload.id.clone(), Entry::Created(payload.ceiling_type.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateCeilingType¶: the concrete ¶DeleteCeilingType¶ of the id it created, none when the id was already taken.

use super::super::delete_ceiling_type::DeleteCeilingType;
use super::CreateCeilingType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.ceiling_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCeilingType(DeleteCeilingType { id: payload.id.clone() })]
}
`,
  },
  "delete-ceiling-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteCeilingType¶: one deleted ceiling type entry; refused while a ceiling still uses the type.

use super::DeleteCeilingType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCeilingType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ceiling_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.ceilings.values().any(|row| row.ceiling_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Ceiling type \\"{}\\" is still used by ceilings.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceiling_types(payload.id.clone(), Entry::Deleted))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteCeilingType¶: the concrete ¶CreateCeilingType¶ carrying the full removed record, none when the ceiling type was absent.

use super::super::create_ceiling_type::CreateCeilingType;
use super::DeleteCeilingType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.ceiling_types.get(&payload.id) {
        Some(ceiling_type) => vec![ModelMutation::CreateCeilingType(CreateCeilingType { id: payload.id.clone(), ceiling_type: ceiling_type.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-ceiling-type": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCeilingType¶: a sparse ceiling type patch of exactly the provided fields. A provided layer stack replaces the whole
//! stack: every layer names an existing material and has a positive finite thickness, and the stack is not empty. A patch that changes nothing is a no-op.
//! Whole-list ruling: the layer stack is ONE owned value, an ordered build-up whose order and thicknesses only mean something
//! together; its layers have no identity and nothing references one, so a provided stack replaces the stack as one field.

use super::SetCeilingType;
use crate::{Entry, Layer, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

${TYPE_FAULT}

pub fn diff(payload: &SetCeilingType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(current) = base.ceiling_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some((code, message, field)) = payload.layers.as_deref().and_then(|layers| fault(layers, base)) {
        return MutationOutcome::refuse(code, message, ["layers"].into_iter().chain(field));
    }
    let change = payload.patch().minimal(current);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ceiling type \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceiling_types(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetCeilingType¶: an absolute ¶SetCeilingType¶ restoring the base value of exactly the fields the forward really changes, none when the ceiling type is absent or nothing changes.

use super::SetCeilingType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetCeilingType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.ceiling_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCeilingType(SetCeilingType::from_patch(payload.id.clone(), restore))]
}
`,
  },
  "create-ceiling": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateCeiling¶: one created ceiling entry. The storey and the ceiling type must exist, the boundary and every hole
//! are valid counter-clockwise loops with the holes strictly inside, the drop is finite and a slope stays within [0, 89 degrees).
//! Area, volume, the sloped solid and the clear height of the rooms below are inferred.

use super::super::elements;
use super::super::horizontal_rules::{holes_fault, loop_fault, slope_fault};
use super::CreateCeiling;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let ceiling = &payload.ceiling;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&ceiling.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \\"{}\\" does not exist.", ceiling.storey), ["ceiling", "storey"]);
    }
    if !base.ceiling_types.contains_key(&ceiling.ceiling_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \\"{}\\" does not exist.", ceiling.ceiling_type), ["ceiling", "ceiling_type"]);
    }
    if let Some(fault) = loop_fault(&ceiling.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "boundary"]);
    }
    if let Some(fault) = holes_fault(&ceiling.boundary, &ceiling.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "holes"]);
    }
    if !ceiling.offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A ceiling drop must be a finite length.", ["ceiling", "offset"]);
    }
    if let Some(fault) = ceiling.slope.as_ref().and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "slope"]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Created(ceiling.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateCeiling¶: the concrete ¶DeleteCeiling¶ of the id it created, none when the id was already taken.

use super::super::delete_ceiling::DeleteCeiling;
use super::CreateCeiling;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCeiling, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.ceilings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCeiling(DeleteCeiling { id: payload.id.clone() })]
}
`,
  },
  "delete-ceiling": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteCeiling¶: the ceiling leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the ceiling leaves.

use super::super::cascade;
use super::DeleteCeiling;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ceilings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Ceiling", Some(&payload.id))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteCeiling¶: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::DeleteCeiling;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteCeiling, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
  "set-ceiling-boundary": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCeilingBoundary¶: the boundary and the holes are validated together as one outline and patched as far
//! as they differ. Both loops follow the create rules (counter-clockwise, no self-intersection, holes strictly inside); an outline
//! equal to the current one is a no-op.
//! Whole-list ruling: the boundary and the holes are ONE planar region that only validates together (every hole strictly inside
//! the boundary); holes have no identity, so the region replaces as the boundary and holes it names.

use super::super::horizontal_rules::{holes_fault, loop_fault};
use super::SetCeilingBoundary;
use crate::{CeilingPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCeilingBoundary, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(ceiling) = base.ceilings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["boundary"]);
    }
    if let Some(fault) = holes_fault(&payload.boundary, &payload.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["holes"]);
    }
    let patch = CeilingPatch {
        boundary: (payload.boundary != ceiling.boundary).then(|| payload.boundary.clone()),
        holes: (payload.holes != ceiling.holes).then(|| payload.holes.clone()),
        ..Default::default()
    };
    if patch == CeilingPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ceiling \\"{}\\" already has this outline.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Patched(patch)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetCeilingBoundary¶: an absolute ¶SetCeilingBoundary¶ back to the base boundary and holes, none when the ceiling is absent.

use super::SetCeilingBoundary;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCeilingBoundary, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.ceilings.get(&payload.id) {
        Some(ceiling) => vec![ModelMutation::SetCeilingBoundary(SetCeilingBoundary { id: payload.id.clone(), boundary: ceiling.boundary.clone(), holes: ceiling.holes.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-ceiling": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetCeiling¶: a sparse ceiling patch of exactly the provided fields that differ. A new ceiling type must exist, the
//! drop is finite and a slope (set or cleared) stays within [0, 89 degrees); providing only equal values, or no field, is a no-op.

use super::super::horizontal_rules::slope_fault;
use super::SetCeiling;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(ceiling) = base.ceilings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(ceiling_type) = payload.ceiling_type.as_ref().filter(|ceiling_type| !base.ceiling_types.contains_key(*ceiling_type)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \\"{ceiling_type}\\" does not exist."), ["ceiling_type"]);
    }
    if payload.offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A ceiling drop must be a finite length.", ["offset"]);
    }
    if let Some(fault) = payload.slope.as_ref().and_then(|slope| slope.value.as_ref()).and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slope"]);
    }
    let change = payload.patch().minimal(ceiling);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ceiling \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetCeiling¶: an absolute ¶SetCeiling¶ restoring the base value of exactly the fields the forward really changes, none when the ceiling is absent or nothing changes.

use super::SetCeiling;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetCeiling, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.ceilings.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetCeiling(SetCeiling::from_patch(payload.id.clone(), restore))]
}
`,
  },
};

const PATCHES: Record<string, { patch: string; fromPatch: string; patchType: string; fields: string[] }> = {
  "set-ceiling-type": { patchType: "CeilingTypePatch", fields: ["name", "layers"], patch: "CeilingTypePatch { name: self.name.clone(), layers: self.layers.clone() }", fromPatch: "Self { id, name: patch.name, layers: patch.layers }" },
  "set-ceiling": { patchType: "CeilingPatch", fields: ["ceiling_type", "offset", "slope", "name"], patch: "CeilingPatch { ceiling_type: self.ceiling_type.clone(), offset: self.offset, slope: self.slope.clone(), name: self.name.clone(), ..Default::default() }", fromPatch: "Self { id, ceiling_type: patch.ceiling_type, offset: patch.offset, slope: patch.slope, name: patch.name }" },
};

function fixLeaf(spec: MineLeaf) {
  const optionals = extras.get(spec.kind)!.optional;
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const file = join(dir, MUTATION + "mutation", RS);
  const hand = PATCHES[spec.kind];
  const used = new Set(spec.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  if (hand) used.add(hand.patchType);
  let source = readFileSync(file, "utf8");
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) {
    const attr = spec.props.find((prop) => prop.name === name)!.rust.startsWith("Option<Assigned") ? '#[value(default, skip_serializing_if = "Option::is_none")]' : '#[value(skip_serializing_if = "Option::is_none")]';
    source = source.replace(new RegExp(`^    pub ${name}: `, "m"), `    ${attr}\n    pub ${name}: `);
  }
  if (hand) {
    const doc = (text: string) => text;
    source = source.replace(
      /^impl MutationKind/m,
      `impl ${spec.variant} {\n    ${doc("/// 🩹 The sparse entity patch this payload names: every provided field, restated values included.")}\n    pub fn patch(&self) -> ${hand.patchType} {\n        ${hand.patch}\n    }\n\n    ${doc("/// 🧩 The payload that provides exactly the fields `patch` names.")}\n    pub fn from_patch(id: String, patch: ${hand.patchType}) -> Self {\n        ${hand.fromPatch}\n    }\n}\n\nimpl MutationKind`,
    );
  }
  writeFileSync(file, source);
  if (optionals.length > 0) {
    const schemaFile = join(dir, readdirName(dir, "schema"), JSONF);
    const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
    schema.required = schema.required.filter((name: string) => !optionals.includes(name));
    writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  }
}

import { readdirSync } from "node:fs";
function readdirName(dir: string, suffix: string) {
  return readdirSync(dir).find((name) => name.endsWith(suffix))!;
}

function writeOnce(spec: MineLeaf) {
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  for (const [folder, body] of [[DIFF + "diff", HAND[spec.kind].diff], [INVERSE + "inverse", HAND[spec.kind].inverse]] as const) {
    mkdirSync(join(dir, folder), { recursive: true });
    const file = join(dir, folder, RS);
    if (!existsSync(file)) writeFileSync(file, body);
  }
}
//#endregion 🔖️Hand

//#region 🔖️Run
const rootPath = join(artifact, RS);
const mounts: string[] = [];
for (const spec of leaves) {
  mounts.push(emitLeaf(spec as unknown as Leaf));
  fixLeaf(spec);
  writeOnce(spec);
}
let root = readFileSync(rootPath, "utf8");
const crlf = root.includes("\r\n");
if (crlf) root = root.replaceAll("\r\n", "\n");
const anchor = "                        //#endregion 🔖️Leaves";
let added = 0;
for (const [index, spec] of leaves.entries()) {
  if (root.includes(`pub mod ${snake(spec.kind)} {`)) continue;
  root = root.replace(anchor, mounts[index] + anchor);
  added += 1;
}
writeFileSync(rootPath, crlf ? root.replaceAll("\n", "\r\n") : root);
console.log(`w06-ceilings: ${leaves.length} leaves emitted, ${added} newly mounted`);
//#endregion 🔖️Run
