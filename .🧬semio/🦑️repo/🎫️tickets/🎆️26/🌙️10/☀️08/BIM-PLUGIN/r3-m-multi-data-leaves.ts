#!/usr/bin/env bun
/**
 * 🧪️ Wave M slice 9 (label `m-multi-data`, binary tags 900 to 999): the multi-element and data leaves of `s.bim.model@1`.
 * `bun r3-m-multi-data-leaves.ts` rewrites boilerplate and fixtures (never a blessed `after`/`diff`), and with `register` splices the
 * mount blocks, the enum variants and the KINDS rows into the shared registration files (idempotent, surgical).
 * Part B (`bun r3-m-multi-data-leaves.ts cascades`) rewrites the fixtures of the four foundation delete leaves for the full cascade.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, fixtures, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const { P } = F;

const element = (kind: string): Prop["ui"]["ref"] => ({ kind });
const id = (label: Label, order = 10): Prop => ({ name: "id", rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "target", label, ref: element("element"), group: "target", order } });
const ids = (): Prop => ({
  name: "ids",
  rust: "Vec<String>",
  schema: { type: "array", items: { type: "string" } },
  ui: { widget: "reference", role: "target", label: { en: "Elements", de: "Elemente" }, description: { en: "Elements to act on; unknown ids refuse the whole call.", de: "Zu bearbeitende Elemente; unbekannte Ids verweigern den ganzen Aufruf." }, ref: element("element"), group: "target", order: 10 },
});
const record = (name: string, def: string, label: Label, group: string, order: number, description?: Label): Prop => ({ name, rust: def, schema: ref(def), ui: { widget: "record", role: "value", label, ...(description ? { description } : {}), group, order } });
const text = (name: string, label: Label, order: number, group = "identity"): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group, order } });
const angle: Prop = {
  name: "angle",
  rust: "f64",
  schema: { type: "number" },
  ui: {
    widget: "dial",
    role: "value",
    label: { en: "Angle", de: "Winkel" },
    description: { en: "Counter-clockwise rotation about the pivot.", de: "Drehung gegen den Uhrzeigersinn um den Drehpunkt." },
    unit: "rad",
    displayUnit: "deg",
    displayFactor: 57.29577951308232,
    step: 0.017453292519943295,
    precision: 4,
    softMin: -Math.PI,
    softMax: Math.PI,
    snaps: [-Math.PI, -Math.PI / 2, 0, Math.PI / 2, Math.PI],
    group: "rotation",
    order: 30,
  } as Prop["ui"],
};

const VARIANTS: [string, Record<string, unknown>, string[]][] = [
  ["Wall", { axis: ref("Axis") }, ["axis"]],
  ["CurtainWall", { axis: ref("Axis") }, ["axis"]],
  ["Column", { position: ref("Point2"), rotation: { type: "number" } }, ["position", "rotation"]],
  ["Beam", { start: ref("Point2"), end: ref("Point2") }, ["start", "end"]],
  ["Slab", { boundary: { type: "array", items: ref("Vertex") }, holes: { type: "array", items: { type: "array", items: ref("Vertex") } }, slope: ref("Slope") }, ["boundary", "holes"]],
  ["Roof", { footprint: { type: "array", items: ref("Vertex") }, shape: ref("RoofShape") }, ["footprint", "shape"]],
  ["Stair", { start: ref("Point2"), direction: { type: "number" } }, ["start", "direction"]],
  ["Railing", { path: { type: "array", items: ref("Point2") } }, ["path"]],
  ["Space", { boundary: ref("SpaceBoundary") }, ["boundary"]],
  ["Grid", { start: ref("Point2"), end: ref("Point2") }, ["start", "end"]],
];
const placementDef = {
  description: "Absolute placement fields of one placed element, tagged by its kind.",
  oneOf: VARIANTS.map(([name, properties, required]) => ({ type: "object", additionalProperties: false, required: [name], properties: { [name]: { type: "object", additionalProperties: false, required, properties } } })),
};
const placements: Prop = {
  name: "placements",
  rust: "BTreeMap<String, Placement>",
  schema: { type: "object", additionalProperties: { $ref: "#/$defs/Placement" } },
  ui: { widget: "record", role: "value", label: { en: "Placements", de: "Platzierungen" }, description: { en: "Element id to its absolute placement fields.", de: "Element-Id auf seine absoluten Platzierungsfelder." }, group: "value", order: 10 },
};

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const missing = (...path: string[]) => reject("mutation.target-missing", path);
const invariant = (...path: string[]) => reject("mutation.invariant", path);
const noop = (...path: string[]) => reject("mutation.no-op", path);
const referenced = (...path: string[]) => reject("mutation.target-referenced", path);

//#region 🔖️Fixtures
const rect = (width: number, depth: number) => ({ Rectangle: { width, depth } });
const V = (x: number, y: number, bulge = 0) => ({ point: P(x, y), bulge });
const box = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
const pset = (...rows: [string, string, unknown][]) => rows.reduce<Record<string, Record<string, unknown>>>((sets, [set, name, value]) => ({ ...sets, [set]: { ...sets[set], [name]: value } }), {});
const text_ = (value: string) => ({ Text: { value } });
const flag = (value: boolean) => ({ Boolean: { value } });
const real = (value: number) => ({ Real: { value } });
const classification = (system: string, code: string, title: string) => ({ system, code, title });

const columnRow = { storey: "st-ground", column_type: "ct-400", position: P(4, 3), rotation: 0.25, base_offset: 0, top: F.storeyTop(0), name: "C1" };
const beamRow = { storey: "st-ground", beam_type: "bt-30x50", start: P(0, 3), end: P(8, 3), top_offset: -0.1, name: "B1" };
const slabRow = { storey: "st-ground", slab_type: "slt-200", boundary: box(0, 0, 8, 6), holes: [box(2, 2, 3, 3)], offset: 0, slope: { direction: 0.5, angle: 0.1 }, name: "Floor" };
const roofRow = { storey: "st-first", roof_type: "rt-200", footprint: box(0, 0, 8, 6), shape: { Gable: { pitch: 0.6, ridge_direction: 0 } }, overhang: 0.3, base_offset: 0, name: "Roof" };
const stairRow = { storey: "st-ground", start: P(1, 1), direction: 0.5, width: 1, flight: "Straight", top: F.storeyTop(0), max_riser: 0.18, min_tread: 0.27, name: "Stair" };
const railingRow = { storey: "st-ground", path: [P(1, 1), P(5, 1)], height: 1, post_spacing: 1.2, material: "m-steel", base_offset: 0, name: "Rail" };
const spaceRow = { storey: "st-ground", number: "G.01", name: "Hall", boundary: { Explicit: { outline: box(0, 0, 4, 3) } }, usage: "Living" };
const curtainRow = { storey: "st-ground", axis: F.line([0, 6], [8, 6]), base_offset: 0, top: F.storeyTop(0), u_spacing: 1.5, v_spacing: 1.2, mullion: rect(0.06, 0.1), panel_material: "m-glass", mullion_material: "m-steel", name: "Facade" };
const gridRow = { building: "bldg-1", label: "A", start: P(0, -1), end: P(0, 7) };

/** 🏡️ The two-storey house with one element of every placed kind, an opening, properties and a classification. */
export const house = (over: Record<string, unknown> = {}) => ({
  ...F.snap({
    materials: { "m-brick": F.material("Brick"), "m-steel": F.material("Steel"), "m-glass": F.material("Glass") },
    wall_types: { "wt-300": F.wallType("Brick 300", [F.layer("m-brick", 0.3)]) },
    slab_types: { "slt-200": F.wallType("Slab 200", [F.layer("m-brick", 0.2)]) },
    roof_types: { "rt-200": F.wallType("Roof 200", [F.layer("m-brick", 0.2)]) },
    column_types: { "ct-400": { name: "Column 400", profile: rect(0.4, 0.4), material: "m-brick" } },
    beam_types: { "bt-30x50": { name: "Beam 30x50", profile: rect(0.3, 0.5), material: "m-brick" } },
    window_types: { "win-1": { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.07, frame_depth: 0.12, panes: 2, material: "m-brick" } },
    sites: { "site-1": F.site("Plot") },
    buildings: { "bldg-1": F.building("site-1", "House") },
    storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3), "st-first": F.storey("bldg-1", "First", 1, 2.8) },
    grids: { "g-a": gridRow },
    walls: {
      "w-south": F.wall("st-ground", "wt-300", F.line([0, 0], [8, 0]), F.storeyTop(0), "South"),
      "w-east": F.wall("st-ground", "wt-300", F.line([8, 0], [8, 6]), F.unconnected(2.4), "East"),
    },
    curtain_walls: { "cw-1": curtainRow },
    columns: { "c-1": columnRow },
    beams: { "b-1": beamRow },
    slabs: { "sl-1": slabRow },
    roofs: { "rf-1": roofRow },
    openings: { "o-1": F.opening("w-south", "Window") },
    stairs: { "s-1": stairRow },
    railings: { "r-1": railingRow },
    spaces: { "sp-1": spaceRow },
    properties: {
      "w-south": pset(["Pset_WallCommon", "FireRating", text_("F90")], ["Pset_WallCommon", "IsExternal", flag(true)]),
      "o-1": pset(["Pset_WindowCommon", "ThermalTransmittance", real(1.1)]),
    },
    classifications: { "w-south": classification("Uniclass", "EF_25_10", "Walls") },
  }),
  ...over,
});
const ALL_PLACED = ["w-south", "w-east", "cw-1", "c-1", "b-1", "sl-1", "rf-1", "s-1", "r-1", "sp-1", "g-a", "o-1"];
const line = (a: [number, number], b: [number, number]) => ({ Line: { start: P(...a), end: P(...b) } });
//#endregion 🔖️Fixtures

