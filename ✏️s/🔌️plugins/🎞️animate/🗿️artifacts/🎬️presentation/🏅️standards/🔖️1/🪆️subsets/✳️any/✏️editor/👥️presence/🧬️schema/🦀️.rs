//! 🧬️ Presentation app presence schema — shared live ephemeral state.

use schema::ArtifactSchema;

//#region 🔖️Presence
/// 👥️ Animate presentation presence — tile selection broadcasts through the framework's typed
/// `PresenceInteraction` now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); no
/// app-specific ephemeral field remains.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.animate.presentation.presence")]
pub struct PresentationPresence {}
//#endregion 🔖️Presence
