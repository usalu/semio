//! ✏️ The edits of a family the window offers: its name and category, a parameter (add one of a kind, rewrite its formula or its kind, remove it) and a solid (add a cuboid, extrusion, revolution or sweep,
//! rename it, rewrite one of its formulas, turn the axis of a revolution, remove it). Each edit is a pure function of the family and yields the mutations that carry exactly the change; whether the
//! result stands (a formula that parses, names only existing parameters and closes no circle) is decided by the mutation, not here. Formulas are canonicalised before they are emitted, so text that does
//! not parse never reaches a mutation.

use crate::mutations::create_family_solid::CreateFamilySolid;
use crate::mutations::delete_family_solid::DeleteFamilySolid;
use crate::mutations::remove_family_parameter::RemoveFamilyParameter;
use crate::mutations::set_family::SetFamily;
use crate::mutations::set_family_parameter::SetFamilyParameter;
use crate::mutations::set_family_solid::SetFamilySolid;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{ExprPoint, ExprPoint3, FamilyCategory, FamilySolid, ModelMutation, ModelSnapshot, ParameterKind, ParametricProfile, SolidAxis, SolidShape};

/// 🧩️ One edit as the command carries it: the part (`family`, `parameter`, `solid`), the operation, the key (a parameter name, a parameter kind token, a solid id with a slot, a shape token) and the value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Edit {
    pub part: String,
    pub op: String,
    pub key: String,
    pub value: String,
}

/// 🔤️ The tokens of the parameter kinds, the categories, the solid shapes and the axes, in display order.
pub const KINDS: [ParameterKind; 7] = [ParameterKind::Length, ParameterKind::Angle, ParameterKind::Real, ParameterKind::Integer, ParameterKind::Boolean, ParameterKind::Text, ParameterKind::Material];
pub const CATEGORIES: [FamilyCategory; 9] = [FamilyCategory::Furniture, FamilyCategory::Equipment, FamilyCategory::Casework, FamilyCategory::Plumbing, FamilyCategory::Lighting, FamilyCategory::Mechanical, FamilyCategory::Electrical, FamilyCategory::Generic, FamilyCategory::Profile];
pub const SHAPES: [&str; 4] = ["Cuboid", "Extrusion", "Revolution", "Sweep"];
pub const AXES: [SolidAxis; 3] = [SolidAxis::X, SolidAxis::Y, SolidAxis::Z];

fn token_of<T: std::fmt::Debug + Copy>(all: &[T], text: &str) -> Option<T> {
    all.iter().copied().find(|candidate| format!("{candidate:?}").eq_ignore_ascii_case(text.trim()))
}

/// 🔤️ The parameter kind a token names.
pub fn kind_of(text: &str) -> Option<ParameterKind> {
    token_of(&KINDS, text)
}

/// 🔤️ The family category a token names.
pub fn category_of(text: &str) -> Option<FamilyCategory> {
    token_of(&CATEGORIES, text)
}

/// 🔤️ The shape token of a solid.
pub fn shape_token(shape: &SolidShape) -> &'static str {
    match shape {
        SolidShape::Cuboid { .. } => SHAPES[0],
        SolidShape::Extrusion { .. } => SHAPES[1],
        SolidShape::Revolution { .. } => SHAPES[2],
        SolidShape::Sweep { .. } => SHAPES[3],
    }
}

fn canonical(text: &str) -> Result<String, &'static str> {
    formula::canonical(text).map_err(|_| "bim.family.formula-invalid")
}

fn quoted(id: &str) -> Result<String, &'static str> {
    (!id.contains(['"', '\\'])).then(|| format!("\"{id}\"")).ok_or("bim.family.material-unquotable")
}

/// 🧱️ The formula a new parameter of `kind` starts with: a metre, a zero angle, one, true, a text or the first material of the model.
pub fn default_formula(snapshot: &ModelSnapshot, kind: ParameterKind) -> Result<String, &'static str> {
    match kind {
        ParameterKind::Length => Ok("1 m".to_string()),
        ParameterKind::Angle => Ok("0 deg".to_string()),
        ParameterKind::Real | ParameterKind::Integer => Ok("1".to_string()),
        ParameterKind::Boolean => Ok("true".to_string()),
        ParameterKind::Text => Ok("\"text\"".to_string()),
        ParameterKind::Material => snapshot.materials.keys().next().ok_or("bim.family.material-missing").and_then(|id| quoted(id)),
    }
}

fn fresh_name(snapshot: &ModelSnapshot, family: &str, kind: ParameterKind) -> String {
    let base = format!("new_{}", format!("{kind:?}").to_ascii_lowercase());
    let taken = |name: &str| snapshot.family_parameters.contains_key(&formula::parameter_id(family, name));
    std::iter::once(base.clone()).chain((2..).map(|n| format!("{base}_{n}"))).find(|name| !taken(name)).expect("an endless range yields a free name")
}

fn point(x: &str, y: &str) -> ExprPoint {
    ExprPoint { x: x.into(), y: y.into() }
}

fn polygon() -> ParametricProfile {
    ParametricProfile::Polygon { points: vec![point("0 m", "0 m"), point("50 mm", "0 m"), point("50 mm", "100 mm"), point("0 m", "100 mm")] }
}

