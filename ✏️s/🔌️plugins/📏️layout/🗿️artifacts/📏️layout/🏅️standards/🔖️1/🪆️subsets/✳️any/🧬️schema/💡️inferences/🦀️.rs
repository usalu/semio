//! 💡️ Layout inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::layout::LayoutSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_layout_topology, LayoutTopology};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Inference
/// 💡️ Everything inferable from a layout snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout.inference")]
pub struct LayoutInference {
    #[derived]
    pub topology: LayoutTopology,
}

// 🧷️ `LayoutSnapshot` has no `Default` impl of its own (its only "empty-ish" constructor,
// `engine::default_document`, actually seeds a full demo document) — so unlike the sibling
// forms/playbook/mathematical inference facets, this can't be written as
// `infer(&LayoutSnapshot::default())`. An empty layout document (no pages/spreads/parent pages) has
// an unambiguous empty topology, so that's what this hand-written `Default` returns directly.
impl Default for LayoutInference {
    fn default() -> Self {
        Self { topology: LayoutTopology::empty() }
    }
}

impl protocol::Inference<LayoutSnapshot> for LayoutInference {
    async fn infer(snapshot: &LayoutSnapshot) -> Self {
        Self { topology: compute_layout_topology(&snapshot.parent_pages, &snapshot.spreads, &snapshot.pages) }
    }
}

impl protocol::InferenceSpec<LayoutSnapshot> for LayoutInference {
    async fn inference_schema_id() -> &'static str {
        "s.layout.layout.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.layout.layout.inference.topology", reads: &["parentPages", "spreads", "pages"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::layout::standards::v1::subsets::any::schema::LayoutBuilder {
    type Snapshot = LayoutSnapshot;
    type Inference = LayoutInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.layout.layout.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `layout_artifact_schema_descriptor`'s registration.
pub async fn layout_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.layout.layout.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn snapshot_with_master_and_spread() -> LayoutSnapshot {
        let json = serde_json::json!({
            "schema": "semio.layout/v1",
            "name": "doc",
            "grid": { "baselineGrid": 12.0, "baselineOffset": 0.0, "snapToBaseline": false },
            "paragraphStyles": [],
            "characterStyles": [],
            "stories": [],
            "links": [],
            "parentPages": [
                { "id": "master-1", "name": "Master", "width": 210.0, "height": 297.0, "layerIds": [], "layers": [], "frames": [] }
            ],
            "spreads": [
                { "id": "spread-1", "name": "Spread 1", "pageIds": ["page-1"] }
            ],
            "pages": [
                {
                    "id": "page-1", "name": "Page 1", "spreadId": "spread-1", "parentPageId": "master-1",
                    "width": 210.0, "height": 297.0,
                    "margins": { "top": 10.0, "right": 10.0, "bottom": 10.0, "left": 10.0 },
                    "columns": { "count": 1, "gutter": 0.0 },
                    "guides": [], "layerIds": [], "layers": [], "frames": [], "overrides": []
                }
            ],
            "printTarget": null
        });
        serde_json::from_value(json).expect("valid layout snapshot json")
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = snapshot_with_master_and_spread();
        assert_eq!(LayoutInference::infer(&snapshot), LayoutInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        let empty = serde_json::from_value::<LayoutSnapshot>(serde_json::json!({
            "schema": "semio.layout/v1",
            "name": "",
            "grid": { "baselineGrid": 12.0, "baselineOffset": 0.0, "snapToBaseline": false },
            "paragraphStyles": [], "characterStyles": [], "stories": [], "links": [],
            "parentPages": [], "spreads": [], "pages": [], "printTarget": null
        }))
        .expect("valid empty layout snapshot json");
        assert_eq!(LayoutInference::infer(&empty), LayoutInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn page_topologically_follows_its_master_and_spread() {
        let snapshot = snapshot_with_master_and_spread();
        let inferred = LayoutInference::infer(&snapshot);
        let master_index = inferred.topology.topo_order.iter().position(|id| id == "master-1").unwrap();
        let spread_index = inferred.topology.topo_order.iter().position(|id| id == "spread-1").unwrap();
        let page_index = inferred.topology.topo_order.iter().position(|id| id == "page-1").unwrap();
        assert!(master_index < page_index);
        assert!(spread_index < page_index);
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.node_count, 3);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
