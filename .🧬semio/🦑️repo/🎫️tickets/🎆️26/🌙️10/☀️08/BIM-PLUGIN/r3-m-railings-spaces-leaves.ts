#!/usr/bin/env bun
/**
 * 🧪️ Wave M slice 8 (`m-railings-spaces`): the six leaves create/delete/set-railing and create/delete/set-space of `s.bim.model@1`
 * (binary tags 800..805). `bun r3-m-railings-spaces-leaves.ts` rewrites their boilerplate, fixtures and the two logic files of each
 * leaf (`🔺️diff`, `↩️inverse`), mounts the leaves in the artifact root once, and prints the enum rows to add to the aggregate.
 * `after`/`diff` fixtures of applied cases are never overwritten: bless them with `BIM_BLESS=1 cargo test`.
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { artifact, em, fixtures, JSONF, mutations, RS } from "./r3-f1-paths.ts";

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
const num = (name: string, label: Label, order: number): Prop => ({ name, rust: "Option<f64>", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order } });
const text = (name: string, label: Label, order: number, group = "identity"): Prop => ({ name, rust: "Option<String>", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group, order } });
const materialRef = (order: number): Prop => ({ name: "material", rust: "Option<String>", schema: { type: "string" }, ui: { widget: "reference", role: "value", label: { en: "Material", de: "Material" }, ref: { kind: "material" }, group: "value", order } });
const pathProp = (order: number): Prop => ({ name: "path", rust: "Option<Vec<Point2>>", schema: { type: "array", items: record("Point2") }, ui: { widget: "record", role: "value", label: { en: "Path", de: "Verlauf" }, group: "value", order } });
const boundaryProp = (order: number): Prop => ({ name: "boundary", rust: "Option<SpaceBoundary>", schema: record("SpaceBoundary"), ui: { widget: "record", role: "value", label: { en: "Boundary", de: "Begrenzung" }, group: "value", order } });

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";

const P = F.P;
const pts = (list: [number, number][]) => list.map(([x, y]) => P(x, y));
const railing = (storey: string, list: [number, number][], over: Record<string, unknown> = {}) => ({ storey, path: pts(list), height: 1, post_spacing: 1.2, material: "m-steel", base_offset: 0, name: "Balustrade", ...over });
const bounded = (x: number, y: number) => ({ Bounded: { seed: P(x, y) } });
const explicit = (list: [number, number][], bulge = 0) => ({ Explicit: { outline: list.map(([x, y]) => ({ point: P(x, y), bulge })) } });
const space = (storey: string, number: string, name: string, boundary: unknown, usage = "Living") => ({ storey, number, name, boundary, usage });

const base = (extra: { railings?: Record<string, unknown>; spaces?: Record<string, unknown> } = {}) => {
  const snapshot: any = F.scene();
  snapshot.materials["m-steel"] = F.material("Steel");
  snapshot.railings = extra.railings ?? {};
  snapshot.spaces = extra.spaces ?? {};
  return snapshot;
};
const WITH_RAILING = () => base({ railings: { "rl-1": railing("st-first", [[0, 0], [4, 0], [4, 3]]) } });
const WITH_SPACES = () =>
  base({
    spaces: {
      "sp-1": space("st-ground", "0.01", "Kitchen", bounded(2, 2), "Kitchen"),
      "sp-2": space("st-ground", "0.02", "Hall", explicit([[0, 0], [3, 0], [3, 2], [0, 2]]), "Circulation"),
      "sp-3": space("st-first", "1.01", "Bedroom", bounded(5, 3), "Sleeping"),
    },
  });

type Extra = { imports: string[]; optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: Leaf, extra: Extra): Leaf => (extras.set(spec.kind, extra), spec);

export const leaves: Leaf[] = [
  leaf(
    {
      kind: "create-railing", emoji: 0x1f6e4, variant: "CreateRailing", verb: "create", entity: "railing", doc: "Brings a new railing onto a storey; its posts, rails and extent are inferred from the authored path, height and post spacing.", displayName: "Create Railing", binaryTag: 800,
      props: [id("railing", "identity", { en: "Railing id", de: "Geländer-Id" }, 10), recordProp("railing", "Railing", { en: "Railing", de: "Geländer" })],
      label: { en: 'format!("Create railing \\"{}\\"", self.railing.name)', de: 'format!("Geländer \\"{}\\" anlegen", self.railing.name)' },
      target,
      cases: [
        { name: "adds", emoji: 0x2705, before: base(), mutation: { id: "rl-1", railing: railing("st-first", [[0, 0], [4, 0], [4, 3]]) }, outcome: ok },
        { name: "duplicate", emoji: 0x1f6ab, before: WITH_RAILING(), mutation: { id: "rl-1", railing: railing("st-ground", [[0, 0], [2, 0]]) }, outcome: reject("mutation.duplicate-id", ["rl-1"]) },
        { name: "storey-missing", emoji: 0x26d4, before: base(), mutation: { id: "rl-1", railing: railing("st-attic", [[0, 0], [2, 0]]) }, outcome: reject("mutation.target-missing", ["railing", "storey"]) },
        { name: "material-missing", emoji: 0x1f9f1, before: base(), mutation: { id: "rl-1", railing: railing("st-first", [[0, 0], [2, 0]], { material: "m-glass" }) }, outcome: reject("mutation.target-missing", ["railing", "material"]) },
        { name: "path-too-short", emoji: 0x1f4cf, before: base(), mutation: { id: "rl-1", railing: railing("st-first", [[1, 1]]) }, outcome: reject("mutation.invariant", ["railing", "path"]) },
        { name: "non-positive-height", emoji: 0x1f4a5, before: base(), mutation: { id: "rl-1", railing: railing("st-first", [[0, 0], [2, 0]], { height: 0 }) }, outcome: reject("mutation.invariant", ["railing", "height"]) },
        { name: "non-positive-post-spacing", emoji: 0x1f6a7, before: base(), mutation: { id: "rl-1", railing: railing("st-first", [[0, 0], [2, 0]], { post_spacing: -1 }) }, outcome: reject("mutation.invariant", ["railing", "post_spacing"]) },
      ],
    },
    { imports: ["Railing"], optional: [] },
  ),
  leaf(
    {
      kind: "delete-railing", emoji: 0x1fa9a, variant: "DeleteRailing", verb: "delete", entity: "railing", doc: "Removes a railing; nothing depends on it.", displayName: "Delete Railing", binaryTag: 801,
      props: [id("railing", "target", { en: "Railing", de: "Geländer" })],
      label: { en: 'format!("Delete railing \\"{}\\"", self.id)', de: 'format!("Geländer \\"{}\\" löschen", self.id)' },
      target,
      cases: [
        { name: "removes", emoji: 0x2705, before: WITH_RAILING(), mutation: { id: "rl-1" }, outcome: ok },
        { name: "missing", emoji: 0x1f6ab, before: base(), mutation: { id: "rl-1" }, outcome: reject("mutation.target-missing", ["rl-1"]) },
      ],
    },
    { imports: [], optional: [] },
  ),
  leaf(
    {
      kind: "set-railing", emoji: 0x1f527, variant: "SetRailing", verb: "set", entity: "railing", doc: "Sets any of a railing's authored path, height, post spacing, material, base offset and name; absent fields stay untouched.", displayName: "Set Railing", binaryTag: 802,
      props: [
        id("railing", "target", { en: "Railing", de: "Geländer" }),
        pathProp(20),
        num("height", { en: "Height (m)", de: "Höhe (m)" }, 30),
        num("post_spacing", { en: "Post spacing (m)", de: "Pfostenabstand (m)" }, 40),
        materialRef(50),
        num("base_offset", { en: "Base offset (m)", de: "Fußversatz (m)" }, 60),
        text("name", { en: "Name", de: "Name" }, 70),
      ],
      label: { en: 'format!("Edit railing \\"{}\\"", self.id)', de: 'format!("Geländer \\"{}\\" ändern", self.id)' },
      target,
      cases: [
        { name: "reshapes", emoji: 0x2705, before: WITH_RAILING(), mutation: { id: "rl-1", path: pts([[0, 0], [6, 0]]), height: 1.1, material: "m-brick" }, outcome: ok },
        { name: "renames-only", emoji: 0x1f3f7, before: WITH_RAILING(), mutation: { id: "rl-1", name: "Stair Guard" }, outcome: ok },
        { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_RAILING(), mutation: { id: "rl-1", height: 1, name: "Balustrade" }, outcome: reject("mutation.no-op", ["rl-1"]) },
        { name: "missing", emoji: 0x26d4, before: base(), mutation: { id: "rl-1", height: 1.1 }, outcome: reject("mutation.target-missing", ["rl-1"]) },
        { name: "path-too-short", emoji: 0x1f4cf, before: WITH_RAILING(), mutation: { id: "rl-1", path: pts([[2, 2]]) }, outcome: reject("mutation.invariant", ["path"]) },
        { name: "non-positive-height", emoji: 0x1f4a5, before: WITH_RAILING(), mutation: { id: "rl-1", height: -0.5 }, outcome: reject("mutation.invariant", ["height"]) },
        { name: "non-positive-post-spacing", emoji: 0x1f6a7, before: WITH_RAILING(), mutation: { id: "rl-1", post_spacing: 0 }, outcome: reject("mutation.invariant", ["post_spacing"]) },
        { name: "material-missing", emoji: 0x1f9f1, before: WITH_RAILING(), mutation: { id: "rl-1", material: "m-glass" }, outcome: reject("mutation.target-missing", ["material"]) },
      ],
    },
    { imports: ["Point2"], optional: ["path", "height", "post_spacing", "material", "base_offset", "name"] },
  ),
  leaf(
    {
      kind: "create-space", emoji: 0x1f6cb, variant: "CreateSpace", verb: "create", entity: "space", doc: "Brings a new room onto a storey; a bounded space stores only its seed point, its outline, area and volume are inferred from the walls around it.", displayName: "Create Space", binaryTag: 803,
      props: [id("space", "identity", { en: "Space id", de: "Raum-Id" }, 10), recordProp("space", "Space", { en: "Space", de: "Raum" })],
      label: { en: 'format!("Create space \\"{}\\"", self.space.name)', de: 'format!("Raum \\"{}\\" anlegen", self.space.name)' },
      target,
      cases: [
        { name: "adds-a-bounded-space", emoji: 0x2705, before: base(), mutation: { id: "sp-1", space: space("st-ground", "0.01", "Kitchen", bounded(2, 2), "Kitchen") }, outcome: ok },
        { name: "adds-an-explicit-space", emoji: 0x1f4d0, before: base(), mutation: { id: "sp-2", space: space("st-ground", "0.02", "Hall", explicit([[0, 0], [3, 0], [3, 2], [0, 2]]), "Circulation") }, outcome: ok },
        { name: "reuses-a-number-on-another-storey", emoji: 0x1f501, before: WITH_SPACES(), mutation: { id: "sp-4", space: space("st-first", "0.01", "Study", bounded(1, 1), "Office") }, outcome: ok },
        { name: "duplicate", emoji: 0x1f6ab, before: WITH_SPACES(), mutation: { id: "sp-1", space: space("st-ground", "0.09", "Other", bounded(1, 1)) }, outcome: reject("mutation.duplicate-id", ["sp-1"]) },
        { name: "storey-missing", emoji: 0x26d4, before: base(), mutation: { id: "sp-1", space: space("st-attic", "2.01", "Loft", bounded(1, 1)) }, outcome: reject("mutation.target-missing", ["space", "storey"]) },
        { name: "number-taken", emoji: 0x1f522, before: WITH_SPACES(), mutation: { id: "sp-4", space: space("st-ground", "0.01", "Pantry", bounded(1, 1)) }, outcome: reject("mutation.invariant", ["space", "number"]) },
        { name: "outline-degenerate", emoji: 0x1f4a5, before: base(), mutation: { id: "sp-1", space: space("st-ground", "0.01", "Sliver", explicit([[0, 0], [2, 0], [4, 0]])) }, outcome: reject("mutation.invariant", ["space", "boundary"]) },
      ],
    },
    { imports: ["Space"], optional: [] },
  ),
  leaf(
    {
      kind: "delete-space", emoji: 0x1f9fd, variant: "DeleteSpace", verb: "delete", entity: "space", doc: "Removes a room; nothing depends on it.", displayName: "Delete Space", binaryTag: 804,
      props: [id("space", "target", { en: "Space", de: "Raum" })],
      label: { en: 'format!("Delete space \\"{}\\"", self.id)', de: 'format!("Raum \\"{}\\" löschen", self.id)' },
      target,
      cases: [
        { name: "removes", emoji: 0x2705, before: WITH_SPACES(), mutation: { id: "sp-2" }, outcome: ok },
        { name: "missing", emoji: 0x1f6ab, before: base(), mutation: { id: "sp-2" }, outcome: reject("mutation.target-missing", ["sp-2"]) },
      ],
    },
    { imports: [], optional: [] },
  ),
  leaf(
    {
      kind: "set-space", emoji: 0x1fa91, variant: "SetSpace", verb: "set", entity: "space", doc: "Sets any of a room's number, name, boundary and usage; absent fields stay untouched and the number stays unique within the storey.", displayName: "Set Space", binaryTag: 805,
      props: [
        id("space", "target", { en: "Space", de: "Raum" }),
        text("number", { en: "Number", de: "Nummer" }, 20),
        text("name", { en: "Name", de: "Name" }, 30),
        boundaryProp(40),
        text("usage", { en: "Usage", de: "Nutzung" }, 50, "value"),
      ],
      label: { en: 'format!("Edit space \\"{}\\"", self.id)', de: 'format!("Raum \\"{}\\" ändern", self.id)' },
      target,
      cases: [
        { name: "renames-and-retypes", emoji: 0x2705, before: WITH_SPACES(), mutation: { id: "sp-1", name: "Cooking Area", usage: "Dining" }, outcome: ok },
        { name: "redraws-the-boundary", emoji: 0x1f4d0, before: WITH_SPACES(), mutation: { id: "sp-1", boundary: explicit([[0, 0], [4, 0], [4, 3], [0, 3]]) }, outcome: ok },
        { name: "renumbers", emoji: 0x1f522, before: WITH_SPACES(), mutation: { id: "sp-3", number: "1.02" }, outcome: ok },
        { name: "nothing-to-change", emoji: 0x1f4a4, before: WITH_SPACES(), mutation: { id: "sp-1", number: "0.01", usage: "Kitchen" }, outcome: reject("mutation.no-op", ["sp-1"]) },
        { name: "missing", emoji: 0x26d4, before: base(), mutation: { id: "sp-1", name: "Void" }, outcome: reject("mutation.target-missing", ["sp-1"]) },
        { name: "number-taken", emoji: 0x1f6ab, before: WITH_SPACES(), mutation: { id: "sp-1", number: "0.02" }, outcome: reject("mutation.invariant", ["number"]) },
        { name: "outline-degenerate", emoji: 0x1f4a5, before: WITH_SPACES(), mutation: { id: "sp-1", boundary: explicit([[0, 0], [1, 1]]) }, outcome: reject("mutation.invariant", ["boundary"]) },
      ],
    },
    { imports: ["SpaceBoundary"], optional: ["number", "name", "boundary", "usage"] },
  ),
];

const dir = (leafSpec: Leaf) => join(mutations, em(leafSpec.emoji) + leafSpec.kind);
const sub = (leafSpec: Leaf, emoji: number, name: string) => join(dir(leafSpec), em(emoji) + name);

const fixup = (leafSpec: Leaf) => {
  const extra = extras.get(leafSpec.kind)!;
  const component = join(sub(leafSpec, 0x1f9a0, "mutation"), RS);
  let source = readFileSync(component, "utf8");
  const imports = ["ModelDiff", "ModelMutation", "ModelSnapshot", ...leafSpec.props.map((prop) => prop.rust).filter((name) => /^[A-Z][A-Za-z0-9]*$/.test(name) && name !== "String"), ...extra.imports];
  source = source.replace(/use crate::\{[^}]*\};/, `use crate::{${[...new Set(imports)].sort().join(", ")}};`);
  for (const name of extra.optional) source = source.replace(new RegExp(`^    pub ${name}: Option<`, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: Option<`);
  writeFileSync(component, source);

  const schemaFile = join(sub(leafSpec, 0x1f9ec, "schema"), JSONF);
  const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
  schema.required = schema.required.filter((name: string) => !extra.optional.includes(name));
  writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
};

const MODULES: Record<string, { diff: string; inverse: string }> = {};

MODULES["create-railing"] = {
  diff: `//! 🔺️ Diff constructor for \`CreateRailing\`: one created railing entry. The storey and the material must exist, the path needs at
//! least two finite points that are not all the same, height and post spacing are positive lengths. Posts, rails and extent are inferred.

use super::CreateRailing;
use crate::{Entry, ModelDiff, ModelSnapshot, Point2};
use protocol::{MutationOutcome, OutcomeCode};

fn traceable(path: &[Point2]) -> bool {
    path.len() >= 2 && path.iter().all(|point| point.x.is_finite() && point.y.is_finite()) && path.windows(2).any(|pair| pair[0] != pair[1])
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

pub fn diff(payload: &CreateRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let railing = &payload.railing;
    if base.railings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Railing \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&railing.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \\"{}\\" does not exist.", railing.storey), ["railing", "storey"]);
    }
    if !base.materials.contains_key(&railing.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \\"{}\\" does not exist.", railing.material), ["railing", "material"]);
    }
    if !traceable(&railing.path) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing path needs at least two distinct finite points.", ["railing", "path"]);
    }
    if !positive(railing.height) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing height must be a positive length.", ["railing", "height"]);
    }
    if !positive(railing.post_spacing) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing post spacing must be a positive length.", ["railing", "post_spacing"]);
    }
    if !railing.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing base offset must be a finite length.", ["railing", "base_offset"]);
    }
    MutationOutcome::new(ModelDiff::railings(payload.id.clone(), Entry::Created(railing.clone())))
}
`,
  inverse: `//! ↩️ Inverse of \`CreateRailing\`: the concrete \`DeleteRailing\` of the id it created, none when the id was already taken.

use super::super::delete_railing::DeleteRailing;
use super::CreateRailing;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRailing, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.railings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRailing(DeleteRailing { id: payload.id.clone() })]
}
`,
};

MODULES["delete-railing"] = {
  diff: `//! 🔺️ Diff constructor for \`DeleteRailing\`: one deleted railing entry. Nothing references a railing, so nothing blocks or cascades.

use super::DeleteRailing;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.railings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Railing \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::railings(payload.id.clone(), Entry::Deleted))
}
`,
  inverse: `//! ↩️ Inverse of \`DeleteRailing\`: the concrete \`CreateRailing\` carrying the full removed record, none when the railing was absent.

use super::super::create_railing::CreateRailing;
use super::DeleteRailing;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteRailing, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.railings.get(&payload.id) {
        Some(railing) => vec![ModelMutation::CreateRailing(CreateRailing { id: payload.id.clone(), railing: railing.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-railing"] = {
  diff: `//! 🔺️ Diff constructor for \`SetRailing\`: a sparse railing patch of exactly the provided fields. The path needs two distinct finite
//! points, height and post spacing are positive, the material must exist; providing only equal values is a no-op.

use super::SetRailing;
use crate::{Entry, ModelDiff, ModelSnapshot, Point2, RailingPatch};
use protocol::{MutationOutcome, OutcomeCode};

fn traceable(path: &[Point2]) -> bool {
    path.len() >= 2 && path.iter().all(|point| point.x.is_finite() && point.y.is_finite()) && path.windows(2).any(|pair| pair[0] != pair[1])
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

pub fn diff(payload: &SetRailing, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(railing) = base.railings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Railing \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.path.as_deref().is_some_and(|path| !traceable(path)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing path needs at least two distinct finite points.", ["path"]);
    }
    if payload.height.is_some_and(|height| !positive(height)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing height must be a positive length.", ["height"]);
    }
    if payload.post_spacing.is_some_and(|spacing| !positive(spacing)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing post spacing must be a positive length.", ["post_spacing"]);
    }
    if payload.base_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A railing base offset must be a finite length.", ["base_offset"]);
    }
    if let Some(material) = payload.material.as_ref().filter(|material| !base.materials.contains_key(*material)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \\"{material}\\" does not exist."), ["material"]);
    }
    let unchanged = payload.path.as_ref().is_none_or(|path| *path == railing.path)
        && payload.height.is_none_or(|height| height == railing.height)
        && payload.post_spacing.is_none_or(|spacing| spacing == railing.post_spacing)
        && payload.material.as_ref().is_none_or(|material| *material == railing.material)
        && payload.base_offset.is_none_or(|offset| offset == railing.base_offset)
        && payload.name.as_ref().is_none_or(|name| *name == railing.name);
    if unchanged {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Railing \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::railings(
        payload.id.clone(),
        Entry::Patched(RailingPatch {
            path: payload.path.clone(),
            height: payload.height,
            post_spacing: payload.post_spacing,
            material: payload.material.clone(),
            base_offset: payload.base_offset,
            name: payload.name.clone(),
            ..Default::default()
        }),
    ))
}
`,
  inverse: `//! ↩️ Inverse of \`SetRailing\`: an absolute \`SetRailing\` carrying the base values of exactly the provided fields, none when the
//! railing is absent.

use super::SetRailing;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetRailing, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.railings.get(&payload.id) {
        Some(railing) => vec![ModelMutation::SetRailing(SetRailing {
            id: payload.id.clone(),
            path: payload.path.as_ref().map(|_| railing.path.clone()),
            height: payload.height.map(|_| railing.height),
            post_spacing: payload.post_spacing.map(|_| railing.post_spacing),
            material: payload.material.as_ref().map(|_| railing.material.clone()),
            base_offset: payload.base_offset.map(|_| railing.base_offset),
            name: payload.name.as_ref().map(|_| railing.name.clone()),
        })],
        None => Vec::new(),
    }
}
`,
};

const BOUNDARY_HELPERS = `fn shoelace(outline: &[Vertex]) -> f64 {
    outline.iter().zip(outline.iter().cycle().skip(1)).map(|(from, to)| from.point.x * to.point.y - to.point.x * from.point.y).sum::<f64>() / 2.0
}

fn drawable(boundary: &SpaceBoundary) -> bool {
    match boundary {
        SpaceBoundary::Bounded { seed } => seed.x.is_finite() && seed.y.is_finite(),
        SpaceBoundary::Explicit { outline } => {
            outline.len() >= 3
                && outline.iter().all(|vertex| vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite())
                && (outline.iter().any(|vertex| vertex.bulge != 0.0) || shoelace(outline).abs() > f64::EPSILON)
        }
    }
}
`;

MODULES["create-space"] = {
  diff: `//! 🔺️ Diff constructor for \`CreateSpace\`: one created space entry. The storey must exist, the number is unique within the storey, a
//! bounded space needs a finite seed and an explicit outline at least three finite vertices that enclose area. Outline, area and
//! volume of a bounded space are inferred from the walls around the seed.

use super::CreateSpace;
use crate::{Entry, ModelDiff, ModelSnapshot, SpaceBoundary, Vertex};
use protocol::{MutationOutcome, OutcomeCode};

${BOUNDARY_HELPERS}
pub fn diff(payload: &CreateSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let space = &payload.space;
    if base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Space \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&space.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \\"{}\\" does not exist.", space.storey), ["space", "storey"]);
    }
    if base.spaces.values().any(|row| row.storey == space.storey && row.number == space.number) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Space number \\"{}\\" is already used on storey \\"{}\\".", space.number, space.storey), ["space", "number"]);
    }
    if !drawable(&space.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A space boundary needs a finite seed or an outline of at least three finite vertices enclosing area.", ["space", "boundary"]);
    }
    MutationOutcome::new(ModelDiff::spaces(payload.id.clone(), Entry::Created(space.clone())))
}
`,
  inverse: `//! ↩️ Inverse of \`CreateSpace\`: the concrete \`DeleteSpace\` of the id it created, none when the id was already taken.

use super::super::delete_space::DeleteSpace;
use super::CreateSpace;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSpace, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSpace(DeleteSpace { id: payload.id.clone() })]
}
`,
};

MODULES["delete-space"] = {
  diff: `//! 🔺️ Diff constructor for \`DeleteSpace\`: one deleted space entry. Nothing references a space, so nothing blocks or cascades.

use super::DeleteSpace;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::spaces(payload.id.clone(), Entry::Deleted))
}
`,
  inverse: `//! ↩️ Inverse of \`DeleteSpace\`: the concrete \`CreateSpace\` carrying the full removed record, none when the space was absent.

use super::super::create_space::CreateSpace;
use super::DeleteSpace;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSpace, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.spaces.get(&payload.id) {
        Some(space) => vec![ModelMutation::CreateSpace(CreateSpace { id: payload.id.clone(), space: space.clone() })],
        None => Vec::new(),
    }
}
`,
};

MODULES["set-space"] = {
  diff: `//! 🔺️ Diff constructor for \`SetSpace\`: a sparse space patch of exactly the provided fields. A new number must stay unique within the
//! storey, a new boundary must be drawable (finite seed, or an outline of three finite vertices enclosing area); providing only
//! equal values is a no-op. Outline, area and volume stay inferred.

use super::SetSpace;
use crate::{Entry, ModelDiff, ModelSnapshot, SpaceBoundary, SpacePatch, Vertex};
use protocol::{MutationOutcome, OutcomeCode};

${BOUNDARY_HELPERS}
pub fn diff(payload: &SetSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(space) = base.spaces.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(number) = &payload.number {
        if base.spaces.iter().any(|(other, row)| *other != payload.id && row.storey == space.storey && row.number == *number) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Space number \\"{number}\\" is already used on storey \\"{}\\".", space.storey), ["number"]);
        }
    }
    if payload.boundary.as_ref().is_some_and(|boundary| !drawable(boundary)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A space boundary needs a finite seed or an outline of at least three finite vertices enclosing area.", ["boundary"]);
    }
    let unchanged = payload.number.as_ref().is_none_or(|number| *number == space.number)
        && payload.name.as_ref().is_none_or(|name| *name == space.name)
        && payload.boundary.as_ref().is_none_or(|boundary| *boundary == space.boundary)
        && payload.usage.as_ref().is_none_or(|usage| *usage == space.usage);
    if unchanged {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Space \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::spaces(
        payload.id.clone(),
        Entry::Patched(SpacePatch { number: payload.number.clone(), name: payload.name.clone(), boundary: payload.boundary.clone(), usage: payload.usage.clone(), ..Default::default() }),
    ))
}
`,
  inverse: `//! ↩️ Inverse of \`SetSpace\`: an absolute \`SetSpace\` carrying the base values of exactly the provided fields, none when the space is
//! absent.

use super::SetSpace;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetSpace, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.spaces.get(&payload.id) {
        Some(space) => vec![ModelMutation::SetSpace(SetSpace {
            id: payload.id.clone(),
            number: payload.number.as_ref().map(|_| space.number.clone()),
            name: payload.name.as_ref().map(|_| space.name.clone()),
            boundary: payload.boundary.as_ref().map(|_| space.boundary.clone()),
            usage: payload.usage.as_ref().map(|_| space.usage.clone()),
        })],
        None => Vec::new(),
    }
}
`,
};

const sorted = (value: unknown): unknown =>
  Array.isArray(value) ? value.map(sorted) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value as object).sort(([a], [b]) => (a < b ? -1 : 1)).map(([key, item]) => [key, sorted(item)])) : value;

/** 🧮️ Independent TypeScript twin of the six diffs: writes `after` and `diff` of applied cases that are still unblessed placeholders. `BIM_BLESS=1 cargo test` overwrites them with the Rust output. */
const preBless = (spec: Leaf) => {
  const fixtureRoot = join(fixtures, em(0x1f9ec) + "mutations", em(spec.emoji) + spec.kind);
  const [verb, entity] = spec.kind.split("-") as [string, string];
  const collection = entity === "railing" ? "railings" : "spaces";
  for (const c of spec.cases) {
    if (c.outcome.status !== "applied") continue;
    const caseDir = join(fixtureRoot, em(c.emoji) + c.name);
    const afterFile = join(caseDir, em(0x1f4f8) + "snapshot", em(0x27a1) + "after", JSONF);
    if (readFileSync(afterFile, "utf8").trim() !== "{}") continue;
    const before = structuredClone(c.before) as any;
    const payload = c.mutation as any;
    const rows = before[collection] as Record<string, any>;
    let entry: any;
    if (verb === "create") entry = { entry: "Created", ...payload[entity] };
    else if (verb === "delete") entry = { entry: "Deleted" };
    else {
      const { id: _id, ...fields } = payload;
      entry = { entry: "Patched", ...fields };
    }
    const next = structuredClone(rows);
    if (verb === "create") next[payload.id] = payload[entity];
    else if (verb === "delete") delete next[payload.id];
    else Object.assign(next[payload.id], Object.fromEntries(Object.entries(entry).filter(([key]) => key !== "entry")));
    before[collection] = next;
    writeFileSync(afterFile, JSON.stringify(sorted(before), null, 2) + "\n");
    writeFileSync(join(caseDir, em(0x1f53a) + "diff", JSONF), JSON.stringify(sorted({ [collection]: { [payload.id]: entry } }), null, 2) + "\n");
  }
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "m-railings-spaces");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec);
    fixup(spec);
    const files = MODULES[spec.kind];
    writeFileSync(join(sub(spec, 0x1f53a, "diff"), RS), files.diff);
    writeFileSync(join(sub(spec, 0x21a9, "inverse"), RS), files.inverse);
    preBless(spec);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);

  const root = join(artifact, RS);
  const anchor = "//#endregion 🔖️Leaves";
  const source = readFileSync(root, "utf8");
  if (source.includes("pub mod create_railing {")) console.log("mounts already present");
  else {
    const at = source.indexOf(anchor);
    const lineStart = source.lastIndexOf("\n", at) + 1;
    writeFileSync(root, source.slice(0, lineStart) + mounts + source.slice(lineStart));
    console.log("mounted the leaves");
  }
  console.log(`emitted ${leaves.length} leaves`);
  void existsSync;
}
