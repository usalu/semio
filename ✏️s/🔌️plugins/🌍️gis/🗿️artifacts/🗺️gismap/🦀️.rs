//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

/// 📸️ Persisted GIS map snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
pub use crate::schema::snapshot::GisMapSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Constants

pub const GIS_MAP_SCHEMA: &str = "gis.map";

/// 🪪️ One canonical map identity shared by definition, composer and both app roles.
pub const GISMAP_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.gis.gismap", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
//#endregion 🔹Constants

//#region 🔹Types
/// 🗺️ One id-keyed spatial feature carried as its full opaque descriptor payload.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch.
    pub data: dsl::DslValue,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MapFeaturePatch {
    pub data: Option<dsl::DslValue>,
}

impl Patchable<MapFeaturePatch> for MapFeature {
    fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<MapFeaturePatch> {
        (self.data != other.data).then(|| MapFeaturePatch { data: Some(other.data.clone()) })
    }
}

//#endregion 🔹Types

//#region 🔖️Composition
/// 🧩️ Composed `s.stdio.semio.drawing`/`s.stdio.semio.image`/`s.stdio.semio.value` child slots
/// (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, design map §4: "map C:drawing+image+value").
/// `positions`/`routes`/`regions` stay gis's own domain-specific id-keyed feature lists (analogous to
/// `📐️cad`'s `nodes`/`references_by_model_definition_id`, kept inline rather than gutted — see that
/// plugin's `🧬️schema/📸️snapshot/🦀️.rs` module doc for the precedent) since they are NOT a
/// duplicated stdio type, just gis's own vocabulary; `drawing`/`value` are DERIVED composed children
/// with stable admitted member identities. Their typed stores move content while the parent handles
/// remain fixed; `gis_map_snapshot_with_derived_children` enforces those identities whenever the
/// parent is constructed or changed. New maps have no `image`: gis
/// carries no raster basemap capability today (see `render_mode`'s app-level raster/vector TOGGLE,
/// which selects a rendering STYLE of the same vector data, not a second raster document) — the slot
/// exists as a typed member. A supplied image handle is preserved; image-free proposal planning
/// rejects such a map without changing or dropping that member.
pub type GisMapDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>;
pub type GisMapImageChild = store::ArtifactChild<SemioImageSnapshot>;
pub type GisMapValueChild = store::ArtifactChild<SemioValueSnapshot>;

/// 🕸️ Stable admitted CHILD handle for the map's composed drawing member.
pub fn gis_map_drawing_child_handle() -> GisMapDrawingChild {
    let child_id = "gismap-drawing".to_string();
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gismap-drawing".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Stable admitted CHILD handle for the map's composed value member.
pub fn gis_map_value_child_handle() -> GisMapValueChild {
    let child_id = "gismap-value".to_string();
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "value".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gismap-value".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🌉️ WRITE direction, real (not a stub): `serde_json::Value` → `SemioValue`, a direct structural
/// mapping (json has no binary/graph-reference primitive, so `Bytes`/`Ref` are never produced —
/// mirrors stdio's own `semio_value_from_json` for its `json` artifact, written locally here since
/// that one converts stdio's OWN `JsonValue` AST, not `serde_json::Value`, and gis already speaks
/// `serde_json::Value` everywhere else in this file).
pub fn semio_value_from_serde_json(value: &serde_json::Value) -> SemioValue {
    match value {
        serde_json::Value::Null => SemioValue::Null,
        serde_json::Value::Bool(value) => SemioValue::Bool { value: *value },
        serde_json::Value::Number(number) => {
            let lexeme = number.to_string();
            if lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E') {
                SemioValue::Float { lexeme }
            } else {
                SemioValue::Int { lexeme }
            }
        }
        serde_json::Value::String(value) => SemioValue::Str { value: value.clone() },
        serde_json::Value::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_serde_json).collect() },
        serde_json::Value::Object(members) => SemioValue::Map { entries: members.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_serde_json(value) }).collect() },
    }
}

/// 🌉️ READ direction, real (not a stub): the exact inverse of `semio_value_from_serde_json`. `Ref`
/// (this format's graph-reference variant) never appears in content this bridge itself produces —
/// resolved defensively to `Null` rather than panicking, matching the honesty convention this
/// ticket's other converters use for out-of-scope input shapes.
pub fn serde_json_from_semio_value(value: &SemioValue) -> serde_json::Value {
    match value {
        SemioValue::Null => serde_json::Value::Null,
        SemioValue::Bool { value } => serde_json::Value::Bool(*value),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => serde_json::from_str(lexeme).unwrap_or(serde_json::Value::Null),
        SemioValue::Str { value } => serde_json::Value::String(value.clone()),
        SemioValue::Bytes { .. } => serde_json::Value::Null,
        SemioValue::List { items } => serde_json::Value::Array(items.iter().map(serde_json_from_semio_value).collect()),
        SemioValue::Map { entries } => serde_json::Value::Object(entries.iter().map(|entry| (entry.key.clone(), serde_json_from_semio_value(&entry.value))).collect()),
        SemioValue::Ref { .. } => serde_json::Value::Null,
    }
}

