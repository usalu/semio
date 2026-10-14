//! ↩️ `retire-property-definition` — undo re-`create`s the definition from BASE state, at its
//! original index; missing id ⇒ `Vec::new()`.

use crate::introduce_property_definition;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::retire_property_definition::mutation::RetirePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RetirePropertyDefinition, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_property_definition::mutation::IntroducePropertyDefinition> {
    Ok((|| {
    let Some(position) = base.catalogue.property_definitions.iter().position(|definition| definition.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_property_definition::mutation::IntroducePropertyDefinition { property_definition: base.catalogue.property_definitions[position].clone(), index: Some(position) })]

    })())
}
//#endregion 🔖️Inverse
