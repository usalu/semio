#!/usr/bin/env bun
/**
 * 🪄️ Wave W05 (label `w05-modify`, binary tags 5000 to 5099): the modify toolset kernel of `s.bim.model@1`. `bun r10-w05-leaves.ts` rewrites
 * the boilerplate and fixtures of the nine leaves (never the hand-written `🔺️diff`/`↩️inverse`, never a blessed `after`/`diff`) and prints
 * nothing else; `bun r10-w05-leaves.ts register` also splices the mount blocks, the enum variants and the KINDS rows into the shared
 * registration files (idempotent, surgical). Every `before` snapshot is cut out of the committed `move-elements` fixture that holds one element
 * of every placed kind, so it follows the schema the other waves migrate the fixtures to.
 */
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, relRoot, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";
import { caseDirs, read } from "./r6-z-mutations-cases.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const MODIFY = "super::super::modify";

//#region 🔖️Props
const idProp = (kind: string, label: Label, order = 10, name = "id"): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "target", label, ref: { kind }, group: "target", order } });
const idsProp: Prop = {
  name: "ids",
  rust: "Vec<String>",
  schema: { type: "array", items: { type: "string" } },
  ui: { widget: "reference", role: "target", label: { en: "Elements", de: "Elemente" }, description: { en: "Elements to act on; unknown ids refuse the whole call, openings are acted on with their host.", de: "Zu bearbeitende Elemente; unbekannte Ids verweigern den ganzen Aufruf, Öffnungen werden mit ihrem Host bearbeitet." }, ref: { kind: "element" }, group: "target", order: 10 },
};
const identity = (name: string, label: Label, order: number, description?: Label): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "identity", label, ...(description ? { description } : {}), group: "identity", order } });
const prefixProp = (order = 30): Prop =>
  identity("prefix", { en: "Id prefix", de: "Id-Präfix" }, order, {
    en: "Prefix of the ids minted for the created records: prefix, copy number and the position of the source in id order.",
    de: "Präfix der Ids der angelegten Datensätze: Präfix, Kopiennummer und Position der Quelle in Id-Reihenfolge.",
  });
const point = (name: string, label: Label, group: string, order: number, description?: Label): Prop => ({ name, rust: "Point2", schema: ref("Point2"), ui: { widget: "record", role: "value", label, ...(description ? { description } : {}), group, order } });
const num = (name: string, label: Label, order: number, group = "value", description?: Label): Prop => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, ...(description ? { description } : {}), group, order } });
const choice = (name: string, rust: string, values: string[], label: Label, order: number): Prop => ({ name, rust, schema: { type: "string", enum: values }, ui: { widget: "select", role: "value", label, group: "value", order } });
const wallEnd = (name = "end", order = 20): Prop => choice(name, `${MODIFY}::WallEnd`, ["Start", "End"], { en: "Wall end", de: "Wandende" }, order);
const pattern: Prop = {
  name: "pattern",
  rust: `${MODIFY}::ArrayPattern`,
  schema: {
    type: "object",
    oneOf: [
      { type: "object", additionalProperties: false, required: ["Linear"], properties: { Linear: { type: "object", additionalProperties: false, required: ["count", "spacing"], properties: { count: { type: "integer" }, spacing: ref("Point2") } } } },
      { type: "object", additionalProperties: false, required: ["Radial"], properties: { Radial: { type: "object", additionalProperties: false, required: ["count", "center", "step"], properties: { count: { type: "integer" }, center: ref("Point2"), step: { type: "number" } } } } },
    ],
  },
  ui: {
    widget: "record",
    role: "value",
    label: { en: "Pattern", de: "Muster" },
    description: { en: "Linear: count copies, each one further by the spacing (m). Radial: count copies, each one further by the step (rad) about the centre (m).", de: "Linear: Anzahl Kopien, jede um den Abstand (m) weiter. Radial: Anzahl Kopien, jede um den Schritt (rad) weiter um den Mittelpunkt (m)." },
    group: "pattern",
    order: 20,
  },
};
const joinProp: Prop = {
  name: "join",
  rust: "Option<crate::EndJoin>",
  schema: ref("EndJoin"),
  ui: { widget: "select", role: "value", label: { en: "Join (empty = automatic)", de: "Verbindung (leer = automatisch)" }, group: "value", order: 30 },
};
const optionalPrefix: Prop = { ...prefixProp(40), name: "prefix", rust: "Option<String>", ui: { ...prefixProp(40).ui, description: { en: "When set, the mirror images are created as copies with ids minted from this prefix and the originals stay; when empty, the elements are mirrored where they stand.", de: "Wenn gesetzt, werden die Spiegelbilder als Kopien mit Ids aus diesem Präfix angelegt und die Originale bleiben; wenn leer, werden die Elemente an Ort und Stelle gespiegelt." } } };
//#endregion 🔖️Props

