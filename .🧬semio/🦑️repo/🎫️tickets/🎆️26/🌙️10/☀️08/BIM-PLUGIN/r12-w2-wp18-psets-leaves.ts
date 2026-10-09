#!/usr/bin/env bun
/**
 * 🏷️ WP-18 (`w2-wp18-psets`): the six leaves create/set/delete-property-template and create/set/delete-classification-system of `s.bim.model@1` (binary tags 18000..18005), the reshaped leaves
 * `set-element-classification` / `remove-element-classification` (one classification per (element, system)) and the new cases of the property and type leaves. `bun r12-w2-wp18-psets-leaves.ts`
 * rewrites the boilerplate (descriptor, payload schema, payload file unless it already carries `from_patch`, test files) and the `before`/`mutation`/`outcome`/`after`/`diff` fixtures of every case
 * (the expectation of an applied case is computed here by `apply`, an independent, tiny re-implementation of the diff algebra, then checked by `kit::applies`), never the hand-written
 * `🔺️diff` and `↩️inverse` files. It prints the mount text into `🗑️generated/w2-wp18-psets/mounts.txt` and `case-mounts.json`; paste them with `r12-w2-wp18-psets-mount.py`.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
const overwrite = (path: string, text: string) => {
  rmSync(path, { force: true });
  writeFileSync(path, text);
};
import { join } from "node:path";
import { emitCase, emitLeaf, type Case, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, fixtures, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: Label, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: Label, order = 20): Prop => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const text = (name: string, label: Label, order: number, group = "identity"): Prop => ({ name, rust: "Option<String>", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group, order } });
const required = (name: string, label: Label, order: number, group = "value"): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group, order } });
const reference = (name: string, kind: string, label: Label, order: number): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group: "value", order } });
const list = (name: string, rust: string, items: unknown, label: Label, order: number): Prop => ({ name, rust: `Option<Vec<${rust}>>`, schema: { type: "array", items }, ui: { widget: "list", role: "value", label, group: "value", order } });
const assigned = (name: string, label: Label, order: number): Prop => ({
  name,
  rust: "Option<Assigned<Option<String>>>",
  schema: { type: "object", additionalProperties: false, required: ["value"], properties: { value: { type: ["string", "null"] } } },
  ui: { widget: "text", role: "value", label, group: "value", order },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const POOL = [0x2705, 0x1f6ab, 0x26d4, 0x1f6d1, 0x1f9ed, 0x1f4a4, 0x1f4dd, 0x1f3af, 0x1f4c9, 0x1f5c2, 0x1f4cc, 0x1f9f1, 0x1f3d8, 0x1f6b6, 0x1f6ae, 0x1f3a8, 0x1f9fc, 0x1f50d, 0x1f4e6, 0x1f517, 0x1f9ee, 0x1f512, 0x1f4a1, 0x1f3f7, 0x1f9e9, 0x1f4ca, 0x1f5d1, 0x1f4d0, 0x1f9ea, 0x1f3d7];
type PCase = { name: string; emoji?: number; before: any; mutation: Record<string, unknown>; outcome: Case["outcome"]; diff?: Record<string, any> };
const cases = (rows: PCase[], pool: number[] = POOL): (Case & { diff?: Record<string, any>; after?: any })[] =>
  rows.map((row, index) => ({ name: row.name, emoji: row.emoji ?? pool[index], before: row.before, mutation: row.mutation, outcome: row.outcome, diff: row.diff, after: row.outcome.status === "applied" ? apply(row.before, row.diff ?? {}) : row.before }));

//#region 🔖️Algebra
const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value));
const SKIPPED_WHEN_EMPTY = ["property_templates", "classification_systems"];
const patchRecord = (collection: string, record: any, rest: any) => {
  if (collection === "classifications") {
    const next = { ...record };
    for (const [system, code] of Object.entries<any>(rest.assigned)) {
      if (code === null) delete next[system];
      else next[system] = code;
    }
    return next;
  }
  if (collection === "properties") {
    const next = clone(record);
    for (const [set, properties] of Object.entries<any>(rest.assigned))
      for (const [name, value] of Object.entries<any>(properties)) {
        if (value === null) {
          delete next[set]?.[name];
          if (next[set] && Object.keys(next[set]).length === 0) delete next[set];
        } else (next[set] ??= {})[name] = value;
      }
    return next;
  }
  const next = { ...record };
  for (const [field, value] of Object.entries<any>(rest)) {
    if (collection === "classification_systems" && field === "source") {
      if (value.value === null) delete next.source;
      else next.source = value.value;
    } else next[field] = value;
  }
  return next;
};
function apply(before: any, diff: Record<string, Record<string, any>>) {
  const next = clone(before);
  for (const [collection, entries] of Object.entries(diff)) {
    next[collection] ??= {};
    for (const [key, entry] of Object.entries<any>(entries)) {
      const { entry: kind, ...rest } = entry;
      if (kind === "Created" || kind === "Replaced") next[collection][key] = rest;
      else if (kind === "Deleted") delete next[collection][key];
      else next[collection][key] = patchRecord(collection, next[collection][key], rest);
    }
  }
  for (const collection of SKIPPED_WHEN_EMPTY) if (next[collection] && Object.keys(next[collection]).length === 0) delete next[collection];
  return next;
}
//#endregion 🔖️Algebra

//#region 🔖️Data
const txt = (value: string) => ({ Text: { value } });
const real = (value: number) => ({ Real: { value } });
const int = (value: number) => ({ Integer: { value } });
const bool = (value: boolean) => ({ Boolean: { value } });
const def = (name: string, kind: string, over: Record<string, unknown> = {}) => ({ name, kind, required: false, allowed: [] as unknown[], ...over });
const template = (name: string, applies_to: string[], properties: unknown[]) => ({ name, applies_to, properties });
const item = (code: string, title: string, parent?: string) => ({ code, title, ...(parent ? { parent } : {}) });
const system = (name: string, edition: string, entries: unknown[], source?: string) => ({ name, edition, ...(source ? { source } : {}), entries });

const FIRE = def("FireRating", "Text", { allowed: [txt("EI30"), txt("EI60"), txt("EI90")] });
const U_VALUE = def("ThermalTransmittance", "Real", { unit: "W/(m2.K)", minimum: 0, maximum: 5, required: true, default_value: real(0.35) });
const EXTERNAL = def("IsExternal", "Boolean", { default_value: bool(false) });
const WALL_COMMON = template("Pset_WallCommon", ["Wall", "WallType"], [FIRE, U_VALUE, EXTERNAL]);
const DOOR_COMMON = template("Pset_DoorCommon", ["Door", "DoorType"], [def("Reference", "Text"), def("AcousticRating", "Text")]);
const UNICLASS = system("Uniclass 2015", "2024-03", [item("EF", "Elements and functions"), item("EF_25", "Walls and barriers", "EF"), item("EF_25_10", "Walls", "EF_25"), item("Pr", "Products"), item("Pr_20", "Structure and general products", "Pr"), item("Pr_20_93", "Structural units and assemblies", "Pr_20")], "https://www.thenbs.com/our-tools/uniclass");
const DIN276 = system("DIN 276", "2018-12", [item("300", "Bauwerk - Baukonstruktionen"), item("330", "Aussenwaende", "300"), item("331", "Tragende Aussenwaende", "330")]);

const base = (parts: { property_templates?: Record<string, unknown>; classification_systems?: Record<string, unknown>; properties?: Record<string, unknown>; classifications?: Record<string, unknown>; extra?: Record<string, unknown> } = {}) => {
  const snapshot: any = F.scene();
  if (parts.property_templates) snapshot.property_templates = parts.property_templates;
  if (parts.classification_systems) snapshot.classification_systems = parts.classification_systems;
  if (parts.properties) snapshot.properties = parts.properties;
  if (parts.classifications) snapshot.classifications = parts.classifications;
  return Object.assign(snapshot, parts.extra ?? {});
};
const TEMPLATES = { "pt-wall": WALL_COMMON, "pt-door": DOOR_COMMON };
const SYSTEMS = { "cs-uni": UNICLASS, "cs-din": DIN276 };
const WITH_TEMPLATES = () => base({ property_templates: TEMPLATES });
const WITH_SYSTEMS = () => base({ classification_systems: SYSTEMS });
const CLASSIFIED = () => base({ classification_systems: SYSTEMS, classifications: { "w-south": { "cs-uni": "EF_25_10" }, "w-east": { "cs-uni": "EF_25_10", "cs-din": "331" }, "wt-300": { "cs-din": "331" } } });
//#endregion 🔖️Data

const created = (collection: string, key: string, record: unknown) => ({ [collection]: { [key]: { entry: "Created", ...(record as object) } } });
const patched = (collection: string, key: string, patch: unknown) => ({ [collection]: { [key]: { entry: "Patched", ...(patch as object) } } });
const deleted = (collection: string, key: string) => ({ [collection]: { [key]: { entry: "Deleted" } } });

const P_NAME = "Pset_WallCommon";
export const leaves: (Leaf & { cases: ReturnType<typeof cases> })[] = [
  {
    kind: "create-property-template", emoji: 0x1f9f0, variant: "CreatePropertyTemplate", verb: "create", entity: "property-template", displayName: "Create Property Template", binaryTag: 18000,
    doc: "Brings a new property set template into the model library: the property set name, the element and type kinds it applies to and the definition of each of its properties. It changes no element; the effective properties are inferred.",
    props: [id("property-template", "identity", { en: "Template id", de: "Vorlagen-Id" }, 10), recordProp("template", "PropertyTemplate", { en: "Template", de: "Vorlage" })],
    label: { en: 'format!("Create property template \\"{}\\"", self.template.name)', de: 'format!("Eigenschaftsvorlage \\"{}\\" anlegen", self.template.name)' },
    target,
    cases: cases([
      { name: "adds", before: WITH_TEMPLATES(), mutation: { id: "pt-slab", template: template("Pset_SlabCommon", ["Slab", "SlabType"], [def("LoadBearing", "Boolean", { required: true }), def("Thickness", "Length", { unit: "m", minimum: 0.05, default_value: { Length: { value: 0.2 } } })]) }, outcome: ok, diff: created("property_templates", "pt-slab", template("Pset_SlabCommon", ["Slab", "SlabType"], [def("LoadBearing", "Boolean", { required: true }), def("Thickness", "Length", { unit: "m", minimum: 0.05, default_value: { Length: { value: 0.2 } } })])) },
      { name: "adds-the-first-template", before: base(), mutation: { id: "pt-wall", template: WALL_COMMON }, outcome: ok, diff: created("property_templates", "pt-wall", WALL_COMMON) },
      { name: "duplicate", before: WITH_TEMPLATES(), mutation: { id: "pt-wall", template: template("Other", ["Wall"], []) }, outcome: reject("mutation.duplicate-id", ["pt-wall"]) },
      { name: "id-taken-by-another-kind", before: WITH_TEMPLATES(), mutation: { id: "w-south", template: template("Other", ["Wall"], []) }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("  ", ["Wall"], []) }, outcome: reject("mutation.invariant", ["template", "name"]) },
      { name: "name-already-defined", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template(P_NAME, ["Slab"], []) }, outcome: reject("mutation.invariant", ["template", "name"]) },
      { name: "kind-listed-twice", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("Pset_X", ["Wall", "Wall"], []) }, outcome: reject("mutation.invariant", ["template", "applies_to"]) },
      { name: "property-defined-twice", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("Pset_X", ["Wall"], [def("A", "Text"), def("A", "Real")]) }, outcome: reject("mutation.invariant", ["template", "properties"]) },
      { name: "default-breaks-the-range", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("Pset_X", ["Wall"], [def("Mass", "Real", { maximum: 5, default_value: real(9) })]) }, outcome: reject("mutation.invariant", ["template", "properties"]) },
      { name: "default-of-another-kind", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("Pset_X", ["Wall"], [def("Mass", "Real", { default_value: txt("heavy") })]) }, outcome: reject("mutation.invariant", ["template", "properties"]) },
      { name: "range-on-text", before: WITH_TEMPLATES(), mutation: { id: "pt-new", template: template("Pset_X", ["Wall"], [def("Label", "Text", { minimum: 1 })]) }, outcome: reject("mutation.invariant", ["template", "properties"]) },
    ]),
  },
  {
    kind: "set-property-template", emoji: 0x1f6e0, variant: "SetPropertyTemplate", verb: "set", entity: "property-template", displayName: "Set Property Template", binaryTag: 18001,
    doc: "Sets any of a property set template's name, applicable kinds and property definitions; absent fields stay untouched and a list replaces the whole list.",
    props: [
      id("property-template", "target", { en: "Template", de: "Vorlage" }),
      text("name", { en: "Name", de: "Name" }, 20),
      list("applies_to", "TemplateTarget", record("TemplateTarget"), { en: "Applies to", de: "Gilt für" }, 30),
      list("properties", "PropertyDef", record("PropertyDef"), { en: "Properties", de: "Eigenschaften" }, 40),
    ],
    label: { en: 'format!("Edit property template \\"{}\\"", self.id)', de: 'format!("Eigenschaftsvorlage \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames-and-retargets", before: WITH_TEMPLATES(), mutation: { id: "pt-wall", name: "Pset_WallStandard", applies_to: ["Wall", "CurtainWall"] }, outcome: ok, diff: patched("property_templates", "pt-wall", { name: "Pset_WallStandard", applies_to: ["Wall", "CurtainWall"] }) },
      { name: "replaces-the-definitions", before: WITH_TEMPLATES(), mutation: { id: "pt-door", properties: [def("Reference", "Text", { required: true }), def("Width", "Length", { minimum: 0.7, maximum: 2.5 })] }, outcome: ok, diff: patched("property_templates", "pt-door", { properties: [def("Reference", "Text", { required: true }), def("Width", "Length", { minimum: 0.7, maximum: 2.5 })] }) },
      { name: "restates-an-unchanged-field", before: WITH_TEMPLATES(), mutation: { id: "pt-door", name: "Pset_DoorCommon", applies_to: ["Door"] }, outcome: ok, diff: patched("property_templates", "pt-door", { applies_to: ["Door"] }) },
      { name: "nothing-to-change", before: WITH_TEMPLATES(), mutation: { id: "pt-door", name: "Pset_DoorCommon", applies_to: ["Door", "DoorType"] }, outcome: reject("mutation.no-op", ["pt-door"]) },
      { name: "missing", before: WITH_TEMPLATES(), mutation: { id: "pt-roof", name: "Pset_RoofCommon" }, outcome: reject("mutation.target-missing", ["pt-roof"]) },
      { name: "blank-name", before: WITH_TEMPLATES(), mutation: { id: "pt-door", name: "" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "name-already-defined", before: WITH_TEMPLATES(), mutation: { id: "pt-door", name: P_NAME }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "kind-listed-twice", before: WITH_TEMPLATES(), mutation: { id: "pt-door", applies_to: ["Door", "Door"] }, outcome: reject("mutation.invariant", ["applies_to"]) },
      { name: "broken-definition", before: WITH_TEMPLATES(), mutation: { id: "pt-door", properties: [def("Width", "Length", { minimum: 3, maximum: 1 })] }, outcome: reject("mutation.invariant", ["properties"]) },
    ]),
  },
  {
    kind: "delete-property-template", emoji: 0x1f5dc, variant: "DeletePropertyTemplate", verb: "delete", entity: "property-template", displayName: "Delete Property Template", binaryTag: 18002,
    doc: "Removes a property set template from the library. Templates own no entries: the property sets of elements stay as they are, they merely lose their definition.",
    props: [id("property-template", "target", { en: "Template", de: "Vorlage" })],
    label: { en: 'format!("Delete property template \\"{}\\"", self.id)', de: 'format!("Eigenschaftsvorlage \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: WITH_TEMPLATES(), mutation: { id: "pt-door" }, outcome: ok, diff: deleted("property_templates", "pt-door") },
      { name: "removes-the-last-template", before: base({ property_templates: { "pt-wall": WALL_COMMON } }), mutation: { id: "pt-wall" }, outcome: ok, diff: deleted("property_templates", "pt-wall") },
      { name: "missing", before: WITH_TEMPLATES(), mutation: { id: "pt-roof" }, outcome: reject("mutation.target-missing", ["pt-roof"]) },
    ]),
  },
  {
    kind: "create-classification-system", emoji: 0x1f4da, variant: "CreateClassificationSystem", verb: "create", entity: "classification-system", displayName: "Create Classification System", binaryTag: 18003,
    doc: "Brings a new classification system into the model library: its name, edition and the authored table of its entries (code, title, parent). Elements are classified through `set-element-classification`.",
    props: [id("classification-system", "identity", { en: "System id", de: "System-Id" }, 10), recordProp("system", "ClassificationSystem", { en: "Classification system", de: "Klassifikationssystem" })],
    label: { en: 'format!("Create classification system \\"{}\\"", self.system.name)', de: 'format!("Klassifikationssystem \\"{}\\" anlegen", self.system.name)' },
    target,
    cases: cases([
      { name: "adds", before: WITH_SYSTEMS(), mutation: { id: "cs-omni", system: system("OmniClass Table 23", "2012", [item("23-13 00 00", "Structural Products"), item("23-13 11 00", "Structural Walls", "23-13 00 00")]) }, outcome: ok, diff: created("classification_systems", "cs-omni", system("OmniClass Table 23", "2012", [item("23-13 00 00", "Structural Products"), item("23-13 11 00", "Structural Walls", "23-13 00 00")])) },
      { name: "adds-the-first-system", before: base(), mutation: { id: "cs-uni", system: UNICLASS }, outcome: ok, diff: created("classification_systems", "cs-uni", UNICLASS) },
      { name: "duplicate", before: WITH_SYSTEMS(), mutation: { id: "cs-uni", system: DIN276 }, outcome: reject("mutation.duplicate-id", ["cs-uni"]) },
      { name: "id-taken-by-another-kind", before: WITH_SYSTEMS(), mutation: { id: "wt-300", system: DIN276 }, outcome: reject("mutation.duplicate-id", ["wt-300"]) },
      { name: "reserved-id", before: WITH_SYSTEMS(), mutation: { id: "entry", system: DIN276 }, outcome: reject("mutation.invariant", ["id"]) },
      { name: "blank-name", before: WITH_SYSTEMS(), mutation: { id: "cs-new", system: system(" ", "1", []) }, outcome: reject("mutation.invariant", ["system", "name"]) },
      { name: "blank-code", before: WITH_SYSTEMS(), mutation: { id: "cs-new", system: system("X", "1", [item("", "Nothing")]) }, outcome: reject("mutation.invariant", ["system", "entries"]) },
      { name: "code-used-twice", before: WITH_SYSTEMS(), mutation: { id: "cs-new", system: system("X", "1", [item("A", "One"), item("A", "Two")]) }, outcome: reject("mutation.invariant", ["system", "entries"]) },
      { name: "parent-missing", before: WITH_SYSTEMS(), mutation: { id: "cs-new", system: system("X", "1", [item("A", "One", "Z")]) }, outcome: reject("mutation.invariant", ["system", "entries"]) },
      { name: "parents-form-a-cycle", before: WITH_SYSTEMS(), mutation: { id: "cs-new", system: system("X", "1", [item("A", "One", "B"), item("B", "Two", "A")]) }, outcome: reject("mutation.invariant", ["system", "entries"]) },
    ]),
  },
  {
    kind: "set-classification-system", emoji: 0x1f4d6, variant: "SetClassificationSystem", verb: "set", entity: "classification-system", displayName: "Set Classification System", binaryTag: 18004,
    doc: "Sets any of a classification system's name, edition, source and entry table (an assigned null removes the source); absent fields stay untouched and the entry table replaces the whole table.",
    props: [
      id("classification-system", "target", { en: "System", de: "System" }),
      text("name", { en: "Name", de: "Name" }, 20),
      text("edition", { en: "Edition", de: "Ausgabe" }, 30, "value"),
      assigned("source", { en: "Source (empty = none)", de: "Quelle (leer = keine)" }, 40),
      list("entries", "ClassificationItem", record("ClassificationItem"), { en: "Entries", de: "Einträge" }, 50),
    ],
    label: { en: 'format!("Edit classification system \\"{}\\"", self.id)', de: 'format!("Klassifikationssystem \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames-and-reeditions", before: WITH_SYSTEMS(), mutation: { id: "cs-din", name: "DIN 276-1", edition: "2018-12 (rev.)" }, outcome: ok, diff: patched("classification_systems", "cs-din", { name: "DIN 276-1", edition: "2018-12 (rev.)" }) },
      { name: "replaces-the-entries", before: WITH_SYSTEMS(), mutation: { id: "cs-din", entries: [item("300", "Bauwerk - Baukonstruktionen"), item("340", "Innenwaende", "300")] }, outcome: ok, diff: patched("classification_systems", "cs-din", { entries: [item("300", "Bauwerk - Baukonstruktionen"), item("340", "Innenwaende", "300")] }) },
      { name: "sets-the-source", before: WITH_SYSTEMS(), mutation: { id: "cs-din", source: { value: "https://www.din.de" } }, outcome: ok, diff: patched("classification_systems", "cs-din", { source: { value: "https://www.din.de" } }) },
      { name: "clears-the-source", before: WITH_SYSTEMS(), mutation: { id: "cs-uni", source: { value: null } }, outcome: ok, diff: patched("classification_systems", "cs-uni", { source: { value: null } }) },
      { name: "restates-an-unchanged-field", before: WITH_SYSTEMS(), mutation: { id: "cs-din", name: "DIN 276", edition: "2020" }, outcome: ok, diff: patched("classification_systems", "cs-din", { edition: "2020" }) },
      { name: "nothing-to-change", before: WITH_SYSTEMS(), mutation: { id: "cs-din", name: "DIN 276", edition: "2018-12" }, outcome: reject("mutation.no-op", ["cs-din"]) },
      { name: "missing", before: WITH_SYSTEMS(), mutation: { id: "cs-omni", name: "OmniClass" }, outcome: reject("mutation.target-missing", ["cs-omni"]) },
      { name: "blank-name", before: WITH_SYSTEMS(), mutation: { id: "cs-din", name: "" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "parent-missing", before: WITH_SYSTEMS(), mutation: { id: "cs-din", entries: [item("330", "Aussenwaende", "300")] }, outcome: reject("mutation.invariant", ["entries"]) },
    ]),
  },
  {
    kind: "delete-classification-system", emoji: 0x1f4d5, variant: "DeleteClassificationSystem", verb: "delete", entity: "classification-system", displayName: "Delete Classification System", binaryTag: 18005,
    doc: "Removes a classification system with every element and type classification that names it; the inverse restores the system and each classification.",
    props: [id("classification-system", "target", { en: "System", de: "System" })],
    inverseRows: { bounded: 8191 },
    label: { en: 'format!("Delete classification system \\"{}\\"", self.id)', de: 'format!("Klassifikationssystem \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: WITH_SYSTEMS(), mutation: { id: "cs-din" }, outcome: ok, diff: deleted("classification_systems", "cs-din") },
      {
        name: "removes-its-classifications",
        before: CLASSIFIED(),
        mutation: { id: "cs-din" },
        outcome: ok,
        diff: { ...deleted("classification_systems", "cs-din"), classifications: { "w-east": { entry: "Patched", assigned: { "cs-din": null } }, "wt-300": { entry: "Deleted" } } },
      },
      { name: "missing", before: WITH_SYSTEMS(), mutation: { id: "cs-omni" }, outcome: reject("mutation.target-missing", ["cs-omni"]) },
    ]),
  },
  {
    kind: "set-element-classification", emoji: 0x1f5c2, variant: "SetElementClassification", verb: "set", entity: "element-classification", displayName: "Set Element Classification", binaryTag: 906,
    doc: "Sets the code of an element or type in one classification system; an element without classifications gets its first, another system adds a classification and the same system replaces the code. The code need not be an entry of the system (a diagnostic reports it).",
    props: [
      id("element", "target", { en: "Element", de: "Element" }),
      reference("system", "classification-system", { en: "System", de: "System" }, 20),
      required("code", { en: "Code", de: "Code" }, 30),
    ],
    label: { en: 'format!("Classify \\"{}\\" as {} in {}", self.id, self.code, self.system)', de: 'format!("\\"{}\\" als {} in {} klassifizieren", self.id, self.code, self.system)' },
    target,
    cases: cases([
      { name: "classifies-an-element", emoji: 0x2705, before: WITH_SYSTEMS(), mutation: { id: "w-south", system: "cs-uni", code: "EF_25_10" }, outcome: ok, diff: created("classifications", "w-south", { "cs-uni": "EF_25_10" }) },
      { name: "reclassifies", emoji: 0x1f69b, before: CLASSIFIED(), mutation: { id: "w-south", system: "cs-uni", code: "EF_25" }, outcome: ok, diff: patched("classifications", "w-south", { assigned: { "cs-uni": "EF_25" } }) },
      { name: "adds-another-system", emoji: 0x1f9e9, before: CLASSIFIED(), mutation: { id: "w-south", system: "cs-din", code: "330" }, outcome: ok, diff: patched("classifications", "w-south", { assigned: { "cs-din": "330" } }) },
      { name: "classifies-a-type", emoji: 0x1f3f7, before: WITH_SYSTEMS(), mutation: { id: "wt-300", system: "cs-din", code: "331" }, outcome: ok, diff: created("classifications", "wt-300", { "cs-din": "331" }) },
      { name: "code-outside-the-table", emoji: 0x1f4dd, before: WITH_SYSTEMS(), mutation: { id: "w-south", system: "cs-uni", code: "Zz_99" }, outcome: ok, diff: created("classifications", "w-south", { "cs-uni": "Zz_99" }) },
      { name: "empty-code", emoji: 0x1f6a7, before: WITH_SYSTEMS(), mutation: { id: "w-south", system: "cs-uni", code: "" }, outcome: reject("mutation.invariant", ["code"]) },
      { name: "unknown-element", emoji: 0x1f6ab, before: WITH_SYSTEMS(), mutation: { id: "w-nowhere", system: "cs-uni", code: "EF_25_10" }, outcome: reject("mutation.target-missing", ["w-nowhere"]) },
      { name: "unknown-system", emoji: 0x26d4, before: WITH_SYSTEMS(), mutation: { id: "w-south", system: "cs-nowhere", code: "EF_25_10" }, outcome: reject("mutation.target-missing", ["system"]) },
      { name: "same-classification", emoji: 0x1f6d1, before: CLASSIFIED(), mutation: { id: "w-south", system: "cs-uni", code: "EF_25_10" }, outcome: reject("mutation.no-op", ["w-south"]) },
    ]),
  },
  {
    kind: "remove-element-classification", emoji: 0x1f5c4, variant: "RemoveElementClassification", verb: "remove", entity: "element-classification", displayName: "Remove Element Classification", binaryTag: 907,
    doc: "Removes the classification of an element or type in one classification system; its other classifications stay.",
    props: [id("element", "target", { en: "Element", de: "Element" }), reference("system", "classification-system", { en: "System", de: "System" }, 20)],
    label: { en: 'format!("Remove classification {} of \\"{}\\"", self.system, self.id)', de: 'format!("Klassifizierung {} von \\"{}\\" entfernen", self.system, self.id)' },
    target,
    cases: cases([
      { name: "removes-one-of-two", emoji: 0x1f522, before: CLASSIFIED(), mutation: { id: "w-east", system: "cs-din" }, outcome: ok, diff: patched("classifications", "w-east", { assigned: { "cs-din": null } }) },
      { name: "removes", emoji: 0x2705, before: CLASSIFIED(), mutation: { id: "w-south", system: "cs-uni" }, outcome: ok, diff: deleted("classifications", "w-south") },
      { name: "unknown-element", emoji: 0x26d4, before: CLASSIFIED(), mutation: { id: "w-nowhere", system: "cs-uni" }, outcome: reject("mutation.target-missing", ["w-nowhere"]) },
      { name: "not-classified", emoji: 0x1f6ab, before: CLASSIFIED(), mutation: { id: "w-south", system: "cs-din" }, outcome: reject("mutation.target-missing", ["w-south", "cs-din"]) },
    ]),
  },
];

//#region 🔖️Emit
const out = join(import.meta.dir, "🗑️generated", "w2-wp18-psets");
mkdirSync(out, { recursive: true });
const writeJson = (path: string, value: unknown) => overwrite(path, JSON.stringify(value, null, 2) + "\n");
const schemaDirName = (dir: string) => readdirSync(dir).find((name) => name.endsWith("schema"))!;
const caseDir = (leaf: Leaf, c: Case) => join(fixtures, em(0x1f9ec) + "mutations", em(leaf.emoji) + leaf.kind, em(c.emoji) + c.name);
const AFTER = [em(0x1f4f8) + "snapshot", em(0x27a1) + "after", JSONF];
const DIFF = [em(0x1f53a) + "diff", JSONF];

const payloads: Record<string, string> = {};
const patchPayload = (variant: string, patchType: string, fields: [string, string][], uses: string, doc: string, label: [string, string], assign: Record<string, string>) => {
  const members = fields.map(([name, type]) => `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: ${type},`).join("\n");
  const kind = variant.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
  const entity = kind.replace(/^set-/, "");
  const names = fields.map(([name]) => name);
  const copy = (name: string) => assign[name] ?? `self.${name}.clone()`;
  return `//! ${doc}\n\nuse crate::{${uses}};\nuse protocol::{MutationKind, SemanticDescriptor};\n\n#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]\npub struct ${variant} {\n    pub id: String,\n${members}\n}\n\nimpl ${variant} {\n    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.\n    pub fn patch(&self) -> ${patchType} {\n        ${patchType} { ${names.map((name) => `${name}: ${copy(name)}`).join(", ")} }\n    }\n\n    /// 🧩 The payload that provides exactly the fields \`patch\` names.\n    pub fn from_patch(id: String, patch: ${patchType}) -> Self {\n        Self { id, ${names.map((name) => `${name}: patch.${name}`).join(", ")} }\n    }\n}\n\nimpl MutationKind<ModelSnapshot, ModelMutation> for ${variant} {\n    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "${entity}", kind: "${kind}", record: "${variant}" };\n    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {\n        super::diff::diff(self, base)\n    }\n    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {\n        Ok(super::inverse::inverse(self, base))\n    }\n    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {\n        semio_framework_ui_locale::LocalizedLabel::native(&${label[0]}, &${label[1]})\n    }\n    fn target(&self) -> Vec<String> {\n        vec![self.id.clone()]\n    }\n}\n`;
};
payloads["set-property-template"] = patchPayload(
  "SetPropertyTemplate",
  "PropertyTemplatePatch",
  [["name", "Option<String>"], ["applies_to", "Option<Vec<TemplateTarget>>"], ["properties", "Option<Vec<PropertyDef>>"]],
  "ModelDiff, ModelMutation, ModelSnapshot, PropertyDef, PropertyTemplatePatch, TemplateTarget",
  "🛠️ `set-property-template` payload. Sets any of a property set template's name, applicable kinds and property definitions; absent fields stay untouched and a list replaces the whole list.",
  ['format!("Edit property template \\"{}\\"", self.id)', 'format!("Eigenschaftsvorlage \\"{}\\" ändern", self.id)'],
  {},
);
payloads["set-classification-system"] = patchPayload(
  "SetClassificationSystem",
  "ClassificationSystemPatch",
  [["name", "Option<String>"], ["edition", "Option<String>"], ["source", "Option<Assigned<Option<String>>>"], ["entries", "Option<Vec<ClassificationItem>>"]],
  "Assigned, ClassificationItem, ClassificationSystemPatch, ModelDiff, ModelMutation, ModelSnapshot",
  "📖️ `set-classification-system` payload. Sets any of a classification system's name, edition, source and entry table (an assigned null removes the source); absent fields stay untouched and the entry table replaces the whole table.",
  ['format!("Edit classification system \\"{}\\"", self.id)', 'format!("Klassifikationssystem \\"{}\\" ändern", self.id)'],
  {},
);

let mounts = "";
for (const leaf of leaves) {
  const leafDir = join(mutations, em(leaf.emoji) + leaf.kind);
  mounts += emitLeaf(leaf);
  const payloadPath = join(leafDir, em(0x1f9a0) + "mutation", RS);
  const hand = payloads[leaf.kind];
  if (hand) overwrite(payloadPath, hand);
  const schemaPath = join(leafDir, schemaDirName(leafDir), JSONF);
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  schema.required = ["mutation", ...leaf.props.filter((prop) => !prop.rust.startsWith("Option<")).map((prop) => prop.name)];
  writeJson(schemaPath, schema);
  for (const c of leaf.cases) {
    const dir = caseDir(leaf, c);
    writeJson(join(dir, ...AFTER), c.after);
    if (c.outcome.status === "applied") writeJson(join(dir, ...DIFF), c.diff ?? {});
  }
}
overwrite(join(out, "mounts.txt"), mounts);
//#endregion 🔖️Emit

//#region 🔖️ExtraCases
const find = (suffix: string) => readdirSync(mutations).find((name) => name.endsWith(suffix))!;
const read = (leafDir: string, caseDir: string, ...tail: string[]) => JSON.parse(readFileSync(join(fixtures, em(0x1f9ec) + "mutations", leafDir, caseDir, ...tail, JSONF), "utf8"));
const caseMounts: Record<string, string> = {};
const addCases = (kind: string, variant: string, rows: PCase[]) => {
  const dirName = find(kind);
  const built = cases(rows, POOL_EXTRA);
  caseMounts[kind.replaceAll("-", "_")] = built.map((c) => emitCase(kind, dirName, variant, c)).join("");
  for (const c of built) {
    const dir = join(fixtures, em(0x1f9ec) + "mutations", dirName, em(c.emoji) + c.name);
    writeJson(join(dir, ...AFTER), c.after);
    if (c.outcome.status === "applied") writeJson(join(dir, ...DIFF), c.diff ?? {});
  }
};
const POOL_EXTRA = [0x1f5c2, 0x1f3f7];
const PROPS = { "wt-300": { Pset_WallTypeCommon: { Reference: txt("W-300") } } };

addCases("set-element-property", "SetElementProperty", [
  { name: "sets-a-type-property", before: base(), mutation: { id: "wt-300", pset: "Pset_WallTypeCommon", property: "Reference", value: txt("W-300") }, outcome: ok, diff: created("properties", "wt-300", PROPS["wt-300"]) },
  { name: "type-property-replaced", before: base({ properties: PROPS }), mutation: { id: "wt-300", pset: "Pset_WallTypeCommon", property: "Reference", value: txt("W-310") }, outcome: ok, diff: patched("properties", "wt-300", { assigned: { Pset_WallTypeCommon: { Reference: txt("W-310") } } }) },
]);
addCases("remove-element-property", "RemoveElementProperty", [
  { name: "removes-a-type-property", before: base({ properties: PROPS }), mutation: { id: "wt-300", pset: "Pset_WallTypeCommon", property: "Reference" }, outcome: ok, diff: deleted("properties", "wt-300") },
]);

const TYPES: [string, string, string, string][] = [
  ["delete-wall-type", "DeleteWallType", "wall_types", "removes"],
  ["delete-slab-type", "DeleteSlabType", "slab_types", "removes"],
  ["delete-roof-type", "DeleteRoofType", "roof_types", "removes"],
  ["delete-ceiling-type", "DeleteCeilingType", "ceiling_types", "removes"],
  ["delete-column-type", "DeleteColumnType", "column_types", "removes"],
  ["delete-beam-type", "DeleteBeamType", "beam_types", "removes"],
  ["delete-window-type", "DeleteWindowType", "window_types", "removes"],
  ["delete-door-type", "DeleteDoorType", "door_types", "removes"],
];
for (const [kind, variant, collection, applied] of TYPES) {
  const dirName = find(kind);
  const caseName = readdirSync(join(fixtures, em(0x1f9ec) + "mutations", dirName)).find((name) => name.endsWith(applied))!;
  const before = read(dirName, caseName, em(0x1f4f8) + "snapshot", em(0x2b05) + "before");
  const mutation = read(dirName, caseName, em(0x1f9a0) + "mutation");
  const key = mutation.id as string;
  const withData = { ...before, classification_systems: { "cs-din": DIN276 }, properties: { ...before.properties, [key]: { Pset_TypeCommon: { Reference: txt("T-1") } } }, classifications: { ...before.classifications, [key]: { "cs-din": "331" } } };
  addCases(kind, variant, [
    { name: "removes-its-data", before: withData, mutation: { id: key }, outcome: ok, diff: { [collection]: { [key]: { entry: "Deleted" } }, properties: { [key]: { entry: "Deleted" } }, classifications: { [key]: { entry: "Deleted" } } } },
  ]);
}
overwrite(join(out, "case-mounts.json"), JSON.stringify(caseMounts, null, 2) + "\n");
//#endregion 🔖️ExtraCases
console.log(`emitted ${leaves.length} leaves and ${Object.keys(caseMounts).length} extended leaves; mounts in ${out}`);
void existsSync;
