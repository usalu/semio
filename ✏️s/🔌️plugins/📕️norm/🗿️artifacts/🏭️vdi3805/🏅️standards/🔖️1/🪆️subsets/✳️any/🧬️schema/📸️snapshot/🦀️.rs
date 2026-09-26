//! 🧬️ Vdi3805 snapshot schema — artifact-lane fields only.

use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.vdi3805", layout = "lines")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Snapshot {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}
crate::impl_norm_artifact_record!(Vdi3805Snapshot, extension = "vdi3805", envelope_id = "norm.vdi3805");

impl Default for Vdi3805Snapshot {
    fn default() -> Self {
        crate::reference_fixture()
    }
}

pub fn encode_vdi3805_snapshot_json(snapshot: &Vdi3805Snapshot) -> String { pack::json::to_json_string(snapshot) }
pub fn decode_vdi3805_snapshot_json(text: &str) -> Result<Vdi3805Snapshot, String> { pack::json::from_json_str(text).map_err(|e| e.to_string()) }
pub fn decode_vdi3805_dsl(text: &str) -> Result<Vdi3805Snapshot, String> { <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| format!("{e:?}")) }
pub fn encode_vdi3805_dsl(snapshot: &Vdi3805Snapshot) -> String { store::ArtifactDsl::print_dsl(snapshot) }
pub fn decode_vdi3805_pack(bytes: &[u8]) -> Result<Vdi3805Snapshot, String> { <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| format!("{e:?}")) }
pub fn encode_vdi3805_pack(snapshot: &Vdi3805Snapshot) -> Vec<u8> { store::ArtifactPack::encode_pack(snapshot) }
