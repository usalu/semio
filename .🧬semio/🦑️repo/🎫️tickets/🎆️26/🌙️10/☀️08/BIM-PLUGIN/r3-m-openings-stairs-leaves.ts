#!/usr/bin/env bun
/**
 * 🪟️ Wave M slice 7 (`m-openings-stairs`): the seven opening and stair leaves of `s.bim.model@1` (binary tags 700 to 706).
 * `bun r3-m-openings-stairs-leaves.ts` is idempotent: it rewrites the generated boilerplate and fixtures of every leaf (never a blessed
 * `after` or `diff`), installs the hand-written files from `🗑️generated/m-openings-stairs/src` once, post-processes optional payload
 * fields, and surgically registers the enum variants, kebab kinds, the shared `placement` module and the mount blocks.
 */
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
type Field = Prop & { optional?: boolean };

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Field => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Field => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const num = (name: string, label: Label, order = 20): Field => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const optional = (field: Field, rust: string): Field => ({ ...field, rust, optional: true });
const optRecord = (name: string, def: string, label: Label, order: number) => optional(recordProp(name, def, label, order), `Option<${def}>`);
const optNum = (name: string, label: Label, order: number) => optional(num(name, label, order), "Option<f64>");
const optText = (name: string, label: Label, order: number) => optional({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } }, "Option<String>");
const optToggle = (name: string, label: Label, order: number) => optional({ name, rust: "bool", schema: { type: "boolean" }, ui: { widget: "toggle", role: "value", label, group: "value", order } }, "Option<bool>");
const optAssigned = (name: string, label: Label, order: number) =>
  optional({ name, rust: "f64", schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { type: ["number", "null"] } } }, ui: { widget: "number", role: "value", label, group: "value", order } }, "Option<Assigned<Option<f64>>>");

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const P = F.P;
const win = (type = "win-1") => ({ Window: { window_type: type } });
const door = (type = "door-1") => ({ Door: { door_type: type } });
const hole = (width: number, height: number) => ({ Void: { width, height } });
const opening = (host: string, kind: unknown, offset: number, sill: number, name: string, extra: Record<string, unknown> = {}) => ({ host, kind, offset, sill, flip_hand: false, flip_facing: false, name, ...extra });
const windowType = (name: string, width: number, height: number, sill: number) => ({ name, width, height, sill, frame_width: 0.07, frame_depth: 0.12, panes: 2, material: "m-brick" });
const doorType = (name: string, width: number, height: number) => ({ name, width, height, frame_width: 0.07, frame_depth: 0.12, leaves: "Single", swing: "Left", material: "m-brick" });
const curtainWall = { storey: "st-ground", axis: F.line([0, 6], [8, 6]), base_offset: 0, top: F.storeyTop(0), u_spacing: 1.5, v_spacing: 1.2, mullion: { Rectangle: { width: 0.06, depth: 0.08 } }, panel_material: "m-brick", mullion_material: "m-brick", name: "Facade" };
const stair = (storey: string, name: string, extra: Record<string, unknown> = {}) => ({ storey, start: P(1, 1), direction: 0, width: 1, flight: "Straight", top: F.storeyTop(0), max_riser: 0.18, min_tread: 0.27, name, ...extra });

const house = (openings: Record<string, unknown> = {}, stairs: Record<string, unknown> = {}) => ({
  ...F.scene({ openings }),
  window_types: { "win-1": windowType("Window 1.2", 1.2, 1.3, 0.9) },
  door_types: { "door-1": doorType("Door 0.9", 0.9, 2.1) },
  curtain_walls: { "cw-1": curtainWall },
  stairs,
});
const kitchen = () => house({ "o-1": opening("w-south", win(), 4, 0.9, "Kitchen") });
const twoOpenings = () => house({ "o-1": opening("w-south", win(), 2, 0.9, "Kitchen"), "o-2": opening("w-south", door(), 5, 0, "Entrance") });
const oneStair = () => house({}, { "s-1": stair("st-ground", "Main stair") });

