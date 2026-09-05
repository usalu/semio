//! 💡️ GIS map inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::gismap::{GisMapDrawingChild, GisMapSnapshot, GisMapValueChild};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{create_node, inverse_semio_drawing_mutation, SemioDrawingMutation};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::{inverse_semio_value_mutation, SemioValueMutation};
use serde::{Deserialize, Serialize};

use super::bounds::{all_lon_lat_pairs, lon_lat_bounds, GisMapBounds};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gismap snapshot. Today: per-collection feature counts and the
/// geographic bounding box across every `positions`/`routes`/`regions` feature (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// feature collections here are small.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap.inference")]
pub struct GisMapInference {
    #[derived]
    pub position_count: usize,
    #[derived]
    pub route_count: usize,
    #[derived]
    pub region_count: usize,
    #[derived]
    pub bounds: Option<GisMapBounds>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapProposalError {
    Identity,
    Stale,
    Bounds,
    Composition,
}

/// 🧩️ One bounded typed parent+drawing+value work owner prepared from an immutable Map base.
#[derive(Debug, PartialEq)]
pub struct GisMapCreateRegionGroupWorkV1 {
    pub parent: crate::artifacts::gismap::mutations::GisMapMutation,
    pub parent_inverse: Vec<crate::artifacts::gismap::mutations::GisMapMutation>,
    pub drawing_child: GisMapDrawingChild,
    pub drawing: SemioDrawingMutation,
    pub drawing_inverse: Vec<SemioDrawingMutation>,
    pub value_child: GisMapValueChild,
    pub value: SemioValueMutation,
    pub value_inverse: Vec<SemioValueMutation>,
}

impl GisMapInference {
    /// 🌐️ Produces one typed region mutation, without applying it or granting approval authority.
    pub fn bounds_proposal(&self, snapshot: &GisMapSnapshot, job_id: &str) -> Result<crate::artifacts::gismap::mutations::GisMapMutation, GisMapProposalError> {
        use crate::artifacts::gismap::{
            mutations::{create_region::CreateRegion, GisMapMutation},
            MapFeature,
        };
        if job_id.len() != 32 || !job_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
            return Err(GisMapProposalError::Identity);
        }
        let id = format!("inference-{job_id}");
        if self.position_count != snapshot.positions.len() || self.route_count != snapshot.routes.len() || self.region_count != snapshot.regions.len() || snapshot.regions.iter().any(|region| region.id == id) {
            return Err(GisMapProposalError::Stale);
        }
        if self.region_count >= 65_536 {
            return Err(GisMapProposalError::Bounds);
        }
        let bounds = self.bounds.as_ref().ok_or(GisMapProposalError::Bounds)?;
        if ![bounds.lon_min, bounds.lon_max, bounds.lat_min, bounds.lat_max].iter().all(|value| value.is_finite())
            || bounds.lon_min < -180.0
            || bounds.lon_max > 180.0
            || bounds.lat_min < -90.0
            || bounds.lat_max > 90.0
            || bounds.lon_min > bounds.lon_max
            || bounds.lat_min > bounds.lat_max
        {
            return Err(GisMapProposalError::Bounds);
        }
        let ring = [[bounds.lon_min, bounds.lat_min], [bounds.lon_max, bounds.lat_min], [bounds.lon_max, bounds.lat_max], [bounds.lon_min, bounds.lat_max], [bounds.lon_min, bounds.lat_min]]
            .map(|point| dsl::DslValue::Array(point.map(dsl::DslValue::float).into()));
        let data = dsl::DslValue::object([("id".into(), dsl::DslValue::String(id.clone())), ("kind".into(), dsl::DslValue::String("inference-bounds".into())), ("ring".into(), dsl::DslValue::Array(ring.into()))]);
        Ok(GisMapMutation::CreateRegion(CreateRegion { index: snapshot.regions.len(), item: MapFeature { id, data } }))
    }

