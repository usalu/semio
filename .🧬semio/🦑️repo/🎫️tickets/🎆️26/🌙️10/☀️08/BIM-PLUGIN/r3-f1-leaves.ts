#!/usr/bin/env bun
/** 🧪️ The twelve foundation leaves of `s.bim.model@1`. `bun r3-f1-leaves.ts` rewrites their boilerplate and fixtures and prints the mount lines. */
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";

const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const record = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const id = (entity: string, role: "target" | "identity", label: { en: string; de: string }, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: entity }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const recordProp = (name: string, def: string, label: { en: string; de: string }, order = 20): Prop => ({ name, rust: def, schema: record(def), ui: { widget: "record", role: "value", label, group: "value", order } });
const num = (name: string, label: { en: string; de: string }, order = 20): Prop => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const text = (name: string, label: { en: string; de: string }, order = 20): Prop => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const int = (name: string, label: { en: string; de: string }, order = 20): Prop => ({ name, rust: "i32", schema: { type: "integer" }, ui: { widget: "integer", role: "value", label, group: "value", order } });
const topProp: Prop = { name: "top", rust: "TopConstraint", schema: record("TopConstraint"), ui: { widget: "record", role: "value", label: { en: "Top", de: "Oberkante" }, group: "value", order: 20 } };

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const U = (n: string) => n;
void U;

const scene = F.scene;
const withFirstWall = () => scene({ walls: { "w-first": F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.storeyTop(0), "First") } });
const siteOnly = () => F.snap({ sites: { "site-1": F.site("Plot") } });
const siteBuilding = () => F.snap({ sites: { "site-1": F.site("Plot") }, buildings: { "bldg-1": F.building("site-1", "House") } });

