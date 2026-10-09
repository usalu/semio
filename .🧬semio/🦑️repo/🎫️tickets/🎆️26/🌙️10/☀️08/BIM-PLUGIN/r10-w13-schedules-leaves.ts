#!/usr/bin/env bun
/**
 * 📋️ Wave W13 (`w13-schedules`): the three leaves create/set/delete-schedule of `s.bim.model@1` (binary tags 13000..13002). `bun r10-w13-schedules-leaves.ts` rewrites the boilerplate
 * (descriptor, payload schema, payload file, test files) and the `before`/`mutation`/`outcome` fixtures of every case, patches the sparse payload of `set-schedule`, writes the hand
 * authored `🔺️diff` and `↩️inverse` sources and never a blessed `after`/`diff` (bless them with `BIM_BLESS=1 cargo test`). The mount text goes to `🗑️generated/w13-schedules/mounts.txt`;
 * `bun r10-w13-schedules-mount.ts` splices it into the artifact root and registers the three kinds in the mutation aggregate.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

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
const text = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<String>", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const category = (order: number): Prop => ({ name: "category", rust: "Option<ScheduleCategory>", schema: record("ScheduleCategory"), ui: { widget: "select", role: "value", label: { en: "Category", de: "Kategorie" }, group: "value", order } });
const items = (name: string, def: string, label: Label, order: number): Prop => ({ name, rust: `Option<Vec<${def}>>`, schema: { type: "array", items: record(def) }, ui: { widget: "list", role: "value", label, group: "value", order } });
const flag = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<bool>", schema: { type: "boolean" }, ui: { widget: "toggle", role: "value", label, group: "value", order } });
const strings = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<Vec<String>>", schema: { type: "array", items: { type: "string" } }, ui: { widget: "list", role: "value", label, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const pascal = (token: string) => token.split("_").map((part) => part[0].toUpperCase() + part.slice(1)).join("");
const field = (token: string) => ({ Field: { field: pascal(token) } });
const property = (set: string, name: string) => ({ Property: { set, name } });
const column = (key: unknown, total = false, heading?: string) => ({ key, ...(heading === undefined ? {} : { heading }), total });
const sort = (key: unknown, descending = false) => ({ key, descending });
const filter = (key: unknown, op: string, value: string) => ({ key, op, value });
const group = (key: unknown) => ({ key });
const schedule = (over: Record<string, unknown> = {}) => ({ name: "Walls", category: "Wall", columns: [column(field("name")), column(field("length"), true)], sort: [], filter: [], group: [], itemize: true, storeys: [] as string[], phases: [] as string[], ...over });
const doors = () => schedule({ name: "Doors", category: "Door", columns: [column(field("name")), column(field("type")), column(field("swing")), column(property("Pset_DoorCommon", "FireRating"), false, "Fire rating"), column(field("count"), true)], sort: [sort(field("name"))], storeys: ["st-ground"], phases: ["New"] });
const takeoff = () => schedule({ name: "Material take-off", category: "Material", columns: [column(field("material")), column(field("layer_volume"), true), column(field("layer_mass"), true)], group: [group(field("material"))], itemize: false });

type Parts = { schedules?: Record<string, unknown> };
const base = (parts: Parts = {}) => {
  const snapshot: any = F.scene();
  if (parts.schedules) snapshot.schedules = parts.schedules;
  return snapshot;
};
const WITH_WALLS = () => base({ schedules: { "sch-walls": schedule() } });
const WITH_DOORS = () => base({ schedules: { "sch-walls": schedule(), "sch-doors": doors() } });

export const leaves: Leaf[] = [
  {
    kind: "create-schedule", emoji: 0x1f4ca, variant: "CreateSchedule", verb: "create", entity: "schedule", doc: "Brings a new schedule into the model: the authored definition (category, columns, sort, filter, grouping, scope) of a table whose rows and totals are inferred.", displayName: "Create Schedule", binaryTag: 13000,
    props: [id("schedule", "identity", { en: "Schedule id", de: "Listen-Id" }, 10), recordProp("schedule", "Schedule", { en: "Schedule", de: "Bauteilliste" })],
    label: { en: 'format!("Create schedule \\"{}\\"", self.schedule.name)', de: 'format!("Bauteilliste \\"{}\\" anlegen", self.schedule.name)' },
    target,
    cases: [
      { name: "adds", emoji: 0x2705, before: base(), mutation: { id: "sch-walls", schedule: schedule() }, outcome: ok },
      { name: "adds-a-scoped-door-schedule", emoji: 0x1f6aa, before: WITH_WALLS(), mutation: { id: "sch-doors", schedule: doors() }, outcome: ok },
      { name: "adds-a-material-take-off", emoji: 0x1f9f1, before: WITH_WALLS(), mutation: { id: "sch-takeoff", schedule: takeoff() }, outcome: ok },
      { name: "duplicate", emoji: 0x1f6ab, before: WITH_WALLS(), mutation: { id: "sch-walls", schedule: schedule({ name: "Other" }) }, outcome: reject("mutation.duplicate-id", ["sch-walls"]) },
      { name: "id-taken-by-another-kind", emoji: 0x1f9ed, before: WITH_WALLS(), mutation: { id: "w-south", schedule: schedule() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "blank-name", emoji: 0x1f4db, before: base(), mutation: { id: "sch-walls", schedule: schedule({ name: " " }) }, outcome: reject("mutation.invariant", ["schedule", "name"]) },
      { name: "no-columns", emoji: 0x1f573, before: base(), mutation: { id: "sch-walls", schedule: schedule({ columns: [] }) }, outcome: reject("mutation.invariant", ["schedule", "columns"]) },
      { name: "field-not-offered", emoji: 0x1f9f2, before: base(), mutation: { id: "sch-walls", schedule: schedule({ columns: [column(field("swing"))] }) }, outcome: reject("mutation.invariant", ["schedule", "columns"]) },
      { name: "repeated-column", emoji: 0x267b, before: base(), mutation: { id: "sch-walls", schedule: schedule({ columns: [column(field("name")), column(field("name"), true)] }) }, outcome: reject("mutation.invariant", ["schedule", "columns"]) },
      { name: "filter-without-a-value", emoji: 0x1f50e, before: base(), mutation: { id: "sch-walls", schedule: schedule({ filter: [filter(field("length"), "Greater", "")] }) }, outcome: reject("mutation.invariant", ["schedule", "filter"]) },
      { name: "storey-missing", emoji: 0x1f3e2, before: base(), mutation: { id: "sch-walls", schedule: schedule({ storeys: ["st-attic"] }) }, outcome: reject("mutation.target-missing", ["schedule", "storeys"]) },
    ],
  },
  {
    kind: "set-schedule", emoji: 0x1f4c8, variant: "SetSchedule", verb: "set", entity: "schedule", doc: "Sets any of a schedule's name, category, columns, sort, filter, grouping, itemization, storey scope and phase scope; absent fields stay untouched and a list replaces the whole list.", displayName: "Set Schedule", binaryTag: 13001,
    props: [
      id("schedule", "target", { en: "Schedule", de: "Bauteilliste" }),
      text("name", { en: "Name", de: "Name" }, 20),
      category(30),
      items("columns", "ScheduleColumn", { en: "Columns", de: "Spalten" }, 40),
      items("sort", "ScheduleSort", { en: "Sort", de: "Sortierung" }, 50),
      items("filter", "ScheduleFilter", { en: "Filter", de: "Filter" }, 60),
      items("group", "ScheduleGroup", { en: "Grouping", de: "Gruppierung" }, 70),
      flag("itemize", { en: "Itemize every instance", de: "Jedes Bauteil einzeln auflisten" }, 80),
      strings("storeys", { en: "Storeys", de: "Geschosse" }, 90),
      { name: "phases", rust: "Option<Vec<Phase>>", schema: { type: "array", items: record("Phase") }, ui: { widget: "list", role: "value", label: { en: "Phases", de: "Phasen" }, group: "value", order: 100 } },
    ],
    label: { en: 'format!("Edit schedule \\"{}\\"", self.id)', de: 'format!("Bauteilliste \\"{}\\" ändern", self.id)' },
    target,
    cases: [
      { name: "renames", emoji: 0x2705, before: WITH_WALLS(), mutation: { id: "sch-walls", name: "Wall list" }, outcome: ok },
      { name: "replaces-the-columns", emoji: 0x1f4d0, before: WITH_WALLS(), mutation: { id: "sch-walls", columns: [column(field("name")), column(field("type")), column(field("net_side_area"), true, "Area"), column(property("Pset_WallCommon", "FireRating"))] }, outcome: ok },
      { name: "sorts-and-filters", emoji: 0x1f50e, before: WITH_WALLS(), mutation: { id: "sch-walls", sort: [sort(field("length"), true), sort(field("name"))], filter: [filter(field("length"), "GreaterOrEqual", "6"), filter(field("name"), "Contains", "st")] }, outcome: ok },
      { name: "groups-and-collapses", emoji: 0x1f5c2, before: WITH_WALLS(), mutation: { id: "sch-walls", group: [group(field("type"))], itemize: false }, outcome: ok },
      { name: "scopes-the-storeys-and-phases", emoji: 0x1f3e2, before: WITH_WALLS(), mutation: { id: "sch-walls", storeys: ["st-ground", "st-first"], phases: ["New", "Existing"] }, outcome: ok },
      { name: "changes-the-category", emoji: 0x1f6aa, before: WITH_WALLS(), mutation: { id: "sch-walls", category: "Slab" }, outcome: ok },
      { name: "restates-an-unchanged-field", emoji: 0x1f9ed, before: WITH_WALLS(), mutation: { id: "sch-walls", name: "Walls", itemize: false }, outcome: ok },
      { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_WALLS(), mutation: { id: "sch-walls", name: "Walls", itemize: true, storeys: [] }, outcome: reject("mutation.no-op", ["sch-walls"]) },
      { name: "missing", emoji: 0x26d4, before: WITH_WALLS(), mutation: { id: "sch-roofs", name: "Roofs" }, outcome: reject("mutation.target-missing", ["sch-roofs"]) },
      { name: "category-drops-a-field", emoji: 0x1f9f2, before: WITH_DOORS(), mutation: { id: "sch-doors", category: "Wall" }, outcome: reject("mutation.invariant", ["columns"]) },
      { name: "no-columns", emoji: 0x1f573, before: WITH_WALLS(), mutation: { id: "sch-walls", columns: [] }, outcome: reject("mutation.invariant", ["columns"]) },
      { name: "blank-name", emoji: 0x1f4db, before: WITH_WALLS(), mutation: { id: "sch-walls", name: "" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "storey-missing", emoji: 0x1f3d8, before: WITH_WALLS(), mutation: { id: "sch-walls", storeys: ["st-attic"] }, outcome: reject("mutation.target-missing", ["storeys"]) },
    ],
  },
  {
    kind: "delete-schedule", emoji: 0x1f4c9, variant: "DeleteSchedule", verb: "delete", entity: "schedule", doc: "Removes a schedule; its table is inferred, so nothing else depends on it.", displayName: "Delete Schedule", binaryTag: 13002,
    props: [id("schedule", "target", { en: "Schedule", de: "Bauteilliste" })],
    label: { en: 'format!("Delete schedule \\"{}\\"", self.id)', de: 'format!("Bauteilliste \\"{}\\" löschen", self.id)' },
    target,
    cases: [
      { name: "removes", emoji: 0x2705, before: WITH_DOORS(), mutation: { id: "sch-doors" }, outcome: ok },
      { name: "missing", emoji: 0x26d4, before: WITH_WALLS(), mutation: { id: "sch-doors" }, outcome: reject("mutation.target-missing", ["sch-doors"]) },
    ],
  },
];

const dirOf = (leaf: Leaf) => join(mutations, em(leaf.emoji) + leaf.kind);
const sub = (leaf: Leaf, emoji: number, name: string) => join(dirOf(leaf), em(emoji) + name);
const bt = "`";
const doc = (text: string) => text.replaceAll("¶", bt);

const FILES: Record<string, { diff: string; inverse: string }> = {
  "create-schedule": {
    diff: doc(String.raw`//! 🔺️ Diff constructor for ¶CreateSchedule¶: one created schedule entry. The id is free across every collection, the definition stands (a name, columns that exist in the category,
//! no repeated key, filters with their values, a scope without repeats) and every storey of the scope exists. Rows and totals are inferred, nothing derived is written.

use super::super::elements;
use super::CreateSchedule;
use crate::schedule_kit::schedule_problem;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some((path, message)) = schedule_problem(&payload.schedule) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["schedule", path]);
    }
    if let Some(storey) = payload.schedule.storeys.iter().find(|storey| !base.storeys.contains_key(*storey)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{storey}\" does not exist."), ["schedule", "storeys"]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Created(payload.schedule.clone())))
}
`),
    inverse: doc(String.raw`//! ↩️ Inverse of ¶CreateSchedule¶: the concrete ¶DeleteSchedule¶ of the id it created, none when the id was already taken.

use super::super::delete_schedule::DeleteSchedule;
use super::CreateSchedule;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.schedules.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSchedule(DeleteSchedule { id: payload.id.clone() })]
}
`),
  },
  "set-schedule": {
    diff: doc(String.raw`//! 🔺️ Diff constructor for ¶SetSchedule¶: a sparse schedule patch of exactly the fields that change. The definition that results must stand (see ¶schedule_problem¶: columns that exist in
//! the category, no repeated key, filters with their values, a scope without repeats) and every storey of a new scope must exist; providing only equal values is a no-op. A list replaces the whole
//! list, because columns, sort keys, filters and grouping levels are one ordered definition and not addressable records.

use super::SetSchedule;
use crate::schedule_kit::schedule_problem;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(schedule) = base.schedules.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Schedule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some((path, message)) = schedule_problem(&payload.patch().write(schedule)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [path]);
    }
    if let Some(storey) = payload.storeys.iter().flatten().find(|storey| !base.storeys.contains_key(*storey)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{storey}\" does not exist."), ["storeys"]);
    }
    let patch = payload.patch().minimal(schedule);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Schedule \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Patched(patch)))
}
`),
    inverse: doc(String.raw`//! ↩️ Inverse of ¶SetSchedule¶: an absolute ¶SetSchedule¶ restoring the base value of exactly the fields the forward really changes, none when the schedule is absent or nothing changes.

use super::SetSchedule;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.schedules.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSchedule(SetSchedule::from_patch(payload.id.clone(), restore))]
}
`),
  },
  "delete-schedule": {
    diff: doc(String.raw`//! 🔺️ Diff constructor for ¶DeleteSchedule¶: the schedule leaves in one sparse diff. Its rows and totals are inferred and nothing else refers to it, so nothing cascades.

use super::DeleteSchedule;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.schedules.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Schedule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Deleted))
}
`),
    inverse: doc(String.raw`//! ↩️ Inverse of ¶DeleteSchedule¶: the concrete ¶CreateSchedule¶ of the removed definition, none when the schedule is absent.

use super::super::create_schedule::CreateSchedule;
use super::DeleteSchedule;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSchedule, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.schedules.get(&payload.id) {
        Some(schedule) => vec![ModelMutation::CreateSchedule(CreateSchedule { id: payload.id.clone(), schedule: schedule.clone() })],
        None => Vec::new(),
    }
}
`),
  },
};

const OPTIONALS = ["name", "category", "columns", "sort", "filter", "group", "itemize", "storeys", "phases"];

const fixSet = (leaf: Leaf) => {
  const file = join(sub(leaf, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(file, "utf8");
  source = source.replace(/use crate::\{[^}]*\};/, "use crate::{ModelDiff, ModelMutation, ModelSnapshot, Phase, SchedulePatch, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleSort};");
  for (const name of OPTIONALS) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  if (!source.includes("fn from_patch")) {
    source = source.replace(
      "\nimpl MutationKind<",
      doc(String.raw`
impl SetSchedule {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SchedulePatch {
        SchedulePatch { name: self.name.clone(), category: self.category, columns: self.columns.clone(), sort: self.sort.clone(), filter: self.filter.clone(), group: self.group.clone(), itemize: self.itemize, storeys: self.storeys.clone(), phases: self.phases.clone() }
    }

    /// 🧩 The payload that provides exactly the fields ¶patch¶ names.
    pub fn from_patch(id: String, patch: SchedulePatch) -> Self {
        Self { id, name: patch.name, category: patch.category, columns: patch.columns, sort: patch.sort, filter: patch.filter, group: patch.group, itemize: patch.itemize, storeys: patch.storeys, phases: patch.phases }
    }
}

impl MutationKind<`),
    );
  }
  writeFileSync(file, source);
  const schemaFile = join(sub(leaf, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !OPTIONALS.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
};

const out = join(import.meta.dir, "🗑️generated", "w13-schedules");
mkdirSync(out, { recursive: true });
let mounts = "";
for (const leaf of leaves) {
  mounts += emitLeaf(leaf);
  if (leaf.kind === "set-schedule") fixSet(leaf);
  writeFileSync(join(sub(leaf, 0x1f53a, "diff"), RS), FILES[leaf.kind].diff);
  writeFileSync(join(sub(leaf, 0x21a9, "inverse"), RS), FILES[leaf.kind].inverse);
}
writeFileSync(join(out, "mounts.txt"), mounts);
console.log(`emitted ${leaves.length} leaves; mounts in ${out}`);