/// 🧊️ The shape a new solid of `token` starts with: a tenth of a metre, all as literal formulas.
pub fn default_shape(token: &str) -> Option<SolidShape> {
    let square = || ParametricProfile::Rectangle { width: "100 mm".into(), depth: "100 mm".into() };
    Some(match token {
        "Cuboid" => SolidShape::Cuboid { x: "0 m".into(), y: "0 m".into(), z: "0 m".into(), width: "100 mm".into(), depth: "100 mm".into(), height: "100 mm".into() },
        "Extrusion" => SolidShape::Extrusion { profile: square(), base: "0 m".into(), height: "100 mm".into() },
        "Revolution" => SolidShape::Revolution { profile: polygon(), axis: SolidAxis::Z, angle: "360 deg".into() },
        "Sweep" => SolidShape::Sweep { profile: ParametricProfile::Rectangle { width: "50 mm".into(), depth: "50 mm".into() }, path: vec![point("0 m", "0 m"), point("1 m", "0 m")] },
        _ => return None,
    })
}

fn fresh_solid_name(snapshot: &ModelSnapshot, family: &str, token: &str) -> String {
    let taken = |name: &str| snapshot.family_solids.values().any(|row| row.family == family && row.name == name);
    (1..).map(|n| format!("{token} {n}")).find(|name| !taken(name)).expect("an endless range yields a free name")
}

fn parameter(snapshot: &ModelSnapshot, family: &str, edit: &Edit) -> Result<Vec<ModelMutation>, &'static str> {
    let set = |name: String, kind: Option<ParameterKind>, value: Option<String>| ModelMutation::SetFamilyParameter(SetFamilyParameter { family: family.to_string(), name, kind, value });
    match edit.op.as_str() {
        "formula" => Ok(vec![set(edit.key.clone(), None, Some(canonical(&edit.value)?))]),
        "kind" => Ok(vec![set(edit.key.clone(), Some(kind_of(&edit.value).ok_or("bim.family.kind-unknown")?), None)]),
        "remove" => Ok(vec![ModelMutation::RemoveFamilyParameter(RemoveFamilyParameter { family: family.to_string(), name: edit.key.clone() })]),
        "add" => {
            let kind = kind_of(&edit.key).ok_or("bim.family.kind-unknown")?;
            Ok(vec![set(fresh_name(snapshot, family, kind), Some(kind), Some(canonical(&default_formula(snapshot, kind)?)?))])
        }
        _ => Err("bim.family.operation-unknown"),
    }
}

fn changed<T: PartialEq + Clone>(before: &T, after: &T) -> Option<T> {
    (before != after).then(|| after.clone())
}

fn solid_patch(id: &str, before: &FamilySolid, after: &FamilySolid) -> SetFamilySolid {
    SetFamilySolid { id: id.to_string(), name: changed(&before.name, &after.name), shape: changed(&before.shape, &after.shape), material: changed(&before.material, &after.material), visible: changed(&before.visible, &after.visible), offset: changed(&before.offset, &after.offset) }
}

fn solid(snapshot: &ModelSnapshot, family: &str, edit: &Edit, fresh: &str) -> Result<Vec<ModelMutation>, &'static str> {
    if edit.op == "add" {
        let shape = default_shape(&edit.key).ok_or("bim.family.shape-unknown")?;
        let material = snapshot.materials.keys().next().ok_or("bim.family.material-missing").and_then(|id| quoted(id))?;
        let row = FamilySolid { family: family.to_string(), name: fresh_solid_name(snapshot, family, &edit.key), shape, material, visible: "true".into(), offset: ExprPoint3 { x: "0 m".into(), y: "0 m".into(), z: "0 m".into() } };
        return Ok(vec![ModelMutation::CreateFamilySolid(CreateFamilySolid { id: fresh.to_string(), solid: row })]);
    }
    let (id, field) = edit.key.split_once('#').unwrap_or((edit.key.as_str(), ""));
    let before = snapshot.family_solids.get(id).filter(|row| row.family == family).ok_or("bim.family.solid-missing")?;
    let mut after = before.clone();
    match edit.op.as_str() {
        "remove" => return Ok(vec![ModelMutation::DeleteFamilySolid(DeleteFamilySolid { id: id.to_string() })]),
        "name" => after.name.clone_from(&edit.value),
        "slot" => {
            if !formula::set_slot(&mut after, field, &canonical(&edit.value)?) {
                return Err("bim.family.slot-unknown");
            }
        }
        "axis" => match &mut after.shape {
            SolidShape::Revolution { axis, .. } => *axis = token_of(&AXES, &edit.value).ok_or("bim.family.axis-unknown")?,
            _ => return Err("bim.family.axis-unavailable"),
        },
        _ => return Err("bim.family.operation-unknown"),
    }
    Ok(vec![ModelMutation::SetFamilySolid(solid_patch(id, before, &after))])
}

/// ✏️ The mutations of `edit` on `family` (`fresh` is the id a new solid takes), or the code of the reason it does not apply.
pub fn apply(snapshot: &ModelSnapshot, family: &str, edit: &Edit, fresh: &str) -> Result<Vec<ModelMutation>, &'static str> {
    if !snapshot.families.contains_key(family) {
        return Err("bim.family.missing");
    }
    match edit.part.as_str() {
        "family" => match edit.op.as_str() {
            "rename" => Ok(vec![ModelMutation::SetFamily(SetFamily { id: family.to_string(), name: Some(edit.value.clone()), category: None })]),
            "category" => Ok(vec![ModelMutation::SetFamily(SetFamily { id: family.to_string(), name: None, category: Some(category_of(&edit.value).ok_or("bim.family.category-unknown")?) })]),
            _ => Err("bim.family.operation-unknown"),
        },
        "parameter" => parameter(snapshot, family, edit),
        "solid" => solid(snapshot, family, edit, fresh),
        _ => Err("bim.family.part-unknown"),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