//#region 🔖️Outcomes
const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const missing = (...path: string[]) => reject("mutation.target-missing", path);
const invariant = (...path: string[]) => reject("mutation.invariant", path);
const noop = (...path: string[]) => reject("mutation.no-op", path);
const duplicate = (...path: string[]) => reject("mutation.duplicate-id", path);
//#endregion 🔖️Outcomes

//#region 🔖️Fixtures
const EVERY = JSON.parse(read("move-elements", caseDirs("move-elements").find((name) => name.endsWith("moves-every-placed-kind"))!, "before"));
const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value));
const ELEMENTS = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "grids"];
const P = (x: number, y: number) => ({ x, y });
const V = (x: number, y: number, bulge = 0) => ({ point: P(x, y), bulge });
const box = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
const line = (a: [number, number], b: [number, number]) => ({ Line: { start: P(...a), end: P(...b) } });
const arc = (a: [number, number], b: [number, number], bulge: number) => ({ Arc: { start: P(...a), end: P(...b), bulge } });
const door = { name: "Door", width: 0.9, height: 2.1, frame_width: 0.07, frame_depth: 0.12, leaves: "Single", swing: "Left", material: "m-brick" };
const opening = (host: string, kind: "Door" | "Window", offset: number, name: string, over: Record<string, unknown> = {}) => ({ host, kind: kind === "Door" ? { Door: { door_type: "dt-1" } } : { Window: { window_type: "win-1" } }, offset, flip_hand: false, flip_facing: false, name, ...over });

/** 🏡️ The committed every-kind house cut down to the listed element ids, with the door type, the extras and no data. */
const scene = (keep: Record<string, string[]>, extra: Record<string, Record<string, unknown>> = {}, over: Record<string, unknown> = {}) => {
  const snapshot = clone(EVERY);
  snapshot.door_types = { "dt-1": door };
  for (const collection of ELEMENTS) snapshot[collection] = { ...Object.fromEntries(Object.entries(snapshot[collection] ?? {}).filter(([id]) => (keep[collection] ?? []).includes(id))), ...(extra[collection] ?? {}) };
  snapshot.properties = {};
  snapshot.classifications = {};
  return Object.assign(snapshot, over);
};
const wallRow = (axis: unknown, name: string, over: Record<string, unknown> = {}) => ({ ...clone(EVERY.walls["w-south"]), axis, name, ...over });
const withData = { properties: { "w-south": { Pset_WallCommon: { FireRating: { Text: { value: "F90" } } } }, "o-window": { Pset_WindowCommon: { ThermalTransmittance: { Real: { value: 1.1 } } } } }, classifications: { "w-south": { system: "Uniclass", code: "EF_25_10", title: "Walls" } } };
const hosted = (over: Record<string, unknown> = {}) => scene({ walls: ["w-south", "w-east"] }, { openings: { "o-door": opening("w-south", "Door", 1.5, "Door"), "o-window": opening("w-south", "Window", 5.5, "Window") } }, over);
const ALL = ["w-south", "w-east", "cw-1", "c-1", "b-1", "sl-1", "rf-1", "s-1", "r-1", "sp-1", "g-a", "o-1"];
const everything = (over: Record<string, unknown> = {}) => scene({ walls: ["w-south", "w-east"], curtain_walls: ["cw-1"], columns: ["c-1"], beams: ["b-1"], slabs: ["sl-1"], roofs: ["rf-1"], openings: ["o-1"], stairs: ["s-1"], railings: ["r-1"], spaces: ["sp-1"], grids: ["g-a"] }, {}, over);
const slab = (boundary: unknown[], over: Record<string, unknown> = {}) => ({ ...clone(EVERY.slabs["sl-1"]), boundary, holes: [], slope: undefined, name: "Floor", ...over });
const slabScene = (row: Record<string, unknown>, id = "sl-1") => scene({}, { slabs: { [id]: row } });
//#endregion 🔖️Fixtures

//#region 🔖️Leaves
const one = "vec![self.id.clone()]";
const many = "self.ids.clone()";
const e = { ok: 0x2705, arc: 0x1f300, no: 0x1f6ab, stop: 0x26d4, missing: 0x1f573, same: 0x1f9f2, zero: 0x1f6d1, other: 0x1f6a7, taken: 0x1f9e9, kinds: 0x1f69b, data: 0x1f517, star: 0x1f31f, pin: 0x1f4cc, hand: 0x1fa9d, id: 0x1f9ed, hole: 0x1f9f1 };