export const leaves: Leaf[] = [
  {
    kind: "create-site", emoji: 0x1f30d, variant: "CreateSite", verb: "create", entity: "site", doc: "Brings a new site into existence.", displayName: "Create Site", binaryTag: 0,
    props: [id("site", "identity", { en: "Site id", de: "Standort-Id" }, 10), recordProp("site", "Site", { en: "Site", de: "Standort" })],
    label: { en: 'format!("Create site \\"{}\\"", self.site.name)', de: 'format!("Standort \\"{}\\" anlegen", self.site.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: F.snap(), mutation: { id: "site-1", site: F.site("Plot") }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: siteOnly(), mutation: { id: "site-1", site: F.site("Other") }, outcome: reject("mutation.duplicate-id", ["site-1"]) },
    ],
  },
  {
    kind: "delete-site", emoji: 0x1f9f9, variant: "DeleteSite", verb: "delete", entity: "site", doc: "Removes a site that no building stands on.", displayName: "Delete Site", binaryTag: 1,
    props: [id("site", "target", { en: "Site", de: "Standort" })],
    label: { en: 'format!("Delete site \\"{}\\"", self.id)', de: 'format!("Standort \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: siteOnly(), mutation: { id: "site-1" }, outcome: ok },
      { name: "has-buildings", emoji: 0x1f6ab, before: siteBuilding(), mutation: { id: "site-1" }, outcome: reject("mutation.target-referenced", ["site-1"]) },
    ],
  },
  {
    kind: "create-building", emoji: 0x1f3e2, variant: "CreateBuilding", verb: "create", entity: "building", doc: "Brings a new building onto an existing site.", displayName: "Create Building", binaryTag: 2,
    props: [id("building", "identity", { en: "Building id", de: "Gebäude-Id" }, 10), recordProp("building", "Building", { en: "Building", de: "Gebäude" })],
    label: { en: 'format!("Create building \\"{}\\"", self.building.name)', de: 'format!("Gebäude \\"{}\\" anlegen", self.building.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: siteOnly(), mutation: { id: "bldg-1", building: F.building("site-1", "House") }, outcome: ok },
      { name: "site-missing", emoji: 0x1f6ab, before: F.snap(), mutation: { id: "bldg-1", building: F.building("site-1", "House") }, outcome: reject("mutation.target-missing", ["building", "site"]) },
    ],
  },
  {
    kind: "delete-building", emoji: 0x1f3da, variant: "DeleteBuilding", verb: "delete", entity: "building", doc: "Removes a building that has neither storeys nor grid lines.", displayName: "Delete Building", binaryTag: 3,
    props: [id("building", "target", { en: "Building", de: "Gebäude" })],
    label: { en: 'format!("Delete building \\"{}\\"", self.id)', de: 'format!("Gebäude \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: siteBuilding(), mutation: { id: "bldg-1" }, outcome: ok },
      { name: "has-storeys", emoji: 0x1f6ab, before: scene(), mutation: { id: "bldg-1" }, outcome: reject("mutation.target-referenced", ["bldg-1"]) },
    ],
  },
  {
    kind: "create-storey", emoji: 0x1fa9c, variant: "CreateStorey", verb: "create", entity: "storey", doc: "Brings a new storey into a building; its level index is unique within the building.", displayName: "Create Storey", binaryTag: 4,
    props: [id("storey", "identity", { en: "Storey id", de: "Geschoss-Id" }, 10), recordProp("storey", "Storey", { en: "Storey", de: "Geschoss" })],
    label: { en: 'format!("Create storey \\"{}\\"", self.storey.name)', de: 'format!("Geschoss \\"{}\\" anlegen", self.storey.name)' },
    target,
    cases: [
      { name: "stacks-above", emoji: 0x2705, before: siteBuilding(), mutation: { id: "st-ground", storey: F.storey("bldg-1", "Ground", 0, 3) }, outcome: ok },
      { name: "level-taken", emoji: 0x1f6ab, before: scene(), mutation: { id: "st-second", storey: F.storey("bldg-1", "Second", 1, 2.8) }, outcome: reject("mutation.invariant", ["storey", "level"]) },
      { name: "building-missing", emoji: 0x26d4, before: siteOnly(), mutation: { id: "st-ground", storey: F.storey("bldg-1", "Ground", 0, 3) }, outcome: reject("mutation.target-missing", ["storey", "building"]) },
    ],
  },
  {
    kind: "rename-storey", emoji: 0x1f3f7, variant: "RenameStorey", verb: "rename", entity: "storey", doc: "Changes a storey's display name.", displayName: "Rename Storey", binaryTag: 5,
    props: [id("storey", "target", { en: "Storey", de: "Geschoss" }), text("name", { en: "Name", de: "Name" })],
    label: { en: 'format!("Rename storey to \\"{}\\"", self.name)', de: 'format!("Geschoss in \\"{}\\" umbenennen", self.name)' },
    target,
    cases: [
      { name: "renames", emoji: 0x2705, before: scene(), mutation: { id: "st-first", name: "Upper Floor" }, outcome: ok },
      { name: "missing", emoji: 0x1f6ab, before: scene(), mutation: { id: "st-attic", name: "Attic" }, outcome: reject("mutation.target-missing", ["st-attic"]) },
    ],
  },
  {
    kind: "set-storey-height", emoji: 0x1f4cf, variant: "SetStoreyHeight", verb: "set", entity: "storey", doc: "Sets a storey's floor-to-floor height in metres; every elevation above and every wall resolved by it follows by inference.", displayName: "Set Storey Height", binaryTag: 6,
    props: [id("storey", "target", { en: "Storey", de: "Geschoss" }), num("height", { en: "Height (m)", de: "Höhe (m)" })],
    label: { en: 'format!("Set storey height to {} m", self.height)', de: 'format!("Geschosshöhe auf {} m setzen", self.height)' },
    target,
    cases: [
      { name: "raises-the-ground-storey", emoji: 0x2705, before: scene(), mutation: { id: "st-ground", height: 3.4 }, outcome: ok },
      { name: "non-positive", emoji: 0x1f6ab, before: scene(), mutation: { id: "st-ground", height: 0 }, outcome: reject("mutation.invariant", ["height"]) },
      { name: "missing", emoji: 0x26d4, before: scene(), mutation: { id: "st-attic", height: 3 }, outcome: reject("mutation.target-missing", ["st-attic"]) },
    ],
  },
  {
    kind: "set-storey-level", emoji: 0x1f522, variant: "SetStoreyLevel", verb: "set", entity: "storey", doc: "Sets a storey's level index; level 0 is the building datum, positive levels stack upward, negative levels downward.", displayName: "Set Storey Level", binaryTag: 7,
    props: [id("storey", "target", { en: "Storey", de: "Geschoss" }), int("level", { en: "Level", de: "Ebene" })],
    label: { en: 'format!("Set storey level to {}", self.level)', de: 'format!("Geschossebene auf {} setzen", self.level)' },
    target,
    cases: [
      { name: "relevels", emoji: 0x2705, before: scene(), mutation: { id: "st-first", level: 2 }, outcome: ok },
      { name: "level-taken", emoji: 0x1f6ab, before: scene(), mutation: { id: "st-first", level: 0 }, outcome: reject("mutation.invariant", ["level"]) },
    ],
  },
  {
    kind: "delete-storey", emoji: 0x1f6ae, variant: "DeleteStorey", verb: "delete", entity: "storey", doc: "Removes a storey and every wall on it; refuses while a kind without a create leaf (or another storey's constraint) still depends on it.", displayName: "Delete Storey", binaryTag: 8,
    props: [id("storey", "target", { en: "Storey", de: "Geschoss" })],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete storey \\"{}\\" with its walls", self.id)', de: 'format!("Geschoss \\"{}\\" mit Wänden löschen", self.id)' },
    target,
    cases: [
      { name: "cascades-the-walls", emoji: 0x2705, before: scene(), mutation: { id: "st-ground" }, outcome: ok },
      { name: "empty-storey", emoji: 0x1f9f2, before: scene(), mutation: { id: "st-first" }, outcome: ok },
      { name: "constrains-another-wall", emoji: 0x1f6ab, before: scene({ walls: { "w-first": F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.toStorey("st-ground", 0.2), "First") } }), mutation: { id: "st-ground" }, outcome: reject("mutation.target-referenced", ["st-ground"]) },
      { name: "hosts-an-opening", emoji: 0x26d4, before: scene({ openings: { "o-1": F.opening("w-south", "Window") } }), mutation: { id: "st-ground" }, outcome: reject("mutation.target-referenced", ["st-ground"]) },
    ],
  },
  {
    kind: "create-wall", emoji: 0x1f9f1, variant: "CreateWall", verb: "create", entity: "wall", doc: "Brings a new wall onto a storey; its height is never stored, it is inferred from the top constraint.", displayName: "Create Wall", binaryTag: 9,
    props: [id("wall", "identity", { en: "Wall id", de: "Wand-Id" }, 10), recordProp("wall", "Wall", { en: "Wall", de: "Wand" })],
    label: { en: 'format!("Create wall \\"{}\\"", self.wall.name)', de: 'format!("Wand \\"{}\\" anlegen", self.wall.name)' },
    target,
    cases: [
      { name: "adds-on-the-first-storey", emoji: 0x2705, before: scene(), mutation: { id: "w-first", wall: F.wall("st-first", "wt-300", F.line([0, 0], [8, 0]), F.storeyTop(0), "First") }, outcome: ok },
      { name: "type-missing", emoji: 0x1f6ab, before: scene(), mutation: { id: "w-first", wall: F.wall("st-first", "wt-missing", F.line([0, 0], [8, 0]), F.storeyTop(0), "First") }, outcome: reject("mutation.target-missing", ["wall", "wall_type"]) },
      { name: "zero-length", emoji: 0x26d4, before: scene(), mutation: { id: "w-first", wall: F.wall("st-first", "wt-300", F.line([1, 1], [1, 1]), F.storeyTop(0), "First") }, outcome: reject("mutation.invariant", ["wall", "axis"]) },
    ],
  },
  {
    kind: "delete-wall", emoji: 0x1f4a5, variant: "DeleteWall", verb: "delete", entity: "wall", doc: "Removes a wall that hosts no opening.", displayName: "Delete Wall", binaryTag: 10,
    props: [id("wall", "target", { en: "Wall", de: "Wand" })],
    label: { en: 'format!("Delete wall \\"{}\\"", self.id)', de: 'format!("Wand \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: scene(), mutation: { id: "w-east" }, outcome: ok },
      { name: "hosts-an-opening", emoji: 0x1f6ab, before: scene({ openings: { "o-1": F.opening("w-south", "Window") } }), mutation: { id: "w-south" }, outcome: reject("mutation.target-referenced", ["w-south"]) },
    ],
  },
  {
    kind: "set-wall-top", emoji: 0x1f51d, variant: "SetWallTop", verb: "set", entity: "wall", doc: "Sets how a wall's top is resolved: free height, the own storey's top, or another storey of the same building.", displayName: "Set Wall Top", binaryTag: 11,
    props: [id("wall", "target", { en: "Wall", de: "Wand" }), topProp],
    label: { en: 'format!("Constrain the top of wall \\"{}\\"", self.id)', de: 'format!("Oberkante von Wand \\"{}\\" festlegen", self.id)' },
    target,
    cases: [
      { name: "constrains-to-the-first-storey", emoji: 0x2705, before: scene(), mutation: { id: "w-east", top: F.toStorey("st-first", 0) }, outcome: ok },
      { name: "frees-the-height", emoji: 0x1f9f2, before: scene(), mutation: { id: "w-south", top: F.unconnected(2.7) }, outcome: ok },
      { name: "storey-missing", emoji: 0x1f6ab, before: scene(), mutation: { id: "w-east", top: F.toStorey("st-attic", 0) }, outcome: reject("mutation.target-missing", ["top", "storey"]) },
    ],
  },
];
void withFirstWall;

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "f1-foundation");
  mkdirSync(out, { recursive: true });
  let all = "";
  for (const leaf of leaves) all += emitLeaf(leaf);
  writeFileSync(join(out, "mounts.txt"), all);
  console.log(`emitted ${leaves.length} leaves`);
}
