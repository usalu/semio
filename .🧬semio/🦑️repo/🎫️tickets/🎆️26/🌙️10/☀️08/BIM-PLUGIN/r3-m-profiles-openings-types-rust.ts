/**
 * 🦀️ Rust sources of the twelve `m-profiles-openings-types` leaves (column, beam, window and door types) and of the shared
 * validity module. The logic is regular across the four entities, so the hand-written `🔺️diff` and `↩️inverse` files are rendered
 * from one entity description each; the rendered files are committed in the leaf directories and are the source of truth after that.
 */

export type Field = { name: string; copy: boolean };
export type Invariant = { field: string; message: string; create: string; set: string };
export type Entity = {
  Pascal: string;
  snake: string;
  kebab: string;
  collection: string;
  title: string;
  fields: Field[];
  profile: boolean;
  invariants: Invariant[];
  users: string;
  usersNoun: string;
  usesOpeningKind: boolean;
};

const lower = (text: string) => text.toLowerCase();
const imports = (names: string[]) => [...names.filter((n) => /^[a-z]/.test(n)).sort(), ...names.filter((n) => /^[A-Z]/.test(n)).sort()].join(", ");

const scalarBlock = (e: Entity, mode: "create" | "set", indent = "    ") => {
  if (e.invariants.length === 0) return "";
  const rows = e.invariants.map((i) => `${indent}    ("${i.field}", ${i[mode]}, "${i.message}"),`).join("\n");
  const path = mode === "create" ? `["${e.snake}", field]` : "[field]";
  return `${indent}let broken = [
${rows}
${indent}]
${indent}.into_iter()
${indent}.find_map(|(field, holds, message)| (!holds).then_some((field, message)));
${indent}if let Some((field, message)) = broken {
${indent}    return MutationOutcome::refuse(OutcomeCode::Invariant, message, ${path});
${indent}}
`;
};

export function createDiff(e: Entity): string {
  const used = ["Entry", "ModelDiff", "ModelSnapshot", ...(e.profile ? ["profile_problem"] : []), ...(e.invariants.some((i) => i.create.includes("is_non_negative_length")) ? ["is_non_negative_length"] : []), ...(e.invariants.some((i) => i.create.includes("is_positive_length")) ? ["is_positive_length"] : [])];
  const check = e.profile
    ? `    if let Some(problem) = profile_problem(&record.profile) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, problem, ["${e.snake}", "profile"]);
    }
`
    : scalarBlock(e, "create");
  return `//! 🔺️ Diff constructor for \`Create${e.Pascal}\`: one created ${lower(e.title)} entry. The id must be free, the material must exist${e.profile ? " and the profile must be a valid cross-section" : " and every dimension must be valid"}.

use super::Create${e.Pascal};
use crate::{${imports(used)}};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Create${e.Pascal}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.${e.snake};
    if base.${e.collection}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("${e.title} \\"{}\\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.materials.contains_key(&record.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \\"{}\\" does not exist.", record.material), ["${e.snake}", "material"]);
    }
${check}    MutationOutcome::new(ModelDiff::${e.collection}(payload.id.clone(), Entry::Created(record.clone())))
}
`;
}

export function createInverse(e: Entity): string {
  return `//! ↩️ Inverse of \`Create${e.Pascal}\`: the concrete \`Delete${e.Pascal}\` of the id it created, none when the id was already taken.

use super::super::delete_${e.snake}::Delete${e.Pascal};
use super::Create${e.Pascal};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Create${e.Pascal}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.${e.collection}.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::Delete${e.Pascal}(Delete${e.Pascal} { id: payload.id.clone() })]
}
`;
}

export function deleteDiff(e: Entity): string {
  const used = ["Entry", "ModelDiff", "ModelSnapshot", ...(e.usesOpeningKind ? ["OpeningKind"] : [])];
  return `//! 🔺️ Diff constructor for \`Delete${e.Pascal}\`: one deleted ${lower(e.title)} entry; refused while ${e.usersNoun} still use it.

use super::Delete${e.Pascal};
use crate::{${imports(used)}};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Delete${e.Pascal}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.${e.collection}.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${e.title} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if ${e.users} {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("${e.title} \\"{}\\" is still used by ${e.usersNoun}.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::${e.collection}(payload.id.clone(), Entry::Deleted))
}
`;
}

export function deleteInverse(e: Entity): string {
  return `//! ↩️ Inverse of \`Delete${e.Pascal}\`: the concrete \`Create${e.Pascal}\` carrying the full removed record, none when the ${lower(e.title)} was absent.

use super::super::create_${e.snake}::Create${e.Pascal};
use super::Delete${e.Pascal};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Delete${e.Pascal}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.${e.collection}.get(&payload.id) {
        Some(record) => vec![ModelMutation::Create${e.Pascal}(Create${e.Pascal} { id: payload.id.clone(), ${e.snake}: record.clone() })],
        None => Vec::new(),
    }
}
`;
}