    /// 🧬️ Builds exactly one stable-member parent+drawing+value CreateRegion work group.
    pub fn create_region_group_work(&self, snapshot: &GisMapSnapshot, job_id: &str) -> Result<GisMapCreateRegionGroupWorkV1, GisMapProposalError> {
        use crate::artifacts::gismap::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
        use crate::artifacts::gismap::schema::{gis_map_descriptor_json, gis_map_snapshot_to_drawing};
        use dsl::{FromValue, ToValue};
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

        let parent = self.bounds_proposal(snapshot, job_id)?;
        let GisMapMutation::CreateRegion(created) = &parent else { return Err(GisMapProposalError::Composition) };
        if snapshot.image.is_some() || snapshot.drawing.child_id != "gismap-drawing" || snapshot.value.child_id != "gismap-value" {
            return Err(GisMapProposalError::Composition);
        }
        let parent_inverse = inverse_gis_map_mutation(snapshot, &parent);
        let mut after = snapshot.clone();
        apply_gis_map_mutation(&mut after, &parent).map_err(|_| GisMapProposalError::Composition)?;
        if after.drawing != snapshot.drawing || after.value != snapshot.value || after.image != snapshot.image {
            return Err(GisMapProposalError::Composition);
        }

        let before_drawing = gis_map_snapshot_to_drawing(snapshot);
        let after_drawing = gis_map_snapshot_to_drawing(&after);
        if before_drawing.schema != after_drawing.schema || before_drawing.canvas != after_drawing.canvas || before_drawing.styles != after_drawing.styles || before_drawing.layers.len() != 1 || after_drawing.layers.len() != 1 {
            return Err(GisMapProposalError::Composition);
        }
        let (before_children, after_children) = match (&before_drawing.layers[0].root, &after_drawing.layers[0].root) {
            (DrawNode::Group { transform: before_transform, children: before_children }, DrawNode::Group { transform: after_transform, children: after_children })
                if before_transform == after_transform
                    && before_drawing.layers[0].id == after_drawing.layers[0].id
                    && before_drawing.layers[0].name == after_drawing.layers[0].name
                    && before_drawing.layers[0].visible == after_drawing.layers[0].visible =>
            {
                (before_children, after_children)
            }
            _ => return Err(GisMapProposalError::Composition),
        };
        if after_children.len() != before_children.len() + 1 || !after_children.starts_with(before_children) {
            return Err(GisMapProposalError::Composition);
        }
        let drawing = SemioDrawingMutation::CreateNode(create_node::CreateNode { parent: NodePath { layer: 0, path: Vec::new() }, index: before_children.len(), node: after_children[before_children.len()].clone() });
        let drawing_inverse = inverse_semio_drawing_mutation(&drawing, &before_drawing);
        let mut projected_drawing = before_drawing.clone();
        apply_semio_drawing_mutation(&mut projected_drawing, &drawing);
        if projected_drawing != after_drawing {
            return Err(GisMapProposalError::Composition);
        }

        let before_value = crate::artifacts::gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(snapshot));
        let after_value = crate::artifacts::gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&after));
        let value_payload = crate::artifacts::gismap::semio_value_from_serde_json(&serde_json::Value::from(&created.item.data));
        let value = SemioValueMutation::from_value(dsl::DslValue::object([
            ("mutation".into(), dsl::DslValue::String("insertListItem".into())),
            ("path".into(), dsl::DslValue::Array(vec![dsl::DslValue::object([("kind".into(), dsl::DslValue::String("key".into())), ("key".into(), dsl::DslValue::String("regions".into()))])])),
            ("index".into(), created.index.to_value()),
            ("value".into(), value_payload.to_value()),
        ]))
        .map_err(|_| GisMapProposalError::Composition)?;
        let value_inverse = inverse_semio_value_mutation(&value, &before_value);
        let mut projected_value = before_value.clone();
        apply_semio_value_mutation(&mut projected_value, &value);
        if projected_value != after_value {
            return Err(GisMapProposalError::Composition);
        }
        let bytes = dsl::os_pack::json::to_json_string(&parent).len()
            + dsl::os_pack::json::to_json_string(&parent_inverse).len()
            + dsl::os_pack::json::to_json_string(&drawing).len()
            + dsl::os_pack::json::to_json_string(&drawing_inverse).len()
            + dsl::os_pack::json::to_json_string(&value).len()
            + dsl::os_pack::json::to_json_string(&value_inverse).len();
        if bytes > 65_536 {
            return Err(GisMapProposalError::Bounds);
        }
        Ok(GisMapCreateRegionGroupWorkV1 { parent, parent_inverse, drawing_child: snapshot.drawing.clone(), drawing, drawing_inverse, value_child: snapshot.value.clone(), value, value_inverse })
    }
}

