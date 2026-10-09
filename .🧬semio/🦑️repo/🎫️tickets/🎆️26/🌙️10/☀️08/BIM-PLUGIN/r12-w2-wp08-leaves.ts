#!/usr/bin/env bun
/**
 * 🧷️ Wave W2 `w2-wp08-walldepth` (binary tags 8000..8003): the four new leaves of the wall depth package of `s.bim.model@1`: `create-wall-sweep`, `set-wall-sweep`, `delete-wall-sweep` and `set-wall-base-slab`.
 * `bun r12-w2-wp08-leaves.ts` rewrites the boilerplate and fixtures through `emitLeaf`, fixes the optional `set-*` fields of the payload structs and schemas, writes the hand logic files once (`🔺️diff`, `↩️inverse`,
 * never overwritten) and mounts the leaves once in the artifact root. `after`/`diff` fixtures of applied cases are never overwritten: bless them with `BIM_BLESS=1 cargo test`.
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
const reference = (name: string, kind: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "reference", role: "value", label, ref: { kind }, group: "value", order } });
const length = (name: string, label: Label, order: number): Mine => ({ name, rust: "f64", schema: { type: "number" }, ui: { widget: "number", role: "value", label, group: "value", order, unit: "m", step: 0.005, precision: 3 } as Prop["ui"] });
const text = (name: string, label: Label, order: number): Mine => ({ name, rust: "String", schema: { type: "string" }, ui: { widget: "text", role: "value", label, group: "identity", order } });
const sideProp = (order: number): Mine => ({ name: "side", rust: "WallSide", schema: record("WallSide"), ui: { widget: "select", role: "value", label: { en: "Side", de: "Seite" }, group: "value", order } });

export const ok = { status: "applied" } as const;
export const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const target = "vec![self.id.clone()]";
const APPLIED = [0x2705, 0x2795, 0x2728, 0x1f44d, 0x1f9f2, 0x1f31f, 0x1f4a1];
const REJECTED = [0x1f6ab, 0x26d4, 0x274c, 0x1f6d1, 0x1f6b7, 0x1f645, 0x1f4db, 0x1f6a7, 0x1f9ef, 0x2757, 0x1f534, 0x1f6a8];
type Row = { name: string; before: unknown; mutation: Record<string, unknown>; outcome: Leaf["cases"][number]["outcome"] };
const cases = (rows: Row[]): Leaf["cases"] => {
  let applied = 0;
  let rejected = 0;
  return rows.map((row) => ({ ...row, emoji: row.outcome.status === "applied" ? APPLIED[applied++] : REJECTED[rejected++] }));
};

const V = (x: number, y: number, bulge = 0) => ({ point: F.P(x, y), bulge });
export const rect = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
export const baseboard = (host = "w-south", extra: Record<string, unknown> = {}) => ({ host, side: "Left", profile: { Rectangle: { width: 0.02, depth: 0.1 } }, height: 0, inset: 0, material: "m-paint", name: "Baseboard", ...extra });
export const slab = (storey: string, name: string) => ({ storey, slab_type: "slt-200", boundary: rect(0, 0, 8, 6), holes: [], offset: 0, phase: "New", name });
export const roof = (storey: string, name: string) => ({ storey, roof_type: "rt-tile", footprint: rect(0, 0, 8, 6), shape: { Gable: { pitch: 0.6, ridge_direction: 0 } }, overhang: 0.3, base_offset: 0, phase: "New", name });

export const library = (extra: Record<string, unknown> = {}) => ({
  ...F.scene(),
  materials: { "m-brick": F.material("Brick"), "m-paint": F.material("Paint"), "m-conc": F.material("Concrete"), "m-tile": F.material("Tile") },
  slab_types: { "slt-200": { name: "Concrete 200", layers: [F.layer("m-conc", 0.2)] } },
  roof_types: { "rt-tile": { name: "Tile roof", layers: [F.layer("m-tile", 0.05, "Finish")] } },
  slabs: { "sl-ground": slab("st-ground", "Ground slab") },
  roofs: { "r-main": roof("st-first", "Main roof") },
  ...extra,
});
export const withSweep = (extra: Record<string, unknown> = {}) => library({ wall_sweeps: { "ws-base": baseboard() }, ...extra });
const withData = () => ({ ...withSweep(), properties: { "ws-base": { Pset_Custom: { Finish: { Text: { value: "painted" } } } } }, classifications: { "ws-base": { system: "Uniclass", code: "Pr_20_93", title: "Skirting" } } });
export const withTwoBuildings = () => library({ buildings: { "bldg-1": F.building("site-1", "House"), "bldg-2": F.building("site-1", "Annex") }, storeys: { "st-ground": F.storey("bldg-1", "Ground", 0, 3), "st-first": F.storey("bldg-1", "First", 1, 2.8), "st-b2": F.storey("bldg-2", "Annex ground", 0, 3) }, slabs: { "sl-ground": slab("st-ground", "Ground slab"), "sl-b2": slab("st-b2", "Annex slab") } });
export const withAttachedBase = () => {
  const model: any = library();
  model.walls["w-south"] = { ...model.walls["w-south"], base_slab: "sl-ground" };
  return model;
};

type Extra = { optional: string[] };
const extras = new Map<string, Extra>();
const leaf = (spec: MineLeaf): MineLeaf => (extras.set(spec.kind, { optional: spec.props.filter((prop) => prop.optional).map((prop) => prop.name) }), spec);

const sweepLabel = { en: "Wall sweep", de: "Wandprofil" };
const hostLabel = { en: "Host wall", de: "Wirtswand" };
const materialLabel = { en: "Material", de: "Material" };
const profileLabel = { en: "Profile", de: "Profil" };
const heightLabel = { en: "Height above the base (m)", de: "Höhe über der Basis (m)" };
const insetLabel = { en: "Inset into the wall (m)", de: "Einrückung in die Wand (m)" };
const nameLabel = { en: "Name", de: "Name" };

export const leaves: MineLeaf[] = [
  leaf({
    kind: "create-wall-sweep", emoji: 0x1fa9b, variant: "CreateWallSweep", verb: "create", entity: "wall-sweep", binaryTag: 8000, displayName: "Create Wall Sweep",
    doc: "Brings a new sweep onto one face of a wall: a profile run along the face (a baseboard, a cornice, a drip rail) at a height above the wall base, optionally set into the wall; its solid, length and areas are inferred.",
    props: [id("wall-sweep", "identity", { en: "Wall sweep id", de: "Wandprofil-Id" }, 10), recordProp("wall_sweep", "WallSweep", sweepLabel)],
    label: { en: 'format!("Create wall sweep \\"{}\\"", self.wall_sweep.name)', de: 'format!("Wandprofil \\"{}\\" anlegen", self.wall_sweep.name)' },
    target,
    cases: cases([
      { name: "adds-a-baseboard", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard() }, outcome: ok },
      { name: "adds-an-embedded-rail", before: library(), mutation: { id: "ws-rail", wall_sweep: baseboard("w-east", { side: "Right", profile: { Circle: { diameter: 0.04 } }, height: 0.9, inset: 0.01, name: "Hand rail" }) }, outcome: ok },
      { name: "duplicate", before: withSweep(), mutation: { id: "ws-base", wall_sweep: baseboard() }, outcome: reject("mutation.duplicate-id", ["ws-base"]) },
      { name: "id-taken-by-another-kind", before: library(), mutation: { id: "w-south", wall_sweep: baseboard() }, outcome: reject("mutation.duplicate-id", ["w-south"]) },
      { name: "host-missing", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard("w-gone") }, outcome: reject("mutation.target-missing", ["wall_sweep", "host"]) },
      { name: "material-missing", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard("w-south", { material: "m-ghost" }) }, outcome: reject("mutation.target-missing", ["wall_sweep", "material"]) },
      { name: "profile-degenerate", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard("w-south", { profile: { Rectangle: { width: 0, depth: 0.1 } } }) }, outcome: reject("mutation.invariant", ["wall_sweep", "profile"]) },
      { name: "negative-height", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard("w-south", { height: -0.1 }) }, outcome: reject("mutation.invariant", ["wall_sweep", "height"]) },
      { name: "inset-swallows-the-profile", before: library(), mutation: { id: "ws-base", wall_sweep: baseboard("w-south", { inset: 0.03 }) }, outcome: reject("mutation.invariant", ["wall_sweep", "inset"]) },
    ]),
  }),
  leaf({
    kind: "set-wall-sweep", emoji: 0x1faa0, variant: "SetWallSweep", verb: "set", entity: "wall-sweep", binaryTag: 8001, displayName: "Set Wall Sweep",
    doc: "Sets exactly the provided fields of a wall sweep: host wall, face, profile, height above the base, inset, material and name; absent fields stay untouched.",
    props: [
      id("wall-sweep", "target", sweepLabel),
      sparse(reference("host", "wall", hostLabel, 20), keep),
      sparse(sideProp(30), keep),
      sparse(recordProp("profile", "Profile", profileLabel, 40), keep),
      sparse(length("height", heightLabel, 50), keep),
      sparse(length("inset", insetLabel, 60), keep),
      sparse(reference("material", "material", materialLabel, 70), keep),
      sparse(text("name", nameLabel, 80), keep),
    ],
    label: { en: 'format!("Change wall sweep \\"{}\\"", self.id)', de: 'format!("Wandprofil \\"{}\\" ändern", self.id)' },
    target,
    cases: cases([
      { name: "lifts-and-resizes", before: withSweep(), mutation: { id: "ws-base", height: 0.05, profile: { Rectangle: { width: 0.03, depth: 0.12 } } }, outcome: ok },
      { name: "moves-to-the-other-face", before: withSweep(), mutation: { id: "ws-base", side: "Right" }, outcome: ok },
      { name: "rehosts", before: withSweep(), mutation: { id: "ws-base", host: "w-east" }, outcome: ok },
      { name: "renames", before: withSweep(), mutation: { id: "ws-base", name: "Skirting" }, outcome: ok },
      { name: "restates-an-unchanged-field", before: withSweep(), mutation: { id: "ws-base", inset: 0, name: "Skirting" }, outcome: ok },
      { name: "unchanged", before: withSweep(), mutation: { id: "ws-base", name: "Baseboard", height: 0 }, outcome: reject("mutation.no-op", ["ws-base"]) },
      { name: "names-no-field", before: withSweep(), mutation: { id: "ws-base" }, outcome: reject("mutation.no-op", ["ws-base"]) },
      { name: "host-missing", before: withSweep(), mutation: { id: "ws-base", host: "w-gone" }, outcome: reject("mutation.target-missing", ["host"]) },
      { name: "material-missing", before: withSweep(), mutation: { id: "ws-base", material: "m-ghost" }, outcome: reject("mutation.target-missing", ["material"]) },
      { name: "profile-degenerate", before: withSweep(), mutation: { id: "ws-base", profile: { Circle: { diameter: 0 } } }, outcome: reject("mutation.invariant", ["profile"]) },
      { name: "negative-height", before: withSweep(), mutation: { id: "ws-base", height: -0.2 }, outcome: reject("mutation.invariant", ["height"]) },
      { name: "inset-swallows-the-profile", before: withSweep(), mutation: { id: "ws-base", inset: 0.03 }, outcome: reject("mutation.invariant", ["inset"]) },
      { name: "missing", before: withSweep(), mutation: { id: "ws-gone", name: "Ghost" }, outcome: reject("mutation.target-missing", ["ws-gone"]) },
    ]),
  }),
  leaf({
    kind: "delete-wall-sweep", emoji: 0x1faa1, variant: "DeleteWallSweep", verb: "delete", entity: "wall-sweep", binaryTag: 8002, displayName: "Delete Wall Sweep",
    doc: "Removes a wall sweep together with its properties and classifications.",
    props: [id("wall-sweep", "target", sweepLabel)],
    label: { en: 'format!("Delete wall sweep \\"{}\\"", self.id)', de: 'format!("Wandprofil \\"{}\\" löschen", self.id)' },
    target,
    inverseRows: { bounded: 4096 },
    cases: cases([
      { name: "removes", before: withSweep(), mutation: { id: "ws-base" }, outcome: ok },
      { name: "removes-its-data", before: withData(), mutation: { id: "ws-base" }, outcome: ok },
      { name: "missing", before: withSweep(), mutation: { id: "ws-gone" }, outcome: reject("mutation.target-missing", ["ws-gone"]) },
    ]),
  }),
  leaf({
    kind: "set-wall-base-slab", emoji: 0x1fa94, variant: "SetWallBaseSlab", verb: "set", entity: "wall", binaryTag: 8003, displayName: "Set Wall Base Slab",
    doc: "Attaches the base of a wall to the top surface of a slab of its building, or frees it again: an attached base follows the slab (a sloped slab included) plus the base offset; the resolved base height is inferred.",
    props: [id("wall", "target", { en: "Wall", de: "Wand" }), sparse(reference("slab", "slab", { en: "Base slab (empty = free)", de: "Basisdecke (leer = frei)" }, 20), { en: "Leave empty to free the base.", de: "Leer lassen, um die Basis zu lösen." })],
    label: { en: 'format!("Attach the base of wall \\"{}\\" to {}", self.id, self.slab.as_deref().map_or("the storey".to_string(), |slab| format!("slab \\"{slab}\\"")))', de: 'format!("Basis der Wand \\"{}\\" an {} anbinden", self.id, self.slab.as_deref().map_or("das Geschoss".to_string(), |slab| format!("Decke \\"{slab}\\"")))' },
    target,
    cases: cases([
      { name: "attaches", before: library(), mutation: { id: "w-south", slab: "sl-ground" }, outcome: ok },
      { name: "frees-the-base", before: withAttachedBase(), mutation: { id: "w-south" }, outcome: ok },
      { name: "unchanged", before: withAttachedBase(), mutation: { id: "w-south", slab: "sl-ground" }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "already-free", before: library(), mutation: { id: "w-south" }, outcome: reject("mutation.no-op", ["w-south"]) },
      { name: "slab-missing", before: library(), mutation: { id: "w-south", slab: "sl-gone" }, outcome: reject("mutation.target-missing", ["slab"]) },
      { name: "slab-in-another-building", before: withTwoBuildings(), mutation: { id: "w-south", slab: "sl-b2" }, outcome: reject("mutation.invariant", ["slab"]) },
      { name: "missing", before: library(), mutation: { id: "w-gone", slab: "sl-ground" }, outcome: reject("mutation.target-missing", ["w-gone"]) },
    ]),
  }),
];

//#region 🔖️Hand
const DIFF = em(0x1f53a);
const INVERSE = em(0x21a9);
const MUTATION = em(0x1f9a0);
const snake = (kind: string) => kind.replaceAll("-", "_");
const rs = (strings: TemplateStringsArray, ...values: string[]) =>
  strings.raw
    .reduce((out, chunk, index) => out + chunk + (values[index] ?? ""), "")
    .replace(/\\u\{([0-9a-fA-F]+)\}/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/\\u([0-9a-fA-F]{4})/g, (_match, hex: string) => String.fromCodePoint(parseInt(hex, 16)))
    .replaceAll("¶", "`")
    .replaceAll('\\\\"', '\\"');

const HAND: Record<string, { diff: string; inverse: string }> = {
  "create-wall-sweep": {
    diff: rs`//! ${DIFF} Diff constructor for ¶CreateWallSweep¶: one created wall sweep entry. The id is free in every collection, the host wall and the material exist, the profile has positive dimensions, the height above the wall base is not
//! negative and the inset leaves part of the profile showing. The solid, the length and the areas of the sweep are inferred.

use super::super::elements;
use super::super::wall_depth::sweep_flaw;
use super::CreateWallSweep;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = sweep_flaw(base, &payload.wall_sweep) {
        return flaw.under(&["wall_sweep"]).refuse();
    }
    MutationOutcome::new(ModelDiff::wall_sweeps(payload.id.clone(), Entry::Created(payload.wall_sweep.clone())))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶CreateWallSweep¶: the concrete ¶DeleteWallSweep¶ of the id it created, none when the id was already taken.

use super::super::delete_wall_sweep::DeleteWallSweep;
use super::CreateWallSweep;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.wall_sweeps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteWallSweep(DeleteWallSweep { id: payload.id.clone() })]
}
`,
  },
  "set-wall-sweep": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetWallSweep¶: a sparse wall sweep patch of exactly the provided fields that differ. The sweep that results must follow the create rules (host and material exist, profile with positive
//! dimensions, height not negative, inset leaving part of the profile showing); providing only equal values, or no field, is a no-op.

use super::super::wall_depth::sweep_flaw;
use super::SetWallSweep;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(sweep) = base.wall_sweeps.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall sweep \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let change = payload.patch().minimal(sweep);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall sweep \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = sweep_flaw(base, &change.write(sweep)) {
        return flaw.refuse();
    }
    MutationOutcome::new(ModelDiff::wall_sweeps(payload.id.clone(), Entry::Patched(change)))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetWallSweep¶: an absolute ¶SetWallSweep¶ restoring the base value of exactly the fields the forward really changes, none when the sweep is absent or nothing changes.

use super::SetWallSweep;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.wall_sweeps.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetWallSweep(SetWallSweep::from_patch(payload.id.clone(), restore))]
}
`,
  },
  "delete-wall-sweep": {
    diff: rs`//! ${DIFF} Diff constructor for ¶DeleteWallSweep¶: the sweep leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteWallSweep;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.wall_sweeps.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall sweep \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Wall sweep", Some(&payload.id))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶DeleteWallSweep¶: one concrete create per removed record and one setter per removed property or classification, in storage order (dependants first, the target last).

use super::super::cascade;
use super::DeleteWallSweep;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
`,
  },
  "set-wall-base-slab": {
    diff: rs`//! ${DIFF} Diff constructor for ¶SetWallBaseSlab¶: a one-field wall patch assigning (or clearing) the slab the base of the wall is attached to. The slab must exist in the building of the wall and the chain of attach references must
//! not loop (authored reads only); restating the current base slab is a no-op. The resolved base height follows by inference.

use super::super::wall_depth::attach_flaw;
use super::super::wall_geometry::Flaw;
use super::SetWallBaseSlab;
use crate::{Assigned, Entry, ModelDiff, ModelSnapshot, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallBaseSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if wall.base_slab == payload.slab {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \\"{}\\" already has this base.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = attach_flaw(base, Some(&payload.id), &wall.storey, &wall.top, payload.slab.as_deref()) {
        let path: Vec<&str> = flaw.path.iter().map(|segment| if segment == "base_slab" { "slab" } else { segment.as_str() }).collect();
        return Flaw::new(flaw.code, &path, flaw.message.clone()).refuse();
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { base_slab: Some(Assigned::new(payload.slab.clone())), ..Default::default() })))
}
`,
    inverse: rs`//! ${INVERSE} Inverse of ¶SetWallBaseSlab¶: an absolute ¶SetWallBaseSlab¶ back to the base slab of the wall (none when it stood on its storey), none when the wall is absent.

use super::SetWallBaseSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallBaseSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallBaseSlab(SetWallBaseSlab { id: payload.id.clone(), slab: wall.base_slab.clone() })],
        None => Vec::new(),
    }
}
`,
  },
};

const PATCHES: Record<string, { patch: string; fromPatch: string; patchType: string }> = {
  "set-wall-sweep": {
    patchType: "WallSweepPatch",
    patch: "WallSweepPatch { host: self.host.clone(), side: self.side, profile: self.profile.clone(), height: self.height, inset: self.inset, material: self.material.clone(), name: self.name.clone() }",
    fromPatch: "Self { id, host: patch.host, side: patch.side, profile: patch.profile, height: patch.height, inset: patch.inset, material: patch.material, name: patch.name }",
  },
};

function fixLeaf(spec: MineLeaf) {
  const optionals = extras.get(spec.kind)!.optional;
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  const file = join(dir, MUTATION + "mutation", RS);
  const hand = PATCHES[spec.kind];
  const used = new Set(spec.props.flatMap((prop) => prop.rust.match(/[A-Z][A-Za-z0-9]*/g) ?? []).filter((name) => !["Option", "String", "Vec"].includes(name)));
  if (hand) used.add(hand.patchType);
  let source = readFileSync(file, "utf8");
  source = source.replace(/^use crate::\{.*\};$/m, `use crate::{${["ModelDiff", "ModelMutation", "ModelSnapshot", ...used].sort().join(", ")}};`);
  for (const name of optionals) source = source.replace(new RegExp(`^    pub ${name}: `, "m"), `    #[value(default, skip_serializing_if = "Option::is_none")]\n    pub ${name}: `);
  if (hand) {
    source = source.replace(
      /^impl MutationKind/m,
      `impl ${spec.variant} {\n    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.\n    pub fn patch(&self) -> ${hand.patchType} {\n        ${hand.patch}\n    }\n\n    /// 🧩 The payload that provides exactly the fields \`patch\` names.\n    pub fn from_patch(id: String, patch: ${hand.patchType}) -> Self {\n        ${hand.fromPatch}\n    }\n}\n\nimpl MutationKind`,
    );
  }
  writeFileSync(file, source);
  if (optionals.length > 0) {
    const schemaFile = join(dir, readdirSync(dir).find((name) => name.endsWith("schema"))!, JSONF);
    const schema = JSON.parse(readFileSync(schemaFile, "utf8"));
    schema.required = schema.required.filter((name: string) => !optionals.includes(name));
    writeFileSync(schemaFile, JSON.stringify(schema, null, 2) + "\n");
  }
}

