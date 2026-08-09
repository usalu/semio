//! 🧬️ Present app presence schema — shared live ephemeral state.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Presence
/// 👥️ Animate present presence — peer tile selection visible live, not persisted in the deck.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.animate.present.presence")]
pub struct PresentPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
}
//#endregion 🔖️Presence
