//! 🖇️ CAD mutation — `ReplaceReferenceMedia` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖇️ Whole-value swap of a reference overlay's media-identity/appearance bundle
/// (`source_url`/`media_kind`/`orientation`/`scale`/`opacity`) — the rarely-touched fields no
/// editor gesture sets independently, unlike `hidden`/`locked`/`width_world`/`origin`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-reference-media")]
pub struct ReplaceReferenceMedia {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_source_url: String,
    pub new_media_kind: String,
    pub new_orientation: Option<[f64; 4]>,
    pub new_scale: Option<f64>,
    pub new_opacity: Option<f64>,
}

impl MutationKind<CadSnapshot, CadMutation> for ReplaceReferenceMedia {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "reference", kind: "replace-reference-media", record: "ReplacedReferenceMedia" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace media of reference \"{}\"", self.reference_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
