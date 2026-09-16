//! 🩹️ Fem2d play app command — `patch-element`: one-field edit of an element (`kind`, `start`, `end`, `materialId`, `sectionId`) → `ReplaceElement`.

use crate::standards::v1::subsets::any::schema::mutations::replace_element;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::{element_id, FemElement};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchElement
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-element")]
pub struct PatchElement {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Re-points one element reference, or swaps `Bar`↔`Beam` while keeping the element's own id and
/// every other field — `FemElement`'s two variants carry the identical five fields, so a `kind` edit
/// is a variant swap and nothing else. Emitted as one `ReplaceElement`.
pub fn handle(payload: &PatchElement, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let element = doc.snapshot.elements.iter().find(|element| element_id(element) == payload.id).ok_or_else(|| Fault::from("fem2d.patch.element-missing"))?;
    let (id, start, end, material_id, section_id, beam) = match element {
        FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, false),
        FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, true),
    };
    let (id, mut start, mut end, mut material_id, mut section_id, mut beam) = (id.clone(), start.clone(), end.clone(), material_id.clone(), section_id.clone(), beam);
    match payload.field.as_str() {
        "kind" => {
            beam = match payload.value.trim() {
                "beam" => true,
                "bar" => false,
                _ => return Err(Fault::from("fem2d.patch.element-value")),
            }
        }
        "start" => start = payload.value.clone(),
        "end" => end = payload.value.clone(),
        "materialId" => material_id = payload.value.clone(),
        "sectionId" => section_id = payload.value.clone(),
        _ => return Err(Fault::from("fem2d.patch.element-field")),
    }
    let new_element = if beam { FemElement::Beam { id, start, end, material_id, section_id } } else { FemElement::Bar { id, start, end, material_id, section_id } };
    if &new_element == element {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceElement(replace_element::ReplaceElement { id: payload.id.clone(), new_element: Box::new(new_element) })]))
}
//#endregion 🔖️PatchElement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
