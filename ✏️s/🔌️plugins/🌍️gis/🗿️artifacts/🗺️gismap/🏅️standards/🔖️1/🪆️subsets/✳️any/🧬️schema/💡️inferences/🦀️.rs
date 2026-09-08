//! 💡️ GIS map inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::{GisMapDrawingChild, GisMapSnapshot, GisMapValueChild};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::{create_node, inverse_semio_drawing_mutation, SemioDrawingMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::DrawNode;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::{inverse_semio_value_mutation, SemioValueMutation};

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
    pub parent: crate::mutations::GisMapMutation,
    pub parent_inverse: Vec<crate::mutations::GisMapMutation>,
    pub drawing_child: GisMapDrawingChild,
    pub drawing: SemioDrawingMutation,
    pub drawing_inverse: Vec<SemioDrawingMutation>,
    pub value_child: GisMapValueChild,
    pub value: SemioValueMutation,
    pub value_inverse: Vec<SemioValueMutation>,
}

impl GisMapInference {
    /// 🌐️ Produces one typed region mutation, without applying it or granting approval authority.
    pub fn bounds_proposal(&self, snapshot: &GisMapSnapshot, job_id: &str) -> Result<crate::mutations::GisMapMutation, GisMapProposalError> {
        use crate::{
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
        use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
        use crate::schema::{gis_map_descriptor_json, gis_map_snapshot_to_drawing};
        use dsl::{FromValue, ToValue};
        use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::apply_semio_drawing_mutation;
        use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::apply_semio_value_mutation;

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

        let before_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(snapshot));
        let after_value = crate::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(&after));
        let value_payload = crate::semio_value_from_serde_json(&serde_json::Value::from(&created.item.data));
        let value = SemioValueMutation::from_value(dsl::DslValue::object([
            ("mutation".into(), dsl::DslValue::String("insertListItem".into())),
            ("path".into(), dsl::DslValue::Array(vec![dsl::DslValue::object([("kind".into(), dsl::DslValue::String("key".into())), ("key".into(), dsl::DslValue::String("regions".into()))])])),
            ("index".into(), created.index.to_value()),
            ("value".into(), value_payload.to_value()),
        ]))
        .map_err(|_| GisMapProposalError::Composition)?;
        let value_inverse = inverse_semio_value_mutation(&value, &before_value);
        let mut projected_value = before_value;
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
impl semio_framework_plugin::ArtifactInferrer for crate::standards::v1::subsets::any::schema::GismapBuilder {
    type Snapshot = GisMapSnapshot;
    type Inference = GisMapInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gismap.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gismap_artifact_schema_descriptor`'s registration.
pub fn gismap_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.gis.gismap.inference",
        inference: ::semio_framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
