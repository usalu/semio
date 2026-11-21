//! 🧬️ GIS terrain snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `GisTerrainSnapshot` now carries a real `store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>` handle for its
//! `mesh` slot, which `dsl::DslRecord`'s derive cannot represent (no `DslField` impl for
//! `ArtifactChild<S>`) — the same reason `✳️object`/`✳️kit` (stdio) and `💠️lowpoly`/`📐️cad` hand-roll
//! their own codecs rather than deriving. This file follows their exact hex/bracket convention, never
//! a hand-written slot list — `#[derive(ArtifactSchema)]` still emits `field_states()`/the `#[child(…)]`
//! slot table for the top-level facets.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;

use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔹Snapshot
/// 📸️ Persisted GIS terrain document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainSnapshot {
    #[state(artifact)]
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point — last-imported `2d.map` descriptor JSON.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub imported_map: Option<crate::schema::imported_map::ImportedMap>,
    /// 🪆️ Exact independently owned mesh handle, preserved by parent scalar edits.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elevation_source: Option<ElevationSource>,
}

impl Default for GisTerrainSnapshot {
    fn default() -> Self {
        Self { exaggeration: 0.0, imported_map: None, mesh: None, elevation_source: None }
    }
}

/// 🏔️ Authored elevation samples persisted with the terrain document.
#[derive(Clone, Debug, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(deny_unknown_fields)]
pub struct ElevationSource {
    pub bounds: [f64; 4],
    pub rows: Vec<Vec<i16>>,
}

impl PartialEq for ElevationSource {
    fn eq(&self, other: &Self) -> bool {
        self.bounds.map(f64::to_bits) == other.bounds.map(f64::to_bits) && self.rows == other.rows
    }
}

#[doc(hidden)]
#[derive(semio_framework_value::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
pub struct ElevationSourceFields {
    bounds: [f64; 4],
    rows: Vec<Vec<i16>>,
}

use semio_framework_value_native_owned::native_decoding::{
    NativeReceive,
    owned::{NativeReceiveTryMap, NativeTryMappedReceiver, tree::NativeValueSource},
};

impl NativeReceiveTryMap<ElevationSourceFields> for ElevationSource {
    fn validate_native(value: &ElevationSourceFields) -> Result<(), semio_framework_value::ValueError> {
        Self::validate_parts(value.bounds, &value.rows)
    }

    fn from_valid_native(value: ElevationSourceFields) -> Self {
        Self { bounds: value.bounds, rows: value.rows }
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for ElevationSource {
    type Receiver = NativeTryMappedReceiver<I, ElevationSourceFields, Self>;

    fn receiver() -> Self::Receiver { Self::Receiver::new() }
}

impl FromValue for ElevationSource {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let ElevationSourceFields { bounds, rows } = ElevationSourceFields::from_value(value)?;
        let source = Self { bounds, rows };
        source.validate()?;
        Ok(source)
    }

    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        let ElevationSourceFields { bounds, rows } = ElevationSourceFields::from_value_controlled(value, control)?;
        let owner = semio_framework_value::DecodedValue::new(Self { bounds, rows }, Self::retire_decoded);
        owner.get().validate()?;
        control.checkpoint()?;
        Ok(owner.take())
    }

    fn retire_decoded(self) {
        ElevationSourceFields::retire_decoded(ElevationSourceFields { bounds: self.bounds, rows: self.rows });
    }
}

impl ElevationSource {
    /// ✅️ Refuses invalid elevation sources before publication or reconstruction.
    pub fn validate(&self) -> Result<(), semio_framework_value::ValueError> {
        Self::validate_parts(self.bounds, &self.rows)
    }

    fn validate_parts(bounds: [f64; 4], rows: &[Vec<i16>]) -> Result<(), semio_framework_value::ValueError> {
        Self::valid_parts(bounds, rows).then_some(()).ok_or_else(|| semio_framework_value::ValueError::literal(
            semio_framework_value::ValueRefusalKind::InvalidValue,
            "elevation source requires ordered WGS84 bounds and exactly 33 rows of 33 samples",
        ))
    }

    /// 📐️ Checks WGS84 extent and the fixed sample matrix dimensions.
    pub fn is_valid(&self) -> bool {
        Self::valid_parts(self.bounds, &self.rows)
    }

    fn valid_parts([west, south, east, north]: [f64; 4], rows: &[Vec<i16>]) -> bool {
        (-180.0..east).contains(&west) && east <= 180.0
            && (-90.0..north).contains(&south) && north <= 90.0
            && rows.len() == 33 && rows.iter().all(|row| row.len() == 33)
    }

    /// 🗺️ Projects the authored samples to the shared terrain grid type.
    pub fn grid(&self) -> semio_framework_surface::terrain::TerrainElevationGrid {
        semio_framework_surface::terrain::TerrainElevationGrid { bounds: self.bounds, rows: self.rows.clone() }
    }
}
//#endregion 🔹Snapshot

//#region 🔖️CodecPrimitives












//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives


//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives














//#endregion 🔖️BinaryPrimitives



//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge

#[path="🧮️row-admission/🦀️.rs"]
pub mod row_admission;
pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::schema::snapshot::value_admission;