const E = { ok: 0x2705, plus: 0x2795, green: 0x1f7e2, square: 0x1f7e9, no: 0x1f6ab, stop: 0x26d4, halt: 0x1f6d1, cross: 0x274c, bang: 0x2757, badge: 0x1f4db, red: 0x1f534, orange: 0x1f7e0, yellow: 0x1f7e1, blue: 0x1f535 };

const leaves: Leaf[] = [
  {
    kind: "create-opening", emoji: 0x1f573, variant: "CreateOpening", verb: "create", entity: "opening", displayName: "Create Opening", binaryTag: 700,
    doc: "Brings a window, door or void into a wall or curtain wall; only its centre offset along the host axis and its sill are stored, the frame follows the host by inference.",
    props: [id("opening", "identity", { en: "Opening id", de: "Öffnungs-Id" }, 10), recordProp("opening", "Opening", { en: "Opening", de: "Öffnung" })],
    label: { en: 'format!("Create opening \\"{}\\"", self.opening.name)', de: 'format!("Öffnung \\"{}\\" anlegen", self.opening.name)' },
    target,
    cases: [
      { name: "adds-a-window", emoji: E.ok, before: house(), mutation: { id: "o-1", opening: opening("w-south", win(), 4, 0.9, "Kitchen") }, outcome: ok },
      { name: "adds-a-door-to-a-curtain-wall", emoji: E.plus, before: house(), mutation: { id: "o-1", opening: opening("cw-1", door(), 2, 0, "Entrance") }, outcome: ok },
      { name: "adds-a-void", emoji: E.green, before: house(), mutation: { id: "o-1", opening: opening("w-east", hole(1, 2), 3, 0.5, "Shaft") }, outcome: ok },
      { name: "duplicate", emoji: E.no, before: kitchen(), mutation: { id: "o-1", opening: opening("w-south", win(), 6, 0.9, "Other") }, outcome: reject("mutation.duplicate-id", ["o-1"]) },
      { name: "host-missing", emoji: E.stop, before: house(), mutation: { id: "o-1", opening: opening("w-none", win(), 4, 0.9, "Kitchen") }, outcome: reject("mutation.target-missing", ["opening", "host"]) },
      { name: "type-missing", emoji: E.halt, before: house(), mutation: { id: "o-1", opening: opening("w-south", win("win-9"), 4, 0.9, "Kitchen") }, outcome: reject("mutation.target-missing", ["opening", "kind"]) },
      { name: "negative-sill", emoji: E.cross, before: house(), mutation: { id: "o-1", opening: opening("w-south", win(), 4, -0.1, "Kitchen") }, outcome: reject("mutation.invariant", ["opening", "sill"]) },
      { name: "non-positive-width", emoji: E.bang, before: house(), mutation: { id: "o-1", opening: opening("w-south", win(), 4, 0.9, "Kitchen", { width: 0 }) }, outcome: reject("mutation.invariant", ["opening", "width"]) },
      { name: "beyond-the-host-end", emoji: E.badge, before: house(), mutation: { id: "o-1", opening: opening("w-south", win(), 7.7, 0.9, "Kitchen") }, outcome: reject("mutation.invariant", ["opening", "offset"]) },
      { name: "overlaps-a-neighbour", emoji: E.red, before: kitchen(), mutation: { id: "o-2", opening: opening("w-south", door(), 4.8, 0, "Entrance") }, outcome: reject("mutation.invariant", ["opening", "offset"]) },
    ],
  },
  {
    kind: "delete-opening", emoji: 0x1fa93, variant: "DeleteOpening", verb: "delete", entity: "opening", displayName: "Delete Opening", binaryTag: 701,
    doc: "Removes an opening from its host; nothing depends on an opening.",
    props: [id("opening", "target", { en: "Opening", de: "Öffnung" })],
    label: { en: 'format!("Delete opening \\"{}\\"", self.id)', de: 'format!("Öffnung \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: E.ok, before: kitchen(), mutation: { id: "o-1" }, outcome: ok },
      { name: "missing", emoji: E.no, before: house(), mutation: { id: "o-1" }, outcome: reject("mutation.target-missing", ["o-1"]) },
    ],
  },
  {
    kind: "move-opening", emoji: 0x1f6f7, variant: "MoveOpening", verb: "move", entity: "opening", displayName: "Move Opening", binaryTag: 702,
    doc: "Moves an opening along its host axis (centre offset from the host start) and optionally re-hosts it to another wall or curtain wall.",
    props: [
      id("opening", "target", { en: "Opening", de: "Öffnung" }),
      num("offset", { en: "Offset along host (m)", de: "Abstand entlang des Trägers (m)" }),
      optional({ name: "host", rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label: { en: "New host", de: "Neuer Träger" }, ref: { kind: "wall" }, group: "value", order: 30 } }, "Option<String>"),
    ],
    label: { en: 'format!("Move opening \\"{}\\" to {} m", self.id, self.offset)', de: 'format!("Öffnung \\"{}\\" auf {} m verschieben", self.id, self.offset)' },
    target,
    cases: [
      { name: "slides-along-the-host", emoji: E.ok, before: kitchen(), mutation: { id: "o-1", offset: 5.5 }, outcome: ok },
      { name: "rehosts-to-another-wall", emoji: E.plus, before: kitchen(), mutation: { id: "o-1", offset: 3, host: "w-east" }, outcome: ok },
      { name: "missing", emoji: E.no, before: house(), mutation: { id: "o-1", offset: 3 }, outcome: reject("mutation.target-missing", ["o-1"]) },
      { name: "host-missing", emoji: E.stop, before: kitchen(), mutation: { id: "o-1", offset: 3, host: "w-none" }, outcome: reject("mutation.target-missing", ["host"]) },
      { name: "beyond-the-host-end", emoji: E.halt, before: kitchen(), mutation: { id: "o-1", offset: 0.3 }, outcome: reject("mutation.invariant", ["offset"]) },
      { name: "overlaps-a-neighbour", emoji: E.cross, before: twoOpenings(), mutation: { id: "o-2", offset: 3 }, outcome: reject("mutation.invariant", ["offset"]) },
      { name: "already-there", emoji: E.bang, before: kitchen(), mutation: { id: "o-1", offset: 4 }, outcome: reject("mutation.no-op", ["o-1"]) },
    ],
  },
  {
    kind: "set-opening", emoji: 0x1f579, variant: "SetOpening", verb: "set", entity: "opening", displayName: "Set Opening", binaryTag: 703,
    doc: "Sparsely changes an opening: kind or type, sill, width and height overrides (an assigned null clears an override), hand and facing flips, name.",
    props: [
      id("opening", "target", { en: "Opening", de: "Öffnung" }),
      optRecord("kind", "OpeningKind", { en: "Kind", de: "Art" }, 20),
      optNum("sill", { en: "Sill (m)", de: "Brüstung (m)" }, 30),
      optAssigned("width", { en: "Width override (m)", de: "Breite überschreiben (m)" }, 40),
      optAssigned("height", { en: "Height override (m)", de: "Höhe überschreiben (m)" }, 50),
      optToggle("flip_hand", { en: "Flip hand", de: "Anschlag spiegeln" }, 60),
      optToggle("flip_facing", { en: "Flip facing", de: "Blickrichtung spiegeln" }, 70),
      optText("name", { en: "Name", de: "Name" }, 80),
    ],
    label: { en: 'format!("Change opening \\"{}\\"", self.id)', de: 'format!("Öffnung \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "resizes", emoji: E.ok, before: kitchen(), mutation: { id: "o-1", width: { value: 1.5 }, height: { value: 1.4 } }, outcome: ok },
      { name: "clears-the-width-override", emoji: E.plus, before: house({ "o-1": opening("w-south", win(), 4, 0.9, "Kitchen", { width: 1.5 }) }), mutation: { id: "o-1", width: { value: null } }, outcome: ok },
      { name: "retypes-and-flips", emoji: E.green, before: kitchen(), mutation: { id: "o-1", kind: door(), sill: 0, flip_hand: true, name: "Back door" }, outcome: ok },
      { name: "missing", emoji: E.no, before: house(), mutation: { id: "o-1", sill: 0.5 }, outcome: reject("mutation.target-missing", ["o-1"]) },
      { name: "type-missing", emoji: E.stop, before: kitchen(), mutation: { id: "o-1", kind: win("win-9") }, outcome: reject("mutation.target-missing", ["kind"]) },
      { name: "negative-sill", emoji: E.halt, before: kitchen(), mutation: { id: "o-1", sill: -0.2 }, outcome: reject("mutation.invariant", ["sill"]) },
      { name: "non-positive-height", emoji: E.cross, before: kitchen(), mutation: { id: "o-1", height: { value: 0 } }, outcome: reject("mutation.invariant", ["height"]) },
      { name: "too-wide-for-the-host", emoji: E.bang, before: kitchen(), mutation: { id: "o-1", width: { value: 9 } }, outcome: reject("mutation.invariant", ["width"]) },
      { name: "overlaps-a-neighbour", emoji: E.badge, before: twoOpenings(), mutation: { id: "o-2", width: { value: 5 } }, outcome: reject("mutation.invariant", ["width"]) },
      { name: "already-set", emoji: E.red, before: kitchen(), mutation: { id: "o-1", sill: 0.9, flip_hand: false }, outcome: reject("mutation.no-op", ["o-1"]) },
    ],
  },
  {
    kind: "create-stair", emoji: 0x1f9d7, variant: "CreateStair", verb: "create", entity: "stair", displayName: "Create Stair", binaryTag: 704,
    doc: "Brings a stair run onto a storey; risers and treads are never stored, they are inferred from the storey levels and the top constraint.",
    props: [id("stair", "identity", { en: "Stair id", de: "Treppen-Id" }, 10), recordProp("stair", "Stair", { en: "Stair", de: "Treppe" })],
    label: { en: 'format!("Create stair \\"{}\\"", self.stair.name)', de: 'format!("Treppe \\"{}\\" anlegen", self.stair.name)' },
    target,
    cases: [
      { name: "adds-a-straight-flight", emoji: E.ok, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair") }, outcome: ok },
      { name: "adds-a-u-run-to-the-first-storey", emoji: E.plus, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { flight: { UTurn: { gap: 0.1 } }, top: F.toStorey("st-first", 0) }) }, outcome: ok },
      { name: "duplicate", emoji: E.no, before: oneStair(), mutation: { id: "s-1", stair: stair("st-ground", "Other") }, outcome: reject("mutation.duplicate-id", ["s-1"]) },
      { name: "storey-missing", emoji: E.stop, before: house(), mutation: { id: "s-1", stair: stair("st-attic", "Main stair") }, outcome: reject("mutation.target-missing", ["stair", "storey"]) },
      { name: "non-positive-width", emoji: E.halt, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { width: 0 }) }, outcome: reject("mutation.invariant", ["stair", "width"]) },
      { name: "invalid-riser", emoji: E.cross, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { max_riser: -0.1 }) }, outcome: reject("mutation.invariant", ["stair", "max_riser"]) },
      { name: "invalid-tread", emoji: E.bang, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { min_tread: 0 }) }, outcome: reject("mutation.invariant", ["stair", "min_tread"]) },
      { name: "broken-flight", emoji: E.badge, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { flight: { UTurn: { gap: -1 } } }) }, outcome: reject("mutation.invariant", ["stair", "flight"]) },
      { name: "top-storey-missing", emoji: E.red, before: house(), mutation: { id: "s-1", stair: stair("st-ground", "Main stair", { top: F.toStorey("st-attic", 0) }) }, outcome: reject("mutation.target-missing", ["stair", "top", "storey"]) },
    ],
  },
  {
    kind: "delete-stair", emoji: 0x1f9e8, variant: "DeleteStair", verb: "delete", entity: "stair", displayName: "Delete Stair", binaryTag: 705,
    doc: "Removes a stair run; nothing depends on a stair.",
    props: [id("stair", "target", { en: "Stair", de: "Treppe" })],
    label: { en: 'format!("Delete stair \\"{}\\"", self.id)', de: 'format!("Treppe \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: E.ok, before: oneStair(), mutation: { id: "s-1" }, outcome: ok },
      { name: "missing", emoji: E.no, before: house(), mutation: { id: "s-1" }, outcome: reject("mutation.target-missing", ["s-1"]) },
    ],
  },
  {
    kind: "set-stair", emoji: 0x1f9ee, variant: "SetStair", verb: "set", entity: "stair", displayName: "Set Stair", binaryTag: 706,
    doc: "Sparsely changes a stair: start, direction, width, flight, top constraint, maximum riser, minimum tread, name.",
    props: [
      id("stair", "target", { en: "Stair", de: "Treppe" }),
      optRecord("start", "Point2", { en: "Start", de: "Anfang" }, 20),
      optNum("direction", { en: "Direction (rad)", de: "Richtung (rad)" }, 30),
      optNum("width", { en: "Width (m)", de: "Breite (m)" }, 40),
      optRecord("flight", "StairFlight", { en: "Flight", de: "Lauf" }, 50),
      optRecord("top", "TopConstraint", { en: "Top", de: "Oberkante" }, 60),
      optNum("max_riser", { en: "Maximum riser (m)", de: "Maximale Steigung (m)" }, 70),
      optNum("min_tread", { en: "Minimum tread (m)", de: "Minimaler Auftritt (m)" }, 80),
      optText("name", { en: "Name", de: "Name" }, 90),
    ],
    label: { en: 'format!("Change stair \\"{}\\"", self.id)', de: 'format!("Treppe \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "moves-and-widens", emoji: E.ok, before: oneStair(), mutation: { id: "s-1", start: P(2, 1), direction: 1.57, width: 1.2 }, outcome: ok },
      { name: "turns-into-an-l-run", emoji: E.plus, before: oneStair(), mutation: { id: "s-1", flight: { LTurn: { split: 0.5, turn: "Left" } }, top: F.toStorey("st-first", 0) }, outcome: ok },
      { name: "renames-and-flattens", emoji: E.green, before: oneStair(), mutation: { id: "s-1", name: "Garden stair", max_riser: 0.175 }, outcome: ok },
      { name: "missing", emoji: E.no, before: house(), mutation: { id: "s-1", width: 1.2 }, outcome: reject("mutation.target-missing", ["s-1"]) },
      { name: "non-positive-width", emoji: E.stop, before: oneStair(), mutation: { id: "s-1", width: 0 }, outcome: reject("mutation.invariant", ["width"]) },
      { name: "invalid-riser", emoji: E.halt, before: oneStair(), mutation: { id: "s-1", max_riser: -0.1 }, outcome: reject("mutation.invariant", ["max_riser"]) },
      { name: "top-storey-missing", emoji: E.cross, before: oneStair(), mutation: { id: "s-1", top: F.toStorey("st-attic", 0) }, outcome: reject("mutation.target-missing", ["top", "storey"]) },
      { name: "already-set", emoji: E.bang, before: oneStair(), mutation: { id: "s-1", width: 1 }, outcome: reject("mutation.no-op", ["s-1"]) },
    ],
  },
];