/// 🌉️ Builds the map's composed `value` child content — the lossless `{positions,routes,regions}`
/// descriptor JSON (`gis_map_descriptor_json`) lifted into a real `SemioValueSnapshot` graph.
pub fn gis_map_value_from_descriptor_json(descriptor_json: &str) -> SemioValueSnapshot {
    let value: serde_json::Value = serde_json::from_str(descriptor_json).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_serde_json(&value), nodes: Vec::new() }
}

/// 🌉️ The exact inverse of `gis_map_value_from_descriptor_json` — recovers the descriptor JSON a
/// `value` child's content actually carries.
pub fn gis_map_descriptor_json_from_value(value: &SemioValueSnapshot) -> String {
    serde_json_from_semio_value(&value.root).to_string()
}

/// 🔄️ Assigns the stable drawing/value coordinates used by [`GisMapSnapshot::default`].
/// A supplied image handle is preserved; content changes belong to typed child mutations.
pub fn gis_map_snapshot_with_derived_children(mut document: GisMapSnapshot) -> GisMapSnapshot {
    document.drawing = gis_map_drawing_child_handle();
    document.value = gis_map_value_child_handle();
    document
}
//#endregion 🔖️Composition

//#region 🔹ArtifactKind
/// 🗺️ The canonical GIS map artifact-kind declaration; payload schemas remain independent.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: GISMAP_DIALECT.artifact_kind.into(),
        name: "2D Map".into(),
        source_format: GIS_MAP_SCHEMA.into(),
        component_kind: "gismap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: GIS_MAP_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
    }
}
//#endregion 🔹ArtifactKind

//#region 💡️InferenceService
/// 💡️ The single executable whole-map inference matching the declared schema family.
pub fn gis_map_inference_service() -> semio_framework_plugin::ArtifactInferenceService {
    semio_framework_plugin::ArtifactInferenceService::new(
        semio_framework_plugin::ArtifactInferenceServiceMetadata {
            owner: "gis",
            artifact_kind: "s.gis.gismap",
            artifact_schema: "s.gis.gismap",
            artifact_schema_version: 1,
            document_schema: GIS_MAP_SCHEMA,
            document_schema_version: 1,
            inference_schema: "s.gis.gismap.inference",
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
        },
        infer_gis_map,
    )
}

/// 🚦️ Rejects invalid identities, cache modes, and allocation bounds before snapshot decoding.
fn admit_gis_map_inference_request(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<usize, semio_framework_plugin::ArtifactInferenceExecutionError> {
    if request.cancellation_id.trim().is_empty() {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.cancellation", "cancellation identity is required"));
    }
    if request.budgets.allocation_bytes == 0 || request.budgets.work_units == 0 || request.budgets.recursion_depth == 0 {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "allocation, work, and recursion budgets must be non-zero"));
    }
    if request.requested_cache_mode == semio_framework_plugin::WireArtifactInferenceCacheMode::Incremental {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.cache-mode", "gismap inference has no incremental algorithm"));
    }
    let request_bytes = request
        .policy
        .len()
        .saturating_add(request.canonical_payload.len())
        .saturating_add(request.previous_state.map_or(0, <[u8]>::len))
        .saturating_add(request.dependencies.iter().fold(0usize, |total, (owner, payload)| total.saturating_add(owner.len()).saturating_add(payload.len())));
    let allocation = usize::try_from(request.budgets.allocation_bytes).map_err(|_| semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "allocation budget exceeds this runtime's address space"))?;
    if request_bytes > allocation {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", format!("request consumes {request_bytes} bytes, above allocation limit {allocation}")));
    }
    Ok(allocation)
}

struct InferenceOutputGuard(Vec<u8>);

impl Drop for InferenceOutputGuard {
    fn drop(&mut self) {
        let pointer = self.0.as_mut_ptr();
        for index in 0..self.0.capacity() {
            unsafe {
                std::ptr::write_volatile(pointer.add(index), 0);
            }
        }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

/// ⏱️ The native GIS service with request-owned cancellation and progress at every value.
pub fn infer_gis_map_controlled(
    request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>,
    checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>,
) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    use crate::standards::v1::subsets::any::schema::inferences::{bounds::controlled_lon_lat_bounds, GisMapInference};
    let allocation = admit_gis_map_inference_request(request)?;
    checkpoint(0)?;
    let snapshot = <GisMapSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload).map_err(|_| semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.snapshot-decode", "invalid canonical map snapshot"))?;
    let mut work = 1u64;
    checkpoint(work)?;
    let bounds = controlled_lon_lat_bounds(&snapshot, &mut |depth| {
        if depth > request.budgets.recursion_depth || depth > 64 {
            return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "feature nesting exceeds the bounded inference depth"));
        }
        work = work.checked_add(1).ok_or_else(|| semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "work counter exhausted"))?;
        if work > request.budgets.work_units {
            return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "inference work budget exhausted"));
        }
        checkpoint(work)
    })?;
    let inference = GisMapInference { position_count: snapshot.positions.len(), route_count: snapshot.routes.len(), region_count: snapshot.regions.len(), bounds };
    let mut canonical_payload = InferenceOutputGuard(semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework_os_kernel::ToValue::to_value(&inference)));
    if canonical_payload.0.len() > allocation {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "inference result exceeds allocation budget"));
    }
    checkpoint(work)?;
    Ok(semio_framework_plugin::ArtifactInferenceExecution {
        canonical_payload: std::mem::take(&mut canonical_payload.0),
        diagnostics: Vec::new(),
        validity: "valid".into(),
        quality: "exact".into(),
        complete: true,
        actual_cache_mode: request.requested_cache_mode.clone(),
    })
}