export const leaves: Leaf[] = [
  {
    kind: "set-wall-end-join", emoji: 0x1f9f7, variant: "SetWallEndJoin", verb: "set", entity: "wall", displayName: "Set Wall End Join", binaryTag: 5000,
    doc: "Sets the authored join preference of one end of a wall: mitered, butted or not joined; an absent join returns the end to the geometry (automatic). Only the wall layout reads it, nothing is derived here.",
    props: [idProp("wall", { en: "Wall", de: "Wand" }), wallEnd(), joinProp],
    label: { en: 'format!("Set the {:?} join of wall \\"{}\\" to {}", self.end, self.id, self.join.map_or("automatic".to_string(), |join| format!("{join:?}")))', de: 'format!("Verbindung am Wandende {:?} von \\"{}\\" auf {} setzen", self.end, self.id, self.join.map_or("automatisch".to_string(), |join| format!("{join:?}")))' },
    target: one,
    cases: [
      { name: "butts-the-start", emoji: e.ok, before: scene({ walls: ["w-south", "w-east"] }), mutation: { id: "w-south", end: "Start", join: "Butt" }, outcome: ok },
      { name: "miters-the-end", emoji: e.arc, before: scene({ walls: ["w-south", "w-east"] }), mutation: { id: "w-east", end: "End", join: "Miter" }, outcome: ok },
      { name: "frees-the-end", emoji: e.star, before: scene({ walls: ["w-south", "w-east"] }), mutation: { id: "w-east", end: "Start", join: "None" }, outcome: ok },
      { name: "back-to-automatic", emoji: e.kinds, before: scene({}, { walls: { "w-south": { ...wallRow(line([0, 0], [8, 0]), "South"), end_join: "Butt" }, "w-east": wallRow(line([8, 0], [8, 6]), "East") } }), mutation: { id: "w-south", end: "End" }, outcome: ok },
      { name: "unchanged", emoji: e.same, before: scene({ walls: ["w-south"] }), mutation: { id: "w-south", end: "Start" }, outcome: noop("w-south") },
      { name: "missing", emoji: e.missing, before: scene({ walls: ["w-south"] }), mutation: { id: "w-attic", end: "Start", join: "Butt" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "copy-elements", emoji: 0x1f46f, variant: "CopyElements", verb: "duplicate", entity: "elements", displayName: "Copy Elements", binaryTag: 5001,
    doc: "Copies placed elements by a vector: a created record per source with minted ids (prefix, copy number, position of the source in id order), the openings of a copied wall or curtain wall hosted on its copy, and the properties and classification of every source. A copied space takes the next free number of its storey and a copied grid line the next free label of its building.",
    props: [idsProp, point("vector", { en: "Translation (m)", de: "Verschiebung (m)" }, "translation", 20), prefixProp()],
    label: { en: 'format!("Copy {} element(s) by ({}, {}) m", self.ids.len(), self.vector.x, self.vector.y)', de: 'format!("{} Element(e) um ({}, {}) m kopieren", self.ids.len(), self.vector.x, self.vector.y)' },
    target: many,
    cases: [
      { name: "copies-a-wall-with-its-openings", emoji: e.ok, before: hosted(withData), mutation: { ids: ["w-south"], vector: P(0, -4), prefix: "cp" }, outcome: ok },
      { name: "copies-every-placed-kind", emoji: e.kinds, before: everything({ ...withData, properties: { "w-south": withData.properties["w-south"], "o-1": withData.properties["o-window"] } }), mutation: { ids: ALL, vector: P(0.5, 10), prefix: "cp" }, outcome: ok },
      { name: "copies-in-place", emoji: e.same, before: hosted(), mutation: { ids: ["w-east"], vector: P(0, 0), prefix: "cp" }, outcome: ok },
      { name: "empty-selection", emoji: e.stop, before: hosted(), mutation: { ids: [], vector: P(1, 0), prefix: "cp" }, outcome: invariant("ids") },
      { name: "unknown-id", emoji: e.no, before: hosted(), mutation: { ids: ["w-south", "w-missing"], vector: P(1, 0), prefix: "cp" }, outcome: missing("w-missing") },
      { name: "opening-alone", emoji: e.other, before: hosted(), mutation: { ids: ["o-door"], vector: P(1, 0), prefix: "cp" }, outcome: invariant("o-door") },
      { name: "storey-has-no-placement", emoji: e.hand, before: hosted(), mutation: { ids: ["st-ground"], vector: P(1, 0), prefix: "cp" }, outcome: invariant("st-ground") },
      { name: "blank-prefix", emoji: e.zero, before: hosted(), mutation: { ids: ["w-south"], vector: P(1, 0), prefix: " " }, outcome: invariant("prefix") },
      { name: "minted-id-taken", emoji: e.taken, before: scene({ walls: ["w-south"] }, { walls: { "cp-1-0": wallRow(line([0, 9], [8, 9]), "Taken") } }), mutation: { ids: ["w-south"], vector: P(0, 3), prefix: "cp" }, outcome: duplicate("prefix") },
    ],
  },
  {
    kind: "mirror-elements", emoji: 0x1f500, variant: "MirrorElements", verb: "replace", entity: "elements", displayName: "Mirror Elements", binaryTag: 5002,
    doc: "Mirrors placed elements about the line through two points: axes, boundaries and paths are reflected, a wall runs the other way so that its interior face stays on its left (its end join preferences trade places), column rotations, slab falls and roof ridges are reflected, a turning stair swaps its hand, and the openings of a mirrored wall take their offset from the other end with the other hand. With a prefix the mirror images are created as copies and the originals stay.",
    props: [idsProp, point("line_start", { en: "Mirror line start (m)", de: "Spiegellinie Anfang (m)" }, "line", 20), point("line_end", { en: "Mirror line end (m)", de: "Spiegellinie Ende (m)" }, "line", 30), optionalPrefix],
    inverseRows: { fixed: 1, perTarget: { ids: 2 } },
    label: { en: 'format!("Mirror {} element(s) about the line through ({}, {}) and ({}, {})", self.ids.len(), self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y)', de: 'format!("{} Element(e) an der Linie durch ({}, {}) und ({}, {}) spiegeln", self.ids.len(), self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y)' },
    target: many,
    cases: [
      { name: "mirrors-walls-and-their-openings", emoji: e.ok, before: hosted({ walls: { "w-south": { ...wallRow(line([0, 0], [8, 0]), "South"), start_join: "Butt", end_join: "Miter" }, "w-east": wallRow(line([8, 0], [8, 6]), "East") } }), mutation: { ids: ["w-south", "w-east"], line_start: P(4, -1), line_end: P(4, 7) }, outcome: ok },
      { name: "mirrors-an-arc-wall-and-a-curtain-wall", emoji: e.arc, before: scene({ curtain_walls: ["cw-1"] }, { walls: { "w-arc": wallRow(arc([0, 0], [8, 0], 0.5), "Arc") }, openings: { "o-arc": opening("w-arc", "Window", 3, "Window"), "o-cw": opening("cw-1", "Door", 2, "Door") } }), mutation: { ids: ["w-arc", "cw-1"], line_start: P(0, 3), line_end: P(1, 3) }, outcome: ok },
      { name: "mirrors-every-placed-kind", emoji: e.kinds, before: everything(), mutation: { ids: ALL, line_start: P(2, 0), line_end: P(2, 1) }, outcome: ok },
      { name: "mirrors-a-turning-stair", emoji: e.star, before: scene({}, { stairs: { "s-l": { ...clone(EVERY.stairs["s-1"]), flight: { LTurn: { split: 0.5, turn: "Left" } } } } }), mutation: { ids: ["s-l"], line_start: P(0, 0), line_end: P(1, 1) }, outcome: ok },
      { name: "mirrors-as-copies", emoji: e.data, before: hosted(withData), mutation: { ids: ["w-south", "w-east"], line_start: P(4, -1), line_end: P(4, 7), prefix: "mi" }, outcome: ok },
      { name: "fixed-hand-stair", emoji: e.other, before: scene({}, { stairs: { "s-u": { ...clone(EVERY.stairs["s-1"]), flight: { UTurn: { gap: 0.1 } } } } }), mutation: { ids: ["s-u"], line_start: P(0, 0), line_end: P(1, 0) }, outcome: invariant("s-u") },
      { name: "line-without-length", emoji: e.zero, before: hosted(), mutation: { ids: ["w-south"], line_start: P(1, 1), line_end: P(1, 1) }, outcome: invariant("line_end") },
      { name: "empty-selection", emoji: e.stop, before: hosted(), mutation: { ids: [], line_start: P(4, -1), line_end: P(4, 7) }, outcome: invariant("ids") },
      { name: "unknown-id", emoji: e.no, before: hosted(), mutation: { ids: ["w-missing"], line_start: P(4, -1), line_end: P(4, 7) }, outcome: missing("w-missing") },
      { name: "already-symmetric", emoji: e.same, before: scene({}, { grids: { "g-m": { building: "bldg-1", label: "M", start: P(4, -1), end: P(4, 7) } } }), mutation: { ids: ["g-m"], line_start: P(4, -1), line_end: P(4, 7) }, outcome: noop("ids") },
    ],
  },
  {
    kind: "array-elements", emoji: 0x1f4a0, variant: "ArrayElements", verb: "duplicate", entity: "elements", displayName: "Array Elements", binaryTag: 5003,
    doc: "Repeats placed elements in a linear array (each copy a further spacing) or a radial array (each copy a further step about a centre), as one set of created records with ids minted from the prefix, the copy number and the position of the source; every copy is computed from the source, so no rounding accumulates.",
    props: [idsProp, prefixProp(), pattern],
    label: { en: 'format!("Array {} element(s) {} times", self.ids.len(), self.pattern.count())', de: 'format!("{} Element(e) {}-mal anordnen", self.ids.len(), self.pattern.count())' },
    target: many,
    cases: [
      { name: "arrays-a-column-in-a-row", emoji: e.ok, before: scene({ columns: ["c-1"] }), mutation: { ids: ["c-1"], prefix: "ar", pattern: { Linear: { count: 3, spacing: P(3, 0) } } }, outcome: ok },
      { name: "arrays-a-column-around-a-centre", emoji: e.arc, before: scene({}, { columns: { "c-r": { ...clone(EVERY.columns["c-1"]), position: P(6, 3), rotation: 0 } } }), mutation: { ids: ["c-r"], prefix: "ar", pattern: { Radial: { count: 3, center: P(4, 3), step: Math.PI / 2 } } }, outcome: ok },
      { name: "arrays-a-wall-with-its-openings", emoji: e.data, before: hosted(withData), mutation: { ids: ["w-south"], prefix: "ar", pattern: { Linear: { count: 2, spacing: P(0, 4) } } }, outcome: ok },
      { name: "no-copies", emoji: e.zero, before: scene({ columns: ["c-1"] }), mutation: { ids: ["c-1"], prefix: "ar", pattern: { Linear: { count: 0, spacing: P(3, 0) } } }, outcome: invariant("pattern") },
      { name: "spacing-without-length", emoji: e.stop, before: scene({ columns: ["c-1"] }), mutation: { ids: ["c-1"], prefix: "ar", pattern: { Linear: { count: 2, spacing: P(0, 0) } } }, outcome: invariant("pattern") },
      { name: "too-many-copies", emoji: e.other, before: scene({ columns: ["c-1"] }), mutation: { ids: ["c-1"], prefix: "ar", pattern: { Linear: { count: 5000, spacing: P(1, 0) } } }, outcome: invariant("pattern") },
      { name: "unknown-id", emoji: e.no, before: scene({ columns: ["c-1"] }), mutation: { ids: ["c-missing"], prefix: "ar", pattern: { Linear: { count: 2, spacing: P(3, 0) } } }, outcome: missing("c-missing") },
      { name: "minted-id-taken", emoji: e.taken, before: scene({ columns: ["c-1"] }, { columns: { "ar-2-0": { ...clone(EVERY.columns["c-1"]), position: P(7, 7) } } }), mutation: { ids: ["c-1"], prefix: "ar", pattern: { Linear: { count: 3, spacing: P(3, 0) } } }, outcome: duplicate("prefix") },
    ],
  },
  {
    kind: "align-elements", emoji: 0x1f4cb, variant: "AlignElements", verb: "move", entity: "elements", displayName: "Align Elements", binaryTag: 5004,
    doc: "Moves placed elements along one axis of the plan so that the lower edge, the middle or the upper edge of the authored extent of each element lies on a target coordinate; arcs count with their extremes, hosted openings follow their host.",
    props: [idsProp, choice("axis", `${MODIFY}::AlignAxis`, ["X", "Y"], { en: "Coordinate", de: "Koordinate" }, 20), choice("edge", `${MODIFY}::AlignEdge`, ["Min", "Center", "Max"], { en: "Edge", de: "Kante" }, 30), num("target", { en: "Target coordinate (m)", de: "Zielkoordinate (m)" }, 40, "value")],
    label: { en: 'format!("Align {} element(s) to {:?} = {} m", self.ids.len(), self.axis, self.target)', de: 'format!("{} Element(e) auf {:?} = {} m ausrichten", self.ids.len(), self.axis, self.target)' },
    target: many,
    cases: [
      { name: "aligns-walls-to-a-line", emoji: e.ok, before: hosted(), mutation: { ids: ["w-south", "w-east"], axis: "X", edge: "Min", target: 2 }, outcome: ok },
      { name: "aligns-centres", emoji: e.star, before: scene({ columns: ["c-1"], beams: ["b-1"] }), mutation: { ids: ["c-1", "b-1"], axis: "Y", edge: "Center", target: 5 }, outcome: ok },
      { name: "aligns-an-arc-by-its-extremes", emoji: e.arc, before: scene({}, { walls: { "w-arc": wallRow(arc([0, 0], [8, 0], 0.5), "Arc") } }), mutation: { ids: ["w-arc"], axis: "Y", edge: "Min", target: 0 }, outcome: ok },
      { name: "already-aligned", emoji: e.same, before: hosted(), mutation: { ids: ["w-south"], axis: "Y", edge: "Min", target: 0 }, outcome: noop("ids") },
      { name: "openings-follow-their-host", emoji: e.hand, before: hosted(), mutation: { ids: ["o-door"], axis: "X", edge: "Min", target: 2 }, outcome: noop("ids") },
      { name: "empty-selection", emoji: e.stop, before: hosted(), mutation: { ids: [], axis: "X", edge: "Min", target: 2 }, outcome: invariant("ids") },
      { name: "unknown-id", emoji: e.no, before: hosted(), mutation: { ids: ["w-missing"], axis: "X", edge: "Min", target: 2 }, outcome: missing("w-missing") },
    ],
  },
  {
    kind: "offset-wall", emoji: 0x1f9f6, variant: "OffsetWall", verb: "create", entity: "wall", displayName: "Offset Wall", binaryTag: 5005,
    doc: "Creates a wall parallel to a wall at a signed distance to the left of its axis direction (to the right when negative): a shifted line, or a concentric arc of the same sweep, with the type, location line, constraints, phase and name of the original; hosted openings stay on the original.",
    props: [idProp("wall", { en: "Wall", de: "Wand" }), identity("new_id", { en: "New wall id", de: "Id der neuen Wand" }, 20), num("distance", { en: "Distance (m, left positive)", de: "Abstand (m, links positiv)" }, 30)],
    label: { en: 'format!("Offset wall \\"{}\\" by {} m", self.id, self.distance)', de: 'format!("Wand \\"{}\\" um {} m versetzen", self.id, self.distance)' },
    target: one,
    cases: [
      { name: "offsets-to-the-left", emoji: e.ok, before: hosted(), mutation: { id: "w-south", new_id: "w-offset", distance: 2 }, outcome: ok },
      { name: "offsets-to-the-right", emoji: e.star, before: hosted(), mutation: { id: "w-east", new_id: "w-offset", distance: -1.5 }, outcome: ok },
      { name: "offsets-an-arc", emoji: e.arc, before: scene({}, { walls: { "w-arc": wallRow(arc([0, 0], [8, 0], 0.5), "Arc") } }), mutation: { id: "w-arc", new_id: "w-arc-2", distance: 1 }, outcome: ok },
      { name: "arc-collapses", emoji: e.other, before: scene({}, { walls: { "w-arc": wallRow(arc([0, 0], [8, 0], 0.5), "Arc") } }), mutation: { id: "w-arc", new_id: "w-arc-2", distance: 6 }, outcome: invariant("distance") },
      { name: "zero-distance", emoji: e.zero, before: hosted(), mutation: { id: "w-south", new_id: "w-offset", distance: 0 }, outcome: invariant("distance") },
      { name: "new-id-taken", emoji: e.taken, before: hosted(), mutation: { id: "w-south", new_id: "w-east", distance: 2 }, outcome: duplicate("w-east") },
      { name: "missing", emoji: e.missing, before: hosted(), mutation: { id: "w-attic", new_id: "w-offset", distance: 2 }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "trim-extend-wall", emoji: 0x1f52a, variant: "TrimExtendWall", verb: "resize", entity: "wall", displayName: "Trim Or Extend Wall", binaryTag: 5006,
    doc: "Trims or extends one end of a wall to the axis of another wall: the intersection of the two axes (lines and circles carried on infinitely) nearest to the old end becomes the new end, as a concrete axis. Openings keep their place in the world: when the start moves their offsets move with it, and an opening that would no longer fit refuses the call.",
    props: [idProp("wall", { en: "Wall", de: "Wand" }), wallEnd(), idProp("wall", { en: "Target wall", de: "Zielwand" }, 30, "target")],
    label: { en: 'format!("Trim or extend the {:?} of wall \\"{}\\" to wall \\"{}\\"", self.end, self.id, self.target)', de: 'format!("{:?} der Wand \\"{}\\" auf Wand \\"{}\\" kürzen oder verlängern", self.end, self.id, self.target)' },
    target: one,
    cases: [
      { name: "trims-to-the-target", emoji: e.ok, before: scene({}, { walls: { "w-south": wallRow(line([0, 0], [10, 0]), "South"), "w-east": wallRow(line([8, -2], [8, 6]), "East") } }), mutation: { id: "w-south", end: "End", target: "w-east" }, outcome: ok },
      { name: "extends-to-the-target", emoji: e.star, before: scene({}, { walls: { "w-south": wallRow(line([0, 0], [6, 0]), "South"), "w-east": wallRow(line([8, -2], [8, 6]), "East") } }), mutation: { id: "w-south", end: "End", target: "w-east" }, outcome: ok },
      { name: "keeps-openings-in-place", emoji: e.hand, before: scene({}, { walls: { "w-south": wallRow(line([-2, 0], [8, 0]), "South"), "w-west": wallRow(line([0, -2], [0, 6]), "West") }, openings: { "o-window": opening("w-south", "Window", 5, "Window") } }), mutation: { id: "w-south", end: "Start", target: "w-west" }, outcome: ok },
      { name: "extends-an-arc", emoji: e.arc, before: scene({}, { walls: { "w-arc": wallRow(arc([0, 0], [4, 0], 0.5), "Arc"), "w-cut": wallRow(line([6, -4], [6, 4]), "Cut") } }), mutation: { id: "w-arc", end: "End", target: "w-cut" }, outcome: ok },
      { name: "never-meets", emoji: e.other, before: scene({}, { walls: { "w-south": wallRow(line([0, 0], [8, 0]), "South"), "w-north": wallRow(line([0, 5], [8, 5]), "North") } }), mutation: { id: "w-south", end: "End", target: "w-north" }, outcome: invariant("target") },
      { name: "behind-the-other-end", emoji: e.zero, before: scene({}, { walls: { "w-south": wallRow(line([0, 0], [8, 0]), "South"), "w-west": wallRow(line([-3, -2], [-3, 6]), "West") } }), mutation: { id: "w-south", end: "End", target: "w-west" }, outcome: invariant("end") },
      { name: "already-there", emoji: e.same, before: scene({ walls: ["w-south", "w-east"] }), mutation: { id: "w-south", end: "End", target: "w-east" }, outcome: noop("w-south") },
      { name: "opening-no-longer-fits", emoji: e.stop, before: scene({}, { walls: { "w-south": wallRow(line([0, 0], [10, 0]), "South"), "w-east": wallRow(line([8, -2], [8, 6]), "East") }, openings: { "o-window": opening("w-south", "Window", 9, "Window") } }), mutation: { id: "w-south", end: "End", target: "w-east" }, outcome: invariant("o-window") },
      { name: "its-own-target", emoji: e.taken, before: hosted(), mutation: { id: "w-south", end: "End", target: "w-south" }, outcome: invariant("target") },
      { name: "target-missing", emoji: e.no, before: hosted(), mutation: { id: "w-south", end: "End", target: "w-attic" }, outcome: missing("target") },
      { name: "missing", emoji: e.missing, before: hosted(), mutation: { id: "w-attic", end: "End", target: "w-east" }, outcome: missing("w-attic") },
    ],
  },
  {
    kind: "split-slab", emoji: 0x1f370, variant: "SplitSlab", verb: "split", entity: "slab", displayName: "Split Slab", binaryTag: 5007,
    doc: "Cuts a slab along the infinite line through two points into two slabs: the piece to the left of the line (seen from its first point to its second) stays, the piece to the right is created with the type, offset, slope and name of the original; holes go with the piece they lie in, split arcs keep their circle. The line must cross the outline exactly twice and no hole.",
    props: [idProp("slab", { en: "Slab", de: "Decke" }), identity("new_id", { en: "New slab id", de: "Id der neuen Decke" }, 20), point("line_start", { en: "Cut line start (m)", de: "Schnittlinie Anfang (m)" }, "line", 30), point("line_end", { en: "Cut line end (m)", de: "Schnittlinie Ende (m)" }, "line", 40)],
    inverseRows: { bounded: 2 },
    label: { en: 'format!("Split slab \\"{}\\" along the line through ({}, {}) and ({}, {})", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y)', de: 'format!("Decke \\"{}\\" entlang der Linie durch ({}, {}) und ({}, {}) teilen", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y)' },
    target: one,
    cases: [
      { name: "splits-a-rectangle", emoji: e.ok, before: slabScene(slab(box(0, 0, 8, 6))), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(3, -1), line_end: P(3, 7) }, outcome: ok },
      { name: "sends-a-hole-with-its-piece", emoji: e.hole, before: slabScene(slab(box(0, 0, 8, 6), { holes: [box(5, 2, 6, 3)] })), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(3, -1), line_end: P(3, 7) }, outcome: ok },
      { name: "splits-an-arc-outline", emoji: e.arc, before: slabScene(slab([V(0, 0), V(8, 0), V(8, 6, 0.3), V(0, 6)])), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(4, 7), line_end: P(4, -1) }, outcome: ok },
      { name: "splits-through-two-corners", emoji: e.star, before: slabScene(slab(box(0, 0, 6, 6))), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(0, 0), line_end: P(6, 6) }, outcome: ok },
      { name: "misses-the-slab", emoji: e.no, before: slabScene(slab(box(0, 0, 8, 6))), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(9, -1), line_end: P(9, 7) }, outcome: invariant("line_start") },
      { name: "crosses-a-hole", emoji: e.other, before: slabScene(slab(box(0, 0, 8, 6), { holes: [box(2, 2, 4, 3)] })), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(3, -1), line_end: P(3, 7) }, outcome: invariant("holes") },
      { name: "cuts-more-than-twice", emoji: e.stop, before: slabScene(slab([V(0, 0), V(9, 0), V(9, 6), V(6, 6), V(6, 2), V(3, 2), V(3, 6), V(0, 6)])), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(-1, 4), line_end: P(10, 4) }, outcome: invariant("line_start") },
      { name: "line-without-length", emoji: e.zero, before: slabScene(slab(box(0, 0, 8, 6))), mutation: { id: "sl-1", new_id: "sl-2", line_start: P(3, 3), line_end: P(3, 3) }, outcome: invariant("line_end") },
      { name: "new-id-taken", emoji: e.taken, before: slabScene(slab(box(0, 0, 8, 6))), mutation: { id: "sl-1", new_id: "w-south", line_start: P(3, -1), line_end: P(3, 7) }, outcome: duplicate("w-south") },
      { name: "missing", emoji: e.missing, before: slabScene(slab(box(0, 0, 8, 6))), mutation: { id: "sl-9", new_id: "sl-2", line_start: P(3, -1), line_end: P(3, 7) }, outcome: missing("sl-9") },
    ],
  },
  {
    kind: "split-beam", emoji: 0x1f956, variant: "SplitBeam", verb: "split", entity: "beam", displayName: "Split Beam", binaryTag: 5008,
    doc: "Splits a beam at the fraction `t` of its length into the original beam, which keeps the first part, and a new beam that continues it with the same type, top offset, phase and name.",
    props: [idProp("beam", { en: "Beam", de: "Träger" }), num("t", { en: "Split position (0 to 1)", de: "Teilungsstelle (0 bis 1)" }, 20), identity("new_id", { en: "New beam id", de: "Id des neuen Trägers" }, 30)],
    inverseRows: { bounded: 2 },
    label: { en: 'format!("Split beam \\"{}\\" at {}", self.id, self.t)', de: 'format!("Träger \\"{}\\" bei {} teilen", self.id, self.t)' },
    target: one,
    cases: [
      { name: "splits-a-beam", emoji: e.ok, before: scene({ beams: ["b-1"] }), mutation: { id: "b-1", t: 0.25, new_id: "b-2" }, outcome: ok },
      { name: "splits-a-slanted-beam", emoji: e.star, before: scene({}, { beams: { "b-s": { ...clone(EVERY.beams["b-1"]), start: P(0, 0), end: P(3, 4) } } }), mutation: { id: "b-s", t: 0.5, new_id: "b-t" }, outcome: ok },
      { name: "at-the-start", emoji: e.no, before: scene({ beams: ["b-1"] }), mutation: { id: "b-1", t: 0, new_id: "b-2" }, outcome: invariant("t") },
      { name: "beyond-the-end", emoji: e.stop, before: scene({ beams: ["b-1"] }), mutation: { id: "b-1", t: 1.5, new_id: "b-2" }, outcome: invariant("t") },
      { name: "new-id-taken", emoji: e.taken, before: scene({ beams: ["b-1"], columns: ["c-1"] }), mutation: { id: "b-1", t: 0.5, new_id: "c-1" }, outcome: duplicate("c-1") },
      { name: "missing", emoji: e.missing, before: scene({ beams: ["b-1"] }), mutation: { id: "b-9", t: 0.5, new_id: "b-2" }, outcome: missing("b-9") },
    ],
  },
];
//#endregion 🔖️Leaves

