#!/usr/bin/env bun
/**
 * 🏗️ Wave M slice 6 (`m-horizontal`): the eight slab and roof leaves of `s.bim.model@1` (create-slab, delete-slab, set-slab-boundary,
 * set-slab, create-roof, delete-roof, set-roof-footprint, set-roof-shape; binary tags 600..607) plus the shared rule module
 * `📐️horizontal-rules`. `bun r3-m-horizontal-leaves.ts` is idempotent: it rewrites the boilerplate and fixtures through `emitLeaf`,
 * patches the sparse (optional) payload fields, writes the hand-authored `🔺️diff` and `↩️inverse` sources and the rule module, and
 * mounts the leaves once in the artifact root. `after`/`diff` fixtures of applied cases are never overwritten: bless them with
 * `BIM_BLESS=1 cargo test`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, rel, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
type Mine = Prop & { optional?: true };
type MineLeaf = Omit<Leaf, "props"> & { props: Mine[]; names?: string[] };

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
const loopProp = (name: string, label: Label, order: number): Mine => ({ name, rust: "Vec<Vertex>", schema: { type: "array", items: record("Vertex") }, ui: { widget: "record", role: "value", label, group: "outline", order } });
const holesProp = (order: number): Mine => ({ name: "holes", rust: "Vec<Vec<Vertex>>", schema: { type: "array", items: { type: "array", items: record("Vertex") } }, ui: { widget: "record", role: "value", label: { en: "Holes", de: "Öffnungen" }, group: "outline", order } });
const slopeProp = (order: number): Mine => ({
  name: "slope",
  rust: "Assigned<Option<Slope>>",
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [{ type: "null" }, record("Slope")] } } },
  ui: { widget: "record", role: "value", label: { en: "Slope", de: "Neigung" }, group: "value", order },
});
const shapeProp = (order: number): Mine => ({ name: "shape", rust: "RoofShape", schema: record("RoofShape"), ui: { widget: "record", role: "value", label: { en: "Shape", de: "Dachform" }, group: "shape", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d];
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
const slab = (storey: string, type: string, boundary: unknown, holes: unknown[], offset: number, label: string, slope?: { direction: number; angle: number }) => ({ storey, slab_type: type, boundary, holes, offset, ...(slope ? { slope } : {}), name: label });
const roof = (storey: string, type: string, footprint: unknown, shape: unknown, overhang: number, baseOffset: number, label: string) => ({ storey, roof_type: type, footprint, shape, overhang, base_offset: baseOffset, name: label });
const gable = (pitch: number, ridge_direction = 0) => ({ Gable: { pitch, ridge_direction } });
const bowTie = ring([[0, 0], [6, 4], [6, 0], [0, 4]]);
const clockwise = ring([[0, 0], [0, 6], [8, 6], [8, 0]]);

const shell = (extra: { slabs?: Record<string, unknown>; roofs?: Record<string, unknown> } = {}) => ({
  ...F.scene(),
  materials: { "m-brick": F.material("Brick"), "m-concrete": F.material("Concrete") },
  slab_types: { "stt-22": { name: "Concrete 22", layers: [F.layer("m-concrete", 0.22)] }, "stt-30": { name: "Concrete 30", layers: [F.layer("m-concrete", 0.3)] } },
  roof_types: { "rt-tiles": { name: "Tiled roof", layers: [F.layer("m-brick", 0.05, "Finish")] } },
  slabs: extra.slabs ?? {},
  roofs: extra.roofs ?? {},
});
const GROUND_SLAB = () => slab("st-ground", "stt-22", rect(0, 0, 8, 6), [rect(3, 2, 4, 3)], 0, "Ground slab");
const SLOPED_SLAB = () => slab("st-ground", "stt-22", rect(0, 0, 8, 6), [rect(3, 2, 4, 3)], 0, "Ground slab", { direction: 0, angle: 0.05 });
const withSlab = () => shell({ slabs: { "sl-ground": GROUND_SLAB() } });
const withSlopedSlab = () => shell({ slabs: { "sl-ground": SLOPED_SLAB() } });
const MAIN_ROOF = () => roof("st-first", "rt-tiles", rect(0, 0, 8, 6), gable(0.6), 0.4, 0, "Main roof");
const withRoof = () => shell({ roofs: { "rf-main": MAIN_ROOF() } });

type Extra = { imports: string[]; optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf, extra: Extra = { imports: [], optional: [] }): MineLeaf => (extras.set(spec.kind, { imports: [...extra.imports, ...(spec.names ?? [])], optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const slabEn = (verb: string) => `format!("${verb} slab \\"{}\\"", self.id)`;
const roofEn = (verb: string) => `format!("${verb} roof \\"{}\\"", self.id)`;

export const leaves: MineLeaf[] = [
  leaf({
    kind: "create-slab", emoji: 0x2b1c, variant: "CreateSlab", verb: "create", entity: "slab", binaryTag: 600, displayName: "Create Slab",
    doc: "Brings a new slab onto a storey; its boundary and holes are closed counter-clockwise loops, area and solid are inferred.",
    props: [id("slab", "identity", { en: "Slab id", de: "Decken-Id" }, 10), recordProp("slab", "Slab", { en: "Slab", de: "Decke" })],
    label: { en: 'format!("Create slab \\"{}\\"", self.slab.name)', de: 'format!("Decke \\"{}\\" anlegen", self.slab.name)' },
    target,
    cases: cases([
      { name: "adds-a-slab-with-a-hole", before: shell(), mutation: { id: "sl-ground", slab: GROUND_SLAB() }, outcome: ok },
      { name: "adds-a-sloped-curved-slab", before: shell(), mutation: { id: "sl-terrace", slab: slab("st-first", "stt-22", [V(0, 0), V(6, 0, 0.4), V(6, 6), V(0, 6)], [], -0.1, "Terrace", { direction: 0, angle: 0.05 }) }, outcome: ok },
      { name: "duplicate", before: withSlab(), mutation: { id: "sl-ground", slab: GROUND_SLAB() }, outcome: reject("mutation.duplicate-id", ["sl-ground"]) },
      { name: "storey-missing", before: shell(), mutation: { id: "sl-ground", slab: slab("st-attic", "stt-22", rect(0, 0, 8, 6), [], 0, "Ground slab") }, outcome: reject("mutation.target-missing", ["slab", "storey"]) },
      { name: "type-missing", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-missing", rect(0, 0, 8, 6), [], 0, "Ground slab") }, outcome: reject("mutation.target-missing", ["slab", "slab_type"]) },
      { name: "too-few-vertices", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-22", ring([[0, 0], [8, 0]]), [], 0, "Ground slab") }, outcome: reject("mutation.invariant", ["slab", "boundary"]) },
      { name: "zero-area", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-22", ring([[0, 0], [2, 0], [4, 0]]), [], 0, "Ground slab") }, outcome: reject("mutation.invariant", ["slab", "boundary"]) },
      { name: "self-intersecting", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-22", bowTie, [], 0, "Ground slab") }, outcome: reject("mutation.invariant", ["slab", "boundary"]) },
      { name: "clockwise", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-22", clockwise, [], 0, "Ground slab") }, outcome: reject("mutation.invariant", ["slab", "boundary"]) },
      { name: "hole-outside", before: shell(), mutation: { id: "sl-ground", slab: slab("st-ground", "stt-22", rect(0, 0, 8, 6), [rect(7, 2, 9, 3)], 0, "Ground slab") }, outcome: reject("mutation.invariant", ["slab", "holes"]) },
    ]),
  }),
  leaf({
    kind: "delete-slab", emoji: 0x1f53b, variant: "DeleteSlab", verb: "delete", entity: "slab", binaryTag: 601, displayName: "Delete Slab",
    doc: "Removes a slab; nothing references a slab, so nothing blocks or cascades.",
    props: [id("slab", "target", { en: "Slab", de: "Decke" })],
    label: { en: slabEn("Delete"), de: 'format!("Decke \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: withSlab(), mutation: { id: "sl-ground" }, outcome: ok },
      { name: "missing", before: withSlab(), mutation: { id: "sl-attic" }, outcome: reject("mutation.target-missing", ["sl-attic"]) },
    ]),
  }),
  leaf({
    kind: "set-slab-boundary", emoji: 0x1f537, variant: "SetSlabBoundary", verb: "set", entity: "slab", binaryTag: 602, displayName: "Set Slab Boundary",
    doc: "Replaces the outline of a slab as one semantic unit: its boundary loop and its hole loops, validated together.",
    props: [id("slab", "target", { en: "Slab", de: "Decke" }), loopProp("boundary", { en: "Boundary", de: "Umriss" }, 20), holesProp(30)],
    label: { en: 'format!("Reshape the outline of slab \\"{}\\"", self.id)', de: 'format!("Umriss der Decke \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "reshapes", before: withSlab(), mutation: { id: "sl-ground", boundary: rect(0, 0, 9, 6), holes: [rect(3, 2, 4, 3)] }, outcome: ok },
      { name: "drops-the-hole", before: withSlab(), mutation: { id: "sl-ground", boundary: rect(0, 0, 8, 6), holes: [] }, outcome: ok },
      { name: "unchanged", before: withSlab(), mutation: { id: "sl-ground", boundary: rect(0, 0, 8, 6), holes: [rect(3, 2, 4, 3)] }, outcome: reject("mutation.no-op", ["sl-ground"]) },
      { name: "hole-outside", before: withSlab(), mutation: { id: "sl-ground", boundary: rect(0, 0, 8, 6), holes: [rect(7, 2, 9, 3)] }, outcome: reject("mutation.invariant", ["holes"]) },
      { name: "overlapping-holes", before: withSlab(), mutation: { id: "sl-ground", boundary: rect(0, 0, 8, 6), holes: [rect(3, 2, 4, 3), rect(3.5, 2.5, 4.5, 3.5)] }, outcome: reject("mutation.invariant", ["holes"]) },
      { name: "self-intersecting", before: withSlab(), mutation: { id: "sl-ground", boundary: bowTie, holes: [] }, outcome: reject("mutation.invariant", ["boundary"]) },
      { name: "missing", before: withSlab(), mutation: { id: "sl-attic", boundary: rect(0, 0, 8, 6), holes: [] }, outcome: reject("mutation.target-missing", ["sl-attic"]) },
    ]),
  }, { imports: ["Vertex"], optional: [] }),
  leaf({
    kind: "set-slab", emoji: 0x1f538, variant: "SetSlab", verb: "set", entity: "slab", binaryTag: 603, displayName: "Set Slab",
    doc: "Sets exactly the provided fields of a slab: type, offset, slope (set or cleared) and name; absent fields stay untouched.",
    props: [
      id("slab", "target", { en: "Slab", de: "Decke" }),
      sparse(reference("slab_type", "slab-type", { en: "Slab type", de: "Deckentyp" }, 20), keep),
      sparse(length("offset", { en: "Offset (m)", de: "Versatz (m)" }, 30), keep),
      sparse(slopeProp(40), keep),
      sparse(text("name", { en: "Name", de: "Name" }, 50), keep),
    ],
    names: ["Assigned", "Slope"],
    label: { en: 'format!("Change slab \\"{}\\"", self.id)', de: 'format!("Decke \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "retypes-and-offsets", before: withSlab(), mutation: { id: "sl-ground", slab_type: "stt-30", offset: -0.3 }, outcome: ok },
      { name: "slopes", before: withSlab(), mutation: { id: "sl-ground", slope: { value: { direction: 1.57, angle: 0.05 } } }, outcome: ok },
      { name: "clears-the-slope", before: withSlopedSlab(), mutation: { id: "sl-ground", slope: { value: null } }, outcome: ok },
      { name: "renames", before: withSlab(), mutation: { id: "sl-ground", name: "Basement slab" }, outcome: ok },
      { name: "unchanged", before: withSlab(), mutation: { id: "sl-ground", name: "Ground slab", offset: 0 }, outcome: reject("mutation.no-op", ["sl-ground"]) },
      { name: "names-no-field", before: withSlab(), mutation: { id: "sl-ground" }, outcome: reject("mutation.no-op", ["sl-ground"]) },
      { name: "type-missing", before: withSlab(), mutation: { id: "sl-ground", slab_type: "stt-missing" }, outcome: reject("mutation.target-missing", ["slab_type"]) },
      { name: "slope-too-steep", before: withSlab(), mutation: { id: "sl-ground", slope: { value: { direction: 0, angle: 1.6 } } }, outcome: reject("mutation.invariant", ["slope"]) },
      { name: "missing", before: withSlab(), mutation: { id: "sl-attic", name: "Attic slab" }, outcome: reject("mutation.target-missing", ["sl-attic"]) },
    ]),
  }),
  leaf({
    kind: "create-roof", emoji: 0x1f3e0, variant: "CreateRoof", verb: "create", entity: "roof", binaryTag: 604, displayName: "Create Roof",
    doc: "Brings a new roof onto a storey; footprint is a closed counter-clockwise loop, pitched shapes need pitches in (0, 89 degrees); the solid is inferred.",
    props: [id("roof", "identity", { en: "Roof id", de: "Dach-Id" }, 10), recordProp("roof", "Roof", { en: "Roof", de: "Dach" })],
    label: { en: 'format!("Create roof \\"{}\\"", self.roof.name)', de: 'format!("Dach \\"{}\\" anlegen", self.roof.name)' },
    target,
    cases: cases([
      { name: "adds-a-gable", before: shell(), mutation: { id: "rf-main", roof: MAIN_ROOF() }, outcome: ok },
      { name: "adds-a-flat-roof", before: shell(), mutation: { id: "rf-flat", roof: roof("st-first", "rt-tiles", rect(0, 0, 8, 6), "Flat", 0, 0.1, "Flat roof") }, outcome: ok },
      { name: "adds-a-mansard", before: shell(), mutation: { id: "rf-mansard", roof: roof("st-first", "rt-tiles", rect(0, 0, 8, 6), { Mansard: { lower_pitch: 1.2, upper_pitch: 0.4, break_height: 1.5 } }, 0.3, 0, "Mansard roof") }, outcome: ok },
      { name: "duplicate", before: withRoof(), mutation: { id: "rf-main", roof: MAIN_ROOF() }, outcome: reject("mutation.duplicate-id", ["rf-main"]) },
      { name: "storey-missing", before: shell(), mutation: { id: "rf-main", roof: roof("st-attic", "rt-tiles", rect(0, 0, 8, 6), gable(0.6), 0.4, 0, "Main roof") }, outcome: reject("mutation.target-missing", ["roof", "storey"]) },
      { name: "type-missing", before: shell(), mutation: { id: "rf-main", roof: roof("st-first", "rt-missing", rect(0, 0, 8, 6), gable(0.6), 0.4, 0, "Main roof") }, outcome: reject("mutation.target-missing", ["roof", "roof_type"]) },
      { name: "self-intersecting", before: shell(), mutation: { id: "rf-main", roof: roof("st-first", "rt-tiles", bowTie, gable(0.6), 0.4, 0, "Main roof") }, outcome: reject("mutation.invariant", ["roof", "footprint"]) },
      { name: "pitch-zero", before: shell(), mutation: { id: "rf-main", roof: roof("st-first", "rt-tiles", rect(0, 0, 8, 6), gable(0), 0.4, 0, "Main roof") }, outcome: reject("mutation.invariant", ["roof", "shape"]) },
      { name: "pitch-too-steep", before: shell(), mutation: { id: "rf-main", roof: roof("st-first", "rt-tiles", rect(0, 0, 8, 6), { Hip: { pitch: 1.56 } }, 0.4, 0, "Main roof") }, outcome: reject("mutation.invariant", ["roof", "shape"]) },
      { name: "negative-overhang", before: shell(), mutation: { id: "rf-main", roof: roof("st-first", "rt-tiles", rect(0, 0, 8, 6), gable(0.6), -0.2, 0, "Main roof") }, outcome: reject("mutation.invariant", ["roof", "overhang"]) },
    ]),
  }),
  leaf({
    kind: "delete-roof", emoji: 0x1f3d8, variant: "DeleteRoof", verb: "delete", entity: "roof", binaryTag: 605, displayName: "Delete Roof",
    doc: "Removes a roof; nothing references a roof, so nothing blocks or cascades.",
    props: [id("roof", "target", { en: "Roof", de: "Dach" })],
    label: { en: roofEn("Delete"), de: 'format!("Dach \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: withRoof(), mutation: { id: "rf-main" }, outcome: ok },
      { name: "missing", before: withRoof(), mutation: { id: "rf-attic" }, outcome: reject("mutation.target-missing", ["rf-attic"]) },
    ]),
  }),
  leaf({
    kind: "set-roof-footprint", emoji: 0x1f463, variant: "SetRoofFootprint", verb: "set", entity: "roof", binaryTag: 606, displayName: "Set Roof Footprint",
    doc: "Replaces the footprint loop of a roof; the shape, overhang and solid follow by inference.",
    props: [id("roof", "target", { en: "Roof", de: "Dach" }), loopProp("footprint", { en: "Footprint", de: "Grundriss" }, 20)],
    label: { en: 'format!("Reshape the footprint of roof \\"{}\\"", self.id)', de: 'format!("Grundriss des Dachs \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "reshapes", before: withRoof(), mutation: { id: "rf-main", footprint: rect(0, 0, 9, 6) }, outcome: ok },
      { name: "curves-an-edge", before: withRoof(), mutation: { id: "rf-main", footprint: [V(0, 0), V(8, 0, 0.3), V(8, 6), V(0, 6)] }, outcome: ok },
      { name: "unchanged", before: withRoof(), mutation: { id: "rf-main", footprint: rect(0, 0, 8, 6) }, outcome: reject("mutation.no-op", ["rf-main"]) },
      { name: "too-few-vertices", before: withRoof(), mutation: { id: "rf-main", footprint: ring([[0, 0], [8, 0]]) }, outcome: reject("mutation.invariant", ["footprint"]) },
      { name: "self-intersecting", before: withRoof(), mutation: { id: "rf-main", footprint: bowTie }, outcome: reject("mutation.invariant", ["footprint"]) },
      { name: "missing", before: withRoof(), mutation: { id: "rf-attic", footprint: rect(0, 0, 8, 6) }, outcome: reject("mutation.target-missing", ["rf-attic"]) },
    ]),
  }, { imports: ["Vertex"], optional: [] }),
  leaf({
    kind: "set-roof-shape", emoji: 0x1f3d4, variant: "SetRoofShape", verb: "set", entity: "roof", binaryTag: 607, displayName: "Set Roof Shape",
    doc: "Sets exactly the provided fields of a roof's form: shape, overhang and base offset; absent fields stay untouched.",
    props: [
      id("roof", "target", { en: "Roof", de: "Dach" }),
      sparse(shapeProp(20), keep),
      sparse(length("overhang", { en: "Overhang (m)", de: "Überstand (m)" }, 30), keep),
      sparse(length("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 40), keep),
    ],
    names: ["RoofShape"],
    label: { en: 'format!("Change the form of roof \\"{}\\"", self.id)', de: 'format!("Form des Dachs \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "hips-the-roof", before: withRoof(), mutation: { id: "rf-main", shape: { Hip: { pitch: 0.5 } } }, outcome: ok },
      { name: "overhangs-and-lifts", before: withRoof(), mutation: { id: "rf-main", overhang: 0.6, base_offset: 0.1 }, outcome: ok },
      { name: "unchanged", before: withRoof(), mutation: { id: "rf-main", shape: gable(0.6), overhang: 0.4 }, outcome: reject("mutation.no-op", ["rf-main"]) },
      { name: "names-no-field", before: withRoof(), mutation: { id: "rf-main" }, outcome: reject("mutation.no-op", ["rf-main"]) },
      { name: "pitch-too-flat", before: withRoof(), mutation: { id: "rf-main", shape: { Shed: { pitch: 0, direction: 0 } } }, outcome: reject("mutation.invariant", ["shape"]) },
      { name: "negative-overhang", before: withRoof(), mutation: { id: "rf-main", overhang: -0.1 }, outcome: reject("mutation.invariant", ["overhang"]) },
      { name: "missing", before: withRoof(), mutation: { id: "rf-attic", overhang: 0.5 }, outcome: reject("mutation.target-missing", ["rf-attic"]) },
    ]),
  }),
];

const rs = (strings: TemplateStringsArray) =>
  strings.raw
    .join("")
    .replace(/\\u\{([0-9a-fA-F]+)\}/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/\\u([0-9a-fA-F]{4})/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replaceAll("¶", "`");
const MODULES: Record<string, { diff: string; inverse: string }> = {};

const RULES = rs`//! 📐️ Shared validity rules of the horizontal elements (slabs and roofs): closed bulged loops, holes strictly inside their boundary,
//! slope and pitch ranges. Pure checks over authored parameters; each returns the reason a value is refused, none when it is fine.

use crate::{RoofShape, Slope, Vertex};
use semio_framework_geometry::bulge::{intersect, Extent};
use semio_framework_geometry::loops;
use semio_framework_geometry::vector::LENGTH_EPS;
use semio_framework_geometry::Point;
use std::f64::consts::PI;

/// 📐️ Steepest accepted inclination (89 degrees, in radians) of a slab slope or a roof pitch.
pub const MAX_INCLINATION: f64 = 89.0 * PI / 180.0;

fn ring(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn crossing(a: &[loops::Vertex], b: &[loops::Vertex]) -> bool {
    let (left, right) = (loops::segments(a), loops::segments(b));
    left.iter().any(|x| right.iter().any(|y| !intersect(x, y, Extent::Bounded).is_empty()))
}

fn enclosed(outer: &[loops::Vertex], inner: &[loops::Vertex]) -> bool {
    loops::contains(outer, inner[0].point)
}

/// 🔷️ Why a loop cannot bound a horizontal element: fewer than three vertices, non-finite or repeated vertices, self-intersection,
/// zero area or clockwise orientation.
pub fn loop_fault(vertices: &[Vertex]) -> Option<&'static str> {
    if vertices.len() < 3 {
        return Some("A loop needs at least three vertices.");
    }
    if !vertices.iter().all(|vertex| vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite()) {
        return Some("A loop needs finite vertices.");
    }
    let outline = ring(vertices);
    if loops::segments(&outline).iter().any(|segment| segment.chord() <= LENGTH_EPS) {
        return Some("A loop must not repeat a vertex.");
    }
    if !loops::self_intersections(&outline).is_empty() {
        return Some("A loop must not intersect itself.");
    }
    let area = loops::signed_area(&outline);
    if area.abs() <= LENGTH_EPS {
        return Some("A loop must enclose area.");
    }
    if area < 0.0 {
        return Some("A loop must run counter-clockwise.");
    }
    None
}

/// 🕳️ Why the holes cannot pierce the boundary: an invalid hole loop, a hole that touches, crosses or leaves the boundary, or
/// two holes that touch, cross or nest.
pub fn holes_fault(boundary: &[Vertex], holes: &[Vec<Vertex>]) -> Option<String> {
    let outer = ring(boundary);
    let outlines: Vec<Vec<loops::Vertex>> = holes.iter().map(|hole| ring(hole)).collect();
    for (index, hole) in holes.iter().enumerate() {
        if let Some(fault) = loop_fault(hole) {
            return Some(format!("Hole {index}: {fault}"));
        }
        if crossing(&outer, &outlines[index]) || !enclosed(&outer, &outlines[index]) {
            return Some(format!("Hole {index} must lie inside the boundary."));
        }
        for (other, earlier) in outlines.iter().enumerate().take(index) {
            if crossing(earlier, &outlines[index]) || enclosed(earlier, &outlines[index]) || enclosed(&outlines[index], earlier) {
                return Some(format!("Holes {other} and {index} must not overlap."));
            }
        }
    }
    None
}

/// 📐️ Why a slab slope is refused: a non-finite direction or an angle outside [0, 89 degrees).
pub fn slope_fault(slope: &Slope) -> Option<&'static str> {
    let valid = slope.direction.is_finite() && slope.angle.is_finite() && (0.0..MAX_INCLINATION).contains(&slope.angle);
    (!valid).then_some("A slope needs a finite direction and an angle in [0, 89 degrees).")
}

/// 🏠️ Why a roof shape is refused: a pitch outside (0, 89 degrees), a non-finite direction or a non-positive mansard break height.
pub fn shape_fault(shape: &RoofShape) -> Option<&'static str> {
    let pitched = |pitch: f64| pitch > 0.0 && pitch < MAX_INCLINATION;
    let valid = match shape {
        RoofShape::Flat => true,
        RoofShape::Shed { pitch, direction } => pitched(*pitch) && direction.is_finite(),
        RoofShape::Gable { pitch, ridge_direction } => pitched(*pitch) && ridge_direction.is_finite(),
        RoofShape::Hip { pitch } => pitched(*pitch),
        RoofShape::Mansard { lower_pitch, upper_pitch, break_height } => pitched(*lower_pitch) && pitched(*upper_pitch) && break_height.is_finite() && *break_height > 0.0,
    };
    (!valid).then_some("A pitched roof needs pitches in (0, 89 degrees), finite directions and a positive break height.")
}

/// 🏠️ Whether a roof overhang is refused: it must be a finite, non-negative length.
pub fn overhang_fault(overhang: f64) -> Option<&'static str> {
    (!(overhang.is_finite() && overhang >= 0.0)).then_some("A roof overhang must be a finite, non-negative length.")
}
`;

MODULES["create-slab"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶CreateSlab¶: one created slab entry. The storey and the slab type must exist, the boundary and every hole
//! are valid counter-clockwise loops with the holes strictly inside, the offset is finite and a slope stays within [0, 89 degrees).
//! Area, volume and the sloped solid are inferred.

use super::super::horizontal_rules::{holes_fault, loop_fault, slope_fault};
use super::CreateSlab;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let slab = &payload.slab;
    if base.slabs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Slab \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&slab.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", slab.storey), ["slab", "storey"]);
    }
    if !base.slab_types.contains_key(&slab.slab_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{}\" does not exist.", slab.slab_type), ["slab", "slab_type"]);
    }
    if let Some(fault) = loop_fault(&slab.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "boundary"]);
    }
    if let Some(fault) = holes_fault(&slab.boundary, &slab.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "holes"]);
    }
    if !slab.offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A slab offset must be a finite length.", ["slab", "offset"]);
    }
    if let Some(fault) = slab.slope.as_ref().and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "slope"]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Created(slab.clone())))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶CreateSlab¶: the concrete ¶DeleteSlab¶ of the id it created, none when the id was already taken.

use super::super::delete_slab::DeleteSlab;
use super::CreateSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.slabs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSlab(DeleteSlab { id: payload.id.clone() })]
}
`,
};

MODULES["delete-slab"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶DeleteSlab¶: one deleted slab entry. Nothing references a slab, so nothing blocks or cascades.

use super::DeleteSlab;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.slabs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Deleted))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶DeleteSlab¶: the concrete ¶CreateSlab¶ carrying the full removed record, none when the slab was absent.

use super::super::create_slab::CreateSlab;
use super::DeleteSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![ModelMutation::CreateSlab(CreateSlab { id: payload.id.clone(), slab: slab.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-slab-boundary"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶SetSlabBoundary¶: the boundary and the holes are validated together as one outline and patched as far
//! as they differ. Both loops follow the create rules (counter-clockwise, no self-intersection, holes strictly inside); an outline
//! equal to the current one is a no-op.

use super::super::horizontal_rules::{holes_fault, loop_fault};
use super::SetSlabBoundary;
use crate::{Entry, ModelDiff, ModelSnapshot, SlabPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSlabBoundary, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(slab) = base.slabs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["boundary"]);
    }
    if let Some(fault) = holes_fault(&payload.boundary, &payload.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["holes"]);
    }
    let patch = SlabPatch {
        boundary: (payload.boundary != slab.boundary).then(|| payload.boundary.clone()),
        holes: (payload.holes != slab.holes).then(|| payload.holes.clone()),
        ..Default::default()
    };
    if patch == SlabPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Slab \"{}\" already has this outline.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Patched(patch)))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶SetSlabBoundary¶: an absolute ¶SetSlabBoundary¶ back to the base boundary and holes, none when the slab is absent.

use super::SetSlabBoundary;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetSlabBoundary, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![ModelMutation::SetSlabBoundary(SetSlabBoundary { id: payload.id.clone(), boundary: slab.boundary.clone(), holes: slab.holes.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-slab"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶SetSlab¶: a sparse slab patch of exactly the provided fields that differ. A new slab type must exist, the
//! offset is finite and a slope (set or cleared) stays within [0, 89 degrees); providing only equal values, or no field, is a no-op.

use super::super::horizontal_rules::slope_fault;
use super::SetSlab;
use crate::{Entry, ModelDiff, ModelSnapshot, SlabPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(slab) = base.slabs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(slab_type) = payload.slab_type.as_ref().filter(|slab_type| !base.slab_types.contains_key(*slab_type)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{slab_type}\" does not exist."), ["slab_type"]);
    }
    if payload.offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A slab offset must be a finite length.", ["offset"]);
    }
    if let Some(fault) = payload.slope.as_ref().and_then(|slope| slope.value.as_ref()).and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slope"]);
    }
    let patch = SlabPatch {
        slab_type: payload.slab_type.clone().filter(|slab_type| *slab_type != slab.slab_type),
        offset: payload.offset.filter(|offset| *offset != slab.offset),
        slope: payload.slope.clone().filter(|slope| slope.value != slab.slope),
        name: payload.name.clone().filter(|name| *name != slab.name),
        ..Default::default()
    };
    if patch == SlabPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Slab \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Patched(patch)))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶SetSlab¶: an absolute ¶SetSlab¶ carrying the base values of exactly the provided fields, none when the slab is absent.

use super::SetSlab;
use crate::{Assigned, ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![ModelMutation::SetSlab(SetSlab {
            id: payload.id.clone(),
            slab_type: payload.slab_type.as_ref().map(|_| slab.slab_type.clone()),
            offset: payload.offset.map(|_| slab.offset),
            slope: payload.slope.as_ref().map(|_| Assigned::new(slab.slope)),
            name: payload.name.as_ref().map(|_| slab.name.clone()),
        })],
        None => Vec::new(),
    }
}
`,
};

MODULES["create-roof"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶CreateRoof¶: one created roof entry. The storey and the roof type must exist, the footprint is a valid
//! counter-clockwise loop, a pitched shape has pitches in (0, 89 degrees), the overhang is not negative and the base offset is finite.
//! The roof solid is inferred.

use super::super::horizontal_rules::{loop_fault, overhang_fault, shape_fault};
use super::CreateRoof;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateRoof, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let roof = &payload.roof;
    if base.roofs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Roof \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&roof.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", roof.storey), ["roof", "storey"]);
    }
    if !base.roof_types.contains_key(&roof.roof_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", roof.roof_type), ["roof", "roof_type"]);
    }
    if let Some(fault) = loop_fault(&roof.footprint) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "footprint"]);
    }
    if let Some(fault) = shape_fault(&roof.shape) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "shape"]);
    }
    if let Some(fault) = overhang_fault(roof.overhang) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "overhang"]);
    }
    if !roof.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A roof base offset must be a finite length.", ["roof", "base_offset"]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Created(roof.clone())))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶CreateRoof¶: the concrete ¶DeleteRoof¶ of the id it created, none when the id was already taken.

use super::super::delete_roof::DeleteRoof;
use super::CreateRoof;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRoof, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.roofs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRoof(DeleteRoof { id: payload.id.clone() })]
}
`,
};

MODULES["delete-roof"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶DeleteRoof¶: one deleted roof entry. Nothing references a roof, so nothing blocks or cascades.

use super::DeleteRoof;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRoof, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.roofs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Deleted))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶DeleteRoof¶: the concrete ¶CreateRoof¶ carrying the full removed record, none when the roof was absent.

use super::super::create_roof::CreateRoof;
use super::DeleteRoof;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteRoof, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![ModelMutation::CreateRoof(CreateRoof { id: payload.id.clone(), roof: roof.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-roof-footprint"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶SetRoofFootprint¶: a one-field roof patch. The footprint follows the create rules (counter-clockwise, no
//! self-intersection, at least three vertices); a footprint equal to the current one is a no-op. Shape and solid follow by inference.

use super::super::horizontal_rules::loop_fault;
use super::SetRoofFootprint;
use crate::{Entry, ModelDiff, ModelSnapshot, RoofPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRoofFootprint, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(roof) = base.roofs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = loop_fault(&payload.footprint) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["footprint"]);
    }
    if roof.footprint == payload.footprint {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Roof \"{}\" already has this footprint.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Patched(RoofPatch { footprint: Some(payload.footprint.clone()), ..Default::default() })))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶SetRoofFootprint¶: an absolute ¶SetRoofFootprint¶ back to the base footprint, none when the roof is absent.

use super::SetRoofFootprint;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetRoofFootprint, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![ModelMutation::SetRoofFootprint(SetRoofFootprint { id: payload.id.clone(), footprint: roof.footprint.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-roof-shape"] = {
  diff: rs`//! 🔺️ Diff constructor for ¶SetRoofShape¶: a sparse roof patch of exactly the provided fields that differ. A pitched shape has pitches
//! in (0, 89 degrees), the overhang is not negative and the base offset is finite; providing only equal values, or no field, is a no-op.

use super::super::horizontal_rules::{overhang_fault, shape_fault};
use super::SetRoofShape;
use crate::{Entry, ModelDiff, ModelSnapshot, RoofPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRoofShape, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(roof) = base.roofs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = payload.shape.as_ref().and_then(shape_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["shape"]);
    }
    if let Some(fault) = payload.overhang.and_then(overhang_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["overhang"]);
    }
    if payload.base_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A roof base offset must be a finite length.", ["base_offset"]);
    }
    let patch = RoofPatch {
        shape: payload.shape.clone().filter(|shape| *shape != roof.shape),
        overhang: payload.overhang.filter(|overhang| *overhang != roof.overhang),
        base_offset: payload.base_offset.filter(|offset| *offset != roof.base_offset),
        ..Default::default()
    };
    if patch == RoofPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Roof \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Patched(patch)))
}
`,
  inverse: rs`//! ↩️ Inverse of ¶SetRoofShape¶: an absolute ¶SetRoofShape¶ carrying the base values of exactly the provided fields, none when the roof
//! is absent.

use super::SetRoofShape;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetRoofShape, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![ModelMutation::SetRoofShape(SetRoofShape {
            id: payload.id.clone(),
            shape: payload.shape.as_ref().map(|_| roof.shape.clone()),
            overhang: payload.overhang.map(|_| roof.overhang),
            base_offset: payload.base_offset.map(|_| roof.base_offset),
        })],
        None => Vec::new(),
    }
}
`,
};

const dir = (spec: MineLeaf) => join(mutations, em(spec.emoji) + spec.kind);
const sub = (spec: MineLeaf, emoji: number, name: string) => join(dir(spec), em(emoji) + name);

const fixup = (spec: MineLeaf) => {
  const extra = extras.get(spec.kind)!;
  const component = join(sub(spec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  const generic = (rust: string) => rust.match(/[A-Z][A-Za-z0-9]*/g) ?? [];
  const imports = ["ModelDiff", "ModelMutation", "ModelSnapshot", ...spec.props.flatMap((prop) => generic(prop.rust)).filter((name) => name !== "String" && name !== "Option" && name !== "Vec"), ...extra.imports];
  source = source.replace(/use crate::\{[^}]*\};/, `use crate::{${[...new Set(imports)].sort().join(", ")}};`);
  for (const name of extra.optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  writeFileSync(component, source);

  const schemaFile = join(sub(spec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !extra.optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "m-horizontal");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec as Leaf);
    fixup(spec);
    const files = MODULES[spec.kind];
    writeFileSync(join(sub(spec, 0x1f53a, "diff"), RS), files.diff);
    writeFileSync(join(sub(spec, 0x21a9, "inverse"), RS), files.inverse);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);

  const rulesDir = join(mutations, em(0x1f9ff) + "horizontal-rules");
  mkdirSync(rulesDir, { recursive: true });
  writeFileSync(join(rulesDir, RS), RULES);
  const rulesMount = `                        #[path = "${rel(join(rulesDir, RS)).slice(rel(artifact).length + 1)}"]\n                        pub mod horizontal_rules;\n\n`;

  const root = join(artifact, RS);
  let source = readFileSync(root, "utf8");
  const anchor = source.match(/\/\/#region [^\r\n]*Leaves/)![0];
  const endAnchor = source.match(/\/\/#endregion [^\r\n]*Leaves/)![0];
  if (source.includes("pub mod create_slab {")) console.log("leaves already mounted");
  else {
    const at = source.indexOf(endAnchor);
    const lineStart = source.lastIndexOf("\n", at) + 1;
    source = source.slice(0, lineStart) + mounts + source.slice(lineStart);
    console.log("mounted the leaves");
  }
  if (source.includes("pub mod horizontal_rules;")) console.log("rules already mounted");
  else {
    const at = source.indexOf(anchor);
    const lineStart = source.lastIndexOf("\n", at) + 1;
    source = source.slice(0, lineStart) + rulesMount + source.slice(lineStart);
    console.log("mounted the rules");
  }
  writeFileSync(root, source);
  console.log(`emitted ${leaves.length} leaves`);
}