/// 🧠️ Whole-map native entry uses the same controlled fold with its finite request budgets.
fn infer_gis_map(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    infer_gis_map_controlled(request, &mut |_| Ok(()))
}
//#endregion 💡️InferenceService

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// the plugin root's `register_gis_exports` fan-out. Relocated from `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes the artifact
/// (kind, schema, io ports, ownership), which is not engine behaviour.
/// 🧾️ Defines s.gis.gismap's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.gis.gismap")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.gis.gismap")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.gis.gismap.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.gis.gismap@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gis.gismap@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.dxf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dxf@r12/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dxf@r12/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"gis.map:gismap")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "gis.map")?)?
                .claim(ArtifactIdentityClaim::codec_extension("gis.map", "gismap")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Map")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "GIS Map")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Karte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Karte")?)?,
        )
}

/// 🔖️ Assembles s.gis.gismap's typed runtime declaration.
#[cfg(feature = "component-app-assembly")]
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::gismap_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .inference_services([gis_map_inference_service()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::gis2d::Gis2dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-position/🧪️tests/💡️adds-lighthouse-685c1e/🦀️.rs"]
                            mod tests_adds_a_lighthouse_position_after_the_harbor;
                        }
                        #[path = "."]
                        pub mod delete_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-position/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-position/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-position/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-position/🧪️tests/🚫️removes-lighthouse-cbbd5c/🦀️.rs"]
                            mod tests_removes_the_lighthouse_position;
                        }
                        #[path = "."]
                        pub mod replace_position_data {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-position-data/🧪️tests/⚓️rewrites-harbor-0bbbf6/🦀️.rs"]
                            mod tests_rewrites_the_harbor_position_payload;
                        }
                        #[path = "."]
                        pub mod reorder_positions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-positions/🧪️tests/⚓️moves-harbor-e3b5fa/🦀️.rs"]
                            mod tests_moves_the_harbor_position_to_the_end;
                        }
                        #[path = "."]
                        pub mod create_route {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️create-route/🧪️tests/🚋️adds-tram-route-after-ferry/🦀️.rs"]
                            mod tests_adds_a_tram_route_after_the_ferry;
                        }
                        #[path = "."]
                        pub mod delete_route {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-route/🧪️tests/🚫️removes-tram-route/🦀️.rs"]
                            mod tests_removes_the_tram_route;
                        }
                        #[path = "."]
                        pub mod replace_route_data {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-route-data/🧪️tests/⛴️rewrites-ferry-635859/🦀️.rs"]
                            mod tests_rewrites_the_ferry_route_payload;
                        }
                        #[path = "."]
                        pub mod reorder_routes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭reorder-routes/🧪️tests/🚌️moves-bus-route-to-front/🦀️.rs"]
                            mod tests_moves_the_bus_route_to_the_front;
                        }
                        #[path = "."]
                        pub mod create_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐create-region/🧪️tests/🏘️adds-old-town-region-7b690a/🦀️.rs"]
                            mod tests_adds_the_old_town_region_after_the_harbor_district;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-region/🧪️tests/🚫️removes-old-town-region/🦀️.rs"]
                            mod tests_removes_the_old_town_region;
                        }
                        #[path = "."]
                        pub mod replace_region_data {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-region-data/🧪️tests/🏘️rewrites-harbor-030143/🦀️.rs"]
                            mod tests_rewrites_the_harbor_district_region_payload;
                        }
                        #[path = "."]
                        pub mod reorder_regions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-regions/🧪️tests/🌳️moves-park-region-0540b5/🦀️.rs"]
                            mod tests_moves_the_park_region_between_the_two_districts;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}
pub use crate::standards::v1::subsets::any::schema::diff::GisMapDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗺️maphost/🦀️.rs"]
        pub mod maphost;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod examples {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
            pub mod demo_session;
            #[cfg(test)]
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
            mod demo_session_tests;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🦀️.rs"]
            pub mod example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🦀️.rs"]
            pub mod features;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💡️inference/🦀️.rs"]
            pub mod inference;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️shell/🦀️.rs"]
            pub mod shell;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️view/🦀️.rs"]
            pub mod view;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod map {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/📏️layer-weights/🦀️.rs"]
                            pub mod layer_weights;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/👁️layers/🦀️.rs"]
                            pub mod layers;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🔽️lod-mode/🦀️.rs"]
                            pub mod lod_mode;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🖼️render-mode/🦀️.rs"]
                            pub mod render_mode;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🎨️vector-style/🦀️.rs"]
                            pub mod vector_style;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gismap {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🦀️.rs"]
                    pub mod map;
                }
            }
        }
    }
}