const listDir = (parent: string, suffix: string) => join(parent, readdirSync(parent).find((name) => name.endsWith(suffix))!);
const staged = join(import.meta.dir, "🗑️generated", "m-openings-stairs");
const src = join(staged, "src");

function patchOptionalFields(leaf: Leaf) {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  const fields = leaf.props as Field[];
  const optionalNames = new Set(fields.filter((field) => field.optional).map((field) => field.name));
  if (optionalNames.size === 0) return;
  const mutationFile = join(leafDir, em(0x1f9a0) + "mutation", RS);
  let text = readFileSync(mutationFile, "utf8");
  const names = new Set(["ModelDiff", "ModelMutation", "ModelSnapshot"]);
  for (const field of fields) for (const match of field.rust.matchAll(/\b[A-Z][A-Za-z0-9]*\b/g)) if (!["String", "Option"].includes(match[0])) names.add(match[0]);
  text = text.replace(/use crate::\{[^}]*\};/, `use crate::{${[...names].sort().join(", ")}};`);
  for (const name of optionalNames) text = text.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  writeFileSync(mutationFile, text);
  const schemaFile = join(leafDir, em(0x1f9ec) + "schema", JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optionalNames.has(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
}

function installHandwritten(leaf: Leaf) {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  for (const [dir, file] of [[em(0x1f53a) + "diff", "diff"], [em(0x21a9) + "inverse", "inverse"]] as const) {
    const target = join(leafDir, dir, RS);
    const source = join(src, `${leaf.kind}.${file}.rs`);
    if (!existsSync(target) && existsSync(source)) copyFileSync(source, target);
  }
}

function registerShared(mounts: string) {
  const aggregateFile = join(mutations, RS);
  let aggregate = readFileSync(aggregateFile, "utf8");
  const missing = leaves.filter((leaf) => !aggregate.includes(`    ${leaf.variant}(super::`));
  if (missing.length) {
    const enumEnd = aggregate.indexOf("\n}\n", aggregate.indexOf("pub enum ModelMutation {"));
    aggregate = aggregate.slice(0, enumEnd) + "\n" + missing.map((leaf) => `    ${leaf.variant}(super::${leaf.kind.replaceAll("-", "_")}::${leaf.variant}),`).join("\n") + aggregate.slice(enumEnd);
    const kindsEnd = aggregate.indexOf("\n];", aggregate.indexOf("pub const KINDS"));
    aggregate = aggregate.slice(0, kindsEnd) + "\n" + missing.map((leaf) => `    "${leaf.kind}",`).join("\n") + aggregate.slice(kindsEnd);
  }
  if (!aggregate.includes("pub mod placement;")) {
    const kit = aggregate.indexOf("pub mod kit;\n") + "pub mod kit;\n".length;
    aggregate = aggregate.slice(0, kit) + `\n#[path = "${em(0x1f4cd)}placement/${RS}"]\npub mod placement;\n` + aggregate.slice(kit);
  }
  writeFileSync(aggregateFile, aggregate);
  const rootFile = join(artifact, RS);
  const root = readFileSync(rootFile, "utf8");
  if (!root.includes("pub mod create_opening {")) {
    const anchor = root.search(/^ *\/\/#endregion .*Leaves/m);
    writeFileSync(rootFile, root.slice(0, anchor) + mounts + root.slice(anchor));
  }
}

mkdirSync(staged, { recursive: true });
const placementTarget = join(mutations, em(0x1f4cd) + "placement", RS);
if (!existsSync(placementTarget) && existsSync(join(src, "placement.rs"))) {
  mkdirSync(join(mutations, em(0x1f4cd) + "placement"), { recursive: true });
  copyFileSync(join(src, "placement.rs"), placementTarget);
}
let mounts = "";
for (const leaf of leaves) {
  mounts += emitLeaf(leaf);
  patchOptionalFields(leaf);
  installHandwritten(leaf);
}
writeFileSync(join(staged, "mounts.txt"), mounts);
registerShared(mounts);
console.log(`emitted ${leaves.length} leaves`);
void listDir;
