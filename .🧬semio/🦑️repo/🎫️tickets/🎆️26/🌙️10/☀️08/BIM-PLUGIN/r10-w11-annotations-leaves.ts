#!/usr/bin/env bun
/**
 * 📏️ Wave W1 `w11-annotations` (binary tags 11000..11014): the fifteen annotation leaves of `s.bim.model@1`: create/set/delete of the dimension, the tag, the
 * text note, the leader and the annotation style. `bun r10-w11-annotations-leaves.ts` rewrites the boilerplate and fixtures through `emitLeaf`, fixes the
 * optional `set-*` fields of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`, never overwritten) and mounts the leaves
 * once in the artifact root. `after`/`diff` fixtures of applied cases are never overwritten: bless them with `BIM_BLESS=1 cargo test`.
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
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
const reference = (name: string, kind: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group: "link", order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"] });
const angle = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "rad", step: 0.01, precision: 4 } as Prop["ui"] });
const text = (name: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const point = (name: string, label: Label, order: number): Mine => ({ name, rust: "Point2", schema: record("Point2"), ui: { widget: "record", role: "value", label, group: "placement", order } });
const choice = (name: string, def: string, label: Label, order: number): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "select", role: "value", label, group: "value", order } });
const count = (name: string, label: Label, order: number): Mine => ({ name, rust: "u32", schema: { type: "integer", minimum: 0 }, ui: { widget: "integer", role: "value", label, group: "value", order } });
const anchorsProp = (order: number): Mine => ({ name: "anchors", rust: "Vec<AnnotationAnchor>", schema: { type: "array", items: record("AnnotationAnchor") }, ui: { widget: "list", role: "value", label: { en: "Anchors", de: "Anker" }, group: "link", order } });
const anchorProp = (order: number): Mine => ({ name: "anchor", rust: "AnnotationAnchor", schema: record("AnnotationAnchor"), ui: { widget: "record", role: "value", label: { en: "Anchor", de: "Anker" }, group: "link", order } });
const lockProp = (order: number): Mine => ({
  name: "lock",
  rust: "Assigned<Option<f64>>",
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { type: ["number", "null"] } } },
  ui: { widget: "number", role: "value", label: { en: "Lock (m)", de: "Sperre (m)" }, group: "value", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"],
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2, 0x1f4aa];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757, 0x1f6c7, 0x1f4a2];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Leaf["cases"][number]["outcome"] };
const cases = (rows: Row[]): Leaf["cases"] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

//#region 🔖️Fixtures
const P = F.P;
const wallEnd = (wall: string, end: "Start" | "End") => ({ WallEnd: { wall, end } });
const face = (wall: string, side: "Left" | "Right") => ({ WallFace: { wall, side } });
const axisOf = (wall: string) => ({ WallAxis: { wall } });
const centre = (opening: string) => ({ OpeningCentre: { opening } });
const grid = (g: string) => ({ Grid: { grid: g } });
const free = (x: number, y: number) => ({ Point: { point: P(x, y) } });
const style = (name: string, textHeight = 0.25) => ({ name, text_height: textHeight, terminator: "Tick", unit: "Metre", precision: 2, mark_size: 0.15, gap: 0.1, overshoot: 0.2 });
const dimension = (anchors: unknown[], extra: Record<string, unknown> = {}) => ({ storey: "st-ground", anchors, angle: 0, offset: -1, style: "as-plan", name: "Dimension", ...extra });
const tag = (element: string, extra: Record<string, unknown> = {}) => ({ storey: "st-ground", element, category: "Name", offset: P(0, -0.5), style: "as-plan", ...extra });
const note = (extra: Record<string, unknown> = {}) => ({ storey: "st-ground", position: P(2, 2), text: "Verify on site", rotation: 0, style: "as-plan", ...extra });
const leader = (anchor: unknown, extra: Record<string, unknown> = {}) => ({ storey: "st-ground", anchor, offset: P(1, 1), text: "Brick 300", style: "as-plan", ...extra });
const windowType = { name: "Window 1.2", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.1, panes: 1, material: "m-brick" };

const library = (extra: Record<string, unknown> = {}) => ({
  ...F.scene(),
  window_types: { "win-1": windowType },
  openings: { "o-win": F.opening("w-south", "South window") },
  grids: { "g-1": { building: "bldg-1", label: "1", start: P(0, 0), end: P(8, 0) } },
  annotation_styles: { "as-plan": style("Plan 1:50"), "as-large": style("Plan 1:100", 0.4) },
  ...extra,
});
const withDimension = () => library({ dimensions: { "dim-south": dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { name: "South length" }) } });
const withLocked = () => library({ dimensions: { "dim-south": dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { name: "South length", lock: 8 }) } });
const withTag = () => library({ tags: { "tag-south": tag("w-south") } });
const withNote = () => library({ text_notes: { "note-1": note() } });
const withLeader = () => library({ leaders: { "lead-1": leader(face("w-south", "Right")) } });
const withDimensionOfStyle = () => library({ dimensions: { "dim-south": dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { style: "as-large" }) } });
const withTagOfStyle = () => library({ tags: { "tag-south": tag("w-south", { style: "as-large" }) } });
const withNoteOfStyle = () => library({ text_notes: { "note-1": note({ style: "as-large" }) } });
const withLeaderOfStyle = () => library({ leaders: { "lead-1": leader(face("w-south", "Right"), { style: "as-large" }) } });
const donor = (before: Record<string, any>) => before;
//#endregion 🔖️Fixtures

type Extra = { optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf): MineLeaf => (extras.set(spec.kind, { optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const styleKind = { en: "Annotation style", de: "Beschriftungsstil" };
const dimLabel = { en: "Dimension", de: "Bemaßung" };
const tagLabel = { en: "Tag", de: "Kennzeichnung" };
const noteLabel = { en: "Text note", de: "Textnotiz" };
const leaderLabel = { en: "Leader", de: "Hinweislinie" };

const dimensionLeaves: MineLeaf[] = [
  leaf({
    kind: "create-dimension", emoji: 0x2194, variant: "CreateDimension", verb: "create", entity: "dimension", binaryTag: 11000, displayName: "Create Dimension",
    doc: "Brings a new dimension onto a storey plan: two or more anchors (free points or elements of the model) measured along a direction; its values and text are inferred from the geometry of the anchors.",
    props: [id("dimension", "identity", { en: "Dimension id", de: "Bemaßungs-Id" }, 10), recordProp("dimension", "Dimension", dimLabel)],
    label: { en: 'format!("Create dimension \\"{}\\"", self.dimension.name)', de: 'format!("Bemaßung \\"{}\\" anlegen", self.dimension.name)' },
    target,
    cases: cases([
      { name: "dimensions-a-wall", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { name: "South length" }) }, outcome: ok },
      { name: "chains-an-opening-centre", before: library(), mutation: { id: "dim-chain", dimension: dimension([wallEnd("w-south", "Start"), centre("o-win"), wallEnd("w-south", "End")], { name: "Window position", offset: 0.8 }) }, outcome: ok },
      { name: "thickness-between-faces", before: library(), mutation: { id: "dim-thickness", dimension: dimension([face("w-south", "Left"), face("w-south", "Right")], { angle: 1.5707963267948966, offset: 1, name: "South thickness" }) }, outcome: ok },
      { name: "locks-a-free-span", before: library(), mutation: { id: "dim-free", dimension: dimension([free(0, 0), free(3, 0), grid("g-1")], { lock: 3, name: "Free span" }) }, outcome: ok },
      { name: "duplicate", before: withDimension(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")]) }, outcome: reject("mutation.duplicate-id", ["dim-south"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")]) }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["dimension", "storey"]) },
      { name: "style-missing", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { style: "as-ghost" }) }, outcome: reject("mutation.target-missing", ["dimension", "style"]) },
      { name: "anchor-missing", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-ghost", "End")]) }, outcome: reject("mutation.target-missing", ["dimension", "anchors"]) },
      { name: "one-anchor", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start")]) }, outcome: reject("mutation.invariant", ["dimension", "anchors"]) },
      { name: "lock-not-positive", before: library(), mutation: { id: "dim-south", dimension: dimension([wallEnd("w-south", "Start"), wallEnd("w-south", "End")], { lock: 0 }) }, outcome: reject("mutation.invariant", ["dimension", "lock"]) },
    ]),
  }),
  leaf({
    kind: "delete-dimension", emoji: 0x1f4ce, variant: "DeleteDimension", verb: "delete", entity: "dimension", binaryTag: 11001, displayName: "Delete Dimension",
    doc: "Removes a dimension; the elements it measured stay untouched.",
    props: [id("dimension", "target", dimLabel)],
    label: { en: 'format!("Delete dimension \\"{}\\"", self.id)', de: 'format!("Bemaßung \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withDimension(), mutation: { id: "dim-south" }, outcome: ok },
      { name: "missing", before: withDimension(), mutation: { id: "dim-west" }, outcome: reject("mutation.target-missing", ["dim-west"]) },
    ]),
  }),
  leaf({
    kind: "set-dimension", emoji: 0x1f4cc, variant: "SetDimension", verb: "set", entity: "dimension", binaryTag: 11002, displayName: "Set Dimension",
    doc: "Sets exactly the provided fields of a dimension: anchors, measuring direction, offset, style, lock (an assigned null removes it) and name; absent fields stay untouched.",
    props: [
      id("dimension", "target", dimLabel),
      sparse(anchorsProp(20), keep),
      sparse(angle("angle", { en: "Direction (rad)", de: "Richtung (rad)" }, 30), keep),
      sparse(length("offset", { en: "Offset (m)", de: "Abstand (m)" }, 40), keep),
      sparse(reference("style", "annotation-style", styleKind, 50), keep),
      sparse(lockProp(60), keep),
      sparse(text("name", { en: "Name", de: "Name" }, 70), keep),
    ],
    label: { en: 'format!("Change dimension \\"{}\\"", self.id)', de: 'format!("Bemaßung \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "moves-the-dimension-line", before: withDimension(), mutation: { id: "dim-south", offset: -1.5 }, outcome: ok },
      { name: "re-anchors", before: withDimension(), mutation: { id: "dim-south", anchors: [wallEnd("w-south", "Start"), centre("o-win"), wallEnd("w-south", "End")] }, outcome: ok },
      { name: "locks-the-value", before: withDimension(), mutation: { id: "dim-south", lock: { value: 8 } }, outcome: ok },
      { name: "removes-the-lock", before: withLocked(), mutation: { id: "dim-south", lock: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withDimension(), mutation: { id: "dim-south", offset: -1, name: "South wall length" }, outcome: ok },
      { name: "unchanged", before: withDimension(), mutation: { id: "dim-south", offset: -1, angle: 0 }, outcome: reject("mutation.no-op", ["dim-south"]) },
      { name: "names-no-field", before: withDimension(), mutation: { id: "dim-south" }, outcome: reject("mutation.no-op", ["dim-south"]) },
      { name: "style-missing", before: withDimension(), mutation: { id: "dim-south", style: "as-ghost" }, outcome: reject("mutation.target-missing", ["style"]) },
      { name: "anchor-missing", before: withDimension(), mutation: { id: "dim-south", anchors: [wallEnd("w-south", "Start"), grid("g-9")] }, outcome: reject("mutation.target-missing", ["anchors"]) },
      { name: "one-anchor", before: withDimension(), mutation: { id: "dim-south", anchors: [wallEnd("w-south", "Start")] }, outcome: reject("mutation.invariant", ["anchors"]) },
      { name: "lock-not-positive", before: withDimension(), mutation: { id: "dim-south", lock: { value: -2 } }, outcome: reject("mutation.invariant", ["lock"]) },
      { name: "missing", before: withDimension(), mutation: { id: "dim-west", offset: 1 }, outcome: reject("mutation.target-missing", ["dim-west"]) },
    ]),
  }),
];

const tagLeaves: MineLeaf[] = [
  leaf({
    kind: "create-tag", emoji: 0x1f516, variant: "CreateTag", verb: "create", entity: "tag", binaryTag: 11003, displayName: "Create Tag",
    doc: "Brings a new tag onto a storey plan: text read from an element (name, type, number or size) and placed beside it, so the tag follows the element.",
    props: [id("tag", "identity", { en: "Tag id", de: "Kennzeichnungs-Id" }, 10), recordProp("tag", "Tag", tagLabel)],
    label: { en: 'format!("Create tag \\"{}\\"", self.id)', de: 'format!("Kennzeichnung \\"{}\\" anlegen", self.id)' },
    target,
    cases: cases([
      { name: "tags-a-wall", before: library(), mutation: { id: "tag-south", tag: tag("w-south") }, outcome: ok },
      { name: "tags-a-window-size", before: library(), mutation: { id: "tag-win", tag: tag("o-win", { category: "Size", offset: P(0, 0.6) }) }, outcome: ok },
      { name: "duplicate", before: withTag(), mutation: { id: "tag-south", tag: tag("w-south") }, outcome: reject("mutation.duplicate-id", ["tag-south"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", tag: tag("w-south") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: library(), mutation: { id: "tag-south", tag: tag("w-south", { storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["tag", "storey"]) },
      { name: "element-missing", before: library(), mutation: { id: "tag-south", tag: tag("w-ghost") }, outcome: reject("mutation.target-missing", ["tag", "element"]) },
      { name: "style-missing", before: library(), mutation: { id: "tag-south", tag: tag("w-south", { style: "as-ghost" }) }, outcome: reject("mutation.target-missing", ["tag", "style"]) },
    ]),
  }),
  leaf({
    kind: "delete-tag", emoji: 0x1f3ab, variant: "DeleteTag", verb: "delete", entity: "tag", binaryTag: 11004, displayName: "Delete Tag",
    doc: "Removes a tag; the element it read stays untouched.",
    props: [id("tag", "target", tagLabel)],
    label: { en: 'format!("Delete tag \\"{}\\"", self.id)', de: 'format!("Kennzeichnung \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withTag(), mutation: { id: "tag-south" }, outcome: ok },
      { name: "missing", before: withTag(), mutation: { id: "tag-east" }, outcome: reject("mutation.target-missing", ["tag-east"]) },
    ]),
  }),
  leaf({
    kind: "set-tag", emoji: 0x1f3f4, variant: "SetTag", verb: "set", entity: "tag", binaryTag: 11005, displayName: "Set Tag",
    doc: "Sets exactly the provided fields of a tag: element, category, offset from the element and style; absent fields stay untouched.",
    props: [
      id("tag", "target", tagLabel),
      sparse(reference("element", "element", { en: "Element", de: "Element" }, 20), keep),
      sparse(choice("category", "TagCategory", { en: "Category", de: "Kategorie" }, 30), keep),
      sparse(point("offset", { en: "Offset", de: "Abstand" }, 40), keep),
      sparse(reference("style", "annotation-style", styleKind, 50), keep),
    ],
    label: { en: 'format!("Change tag \\"{}\\"", self.id)', de: 'format!("Kennzeichnung \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "reads-the-type", before: withTag(), mutation: { id: "tag-south", category: "Type" }, outcome: ok },
      { name: "retargets-and-moves", before: withTag(), mutation: { id: "tag-south", element: "w-east", offset: P(0.4, 0) }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withTag(), mutation: { id: "tag-south", category: "Number", style: "as-plan" }, outcome: ok },
      { name: "unchanged", before: withTag(), mutation: { id: "tag-south", category: "Name", element: "w-south" }, outcome: reject("mutation.no-op", ["tag-south"]) },
      { name: "names-no-field", before: withTag(), mutation: { id: "tag-south" }, outcome: reject("mutation.no-op", ["tag-south"]) },
      { name: "element-missing", before: withTag(), mutation: { id: "tag-south", element: "w-ghost" }, outcome: reject("mutation.target-missing", ["element"]) },
      { name: "style-missing", before: withTag(), mutation: { id: "tag-south", style: "as-ghost" }, outcome: reject("mutation.target-missing", ["style"]) },
      { name: "missing", before: withTag(), mutation: { id: "tag-east", category: "Type" }, outcome: reject("mutation.target-missing", ["tag-east"]) },
    ]),
  }),
];

const noteLeaves: MineLeaf[] = [
  leaf({
    kind: "create-text-note", emoji: 0x1f5d2, variant: "CreateTextNote", verb: "create", entity: "text-note", binaryTag: 11006, displayName: "Create Text Note",
    doc: "Brings a free text onto a storey plan at a position, rotated and set in an annotation style.",
    props: [id("text-note", "identity", { en: "Text note id", de: "Textnotiz-Id" }, 10), recordProp("text_note", "TextNote", noteLabel)],
    label: { en: 'format!("Create text note \\"{}\\"", self.id)', de: 'format!("Textnotiz \\"{}\\" anlegen", self.id)' },
    target,
    cases: cases([
      { name: "adds", before: library(), mutation: { id: "note-1", text_note: note() }, outcome: ok },
      { name: "adds-rotated", before: library(), mutation: { id: "note-2", text_note: note({ text: "Fire wall", rotation: 1.5707963267948966, position: P(8.5, 3) }) }, outcome: ok },
      { name: "duplicate", before: withNote(), mutation: { id: "note-1", text_note: note() }, outcome: reject("mutation.duplicate-id", ["note-1"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", text_note: note() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: library(), mutation: { id: "note-1", text_note: note({ storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["text_note", "storey"]) },
      { name: "style-missing", before: library(), mutation: { id: "note-1", text_note: note({ style: "as-ghost" }) }, outcome: reject("mutation.target-missing", ["text_note", "style"]) },
      { name: "blank", before: library(), mutation: { id: "note-1", text_note: note({ text: "  " }) }, outcome: reject("mutation.invariant", ["text_note", "text"]) },
    ]),
  }),
  leaf({
    kind: "delete-text-note", emoji: 0x1f4c3, variant: "DeleteTextNote", verb: "delete", entity: "text-note", binaryTag: 11007, displayName: "Delete Text Note",
    doc: "Removes a text note.",
    props: [id("text-note", "target", noteLabel)],
    label: { en: 'format!("Delete text note \\"{}\\"", self.id)', de: 'format!("Textnotiz \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withNote(), mutation: { id: "note-1" }, outcome: ok },
      { name: "missing", before: withNote(), mutation: { id: "note-2" }, outcome: reject("mutation.target-missing", ["note-2"]) },
    ]),
  }),
  leaf({
    kind: "set-text-note", emoji: 0x1f4dd, variant: "SetTextNote", verb: "set", entity: "text-note", binaryTag: 11008, displayName: "Set Text Note",
    doc: "Sets exactly the provided fields of a text note: position, text, rotation and style; absent fields stay untouched.",
    props: [
      id("text-note", "target", noteLabel),
      sparse(point("position", { en: "Position", de: "Position" }, 20), keep),
      sparse(text("text", { en: "Text", de: "Text" }, 30), keep),
      sparse(angle("rotation", { en: "Rotation (rad)", de: "Drehung (rad)" }, 40), keep),
      sparse(reference("style", "annotation-style", styleKind, 50), keep),
    ],
    label: { en: 'format!("Change text note \\"{}\\"", self.id)', de: 'format!("Textnotiz \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "rewrites", before: withNote(), mutation: { id: "note-1", text: "Verified" }, outcome: ok },
      { name: "moves-and-turns", before: withNote(), mutation: { id: "note-1", position: P(5, 1), rotation: 0.5 }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withNote(), mutation: { id: "note-1", rotation: 0, text: "Verify again" }, outcome: ok },
      { name: "unchanged", before: withNote(), mutation: { id: "note-1", text: "Verify on site" }, outcome: reject("mutation.no-op", ["note-1"]) },
      { name: "names-no-field", before: withNote(), mutation: { id: "note-1" }, outcome: reject("mutation.no-op", ["note-1"]) },
      { name: "blank", before: withNote(), mutation: { id: "note-1", text: "" }, outcome: reject("mutation.invariant", ["text"]) },
      { name: "style-missing", before: withNote(), mutation: { id: "note-1", style: "as-ghost" }, outcome: reject("mutation.target-missing", ["style"]) },
      { name: "missing", before: withNote(), mutation: { id: "note-2", text: "x" }, outcome: reject("mutation.target-missing", ["note-2"]) },
    ]),
  }),
];

const leaderLeaves: MineLeaf[] = [
  leaf({
    kind: "create-leader", emoji: 0x2197, variant: "CreateLeader", verb: "create", entity: "leader", binaryTag: 11009, displayName: "Create Leader",
    doc: "Brings a new leader onto a storey plan: a text joined by a line to an anchor (a free point or an element of the model), placed beside the anchor so it follows the element.",
    props: [id("leader", "identity", { en: "Leader id", de: "Hinweislinien-Id" }, 10), recordProp("leader", "Leader", leaderLabel)],
    label: { en: 'format!("Create leader \\"{}\\"", self.id)', de: 'format!("Hinweislinie \\"{}\\" anlegen", self.id)' },
    target,
    cases: cases([
      { name: "points-at-a-wall-face", before: library(), mutation: { id: "lead-1", leader: leader(face("w-south", "Right")) }, outcome: ok },
      { name: "points-at-a-free-point", before: library(), mutation: { id: "lead-2", leader: leader(free(4, 3), { text: "Shaft" }) }, outcome: ok },
      { name: "duplicate", before: withLeader(), mutation: { id: "lead-1", leader: leader(face("w-south", "Right")) }, outcome: reject("mutation.duplicate-id", ["lead-1"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", leader: leader(face("w-south", "Right")) }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: library(), mutation: { id: "lead-1", leader: leader(face("w-south", "Right"), { storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["leader", "storey"]) },
      { name: "anchor-missing", before: library(), mutation: { id: "lead-1", leader: leader(axisOf("w-ghost")) }, outcome: reject("mutation.target-missing", ["leader", "anchor"]) },
      { name: "style-missing", before: library(), mutation: { id: "lead-1", leader: leader(face("w-south", "Right"), { style: "as-ghost" }) }, outcome: reject("mutation.target-missing", ["leader", "style"]) },
      { name: "blank", before: library(), mutation: { id: "lead-1", leader: leader(face("w-south", "Right"), { text: " " }) }, outcome: reject("mutation.invariant", ["leader", "text"]) },
    ]),
  }),
  leaf({
    kind: "delete-leader", emoji: 0x21aa, variant: "DeleteLeader", verb: "delete", entity: "leader", binaryTag: 11010, displayName: "Delete Leader",
    doc: "Removes a leader; the element it pointed at stays untouched.",
    props: [id("leader", "target", leaderLabel)],
    label: { en: 'format!("Delete leader \\"{}\\"", self.id)', de: 'format!("Hinweislinie \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withLeader(), mutation: { id: "lead-1" }, outcome: ok },
      { name: "missing", before: withLeader(), mutation: { id: "lead-2" }, outcome: reject("mutation.target-missing", ["lead-2"]) },
    ]),
  }),
  leaf({
    kind: "set-leader", emoji: 0x2934, variant: "SetLeader", verb: "set", entity: "leader", binaryTag: 11011, displayName: "Set Leader",
    doc: "Sets exactly the provided fields of a leader: anchor, offset of the text from the anchor, text and style; absent fields stay untouched.",
    props: [
      id("leader", "target", leaderLabel),
      sparse(anchorProp(20), keep),
      sparse(point("offset", { en: "Offset", de: "Abstand" }, 30), keep),
      sparse(text("text", { en: "Text", de: "Text" }, 40), keep),
      sparse(reference("style", "annotation-style", styleKind, 50), keep),
    ],
    label: { en: 'format!("Change leader \\"{}\\"", self.id)', de: 'format!("Hinweislinie \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "rewrites", before: withLeader(), mutation: { id: "lead-1", text: "Brick 365" }, outcome: ok },
      { name: "re-anchors", before: withLeader(), mutation: { id: "lead-1", anchor: face("w-east", "Left"), offset: P(-1, 0.5) }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withLeader(), mutation: { id: "lead-1", text: "Brick 300", offset: P(2, 2) }, outcome: ok },
      { name: "unchanged", before: withLeader(), mutation: { id: "lead-1", text: "Brick 300" }, outcome: reject("mutation.no-op", ["lead-1"]) },
      { name: "names-no-field", before: withLeader(), mutation: { id: "lead-1" }, outcome: reject("mutation.no-op", ["lead-1"]) },
      { name: "anchor-missing", before: withLeader(), mutation: { id: "lead-1", anchor: centre("o-ghost") }, outcome: reject("mutation.target-missing", ["anchor"]) },
      { name: "blank", before: withLeader(), mutation: { id: "lead-1", text: "" }, outcome: reject("mutation.invariant", ["text"]) },
      { name: "missing", before: withLeader(), mutation: { id: "lead-2", text: "x" }, outcome: reject("mutation.target-missing", ["lead-2"]) },
    ]),
  }),
];

const styleLeaves: MineLeaf[] = [
  leaf({
    kind: "create-annotation-style", emoji: 0x1f3a8, variant: "CreateAnnotationStyle", verb: "create", entity: "annotation-style", binaryTag: 11012, displayName: "Create Annotation Style",
    doc: "Brings a new annotation style into the library: text height, line end mark, printed unit and precision shared by dimensions, tags, notes and leaders.",
    props: [id("annotation-style", "identity", { en: "Style id", de: "Stil-Id" }, 10), recordProp("annotation_style", "AnnotationStyle", styleKind)],
    label: { en: 'format!("Create annotation style \\"{}\\"", self.annotation_style.name)', de: 'format!("Beschriftungsstil \\"{}\\" anlegen", self.annotation_style.name)' },
    target,
    cases: cases([
      { name: "adds", before: F.scene(), mutation: { id: "as-plan", annotation_style: style("Plan 1:50") }, outcome: ok },
      { name: "adds-a-millimetre-style", before: library(), mutation: { id: "as-mm", annotation_style: { ...style("Millimetres"), unit: "Millimetre", precision: 0, terminator: "Arrow" } }, outcome: ok },
      { name: "duplicate", before: library(), mutation: { id: "as-plan", annotation_style: style("Again") }, outcome: reject("mutation.duplicate-id", ["as-plan"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", annotation_style: style("Wall") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", before: library(), mutation: { id: "as-new", annotation_style: style(" ") }, outcome: reject("mutation.invariant", ["annotation_style", "name"]) },
      { name: "text-height-not-positive", before: library(), mutation: { id: "as-new", annotation_style: style("Flat", 0) }, outcome: reject("mutation.invariant", ["annotation_style", "text_height"]) },
      { name: "too-many-decimals", before: library(), mutation: { id: "as-new", annotation_style: { ...style("Precise"), precision: 9 } }, outcome: reject("mutation.invariant", ["annotation_style", "precision"]) },
    ]),
  }),
  leaf({
    kind: "delete-annotation-style", emoji: 0x1f9fa, variant: "DeleteAnnotationStyle", verb: "delete", entity: "annotation-style", binaryTag: 11013, displayName: "Delete Annotation Style",
    doc: "Removes an annotation style that no dimension, tag, text note or leader uses.",
    props: [id("annotation-style", "target", styleKind)],
    label: { en: 'format!("Delete annotation style \\"{}\\"", self.id)', de: 'format!("Beschriftungsstil \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: withDimension(), mutation: { id: "as-large" }, outcome: ok },
      { name: "used-by-dimensions", before: withDimensionOfStyle(), mutation: { id: "as-large" }, outcome: reject("mutation.target-referenced", ["as-large"]) },
      { name: "used-by-tags", before: withTagOfStyle(), mutation: { id: "as-large" }, outcome: reject("mutation.target-referenced", ["as-large"]) },
      { name: "used-by-text-notes", before: withNoteOfStyle(), mutation: { id: "as-large" }, outcome: reject("mutation.target-referenced", ["as-large"]) },
      { name: "used-by-leaders", before: withLeaderOfStyle(), mutation: { id: "as-large" }, outcome: reject("mutation.target-referenced", ["as-large"]) },
      { name: "missing", before: library(), mutation: { id: "as-ghost" }, outcome: reject("mutation.target-missing", ["as-ghost"]) },
    ]),
  }),
  leaf({
    kind: "set-annotation-style", emoji: 0x1f58a, variant: "SetAnnotationStyle", verb: "set", entity: "annotation-style", binaryTag: 11014, displayName: "Set Annotation Style",
    doc: "Sets exactly the provided fields of an annotation style; every dimension, tag, note and leader that uses the style follows by inference.",
    props: [
      id("annotation-style", "target", styleKind),
      sparse(text("name", { en: "Name", de: "Name" }, 20), keep),
      sparse(length("text_height", { en: "Text height (m)", de: "Texthöhe (m)" }, 30), keep),
      sparse(choice("terminator", "Terminator", { en: "Line end", de: "Linienende" }, 40), keep),
      sparse(choice("unit", "DimensionUnit", { en: "Unit", de: "Einheit" }, 50), keep),
      sparse(count("precision", { en: "Decimals", de: "Nachkommastellen" }, 60), keep),
      sparse(length("mark_size", { en: "Mark size (m)", de: "Markengröße (m)" }, 70), keep),
      sparse(length("gap", { en: "Extension gap (m)", de: "Hilfslinienabstand (m)" }, 80), keep),
      sparse(length("overshoot", { en: "Extension overshoot (m)", de: "Hilfslinienüberstand (m)" }, 90), keep),
    ],
    label: { en: 'format!("Change annotation style \\"{}\\"", self.id)', de: 'format!("Beschriftungsstil \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "scales-the-text", before: library(), mutation: { id: "as-plan", text_height: 0.35 }, outcome: ok },
      { name: "prints-centimetres", before: library(), mutation: { id: "as-plan", unit: "Centimetre", precision: 1, terminator: "Dot" }, outcome: ok },
      { name: "renames", before: library(), mutation: { id: "as-plan", name: "Plan 1:50 (renamed)" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: library(), mutation: { id: "as-plan", precision: 2, gap: 0.2 }, outcome: ok },
      { name: "unchanged", before: library(), mutation: { id: "as-plan", precision: 2, unit: "Metre" }, outcome: reject("mutation.no-op", ["as-plan"]) },
      { name: "names-no-field", before: library(), mutation: { id: "as-plan" }, outcome: reject("mutation.no-op", ["as-plan"]) },
      { name: "text-height-not-positive", before: library(), mutation: { id: "as-plan", text_height: -0.1 }, outcome: reject("mutation.invariant", ["text_height"]) },
      { name: "too-many-decimals", before: library(), mutation: { id: "as-plan", precision: 7 }, outcome: reject("mutation.invariant", ["precision"]) },
      { name: "missing", before: library(), mutation: { id: "as-ghost", precision: 1 }, outcome: reject("mutation.target-missing", ["as-ghost"]) },
    ]),
  }),
];
void donor;

export const leaves: MineLeaf[] = [...styleLeaves, ...dimensionLeaves, ...tagLeaves, ...noteLeaves, ...leaderLeaves];

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...parts: unknown[]) => strings.reduce((out, chunk, index) => out + chunk + (index < parts.length ? String(parts[index]) : ""), "").replaceAll("¶", "`");

type Entity = { V: string; field: string; coll: string; human: string; rules: string; call: string; patch?: { type: string; build: string; from: string } };
const ENTITIES: Record<string, Entity> = {
  dimension: {
    V: "Dimension", field: "dimension", coll: "dimensions", human: "Dimension",
    rules: "The storey and the style must exist, there are at least two anchors that name existing elements or finite points, the direction and offset are finite and a lock is a positive length.",
    call: "annotating::dimension_fault(base, &payload.dimension)",
    patch: { type: "DimensionPatch", build: "DimensionPatch { anchors: self.anchors.clone(), angle: self.angle, offset: self.offset, style: self.style.clone(), lock: self.lock.clone(), name: self.name.clone(), ..Default::default() }", from: "Self { id, anchors: patch.anchors, angle: patch.angle, offset: patch.offset, style: patch.style, lock: patch.lock, name: patch.name }" },
  },
  tag: {
    V: "Tag", field: "tag", coll: "tags", human: "Tag",
    rules: "The storey and the style must exist, the element is one that can be tagged and the offset is finite.",
    call: "annotating::tag_fault(base, &payload.tag)",
    patch: { type: "TagPatch", build: "TagPatch { element: self.element.clone(), category: self.category, offset: self.offset, style: self.style.clone(), ..Default::default() }", from: "Self { id, element: patch.element, category: patch.category, offset: patch.offset, style: patch.style }" },
  },
  "text-note": {
    V: "TextNote", field: "text_note", coll: "text_notes", human: "Text note",
    rules: "The storey and the style must exist, the position and rotation are finite and the text is not blank.",
    call: "annotating::note_fault(base, &payload.text_note)",
    patch: { type: "TextNotePatch", build: "TextNotePatch { position: self.position, text: self.text.clone(), rotation: self.rotation, style: self.style.clone(), ..Default::default() }", from: "Self { id, position: patch.position, text: patch.text, rotation: patch.rotation, style: patch.style }" },
  },
  leader: {
    V: "Leader", field: "leader", coll: "leaders", human: "Leader",
    rules: "The storey and the style must exist, the anchor names an existing element or a finite point, the offset is finite and the text is not blank.",
    call: "annotating::leader_fault(base, &payload.leader)",
    patch: { type: "LeaderPatch", build: "LeaderPatch { anchor: self.anchor.clone(), offset: self.offset, text: self.text.clone(), style: self.style.clone(), ..Default::default() }", from: "Self { id, anchor: patch.anchor, offset: patch.offset, text: patch.text, style: patch.style }" },
  },
  "annotation-style": {
    V: "AnnotationStyle", field: "annotation_style", coll: "annotation_styles", human: "Annotation style",
    rules: "The name is not blank, the text height is positive, mark size, gap and overshoot are lengths of zero or more and at most six decimals are printed.",
    call: "annotating::style_record_fault(&payload.annotation_style)",
    patch: { type: "AnnotationStylePatch", build: "AnnotationStylePatch { name: self.name.clone(), text_height: self.text_height, terminator: self.terminator, unit: self.unit, precision: self.precision, mark_size: self.mark_size, gap: self.gap, overshoot: self.overshoot }", from: "Self { id, name: patch.name, text_height: patch.text_height, terminator: patch.terminator, unit: patch.unit, precision: patch.precision, mark_size: patch.mark_size, gap: patch.gap, overshoot: patch.overshoot }" },
  },
};

const handCreate = (e: Entity) => ({
  diff: rs`//! ${DIFF} Diff constructor for ¶Create${e.V}¶: one created ${e.human.toLowerCase()} entry. The id is free in every collection. ${e.rules}
//! Everything the ${e.human.toLowerCase()} shows is inferred from the current geometry of what it names, never stored.

use super::super::annotating;
use super::super::elements;
use super::Create${e.V};
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Create${e.V}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = ${e.call} {
        let path = fault.path(Some("${e.field}"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::${e.coll}(payload.id.clone(), Entry::Created(payload.${e.field}.clone())))
}
`,
  inverse: rs`//! ${INVERSE} Inverse of ¶Create${e.V}¶: the concrete ¶Delete${e.V}¶ of the id it created, none when the id was already taken.

use super::super::delete_${snake(e.field)}::Delete${e.V};
use super::Create${e.V};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Create${e.V}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${e.coll}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::Delete${e.V}(Delete${e.V} { id: payload.id.clone() })]
}
`,
});

const handDelete = (e: Entity) => ({
  diff: rs`//! ${DIFF} Diff constructor for ¶Delete${e.V}¶: the ${e.human.toLowerCase()} leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use super::super::cascade;
use super::Delete${e.V};
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Delete${e.V}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${e.coll}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${e.human} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "${e.human}", Some(&payload.id))
}
`,
  inverse: rs`//! ${INVERSE} Inverse of ¶Delete${e.V}¶: the concrete ¶Create${e.V}¶ carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::Delete${e.V};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Delete${e.V}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
});

const handDeleteStyle = (e: Entity) => ({
  diff: rs`//! ${DIFF} Diff constructor for ¶Delete${e.V}¶: one deleted style entry; refused while a dimension, tag, text note or leader still uses it.

use super::Delete${e.V};
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Delete${e.V}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.annotation_styles.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${e.human} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let used = base.dimensions.values().any(|row| row.style == payload.id)
        || base.tags.values().any(|row| row.style == payload.id)
        || base.text_notes.values().any(|row| row.style == payload.id)
        || base.leaders.values().any(|row| row.style == payload.id);
    if used {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("${e.human} \\"{}\\" is still used by annotations.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::annotation_styles(payload.id.clone(), Entry::Deleted))
}
`,
  inverse: rs`//! ${INVERSE} Inverse of ¶Delete${e.V}¶: the concrete ¶Create${e.V}¶ carrying the full removed record, none when the style was absent.

use super::super::create_annotation_style::CreateAnnotationStyle;
use super::Delete${e.V};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Delete${e.V}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.annotation_styles.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateAnnotationStyle(CreateAnnotationStyle { id: payload.id.clone(), annotation_style: record.clone() })],
        None => Vec::new(),
    }
}
`,
});

const handSet = (e: Entity) => {
  const check = e.V === "AnnotationStyle" ? `annotating::style_record_fault(&next)` : `annotating::${e.field === "text_note" ? "note" : e.field}_fault(base, &next)`;
  return {
    diff: rs`//! ${DIFF} Diff constructor for ¶Set${e.V}¶: a sparse ${e.human.toLowerCase()} patch of exactly the provided fields that differ. The record that results must break none of the create rules: ${e.rules.charAt(0).toLowerCase() + e.rules.slice(1)}
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::Set${e.V};
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Set${e.V}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.${e.coll}.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${e.human} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = ${check} {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("${e.human} \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::${e.coll}(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶Set${e.V}¶: an absolute ¶Set${e.V}¶ restoring the base value of exactly the fields the forward really changes, none when the ${e.human.toLowerCase()} is absent or nothing changes.

use super::Set${e.V};
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &Set${e.V}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.${e.coll}.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::Set${e.V}(Set${e.V}::from_patch(payload.id.clone(), restore))]
}
`,
  };
};

const hand = (leaf: MineLeaf) => {
  const [verb, ...rest] = leaf.kind.split("-");
  const entity = ENTITIES[rest.join("-")];
  if (verb === "create") return handCreate(entity);
  if (verb === "set") return handSet(entity);
  return entity.V === "AnnotationStyle" ? handDeleteStyle(entity) : handDelete(entity);
};

const readdirName = (dir: string, suffix: string) => readdirSync(dir).find((name) => name.endsWith(suffix))!;

function fixLeaf(spec: MineLeaf) {
  const optionals = extras.get(spec.kind)!.optional;
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const file = join(dir, MUTATION + "mutation", RS);
  const [verb, ...rest] = spec.kind.split("-");
  const entity = ENTITIES[rest.join("-")];
  const patch = verb === "set" ? entity.patch : undefined;
  const used = new Set(spec.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  if (patch) used.add(patch.type);
  let source = readFileSync(file, "utf8");
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) source = source.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  if (patch) {
    source = source.replace(
      /^impl MutationKind/m,
      `impl ${spec.variant} {\n    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.\n    pub fn patch(&self) -> ${patch.type} {\n        ${patch.build}\n    }\n\n    /// 🧩 The payload that provides exactly the fields \`patch\` names.\n    pub fn from_patch(id: String, patch: ${patch.type}) -> Self {\n        ${patch.from}\n    }\n}\n\nimpl MutationKind`,
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

function writeOnce(spec: MineLeaf) {
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const body = hand(spec);
  for (const [folder, text] of [[DIFF + "diff", body.diff], [INVERSE + "inverse", body.inverse]] as const) {
    mkdirSync(join(dir, folder), { recursive: true });
    const file = join(dir, folder, RS);
    if (!existsSync(file)) writeFileSync(file, text);
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
console.log(`w11-annotations: ${leaves.length} leaves emitted, ${added} newly mounted`);
//#endregion 🔖️Run