const target = "self.ids.clone()";
const one = "vec![self.id.clone()]";
const emoji = { applied: 0x2705, applied2: 0x1f69b, applied3: 0x1f300, unknown: 0x1f6ab, empty: 0x26d4, zero: 0x1f6d1, other: 0x1f6a7, last: 0x1f9f2 };

export const leaves: Leaf[] = [
  {
    kind: "move-elements", emoji: 0x1f69a, variant: "MoveElements", verb: "move", entity: "elements", binaryTag: 900, displayName: "Move Elements",
    doc: "Translates placed elements by a vector: walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and grid lines each get one sparse patch of exactly their placement fields; hosted openings follow their host by inference.",
    props: [ids(), record("vector", "Point2", { en: "Translation (m)", de: "Verschiebung (m)" }, "translation", 20)],
    label: { en: 'format!("Move {} element(s) by ({}, {}) m", self.ids.len(), self.vector.x, self.vector.y)', de: 'format!("{} Element(e) um ({}, {}) m verschieben", self.ids.len(), self.vector.x, self.vector.y)' },
    target,
    cases: [
      { name: "moves-a-wall-and-its-opening", emoji: emoji.applied, before: house(), mutation: { ids: ["w-south", "o-1"], vector: P(1.5, -2) }, outcome: ok },
      { name: "moves-every-placed-kind", emoji: emoji.applied2, before: house(), mutation: { ids: ALL_PLACED, vector: P(0.25, 4) }, outcome: ok },
      { name: "unknown-id", emoji: emoji.unknown, before: house(), mutation: { ids: ["w-south", "w-missing"], vector: P(1, 0) }, outcome: missing("w-missing") },
      { name: "empty-selection", emoji: emoji.empty, before: house(), mutation: { ids: [], vector: P(1, 0) }, outcome: invariant("ids") },
      { name: "zero-vector", emoji: emoji.zero, before: house(), mutation: { ids: ["w-south"], vector: P(0, 0) }, outcome: noop("vector") },
      { name: "storey-has-no-placement", emoji: emoji.other, before: house(), mutation: { ids: ["st-ground"], vector: P(1, 0) }, outcome: invariant("st-ground") },
    ],
  },
  {
    kind: "rotate-elements", emoji: 0x1f3a1, variant: "RotateElements", verb: "rotate", entity: "elements", binaryTag: 901, displayName: "Rotate Elements",
    doc: "Turns placed elements counter-clockwise about a pivot, one sparse patch of exactly the placement fields per element; column rotation, stair direction, slab fall and roof ridge direction turn with them and hosted openings follow by inference.",
    props: [ids(), record("pivot", "Point2", { en: "Pivot (m)", de: "Drehpunkt (m)" }, "rotation", 20), angle],
    label: { en: 'format!("Rotate {} element(s) by {} rad", self.ids.len(), self.angle)', de: 'format!("{} Element(e) um {} rad drehen", self.ids.len(), self.angle)' },
    target,
    cases: [
      { name: "turns-a-wall-and-a-column", emoji: emoji.applied, before: house(), mutation: { ids: ["w-south", "c-1"], pivot: P(0, 0), angle: Math.PI / 2 }, outcome: ok },
      { name: "turns-every-placed-kind", emoji: emoji.applied2, before: house(), mutation: { ids: ALL_PLACED, pivot: P(4, 3), angle: Math.PI / 2 }, outcome: ok },
      { name: "turns-by-thirty-degrees", emoji: emoji.applied3, before: house(), mutation: { ids: ["w-south", "b-1"], pivot: P(1, 1), angle: Math.PI / 6 }, outcome: ok },
      { name: "unknown-id", emoji: emoji.unknown, before: house(), mutation: { ids: ["w-missing"], pivot: P(0, 0), angle: 1 }, outcome: missing("w-missing") },
      { name: "empty-selection", emoji: emoji.empty, before: house(), mutation: { ids: [], pivot: P(0, 0), angle: 1 }, outcome: invariant("ids") },
      { name: "zero-angle", emoji: emoji.zero, before: house(), mutation: { ids: ["w-south"], pivot: P(0, 0), angle: 0 }, outcome: noop("angle") },
      { name: "storey-has-no-placement", emoji: emoji.other, before: house(), mutation: { ids: ["st-ground"], pivot: P(0, 0), angle: 1 }, outcome: invariant("st-ground") },
    ],
  },
  {
    kind: "place-elements", emoji: 0x1faa7, variant: "PlaceElements", verb: "set", entity: "elements", binaryTag: 908, displayName: "Place Elements",
    doc: "Writes absolute placement fields onto placed elements, one sparse patch per element; the exact inverse of a move or a rotation, because it restores the base placements without any floating-point back transformation.",
    props: [placements],
    uses: ["use super::super::elements::Placement;", "use std::collections::BTreeMap;"],
    label: { en: 'format!("Place {} element(s)", self.placements.len())', de: 'format!("{} Element(e) platzieren", self.placements.len())' },
    target: "self.placements.keys().cloned().collect()",
    cases: [
      { name: "sets-absolute-placements", emoji: emoji.applied, before: house(), mutation: { placements: { "w-south": { Wall: { axis: line([1, 1], [9, 1]) } }, "c-1": { Column: { position: P(5, 5), rotation: 1 } } } }, outcome: ok },
      { name: "unknown-id", emoji: emoji.unknown, before: house(), mutation: { placements: { "w-missing": { Wall: { axis: line([1, 1], [9, 1]) } } } }, outcome: missing("w-missing") },
      { name: "kind-mismatch", emoji: emoji.other, before: house(), mutation: { placements: { "w-south": { Column: { position: P(5, 5), rotation: 1 } } } }, outcome: invariant("w-south") },
      { name: "same-placement", emoji: emoji.zero, before: house(), mutation: { placements: { "w-south": { Wall: { axis: line([0, 0], [8, 0]) } } } }, outcome: noop("placements") },
      { name: "empty-map", emoji: emoji.empty, before: house(), mutation: { placements: {} }, outcome: invariant("placements") },
    ],
  },
  {
    kind: "delete-elements", emoji: 0x1f4a3, variant: "DeleteElements", verb: "delete", entity: "elements", binaryTag: 902, displayName: "Delete Elements",
    doc: "Removes any set of elements together with everything that depends on them (buildings of a site, storeys and grid lines of a building, the contents of a storey, openings of walls and curtain walls) and with their properties and classifications; refuses while a surviving element's top constraint still points at a removed storey.",
    props: [ids()],
    inverseRows: { bounded: 65536 },
    label: { en: 'format!("Delete {} element(s) with their dependants", self.ids.len())', de: 'format!("{} Element(e) samt Abhängigen löschen", self.ids.len())' },
    target,
    cases: [
      { name: "cascades-openings-and-data", emoji: emoji.applied, before: house(), mutation: { ids: ["w-south"] }, outcome: ok },
      { name: "removes-furnishing-kinds", emoji: emoji.applied2, before: house(), mutation: { ids: ["cw-1", "c-1", "b-1", "sl-1", "rf-1", "s-1", "r-1", "sp-1", "g-a"] }, outcome: ok },
      { name: "removes-a-storey-with-everything-on-it", emoji: emoji.applied3, before: house(), mutation: { ids: ["st-ground"] }, outcome: ok },
      { name: "unknown-id", emoji: emoji.unknown, before: house(), mutation: { ids: ["w-south", "w-missing"] }, outcome: missing("w-missing") },
      { name: "empty-selection", emoji: emoji.empty, before: house(), mutation: { ids: [] }, outcome: invariant("ids") },
      { name: "pinned-storey", emoji: emoji.other, before: house({ walls: { ...house().walls, "w-first": F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.toStorey("st-ground", 0.2), "First") } }), mutation: { ids: ["st-ground"] }, outcome: referenced("st-ground") },
    ],
  },
  {
    kind: "rename-element", emoji: 0x1faaa, variant: "RenameElement", verb: "rename", entity: "element", binaryTag: 903, displayName: "Rename Element",
    doc: "Changes the display name of any element (the label of a grid line) in whichever collection holds the id.",
    props: [id({ en: "Element", de: "Element" }), text("name", { en: "Name", de: "Name" }, 20)],
    label: { en: 'format!("Rename element \\"{}\\" to \\"{}\\"", self.id, self.name)', de: 'format!("Element \\"{}\\" in \\"{}\\" umbenennen", self.id, self.name)' },
    target: one,
    cases: [
      { name: "renames-a-wall", emoji: emoji.applied, before: house(), mutation: { id: "w-south", name: "Front" }, outcome: ok },
      { name: "relabels-a-grid-line", emoji: emoji.applied2, before: house(), mutation: { id: "g-a", name: "A1" }, outcome: ok },
      { name: "unknown-id", emoji: emoji.unknown, before: house(), mutation: { id: "w-missing", name: "Front" }, outcome: missing("w-missing") },
      { name: "same-name", emoji: emoji.zero, before: house(), mutation: { id: "w-south", name: "South" }, outcome: noop("w-south") },
    ],
  },
  {
    kind: "set-element-property", emoji: 0x1f9fe, variant: "SetElementProperty", verb: "set", entity: "element-property", binaryTag: 904, displayName: "Set Element Property",
    doc: "Sets one typed property of an element's property set; an element without properties gets its first entry, an existing value is replaced.",
    props: [id({ en: "Element", de: "Element" }), text("pset", { en: "Property set", de: "Eigenschaftsgruppe" }, 20, "property"), text("property", { en: "Property", de: "Eigenschaft" }, 30, "property"), record("value", "PropertyValue", { en: "Value", de: "Wert" }, "property", 40)],
    label: { en: 'format!("Set property {}.{} of \\"{}\\"", self.pset, self.property, self.id)', de: 'format!("Eigenschaft {}.{} von \\"{}\\" setzen", self.pset, self.property, self.id)' },
    target: one,
    cases: [
      { name: "adds-the-first-property", emoji: emoji.applied, before: house(), mutation: { id: "c-1", pset: "Pset_ColumnCommon", property: "LoadBearing", value: flag(true) }, outcome: ok },
      { name: "adds-to-an-existing-set", emoji: emoji.applied2, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "Combustible", value: flag(false) }, outcome: ok },
      { name: "replaces-a-value", emoji: emoji.applied3, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "FireRating", value: text_("F120") }, outcome: ok },
      { name: "unknown-element", emoji: emoji.unknown, before: house(), mutation: { id: "w-missing", pset: "Pset_WallCommon", property: "FireRating", value: text_("F90") }, outcome: missing("w-missing") },
      { name: "blank-property", emoji: emoji.other, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "", value: text_("F90") }, outcome: invariant("property") },
      { name: "same-value", emoji: emoji.zero, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "FireRating", value: text_("F90") }, outcome: noop("w-south") },
    ],
  },
  {
    kind: "remove-element-property", emoji: 0x1fae7, variant: "RemoveElementProperty", verb: "remove", entity: "element-property", binaryTag: 905, displayName: "Remove Element Property",
    doc: "Removes one property of an element; removing its last property deletes the element's property entry.",
    props: [id({ en: "Element", de: "Element" }), text("pset", { en: "Property set", de: "Eigenschaftsgruppe" }, 20, "property"), text("property", { en: "Property", de: "Eigenschaft" }, 30, "property")],
    label: { en: 'format!("Remove property {}.{} of \\"{}\\"", self.pset, self.property, self.id)', de: 'format!("Eigenschaft {}.{} von \\"{}\\" entfernen", self.pset, self.property, self.id)' },
    target: one,
    cases: [
      { name: "removes-one-of-two", emoji: emoji.applied, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "IsExternal" }, outcome: ok },
      { name: "removes-the-last-property", emoji: emoji.applied2, before: house(), mutation: { id: "o-1", pset: "Pset_WindowCommon", property: "ThermalTransmittance" }, outcome: ok },
      { name: "missing-property", emoji: emoji.unknown, before: house(), mutation: { id: "w-south", pset: "Pset_WallCommon", property: "Nope" }, outcome: missing("w-south", "Pset_WallCommon", "Nope") },
      { name: "unknown-element", emoji: emoji.empty, before: house(), mutation: { id: "w-missing", pset: "Pset_WallCommon", property: "IsExternal" }, outcome: missing("w-missing") },
    ],
  },
  {
    kind: "set-element-classification", emoji: 0x1f5c2, variant: "SetElementClassification", verb: "set", entity: "element-classification", binaryTag: 906, displayName: "Set Element Classification",
    doc: "Sets the classification reference (system, code, title) of an element; an element without one gets it created, an existing one is patched field by field.",
    props: [id({ en: "Element", de: "Element" }), record("classification", "Classification", { en: "Classification", de: "Klassifizierung" }, "classification", 20)],
    label: { en: 'format!("Classify element \\"{}\\" as {} {}", self.id, self.classification.system, self.classification.code)', de: 'format!("Element \\"{}\\" als {} {} klassifizieren", self.id, self.classification.system, self.classification.code)' },
    target: one,
    cases: [
      { name: "classifies-an-element", emoji: emoji.applied, before: house(), mutation: { id: "c-1", classification: classification("Uniclass", "Ss_25_10", "Columns") }, outcome: ok },
      { name: "reclassifies", emoji: emoji.applied2, before: house(), mutation: { id: "w-south", classification: classification("Uniclass", "EF_25_11", "Walls") }, outcome: ok },
      { name: "unknown-element", emoji: emoji.unknown, before: house(), mutation: { id: "w-missing", classification: classification("Uniclass", "EF_25_10", "Walls") }, outcome: missing("w-missing") },
      { name: "empty-code", emoji: emoji.other, before: house(), mutation: { id: "w-south", classification: classification("Uniclass", "", "Walls") }, outcome: invariant("classification", "code") },
      { name: "same-classification", emoji: emoji.zero, before: house(), mutation: { id: "w-south", classification: classification("Uniclass", "EF_25_10", "Walls") }, outcome: noop("w-south") },
    ],
  },
  {
    kind: "remove-element-classification", emoji: 0x1f5c4, variant: "RemoveElementClassification", verb: "remove", entity: "element-classification", binaryTag: 907, displayName: "Remove Element Classification",
    doc: "Removes the classification reference of an element.",
    props: [id({ en: "Element", de: "Element" })],
    label: { en: 'format!("Remove classification of \\"{}\\"", self.id)', de: 'format!("Klassifizierung von \\"{}\\" entfernen", self.id)' },
    target: one,
    cases: [
      { name: "removes", emoji: emoji.applied, before: house(), mutation: { id: "w-south" }, outcome: ok },
      { name: "not-classified", emoji: emoji.unknown, before: house(), mutation: { id: "c-1" }, outcome: missing("c-1", "classification") },
      { name: "unknown-element", emoji: emoji.empty, before: house(), mutation: { id: "w-missing" }, outcome: missing("w-missing") },
    ],
  },
];

