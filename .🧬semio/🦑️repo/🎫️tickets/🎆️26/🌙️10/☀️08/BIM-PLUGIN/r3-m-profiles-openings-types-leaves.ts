#!/usr/bin/env bun
/**
 * 🧪️ Wave M, slice 2 (`m-profiles-openings-types`, wire tags 200 to 299): create / delete / set of column, beam, window and door types.
 * `bun r3-m-profiles-openings-types-leaves.ts` is idempotent: it rewrites every leaf's boilerplate and fixtures (never a blessed `after`
 * or `diff`), renders the `🔺️diff` / `↩️inverse` logic, writes the shared validity module and registers the leaves surgically
 * (enum, `KINDS`, mount region, snapshot module). Bless afterwards with `BIM_BLESS=1 cargo test … profiles`.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Case, type Label, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, child, em, JSONF, mutations, RS, schema } from "./r3-f1-paths.ts";
import { createDiff, createInverse, deleteDiff, deleteInverse, setDiff, setInverse, validityModule, type Entity } from "./r3-m-profiles-openings-types-rust.ts";

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const idProp = (entity: string, role: "target" | "identity", label: Label, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const prop = (name: string, rust: string, schema: Record<string, unknown>, widget: string, label: Label, order: number, group = "value", extra: Record<string, unknown> = {}): Prop => ({ name, rust, schema, ui: { widget, role: "value", label, group, order, ...extra } });

type FieldSpec = { name: string; prop: (order: number) => Prop };
const length = (name: string, en: string, de: string): FieldSpec => ({ name, prop: (order) => prop(name, "f64", { type: "number" }, "number", { en, de }, order) });
const fields = {
  name: { name: "name", prop: (order) => prop("name", "String", { type: "string" }, "text", { en: "Name", de: "Name" }, order, "identity") } as FieldSpec,
  profile: { name: "profile", prop: (order) => prop("profile", "Profile", record("Profile"), "record", { en: "Profile", de: "Profil" }, order) } as FieldSpec,
  material: { name: "material", prop: (order) => prop("material", "String", { type: "string" }, "reference", { en: "Material", de: "Material" }, order, "value", { ref: { kind: "material" } }) } as FieldSpec,
  width: length("width", "Width (m)", "Breite (m)"),
  height: length("height", "Height (m)", "Höhe (m)"),
  sill: length("sill", "Sill height (m)", "Brüstungshöhe (m)"),
  frame_width: length("frame_width", "Frame width (m)", "Rahmenbreite (m)"),
  frame_depth: length("frame_depth", "Frame depth (m)", "Rahmentiefe (m)"),
  panes: { name: "panes", prop: (order) => prop("panes", "u32", { type: "integer" }, "integer", { en: "Panes", de: "Scheiben" }, order) } as FieldSpec,
  leaves: { name: "leaves", prop: (order) => prop("leaves", "DoorLeaves", record("DoorLeaves"), "select", { en: "Leaves", de: "Flügel" }, order) } as FieldSpec,
  swing: { name: "swing", prop: (order) => prop("swing", "Swing", record("Swing"), "select", { en: "Swing", de: "Aufschlag" }, order) } as FieldSpec,
};

const lengthRule = (field: string, noun: string, fn = "is_positive_length", what = "a positive length") => ({
  field,
  message: `A ${noun} must be ${what}.`,
  create: `${fn}(record.${field})`,
  set: `payload.${field}.is_none_or(${fn})`,
});
const dimensions = (noun: string) => [lengthRule("width", `${noun} width`), lengthRule("height", `${noun} height`)];
const frame = (noun: string) => [lengthRule("frame_width", `${noun} frame width`), lengthRule("frame_depth", `${noun} frame depth`)];

type Family = {
  entity: Entity;
  de: string;
  refKind: string;
  tag: number;
  emojis: [number, number, number];
  fieldNames: (keyof typeof fields)[];
  rules: Entity["invariants"];
};

const families: Family[] = [
  {
    de: "Stützentyp", refKind: "column-type", tag: 200, emojis: [0x1f5fc, 0x1f3fa, 0x1f39a], fieldNames: ["name", "profile", "material"], rules: [],
    entity: { Pascal: "ColumnType", snake: "column_type", kebab: "column-type", collection: "column_types", title: "Column type", fields: [], profile: true, invariants: [], users: "base.columns.values().any(|row| row.column_type == payload.id)", usersNoun: "columns", usesOpeningKind: false },
  },
  {
    de: "Trägertyp", refKind: "beam-type", tag: 203, emojis: [0x1f309, 0x1faa2, 0x1f39e], fieldNames: ["name", "profile", "material"], rules: [],
    entity: { Pascal: "BeamType", snake: "beam_type", kebab: "beam-type", collection: "beam_types", title: "Beam type", fields: [], profile: true, invariants: [], users: "base.beams.values().any(|row| row.beam_type == payload.id)", usersNoun: "beams", usesOpeningKind: false },
  },
  {
    de: "Fenstertyp", refKind: "window-type", tag: 206, emojis: [0x1f5bc, 0x1f9ca, 0x1f505], fieldNames: ["name", "width", "height", "sill", "frame_width", "frame_depth", "panes", "material"],
    rules: [...dimensions("window"), lengthRule("sill", "window sill", "is_non_negative_length", "zero or more"), ...frame("window"), { field: "panes", message: "A window needs at least one pane.", create: "record.panes >= 1", set: "payload.panes.is_none_or(|panes| panes >= 1)" }],
    entity: { Pascal: "WindowType", snake: "window_type", kebab: "window-type", collection: "window_types", title: "Window type", fields: [], profile: false, invariants: [], users: "base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Window { window_type } if *window_type == payload.id))", usersNoun: "openings", usesOpeningKind: true },
  },
  {
    de: "Türtyp", refKind: "door-type", tag: 209, emojis: [0x1f6d7, 0x1f512, 0x1f511], fieldNames: ["name", "width", "height", "frame_width", "frame_depth", "leaves", "swing", "material"],
    rules: [...dimensions("door"), ...frame("door")],
    entity: { Pascal: "DoorType", snake: "door_type", kebab: "door-type", collection: "door_types", title: "Door type", fields: [], profile: false, invariants: [], users: "base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Door { door_type } if *door_type == payload.id))", usersNoun: "openings", usesOpeningKind: true },
  },
];
const COPY = new Set(["width", "height", "sill", "frame_width", "frame_depth", "panes", "leaves", "swing"]);
for (const family of families) {
  family.entity.fields = family.fieldNames.map((name) => ({ name, copy: COPY.has(name) }));
  family.entity.invariants = family.rules;
}

const mat = (name: string, category: string) => ({ ...F.material(name), category });
const rect = (width: number, depth: number) => ({ Rectangle: { width, depth } });
const circle = (diameter: number) => ({ Circle: { diameter } });
const ishape = (width: number, depth: number, web: number, flange: number) => ({ IShape: { width, depth, web, flange } });
const vtx = (x: number, y: number, bulge = 0) => ({ point: F.P(x, y), bulge });
const custom = (...outline: unknown[]) => ({ Custom: { outline } });
const lShape = custom(vtx(0, 0), vtx(0.4, 0), vtx(0.4, 0.1), vtx(0.1, 0.1), vtx(0.1, 0.4), vtx(0, 0.4));
const dShape = custom(vtx(-0.2, 0, 1), vtx(0.2, 0), vtx(0.2, 0.3), vtx(-0.2, 0.3));
const clockwise = custom(vtx(0, 0), vtx(0, 0.4), vtx(0.4, 0.4), vtx(0.4, 0));
const bowTie = custom(vtx(0, 0), vtx(0.4, 0.4), vtx(0.4, 0), vtx(0, 0.4));
const profiled = (name: string, profile: unknown, material: string) => ({ name, profile, material });
const windowRecord = (over: Record<string, unknown> = {}) => ({ name: "Window 1.20", width: 1.2, height: 1.4, sill: 0.9, frame_width: 0.07, frame_depth: 0.1, panes: 2, material: "m-timber", ...over });
const doorRecord = (over: Record<string, unknown> = {}) => ({ name: "Door 0.90", width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.1, leaves: "Single", swing: "Left", material: "m-timber", ...over });
const column = { storey: "st-ground", column_type: "ct-400", position: F.P(1, 1), rotation: 0, base_offset: 0, top: F.storeyTop(0), name: "C1" };
const beam = { storey: "st-ground", beam_type: "bt-i300", start: F.P(0, 0), end: F.P(4, 0), top_offset: 0, name: "B1" };
const doorOpening = { host: "w-south", kind: { Door: { door_type: "door-1" } }, offset: 1, sill: 0, flip_hand: false, flip_facing: false, name: "Door" };

const base = (extra: Record<string, unknown> = {}) => {
  const scene = F.scene();
  return {
    ...scene,
    materials: { ...scene.materials, "m-concrete": mat("Concrete", "Concrete"), "m-steel": mat("Steel", "Metal"), "m-timber": mat("Timber", "Wood") },
    column_types: { "ct-400": profiled("Concrete 400", rect(0.4, 0.4), "m-concrete") },
    beam_types: { "bt-i300": profiled("Steel I 300", ishape(0.15, 0.3, 0.007, 0.011), "m-steel") },
    window_types: { "win-1": windowRecord() },
    door_types: { "door-1": doorRecord() },
    ...extra,
  };
};

type Spec = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Case["outcome"] };
const APPLIED = [0x2705, 0x1f9f2, 0x2728, 0x1f31f, 0x1f340];
const REJECTED = [0x1f6ab, 0x26d4, 0x1f6d1, 0x274c, 0x1f4db, 0x1f6b7, 0x1f6b3, 0x1f6af, 0x1f6b1, 0x1f4a2];
const numbered = (specs: Spec[]): Case[] => {
  let applied = 0;
  let rejected = 0;
  return specs.map((spec) => ({ ...spec, emoji: spec.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};
const adds = (name: string, before: unknown, mutation: Record<string, unknown>): Spec => ({ name, before, mutation, outcome: ok });
const refuses = (name: string, before: unknown, mutation: Record<string, unknown>, code: string, path: string[]): Spec => ({ name, before, mutation, outcome: reject(code, path) });
const missing = (id: string) => reject("mutation.target-missing", [id]);
const duplicate = (id: string) => reject("mutation.duplicate-id", [id]);
const noop = (id: string) => reject("mutation.no-op", [id]);

const leafOf = (family: Family, verb: "create" | "delete" | "set", index: number, cases: Case[]): Leaf => {
  const e = family.entity;
  const kind = `${verb}-${e.kebab}`;
  const title = e.title.toLowerCase();
  const optionalProps = (): Prop[] => family.fieldNames.map((name, i) => fields[name].prop(20 + 10 * i));
  const common = { kind, emoji: family.emojis[index], variant: `${verb[0].toUpperCase()}${verb.slice(1)}${e.Pascal}`, verb, entity: e.kebab, target, cases, binaryTag: family.tag + index, displayName: `${verb[0].toUpperCase()}${verb.slice(1)} ${e.title}` };
  if (verb === "create") {
    return {
      ...common,
      doc: `Brings a new ${title} into the library; it needs a free id, an existing material and ${e.profile ? "a valid profile" : "valid dimensions"}.`,
      props: [idProp(family.refKind, "identity", { en: `${e.title} id`, de: `${family.de}-Id` }), prop(e.snake, e.Pascal, record(e.Pascal), "record", { en: e.title, de: family.de }, 20)],
      label: { en: `format!("Create ${title} \\"{}\\"", self.${e.snake}.name)`, de: `format!("${family.de} \\"{}\\" anlegen", self.${e.snake}.name)` },
    };
  }
  if (verb === "delete") {
    return {
      ...common,
      doc: `Removes a ${title} that no ${e.usersNoun === "openings" ? "opening" : e.usersNoun.slice(0, -1)} uses.`,
      props: [idProp(family.refKind, "target", { en: e.title, de: family.de })],
      label: { en: `format!("Delete ${title} \\"{}\\"", self.id)`, de: `format!("${family.de} \\"{}\\" löschen", self.id)` },
    };
  }
  return {
    ...common,
    doc: `Changes exactly the provided fields of a ${title}; every element of the type follows by inference.`,
    props: [idProp(family.refKind, "target", { en: e.title, de: family.de }), ...optionalProps()],
    label: { en: `format!("Change ${title} \\"{}\\"", self.id)`, de: `format!("${family.de} \\"{}\\" ändern", self.id)` },
  };
};

const profileCases = (family: Family) => {
  const column_ = family.entity.snake === "column_type";
  const existing = column_ ? "ct-400" : "bt-i300";
  const record_ = (profile: unknown, material = column_ ? "m-concrete" : "m-steel") => profiled(column_ ? "Column" : "Beam", profile, material);
  const withUser = base(column_ ? { columns: { "c-1": column } } : { beams: { "b-1": beam } });
  const field = family.entity.snake;
  const createCases: Spec[] = [
    adds(column_ ? "adds-a-round-column" : "adds-an-i-shape", base(), { id: "new-1", [field]: record_(column_ ? circle(0.5) : ishape(0.1, 0.2, 0.006, 0.009)) }),
    adds(column_ ? "adds-an-l-shaped-outline" : "adds-a-bulged-outline", base(), { id: "new-2", [field]: record_(column_ ? lShape : dShape) }),
    refuses("duplicate", base(), { id: existing, [field]: record_(rect(0.3, 0.3)) }, "mutation.duplicate-id", [existing]),
    refuses("material-missing", base(), { id: "new-1", [field]: record_(rect(0.3, 0.3), "m-glass") }, "mutation.target-missing", [field, "material"]),
    refuses(column_ ? "non-positive-profile" : "oversized-flanges", base(), { id: "new-1", [field]: record_(column_ ? rect(0, 0.4) : ishape(0.15, 0.3, 0.007, 0.16)) }, "mutation.invariant", [field, "profile"]),
    refuses(column_ ? "clockwise-outline" : "self-crossing-outline", base(), { id: "new-1", [field]: record_(column_ ? clockwise : bowTie) }, "mutation.invariant", [field, "profile"]),
  ];
  const deleteCases: Spec[] = [
    adds("removes", base(), { id: existing }),
    refuses("in-use", withUser, { id: existing }, "mutation.target-referenced", [existing]),
    refuses("missing", base(), { id: "gone-1" }, "mutation.target-missing", ["gone-1"]),
  ];
  const original = column_ ? base().column_types["ct-400"] : base().beam_types["bt-i300"];
  const setCases: Spec[] = [
    adds("renames", base(), { id: existing, name: "Renamed" }),
    adds("swaps-the-profile", base(), { id: existing, profile: column_ ? circle(0.45) : rect(0.2, 0.4) }),
    adds("changes-every-field", base(), { id: existing, name: "Everything", profile: ishape(0.2, 0.4, 0.008, 0.013), material: column_ ? "m-steel" : "m-concrete" }),
    { name: "missing", before: base(), mutation: { id: "gone-1", name: "x" }, outcome: missing("gone-1") },
    refuses("material-missing", base(), { id: existing, material: "m-glass" }, "mutation.target-missing", ["material"]),
    refuses("invalid-profile", base(), { id: existing, profile: column_ ? circle(0) : clockwise }, "mutation.invariant", ["profile"]),
    { name: "unchanged", before: base(), mutation: { id: existing, name: original.name, profile: original.profile }, outcome: noop(existing) },
  ];
  return { createCases, deleteCases, setCases };
};

const windowCases = (family: Family) => {
  const record_ = (over: Record<string, unknown> = {}) => windowRecord({ name: "Window 0.60", width: 0.6, height: 1, sill: 1.1, frame_width: 0.05, panes: 1, ...over });
  const create = (name: string, over: Record<string, unknown>, outcome: Case["outcome"]): Spec => ({ name, before: base(), mutation: { id: "win-2", window_type: record_(over) }, outcome });
  const invariant = (field: string) => reject("mutation.invariant", ["window_type", field]);
  const withOpening = base({ openings: { "o-1": F.opening("w-south", "Window") } });
  const set = (name: string, mutation: Record<string, unknown>, outcome: Case["outcome"]): Spec => ({ name, before: base(), mutation: { id: "win-1", ...mutation }, outcome });
  return {
    createCases: [
      create("adds", {}, ok),
      create("adds-a-floor-level-window", { height: 2.1, sill: 0 }, ok),
      { name: "duplicate", before: base(), mutation: { id: "win-1", window_type: record_() }, outcome: duplicate("win-1") },
      create("material-missing", { material: "m-glass" }, reject("mutation.target-missing", ["window_type", "material"])),
      create("non-positive-width", { width: 0 }, invariant("width")),
      create("negative-sill", { sill: -0.1 }, invariant("sill")),
      create("no-panes", { panes: 0 }, invariant("panes")),
      create("non-positive-frame", { frame_width: 0 }, invariant("frame_width")),
    ] as Spec[],
    deleteCases: [
      adds("removes", base(), { id: "win-1" }),
      refuses("in-use", withOpening, { id: "win-1" }, "mutation.target-referenced", ["win-1"]),
      refuses("missing", base(), { id: "gone-1" }, "mutation.target-missing", ["gone-1"]),
    ],
    setCases: [
      set("widens", { width: 1.5 }, ok),
      set("lowers-the-sill-and-adds-panes", { sill: 0, panes: 3 }, ok),
      set("renames-and-reframes", { name: "Steel window", frame_width: 0.08, frame_depth: 0.12, material: "m-steel" }, ok),
      { name: "missing", before: base(), mutation: { id: "gone-1", width: 1 }, outcome: missing("gone-1") },
      set("material-missing", { material: "m-glass" }, reject("mutation.target-missing", ["material"])),
      set("non-positive-width", { width: 0 }, reject("mutation.invariant", ["width"])),
      set("negative-sill", { sill: -0.2 }, reject("mutation.invariant", ["sill"])),
      set("no-panes", { panes: 0 }, reject("mutation.invariant", ["panes"])),
      set("non-positive-frame-depth", { frame_depth: -0.1 }, reject("mutation.invariant", ["frame_depth"])),
      set("unchanged", { width: 1.2, panes: 2 }, noop("win-1")),
    ] as Spec[],
  };
};

const doorCases = (family: Family) => {
  const record_ = (over: Record<string, unknown> = {}) => doorRecord({ name: "Door 1.80", width: 1.8, leaves: "Double", swing: "Right", ...over });
  const create = (name: string, over: Record<string, unknown>, outcome: Case["outcome"]): Spec => ({ name, before: base(), mutation: { id: "door-2", door_type: record_(over) }, outcome });
  const invariant = (field: string) => reject("mutation.invariant", ["door_type", field]);
  const withOpening = base({ openings: { "o-1": doorOpening } });
  const set = (name: string, mutation: Record<string, unknown>, outcome: Case["outcome"]): Spec => ({ name, before: base(), mutation: { id: "door-1", ...mutation }, outcome });
  return {
    createCases: [
      create("adds-a-double-door", {}, ok),
      { name: "duplicate", before: base(), mutation: { id: "door-1", door_type: record_() }, outcome: duplicate("door-1") },
      create("material-missing", { material: "m-glass" }, reject("mutation.target-missing", ["door_type", "material"])),
      create("non-positive-height", { height: 0 }, invariant("height")),
      create("non-positive-frame", { frame_depth: -0.1 }, invariant("frame_depth")),
    ] as Spec[],
    deleteCases: [
      adds("removes", base(), { id: "door-1" }),
      refuses("in-use", withOpening, { id: "door-1" }, "mutation.target-referenced", ["door-1"]),
      refuses("missing", base(), { id: "gone-1" }, "mutation.target-missing", ["gone-1"]),
    ],
    setCases: [
      set("widens", { width: 1.1 }, ok),
      set("swaps-leaves-and-swing", { leaves: "Double", swing: "Right" }, ok),
      { name: "missing", before: base(), mutation: { id: "gone-1", width: 1 }, outcome: missing("gone-1") },
      set("material-missing", { material: "m-glass" }, reject("mutation.target-missing", ["material"])),
      set("non-positive-height", { height: 0 }, reject("mutation.invariant", ["height"])),
      set("non-positive-frame-width", { frame_width: 0 }, reject("mutation.invariant", ["frame_width"])),
      set("unchanged", { leaves: "Single", swing: "Left" }, noop("door-1")),
    ] as Spec[],
  };
};

const casesOf = (family: Family) => (family.entity.snake === "window_type" ? windowCases(family) : family.entity.snake === "door_type" ? doorCases(family) : profileCases(family));

export const leaves: Leaf[] = families.flatMap((family) => {
  const { createCases, deleteCases, setCases } = casesOf(family);
  return [leafOf(family, "create", 0, numbered(createCases)), leafOf(family, "delete", 1, numbered(deleteCases)), leafOf(family, "set", 2, numbered(setCases))];
});

const optionalNames = (family: Family) => family.fieldNames;

const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);

const patchOptional = (leaf: Leaf, family: Family) => {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  const rust = join(leafDir, em(0x1f9a0) + "mutation", RS);
  let source = readFileSync(rust, "utf8");
  for (const name of optionalNames(family)) {
    const field = leaf.props.find((p) => p.name === name)!;
    source = source.replace(`    pub ${name}: ${field.rust},`, `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<${field.rust}>,`);
  }
  writeFileSync(rust, source);
  const schemaFile = join(leafDir, em(0x1f9ec) + "schema", JSONF);
  const doc = JSON.parse(readFileSync(schemaFile, "utf8")) as { required: string[] };
  doc.required = doc.required.filter((name) => !optionalNames(family).includes(name as never));
  writeFileSync(schemaFile, JSON.stringify(doc, null, 2) + "\n");
};

const hand = (leaf: Leaf, diff: string, inverse: string) => {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  writeFileSync(join(leafDir, em(0x1f53a) + "diff", RS), diff);
  writeFileSync(join(leafDir, em(0x21a9) + "inverse", RS), inverse);
};

const spliceOnce = (file: string, present: string, edit: (source: string) => string) => {
  const source = readFileSync(file, "utf8");
  if (source.includes(present)) return false;
  const next = edit(source);
  if (next === source) throw new Error(`anchor not found in ${file}`);
  writeFileSync(file, next);
  return true;
};

const assertEmojiFree = () => {
  const mine = new Map(leaves.map((leaf) => [leaf.emoji, leaf.kind]));
  for (const name of readdirSync(mutations)) {
    const first = name.codePointAt(0)!;
    const kind = name.replace(/^[^a-z]+/, "");
    if (mine.has(first) && mine.get(first) !== kind) throw new Error(`emoji collision: ${name} vs ${mine.get(first)}`);
  }
  if (new Set(leaves.map((leaf) => leaf.emoji)).size !== leaves.length) throw new Error("own emoji collision");
};

if (import.meta.main) {
  assertEmojiFree();
  const out = join(import.meta.dir, "🗑️generated", "m-profiles-openings-types");
  mkdirSync(out, { recursive: true });
  const snapshot = child(schema, "snapshot");
  const validity = join(snapshot, em(0x2705) + "validity");
  mkdirSync(validity, { recursive: true });
  writeFileSync(join(validity, RS), validityModule);
  const snapshotRs = join(snapshot, RS);
  const modLine = `#[path = "${em(0x2705)}validity/${RS}"]\npub mod validity;\n\npub use validity::*;\n`;
  console.log(spliceOnce(snapshotRs, "pub mod validity;", (source) => source.replace("pub use values::*;\n", (match) => `${match}\n${modLine}`)) ? "snapshot: validity mounted" : "snapshot: validity already mounted");

  let mounts = "";
  leaves.forEach((leaf, i) => {
    const family = families[Math.floor(i / 3)];
    const verb = leaf.verb as "create" | "delete" | "set";
    mounts += emitLeaf(leaf);
    if (verb === "set") patchOptional(leaf, family);
    const e = family.entity;
    hand(leaf, { create: createDiff, delete: deleteDiff, set: setDiff }[verb](e), { create: createInverse, delete: deleteInverse, set: setInverse }[verb](e));
  });
  writeFileSync(join(out, "mounts.txt"), mounts);

  const aggregate = join(mutations, RS);
  const variants = leaves.map((leaf) => `    ${leaf.variant}(super::${leaf.kind.replaceAll("-", "_")}::${leaf.variant}),\n`).join("");
  const kinds = leaves.map((leaf) => `    "${leaf.kind}",\n`).join("");
  console.log(spliceOnce(aggregate, "CreateColumnType(", (source) => source.replace(/(    SetWallTop\(super::set_wall_top::SetWallTop\),\n)/, (match) => match + variants).replace(/(    "set-wall-top",\n)/, (match) => match + kinds)) ? "aggregate: registered" : "aggregate: already registered");

  const root = join(artifact, RS);
  const anchor = `//#endregion ${em(0x1f516)}Leaves`;
  console.log(spliceOnce(root, "pub mod create_column_type", (source) => {
    const at = source.indexOf(anchor);
    if (at < 0) return source;
    const lineStart = source.lastIndexOf("\n", at) + 1;
    return source.slice(0, lineStart) + mounts + source.slice(lineStart);
  }) ? "root: mounted" : "root: already mounted");
  console.log(`emitted ${leaves.length} leaves`);
  void existsSync;
}
