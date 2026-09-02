//! 🧬️ Present app presence schema — shared live ephemeral state.

use schema::ArtifactSchema;

//#region 🔖️Presence
/// 👥️ Animate present presence — tile selection broadcasts through the framework's typed
/// `PresenceInteraction` now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); no
/// app-specific ephemeral field remains.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.animate.present.presence")]
pub struct PresentPresence {}
//#endregion 🔖️Presence