function writeOnce(spec: MineLeaf) {
  const dir = join(mutations, em(spec.emoji) + spec.kind);
  for (const [folder, body] of [[DIFF + "diff", HAND[spec.kind].diff], [INVERSE + "inverse", HAND[spec.kind].inverse]] as const) {
    mkdirSync(join(dir, folder), { recursive: true });
    const file = join(dir, folder, RS);
    if (!existsSync(file)) writeFileSync(file, body);
  }
}
//#endregion 🔖️Hand

//#region 🔖️Run
if (import.meta.main) {
  const rootPath = join(artifact, RS);
  const mounts: string[] = [];
  for (const spec of leaves) {
    mounts.push(emitLeaf(spec as unknown as Leaf));
    fixLeaf(spec);
    writeOnce(spec);
  }
  let root = readFileSync(rootPath, "utf8");
  const crlf = root.includes("\r\n");
  if (crlf) root = root.replaceAll("\r\n", "\n");
  const anchor = "                        //#endregion 🔖️Leaves";
  let added = 0;
  const pending: string[] = [];
  for (const [index, spec] of leaves.entries()) {
    if (root.includes(`pub mod ${snake(spec.kind)} {`)) continue;
    root = root.replace(anchor, mounts[index] + anchor);
    pending.push(mounts[index]);
    added += 1;
  }
  try {
    writeFileSync(rootPath, crlf ? root.replaceAll("\n", "\r\n") : root);
  } catch (error) {
    const out = join(import.meta.dir, "🗑️generated", "w2-wp08-walldepth");
    mkdirSync(out, { recursive: true });
    writeFileSync(join(out, "mounts.txt"), pending.join(""));
    console.log(`root file is mapped by a build (${(error as Error).message}); the new mounts are in ${out}/mounts.txt, insert them before the Leaves anchor with the Edit tool`);
  }
  console.log(`w2-wp08-walldepth: ${leaves.length} leaves emitted, ${added} newly mounted`);
}
//#endregion 🔖️Run
