//! 💡️ GIS terrain inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::gisterrain::GisTerrainSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::bounds::{imported_lon_lat_positions, lon_lat_bounds, GisTerrainBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gisterrain snapshot. Today: the geographic bounding box and
/// position count of the `map:in` overlay decoded from `imported_features_json` (see
/// `📦bounds/🦀️component.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// overlay is small and re-decoding is O(positions).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain.inference")]
pub struct GisTerrainInference {
    #[state(inferred)]
    pub position_count: usize,
    #[state(inferred)]
    pub bounds: Option<GisTerrainBounds>,
}

impl protocol::Inference<GisTerrainSnapshot> for GisTerrainInference {
    fn infer(snapshot: &GisTerrainSnapshot) -> Self {
        let positions = imported_lon_lat_positions(snapshot);
        Self { position_count: positions.len(), bounds: lon_lat_bounds(&positions) }
    }
}

impl protocol::InferenceSpec<GisTerrainSnapshot> for GisTerrainInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gisterrain.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.positionCount", reads: &["importedFeaturesJson"] },
            protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.bounds", reads: &["importedFeaturesJson"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::gisterrain::standards::v1::subsets::any::schema::GisterrainBuilder {
    type Snapshot = GisTerrainSnapshot;
    type Inference = GisTerrainInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️FixtureText
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `parse_descriptor` is a pure
/// snapshot → projection function (`&GisTerrainSnapshot` → `TerrainDescriptorJson`), matching the
/// `🧬️schema/💡️inferences/` destination — same family as `GisTerrainInference` above, just not yet
/// wired through the typed `#[state(inferred)]` registry.
///
/// ⚠️ `crate::modules::terrain` below is a KNOWN PRE-EXISTING unresolved import — verified (git log,
/// no `pub mod modules` anywhere in this crate's `📦️glue.rs`) to predate this ticket's changes and
/// reported by another session. It is carried forward unchanged here, not fixed: out of this
/// ticket's scope (engine dissolution, not import repair).
use crate::modules::terrain::{TerrainDescriptorJson, TerrainPositionData, TerrainProjectOrigin};

/// 📜️ Hand-rolled reader for the `.gisterrain` fixture's `origin`/`position` scenery lines — the
/// read-only pins/project-origin data rendered alongside the document; the `gisterrain
/// exaggeration=...` header line those same files start with is instead read by
/// `GisTerrainSnapshot`'s own derive-generated `ArtifactDsl`, since exaggeration is undoable document
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

/// 🔌️ `map:in`'s overlay pin layer (see `GisTerrainSnapshot::imported_features_json`), decoded from
/// its `{positions:[{id,lon,lat,label?,icon?}]}` descriptor JSON — malformed/empty JSON (including the
/// default empty string) simply contributes no extra pins.
fn imported_positions(document: &GisTerrainSnapshot) -> Vec<TerrainPositionData> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&document.imported_features_json) else {
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
/// `crate::artifacts::gisterrain::schema::default_terrain_document`.
pub fn parse_descriptor(document: &GisTerrainSnapshot) -> TerrainDescriptorJson {
    let mut descriptor = terrain_fixture_text::parse_descriptor(
        crate::artifacts::gisterrain::dsl::REUSE_TERRAIN_EXAMPLE_TEXT,
        crate::artifacts::gisterrain::GIS_3D_TERRAIN_SCHEMA,
        document.exaggeration,
    );
    descriptor.positions.extend(imported_positions(document));
    descriptor
}
//#endregion 🔖️FixtureText

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gisterrain.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `gisterrain_artifact_schema_descriptor`'s registration.
pub fn gisterrain_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.gis.gisterrain.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = GisTerrainSnapshot {
            exaggeration: 1.5,
            imported_features_json: serde_json::json!({ "positions": [{ "id": "p1", "lon": 5.58, "lat": 50.60 }] }).to_string(),
        };
        assert_eq!(GisTerrainInference::infer(&snapshot), GisTerrainInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(GisTerrainInference::infer(&GisTerrainSnapshot::default()), GisTerrainInference::default());
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️FixtureText
    /// 🧭️ Relocated from the artifact's `⚙️engine` tests alongside `parse_descriptor`
    /// (`🔖️FixtureText` above).
    ///
    /// 📜️ The `.gisterrain` fixture's `gisterrain exaggeration=...` header is parsed twice for two
    /// different purposes (see `parse_descriptor`/`default_terrain_document`'s docs); this proves the
    /// scenery-data reader (`terrain_fixture_text`) still recovers the bundled fixture's pins/origin
    /// after the document-only conversion — i.e. converting the fixture to the DSL didn't lose data.
    #[test]
    fn terrain_fixture_text_recovers_bundled_scenery_data() {
        let descriptor = parse_descriptor(&GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: String::new() });
        assert_eq!(descriptor.project_origin.lon, 5.5818);
        assert_eq!(descriptor.project_origin.lat, 50.603);
        assert_eq!(descriptor.positions.len(), 2);
        assert_eq!(descriptor.positions[0].id, "p_institut_de_botanique_ulg_liege");
    }

    /// 🔌️ `map:in`'s overlay layer renders as extra pins alongside the fixture's own two.
    #[test]
    fn imported_map_features_render_as_extra_pins() {
        let document = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: serde_json::json!({ "positions": [{ "id": "imported-1", "lon": 5.58, "lat": 50.60 }] }).to_string() };
        let descriptor = parse_descriptor(&document);
        assert_eq!(descriptor.positions.len(), 3, "2 fixture pins + 1 imported pin");
        assert!(descriptor.positions.iter().any(|position| position.id == "imported-1"));
    }
    //#endregion 🧪️FixtureText
}
//#endregion 🧪️Tests
