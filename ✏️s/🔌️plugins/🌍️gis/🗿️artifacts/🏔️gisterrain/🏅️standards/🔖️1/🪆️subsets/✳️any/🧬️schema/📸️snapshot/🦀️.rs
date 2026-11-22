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
use semio_framework_surface::terrain::missing_samples::{self, MAX_MASK_HEX_BYTES};
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

pub const ELEVATION_MIN_SIDE: usize = 2;
pub const ELEVATION_MAX_SIDE: usize = 65;
pub const ELEVATION_MAX_SAMPLES: usize = ELEVATION_MAX_SIDE * ELEVATION_MAX_SIDE;

pub(super) fn elevation_side_in_range(size: usize) -> bool {
    (ELEVATION_MIN_SIDE..=ELEVATION_MAX_SIDE).contains(&size)
}

fn elevation_shape_error() -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(
        semio_framework_value::ValueRefusalKind::InvalidValue,
        "elevation source requires ordered WGS84 bounds, a rectangular grid with 2 to 65 samples per axis and canonical missing samples",
    )
}

/// 🏔️ Authored elevations in signed millimetres, stored south to north and west to east.
#[derive(Clone, Debug, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ElevationSource {
    pub rows_millimetres: Vec<Vec<i32>>,
    pub bounds: [f64; 4],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub missing_samples: Option<String>,
}

impl PartialEq for ElevationSource {
    fn eq(&self, other: &Self) -> bool {
        self.bounds.map(f64::to_bits) == other.bounds.map(f64::to_bits)
            && self.rows_millimetres == other.rows_millimetres
            && self.missing_samples == other.missing_samples
    }
}

#[doc(hidden)]
#[derive(semio_framework_value::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ElevationSourceFields {
    bounds: [f64; 4],
    rows_millimetres: Vec<Vec<i32>>,
    #[value(default)]
    missing_samples: DecodedMissingSamples,
}

impl ElevationSourceFields {
    fn into_source(self) -> ElevationSource {
        ElevationSource { bounds: self.bounds, rows_millimetres: self.rows_millimetres, missing_samples: self.missing_samples.0 }
    }

    fn preflight(
        value: &semio_framework_value::DslValue,
        mut checkpoint: impl FnMut() -> Result<(), semio_framework_value::ValueError>,
    ) -> Result<(), semio_framework_value::ValueError> {
        use semio_framework_value::DslValue;
        let members = value.as_object().ok_or_else(elevation_shape_error)?;
        if !(2..=3).contains(&members.len()) || !members.iter().any(|(name, _)| name == "bounds") {
            return Err(elevation_shape_error());
        }
        let samples = value.get("rowsMillimetres").and_then(DslValue::as_array).ok_or_else(elevation_shape_error)?;
        if !elevation_side_in_range(samples.len()) {
            return Err(elevation_shape_error());
        }
        let columns = samples.first().and_then(DslValue::as_array).map(|row| row.len())
            .filter(|columns| elevation_side_in_range(*columns)).ok_or_else(elevation_shape_error)?;
        if let Some(mask) = value.get("missingSamples") {
            let DslValue::String(mask) = mask else { return Err(elevation_shape_error()); };
            if mask.len() != (columns * samples.len()).div_ceil(8) * 2 { return Err(elevation_shape_error()); }
        }
        for row in samples {
            checkpoint()?;
            if row.as_array().is_none_or(|row| row.len() != columns) {
                return Err(elevation_shape_error());
            }
        }
        Ok(())
    }
}

use semio_framework_value_native_owned::native_decoding::{
    NativeReceive,
    OriginalReceivingPrimary,
    owned::{
        NativeDefault, NativeDefaultValue, NativeDefaultValueCursor, NativeMappedReceiver, NativeReceiveMap, NativeReceiveTryMap, NativeTryMappedReceiver,
        native_admit_default,
        tree::{NativeValueSource, fixed},
    },
};
use semio_framework_value::retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneStep, RetirementDemand};

#[doc(hidden)]
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct DecodedMissingSamples(Option<String>);

impl FromValue for DecodedMissingSamples {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        String::from_value(value).map(Self::from_native)
    }

    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        String::from_value_controlled(value, control).map(Self::from_native)
    }

    fn retire_decoded(self) { Option::<String>::retire_decoded(self.0); }
}

impl NativeDefaultValue for DecodedMissingSamples {
    fn default_demand() -> RetirementDemand { fixed(size_of::<Option<Self>>()) }

    fn default_work_demand() -> Result<u64, semio_framework_value::ValueError> { Ok(1) }

