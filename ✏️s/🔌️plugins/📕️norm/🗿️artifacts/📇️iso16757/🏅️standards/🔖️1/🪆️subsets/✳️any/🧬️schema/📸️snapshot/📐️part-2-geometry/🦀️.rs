//! 📐️ Part 2 of the ISO 16757 snapshot: the space classification, ports, bounding box, geometry nodes and objects, surfaces and the geometry catalogue. See [`super`].

use std::collections::BTreeMap;

/// 📐️ Space classification per Part 2 §5.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum SpaceKind {
    Overall,
    Operation,
    Access,
    #[dsl(key = "placementTransportation")]
    PlacementTransportation,
    Installation,
}

/// 🔌️ Port medium and direction.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PortDefinition {
    pub id: String,
    pub medium: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    pub port_type: String,
}

/// 📦️ Axis-aligned bounding box.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BoundingBox {
    pub fn from_size(width: f64, height: f64, depth: f64) -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [width, height, depth] }
    }

    pub fn volume_m3(self) -> f64 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        dx * dy * dz
    }

    pub fn overlaps(self, other: Self, clearance_m: f64) -> bool {
        self.min[0] - clearance_m < other.max[0]
            && self.max[0] + clearance_m > other.min[0]
            && self.min[1] - clearance_m < other.max[1]
            && self.max[1] + clearance_m > other.min[1]
            && self.min[2] - clearance_m < other.max[2]
            && self.max[2] + clearance_m > other.min[2]
    }
}

/// 🧱️ CSG primitive kind registry entry.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PrimitiveKind {
    pub id: String,
    pub parameters: Vec<String>,
}

/// 🌳️ Geometry node in a CSG tree — recursive `#[derive(dsl::DslEnum)]`: `Transform.child` is
/// `#[dsl(statements)] Box<GeometryNode>` (exactly one nested tagged value) and
/// `Boolean.children` is `#[dsl(statements, block)] Vec<GeometryNode>` (a nested tagged
/// collection), both recursing back into this same enum's own `DslVariants` impl.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "node", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(tag = "node", rename_all = "camelCase")]
#[dsl(retire_with="crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_geometry")]
pub enum GeometryNode {
    Primitive {
        kind: String,
        parameters: BTreeMap<String, f64>,
    },
    Transform {
        translation: [f64; 3],
        rotation_deg: [f64; 3],
        #[dsl(statements, block)]
        child: Box<GeometryNode>,
    },
    Boolean {
        operator: BooleanOperator,
        #[dsl(statements, block)]
        children: Vec<GeometryNode>,
    },
    Reference {
        geometry_id: String,
    },
}

/// ➕️ Boolean CSG operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanOperator {
    Union,
    Intersection,
    Difference,
}

/// 🏗️ Complete geometry object.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GeometryObject {
    pub id: String,
    #[dsl(statements, block)]
    pub shape: Option<GeometryNode>,
    #[dsl(statements, block)]
    pub symbolic: Option<GeometryNode>,
    pub spaces: Vec<SpaceEnvelope>,
    pub surfaces: Vec<SurfaceDefinition>,
    pub ports: Vec<PortDefinition>,
    pub parameter_bindings: BTreeMap<String, String>,
}

/// 📦️ Space envelope with kind.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SpaceEnvelope {
    #[cfg_attr(test, serde(default = "default_installation_space_id"))]
    pub id: String,
    pub kind: SpaceKind,
    pub bounds: BoundingBox,
}

fn default_installation_space_id() -> String {
    "installation".into()
}

/// 🎨️ Semantic surface.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SurfaceDefinition {
    pub id: String,
    pub purpose: String,
    pub bounds: BoundingBox,
}

/// 📚️ Geometry catalogue index.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GeometryCatalogue {
    pub objects: BTreeMap<String, GeometryObject>,
    #[dsl(table)]
    pub primitive_registry: Vec<PrimitiveKind>,
}

impl GeometryCatalogue {
    pub fn default_primitives() -> Vec<PrimitiveKind> {
        vec![
            PrimitiveKind { id: "box".into(), parameters: vec!["width".into(), "height".into(), "depth".into()] },
            PrimitiveKind { id: "cylinder".into(), parameters: vec!["radius".into(), "height".into()] },
            PrimitiveKind { id: "sphere".into(), parameters: vec!["radius".into()] },
        ]
    }
}