impl protocol::Inference<GisMapSnapshot> for GisMapInference {
    fn infer(snapshot: &GisMapSnapshot) -> Self {
        Self { position_count: snapshot.positions.len(), route_count: snapshot.routes.len(), region_count: snapshot.regions.len(), bounds: lon_lat_bounds(&all_lon_lat_pairs(snapshot)) }
    }
}

impl protocol::InferenceSpec<GisMapSnapshot> for GisMapInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gismap.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.positionCount", reads: &["positions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.routeCount", reads: &["routes"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.regionCount", reads: &["regions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.bounds", reads: &["positions", "routes", "regions"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::gismap::standards::v1::subsets::any::schema::GismapBuilder {
    type Snapshot = GisMapSnapshot;
    type Inference = GisMapInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gismap.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gismap_artifact_schema_descriptor`'s registration.
pub fn gismap_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.gis.gismap.inference",
        inference: schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::gismap::{gis_map_snapshot_with_derived_children, MapFeature};
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = GisMapSnapshot { positions: vec![MapFeature { id: "p1".into(), data: dsl::DslValue::from(serde_json::json!({ "lon": 1.0, "lat": 2.0 })) }], routes: Vec::new(), regions: Vec::new(), ..Default::default() };
        assert_eq!(GisMapInference::infer(&snapshot), GisMapInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(GisMapInference::infer(&GisMapSnapshot::default()), GisMapInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn map_create_region_group_work_stabilizes_parent_drawing_value_without_image() {
        use crate::artifacts::gismap::mutations::apply_gis_map_mutation;
        use crate::artifacts::gismap::schema::{gis_map_descriptor_json, gis_map_snapshot_to_drawing};
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

        let feature = |id: &str, data: serde_json::Value| MapFeature { id: id.into(), data: dsl::DslValue::from(data) };
        let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot {
            positions: vec![feature("point-a", serde_json::json!({ "id": "point-a", "lon": 7, "lat": 47 }))],
            routes: vec![feature("route-a", serde_json::json!({ "id": "route-a", "points": [[8, 46], [9, 48]] }))],
            ..Default::default()
        });
        let inferred = GisMapInference::infer(&snapshot);
        let work = inferred.create_region_group_work(&snapshot, "11111111111111111111111111111111").expect("typed group work");
        assert_eq!(work.drawing_child.child_id, "gismap-drawing");
        assert_eq!(work.value_child.child_id, "gismap-value");
        assert!(snapshot.image.is_none());

        let mut parent_after = snapshot.clone();
        apply_gis_map_mutation(&mut parent_after, &work.parent).expect("parent applies");
        assert_eq!(parent_after.drawing, snapshot.drawing);
        assert_eq!(parent_after.value, snapshot.value);
        let before_drawing = gis_map_snapshot_to_drawing(&snapshot);
        let after_drawing = gis_map_snapshot_to_drawing(&parent_after);
        let mut projected_drawing = before_drawing.clone();
        apply_semio_drawing_mutation(&mut projected_drawing, &work.drawing);
        assert_eq!(projected_drawing, after_drawing);
        for inverse in &work.drawing_inverse {
            apply_semio_drawing_mutation(&mut projected_drawing, inverse);
        }
        assert_eq!(projected_drawing, before_drawing);

        let before_value = crate::artifacts::gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&snapshot));
        let after_value = crate::artifacts::gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&parent_after));
        let mut projected_value = before_value.clone();
        apply_semio_value_mutation(&mut projected_value, &work.value);
        assert_eq!(projected_value, after_value);
        for inverse in &work.value_inverse {
            apply_semio_value_mutation(&mut projected_value, inverse);
        }
        assert_eq!(projected_value, before_value);

        for inverse in &work.parent_inverse {
            apply_gis_map_mutation(&mut parent_after, inverse).expect("parent inverse applies");
        }
        assert_eq!(parent_after, snapshot);
        let mut forged = snapshot.clone();
        forged.drawing.child_id = "forged".into();
        assert_eq!(inferred.create_region_group_work(&forged, "11111111111111111111111111111111").unwrap_err(), GisMapProposalError::Composition);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
