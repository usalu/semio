//! ⚙️ GIS terrain artifact — headless compute over the terrain projection (constitutional: engine).
//!
//! 🧭️ Placement rule for helpers: anything here takes ONLY document-side types
//! (`Gis3dTerrainDocument`, the fixture text, the terrain descriptor). Helpers that also need the 🧊️3d
//! app's view state (`crate::apps::gis3d::config::Gis3dConfig`) stay at app/window level — an artifact
//! must never depend on an app.

use crate::artifacts::gisterrain::dsl::REUSE_TERRAIN_EXAMPLE_TEXT;
use crate::artifacts::gisterrain::{Gis3dTerrainDocument, GIS_3D_TERRAIN_SCHEMA};
use framework_surface_terrain::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};
use serde_json::Value;

//#region 🔖️DocumentHelpers
pub fn empty_gis3d_terrain_projection() -> Gis3dTerrainDocument {
    Gis3dTerrainDocument { exaggeration: 1.0, ..Default::default() }
}

/// 🗺️ The default terrain document, seeded from the bundled reuse example's `gisterrain
/// exaggeration=...` header (see `crate::artifacts::gisterrain::Gis3dTerrainDocument`'s
/// derive-generated `.gisterrain` DSL).
pub fn default_terrain_document() -> Gis3dTerrainDocument {
    <Gis3dTerrainDocument as store::DocumentDsl>::parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis3d_terrain_projection())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️FixtureText
/// 📜️ Hand-rolled reader for the `.gisterrain` fixture's `origin`/`position` scenery lines — the
/// read-only pins/project-origin data rendered alongside the document; the `gisterrain
/// exaggeration=...` header line those same files start with is instead read by
/// `Gis3dTerrainDocument`'s own derive-generated `DocumentDsl`, since exaggeration is undoable document
/// state.
mod terrain_fixture_text {
    use super::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};

    /// 🔤️ Splits one line into whitespace-separated tokens, treating a `"..."` quoted run (escapes
    /// `\\`, `\"`, `\n`) as part of the token it's glued to — so `label="Institut de Botanique"`
    /// lexes as one `label=Institut de Botanique` token even though the value contains spaces.
    fn line_tokens(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }
            let mut token = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                if c == '"' {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        if c == '\\' {
                            match chars.next() {
                                Some('n') => token.push('\n'),
                                Some('"') => token.push('"'),
                                Some('\\') => token.push('\\'),
                                Some(other) => {
                                    token.push('\\');
                                    token.push(other);
                                }
                                None => {}
                            }
                        } else {
                            token.push(c);
                        }
                    }
                } else {
                    token.push(c);
                    chars.next();
                }
            }
            tokens.push(token);
        }
        tokens
    }

    fn kv_lookup<'a>(tokens: &'a [String], key: &str) -> Option<&'a str> {
        tokens.iter().find_map(|token| token.strip_prefix(&format!("{key}=")))
    }

    fn parse_project_origin(tokens: &[String]) -> Option<TerrainProjectOrigin> {
        Some(TerrainProjectOrigin { lon: kv_lookup(tokens, "lon")?.parse().ok()?, lat: kv_lookup(tokens, "lat")?.parse().ok()? })
    }

    fn parse_position(tokens: &[String]) -> Option<TerrainPositionData> {
        Some(TerrainPositionData {
            id: kv_lookup(tokens, "id")?.to_string(),
            lon: kv_lookup(tokens, "lon")?.parse().ok()?,
            lat: kv_lookup(tokens, "lat")?.parse().ok()?,
            label: kv_lookup(tokens, "label").map(str::to_string),
            icon: kv_lookup(tokens, "icon").map(str::to_string),
        })
    }

    /// 📥️ Parses every `origin`/`position` line of the fixture text (its `gisterrain exaggeration=...`
    /// header is parsed separately, see module docs); malformed or missing lines simply contribute
    /// nothing, so a truncated/empty fixture yields the world origin with no positions rather than an error.
    pub(super) fn parse_descriptor(text: &str, schema: &str, exaggeration: f64) -> TerrainDescriptorJson {
        let mut project_origin = TerrainProjectOrigin { lon: 0.0, lat: 0.0 };
        let mut positions = Vec::new();
        for line in text.lines() {
            let tokens = line_tokens(line);
            match tokens.first().map(String::as_str) {
                Some("origin") => {
                    if let Some(origin) = parse_project_origin(&tokens) {
                        project_origin = origin;
                    }
                }
                Some("position") => {
                    if let Some(position) = parse_position(&tokens) {
                        positions.push(position);
                    }
                }
                _ => {}
            }
        }
        TerrainDescriptorJson { schema: schema.to_string(), project_origin, positions, exaggeration }
    }
}

