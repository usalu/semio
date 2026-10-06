//! 🧬️ Cad snapshot schema — artifact-lane fields only.

use crate::{empty_cad_snapshot, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::CadReferenceIndex;




//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the four per-pane object/geometry field
/// pairs that used to duplicate `SemioBrepSnapshot`'s topology inline (`CadObject`/`CadGeometry` at
/// `crate::🦀️.rs`) are replaced by four fixed composed
/// `s.stdio.semio.model` CHILD slots — one per `CadPaneId` — plus a forward `drawings` composition
/// slot per the design map's `cad | engineering assembly | model, drawing` row. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
///
/// 🛡️ `deny_unknown_fields` closes that replacement: a snapshot still carrying the retired inline
/// `objects`/`shapeGeometry`/`activeModelDefinitionId` keys must FAIL to decode, never decode with
/// them silently dropped (`🧫️fixtures/🪪️document`'s `invalidDocuments`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "cad")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[value(default)]
    #[state(artifact)]
    pub references_by_model_definition_id: CadReferenceIndex,
    #[value(default)]
    #[state(artifact)]
    pub nodes: Vec<CadNode>,
}

//#region 🔖️ExactChildren



//#endregion 🔖️ExactChildren

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot
