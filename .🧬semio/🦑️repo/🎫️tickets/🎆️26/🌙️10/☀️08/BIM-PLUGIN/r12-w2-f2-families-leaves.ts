#!/usr/bin/env bun
/**
 * 🧩 Wave W2 `w2-f2-families` (binary tags 23000..23007): the eight family leaves of `s.bim.model@1`: create/set/delete of a family, set/remove of one of its parameters (a parameter is addressed by family
 * and name, the key `family.name`) and create/set/delete of one of its solids. `bun r12-w2-f2-families-leaves.ts` rewrites the boilerplate and the fixtures through `emitLeaf`, fixes the optional `set-*`
 * fields of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`, never overwritten), mounts the leaves once in the artifact root and registers the variants in the
 * mutation aggregate. `after`/`diff` fixtures of applied cases are never overwritten: bless them with `BIM_BLESS=1 cargo test`.
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
const text = (name: string, label: Label, order: number, role: "identity" | "value" = "value"): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role, label, group: role === "identity" ? "identity" : "value", order } });
const choice = (name: string, def: string, label: Label, order: number): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "select", role: "value", label, group: "value", order } });
const familyRef = (order = 10): Mine => ({ name: "family", rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "target", label: { en: "Family", de: "Familie" }, ref: { kind: "family" }, group: "target", order } });
const formulaProp = (name: string, label: Label, order: number): Mine => text(name, label, order);

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2, 0x1f4aa];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757, 0x1f6c7, 0x1f4a2, 0x1f6af, 0x1f5ef];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Leaf["cases"][number]["outcome"] };
const cases = (rows: Row[]): Leaf["cases"] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

//#region 🔖️Fixtures
const zero = { x: "0 m", y: "0 m", z: "0 m" };
const rectangle = (width: string, depth: string) => ({ Rectangle: { width, depth } });
const iShape = (width: string, depth: string, web: string, flange: string) => ({ IShape: { width, depth, web, flange } });
const extrusion = (profile: unknown, base: string, height: string) => ({ Extrusion: { profile, base, height } });
const cuboid = (x: string, y: string, z: string, width: string, depth: string, height: string) => ({ Cuboid: { x, y, z, width, depth, height } });
const family = (name: string, category = "Furniture") => ({ name, category });
const parameter = (owner: string, name: string, kind: string, value: string) => ({ family: owner, name, kind, value });
const solid = (owner: string, name: string, shape: unknown, extra: Record<string, unknown> = {}) => ({ family: owner, name, shape, material: "\"m-brick\"", visible: "true", offset: zero, ...extra });
const key = (owner: string, name: string) => `${owner}.${name}`;
const pair = (owner: string, name: string, kind: string, value: string) => [key(owner, name), parameter(owner, name, kind, value)] as const;
const mapOf = (...rows: (readonly [string, unknown])[]) => Object.fromEntries(rows);

const table = () => ({
  families: { "fam-table": family("Table") },
  family_parameters: mapOf(pair("fam-table", "width", "Length", "1.6 m"), pair("fam-table", "depth", "Length", "0.8 m"), pair("fam-table", "double", "Length", "2 * width")),
  family_solids: { "s-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "0.7 m", "width", "depth", "40 mm")) },
});
const library = (extra: Record<string, unknown> = {}) => ({
  ...F.scene(),
  families: { "fam-table": family("Table"), "fam-hea": family("HEA 200", "Profile") },
  family_parameters: mapOf(pair("fam-table", "width", "Length", "1.6 m"), pair("fam-table", "depth", "Length", "0.8 m"), pair("fam-table", "double", "Length", "2 * width"), pair("fam-hea", "h", "Length", "190 mm"), pair("fam-hea", "b", "Length", "200 mm")),
  family_solids: { "s-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "0.7 m", "width", "depth", "40 mm")), "s-hea": solid("fam-hea", "HEA", extrusion(iShape("b", "h", "6.5 mm", "10 mm"), "0 m", "1 m")) },
  ...extra,
});
const empty = () => ({ ...F.scene(), families: { "fam-bare": family("Bare") } });
const withBeamType = () => library({ beam_types: { "bt-hea": { name: "HEA beam", profile: { Family: { family: "fam-hea" } }, material: "m-brick" } } });
const withColumnType = () => library({ column_types: { "ct-hea": { name: "HEA column", profile: { Family: { family: "fam-hea" } }, material: "m-brick" } } });
const withTakenKey = () => library({ walls: { ...F.scene().walls, "fam-table.legs": F.wall("st-ground", "wt-300", F.line([0, 0], [1, 0]), F.storeyTop(0), "Odd wall") } });
//#endregion 🔖️Fixtures

type Extra = { optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf): MineLeaf => (extras.set(spec.kind, { optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const familyLabel = { en: "Family", de: "Familie" };
const solidLabel = { en: "Family solid", de: "Familienkörper" };
const nameLabel = { en: "Name", de: "Name" };
const target = "vec![self.id.clone()]";
const parameterTarget = "vec![format!(\"{}.{}\", self.family, self.name)]";

export const leaves: MineLeaf[] = [
  leaf({
    kind: "create-family", emoji: 0x1f9e9, variant: "CreateFamily", verb: "create", entity: "family", binaryTag: 23000, displayName: "Create Family",
    doc: "Brings a new, empty parametric family into the project: a name and a category; parameters and solids are added afterwards.",
    props: [id("family", "identity", { en: "Family id", de: "Familien-Id" }, 10), recordProp("family", "Family", familyLabel)],
    label: { en: 'format!("Create family \\"{}\\"", self.family.name)', de: 'format!("Familie \\"{}\\" anlegen", self.family.name)' },
    target,
    cases: cases([
      { name: "creates-a-table-family", before: F.scene(), mutation: { id: "fam-table", family: family("Table") }, outcome: ok },
      { name: "creates-a-profile-family", before: library(), mutation: { id: "fam-rhs", family: family("RHS 100", "Profile") }, outcome: ok },
      { name: "duplicate", before: library(), mutation: { id: "fam-table", family: family("Again") }, outcome: reject("mutation.duplicate-id", ["fam-table"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", family: family("Wall") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", before: library(), mutation: { id: "fam-new", family: family(" ") }, outcome: reject("mutation.invariant", ["family", "name"]) },
    ]),
  }),
  leaf({
    kind: "delete-family", emoji: 0x1fa85, variant: "DeleteFamily", verb: "delete", entity: "family", binaryTag: 23001, displayName: "Delete Family",
    doc: "Removes a family with its parameters and solids; refused while a column type, beam type, curtain wall type, wall sweep or railing uses it as a profile.",
    props: [id("family", "target", familyLabel)],
    label: { en: 'format!("Delete family \\"{}\\" with its parts", self.id)', de: 'format!("Familie \\"{}\\" mit ihren Teilen löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes-with-its-parts", before: library(), mutation: { id: "fam-table" }, outcome: ok },
      { name: "removes-an-empty-family", before: empty(), mutation: { id: "fam-bare" }, outcome: ok },
      { name: "used-by-a-column-type", before: withColumnType(), mutation: { id: "fam-hea" }, outcome: reject("mutation.target-referenced", ["fam-hea"]) },
      { name: "used-by-a-beam-type", before: withBeamType(), mutation: { id: "fam-hea" }, outcome: reject("mutation.target-referenced", ["fam-hea"]) },
      { name: "missing", before: library(), mutation: { id: "fam-ghost" }, outcome: reject("mutation.target-missing", ["fam-ghost"]) },
    ]),
  }),
  leaf({
    kind: "set-family", emoji: 0x1fa86, variant: "SetFamily", verb: "set", entity: "family", binaryTag: 23002, displayName: "Set Family",
    doc: "Sets exactly the provided fields of a family, its name and its category; a profile family cannot leave its category while a type uses it as a profile.",
    props: [id("family", "target", familyLabel), sparse(text("name", nameLabel, 20), keep), sparse(choice("category", "FamilyCategory", { en: "Category", de: "Kategorie" }, 30), keep)],
    label: { en: 'format!("Change family \\"{}\\"", self.id)', de: 'format!("Familie \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: library(), mutation: { id: "fam-table", name: "Dining table" }, outcome: ok },
      { name: "changes-the-category", before: library(), mutation: { id: "fam-table", category: "Casework" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: library(), mutation: { id: "fam-table", name: "Table", category: "Equipment" }, outcome: ok },
      { name: "unchanged", before: library(), mutation: { id: "fam-table", name: "Table", category: "Furniture" }, outcome: reject("mutation.no-op", ["fam-table"]) },
      { name: "names-no-field", before: library(), mutation: { id: "fam-table" }, outcome: reject("mutation.no-op", ["fam-table"]) },
      { name: "blank-name", before: library(), mutation: { id: "fam-table", name: "  " }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "leaves-profile-while-used", before: withColumnType(), mutation: { id: "fam-hea", category: "Generic" }, outcome: reject("mutation.target-referenced", ["fam-hea"]) },
      { name: "missing", before: library(), mutation: { id: "fam-ghost", name: "Ghost" }, outcome: reject("mutation.target-missing", ["fam-ghost"]) },
    ]),
  }),
  leaf({
    kind: "set-family-parameter", emoji: 0x1f521, variant: "SetFamilyParameter", verb: "set", entity: "family-parameter", binaryTag: 23003, displayName: "Set Family Parameter",
    doc: "Sets exactly the provided kind and formula of the parameter `name` of a family, or adds it (a new parameter needs both). Formulas are canonical text of the expression language, use only existing parameters and never close a circle.",
    props: [familyRef(10), text("name", { en: "Parameter name", de: "Parametername" }, 20, "identity"), sparse(choice("kind", "ParameterKind", { en: "Kind", de: "Art" }, 30), keep), sparse(formulaProp("value", { en: "Formula", de: "Formel" }, 40), keep)],
    label: { en: 'format!("Set parameter \\"{}\\" of family \\"{}\\"", self.name, self.family)', de: 'format!("Parameter \\"{}\\" der Familie \\"{}\\" setzen", self.name, self.family)' },
    target: parameterTarget,
    cases: cases([
      { name: "adds-a-parameter", before: library(), mutation: { family: "fam-table", name: "height", kind: "Length", value: "0.74 m" }, outcome: ok },
      { name: "adds-a-dependent-parameter", before: library(), mutation: { family: "fam-table", name: "quad", kind: "Length", value: "double * 2" }, outcome: ok },
      { name: "changes-a-formula", before: library(), mutation: { family: "fam-table", name: "width", value: "1.8 m" }, outcome: ok },
      { name: "changes-the-kind", before: library(), mutation: { family: "fam-table", name: "double", kind: "Real", value: "2 * width / 1 m" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: library(), mutation: { family: "fam-table", name: "width", kind: "Length", value: "1.7 m" }, outcome: ok },
      { name: "unchanged", before: library(), mutation: { family: "fam-table", name: "width", kind: "Length", value: "1.6 m" }, outcome: reject("mutation.no-op", ["fam-table.width"]) },
      { name: "names-no-field", before: library(), mutation: { family: "fam-table", name: "width" }, outcome: reject("mutation.no-op", ["fam-table.width"]) },
      { name: "family-missing", before: library(), mutation: { family: "fam-ghost", name: "width", kind: "Length", value: "1 m" }, outcome: reject("mutation.target-missing", ["family"]) },
      { name: "bad-name", before: library(), mutation: { family: "fam-table", name: "2 wide", kind: "Length", value: "1 m" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "formula-does-not-parse", before: library(), mutation: { family: "fam-table", name: "width", value: "2 *" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "unknown-parameter", before: library(), mutation: { family: "fam-table", name: "width", value: "ghost + 1 m" }, outcome: reject("mutation.target-missing", ["value"]) },
      { name: "closes-a-circle", before: library(), mutation: { family: "fam-table", name: "width", value: "double" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "new-needs-a-kind", before: library(), mutation: { family: "fam-table", name: "height", value: "0.74 m" }, outcome: reject("mutation.invariant", ["kind"]) },
      { name: "new-needs-a-formula", before: library(), mutation: { family: "fam-table", name: "height", kind: "Length" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "id-taken-by-another-kind", before: withTakenKey(), mutation: { family: "fam-table", name: "legs", kind: "Integer", value: "4" }, outcome: reject("mutation.duplicate-id", ["fam-table.legs"]) },
    ]),
  }),
  leaf({
    kind: "remove-family-parameter", emoji: 0x1f520, variant: "RemoveFamilyParameter", verb: "remove", entity: "family-parameter", binaryTag: 23004, displayName: "Remove Family Parameter",
    doc: "Removes the parameter `name` of a family; refused while the formula of another parameter or a solid of the family refers to it.",
    props: [familyRef(10), text("name", { en: "Parameter name", de: "Parametername" }, 20, "identity")],
    label: { en: 'format!("Remove parameter \\"{}\\" of family \\"{}\\"", self.name, self.family)', de: 'format!("Parameter \\"{}\\" der Familie \\"{}\\" entfernen", self.name, self.family)' },
    target: parameterTarget,
    cases: cases([
      { name: "removes-an-unused-parameter", before: library(), mutation: { family: "fam-table", name: "double" }, outcome: ok },
      { name: "used-by-a-formula", before: library(), mutation: { family: "fam-table", name: "width" }, outcome: reject("mutation.target-referenced", ["fam-table.width"]) },
      { name: "used-by-a-solid", before: library(), mutation: { family: "fam-table", name: "depth" }, outcome: reject("mutation.target-referenced", ["fam-table.depth"]) },
      { name: "missing", before: library(), mutation: { family: "fam-table", name: "ghost" }, outcome: reject("mutation.target-missing", ["fam-table.ghost"]) },
      { name: "family-missing", before: library(), mutation: { family: "fam-ghost", name: "width" }, outcome: reject("mutation.target-missing", ["family"]) },
    ]),
  }),
  leaf({
    kind: "create-family-solid", emoji: 0x1f536, variant: "CreateFamilySolid", verb: "create", entity: "family-solid", binaryTag: 23005, displayName: "Create Family Solid",
    doc: "Adds a solid to a family: an extrusion, revolution, sweep or cuboid whose dimensions, material and visibility are formulas of the family parameters.",
    props: [id("family-solid", "identity", { en: "Solid id", de: "Körper-Id" }, 10), recordProp("solid", "FamilySolid", solidLabel)],
    label: { en: 'format!("Create family solid \\"{}\\"", self.solid.name)', de: 'format!("Familienkörper \\"{}\\" anlegen", self.solid.name)' },
    target,
    cases: cases([
      { name: "extrudes-a-profile", before: library(), mutation: { id: "s-plate", solid: solid("fam-table", "Plate", extrusion(rectangle("width", "depth"), "0 m", "20 mm")) }, outcome: ok },
      { name: "places-a-cuboid", before: library(), mutation: { id: "s-leg", solid: solid("fam-table", "Leg", cuboid("0 m", "0 m", "0 m", "60 mm", "60 mm", "0.7 m"), { offset: { x: "0.05 m", y: "0.05 m", z: "0 m" } }) }, outcome: ok },
      { name: "sweeps-a-rail", before: library(), mutation: { id: "s-rail", solid: solid("fam-table", "Rail", { Sweep: { profile: rectangle("40 mm", "40 mm"), path: [{ x: "0 m", y: "0 m" }, { x: "width", y: "0 m" }, { x: "width", y: "depth" }] }, }) }, outcome: ok },
      { name: "revolves-a-foot", before: library(), mutation: { id: "s-foot", solid: solid("fam-table", "Foot", { Revolution: { profile: { Polygon: { points: [{ x: "0 m", y: "0 m" }, { x: "50 mm", y: "0 m" }, { x: "50 mm", y: "20 mm" }, { x: "0 m", y: "20 mm" }] } }, axis: "Z", angle: "360 deg" } }) }, outcome: ok },
      { name: "duplicate", before: library(), mutation: { id: "s-top", solid: solid("fam-table", "Top", cuboid("0 m", "0 m", "0 m", "1 m", "1 m", "1 m")) }, outcome: reject("mutation.duplicate-id", ["s-top"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", solid: solid("fam-table", "Wall", cuboid("0 m", "0 m", "0 m", "1 m", "1 m", "1 m")) }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "family-missing", before: library(), mutation: { id: "s-new", solid: solid("fam-ghost", "Box", cuboid("0 m", "0 m", "0 m", "1 m", "1 m", "1 m")) }, outcome: reject("mutation.target-missing", ["solid", "family"]) },
      { name: "blank-name", before: library(), mutation: { id: "s-new", solid: solid("fam-table", " ", cuboid("0 m", "0 m", "0 m", "1 m", "1 m", "1 m")) }, outcome: reject("mutation.invariant", ["solid", "name"]) },
      { name: "formula-does-not-parse", before: library(), mutation: { id: "s-new", solid: solid("fam-table", "Box", cuboid("0 m", "0 m", "0 m", "width +", "1 m", "1 m")) }, outcome: reject("mutation.invariant", ["solid", "width"]) },
      { name: "unknown-parameter", before: library(), mutation: { id: "s-new", solid: solid("fam-table", "Box", cuboid("0 m", "0 m", "0 m", "ghost", "1 m", "1 m")) }, outcome: reject("mutation.target-missing", ["solid", "width"]) },
      { name: "polygon-too-small", before: library(), mutation: { id: "s-new", solid: solid("fam-table", "Flat", extrusion({ Polygon: { points: [{ x: "0 m", y: "0 m" }, { x: "1 m", y: "0 m" }] } }, "0 m", "1 m")) }, outcome: reject("mutation.invariant", ["solid", "shape"]) },
    ]),
  }),
  leaf({
    kind: "delete-family-solid", emoji: 0x1f539, variant: "DeleteFamilySolid", verb: "delete", entity: "family-solid", binaryTag: 23006, displayName: "Delete Family Solid",
    doc: "Removes one solid from its family; the parameters stay untouched.",
    props: [id("family-solid", "target", solidLabel)],
    label: { en: 'format!("Delete family solid \\"{}\\"", self.id)', de: 'format!("Familienkörper \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: library(), mutation: { id: "s-top" }, outcome: ok },
      { name: "missing", before: library(), mutation: { id: "s-ghost" }, outcome: reject("mutation.target-missing", ["s-ghost"]) },
    ]),
  }),
  leaf({
    kind: "set-family-solid", emoji: 0x1f4e6, variant: "SetFamilySolid", verb: "set", entity: "family-solid", binaryTag: 23007, displayName: "Set Family Solid",
    doc: "Sets exactly the provided fields of a family solid: name, shape, material, visibility and offset; absent fields stay untouched.",
    props: [
      id("family-solid", "target", solidLabel),
      sparse(text("name", nameLabel, 20), keep),
      sparse(recordProp("shape", "SolidShape", { en: "Shape", de: "Form" }, 30), keep),
      sparse(formulaProp("material", { en: "Material formula", de: "Materialformel" }, 40), keep),
      sparse(formulaProp("visible", { en: "Visible formula", de: "Sichtbarkeitsformel" }, 50), keep),
      sparse(recordProp("offset", "ExprPoint3", { en: "Offset", de: "Versatz" }, 60), keep),
    ],
    label: { en: 'format!("Change family solid \\"{}\\"", self.id)', de: 'format!("Familienkörper \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "hides-the-solid", before: library(), mutation: { id: "s-top", visible: "width > 2 m" }, outcome: ok },
      { name: "reshapes", before: library(), mutation: { id: "s-top", shape: cuboid("0 m", "0 m", "0.7 m", "width", "depth", "60 mm") }, outcome: ok },
      { name: "moves", before: library(), mutation: { id: "s-top", offset: { x: "0.1 m", y: "0 m", z: "0 m" } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: library(), mutation: { id: "s-top", name: "Top", visible: "false" }, outcome: ok },
      { name: "unchanged", before: library(), mutation: { id: "s-top", visible: "true", name: "Top" }, outcome: reject("mutation.no-op", ["s-top"]) },
      { name: "names-no-field", before: library(), mutation: { id: "s-top" }, outcome: reject("mutation.no-op", ["s-top"]) },
      { name: "formula-does-not-parse", before: library(), mutation: { id: "s-top", visible: "(" }, outcome: reject("mutation.invariant", ["visible"]) },
      { name: "unknown-parameter", before: library(), mutation: { id: "s-top", visible: "ghost > 1 m" }, outcome: reject("mutation.target-missing", ["visible"]) },
      { name: "polygon-too-small", before: library(), mutation: { id: "s-hea", shape: extrusion({ Polygon: { points: [{ x: "0 m", y: "0 m" }] } }, "0 m", "1 m") }, outcome: reject("mutation.invariant", ["shape"]) },
      { name: "blank-name", before: library(), mutation: { id: "s-top", name: "" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "missing", before: library(), mutation: { id: "s-ghost", visible: "true" }, outcome: reject("mutation.target-missing", ["s-ghost"]) },
    ]),
  }),
];
void table;

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...parts: unknown[]) => strings.reduce((out, chunk, index) => out + chunk + (index < parts.length ? String(parts[index]) : ""), "").replaceAll("¶", "`");

const hand: Record<string, { diff: string; inverse: string }> = {
  "create-family": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateFamily¶: one created family entry. The id is free in every collection and the name is not blank. Parameters and solids come afterwards.

use super::super::elements;
use super::super::family_rules;
use super::CreateFamily;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = family_rules::family_record_fault(&payload.family) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("family")));
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Created(payload.family.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateFamily¶: the concrete ¶DeleteFamily¶ of the id it created, none when the id was already taken.

use super::super::delete_family::DeleteFamily;
use super::CreateFamily;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.families.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteFamily(DeleteFamily { id: payload.id.clone() })]
}
`,
  },
  "delete-family": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteFamily¶: the family, its parameters and its solids leave in one sparse diff. A family a column type, beam type, curtain wall type mullion, wall sweep or railing section
//! uses as its profile cannot go: refused as ¶mutation.target-referenced¶.

use super::super::family_rules;
use super::DeleteFamily;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(noun) = family_rules::profile_user(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \\"{}\\" is still the profile of {noun}.", payload.id), [payload.id.clone()]);
    }
    let parameters: Vec<String> = base.family_parameters.iter().filter(|(_, row)| row.family == payload.id).map(|(key, _)| key.clone()).collect();
    let solids: Vec<String> = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, _)| id.clone()).collect();
    let removed = parameters.len() + solids.len();
    let outcome = MutationOutcome::new(ModelDiff {
        families: Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted)),
        family_parameters: (!parameters.is_empty()).then(|| KeyedDelta(parameters.into_iter().map(|key| (key, Entry::Deleted)).collect())),
        family_solids: (!solids.is_empty()).then(|| KeyedDelta(solids.into_iter().map(|id| (id, Entry::Deleted)).collect())),
        ..ModelDiff::default()
    });
    if removed == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Family \\"{}\\" took {removed} parameter(s) and solid(s) with it.", payload.id))
    }
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteFamily¶: one concrete create per removed record. The store replays the vector reversed, so it holds the solids first, then the parameters against their dependency order and the family
//! last: the family is recreated first, then every parameter after the ones its formula uses, then the solids that use the parameters.

use super::super::create_family::CreateFamily;
use super::super::create_family_solid::CreateFamilySolid;
use super::super::family_rules;
use super::super::set_family_parameter::SetFamilyParameter;
use super::DeleteFamily;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(family) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let solids = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, row)| ModelMutation::CreateFamilySolid(CreateFamilySolid { id: id.clone(), solid: row.clone() }));
    let parameters = family_rules::parameters_in_order(base, &payload.id).into_iter().rev().map(|row| ModelMutation::SetFamilyParameter(SetFamilyParameter { family: row.family.clone(), name: row.name.clone(), kind: Some(row.kind), value: Some(row.value.clone()) }));
    solids.chain(parameters).chain(std::iter::once(ModelMutation::CreateFamily(CreateFamily { id: payload.id.clone(), family: family.clone() }))).collect()
}
`,
  },
  "set-family": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetFamily¶: a sparse family patch of exactly the provided fields that differ. The name stays non-blank; a profile family cannot leave its category while a type uses it as a profile.
//! Providing only equal values, or no field, is a no-op.

use super::super::family_rules;
use super::SetFamily;
use crate::{Entry, FamilyCategory, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.families.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = family_rules::family_record_fault(&next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Family \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if record.category == FamilyCategory::Profile && next.category != FamilyCategory::Profile {
        if let Some(noun) = family_rules::profile_user(base, &payload.id) {
            return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \\"{}\\" is still the profile of {noun} and must stay a profile.", payload.id), [payload.id.clone()]);
        }
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetFamily¶: an absolute ¶SetFamily¶ restoring the base value of exactly the fields the forward really changes, none when the family is absent or nothing changes.

use super::SetFamily;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetFamily(SetFamily::from_patch(payload.id.clone(), restore))]
}
`,
  },
  "set-family-parameter": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetFamilyParameter¶: the parameter ¶name¶ of a family is one record keyed ¶family.name¶. An existing parameter gets a sparse patch of exactly the provided kind and formula
//! that differ (providing nothing new is a no-op); an absent one is created and needs both. A formula must parse, use only existing parameters of the family and never close a circle; whether it also
//! computes the declared kind is the business of the inference (a diagnostic), not of the diff. The key is free in every other collection.

use super::super::elements;
use super::super::family_rules;
use super::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{Entry, FamilyParameter, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamilyParameter, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.family) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \\"{}\\" does not exist.", payload.family), ["family"]);
    }
    if let Some(fault) = family_rules::name_fault(&payload.name) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    if let Some(text) = &payload.value {
        let fault = family_rules::formula_fault(text, "value").or_else(|| family_rules::reference_fault(base, &payload.family, text, "value")).or_else(|| family_rules::cycle_fault(base, &payload.family, &payload.name, text));
        if let Some(fault) = fault {
            return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
        }
    }
    let key = formula::parameter_id(&payload.family, &payload.name);
    if let Some(record) = base.family_parameters.get(&key) {
        let change = payload.patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Parameter \\"{}\\" of family \\"{}\\" already has these values.", payload.name, payload.family), [key]);
        }
        return MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Patched(change)));
    }
    if let Some(noun) = elements::taken(base, &key) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{key}\\" already exists."), [key]);
    }
    let Some(kind) = payload.kind else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A new parameter needs a kind.", ["kind"]);
    };
    let Some(value) = payload.value.clone() else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A new parameter needs a formula.", ["value"]);
    };
    MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Created(FamilyParameter { family: payload.family.clone(), name: payload.name.clone(), kind, value })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetFamilyParameter¶: for an existing parameter an absolute ¶SetFamilyParameter¶ restoring the base value of exactly the fields the forward really changes, for a created one the
//! concrete ¶RemoveFamilyParameter¶; none when the family is absent or nothing changes.

use super::super::remove_family_parameter::RemoveFamilyParameter;
use super::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamilyParameter, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.families.contains_key(&payload.family) {
        return Vec::new();
    }
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        None => vec![ModelMutation::RemoveFamilyParameter(RemoveFamilyParameter { family: payload.family.clone(), name: payload.name.clone() })],
        Some(record) => {
            let restore = payload.patch().minimal(record).restoring(record);
            if restore.is_empty() {
                return Vec::new();
            }
            vec![ModelMutation::SetFamilyParameter(SetFamilyParameter::from_patch(payload.family.clone(), payload.name.clone(), restore))]
        }
    }
}
`,
  },
  "remove-family-parameter": {
    diff: rs`//! ${DIFF} Diff constructor for ¶RemoveFamilyParameter¶: the parameter record leaves. Refused while the formula of another parameter or a solid of the family refers to it (computed from the authored
//! text of the formulas only).

use super::super::family_rules;
use super::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.family) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \\"{}\\" does not exist.", payload.family), ["family"]);
    }
    let key = formula::parameter_id(&payload.family, &payload.name);
    if !base.family_parameters.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Parameter \\"{}\\" of family \\"{}\\" does not exist.", payload.name, payload.family), [key]);
    }
    if let Some(user) = family_rules::parameter_user(base, &payload.family, &payload.name) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Parameter \\"{}\\" is still used by {user}.", payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Deleted))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶RemoveFamilyParameter¶: the concrete ¶SetFamilyParameter¶ carrying the kind and the formula of the removed parameter, none when it was absent.

use super::super::set_family_parameter::SetFamilyParameter;
use super::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        Some(record) => vec![ModelMutation::SetFamilyParameter(SetFamilyParameter { family: payload.family.clone(), name: payload.name.clone(), kind: Some(record.kind), value: Some(record.value.clone()) })],
        None => Vec::new(),
    }
}
`,
  },
  "create-family-solid": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateFamilySolid¶: one created solid entry. The id is free in every collection, the family exists, the name is not blank, every formula slot parses and uses only
//! existing parameters of the family, and a polygon has three points and a sweep path two. The mesh, the volume and the issues of the solid are inferred, never stored.

use super::super::elements;
use super::super::family_rules;
use super::CreateFamilySolid;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = family_rules::solid_fault(base, &payload.solid) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("solid")));
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Created(payload.solid.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateFamilySolid¶: the concrete ¶DeleteFamilySolid¶ of the id it created, none when the id was already taken.

use super::super::delete_family_solid::DeleteFamilySolid;
use super::CreateFamilySolid;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.family_solids.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteFamilySolid(DeleteFamilySolid { id: payload.id.clone() })]
}
`,
  },
  "delete-family-solid": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteFamilySolid¶: the solid leaves; the parameters of its family stay untouched.

use super::DeleteFamilySolid;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.family_solids.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family solid \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Deleted))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteFamilySolid¶: the concrete ¶CreateFamilySolid¶ carrying the full removed record, none when the solid was absent.

use super::super::create_family_solid::CreateFamilySolid;
use super::DeleteFamilySolid;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.family_solids.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateFamilySolid(CreateFamilySolid { id: payload.id.clone(), solid: record.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "set-family-solid": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetFamilySolid¶: a sparse solid patch of exactly the provided fields that differ. The solid that results must break none of the create rules (its family cannot change).
//! Providing only equal values, or no field, is a no-op.

use super::super::family_rules;
use super::SetFamilySolid;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.family_solids.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family solid \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = family_rules::solid_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Family solid \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetFamilySolid¶: an absolute ¶SetFamilySolid¶ restoring the base value of exactly the fields the forward really changes, none when the solid is absent or nothing changes.

use super::SetFamilySolid;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.family_solids.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetFamilySolid(SetFamilySolid::from_patch(payload.id.clone(), restore))]
}
`,
  },
};

type Patchable = { type: string; build: string; from: string; args: string };
const PATCHES: Record<string, Patchable> = {
  "set-family": { type: "FamilyPatch", args: "id: String", build: "FamilyPatch { name: self.name.clone(), category: self.category, ..Default::default() }", from: "Self { id, name: patch.name, category: patch.category }" },
  "set-family-parameter": { type: "FamilyParameterPatch", args: "family: String, name: String", build: "FamilyParameterPatch { kind: self.kind, value: self.value.clone(), ..Default::default() }", from: "Self { family, name, kind: patch.kind, value: patch.value }" },
  "set-family-solid": { type: "FamilySolidPatch", args: "id: String", build: "FamilySolidPatch { name: self.name.clone(), shape: self.shape.clone(), material: self.material.clone(), visible: self.visible.clone(), offset: self.offset.clone(), ..Default::default() }", from: "Self { id, name: patch.name, shape: patch.shape, material: patch.material, visible: patch.visible, offset: patch.offset }" },
};

const readdirName = (dir: string, suffix: string) => readdirSync(dir).find((name) => name.endsWith(suffix))!;

function fixLeaf(spec: MineLeaf) {
  const optionals = extras.get(spec.kind)!.optional;
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const file = join(dir, MUTATION + "mutation", RS);
  const patch = PATCHES[spec.kind];
  const used = new Set(spec.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  if (patch) used.add(patch.type);
  let source = readFileSync(file, "utf8");
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) source = source.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  if (patch && !source.includes("pub fn patch(")) {
    source = source.replace(
      /^impl MutationKind/m,
      `impl ${spec.variant} {\n    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.\n    pub fn patch(&self) -> ${patch.type} {\n        ${patch.build}\n    }\n\n    /// 🧩 The payload that provides exactly the fields \`patch\` names.\n    pub fn from_patch(${patch.args}, patch: ${patch.type}) -> Self {\n        ${patch.from}\n    }\n}\n\nimpl MutationKind`,
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
  const body = hand[spec.kind];
  for (const [folder, text] of [[DIFF + "diff", body.diff], [INVERSE + "inverse", body.inverse]] as const) {
    mkdirSync(join(dir, folder), { recursive: true });
    const file = join(dir, folder, RS);
    if (!existsSync(file)) writeFileSync(file, text);
  }
}
//#endregion 🔖️Hand

//#region 🔖️Run
const used = new Set(readdirSync(mutations).map((name) => name.codePointAt(0)));
for (const spec of leaves) {
  const present = readdirSync(mutations).some((name) => name.endsWith(spec.kind) && name.codePointAt(0) === spec.emoji);
  if (!present && used.has(spec.emoji)) throw new Error(`emoji ${spec.emoji.toString(16)} of ${spec.kind} is taken`);
}
const rootPath = join(artifact, RS);
const mounts: string[] = [];
for (const spec of leaves) {
  mounts.push(emitLeaf(spec as unknown as Leaf));
  fixLeaf(spec);
  writeOnce(spec);
}
const edit = (file: string, apply: (source: string) => string) => {
  const source = readFileSync(file, "utf8");
  const crlf = source.includes("\r\n");
  const next = apply(crlf ? source.replaceAll("\r\n", "\n") : source);
  writeFileSync(file, crlf ? next.replaceAll("\n", "\r\n") : next);
};
let mounted = 0;
edit(rootPath, (root) => {
  const anchor = "                        //#endregion 🔖️Leaves";
  for (const [index, spec] of leaves.entries()) {
    if (root.includes(`pub mod ${snake(spec.kind)} {`)) continue;
    root = root.replace(anchor, mounts[index] + anchor);
    mounted += 1;
  }
  return root;
});
let registered = 0;
edit(join(mutations, RS), (aggregate) => {
  if (!aggregate.includes("pub mod family_rules;")) {
    aggregate = aggregate.replace("pub mod annotating;\n", `pub mod annotating;\n\n#[path = "${em(0x1f529)}family-rules/${RS}"]\npub mod family_rules;\n`);
  }
  for (const spec of leaves) {
    if (aggregate.includes(`    ${spec.variant}(super::${snake(spec.kind)}::${spec.variant}),`)) continue;
    aggregate = aggregate.replace("    CreateAnnotationStyle(super::create_annotation_style::CreateAnnotationStyle),\n", `    CreateAnnotationStyle(super::create_annotation_style::CreateAnnotationStyle),\n    ${spec.variant}(super::${snake(spec.kind)}::${spec.variant}),\n`);
    aggregate = aggregate.replace('    "create-annotation-style",\n', `    "create-annotation-style",\n    "${spec.kind}",\n`);
    registered += 1;
  }
  return aggregate;
});
console.log(`w2-f2-families: ${leaves.length} leaves emitted, ${mounted} newly mounted, ${registered} newly registered`);
//#endregion 🔖️Run
