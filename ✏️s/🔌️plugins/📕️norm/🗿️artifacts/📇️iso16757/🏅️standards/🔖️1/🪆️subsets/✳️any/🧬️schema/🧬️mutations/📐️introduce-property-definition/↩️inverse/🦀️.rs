//! ↩️ `introduce-property-definition` — undo is `retire-property-definition`, unless `base` already
//! had this id (then `create` was a no-op and there's nothing to undo).

use crate::retire_property_definition;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::introduce_property_definition::mutation::IntroducePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &IntroducePropertyDefinition, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_property_definition::mutation::RetirePropertyDefinition> {
    Ok((|| {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return Vec::new();
    }
    vec![Op::from(retire_property_definition::mutation::RetirePropertyDefinition { id: payload.property_definition.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
