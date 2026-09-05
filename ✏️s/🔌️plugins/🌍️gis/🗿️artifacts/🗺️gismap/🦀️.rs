//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).

use semio_framework_value_derive::{FromValue, ToValue};
pub use crate::artifacts::gismap::schema::snapshot::GisMapSnapshot;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
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

/// 📸️ Persisted GIS map snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
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
/// parent is constructed or changed. `image` is honestly always absent: gis
/// carries no raster basemap capability today (see `render_mode`'s app-level raster/vector TOGGLE,
/// which selects a rendering STYLE of the same vector data, not a second raster document) — the slot
/// exists, real and typed, for the day a basemap capture lands, not as a stub.
pub type GisMapDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>;
pub type GisMapImageChild = store::ArtifactChild<SemioImageSnapshot>;
pub type GisMapValueChild = store::ArtifactChild<SemioValueSnapshot>;

/// 🕸️ Stable admitted CHILD handle for the map's composed drawing member.
pub fn gis_map_drawing_child_handle(_content_key: &str) -> GisMapDrawingChild {
    let child_id = "gismap-drawing".to_string();
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gismap-drawing".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Stable admitted CHILD handle for the map's composed value member.
pub fn gis_map_value_child_handle(_content_key: &str) -> GisMapValueChild {
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

/// 🔄️ Re-derives `drawing`/`value` from `document`'s CURRENT `positions`/`routes`/`regions` — the
/// single call every constructor/mutator funnels through so the composed children never drift from
/// what they actually describe (`image` stays `None`, honestly — see this region's own doc comment).
/// Uses the same stable drawing/value coordinates as `GisMapSnapshot::default()`; content changes
/// are expressed as typed child mutations and never by re-minting a child member.
pub fn gis_map_snapshot_with_derived_children(mut document: GisMapSnapshot) -> GisMapSnapshot {
    let content_key = crate::artifacts::gismap::schema::snapshot::gis_map_content_key(&document.positions, &document.routes, &document.regions);
    document.drawing = gis_map_drawing_child_handle(&content_key);
    document.value = gis_map_value_child_handle(&content_key);
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
        export_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
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
fn admit_gis_map_inference_request(
    request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>,
) -> Result<usize, semio_framework_plugin::ArtifactInferenceExecutionError> {
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
    let allocation = usize::try_from(request.budgets.allocation_bytes)
        .map_err(|_| semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.budget", "allocation budget exceeds this runtime's address space"))?;
    if request_bytes > allocation {
        return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new(
            "gis.gismap.inference.budget",
            format!("request consumes {request_bytes} bytes, above allocation limit {allocation}"),
        ));
    }
    Ok(allocation)
}

struct InferenceOutputGuard(Vec<u8>);

impl Drop for InferenceOutputGuard {
    fn drop(&mut self) {
        let pointer = self.0.as_mut_ptr();
        for index in 0..self.0.capacity() { unsafe { std::ptr::write_volatile(pointer.add(index), 0); } }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

/// ⏱️ The native GIS service with request-owned cancellation and progress at every value.
pub fn infer_gis_map_controlled(
    request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>,
    checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>,
) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    use crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::{bounds::controlled_lon_lat_bounds, GisMapInference};
    let allocation = admit_gis_map_inference_request(request)?;
    checkpoint(0)?;
    let snapshot = <GisMapSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload)
        .map_err(|_| semio_framework_plugin::ArtifactInferenceExecutionError::new("gis.gismap.inference.snapshot-decode", "invalid canonical map snapshot"))?;
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
        canonical_payload: std::mem::take(&mut canonical_payload.0), diagnostics: Vec::new(), validity: "valid".into(), quality: "exact".into(), complete: true, actual_cache_mode: request.requested_cache_mode.clone(),
    })
}

/// 🧠️ Whole-map native entry uses the same controlled fold with its finite request budgets.
fn infer_gis_map(
    request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>,
) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
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
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.gis.gismap@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gis.gismap@1/*")?)?,
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
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Karte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Karte")?)?)
}

/// 🔖️ Assembles s.gis.gismap's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::gismap::schema::gismap_artifact_schema_descriptor())
        .inferences([crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .inference_services([gis_map_inference_service()])
        .composers(crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::gis2d::Gis2dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;
    use geo::BoundingRect;
    use semio_framework_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

    fn vector_snapshot(case: &serde_json::Value) -> GisMapSnapshot {
        let snapshot = &case["snapshot"];
        gis_map_snapshot_with_derived_children(GisMapSnapshot {
            positions: serde_json::from_value(snapshot["positions"].clone()).expect("positions vector"),
            routes: serde_json::from_value(snapshot["routes"].clone()).expect("routes vector"),
            regions: serde_json::from_value(snapshot["regions"].clone()).expect("regions vector"),
            ..Default::default()
        })
    }

    fn execute(snapshot: &GisMapSnapshot, budgets: &WireArtifactInferenceBudget, cancellation_id: &str, cache_mode: WireArtifactInferenceCacheMode) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
        let pack = <GisMapSnapshot as store::ArtifactPack>::encode_pack(snapshot);
        gis_map_inference_service().infer(&ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets,
            cancellation_id,
            previous_state: None,
            requested_cache_mode: cache_mode,
            canonical_payload: &pack,
            dependencies: &[],
        })
    }

    #[semio_framework_async_macros::async_test]
    async fn map_artifact_kind_matches_the_map_out_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, GISMAP_DIALECT.artifact_kind);
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
    }


    #[semio_framework_async_macros::async_test]
    async fn the_map_snapshot_defaults_to_empty_feature_collections() {
        let document = GisMapSnapshot::default();
        assert!(document.positions.is_empty());
        assert!(document.routes.is_empty());
        assert!(document.regions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn declaration_exposes_one_executable_whole_map_inference() {
        let service = gis_map_inference_service();
        let metadata = service.metadata();
        assert_eq!(metadata.owner, "gis");
        assert_eq!(metadata.artifact_kind, "s.gis.gismap");
        assert_eq!(metadata.artifact_schema, "s.gis.gismap");
        assert_eq!(metadata.inference_schema, "s.gis.gismap.inference");
        let mut registry = ArtifactInferenceServiceRegistry::new();
        registry.register(service).expect("service registers");
        declaration().expect("service-bearing declaration builds");
        crate::plugin::plugin().expect("assembled plugin accepts service-bearing declaration");
    }

    #[semio_framework_async_macros::async_test]
    async fn language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗺️infer-gismap-1/🧫️fixtures/🔣️.json")).expect("language-neutral inference vectors");
        assert_eq!(vectors["subjectSchema"], "../../../🧬️schema/💡️inferences/🔣️.json");
        assert_eq!(vectors["inferenceSchema"], "s.gis.gismap.inference");
        assert_eq!(vectors["schemaVersion"], 1);
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 32 };
        for case in vectors["cases"].as_array().expect("cases") {
            let snapshot = vector_snapshot(case);
            let first = execute(&snapshot, &budgets, case["id"].as_str().expect("case id"), WireArtifactInferenceCacheMode::Cold).expect("inference succeeds");
            let second = execute(&snapshot, &budgets, &format!("{}-repeat", case["id"].as_str().expect("case id")), WireArtifactInferenceCacheMode::Cold).expect("repeat succeeds");
            assert_eq!(first.canonical_payload, second.canonical_payload);
            assert_eq!(first.validity, "valid");
            assert_eq!(first.quality, "exact");
            assert!(first.complete);
            let inference = <crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference as semio_framework_os_kernel::FromValue>::from_value(
                semio_framework_os_kernel::pack_rt::decode_wire_value(&first.canonical_payload).expect("canonical inference payload"),
            )
            .expect("typed inference");
            let expected = &case["expected"];
            assert_eq!(inference.position_count as u64, expected["positionCount"].as_u64().expect("position count"));
            assert_eq!(inference.route_count as u64, expected["routeCount"].as_u64().expect("route count"));
            assert_eq!(inference.region_count as u64, expected["regionCount"].as_u64().expect("region count"));
            let oracle_points = case["oracleCoordinates"]
                .as_array()
                .expect("oracle coordinates")
                .iter()
                .map(|point| geo::Point::new(point[0].as_f64().expect("longitude"), point[1].as_f64().expect("latitude")))
                .collect::<geo::MultiPoint>();
            let oracle = oracle_points.bounding_rect();
            match (inference.bounds, oracle, expected["bounds"].as_object()) {
                (None, None, None) => {}
                (Some(actual), Some(oracle), Some(expected)) => {
                    assert_eq!(actual.lon_min, oracle.min().x);
                    assert_eq!(actual.lon_max, oracle.max().x);
                    assert_eq!(actual.lat_min, oracle.min().y);
                    assert_eq!(actual.lat_max, oracle.max().y);
                    assert_eq!(actual.lon_min, expected["lonMin"].as_f64().expect("lon min"));
                    assert_eq!(actual.lon_max, expected["lonMax"].as_f64().expect("lon max"));
                    assert_eq!(actual.lat_min, expected["latMin"].as_f64().expect("lat min"));
                    assert_eq!(actual.lat_max, expected["latMax"].as_f64().expect("lat max"));
                }
                state => panic!("vector, subject, and geo oracle disagree: {state:?}"),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_snapshot_is_a_structured_execution_error() {
        let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000, work_units: 10, recursion_depth: 4 };
        let error = gis_map_inference_service()
            .infer(&ArtifactInferenceExecutionRequest {
                policy: &[],
                budgets: &budgets,
                cancellation_id: "malformed",
                previous_state: None,
                requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
                canonical_payload: b"not-a-gismap-pack",
                dependencies: &[],
            })
            .err()
            .expect("malformed snapshot must fail");
        assert_eq!(error.code, "gis.gismap.inference.snapshot-decode");
    }

    #[semio_framework_async_macros::async_test]
    async fn service_enforces_work_recursion_and_cancellation_identity() {
        let snapshot = GisMapSnapshot {
            positions: vec![MapFeature { id: "nested".into(), data: dsl::DslValue::from(serde_json::json!({ "geometry": { "lon": 1.0, "lat": 2.0 } })) }],
            ..Default::default()
        };
        let no_work = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 32 };
        assert_eq!(execute(&snapshot, &no_work, "work", WireArtifactInferenceCacheMode::Cold).err().expect("work budget").code, "gis.gismap.inference.budget");
        let no_allocation = WireArtifactInferenceBudget { allocation_bytes: 1, work_units: 1_000, recursion_depth: 32 };
        assert_eq!(execute(&snapshot, &no_allocation, "allocation", WireArtifactInferenceCacheMode::Cold).err().expect("allocation budget").code, "gis.gismap.inference.budget");
        let no_depth = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 1 };
        assert_eq!(execute(&snapshot, &no_depth, "depth", WireArtifactInferenceCacheMode::Cold).err().expect("recursion budget").code, "gis.gismap.inference.budget");
        let valid = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1_000, recursion_depth: 32 };
        assert_eq!(execute(&snapshot, &valid, "", WireArtifactInferenceCacheMode::Cold).err().expect("cancellation identity").code, "gis.gismap.inference.cancellation");
        assert_eq!(execute(&snapshot, &valid, "incremental", WireArtifactInferenceCacheMode::Incremental).err().expect("unsupported incremental cache").code, "gis.gismap.inference.cache-mode");
        assert_eq!(execute(&snapshot, &valid, "bypass", WireArtifactInferenceCacheMode::Bypass).expect("bypass succeeds").actual_cache_mode, WireArtifactInferenceCacheMode::Bypass);
    }
}
//#endregion 🔹Tests
