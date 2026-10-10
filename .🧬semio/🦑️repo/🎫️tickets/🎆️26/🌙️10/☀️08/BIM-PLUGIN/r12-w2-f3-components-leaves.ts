#!/usr/bin/env bun
/**
 * 🪑 Wave W2 `w2-f3-leaves` (binary tags 10000..10007): the eight component and MEP leaves of `s.bim.model@1`: create/set/delete of a placed component, set/remove of one of its per-instance parameter
 * overrides (an override is addressed by component and name, the key `component.name`) and create/set/delete of a routed MEP element. `bun r12-w2-f3-components-leaves.ts` rewrites the boilerplate and
 * the fixtures through `emitLeaf`, fixes the optional `set-*` fields of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`, never overwritten), mounts the leaves
 * once in the artifact root and registers the variants and the shared rules in the mutation aggregate. `after`/`diff` fixtures of applied cases are never overwritten: bless them with
 * `BIM_BLESS=1 cargo test`.
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
const reference = (name: string, kind: string, label: Label, order: number, role: "target" | "value" = "value"): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role, label, ref: { kind }, group: role === "target" ? "target" : "placement", order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "placement", order, unit: "m", step: 0.01, precision: 3 } as Prop["ui"] });
const angle = (name: string, label: Label, order: number): Mine => ({
  name,
  rust: "f64",
  schema: { type: "number" },
  ui: { widget: "dial", role: "value", label, group: "placement", order, unit: "rad", displayUnit: "deg", displayFactor: 57.29577951308232, step: 0.017453292519943295, precision: 4 } as Prop["ui"],
});
const toggle = (name: string, label: Label, order: number): Mine => ({ name, rust: "bool", schema: { type: "boolean" }, ui: { widget: "toggle", role: "value", label, group: "placement", order } });
const point = (name: string, def: string, label: Label, order: number): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "placement", order } });
const choice = (name: string, def: string, label: Label, order: number): Mine => ({ name, rust: def, schema: record(def), ui: { widget: "select", role: "value", label, group: "value", order } });
const pathProp = (order: number): Mine => ({ name: "path", rust: "Vec<Point3>", schema: { type: "array", items: record("Point3") }, ui: { widget: "list", role: "value", label: { en: "Path", de: "Verlauf" }, group: "route", order } });
const assigned = (name: string, rust: string, inner: unknown, label: Label, order: number): Mine => ({
  name,
  rust: `Assigned<Option<${rust}>>`,
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [{ type: "null" }, inner] } } },
  ui: { widget: "record", role: "value", label, group: "placement", order },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2, 0x1f4aa, 0x1f389, 0x1f525, 0x1f680, 0x1f31f, 0x1f308, 0x1f340, 0x1f4a1, 0x1f48e];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757, 0x1f4e3, 0x1f4a2, 0x1f6af, 0x1f5ef, 0x1f6a8, 0x1f6b3, 0x1f6a9, 0x1f4a5, 0x1f480, 0x1f47f];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Leaf["cases"][number]["outcome"] };
const cases = (rows: Row[]): Leaf["cases"] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

//#region 🔖️Fixtures
const zero = { x: "0 m", y: "0 m", z: "0 m" };
const cuboid = (x: string, y: string, z: string, width: string, depth: string, height: string) => ({ Cuboid: { x, y, z, width, depth, height } });
const family = (name: string, category = "Furniture") => ({ name, category });
const parameter = (owner: string, name: string, kind: string, value: string) => ({ family: owner, name, kind, value });
const solid = (owner: string, name: string, shape: unknown) => ({ family: owner, name, shape, material: "\"m-brick\"", visible: "true", offset: zero });
const key = (owner: string, name: string) => `${owner}.${name}`;
const pair = (owner: string, name: string, kind: string, value: string) => [key(owner, name), parameter(owner, name, kind, value)] as const;
const mapOf = (...rows: (readonly [string, unknown])[]) => Object.fromEntries(rows);
const P2 = (x: number, y: number) => ({ x, y });
const P3 = (x: number, y: number, z: number) => ({ x, y, z });
const component = (owner: string, x: number, y: number, name: string, extra: Record<string, unknown> = {}) => ({ storey: "st-ground", family: owner, position: P2(x, y), elevation: 0, rotation: 0, mirrored: false, ...extra, name });
const override = (owner: string, name: string, value: string) => [key(owner, name), { component: owner, name, value }] as const;
const duct = (width: number, height: number) => ({ Duct: { width, height } });
const pipe = (diameter: number) => ({ Pipe: { diameter } });
const tray = (width: number, height: number) => ({ Tray: { width, height } });
const run = (system: string, shape: unknown, path: unknown[], name: string, storey = "st-ground") => ({ storey, system, shape, path, name });

const library = (extra: Record<string, unknown> = {}) => ({
  ...F.scene({ walls: { "w-upper": F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.storeyTop(0), "Upper") } }),
  families: { "fam-table": family("Table"), "fam-basin": family("Basin", "Plumbing"), "fam-lamp": family("Lamp", "Lighting"), "fam-hea": family("HEA 200", "Profile") },
  family_parameters: mapOf(
    pair("fam-table", "width", "Length", "1.6 m"),
    pair("fam-table", "depth", "Length", "0.8 m"),
    pair("fam-table", "double", "Length", "2 * width"),
    pair("fam-basin", "width", "Length", "0.6 m"),
    pair("fam-lamp", "size", "Length", "0.3 m"),
    pair("fam-hea", "h", "Length", "190 mm"),
  ),
  family_solids: {
    "s-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "0.7 m", "width", "depth", "40 mm")),
    "s-bowl": solid("fam-basin", "Bowl", cuboid("0 m", "0 m", "0 m", "width", "0.4 m", "0.2 m")),
    "s-body": solid("fam-lamp", "Body", cuboid("0 m", "0 m", "0 m", "size", "size", "size")),
  },
  ...extra,
});
const base = (extra: Record<string, unknown> = {}) => library(extra);
const placed = (extra: Record<string, unknown> = {}) =>
  library({
    components: {
      "cmp-table": component("fam-table", 2, 3, "Table 1"),
      "cmp-basin": component("fam-basin", 3, 0.3, "Basin 1", { host: "w-south", elevation: 0.85 }),
      "cmp-lamp": component("fam-lamp", 4, 3, "Lamp 1", { elevation: 2.4, system: "Lighting" }),
    },
    component_overrides: mapOf(override("cmp-table", "width", "2 m")),
    ...extra,
  });
const withData = () =>
  placed({
    properties: { "cmp-table": { Pset_ComponentCommon: { Reference: { Text: { value: "T-01" } } } } },
    classifications: { "cmp-table": { "cs-uniclass": "Ss_40_10" } },
    classification_systems: { "cs-uniclass": { name: "Uniclass", edition: "", entries: [{ code: "Ss_40_10", title: "Furniture" }] } },
  });
const withCircle = () =>
  placed({
    component_overrides: mapOf(override("cmp-table", "width", "2 m"), override("cmp-table", "double", "depth + 1 m")),
  });
const withTakenKey = () => placed({ walls: { ...F.scene().walls, "cmp-lamp.size": F.wall("st-ground", "wt-300", F.line([0, 0], [1, 0]), F.storeyTop(0), "Odd wall") } });
const routed = (extra: Record<string, unknown> = {}) =>
  library({
    mep_elements: {
      "mep-duct": run("Supply", duct(0.3, 0.2), [P3(1, 1, 2.5), P3(5, 1, 2.5)], "Supply duct"),
      "mep-pipe": run("DomesticWater", pipe(0.05), [P3(1, 2, 0.5), P3(1, 2, 2), P3(3, 2, 2)], "Cold water"),
    },
    ...extra,
  });
const routedWithData = () => routed({ properties: { "mep-duct": { Pset_DuctCommon: { Reference: { Text: { value: "D-01" } } } } } });
void base;
//#endregion 🔖️Fixtures

type Extra = { optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf): MineLeaf => (extras.set(spec.kind, { optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const componentLabel = { en: "Component", de: "Komponente" };
const mepLabel = { en: "MEP element", de: "TGA-Element" };
const nameLabel = { en: "Name", de: "Name" };
const target = "vec![self.id.clone()]";
const overrideTarget = "vec![format!(\"{}.{}\", self.component, self.name)]";
const systemLabel = { en: "System", de: "System" };

export const leaves: MineLeaf[] = [
  leaf({
    kind: "create-component", emoji: 0x1f6cf, variant: "CreateComponent", verb: "create", entity: "component", binaryTag: 10000, displayName: "Create Component",
    doc: "Places an instance of a family: a storey, a family, a plan position with an elevation above the storey, a rotation, a mirror flag, an optional host wall and an optional system for a terminal.",
    props: [id("component", "identity", { en: "Component id", de: "Komponenten-Id" }, 10), recordProp("component", "Component", componentLabel)],
    label: { en: 'format!("Place component \\"{}\\"", self.component.name)', de: 'format!("Komponente \\"{}\\" platzieren", self.component.name)' },
    target,
    cases: cases([
      { name: "places-a-table", before: placed(), mutation: { id: "cmp-new", component: component("fam-table", 5, 2, "Table 2") }, outcome: ok },
      { name: "mounts-a-basin", before: placed(), mutation: { id: "cmp-new", component: component("fam-basin", 6, 0.3, "Basin 2", { host: "w-south", elevation: 0.85 }) }, outcome: ok },
      { name: "places-a-terminal", before: placed(), mutation: { id: "cmp-new", component: component("fam-lamp", 1, 1, "Lamp 2", { elevation: 2.4, system: "Supply", rotation: 1.57, mirrored: true }) }, outcome: ok },
      { name: "places-on-the-first-storey", before: placed(), mutation: { id: "cmp-new", component: component("fam-table", 2, 2, "Table 3", { storey: "st-first", elevation: 0.1 }) }, outcome: ok },
      { name: "duplicate", before: placed(), mutation: { id: "cmp-table", component: component("fam-table", 5, 2, "Again") }, outcome: reject("mutation.duplicate-id", ["cmp-table"]) },
      { name: "id-taken-by-another-kind", before: placed(), mutation: { id: "w-south", component: component("fam-table", 5, 2, "Wall") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: placed(), mutation: { id: "cmp-new", component: component("fam-table", 5, 2, "Lost", { storey: "st-attic" }) }, outcome: reject("mutation.target-missing", ["component", "storey"]) },
      { name: "family-missing", before: placed(), mutation: { id: "cmp-new", component: component("fam-ghost", 5, 2, "Lost") }, outcome: reject("mutation.target-missing", ["component", "family"]) },
      { name: "profile-family", before: placed(), mutation: { id: "cmp-new", component: component("fam-hea", 5, 2, "Profile") }, outcome: reject("mutation.invariant", ["component", "family"]) },
      { name: "host-missing", before: placed(), mutation: { id: "cmp-new", component: component("fam-basin", 5, 0.3, "Basin 3", { host: "w-ghost" }) }, outcome: reject("mutation.target-missing", ["component", "host"]) },
      { name: "host-is-no-wall", before: placed(), mutation: { id: "cmp-new", component: component("fam-basin", 5, 0.3, "Basin 3", { host: "st-ground" }) }, outcome: reject("mutation.invariant", ["component", "host"]) },
      { name: "host-on-another-storey", before: placed(), mutation: { id: "cmp-new", component: component("fam-basin", 5, 0.3, "Basin 3", { host: "w-upper" }) }, outcome: reject("mutation.invariant", ["component", "host"]) },
    ]),
  }),
  leaf({
    kind: "set-component", emoji: 0x1f6c1, variant: "SetComponent", verb: "set", entity: "component", binaryTag: 10001, displayName: "Set Component",
    doc: "Sets exactly the provided fields of a component: storey, family, position, elevation, rotation, mirror flag, host wall (set or cleared), system (set or cleared) and name; absent fields stay untouched.",
    props: [
      id("component", "target", componentLabel),
      sparse(reference("storey", "storey", { en: "Storey", de: "Geschoss" }, 20), keep),
      sparse(reference("family", "family", { en: "Family", de: "Familie" }, 30), keep),
      sparse(point("position", "Point2", { en: "Position", de: "Position" }, 40), keep),
      sparse(length("elevation", { en: "Elevation (m)", de: "Höhe (m)" }, 50), keep),
      sparse(angle("rotation", { en: "Rotation", de: "Drehung" }, 60), keep),
      sparse(toggle("mirrored", { en: "Mirrored", de: "Gespiegelt" }, 70), keep),
      sparse(assigned("host", "String", { type: "string" }, { en: "Host wall", de: "Trägerwand" }, 80), keep),
      sparse(assigned("system", "MepSystem", record("MepSystem"), systemLabel, 90), keep),
      sparse(text("name", nameLabel, 100), keep),
    ],
    label: { en: 'format!("Change component \\"{}\\"", self.id)', de: 'format!("Komponente \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "moves", before: placed(), mutation: { id: "cmp-table", position: P2(2.5, 3.5) }, outcome: ok },
      { name: "raises", before: placed(), mutation: { id: "cmp-table", elevation: 0.05 }, outcome: ok },
      { name: "turns", before: placed(), mutation: { id: "cmp-table", rotation: 1.57 }, outcome: ok },
      { name: "mirrors", before: placed(), mutation: { id: "cmp-table", mirrored: true }, outcome: ok },
      { name: "renames", before: placed(), mutation: { id: "cmp-table", name: "Dining table" }, outcome: ok },
      { name: "mounts-on-a-wall", before: placed(), mutation: { id: "cmp-table", host: { value: "w-south" } }, outcome: ok },
      { name: "unmounts", before: placed(), mutation: { id: "cmp-basin", host: { value: null } }, outcome: ok },
      { name: "becomes-a-terminal", before: placed(), mutation: { id: "cmp-table", system: { value: "Supply" } }, outcome: ok },
      { name: "clears-the-system", before: placed(), mutation: { id: "cmp-lamp", system: { value: null } }, outcome: ok },
      { name: "swaps-the-family", before: placed(), mutation: { id: "cmp-basin", family: "fam-table" }, outcome: ok },
      { name: "moves-to-another-storey", before: placed(), mutation: { id: "cmp-table", storey: "st-first" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: placed(), mutation: { id: "cmp-table", name: "Table 1", elevation: 0.1 }, outcome: ok },
      { name: "unchanged", before: placed(), mutation: { id: "cmp-table", name: "Table 1", elevation: 0 }, outcome: reject("mutation.no-op", ["cmp-table"]) },
      { name: "names-no-field", before: placed(), mutation: { id: "cmp-table" }, outcome: reject("mutation.no-op", ["cmp-table"]) },
      { name: "mounted-keeps-its-storey", before: placed(), mutation: { id: "cmp-basin", storey: "st-first" }, outcome: reject("mutation.invariant", ["storey"]) },
      { name: "host-missing", before: placed(), mutation: { id: "cmp-table", host: { value: "w-ghost" } }, outcome: reject("mutation.target-missing", ["host"]) },
      { name: "host-is-no-wall", before: placed(), mutation: { id: "cmp-table", host: { value: "st-ground" } }, outcome: reject("mutation.invariant", ["host"]) },
      { name: "host-on-another-storey", before: placed(), mutation: { id: "cmp-table", host: { value: "w-upper" } }, outcome: reject("mutation.invariant", ["host"]) },
      { name: "family-missing", before: placed(), mutation: { id: "cmp-table", family: "fam-ghost" }, outcome: reject("mutation.target-missing", ["family"]) },
      { name: "profile-family", before: placed(), mutation: { id: "cmp-table", family: "fam-hea" }, outcome: reject("mutation.invariant", ["family"]) },
      { name: "override-needs-its-parameter", before: placed(), mutation: { id: "cmp-table", family: "fam-lamp" }, outcome: reject("mutation.invariant", ["family"]) },
      { name: "storey-missing", before: placed(), mutation: { id: "cmp-table", storey: "st-attic" }, outcome: reject("mutation.target-missing", ["storey"]) },
      { name: "missing", before: placed(), mutation: { id: "cmp-ghost", elevation: 1 }, outcome: reject("mutation.target-missing", ["cmp-ghost"]) },
    ]),
  }),
  leaf({
    kind: "delete-component", emoji: 0x1f6bd, variant: "DeleteComponent", verb: "delete", entity: "component", binaryTag: 10002, displayName: "Delete Component",
    doc: "Removes a component together with its parameter overrides, properties and classifications.",
    props: [id("component", "target", componentLabel)],
    label: { en: 'format!("Delete component \\"{}\\" with its overrides", self.id)', de: 'format!("Komponente \\"{}\\" mit ihren Überschreibungen löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes-with-its-overrides", before: placed(), mutation: { id: "cmp-table" }, outcome: ok },
      { name: "removes-a-plain-component", before: placed(), mutation: { id: "cmp-lamp" }, outcome: ok },
      { name: "removes-its-data", before: withData(), mutation: { id: "cmp-table" }, outcome: ok },
      { name: "missing", before: placed(), mutation: { id: "cmp-ghost" }, outcome: reject("mutation.target-missing", ["cmp-ghost"]) },
    ]),
  }),
  leaf({
    kind: "set-component-override", emoji: 0x1f6bf, variant: "SetComponentOverride", verb: "set", entity: "component-override", binaryTag: 10003, displayName: "Set Component Override",
    doc: "Overrides the formula of the family parameter `name` for one component, or replaces its override. The formula must parse, use only parameters of the family of the component and never close a circle under the overrides of the component.",
    props: [reference("component", "component", componentLabel, 10, "target"), text("name", { en: "Parameter name", de: "Parametername" }, 20, "identity"), text("value", { en: "Formula", de: "Formel" }, 30)],
    label: { en: 'format!("Override parameter \\"{}\\" of component \\"{}\\"", self.name, self.component)', de: 'format!("Parameter \\"{}\\" der Komponente \\"{}\\" überschreiben", self.name, self.component)' },
    target: overrideTarget,
    cases: cases([
      { name: "adds-an-override", before: placed(), mutation: { component: "cmp-table", name: "depth", value: "0.9 m" }, outcome: ok },
      { name: "replaces-an-override", before: placed(), mutation: { component: "cmp-table", name: "width", value: "2.2 m" }, outcome: ok },
      { name: "uses-another-parameter", before: placed(), mutation: { component: "cmp-table", name: "width", value: "depth * 2" }, outcome: ok },
      { name: "breaks-a-family-circle-by-override", before: withCircle(), mutation: { component: "cmp-table", name: "width", value: "double" }, outcome: ok },
      { name: "unchanged", before: placed(), mutation: { component: "cmp-table", name: "width", value: "2 m" }, outcome: reject("mutation.no-op", ["cmp-table.width"]) },
      { name: "component-missing", before: placed(), mutation: { component: "cmp-ghost", name: "width", value: "2 m" }, outcome: reject("mutation.target-missing", ["component"]) },
      { name: "parameter-missing", before: placed(), mutation: { component: "cmp-table", name: "height", value: "2 m" }, outcome: reject("mutation.target-missing", ["name"]) },
      { name: "formula-does-not-parse", before: placed(), mutation: { component: "cmp-table", name: "width", value: "2 *" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "unknown-parameter", before: placed(), mutation: { component: "cmp-table", name: "width", value: "ghost + 1 m" }, outcome: reject("mutation.target-missing", ["value"]) },
      { name: "closes-a-circle", before: placed(), mutation: { component: "cmp-table", name: "width", value: "double" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "closes-a-circle-by-override", before: withCircle(), mutation: { component: "cmp-table", name: "depth", value: "double" }, outcome: reject("mutation.invariant", ["value"]) },
      { name: "id-taken-by-another-kind", before: withTakenKey(), mutation: { component: "cmp-lamp", name: "size", value: "0.4 m" }, outcome: reject("mutation.duplicate-id", ["cmp-lamp.size"]) },
    ]),
  }),
  leaf({
    kind: "remove-component-override", emoji: 0x1f6b0, variant: "RemoveComponentOverride", verb: "remove", entity: "component-override", binaryTag: 10004, displayName: "Remove Component Override",
    doc: "Removes the override of the family parameter `name` of one component; the component evaluates the formula of its family again.",
    props: [reference("component", "component", componentLabel, 10, "target"), text("name", { en: "Parameter name", de: "Parametername" }, 20, "identity")],
    label: { en: 'format!("Reset parameter \\"{}\\" of component \\"{}\\"", self.name, self.component)', de: 'format!("Parameter \\"{}\\" der Komponente \\"{}\\" zurücksetzen", self.name, self.component)' },
    target: overrideTarget,
    cases: cases([
      { name: "removes", before: placed(), mutation: { component: "cmp-table", name: "width" }, outcome: ok },
      { name: "removes-one-of-two", before: withCircle(), mutation: { component: "cmp-table", name: "double" }, outcome: ok },
      { name: "no-such-override", before: placed(), mutation: { component: "cmp-table", name: "depth" }, outcome: reject("mutation.target-missing", ["cmp-table.depth"]) },
      { name: "component-missing", before: placed(), mutation: { component: "cmp-ghost", name: "width" }, outcome: reject("mutation.target-missing", ["component"]) },
    ]),
  }),
  leaf({
    kind: "create-mep-element", emoji: 0x1f4a7, variant: "CreateMepElement", verb: "create", entity: "mep-element", binaryTag: 10005, displayName: "Create MEP Element",
    doc: "Routes a duct, pipe or cable tray: a storey, a system, a cross-section in metres and a path of at least two distinct points whose height is measured above the storey.",
    props: [id("mep-element", "identity", { en: "MEP element id", de: "TGA-Element-Id" }, 10), recordProp("mep", "MepElement", mepLabel)],
    label: { en: 'format!("Route MEP element \\"{}\\"", self.mep.name)', de: 'format!("TGA-Element \\"{}\\" verlegen", self.mep.name)' },
    target,
    cases: cases([
      { name: "routes-a-duct", before: routed(), mutation: { id: "mep-new", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Return duct") }, outcome: ok },
      { name: "routes-a-pipe-with-a-riser", before: routed(), mutation: { id: "mep-new", mep: run("Waste", pipe(0.1), [P3(2, 2, 0), P3(2, 2, 2.8), P3(4, 2, 2.8)], "Waste stack") }, outcome: ok },
      { name: "routes-a-cable-tray", before: routed(), mutation: { id: "mep-new", mep: run("Power", tray(0.3, 0.06), [P3(0, 0.5, 2.7), P3(7, 0.5, 2.7)], "Power tray", "st-first") }, outcome: ok },
      { name: "duplicate", before: routed(), mutation: { id: "mep-duct", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Again") }, outcome: reject("mutation.duplicate-id", ["mep-duct"]) },
      { name: "id-taken-by-another-kind", before: routed(), mutation: { id: "w-south", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Wall") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "storey-missing", before: routed(), mutation: { id: "mep-new", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Lost", "st-attic") }, outcome: reject("mutation.target-missing", ["mep", "storey"]) },
      { name: "duct-without-width", before: routed(), mutation: { id: "mep-new", mep: run("Return", duct(0, 0.2), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Flat") }, outcome: reject("mutation.invariant", ["mep", "shape"]) },
      { name: "pipe-without-diameter", before: routed(), mutation: { id: "mep-new", mep: run("Gas", pipe(-0.02), [P3(0, 4, 2.6), P3(6, 4, 2.6)], "Thin") }, outcome: reject("mutation.invariant", ["mep", "shape"]) },
      { name: "path-too-short", before: routed(), mutation: { id: "mep-new", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6)], "Stub") }, outcome: reject("mutation.invariant", ["mep", "path"]) },
      { name: "repeated-point", before: routed(), mutation: { id: "mep-new", mep: run("Return", duct(0.4, 0.2), [P3(0, 4, 2.6), P3(3, 4, 2.6), P3(3, 4, 2.6), P3(6, 4, 2.6)], "Kink") }, outcome: reject("mutation.invariant", ["mep", "path"]) },
    ]),
  }),
  leaf({
    kind: "set-mep-element", emoji: 0x1f4a8, variant: "SetMepElement", verb: "set", entity: "mep-element", binaryTag: 10006, displayName: "Set MEP Element",
    doc: "Sets exactly the provided fields of an MEP element: storey, system, cross-section, path and name; absent fields stay untouched. A provided path replaces the path as one field.",
    props: [
      id("mep-element", "target", mepLabel),
      sparse(reference("storey", "storey", { en: "Storey", de: "Geschoss" }, 20), keep),
      sparse(choice("system", "MepSystem", systemLabel, 30), keep),
      sparse(point("shape", "MepShape", { en: "Section", de: "Querschnitt" }, 40), keep),
      sparse(pathProp(50), keep),
      sparse(text("name", nameLabel, 60), keep),
    ],
    label: { en: 'format!("Change MEP element \\"{}\\"", self.id)', de: 'format!("TGA-Element \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "resizes", before: routed(), mutation: { id: "mep-duct", shape: duct(0.4, 0.25) }, outcome: ok },
      { name: "changes-the-section-kind", before: routed(), mutation: { id: "mep-duct", shape: pipe(0.2) }, outcome: ok },
      { name: "reroutes", before: routed(), mutation: { id: "mep-pipe", path: [P3(1, 2, 0.5), P3(1, 2, 2.2), P3(4, 2, 2.2), P3(4, 5, 2.2)] }, outcome: ok },
      { name: "changes-the-system", before: routed(), mutation: { id: "mep-duct", system: "Exhaust" }, outcome: ok },
      { name: "renames", before: routed(), mutation: { id: "mep-duct", name: "Main supply" }, outcome: ok },
      { name: "moves-to-another-storey", before: routed(), mutation: { id: "mep-duct", storey: "st-first" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: routed(), mutation: { id: "mep-duct", system: "Supply", name: "Main supply" }, outcome: ok },
      { name: "unchanged", before: routed(), mutation: { id: "mep-duct", system: "Supply", name: "Supply duct" }, outcome: reject("mutation.no-op", ["mep-duct"]) },
      { name: "names-no-field", before: routed(), mutation: { id: "mep-duct" }, outcome: reject("mutation.no-op", ["mep-duct"]) },
      { name: "section-not-positive", before: routed(), mutation: { id: "mep-duct", shape: duct(0.3, 0) }, outcome: reject("mutation.invariant", ["shape"]) },
      { name: "path-too-short", before: routed(), mutation: { id: "mep-duct", path: [P3(0, 0, 0)] }, outcome: reject("mutation.invariant", ["path"]) },
      { name: "repeated-point", before: routed(), mutation: { id: "mep-duct", path: [P3(1, 1, 2.5), P3(1, 1, 2.5)] }, outcome: reject("mutation.invariant", ["path"]) },
      { name: "storey-missing", before: routed(), mutation: { id: "mep-duct", storey: "st-attic" }, outcome: reject("mutation.target-missing", ["storey"]) },
      { name: "missing", before: routed(), mutation: { id: "mep-ghost", name: "Ghost" }, outcome: reject("mutation.target-missing", ["mep-ghost"]) },
    ]),
  }),
  leaf({
    kind: "delete-mep-element", emoji: 0x1f4a6, variant: "DeleteMepElement", verb: "delete", entity: "mep-element", binaryTag: 10007, displayName: "Delete MEP Element",
    doc: "Removes a routed MEP element together with its properties and classifications.",
    props: [id("mep-element", "target", mepLabel)],
    label: { en: 'format!("Delete MEP element \\"{}\\"", self.id)', de: 'format!("TGA-Element \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: routed(), mutation: { id: "mep-duct" }, outcome: ok },
      { name: "removes-its-data", before: routedWithData(), mutation: { id: "mep-duct" }, outcome: ok },
      { name: "missing", before: routed(), mutation: { id: "mep-ghost" }, outcome: reject("mutation.target-missing", ["mep-ghost"]) },
    ]),
  }),
];

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...parts: unknown[]) => strings.reduce((out, chunk, index) => out + chunk + (index < parts.length ? String(parts[index]) : ""), "").replaceAll("¶", "`");

const hand: Record<string, { diff: string; inverse: string }> = {
  "create-component": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateComponent¶: one created component entry. The id is free in every collection, the storey exists, the family exists and is no profile, every number is finite and a host is a
//! wall of the same storey. The solids, the fit onto the host, the volume and the issues of the instance are inferred, never stored.

use super::super::component_rules;
use super::super::elements;
use super::CreateComponent;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = component_rules::component_fault(base, &payload.component) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("component")));
    }
    MutationOutcome::new(ModelDiff::components(payload.id.clone(), Entry::Created(payload.component.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateComponent¶: the concrete ¶DeleteComponent¶ of the id it created, none when the id was already taken.

use super::super::delete_component::DeleteComponent;
use super::CreateComponent;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.components.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteComponent(DeleteComponent { id: payload.id.clone() })]
}
`,
  },
  "set-component": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetComponent¶: a sparse component patch of exactly the provided fields that differ. The component that results must break none of the create rules; a mounted component
//! stands on the storey of its wall, so it cannot change its storey unless it changes its wall with it, and a component that changes its family keeps only overrides the new family can evaluate. Moving to
//! another storey keeps the elevation. Providing only equal values, or no field, is a no-op.

use super::super::component_rules;
use super::SetComponent;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.components.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(wall) = next.host.as_ref().filter(|wall| record.host.as_ref() == Some(*wall) && next.storey != record.storey) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Component \\"{}\\" is mounted on wall \\"{wall}\\" and stands on its storey.", payload.id), ["storey"]);
    }
    if let Some(fault) = component_rules::component_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    if next.family != record.family {
        if let Some(fault) = component_rules::family_swap_fault(base, &payload.id, &next.family) {
            return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
        }
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Component \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::components(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetComponent¶: an absolute ¶SetComponent¶ restoring the base value of exactly the fields the forward really changes, none when the component is absent or nothing changes.

use super::SetComponent;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.components.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetComponent(SetComponent::from_patch(payload.id.clone(), restore))]
}
`,
  },
  "delete-component": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteComponent¶: the component leaves in one sparse diff together with its parameter overrides and its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the component leaves.

use super::super::cascade;
use super::DeleteComponent;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.components.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Component", Some(&payload.id))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteComponent¶: the concrete ¶CreateComponent¶ of the removed component, one ¶SetComponentOverride¶ per removed override and one setter per removed property or classification, in storage
//! order (dependants first, the component last), so the store, which replays the vector reversed, recreates the component before anything that belongs to it.

use super::super::cascade;
use super::DeleteComponent;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
  "set-component-override": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetComponentOverride¶: the override of parameter ¶name¶ of a component is one record keyed ¶component.name¶. An existing override gets a one-field patch with the new formula
//! (the same formula is a no-op), an absent one is created. The component and the parameter of its family exist, the formula parses, uses only parameters of the family and closes no circle under the
//! overrides of the component; whether it also computes the kind of the parameter is the business of the inference (a diagnostic), not of the diff. The key is free in every other collection.

use super::super::component_rules;
use super::super::elements;
use super::super::family_rules;
use super::SetComponentOverride;
use crate::{ComponentOverride, ComponentOverridePatch, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetComponentOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(fault) = component_rules::override_fault(base, &payload.component, &payload.name, &payload.value) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let key = family_rules::parameter_key(&payload.component, &payload.name);
    if let Some(record) = base.component_overrides.get(&key) {
        let change = ComponentOverridePatch { value: Some(payload.value.clone()), ..Default::default() }.minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Parameter \\"{}\\" of component \\"{}\\" already has this override.", payload.name, payload.component), [key]);
        }
        return MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Patched(change)));
    }
    if let Some(noun) = elements::taken(base, &key) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{key}\\" already exists."), [key]);
    }
    MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Created(ComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: payload.value.clone() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetComponentOverride¶: for an existing override an absolute ¶SetComponentOverride¶ restoring its base formula, for a created one the concrete ¶RemoveComponentOverride¶; none when the
//! component is absent or nothing changes.

use super::super::remove_component_override::RemoveComponentOverride;
use super::super::family_rules;
use super::SetComponentOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetComponentOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.components.contains_key(&payload.component) {
        return Vec::new();
    }
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        None => vec![ModelMutation::RemoveComponentOverride(RemoveComponentOverride { component: payload.component.clone(), name: payload.name.clone() })],
        Some(record) if record.value == payload.value => Vec::new(),
        Some(record) => vec![ModelMutation::SetComponentOverride(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
    }
}
`,
  },
  "remove-component-override": {
    diff: rs`//! ${DIFF} Diff constructor for ¶RemoveComponentOverride¶: the override record leaves and the component evaluates the formula of its family again. Refused when the component or the override is absent.

use super::super::family_rules;
use super::RemoveComponentOverride;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.components.contains_key(&payload.component) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \\"{}\\" does not exist.", payload.component), ["component"]);
    }
    let key = family_rules::parameter_key(&payload.component, &payload.name);
    if !base.component_overrides.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \\"{}\\" does not override parameter \\"{}\\".", payload.component, payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Deleted))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶RemoveComponentOverride¶: the concrete ¶SetComponentOverride¶ carrying the formula of the removed override, none when it was absent.

use super::super::family_rules;
use super::super::set_component_override::SetComponentOverride;
use super::RemoveComponentOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        Some(record) => vec![ModelMutation::SetComponentOverride(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
        None => Vec::new(),
    }
}
`,
  },
  "create-mep-element": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateMepElement¶: one created MEP element entry. The id is free in every collection, the storey exists, every dimension of the section is a positive length and the path has at
//! least two finite points of which no two consecutive ones coincide. The solid, the length, the volume and the clashes are inferred, never stored.

use super::super::component_rules;
use super::super::elements;
use super::CreateMepElement;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = component_rules::mep_fault(base, &payload.mep) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("mep")));
    }
    MutationOutcome::new(ModelDiff::mep_elements(payload.id.clone(), Entry::Created(payload.mep.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateMepElement¶: the concrete ¶DeleteMepElement¶ of the id it created, none when the id was already taken.

use super::super::delete_mep_element::DeleteMepElement;
use super::CreateMepElement;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateMepElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.mep_elements.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteMepElement(DeleteMepElement { id: payload.id.clone() })]
}
`,
  },
  "set-mep-element": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetMepElement¶: a sparse MEP element patch of exactly the provided fields that differ. The element that results must break none of the create rules. Whole-list ruling: the path
//! is ONE centre line, so a provided path replaces the path as one field. Providing only equal values, or no field, is a no-op.

use super::super::component_rules;
use super::SetMepElement;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.mep_elements.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("MEP element \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = component_rules::mep_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("MEP element \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::mep_elements(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetMepElement¶: an absolute ¶SetMepElement¶ restoring the base value of exactly the fields the forward really changes, none when the element is absent or nothing changes.

use super::SetMepElement;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetMepElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.mep_elements.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetMepElement(SetMepElement::from_patch(payload.id.clone(), restore))]
}
`,
  },
  "delete-mep-element": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteMepElement¶: the MEP element leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteMepElement;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.mep_elements.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("MEP element \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "MEP element", Some(&payload.id))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteMepElement¶: the concrete ¶CreateMepElement¶ of the removed element and one setter per removed property or classification, in storage order (the element last), so the store,
//! which replays the vector reversed, recreates the element before its data.

use super::super::cascade;
use super::DeleteMepElement;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteMepElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
};

type Patchable = { type: string; build: string; from: string; args: string };
const PATCHES: Record<string, Patchable> = {
  "set-component": {
    type: "ComponentPatch",
    args: "id: String",
    build: "ComponentPatch { storey: self.storey.clone(), family: self.family.clone(), position: self.position, elevation: self.elevation, rotation: self.rotation, mirrored: self.mirrored, host: self.host.clone(), system: self.system.clone(), name: self.name.clone() }",
    from: "Self { id, storey: patch.storey, family: patch.family, position: patch.position, elevation: patch.elevation, rotation: patch.rotation, mirrored: patch.mirrored, host: patch.host, system: patch.system, name: patch.name }",
  },
  "set-mep-element": {
    type: "MepElementPatch",
    args: "id: String",
    build: "MepElementPatch { storey: self.storey.clone(), system: self.system, shape: self.shape.clone(), path: self.path.clone(), name: self.name.clone() }",
    from: "Self { id, storey: patch.storey, system: patch.system, shape: patch.shape, path: patch.path, name: patch.name }",
  },
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
  if (!aggregate.includes("pub mod component_rules;")) {
    aggregate = aggregate.replace("pub mod family_rules;\n", `pub mod family_rules;\n\n#[path = "${em(0x1f50c)}component-rules/${RS}"]\npub mod component_rules;\n`);
  }
  for (const spec of leaves) {
    if (aggregate.includes(`    ${spec.variant}(super::${snake(spec.kind)}::${spec.variant}),`)) continue;
    aggregate = aggregate.replace("    CreateAnnotationStyle(super::create_annotation_style::CreateAnnotationStyle),\n", `    CreateAnnotationStyle(super::create_annotation_style::CreateAnnotationStyle),\n    ${spec.variant}(super::${snake(spec.kind)}::${spec.variant}),\n`);
    aggregate = aggregate.replace('    "create-annotation-style",\n', `    "create-annotation-style",\n    "${spec.kind}",\n`);
    registered += 1;
  }
  return aggregate;
});
console.log(`w2-f3-leaves: ${leaves.length} leaves emitted, ${mounted} newly mounted, ${registered} newly registered`);
//#endregion 🔖️Run