//#region 🔖️Post
const leafDir = (leaf: Leaf) => join(mutations, em(leaf.emoji) + leaf.kind);

/** 🧩️ Optional payload fields are absent from the wire form: `#[value(default, skip_serializing_if)]` and not required. */
const sparse = (leaf: Leaf) => {
  const optional = leaf.props.filter((prop) => prop.rust.startsWith("Option<"));
  if (optional.length === 0) return;
  const file = join(leafDir(leaf), em(0x1f9a0) + "mutation", RS);
  let source = readFileSync(file, "utf8");
  for (const prop of optional) source = source.replace(new RegExp(`^    pub ${prop.name}: `, "m"), (head) => `    #[value(default, skip_serializing_if = "Option::is_none")]\n${head}`);
  writeFileSync(file, source);
  const schemaFile = join(leafDir(leaf), readdirSync(leafDir(leaf)).find((name) => name.endsWith("schema"))!, JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optional.some((prop) => prop.name === name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
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

const helperDir = join(mutations, em(0x1f9d9) + "modify");
const helperMount = `                        #[path = "."]
                        pub mod modify {
                            #[path = "${relRoot(join(helperDir, RS))}"]
                            mod component;
                            pub use component::*;
                        }
`;

if (import.meta.main) {
  const scratch = join(import.meta.dir, "🗑️generated", "w05-modify");
  mkdirSync(scratch, { recursive: true });
  const mounts = new Map<string, string>([["modify", helperMount]]);
  for (const leaf of leaves) {
    mounts.set(leaf.kind.replaceAll("-", "_"), emitLeaf(leaf));
    sparse(leaf);
  }
  writeFileSync(join(scratch, "mounts.txt"), [...mounts.values()].join(""));
  if (process.argv.includes("register")) register(mounts);
  console.log(`emitted ${leaves.length} leaves`);
}
