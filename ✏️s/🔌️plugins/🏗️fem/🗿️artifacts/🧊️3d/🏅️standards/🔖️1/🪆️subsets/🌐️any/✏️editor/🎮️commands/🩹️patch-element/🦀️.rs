//! 🩹️ Fem3d play app command — `patch-element`: one-field edit of a member (`kind`, `start`, `end`,
//! `materialId`, `sectionId`, `roll`) → `ReplaceElement`.

use crate::standards::v1::subsets::any::schema::mutations::replace_element;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{element_id, FemElement};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchElement
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-element")]
pub struct PatchElement {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🔩️ The same member re-spelled as the other variant: a `Frame` becomes a `Bar` by dropping its
/// roll, a `Bar` becomes a `Frame` with roll zero.
fn with_kind(element: &FemElement, kind: &str) -> Result<FemElement, Fault> {
    let (id, start, end, material_id, section_id, roll) = match element {
        FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, 0.0),
        FemElement::Frame { id, start, end, material_id, section_id, roll } => (id, start, end, material_id, section_id, *roll),
    };
    match kind {
        "bar" => Ok(FemElement::Bar { id: id.clone(), start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() }),
        "frame" => Ok(FemElement::Frame { id: id.clone(), start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone(), roll }),
        _ => Err(Fault::from("fem3d.patch.element-value")),
    }
}

pub fn handle(payload: &PatchElement, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let element = doc.snapshot.elements.iter().find(|element| element_id(element) == payload.id).ok_or_else(|| Fault::from("fem3d.patch.element-missing"))?;
    let mut new_element = element.clone();
    let value = payload.value.trim();
    match (&mut new_element, payload.field.as_str()) {
        (_, "kind") => new_element = with_kind(element, value)?,
        (FemElement::Bar { start, .. } | FemElement::Frame { start, .. }, "start") => *start = value.to_string(),
        (FemElement::Bar { end, .. } | FemElement::Frame { end, .. }, "end") => *end = value.to_string(),
        (FemElement::Bar { material_id, .. } | FemElement::Frame { material_id, .. }, "materialId") => *material_id = value.to_string(),
        (FemElement::Bar { section_id, .. } | FemElement::Frame { section_id, .. }, "sectionId") => *section_id = value.to_string(),
        (FemElement::Frame { roll, .. }, "roll") => *roll = value.parse().map_err(|_| Fault::from("fem3d.patch.element-value"))?,
        _ => return Err(Fault::from("fem3d.patch.element-field")),
    }
    if &new_element == element {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceElement(replace_element::ReplaceElement { id: payload.id.clone(), new_element: Box::new(new_element) })]))
}
//#endregion 🔖️PatchElement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
