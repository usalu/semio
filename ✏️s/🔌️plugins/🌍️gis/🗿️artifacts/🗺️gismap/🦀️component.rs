//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).

pub use crate::artifacts::gismap::schema::snapshot::GisMapSnapshot;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

//#region 🔹Constants

pub const GIS_MAP_SCHEMA: &str = "gis.map";

/// 🪪️ The canonical surface dialect for `s.gis.gismap@1/*` (contract §1 grammar) — lives at the
/// ARTIFACT root, not under `✏️editor`/`👁️viewer`, so a viewer file can read it without ever
/// importing through the sibling `editor` module. `artifact_kind` is the 3-part schema id this
/// file's own `definition()` claims (`s.gis.gismap`), NOT the 2-part `ArtifactIdentity::parse("s.gismap")`
/// string above and NOT the module-private `🚪️io/🦀️component.rs` `GISMAP_DIALECT` (an older,
/// unrelated io/composer const with a different 2-part `artifact_kind` — different file, no collision).
pub const GISMAP_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.gis.gismap", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
//#endregion 🔹Constants

//#region 🔹Types
/// 🗺️ One id-keyed spatial feature carried as its full opaque descriptor payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch.
    pub data: dsl::DslValue,
}

impl Identified<String> for MapFeature {
    async fn id(&self) -> &String {
        &self.id
    }
}

/// Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeaturePatch {
    pub data: Option<dsl::DslValue>,
}

impl Patchable<MapFeaturePatch> for MapFeature {
    async fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
    }

    async fn diff_patch(&self, other: &Self) -> Option<MapFeaturePatch> {
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
/// plugin's `🧬️schema/📸️snapshot/🦀️component.rs` module doc for the precedent) since they are NOT a
/// duplicated stdio type, just gis's own vocabulary; `drawing`/`value` are DERIVED composed children,
/// deterministically re-minted from the current feature collections by
/// `gis_map_snapshot_with_derived_children` every time the document changes (`GisMapArtifact::to_snapshot`,
/// `apply_gis_map_mutation`), never independently mutable. `image` is honestly always absent: gis
/// carries no raster basemap capability today (see `render_mode`'s app-level raster/vector TOGGLE,
/// which selects a rendering STYLE of the same vector data, not a second raster document) — the slot
/// exists, real and typed, for the day a basemap capture lands, not as a stub.
pub type GisMapDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>;
pub type GisMapImageChild = store::ArtifactChild<SemioImageSnapshot>;
pub type GisMapValueChild = store::ArtifactChild<SemioValueSnapshot>;

/// 🕸️ Deterministic content-addressed CHILD handle for the map's composed drawing — same
/// `(child_id, target)` for identical `content_key`, a different pair once the features actually
/// change. Mirrors `🏔️gisterrain`'s `gis_terrain_mesh_child_handle`/`💠️lowpoly`'s `mesh_child_handle`.
pub async fn gis_map_drawing_child_handle(content_key: &str) -> GisMapDrawingChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_key.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("gismap-drawing-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gismap-drawing".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the map's composed value graph — same
/// hashing/dialect shape as `gis_map_drawing_child_handle`, targeting `s.stdio.semio.value` instead.
pub async fn gis_map_value_child_handle(content_key: &str) -> GisMapValueChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_key.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("gismap-value-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "value".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gismap-value".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🌉️ WRITE direction, real (not a stub): `serde_json::Value` → `SemioValue`, a direct structural
/// mapping (json has no binary/graph-reference primitive, so `Bytes`/`Ref` are never produced —
/// mirrors stdio's own `semio_value_from_json` for its `json` artifact, written locally here since
/// that one converts stdio's OWN `JsonValue` AST, not `serde_json::Value`, and gis already speaks
/// `serde_json::Value` everywhere else in this file).
pub async fn semio_value_from_serde_json(value: &serde_json::Value) -> SemioValue {
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
pub async fn serde_json_from_semio_value(value: &SemioValue) -> serde_json::Value {
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
pub async fn gis_map_value_from_descriptor_json(descriptor_json: &str) -> SemioValueSnapshot {
    let value: serde_json::Value = serde_json::from_str(descriptor_json).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_serde_json(&value), nodes: Vec::new() }
}

/// 🌉️ The exact inverse of `gis_map_value_from_descriptor_json` — recovers the descriptor JSON a
/// `value` child's content actually carries.
pub async fn gis_map_descriptor_json_from_value(value: &SemioValueSnapshot) -> String {
    serde_json_from_semio_value(&value.root).to_string()
}

/// 🔄️ Re-derives `drawing`/`value` from `document`'s CURRENT `positions`/`routes`/`regions` — the
/// single call every constructor/mutator funnels through so the composed children never drift from
/// what they actually describe (`image` stays `None`, honestly — see this region's own doc comment).
/// Uses the SAME `gis_map_content_key` hash basis `GisMapSnapshot::default()` uses (`📸️snapshot/🦀️component.rs`)
/// so two paths building an identical empty/edited document always converge on the identical handles.
pub async fn gis_map_snapshot_with_derived_children(mut document: GisMapSnapshot) -> GisMapSnapshot {
    let content_key = crate::artifacts::gismap::schema::snapshot::gis_map_content_key(&document.positions, &document.routes, &document.regions);
    document.drawing = gis_map_drawing_child_handle(&content_key);
    document.value = gis_map_value_child_handle(&content_key);
    document
}
//#endregion 🔖️Composition

//#region 🔹ArtifactKind
/// The `2d.map` artifact kind declaration.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.map".into(),
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

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// the plugin root's `register_gis_exports` fan-out. Relocated from `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes the artifact
/// (kind, schema, io ports, ownership), which is not engine behaviour.
/// 🧾️ Defines s.gismap's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.gismap")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.gis.gismap")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.gis.gismap.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gismap.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.gismap@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gismap@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.composer.dxf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dxf@r12/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dxf@r12/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"gis.map:gismap")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "gis.map")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "gismap")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Map")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "GIS Map")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.gismap.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"GIS Karte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Karte")?)?)
}

/// 🔖️ Assembles s.gismap's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::gismap::schema::gismap_artifact_schema_descriptor())
        .inferences([crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .composers(crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::gis2d::Gis2dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn map_artifact_kind_matches_the_map_out_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.map");
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn the_map_snapshot_defaults_to_empty_feature_collections() {
        let document = GisMapSnapshot::default();
        assert!(document.positions.is_empty());
        assert!(document.routes.is_empty());
        assert!(document.regions.is_empty());
    }
}
//#endregion 🔹Tests
