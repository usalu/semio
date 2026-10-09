#!/usr/bin/env bun
/**
 * 🏘️ Wave W07 (`w07-zones`): the six leaves create/set/delete-zone and create/set/delete-area-scheme of `s.bim.model@1` (binary tags 7000..7005)
 * plus the new cases of `create-space`, `set-space` and `delete-material`. `bun r10-w07-zones-leaves.ts` rewrites the boilerplate (descriptor, payload
 * schema, payload file, test files) and the `before`/`mutation`/`outcome` fixtures of every case, never the hand-written `🔺️diff` and `↩️inverse`
 * files and never a payload file that already carries its hand-written `patch`/`from_patch` and never a blessed `after`/`diff` (bless them with `BIM_BLESS=1 cargo test`). It prints the mount text of the new leaves and of the new cases
 * of the three existing leaves into `🗑️generated/w07-zones/mounts.txt` and `…/case-mounts.json`; paste them with `r10-w07-zones-mount.py`.
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
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
const num = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<f64>", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const measure = (order: number): Prop => ({ name: "measure", rust: "Option<AreaMeasure>", schema: record("AreaMeasure"), ui: { widget: "select", role: "value", label: { en: "Measure", de: "Fläche" }, group: "value", order } });
const list = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<Vec<String>>", schema: { type: "array", items: { type: "string" } }, ui: { widget: "list", role: "value", label, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const P = F.P;
const bounded = (x: number, y: number) => ({ Bounded: { seed: P(x, y) } });
const explicit = (list: [number, number][]) => ({ Explicit: { outline: list.map(([x, y]) => ({ point: P(x, y), bulge: 0 })) } });
const space = (storey: string, number: string, name: string, boundary: unknown, usage: string, over: Record<string, unknown> = {}) => ({ storey, number, name, boundary, usage, phase: "New", ...over });
const zone = (name: string, category: string, occupancy_density: number) => ({ name, category, occupancy_density });
const scheme = (name: string, measure: string, usages: string[], zones: string[]) => ({ name, measure, usages, zones });

type Parts = { zones?: Record<string, unknown>; area_schemes?: Record<string, unknown>; spaces?: Record<string, unknown>; properties?: Record<string, unknown>; classifications?: Record<string, unknown> };
const base = (parts: Parts = {}) => {
  const snapshot: any = F.scene();
  snapshot.materials["m-paint"] = F.material("Paint");
  snapshot.materials["m-tile"] = F.material("Tile");
  snapshot.spaces = parts.spaces ?? {};
  if (parts.properties) snapshot.properties = parts.properties;
  if (parts.classifications) snapshot.classifications = parts.classifications;
  if (parts.zones) snapshot.zones = parts.zones;
  if (parts.area_schemes) snapshot.area_schemes = parts.area_schemes;
  return snapshot;
};
const SPACES = {
  "sp-1": space("st-ground", "0.01", "Kitchen", bounded(2, 2), "Kitchen"),
  "sp-2": space("st-ground", "0.02", "Hall", explicit([[0, 0], [3, 0], [3, 2], [0, 2]]), "Circulation"),
  "sp-3": space("st-first", "1.01", "Bedroom", bounded(5, 3), "Sleeping"),
};
const ZONES = { "z-day": zone("Day zone", "Ventilation", 0.05), "z-night": zone("Night zone", "Ventilation", 0.02) };
const MEMBERS = { ...SPACES, "sp-1": { ...SPACES["sp-1"], zone: "z-day" }, "sp-2": { ...SPACES["sp-2"], zone: "z-day" }, "sp-3": { ...SPACES["sp-3"], zone: "z-night" } };
const WITH_ZONES = () => base({ spaces: SPACES, zones: ZONES });
const WITH_MEMBERS = () => base({ spaces: MEMBERS, zones: ZONES });
const WITH_SCHEME = () => base({ spaces: MEMBERS, zones: ZONES, area_schemes: { "as-nsa": scheme("Net sales area", "Net", ["Kitchen"], ["z-day"]) } });
const DATA = { properties: { "z-day": { Pset_ZoneCommon: { Reference: { Text: { value: "Z-1" } } } } }, classifications: { "z-day": { system: "Uniclass", code: "Zz_10", title: "Zones" } } };

export const leaves: Leaf[] = [
  {
    kind: "create-zone", emoji: 0x1f5fe, variant: "CreateZone", verb: "create", entity: "zone", doc: "Brings a new zone into the model: a named group of spaces with a purpose and an occupancy density; the spaces join it through `set-space`.", displayName: "Create Zone", binaryTag: 7000,
    props: [id("zone", "identity", { en: "Zone id", de: "Zonen-Id" }, 10), recordProp("zone", "Zone", { en: "Zone", de: "Zone" })],
    label: { en: 'format!("Create zone \\"{}\\"", self.zone.name)', de: 'format!("Zone \\"{}\\" anlegen", self.zone.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: WITH_ZONES(), mutation: { id: "z-office", zone: zone("Office wing", "Tenant", 0.1) }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: WITH_ZONES(), mutation: { id: "z-day", zone: zone("Other", "Tenant", 0.1) }, outcome: reject("mutation.duplicate-id", ["z-day"]) },
      { name: "id-taken-by-another-kind", emoji: 0x1f9ed, before: WITH_ZONES(), mutation: { id: "sp-1", zone: zone("Other", "Tenant", 0.1) }, outcome: reject("mutation.duplicate-id", ["sp-1"]) },
      { name: "negative-density", emoji: 0x1f4c9, before: WITH_ZONES(), mutation: { id: "z-office", zone: zone("Office wing", "Tenant", -0.1) }, outcome: reject("mutation.invariant", ["zone", "occupancy_density"]) },
    ],
  },
  {
    kind: "set-zone", emoji: 0x1fa84, variant: "SetZone", verb: "set", entity: "zone", doc: "Sets any of a zone's name, category and occupancy density; absent fields stay untouched.", displayName: "Set Zone", binaryTag: 7001,
    props: [
      id("zone", "target", { en: "Zone", de: "Zone" }),
      text("name", { en: "Name", de: "Name" }, 20),
      text("category", { en: "Category", de: "Kategorie" }, 30, "value"),
      num("occupancy_density", { en: "Occupancy (persons/m²)", de: "Belegung (Personen/m²)" }, 40),
    ],
    label: { en: 'format!("Edit zone \\"{}\\"", self.id)', de: 'format!("Zone \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "renames-and-recategorises", emoji: 0x2705, before: WITH_ZONES(), mutation: { id: "z-day", name: "Living zone", category: "Fire compartment" }, outcome: ok },
      { name: "sets-the-density", emoji: 0x1f465, before: WITH_ZONES(), mutation: { id: "z-night", occupancy_density: 0.04 }, outcome: ok },
      { name: "restates-an-unchanged-field", emoji: 0x1f9ed, before: WITH_ZONES(), mutation: { id: "z-day", name: "Day zone", occupancy_density: 0.08 }, outcome: ok },
      { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_ZONES(), mutation: { id: "z-day", name: "Day zone", category: "Ventilation" }, outcome: reject("mutation.no-op", ["z-day"]) },
      { name: "missing", emoji: 0x26d4, before: WITH_ZONES(), mutation: { id: "z-attic", name: "Loft" }, outcome: reject("mutation.target-missing", ["z-attic"]) },
      { name: "negative-density", emoji: 0x1f4c9, before: WITH_ZONES(), mutation: { id: "z-day", occupancy_density: -1 }, outcome: reject("mutation.invariant", ["occupancy_density"]) },
    ],
  },
  {
    kind: "delete-zone", emoji: 0x1f9ef, variant: "DeleteZone", verb: "delete", entity: "zone", doc: "Removes a zone with its properties and classifications and clears the zone of every space that belonged to it; refuses while an area scheme still counts it.", displayName: "Delete Zone", binaryTag: 7002,
    props: [id("zone", "target", { en: "Zone", de: "Zone" })],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete zone \\"{}\\"", self.id)', de: 'format!("Zone \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "clears-the-memberships", emoji: 0x2705, before: WITH_MEMBERS(), mutation: { id: "z-day" }, outcome: ok },
      { name: "removes-its-data", emoji: 0x1f5c2, before: base({ spaces: MEMBERS, zones: ZONES, ...DATA }), mutation: { id: "z-day" }, outcome: ok },
      { name: "empty-zone", emoji: 0x1f6d6, before: base({ spaces: SPACES, zones: ZONES }), mutation: { id: "z-night" }, outcome: ok },
      { name: "counted-by-an-area-scheme", emoji: 0x1f5c3, before: WITH_SCHEME(), mutation: { id: "z-day" }, outcome: reject("mutation.target-referenced", ["z-day"]) },
      { name: "missing", emoji: 0x26d4, before: WITH_ZONES(), mutation: { id: "z-attic" }, outcome: reject("mutation.target-missing", ["z-attic"]) },
    ],
  },
  {
    kind: "create-area-scheme", emoji: 0x1f5c3, variant: "CreateAreaScheme", verb: "create", entity: "area-scheme", doc: "Brings a new area scheme into the model: the authored rule that decides which spaces a gross, net or rentable area adds up, by usage and by zone.", displayName: "Create Area Scheme", binaryTag: 7003,
    props: [id("area-scheme", "identity", { en: "Area scheme id", de: "Flächenschema-Id" }, 10), recordProp("area_scheme", "AreaScheme", { en: "Area scheme", de: "Flächenschema" })],
    label: { en: 'format!("Create area scheme \\"{}\\"", self.area_scheme.name)', de: 'format!("Flächenschema \\"{}\\" anlegen", self.area_scheme.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: WITH_MEMBERS(), mutation: { id: "as-gfa", area_scheme: scheme("Gross floor area", "Gross", [], []) }, outcome: ok },
      { name: "adds-a-restricted-scheme", emoji: 0x1f3af, before: WITH_MEMBERS(), mutation: { id: "as-nsa", area_scheme: scheme("Net sales area", "Net", ["Kitchen", "Sleeping"], ["z-day"]) }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: WITH_SCHEME(), mutation: { id: "as-nsa", area_scheme: scheme("Other", "Net", [], []) }, outcome: reject("mutation.duplicate-id", ["as-nsa"]) },
      { name: "id-taken-by-another-kind", emoji: 0x1f9ed, before: WITH_SCHEME(), mutation: { id: "z-day", area_scheme: scheme("Other", "Net", [], []) }, outcome: reject("mutation.duplicate-id", ["z-day"]) },
      { name: "zone-missing", emoji: 0x1f3d8, before: WITH_MEMBERS(), mutation: { id: "as-gfa", area_scheme: scheme("Gross floor area", "Gross", [], ["z-attic"]) }, outcome: reject("mutation.target-missing", ["area_scheme", "zones"]) },
      { name: "blank-usage", emoji: 0x1f4dd, before: WITH_MEMBERS(), mutation: { id: "as-gfa", area_scheme: scheme("Gross floor area", "Gross", ["Kitchen", " "], []) }, outcome: reject("mutation.invariant", ["area_scheme", "usages"]) },
    ],
  },
  {
    kind: "set-area-scheme", emoji: 0x1f5f3, variant: "SetAreaScheme", verb: "set", entity: "area-scheme", doc: "Sets any of an area scheme's name, measure, counted usages and counted zones; a list replaces the whole rule list and an empty list counts everything.", displayName: "Set Area Scheme", binaryTag: 7004,
    props: [
      id("area-scheme", "target", { en: "Area scheme", de: "Flächenschema" }),
      text("name", { en: "Name", de: "Name" }, 20),
      measure(30),
      list("usages", { en: "Counted usages", de: "Gezählte Nutzungen" }, 40),
      list("zones", { en: "Counted zones", de: "Gezählte Zonen" }, 50),
    ],
    label: { en: 'format!("Edit area scheme \\"{}\\"", self.id)', de: 'format!("Flächenschema \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "retargets-the-rule", emoji: 0x2705, before: WITH_SCHEME(), mutation: { id: "as-nsa", usages: ["Kitchen", "Sleeping"], zones: ["z-day", "z-night"] }, outcome: ok },
      { name: "changes-the-measure", emoji: 0x1f4d0, before: WITH_SCHEME(), mutation: { id: "as-nsa", name: "Gross sales area", measure: "Gross" }, outcome: ok },
      { name: "counts-everything", emoji: 0x267e, before: WITH_SCHEME(), mutation: { id: "as-nsa", usages: [], zones: [] }, outcome: ok },
      { name: "restates-an-unchanged-field", emoji: 0x1f9ed, before: WITH_SCHEME(), mutation: { id: "as-nsa", measure: "Net", usages: ["Kitchen", "Sleeping"] }, outcome: ok },
      { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_SCHEME(), mutation: { id: "as-nsa", measure: "Net", zones: ["z-day"] }, outcome: reject("mutation.no-op", ["as-nsa"]) },
      { name: "missing", emoji: 0x26d4, before: WITH_SCHEME(), mutation: { id: "as-gfa", measure: "Gross" }, outcome: reject("mutation.target-missing", ["as-gfa"]) },
      { name: "zone-missing", emoji: 0x1f3d8, before: WITH_SCHEME(), mutation: { id: "as-nsa", zones: ["z-attic"] }, outcome: reject("mutation.target-missing", ["zones"]) },
      { name: "blank-usage", emoji: 0x1f4dd, before: WITH_SCHEME(), mutation: { id: "as-nsa", usages: [""] }, outcome: reject("mutation.invariant", ["usages"]) },
    ],
  },
  {
    kind: "delete-area-scheme", emoji: 0x1f9fb, variant: "DeleteAreaScheme", verb: "delete", entity: "area-scheme", doc: "Removes an area scheme with its properties and classifications; nothing else depends on it.", displayName: "Delete Area Scheme", binaryTag: 7005,
    props: [id("area-scheme", "target", { en: "Area scheme", de: "Flächenschema" })],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete area scheme \\"{}\\"", self.id)', de: 'format!("Flächenschema \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: WITH_SCHEME(), mutation: { id: "as-nsa" }, outcome: ok },
      { name: "removes-its-data", emoji: 0x1f5c2, before: base({ spaces: MEMBERS, zones: ZONES, area_schemes: { "as-nsa": scheme("Net sales area", "Net", ["Kitchen"], ["z-day"]) }, properties: { "as-nsa": { Pset_AreaScheme: { Reference: { Text: { value: "NSA" } } } } }, classifications: { "as-nsa": { system: "DIN 277", code: "NUF", title: "Nutzungsfläche" } } }), mutation: { id: "as-nsa" }, outcome: ok },
      { name: "missing", emoji: 0x26d4, before: WITH_SCHEME(), mutation: { id: "as-gfa" }, outcome: reject("mutation.target-missing", ["as-gfa"]) },
    ],
  },
];

const out = join(import.meta.dir, "🗑️generated", "w07-zones");
mkdirSync(out, { recursive: true });
let mounts = "";
for (const leaf of leaves) {
  const payload = join(mutations, em(leaf.emoji) + leaf.kind, em(0x1f9a0) + "mutation", RS);
  const kept = existsSync(payload) ? readFileSync(payload, "utf8") : "";
  mounts += emitLeaf(leaf);
  if (kept.includes("pub fn from_patch")) writeFileSync(payload, kept);
}
writeFileSync(join(out, "mounts.txt"), mounts);

const read = (leafDir: string, caseDir: string, ...tail: string[]) => JSON.parse(readFileSync(join(fixtures, em(0x1f9ec) + "mutations", leafDir, caseDir, ...tail, JSONF), "utf8"));
const spaceCase = (leafDir: string, caseDir: string) => read(leafDir, caseDir, em(0x1f4f8) + "snapshot", em(0x2b05) + "before");
const withMaterials = (snapshot: any) => ({ ...snapshot, materials: { ...snapshot.materials, "m-paint": F.material("Paint"), "m-tile": F.material("Tile") } });
const withZones = (snapshot: any) => ({ ...snapshot, zones: ZONES });

const SET_SPACE = em(0x1fa91) + "set-space";
const CREATE_SPACE = em(0x1f6cb) + "create-space";
const DELETE_MATERIAL = em(0x1f5d1) + "delete-material";
const setSpaceBase = () => withZones(withMaterials(spaceCase(SET_SPACE, em(0x2705) + "renames-and-retypes")));
const setSpaceZoned = () => ({ ...setSpaceBase(), spaces: { ...setSpaceBase().spaces, "sp-1": { ...setSpaceBase().spaces["sp-1"], zone: "z-day" } } });
const setSpaceFinished = () => ({ ...setSpaceBase(), spaces: { ...setSpaceBase().spaces, "sp-1": { ...setSpaceBase().spaces["sp-1"], zone: "z-day", floor_finish: "m-tile", wall_finish: "m-paint" } } });
const createSpaceBase = () => withZones(withMaterials(spaceCase(CREATE_SPACE, em(0x2705) + "adds-a-bounded-space")));

const setSpaceCases: Case[] = [
  { name: "assigns-a-zone", emoji: 0x1f3d8, before: setSpaceBase(), mutation: { id: "sp-1", zone: { value: "z-day" } }, outcome: ok },
  { name: "moves-to-another-zone", emoji: 0x1f6b6, before: setSpaceZoned(), mutation: { id: "sp-1", zone: { value: "z-night" } }, outcome: ok },
  { name: "clears-the-zone", emoji: 0x1f6ae, before: setSpaceZoned(), mutation: { id: "sp-1", zone: { value: null } }, outcome: ok },
  { name: "finishes-the-room", emoji: 0x1f3a8, before: setSpaceBase(), mutation: { id: "sp-1", floor_finish: { value: "m-tile" }, wall_finish: { value: "m-paint" }, ceiling_finish: { value: "m-paint" } }, outcome: ok },
  { name: "clears-a-finish", emoji: 0x1f9fc, before: setSpaceFinished(), mutation: { id: "sp-1", wall_finish: { value: null }, floor_finish: { value: "m-tile" } }, outcome: ok },
  { name: "zone-already-set", emoji: 0x1f4cc, before: setSpaceZoned(), mutation: { id: "sp-1", zone: { value: "z-day" } }, outcome: reject("mutation.no-op", ["sp-1"]) },
  { name: "zone-missing", emoji: 0x1f5fa, before: setSpaceBase(), mutation: { id: "sp-1", zone: { value: "z-attic" } }, outcome: reject("mutation.target-missing", ["zone"]) },
  { name: "finish-material-missing", emoji: 0x1f9f1, before: setSpaceBase(), mutation: { id: "sp-1", ceiling_finish: { value: "m-gold" } }, outcome: reject("mutation.target-missing", ["ceiling_finish"]) },
];
const createSpaceCases: Case[] = [
  { name: "adds-a-finished-space", emoji: 0x1f3a8, before: createSpaceBase(), mutation: { id: "sp-9", space: space("st-ground", "0.09", "Studio", bounded(2, 2), "Office", { zone: "z-day", floor_finish: "m-tile", wall_finish: "m-paint", ceiling_finish: "m-paint" }) }, outcome: ok },
  { name: "zone-missing", emoji: 0x1f3d8, before: createSpaceBase(), mutation: { id: "sp-9", space: space("st-ground", "0.09", "Studio", bounded(2, 2), "Office", { zone: "z-attic" }) }, outcome: reject("mutation.target-missing", ["space", "zone"]) },
  { name: "finish-material-missing", emoji: 0x1f9f1, before: createSpaceBase(), mutation: { id: "sp-9", space: space("st-ground", "0.09", "Studio", bounded(2, 2), "Office", { wall_finish: "m-gold" }) }, outcome: reject("mutation.target-missing", ["space", "wall_finish"]) },
];
const deleteMaterialCases: Case[] = [
  {
    name: "used-by-a-space-finish",
    emoji: 0x1f6cb,
    before: { ...read(DELETE_MATERIAL, em(0x2705) + "removes", em(0x1f4f8) + "snapshot", em(0x2b05) + "before"), spaces: { "sp-1": space("st-ground", "0.01", "Kitchen", bounded(2, 2), "Kitchen", { floor_finish: "m-spare" }) } },
    mutation: { id: "m-spare" },
    outcome: reject("mutation.target-referenced", ["m-spare"]),
  },
];

const caseMounts: Record<string, string> = {};
const mountCases = (module: string, dirName: string, variant: string, kind: string, cases: Case[]) => {
  caseMounts[module] = cases.map((c) => emitCase(kind, dirName, variant, c)).join("");
};
mountCases("set_space", SET_SPACE, "SetSpace", "set-space", setSpaceCases);
mountCases("create_space", CREATE_SPACE, "CreateSpace", "create-space", createSpaceCases);
mountCases("delete_material", DELETE_MATERIAL, "DeleteMaterial", "delete-material", deleteMaterialCases);
writeFileSync(join(out, "case-mounts.json"), JSON.stringify(caseMounts, null, 2) + "\n");
console.log(`emitted ${leaves.length} leaves and ${setSpaceCases.length + createSpaceCases.length + deleteMaterialCases.length} extra cases; mounts in ${out}`);
void mutations;
void RS;
