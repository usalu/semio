#!/usr/bin/env bun
/**
 * 🖼️ Wave W12 (`w12-views`): the three leaves create-view, set-view and delete-view of `s.bim.model@1` (binary tags 12000..12002).
 * `bun r10-w12-views-leaves.ts` rewrites their boilerplate, their hand-written `diff`/`inverse` and their fixtures (never a blessed `after`/`diff`)
 * and writes the mount text to `🗑️generated/w12-views/mounts.txt`. Bless with `BIM_BLESS=1 cargo test … <kind>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const ART = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json";
const ref = (def: string) => ({ $ref: `${ART}#/$defs/${def}` });
const assigned = (inner: unknown) => ({ type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [inner, { type: "null" }] } } });
const id = (role: "target" | "identity", label: Label, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind: "view" }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const prop = (name: string, rust: string, schema: unknown, widget: string, label: Label, order: number, refKind?: string): Prop => ({
  name,
  rust,
  schema: schema as Record<string, unknown>,
  ui: { widget, role: "value", label, group: "value", order, ...(refKind ? { ref: { kind: refKind } } : {}) },
});

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const P = F.P;

//#region 🔖️Fixtures
const view = (over: Record<string, unknown>) => ({ building: "bldg-1", name: "View", kind: "Plan", depth: 100, hidden: [], scale: 100, detail: "Medium", ...over });
const plan = (name: string, storey: string, over: Record<string, unknown> = {}) => view({ name, kind: "Plan", storey, ...over });
const plane = (a: [number, number], b: [number, number]) => ({ start: P(...a), end: P(...b) });
const section = (name: string, over: Record<string, unknown> = {}) => view({ name, kind: "Section", plane: plane([0, 3], [8, 3]), ...over });
const elevation = (name: string, over: Record<string, unknown> = {}) => view({ name, kind: "Elevation", plane: plane([8, -3], [0, -3]), ...over });
const camera = { target: P(4, 3), target_height: 1.5, azimuth: 0.8, pitch: 0.5, distance: 25 };
const orbit = (name: string, over: Record<string, unknown> = {}) => view({ name, kind: "Orthographic", camera, ...over });
const crop = (x0: number, y0: number, x1: number, y1: number) => ({ min: P(x0, y0), max: P(x1, y1) });

const scene = (views: Record<string, unknown>, extra: Record<string, unknown> = {}) => ({ ...F.scene(), ...extra, views });
const base = () => scene({ "v-ground": plan("Ground plan", "st-ground") });
const rich = () => scene({ "v-ground": plan("Ground plan", "st-ground"), "v-section": section("Section A"), "v-orbit": orbit("Iso") });
const annex = () => {
  const snapshot: any = base();
  snapshot.buildings["bldg-2"] = F.building("site-1", "Annex");
  snapshot.storeys["st-annex"] = F.storey("bldg-2", "Annex ground", 0, 3);
  return snapshot;
};
const withData = () => {
  const snapshot: any = rich();
  snapshot.properties = { "v-section": { Pset_View: { Sheet: { Text: { value: "A-101" } } } } };
  snapshot.classifications = { "v-section": { system: "DIN 276", code: "710", title: "Planung" } };
  return snapshot;
};
//#endregion 🔖️Fixtures

const target = "vec![self.id.clone()]";
const MUTATION_USES = "use crate::{Assigned, DetailLevel, Phase, ViewCamera, ViewCategory, ViewCrop, ViewPatch, ViewPlane};";

const emoji = ["2705", "1f9f2", "1f9f5", "1f9f4", "1f9f3", "1f9f1", "1f9f0", "1f9ef", "1f9ee", "1f9ed", "1f9e9", "1f9e8", "1f9e7", "1f9e6", "1f9e5", "1f9e4", "1f9e3", "1f9e2", "1f9e1", "1f9e0", "1f9df", "1f9de", "1f9dd", "1f9dc", "1f9db", "1f9da", "1f9d9"].map((hex) => parseInt(hex, 16));
const cases = <T extends { name: string; emoji?: number }>(rows: T[]) => rows.map((row, index) => ({ ...row, emoji: emoji[index] }));

export const leaves: Leaf[] = [
  {
    kind: "create-view", emoji: 0x1f4fd, variant: "CreateView", verb: "create", entity: "view", binaryTag: 12000, displayName: "Create View",
    doc: "Brings a new view into a building: a plan or ceiling plan of one of its storeys, a section or elevation through a vertical plane, or a camera. Each kind owns exactly its fields; the linework it draws is inferred.",
    props: [id("identity", { en: "View id", de: "Ansichts-Id" }), prop("view", "View", ref("View"), "record", { en: "View", de: "Ansicht" }, 20)],
    label: { en: 'format!("Create view \\"{}\\"", self.view.name)', de: 'format!("Ansicht \\"{}\\" anlegen", self.view.name)' },
    target,
    inverseRows: undefined,
    cases: cases([
      { name: "adds-a-plan", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first") }, outcome: ok },
      { name: "adds-a-ceiling-plan", before: base(), mutation: { id: "v-ceiling", view: view({ name: "Ground ceiling", kind: "CeilingPlan", storey: "st-ground", cut_height: 2.4 }) }, outcome: ok },
      { name: "adds-a-section", before: base(), mutation: { id: "v-section", view: section("Section A") }, outcome: ok },
      { name: "adds-an-elevation", before: base(), mutation: { id: "v-south", view: elevation("South", { depth: 40 }) }, outcome: ok },
      { name: "adds-a-camera", before: base(), mutation: { id: "v-eye", view: view({ name: "Eye level", kind: "Perspective", camera }) }, outcome: ok },
      { name: "adds-a-configured-plan", before: base(), mutation: { id: "v-detail", view: plan("Detail plan", "st-first", { cut_height: 0.9, depth: 3, crop: crop(0, 0, 6, 4), hidden: ["Beams", "Grids"], phase: "New", scale: 50, detail: "Fine" }) }, outcome: ok },
      { name: "duplicate-id", before: base(), mutation: { id: "v-ground", view: plan("First plan", "st-first") }, outcome: reject("mutation.duplicate-id", ["v-ground"]) },
      { name: "id-taken-by-another-kind", before: base(), mutation: { id: "w-south", view: plan("First plan", "st-first") }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "building-missing", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first", { building: "bldg-9" }) }, outcome: reject("mutation.target-missing", ["view", "building"]) },
      { name: "storey-missing", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-attic") }, outcome: reject("mutation.target-missing", ["view", "storey"]) },
      { name: "storey-of-another-building", before: annex(), mutation: { id: "v-first", view: plan("First plan", "st-annex") }, outcome: reject("mutation.invariant", ["view", "storey"]) },
      { name: "plan-without-storey", before: base(), mutation: { id: "v-first", view: view({ name: "First plan", kind: "Plan" }) }, outcome: reject("mutation.invariant", ["view", "storey"]) },
      { name: "section-without-plane", before: base(), mutation: { id: "v-section", view: view({ name: "Section A", kind: "Section" }) }, outcome: reject("mutation.invariant", ["view", "plane"]) },
      { name: "plane-without-length", before: base(), mutation: { id: "v-section", view: section("Section A", { plane: plane([2, 2], [2, 2]) }) }, outcome: reject("mutation.invariant", ["view", "plane"]) },
      { name: "camera-without-camera", before: base(), mutation: { id: "v-eye", view: view({ name: "Eye level", kind: "Perspective" }) }, outcome: reject("mutation.invariant", ["view", "camera"]) },
      { name: "blank-name", before: base(), mutation: { id: "v-first", view: plan(" ", "st-first") }, outcome: reject("mutation.invariant", ["view", "name"]) },
      { name: "name-taken", before: base(), mutation: { id: "v-first", view: plan("Ground plan", "st-first") }, outcome: reject("mutation.invariant", ["view", "name"]) },
      { name: "depth-zero", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first", { depth: 0 }) }, outcome: reject("mutation.invariant", ["view", "depth"]) },
      { name: "unordered-hidden", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first", { hidden: ["Beams", "Walls"] }) }, outcome: reject("mutation.invariant", ["view", "hidden"]) },
      { name: "scale-zero", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first", { scale: 0 }) }, outcome: reject("mutation.invariant", ["view", "scale"]) },
      { name: "empty-crop", before: base(), mutation: { id: "v-first", view: plan("First plan", "st-first", { crop: crop(2, 0, 1, 4) }) }, outcome: reject("mutation.invariant", ["view", "crop"]) },
      { name: "cut-height-on-a-section", before: base(), mutation: { id: "v-section", view: section("Section A", { cut_height: 1 }) }, outcome: reject("mutation.invariant", ["view", "cut_height"]) },
    ]),
  },
  {
    kind: "set-view", emoji: 0x1f52d, variant: "SetView", verb: "set", entity: "view", binaryTag: 12001, displayName: "Set View",
    doc: "Sparsely changes a view: name, storey of a plan, plane of a section or elevation, camera, cut height (an assigned null returns to the convention), depth, crop (an assigned null clears it), hidden categories, phase filter (an assigned null shows every phase), scale and detail level. The kind and the building of a view never change.",
    props: [
      id("target", { en: "View", de: "Ansicht" }),
      prop("name", "Option<String>", { type: "string" }, "text", { en: "Name", de: "Name" }, 20),
      prop("storey", "Option<String>", { type: "string" }, "reference", { en: "Storey", de: "Geschoss" }, 30, "storey"),
      prop("plane", "Option<ViewPlane>", ref("ViewPlane"), "record", { en: "Plane", de: "Ebene" }, 40),
      prop("camera", "Option<ViewCamera>", ref("ViewCamera"), "record", { en: "Camera", de: "Kamera" }, 50),
      prop("cut_height", "Option<Assigned<Option<f64>>>", assigned({ type: "number" }), "number", { en: "Cut height (m, empty = convention)", de: "Schnitthöhe (m, leer = Vorgabe)" }, 60),
      prop("depth", "Option<f64>", { type: "number" }, "number", { en: "Depth (m)", de: "Tiefe (m)" }, 70),
      prop("crop", "Option<Assigned<Option<ViewCrop>>>", assigned(ref("ViewCrop")), "record", { en: "Crop (empty = none)", de: "Zuschnitt (leer = keiner)" }, 80),
      prop("hidden", "Option<Vec<ViewCategory>>", { type: "array", items: ref("ViewCategory") }, "list", { en: "Hidden categories", de: "Ausgeblendete Kategorien" }, 90),
      prop("phase", "Option<Assigned<Option<Phase>>>", assigned(ref("Phase")), "select", { en: "Phase filter (empty = all)", de: "Phasenfilter (leer = alle)" }, 100),
      prop("scale", "Option<u32>", { type: "integer", minimum: 1, maximum: 10000 }, "integer", { en: "Scale 1:n", de: "Massstab 1:n" }, 110),
      prop("detail", "Option<DetailLevel>", ref("DetailLevel"), "select", { en: "Detail level", de: "Detailgrad" }, 120),
    ],
    uses: [MUTATION_USES],
    label: { en: 'format!("Change view \\"{}\\"", self.id)', de: 'format!("Ansicht \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "renames", before: rich(), mutation: { id: "v-ground", name: "Entrance plan" }, outcome: ok },
      { name: "moves-the-plane", before: rich(), mutation: { id: "v-section", plane: plane([0, 4], [8, 4]) }, outcome: ok },
      { name: "retargets-the-plan", before: rich(), mutation: { id: "v-ground", storey: "st-first" }, outcome: ok },
      { name: "cuts-lower", before: rich(), mutation: { id: "v-ground", cut_height: { value: 0.9 } }, outcome: ok },
      { name: "clears-the-cut", before: scene({ "v-ground": plan("Ground plan", "st-ground", { cut_height: 0.9 }) }), mutation: { id: "v-ground", cut_height: { value: null } }, outcome: ok },
      { name: "crops", before: rich(), mutation: { id: "v-ground", crop: { value: crop(0, 0, 6, 4) } }, outcome: ok },
      { name: "clears-the-crop", before: scene({ "v-ground": plan("Ground plan", "st-ground", { crop: crop(0, 0, 6, 4) }) }), mutation: { id: "v-ground", crop: { value: null } }, outcome: ok },
      { name: "hides-categories", before: rich(), mutation: { id: "v-section", hidden: ["Beams", "Grids"] }, outcome: ok },
      { name: "filters-by-phase", before: rich(), mutation: { id: "v-ground", phase: { value: "New" } }, outcome: ok },
      { name: "clears-the-phase", before: scene({ "v-ground": plan("Ground plan", "st-ground", { phase: "New" }) }), mutation: { id: "v-ground", phase: { value: null } }, outcome: ok },
      { name: "rescales", before: rich(), mutation: { id: "v-section", scale: 50, detail: "Fine", depth: 5 }, outcome: ok },
      { name: "orbits", before: rich(), mutation: { id: "v-orbit", camera: { ...camera, azimuth: 1.2 } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: rich(), mutation: { id: "v-section", name: "Section A", depth: 7 }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "v-gone", name: "Gone" }, outcome: reject("mutation.target-missing", ["v-gone"]) },
      { name: "unchanged", before: rich(), mutation: { id: "v-section", name: "Section A", scale: 100 }, outcome: reject("mutation.no-op", ["v-section"]) },
      { name: "empty-patch", before: rich(), mutation: { id: "v-section" }, outcome: reject("mutation.no-op", ["v-section"]) },
      { name: "name-taken", before: rich(), mutation: { id: "v-section", name: "Ground plan" }, outcome: reject("mutation.invariant", ["name"]) },
      { name: "plane-on-a-plan", before: rich(), mutation: { id: "v-ground", plane: plane([0, 0], [4, 0]) }, outcome: reject("mutation.invariant", ["plane"]) },
      { name: "camera-on-a-section", before: rich(), mutation: { id: "v-section", camera }, outcome: reject("mutation.invariant", ["camera"]) },
      { name: "storey-on-a-section", before: rich(), mutation: { id: "v-section", storey: "st-ground" }, outcome: reject("mutation.invariant", ["storey"]) },
      { name: "storey-missing", before: rich(), mutation: { id: "v-ground", storey: "st-attic" }, outcome: reject("mutation.target-missing", ["storey"]) },
      { name: "storey-of-another-building", before: (() => { const snapshot: any = annex(); snapshot.views["v-section"] = section("Section A"); return snapshot; })(), mutation: { id: "v-ground", storey: "st-annex" }, outcome: reject("mutation.invariant", ["storey"]) },
      { name: "depth-zero", before: rich(), mutation: { id: "v-section", depth: 0 }, outcome: reject("mutation.invariant", ["depth"]) },
      { name: "scale-too-large", before: rich(), mutation: { id: "v-section", scale: 10001 }, outcome: reject("mutation.invariant", ["scale"]) },
      { name: "duplicate-hidden", before: rich(), mutation: { id: "v-section", hidden: ["Walls", "Walls"] }, outcome: reject("mutation.invariant", ["hidden"]) },
      { name: "empty-crop", before: rich(), mutation: { id: "v-ground", crop: { value: crop(1, 1, 1, 4) } }, outcome: reject("mutation.invariant", ["crop"]) },
    ]),
  },
  {
    kind: "delete-view", emoji: 0x1f4fa, variant: "DeleteView", verb: "delete", entity: "view", binaryTag: 12002, displayName: "Delete View",
    doc: "Removes a view together with its properties and classifications. Views are leaves of the model: nothing else depends on them yet.",
    props: [id("target", { en: "View", de: "Ansicht" })],
    inverseRows: { bounded: 4096 },
    label: { en: 'format!("Delete view \\"{}\\"", self.id)', de: 'format!("Ansicht \\"{}\\" löschen", self.id)' },
    target,
    cases: cases([
      { name: "removes", before: rich(), mutation: { id: "v-section" }, outcome: ok },
      { name: "removes-its-data", before: withData(), mutation: { id: "v-section" }, outcome: ok },
      { name: "missing", before: rich(), mutation: { id: "v-gone" }, outcome: reject("mutation.target-missing", ["v-gone"]) },
    ]),
  },
];

const OPTIONAL: Record<string, string[]> = { "set-view": ["name", "storey", "plane", "camera", "cut_height", "depth", "crop", "hidden", "phase", "scale", "detail"] };

const MODULES: Record<string, { diff: string; inverse: string; mutation?: (source: string) => string }> = {
  "create-view": {
    diff: `//! 🔺️ Diff constructor for \`CreateView\`: one created view entry. The id must be free across every collection, then the view must be writable: its building exists, its
//! name is filled and unique within the building, each kind owns exactly its fields (see \`view_problem\`), the numbers are positive and finite and the hidden categories are
//! unique and in category order.

use super::super::elements;
use super::CreateView;
use crate::{view_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = view_problem(base, &payload.id, &payload.view) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["view", problem.field]);
    }
    MutationOutcome::new(ModelDiff::views(payload.id.clone(), Entry::Created(payload.view.clone())))
}
`,
    inverse: `//! ↩️ Inverse of \`CreateView\`: the concrete \`DeleteView\` of the id it created, none when the id was already taken.

use super::super::delete_view::DeleteView;
use super::CreateView;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateView, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.views.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteView(DeleteView { id: payload.id.clone() })]
}
`,
  },
  "set-view": {
    diff: `//! 🔺️ Diff constructor for \`SetView\`: a sparse view patch of exactly the provided fields that differ. The view that results must be writable (see \`view_problem\`): a kind
//! owns exactly its fields, so a plane on a plan, a camera on a section or a storey on an elevation is refused, a new storey must exist in the building of the view, the name
//! stays unique and the numbers stay positive and finite. A patch that restates the current values is a \`mutation.no-op\`.

use super::SetView;
use crate::{view_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(view) = base.views.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("View \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(view);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("View \\"{}\\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = view_problem(base, &payload.id, &patch.write(view)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::views(payload.id.clone(), Entry::Patched(patch)))
}
`,
    inverse: `//! ↩️ Inverse of \`SetView\`: an absolute \`SetView\` restoring the base value of exactly the fields the forward really changes, none when the view is absent or nothing changes.

use super::SetView;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetView, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.views.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).negate(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetView(SetView::from_patch(payload.id.clone(), restore))]
}
`,
    mutation: (source) =>
      source.replace(
        "impl MutationKind<ModelSnapshot, ModelMutation> for SetView {",
        `impl SetView {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ViewPatch {
        ViewPatch {
            name: self.name.clone(),
            storey: self.storey.clone().map(|storey| Assigned::new(Some(storey))),
            plane: self.plane.map(|plane| Assigned::new(Some(plane))),
            camera: self.camera.map(|camera| Assigned::new(Some(camera))),
            cut_height: self.cut_height.clone(),
            depth: self.depth,
            crop: self.crop.clone(),
            hidden: self.hidden.clone(),
            phase: self.phase.clone(),
            scale: self.scale,
            detail: self.detail,
            ..Default::default()
        }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(id: String, patch: ViewPatch) -> Self {
        Self {
            id,
            name: patch.name,
            storey: patch.storey.and_then(|storey| storey.value),
            plane: patch.plane.and_then(|plane| plane.value),
            camera: patch.camera.and_then(|camera| camera.value),
            cut_height: patch.cut_height,
            depth: patch.depth,
            crop: patch.crop,
            hidden: patch.hidden,
            phase: patch.phase,
            scale: patch.scale,
            detail: patch.detail,
        }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetView {`,
      ),
  },
  "delete-view": {
    diff: `//! 🔺️ Diff constructor for \`DeleteView\`: the view leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the view leaves.

use super::super::cascade;
use super::DeleteView;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.views.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("View \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "View", Some(&payload.id))
}
`,
    inverse: `//! ↩️ Inverse of \`DeleteView\`: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::DeleteView;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteView, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
};

const dir = (spec: Leaf) => join(mutations, em(spec.emoji) + spec.kind);
const sub = (spec: Leaf, emojiPoint: number, name: string) => join(dir(spec), em(emojiPoint) + name);

const fixup = (spec: Leaf) => {
  const optional = OPTIONAL[spec.kind] ?? [];
  const component = join(sub(spec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  for (const name of optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  source = MODULES[spec.kind].mutation?.(source) ?? source;
  writeFileSync(component, source);
  const schemaFile = join(sub(spec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  writeFileSync(join(sub(spec, 0x1f53a, "diff"), RS), MODULES[spec.kind].diff);
  writeFileSync(join(sub(spec, 0x21a9, "inverse"), RS), MODULES[spec.kind].inverse);
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "w12-views");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec);
    fixup(spec);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);
  console.log(`emitted ${leaves.length} leaves`);
}
