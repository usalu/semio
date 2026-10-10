#!/usr/bin/env bun
/**
 * 🌡️ Wave W2 (`w2-wp20-energy`): the three leaves of the energy vocabulary of `s.bim.model@1` (binary tags 20000..20002): `set-space-conditions` (sparse, creates the record of a space on first use),
 * `remove-space-conditions` and `set-type-thermal-data` (U-value, g-value and frame fraction of a window type, U-value of a door type), plus the cascade case `removes-its-conditions` of the existing `delete-space` leaf.
 * `bun r12-w2-wp20-leaves.ts` rewrites their boilerplate, their hand-written `diff`/`inverse` and their fixtures (never a blessed `after`/`diff`) and writes the mount text to `🗑️generated/w2-wp20-energy/mounts.txt`.
 * Bless with `BIM_BLESS=1 cargo test … <kind>`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { emitCase, emitLeaf, type Leaf, type Prop } from "./r3-f1-gen-leaf.ts";
import * as F from "./r3-f1-fixtures.ts";
import { em, JSONF, mutations, RS } from "./r3-f1-paths.ts";

type Label = { en: string; de: string };
const assigned = (inner: unknown) => ({ type: "object", additionalProperties: false, required: ["value"], properties: { value: { oneOf: [inner, { type: "null" }] } } });
const id = (role: "target" | "identity", label: Label, kind: string, order = 10): Prop => ({
  name: "id",
  rust: "String",
  schema: { type: "string" },
  ui: role === "target" ? { widget: "reference", role, label, ref: { kind }, group: "target", order } : { widget: "text", role: "identity", label, group: "identity", order },
});
const prop = (name: string, rust: string, schema: unknown, widget: string, label: Label, order: number, group = "value"): Prop => ({ name, rust, schema: schema as Record<string, unknown>, ui: { widget, role: "value", label, group, order } });
const real = (name: string, label: Label, order: number, group = "value") => prop(name, "Option<Assigned<Option<f64>>>", assigned({ type: "number" }), "number", label, order, group);
const word = (name: string, label: Label, order: number, group = "value") => prop(name, "Option<Assigned<Option<String>>>", assigned({ type: "string" }), "text", label, order, group);

const ok = { status: "applied" } as const;
const reject = (code: string, path: string[]) => ({ status: "rejected" as const, code, path });
const P = F.P;
const target = "vec![self.id.clone()]";

//#region 🔖️Fixtures
const space = (over: Record<string, unknown> = {}) => ({ storey: "st-ground", number: "0.01", name: "Living", boundary: { Bounded: { seed: P(4, 3) } }, usage: "Living", phase: "New", ...over });
const win = (over: Record<string, unknown> = {}) => ({ name: "Triple glazing", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 3, material: "m-brick", ...over });
const door = (over: Record<string, unknown> = {}) => ({ name: "Entrance", width: 1, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: "Single", swing: "Left", material: "m-brick", ...over });
const living = { occupancy: "Residential", occupancy_density: 0.04, heating_setpoint: 20, cooling_setpoint: 26, ventilation_rate: 0.3, lighting_power_density: 5, equipment_power_density: 3, schedule: "Residential day" };
const scene = (extra: Record<string, unknown> = {}) => ({ ...F.scene(), spaces: { "sp-1": space(), "sp-2": space({ number: "0.02", name: "Hall", boundary: { Bounded: { seed: P(1, 1) } } }) }, window_types: { "win-1": win() }, door_types: { "dr-1": door() }, ...extra });
const conditioned = () => scene({ space_conditions: { "sp-1": living } });
const sparse = () => scene({ space_conditions: { "sp-1": { heating_setpoint: 20 } } });
const thermal = () => scene({ window_types: { "win-1": win({ u_value: 0.9, g_value: 0.5, frame_fraction: 0.25 }) }, door_types: { "dr-1": door({ u_value: 1.3 }) } });
const withCurtain = () => scene({ curtain_wall_types: { cwt: { name: "Facade", u_grid: { Spacing: { spacing: 1.5 } }, v_grid: { Spacing: { spacing: 1.5 } }, interior_mullion: { Rectangle: { width: 0.05, depth: 0.1 } }, border_mullion: { Rectangle: { width: 0.05, depth: 0.1 } }, panel: "Glass", panel_material: "m-brick", mullion_material: "m-brick" } } });
const dataCarrying = () => ({ ...conditioned(), properties: { "sp-1": { Pset_SpaceCommon: { Reference: { Text: { value: "R-1" } } } } } });
//#endregion 🔖️Fixtures

const emoji = ["2705", "1f9f2", "1f9f5", "1f9f4", "1f9f3", "1f9f1", "1f9f0", "1f9ef", "1f9ee", "1f9ed", "1f9e9", "1f9e8", "1f9e7", "1f9e6", "1f9e5", "1f9e4", "1f9e3", "1f9e2", "1f9e1", "1f9e0", "1f9df", "1f9de", "1f9dd", "1f9dc"].map((hex) => parseInt(hex, 16));
const cases = <T extends { name: string; emoji?: number }>(rows: T[]) => rows.map((row, index) => ({ ...row, emoji: emoji[index] }));

const spaceFields: [string, string, Label, "real" | "word"][] = [
  ["occupancy", "occupancy", { en: "Occupancy type", de: "Nutzungsart" }, "word"],
  ["occupancy_density", "occupancy_density", { en: "Occupancy (persons/m²)", de: "Belegung (Personen/m²)" }, "real"],
  ["heating_setpoint", "heating_setpoint", { en: "Heating set point (°C)", de: "Heizsollwert (°C)" }, "real"],
  ["cooling_setpoint", "cooling_setpoint", { en: "Cooling set point (°C)", de: "Kühlsollwert (°C)" }, "real"],
  ["ventilation_rate", "ventilation_rate", { en: "Ventilation (L/s·m²)", de: "Lüftung (L/s·m²)" }, "real"],
  ["lighting_power_density", "lighting_power_density", { en: "Lighting (W/m²)", de: "Beleuchtung (W/m²)" }, "real"],
  ["equipment_power_density", "equipment_power_density", { en: "Equipment (W/m²)", de: "Geräte (W/m²)" }, "real"],
  ["schedule", "schedule", { en: "Schedule profile", de: "Nutzungsprofil" }, "word"],
];
const spaceProps = spaceFields.map(([name, , label, kind], index) => (kind === "real" ? real(name, label, 20 + index * 10) : word(name, label, 20 + index * 10)));
const spaceNames = spaceFields.map(([name]) => name);

export const leaves: Leaf[] = [
  {
    kind: "set-space-conditions", emoji: 0x1f321, variant: "SetSpaceConditions", verb: "set", entity: "space-conditions", binaryTag: 20000, displayName: "Set Space Conditions",
    doc: "Sparsely sets the thermal conditions of a space (occupancy type and density, heating and cooling set points, ventilation, lighting and equipment power density, schedule profile); the first use creates the record of the space, an assigned null clears a field. The envelope, the areas and the U-values are inferred.",
    props: [id("target", { en: "Space", de: "Raum" }, "space"), ...spaceProps],
    uses: ["use crate::{Assigned, SpaceConditionsPatch};"],
    label: { en: 'format!("Set conditions of space \\"{}\\"", self.id)', de: 'format!("Bedingungen des Raums \\"{}\\" setzen", self.id)' },
    target,
    cases: cases([
      { name: "creates-the-record", before: scene(), mutation: { id: "sp-1", heating_setpoint: { value: 20 }, cooling_setpoint: { value: 26 } }, outcome: ok },
      { name: "creates-a-full-record", before: scene(), mutation: Object.fromEntries([["id", "sp-1"], ...Object.entries(living).map(([key, value]) => [key, { value }])]), outcome: ok },
      { name: "creates-an-empty-record", before: scene(), mutation: { id: "sp-2" }, outcome: ok },
      { name: "raises-the-set-point", before: sparse(), mutation: { id: "sp-1", heating_setpoint: { value: 22 } }, outcome: ok },
      { name: "adds-ventilation-and-loads", before: sparse(), mutation: { id: "sp-1", ventilation_rate: { value: 0.3 }, lighting_power_density: { value: 5 }, equipment_power_density: { value: 3 } }, outcome: ok },
      { name: "names-occupancy-and-schedule", before: sparse(), mutation: { id: "sp-1", occupancy: { value: "Office" }, schedule: { value: "Office 08-18" } }, outcome: ok },
      { name: "clears-a-field", before: conditioned(), mutation: { id: "sp-1", cooling_setpoint: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: conditioned(), mutation: { id: "sp-1", heating_setpoint: { value: 20 }, ventilation_rate: { value: 0.5 } }, outcome: ok },
      { name: "unchanged", before: conditioned(), mutation: { id: "sp-1", heating_setpoint: { value: 20 }, occupancy: { value: "Residential" } }, outcome: reject("mutation.no-op", ["sp-1"]) },
      { name: "empty-patch-on-a-record", before: conditioned(), mutation: { id: "sp-1" }, outcome: reject("mutation.no-op", ["sp-1"]) },
      { name: "space-missing", before: scene(), mutation: { id: "sp-9", heating_setpoint: { value: 20 } }, outcome: reject("mutation.target-missing", ["sp-9"]) },
      { name: "cooling-below-heating", before: sparse(), mutation: { id: "sp-1", cooling_setpoint: { value: 18 } }, outcome: reject("mutation.invariant", ["cooling_setpoint"]) },
      { name: "set-point-out-of-range", before: scene(), mutation: { id: "sp-1", heating_setpoint: { value: 80 } }, outcome: reject("mutation.invariant", ["heating_setpoint"]) },
      { name: "negative-density", before: scene(), mutation: { id: "sp-1", occupancy_density: { value: -0.1 } }, outcome: reject("mutation.invariant", ["occupancy_density"]) },
      { name: "negative-ventilation", before: scene(), mutation: { id: "sp-1", ventilation_rate: { value: -1 } }, outcome: reject("mutation.invariant", ["ventilation_rate"]) },
      { name: "negative-lighting", before: scene(), mutation: { id: "sp-1", lighting_power_density: { value: -5 } }, outcome: reject("mutation.invariant", ["lighting_power_density"]) },
      { name: "blank-schedule", before: scene(), mutation: { id: "sp-1", schedule: { value: "  " } }, outcome: reject("mutation.invariant", ["schedule"]) },
    ]),
  },
  {
    kind: "remove-space-conditions", emoji: 0x1f976, variant: "RemoveSpaceConditions", verb: "remove", entity: "space-conditions", binaryTag: 20001, displayName: "Remove Space Conditions",
    doc: "Removes the thermal conditions of a space; the space itself stays, and an unheated space is a thermal boundary of its heated neighbours again.",
    props: [id("target", { en: "Space", de: "Raum" }, "space")],
    label: { en: 'format!("Remove conditions of space \\"{}\\"", self.id)', de: 'format!("Bedingungen des Raums \\"{}\\" entfernen", self.id)' },
    target,
    cases: cases([
      { name: "removes-the-record", before: conditioned(), mutation: { id: "sp-1" }, outcome: ok },
      { name: "removes-a-sparse-record", before: sparse(), mutation: { id: "sp-1" }, outcome: ok },
      { name: "space-without-conditions", before: scene(), mutation: { id: "sp-1" }, outcome: reject("mutation.no-op", ["sp-1"]) },
      { name: "space-missing", before: scene(), mutation: { id: "sp-9" }, outcome: reject("mutation.target-missing", ["sp-9"]) },
    ]),
  },
  {
    kind: "set-type-thermal-data", emoji: 0x2668, variant: "SetTypeThermalData", verb: "set", entity: "type-thermal-data", binaryTag: 20002, displayName: "Set Type Thermal Data",
    doc: "Sparsely sets the thermal data of a window type (U-value of the whole window, g-value of the glazing, frame fraction), of a curtain wall type (the same three for the whole facade) or of a door type (U-value); an assigned null clears a field. The envelope U-values and solar gains are inferred from them.",
    props: [
      id("target", { en: "Window, door or curtain wall type", de: "Fenster-, Tür- oder Fassadentyp" }, "window-type"),
      real("u_value", { en: "U-value (W/m²K)", de: "U-Wert (W/m²K)" }, 20),
      real("g_value", { en: "g-value (0..1)", de: "g-Wert (0..1)" }, 30),
      real("frame_fraction", { en: "Frame fraction (0..1)", de: "Rahmenanteil (0..1)" }, 40),
    ],
    uses: ["use crate::{Assigned, CurtainWallTypePatch, DoorTypePatch, WindowTypePatch};"],
    label: { en: 'format!("Set thermal data of type \\"{}\\"", self.id)', de: 'format!("Thermische Daten des Typs \\"{}\\" setzen", self.id)' },
    target,
    cases: cases([
      { name: "states-a-window", before: scene(), mutation: { id: "win-1", u_value: { value: 0.9 }, g_value: { value: 0.5 }, frame_fraction: { value: 0.25 } }, outcome: ok },
      { name: "states-a-door", before: scene(), mutation: { id: "dr-1", u_value: { value: 1.3 } }, outcome: ok },
      { name: "improves-the-glazing", before: thermal(), mutation: { id: "win-1", u_value: { value: 0.7 } }, outcome: ok },
      { name: "clears-the-g-value", before: thermal(), mutation: { id: "win-1", g_value: { value: null } }, outcome: ok },
      { name: "restates-an-unchanged-field", before: thermal(), mutation: { id: "win-1", u_value: { value: 0.9 }, g_value: { value: 0.6 } }, outcome: ok },
      { name: "unchanged", before: thermal(), mutation: { id: "win-1", u_value: { value: 0.9 }, frame_fraction: { value: 0.25 } }, outcome: reject("mutation.no-op", ["win-1"]) },
      { name: "empty-patch", before: thermal(), mutation: { id: "win-1" }, outcome: reject("mutation.no-op", ["win-1"]) },
      { name: "type-missing", before: scene(), mutation: { id: "win-9", u_value: { value: 1 } }, outcome: reject("mutation.target-missing", ["win-9"]) },
      { name: "u-value-zero", before: scene(), mutation: { id: "win-1", u_value: { value: 0 } }, outcome: reject("mutation.invariant", ["u_value"]) },
      { name: "g-value-above-one", before: scene(), mutation: { id: "win-1", g_value: { value: 1.5 } }, outcome: reject("mutation.invariant", ["g_value"]) },
      { name: "frame-fraction-one", before: scene(), mutation: { id: "win-1", frame_fraction: { value: 1 } }, outcome: reject("mutation.invariant", ["frame_fraction"]) },
      { name: "states-a-curtain-wall", before: withCurtain(), mutation: { id: "cwt", u_value: { value: 1.4 }, g_value: { value: 0.4 }, frame_fraction: { value: 0.15 } }, outcome: ok },
      { name: "curtain-g-value-above-one", before: withCurtain(), mutation: { id: "cwt", g_value: { value: 1.5 } }, outcome: reject("mutation.invariant", ["g_value"]) },
      { name: "door-has-no-g-value", before: scene(), mutation: { id: "dr-1", g_value: { value: 0.5 } }, outcome: reject("mutation.invariant", ["g_value"]) },
      { name: "door-has-no-frame-fraction", before: scene(), mutation: { id: "dr-1", frame_fraction: { value: 0.2 } }, outcome: reject("mutation.invariant", ["frame_fraction"]) },
    ]),
  },
];

const patchImpl = (variant: string, patch: string, fields: string[], rest: string) => `impl ${variant} {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ${patch} {
        ${patch} { ${fields.map((field) => `${field}: self.${field}.clone()`).join(", ")}${rest} }
    }

    /// 🧩 The payload that provides exactly the fields \`patch\` names.
    pub fn from_patch(id: String, patch: ${patch}) -> Self {
        Self { id, ${fields.map((field) => `${field}: patch.${field}`).join(", ")} }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for ${variant} {`;

const statingImpl = `impl SetSpaceConditions {
    /// 🌡️ The payload that states every field of \`record\` (an unstated field as an assigned null): applied to a space without conditions it recreates the record exactly.
    pub fn stating(id: &str, record: &crate::SpaceConditions) -> Self {
        Self {
            id: id.to_string(),
${spaceNames.map((name) => `            ${name}: Some(Assigned::new(record.${name}.clone())),`).join("\n")}
        }
    }
}

`;

const OPTIONAL: Record<string, string[]> = {
  "set-space-conditions": spaceNames,
  "set-type-thermal-data": ["u_value", "g_value", "frame_fraction"],
};

const setDiff = `//! 🔺️ Diff constructor for \`SetSpaceConditions\`: the first use creates the record of the space from the provided fields, later uses a sparse patch of exactly the provided fields that differ. The space must exist and the
//! resulting conditions must be writable (see \`conditions_problem\`); providing only equal values is a no-op.

use super::SetSpaceConditions;
use crate::{conditions_problem, Entry, ModelDiff, ModelSnapshot, Patch, SpaceConditions};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSpaceConditions, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let patch = payload.patch();
    match base.space_conditions.get(&payload.id) {
        None => {
            let record = patch.write(&SpaceConditions::empty());
            if let Some((field, message)) = conditions_problem(&record) {
                return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
            }
            MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Created(record)))
        }
        Some(record) => {
            let change = patch.minimal(record);
            if change.is_empty() {
                return MutationOutcome::refuse(OutcomeCode::NoOp, format!("The conditions of space \\"{}\\" already have these values.", payload.id), [payload.id.clone()]);
            }
            if let Some((field, message)) = conditions_problem(&change.write(record)) {
                return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
            }
            MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Patched(change)))
        }
    }
}
`;

const setInverse = `//! ↩️ Inverse of \`SetSpaceConditions\`: \`RemoveSpaceConditions\` when the forward creates the record, else an absolute \`SetSpaceConditions\` restoring the base value of exactly the fields the forward really changes.

use super::SetSpaceConditions;
use crate::{ModelMutation, ModelSnapshot, Patch};
use super::super::remove_space_conditions::RemoveSpaceConditions;

pub fn inverse(payload: &SetSpaceConditions, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return vec![ModelMutation::RemoveSpaceConditions(RemoveSpaceConditions { id: payload.id.clone() })];
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSpaceConditions(SetSpaceConditions::from_patch(payload.id.clone(), restore))]
}
`;

const removeDiff = `//! 🔺️ Diff constructor for \`RemoveSpaceConditions\`: the deletion of the conditions record of a space. The space must exist; a space without conditions is a no-op.

use super::RemoveSpaceConditions;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveSpaceConditions, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.space_conditions.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Space \\"{}\\" has no conditions.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Deleted))
}
`;

const removeInverse = `//! ↩️ Inverse of \`RemoveSpaceConditions\`: one absolute \`SetSpaceConditions\` that states every field of the removed record (an unstated field as an assigned null), none when the space has no conditions.

use super::RemoveSpaceConditions;
use crate::{ModelMutation, ModelSnapshot};
use super::super::set_space_conditions::SetSpaceConditions;

pub fn inverse(payload: &RemoveSpaceConditions, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.space_conditions.get(&payload.id) else {
        return Vec::new();
    };
    vec![ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(&payload.id, record))]
}
`;

const typeDiff = `//! 🔺️ Diff constructor for \`SetTypeThermalData\`: a sparse patch of the thermal fields of a window type (U-value, g-value, frame fraction) or of a door type (U-value) that differ from the base. Ids are unique across the
//! model, so the id names at most one of them; a door type has no g-value and no frame fraction. The written values must stay in range (see \`window_thermal_problem\`); providing only equal values is a no-op.

use super::SetTypeThermalData;
use crate::{curtain_thermal_problem, door_thermal_problem, window_thermal_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetTypeThermalData, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(record) = base.window_types.get(&payload.id) {
        let change = payload.window_patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Window type \\"{}\\" already has these thermal values.", payload.id), [payload.id.clone()]);
        }
        if let Some((field, message)) = window_thermal_problem(&change.write(record)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
        }
        return MutationOutcome::new(ModelDiff::window_types(payload.id.clone(), Entry::Patched(change)));
    }
    if let Some(record) = base.curtain_wall_types.get(&payload.id) {
        let change = payload.curtain_patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall type \\"{}\\" already has these thermal values.", payload.id), [payload.id.clone()]);
        }
        if let Some((field, message)) = curtain_thermal_problem(&change.write(record)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
        }
        return MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Patched(change)));
    }
    let Some(record) = base.door_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Window, door or curtain wall type \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.g_value.is_some() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A door type has no g-value.", ["g_value"]);
    }
    if payload.frame_fraction.is_some() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A door type has no frame fraction.", ["frame_fraction"]);
    }
    let change = payload.door_patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Door type \\"{}\\" already has this thermal value.", payload.id), [payload.id.clone()]);
    }
    if let Some((field, message)) = door_thermal_problem(&change.write(record)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
    }
    MutationOutcome::new(ModelDiff::door_types(payload.id.clone(), Entry::Patched(change)))
}
`;

const typeInverse = `//! ↩️ Inverse of \`SetTypeThermalData\`: an absolute \`SetTypeThermalData\` restoring the base value of exactly the fields the forward really changes, none when the type is absent or nothing changes.

use super::SetTypeThermalData;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetTypeThermalData, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let restore = if let Some(record) = base.window_types.get(&payload.id) {
        let patch = payload.window_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: patch.g_value, frame_fraction: patch.frame_fraction }
    } else if let Some(record) = base.curtain_wall_types.get(&payload.id) {
        let patch = payload.curtain_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: patch.g_value, frame_fraction: patch.frame_fraction }
    } else if let Some(record) = base.door_types.get(&payload.id) {
        let patch = payload.door_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: None, frame_fraction: None }
    } else {
        return Vec::new();
    };
    if restore.u_value.is_none() && restore.g_value.is_none() && restore.frame_fraction.is_none() {
        return Vec::new();
    }
    vec![ModelMutation::SetTypeThermalData(restore)]
}
`;

const typePatchImpl = `impl SetTypeThermalData {
    /// 🩹 The sparse window type patch this payload names: every provided field, restated values included.
    pub fn window_patch(&self) -> WindowTypePatch {
        WindowTypePatch { u_value: self.u_value.clone(), g_value: self.g_value.clone(), frame_fraction: self.frame_fraction.clone(), ..Default::default() }
    }

    /// 🩹 The sparse curtain wall type patch this payload names: every provided field, restated values included.
    pub fn curtain_patch(&self) -> CurtainWallTypePatch {
        CurtainWallTypePatch { u_value: self.u_value.clone(), g_value: self.g_value.clone(), frame_fraction: self.frame_fraction.clone(), ..Default::default() }
    }

    /// 🩹 The sparse door type patch this payload names: the U-value, when provided.
    pub fn door_patch(&self) -> DoorTypePatch {
        DoorTypePatch { u_value: self.u_value.clone(), ..Default::default() }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetTypeThermalData {`;

const MODULES: Record<string, { diff: string; inverse: string; mutation?: (source: string) => string }> = {
  "set-space-conditions": { diff: setDiff, inverse: setInverse, mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetSpaceConditions {", statingImpl + patchImpl("SetSpaceConditions", "SpaceConditionsPatch", spaceNames, "")) },
  "remove-space-conditions": { diff: removeDiff, inverse: removeInverse },
  "set-type-thermal-data": { diff: typeDiff, inverse: typeInverse, mutation: (source) => source.replace("impl MutationKind<ModelSnapshot, ModelMutation> for SetTypeThermalData {", typePatchImpl) },
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

const spaceCascade = {
  name: "removes-its-conditions",
  emoji: 0x1f321,
  before: conditioned(),
  mutation: { id: "sp-1" },
  outcome: ok,
};

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "w2-wp20-energy");
  mkdirSync(out, { recursive: true });
  let mounts = "";
  for (const spec of leaves) {
    mounts += emitLeaf(spec);
    fixup(spec);
  }
  writeFileSync(join(out, "mounts.txt"), mounts);
  writeFileSync(join(out, "delete-space-case.txt"), emitCase("delete-space", em(0x1f9fd) + "delete-space", "DeleteSpace", spaceCascade));
  console.log(`emitted ${leaves.length} leaves and 1 delete-space case`);
}