/// 🔌️ `map:in`'s overlay pin layer (see `Gis3dTerrainDocument::imported_features_json`), decoded from
/// its `{positions:[{id,lon,lat,label?,icon?}]}` descriptor JSON — malformed/empty JSON (including the
/// default empty string) simply contributes no extra pins.
fn imported_positions(document: &Gis3dTerrainDocument) -> Vec<TerrainPositionData> {
    let Ok(value) = serde_json::from_str::<Value>(&document.imported_features_json) else {
        return Vec::new();
    };
    let Some(positions) = value.get("positions").and_then(|value| value.as_array()) else {
        return Vec::new();
    };
    positions
        .iter()
        .filter_map(|entry| {
            Some(TerrainPositionData {
                id: entry.get("id").and_then(|value| value.as_str())?.to_string(),
                lon: entry.get("lon").and_then(|value| value.as_f64())?,
                lat: entry.get("lat").and_then(|value| value.as_f64())?,
                label: entry.get("label").and_then(|value| value.as_str()).map(str::to_string),
                icon: entry.get("icon").and_then(|value| value.as_str()).map(str::to_string),
            })
        })
        .collect()
}

/// 🏔️ The full rendering descriptor (project origin + fixture pins + `map:in` overlay pins +
/// exaggeration) for the given document — `exaggeration` always mirrors the LIVE document, and the
/// bundled fixture's own `gisterrain exaggeration=...` header only ever seeds it once via
/// [`default_terrain_document`].
pub fn parse_descriptor(document: &Gis3dTerrainDocument) -> TerrainDescriptorJson {
    let mut descriptor = terrain_fixture_text::parse_descriptor(REUSE_TERRAIN_EXAMPLE_TEXT, GIS_3D_TERRAIN_SCHEMA, document.exaggeration);
    descriptor.positions.extend(imported_positions(document));
    descriptor
}
//#endregion 🔖️FixtureText

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`), plus the two app-specific workflow
/// ports (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe):
/// `map:in` (a `2d.map` producer — gis2d's `map:out` — feeds an overlay pin layer, see
/// `Gis3dTerrainDocument::imported_features_json`) and `scene:out` (this terrain as `3d.mesh`).
/// `document_media_type` is Data×Value (the document is a scalar "exaggeration + imported overlay"
/// record, not itself mesh geometry — `scene:out` is the actual renderable mesh/terrain surface).
pub fn gis3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: GIS_3D_TERRAIN_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![gis3d_map_in_port(), gis3d_scene_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "gis.terrain".into(), name: "GIS Terrain".into(), dimension: "3d".into(), component_kind: "gisterrain".into() },
    }
}

/// 🔌️ `map:in` — a `2d.map` producer (gis2d's `map:out`) feeding an overlay pin layer into this
/// terrain (see `Gis3dTerrainDocument::imported_features_json`). `One`/optional: exactly one map may
/// be draped onto a terrain at a time, and a terrain with no upstream edge is valid.
pub fn gis3d_map_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:in".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: Some("2d.map".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::One,
    }
}

/// 🔌️ `scene:out` — this terrain as `3d.mesh` (kind already registered by lowpoly; reused verbatim,
/// not redeclared — WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `Many`/optional: several
/// downstream consumers may fan out from one terrain, and a terrain with no downstream edge is valid.
pub fn gis3d_scene_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "scene:out".into(),
        label: "Scene".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🎞️ `scene:out`'s `Media` value. First pass (mirrors this app's own "deliberately minimal" module
/// doc): gis3d has no CPU-side heightmap tessellator yet (rendering is scene-descriptor driven, see the
/// 🏔️terrain window's `render`/`build_terrain_scene_json`), so this exports the same terrain descriptor
/// fields (exaggeration + imported overlay) as a structured `3d.mesh` payload rather than a real
/// triangulated mesh — an honest placeholder for the day a tessellator lands, not a silent fake.
pub fn gis3d_scene_media(document: &Gis3dTerrainDocument) -> semio_framework_plugin::Media {
    semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        payload: semio_framework_plugin::MediaPayload::Structured {
            schema: "3d.mesh".into(),
            json: serde_json::json!({
                "exaggeration": document.exaggeration,
                "importedFeatures": serde_json::from_str::<Value>(&document.imported_features_json).unwrap_or(serde_json::json!(null)),
            })
            .to_string(),
        },
    }
}
//#endregion 🔖️Io

//#region 🔖️Registration
/// 🗂️ Native setup hook for the `gis.terrain` artifact — registers the pack↔dsl document codec
/// `framework/sync`'s `FolderEndpoint` reaches for. Called from the plugin root's `📦️glue.rs` setup fn.
pub fn register() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::gis3d::Gis3dPlayApp>(GIS_3D_TERRAIN_SCHEMA);
}
//#endregion 🔖️Registration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// 📜️ The `.gisterrain` fixture's `gisterrain exaggeration=...` header is parsed twice for two
    /// different purposes (see `parse_descriptor`/`default_terrain_document`'s docs); this proves the
    /// scenery-data reader (`terrain_fixture_text`) still recovers the bundled fixture's pins/origin
    /// after the document-only conversion — i.e. converting the fixture to the DSL didn't lose data.
    #[test]
    fn terrain_fixture_text_recovers_bundled_scenery_data() {
        let descriptor = parse_descriptor(&Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: String::new() });
        assert_eq!(descriptor.project_origin.lon, 5.5818);
        assert_eq!(descriptor.project_origin.lat, 50.603);
        assert_eq!(descriptor.positions.len(), 2);
        assert_eq!(descriptor.positions[0].id, "p_institut_de_botanique_ulg_liege");
    }

    /// 🔌️ `map:in`'s overlay layer renders as extra pins alongside the fixture's own two.
    #[test]
    fn imported_map_features_render_as_extra_pins() {
        let document = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: json!({ "positions": [{ "id": "imported-1", "lon": 5.58, "lat": 50.60 }] }).to_string() };
        let descriptor = parse_descriptor(&document);
        assert_eq!(descriptor.positions.len(), 3, "2 fixture pins + 1 imported pin");
        assert!(descriptor.positions.iter().any(|position| position.id == "imported-1"));
    }

    #[test]
    fn default_terrain_document_seeds_the_fixture_exaggeration() {
        assert_eq!(default_terrain_document().exaggeration, 1.5);
        assert_eq!(empty_gis3d_terrain_projection().exaggeration, 1.0);
    }

    #[test]
    fn gis3d_io_declares_the_map_in_and_scene_out_ports() {
        let io = gis3d_io();
        assert_eq!(io.document_schema, GIS_3D_TERRAIN_SCHEMA);
        let ports = io.all_ports();
        let map_in = ports.iter().find(|port| port.id == "map:in").expect("map:in declared");
        assert_eq!(map_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(map_in.kind_id.as_deref(), Some("2d.map"));
        let scene_out = ports.iter().find(|port| port.id == "scene:out").expect("scene:out declared");
        assert_eq!(scene_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(scene_out.kind_id.as_deref(), Some("3d.mesh"));
    }

    #[test]
    fn gis3d_scene_media_exports_the_terrain_descriptor() {
        let document = default_terrain_document();
        let media = gis3d_scene_media(&document);
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else {
            panic!("expected a structured scene:out payload");
        };
        assert_eq!(schema, "3d.mesh");
        assert!(json.contains("exaggeration"));
    }
}
//#endregion 🧪️Tests
