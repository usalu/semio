//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).


pub use crate::artifacts::gismap::schema::snapshot::GisMapSnapshot;
pub use crate::artifacts::gismap::schema::mutations::GisMapMutation;
pub use crate::artifacts::gismap::schema::diff::GisMapDiff;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

//#region 🔹Constants


pub const GIS_MAP_SCHEMA: &str = "gis.map";
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
    fn id(&self) -> &String {
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
pub fn gis_map_drawing_child_handle(content_key: &str) -> GisMapDrawingChild {
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
pub fn gis_map_value_child_handle(content_key: &str) -> GisMapValueChild {
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
pub fn semio_value_from_serde_json(value: &serde_json::Value) -> SemioValue {
    match value {
        serde_json::Value::Null => SemioValue::Null,
        serde_json::Value::Bool(value) => SemioValue::Bool { value: *value },
        serde_json::Value::Number(number) => {
            let lexeme = number.to_string();
            if lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E') { SemioValue::Float { lexeme } } else { SemioValue::Int { lexeme } }
        }
        serde_json::Value::String(value) => SemioValue::Str { value: value.clone() },
        serde_json::Value::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_serde_json).collect() },
        serde_json::Value::Object(members) => {
            SemioValue::Map { entries: members.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_serde_json(value) }).collect() }
        }
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
/// Uses the SAME `gis_map_content_key` hash basis `GisMapSnapshot::default()` uses (`📸️snapshot/🦀️component.rs`)
/// so two paths building an identical empty/edited document always converge on the identical handles.
pub fn gis_map_snapshot_with_derived_children(mut document: GisMapSnapshot) -> GisMapSnapshot {
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
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.gismap.schema.artifact", "schema", "s.gis.gismap", &[("schema", "s.gis.gismap")], None),
        ("s.gismap.inference.artifact", "inference", "s.gis.gismap.inference", &[("schema", "s.gis.gismap.inference")], None),
        ("s.gismap.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.gismap.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.gismap.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.gismap.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.gismap.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.gismap.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.gismap.grammar.document", "grammar", "gis.gismap", &[("grammar", "gis.gismap")], None),
        ("s.gismap.grammar.op", "grammar", "gis.gismap.op", &[("grammar", "gis.gismap.op")], None),
        ("s.gismap.grammar.diff", "grammar", "gis.gismap.diff", &[("grammar", "gis.gismap.diff")], None),
        ("s.gismap.grammar.pack", "grammar", "gismap.pack", &[("grammar", "gismap.pack")], None),
        ("s.gismap.grammar.spr", "grammar", "gismap.spr", &[("grammar", "gismap.spr")], None),
        ("s.gismap.codec.document", "codec", "gis.map:gismap", &[("codec", "gis.map"), ("extension", "gismap")], None),
        ("s.gismap.localization.en", "localization", "GIS Map", &[], Some(("en", "GIS Map"))),
        ("s.gismap.localization.de", "localization", "GIS Karte", &[], Some(("de", "GIS Karte"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.gismap")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::gismap::schema::gismap_artifact_schema_descriptor())
        .inferences([crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .composers(crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::gis2d::Gis2dPlayApp>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below. Relocated alongside
/// `declaration()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2) — its only caller.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "gis.gismap",
                    extension: Some("gismap"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::gismap::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gismap"),
                },
                dsl::LanguageSpec {
                    id: "gis.gismap.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::gismap::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gismap.op"),
                },
                dsl::LanguageSpec {
                    id: "gis.gismap.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::gismap::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("gis.gismap.diff"),
                },
                dsl::LanguageSpec {
                    id: "gismap.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gismap.pack"),
                },
                dsl::LanguageSpec {
                    id: "gismap.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gismap.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_artifact_kind_matches_the_map_out_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.map");
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
    }

    #[test]
    fn the_map_snapshot_defaults_to_empty_feature_collections() {
        let document = GisMapSnapshot::default();
        assert!(document.positions.is_empty());
        assert!(document.routes.is_empty());
        assert!(document.regions.is_empty());
    }

    //#region 🔌️HostIoRegistration
    /// 🧪️ 🌍️gis — and only 🌍️gis — puts `"2d.map"`'s media codecs into the OS registry. Before
    /// ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D2 the sole
    /// registrant was `🎪️demonstrator`'s verfolgen pane, so this crate on its own registered
    /// nothing; the assertions run entirely inside the owner crate, which is the property that was
    /// missing.
    ///
    /// Unlike solids (`solid_exporter_for`), the OS media handler map exposes no membership
    /// predicate to plugins — `export_os_app_instance_media_kind` needs the `WorkflowNode` type
    /// that lives behind the `os-host-full` feature plugins do not enable. So what is pinned here
    /// is everything observable from the owner side: the registration links and runs (twice — the
    /// registry is keyed, not appended), it keys on this artifact's OWN declared `ArtifactKindSpec`
    /// id rather than a foreign kind, and the DWG bridge fn it hands the OS is gis's own and
    /// actually produces a map document.
    ///
    /// Deliberately NOT asserted: the svg half (`gis2d_document_json_to_svg`). That bridge renders
    /// through `io_dispatch`, whose drawing→svg composer entry is registered by 🗄️stdio's plugin
    /// build, which never runs in a bare gis unit test — so asserting on it would measure another
    /// plugin's registration, not this one's ownership.
    #[test]
    fn gis_owns_the_host_io_registration_for_its_own_kind() {
        use crate::artifacts::gismap::standards::v1::subsets::any::io::{register_host_io, GIS_MAP_KIND};
        assert_eq!(GIS_MAP_KIND, artifact_kind().id, "register_host_io must key on the kind this artifact itself declares");
        register_host_io();
        register_host_io();
        use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        let imported = crate::artifacts::gismap::schema::gis2d_document_json_from_dwg(&drawing).expect("registered dwg bridge imports");
        let document: GisMapSnapshot = serde_json::from_value(imported).expect("dwg bridge yields a map document");
        assert_eq!(document.positions.len(), 1, "registered dwg bridge must lower the drawing's point into a position feature");
    }
    //#endregion 🔌️HostIoRegistration
}
//#endregion 🔹Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("GisMapComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