//#region 🔖️Post
const leafDir = (leaf: Leaf) => join(mutations, em(leaf.emoji) + leaf.kind);
const schemaFile = (leaf: Leaf) => join(leafDir(leaf), readdirSync(leafDir(leaf)).find((name) => name.endsWith("schema"))!, JSONF);

const addDefs = (leaf: Leaf) => {
  if (leaf.kind !== "place-elements") return;
  const path = schemaFile(leaf);
  const schema = JSON.parse(readFileSync(path, "utf8"));
  writeFileSync(path, JSON.stringify({ ...schema, $defs: { Placement: placementDef } }, null, 2) + "\n");
};

const register = (mounts: Map<string, string>) => {
  const root = join(artifact, RS);
  const marker = "//#endregion 🔖️Leaves";
  const rootRaw = readFileSync(root, "utf8");
  const crlf = rootRaw.includes("\r\n");
  let text = rootRaw.replaceAll("\r\n", "\n");
  const fresh = [...mounts].filter(([module]) => !text.includes(`pub mod ${module} {`));
  const at = text.indexOf(marker);
  const lineStart = text.lastIndexOf("\n", at) + 1;
  text = text.slice(0, lineStart) + fresh.map(([, block]) => block).join("") + text.slice(lineStart);
  if (fresh.length) writeFileSync(root, crlf ? text.replaceAll("\n", "\r\n") : text);
  const aggregate = join(mutations, RS);
  const aggregateRaw = readFileSync(aggregate, "utf8");
  const aggregateCrlf = aggregateRaw.includes("\r\n");
  let source = aggregateRaw.replaceAll("\r\n", "\n");
  const variants = leaves.filter((leaf) => !source.includes(`${leaf.variant}(super::`));
  const enumStart = source.indexOf("pub enum ModelMutation {");
  const enumEnd = source.indexOf("\n}\n", enumStart);
  source = source.slice(0, enumEnd + 1) + variants.map((leaf) => `    ${leaf.variant}(super::${leaf.kind.replaceAll("-", "_")}::${leaf.variant}),\n`).join("") + source.slice(enumEnd + 1);
  const kinds = leaves.filter((leaf) => !source.includes(`"${leaf.kind}",`));
  const kindsStart = source.indexOf("pub const KINDS");
  const kindsEnd = source.indexOf("\n];\n", kindsStart);
  source = source.slice(0, kindsEnd + 1) + kinds.map((leaf) => `    "${leaf.kind}",\n`).join("") + source.slice(kindsEnd + 1);
  if (variants.length || kinds.length) writeFileSync(aggregate, aggregateCrlf ? source.replaceAll("\n", "\r\n") : source);
  console.log(`registered ${fresh.length} mount blocks, ${variants.length} variants, ${kinds.length} kinds`);
};
//#endregion 🔖️Post

if (import.meta.main) {
  const scratch = join(import.meta.dir, "🗑️generated", "m-multi-data");
  mkdirSync(scratch, { recursive: true });
  const mounts = new Map<string, string>();
  for (const leaf of leaves) {
    mounts.set(leaf.kind.replaceAll("-", "_"), emitLeaf(leaf));
    addDefs(leaf);
  }
  writeFileSync(join(scratch, "mounts.txt"), [...mounts.values()].join(""));
  if (process.argv.includes("register")) register(mounts);
  console.log(`emitted ${leaves.length} leaves`);
}
void existsSync;
void rmSync;
void fixtures;