    fn admit_default<'reply>(
        output: &mut Option<Self>,
        effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'reply, RetainedCloneStep, OriginalReceivingPrimary>,
    ) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, OriginalReceivingPrimary>, semio_framework_value::ValueError> {
        native_admit_default(output, Self::default_demand(), Self::default, effect, reply)
    }

    fn recover_default<'reply>(
        _: &mut Option<Self>,
        _: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'reply, RetainedCloneStep, OriginalReceivingPrimary>,
    ) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, OriginalReceivingPrimary>, semio_framework_value::ValueError> {
        Ok(reply.pending())
    }
}

impl NativeDefault for DecodedMissingSamples {
    type Cursor = NativeDefaultValueCursor<Self>;

    fn default_cursor() -> Self::Cursor { Self::Cursor::new() }
}

impl NativeReceiveMap<String> for DecodedMissingSamples {
    fn from_native(value: String) -> Self { Self(Some(value)) }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for DecodedMissingSamples {
    type Receiver = NativeMappedReceiver<I, String, Self>;

    fn receiver() -> Self::Receiver { Self::Receiver::new() }
}

impl NativeReceiveTryMap<ElevationSourceFields> for ElevationSource {
    fn validate_native(value: &ElevationSourceFields) -> Result<(), semio_framework_value::ValueError> {
        Self::validate_parts(value.bounds, &value.rows_millimetres, value.missing_samples.0.as_deref())
    }

    fn from_valid_native(value: ElevationSourceFields) -> Self {
        value.into_source()
    }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for ElevationSource {
    type Receiver = NativeTryMappedReceiver<I, ElevationSourceFields, Self>;

    fn receiver() -> Self::Receiver { Self::Receiver::new() }
}

impl FromValue for ElevationSource {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        ElevationSourceFields::preflight(&value, || Ok(()))?;
        let source = ElevationSourceFields::from_value(value)?.into_source();
        source.validate()?;
        Ok(source)
    }

    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        ElevationSourceFields::preflight(value, || control.checkpoint())?;
        let source = ElevationSourceFields::from_value_controlled(value, control)?.into_source();
        let owner = semio_framework_value::DecodedValue::new(source, Self::retire_decoded);
        owner.get().validate()?;
        control.checkpoint()?;
        Ok(owner.take())
    }

    fn retire_decoded(self) {
        ElevationSourceFields::retire_decoded(ElevationSourceFields {
            bounds: self.bounds, rows_millimetres: self.rows_millimetres, missing_samples: DecodedMissingSamples(self.missing_samples),
        });
    }
}

impl ElevationSource {
    /// ✅️ Refuses invalid elevation sources before publication or reconstruction.
    pub fn validate(&self) -> Result<(), semio_framework_value::ValueError> {
        Self::validate_parts(self.bounds, &self.rows_millimetres, self.missing_samples.as_deref())
    }

    fn validate_parts(bounds: [f64; 4], rows: &[Vec<i32>], mask: Option<&str>) -> Result<(), semio_framework_value::ValueError> {
        Self::valid_parts(bounds, rows, mask).then_some(()).ok_or_else(elevation_shape_error)
    }

    /// 📐️ Returns the column and row counts of a rectangular supported grid.
    pub fn dimensions(&self) -> Option<(usize, usize)> {
        Self::dimensions_of(&self.rows_millimetres)
    }

    fn dimensions_of(rows: &[Vec<i32>]) -> Option<(usize, usize)> {
        let columns = rows.first()?.len();
        (elevation_side_in_range(columns) && elevation_side_in_range(rows.len())
            && rows.iter().all(|row| row.len() == columns)).then_some((columns, rows.len()))
    }

    /// 🧭️ Checks ordered WGS84 sample-centre bounds and grid dimensions.
    pub fn is_valid(&self) -> bool {
        Self::valid_parts(self.bounds, &self.rows_millimetres, self.missing_samples.as_deref())
    }

    fn valid_parts([west, south, east, north]: [f64; 4], rows: &[Vec<i32>], mask: Option<&str>) -> bool {
        (-180.0..east).contains(&west) && east <= 180.0
            && (-90.0..north).contains(&south) && north <= 90.0
            && Self::dimensions_of(rows).is_some_and(|(width, height)| {
                mask.is_none_or(|text| text.len() <= MAX_MASK_HEX_BYTES)
                    && missing_samples::is_valid(mask, width * height, rows.iter().flatten().copied())
            })
    }

    /// 🗺️ Projects the authored samples to the shared terrain grid type.
    pub fn grid(&self) -> semio_framework_surface::terrain::TerrainElevationGrid {
        semio_framework_surface::terrain::TerrainElevationGrid {
            bounds: self.bounds, rows_millimetres: self.rows_millimetres.clone(), missing_samples: self.missing_samples.clone(),
        }
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