export function setDiff(e: Entity): string {
  const used = ["Entry", "ModelDiff", "ModelSnapshot", "Patch", `${e.Pascal}Patch`, ...(e.profile ? ["profile_problem"] : []), ...(e.invariants.some((i) => i.set.includes("is_non_negative_length")) ? ["is_non_negative_length"] : []), ...(e.invariants.some((i) => i.set.includes("is_positive_length")) ? ["is_positive_length"] : [])];
  const check = e.profile
    ? `    if let Some(problem) = payload.profile.as_ref().and_then(profile_problem) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, problem, ["profile"]);
    }
`
    : scalarBlock(e, "set");
  const patch = e.fields.map((f) => `${f.name}: payload.${f.name}${f.copy ? "" : ".clone()"}`).join(", ");
  return `//! 🔺️ Diff constructor for \`Set${e.Pascal}\`: a sparse ${lower(e.title)} patch of exactly the provided fields. A provided material
//! must exist and every provided value must keep the ${e.profile ? "profile a valid cross-section" : "dimensions valid"}; providing nothing new is a no-op.

use super::Set${e.Pascal};
use crate::{${imports(used)}};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &Set${e.Pascal}, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.${e.collection}.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("${e.title} \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(material) = payload.material.as_ref().filter(|material| !base.materials.contains_key(*material)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \\"{material}\\" does not exist."), ["material"]);
    }
${check}    let patch = ${e.Pascal}Patch { ${patch} };
    if patch.write(record) == *record {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("${e.title} \\"{}\\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::${e.collection}(payload.id.clone(), Entry::Patched(patch)))
}
`;
}

export function setInverse(e: Entity): string {
  const rows = e.fields.map((f) => `            ${f.name}: payload.${f.name}.as_ref().map(|_| record.${f.name}${f.copy ? "" : ".clone()"}),`).join("\n");
  return `//! ↩️ Inverse of \`Set${e.Pascal}\`: an absolute \`Set${e.Pascal}\` carrying the base values of exactly the fields the payload provides,
//! none when the ${lower(e.title)} is absent.

use super::Set${e.Pascal};
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &Set${e.Pascal}, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.${e.collection}.get(&payload.id) {
        Some(record) => vec![ModelMutation::Set${e.Pascal}(Set${e.Pascal} {
            id: payload.id.clone(),
${rows}
        })],
        None => Vec::new(),
    }
}
`;
}

export const validityModule = `//! ✅️ Authored-value validity rules shared by every leaf that writes a length, a profile or an outline: pure, total and base-free.

use super::{Point2, Profile, Vertex};

const AREA_EPSILON: f64 = 1e-9;

/// 📏️ A finite length above zero.
pub fn is_positive_length(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// 📐️ A finite length of zero or more.
pub fn is_non_negative_length(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn segment_area(from: &Vertex, to: &Vertex) -> f64 {
    if from.bulge == 0.0 {
        return 0.0;
    }
    let sweep = 4.0 * from.bulge.atan();
    let chord = (to.point.x - from.point.x).hypot(to.point.y - from.point.y);
    let half = (sweep / 2.0).sin();
    chord * chord / (8.0 * half * half) * (sweep - sweep.sin())
}

/// 🧮️ Signed area of a closed loop of bulged segments: positive when the loop runs counter-clockwise.
pub fn signed_loop_area(outline: &[Vertex]) -> f64 {
    let count = outline.len();
    (0..count)
        .map(|index| {
            let (from, to) = (&outline[index], &outline[(index + 1) % count]);
            (from.point.x * to.point.y - to.point.x * from.point.y) / 2.0 + segment_area(from, to)
        })
        .sum()
}

fn side(origin: Point2, a: Point2, b: Point2) -> f64 {
    (a.x - origin.x) * (b.y - origin.y) - (a.y - origin.y) * (b.x - origin.x)
}

fn chords_cross(first: (Point2, Point2), second: (Point2, Point2)) -> bool {
    side(second.0, second.1, first.0) * side(second.0, second.1, first.1) < 0.0 && side(first.0, first.1, second.0) * side(first.0, first.1, second.1) < 0.0
}

/// 🔲️ Why a closed outline cannot bound a section, none when it can: at least three finite vertices, no repeated vertex (the
/// loop closes itself), no crossing chords and a counter-clockwise orientation with an area.
pub fn outline_problem(outline: &[Vertex]) -> Option<&'static str> {
    let count = outline.len();
    if count < 3 {
        return Some("A custom outline needs at least three vertices.");
    }
    if outline.iter().any(|vertex| !(vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite())) {
        return Some("A custom outline must be made of finite coordinates and bulges.");
    }
    if (0..count).any(|index| outline[index].point == outline[(index + 1) % count].point) {
        return Some("A custom outline must not repeat a vertex; it closes itself.");
    }
    let chord = |index: usize| (outline[index].point, outline[(index + 1) % count].point);
    if (0..count).any(|first| ((first + 2)..count).filter(|second| (second + 1) % count != first).any(|second| chords_cross(chord(first), chord(second)))) {
        return Some("A custom outline must not cross itself.");
    }
    if signed_loop_area(outline) <= AREA_EPSILON {
        return Some("A custom outline must run counter-clockwise and enclose an area.");
    }
    None
}

/// ▭️ Why a profile cannot be swept, none when it can: positive dimensions, an I web and flanges that fit inside the section,
/// a valid closed outline for a custom profile.
pub fn profile_problem(profile: &Profile) -> Option<&'static str> {
    match profile {
        Profile::Rectangle { width, depth } => (!(is_positive_length(*width) && is_positive_length(*depth))).then_some("A rectangle profile needs a positive width and depth."),
        Profile::Circle { diameter } => (!is_positive_length(*diameter)).then_some("A circle profile needs a positive diameter."),
        Profile::IShape { width, depth, web, flange } => {
            if ![width, depth, web, flange].into_iter().all(|value| is_positive_length(*value)) {
                Some("An I profile needs a positive width, depth, web and flange.")
            } else if *web >= *width || 2.0 * *flange >= *depth {
                Some("The web and flanges of an I profile must fit inside its width and depth.")
            } else {
                None
            }
        }
        Profile::Custom { outline } => outline_problem(outline),
    }
}
`;
