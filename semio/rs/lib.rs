//! 🦀 semio rust control plane — in-memory Arc-reference architecture (code-first GraphQL).
//!
//! Every entity is one hand-written Rust struct shared as
//! `Arc<Self>` with interior `async_lock::RwLock` per mutable field. GraphQL resolvers take
//! `&self` on the entity (deref'd through the Arc) and return `Arc<Child>` for relationships,
//! so a query like `wip.theKit.design(id).piece(id).position` only acquires the locks it actually
//! needs and never deep-copies an aggregate Vec. There is exactly **one** `emit_event` in
//! the entire crate ([`event::EventBus::emit_event`]); every mutation routes through it.
//!
//! Worker topology: a parent router hosts the GraphQL schema and dispatches commands to two
//! child workers (`wip` + `authoritative`), each owning its own [`vcs::Graph`] as a shared
//! `Arc`. On native targets both children run as in-process async actors; on `wasm32` they
//! live in dedicated web workers wired through [`wasm_bridge`].
//!
//! Kit graph engine ([`kit_graph_engine`]): pointer-backed design/piece slot tables, deterministic
//!  diffs, and `projectionFingerprint` aligned with kit-store golden fixtures.

#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

//#region 🧬 entity_dsl

/// 📜 SDL fragment registry: each entity family implements [`HasSdlFragment`] (W1+); W0 keeps the hook + static golden tail.
pub mod sdl_registry {
    /// @emoji 📜 One collected `type` / `interface` / `input` SDL block per registered entity or operation group.
    pub trait HasSdlFragment {
        const SDL_FRAGMENT: &'static str;
    }

    /// 📜 Ordered list of static fragments (empty until entity families emit `SDL_FRAGMENT` constants).
    pub fn all_fragments() -> Vec<&'static str> {
        let mut v = Vec::new();
        crate::push_all_fragments(&mut v);
        crate::push_operation_fragments(&mut v);
        v
    }
}

/// @emoji 🧬 Roster: records entity families for `push_all_fragments` and later owner/interface codegen (bodies are macro-only per AGENTS).
macro_rules! register_entities {
    ( $( $region:ident : [ $( $name:ident ),* $(,)? ] ),* $(,)? ) => {
        pub(crate) fn push_all_fragments(out: &mut Vec<&'static str>) {
            let _ = out;
            $( $(
                let _ = stringify!($name);
            )* )*
        }
    };
}

macro_rules! register_operations {
    ( $( $artifact:ident : [ $( $name:ident ),* $(,)? ] ),* $(,)? ) => {
        pub(crate) fn push_operation_fragments(out: &mut Vec<&'static str>) {
            let _ = out;
            $( $(
                let _ = stringify!($name);
            )* )*
        }
    };
}

register_entities! {
    geom:   [Vector, Point, Coordinate, Offset, Plane, Position, Location, Place],
    meta:   [Attribute, Author, File, Folder, Prop, Benchmark, Quality, Tag, Concept, Stat, Layer, Group, Family],
    type_:  [Type, Port, Connector, Representation],
    design: [Design, Piece, Side, Connection, Clump],
    root:   [Kit],
    vcs:    [Edit, Change, Checkpoint, TheKit, Alternative, Graph, Session, Conflict],
}

register_operations! {
    tag:        [CreatedTag, CreatedTags, RenamedTag, UpdatedTagDescription, UpdatedTagIcon, AddedAttributeToTag, AddedAttributesToTag, RemovedAttributeFromTag, RemovedAttributesFromTag, DeletedTag, DeletedTags],
    concept:    [CreatedConcept, RenamedConcept, UpdatedConceptDescription, UpdatedConceptIcon, AddedAttributeToConcept, AddedAttributesToConcept, RemovedAttributeFromConcept, RemovedAttributesFromConcept, DeletedConcept, DeletedConcepts],
    quality:    [CreatedQuality, RenamedQuality, UpdatedQualityDescription, UpdatedQualityIcon, AddedAttributeToQuality, AddedAttributesToQuality, RemovedAttributeFromQuality, RemovedAttributesFromQuality, DeletedQuality, DeletedQualities],
    port:       [CreatedPort, CreatedPorts, RenamedPort, UpdatedPortDescription, UpdatedPortIcon, AddedAttributeToPort, AddedAttributesToPort, RemovedAttributeFromPort, RemovedAttributesFromPort, DeletedPort, DeletedPorts],
    type_:      [CreatedType, RenamedType],
    design:     [CreatedDesign, CreatedDesigns, DeletedDesign, DeletedDesigns, FlattenedDesign, AddedAttributeToDesign, AddedAttributesToDesign, RemovedAttributeFromDesign, RemovedAttributesFromDesign],
    piece:      [CreatedFixedPiece, FixedPiece, FixedPieces, DraggedPieces, DraggedPiece, AddedChildPieceWithParentConnection, AddedChildPiecesWithParentConnections, AddedHangingChildPieceWithParentConnection, AddedHangingChildPiecesWithParentConnections, RenamedPiece, UpdatedPieceDescription, MovedPiece, MovedPieces, ChangedPieceToType, ChangedPiecesToType, AddedAttributeToPiece, AddedAttributesToPiece, RemovedAttributeFromPiece, RemovedAttributesFromPiece, DeletedPiece, DeletedPieces, DeletedPiecesAndConnections],
    kit:        [RenamedKit, ChangedDescription],
    connector:  [AddedConnector, AddedConnectors, RenamedConnector, UpdatedConnectorDescription, UpdatedConnectorIcon, RemovedConnector, RemovedConnectors],
}

#[macro_export]
macro_rules! simple_conn_sync {
    ($Conn:ident, $Edge:ident, $node:ty, $hash_fn:expr) => {
        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Edge {
            pub cursor: String,
            pub node: $node,
        }

        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Conn {
            pub edges: Vec<$Edge>,
            #[graphql(name = "pageInfo")]
            pub page_info: std::sync::Arc<$crate::gql_relay::PageInfo>,
            pub hash: String,
        }

        impl $Conn {
            pub fn from_rows(rows: Vec<$node>) -> Self {
                let mut child_hashes = Vec::with_capacity(rows.len());
                for r in &rows {
                    child_hashes.push($hash_fn(r));
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = rows.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        }
    };
}

#[macro_export]
macro_rules! simple_conn_entity {
    ($Conn:ident, $Edge:ident, $node:ty) => {
        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Edge {
            pub cursor: String,
            pub node: $node,
        }

        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Conn {
            pub edges: Vec<$Edge>,
            #[graphql(name = "pageInfo")]
            pub page_info: std::sync::Arc<$crate::gql_relay::PageInfo>,
            pub hash: String,
        }

        impl $Conn {
            pub async fn from_rows(rows: Vec<$node>) -> Self {
                let mut child_hashes = Vec::with_capacity(rows.len());
                for r in &rows {
                    child_hashes.push(r.compute_hash().await);
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = rows.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        }
    };
}

/// @emoji 🪢 `entity_full_family!` — relay Edge/Connection for geometry (`VectorEdge`…`LocationEdge`).
#[macro_export]
macro_rules! entity_full_family {
    (
        $base:ident,
        $node:ty,
        relay = ($conn:ident, $edge:ident)
    ) => {
        paste::paste! {
            simple_conn_entity!($conn, $edge, $node);
        }
    };
}

/// @emoji 🪢 `entity_relay!` — forwards to [`simple_conn_entity!`] for non-geometry relay shells.
#[macro_export]
macro_rules! entity_relay {
    ($Conn:ident, $Edge:ident, $Node:ty) => {
        simple_conn_entity!($Conn, $Edge, $Node);
    };
}

/// @emoji 🪜 `entity_diffs!` — expands modification / diff relay ladder (filled when diff families migrate).
#[macro_export]
macro_rules! entity_diffs {
    ($($_base:ident),* $(,)?) => {};
}

/// @emoji 🪢 `entity_owner!` — expands owner/owned union shells (filled when mega-unions derive from roster).
#[macro_export]
macro_rules! entity_owner {
    ($($_base:ident),* $(,)?) => {};
}

//#endregion 🧬 entity_dsl

//#region 🆔 id

pub mod id {
    //! 🆔 Immutable uuid-v7 wrapper used by every entity.
    use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Id(pub String);

    impl Id {
        /// 🆕 Mint a fresh uuid-v7 (timestamped, monotonic).
        pub async fn new() -> Self {
            Self(uuid::Uuid::now_v7().to_string())
        }

        pub(crate) fn new_sync() -> Self {
            Self(uuid::Uuid::now_v7().to_string())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl From<String> for Id {
        fn from(s: String) -> Self {
            Self(s)
        }
    }

    impl From<&str> for Id {
        fn from(s: &str) -> Self {
            Self(s.to_string())
        }
    }

    #[Scalar]
    impl ScalarType for Id {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok(Self(s)),
                _ => Err(InputValueError::expected_type(value)),
            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }
}

//#endregion 🆔 id

//#region ⏱️ timestamp

pub mod timestamp {
    //! ⏱️ ISO-8601 millisecond-precision timestamp scalar.
    use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Timestamp(pub String);

    #[Scalar]
    impl ScalarType for Timestamp {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok(Self(s)),
                _ => Err(InputValueError::expected_type(value)),
            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }
}

//#endregion ⏱️ timestamp

//#region 🚨 error

pub mod error {
    //! 🚨 Crate-wide error type wired through the event bus as `OperationFailed`.
    use async_graphql::SimpleObject;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    #[derive(Clone, Debug, Error, Serialize, Deserialize, SimpleObject)]
    #[graphql(name = "Error")]
    #[error("{kind}: {message}")]
    pub struct SemioError {
        pub kind: String,
        pub message: String,
        pub request_id: Option<String>,
    }

    impl Default for SemioError {
        fn default() -> Self {
            Self { kind: String::new(), message: String::new(), request_id: None }
        }
    }

    impl SemioError {
        pub fn invalid<S: Into<String>>(msg: S) -> Self {
            Self { kind: "Invalid".to_string(), message: msg.into(), request_id: None }
        }
        pub fn not_found<S: Into<String>>(kind: S, id: S) -> Self {
            Self { kind: "NotFound".to_string(), message: format!("{}({})", kind.into(), id.into()), request_id: None }
        }
        pub fn with_request(mut self, id: crate::id::Id) -> Self {
            self.request_id = Some(id.0);
            self
        }
    }

    pub type Result<T> = std::result::Result<T, SemioError>;
}

//#endregion 🚨 error

//#region 📐 geom

pub mod geom {
    //! 📐 Geometry: `values` (Copy DTOs for serde / kit engine) + `entity` (`Arc` graph nodes, WeakEntity-style ids).
    use async_graphql::InputObject;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "VectorInput")]
    pub struct Vector {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "PointInput")]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "CoordinateInput")]
    pub struct Coordinate {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "OffsetInput")]
    pub struct Offset {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "PlaneInput")]
    pub struct Plane {
        #[serde(default)]
        pub origin: Point,
        #[graphql(name = "xAxis")]
        #[serde(alias = "xAxis", default)]
        pub x_axis: Vector,
        #[graphql(name = "yAxis")]
        #[serde(alias = "yAxis", default)]
        pub y_axis: Vector,
    }

    impl Default for Plane {
        /// @emoji ◭ World XY plane through origin; hydrates kit JSON that omits plane axes.
        fn default() -> Self {
            Self { origin: Point::default(), x_axis: Vector { x: 1.0, y: 0.0, z: 0.0 }, y_axis: Vector { x: 0.0, y: 1.0, z: 0.0 } }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "PositionInput")]
    pub struct Position {
        #[serde(default)]
        pub center: Coordinate,
        #[serde(default)]
        pub plane: Plane,
    }

    impl Default for Position {
        fn default() -> Self {
            Self { center: Coordinate::default(), plane: Plane::default() }
        }
    }

    /// @emoji 🌍 Wire `LocationInput` (lon/lat/alt) for [`entity::LocationNode`].
    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "LocationInput")]
    pub struct LocationInput {
        pub longitude: f64,
        pub latitude: f64,
        pub altitude: f64,
    }

    //#region 📐 entity
    pub mod entity {
        //! 📐 `Arc` geometry nodes (target WeakEntity / Entity graph shapes); `#[Object]` impls live after [`crate::iface`].
        use std::sync::Arc;

        use async_lock::RwLock;

        use crate::hash::{h, merkle_node_str};
        use crate::id::Id;

        use super::{Coordinate, Plane, Point, Position, Vector};

        fn weak(prefix: &str, parts: &[&str]) -> Id {
            Id::from(format!("semio:weak:{prefix}:{}", h(parts)))
        }

        /// @emoji 📍 Coordinate WeakEntity data node.
        #[derive(Debug)]
        pub struct CoordinateNode {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl CoordinateNode {
            pub fn from_value(c: Coordinate) -> Arc<Self> {
                let id = weak("coordinate", &[&format!("{:.9}", c.u), &format!("{:.9}", c.v)]);
                Arc::new(Self { id, u: RwLock::new(c.u), v: RwLock::new(c.v) })
            }

            /// @emoji 🪪 Merkle leaf: id + live u/v (matches [`super::Coordinate`] payload).
            pub async fn compute_hash(&self) -> String {
                let u = *self.u.read().await;
                let v = *self.v.read().await;
                merkle_node_str(&["semio:geom:Coordinate", self.id.as_str(), &format!("{u:.9}"), &format!("{v:.9}")], Vec::new())
            }
        }

        /// @emoji ↗ Vector WeakEntity data node.
        #[derive(Debug)]
        pub struct VectorNode {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl VectorNode {
            pub fn from_value(v: Vector) -> Arc<Self> {
                let id = weak("vector", &[&format!("{:.9}", v.x), &format!("{:.9}", v.y), &format!("{:.9}", v.z)]);
                Arc::new(Self { id, x: RwLock::new(v.x), y: RwLock::new(v.y), z: RwLock::new(v.z) })
            }

            /// @emoji 🪪 Merkle leaf: id + live x/y/z.
            pub async fn compute_hash(&self) -> String {
                let x = *self.x.read().await;
                let y = *self.y.read().await;
                let z = *self.z.read().await;
                merkle_node_str(&["semio:geom:Vector", self.id.as_str(), &format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")], Vec::new())
            }
        }

        /// @emoji ◆ Point WeakEntity data node.
        #[derive(Debug)]
        pub struct PointNode {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl PointNode {
            pub fn from_value(p: Point) -> Arc<Self> {
                let id = weak("point", &[&format!("{:.9}", p.x), &format!("{:.9}", p.y), &format!("{:.9}", p.z)]);
                Arc::new(Self { id, x: RwLock::new(p.x), y: RwLock::new(p.y), z: RwLock::new(p.z) })
            }

            /// @emoji 🪪 Merkle leaf: id + live x/y/z.
            pub async fn compute_hash(&self) -> String {
                let x = *self.x.read().await;
                let y = *self.y.read().await;
                let z = *self.z.read().await;
                merkle_node_str(&["semio:geom:Point", self.id.as_str(), &format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")], Vec::new())
            }
        }

        /// @emoji ▭ Plane WeakEntity data node (owns origin + axes).
        #[derive(Debug)]
        pub struct PlaneNode {
            pub id: Id,
            pub origin: Arc<PointNode>,
            pub x_axis: Arc<VectorNode>,
            pub y_axis: Arc<VectorNode>,
        }

        impl PlaneNode {
            pub fn from_value(pl: Plane) -> Arc<Self> {
                let origin = PointNode::from_value(pl.origin);
                let x_axis = VectorNode::from_value(pl.x_axis);
                let y_axis = VectorNode::from_value(pl.y_axis);
                let id = weak("plane", &[origin.id.as_str(), x_axis.id.as_str(), y_axis.id.as_str()]);
                Arc::new(Self { id, origin, x_axis, y_axis })
            }

            /// @emoji 🪪 Merkle node: sorted child digests of origin + axes.
            pub async fn compute_hash(&self) -> String {
                let mut ch = vec![self.origin.compute_hash().await, self.x_axis.compute_hash().await, self.y_axis.compute_hash().await];
                ch.sort();
                merkle_node_str(&["semio:geom:Plane", self.id.as_str()], ch)
            }
        }

        /// @emoji ↖ WeakEntity-style offset (piece drag input echo).
        #[derive(Debug)]
        pub struct OffsetNode {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl OffsetNode {
            pub fn from_value(o: super::Offset) -> Arc<Self> {
                let id = weak("offset", &[&format!("{:.9}", o.u), &format!("{:.9}", o.v)]);
                Arc::new(Self { id, u: RwLock::new(o.u), v: RwLock::new(o.v) })
            }

            /// @emoji 🪪 Merkle leaf: id + live u/v.
            pub async fn compute_hash(&self) -> String {
                let u = *self.u.read().await;
                let v = *self.v.read().await;
                merkle_node_str(&["semio:geom:Offset", self.id.as_str(), &format!("{u:.9}"), &format!("{v:.9}")], Vec::new())
            }
        }

        /// @emoji ⌖ Position WeakEntity root (center + plane); mirrors live [`super::Position`] DTO via RwLock sync.
        #[derive(Debug)]
        pub struct PositionNode {
            pub id: Id,
            pub center: Arc<CoordinateNode>,
            pub plane: Arc<PlaneNode>,
            pub data: RwLock<Position>,
        }

        impl PositionNode {
            pub fn from_position_value(value: Position) -> Arc<Self> {
                let center = CoordinateNode::from_value(value.center);
                let plane = PlaneNode::from_value(value.plane);
                let id = weak("position", &[center.id.as_str(), plane.id.as_str()]);
                Arc::new(Self { id, center, plane, data: RwLock::new(value) })
            }

            pub async fn snapshot_value(&self) -> Position {
                *self.data.read().await
            }

            /// @emoji 🪪 Merkle node: live [`Position`] payload plus sorted digests of center + plane arcs.
            pub async fn compute_hash(&self) -> String {
                let p = *self.data.read().await;
                let flat = format!(
                    "{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}",
                    p.center.u, p.center.v, p.plane.origin.x, p.plane.origin.y, p.plane.origin.z, p.plane.x_axis.x, p.plane.x_axis.y, p.plane.x_axis.z, p.plane.y_axis.x, p.plane.y_axis.y, p.plane.y_axis.z,
                );
                let mut ch = vec![self.center.compute_hash().await, self.plane.compute_hash().await];
                ch.sort();
                merkle_node_str(&["semio:geom:Position", self.id.as_str(), flat.as_str()], ch)
            }
        }

        /// @emoji 🌍 WeakEntity-style geographic location (lon/lat/alt).
        #[derive(Debug)]
        pub struct LocationNode {
            pub id: Id,
            pub longitude: RwLock<f64>,
            pub latitude: RwLock<f64>,
            pub altitude: RwLock<f64>,
        }

        impl LocationNode {
            pub fn from_value(loc: super::LocationInput) -> Arc<Self> {
                let id = weak("location", &[&format!("{:.9}", loc.longitude), &format!("{:.9}", loc.latitude), &format!("{:.9}", loc.altitude)]);
                Arc::new(Self { id, longitude: RwLock::new(loc.longitude), latitude: RwLock::new(loc.latitude), altitude: RwLock::new(loc.altitude) })
            }

            /// @emoji 🪪 Merkle leaf over lon/lat/alt fields.
            pub async fn compute_hash(&self) -> String {
                let lo = *self.longitude.read().await;
                let la = *self.latitude.read().await;
                let al = *self.altitude.read().await;
                merkle_node_str(&["semio:geom:Location", self.id.as_str(), &format!("{lo:.9}"), &format!("{la:.9}"), &format!("{al:.9}")], Vec::new())
            }
        }

        /// @emoji 🧭 Placeholder StrongEntity shell for `Place` (full meta wiring lands with meta lift).
        #[derive(Debug)]
        pub struct PlaceNode {
            pub id: Id,
            pub label: RwLock<Option<String>>,
        }

        impl PlaceNode {
            pub async fn new() -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, label: RwLock::new(None) })
            }

            /// @emoji 🪪 Merkle leaf: id + optional label.
            pub async fn compute_hash(&self) -> String {
                let lb = self.label.read().await.clone().unwrap_or_default();
                merkle_node_str(&["semio:geom:Place", self.id.as_str(), lb.as_str()], Vec::new())
            }
        }

        //#region 🔧 Default stubs (schema codegen union / interface defaults)
        impl Default for CoordinateNode {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for VectorNode {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for PointNode {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for PlaneNode {
            fn default() -> Self {
                Self { id: Id::default(), origin: Arc::new(PointNode::default()), x_axis: Arc::new(VectorNode::default()), y_axis: Arc::new(VectorNode::default()) }
            }
        }

        impl Default for OffsetNode {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for PositionNode {
            fn default() -> Self {
                Self { id: Id::default(), center: Arc::new(CoordinateNode::default()), plane: Arc::new(PlaneNode::default()), data: RwLock::new(Position::default()) }
            }
        }

        impl Default for LocationNode {
            fn default() -> Self {
                Self { id: Id::default(), longitude: RwLock::new(0.0), latitude: RwLock::new(0.0), altitude: RwLock::new(0.0) }
            }
        }

        impl Default for PlaceNode {
            fn default() -> Self {
                Self { id: Id::default(), label: RwLock::new(None) }
            }
        }
        //#endregion 🔧 Default stubs (schema codegen union / interface defaults)
    }
    //#endregion 📐 entity
}

//#endregion 📐 geom

//#region 🪢 gql_relay

/// 🪢 Relay `PageInfo` + connection shells for static GraphQL (edges, pageInfo, hash).
#[allow(unused_macros)]
pub mod gql_relay {
    use std::sync::Arc;

    use async_graphql::SimpleObject;

    use crate::hash::{h, merkle_collection};
    use crate::id::Id;
    use crate::kit::design::connection::Side;
    use crate::kit::design::piece::Piece;
    use crate::kit::design::Design;
    use crate::kit::r#type::{Connector, Representation, Type};
    use crate::meta::{Author, Benchmark, Concept, File, Folder, Group, Layer, Prop, Quality, Stat, Tag};
    use crate::vcs::{Alternative, Change, Checkpoint, Conflict};

    fn edge_cursor(i: usize) -> String {
        format!("e{i}")
    }

    #[derive(Clone, Debug, Default, SimpleObject)]
    pub struct PageInfo {
        #[graphql(name = "hasNextPage")]
        pub has_next_page: bool,
        #[graphql(name = "hasPreviousPage")]
        pub has_previous_page: bool,
        #[graphql(name = "startCursor")]
        pub start_cursor: Option<String>,
        #[graphql(name = "endCursor")]
        pub end_cursor: Option<String>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct DesignEdge {
        pub cursor: String,
        pub node: Arc<Design>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct DesignConnection {
        pub edges: Vec<DesignEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl DesignConnection {
        pub async fn from_designs(rows: Vec<Arc<Design>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for d in &rows {
                child_hashes.push(d.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, d)| DesignEdge { cursor: edge_cursor(i), node: d }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct PieceEdge {
        pub cursor: String,
        pub node: Arc<Piece>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct PieceConnection {
        pub edges: Vec<PieceEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl PieceConnection {
        pub async fn from_pieces(rows: Vec<Arc<Piece>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for p in &rows {
                child_hashes.push(p.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, p)| PieceEdge { cursor: edge_cursor(i), node: p }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct TypeEdge {
        pub cursor: String,
        pub node: Arc<Type>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct TypeConnection {
        pub edges: Vec<TypeEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl TypeConnection {
        pub async fn from_types(rows: Vec<Arc<Type>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for t in &rows {
                child_hashes.push(t.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, t)| TypeEdge { cursor: edge_cursor(i), node: t }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct ConnectorEdge {
        pub cursor: String,
        pub node: Arc<Connector>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct ConnectorConnection {
        pub edges: Vec<ConnectorEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl ConnectorConnection {
        pub async fn from_connectors(rows: Vec<Arc<Connector>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for c in &rows {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, c)| ConnectorEdge { cursor: edge_cursor(i), node: c }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct RepresentationEdge {
        pub cursor: String,
        pub node: Arc<Representation>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct RepresentationConnection {
        pub edges: Vec<RepresentationEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl RepresentationConnection {
        pub async fn from_representations(rows: Vec<Arc<Representation>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for r in &rows {
                child_hashes.push(r.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, r)| RepresentationEdge { cursor: edge_cursor(i), node: r }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct SideEdge {
        pub cursor: String,
        pub node: Arc<Side>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct SideConnection {
        pub edges: Vec<SideEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl SideConnection {
        pub async fn from_sides(rows: Vec<Arc<Side>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for s in &rows {
                child_hashes.push(s.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, s)| SideEdge { cursor: edge_cursor(i), node: s }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct BlueprintEdge {
        pub cursor: String,
        pub node: crate::kit::r#type::Blueprint,
    }

    #[derive(Clone, SimpleObject)]
    pub struct BlueprintConnection {
        pub edges: Vec<BlueprintEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl BlueprintConnection {
        pub async fn from_blueprints(rows: Vec<crate::kit::r#type::Blueprint>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for b in &rows {
                let h = match b {
                    crate::kit::r#type::Blueprint::Type(t) => t.compute_hash().await,
                    crate::kit::r#type::Blueprint::Design(d) => d.compute_hash().await,
                };
                child_hashes.push(h);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, node)| BlueprintEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct ConflictEdge {
        pub cursor: String,
        pub node: Arc<Conflict>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct ConflictConnection {
        pub edges: Vec<ConflictEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl ConflictConnection {
        pub async fn from_conflicts(rows: Vec<Arc<Conflict>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for c in &rows {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, c)| ConflictEdge { cursor: edge_cursor(i), node: c }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct AlternativeEdge {
        pub cursor: String,
        pub node: Arc<Alternative>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct AlternativeConnection {
        pub edges: Vec<AlternativeEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl AlternativeConnection {
        pub async fn from_alternatives(rows: Vec<Arc<Alternative>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for a in &rows {
                child_hashes.push(a.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, a)| AlternativeEdge { cursor: edge_cursor(i), node: a }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct ChangeEdge {
        pub cursor: String,
        pub node: Arc<Change>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct ChangeConnection {
        pub edges: Vec<ChangeEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl ChangeConnection {
        pub async fn from_changes(rows: Vec<Arc<Change>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for c in &rows {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, c)| ChangeEdge { cursor: edge_cursor(i), node: c }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct EditEdge {
        pub cursor: String,
        pub node: Arc<crate::vcs::Edit>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct EditConnection {
        pub edges: Vec<EditEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl EditConnection {
        pub async fn from_edits(rows: Vec<Arc<crate::vcs::Edit>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for e in &rows {
                child_hashes.push(e.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, e)| EditEdge { cursor: edge_cursor(i), node: e }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationEdge {
        pub cursor: String,
        pub node: Arc<crate::operation::OperationIface>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationConnection {
        pub edges: Vec<OperationEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl OperationConnection {
        pub fn from_iface_rows(rows: Vec<Arc<crate::operation::OperationIface>>) -> Self {
            let child_hashes: Vec<String> = rows.iter().map(|o| h(&[o.row_id().as_str()])).collect();
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, o)| OperationEdge { cursor: edge_cursor(i), node: o }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct CheckpointEdge {
        pub cursor: String,
        pub node: Arc<Checkpoint>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct CheckpointConnection {
        pub edges: Vec<CheckpointEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl CheckpointConnection {
        pub async fn from_checkpoints(rows: Vec<Arc<Checkpoint>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for c in &rows {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, c)| CheckpointEdge { cursor: edge_cursor(i), node: c }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    #[derive(Clone, SimpleObject)]
    #[graphql(name = "ConnectionEdge")]
    pub struct ConnectionEdge {
        pub cursor: String,
        pub node: Arc<crate::kit::design::connection::Connection>,
    }

    #[derive(Clone, SimpleObject)]
    #[graphql(name = "ConnectionConnection")]
    pub struct ConnectionConnection {
        pub edges: Vec<ConnectionEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl ConnectionConnection {
        pub async fn from_connections(rows: Vec<Arc<crate::kit::design::connection::Connection>>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for c in &rows {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, node)| ConnectionEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    crate::simple_conn_sync!(FileConnection, FileEdge, File, |f: &File| f.compute_entity_hash());
    crate::simple_conn_sync!(FolderConnection, FolderEdge, Folder, |f: &Folder| f.compute_entity_hash());
    crate::simple_conn_sync!(AuthorConnection, AuthorEdge, Author, |a: &Author| a.compute_entity_hash());
    crate::simple_conn_entity!(ConceptConnection, ConceptEdge, std::sync::Arc<Concept>);
    crate::simple_conn_entity!(TagConnection, TagEdge, std::sync::Arc<Tag>);
    crate::simple_conn_entity!(QualityConnection, QualityEdge, std::sync::Arc<Quality>);
    crate::simple_conn_entity!(PortConnection, PortEdge, std::sync::Arc<crate::kit::r#type::Port>);
    crate::simple_conn_entity!(PlaceConnection, PlaceEdge, std::sync::Arc<crate::geom::entity::PlaceNode>);
    crate::simple_conn_sync!(BenchmarkConnection, BenchmarkEdge, Benchmark, |b: &Benchmark| b.compute_entity_hash());
    crate::simple_conn_sync!(PropConnection, PropEdge, Prop, |p: &Prop| p.compute_entity_hash());
    crate::simple_conn_sync!(AttributeConnection, AttributeEdge, crate::meta::Attribute, |a: &crate::meta::Attribute| a.compute_entity_hash());
    crate::simple_conn_sync!(StatConnection, StatEdge, Stat, |s: &Stat| s.compute_entity_hash());
    crate::simple_conn_sync!(LayerConnection, LayerEdge, Layer, |l: &Layer| l.compute_entity_hash());
    crate::simple_conn_sync!(GroupConnection, GroupEdge, Group, |g: &Group| g.compute_entity_hash());

    crate::entity_full_family!(Vector, Arc<crate::geom::entity::VectorNode>, relay = (VectorConnection, VectorEdge));
    crate::entity_full_family!(Point, Arc<crate::geom::entity::PointNode>, relay = (PointConnection, PointEdge));
    crate::entity_full_family!(Coordinate, Arc<crate::geom::entity::CoordinateNode>, relay = (CoordinateConnection, CoordinateEdge));
    crate::entity_full_family!(Offset, Arc<crate::geom::entity::OffsetNode>, relay = (OffsetConnection, OffsetEdge));
    crate::entity_full_family!(Plane, Arc<crate::geom::entity::PlaneNode>, relay = (PlaneConnection, PlaneEdge));
    crate::entity_full_family!(Position, Arc<crate::geom::entity::PositionNode>, relay = (PositionConnection, PositionEdge));
    crate::entity_full_family!(Location, Arc<crate::geom::entity::LocationNode>, relay = (LocationConnection, LocationEdge));

    /// @emoji 🧷 Kit [`Family`] SDL shell — Artifact [`name`]/[`description`]/[`icon`] are persisted kit fields.
    #[derive(Clone, Debug, Default, SimpleObject)]
    #[graphql(complex)]
    pub struct Family {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
    }

    impl Family {
        /// @emoji 🪪 Stable digest for relay [`FamilyConnection`] (matches GraphQL `Family.hash`).
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Family", self.id.as_str(), self.name.as_str(), self.description.as_deref().unwrap_or(""), self.icon.as_deref().unwrap_or("")], Vec::new())
        }
    }

    #[async_graphql::ComplexObject]
    impl Family {
        /// @emoji 🪪 Merkle leaf over the family row (Artifact data fields).
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct FamilyEdge {
        pub cursor: String,
        pub node: Family,
    }

    #[derive(Clone, SimpleObject)]
    pub struct FamilyConnection {
        pub edges: Vec<FamilyEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl FamilyConnection {
        pub fn from_rows(rows: Vec<Family>) -> Self {
            let mut child_hashes = Vec::with_capacity(rows.len());
            for f in &rows {
                child_hashes.push(f.compute_entity_hash());
            }
            let hash = merkle_collection(child_hashes);
            let edges = rows.into_iter().enumerate().map(|(i, node)| FamilyEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }
}

//#endregion 🪢 gql_relay

//#region 🏷️ meta

pub mod meta {
    //! 🏷️ Metadata: DTO [`SimpleObject`] shells plus Arc-backed [`Tag`]/[`Concept`]/[`Quality`] entities (SDL `Entity`).
    use std::sync::{Arc, Weak};

    use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
    use async_lock::RwLock;
    use serde::{Deserialize, Serialize};

    use crate::id::Id;
    use crate::timestamp::Timestamp;

    //#region 🧾 graphql inputs
    /// @emoji 🧾 SDL `AttributeInput` — instantiates [`Attribute`] rows on entity create/update paths.
    #[derive(Clone, Debug, Default, Serialize, Deserialize, InputObject)]
    pub struct AttributeInput {
        pub key: String,
        pub value: Option<String>,
        pub definition: Option<String>,
    }

    /// @emoji 🧾 SDL `TagInput`.
    #[derive(Clone, Debug, Default, Serialize, Deserialize, InputObject)]
    pub struct TagInput {
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub order: Option<i32>,
        pub attributes: Option<Vec<AttributeInput>>,
    }

    /// @emoji 🧾 SDL `ConceptInput`.
    #[derive(Clone, Debug, Default, Serialize, Deserialize, InputObject)]
    pub struct ConceptInput {
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub order: Option<i32>,
        pub attributes: Option<Vec<AttributeInput>>,
    }

    /// @emoji 🧾 SDL `QualityInput` (subset aligned to persisted kit fields).
    #[derive(Clone, Debug, Default, Serialize, Deserialize, InputObject)]
    pub struct QualityInput {
        pub key: String,
        pub value: Option<String>,
        pub unit: Option<String>,
        pub definition: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub attributes: Option<Vec<AttributeInput>>,
    }
    //#endregion 🧾 graphql inputs

    impl AttributeInput {
        /// @emoji ➕ Mint a persisted [`Attribute`] from GraphQL input (fresh [`Id`]).
        pub async fn into_attribute(self) -> Attribute {
            Attribute { id: Id::new().await, key: self.key, value: self.value.unwrap_or_default(), definition: self.definition }
        }

        /// @emoji 🪪 Rebuild a persisted [`Attribute`] using a caller-supplied id from a normalized operation scope.
        pub fn into_attribute_with_id(self, id: Id) -> Attribute {
            Attribute { id, key: self.key, value: self.value.unwrap_or_default(), definition: self.definition }
        }
    }

    /// @emoji ➕ Expand optional GraphQL attribute rows into minted [`Attribute`] entities.
    pub async fn attributes_from_inputs(inp: Option<Vec<AttributeInput>>) -> Vec<Attribute> {
        let mut v = Vec::new();
        for a in inp.into_iter().flatten() {
            v.push(a.into_attribute().await);
        }
        v
    }

    /// @emoji 🪪 Rebuild optional GraphQL attribute rows using the ids already recorded in operation scope.
    pub fn attributes_from_inputs_with_ids(inp: Option<Vec<AttributeInput>>, ids: &[Id]) -> Result<Vec<Attribute>, crate::error::SemioError> {
        let attrs = inp.unwrap_or_default();
        if attrs.len() != ids.len() {
            return Err(crate::error::SemioError::invalid(format!("attribute id count mismatch: expected {}, got {}", attrs.len(), ids.len())));
        }
        Ok(attrs.into_iter().zip(ids.iter().cloned()).map(|(attr, id)| attr.into_attribute_with_id(id)).collect())
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct File {
        pub id: Id,
        pub url: String,
        pub mime: Option<String>,
        pub size: Option<i32>,
        /// @emoji 📎 Blob/content digest on the wire (`hash` in JSON); omitted from GraphQL in favor of entity [`File::hash`] resolver.
        #[graphql(skip)]
        pub hash: String,
        pub description: Option<String>,
        pub created: Option<Timestamp>,
        pub updated: Option<Timestamp>,
    }

    impl File {
        /// @emoji 🌿 Blake3 leaf over every persisted [`File`] column (blob digest in [`File::hash`] JSON field).
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(
                &[
                    "semio:meta:File",
                    self.id.as_str(),
                    self.url.as_str(),
                    self.mime.as_deref().unwrap_or(""),
                    &self.size.map(|sz| sz.to_string()).unwrap_or_default(),
                    self.hash.as_str(),
                    self.description.as_deref().unwrap_or(""),
                    self.created.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                    self.updated.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                ],
                Vec::new(),
            )
        }
    }

    #[ComplexObject]
    impl File {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Folder {
        pub id: Id,
        pub path: String,
        pub description: Option<String>,
    }

    impl Folder {
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Folder", self.id.as_str(), self.path.as_str(), self.description.as_deref().unwrap_or("")], Vec::new())
        }
    }

    #[ComplexObject]
    impl Folder {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Author {
        pub id: Id,
        pub name: String,
        pub email: String,
        pub role: Option<String>,
        pub rank: Option<i32>,
    }

    impl Author {
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Author", self.id.as_str(), self.name.as_str(), self.email.as_str(), self.role.as_deref().unwrap_or(""), &self.rank.map(|r| r.to_string()).unwrap_or_default()], Vec::new())
        }
    }

    #[ComplexObject]
    impl Author {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Attribute {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub definition: Option<String>,
    }

    impl Attribute {
        /// @emoji 🌿 Blake3 leaf over persisted attribute columns (no owner weak refs).
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Attribute", self.id.as_str(), self.key.as_str(), self.value.as_str(), self.definition.as_deref().unwrap_or("")], Vec::new())
        }
    }

    /// 🏷️ Hand union for `Attribute.owner` (subset of carriers Attribute can hang off of).
    #[derive(Clone, async_graphql::Union)]
    pub enum AttributeOwner {
        Piece(std::sync::Arc<crate::kit::design::piece::Piece>),
        Connector(std::sync::Arc<crate::kit::r#type::Connector>),
        Representation(std::sync::Arc<crate::kit::r#type::Representation>),
        Connection(std::sync::Arc<crate::kit::design::connection::Connection>),
        Kit(std::sync::Arc<crate::kit::Kit>),
        Design(std::sync::Arc<crate::kit::design::Design>),
        Type(std::sync::Arc<crate::kit::r#type::Type>),
        Concept(std::sync::Arc<crate::meta::Concept>),
        Tag(std::sync::Arc<crate::meta::Tag>),
    }

    #[Object(name = "Attribute")]
    impl Attribute {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
        pub async fn owner(&self) -> AttributeOwner {
            AttributeOwner::Kit(std::sync::Arc::new(crate::kit::Kit::default()))
        }
        pub async fn key(&self) -> String {
            self.key.clone()
        }
        pub async fn value(&self) -> Option<String> {
            Some(self.value.clone())
        }
        pub async fn definition(&self) -> Option<String> {
            self.definition.clone()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Benchmark {
        pub id: Id,
        pub name: String,
        pub min: Option<f64>,
        pub max: Option<f64>,
        #[graphql(name = "minExcluded")]
        pub min_excluded: Option<bool>,
        #[graphql(name = "maxExcluded")]
        pub max_excluded: Option<bool>,
    }

    impl Benchmark {
        pub fn compute_entity_hash(&self) -> String {
            let min = self.min.map(|v| format!("{v:.9}")).unwrap_or_default();
            let max = self.max.map(|v| format!("{v:.9}")).unwrap_or_default();
            let minx = self.min_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let maxx = self.max_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(&["semio:meta:Benchmark", self.id.as_str(), self.name.as_str(), min.as_str(), max.as_str(), minx, maxx], Vec::new())
        }
    }

    #[ComplexObject]
    impl Benchmark {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Prop {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        #[graphql(skip)]
        #[serde(skip)]
        pub quality: Option<std::sync::Arc<Quality>>,
    }

    impl Prop {
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Prop", self.id.as_str(), self.key.as_str(), self.value.as_str(), self.unit.as_deref().unwrap_or("")], Vec::new())
        }
    }

    #[ComplexObject]
    impl Prop {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    /// @emoji 🪢 Resolved kit/type/representation owner for a [`Tag`] (write path sets exactly one arm).
    #[derive(Debug)]
    pub enum TagOwnerSlot {
        Unset,
        Kit(Weak<crate::kit::Kit>),
        Type(Weak<crate::kit::r#type::Type>),
        Rep(Weak<crate::kit::r#type::Representation>),
    }

    impl Default for TagOwnerSlot {
        fn default() -> Self {
            Self::Unset
        }
    }

    /// @emoji 🏷️ SDL `Tag` — Arc-shared entity with interior mutability (GraphQL `#[Object]` in `meta_objects` region).
    #[derive(Debug)]
    pub struct Tag {
        pub id: Id,
        pub owner: RwLock<TagOwnerSlot>,
        pub name: RwLock<String>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub order: RwLock<Option<i32>>,
        pub attributes: RwLock<Vec<Attribute>>,
    }

    impl Tag {
        pub async fn new(owner: TagOwnerSlot, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>, attributes: Vec<Attribute>) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner: RwLock::new(owner), name: RwLock::new(name), description: RwLock::new(description), icon: RwLock::new(icon), order: RwLock::new(order), attributes: RwLock::new(attributes) })
        }

        pub fn new_with_id(owner: TagOwnerSlot, id: Id, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>, attributes: Vec<Attribute>) -> Arc<Self> {
            Arc::new(Self { id, owner: RwLock::new(owner), name: RwLock::new(name), description: RwLock::new(description), icon: RwLock::new(icon), order: RwLock::new(order), attributes: RwLock::new(attributes) })
        }

        pub async fn compute_hash(&self) -> String {
            let n = self.name.read().await;
            let d = self.description.read().await.clone().unwrap_or_default();
            let ic = self.icon.read().await.clone().unwrap_or_default();
            let ord = self.order.read().await.map(|o| o.to_string()).unwrap_or_default();
            let attrs = self.attributes.read().await;
            let mut child_hashes: Vec<String> = attrs.iter().map(Attribute::compute_entity_hash).collect();
            child_hashes.sort();
            crate::hash::merkle_node_str(&["semio:meta:Tag", self.id.as_str(), n.as_str(), d.as_str(), ic.as_str(), ord.as_str()], child_hashes)
        }
    }

    impl Default for Tag {
        fn default() -> Self {
            Self { id: Id::default(), owner: RwLock::new(TagOwnerSlot::default()), name: RwLock::new(String::new()), description: RwLock::new(None), icon: RwLock::new(None), order: RwLock::new(None), attributes: RwLock::new(Vec::new()) }
        }
    }

    /// @emoji 🪢 Resolved kit/type owner for a [`Concept`].
    #[derive(Debug)]
    pub enum ConceptOwnerSlot {
        Unset,
        Kit(Weak<crate::kit::Kit>),
        Type(Weak<crate::kit::r#type::Type>),
    }

    impl Default for ConceptOwnerSlot {
        fn default() -> Self {
            Self::Unset
        }
    }

    /// @emoji 🏷️ SDL `Concept` entity.
    #[derive(Debug)]
    pub struct Concept {
        pub id: Id,
        pub owner: RwLock<ConceptOwnerSlot>,
        pub name: RwLock<String>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub order: RwLock<Option<i32>>,
        pub attributes: RwLock<Vec<Attribute>>,
    }

    impl Concept {
        pub async fn new(owner: ConceptOwnerSlot, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>, attributes: Vec<Attribute>) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner: RwLock::new(owner), name: RwLock::new(name), description: RwLock::new(description), icon: RwLock::new(icon), order: RwLock::new(order), attributes: RwLock::new(attributes) })
        }

        pub fn new_with_id(owner: ConceptOwnerSlot, id: Id, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>, attributes: Vec<Attribute>) -> Arc<Self> {
            Arc::new(Self { id, owner: RwLock::new(owner), name: RwLock::new(name), description: RwLock::new(description), icon: RwLock::new(icon), order: RwLock::new(order), attributes: RwLock::new(attributes) })
        }

        pub async fn compute_hash(&self) -> String {
            let n = self.name.read().await;
            let d = self.description.read().await.clone().unwrap_or_default();
            let ic = self.icon.read().await.clone().unwrap_or_default();
            let ord = self.order.read().await.map(|o| o.to_string()).unwrap_or_default();
            let attrs = self.attributes.read().await;
            let mut child_hashes: Vec<String> = attrs.iter().map(Attribute::compute_entity_hash).collect();
            child_hashes.sort();
            crate::hash::merkle_node_str(&["semio:meta:Concept", self.id.as_str(), n.as_str(), d.as_str(), ic.as_str(), ord.as_str()], child_hashes)
        }
    }

    impl Default for Concept {
        fn default() -> Self {
            Self { id: Id::default(), owner: RwLock::new(ConceptOwnerSlot::default()), name: RwLock::new(String::new()), description: RwLock::new(None), icon: RwLock::new(None), order: RwLock::new(None), attributes: RwLock::new(Vec::new()) }
        }
    }

    /// @emoji 🪢 Resolved multi-parent owner for [`Quality`] (connector/representation/type/design/kit).
    #[derive(Debug)]
    pub enum QualityOwnerSlot {
        Unset,
        Kit(Weak<crate::kit::Kit>),
        Type(Weak<crate::kit::r#type::Type>),
        Rep(Weak<crate::kit::r#type::Representation>),
        Conn(Weak<crate::kit::r#type::Connector>),
        Design(Weak<crate::kit::design::Design>),
    }

    impl Default for QualityOwnerSlot {
        fn default() -> Self {
            Self::Unset
        }
    }

    /// @emoji 🏷️ SDL `Quality` entity (benchmarks stay value-typed [`Benchmark`] rows).
    #[derive(Debug)]
    pub struct Quality {
        pub id: Id,
        pub owner: RwLock<QualityOwnerSlot>,
        pub key: RwLock<String>,
        pub value: RwLock<Option<String>>,
        pub unit: RwLock<Option<String>>,
        pub definition: RwLock<Option<String>>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub benchmarks: RwLock<Vec<Benchmark>>,
        pub attributes: RwLock<Vec<Attribute>>,
    }

    impl Quality {
        pub async fn new(
            owner: QualityOwnerSlot,
            key: String,
            value: Option<String>,
            unit: Option<String>,
            definition: Option<String>,
            description: Option<String>,
            icon: Option<String>,
            benchmarks: Vec<Benchmark>,
            attributes: Vec<Attribute>,
        ) -> Arc<Self> {
            Arc::new(Self {
                id: Id::new().await,
                owner: RwLock::new(owner),
                key: RwLock::new(key),
                value: RwLock::new(value),
                unit: RwLock::new(unit),
                definition: RwLock::new(definition),
                description: RwLock::new(description),
                icon: RwLock::new(icon),
                benchmarks: RwLock::new(benchmarks),
                attributes: RwLock::new(attributes),
            })
        }

        pub fn new_with_id(
            owner: QualityOwnerSlot,
            id: Id,
            key: String,
            value: Option<String>,
            unit: Option<String>,
            definition: Option<String>,
            description: Option<String>,
            icon: Option<String>,
            benchmarks: Vec<Benchmark>,
            attributes: Vec<Attribute>,
        ) -> Arc<Self> {
            Arc::new(Self {
                id,
                owner: RwLock::new(owner),
                key: RwLock::new(key),
                value: RwLock::new(value),
                unit: RwLock::new(unit),
                definition: RwLock::new(definition),
                description: RwLock::new(description),
                icon: RwLock::new(icon),
                benchmarks: RwLock::new(benchmarks),
                attributes: RwLock::new(attributes),
            })
        }

        pub async fn compute_hash(&self) -> String {
            let k = self.key.read().await;
            let v = self.value.read().await.clone().unwrap_or_default();
            let u = self.unit.read().await.clone().unwrap_or_default();
            let def = self.definition.read().await.clone().unwrap_or_default();
            let desc = self.description.read().await.clone().unwrap_or_default();
            let ic = self.icon.read().await.clone().unwrap_or_default();
            let bm = self.benchmarks.read().await;
            let av = self.attributes.read().await;
            let mut child_hashes: Vec<String> = bm.iter().map(Benchmark::compute_entity_hash).collect();
            child_hashes.extend(av.iter().map(Attribute::compute_entity_hash));
            child_hashes.sort();
            crate::hash::merkle_node_str(&["semio:meta:Quality", self.id.as_str(), k.as_str(), v.as_str(), u.as_str(), def.as_str(), desc.as_str(), ic.as_str()], child_hashes)
        }
    }

    impl Default for Quality {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner: RwLock::new(QualityOwnerSlot::default()),
                key: RwLock::new(String::new()),
                value: RwLock::new(None),
                unit: RwLock::new(None),
                definition: RwLock::new(None),
                description: RwLock::new(None),
                icon: RwLock::new(None),
                benchmarks: RwLock::new(Vec::new()),
                attributes: RwLock::new(Vec::new()),
            }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Stat {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        pub description: Option<String>,
    }

    impl Stat {
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["semio:meta:Stat", self.id.as_str(), self.key.as_str(), self.value.as_str(), self.unit.as_deref().unwrap_or(""), self.description.as_deref().unwrap_or("")], Vec::new())
        }
    }

    #[ComplexObject]
    impl Stat {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Layer {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        /// @emoji 🏷️ Artifact `icon` on SDL `Layer`.
        pub icon: String,
        pub color: Option<String>,
        pub order: Option<i32>,
        pub visible: Option<bool>,
        pub locked: Option<bool>,
    }

    impl Layer {
        pub fn compute_entity_hash(&self) -> String {
            let vis = self.visible.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let lck = self.locked.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(
                &["semio:meta:Layer", self.id.as_str(), self.name.as_str(), self.description.as_deref().unwrap_or(""), self.icon.as_str(), self.color.as_deref().unwrap_or(""), &self.order.map(|o| o.to_string()).unwrap_or_default(), vis, lck],
                Vec::new(),
            )
        }
    }

    #[ComplexObject]
    impl Layer {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    #[graphql(complex)]
    pub struct Group {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub color: Option<String>,
        pub icon: Option<String>,
        #[graphql(skip)]
        pub piece_ids: Vec<Id>,
    }

    impl Group {
        pub fn compute_entity_hash(&self) -> String {
            let mut ids: Vec<String> = self.piece_ids.iter().map(|i| i.as_str().to_string()).collect();
            ids.sort();
            let joined = ids.join("\x1e");
            crate::hash::merkle_node_str(&["semio:meta:Group", self.id.as_str(), self.name.as_str(), self.description.as_deref().unwrap_or(""), self.color.as_deref().unwrap_or(""), self.icon.as_deref().unwrap_or(""), joined.as_str()], Vec::new())
        }
    }

    #[ComplexObject]
    impl Group {
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
    }
}

//#endregion 🏷️ meta

//#region 🪪 hash

pub mod hash {
    //! 🪪 Blake3 Merkle helpers: [`h`] for delimiter-joined parts; [`merkle_node_str`] for ordered own fields plus sorted child digests; [`merkle_collection`] for relay connection hashes.
    use blake3::Hasher;

    pub fn h<S: AsRef<[u8]>>(parts: &[S]) -> String {
        let mut hasher = Hasher::new();
        for p in parts {
            hasher.update(p.as_ref());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
    }

    /// @emoji 🌳 Merkle fold: concatenates `own` in order, then **sorted** `children` hex digests (order-independent set hashing).
    pub fn merkle_node_str(own: &[&str], mut children: Vec<String>) -> String {
        children.sort();
        let mut hasher = Hasher::new();
        for s in own {
            hasher.update(s.as_bytes());
            hasher.update(b"\x1f");
        }
        for c in &children {
            hasher.update(c.as_bytes());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
    }

    /// @emoji 🪢 Relay collection hash: sorted child entity hashes under a stable collection tag.
    pub fn merkle_collection(children: Vec<String>) -> String {
        merkle_node_str(&["semio:relay:collection"], children)
    }
}

//#endregion 🪪 hash

//#region 📦 kit

pub mod kit {
    //! 📦 Kit ↔ Type ↔ Design entity tree (Arc + interior RwLock per mutable field).

    //#region 🏠 type
    pub mod r#type {
        //! 🏠 Types, their connectors and representations.
        use std::sync::{Arc, Weak};

        use async_graphql::{Object, Union};
        use async_lock::RwLock;

        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Concept, File, Prop, Quality, Stat, Tag};
        use crate::timestamp::Timestamp;

        //#region 🛟 port
        /// 🔌 Kit-level named attachment point; referenced by [`Connector`] and [`super::connection::Side`].
        #[derive(Debug)]
        pub struct Port {
            pub id: Id,
            pub owner_type: Weak<Type>,
            pub code: RwLock<Option<String>>,
            pub label: RwLock<Option<String>>,
            pub order: RwLock<Option<i32>>,
        }

        impl Default for Port {
            fn default() -> Self {
                Self { id: Id::default(), owner_type: Weak::new(), code: RwLock::new(None), label: RwLock::new(None), order: RwLock::new(None) }
            }
        }

        impl Port {
            pub async fn new(owner_type: Weak<Type>) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_type, code: RwLock::new(None), label: RwLock::new(None), order: RwLock::new(None) })
            }

            pub async fn compute_hash(&self) -> String {
                let code = self.code.read().await.clone().unwrap_or_default();
                let label = self.label.read().await.clone().unwrap_or_default();
                let ord = self.order.read().await.map(|i| i.to_string()).unwrap_or_default();
                h(&[self.id.as_str(), code.as_str(), label.as_str(), ord.as_str()])
            }
        }

        #[Object(name = "Port")]
        impl Port {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            pub async fn code(&self) -> Option<String> {
                self.code.read().await.clone()
            }
            pub async fn label(&self) -> Option<String> {
                self.label.read().await.clone()
            }
            pub async fn order(&self) -> Option<i32> {
                *self.order.read().await
            }
        }
        //#endregion 🛟 port

        //#region ⚓ connector
        pub struct Connector {
            pub id: Id,
            pub owner_type: Weak<Type>,
            /// @emoji 🏷️ SDL `Connector.name` (Artifact).
            pub name: RwLock<String>,
            pub code: RwLock<String>,
            pub description: RwLock<String>,
            /// @emoji 🏷️ SDL `Connector.icon` (Artifact).
            pub icon: RwLock<String>,
            /// @emoji 🔗 Resolved port pointer (`# data` on the wire).
            pub port: RwLock<Option<Arc<Port>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Connector {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_type: Weak::new(),
                    name: RwLock::new(String::new()),
                    code: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    port: RwLock::new(None),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                }
            }
        }

        impl Connector {
            pub async fn new(owner_type: Weak<Type>, code: String) -> Arc<Self> {
                Arc::new(Self {
                    id: Id::new().await,
                    owner_type,
                    name: RwLock::new(String::new()),
                    code: RwLock::new(code),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    port: RwLock::new(None),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                let code = self.code.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                h(&[self.id.as_str(), name.as_str(), code.as_str(), desc.as_str(), icon.as_str()])
            }
        }

        #[Object(name = "Connector")]
        impl Connector {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn code(&self) -> String {
                self.code.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn port(&self) -> Option<Arc<Port>> {
                self.port.read().await.clone()
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
            }
        }
        //#endregion ⚓ connector

        //#region 💾 representation
        pub struct Representation {
            pub id: Id,
            pub owner_type: Weak<Type>,
            /// @emoji 🏷️ SDL `Representation.name` (Artifact).
            pub name: RwLock<String>,
            pub url: RwLock<String>,
            pub description: RwLock<String>,
            /// @emoji 🏷️ SDL `Representation.icon` (Artifact).
            pub icon: RwLock<String>,
            pub file: RwLock<Option<File>>,
            pub tags: RwLock<Vec<Arc<Tag>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Representation {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_type: Weak::new(),
                    name: RwLock::new(String::new()),
                    url: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    file: RwLock::new(None),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                }
            }
        }

        impl Representation {
            pub async fn new(owner_type: Weak<Type>, url: String) -> Arc<Self> {
                Arc::new(Self {
                    id: Id::new().await,
                    owner_type,
                    name: RwLock::new(String::new()),
                    url: RwLock::new(url),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    file: RwLock::new(None),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let url = self.url.read().await;
                let name = self.name.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                h(&[self.id.as_str(), name.as_str(), url.as_str(), desc.as_str(), icon.as_str()])
            }
        }

        #[Object(name = "Representation")]
        impl Representation {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn url(&self) -> String {
                self.url.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn file(&self) -> Option<File> {
                self.file.read().await.clone()
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
            }
        }
        //#endregion 💾 representation

        //#region 🏠 type
        pub struct Type {
            pub id: Id,
            pub owner_kit: Weak<crate::kit::Kit>,
            pub name: RwLock<String>,
            pub description: RwLock<String>,
            pub icon: RwLock<String>,
            pub image: RwLock<String>,
            pub unit: RwLock<String>,
            pub created: RwLock<Option<Timestamp>>,
            pub updated: RwLock<Option<Timestamp>>,
            pub connectors: RwLock<Vec<Arc<Connector>>>,
            pub ports: RwLock<Vec<Arc<Port>>>,
            pub representations: RwLock<Vec<Arc<Representation>>>,
            /// 🧷 Refreshed from `connectors` before single-id GraphQL lookups (no stale `Id` scans in resolvers).
            pub connector_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Connector>>>,
            pub port_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Port>>>,
            pub representation_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Representation>>>,
            pub authors: RwLock<Vec<Author>>,
            pub concepts: RwLock<Vec<Arc<Concept>>>,
            pub tags: RwLock<Vec<Arc<Tag>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub props: RwLock<Vec<Prop>>,
            pub attributes: RwLock<Vec<Attribute>>,
            pub stats: RwLock<Vec<Stat>>,
        }

        impl Default for Type {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_kit: Weak::new(),
                    name: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    image: RwLock::new(String::new()),
                    unit: RwLock::new(String::new()),
                    created: RwLock::new(None),
                    updated: RwLock::new(None),
                    connectors: RwLock::new(Vec::new()),
                    ports: RwLock::new(Vec::new()),
                    representations: RwLock::new(Vec::new()),
                    connector_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    port_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    representation_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    authors: RwLock::new(Vec::new()),
                    concepts: RwLock::new(Vec::new()),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    props: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                    stats: RwLock::new(Vec::new()),
                }
            }
        }

        impl Type {
            pub async fn new(owner_kit: Weak<crate::kit::Kit>, name: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_kit, name: RwLock::new(name), ..Default::default() })
            }

            /// 🧾 Insert a workspace kind with caller-controlled external [`Id`] (wasm / JSON snapshot hydration).
            pub async fn new_with_external_id(owner_kit: Weak<crate::kit::Kit>, id: Id, name: String) -> Arc<Self> {
                Arc::new(Self { id, owner_kit, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                let image = self.image.read().await;
                let unit = self.unit.read().await;
                h(&[self.id.as_str(), name.as_str(), desc.as_str(), icon.as_str(), image.as_str(), unit.as_str()])
            }

            /// 🧷 Rebuild weak maps from the live vecs (call before `connector` / `representation` field resolution).
            pub async fn refresh_connector_child_weak_maps(&self) {
                {
                    let v = self.connectors.read().await;
                    let mut m = self.connector_weak_by_id.write().await;
                    m.clear();
                    for c in v.iter() {
                        m.insert(c.id.clone(), Arc::downgrade(c));
                    }
                }
                {
                    let v = self.ports.read().await;
                    let mut m = self.port_weak_by_id.write().await;
                    m.clear();
                    for p in v.iter() {
                        m.insert(p.id.clone(), Arc::downgrade(p));
                    }
                }
                {
                    let v = self.representations.read().await;
                    let mut m = self.representation_weak_by_id.write().await;
                    m.clear();
                    for r in v.iter() {
                        m.insert(r.id.clone(), Arc::downgrade(r));
                    }
                }
            }
            pub async fn best_representation_for_tags(&self, tag_ids: &[Id]) -> Option<Arc<Representation>> {
                use std::collections::HashSet;
                let want: HashSet<&Id> = tag_ids.iter().collect();
                let reps = self.representations.read().await;
                let mut best: Option<(usize, Arc<Representation>)> = None;
                for r in reps.iter() {
                    let tags = r.tags.read().await;
                    let score = tags.iter().filter(|t| want.contains(&t.id)).count();
                    if best.as_ref().is_none_or(|(s, _)| score > *s) {
                        best = Some((score, r.clone()));
                    }
                }
                best.map(|(_, r)| r)
            }
        }

        #[Object(name = "Type")]
        impl Type {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Arc<crate::kit::Kit> {
                self.owner_kit.upgrade().unwrap_or_default()
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn image(&self) -> String {
                self.image.read().await.clone()
            }
            pub async fn unit(&self) -> String {
                self.unit.read().await.clone()
            }
            pub async fn created(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            pub async fn updated(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            pub async fn connectors(&self) -> crate::gql_relay::ConnectorConnection {
                crate::gql_relay::ConnectorConnection::from_connectors(self.connectors.read().await.clone()).await
            }
            pub async fn connector(&self, id: Id) -> Option<Arc<Connector>> {
                self.refresh_connector_child_weak_maps().await;
                self.connector_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            pub async fn representations(&self) -> crate::gql_relay::RepresentationConnection {
                crate::gql_relay::RepresentationConnection::from_representations(self.representations.read().await.clone()).await
            }
            pub async fn representation(&self, id: Id) -> Option<Arc<Representation>> {
                self.refresh_connector_child_weak_maps().await;
                self.representation_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            #[graphql(name = "bestRepresentation")]
            pub async fn best_representation(&self, tag_ids: Vec<Id>) -> Option<Arc<Representation>> {
                self.best_representation_for_tags(&tag_ids).await
            }
            pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
            }
            pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
                crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone()).await
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_rows(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_rows(self.stats.read().await.clone())
            }
        }
        //#endregion 🏠 type

        //#region 🧩 blueprint
        /// 🧩 Blueprint union (Type | Design) — what a Piece references and what a Piece's owner is.
        #[derive(Clone, Union)]
        #[graphql(name = "Blueprint")]
        pub enum Blueprint {
            Type(Arc<Type>),
            Design(Arc<super::design::Design>),
        }

        impl Default for Blueprint {
            fn default() -> Self {
                Blueprint::Type(Arc::default())
            }
        }
        //#endregion 🧩 blueprint
    }
    //#endregion 🏠 type

    //#region 🏘 design
    pub mod design {
        //! 🏘 Designs and their pieces, connections, layers, groups.

        //#region ⭕ piece
        pub mod piece {
            //! ⭕ Piece (instance of a Type or Design within a Design).
            use std::sync::{Arc, Weak};

            use async_graphql::{Enum, Object};
            use async_lock::RwLock;

            use crate::geom::entity::PositionNode;
            use crate::geom::Position;
            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::{Attribute, Prop};

            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Enum)]
            #[graphql(name = "PieceConnectionKind")]
            pub enum PieceConnectionKind {
                #[graphql(name = "FIXED")]
                #[default]
                Fixed,
                #[graphql(name = "CONNECTED")]
                Connected,
            }

            pub struct Piece {
                pub id: Id,
                pub owner_design: Weak<super::Design>,
                pub name: RwLock<Option<String>>,
                pub description: RwLock<Option<String>>,
                pub position: RwLock<Option<Arc<PositionNode>>>,
                pub scale: RwLock<Option<f64>>,
                pub blueprint: RwLock<super::super::r#type::Blueprint>,
                pub connection_kind: RwLock<Option<PieceConnectionKind>>,
                pub parent_piece: RwLock<Weak<Piece>>,
                pub parent_connection: RwLock<Weak<super::connection::Connection>>,
                pub child_pieces: RwLock<Vec<Arc<Piece>>>,
                pub child_connections: RwLock<Vec<Arc<super::connection::Connection>>>,
                pub depth: RwLock<i32>,
                pub path: RwLock<Vec<Weak<Piece>>>,
                pub props: RwLock<Vec<Prop>>,
                pub attributes: RwLock<Vec<Attribute>>,
            }

            impl Default for Piece {
                fn default() -> Self {
                    Self {
                        id: Id::default(),
                        owner_design: Weak::new(),
                        name: RwLock::new(None),
                        description: RwLock::new(None),
                        position: RwLock::new(None),
                        scale: RwLock::new(None),
                        blueprint: RwLock::new(super::super::r#type::Blueprint::default()),
                        connection_kind: RwLock::new(None),
                        parent_piece: RwLock::new(Weak::new()),
                        parent_connection: RwLock::new(Weak::new()),
                        child_pieces: RwLock::new(Vec::new()),
                        child_connections: RwLock::new(Vec::new()),
                        depth: RwLock::new(0),
                        path: RwLock::new(Vec::new()),
                        props: RwLock::new(Vec::new()),
                        attributes: RwLock::new(Vec::new()),
                    }
                }
            }

            impl Piece {
                pub async fn new_fixed(owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: Position) -> Arc<Self> {
                    let pos_node = PositionNode::from_position_value(position);
                    Arc::new(Self { id: Id::new().await, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                /// 🧾 Hydrated workspace piece aligned to external JSON id (facade snapshot hydration).
                pub async fn new_fixed_with_external_id(id: Id, owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: Position) -> Arc<Self> {
                    let pos_node = PositionNode::from_position_value(position);
                    Arc::new(Self { id, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                pub async fn set_name(&self, name: Option<String>) {
                    *self.name.write().await = name;
                }
                pub async fn set_description(&self, description: Option<String>) {
                    *self.description.write().await = description;
                }
                pub async fn set_position(&self, position: Option<Position>) {
                    let mut g = self.position.write().await;
                    *g = position.map(PositionNode::from_position_value);
                }

                pub async fn compute_hash(&self) -> String {
                    let name = self.name.read().await;
                    h(&[self.id.as_str(), name.as_deref().unwrap_or("")])
                }

                pub async fn compute_flat_position(&self) -> Position {
                    if let Some(n) = self.position.read().await.as_ref() {
                        return n.snapshot_value().await;
                    }
                    Position::default()
                }
            }

            #[Object(name = "Piece")]
            impl Piece {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> super::super::r#type::Blueprint {
                    super::super::r#type::Blueprint::Design(self.owner_design.upgrade().unwrap_or_default())
                }
                pub async fn name(&self) -> Option<String> {
                    self.name.read().await.clone()
                }
                pub async fn description(&self) -> Option<String> {
                    self.description.read().await.clone()
                }
                pub async fn position(&self) -> Option<Arc<PositionNode>> {
                    self.position.read().await.clone()
                }
                pub async fn scale(&self) -> Option<f64> {
                    *self.scale.read().await
                }
                pub async fn blueprint(&self) -> super::super::r#type::Blueprint {
                    self.blueprint.read().await.clone()
                }
                #[graphql(name = "connectionKind")]
                pub async fn connection_kind(&self) -> Option<PieceConnectionKind> {
                    *self.connection_kind.read().await
                }
                #[graphql(name = "flatPosition")]
                pub async fn flat_position(&self) -> Arc<PositionNode> {
                    if let Some(n) = self.position.read().await.clone() {
                        return n;
                    }
                    PositionNode::from_position_value(Position::default())
                }
                #[graphql(name = "replaceableBlueprints")]
                pub async fn replaceable_blueprints(&self) -> crate::gql_relay::BlueprintConnection {
                    crate::gql_relay::BlueprintConnection::from_blueprints(Vec::new()).await
                }
                #[graphql(name = "parentConnection")]
                pub async fn parent_connection(&self) -> Option<Arc<super::connection::Connection>> {
                    self.parent_connection.read().await.upgrade()
                }
                #[graphql(name = "childConnections")]
                pub async fn child_connections(&self) -> Vec<Arc<super::connection::Connection>> {
                    self.child_connections.read().await.clone()
                }
                #[graphql(name = "parentPiece")]
                pub async fn parent_piece(&self) -> Option<Arc<Piece>> {
                    self.parent_piece.read().await.upgrade()
                }
                #[graphql(name = "childPieces")]
                pub async fn child_pieces(&self) -> Vec<Arc<Piece>> {
                    self.child_pieces.read().await.clone()
                }
                pub async fn depth(&self) -> i32 {
                    *self.depth.read().await
                }
                pub async fn path(&self) -> Vec<Arc<Piece>> {
                    self.path.read().await.iter().filter_map(|w| w.upgrade()).collect()
                }
                pub async fn props(&self) -> Vec<Prop> {
                    self.props.read().await.clone()
                }
                pub async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
                }
            }
        }
        //#endregion ⭕ piece

        //#region 🔗 connection
        pub mod connection {
            //! 🔗 Connection between two piece sides + the Side value.
            use std::sync::{Arc, Weak};

            use async_graphql::{Object, Union};
            use async_lock::RwLock;

            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::Attribute;

            //#region ⛓️ side
            pub struct Side {
                pub id: Id,
                /// @emoji 🔗 Owning connection when sides are wired into a [`Connection`].
                pub owner_connection: RwLock<Weak<Connection>>,
                pub piece: RwLock<Arc<super::piece::Piece>>,
                pub port: RwLock<Option<Arc<super::super::r#type::Port>>>,
                pub design_piece: RwLock<Option<Arc<super::piece::Piece>>>,
                pub connector: RwLock<Option<Arc<super::super::r#type::Connector>>>,
            }

            impl Default for Side {
                fn default() -> Self {
                    Self { id: Id::default(), owner_connection: RwLock::new(Weak::new()), piece: RwLock::new(Arc::new(super::piece::Piece::default())), port: RwLock::new(None), design_piece: RwLock::new(None), connector: RwLock::new(None) }
                }
            }

            impl Side {
                pub async fn new(piece: Arc<super::piece::Piece>) -> Arc<Self> {
                    Arc::new(Self { id: Id::new().await, piece: RwLock::new(piece), ..Default::default() })
                }

                pub async fn compute_hash(&self) -> String {
                    h(&[self.id.as_str()])
                }
            }

            /// @emoji 🔗 SDL `union SideOwner = Connection`.
            #[derive(Clone, Union)]
            #[graphql(name = "SideOwner")]
            pub enum SideOwner {
                Connection(Arc<Connection>),
            }

            #[Object(name = "Side")]
            impl Side {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> SideOwner {
                    SideOwner::Connection(self.owner_connection.read().await.upgrade().unwrap_or_default())
                }
                pub async fn piece(&self) -> Arc<super::piece::Piece> {
                    self.piece.read().await.clone()
                }
                pub async fn port(&self) -> Option<Arc<super::super::r#type::Port>> {
                    self.port.read().await.clone()
                }
                #[graphql(name = "designPiece")]
                pub async fn design_piece(&self) -> Option<Arc<super::piece::Piece>> {
                    self.design_piece.read().await.clone()
                }
                pub async fn connector(&self) -> Option<Arc<super::super::r#type::Connector>> {
                    self.connector.read().await.clone()
                }
            }
            //#endregion ⛓️ side

            //#region 🔗 connection
            pub struct Connection {
                pub id: Id,
                pub owner_design: Weak<super::Design>,
                /// @emoji 🏷️ SDL Artifact `name` / `description` / `icon`.
                pub name: RwLock<String>,
                pub description: RwLock<String>,
                pub icon: RwLock<String>,
                pub connected: RwLock<Arc<Side>>,
                pub connecting: RwLock<Arc<Side>>,
                pub gap: RwLock<Option<f64>>,
                pub shift: RwLock<Option<f64>>,
                pub rise: RwLock<Option<f64>>,
                pub rotation: RwLock<Option<f64>>,
                pub turn: RwLock<Option<f64>>,
                pub tilt: RwLock<Option<f64>>,
                pub u: RwLock<Option<f64>>,
                pub v: RwLock<Option<f64>>,
                pub attributes: RwLock<Vec<Attribute>>,
            }

            impl Default for Connection {
                fn default() -> Self {
                    Self {
                        id: Id::default(),
                        owner_design: Weak::new(),
                        name: RwLock::new(String::new()),
                        description: RwLock::new(String::new()),
                        icon: RwLock::new(String::new()),
                        connected: RwLock::new(Arc::new(Side::default())),
                        connecting: RwLock::new(Arc::new(Side::default())),
                        gap: RwLock::new(None),
                        shift: RwLock::new(None),
                        rise: RwLock::new(None),
                        rotation: RwLock::new(None),
                        turn: RwLock::new(None),
                        tilt: RwLock::new(None),
                        u: RwLock::new(None),
                        v: RwLock::new(None),
                        attributes: RwLock::new(Vec::new()),
                    }
                }
            }

            impl Connection {
                pub async fn compute_hash(&self) -> String {
                    let connected = self.connected.read().await;
                    let connecting = self.connecting.read().await;
                    let cp = connected.piece.read().await.id.0.clone();
                    let np = connecting.piece.read().await.id.0.clone();
                    h(&[self.id.as_str(), cp.as_str(), np.as_str()])
                }
            }

            #[Object(name = "Connection")]
            impl Connection {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> Arc<super::Design> {
                    self.owner_design.upgrade().unwrap_or_default()
                }
                pub async fn name(&self) -> String {
                    self.name.read().await.clone()
                }
                pub async fn description(&self) -> String {
                    self.description.read().await.clone()
                }
                pub async fn icon(&self) -> String {
                    self.icon.read().await.clone()
                }
                pub async fn connected(&self) -> Arc<Side> {
                    self.connected.read().await.clone()
                }
                pub async fn connecting(&self) -> Arc<Side> {
                    self.connecting.read().await.clone()
                }
                pub async fn gap(&self) -> Option<f64> {
                    *self.gap.read().await
                }
                pub async fn shift(&self) -> Option<f64> {
                    *self.shift.read().await
                }
                pub async fn rise(&self) -> Option<f64> {
                    *self.rise.read().await
                }
                pub async fn rotation(&self) -> Option<f64> {
                    *self.rotation.read().await
                }
                pub async fn turn(&self) -> Option<f64> {
                    *self.turn.read().await
                }
                pub async fn tilt(&self) -> Option<f64> {
                    *self.tilt.read().await
                }
                pub async fn u(&self) -> Option<f64> {
                    *self.u.read().await
                }
                pub async fn v(&self) -> Option<f64> {
                    *self.v.read().await
                }
                pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                    crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
                }
            }
            //#endregion 🔗 connection
        }
        //#endregion 🔗 connection

        //#region 🏘 design
        use std::collections::HashMap;
        use std::sync::{Arc, Weak};

        use async_graphql::{Object, Union};
        use async_lock::RwLock;

        use crate::geom::entity::LocationNode;
        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Group, Layer, Prop, Quality, Stat};
        use crate::timestamp::Timestamp;

        //#region 🧱 clump
        /// @emoji 🧱 SDL `Clump` — connected-component bucket for layout (`WeakEntity` projection hook).
        pub struct Clump {
            pub id: Id,
            pub owner_design: Weak<Design>,
        }

        impl Default for Clump {
            fn default() -> Self {
                Self { id: Id::default(), owner_design: Weak::new() }
            }
        }

        #[Object(name = "Clump")]
        impl Clump {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                h(&[self.id.as_str()])
            }
            pub async fn owner(&self) -> Arc<Design> {
                self.owner_design.upgrade().unwrap_or_default()
            }
            #[graphql(name = "fixedPiece")]
            pub async fn fixed_piece(&self) -> Option<Arc<piece::Piece>> {
                None
            }
            #[graphql(name = "connectedPieces")]
            pub async fn connected_pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
            pub async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
        }
        //#endregion 🧱 clump

        /// @emoji 🏠 SDL `union DesignOwner = Kit`.
        #[derive(Clone, Union)]
        #[graphql(name = "DesignOwner")]
        pub enum DesignOwner {
            Kit(Arc<super::Kit>),
        }

        pub struct Design {
            pub id: Id,
            pub owner_kit: Weak<crate::kit::Kit>,
            pub name: RwLock<String>,
            pub description: RwLock<Option<String>>,
            pub icon: RwLock<Option<String>>,
            pub image: RwLock<Option<String>>,
            pub location: RwLock<Option<Arc<LocationNode>>>,
            pub unit: RwLock<Option<String>>,
            pub created: RwLock<Option<Timestamp>>,
            pub updated: RwLock<Option<Timestamp>>,
            pub pieces: RwLock<Vec<Arc<piece::Piece>>>,
            /// 🧷 Write-side only: external piece [`Id`] → `Weak` (GraphQL `piece(id:)` upgrades here; no vec index table).
            pub piece_weak_by_external_id: RwLock<HashMap<Id, Weak<piece::Piece>>>,
            pub connections: RwLock<Vec<Arc<connection::Connection>>>,
            pub layers: RwLock<Vec<Layer>>,
            pub groups: RwLock<Vec<Group>>,
            pub authors: RwLock<Vec<Author>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub props: RwLock<Vec<Prop>>,
            pub attributes: RwLock<Vec<Attribute>>,
            pub stats: RwLock<Vec<Stat>>,
        }

        impl Default for Design {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_kit: Weak::new(),
                    name: RwLock::new(String::new()),
                    description: RwLock::new(None),
                    icon: RwLock::new(None),
                    image: RwLock::new(None),
                    location: RwLock::new(None),
                    unit: RwLock::new(None),
                    created: RwLock::new(None),
                    updated: RwLock::new(None),
                    pieces: RwLock::new(Vec::new()),
                    piece_weak_by_external_id: RwLock::new(HashMap::new()),
                    connections: RwLock::new(Vec::new()),
                    layers: RwLock::new(Vec::new()),
                    groups: RwLock::new(Vec::new()),
                    authors: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    props: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                    stats: RwLock::new(Vec::new()),
                }
            }
        }

        impl Design {
            pub async fn new(owner_kit: Weak<crate::kit::Kit>, name: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_kit, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn with_id(owner_kit: Weak<crate::kit::Kit>, id: Id, name: String) -> Arc<Self> {
                Arc::new(Self { id, owner_kit, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                h(&[self.id.as_str(), name.as_str()])
            }

            /// 🆕 Push a piece into this design's pieces; returns the same Arc (refcount + 1) for the caller.
            pub async fn insert_piece(&self, piece: Arc<piece::Piece>) -> Arc<piece::Piece> {
                let mut pieces = self.pieces.write().await;
                let mut weak_ix = self.piece_weak_by_external_id.write().await;
                let pid = piece.id.clone();
                weak_ix.insert(pid, Arc::downgrade(&piece));
                pieces.push(piece.clone());
                piece
            }

            /// @emoji 🗑 Remove a piece from this design's ordered list and external-id index.
            pub async fn delete_piece_by_external_id(&self, piece_id: &Id) -> Result<(), crate::error::SemioError> {
                let mut pieces = self.pieces.write().await;
                let start_len = pieces.len();
                pieces.retain(|piece| &piece.id != piece_id);
                if pieces.len() == start_len {
                    return Err(crate::error::SemioError::not_found("Piece", piece_id.as_str()));
                }
                self.piece_weak_by_external_id.write().await.remove(piece_id);
                Ok(())
            }

            /// @emoji 🪢 Command / GraphQL boundary: resolve a piece by external [`Id`] via the write-side weak map.
            pub async fn piece_by_external_id(&self, id: &Id) -> Option<Arc<piece::Piece>> {
                self.piece_weak_by_external_id.read().await.get(id).and_then(|w| w.upgrade())
            }

            pub async fn hydrate_pieces_from_snapshot_json(des: &Arc<Self>, kit: &Arc<crate::kit::Kit>, d_json: &serde_json::Value) -> Result<(), crate::error::SemioError> {
                {
                    let mut pcs = des.pieces.write().await;
                    pcs.clear();
                }
                *des.piece_weak_by_external_id.write().await = HashMap::new();
                let plist = d_json.get("pieces").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
                let owner_des = Arc::downgrade(des);
                for pj in plist {
                    let pid = pj.get("id").and_then(|x| x.as_str()).ok_or_else(|| crate::error::SemioError::invalid("design piece missing id"))?;
                    let type_id_raw = match pj.get("type") {
                        Some(serde_json::Value::String(s)) => s.as_str(),
                        Some(serde_json::Value::Object(map)) => map.get("id").and_then(|x| x.as_str()).ok_or_else(|| crate::error::SemioError::invalid("design piece type object missing id"))?,
                        _ => {
                            return Err(crate::error::SemioError::invalid("design piece missing type (string id or { id })"));
                        }
                    };
                    let ty = kit.types.read().await.iter().find(|t| t.id.as_str() == type_id_raw).cloned().ok_or_else(|| crate::error::SemioError::not_found("Type", type_id_raw))?;
                    let pose = pj.get("pose");
                    let plane_val = pj.get("plane").cloned().or_else(|| pose.and_then(|p| p.get("plane")).cloned()).unwrap_or_else(|| serde_json::json!({}));
                    let center_val = pj.get("center").cloned().or_else(|| pose.and_then(|p| p.get("center")).cloned()).unwrap_or_else(|| serde_json::json!({"u":0.0,"v":0.0}));
                    let position: crate::geom::Position = serde_json::from_value(serde_json::json!({"plane": plane_val, "center": center_val})).map_err(|e| crate::error::SemioError::invalid(format!("piece position serde: {}", e)))?;
                    let scale = pj.get("scale").and_then(|s| s.as_f64()).unwrap_or(1.0);
                    let nm_opt = pj.get("name").and_then(|x| x.as_str());
                    let bp = crate::kit::r#type::Blueprint::Type(ty.clone());
                    let piece = piece::Piece::new_fixed_with_external_id(pid.into(), owner_des.clone(), bp, position).await;
                    if let Some(nm) = nm_opt {
                        piece.set_name(Some(nm.to_string())).await;
                    }
                    *piece.scale.write().await = Some(scale);
                    let _ = des.insert_piece(piece).await;
                }
                Ok(())
            }
        }

        #[Object(name = "Design")]
        impl Design {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> DesignOwner {
                DesignOwner::Kit(self.owner_kit.upgrade().unwrap_or_default())
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> Option<String> {
                self.icon.read().await.clone()
            }
            pub async fn image(&self) -> Option<String> {
                self.image.read().await.clone()
            }
            pub async fn location(&self) -> Option<Arc<LocationNode>> {
                self.location.read().await.clone()
            }
            pub async fn unit(&self) -> Option<String> {
                self.unit.read().await.clone()
            }
            #[graphql(name = "createdAt")]
            pub async fn created_at(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            #[graphql(name = "updatedAt")]
            pub async fn updated_at(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            pub async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.pieces.read().await.clone()).await
            }
            pub async fn piece(&self, id: Id) -> Option<Arc<piece::Piece>> {
                self.piece_by_external_id(&id).await
            }
            pub async fn connections(&self) -> crate::gql_relay::ConnectionConnection {
                crate::gql_relay::ConnectionConnection::from_connections(self.connections.read().await.clone()).await
            }
            pub async fn connection(&self, id: Id) -> Option<Arc<connection::Connection>> {
                self.connections.read().await.iter().find(|c| c.id == id).cloned()
            }
            pub async fn layers(&self) -> crate::gql_relay::LayerConnection {
                crate::gql_relay::LayerConnection::from_rows(self.layers.read().await.clone())
            }
            pub async fn groups(&self) -> crate::gql_relay::GroupConnection {
                crate::gql_relay::GroupConnection::from_rows(self.groups.read().await.clone())
            }
            pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_rows(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_rows(self.stats.read().await.clone())
            }
            #[graphql(name = "qualitySum")]
            pub async fn quality_sum(&self, _quality_id: Id) -> f64 {
                0.0
            }

            pub async fn references(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(Vec::new()).await
            }
            #[graphql(name = "referencedBy")]
            pub async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
        }
        //#endregion 🏘 design
    }
    //#endregion 🏘 design

    //#region 📚 kit_target_operations
    /// 🧾 Arc-backed operation `*Input` shells for Quality / Tag / Concept / Port (`target.schema.graphql` nested `#region Operations`).
    pub mod target_operations {
        use std::sync::Arc;

        use async_graphql::SimpleObject;

        use crate::gql_relay::{AttributeConnection, ConceptConnection, PortConnection, QualityConnection, TagConnection};
        use crate::kit::r#type::Port;
        use crate::meta::{Attribute, Concept, Quality, Tag};

        //#region 🔖 Quality inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedQualityInput")]
        pub struct CreatedQualityInput {
            pub quality: Arc<Quality>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedQualitiesInput")]
        pub struct CreatedQualitiesInput {
            pub qualities: QualityConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedQualityInput")]
        pub struct RenamedQualityInput {
            pub key: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedQualityDescriptionInput")]
        pub struct UpdatedQualityDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedQualityIconInput")]
        pub struct UpdatedQualityIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToQualityInput")]
        pub struct AddedAttributeToQualityInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToQualityInput")]
        pub struct AddedAttributesToQualityInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromQualityInput")]
        pub struct RemovedAttributeFromQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromQualityInput")]
        pub struct RemovedAttributesFromQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedQualityInput")]
        pub struct DeletedQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedQualitiesInput")]
        pub struct DeletedQualitiesInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🔖 Quality inputs

        //#region 🏷️ Tag inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedTagInput")]
        pub struct CreatedTagInput {
            pub tag: Arc<Tag>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedTagsInput")]
        pub struct CreatedTagsInput {
            pub tags: TagConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedTagInput")]
        pub struct RenamedTagInput {
            pub name: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedTagDescriptionInput")]
        pub struct UpdatedTagDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedTagIconInput")]
        pub struct UpdatedTagIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToTagInput")]
        pub struct AddedAttributeToTagInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToTagInput")]
        pub struct AddedAttributesToTagInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromTagInput")]
        pub struct RemovedAttributeFromTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromTagInput")]
        pub struct RemovedAttributesFromTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedTagInput")]
        pub struct DeletedTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedTagsInput")]
        pub struct DeletedTagsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🏷️ Tag inputs

        //#region 💡 Concept inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedConceptInput")]
        pub struct CreatedConceptInput {
            pub concept: Arc<Concept>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedConceptsInput")]
        pub struct CreatedConceptsInput {
            pub concepts: ConceptConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedConceptInput")]
        pub struct RenamedConceptInput {
            pub name: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedConceptDescriptionInput")]
        pub struct UpdatedConceptDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedConceptIconInput")]
        pub struct UpdatedConceptIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToConceptInput")]
        pub struct AddedAttributeToConceptInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToConceptInput")]
        pub struct AddedAttributesToConceptInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromConceptInput")]
        pub struct RemovedAttributeFromConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromConceptInput")]
        pub struct RemovedAttributesFromConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedConceptInput")]
        pub struct DeletedConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedConceptsInput")]
        pub struct DeletedConceptsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 💡 Concept inputs

        //#region 🔌 Port inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedPortInput")]
        pub struct CreatedPortInput {
            pub port: Arc<Port>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedPortsInput")]
        pub struct CreatedPortsInput {
            pub ports: PortConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedPortInput")]
        pub struct RenamedPortInput {
            pub code: String,
            pub label: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedPortDescriptionInput")]
        pub struct UpdatedPortDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedPortIconInput")]
        pub struct UpdatedPortIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToPortInput")]
        pub struct AddedAttributeToPortInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToPortInput")]
        pub struct AddedAttributesToPortInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromPortInput")]
        pub struct RemovedAttributeFromPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromPortInput")]
        pub struct RemovedAttributesFromPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedPortInput")]
        pub struct DeletedPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedPortsInput")]
        pub struct DeletedPortsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🔌 Port inputs
    }
    //#endregion 📚 kit_target_operations

    //#region 📦 kit
    use std::collections::HashMap;
    use std::sync::{Arc, Weak};

    use async_graphql::{Object, Union};
    use async_lock::RwLock;

    use crate::hash::h;
    use crate::id::Id;
    use crate::meta::{Attribute, Author, Concept, File, Folder, Prop, Quality, Stat, Tag};
    use crate::timestamp::Timestamp;

    /// @emoji 🔗 SDL `union KitOwner = Graph | Checkpoint | Alternative`.
    #[derive(Clone, Union)]
    #[graphql(name = "KitOwner")]
    pub enum KitOwner {
        Graph(Arc<crate::vcs::Graph>),
        Checkpoint(Arc<crate::vcs::Checkpoint>),
        Alternative(Arc<crate::vcs::Alternative>),
    }

    pub struct Kit {
        pub id: Id,
        pub owner_graph: Weak<crate::vcs::Graph>,
        pub name: RwLock<String>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub image: RwLock<Option<String>>,
        pub preview: RwLock<Option<String>>,
        pub remote: RwLock<Option<String>>,
        pub homepage: RwLock<Option<String>>,
        pub license: RwLock<Option<String>>,
        pub uri: RwLock<Option<String>>,
        pub created: RwLock<Option<Timestamp>>,
        pub updated: RwLock<Option<Timestamp>>,
        pub version: RwLock<Option<String>>,
        pub designs: RwLock<Vec<Arc<design::Design>>>,
        /// 🧷 Write-side only: external design [`Id`] → `Weak` (GraphQL `design(id:)` upgrades here).
        pub design_weak_by_id: RwLock<HashMap<Id, Weak<design::Design>>>,
        pub types: RwLock<Vec<Arc<r#type::Type>>>,
        /// 🧷 Write-side: type [`Id`] → `Weak` for GraphQL `type(id:)` (filled on snapshot hydration).
        pub type_weak_by_id: RwLock<HashMap<Id, Weak<r#type::Type>>>,
        pub files: RwLock<Vec<File>>,
        pub folders: RwLock<Vec<Folder>>,
        pub authors: RwLock<Vec<Author>>,
        pub concepts: RwLock<Vec<Arc<Concept>>>,
        pub tags: RwLock<Vec<Arc<Tag>>>,
        pub qualities: RwLock<Vec<Arc<Quality>>>,
        pub props: RwLock<Vec<Prop>>,
        pub attributes: RwLock<Vec<Attribute>>,
        pub stats: RwLock<Vec<Stat>>,
        /// 🧷 Kit-wide tag identity map (all tag owners).
        pub tag_by_id: RwLock<HashMap<Id, Arc<Tag>>>,
        pub concept_by_id: RwLock<HashMap<Id, Arc<Concept>>>,
        pub quality_by_id: RwLock<HashMap<Id, Arc<Quality>>>,
        /// @emoji 🔢 Monotonic counter bumped by every GraphQL graph mutation (test / backbone observability).
        pub touch_epoch: RwLock<u64>,
        /// 🧭 Optional client-facing kit id from WASM/JSON hydration (`@semio/js` DTO `id`); when None, fall back to internally minted [`Kit::id`].
        pub snapshot_external_kit_id: RwLock<Option<Id>>,
    }

    impl Default for Kit {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_graph: Weak::new(),
                name: RwLock::new(String::new()),
                description: RwLock::new(None),
                icon: RwLock::new(None),
                image: RwLock::new(None),
                preview: RwLock::new(None),
                remote: RwLock::new(None),
                homepage: RwLock::new(None),
                license: RwLock::new(None),
                uri: RwLock::new(None),
                created: RwLock::new(None),
                updated: RwLock::new(None),
                version: RwLock::new(None),
                designs: RwLock::new(Vec::new()),
                design_weak_by_id: RwLock::new(HashMap::new()),
                types: RwLock::new(Vec::new()),
                type_weak_by_id: RwLock::new(HashMap::new()),
                files: RwLock::new(Vec::new()),
                folders: RwLock::new(Vec::new()),
                authors: RwLock::new(Vec::new()),
                concepts: RwLock::new(Vec::new()),
                tags: RwLock::new(Vec::new()),
                qualities: RwLock::new(Vec::new()),
                props: RwLock::new(Vec::new()),
                attributes: RwLock::new(Vec::new()),
                stats: RwLock::new(Vec::new()),
                tag_by_id: RwLock::new(HashMap::new()),
                concept_by_id: RwLock::new(HashMap::new()),
                quality_by_id: RwLock::new(HashMap::new()),
                touch_epoch: RwLock::new(0),
                snapshot_external_kit_id: RwLock::new(None),
            }
        }
    }

    impl Kit {
        pub async fn new(owner_graph: Weak<crate::vcs::Graph>, name: String) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_graph, name: RwLock::new(name), ..Default::default() })
        }

        pub(crate) fn new_sync(owner_graph: Weak<crate::vcs::Graph>, name: String) -> Arc<Self> {
            Arc::new(Self { id: Id::new_sync(), owner_graph, name: RwLock::new(name), ..Default::default() })
        }

        pub async fn workspace_kit_id(&self) -> Id {
            self.snapshot_external_kit_id.read().await.clone().unwrap_or_else(|| self.id.clone())
        }

        pub async fn bump_touch_epoch(&self) {
            let mut g = self.touch_epoch.write().await;
            *g = g.saturating_add(1);
        }

        /// @emoji 🧬 Deep-clone this kit graph (snapshot JSON round-trip) for immutable graph `initialKit` baselines / replay scratch kits.
        pub async fn deep_clone(self: &Arc<Self>) -> Arc<Kit> {
            let snap = self.kit_full_snapshot_value().await;
            let owner = self.owner_graph.clone();
            let nm = self.name.read().await.clone();
            let row = Kit::new_sync(owner, nm);
            let _ = row.hydrate_from_kit_full_snapshot_json(&snap).await;
            row
        }

        /// @emoji 📦 Single mutation entry: walks canonical [`crate::operation::CanonicalKitDiff`] from [`crate::operation::KitOperation::to_diff`].
        pub async fn apply_diff(self: &Arc<Self>, diff: &crate::operation::KitDiff) -> Result<(), crate::error::SemioError> {
            let d = &diff.0;
            if let Some(s) = &d.name {
                *self.name.write().await = s.clone();
            }
            if let Some(s) = &d.version {
                *self.version.write().await = Some(s.clone());
            }
            if let Some(s) = &d.description {
                *self.description.write().await = Some(s.clone());
            }
            if let Some(s) = &d.icon {
                *self.icon.write().await = Some(s.clone());
            }
            if let Some(s) = &d.image {
                *self.image.write().await = Some(s.clone());
            }
            if let Some(s) = &d.preview {
                *self.preview.write().await = Some(s.clone());
            }
            if let Some(s) = &d.remote {
                *self.remote.write().await = Some(s.clone());
            }
            if let Some(s) = &d.homepage {
                *self.homepage.write().await = Some(s.clone());
            }
            if let Some(s) = &d.license {
                *self.license.write().await = Some(s.clone());
            }
            if let Some(v) = &d.types {
                self.apply_types_diff_json(v).await?;
            }
            if let Some(v) = &d.designs {
                self.apply_designs_diff_json(v).await?;
            }
            if let Some(t) = &d.tags {
                self.apply_tags_collection_diff(t).await?;
            }
            if let Some(c) = &d.concepts {
                self.apply_concepts_collection_diff(c).await?;
            }
            if let Some(q) = &d.qualities {
                self.apply_qualities_collection_diff(q).await?;
            }
            if let Some(v) = &d.files {
                if Self::json_diff_non_trivial(v) {
                    return Err(crate::error::SemioError::invalid("kit diff `files` subtree apply not implemented"));
                }
            }
            if let Some(v) = &d.folders {
                if Self::json_diff_non_trivial(v) {
                    return Err(crate::error::SemioError::invalid("kit diff `folders` subtree apply not implemented"));
                }
            }
            if let Some(v) = &d.authors {
                if Self::json_diff_non_trivial(v) {
                    return Err(crate::error::SemioError::invalid("kit diff `authors` subtree apply not implemented"));
                }
            }
            self.bump_touch_epoch().await;
            Ok(())
        }

        fn json_diff_non_trivial(v: &serde_json::Value) -> bool {
            match v {
                serde_json::Value::Null => false,
                serde_json::Value::Object(m) if m.is_empty() => false,
                serde_json::Value::Object(m) => m.iter().any(|(_, x)| match x {
                    serde_json::Value::Array(a) => !a.is_empty(),
                    serde_json::Value::Object(o) => !o.is_empty(),
                    serde_json::Value::Null => false,
                    _ => true,
                }),
                serde_json::Value::Array(a) => !a.is_empty(),
                _ => true,
            }
        }

        async fn apply_types_diff_json(self: &Arc<Self>, v: &serde_json::Value) -> Result<(), crate::error::SemioError> {
            let Some(obj) = v.as_object() else {
                return Ok(());
            };
            if let Some(serde_json::Value::Array(removed)) = obj.get("removed") {
                for row in removed {
                    let id: Id = serde_json::from_value(row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("type removed.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                    let mut tys = self.types.write().await;
                    tys.retain(|t| t.id != id);
                    drop(tys);
                    self.type_weak_by_id.write().await.remove(&id);
                }
            }
            if let Some(serde_json::Value::Array(modified)) = obj.get("modified") {
                for row in modified {
                    let tid: Id = serde_json::from_value(row.get("type").and_then(|t| t.get("id")).cloned().ok_or_else(|| crate::error::SemioError::invalid("type modified.type.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                    let Some(ty) = self.type_by_external_id(&tid).await else {
                        continue;
                    };
                    let diff = row.get("diff").cloned().unwrap_or(serde_json::json!({}));
                    if let Some(s) = diff.get("name").and_then(|x| x.as_str()) {
                        *ty.name.write().await = s.to_string();
                    }
                    if diff.get("description").is_some() {
                        *ty.description.write().await = diff.get("description").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    }
                    if diff.get("icon").is_some() {
                        *ty.icon.write().await = diff.get("icon").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    }
                    if diff.get("image").is_some() {
                        *ty.image.write().await = diff.get("image").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    }
                    if diff.get("unit").is_some() {
                        *ty.unit.write().await = diff.get("unit").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    }
                }
            }
            if obj.get("added").and_then(|x| x.as_array()).map(|a| !a.is_empty()).unwrap_or(false) {
                return Err(crate::error::SemioError::invalid("kit diff `types.added` apply not implemented"));
            }
            Ok(())
        }

        async fn apply_designs_diff_json(self: &Arc<Self>, v: &serde_json::Value) -> Result<(), crate::error::SemioError> {
            let Some(obj) = v.as_object() else {
                return Ok(());
            };
            if let Some(serde_json::Value::Array(removed)) = obj.get("removed") {
                for row in removed {
                    let id: Id = serde_json::from_value(row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("design removed.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                    let mut ds = self.designs.write().await;
                    ds.retain(|d| d.id != id);
                    drop(ds);
                    self.design_weak_by_id.write().await.remove(&id);
                }
            }
            if let Some(serde_json::Value::Array(modified)) = obj.get("modified") {
                for row in modified {
                    let design_id: Id =
                        serde_json::from_value(row.get("design").and_then(|d| d.get("id")).cloned().ok_or_else(|| crate::error::SemioError::invalid("design modified.design.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                    let diff = row.get("diff").cloned().unwrap_or(serde_json::json!({}));
                    if let Some(design) = self.design_by_external_id(&design_id).await {
                        if let Some(s) = diff.get("name").and_then(|x| x.as_str()) {
                            *design.name.write().await = s.to_string();
                        }
                        if diff.get("description").is_some() {
                            *design.description.write().await = diff.get("description").and_then(|x| x.as_str()).map(|s| s.to_string());
                        }
                        if diff.get("icon").is_some() {
                            *design.icon.write().await = diff.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string());
                        }
                        if diff.get("image").is_some() {
                            *design.image.write().await = diff.get("image").and_then(|x| x.as_str()).map(|s| s.to_string());
                        }
                    }
                    if let Some(pr) = diff.get("pieces").and_then(|p| p.get("removed")).and_then(|x| x.as_array()) {
                        for pr_row in pr {
                            let piece_id: Id = serde_json::from_value(pr_row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("piece removed.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                            let design = self.design_by_external_id(&design_id).await.ok_or_else(|| crate::error::SemioError::not_found("Design", design_id.as_str()))?;
                            design.delete_piece_by_external_id(&piece_id).await?;
                        }
                    }
                    if let Some(pa) = diff.get("pieces").and_then(|p| p.get("added")).and_then(|x| x.as_array()) {
                        for piece_v in pa {
                            self.apply_design_piece_added_json(&design_id, piece_v).await?;
                        }
                    }
                    if let Some(pm) = diff.get("pieces").and_then(|p| p.get("modified")).and_then(|x| x.as_array()) {
                        for prow in pm {
                            let piece_id: Id = serde_json::from_value(prow.get("piece").and_then(|p| p.get("id")).cloned().ok_or_else(|| crate::error::SemioError::invalid("piece modified.piece.id"))?)
                                .map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                            let pdiff = prow.get("diff").cloned().unwrap_or(serde_json::json!({}));
                            self.apply_design_piece_modified_json(&design_id, &piece_id, &pdiff).await?;
                        }
                    }
                }
            }
            if let Some(serde_json::Value::Array(added)) = obj.get("added") {
                for _design_v in added {
                    return Err(crate::error::SemioError::invalid("kit diff `designs.added` apply not implemented"));
                }
            }
            Ok(())
        }

        async fn apply_design_piece_added_json(self: &Arc<Self>, design_id: &Id, piece_v: &serde_json::Value) -> Result<(), crate::error::SemioError> {
            let piece_id: Id = serde_json::from_value(piece_v.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("piece.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
            let blueprint_id: Id = piece_v.get("blueprintId").and_then(|x| x.as_str()).map(Id::from).unwrap_or_else(|| piece_id.clone());
            let position: crate::geom::Position = piece_v.get("pose").map(|p| serde_json::from_value(p.clone())).transpose().map_err(|e| crate::error::SemioError::invalid(e.to_string()))?.unwrap_or_default();
            let name = piece_v.get("name").and_then(|x| x.as_str()).map(|s| s.to_string());
            let description = piece_v.get("description").and_then(|x| x.as_str()).map(|s| s.to_string());
            let (_handle, design) = self.bind_external_design_id(design_id).await;
            let blueprint_type = crate::kit::r#type::Type::new(Arc::downgrade(self), format!("type-{}", blueprint_id.as_str())).await;
            let blueprint = crate::kit::r#type::Blueprint::Type(blueprint_type);
            let piece = crate::kit::design::piece::Piece::new_fixed_with_external_id(piece_id, Arc::downgrade(&design), blueprint, position).await;
            piece.set_name(name).await;
            piece.set_description(description).await;
            let _ = design.insert_piece(piece).await;
            Ok(())
        }

        async fn apply_design_piece_modified_json(self: &Arc<Self>, design_id: &Id, piece_id: &Id, pdiff: &serde_json::Value) -> Result<(), crate::error::SemioError> {
            use crate::geom::entity::PositionNode;
            let design = self.design_by_external_id(design_id).await.ok_or_else(|| crate::error::SemioError::not_found("Design", design_id.as_str()))?;
            let piece = design.piece_by_external_id(piece_id).await.ok_or_else(|| crate::error::SemioError::not_found("Piece", piece_id.as_str()))?;
            if let Some(true) = pdiff.get("fixPiece").and_then(|x| x.as_bool()) {
                *piece.connection_kind.write().await = Some(crate::kit::design::piece::PieceConnectionKind::Fixed);
                return Ok(());
            }
            if let Some(du) = pdiff.get("drag").and_then(|d| d.get("u")).and_then(|x| x.as_f64()) {
                let dv = pdiff.get("drag").and_then(|d| d.get("v")).and_then(|x| x.as_f64()).unwrap_or(0.0);
                let offset = crate::geom::Offset { u: du, v: dv };
                let pos_slot = piece.position.read().await.clone();
                if let Some(pos) = pos_slot {
                    let mut d = pos.data.write().await;
                    d.center.u += offset.u;
                    d.center.v += offset.v;
                    *pos.center.u.write().await = d.center.u;
                    *pos.center.v.write().await = d.center.v;
                } else {
                    let n = PositionNode::from_position_value(crate::geom::Position::default());
                    {
                        let mut d = n.data.write().await;
                        d.center.u += offset.u;
                        d.center.v += offset.v;
                    }
                    *n.center.u.write().await = n.data.read().await.center.u;
                    *n.center.v.write().await = n.data.read().await.center.v;
                    *piece.position.write().await = Some(n);
                }
                return Ok(());
            }
            if let Some(pose_v) = pdiff.get("pose") {
                let position: crate::geom::Position = serde_json::from_value(pose_v.clone()).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let n = PositionNode::from_position_value(position);
                *piece.position.write().await = Some(n);
                return Ok(());
            }
            if let Some(n) = pdiff.get("name").and_then(|x| x.as_str()) {
                piece.set_name(Some(n.to_string())).await;
            }
            if pdiff.get("description").is_some() {
                let s = pdiff.get("description").and_then(|x| x.as_str()).map(|s| s.to_string());
                piece.set_description(s).await;
            }
            Ok(())
        }

        async fn apply_tags_collection_diff(self: &Arc<Self>, t: &crate::operation::TagsCollectionDiff) -> Result<(), crate::error::SemioError> {
            for r in &t.removed {
                self.delete_tag_by_id(&r.id).await?;
            }
            for m in &t.modified {
                let tag = self.find_tag(&m.tag.id).await.ok_or_else(|| crate::error::SemioError::not_found("Tag", m.tag.id.as_str()))?;
                if let Some(s) = &m.diff.name {
                    *tag.name.write().await = s.clone();
                }
                if m.diff.description.is_some() {
                    *tag.description.write().await = m.diff.description.clone();
                }
                if m.diff.icon.is_some() {
                    *tag.icon.write().await = m.diff.icon.clone();
                }
            }
            for row in &t.added {
                let owner_id: Id = match row.get("ownerId").and_then(|x| x.as_str()).map(Id::from) {
                    Some(id) => id,
                    None => self.workspace_kit_id().await,
                };
                let tag_id: Id = serde_json::from_value(row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("tag added.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let attribute_ids: Vec<Id> = serde_json::from_value(row.get("attributeIds").cloned().unwrap_or(serde_json::json!([]))).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let tag_input: crate::meta::TagInput = serde_json::from_value(serde_json::json!({
                    "name": row.get("name"),
                    "description": row.get("description"),
                    "icon": row.get("icon"),
                    "order": row.get("order"),
                    "attributes": row.get("attributes"),
                }))
                .map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                self.apply_create_tag_scoped(&owner_id, &tag_id, &attribute_ids, tag_input).await?;
            }
            Ok(())
        }

        async fn apply_concepts_collection_diff(self: &Arc<Self>, c: &crate::operation::ConceptsCollectionDiff) -> Result<(), crate::error::SemioError> {
            for r in &c.removed {
                self.delete_concept_by_id(&r.id).await?;
            }
            for m in &c.modified {
                let concept = self.find_concept(&m.concept.id).await.ok_or_else(|| crate::error::SemioError::not_found("Concept", m.concept.id.as_str()))?;
                if let Some(s) = &m.diff.name {
                    *concept.name.write().await = s.clone();
                }
                if m.diff.description.is_some() {
                    *concept.description.write().await = m.diff.description.clone();
                }
                if m.diff.icon.is_some() {
                    *concept.icon.write().await = m.diff.icon.clone();
                }
            }
            for row in &c.added {
                let owner_id: Id = match row.get("ownerId").and_then(|x| x.as_str()).map(Id::from) {
                    Some(id) => id,
                    None => self.workspace_kit_id().await,
                };
                let concept_id: Id = serde_json::from_value(row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("concept added.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let attribute_ids: Vec<Id> = serde_json::from_value(row.get("attributeIds").cloned().unwrap_or(serde_json::json!([]))).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let input: crate::meta::ConceptInput = serde_json::from_value(serde_json::json!({
                    "name": row.get("name"),
                    "description": row.get("description"),
                    "icon": row.get("icon"),
                    "order": row.get("order"),
                    "attributes": row.get("attributes"),
                }))
                .map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                self.apply_create_concept_scoped(&owner_id, &concept_id, &attribute_ids, input).await?;
            }
            Ok(())
        }

        async fn apply_qualities_collection_diff(self: &Arc<Self>, q: &crate::operation::QualitiesCollectionDiff) -> Result<(), crate::error::SemioError> {
            for r in &q.removed {
                self.delete_quality_by_id(&r.id).await?;
            }
            for m in &q.modified {
                let quality = self.find_quality(&m.quality.id).await.ok_or_else(|| crate::error::SemioError::not_found("Quality", m.quality.id.as_str()))?;
                if m.diff.description.is_some() {
                    *quality.description.write().await = m.diff.description.clone();
                }
                if m.diff.icon.is_some() {
                    *quality.icon.write().await = m.diff.icon.clone();
                }
                if let Some(s) = &m.diff.key {
                    *quality.key.write().await = s.clone();
                }
                if m.diff.value.is_some() {
                    *quality.value.write().await = m.diff.value.clone();
                }
                if m.diff.unit.is_some() {
                    *quality.unit.write().await = m.diff.unit.clone();
                }
                if m.diff.definition.is_some() {
                    *quality.definition.write().await = m.diff.definition.clone();
                }
            }
            for row in &q.added {
                let owner_id: Id = match row.get("ownerId").and_then(|x| x.as_str()).map(Id::from) {
                    Some(id) => id,
                    None => self.workspace_kit_id().await,
                };
                let quality_id: Id = serde_json::from_value(row.get("id").cloned().ok_or_else(|| crate::error::SemioError::invalid("quality added.id"))?).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let attribute_ids: Vec<Id> = serde_json::from_value(row.get("attributeIds").cloned().unwrap_or(serde_json::json!([]))).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let benchmark_ids: Vec<Id> = serde_json::from_value(row.get("benchmarkIds").cloned().unwrap_or(serde_json::json!([]))).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                let input: crate::meta::QualityInput = serde_json::from_value(serde_json::json!({
                    "key": row.get("key").cloned().unwrap_or(serde_json::json!("")),
                    "value": row.get("value"),
                    "unit": row.get("unit"),
                    "definition": row.get("definition"),
                    "description": row.get("description"),
                    "icon": row.get("icon"),
                    "attributes": row.get("attributes"),
                }))
                .map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
                self.apply_create_quality_scoped(&owner_id, &quality_id, &attribute_ids, &benchmark_ids, input).await?;
            }
            Ok(())
        }

        /// @emoji 🪪 Inlined tag creation for [`Kit::apply_diff`] (no public mutator surface).
        async fn apply_create_tag_scoped(self: &Arc<Self>, owner_id: &Id, tag_id: &Id, attribute_ids: &[Id], input: crate::meta::TagInput) -> Result<(), crate::error::SemioError> {
            let slot = self.resolve_tag_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let tag = crate::meta::Tag::new_with_id(slot, tag_id.clone(), input.name, input.description, input.icon, input.order, attrs);
            self.register_tag(tag.clone()).await;
            match &*tag.owner.read().await {
                crate::meta::TagOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Unset => {}
            }
            Ok(())
        }

        async fn apply_create_concept_scoped(self: &Arc<Self>, owner_id: &Id, concept_id: &Id, attribute_ids: &[Id], input: crate::meta::ConceptInput) -> Result<(), crate::error::SemioError> {
            let slot = self.resolve_concept_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let c = crate::meta::Concept::new_with_id(slot, concept_id.clone(), input.name, input.description, input.icon, input.order, attrs);
            self.register_concept(c.clone()).await;
            match &*c.owner.read().await {
                crate::meta::ConceptOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.concepts.write().await.push(c.clone());
                    }
                }
                crate::meta::ConceptOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.concepts.write().await.push(c.clone());
                    }
                }
                crate::meta::ConceptOwnerSlot::Unset => {}
            }
            Ok(())
        }

        async fn apply_create_quality_scoped(self: &Arc<Self>, owner_id: &Id, quality_id: &Id, attribute_ids: &[Id], benchmark_ids: &[Id], input: crate::meta::QualityInput) -> Result<(), crate::error::SemioError> {
            let slot = self.resolve_quality_owner_slot(owner_id).await?;
            if !benchmark_ids.is_empty() {
                return Err(crate::error::SemioError::invalid("quality benchmark ids are not supported yet"));
            }
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let q = crate::meta::Quality::new_with_id(slot, quality_id.clone(), input.key, input.value, input.unit, input.definition, input.description, input.icon, Vec::new(), attrs);
            self.register_quality(q.clone()).await;
            match &*q.owner.read().await {
                crate::meta::QualityOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Conn(w) => {
                    if let Some(c) = w.upgrade() {
                        c.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Design(w) => {
                    if let Some(d) = w.upgrade() {
                        d.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Unset => {}
            }
            Ok(())
        }

        pub async fn compute_hash(&self) -> String {
            let name = self.name.read().await;
            let kid = self.workspace_kit_id().await;
            h(&[kid.as_str(), name.as_str()])
        }

        pub async fn design_by_external_id(&self, id: &Id) -> Option<Arc<design::Design>> {
            self.design_weak_by_id.read().await.get(id).and_then(|w| w.upgrade())
        }
        pub async fn type_by_external_id(&self, id: &Id) -> Option<Arc<r#type::Type>> {
            self.type_weak_by_id.read().await.get(id).and_then(|w| w.upgrade())
        }

        /// @emoji 🔎 Locate a representation by id across all kit types.
        pub async fn find_representation(&self, id: &Id) -> Option<Arc<r#type::Representation>> {
            let tys = self.types.read().await;
            for t in tys.iter() {
                let reps = t.representations.read().await;
                for r in reps.iter() {
                    if &r.id == id {
                        return Some(r.clone());
                    }
                }
            }
            None
        }

        /// @emoji 🔎 Locate a connector by id across all kit types.
        pub async fn find_connector(&self, id: &Id) -> Option<Arc<r#type::Connector>> {
            let tys = self.types.read().await;
            for t in tys.iter() {
                let conns = t.connectors.read().await;
                for c in conns.iter() {
                    if &c.id == id {
                        return Some(c.clone());
                    }
                }
            }
            None
        }

        pub async fn register_tag(self: &Arc<Self>, t: Arc<Tag>) {
            self.tag_by_id.write().await.insert(t.id.clone(), t);
        }
        pub async fn find_tag(&self, id: &Id) -> Option<Arc<Tag>> {
            self.tag_by_id.read().await.get(id).cloned()
        }
        pub async fn register_concept(self: &Arc<Self>, c: Arc<Concept>) {
            self.concept_by_id.write().await.insert(c.id.clone(), c);
        }
        pub async fn find_concept(&self, id: &Id) -> Option<Arc<Concept>> {
            self.concept_by_id.read().await.get(id).cloned()
        }
        pub async fn register_quality(self: &Arc<Self>, q: Arc<Quality>) {
            self.quality_by_id.write().await.insert(q.id.clone(), q);
        }
        pub async fn find_quality(&self, id: &Id) -> Option<Arc<Quality>> {
            self.quality_by_id.read().await.get(id).cloned()
        }

        /// @emoji 🪢 Resolve SDL `TagInput` owner (`Kit` root id, `Type` id, or `Representation` id).
        pub async fn resolve_tag_owner_slot(self: &Arc<Self>, owner_id: &Id) -> Result<crate::meta::TagOwnerSlot, crate::error::SemioError> {
            let kid = self.workspace_kit_id().await;
            if owner_id == &kid || owner_id == &self.id {
                return Ok(crate::meta::TagOwnerSlot::Kit(Arc::downgrade(self)));
            }
            if let Some(ty) = self.type_by_external_id(owner_id).await {
                return Ok(crate::meta::TagOwnerSlot::Type(Arc::downgrade(&ty)));
            }
            if let Some(rep) = self.find_representation(owner_id).await {
                return Ok(crate::meta::TagOwnerSlot::Rep(Arc::downgrade(&rep)));
            }
            Err(crate::error::SemioError::not_found("TagOwner", owner_id.as_str()))
        }

        /// @emoji 🪢 Resolve SDL `ConceptInput` owner (`Kit` or `Type`).
        pub async fn resolve_concept_owner_slot(self: &Arc<Self>, owner_id: &Id) -> Result<crate::meta::ConceptOwnerSlot, crate::error::SemioError> {
            let kid = self.workspace_kit_id().await;
            if owner_id == &kid || owner_id == &self.id {
                return Ok(crate::meta::ConceptOwnerSlot::Kit(Arc::downgrade(self)));
            }
            if let Some(ty) = self.type_by_external_id(owner_id).await {
                return Ok(crate::meta::ConceptOwnerSlot::Type(Arc::downgrade(&ty)));
            }
            Err(crate::error::SemioError::not_found("ConceptOwner", owner_id.as_str()))
        }

        /// @emoji 🪢 Resolve SDL `QualityInput` owner (kit/type/representation/connector/design).
        pub async fn resolve_quality_owner_slot(self: &Arc<Self>, owner_id: &Id) -> Result<crate::meta::QualityOwnerSlot, crate::error::SemioError> {
            let kid = self.workspace_kit_id().await;
            if owner_id == &kid || owner_id == &self.id {
                return Ok(crate::meta::QualityOwnerSlot::Kit(Arc::downgrade(self)));
            }
            if let Some(ty) = self.type_by_external_id(owner_id).await {
                return Ok(crate::meta::QualityOwnerSlot::Type(Arc::downgrade(&ty)));
            }
            if let Some(rep) = self.find_representation(owner_id).await {
                return Ok(crate::meta::QualityOwnerSlot::Rep(Arc::downgrade(&rep)));
            }
            if let Some(conn) = self.find_connector(owner_id).await {
                return Ok(crate::meta::QualityOwnerSlot::Conn(Arc::downgrade(&conn)));
            }
            if let Some(des) = self.design_by_external_id(owner_id).await {
                return Ok(crate::meta::QualityOwnerSlot::Design(Arc::downgrade(&des)));
            }
            Err(crate::error::SemioError::not_found("QualityOwner", owner_id.as_str()))
        }

        /// @emoji ➕ Create [`Tag`], register globally, append to owner collection.
        pub async fn create_and_register_tag(self: &Arc<Self>, owner_id: &Id, input: crate::meta::TagInput) -> Result<Arc<Tag>, crate::error::SemioError> {
            let attribute_count = input.attributes.as_ref().map(|items| items.len()).unwrap_or_default();
            let tag_id = Id::new().await;
            let mut attribute_ids = Vec::with_capacity(attribute_count);
            for _ in 0..attribute_count {
                attribute_ids.push(Id::new().await);
            }
            self.create_and_register_tag_with_scope(owner_id, &tag_id, &attribute_ids, input).await
        }

        /// @emoji 🪪 Create [`Tag`] from a normalized operation scope carrying every referenced id.
        pub async fn create_and_register_tag_with_scope(self: &Arc<Self>, owner_id: &Id, tag_id: &Id, attribute_ids: &[Id], input: crate::meta::TagInput) -> Result<Arc<Tag>, crate::error::SemioError> {
            let slot = self.resolve_tag_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let tag = Tag::new_with_id(slot, tag_id.clone(), input.name, input.description, input.icon, input.order, attrs);
            self.register_tag(tag.clone()).await;
            match &*tag.owner.read().await {
                crate::meta::TagOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.tags.write().await.push(tag.clone());
                    }
                }
                crate::meta::TagOwnerSlot::Unset => {}
            }
            Ok(tag)
        }

        /// @emoji 🗑 Remove a tag from its owner vec + id map.
        pub async fn delete_tag_by_id(self: &Arc<Self>, tag_id: &Id) -> Result<(), crate::error::SemioError> {
            let tag = self.find_tag(tag_id).await.ok_or_else(|| crate::error::SemioError::not_found("Tag", tag_id.as_str()))?;
            match &*tag.owner.read().await {
                crate::meta::TagOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.tags.write().await.retain(|t| &t.id != tag_id);
                    }
                }
                crate::meta::TagOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.tags.write().await.retain(|t| &t.id != tag_id);
                    }
                }
                crate::meta::TagOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.tags.write().await.retain(|t| &t.id != tag_id);
                    }
                }
                crate::meta::TagOwnerSlot::Unset => {}
            }
            self.tag_by_id.write().await.remove(tag_id);
            Ok(())
        }

        /// @emoji ➕ Create [`Concept`] under kit or type.
        pub async fn create_and_register_concept(self: &Arc<Self>, owner_id: &Id, input: crate::meta::ConceptInput) -> Result<Arc<Concept>, crate::error::SemioError> {
            let attribute_count = input.attributes.as_ref().map(|items| items.len()).unwrap_or_default();
            let concept_id = Id::new().await;
            let mut attribute_ids = Vec::with_capacity(attribute_count);
            for _ in 0..attribute_count {
                attribute_ids.push(Id::new().await);
            }
            self.create_and_register_concept_with_scope(owner_id, &concept_id, &attribute_ids, input).await
        }

        /// @emoji 🪪 Create [`Concept`] from a normalized operation scope carrying every referenced id.
        pub async fn create_and_register_concept_with_scope(self: &Arc<Self>, owner_id: &Id, concept_id: &Id, attribute_ids: &[Id], input: crate::meta::ConceptInput) -> Result<Arc<Concept>, crate::error::SemioError> {
            let slot = self.resolve_concept_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let c = Concept::new_with_id(slot, concept_id.clone(), input.name, input.description, input.icon, input.order, attrs);
            self.register_concept(c.clone()).await;
            match &*c.owner.read().await {
                crate::meta::ConceptOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.concepts.write().await.push(c.clone());
                    }
                }
                crate::meta::ConceptOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.concepts.write().await.push(c.clone());
                    }
                }
                crate::meta::ConceptOwnerSlot::Unset => {}
            }
            Ok(c)
        }

        /// @emoji 🗑 Remove a concept from its owner vec + id map.
        pub async fn delete_concept_by_id(self: &Arc<Self>, concept_id: &Id) -> Result<(), crate::error::SemioError> {
            let concept = self.find_concept(concept_id).await.ok_or_else(|| crate::error::SemioError::not_found("Concept", concept_id.as_str()))?;
            match &*concept.owner.read().await {
                crate::meta::ConceptOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.concepts.write().await.retain(|item| &item.id != concept_id);
                    }
                }
                crate::meta::ConceptOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.concepts.write().await.retain(|item| &item.id != concept_id);
                    }
                }
                crate::meta::ConceptOwnerSlot::Unset => {}
            }
            self.concept_by_id.write().await.remove(concept_id);
            Ok(())
        }

        /// @emoji ➕ Create [`Quality`] under resolved owner (kit/type/representation/connector/design).
        pub async fn create_and_register_quality(self: &Arc<Self>, owner_id: &Id, input: crate::meta::QualityInput) -> Result<Arc<Quality>, crate::error::SemioError> {
            let attribute_count = input.attributes.as_ref().map(|items| items.len()).unwrap_or_default();
            let quality_id = Id::new().await;
            let mut attribute_ids = Vec::with_capacity(attribute_count);
            for _ in 0..attribute_count {
                attribute_ids.push(Id::new().await);
            }
            self.create_and_register_quality_with_scope(owner_id, &quality_id, &attribute_ids, &[], input).await
        }

        /// @emoji 🪪 Create [`Quality`] from a normalized operation scope carrying every referenced id.
        pub async fn create_and_register_quality_with_scope(self: &Arc<Self>, owner_id: &Id, quality_id: &Id, attribute_ids: &[Id], benchmark_ids: &[Id], input: crate::meta::QualityInput) -> Result<Arc<Quality>, crate::error::SemioError> {
            let slot = self.resolve_quality_owner_slot(owner_id).await?;
            if !benchmark_ids.is_empty() {
                return Err(crate::error::SemioError::invalid("quality benchmark ids are not supported yet"));
            }
            let attrs = crate::meta::attributes_from_inputs_with_ids(input.attributes.clone(), attribute_ids)?;
            let q = Quality::new_with_id(slot, quality_id.clone(), input.key, input.value, input.unit, input.definition, input.description, input.icon, Vec::new(), attrs);
            self.register_quality(q.clone()).await;
            match &*q.owner.read().await {
                crate::meta::QualityOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Conn(w) => {
                    if let Some(c) = w.upgrade() {
                        c.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Design(w) => {
                    if let Some(d) = w.upgrade() {
                        d.qualities.write().await.push(q.clone());
                    }
                }
                crate::meta::QualityOwnerSlot::Unset => {}
            }
            Ok(q)
        }

        /// @emoji 🗑 Remove a quality from its owner vec + id map.
        pub async fn delete_quality_by_id(self: &Arc<Self>, quality_id: &Id) -> Result<(), crate::error::SemioError> {
            let quality = self.find_quality(quality_id).await.ok_or_else(|| crate::error::SemioError::not_found("Quality", quality_id.as_str()))?;
            match &*quality.owner.read().await {
                crate::meta::QualityOwnerSlot::Kit(w) => {
                    if let Some(k) = w.upgrade() {
                        k.qualities.write().await.retain(|item| &item.id != quality_id);
                    }
                }
                crate::meta::QualityOwnerSlot::Type(w) => {
                    if let Some(t) = w.upgrade() {
                        t.qualities.write().await.retain(|item| &item.id != quality_id);
                    }
                }
                crate::meta::QualityOwnerSlot::Rep(w) => {
                    if let Some(r) = w.upgrade() {
                        r.qualities.write().await.retain(|item| &item.id != quality_id);
                    }
                }
                crate::meta::QualityOwnerSlot::Conn(w) => {
                    if let Some(c) = w.upgrade() {
                        c.qualities.write().await.retain(|item| &item.id != quality_id);
                    }
                }
                crate::meta::QualityOwnerSlot::Design(w) => {
                    if let Some(d) = w.upgrade() {
                        d.qualities.write().await.retain(|item| &item.id != quality_id);
                    }
                }
                crate::meta::QualityOwnerSlot::Unset => {}
            }
            self.quality_by_id.write().await.remove(quality_id);
            Ok(())
        }

        /// @emoji 🆕 Insert (or look up) a design by id, returning the shared Arc (maintains [`Kit::design_weak_by_id`]).
        pub async fn ensure_design(self: &Arc<Self>, design_id: &Id) -> Arc<design::Design> {
            {
                let map = self.design_weak_by_id.read().await;
                if let Some(w) = map.get(design_id) {
                    if let Some(d) = w.upgrade() {
                        return d;
                    }
                }
            }
            let mut designs = self.designs.write().await;
            let mut map = self.design_weak_by_id.write().await;
            if let Some(w) = map.get(design_id) {
                if let Some(d) = w.upgrade() {
                    return d;
                }
            }
            let d = design::Design::with_id(Arc::downgrade(self), design_id.clone(), format!("design-{}", design_id.as_str())).await;
            map.insert(design_id.clone(), Arc::downgrade(&d));
            designs.push(d.clone());
            d
        }

        /// 🧷 Single external-id translation → opaque handle + shared [`Arc`] for all subsequent hot-path graph work.
        pub async fn bind_external_design_id(self: &Arc<Self>, design_id: &Id) -> (crate::kit_graph_engine::DesignHandle, Arc<design::Design>) {
            let design = self.ensure_design(design_id).await;
            let slot = {
                let designs = self.designs.read().await;
                designs.iter().position(|d| &d.id == design_id).expect("design slot after ensure_design") as u32
            };
            (crate::kit_graph_engine::DesignHandle(slot), design)
        }

        /// @emoji 🔁 Clears every **layout** node’s placed pieces and piece slot maps so [`crate::kit_backbone`] can replay without duplicating projections; kit metadata and empty layout shells stay resident (detach leaves this graph materialized in memory).
        pub async fn clear_piece_projections_for_backbone_replay(self: &Arc<Self>) {
            let designs = self.designs.read().await;
            for design in designs.iter() {
                design.pieces.write().await.clear();
                design.piece_weak_by_external_id.write().await.clear();
            }
        }

        /// 🧾 Overlays layout + metadata from `@semio/js` `KitFullDto`-style JSON (`types`, `designs`, `pieces`); control plane authoritative copy stays in-process.
        pub async fn hydrate_from_kit_full_snapshot_json(self: &Arc<Self>, dto: &serde_json::Value) -> Result<(), crate::error::SemioError> {
            if let Some(n) = dto.get("name").and_then(|v| v.as_str()) {
                *self.name.write().await = n.to_string();
            }
            if let Some(id_override) = dto.get("id").and_then(|v| v.as_str()) {
                *self.snapshot_external_kit_id.write().await = Some(Id::from(id_override));
            } else {
                *self.snapshot_external_kit_id.write().await = None;
            }

            {
                let mut tys = self.types.write().await;
                let mut tw = self.type_weak_by_id.write().await;
                tys.clear();
                tw.clear();
                let types_arr = dto.get("types").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
                let owner = Arc::downgrade(self);
                for t in &types_arr {
                    let Some(ts) = t.get("id").and_then(|x| x.as_str()) else { continue };
                    let nm = t.get("name").and_then(|x| x.as_str()).unwrap_or("");
                    let row = crate::kit::r#type::Type::new_with_external_id(owner.clone(), ts.into(), nm.to_string()).await;
                    tw.insert(row.id.clone(), Arc::downgrade(&row));
                    tys.push(row);
                }
            }

            let owner = Arc::downgrade(self);
            let design_arr_owned = dto.get("designs").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
            let mut appended: Vec<Arc<design::Design>> = Vec::new();
            for d in &design_arr_owned {
                let Some(ds) = d.get("id").and_then(|x| x.as_str()) else { continue };
                let dn = d.get("name").and_then(|x| x.as_str()).unwrap_or(ds);
                let des = crate::kit::design::Design::with_id(owner.clone(), ds.into(), dn.to_string()).await;
                design::Design::hydrate_pieces_from_snapshot_json(&des, self, d).await?;
                appended.push(des);
            }
            {
                let mut designs_slot = self.designs.write().await;
                let mut weak_map = self.design_weak_by_id.write().await;
                designs_slot.clear();
                weak_map.clear();
                for des in appended {
                    let did = des.id.clone();
                    weak_map.insert(did, Arc::downgrade(&des));
                    designs_slot.push(des);
                }
            }

            Ok(())
        }

        /// 🧾 Canonical JSON snapshot consumed by `@semio/js` `fullSnapshot` (single RS truth projected to DTO-shaped JSON).
        pub async fn kit_full_snapshot_value(&self) -> serde_json::Value {
            let id = self.workspace_kit_id().await;
            let name = self.name.read().await.clone();
            let created = self.created.read().await.as_ref().map(|t| t.0.clone());
            let updated = self.updated.read().await.as_ref().map(|t| t.0.clone());

            let types = {
                let tys = self.types.read().await;
                let mut out = Vec::<serde_json::Value>::with_capacity(tys.len());
                for t in tys.iter() {
                    let tid = t.id.clone();
                    let nm = t.name.read().await.clone();
                    out.push(serde_json::json!({"id": tid.as_str(), "name": nm, "connectors": []}));
                }
                serde_json::Value::Array(out)
            };

            let mut designs_arr = Vec::<serde_json::Value>::new();
            {
                let dz = self.designs.read().await;
                for d in dz.iter() {
                    let mut pieces_arr = Vec::<serde_json::Value>::new();
                    let plist = d.pieces.read().await;
                    for p in plist.iter() {
                        let pv = if let Some(n) = p.position.read().await.as_ref() { n.snapshot_value().await } else { crate::geom::Position::default() };
                        let tid = match p.blueprint.read().await.clone() {
                            crate::kit::r#type::Blueprint::Type(ty) => ty.id.clone(),
                            crate::kit::r#type::Blueprint::Design(_) => continue,
                        };
                        let pname = (*p.name.read().await).clone().unwrap_or_default();
                        let pl = pv.plane;
                        pieces_arr.push(serde_json::json!({
                          "id": p.id.as_str(),
                          "name": pname,
                          "type": { "id": tid.as_str() },
                          "plane": {
                            "origin": { "x": pl.origin.x, "y": pl.origin.y, "z": pl.origin.z },
                            "xAxis": { "x": pl.x_axis.x, "y": pl.x_axis.y, "z": pl.x_axis.z },
                            "yAxis": { "x": pl.y_axis.x, "y": pl.y_axis.y, "z": pl.y_axis.z },
                          },
                          "center": { "u": pv.center.u, "v": pv.center.v },
                          "scale": p.scale.read().await.unwrap_or(1.0),
                          "color": "#000000",
                          "props": [],
                          "attributes": [],
                        }));
                    }
                    designs_arr.push(serde_json::json!({
                       "id": d.id.as_str(),
                       "name": d.name.read().await.clone(),
                       "pieces": pieces_arr,
                       "connections": [],
                    }));
                }
            }

            serde_json::json!({
                "id": id.as_str(),
                "name": name,
                "createdAt": created.unwrap_or_else(|| "2020-01-01T00:00:00.000Z".to_string()),
                "updatedAt": updated.unwrap_or_else(|| "2020-01-01T00:00:00.000Z".to_string()),
                "types": types,
                "designs": designs_arr,
                "authors": [],
                "concepts": [],
                "qualities": [],
                "tags": [],
                "props": [],
                "folders": [],
                "files": [],
                "layers": [],
                "stats": [],
                "groups": [],
            })
        }
    }

    #[Object(name = "Kit")]
    impl Kit {
        pub async fn id(&self) -> Id {
            self.workspace_kit_id().await
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> KitOwner {
            match self.owner_graph.upgrade() {
                Some(g) => KitOwner::Graph(g),
                None => KitOwner::Graph(Arc::new(crate::vcs::Graph::default())),
            }
        }
        pub async fn checkpoint(&self) -> Option<Arc<crate::vcs::Checkpoint>> {
            None
        }
        pub async fn name(&self) -> String {
            self.name.read().await.clone()
        }
        pub async fn description(&self) -> Option<String> {
            self.description.read().await.clone()
        }
        pub async fn icon(&self) -> Option<String> {
            self.icon.read().await.clone()
        }
        pub async fn image(&self) -> Option<String> {
            self.image.read().await.clone()
        }
        pub async fn preview(&self) -> Option<String> {
            self.preview.read().await.clone()
        }
        pub async fn remote(&self) -> Option<String> {
            self.remote.read().await.clone()
        }
        pub async fn homepage(&self) -> Option<String> {
            self.homepage.read().await.clone()
        }
        pub async fn license(&self) -> Option<String> {
            self.license.read().await.clone()
        }
        pub async fn uri(&self) -> Option<String> {
            self.uri.read().await.clone()
        }
        #[graphql(name = "createdAt")]
        pub async fn created_at(&self) -> Option<Timestamp> {
            self.created.read().await.clone()
        }
        #[graphql(name = "updatedAt")]
        pub async fn updated_at(&self) -> Option<Timestamp> {
            self.updated.read().await.clone()
        }
        pub async fn version(&self) -> Option<String> {
            self.version.read().await.clone()
        }
        pub async fn design(&self, id: Id) -> Option<Arc<design::Design>> {
            self.design_by_external_id(&id).await
        }
        pub async fn designs(&self) -> crate::gql_relay::DesignConnection {
            crate::gql_relay::DesignConnection::from_designs(self.designs.read().await.clone()).await
        }
        #[graphql(name = "type")]
        pub async fn type_(&self, id: Id) -> Option<Arc<r#type::Type>> {
            self.type_by_external_id(&id).await
        }
        pub async fn types(&self) -> crate::gql_relay::TypeConnection {
            crate::gql_relay::TypeConnection::from_types(self.types.read().await.clone()).await
        }
        pub async fn files(&self) -> crate::gql_relay::FileConnection {
            crate::gql_relay::FileConnection::from_rows(self.files.read().await.clone())
        }
        pub async fn folders(&self) -> crate::gql_relay::FolderConnection {
            crate::gql_relay::FolderConnection::from_rows(self.folders.read().await.clone())
        }
        pub async fn families(&self) -> crate::gql_relay::FamilyConnection {
            crate::gql_relay::FamilyConnection::from_rows(Vec::new())
        }
        pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
            crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
        }
        pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
            crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone()).await
        }
        pub async fn tags(&self) -> crate::gql_relay::TagConnection {
            crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone()).await
        }
        pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
            crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone()).await
        }
        pub async fn props(&self) -> crate::gql_relay::PropConnection {
            crate::gql_relay::PropConnection::from_rows(self.props.read().await.clone())
        }
        pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
            crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
        }
        pub async fn stats(&self) -> crate::gql_relay::StatConnection {
            crate::gql_relay::StatConnection::from_rows(self.stats.read().await.clone())
        }
    }
    //#endregion 📦 kit
}

//#endregion 📦 kit

//#region 🏷️ meta graphql

/// @emoji 🔗 SDL `union TagOwner`.
#[derive(Clone, async_graphql::Union)]
#[graphql(name = "TagOwner")]
pub enum TagOwnerUnion {
    Kit(std::sync::Arc<crate::kit::Kit>),
    Type(std::sync::Arc<crate::kit::r#type::Type>),
    Representation(std::sync::Arc<crate::kit::r#type::Representation>),
}

/// @emoji 🔗 SDL `union ConceptOwner`.
#[derive(Clone, async_graphql::Union)]
#[graphql(name = "ConceptOwner")]
pub enum ConceptOwnerUnion {
    Kit(std::sync::Arc<crate::kit::Kit>),
    Type(std::sync::Arc<crate::kit::r#type::Type>),
}

/// @emoji 🔗 SDL `union QualityOwner`.
#[derive(Clone, async_graphql::Union)]
#[graphql(name = "QualityOwner")]
pub enum QualityOwnerUnion {
    Connector(std::sync::Arc<crate::kit::r#type::Connector>),
    Representation(std::sync::Arc<crate::kit::r#type::Representation>),
    Type(std::sync::Arc<crate::kit::r#type::Type>),
    Design(std::sync::Arc<crate::kit::design::Design>),
    Kit(std::sync::Arc<crate::kit::Kit>),
}

#[async_graphql::Object(name = "Tag")]
impl crate::meta::Tag {
    pub async fn id(&self) -> crate::id::Id {
        self.id.clone()
    }
    pub async fn hash(&self) -> String {
        self.compute_hash().await
    }
    pub async fn owner(&self) -> async_graphql::Result<TagOwnerUnion> {
        match &*self.owner.read().await {
            crate::meta::TagOwnerSlot::Kit(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Tag.kit owner dropped")).map(TagOwnerUnion::Kit),
            crate::meta::TagOwnerSlot::Type(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Tag.type owner dropped")).map(TagOwnerUnion::Type),
            crate::meta::TagOwnerSlot::Rep(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Tag.representation owner dropped")).map(TagOwnerUnion::Representation),
            crate::meta::TagOwnerSlot::Unset => Err(async_graphql::Error::new("Tag.owner unset")),
        }
    }
    pub async fn name(&self) -> String {
        self.name.read().await.clone()
    }
    pub async fn description(&self) -> Option<String> {
        self.description.read().await.clone()
    }
    pub async fn icon(&self) -> Option<String> {
        self.icon.read().await.clone()
    }
    pub async fn order(&self) -> Option<i32> {
        *self.order.read().await
    }
    pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
        crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
    }
}

#[async_graphql::Object(name = "Concept")]
impl crate::meta::Concept {
    pub async fn id(&self) -> crate::id::Id {
        self.id.clone()
    }
    pub async fn hash(&self) -> String {
        self.compute_hash().await
    }
    pub async fn owner(&self) -> async_graphql::Result<ConceptOwnerUnion> {
        match &*self.owner.read().await {
            crate::meta::ConceptOwnerSlot::Kit(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Concept.kit owner dropped")).map(ConceptOwnerUnion::Kit),
            crate::meta::ConceptOwnerSlot::Type(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Concept.type owner dropped")).map(ConceptOwnerUnion::Type),
            crate::meta::ConceptOwnerSlot::Unset => Err(async_graphql::Error::new("Concept.owner unset")),
        }
    }
    pub async fn name(&self) -> String {
        self.name.read().await.clone()
    }
    pub async fn description(&self) -> Option<String> {
        self.description.read().await.clone()
    }
    pub async fn icon(&self) -> Option<String> {
        self.icon.read().await.clone()
    }
    pub async fn order(&self) -> Option<i32> {
        *self.order.read().await
    }
    pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
        crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
    }
}

#[async_graphql::Object(name = "Quality")]
impl crate::meta::Quality {
    pub async fn id(&self) -> crate::id::Id {
        self.id.clone()
    }
    pub async fn hash(&self) -> String {
        self.compute_hash().await
    }
    pub async fn owner(&self) -> async_graphql::Result<QualityOwnerUnion> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Kit(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Quality.kit owner dropped")).map(QualityOwnerUnion::Kit),
            crate::meta::QualityOwnerSlot::Type(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Quality.type owner dropped")).map(QualityOwnerUnion::Type),
            crate::meta::QualityOwnerSlot::Rep(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Quality.representation owner dropped")).map(QualityOwnerUnion::Representation),
            crate::meta::QualityOwnerSlot::Conn(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Quality.connector owner dropped")).map(QualityOwnerUnion::Connector),
            crate::meta::QualityOwnerSlot::Design(w) => w.upgrade().ok_or_else(|| async_graphql::Error::new("Quality.design owner dropped")).map(QualityOwnerUnion::Design),
            crate::meta::QualityOwnerSlot::Unset => Err(async_graphql::Error::new("Quality.owner unset")),
        }
    }
    pub async fn key(&self) -> String {
        self.key.read().await.clone()
    }
    pub async fn value(&self) -> Option<String> {
        self.value.read().await.clone()
    }
    pub async fn unit(&self) -> Option<String> {
        self.unit.read().await.clone()
    }
    pub async fn definition(&self) -> Option<String> {
        self.definition.read().await.clone()
    }
    pub async fn description(&self) -> Option<String> {
        self.description.read().await.clone()
    }
    pub async fn icon(&self) -> Option<String> {
        self.icon.read().await.clone()
    }
    pub async fn benchmarks(&self) -> crate::gql_relay::BenchmarkConnection {
        crate::gql_relay::BenchmarkConnection::from_rows(self.benchmarks.read().await.clone())
    }
    pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
        crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
    }
}

//#endregion 🏷️ meta graphql

//#region 🌿 vcs

pub mod vcs {
    //! 🌿 Version-control entities — change, edit, draft, checkpoint, alternative, graph, session, conflict.
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Weak};

    use async_graphql::{InputObject, Object, Union};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::hash::{h, merkle_node_str};
    use crate::id::Id;
    use crate::kit::Kit;
    use crate::meta::Author;
    use crate::operation;
    use crate::timestamp::Timestamp;

    //#region 🪪 change
    pub struct Change {
        pub id: Id,
        pub owner: RwLock<Option<ChangeOwnerRef>>,
        pub parent_edit: RwLock<Weak<Edit>>,
        pub started_at: RwLock<Option<Timestamp>>,
        pub saved_at: RwLock<Option<Timestamp>>,
        pub description: RwLock<String>,
        pub origin: RwLock<String>,
        /// @emoji 📜 Forward [`crate::operation::KitOperation`] steps (materialized via `Kit::apply_diff`).
        pub forwards: RwLock<Vec<operation::KitOperation>>,
        /// @emoji 📜 Backward companion operations for explicit undo/redo (same materialization pipeline).
        pub backwards: RwLock<Vec<operation::KitOperation>>,
    }

    /// 🔗 Weak owner lane matching SDL `Alternative | Checkpoint` for persisted [`Change`] rows.
    #[derive(Clone)]
    pub enum ChangeOwnerRef {
        Alternative(Weak<Alternative>),
        Checkpoint(Weak<Checkpoint>),
    }

    impl Default for Change {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner: RwLock::new(None),
                parent_edit: RwLock::new(Weak::new()),
                started_at: RwLock::new(None),
                saved_at: RwLock::new(None),
                description: RwLock::new(String::new()),
                origin: RwLock::new(String::new()),
                forwards: RwLock::new(Vec::new()),
                backwards: RwLock::new(Vec::new()),
            }
        }
    }

    impl Change {
        pub async fn new() -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, ..Default::default() })
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
    }

    #[Object(name = "Change")]
    impl Change {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> ChangeOwnerUnion {
            match self.owner.read().await.clone() {
                Some(ChangeOwnerRef::Alternative(w)) => ChangeOwnerUnion::Alternative(w.upgrade().unwrap_or_default()),
                Some(ChangeOwnerRef::Checkpoint(w)) => ChangeOwnerUnion::Checkpoint(w.upgrade().unwrap_or_default()),
                None => ChangeOwnerUnion::Checkpoint(Arc::default()),
            }
        }

        pub async fn edits(&self) -> crate::gql_relay::EditConnection {
            if let Some(ed) = self.parent_edit.read().await.upgrade() {
                crate::gql_relay::EditConnection::from_edits(vec![ed]).await
            } else {
                crate::gql_relay::EditConnection::empty()
            }
        }

        #[graphql(name = "startedAt")]
        pub async fn started_at(&self) -> Timestamp {
            self.started_at.read().await.clone().unwrap_or_default()
        }
        #[graphql(name = "savedAt")]
        pub async fn saved_at(&self) -> Option<Timestamp> {
            self.saved_at.read().await.clone()
        }
        pub async fn saved(&self) -> bool {
            self.saved_at.read().await.is_some()
        }
        pub async fn description(&self) -> String {
            self.description.read().await.clone()
        }
        pub async fn origin(&self) -> String {
            self.origin.read().await.clone()
        }

        /// @emoji 🔗 Ordered  operation record ids constituting the forwards side (bundle `OpLog` ids) when persisted.
        #[graphql(name = "forwardOpRecordIds")]
        pub async fn forward__op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        /// @emoji 🔗 Ordered  operation record ids for backwards / inverse application when persisted separately from `OperationKind`.
        #[graphql(name = "backwardOpRecordIds")]
        pub async fn backward__op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }
    }

    #[derive(Clone, Union)]
    #[graphql(name = "EditOwner")]
    pub enum EditOwnerUnion {
        Alternative(Arc<Alternative>),
        Checkpoint(Arc<Checkpoint>),
    }

    #[derive(Clone, Union)]
    #[graphql(name = "ChangeOwner")]
    pub enum ChangeOwnerUnion {
        Alternative(Arc<Alternative>),
        Checkpoint(Arc<Checkpoint>),
    }
    //#endregion 🪪 change

    //#region 💼 edit
    pub struct Edit {
        pub id: Id,
        pub owner_draft: Weak<Draft>,
        pub changes: RwLock<Vec<Arc<Change>>>,
        pub forward_iface_ops: RwLock<Vec<Arc<operation::OperationIface>>>,
        pub backward_iface_ops: RwLock<Vec<Arc<operation::OperationIface>>>,
        pub sequence_number: RwLock<i32>,
        pub started_at: RwLock<Option<Timestamp>>,
        pub finished_at: RwLock<Option<Timestamp>>,
        pub description: RwLock<String>,
        pub origin: RwLock<String>,
    }

    impl Default for Edit {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_draft: Weak::new(),
                changes: RwLock::new(Vec::new()),
                forward_iface_ops: RwLock::new(Vec::new()),
                backward_iface_ops: RwLock::new(Vec::new()),
                sequence_number: RwLock::new(0),
                started_at: RwLock::new(None),
                finished_at: RwLock::new(None),
                description: RwLock::new(String::new()),
                origin: RwLock::new(String::new()),
            }
        }
    }

    impl Edit {
        pub async fn new(owner_draft: Weak<Draft>) -> Arc<Self> {
            Self::with_id(owner_draft, Id::new().await, 0).await
        }
        pub async fn with_id(owner_draft: Weak<Draft>, id: Id, sequence_number: i32) -> Arc<Self> {
            Arc::new(Self {
                id,
                owner_draft,
                changes: RwLock::new(Vec::new()),
                forward_iface_ops: RwLock::new(Vec::new()),
                backward_iface_ops: RwLock::new(Vec::new()),
                sequence_number: RwLock::new(sequence_number),
                started_at: RwLock::new(Some(Timestamp::default())),
                finished_at: RwLock::new(None),
                description: RwLock::new(String::new()),
                origin: RwLock::new(String::new()),
            })
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn record(&self, change: Arc<Change>) {
            self.changes.write().await.push(change);
        }
    }

    /// @emoji 🧾 Flatten write-session records into target-schema `ChangeConnection` rows for a version lane.
    async fn changes_from_edits(edits: Vec<Arc<Edit>>) -> Vec<Arc<Change>> {
        let mut out = Vec::new();
        for ed in edits {
            out.extend(ed.changes.read().await.iter().cloned());
        }
        out
    }

    #[Object(name = "Edit")]
    impl Edit {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<EditOwnerUnion> {
            let d = self.owner_draft.upgrade()?;
            if let Some(a) = d.owner_alternative.upgrade() {
                return Some(EditOwnerUnion::Alternative(a));
            }
            let cp = { d.parent_checkpoint.read().await.upgrade() };
            cp.map(EditOwnerUnion::Checkpoint)
        }
        pub async fn forwards(&self) -> crate::gql_relay::OperationConnection {
            crate::gql_relay::OperationConnection::from_iface_rows(self.forward_iface_ops.read().await.clone())
        }
        pub async fn backwards(&self) -> crate::gql_relay::OperationConnection {
            crate::gql_relay::OperationConnection::from_iface_rows(self.backward_iface_ops.read().await.clone())
        }
        #[graphql(name = "sequenceNumber")]
        pub async fn sequence_number(&self) -> i32 {
            *self.sequence_number.read().await
        }
        #[graphql(name = "startedAt")]
        pub async fn started_at(&self) -> Timestamp {
            self.started_at.read().await.clone().unwrap_or_default()
        }
        #[graphql(name = "finishedAt")]
        pub async fn finished_at(&self) -> Option<Timestamp> {
            self.finished_at.read().await.clone()
        }
        pub async fn finished(&self) -> Option<bool> {
            Some(self.finished_at.read().await.is_some())
        }
        pub async fn description(&self) -> String {
            self.description.read().await.clone()
        }
        pub async fn origin(&self) -> String {
            self.origin.read().await.clone()
        }
        pub async fn changes(&self) -> Vec<Arc<Change>> {
            self.changes.read().await.clone()
        }
    }
    //#endregion 💼 edit

    //#region 📝 draft
    pub struct Draft {
        pub id: Id,
        pub owner_alternative: Weak<Alternative>,
        pub parent_checkpoint: RwLock<Weak<Checkpoint>>,
        pub target_alternative: RwLock<Weak<Alternative>>,
        pub open_transaction: RwLock<Weak<Edit>>,
        pub finalized_transactions: RwLock<Vec<Arc<Edit>>>,
        pub redo_transactions: RwLock<Vec<Arc<Edit>>>,
        pub transactions: RwLock<Vec<Arc<Edit>>>,
        /// @emoji 🔢 Bumped on every recorded operation; drives [`Graph::materialized_cache`] invalidation.
        pub change_seq: AtomicU64,
    }

    impl Default for Draft {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_alternative: Weak::new(),
                parent_checkpoint: RwLock::new(Weak::new()),
                target_alternative: RwLock::new(Weak::new()),
                open_transaction: RwLock::new(Weak::new()),
                finalized_transactions: RwLock::new(Vec::new()),
                redo_transactions: RwLock::new(Vec::new()),
                transactions: RwLock::new(Vec::new()),
                change_seq: AtomicU64::new(0),
            }
        }
    }

    impl Draft {
        pub async fn new() -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, ..Default::default() })
        }
        pub async fn with_id(id: Id) -> Arc<Self> {
            Arc::new(Self { id, ..Default::default() })
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }

        /// 🆕 Look up (or open) the transaction matching `id` and stash it as the open transaction.
        pub async fn ensure_transaction(self: &Arc<Self>, id: &Id) -> Arc<Edit> {
            if let Some(t) = self.transactions.read().await.iter().find(|t| &t.id == id).cloned() {
                *self.open_transaction.write().await = Arc::downgrade(&t);
                return t;
            }
            let seq = self.transactions.read().await.len() as i32;
            let t = Edit::with_id(Arc::downgrade(self), id.clone(), seq).await;
            self.transactions.write().await.push(t.clone());
            *self.open_transaction.write().await = Arc::downgrade(&t);
            t
        }
    }
    //#endregion 📝 draft

    //#region 📍 kit read point
    /// @emoji 📍 GraphQL input for [`Graph::materialized_kit_at_point`] (wip anchors).
    #[derive(Clone, Debug, InputObject)]
    #[graphql(name = "KitReadPointInput")]
    pub struct KitReadPointInput {
        #[graphql(name = "theKit")]
        pub the_kit: Option<bool>,
        #[graphql(name = "checkpointId")]
        pub checkpoint_id: Option<Id>,
        #[graphql(name = "checkpointChangeId")]
        pub checkpoint_change_id: Option<Id>,
        #[graphql(name = "checkpointOperationId")]
        pub checkpoint_operation_id: Option<Id>,
        #[graphql(name = "alternativeId")]
        pub alternative_id: Option<Id>,
        #[graphql(name = "draftId")]
        pub draft_id: Option<Id>,
        #[graphql(name = "draftAlternativeId")]
        pub draft_alternative_id: Option<Id>,
        #[graphql(name = "draftTransactionId")]
        pub draft_transaction_id: Option<Id>,
        #[graphql(name = "draftOperationId")]
        pub draft_operation_id: Option<Id>,
        #[graphql(name = "draftChangeId")]
        pub draft_change_id: Option<Id>,
    }
    //#endregion 📍 kit read point

    //#region 📖 read write version
    /// @emoji 📖 Placeholder version entity for `OwnerEntity` (`ReadVersion` SDL lane).
    pub struct ReadVersion {
        pub id: Id,
    }

    impl ReadVersion {
        pub async fn compute_hash(&self) -> String {
            h(&["semio:vcs:ReadVersion", self.id.as_str()])
        }
    }

    #[Object(name = "ReadVersion")]
    impl ReadVersion {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
    }

    /// @emoji 📖 Placeholder version entity for `OwnerEntity` (`WriteVersion` SDL lane).
    pub struct WriteVersion {
        pub id: Id,
    }

    impl WriteVersion {
        pub async fn compute_hash(&self) -> String {
            h(&["semio:vcs:WriteVersion", self.id.as_str()])
        }
    }

    #[Object(name = "WriteVersion")]
    impl WriteVersion {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
    }
    //#endregion 📖 read write version

    //#region 🪧 checkpoint
    pub struct Checkpoint {
        pub id: Id,
        pub timestamp: RwLock<Option<Timestamp>>,
        pub authors: RwLock<Vec<Author>>,
        /// @emoji 🔗 Owning graph for [`Checkpoint::initial`] / [`Checkpoint::kit`] (single [`Graph::initial_kit`] baseline, not per-checkpoint kit storage).
        pub owner_graph: Weak<Graph>,
        pub parent_checkpoint: RwLock<Weak<Checkpoint>>,
        pub message: RwLock<Option<String>>,
        pub is_release: RwLock<bool>,
        pub change_count: RwLock<i32>,
    }

    impl Default for Checkpoint {
        fn default() -> Self {
            Self {
                id: Id::default(),
                timestamp: RwLock::new(None),
                authors: RwLock::new(Vec::new()),
                owner_graph: Weak::new(),
                parent_checkpoint: RwLock::new(Weak::new()),
                message: RwLock::new(None),
                is_release: RwLock::new(false),
                change_count: RwLock::new(0),
            }
        }
    }

    impl Checkpoint {
        pub async fn new() -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, ..Default::default() })
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
    }

    #[Object(name = "Checkpoint")]
    impl Checkpoint {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn timestamp(&self) -> Option<Timestamp> {
            self.timestamp.read().await.clone()
        }
        pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
            crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
        }

        /// @emoji 🔗 SDL `Checkpoint.parent` — prior checkpoint in the spine (none for the seed checkpoint).
        pub async fn parent(&self) -> Option<Arc<Checkpoint>> {
            self.parent_checkpoint.read().await.upgrade()
        }

        /// @emoji 🔗 SDL `Checkpoint.ancestors` — ordered walk up `parent` (immediate parent first).
        pub async fn ancestors(&self) -> Vec<Arc<Checkpoint>> {
            let mut out = Vec::new();
            let mut cur = self.parent_checkpoint.read().await.upgrade();
            while let Some(c) = cur {
                let next = c.parent_checkpoint.read().await.upgrade();
                out.push(c);
                cur = next;
            }
            out
        }

        /// @emoji 🌱 SDL `Checkpoint.initial` — graph-level [`Graph::initial_kit`] baseline (saved once per graph, not duplicated on each checkpoint).
        pub async fn initial(&self) -> Option<Arc<Kit>> {
            match self.owner_graph.upgrade() {
                Some(g) => Some(g.initial_kit.read().await.clone()),
                None => None,
            }
        }

        /// @emoji 📦 SDL `Checkpoint.kit` — materialized kit at this checkpoint; until checkpoint-owned `changes` are wired, matches `initial`.
        pub async fn kit(&self) -> Option<Arc<Kit>> {
            match self.owner_graph.upgrade() {
                Some(g) => Some(g.initial_kit.read().await.clone()),
                None => None,
            }
        }

        pub async fn changes(&self) -> crate::gql_relay::ChangeConnection {
            crate::gql_relay::ChangeConnection::empty()
        }

        pub async fn change(&self, id: Id) -> Option<Arc<Change>> {
            let _ = id;
            None
        }

        pub async fn message(&self) -> String {
            self.message.read().await.clone().unwrap_or_default()
        }
    }
    //#endregion 🪧 checkpoint

    //#region 🧭 the kit version
    pub struct TheKit {
        pub owner_graph: Weak<Graph>,
    }

    impl TheKit {
        /// @emoji 🧭 Main kit version wrapper; exposes `kit` plus version change lanes without making `Kit` itself a version.
        pub fn new(owner_graph: Weak<Graph>) -> Arc<Self> {
            Arc::new(Self { owner_graph })
        }

        pub async fn compute_hash(&self) -> String {
            match self.owner_graph.upgrade() {
                Some(g) => h(&["the-kit", g.id.as_str()]),
                None => h(&["the-kit"]),
            }
        }

        async fn graph(&self) -> Option<Arc<Graph>> {
            self.owner_graph.upgrade()
        }
    }

    #[Object(name = "TheKit")]
    impl TheKit {
        pub async fn id(&self) -> Id {
            self.graph().await.map(|g| g.id.clone()).unwrap_or_default()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<Arc<Graph>> {
            self.graph().await
        }
        pub async fn checkpoint(&self) -> crate::gql_relay::CheckpointConnection {
            match self.graph().await {
                Some(g) => crate::gql_relay::CheckpointConnection::from_checkpoints(g.checkpoints.read().await.clone()).await,
                None => crate::gql_relay::CheckpointConnection::from_checkpoints(Vec::new()).await,
            }
        }
        #[graphql(name = "latestWipCheckpointAncestor")]
        pub async fn latest_wip_checkpoint_ancestor(&self) -> Option<Arc<Checkpoint>> {
            let g = self.graph().await?;
            let checkpoints = g.checkpoints.read().await;
            checkpoints.last().cloned()
        }
        #[graphql(name = "savedChanges")]
        pub async fn saved_changes(&self) -> crate::gql_relay::ChangeConnection {
            match self.graph().await {
                Some(g) => g.saved_change_connection_for_main_line().await,
                None => crate::gql_relay::ChangeConnection::empty(),
            }
        }
        #[graphql(name = "unsavedChanges")]
        pub async fn unsaved_changes(&self) -> crate::gql_relay::ChangeConnection {
            match self.graph().await {
                Some(g) => g.unsaved_change_connection_for_main_line().await,
                None => crate::gql_relay::ChangeConnection::empty(),
            }
        }
        pub async fn kit(&self) -> Arc<Kit> {
            match self.graph().await {
                Some(g) => g.materialized_head_kit().await,
                None => Arc::default(),
            }
        }
    }
    //#endregion 🧭 the kit version

    //#region 🌱 alternative
    pub struct Alternative {
        pub id: Id,
        pub owner_graph: Weak<Graph>,
        pub name: RwLock<String>,
        pub start: RwLock<Weak<Checkpoint>>,
        pub checkpoints: RwLock<Vec<Arc<Checkpoint>>>,
        pub kit: RwLock<Option<Arc<Kit>>>,
        pub draft: RwLock<Weak<Draft>>,
        pub transaction: RwLock<Weak<Edit>>,
    }

    impl Default for Alternative {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_graph: Weak::new(),
                name: RwLock::new(String::new()),
                start: RwLock::new(Weak::new()),
                checkpoints: RwLock::new(Vec::new()),
                kit: RwLock::new(None),
                draft: RwLock::new(Weak::new()),
                transaction: RwLock::new(Weak::new()),
            }
        }
    }

    impl Alternative {
        pub async fn compute_hash(&self) -> String {
            let name = self.name.read().await;
            h(&[self.id.as_str(), name.as_str()])
        }
    }

    #[Object(name = "Alternative")]
    impl Alternative {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<Arc<Graph>> {
            self.owner_graph.upgrade()
        }
        pub async fn name(&self) -> String {
            self.name.read().await.clone()
        }
        pub async fn start(&self) -> Arc<Checkpoint> {
            self.start.read().await.upgrade().unwrap_or_default()
        }
        pub async fn checkpoints(&self) -> Vec<Arc<Checkpoint>> {
            self.checkpoints.read().await.clone()
        }
        pub async fn store(&self) -> Arc<Kit> {
            self.kit.read().await.clone().unwrap_or_default()
        }
        pub async fn checkpoint(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.checkpoints.read().await.clone()).await
        }
        #[graphql(name = "latestWipCheckpointAncestor")]
        pub async fn latest_wip_checkpoint_ancestor(&self) -> Option<Arc<Checkpoint>> {
            self.checkpoints.read().await.last().cloned()
        }
        #[graphql(name = "savedChanges")]
        pub async fn saved_changes(&self) -> crate::gql_relay::ChangeConnection {
            match self.draft.read().await.upgrade() {
                Some(d) => crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(d.finalized_transactions.read().await.clone()).await).await,
                None => crate::gql_relay::ChangeConnection::empty(),
            }
        }
        #[graphql(name = "unsavedChanges")]
        pub async fn unsaved_changes(&self) -> crate::gql_relay::ChangeConnection {
            match self.draft.read().await.upgrade() {
                Some(d) => crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(d.transactions.read().await.clone()).await).await,
                None => crate::gql_relay::ChangeConnection::empty(),
            }
        }
        pub async fn kit(&self) -> Arc<Kit> {
            match self.owner_graph.upgrade() {
                Some(g) => match self.draft.read().await.upgrade() {
                    Some(d) => g.materialized_kit_for_draft(&d.id).await,
                    None => self.kit.read().await.clone().unwrap_or_default(),
                },
                None => self.kit.read().await.clone().unwrap_or_default(),
            }
        }
    }
    //#endregion 🌱 alternative

    //#region 🌐 graph
    /// @emoji 🔗 `Graph.owner` — target SDL `union GraphOwner = Session`.
    #[derive(Clone, Union)]
    pub enum GraphOwner {
        Session(Arc<Session>),
    }

    /// @emoji 📦 Cached materialized [`Kit`] for a draft revision (`change_seq`).
    pub struct MaterializedSlot {
        pub draft_id: Id,
        pub change_seq: u64,
        pub kit: Arc<Kit>,
    }

    pub struct Graph {
        pub id: Id,
        pub owner_session: RwLock<Weak<Session>>,
        pub self_weak: std::sync::Mutex<std::sync::Weak<Graph>>,
        pub initial_kit: RwLock<Arc<Kit>>,
        /// @emoji 🏗️ Mutable kit root replayed through [`Graph::materialized_kit_for_draft`].
        pub parent_root_for_active_draft: RwLock<Arc<Kit>>,
        pub materialized_cache: RwLock<Option<MaterializedSlot>>,
        pub alternatives: RwLock<Vec<Arc<Alternative>>>,
        pub checkpoints: RwLock<Vec<Arc<Checkpoint>>>,
        pub releases: RwLock<Vec<Arc<Checkpoint>>>,
        pub drafts: RwLock<Vec<Arc<Draft>>>,
        pub op_history: RwLock<Vec<Arc<crate::operation::OperationIface>>>,
    }

    impl Default for Graph {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_session: RwLock::new(Weak::new()),
                self_weak: std::sync::Mutex::new(Weak::new()),
                initial_kit: RwLock::new(Arc::default()),
                parent_root_for_active_draft: RwLock::new(Arc::default()),
                materialized_cache: RwLock::new(None),
                alternatives: RwLock::new(Vec::new()),
                checkpoints: RwLock::new(Vec::new()),
                releases: RwLock::new(Vec::new()),
                drafts: RwLock::new(Vec::new()),
                op_history: RwLock::new(Vec::new()),
            }
        }
    }

    impl Graph {
        /// 🆕 Build a brand-new Graph; seeds [`Graph::parent_root_for_active_draft`] from a deep-cloned empty [`Kit`] so checkpoint roots never alias live mutation.
        pub async fn new() -> Arc<Self> {
            let id = Id::new().await;
            let g = Arc::new_cyclic(|weak_self: &Weak<Graph>| {
                let kit = crate::kit::Kit::new_sync(weak_self.clone(), "the kit".to_string());
                Self {
                    id,
                    owner_session: RwLock::new(Weak::new()),
                    self_weak: std::sync::Mutex::new(weak_self.clone()),
                    initial_kit: RwLock::new(Arc::default()),
                    parent_root_for_active_draft: RwLock::new(kit.clone()),
                    materialized_cache: RwLock::new(None),
                    alternatives: RwLock::new(Vec::new()),
                    checkpoints: RwLock::new(Vec::new()),
                    releases: RwLock::new(Vec::new()),
                    drafts: RwLock::new(Vec::new()),
                    op_history: RwLock::new(Vec::new()),
                }
            });
            let baseline = g.parent_root_for_active_draft.read().await.clone().deep_clone().await;
            *g.initial_kit.write().await = baseline.clone();
            *g.parent_root_for_active_draft.write().await = baseline;
            g
        }

        /// @emoji 🔗 Upgrade `&Graph` to [`Arc`] via the cyclic weak slot (panics if weak is unset).
        pub fn arc_here(&self) -> Arc<Graph> {
            self.self_weak.lock().ok().and_then(|slot| slot.upgrade()).expect("Graph.self_weak upgrade")
        }

        /// @emoji 📦 Materialized kit for the default seed draft (GraphQL `theKit` / node resolution).
        pub async fn materialized_head_kit(self: &Arc<Self>) -> Arc<Kit> {
            let d = self.ensure_default_seed_state().await;
            self.materialized_kit_for_draft(&d.id).await
        }

        /// @emoji 📦 Same as [`Graph::materialized_head_kit`] but callable from `&Graph` resolvers.
        pub async fn materialized_head_kit_from_ref(&self) -> Arc<Kit> {
            self.arc_here().materialized_head_kit().await
        }

        /// 🛰️ WIP bootstrap for `@semio/js` WASM: hydrates [`Graph::parent_root_for_active_draft`] then re-seeds a deep-cloned immutable parent line.
        pub async fn new_overlay_from_kit_json(dto_json: serde_json::Value) -> Result<Arc<Self>, SemioError> {
            let g = Self::new().await;
            {
                let mut slot = g.parent_root_for_active_draft.write().await;
                slot.hydrate_from_kit_full_snapshot_json(&dto_json).await?;
                if let Some(c) = dto_json.get("createdAt").and_then(|v| v.as_str()) {
                    *slot.created.write().await = Some(crate::timestamp::Timestamp(c.to_string()));
                }
                if let Some(u) = dto_json.get("updatedAt").and_then(|v| v.as_str()) {
                    *slot.updated.write().await = Some(crate::timestamp::Timestamp(u.to_string()));
                }
                let cloned = slot.deep_clone().await;
                *slot = cloned;
            }
            {
                let ini = g.parent_root_for_active_draft.read().await.deep_clone().await;
                *g.initial_kit.write().await = ini;
            }
            Ok(g)
        }

        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }

        /// @emoji 🧊 Invalidate lazily materialized kit cache (abort / record operation).
        pub async fn invalidate_materialized_cache(self: &Arc<Self>) {
            *self.materialized_cache.write().await = None;
        }

        /// @emoji 🧾 Saved changes on the main kit version (`finalized_transactions` only).
        pub async fn saved_change_connection_for_main_line(self: &Arc<Self>) -> crate::gql_relay::ChangeConnection {
            let draft = self.ensure_default_seed_state().await;
            let txs = draft.finalized_transactions.read().await.clone();
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(txs).await).await
        }

        /// @emoji 🧾 Unsaved changes on the main kit version, projected from open internal write sessions.
        pub async fn unsaved_change_connection_for_main_line(self: &Arc<Self>) -> crate::gql_relay::ChangeConnection {
            let draft = self.ensure_default_seed_state().await;
            let transactions = {
                let guard = draft.transactions.read().await;
                guard.clone()
            };
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(transactions).await).await
        }

        /// @emoji 📦 Deterministic materialized [`Kit`] for `draft_id`: clone [`Graph::parent_root_for_active_draft`] and replay recorded [`KitOperation`] forwards.
        pub async fn materialized_kit_for_draft(self: &Arc<Self>, draft_id: &Id) -> Arc<Kit> {
            let draft = self.ensure_draft(draft_id).await;
            let seq = draft.change_seq.load(Ordering::Relaxed);
            {
                let cache = self.materialized_cache.read().await;
                if let Some(slot) = cache.as_ref() {
                    if slot.draft_id == *draft_id && slot.change_seq == seq {
                        return slot.kit.clone();
                    }
                }
            }
            let base = self.parent_root_for_active_draft.read().await.clone();
            let mat = base.deep_clone().await;
            let mut edits: Vec<Arc<Edit>> = Vec::new();
            edits.extend(draft.finalized_transactions.read().await.clone());
            edits.extend(draft.transactions.read().await.clone());
            for ed in edits {
                let changes = ed.changes.read().await.clone();
                for ch in changes {
                    let forwards = ch.forwards.read().await.clone();
                    for op in forwards {
                        let diff = match op.to_diff(&mat).await {
                            Ok(d) => d,
                            Err(_) => continue,
                        };
                        if mat.apply_diff(&diff).await.is_err() {
                            continue;
                        }
                    }
                }
            }
            *self.materialized_cache.write().await = Some(MaterializedSlot { draft_id: draft_id.clone(), change_seq: seq, kit: mat.clone() });
            mat
        }

        /// @emoji 📝 Append one forward operation plus backward operations onto the open transaction's tail [`Change`], bumping draft `change_seq`.
        pub async fn record_op_in_open_transaction(self: &Arc<Self>, draft_id: &Id, transaction_id: &Id, forward: crate::operation::KitOperation, backwards: Vec<crate::operation::KitOperation>) -> Result<(), SemioError> {
            let draft = self.ensure_draft(draft_id).await;
            let _ = draft.ensure_transaction(transaction_id).await;
            let tx = draft.transactions.read().await.iter().find(|t| &t.id == transaction_id).cloned().ok_or_else(|| SemioError::not_found("Edit", transaction_id.as_str()))?;
            let change = {
                let mut chs = tx.changes.write().await;
                if let Some(last) = chs.last() {
                    last.clone()
                } else {
                    let c = Change::new().await;
                    chs.push(c.clone());
                    c
                }
            };
            *change.parent_edit.write().await = Arc::downgrade(&tx);
            let lane_owner = if let Some(alt) = draft.owner_alternative.upgrade() {
                Some(ChangeOwnerRef::Alternative(Arc::downgrade(&alt)))
            } else if let Some(cp) = draft.parent_checkpoint.read().await.upgrade() {
                Some(ChangeOwnerRef::Checkpoint(Arc::downgrade(&cp)))
            } else {
                None
            };
            *change.owner.write().await = lane_owner;
            change.forwards.write().await.push(forward);
            change.backwards.write().await.extend(backwards);
            draft.change_seq.fetch_add(1, Ordering::Relaxed);
            self.invalidate_materialized_cache().await;
            Ok(())
        }

        /// @emoji 🌱 Ensure the graph has a seed `Checkpoint` on the main spine and a default `Draft`
        /// hanging off it. Idempotent: if either already exists it's reused. Returns the active default draft.
        /// Sketchpad calls this through `Mutation.kitStoreInitializeDefaults` when a fresh dev kit (json file)
        /// is mounted so the on-disk bundle immediately exposes "root + first checkpoint + first draft".
        pub async fn ensure_default_seed_state(self: &Arc<Self>) -> Arc<Draft> {
            let checkpoint = {
                let cps = self.checkpoints.read().await;
                if let Some(c) = cps.first().cloned() {
                    c
                } else {
                    drop(cps);
                    let id = Id::new().await;
                    let cp = Arc::new(Checkpoint {
                        id,
                        timestamp: RwLock::new(None),
                        authors: RwLock::new(Vec::new()),
                        owner_graph: Arc::downgrade(self),
                        parent_checkpoint: RwLock::new(Weak::new()),
                        message: RwLock::new(Some("init".to_string())),
                        is_release: RwLock::new(false),
                        change_count: RwLock::new(0),
                    });
                    self.checkpoints.write().await.push(cp.clone());
                    cp
                }
            };
            let draft = {
                let drafts = self.drafts.read().await;
                if let Some(d) = drafts.first().cloned() {
                    d
                } else {
                    drop(drafts);
                    let d = Draft::new().await;
                    *d.parent_checkpoint.write().await = Arc::downgrade(&checkpoint);
                    self.drafts.write().await.push(d.clone());
                    d
                }
            };
            draft
        }

        /// @emoji 🌱 Fork a new named alternative from the current draft tip checkpoint (`source` `None` = main kit line).
        pub async fn create_alternative_from_tip(self: &Arc<Self>, name: String, source_alternative_id: Option<&Id>) -> Result<Id, SemioError> {
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(SemioError::invalid("alternative name required"));
            }

            self.ensure_default_seed_state().await;

            let source_draft = match source_alternative_id {
                None => self.drafts.read().await.iter().find(|d| d.owner_alternative.upgrade().is_none()).cloned().ok_or_else(|| SemioError::invalid("no main-line draft"))?,
                Some(aid) => {
                    let alt = {
                        let alts = self.alternatives.read().await;
                        alts.iter().find(|a| &a.id == aid).ok_or_else(|| SemioError::not_found("Alternative", aid.as_str()))?.clone()
                    };
                    let draft_slot = alt.draft.read().await;
                    draft_slot.upgrade().ok_or_else(|| SemioError::invalid("alternative has no draft"))?
                }
            };

            let parent_cp = source_draft.parent_checkpoint.read().await.upgrade().ok_or_else(|| SemioError::invalid("draft has no parent checkpoint"))?;

            let parent_root = self.initial_kit.read().await.clone();

            let new_alt_id = Id::new().await;
            let new_alt = Arc::new(Alternative {
                id: new_alt_id.clone(),
                owner_graph: Arc::downgrade(self),
                name: RwLock::new(name),
                start: RwLock::new(Arc::downgrade(&parent_cp)),
                checkpoints: RwLock::new(vec![parent_cp.clone()]),
                kit: RwLock::new(Some(parent_root)),
                draft: RwLock::new(Weak::new()),
                transaction: RwLock::new(Weak::new()),
            });

            let new_draft = Arc::new(Draft {
                id: Id::new().await,
                owner_alternative: Arc::downgrade(&new_alt),
                parent_checkpoint: RwLock::new(Arc::downgrade(&parent_cp)),
                target_alternative: RwLock::new(Weak::new()),
                open_transaction: RwLock::new(Weak::new()),
                finalized_transactions: RwLock::new(Vec::new()),
                redo_transactions: RwLock::new(Vec::new()),
                transactions: RwLock::new(Vec::new()),
                change_seq: AtomicU64::new(0),
            });

            *new_alt.draft.write().await = Arc::downgrade(&new_draft);

            self.alternatives.write().await.push(new_alt);
            self.drafts.write().await.push(new_draft);

            Ok(new_alt_id)
        }

        pub async fn ensure_draft(self: &Arc<Self>, draft_id: &Id) -> Arc<Draft> {
            if let Some(d) = self.drafts.read().await.iter().find(|d| &d.id == draft_id).cloned() {
                return d;
            }
            let d = Draft::with_id(draft_id.clone()).await;
            self.drafts.write().await.push(d.clone());
            d
        }

        /// @emoji 🟢 Open a brand-new transaction inside `draft_id` (draft is created on demand) and mark it as the draft's open transaction.
        pub async fn open_transaction(self: &Arc<Self>, draft_id: &Id) -> Arc<Edit> {
            let draft = self.ensure_draft(draft_id).await;
            let tx_id = Id::new().await;
            let seq = draft.transactions.read().await.len() as i32;
            let tx = Edit::with_id(Arc::downgrade(&draft), tx_id, seq).await;
            draft.transactions.write().await.push(tx.clone());
            *draft.open_transaction.write().await = Arc::downgrade(&tx);
            tx
        }

        /// @emoji ✅ Mark a transaction as finalized: moved from `transactions` into `finalized_transactions`; clears the draft's open pointer if it matched.
        pub async fn commit_transaction(self: &Arc<Self>, draft_id: &Id, transaction_id: &Id) -> Result<(), SemioError> {
            let draft = self.drafts.read().await.iter().find(|d| &d.id == draft_id).cloned().ok_or_else(|| SemioError::not_found("Draft", draft_id.as_str()))?;
            let tx = {
                let mut txs = draft.transactions.write().await;
                let pos = txs.iter().position(|t| &t.id == transaction_id).ok_or_else(|| SemioError::not_found("Edit", transaction_id.as_str()))?;
                txs.remove(pos)
            };
            draft.finalized_transactions.write().await.push(tx);
            // 🧹 Clear the draft's open pointer if it referred to the just-committed transaction.
            let open = draft.open_transaction.read().await.upgrade();
            if let Some(open_tx) = open {
                if &open_tx.id == transaction_id {
                    *draft.open_transaction.write().await = std::sync::Weak::new();
                }
            }
            Ok(())
        }

        /// @emoji ⛔ Drop a transaction from a draft entirely; clears the draft's open pointer if it matched.
        pub async fn abort_transaction(self: &Arc<Self>, draft_id: &Id, transaction_id: &Id) -> Result<(), SemioError> {
            let draft = self.drafts.read().await.iter().find(|d| &d.id == draft_id).cloned().ok_or_else(|| SemioError::not_found("Draft", draft_id.as_str()))?;
            {
                let mut txs = draft.transactions.write().await;
                let pos = txs.iter().position(|t| &t.id == transaction_id).ok_or_else(|| SemioError::not_found("Edit", transaction_id.as_str()))?;
                txs.remove(pos);
            }
            let open = draft.open_transaction.read().await.upgrade();
            if let Some(open_tx) = open {
                if &open_tx.id == transaction_id {
                    *draft.open_transaction.write().await = std::sync::Weak::new();
                }
            }
            draft.change_seq.fetch_add(1, Ordering::Relaxed);
            self.invalidate_materialized_cache().await;
            Ok(())
        }

        /// @emoji 📍 Materialized [`Kit`] at any readable `wip` anchor (main line, checkpoint root, alternative draft tip, explicit draft).
        pub async fn materialized_kit_at_point(self: &Arc<Self>, p: KitReadPointInput) -> Result<Arc<Kit>, SemioError> {
            if p.the_kit == Some(true) {
                return Ok(self.materialized_head_kit().await);
            }
            if let Some(cid) = p.checkpoint_id.clone() {
                let _ = (p.checkpoint_change_id.clone(), p.checkpoint_operation_id.clone());
                let cps = self.checkpoints.read().await;
                let _cp = cps.iter().find(|c| c.id == cid).cloned().ok_or_else(|| SemioError::not_found("Checkpoint", cid.as_str()))?;
                return Ok(self.initial_kit.read().await.clone());
            }
            if let Some(aid) = p.alternative_id.clone() {
                let alts = self.alternatives.read().await;
                let alt = alts.iter().find(|a| a.id == aid).cloned().ok_or_else(|| SemioError::not_found("Alternative", aid.as_str()))?;
                let draft = alt.draft.read().await.upgrade().ok_or_else(|| SemioError::invalid("alternative has no draft"))?;
                let _ = (p.draft_transaction_id.clone(), p.draft_operation_id.clone(), p.draft_change_id.clone());
                return Ok(self.materialized_kit_for_draft(&draft.id).await);
            }
            if let Some(did) = p.draft_id.clone() {
                let draft = self.drafts.read().await.iter().find(|d| d.id == did).cloned().ok_or_else(|| SemioError::not_found("Draft", did.as_str()))?;
                if let Some(exp_alt) = p.draft_alternative_id.clone() {
                    let owner = draft.owner_alternative.upgrade();
                    let oid = owner.map(|a| a.id.clone());
                    if oid.as_ref() != Some(&exp_alt) {
                        return Err(SemioError::invalid("draft alternative mismatch"));
                    }
                }
                let _ = (p.draft_transaction_id.clone(), p.draft_operation_id.clone(), p.draft_change_id.clone());
                return Ok(self.materialized_kit_for_draft(&draft.id).await);
            }
            Ok(self.materialized_head_kit().await)
        }

        /// @emoji 🔧 Apply `createFixedPiece` via [`Graph::record_op_in_open_transaction`] (tests / golden replay).
        pub async fn apply_create_fixed_piece(
            self: &Arc<Self>,
            draft_id: Id,
            transaction_id: Id,
            design_id: Id,
            blueprint_id: Id,
            position: crate::geom::Position,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<(Arc<crate::kit::design::piece::Piece>,), SemioError> {
            let piece_id = Id::new().await;
            let forward = crate::operation::KitOperation::CreateFixedPiece {
                scope: crate::operation::Scope::CreateFixedPiece {
                    design_id,
                    piece_id: piece_id.clone(),
                    blueprint_id,
                    attribute_ids: Vec::new(),
                },
                input: crate::operation::Input::FixedPiece { position, name, description },
            };
            let before = self.materialized_kit_for_draft(&draft_id).await;
            let backwards = forward.to_backwards(&before).await?;
            self.record_op_in_open_transaction(&draft_id, &transaction_id, forward, backwards).await?;
            let after = self.materialized_kit_for_draft(&draft_id).await;
            let piece = after
                .design_by_external_id(&design_id)
                .await
                .ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?
                .piece_by_external_id(&piece_id)
                .await
                .ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
            Ok((piece,))
        }
    }

    #[Object(name = "Graph")]
    impl Graph {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> GraphOwner {
            let g = self.owner_session.read().await;
            match g.upgrade() {
                Some(s) => GraphOwner::Session(s),
                None => GraphOwner::Session(Arc::new(Session::default())),
            }
        }
        #[graphql(name = "theKit")]
        pub async fn the_kit(&self) -> crate::gql::interfaces::VersionIface {
            crate::gql::interfaces::VersionIface::TheKit(TheKit::new(Arc::downgrade(&self.arc_here())))
        }
        #[graphql(name = "initialKit")]
        pub async fn initial_kit(&self) -> Option<Arc<Kit>> {
            Some(self.initial_kit.read().await.clone())
        }
        pub async fn alternative(&self, id: Id) -> Option<Arc<Alternative>> {
            self.alternatives.read().await.iter().find(|a| a.id == id).cloned()
        }
        pub async fn alternatives(&self) -> crate::gql_relay::AlternativeConnection {
            crate::gql_relay::AlternativeConnection::from_alternatives(self.alternatives.read().await.clone()).await
        }
        pub async fn checkpoint(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.checkpoints.read().await.iter().find(|c| c.id == id).cloned()
        }
        pub async fn checkpoints(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.checkpoints.read().await.clone()).await
        }
        pub async fn release(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.releases.read().await.iter().find(|c| c.id == id).cloned()
        }
        pub async fn releases(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.releases.read().await.clone()).await
        }
    }
    //#endregion 🌐 graph

    //#region 👤 session
    pub struct Session {
        pub id: Id,
        pub started_at: RwLock<Option<Timestamp>>,
        pub drafts: RwLock<Vec<Arc<Draft>>>,
    }

    impl Default for Session {
        fn default() -> Self {
            Self { id: Id::default(), started_at: RwLock::new(None), drafts: RwLock::new(Vec::new()) }
        }
    }

    impl Session {
        pub async fn new() -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, ..Default::default() })
        }
    }

    #[Object(name = "Session")]
    impl Session {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        #[graphql(name = "startedAt")]
        pub async fn started_at(&self) -> Option<Timestamp> {
            self.started_at.read().await.clone()
        }
    }
    //#endregion 👤 session

    //#region ⚠️ conflict
    pub struct Conflict {
        pub id: Id,
        pub backbone_tip: RwLock<Option<String>>,
        pub reason: RwLock<String>,
        pub created_at: RwLock<Timestamp>,
    }

    impl Default for Conflict {
        fn default() -> Self {
            Self { id: Id::default(), backbone_tip: RwLock::new(None), reason: RwLock::new(String::new()), created_at: RwLock::new(Timestamp::default()) }
        }
    }

    impl Conflict {
        /// @emoji 🪪 Merkle leaf: id, optional backbone tip, reason, created-at (relay + GraphQL `Conflict.hash`).
        pub async fn compute_hash(&self) -> String {
            let tip = self.backbone_tip.read().await.clone().unwrap_or_default();
            let reason = self.reason.read().await.clone();
            let created = self.created_at.read().await.clone();
            merkle_node_str(&["semio:vcs:Conflict", self.id.as_str(), tip.as_str(), reason.as_str(), created.0.as_str()], Vec::new())
        }
    }

    #[Object(name = "Conflict")]
    impl Conflict {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        #[graphql(name = "backboneTip")]
        pub async fn backbone_tip(&self) -> Option<String> {
            self.backbone_tip.read().await.clone()
        }
        pub async fn reason(&self) -> String {
            self.reason.read().await.clone()
        }
        #[graphql(name = "createdAt")]
        pub async fn created_at(&self) -> Timestamp {
            self.created_at.read().await.clone()
        }
    }
    //#endregion ⚠️ conflict
}
//#endregion 🌿 vcs

//#region 🧷 iface

/// 🧷 Cross-cutting GraphQL `OwnerEntity` / `OwnedEntity` unions and empty Relay shells (expanded as more entities register).
pub mod iface {
    use std::sync::Arc;

    use async_graphql::{Object, SimpleObject, Union};

    use crate::geom::entity::{CoordinateNode, LocationNode, OffsetNode, PlaceNode, PlaneNode, PointNode, PositionNode, VectorNode};
    use crate::hash::merkle_collection;
    use crate::id::Id;
    use crate::kit::design::piece::Piece;
    use crate::kit::design::Design;
    use crate::kit::Kit;
    use crate::vcs::{Alternative, Checkpoint, Conflict, Graph, ReadVersion, Session, WriteVersion};

    /// @emoji 🔗 SDL `OwnerEntity` subset (grow toward full target union).
    #[derive(Clone, Union)]
    pub enum OwnerEntity {
        Kit(Arc<Kit>),
        Type(std::sync::Arc<crate::kit::r#type::Type>),
        Representation(std::sync::Arc<crate::kit::r#type::Representation>),
        Connector(std::sync::Arc<crate::kit::r#type::Connector>),
        Design(Arc<Design>),
        Piece(Arc<Piece>),
        Graph(Arc<Graph>),
        Session(Arc<Session>),
        Checkpoint(Arc<Checkpoint>),
        Alternative(Arc<Alternative>),
        Conflict(Arc<Conflict>),
        ReadVersion(Arc<ReadVersion>),
        WriteVersion(Arc<WriteVersion>),
        Position(Arc<PositionNode>),
        Coordinate(Arc<CoordinateNode>),
        Plane(Arc<PlaneNode>),
        Point(Arc<PointNode>),
        Vector(Arc<VectorNode>),
        Place(Arc<PlaceNode>),
        Offset(Arc<OffsetNode>),
        Location(Arc<LocationNode>),
    }

    /// @emoji 🔗 SDL `OwnedEntity` subset for non-empty `ownedEntities` edges.
    #[derive(Clone, Union)]
    pub enum OwnedEntity {
        Kit(Arc<Kit>),
        Design(Arc<Design>),
        Piece(Arc<Piece>),
        Position(Arc<PositionNode>),
    }

    #[derive(Clone, SimpleObject)]
    pub struct OwnedEntityEdge {
        pub cursor: String,
        pub node: OwnedEntity,
    }

    #[derive(Clone, SimpleObject)]
    pub struct OwnedEntityConnection {
        pub edges: Vec<OwnedEntityEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: crate::gql_relay::PageInfo,
        pub hash: String,
    }

    impl OwnedEntityConnection {
        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: crate::gql_relay::PageInfo::default(), hash: merkle_collection(Vec::new()) }
        }
    }

    /// 🏷️ Wrap an [`OwnerEntity`] in `Arc` (resolver convenience).
    pub fn owner_entity_arc(e: OwnerEntity) -> Arc<OwnerEntity> {
        Arc::new(e)
    }

    /// 🏷️ Map an `Option<OwnerEntity>` resolver value into the `Arc<OwnerEntity>` shape.
    pub fn owner_entity_arc_opt(o: Option<OwnerEntity>) -> Option<Arc<OwnerEntity>> {
        o.map(Arc::new)
    }

    /// 🏷️ Empty `OwnedEntityConnection` shell (used by entities with no owned children yet).
    pub fn empty_owned_entity_connection() -> Arc<OwnedEntityConnection> {
        Arc::new(OwnedEntityConnection::empty())
    }

    /// @emoji 🌐 Global `node` / `entity` resolution union (Relay `Node` stand-in until full `Entity` interface wiring).
    #[derive(Clone, Union)]
    pub enum GqlNode {
        Graph(Arc<Graph>),
        Kit(Arc<Kit>),
        Design(Arc<Design>),
        Piece(Arc<Piece>),
        Session(Arc<Session>),
        Conflict(Arc<Conflict>),
        Tag(std::sync::Arc<crate::meta::Tag>),
        Concept(std::sync::Arc<crate::meta::Concept>),
        Quality(std::sync::Arc<crate::meta::Quality>),
    }

    /// @emoji 🔎 Resolve a global id against WIP + authoritative graphs, sessions, and conflicts.
    pub async fn resolve_node(rt: &crate::worker::ParentRuntime, id: &Id) -> Option<GqlNode> {
        for g in [&rt.wip_graph, &rt.auth_graph] {
            if &g.id == id {
                return Some(GqlNode::Graph(g.clone()));
            }
            let kit = g.parent_root_for_active_draft.read().await.clone();
            let kid = kit.workspace_kit_id().await;
            if id == &kid || id == &kit.id {
                return Some(GqlNode::Kit(kit.clone()));
            }
            if let Some(t) = kit.find_tag(id).await {
                return Some(GqlNode::Tag(t));
            }
            if let Some(c) = kit.find_concept(id).await {
                return Some(GqlNode::Concept(c));
            }
            if let Some(q) = kit.find_quality(id).await {
                return Some(GqlNode::Quality(q));
            }
            let designs = kit.designs.read().await;
            for d in designs.iter() {
                if &d.id == id {
                    return Some(GqlNode::Design(d.clone()));
                }
                let pieces = d.pieces.read().await;
                for p in pieces.iter() {
                    if &p.id == id {
                        return Some(GqlNode::Piece(p.clone()));
                    }
                }
            }
        }
        let sessions = rt.sessions.read().await;
        for s in sessions.iter() {
            if &s.id == id {
                return Some(GqlNode::Session(s.clone()));
            }
        }
        let conflicts = rt.conflicts.read().await;
        for c in conflicts.iter() {
            if &c.id == id {
                return Some(GqlNode::Conflict(c.clone()));
            }
        }
        None
    }

    /// @emoji 📍 Resolve `pieceInDesign` on the WIP graph line.
    pub async fn piece_in_design_on_wip(rt: &crate::worker::ParentRuntime, design_id: &Id, piece_id: &Id) -> Option<Arc<Piece>> {
        let g = &rt.wip_graph;
        let kit = g.parent_root_for_active_draft.read().await.clone();
        let des = kit.design_by_external_id(design_id).await?;
        des.piece_by_external_id(piece_id).await
    }

    /// @emoji 📍 `alternativePieceKind` stub (returns `None` until alternative graph model is wired).
    pub async fn alternative_piece_kind(_rt: &crate::worker::ParentRuntime, _piece_id: &Id) -> Option<String> {
        None
    }

    /// @emoji 📍 WeakEntity + entity shell for [`CoordinateNode`] (SDL `Coordinate`).
    #[Object(name = "Coordinate")]
    impl CoordinateNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn u(&self) -> f64 {
            *self.u.read().await
        }
        pub async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    #[Object(name = "Vector")]
    impl VectorNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn x(&self) -> f64 {
            *self.x.read().await
        }
        pub async fn y(&self) -> f64 {
            *self.y.read().await
        }
        pub async fn z(&self) -> f64 {
            *self.z.read().await
        }
    }

    #[Object(name = "Point")]
    impl PointNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn x(&self) -> f64 {
            *self.x.read().await
        }
        pub async fn y(&self) -> f64 {
            *self.y.read().await
        }
        pub async fn z(&self) -> f64 {
            *self.z.read().await
        }
    }

    #[Object(name = "Plane")]
    impl PlaneNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn origin(&self) -> Arc<PointNode> {
            self.origin.clone()
        }
        #[graphql(name = "xAxis")]
        pub async fn x_axis(&self) -> Arc<VectorNode> {
            self.x_axis.clone()
        }
        #[graphql(name = "yAxis")]
        pub async fn y_axis(&self) -> Arc<VectorNode> {
            self.y_axis.clone()
        }
    }

    #[Object(name = "Position")]
    impl PositionNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn center(&self) -> Arc<CoordinateNode> {
            self.center.clone()
        }
        pub async fn plane(&self) -> Arc<PlaneNode> {
            self.plane.clone()
        }
    }

    #[Object(name = "Offset")]
    impl OffsetNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn u(&self) -> f64 {
            *self.u.read().await
        }
        pub async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    /// @emoji 🌍 WeakEntity shell for [`LocationNode`] (SDL `Location`).
    #[Object(name = "Location")]
    impl LocationNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn longitude(&self) -> f64 {
            *self.longitude.read().await
        }
        pub async fn latitude(&self) -> f64 {
            *self.latitude.read().await
        }
        pub async fn altitude(&self) -> f64 {
            *self.altitude.read().await
        }
    }

    #[Object(name = "Place")]
    impl PlaceNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
}

//#endregion 🧷 iface

//#region ⚙️ operation

pub mod operation {
    //! ⚙️ Operation entities and their inputs. Operations carry `Arc<Entity>` payloads so the
    //! event bus broadcasts shared references, never deep-copied entity data.
    use std::sync::{Arc, Weak};

    use async_graphql::{InputObject, Interface, Object, OneofObject, Union};
    use serde::{Deserialize, Serialize};

    use crate::error::SemioError;
    use crate::geom::{Offset, Position};
    use crate::id::Id;
    use crate::iface::{empty_owned_entity_connection, OwnedEntityConnection, OwnerEntity};
    use crate::meta::{ConceptInput, QualityInput, TagInput};
    use crate::vcs::Edit;

    /// 🏷️ Hand union for `Operation.owner` (every operation is owned by an `Edit`).
    #[derive(Clone, Union)]
    pub enum OperationOwner {
        Edit(Arc<Edit>),
    }

    impl Default for OperationOwner {
        fn default() -> Self {
            Self::Edit(Arc::default())
        }
    }

    //#region 🧭 normalized operation contract
    //#region 🔖 canonical_kit_diff
    /// @emoji 📦 `Id` reference wrapper matching `{ "id": "…" }` rows in kit diff JSON.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct IdRef {
        pub id: Id,
    }

    /// @emoji 📦 Sparse `tags` triple (`metabolism.kit.diff.semio.json`).
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct TagsCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<TagModifiedRow>,
        pub added: Vec<serde_json::Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct TagModifiedRow {
        pub tag: IdRef,
        pub diff: TagPatch,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct TagPatch {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
    }

    /// @emoji 📦 Sparse `concepts` triple.
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct ConceptsCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<ConceptModifiedRow>,
        pub added: Vec<serde_json::Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ConceptModifiedRow {
        pub concept: IdRef,
        pub diff: ConceptPatch,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct ConceptPatch {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
    }

    /// @emoji 📦 Sparse `qualities` triple (kit-level).
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct QualitiesCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<QualityModifiedRow>,
        pub added: Vec<serde_json::Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct QualityModifiedRow {
        pub quality: IdRef,
        pub diff: QualityPatch,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct QualityPatch {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
    }

    /// @emoji 📦 Canonical sparse kit diff (camelCase) aligned with [`semio/assets/semio/metabolism.kit.diff.semio.json`]; `types` / `designs` / `files` / `folders` / `authors` keep raw JSON subtrees for full golden round-trip until every subtree has a typed apply path.
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase", default)]
    pub struct CanonicalKitDiff {
        pub name: Option<String>,
        pub version: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub remote: Option<String>,
        pub homepage: Option<String>,
        pub license: Option<String>,
        pub preview: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub types: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub designs: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<TagsCollectionDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub concepts: Option<ConceptsCollectionDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qualities: Option<QualitiesCollectionDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub files: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub folders: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub authors: Option<serde_json::Value>,
    }

    /// @emoji 📦 Persisted / replayed kit transition: canonical [`CanonicalKitDiff`] only (no `__ops` envelope).
    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    #[serde(transparent)]
    pub struct KitDiff(pub CanonicalKitDiff);

    impl KitDiff {
        /// @emoji 🔗 Shallow-merge `other` scalar/collection fields into `self.0` (used when coalescing multi-target diffs).
        pub fn absorb(&mut self, other: CanonicalKitDiff) {
            let a = &mut self.0;
            let b = other;
            macro_rules! opt {
                ($field:ident) => {
                    if b.$field.is_some() {
                        a.$field = b.$field;
                    }
                };
            }
            opt!(name);
            opt!(version);
            opt!(description);
            opt!(icon);
            opt!(image);
            opt!(remote);
            opt!(homepage);
            opt!(license);
            opt!(preview);
            opt!(types);
            opt!(designs);
            opt!(tags);
            opt!(concepts);
            opt!(qualities);
            opt!(files);
            opt!(folders);
            opt!(authors);
        }
    }
    //#endregion 🔖 canonical_kit_diff

    /// @emoji 🚫 Empty payload marker for scope-only operations (serde shape).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct NoInput;

    /// @emoji 🪪 Kit root scope; the kit id is implicit from the target graph line.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RenameKitScope;

    /// @emoji 🪪 Single entity scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct EntityScope {
        pub entity_id: Id,
    }

    /// @emoji 🪪 Single tag scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TagScope {
        pub tag_id: Id,
    }

    /// @emoji 🪪 Multi-tag scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TagsScope {
        pub tag_ids: Vec<Id>,
    }

    /// @emoji 🪪 Single concept scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ConceptScope {
        pub concept_id: Id,
    }

    /// @emoji 🪪 Single quality scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct QualityScope {
        pub quality_id: Id,
    }

    /// @emoji 🪪 Create-tag scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateTagScope {
        pub owner_id: Id,
        pub tag_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Batch create-tag scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateTagsScope {
        pub owner_id: Id,
        pub tag_ids: Vec<Id>,
        pub attribute_ids: Vec<Vec<Id>>,
    }

    /// @emoji 🪪 Create-concept scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateConceptScope {
        pub owner_id: Id,
        pub concept_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Create-quality scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateQualityScope {
        pub owner_id: Id,
        pub quality_id: Id,
        pub attribute_ids: Vec<Id>,
        pub benchmark_ids: Vec<Id>,
    }

    /// @emoji 🪪 Create-piece scope with the parent design id and the pre-minted piece id.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateFixedPieceScope {
        pub design_id: Id,
        pub piece_id: Id,
        pub blueprint_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Single piece-in-design scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PieceInDesignScope {
        pub design_id: Id,
        pub piece_id: Id,
    }

    /// @emoji 🪪 Multi-piece-in-design scope.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PiecesInDesignScope {
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
    }

    /// @emoji ✏️ Rename-kit payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RenameKitInput {
        pub name: String,
    }

    /// @emoji ✏️ Rename-tag payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RenameTagInput {
        pub name: String,
    }

    /// @emoji ✏️ Generic description payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangeDescriptionInput {
        pub description: Option<String>,
    }

    /// @emoji ✏️ Generic icon payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangeIconInput {
        pub icon: Option<String>,
    }

    /// @emoji ✏️ Generic image payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangeImageInput {
        pub image: Option<String>,
    }

    /// @emoji ✏️ Batch tag payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateTagsInput {
        pub tags: Vec<TagInput>,
    }

    /// @emoji ✏️ Fixed-piece creation payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreateFixedPieceInput {
        pub position: Position,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    /// @emoji ✏️ Piece drag payload.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct DragPieceInput {
        pub offset: Offset,
    }

    /// @emoji 🧭 Shared scope payload: every distinct id-shape used across [`KitOperation`] commands.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Scope {
        Kit,
        Entity { entity_id: Id },
        Tag { tag_id: Id },
        Tags { tag_ids: Vec<Id> },
        Concept { concept_id: Id },
        Quality { quality_id: Id },
        CreateTag { owner_id: Id, tag_id: Id, attribute_ids: Vec<Id> },
        CreateTags { owner_id: Id, tag_ids: Vec<Id>, attribute_ids: Vec<Vec<Id>> },
        CreateConcept { owner_id: Id, concept_id: Id, attribute_ids: Vec<Id> },
        CreateQuality { owner_id: Id, quality_id: Id, attribute_ids: Vec<Id>, benchmark_ids: Vec<Id> },
        CreateFixedPiece { design_id: Id, piece_id: Id, blueprint_id: Id, attribute_ids: Vec<Id> },
        PieceInDesign { design_id: Id, piece_id: Id },
        PiecesInDesign { design_id: Id, piece_ids: Vec<Id> },
    }

    /// @emoji 🧭 Shared non-id input payload reused across commands with the same shape.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Input {
        None,
        Name { name: String },
        Description { description: Option<String> },
        Icon { icon: Option<String> },
        Image { image: Option<String> },
        Tag { tag: TagInput },
        Tags { tags: Vec<TagInput> },
        Concept { concept: ConceptInput },
        Quality { quality: QualityInput },
        FixedPiece { position: Position, name: Option<String>, description: Option<String> },
        Offset { offset: Offset },
    }

    /// @emoji 🧩 Normalized  operation surface: every variant is `{ scope: Scope, input: Input }`.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum KitOperation {
        RenameKit { scope: Scope, input: Input },
        ChangeDescription { scope: Scope, input: Input },
        ChangeIcon { scope: Scope, input: Input },
        ChangeImage { scope: Scope, input: Input },
        CreateTag { scope: Scope, input: Input },
        CreateTags { scope: Scope, input: Input },
        DeleteTag { scope: Scope, input: Input },
        DeleteTags { scope: Scope, input: Input },
        RenameTag { scope: Scope, input: Input },
        CreateConcept { scope: Scope, input: Input },
        DeleteConcept { scope: Scope, input: Input },
        CreateQuality { scope: Scope, input: Input },
        DeleteQuality { scope: Scope, input: Input },
        CreateFixedPiece { scope: Scope, input: Input },
        DeletePieceInDesign { scope: Scope, input: Input },
        DragPieceInDesign { scope: Scope, input: Input },
        DragPiecesInDesign { scope: Scope, input: Input },
        FixPieceInDesign { scope: Scope, input: Input },
    }

    impl KitOperation {
        pub fn kind(&self) -> &'static str {
            match self {
                KitOperation::RenameKit { .. } => "renameKit",
                KitOperation::ChangeDescription { .. } => "changeDescription",
                KitOperation::ChangeIcon { .. } => "changeIcon",
                KitOperation::ChangeImage { .. } => "changeImage",
                KitOperation::CreateTag { .. } => "createTag",
                KitOperation::CreateTags { .. } => "createTags",
                KitOperation::DeleteTag { .. } => "deleteTag",
                KitOperation::DeleteTags { .. } => "deleteTags",
                KitOperation::RenameTag { .. } => "renameTag",
                KitOperation::CreateConcept { .. } => "createConcept",
                KitOperation::DeleteConcept { .. } => "deleteConcept",
                KitOperation::CreateQuality { .. } => "createQuality",
                KitOperation::DeleteQuality { .. } => "deleteQuality",
                KitOperation::CreateFixedPiece { .. } => "createFixedPiece",
                KitOperation::DeletePieceInDesign { .. } => "deletePieceInDesign",
                KitOperation::DragPieceInDesign { .. } => "dragPieceInDesign",
                KitOperation::DragPiecesInDesign { .. } => "dragPiecesInDesign",
                KitOperation::FixPieceInDesign { .. } => "fixPieceInDesign",
            }
        }

        pub fn payload_json(&self) -> Result<String, SemioError> {
            serde_json::to_string(self).map_err(|e| SemioError::invalid(e.to_string()))
        }

        pub fn from_kind_and_payload(kind: &str, payload_json: &str) -> Result<Self, SemioError> {
            let operation: KitOperation = serde_json::from_str(payload_json).map_err(|e| SemioError::invalid(e.to_string()))?;
            if operation.kind() != kind {
                return Err(SemioError::invalid(format!("operation kind mismatch: expected `{kind}`, got `{}`", operation.kind())));
            }
            Ok(operation)
        }

        /// Pure: read pre-state and produce a structural diff without mutating the kit.
        pub async fn to_diff(&self, kit: &Arc<crate::kit::Kit>) -> Result<KitDiff, SemioError> {
            match self {
                KitOperation::RenameKit { scope, input } => {
                    let Scope::Kit = scope else {
                        return Err(SemioError::invalid("renameKit expects Scope::Kit"));
                    };
                    let Input::Name { name } = input else {
                        return Err(SemioError::invalid("renameKit expects Input::Name"));
                    };
                    if name.chars().count() > 256 {
                        return Err(SemioError::invalid(format!("Kit name too long: {} > 256", name.chars().count())));
                    }
                    Ok(KitDiff(CanonicalKitDiff { name: Some(name.clone()), ..Default::default() }))
                }
                KitOperation::ChangeDescription { scope, input } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeDescription expects Scope::Entity"));
                    };
                    let Input::Description { description } = input else {
                        return Err(SemioError::invalid("changeDescription expects Input::Description"));
                    };
                    entity_description(kit, entity_id).await?;
                    let kid = kit.workspace_kit_id().await;
                    if entity_id == &kid || entity_id == &kit.id {
                        return Ok(KitDiff(CanonicalKitDiff { description: description.clone(), ..Default::default() }));
                    }
                    if kit.find_tag(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            tags: Some(TagsCollectionDiff { modified: vec![TagModifiedRow { tag: IdRef { id: entity_id.clone() }, diff: TagPatch { description: description.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.find_concept(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            concepts: Some(ConceptsCollectionDiff {
                                modified: vec![ConceptModifiedRow { concept: IdRef { id: entity_id.clone() }, diff: ConceptPatch { description: description.clone(), ..Default::default() } }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                    if kit.find_quality(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            qualities: Some(QualitiesCollectionDiff {
                                modified: vec![QualityModifiedRow { quality: IdRef { id: entity_id.clone() }, diff: QualityPatch { description: description.clone(), ..Default::default() } }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                    if kit.type_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            types: Some(serde_json::json!({
                                "modified": [{ "type": { "id": entity_id }, "diff": { "description": description } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(serde_json::json!({
                                "modified": [{ "design": { "id": entity_id }, "diff": { "description": description } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("DescriptionEntity", entity_id.as_str()))
                }
                KitOperation::ChangeIcon { scope, input } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeIcon expects Scope::Entity"));
                    };
                    let Input::Icon { icon } = input else {
                        return Err(SemioError::invalid("changeIcon expects Input::Icon"));
                    };
                    entity_icon(kit, entity_id).await?;
                    let kid = kit.workspace_kit_id().await;
                    if entity_id == &kid || entity_id == &kit.id {
                        return Ok(KitDiff(CanonicalKitDiff { icon: icon.clone(), ..Default::default() }));
                    }
                    if kit.find_tag(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            tags: Some(TagsCollectionDiff { modified: vec![TagModifiedRow { tag: IdRef { id: entity_id.clone() }, diff: TagPatch { icon: icon.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.find_concept(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            concepts: Some(ConceptsCollectionDiff { modified: vec![ConceptModifiedRow { concept: IdRef { id: entity_id.clone() }, diff: ConceptPatch { icon: icon.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.find_quality(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            qualities: Some(QualitiesCollectionDiff { modified: vec![QualityModifiedRow { quality: IdRef { id: entity_id.clone() }, diff: QualityPatch { icon: icon.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.type_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            types: Some(serde_json::json!({
                                "modified": [{ "type": { "id": entity_id }, "diff": { "icon": icon } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(serde_json::json!({
                                "modified": [{ "design": { "id": entity_id }, "diff": { "icon": icon } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("IconEntity", entity_id.as_str()))
                }
                KitOperation::ChangeImage { scope, input } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeImage expects Scope::Entity"));
                    };
                    let Input::Image { image } = input else {
                        return Err(SemioError::invalid("changeImage expects Input::Image"));
                    };
                    entity_image(kit, entity_id).await?;
                    let kid = kit.workspace_kit_id().await;
                    if entity_id == &kid || entity_id == &kit.id {
                        return Ok(KitDiff(CanonicalKitDiff { image: image.clone(), ..Default::default() }));
                    }
                    if kit.type_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            types: Some(serde_json::json!({
                                "modified": [{ "type": { "id": entity_id }, "diff": { "image": image } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(serde_json::json!({
                                "modified": [{ "design": { "id": entity_id }, "diff": { "image": image } }]
                            })),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("ImageEntity", entity_id.as_str()))
                }
                KitOperation::CreateTag { scope, input } => {
                    let Scope::CreateTag { owner_id, tag_id, attribute_ids } = scope else {
                        return Err(SemioError::invalid("createTag expects Scope::CreateTag"));
                    };
                    let Input::Tag { tag } = input else {
                        return Err(SemioError::invalid("createTag expects Input::Tag"));
                    };
                    kit.resolve_tag_owner_slot(owner_id).await?;
                    if kit.find_tag(tag_id).await.is_some() {
                        return Err(SemioError::invalid(format!("Tag already exists: {}", tag_id.as_str())));
                    }
                    validate_attribute_ids(tag.attributes.as_ref().map(|items| items.len()).unwrap_or_default(), attribute_ids)?;
                    let mut tv = serde_json::to_value(tag).map_err(|e| SemioError::invalid(e.to_string()))?;
                    if let serde_json::Value::Object(ref mut m) = tv {
                        m.insert("id".to_string(), serde_json::json!(tag_id.as_str()));
                        m.insert("ownerId".to_string(), serde_json::json!(owner_id.as_str()));
                        m.insert("attributeIds".to_string(), serde_json::to_value(attribute_ids).map_err(|e| SemioError::invalid(e.to_string()))?);
                    }
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { added: vec![tv], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::CreateTags { scope, input } => {
                    let Scope::CreateTags { owner_id, tag_ids, attribute_ids } = scope else {
                        return Err(SemioError::invalid("createTags expects Scope::CreateTags"));
                    };
                    let Input::Tags { tags } = input else {
                        return Err(SemioError::invalid("createTags expects Input::Tags"));
                    };
                    if tag_ids.len() != tags.len() || attribute_ids.len() != tags.len() {
                        return Err(SemioError::invalid("createTags scope length mismatch"));
                    }
                    kit.resolve_tag_owner_slot(owner_id).await?;
                    let mut added = Vec::new();
                    for (index, tag) in tags.iter().enumerate() {
                        if kit.find_tag(&tag_ids[index]).await.is_some() {
                            return Err(SemioError::invalid(format!("Tag already exists: {}", tag_ids[index].as_str())));
                        }
                        validate_attribute_ids(tag.attributes.as_ref().map(|items| items.len()).unwrap_or_default(), &attribute_ids[index])?;
                        let mut tv = serde_json::to_value(tag).map_err(|e| SemioError::invalid(e.to_string()))?;
                        if let serde_json::Value::Object(ref mut m) = tv {
                            m.insert("id".to_string(), serde_json::json!(tag_ids[index].as_str()));
                            m.insert("ownerId".to_string(), serde_json::json!(owner_id.as_str()));
                            m.insert("attributeIds".to_string(), serde_json::to_value(&attribute_ids[index]).map_err(|e| SemioError::invalid(e.to_string()))?);
                        }
                        added.push(tv);
                    }
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { added, ..Default::default() }), ..Default::default() }))
                }
                KitOperation::DeleteTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("deleteTag expects Scope::Tag"));
                    };
                    ensure_tag(kit, tag_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { removed: vec![IdRef { id: tag_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::DeleteTags { scope, .. } => {
                    let Scope::Tags { tag_ids } = scope else {
                        return Err(SemioError::invalid("deleteTags expects Scope::Tags"));
                    };
                    let mut removed = Vec::new();
                    for tag_id in tag_ids {
                        ensure_tag(kit, tag_id).await?;
                        removed.push(IdRef { id: (*tag_id).clone() });
                    }
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { removed, ..Default::default() }), ..Default::default() }))
                }
                KitOperation::RenameTag { scope, input } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("renameTag expects Scope::Tag"));
                    };
                    let Input::Name { name } = input else {
                        return Err(SemioError::invalid("renameTag expects Input::Name"));
                    };
                    ensure_tag(kit, tag_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        tags: Some(TagsCollectionDiff { modified: vec![TagModifiedRow { tag: IdRef { id: tag_id.clone() }, diff: TagPatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }),
                        ..Default::default()
                    }))
                }
                KitOperation::CreateConcept { scope, input } => {
                    let Scope::CreateConcept { owner_id, concept_id, attribute_ids } = scope else {
                        return Err(SemioError::invalid("createConcept expects Scope::CreateConcept"));
                    };
                    let Input::Concept { concept } = input else {
                        return Err(SemioError::invalid("createConcept expects Input::Concept"));
                    };
                    kit.resolve_concept_owner_slot(owner_id).await?;
                    if kit.find_concept(concept_id).await.is_some() {
                        return Err(SemioError::invalid(format!("Concept already exists: {}", concept_id.as_str())));
                    }
                    validate_attribute_ids(concept.attributes.as_ref().map(|items| items.len()).unwrap_or_default(), attribute_ids)?;
                    let mut cv = serde_json::to_value(concept).map_err(|e| SemioError::invalid(e.to_string()))?;
                    if let serde_json::Value::Object(ref mut m) = cv {
                        m.insert("id".to_string(), serde_json::json!(concept_id.as_str()));
                        m.insert("ownerId".to_string(), serde_json::json!(owner_id.as_str()));
                        m.insert("attributeIds".to_string(), serde_json::to_value(attribute_ids).map_err(|e| SemioError::invalid(e.to_string()))?);
                    }
                    Ok(KitDiff(CanonicalKitDiff { concepts: Some(ConceptsCollectionDiff { added: vec![cv], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::DeleteConcept { scope, .. } => {
                    let Scope::Concept { concept_id } = scope else {
                        return Err(SemioError::invalid("deleteConcept expects Scope::Concept"));
                    };
                    ensure_concept(kit, concept_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { concepts: Some(ConceptsCollectionDiff { removed: vec![IdRef { id: concept_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::CreateQuality { scope, input } => {
                    let Scope::CreateQuality { owner_id, quality_id, attribute_ids, benchmark_ids } = scope else {
                        return Err(SemioError::invalid("createQuality expects Scope::CreateQuality"));
                    };
                    let Input::Quality { quality } = input else {
                        return Err(SemioError::invalid("createQuality expects Input::Quality"));
                    };
                    kit.resolve_quality_owner_slot(owner_id).await?;
                    if kit.find_quality(quality_id).await.is_some() {
                        return Err(SemioError::invalid(format!("Quality already exists: {}", quality_id.as_str())));
                    }
                    validate_attribute_ids(quality.attributes.as_ref().map(|items| items.len()).unwrap_or_default(), attribute_ids)?;
                    if !benchmark_ids.is_empty() {
                        return Err(SemioError::invalid("quality benchmark ids are not supported yet"));
                    }
                    let mut qv = serde_json::to_value(quality).map_err(|e| SemioError::invalid(e.to_string()))?;
                    if let serde_json::Value::Object(ref mut m) = qv {
                        m.insert("id".to_string(), serde_json::json!(quality_id.as_str()));
                        m.insert("ownerId".to_string(), serde_json::json!(owner_id.as_str()));
                        m.insert("attributeIds".to_string(), serde_json::to_value(attribute_ids).map_err(|e| SemioError::invalid(e.to_string()))?);
                        m.insert("benchmarkIds".to_string(), serde_json::to_value(benchmark_ids).map_err(|e| SemioError::invalid(e.to_string()))?);
                    }
                    Ok(KitDiff(CanonicalKitDiff { qualities: Some(QualitiesCollectionDiff { added: vec![qv], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::DeleteQuality { scope, .. } => {
                    let Scope::Quality { quality_id } = scope else {
                        return Err(SemioError::invalid("deleteQuality expects Scope::Quality"));
                    };
                    ensure_quality(kit, quality_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { qualities: Some(QualitiesCollectionDiff { removed: vec![IdRef { id: quality_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                KitOperation::CreateFixedPiece { scope, input } => {
                    let Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    let Input::FixedPiece { position, name, description } = input else {
                        return Err(SemioError::invalid("createFixedPiece expects Input::FixedPiece"));
                    };
                    if !attribute_ids.is_empty() {
                        return Err(SemioError::invalid("piece attribute ids are not supported yet"));
                    }
                    let pose = serde_json::to_value(position).map_err(|e| SemioError::invalid(e.to_string()))?;
                    let piece_json = serde_json::json!({
                        "id": piece_id.as_str(),
                        "blueprintId": blueprint_id.as_str(),
                        "name": name,
                        "description": description,
                        "scale": 1.0,
                        "props": [],
                        "attributes": [],
                        "pose": pose,
                    });
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(serde_json::json!({
                            "modified": [{
                                "design": { "id": design_id.as_str() },
                                "diff": { "pieces": { "added": [piece_json] } }
                            }]
                        })),
                        ..Default::default()
                    }))
                }
                KitOperation::DeletePieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("deletePieceInDesign expects Scope::PieceInDesign"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(serde_json::json!({
                            "modified": [{
                                "design": { "id": design_id.as_str() },
                                "diff": { "pieces": { "removed": [{ "id": piece_id.as_str() }] } }
                            }]
                        })),
                        ..Default::default()
                    }))
                }
                KitOperation::DragPieceInDesign { scope, input } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Scope::PieceInDesign"));
                    };
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Input::Offset"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(serde_json::json!({
                            "modified": [{
                                "design": { "id": design_id.as_str() },
                                "diff": { "pieces": { "modified": [{
                                    "piece": { "id": piece_id.as_str() },
                                    "diff": { "drag": { "u": offset.u, "v": offset.v } }
                                }] } }
                            }]
                        })),
                        ..Default::default()
                    }))
                }
                KitOperation::DragPiecesInDesign { scope, input } => {
                    let Scope::PiecesInDesign { design_id, piece_ids } = scope else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Scope::PiecesInDesign"));
                    };
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Input::Offset"));
                    };
                    let mut pm = Vec::new();
                    for piece_id in piece_ids {
                        ensure_piece(kit, design_id, piece_id).await?;
                        pm.push(serde_json::json!({
                            "piece": { "id": piece_id.as_str() },
                            "diff": { "drag": { "u": offset.u, "v": offset.v } }
                        }));
                    }
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(serde_json::json!({
                            "modified": [{
                                "design": { "id": design_id.as_str() },
                                "diff": { "pieces": { "modified": pm } }
                            }]
                        })),
                        ..Default::default()
                    }))
                }
                KitOperation::FixPieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("fixPieceInDesign expects Scope::PieceInDesign"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(serde_json::json!({
                            "modified": [{
                                "design": { "id": design_id.as_str() },
                                "diff": { "pieces": { "modified": [{
                                    "piece": { "id": piece_id.as_str() },
                                    "diff": { "fixPiece": true }
                                }] } }
                            }]
                        })),
                        ..Default::default()
                    }))
                }
            }
        }

        /// Pure: read pre-state and return the ordered list of backward operations.
        pub async fn to_backwards(&self, kit: &Arc<crate::kit::Kit>) -> Result<Vec<KitOperation>, SemioError> {
            match self {
                KitOperation::RenameKit { .. } => Ok(vec![KitOperation::RenameKit { scope: Scope::Kit, input: Input::Name { name: kit.name.read().await.clone() } }]),
                KitOperation::ChangeDescription { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeDescription expects Scope::Entity"));
                    };
                    Ok(vec![KitOperation::ChangeDescription { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Description { description: entity_description(kit, entity_id).await? } }])
                }
                KitOperation::ChangeIcon { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeIcon expects Scope::Entity"));
                    };
                    Ok(vec![KitOperation::ChangeIcon { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Icon { icon: entity_icon(kit, entity_id).await? } }])
                }
                KitOperation::ChangeImage { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeImage expects Scope::Entity"));
                    };
                    Ok(vec![KitOperation::ChangeImage { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Image { image: entity_image(kit, entity_id).await? } }])
                }
                KitOperation::CreateTag { scope, .. } => {
                    let Scope::CreateTag { tag_id, .. } = scope else {
                        return Err(SemioError::invalid("createTag expects Scope::CreateTag"));
                    };
                    Ok(vec![KitOperation::DeleteTag { scope: Scope::Tag { tag_id: tag_id.clone() }, input: Input::None }])
                }
                KitOperation::CreateTags { scope, .. } => {
                    let Scope::CreateTags { tag_ids, .. } = scope else {
                        return Err(SemioError::invalid("createTags expects Scope::CreateTags"));
                    };
                    Ok(vec![KitOperation::DeleteTags { scope: Scope::Tags { tag_ids: tag_ids.clone() }, input: Input::None }])
                }
                KitOperation::DeleteTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("deleteTag expects Scope::Tag"));
                    };
                    let tag = ensure_tag(kit, tag_id).await?;
                    let owner_id = tag_owner_id(kit, &tag).await?;
                    let attributes = tag.attributes.read().await.clone();
                    Ok(vec![KitOperation::CreateTag {
                        scope: Scope::CreateTag { owner_id, tag_id: tag.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect() },
                        input: Input::Tag { tag: tag_input_from_entity(&tag).await },
                    }])
                }
                KitOperation::DeleteTags { scope, .. } => {
                    let Scope::Tags { tag_ids } = scope else {
                        return Err(SemioError::invalid("deleteTags expects Scope::Tags"));
                    };
                    let mut tags = Vec::new();
                    let mut out_tag_ids = Vec::new();
                    let mut attribute_ids = Vec::new();
                    let mut owner_id: Option<Id> = None;
                    for tag_id in tag_ids {
                        let tag = ensure_tag(kit, tag_id).await?;
                        let current_owner_id = tag_owner_id(kit, &tag).await?;
                        if let Some(existing_owner_id) = &owner_id {
                            if existing_owner_id != &current_owner_id {
                                return Err(SemioError::invalid("deleteTags backwards requires a single owner id"));
                            }
                        } else {
                            owner_id = Some(current_owner_id);
                        }
                        let attrs = tag.attributes.read().await.clone();
                        out_tag_ids.push(tag.id.clone());
                        attribute_ids.push(attrs.iter().map(|attribute| attribute.id.clone()).collect());
                        tags.push(tag_input_from_entity(&tag).await);
                    }
                    Ok(vec![KitOperation::CreateTags { scope: Scope::CreateTags { owner_id: owner_id.unwrap_or_default(), tag_ids: out_tag_ids, attribute_ids }, input: Input::Tags { tags } }])
                }
                KitOperation::RenameTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("renameTag expects Scope::Tag"));
                    };
                    let tag = ensure_tag(kit, tag_id).await?;
                    let name = {
                        let guard = tag.name.read().await;
                        guard.clone()
                    };
                    drop(tag);
                    Ok(vec![KitOperation::RenameTag { scope: Scope::Tag { tag_id: tag_id.clone() }, input: Input::Name { name } }])
                }
                KitOperation::CreateConcept { scope, .. } => {
                    let Scope::CreateConcept { concept_id, .. } = scope else {
                        return Err(SemioError::invalid("createConcept expects Scope::CreateConcept"));
                    };
                    Ok(vec![KitOperation::DeleteConcept { scope: Scope::Concept { concept_id: concept_id.clone() }, input: Input::None }])
                }
                KitOperation::DeleteConcept { scope, .. } => {
                    let Scope::Concept { concept_id } = scope else {
                        return Err(SemioError::invalid("deleteConcept expects Scope::Concept"));
                    };
                    let concept = ensure_concept(kit, concept_id).await?;
                    let owner_id = concept_owner_id(kit, &concept).await?;
                    let attributes = concept.attributes.read().await.clone();
                    Ok(vec![KitOperation::CreateConcept {
                        scope: Scope::CreateConcept { owner_id, concept_id: concept.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect() },
                        input: Input::Concept { concept: concept_input_from_entity(&concept).await },
                    }])
                }
                KitOperation::CreateQuality { scope, .. } => {
                    let Scope::CreateQuality { quality_id, .. } = scope else {
                        return Err(SemioError::invalid("createQuality expects Scope::CreateQuality"));
                    };
                    Ok(vec![KitOperation::DeleteQuality { scope: Scope::Quality { quality_id: quality_id.clone() }, input: Input::None }])
                }
                KitOperation::DeleteQuality { scope, .. } => {
                    let Scope::Quality { quality_id } = scope else {
                        return Err(SemioError::invalid("deleteQuality expects Scope::Quality"));
                    };
                    let quality = ensure_quality(kit, quality_id).await?;
                    let owner_id = quality_owner_id(kit, &quality).await?;
                    let attributes = quality.attributes.read().await.clone();
                    let benchmarks = quality.benchmarks.read().await.clone();
                    if !benchmarks.is_empty() {
                        return Err(SemioError::invalid("deleteQuality backwards does not support benchmarks yet"));
                    }
                    Ok(vec![KitOperation::CreateQuality {
                        scope: Scope::CreateQuality { owner_id, quality_id: quality.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect(), benchmark_ids: Vec::new() },
                        input: Input::Quality { quality: quality_input_from_entity(&quality).await },
                    }])
                }
                KitOperation::CreateFixedPiece { scope, .. } => {
                    let Scope::CreateFixedPiece { design_id, piece_id, .. } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    Ok(vec![KitOperation::DeletePieceInDesign { scope: Scope::PieceInDesign { design_id: design_id.clone(), piece_id: piece_id.clone() }, input: Input::None }])
                }
                KitOperation::DeletePieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("deletePieceInDesign expects Scope::PieceInDesign"));
                    };
                    let piece = ensure_piece(kit, design_id, piece_id).await?;
                    let piece_id = piece.id.clone();
                    let blueprint_id = match &*piece.blueprint.read().await {
                        crate::kit::r#type::Blueprint::Type(ty) => ty.id.clone(),
                        crate::kit::r#type::Blueprint::Design(design) => design.id.clone(),
                    };
                    let attribute_ids = {
                        let guard = piece.attributes.read().await;
                        guard.iter().map(|attribute| attribute.id.clone()).collect()
                    };
                    let position = piece.compute_flat_position().await;
                    let name = {
                        let guard = piece.name.read().await;
                        guard.clone()
                    };
                    let description = {
                        let guard = piece.description.read().await;
                        guard.clone()
                    };
                    drop(piece);
                    Ok(vec![KitOperation::CreateFixedPiece { scope: Scope::CreateFixedPiece { design_id: design_id.clone(), piece_id, blueprint_id, attribute_ids }, input: Input::FixedPiece { position, name, description } }])
                }
                KitOperation::DragPieceInDesign { scope, input } => {
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Input::Offset"));
                    };
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Scope::PieceInDesign"));
                    };
                    Ok(vec![KitOperation::DragPieceInDesign { scope: Scope::PieceInDesign { design_id: design_id.clone(), piece_id: piece_id.clone() }, input: Input::Offset { offset: Offset { u: -offset.u, v: -offset.v } } }])
                }
                KitOperation::DragPiecesInDesign { scope, input } => {
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Input::Offset"));
                    };
                    let Scope::PiecesInDesign { design_id, piece_ids } = scope else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Scope::PiecesInDesign"));
                    };
                    Ok(vec![KitOperation::DragPiecesInDesign { scope: Scope::PiecesInDesign { design_id: design_id.clone(), piece_ids: piece_ids.clone() }, input: Input::Offset { offset: Offset { u: -offset.u, v: -offset.v } } }])
                }
                KitOperation::FixPieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("fixPieceInDesign expects Scope::PieceInDesign"));
                    };
                    let piece = ensure_piece(kit, design_id, piece_id).await?;
                    let connection_kind = {
                        let guard = piece.connection_kind.read().await;
                        *guard
                    };
                    drop(piece);
                    match connection_kind {
                        Some(crate::kit::design::piece::PieceConnectionKind::Fixed) => Ok(Vec::new()),
                        _ => Err(SemioError::invalid("fixPieceInDesign backwards is unsupported for non-fixed pre-state")),
                    }
                }
            }
        }
    }

    fn validate_attribute_ids(expected: usize, actual: &[Id]) -> Result<(), SemioError> {
        if expected != actual.len() {
            return Err(SemioError::invalid(format!("attribute id count mismatch: expected {}, got {}", expected, actual.len())));
        }
        Ok(())
    }

    async fn ensure_tag(kit: &Arc<crate::kit::Kit>, tag_id: &Id) -> Result<Arc<crate::meta::Tag>, SemioError> {
        kit.find_tag(tag_id).await.ok_or_else(|| SemioError::not_found("Tag", tag_id.as_str()))
    }

    async fn ensure_concept(kit: &Arc<crate::kit::Kit>, concept_id: &Id) -> Result<Arc<crate::meta::Concept>, SemioError> {
        kit.find_concept(concept_id).await.ok_or_else(|| SemioError::not_found("Concept", concept_id.as_str()))
    }

    async fn ensure_quality(kit: &Arc<crate::kit::Kit>, quality_id: &Id) -> Result<Arc<crate::meta::Quality>, SemioError> {
        kit.find_quality(quality_id).await.ok_or_else(|| SemioError::not_found("Quality", quality_id.as_str()))
    }

    async fn ensure_piece(kit: &Arc<crate::kit::Kit>, design_id: &Id, piece_id: &Id) -> Result<Arc<crate::kit::design::piece::Piece>, SemioError> {
        let design = kit.design_by_external_id(design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
        design.piece_by_external_id(piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))
    }

    async fn entity_description(kit: &Arc<crate::kit::Kit>, entity_id: &Id) -> Result<Option<String>, SemioError> {
        let kid = kit.workspace_kit_id().await;
        if entity_id == &kid || entity_id == &kit.id {
            return Ok(kit.description.read().await.clone());
        }
        if let Some(tag) = kit.find_tag(entity_id).await {
            return Ok(tag.description.read().await.clone());
        }
        if let Some(concept) = kit.find_concept(entity_id).await {
            return Ok(concept.description.read().await.clone());
        }
        if let Some(quality) = kit.find_quality(entity_id).await {
            return Ok(quality.description.read().await.clone());
        }
        if let Some(ty) = kit.type_by_external_id(entity_id).await {
            return Ok(Some(ty.description.read().await.clone()));
        }
        if let Some(design) = kit.design_by_external_id(entity_id).await {
            return Ok(design.description.read().await.clone());
        }
        Err(SemioError::not_found("DescriptionEntity", entity_id.as_str()))
    }

    async fn entity_icon(kit: &Arc<crate::kit::Kit>, entity_id: &Id) -> Result<Option<String>, SemioError> {
        let kid = kit.workspace_kit_id().await;
        if entity_id == &kid || entity_id == &kit.id {
            return Ok(kit.icon.read().await.clone());
        }
        if let Some(tag) = kit.find_tag(entity_id).await {
            return Ok(tag.icon.read().await.clone());
        }
        if let Some(concept) = kit.find_concept(entity_id).await {
            return Ok(concept.icon.read().await.clone());
        }
        if let Some(quality) = kit.find_quality(entity_id).await {
            return Ok(quality.icon.read().await.clone());
        }
        if let Some(ty) = kit.type_by_external_id(entity_id).await {
            return Ok(Some(ty.icon.read().await.clone()));
        }
        if let Some(design) = kit.design_by_external_id(entity_id).await {
            return Ok(design.icon.read().await.clone());
        }
        Err(SemioError::not_found("IconEntity", entity_id.as_str()))
    }

    async fn entity_image(kit: &Arc<crate::kit::Kit>, entity_id: &Id) -> Result<Option<String>, SemioError> {
        let kid = kit.workspace_kit_id().await;
        if entity_id == &kid || entity_id == &kit.id {
            return Ok(kit.image.read().await.clone());
        }
        if let Some(ty) = kit.type_by_external_id(entity_id).await {
            return Ok(Some(ty.image.read().await.clone()));
        }
        if let Some(design) = kit.design_by_external_id(entity_id).await {
            return Ok(design.image.read().await.clone());
        }
        Err(SemioError::not_found("ImageEntity", entity_id.as_str()))
    }

    pub(crate) async fn tag_owner_id(kit: &Arc<crate::kit::Kit>, tag: &Arc<crate::meta::Tag>) -> Result<Id, SemioError> {
        match &*tag.owner.read().await {
            crate::meta::TagOwnerSlot::Kit(_) => Ok(kit.workspace_kit_id().await),
            crate::meta::TagOwnerSlot::Type(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Tag owner dropped")),
            crate::meta::TagOwnerSlot::Rep(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Tag owner dropped")),
            crate::meta::TagOwnerSlot::Unset => Err(SemioError::invalid("Tag owner unset")),
        }
    }

    pub(crate) async fn concept_owner_id(kit: &Arc<crate::kit::Kit>, concept: &Arc<crate::meta::Concept>) -> Result<Id, SemioError> {
        match &*concept.owner.read().await {
            crate::meta::ConceptOwnerSlot::Kit(_) => Ok(kit.workspace_kit_id().await),
            crate::meta::ConceptOwnerSlot::Type(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Concept owner dropped")),
            crate::meta::ConceptOwnerSlot::Unset => Err(SemioError::invalid("Concept owner unset")),
        }
    }

    pub(crate) async fn quality_owner_id(kit: &Arc<crate::kit::Kit>, quality: &Arc<crate::meta::Quality>) -> Result<Id, SemioError> {
        match &*quality.owner.read().await {
            crate::meta::QualityOwnerSlot::Kit(_) => Ok(kit.workspace_kit_id().await),
            crate::meta::QualityOwnerSlot::Type(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Quality owner dropped")),
            crate::meta::QualityOwnerSlot::Rep(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Quality owner dropped")),
            crate::meta::QualityOwnerSlot::Conn(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Quality owner dropped")),
            crate::meta::QualityOwnerSlot::Design(owner) => owner.upgrade().map(|value| value.id.clone()).ok_or_else(|| SemioError::invalid("Quality owner dropped")),
            crate::meta::QualityOwnerSlot::Unset => Err(SemioError::invalid("Quality owner unset")),
        }
    }

    async fn tag_input_from_entity(tag: &Arc<crate::meta::Tag>) -> TagInput {
        TagInput {
            name: tag.name.read().await.clone(),
            description: tag.description.read().await.clone(),
            icon: tag.icon.read().await.clone(),
            order: *tag.order.read().await,
            attributes: Some(tag.attributes.read().await.iter().map(|attribute| crate::meta::AttributeInput { key: attribute.key.clone(), value: Some(attribute.value.clone()), definition: attribute.definition.clone() }).collect()),
        }
    }

    async fn concept_input_from_entity(concept: &Arc<crate::meta::Concept>) -> ConceptInput {
        ConceptInput {
            name: concept.name.read().await.clone(),
            description: concept.description.read().await.clone(),
            icon: concept.icon.read().await.clone(),
            order: *concept.order.read().await,
            attributes: Some(concept.attributes.read().await.iter().map(|attribute| crate::meta::AttributeInput { key: attribute.key.clone(), value: Some(attribute.value.clone()), definition: attribute.definition.clone() }).collect()),
        }
    }

    async fn quality_input_from_entity(quality: &Arc<crate::meta::Quality>) -> QualityInput {
        QualityInput {
            key: quality.key.read().await.clone(),
            value: quality.value.read().await.clone(),
            unit: quality.unit.read().await.clone(),
            definition: quality.definition.read().await.clone(),
            description: quality.description.read().await.clone(),
            icon: quality.icon.read().await.clone(),
            attributes: Some(quality.attributes.read().await.iter().map(|attribute| crate::meta::AttributeInput { key: attribute.key.clone(), value: Some(attribute.value.clone()), definition: attribute.definition.clone() }).collect()),
        }
    }
    //#endregion 🧭 normalized operation contract

    //#region 🧾 inputs
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreatedFixedPieceInput {
        pub design_id: Id,
        pub blueprint_id: Id,
        pub position: Position,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    #[Object(name = "CreatedFixedPieceInput")]
    impl CreatedFixedPieceInput {
        #[graphql(name = "designId")]
        pub async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "blueprintId")]
        pub async fn blueprint_id(&self) -> Id {
            self.blueprint_id.clone()
        }
        pub async fn position(&self) -> Arc<crate::geom::entity::PositionNode> {
            crate::geom::entity::PositionNode::from_position_value(self.position)
        }
        pub async fn name(&self) -> Option<String> {
            self.name.clone()
        }
        pub async fn description(&self) -> Option<String> {
            self.description.clone()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct FixedPieceInput {
        pub design_id: Id,
        pub piece_id: Id,
    }

    #[Object(name = "FixedPieceInput")]
    impl FixedPieceInput {
        #[graphql(name = "designId")]
        pub async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "pieceId")]
        pub async fn piece_id(&self) -> Id {
            self.piece_id.clone()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct DraggedPieceInput {
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
        pub offset: Offset,
    }

    #[Object(name = "DraggedPieceInput")]
    impl DraggedPieceInput {
        #[graphql(name = "designId")]
        pub async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "pieceIds")]
        pub async fn piece_ids(&self) -> Vec<Id> {
            self.piece_ids.clone()
        }
        pub async fn offset(&self) -> Arc<crate::geom::entity::OffsetNode> {
            crate::geom::entity::OffsetNode::from_value(self.offset)
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RenamedKitInput {
        pub name: String,
    }

    #[Object(name = "RenamedKitInput")]
    impl RenamedKitInput {
        pub async fn name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangedDescriptionInput {
        pub entity_id: Id,
        pub description: String,
    }

    #[Object(name = "ChangedDescriptionInput")]
    impl ChangedDescriptionInput {
        #[graphql(name = "entityId")]
        pub async fn entity_id(&self) -> Id {
            self.entity_id.clone()
        }
        pub async fn description(&self) -> String {
            self.description.clone()
        }
    }

    /// 🧾 The schema's `union OperationInput = …` (one variant resolved at a time).
    #[derive(Clone, Union)]
    #[graphql(name = "OperationInput")]
    pub enum OperationInputUnion {
        RenamedKit(RenamedKitInput),
        ChangedDescription(ChangedDescriptionInput),
        CreatedFixedPiece(CreatedFixedPieceInput),
        FixedPiece(FixedPieceInput),
        DraggedPiece(DraggedPieceInput),
    }
    //#endregion 🧾 inputs

    //#region 🧭 graph workspace + backbone store kind (readable/writable selectors)
    /// @emoji 🧭 Which materialized graph line a read or write targets (`wip` vs `authoritative`).
    #[derive(Clone, Copy, Debug, Eq, PartialEq, async_graphql::Enum)]
    #[graphql(name = "KitGraphWorkspace")]
    pub enum KitGraphWorkspace {
        #[graphql(name = "WIP")]
        Wip,
        #[graphql(name = "AUTHORITATIVE")]
        Authoritative,
    }

    /// @emoji 🗄️ Dev JSON bundle vs local `.semio/` backbone — capability surface without SQL leakage.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, async_graphql::Enum)]
    #[graphql(name = "BackboneStoreKind")]
    pub enum BackboneStoreKind {
        #[graphql(name = "DEV_JSON")]
        DevJson,
        #[graphql(name = "LOCAL_DOT_SEMIO")]
        LocalDotSemio,
    }
    //#endregion 🧭 graph workspace + backbone store kind (readable/writable selectors)

    //#region 📜  operation record (kit bundle / operation log contract)
    /// @emoji 📜 One persisted  operation: stable id, kind string, JSON payload, monotonic sequence index.
    #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
    #[graphql(name = "OpRecord")]
    pub struct OpRecord {
        pub id: Id,
        #[graphql(name = "opKind")]
        pub op_kind: String,
        #[graphql(name = "payloadJson")]
        pub payload_json: String,
        pub sequence: i32,
    }
    //#endregion 📜  operation record (kit bundle / operation log contract)

    //#region 📦 diff
    /// 📜 Ephemeral  diff payload for the kit engine; replay/apply diff carrier (not a GraphQL output type).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Diff {
        pub id: Id,
        pub summary: Option<String>,
    }
    //#endregion 📦 diff

    //#region 🪄 operations
    pub struct CreatedFixedPiece {
        pub id: Id,
        pub owner_edit: Weak<Edit>,
        pub input: CreatedFixedPieceInput,
        pub diff: Diff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl CreatedFixedPiece {
        pub async fn new(input: CreatedFixedPieceInput, piece: Arc<crate::kit::design::piece::Piece>, diff: Diff) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_edit: Weak::new(), input, diff, piece })
        }
    }

    impl Default for CreatedFixedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_edit: Weak::new(), input: CreatedFixedPieceInput::default(), diff: Diff::default(), piece: Arc::default() }
        }
    }

    #[Object(name = "CreatedFixedPiece")]
    impl CreatedFixedPiece {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> Arc<OperationOwner> {
            Arc::new(OperationOwner::Edit(self.owner_edit.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<Arc<OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<Arc<OwnedEntityConnection>> {
            Some(empty_owned_entity_connection())
        }
        pub async fn piece(&self) -> Arc<crate::kit::design::piece::Piece> {
            self.piece.clone()
        }
    }

    pub struct FixedPiece {
        pub id: Id,
        pub owner_edit: Weak<Edit>,
        pub input: FixedPieceInput,
        pub diff: Diff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl Default for FixedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_edit: Weak::new(), input: FixedPieceInput::default(), diff: Diff::default(), piece: Arc::default() }
        }
    }

    #[Object(name = "FixedPiece")]
    impl FixedPiece {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> Arc<OperationOwner> {
            Arc::new(OperationOwner::Edit(self.owner_edit.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<Arc<OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<Arc<OwnedEntityConnection>> {
            Some(empty_owned_entity_connection())
        }
        pub async fn piece(&self) -> Arc<crate::kit::design::piece::Piece> {
            self.piece.clone()
        }
    }

    pub struct DraggedPiece {
        pub id: Id,
        pub owner_edit: Weak<Edit>,
        pub input: DraggedPieceInput,
        pub diff: Diff,
        pub pieces: Vec<Arc<crate::kit::design::piece::Piece>>,
    }

    impl Default for DraggedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_edit: Weak::new(), input: DraggedPieceInput::default(), diff: Diff::default(), pieces: Vec::new() }
        }
    }

    #[Object(name = "DraggedPiece")]
    impl DraggedPiece {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> Arc<OperationOwner> {
            Arc::new(OperationOwner::Edit(self.owner_edit.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<Arc<OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<Arc<OwnedEntityConnection>> {
            Some(empty_owned_entity_connection())
        }
        pub async fn pieces(&self) -> Vec<Arc<crate::kit::design::piece::Piece>> {
            self.pieces.clone()
        }
    }

    pub struct RenamedKit {
        pub id: Id,
        /// @emoji Correlates with the `renameKit` mutation return value and `CommandReceipt.requestId`.
        pub request_id: Id,
        pub owner_edit: Weak<Edit>,
        pub input: RenamedKitInput,
        pub diff: Diff,
        pub kit: Arc<crate::kit::Kit>,
    }

    impl Default for RenamedKit {
        fn default() -> Self {
            Self { id: Id::default(), request_id: Id::default(), owner_edit: Weak::new(), input: RenamedKitInput::default(), diff: Diff::default(), kit: Arc::default() }
        }
    }

    #[Object(name = "RenamedKit")]
    impl RenamedKit {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        #[graphql(name = "requestId")]
        pub async fn request_id_field(&self) -> Id {
            self.request_id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> Arc<OperationOwner> {
            Arc::new(OperationOwner::Edit(self.owner_edit.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<Arc<OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<Arc<OwnedEntityConnection>> {
            Some(empty_owned_entity_connection())
        }
        pub async fn kit(&self) -> Arc<crate::kit::Kit> {
            self.kit.clone()
        }
    }

    pub struct ChangedDescription {
        pub id: Id,
        pub owner_edit: Weak<Edit>,
        pub input: ChangedDescriptionInput,
        pub diff: Diff,
        pub entity: Arc<crate::kit::Kit>,
    }

    impl Default for ChangedDescription {
        fn default() -> Self {
            Self { id: Id::default(), owner_edit: Weak::new(), input: ChangedDescriptionInput::default(), diff: Diff::default(), entity: Arc::default() }
        }
    }

    #[Object(name = "ChangedDescription")]
    impl ChangedDescription {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> Arc<OperationOwner> {
            Arc::new(OperationOwner::Edit(self.owner_edit.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<Arc<OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<Arc<OwnedEntityConnection>> {
            Some(empty_owned_entity_connection())
        }
        pub async fn entity(&self) -> Arc<crate::kit::Kit> {
            self.entity.clone()
        }
    }

    /// 🌗 Sum-type carrying any operation through the event bus / change log (Arc-shared).
    #[derive(Clone, Union)]
    #[graphql(name = "OperationKind")]
    pub enum OperationKind {
        CreatedFixedPiece(Arc<CreatedFixedPiece>),
        FixedPiece(Arc<FixedPiece>),
        DraggedPiece(Arc<DraggedPiece>),
        RenamedKit(Arc<RenamedKit>),
        ChangedDescription(Arc<ChangedDescription>),
    }

    /// 🪄 GraphQL `interface Operation` (each variant is an Arc-shared operation entity).
    #[derive(Clone, Interface)]
    #[graphql(
        name = "Operation",
        field(name = "id", ty = "crate::id::Id"),
        field(name = "hash", ty = "String"),
        field(name = "owner", ty = "std::sync::Arc<crate::operation::OperationOwner>"),
        field(name = "ownerEntity", method = "owner_entity", ty = "Option<std::sync::Arc<crate::iface::OwnerEntity>>"),
        field(name = "ownedEntities", method = "owned_entities", ty = "Option<std::sync::Arc<crate::iface::OwnedEntityConnection>>")
    )]
    pub enum OperationIface {
        CreatedFixedPiece(Arc<CreatedFixedPiece>),
        FixedPiece(Arc<FixedPiece>),
        DraggedPiece(Arc<DraggedPiece>),
        RenamedKit(Arc<RenamedKit>),
        ChangedDescription(Arc<ChangedDescription>),
    }

    impl Default for OperationIface {
        fn default() -> Self {
            Self::CreatedFixedPiece(Arc::new(CreatedFixedPiece::default()))
        }
    }

    impl OperationIface {
        /// @emoji 🪪 Stable row id for relay operation edges / merkle shells.
        pub fn row_id(&self) -> Id {
            match self {
                OperationIface::CreatedFixedPiece(o) => o.id.clone(),
                OperationIface::FixedPiece(o) => o.id.clone(),
                OperationIface::DraggedPiece(o) => o.id.clone(),
                OperationIface::RenamedKit(o) => o.id.clone(),
                OperationIface::ChangedDescription(o) => o.id.clone(),
            }
        }
    }

    /// 🧾 OneOf input-object surface for batched submissions.
    #[derive(Clone, Debug, OneofObject)]
    #[graphql(name = "OperationInputOneOf")]
    pub enum OperationInputOneOf {
        CreatedFixedPiece(CreatedFixedPieceInputDto),
        FixedPiece(FixedPieceInputDto),
        DraggedPiece(DraggedPieceInputDto),
        RenamedKit(RenamedKitInputDto),
        ChangedDescription(ChangedDescriptionInputDto),
    }

    #[derive(Clone, Debug, InputObject)]
    pub struct CreatedFixedPieceInputDto {
        #[graphql(name = "designId")]
        pub design_id: Id,
        #[graphql(name = "blueprintId")]
        pub blueprint_id: Id,
        pub position: Position,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, InputObject)]
    pub struct FixedPieceInputDto {
        #[graphql(name = "designId")]
        pub design_id: Id,
        #[graphql(name = "pieceId")]
        pub piece_id: Id,
    }

    #[derive(Clone, Debug, InputObject)]
    pub struct DraggedPieceInputDto {
        #[graphql(name = "designId")]
        pub design_id: Id,
        #[graphql(name = "pieceIds")]
        pub piece_ids: Vec<Id>,
        pub offset: Offset,
    }

    #[derive(Clone, Debug, InputObject)]
    pub struct RenamedKitInputDto {
        pub name: String,
    }

    #[derive(Clone, Debug, InputObject)]
    pub struct ChangedDescriptionInputDto {
        #[graphql(name = "entityId")]
        pub entity_id: Id,
        pub description: String,
    }
    //#endregion 🪄 operations

    //#region 📡 commands
    /// 📡 Internal command envelope passed parent → child runtime over the work queue.
    #[derive(Clone, Debug)]
    pub enum Command {
        ApplyKitOperation { request_id: Id, draft_id: Id, transaction_id: Id, operation: KitOperation },
        BackboneAttach { request_id: Id, connection_uri: String, store_kind: BackboneStoreKind },
        BackboneDetach { request_id: Id, connection_uri: String },
    }

    impl Command {
        pub fn request_id(&self) -> &Id {
            match self {
                Command::ApplyKitOperation { request_id, .. } => request_id,
                Command::BackboneAttach { request_id, .. } => request_id,
                Command::BackboneDetach { request_id, .. } => request_id,
            }
        }
    }

    /// ✅ Lightweight signal that a command was accepted (used by `commandSucceeded`).
    #[derive(Clone, Debug, Default, Serialize, Deserialize, async_graphql::SimpleObject)]
    #[graphql(name = "Command")]
    pub struct CommandReceipt {
        #[graphql(name = "requestId")]
        pub request_id: Id,
        pub kind: String,
    }
    //#endregion 📡 commands

    /// @emoji 🧩 Declarative operation row registration hook (`operations! { CreatedFixedPiece, … }`) — expand to typed operation structs + history wiring.
    macro_rules! operations {
        ($($row:ident),* $(,)?) => {
            /// @emoji 🔢 Row count listed in `operations! { … }` (static registry grows toward ~100 SDL operations).
            pub const GRAPH_OP_REGISTRY_ROWS: usize = [$(stringify!($row)),*].len();
        };
    }

    operations! {
        CreatedFixedPiece,
        FixedPiece,
        DraggedPiece,
        RenamedKit,
        ChangedDescription,
        CreateTag,
        CreateTags,
        RenameTag,
        UpdateTagDescription,
        UpdateTagIcon,
        AddAttributeToTag,
        AddAttributesToTag,
        RemoveAttributeFromTag,
        RemoveAttributesFromTag,
        DeleteTag,
        DeleteTags,
        CreateConcept,
        CreateConcepts,
        RenameConcept,
        UpdateConceptDescription,
        UpdateConceptIcon,
        AddAttributeToConcept,
        AddAttributesToConcept,
        RemoveAttributeFromConcept,
        RemoveAttributesFromConcept,
        DeleteConcept,
        DeleteConcepts,
        CreatePort,
        CreatePorts,
        RenamePort,
        UpdatePortDescription,
        UpdatePortIcon,
        AddAttributeToPort,
        AddAttributesToPort,
        RemoveAttributeFromPort,
        RemoveAttributesFromPort,
        DeletePort,
        DeletePorts,
        CreateQuality,
        CreateQualities,
        RenameQuality,
        UpdateQualityDescription,
        UpdateQualityIcon,
        AddAttributeToQuality,
        AddAttributesToQuality,
        RemoveAttributeFromQuality,
        RemoveAttributesFromQuality,
        DeleteQuality,
        DeleteQualities,
        CreateType,
        CreateTypes,
        RenameType,
        UpdateTypeDescription,
        UpdateTypeIcon,
        AddAttributeToType,
        AddAttributesToType,
        RemoveAttributeFromType,
        RemoveAttributesFromType,
        DeleteType,
        DeleteTypes,
        AddConnectorToType,
        AddConnectorsToType,
        RenameConnectorInType,
        UpdateConnectorDescriptionInType,
        UpdateConnectorIconInType,
        RemoveConnectorFromType,
        RemoveConnectorsFromType,
        CreateDesign,
        CreateDesigns,
        DeleteDesign,
        DeleteDesigns,
        FlattenDesign,
        AddAttributeToDesign,
        AddAttributesToDesign,
        RemoveAttributeFromDesign,
        RemoveAttributesFromDesign,
        AddFixedPieceToDesign,
        AddChildPieceWithParentConnectionToDesign,
        AddChildPiecesWithParentConnectionsToDesign,
        AddHangingChildPieceWithParentConnectionToDesign,
        AddHangingChildPiecesWithParentConnectionsToDesign,
        RenamePieceInDesign,
        UpdatePieceDescriptionInDesign,
        DragPieceInDesign,
        DragPiecesInDesign,
        MovePieceInDesign,
        MovePiecesInDesign,
        FixPieceInDesign,
        FixPiecesInDesign,
        ChangePieceToTypeInDesign,
        ChangePiecesToTypeInDesign,
        AddAttributeToPiece,
        AddAttributesToPiece,
        RemoveAttributeFromPiece,
        RemoveAttributesFromPiece,
        DeletePieceInDesign,
        DeletePiecesInDesign,
        DeletePiecesAndConnectionsInDesign
    }
}

//#endregion ⚙️ operation

//#region 🧩 kit graph engine

pub mod kit_graph_engine {
    //! 🧩 Core kit graph engine: internal handle-backed slots, deterministic ephemeral  diffs, async apply for bundle replay and multi-`Graph` states (`wip` / `authoritative`).
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use serde::Deserialize;

    use crate::error::SemioError;
    use crate::hash::h;
    use crate::id::Id;
    use crate::kit;
    use crate::operation;
    use crate::vcs::Graph;

    //#region 🧷 handles
    /// @emoji 🧷 Opaque internal design slot index; external [`Id`] maps only in [`kit::Kit::bind_external_design_id`].
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct DesignHandle(pub u32);
    //#endregion 🧷 handles

    //#region 🔢 projection fingerprint
    /// @emoji 🔢 Stable `projectionFingerprint`: blake3-style [`h`] over sorted piece centers (matches golden `kit-store.golden.expected`).
    pub async fn projection_fingerprint_for_kit(kit: &kit::Kit) -> String {
        let designs = kit.designs.read().await;
        let mut by_design: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        for d in designs.iter() {
            let mut pts: Vec<(f64, f64)> = Vec::new();
            for p in d.pieces.read().await.iter() {
                if let Some(node) = p.position.read().await.as_ref() {
                    let pos = node.snapshot_value().await;
                    pts.push((pos.center.u, pos.center.v));
                }
            }
            pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            by_design.insert(d.id.as_str().to_string(), pts);
        }
        let flattened: Vec<String> = by_design
            .into_iter()
            .map(|(id, pts)| {
                let inner = pts.iter().map(|(u, v)| format!("{u:.9}:{v:.9}")).collect::<Vec<_>>().join(";");
                format!("{id}@{inner}")
            })
            .collect();
        h(&[flattened.join("|")])
    }
    //#endregion 🔢 projection fingerprint

    //#region 📦  diff
    /// @emoji 📦 Deterministic non-persisted diff from operation kind + payload + projection fingerprint transition.
    pub fn deterministic__diff(op_kind: &str, payload_json: &str, projection_fp_before: &str, projection_fp_after: &str) -> operation::Diff {
        let digest = h(&[op_kind, payload_json, projection_fp_before, projection_fp_after]);
        operation::Diff { id: Id::from(format!("semio:diff:{digest}")), summary: Some(digest) }
    }
    //#endregion 📦  diff

    //#region 🪡  operation apply
    /// @emoji 🧾 Output of [`apply__operation_json`]: ephemeral diff + optional created entities.
    pub struct AppliedOp {
        pub diff: operation::Diff,
        pub created_piece: Option<Arc<kit::design::piece::Piece>>,
    }

    #[derive(Debug, Deserialize, serde::Serialize)]
    struct CreatedFixedPiecePayload {
        #[serde(rename = "designId")]
        design_id: String,
        #[serde(rename = "blueprintId")]
        blueprint_id: String,
        position: crate::geom::Position,
        name: Option<String>,
        description: Option<String>,
    }

    /// @emoji 🪡 Async apply for one persisted  operation (`kind` + JSON payload); supports both legacy fixture payloads and normalized scoped operations.
    pub async fn apply__operation_json(graph: &Arc<Graph>, draft_id: &Id, transaction_id: &Id, op_kind: &str, payload_json: &str) -> Result<AppliedOp, SemioError> {
        match op_kind {
            "createdFixedPiece" => {
                let payload: CreatedFixedPiecePayload = serde_json::from_str(payload_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                let input_ser = serde_json::to_string(&payload).map_err(|e| SemioError::invalid(e.to_string()))?;
                let design_id = Id::from(payload.design_id.as_str());
                let blueprint_id = Id::from(payload.blueprint_id.as_str());
                let piece_id = Id::new().await;
                let forward = operation::KitOperation::CreateFixedPiece {
                    scope: operation::Scope::CreateFixedPiece { design_id: design_id.clone(), piece_id: piece_id.clone(), blueprint_id, attribute_ids: Vec::new() },
                    input: operation::Input::FixedPiece { position: payload.position, name: payload.name, description: payload.description },
                };
                let before = graph.materialized_kit_for_draft(draft_id).await;
                let backwards = forward.to_backwards(&before).await?;
                graph.record_op_in_open_transaction(draft_id, transaction_id, forward, backwards).await?;
                let after = graph.materialized_kit_for_draft(draft_id).await;
                let design = after.design_by_external_id(&design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
                let piece = design.piece_by_external_id(&piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
                let fp_before = projection_fingerprint_for_kit(before.as_ref()).await;
                let fp_after = projection_fingerprint_for_kit(after.as_ref()).await;
                let diff = deterministic__diff("createdFixedPiece", &input_ser, &fp_before, &fp_after);
                Ok(AppliedOp { diff, created_piece: Some(piece) })
            }
            "createFixedPiece" => {
                let operation = operation::KitOperation::from_kind_and_payload(op_kind, payload_json)?;
                let (design_id, piece_id) = match &operation {
                    operation::KitOperation::CreateFixedPiece { scope, .. } => match scope {
                        operation::Scope::CreateFixedPiece { design_id, piece_id, .. } => (design_id.clone(), piece_id.clone()),
                        _ => return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece")),
                    },
                    _ => return Err(SemioError::invalid("createFixedPiece payload did not decode to CreateFixedPiece")),
                };
                let before = graph.materialized_kit_for_draft(draft_id).await;
                let backwards = operation.to_backwards(&before).await?;
                graph.record_op_in_open_transaction(draft_id, transaction_id, operation.clone(), backwards).await?;
                let after = graph.materialized_kit_for_draft(draft_id).await;
                let design = after.design_by_external_id(&design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
                let piece = design.piece_by_external_id(&piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
                let fp_before = projection_fingerprint_for_kit(before.as_ref()).await;
                let fp_after = projection_fingerprint_for_kit(after.as_ref()).await;
                let diff = deterministic__diff("createFixedPiece", payload_json, &fp_before, &fp_after);
                Ok(AppliedOp { diff, created_piece: Some(piece) })
            }
            other => Err(SemioError::invalid(format!("unsupported  operation kind `{other}`"))),
        }
    }
    //#endregion 🪡  operation apply
}

//#endregion 🧩 kit graph engine

//#region 🗄️ kit backbone persistence (native)

pub mod kit_backbone {
    //! @emoji 🗄️ Dev JSON + local `.semio/` kit backbones: atomic single-file writes, multi-db SQLite + blobs dir, replay via [`kit_graph_engine::apply__operation_json`].
    //! 🌐 The bundle wire format (`KitStoreBundleFile` + DTOs + `from_graph` / `hydrate_into_graph`) is wasm-compatible —
    //! sketchpad's WASM runtime serializes / hydrates the metabolism-shaped JSON directly. The SQLite + filesystem-IO parts
    //! (atomic writes, `DevJsonAttached`, `LocalAttached`) are native-only and gated below.

    use std::sync::Arc;

    #[cfg(not(target_arch = "wasm32"))]
    use std::path::{Path, PathBuf};

    #[cfg(not(target_arch = "wasm32"))]
    use rusqlite::Connection;

    use crate::error::SemioError;
    use crate::id::Id;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::operation::BackboneStoreKind;
    use crate::vcs::Graph;

    //#region 🧾 wire format

    /// @emoji 🪪 On-disk schema marker stamped at the bundle root; matches `semio/assets/semio/metabolism.new.kit.semio.json`.
    pub const KIT_STORE_BUNDLE_SCHEMA: &str = "🎆26🌙06⬆️1";

    /// @emoji 🧾 Blake3 hex (empty-input digest) used on the wire until per-row merkle is filled.
    pub const KIT_BUNDLE_HASH_STUB: &str = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
    /// @emoji 🧾 ISO timestamp used when a checkpoint has no persisted time yet.
    pub const KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB: &str = "2020-01-01T00:00:00.000Z";

    /// @emoji 📎 Resolve kit snapshot collection slices whether serialized as a legacy JSON array or a `{ hash, items }` block (`metabolism.new.kit.semio.json`).
    pub(crate) fn json_array_or_block_items_ref(v: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
        match v {
            serde_json::Value::Array(a) => Some(a),
            serde_json::Value::Object(o) => o.get("items").and_then(|x| x.as_array()),
            _ => None,
        }
    }

    /// @emoji 📎 Mutable slice for hydrate / blob merge paths that must accept block lists or legacy arrays.
    pub(crate) fn json_array_or_block_items_mut(v: &mut serde_json::Value) -> Option<&mut Vec<serde_json::Value>> {
        match v {
            serde_json::Value::Array(a) => Some(a),
            serde_json::Value::Object(o) => o.get_mut("items").and_then(|x| x.as_array_mut()),
            _ => None,
        }
    }

    /// @emoji 📜 `{hash, items: [T]}` envelope — the universal "block-hashed list" reused in every nested collection of the bundle.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct BlockHashedListDto<T> {
        pub hash: String,
        pub items: Vec<T>,
    }

    impl<T> Default for BlockHashedListDto<T> {
        fn default() -> Self {
            Self { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: Vec::new() }
        }
    }

    /// @emoji 🔗 `{id, hash}` typed reference to another node in the bundle (authors, qualities, ports, families, …).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct HashRefDto {
        pub id: String,
        pub hash: String,
    }

    /// @emoji 📦 Top-level on-disk kit store bundle (mirrors `metabolism.new.kit.semio.json`: `schema / wip / authoritative / stage / conflicts / blobs`; each graph snapshot holds kit seed JSON under `initialKit`).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct KitStoreBundleFile {
        pub schema: String,
        pub wip: GraphSnapshotDto,
        pub authoritative: GraphSnapshotDto,
        pub stage: GraphSnapshotDto,
        #[serde(default)]
        pub conflicts: BlockHashedListDto<serde_json::Value>,
        #[serde(default)]
        pub blobs: BlockHashedListDto<serde_json::Value>,
    }

    /// @emoji 🌐 One graph snapshot (head pointer used as `wip` / `authoritative` / `stage` heads in the bundle).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct GraphSnapshotDto {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub authors: BlockHashedListDto<HashRefDto>,
        /// @emoji 📦 Wire key `initialKit` — persisted kit seed for this snapshot (GraphQL `Graph.theKit` is the live materialization, not this JSON name).
        #[serde(rename = "initialKit", default = "empty_root_value")]
        pub root: serde_json::Value,
        #[serde(rename = "theKit", default)]
        pub the_kit: TheKitVersionDto,
        #[serde(default)]
        pub checkpoints: BlockHashedListDto<serde_json::Value>,
        #[serde(default)]
        pub alternatives: BlockHashedListDto<AlternativeVersionDto>,
    }

    /// @emoji 🧭 Main kit version row; version-scoped changes live here, not on the graph snapshot.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct TheKitVersionDto {
        pub id: String,
        pub hash: String,
        #[serde(rename = "savedChanges", default)]
        pub saved_changes: BlockHashedListDto<VersionChangeDto>,
        #[serde(rename = "unsavedChanges", default)]
        pub unsaved_changes: BlockHashedListDto<VersionChangeDto>,
    }

    impl Default for TheKitVersionDto {
        fn default() -> Self {
            Self { id: "the-kit".to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), saved_changes: BlockHashedListDto::default(), unsaved_changes: BlockHashedListDto::default() }
        }
    }

    /// @emoji 🌿 Alternative version row; each alternative owns its own version-scoped changes.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct AlternativeVersionDto {
        pub id: String,
        pub hash: String,
        pub name: String,
        #[serde(rename = "savedChanges", default)]
        pub saved_changes: BlockHashedListDto<VersionChangeDto>,
        #[serde(rename = "unsavedChanges", default)]
        pub unsaved_changes: BlockHashedListDto<VersionChangeDto>,
    }

    /// @emoji 🧾 Version change record containing ordered edits directly on `the kit` or an alternative.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct VersionChangeDto {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub edits: BlockHashedListDto<VersionEditDto>,
        #[serde(rename = "startedAt")]
        pub started_at: String,
        #[serde(rename = "savedAt", default, skip_serializing_if = "Option::is_none")]
        pub saved_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
    }

    /// @emoji ✏️ Version edit record with forward and backward  operation steps.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct VersionEditDto {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub forwards: BlockHashedListDto<OperationStepDto>,
        #[serde(default)]
        pub backwards: BlockHashedListDto<OperationStepDto>,
        #[serde(rename = "sequenceNumber")]
        pub sequence_number: i32,
        #[serde(rename = "startedAt")]
        pub started_at: String,
        #[serde(rename = "finishedAt", default, skip_serializing_if = "Option::is_none")]
        pub finished_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
    }

    /// @emoji 🪡 One  operation step inside an edit.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct OperationStepDto {
        pub id: String,
        pub hash: String,
        pub kind: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default)]
        pub input: serde_json::Value,
    }

    /// @emoji 🌱 Empty `root` projection placeholder used until [`Kit`] dumps a real metabolism-shaped root.
    fn empty_root_value() -> serde_json::Value {
        serde_json::json!({
            "hash": KIT_BUNDLE_HASH_STUB,
            "name": "",
            "types": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
            "designs": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
        })
    }

    impl GraphSnapshotDto {
        /// @emoji 🌱 Empty graph snapshot stamped with `kit_id` (used for fresh `wip` / `authoritative` / `stage` heads).
        pub fn empty(kit_id: &str) -> Self {
            Self {
                id: kit_id.to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                authors: BlockHashedListDto::default(),
                root: empty_root_value(),
                the_kit: TheKitVersionDto::default(),
                checkpoints: BlockHashedListDto::default(),
                alternatives: BlockHashedListDto::default(),
            }
        }
    }

    impl KitStoreBundleFile {
        /// @emoji 🌱 Fresh empty bundle stamped with [`KIT_STORE_BUNDLE_SCHEMA`]; kit ids fill in once the live kit projects into `root`.
        pub fn template() -> Self {
            Self {
                schema: KIT_STORE_BUNDLE_SCHEMA.to_string(),
                wip: GraphSnapshotDto::empty(""),
                authoritative: GraphSnapshotDto::empty(""),
                stage: GraphSnapshotDto::empty(""),
                conflicts: BlockHashedListDto::default(),
                blobs: BlockHashedListDto::default(),
            }
        }

        /// @emoji 🔁 Flatten every recorded `wip` version edit into ordered [`StoredOperation`] records ready for replay.
        pub fn wip__ops(&self) -> Vec<StoredOperation> {
            let mut out = Vec::new();
            Self::push__ops_from_version_changes(&mut out, self.wip.the_kit.saved_changes.items.iter().chain(self.wip.the_kit.unsaved_changes.items.iter()), "the-kit");
            for alternative in &self.wip.alternatives.items {
                Self::push__ops_from_version_changes(&mut out, alternative.saved_changes.items.iter().chain(alternative.unsaved_changes.items.iter()), alternative.id.as_str());
            }
            out
        }

        fn push__ops_from_version_changes<'a>(out: &mut Vec<StoredOperation>, changes: impl Iterator<Item = &'a VersionChangeDto>, fallback_draft_id: &str) {
            for change in changes {
                let draft_id = change.origin.clone().unwrap_or_else(|| fallback_draft_id.to_string());
                for edit in &change.edits.items {
                    for step in &edit.forwards.items {
                        out.push(StoredOperation { draft_id: draft_id.clone(), transaction_id: change.id.clone(), kind: step.kind.clone(), input: step.input.clone() });
                    }
                }
            }
        }

        /// @emoji 📸 Project one live  change into a bundle edit.
        async fn edit_dto_from_runtime_change(ch: &Arc<crate::vcs::Change>, sequence_number: i32) -> VersionEditDto {
            let mut forward_items: Vec<OperationStepDto> = Vec::new();
            let mut backward_items: Vec<OperationStepDto> = Vec::new();
            for operation in ch.forwards.read().await.iter() {
                forward_items.push(OperationStepDto {
                    id: Id::new().await.as_str().to_string(),
                    hash: KIT_BUNDLE_HASH_STUB.to_string(),
                    kind: operation.kind().to_string(),
                    description: None,
                    input: serde_json::to_value(operation).unwrap_or_else(|_| serde_json::json!({})),
                });
            }
            for operation in ch.backwards.read().await.iter() {
                backward_items.push(OperationStepDto {
                    id: Id::new().await.as_str().to_string(),
                    hash: KIT_BUNDLE_HASH_STUB.to_string(),
                    kind: operation.kind().to_string(),
                    description: None,
                    input: serde_json::to_value(operation).unwrap_or_else(|_| serde_json::json!({})),
                });
            }
            VersionEditDto {
                id: ch.id.as_str().to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                forwards: BlockHashedListDto { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: forward_items },
                backwards: BlockHashedListDto { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: backward_items },
                sequence_number,
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                finished_at: Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()),
                description: None,
                origin: None,
            }
        }

        /// @emoji 📸 Project one live write session into a version change with edits.
        async fn change_dto_from_runtime_edit(tx: &Arc<crate::vcs::Edit>, saved: bool) -> VersionChangeDto {
            let mut edits = Vec::new();
            for (idx, ch) in tx.changes.read().await.iter().enumerate() {
                edits.push(Self::edit_dto_from_runtime_change(ch, (idx + 1) as i32).await);
            }
            VersionChangeDto {
                id: tx.id.as_str().to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                edits: BlockHashedListDto { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: edits },
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                saved_at: if saved { Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()) } else { None },
                description: None,
                origin: None,
            }
        }

        async fn change_lists_from_draft(draft: &Arc<crate::vcs::Draft>) -> (BlockHashedListDto<VersionChangeDto>, BlockHashedListDto<VersionChangeDto>) {
            let mut saved = BlockHashedListDto::default();
            let mut unsaved = BlockHashedListDto::default();
            for tx in draft.finalized_transactions.read().await.iter() {
                saved.items.push(Self::change_dto_from_runtime_edit(tx, true).await);
            }
            for tx in draft.transactions.read().await.iter() {
                unsaved.items.push(Self::change_dto_from_runtime_edit(tx, false).await);
            }
            (saved, unsaved)
        }

        /// @emoji 📸 Project the live `Graph` into a metabolism-shaped bundle ready for atomic write.
        /// `wip.id` mirrors the graph id; `wip.initialKit` is the immutable [`Graph::initial_kit`] baseline (SDL `Graph.initialKit`); head materialization stays on `theKit.kit` / version changes.
        pub async fn from_graph(graph: &crate::vcs::Graph) -> Self {
            let mut bundle = Self::template();
            let g = graph.arc_here();
            let initial_dto = g.initial_kit.read().await.kit_full_snapshot_value().await;
            let gid = graph.id.as_str().to_string();
            bundle.wip.id = gid.clone();
            bundle.authoritative.id = gid.clone();
            bundle.stage.id = gid.clone();
            bundle.wip.the_kit.id = gid.clone();
            bundle.authoritative.the_kit.id = gid.clone();
            bundle.stage.the_kit.id = gid;
            bundle.wip.root = initial_dto.clone();
            bundle.authoritative.root = initial_dto.clone();
            bundle.stage.root = initial_dto;

            // 🪧 Project checkpoints (metadata only; kit baselines live on `Graph.initialKit`).
            for cp in graph.checkpoints.read().await.iter() {
                let msg = cp.message.read().await.clone().unwrap_or_default();
                let ts = cp.timestamp.read().await.clone().map(|t| t.0).unwrap_or_else(|| KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string());
                bundle.wip.checkpoints.items.push(serde_json::json!({
                    "id": cp.id.as_str(),
                    "hash": KIT_BUNDLE_HASH_STUB,
                    "timestamp": ts,
                    "message": msg,
                    "authors": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
                    "changes": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
                }));
            }

            // 🧾 Project version changes and edits directly on the wip version rows.
            for draft in graph.drafts.read().await.iter() {
                if draft.owner_alternative.upgrade().is_none() {
                    let (saved, unsaved) = Self::change_lists_from_draft(draft).await;
                    bundle.wip.the_kit.saved_changes.items.extend(saved.items);
                    bundle.wip.the_kit.unsaved_changes.items.extend(unsaved.items);
                }
            }
            for alternative in graph.alternatives.read().await.iter() {
                let (saved_changes, unsaved_changes) = match alternative.draft.read().await.upgrade() {
                    Some(draft) => Self::change_lists_from_draft(&draft).await,
                    None => (BlockHashedListDto::default(), BlockHashedListDto::default()),
                };
                bundle.wip.alternatives.items.push(AlternativeVersionDto { id: alternative.id.as_str().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), name: alternative.name.read().await.clone(), saved_changes, unsaved_changes });
            }

            Self::hoist_inline_file_blobs_for_storage(&mut bundle);
            bundle
        }

        //#region 📦 bundle file blobs (content-addressed outside kit projection JSON)

        /// @emoji 🔢 Blake3 hex digest of the UTF-8 blob wire (`data:` URL or raw); identical bytes ⇒ identical digest ⇒ one row in [`KitStoreBundleFile::blobs`].
        pub(crate) fn digest_kit_blob_wire(wire: &str) -> String {
            blake3::hash(wire.as_bytes()).to_hex().to_string()
        }

        /// @emoji 📦 Hoist each `files[].blob` into [`KitStoreBundleFile::blobs`] keyed by [`digest_kit_blob_wire`], set `files[].blobHash`, strip inline payload (shared digest dedupes across graph `initialKit` projections).
        pub fn hoist_inline_file_blobs_for_storage(bundle: &mut KitStoreBundleFile) {
            let mut seen_digest = std::collections::HashSet::<String>::new();
            let mut collected: Vec<serde_json::Value> = Vec::new();
            Self::take_file_blobs_from_kit_json_into(&mut bundle.wip.root, &mut seen_digest, &mut collected);
            bundle.blobs.items.extend(collected);
            Self::purge_unreferenced_blobs(bundle);
        }

        /// @emoji 🧹 Drop [`blobs`] rows whose digest is not referenced by any `files[].blobHash` on `wip` / `authoritative` / `stage` `initialKit` snapshots.
        pub fn purge_unreferenced_blobs(bundle: &mut KitStoreBundleFile) {
            let refs = Self::referenced_blob_hashes_from_bundle(bundle);
            bundle.blobs.items.retain(|b| b.get("hash").and_then(|x| x.as_str()).map(|h| refs.contains(h)).unwrap_or(false));
        }

        fn referenced_blob_hashes_from_bundle(bundle: &KitStoreBundleFile) -> std::collections::HashSet<String> {
            let mut s = std::collections::HashSet::new();
            Self::collect_blob_hashes_from_kit_projection(&bundle.wip.root, &mut s);
            Self::collect_blob_hashes_from_kit_projection(&bundle.authoritative.root, &mut s);
            Self::collect_blob_hashes_from_kit_projection(&bundle.stage.root, &mut s);
            s
        }

        fn collect_blob_hashes_from_kit_projection(kit: &serde_json::Value, out: &mut std::collections::HashSet<String>) {
            let Some(files_val) = kit.get("files") else {
                return;
            };
            let Some(files) = crate::kit_backbone::json_array_or_block_items_ref(files_val) else {
                return;
            };
            for f in files {
                let Some(o) = f.as_object() else { continue };
                if let Some(h) = o.get("blobHash").and_then(|x| x.as_str()) {
                    out.insert(h.to_string());
                }
            }
        }

        fn take_file_blobs_from_kit_json_into(kit: &mut serde_json::Value, seen_digest: &mut std::collections::HashSet<String>, out: &mut Vec<serde_json::Value>) {
            let Some(files_holder) = kit.get_mut("files") else {
                return;
            };
            let Some(files) = crate::kit_backbone::json_array_or_block_items_mut(files_holder) else {
                return;
            };
            for f in files.iter_mut() {
                let Some(obj) = f.as_object_mut() else { continue };
                let Some(blob_v) = obj.remove("blob") else { continue };
                let blob_str = match blob_v.as_str() {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                let digest = Self::digest_kit_blob_wire(&blob_str);
                obj.insert("blobHash".to_string(), serde_json::Value::String(digest.clone()));
                if seen_digest.insert(digest.clone()) {
                    out.push(serde_json::json!({
                        "hash": digest,
                        "blob": blob_str,
                    }));
                }
            }
        }

        /// @emoji 📎 Merge [`blobs`] into a kit projection clone for hydrate (`files[].blob` restored from `files[].blobHash`); does not mutate the persisted bundle JSON shape.
        pub(crate) fn merge_bundle_file_blobs_into_kit_json(kit: &mut serde_json::Value, blobs: &[serde_json::Value]) {
            if blobs.is_empty() {
                return;
            }
            let mut by_digest: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();
            for b in blobs {
                let Some(d) = b.get("hash").and_then(|x| x.as_str()) else { continue };
                if let Some(blob) = b.get("blob") {
                    by_digest.insert(d.to_string(), blob.clone());
                }
            }
            if by_digest.is_empty() {
                return;
            }
            let Some(files_holder) = kit.get_mut("files") else {
                return;
            };
            let Some(files) = crate::kit_backbone::json_array_or_block_items_mut(files_holder) else {
                return;
            };
            for f in files.iter_mut() {
                let Some(obj) = f.as_object_mut() else { continue };
                let Some(h) = obj.get("blobHash").and_then(|x| x.as_str()) else { continue };
                if let Some(blob) = by_digest.get(h) {
                    obj.insert("blob".to_string(), blob.clone());
                }
            }
        }

        //#endregion 📦 bundle file blobs (content-addressed outside kit projection JSON)

        /// @emoji 🩻 Hydrate the live graph from a previously-persisted bundle JSON and keep version changes available for  replay.
        pub async fn hydrate_into_graph(graph: &std::sync::Arc<crate::vcs::Graph>, json: &str) -> Result<Self, SemioError> {
            let bundle: Self = serde_json::from_str(json).map_err(|e| SemioError::invalid(format!("bundle parse: {e}")))?;
            if bundle.schema != KIT_STORE_BUNDLE_SCHEMA {
                return Err(SemioError::invalid(format!("bundle schema mismatch: {} != {}", bundle.schema, KIT_STORE_BUNDLE_SCHEMA)));
            }
            let mut wip_root = bundle.wip.root.clone();
            Self::merge_bundle_file_blobs_into_kit_json(&mut wip_root, &bundle.blobs.items);
            if !wip_root.is_null() && wip_root.is_object() {
                graph.parent_root_for_active_draft.write().await.hydrate_from_kit_full_snapshot_json(&wip_root).await?;
                let ini = graph.parent_root_for_active_draft.read().await.deep_clone().await;
                *graph.initial_kit.write().await = ini;
            }
            Ok(bundle)
        }

        /// @emoji 🌱 Initialize an empty bundle with a non-empty `wip` head: empty root projection stamped with `kit_id`,
        /// a single seed checkpoint anchored on the empty kit, and one empty unsaved change directly on the version.
        /// This is the "create dev kit" bootstrap state that sketchpad sees the moment a JSON file is opened/created.
        pub fn initialize_with_unsaved_change(kit_id: &str, change_id: &str, checkpoint_id: &str) -> Self {
            let mut bundle = Self::template();
            bundle.wip.id = kit_id.to_string();
            bundle.authoritative.id = kit_id.to_string();
            bundle.stage.id = kit_id.to_string();
            bundle.wip.the_kit.id = kit_id.to_string();
            bundle.authoritative.the_kit.id = kit_id.to_string();
            bundle.stage.the_kit.id = kit_id.to_string();
            // 🪧 First checkpoint anchors the kit at its initial empty projection (no changes yet).
            bundle.wip.checkpoints.items.push(serde_json::json!({
                "id": checkpoint_id,
                "hash": KIT_BUNDLE_HASH_STUB,
                "timestamp": KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB,
                "message": "init",
                "authors": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
                "changes": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
            }));
            // 🧾 Initial unsaved change on the version; edits are added when user actions run.
            bundle.wip.the_kit.unsaved_changes.items.push(VersionChangeDto {
                id: change_id.to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                edits: BlockHashedListDto::default(),
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                saved_at: None,
                description: None,
                origin: None,
            });
            bundle
        }

        /// @emoji ➕ Append a single forward operation to an unsaved version change (creating the change/edit if absent).
        pub fn append_unsaved_edit(&mut self, change_id: &str, kind: &str, input: serde_json::Value) {
            self.append_unsaved_edit_with_origin(change_id, None, kind, input);
        }

        /// @emoji ➕ Append a forward operation to an unsaved version change and keep an optional replay origin anchor.
        pub fn append_unsaved_edit_with_origin(&mut self, change_id: &str, origin: Option<String>, kind: &str, input: serde_json::Value) {
            let changes = &mut self.wip.the_kit.unsaved_changes.items;
            let change_idx = match changes.iter().position(|c| c.id == change_id) {
                Some(i) => i,
                None => {
                    changes.push(VersionChangeDto {
                        id: change_id.to_string(),
                        hash: KIT_BUNDLE_HASH_STUB.to_string(),
                        edits: BlockHashedListDto::default(),
                        started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                        saved_at: None,
                        description: None,
                        origin,
                    });
                    changes.len() - 1
                }
            };
            if changes[change_idx].edits.items.is_empty() {
                changes[change_idx].edits.items.push(VersionEditDto {
                    id: uuid::Uuid::now_v7().to_string(),
                    hash: KIT_BUNDLE_HASH_STUB.to_string(),
                    forwards: BlockHashedListDto::default(),
                    backwards: BlockHashedListDto::default(),
                    sequence_number: 1,
                    started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                    finished_at: Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()),
                    description: None,
                    origin: None,
                });
            }
            changes[change_idx].edits.items[0].forwards.items.push(OperationStepDto { id: uuid::Uuid::now_v7().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), kind: kind.to_string(), description: None, input });
        }

        /// @emoji 🪪 Build a metabolism-shaped bundle from a flat ordered  operation log (used by golden test fixtures and import paths).
        pub fn from_stored__ops(operations: &[StoredOperation]) -> Self {
            let mut bundle = Self::template();
            for operation in operations {
                bundle.append_unsaved_edit_with_origin(&operation.transaction_id, Some(operation.draft_id.clone()), &operation.kind, operation.input.clone());
            }
            bundle
        }
    }

    /// @emoji 📜 Internal value type used by replay + the SQLite local-`.semio/` path; not part of the on-disk dev-json wire format.
    #[derive(Clone, Debug)]
    pub struct StoredOperation {
        pub draft_id: String,
        pub transaction_id: String,
        pub kind: String,
        pub input: serde_json::Value,
    }
    //#endregion 🧾 wire format

    //#region 🧭 paths + uri (native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn normalize_connection_uri(raw: &str) -> String {
        raw.trim().to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn filesystem_path_from_uri(uri: &str) -> Result<PathBuf, SemioError> {
        let u = uri.trim();
        let p = if let Some(r) = u.strip_prefix("file://") { r } else { u };
        if p.is_empty() {
            return Err(SemioError::invalid("empty backbone connectionUri"));
        }
        Ok(PathBuf::from(p))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn resolve_local_semio_root(project_or_dot_semio: &Path) -> PathBuf {
        if project_or_dot_semio.file_name().and_then(|s| s.to_str()) == Some(".semio") {
            project_or_dot_semio.to_path_buf()
        } else {
            project_or_dot_semio.join(".semio")
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn init_local_dot_semio_layout(semio_root: &Path) -> Result<(), SemioError> {
        std::fs::create_dir_all(semio_root).map_err(|e| SemioError::invalid(format!("create .semio root: {e}")))?;
        std::fs::create_dir_all(semio_root.join("blobs")).map_err(|e| SemioError::invalid(format!("create blobs dir: {e}")))?;
        let ddl = r#"
CREATE TABLE IF NOT EXISTS _op_log (
  seq INTEGER PRIMARY KEY AUTOINCREMENT,
  draft_id TEXT NOT NULL,
  transaction_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  input_json TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS conflict_stub (
  id INTEGER PRIMARY KEY
);
"#;
        for name in ["wip.db", "staged.db", "authoritative.db", "conflicts.db"] {
            let db = semio_root.join(name);
            let conn = Connection::open(&db).map_err(|e| SemioError::invalid(format!("open {name}: {e}")))?;
            conn.execute_batch(ddl).map_err(|e| SemioError::invalid(format!("init {name}: {e}")))?;
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn db_file_for_child(semio_root: &Path, child_label: &'static str) -> Result<PathBuf, SemioError> {
        let name = match child_label {
            "wip" => "wip.db",
            "auth" => "authoritative.db",
            other => return Err(SemioError::invalid(format!("unknown child label `{other}` for local backbone"))),
        };
        Ok(semio_root.join(name))
    }
    //#endregion 🧭 paths + uri

    //#region ✍️ atomic json (native only)
    #[cfg(not(target_arch = "wasm32"))]
    fn atomic_write_bundle(path: &Path, doc: &KitStoreBundleFile) -> Result<(), SemioError> {
        let parent = path.parent().ok_or_else(|| SemioError::invalid("kit-store bundle path has no parent directory"))?;
        std::fs::create_dir_all(parent).map_err(|e| SemioError::invalid(format!("create kit-store bundle parent: {e}")))?;
        let tmp = path.with_extension("tmp.semio-write");
        let body = serde_json::to_string_pretty(doc).map_err(|e| SemioError::invalid(e.to_string()))?;
        std::fs::write(&tmp, body).map_err(|e| SemioError::invalid(format!("write temp kit-store bundle: {e}")))?;
        std::fs::rename(&tmp, path).map_err(|e| SemioError::invalid(format!("rename kit-store bundle: {e}")))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_or_init_bundle(path: &Path) -> Result<KitStoreBundleFile, SemioError> {
        if !path.exists() {
            return Ok(KitStoreBundleFile::template());
        }
        let s = std::fs::read_to_string(path).map_err(|e| SemioError::invalid(format!("read kit-store bundle: {e}")))?;
        serde_json::from_str(&s).map_err(|e| SemioError::invalid(format!("parse kit-store bundle: {e}")))
    }
    //#endregion ✍️ atomic json

    //#region 🔁 replay
    pub async fn replay_stored_ops(graph: &Arc<Graph>, operations: &[StoredOperation]) -> Result<(), SemioError> {
        graph.parent_root_for_active_draft.read().await.clear_piece_projections_for_backbone_replay().await;
        for operation in operations {
            let draft_id = Id::from(operation.draft_id.as_str());
            let transaction_id = Id::from(operation.transaction_id.as_str());
            let payload = serde_json::to_string(&operation.input).map_err(|e| SemioError::invalid(e.to_string()))?;
            crate::kit_graph_engine::apply__operation_json(graph, &draft_id, &transaction_id, operation.kind.as_str(), &payload).await?;
        }
        Ok(())
    }
    //#endregion 🔁 replay

    //#region 🧩 attached variants (native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub struct DevJsonAttached {
        path: PathBuf,
        connection_uri_normalized: String,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl DevJsonAttached {
        /// @emoji 📥 Read the on-disk bundle (or fresh template if file is absent).
        pub fn read_bundle(&self) -> Result<KitStoreBundleFile, SemioError> {
            read_or_init_bundle(&self.path)
        }

        /// @emoji ➕ Append a forward  operation step into the targeted unsaved version change and atomically rewrite the bundle.
        pub fn append_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            let mut doc = self.read_bundle()?;
            let _ = draft_id;
            doc.append_unsaved_edit(transaction_id.as_str(), kind, input.clone());
            atomic_write_bundle(&self.path, &doc)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub struct LocalAttached {
        #[allow(dead_code)]
        semio_root: PathBuf,
        db_path: PathBuf,
        connection_uri_normalized: String,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl LocalAttached {
        pub fn append_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite append: {e}")))?;
            let input_json = serde_json::to_string(input).map_err(|e| SemioError::invalid(e.to_string()))?;
            conn.execute("INSERT INTO _op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![draft_id.as_str(), transaction_id.as_str(), kind, input_json])
                .map_err(|e| SemioError::invalid(format!("sqlite insert: {e}")))?;
            Ok(())
        }

        fn load_ops(&self) -> Result<Vec<StoredOperation>, SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite read: {e}")))?;
            let mut stmt = conn.prepare("SELECT draft_id, transaction_id, kind, input_json FROM _op_log ORDER BY seq ASC").map_err(|e| SemioError::invalid(format!("sqlite prepare: {e}")))?;
            let mut rows = stmt.query([]).map_err(|e| SemioError::invalid(format!("sqlite query: {e}")))?;
            let mut out = Vec::new();
            while let Some(row) = rows.next().map_err(|e| SemioError::invalid(format!("sqlite row: {e}")))? {
                let draft_id: String = row.get(0).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let transaction_id: String = row.get(1).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let kind: String = row.get(2).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input_json: String = row.get(3).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input: serde_json::Value = serde_json::from_str(&input_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                out.push(StoredOperation { draft_id, transaction_id, kind, input });
            }
            Ok(out)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub enum AttachedBackbone {
        DevJson(DevJsonAttached),
        Local(LocalAttached),
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl AttachedBackbone {
        pub async fn mount_and_replay(connection_uri: &str, store_kind: BackboneStoreKind, child_label: &'static str, graph: &Arc<Graph>) -> Result<Self, SemioError> {
            let norm = normalize_connection_uri(connection_uri);
            let path = filesystem_path_from_uri(&norm)?;
            let mut this = match store_kind {
                BackboneStoreKind::DevJson => Self::DevJson(DevJsonAttached { path, connection_uri_normalized: norm }),
                BackboneStoreKind::LocalDotSemio => {
                    let semio_root = resolve_local_semio_root(&path);
                    init_local_dot_semio_layout(&semio_root)?;
                    let db_path = db_file_for_child(&semio_root, child_label)?;
                    Self::Local(LocalAttached { semio_root, db_path, connection_uri_normalized: norm })
                }
            };
            this.replay_into_graph(graph).await?;
            Ok(this)
        }

        pub async fn replay_into_graph(&mut self, graph: &Arc<Graph>) -> Result<(), SemioError> {
            let operations: Vec<StoredOperation> = match self {
                AttachedBackbone::DevJson(d) => d.read_bundle()?.wip__ops(),
                AttachedBackbone::Local(l) => l.load_ops()?,
            };
            replay_stored_ops(graph, &operations).await
        }

        pub fn append__op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            match self {
                AttachedBackbone::DevJson(d) => d.append_op(draft_id, transaction_id, kind, input),
                AttachedBackbone::Local(l) => l.append_op(draft_id, transaction_id, kind, input),
            }
        }

        pub fn normalized_connection_uri(&self) -> &str {
            match self {
                AttachedBackbone::DevJson(d) => d.connection_uri_normalized.as_str(),
                AttachedBackbone::Local(l) => l.connection_uri_normalized.as_str(),
            }
        }
    }
    //#endregion 🧩 attached variants

    /// @emoji 🧪 Build [`StoredOperation`] rows from `kit-store.golden.operations.semio.json` (US-001 fixture) for persistence tests.
    pub fn stored_ops_from_golden_ops_json(src: &serde_json::Value) -> Result<Vec<StoredOperation>, SemioError> {
        let draft_id = src["draftId"].as_str().ok_or_else(|| SemioError::invalid("golden operations missing draftId"))?.to_string();
        let transaction_id = src["transactionId"].as_str().ok_or_else(|| SemioError::invalid("golden operations missing transactionId"))?.to_string();
        let arr = src["operations"].as_array().ok_or_else(|| SemioError::invalid("golden operations missing operations"))?;
        let mut out = Vec::new();
        for rec in arr {
            let kind = rec["kind"].as_str().ok_or_else(|| SemioError::invalid("operation.kind"))?;
            let input = rec.get("input").cloned().ok_or_else(|| SemioError::invalid("operation.input"))?;
            out.push(StoredOperation { draft_id: draft_id.clone(), transaction_id: transaction_id.clone(), kind: kind.to_string(), input });
        }
        Ok(out)
    }
}

//#endregion 🗄️ kit backbone persistence (native)

//#region 📣 event

pub mod event {
    //! 📣 The single emit point of the entire crate. Variants carry Arc-shared payloads.
    use std::sync::Arc;

    use async_broadcast::{InactiveReceiver, Receiver, Sender};
    use async_lock::Mutex;

    use crate::error::SemioError;
    use crate::operation;

    /// 🌐 Broadcast envelope for every observable thing the control plane emits.
    #[derive(Clone)]
    pub enum Event {
        CommandSucceeded(operation::CommandReceipt),
        OperationSucceeded(operation::OperationKind),
        OperationFailed(SemioError),
        CreatedFixedPiece(Arc<operation::CreatedFixedPiece>),
        FixedPiece(Arc<operation::FixedPiece>),
        DraggedPiece(Arc<operation::DraggedPiece>),
        RenamedKit(Arc<operation::RenamedKit>),
        ChangedDescription(Arc<operation::ChangedDescription>),
    }

    /// 📣 The bus. Holds the only `emit_event` function in the crate.
    pub struct EventBus {
        tx: Mutex<Sender<Event>>,
        keep_alive: InactiveReceiver<Event>,
    }

    impl EventBus {
        pub fn new(capacity: usize) -> Arc<Self> {
            let (mut tx, rx) = async_broadcast::broadcast(capacity);
            tx.set_overflow(true);
            // No active receivers? still proceed (drop the message) instead of awaiting one.
            tx.set_await_active(false);
            Arc::new(Self { tx: Mutex::new(tx), keep_alive: rx.deactivate() })
        }

        /// 📣 The **only** `emit_event` in the entire crate. All other code paths must call this.
        pub async fn emit_event(&self, ev: Event) {
            let tx = self.tx.lock().await;
            let _ = tx.broadcast_direct(ev).await;
        }

        /// 🔔 New subscriber receiver.
        pub fn subscribe(&self) -> Receiver<Event> {
            self.keep_alive.activate_cloned()
        }
    }
}

//#endregion 📣 event

//#region 🧵 worker

pub mod worker {
    //! 🧵 Parent router + two child runtimes (wip + authoritative).
    //!
    //! Native: both children are spawned on `std::thread + futures-lite::block_on`.
    //! Wasm: each child lives in a dedicated [`web_sys::Worker`]; messages cross via [`crate::wasm_bridge`].
    use std::sync::Arc;

    use async_channel::{Receiver, Sender};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::event::{Event, EventBus};
    use crate::id::Id;
    use crate::operation::{BackboneStoreKind, Command, CommandReceipt, CreatedFixedPiece, CreatedFixedPieceInput, Diff, Input, KitOperation, OperationIface, RenamedKit, RenamedKitInput as OperationRenamedKitInput, Scope};
    use crate::vcs::{Conflict, Graph, Session};

    //#region 🗄️ backbone slot
    /// @emoji 🗄️ Per-child optional persistence tail: native disk backbones; wasm stub (attach returns `NotSupported`).
    pub struct BackboneNativeCell {
        #[cfg(not(target_arch = "wasm32"))]
        slot: Arc<RwLock<Option<crate::kit_backbone::AttachedBackbone>>>,
    }

    impl BackboneNativeCell {
        pub fn new() -> Self {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Self { slot: Arc::new(RwLock::new(None)) }
            }
            #[cfg(target_arch = "wasm32")]
            {
                Self {}
            }
        }

        pub async fn mount(&self, graph: &Arc<Graph>, child_label: &'static str, uri: &str, store_kind: BackboneStoreKind) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let bone = crate::kit_backbone::AttachedBackbone::mount_and_replay(uri, store_kind, child_label, graph).await?;
                *self.slot.write().await = Some(bone);
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (graph, child_label, uri, store_kind);
                Err(SemioError::invalid("Attachable kit backbones use native disk (atomic JSON / SQLite); drive them from native hosts over GraphQL IPC instead of WASM."))
            }
        }

        pub async fn detach_matching(&self, uri: &str) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let norm = crate::kit_backbone::normalize_connection_uri(uri);
                let mut guard = self.slot.write().await;
                match &*guard {
                    Some(current) if current.normalized_connection_uri() != norm => {
                        return Err(SemioError::invalid("`connectionUri` did not match the attached backbone; detach aborted to avoid confusing persistence drift."));
                    }
                    _ => {}
                }
                guard.take();
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = uri;
                Ok(())
            }
        }

        pub async fn record_kit_operation_if_attached(&self, draft_id: &Id, transaction_id: &Id, operation: &crate::operation::KitOperation) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let mut guard = self.slot.write().await;
                if let Some(backbone) = guard.as_mut() {
                    let payload = serde_json::to_value(operation).map_err(|e| SemioError::invalid(e.to_string()))?;
                    backbone.append__op(draft_id, transaction_id, operation.kind(), &payload)?;
                }
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (draft_id, transaction_id, operation);
                Ok(())
            }
        }
    }
    //#endregion 🗄️ backbone slot

    /// 🚪 Per-child handle held by the parent: send commands in (events flow through the shared bus).
    pub struct ChildPort {
        inbound: Sender<Command>,
    }

    impl ChildPort {
        pub async fn send(&self, cmd: Command) {
            let _ = self.inbound.send(cmd).await;
        }
    }

    /// 🛰️ Parent runtime hosting the GraphQL schema + routing logic.
    pub struct ParentRuntime {
        pub bus: Arc<EventBus>,
        pub wip: ChildPort,
        pub auth: ChildPort,
        pub wip_graph: Arc<Graph>,
        pub auth_graph: Arc<Graph>,
        pub sessions: RwLock<Vec<Arc<Session>>>,
        pub conflicts: RwLock<Vec<Arc<Conflict>>>,
        pub wip_kit_scope: RwLock<Option<(Id, Id)>>,
    }

    impl ParentRuntime {
        /// 🛰️ Spawn parent + two child runtimes (in-process on native).
        pub async fn spawn() -> Arc<Self> {
            let bus = EventBus::new(1024);

            let wip_graph = Graph::new().await;
            let auth_graph = Graph::new().await;

            let (wip_tx, wip_rx) = async_channel::unbounded::<Command>();
            let (auth_tx, auth_rx) = async_channel::unbounded::<Command>();

            spawn_child("wip", wip_graph.clone(), bus.clone(), wip_rx);
            spawn_child("auth", auth_graph.clone(), bus.clone(), auth_rx);

            let sess = crate::vcs::Session::new().await;
            let sessions = RwLock::new(vec![sess]);

            Arc::new(Self { bus, wip: ChildPort { inbound: wip_tx }, auth: ChildPort { inbound: auth_tx }, wip_graph, auth_graph, sessions, conflicts: RwLock::new(Vec::new()), wip_kit_scope: RwLock::new(None) })
        }

        /// 🛰️ WASM/host bootstrap: hydrate WIP [`Graph`] from `@semio/js` kit JSON snapshot; authoritative line stays mint-empty.
        pub async fn spawn_wip_overlay_from_kit_dto(dto: serde_json::Value) -> Result<Arc<Self>, crate::error::SemioError> {
            let bus = EventBus::new(1024);

            let wip_graph = Graph::new_overlay_from_kit_json(dto).await?;
            let auth_graph = Graph::new().await;

            let (wip_tx, wip_rx) = async_channel::unbounded::<Command>();
            let (auth_tx, auth_rx) = async_channel::unbounded::<Command>();

            spawn_child("wip", wip_graph.clone(), bus.clone(), wip_rx);
            spawn_child("auth", auth_graph.clone(), bus.clone(), auth_rx);

            let sess = crate::vcs::Session::new().await;
            let sessions = RwLock::new(vec![sess]);

            Ok(Arc::new(Self { bus, wip: ChildPort { inbound: wip_tx }, auth: ChildPort { inbound: auth_tx }, wip_graph, auth_graph, sessions, conflicts: RwLock::new(Vec::new()), wip_kit_scope: RwLock::new(None) }))
        }

        pub async fn dispatch_wip(&self, cmd: Command) -> Id {
            let id = cmd.request_id().clone();
            self.wip.send(cmd).await;
            id
        }

        pub async fn dispatch_auth(&self, cmd: Command) -> Id {
            let id = cmd.request_id().clone();
            self.auth.send(cmd).await;
            id
        }
    }

    fn spawn_child(label: &'static str, graph: Arc<Graph>, bus: Arc<EventBus>, inbox: Receiver<Command>) {
        let fut = async move { ChildRuntime { label, graph, bus, inbox, backbone: BackboneNativeCell::new() }.run().await };
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(fut);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::spawn(move || futures_lite::future::block_on(fut));
        }
    }

    /// 🧵 In-worker actor: drains the inbox, applies, emits.
    pub struct ChildRuntime {
        pub label: &'static str,
        pub graph: Arc<Graph>,
        pub bus: Arc<EventBus>,
        pub inbox: Receiver<Command>,
        pub backbone: BackboneNativeCell,
    }

    impl ChildRuntime {
        pub async fn run(self) {
            while let Ok(cmd) = self.inbox.recv().await {
                let request_id = cmd.request_id().clone();
                let kind = match &cmd {
                    Command::ApplyKitOperation { operation, .. } => operation.kind(),
                    Command::BackboneAttach { .. } => "backboneAttach",
                    Command::BackboneDetach { .. } => "backboneDetach",
                };

                if let Err(e) = self.apply(cmd).await {
                    let err = e.with_request(request_id);
                    self.bus.emit_event(Event::OperationFailed(err)).await;
                } else {
                    self.bus.emit_event(Event::CommandSucceeded(CommandReceipt { request_id, kind: kind.to_string() })).await;
                }
            }
        }

        pub async fn apply(&self, cmd: Command) -> Result<(), SemioError> {
            match cmd {
                Command::ApplyKitOperation { request_id, draft_id, transaction_id, operation } => self.apply_kit_operation(request_id, draft_id, transaction_id, operation).await,
                Command::BackboneAttach { connection_uri, store_kind, .. } => self.backbone.mount(&self.graph, self.label, &connection_uri, store_kind).await,
                Command::BackboneDetach { connection_uri, .. } => self.backbone.detach_matching(&connection_uri).await,
            }
        }

        async fn apply_kit_operation(&self, request_id: Id, draft_id: Id, transaction_id: Id, operation: KitOperation) -> Result<(), SemioError> {
            let graph = self.graph.clone();
            let before_kit = graph.materialized_kit_for_draft(&draft_id).await;
            let _ = operation.to_diff(&before_kit).await?;
            let backwards = operation.to_backwards(&before_kit).await?;
            let forward = operation.clone();
            graph.record_op_in_open_transaction(&draft_id, &transaction_id, forward, backwards).await?;
            let after_kit = graph.materialized_kit_for_draft(&draft_id).await;

            let tx_edit = {
                let d = graph.drafts.read().await.iter().find(|d| d.id == draft_id).cloned().ok_or_else(|| SemioError::not_found("Draft", draft_id.as_str()))?;
                let txs = d.transactions.read().await.clone();
                txs.into_iter().find(|t| t.id == transaction_id).ok_or_else(|| SemioError::not_found("Edit", transaction_id.as_str()))?
            };

            match &operation {
                KitOperation::RenameKit { input, .. } => {
                    let Input::Name { name } = input else {
                        return Err(SemioError::invalid("renameKit expects Input::Name"));
                    };
                    let mut diff = Diff::default();
                    diff.id = Id::new().await;
                    diff.summary = Some("renameKit".to_string());
                    let op_evt = Arc::new(RenamedKit { id: Id::new().await, request_id, owner_edit: Arc::downgrade(&tx_edit), input: OperationRenamedKitInput { name: name.clone() }, diff, kit: after_kit.clone() });
                    let iface = Arc::new(OperationIface::RenamedKit(op_evt.clone()));
                    graph.op_history.write().await.push(iface.clone());
                    tx_edit.forward_iface_ops.write().await.push(iface);
                    self.bus.emit_event(Event::RenamedKit(op_evt)).await;
                }
                KitOperation::CreateFixedPiece { scope, input } => {
                    let persisted = operation.clone();
                    self.backbone.record_kit_operation_if_attached(&draft_id, &transaction_id, &persisted).await?;
                    let Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, .. } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    let Input::FixedPiece { position, name, description } = input else {
                        return Err(SemioError::invalid("createFixedPiece expects Input::FixedPiece"));
                    };
                    let design = after_kit.design_by_external_id(design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
                    let piece = design.piece_by_external_id(piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
                    let payload_json = persisted.payload_json()?;
                    let fp_before = crate::kit_graph_engine::projection_fingerprint_for_kit(before_kit.as_ref()).await;
                    let fp_after = crate::kit_graph_engine::projection_fingerprint_for_kit(after_kit.as_ref()).await;
                    let diff = crate::kit_graph_engine::deterministic__diff("createFixedPiece", &payload_json, &fp_before, &fp_after);
                    let created_input = CreatedFixedPieceInput { design_id: design_id.clone(), blueprint_id: blueprint_id.clone(), position: position.clone(), name: name.clone(), description: description.clone() };
                    let op_evt = Arc::new(CreatedFixedPiece { id: Id::new().await, owner_edit: Arc::downgrade(&tx_edit), input: created_input, diff, piece });
                    let iface = Arc::new(OperationIface::CreatedFixedPiece(op_evt.clone()));
                    graph.op_history.write().await.push(iface.clone());
                    tx_edit.forward_iface_ops.write().await.push(iface);
                    self.bus.emit_event(Event::CreatedFixedPiece(op_evt)).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}

//#endregion 🧵 worker

//#region 🌐 gql

pub mod gql {
    //! 🌐 Type-safe static GraphQL schema via `Schema::build` (embedded target SDL string for tooling).
    use async_graphql::types::Json;
    use async_graphql::{Context, Object, Schema, Subscription};
    use async_stream::stream;
    use futures_util::Stream;
    use std::pin::Pin;
    use std::sync::Arc;

    use crate::event::{Event, EventBus};
    use crate::geom::{Offset, Position};
    use crate::id::Id;
    use crate::operation::{Command, Input, KitOperation, Scope};
    use crate::vcs::Graph;
    use crate::worker::ParentRuntime;

    //#region 🌐 interfaces
    /// @emoji 🌐 SDL `Node` + `EntityEdge` interfaces (geometry variants). `EntityConnection` + `Entity`/`WeakEntity`/… need resolver-aligned field types (register after `page_info`/`Arc` story settles).
    pub mod interfaces {
        use std::sync::Arc;

        use async_graphql::Interface;

        use crate::geom::entity::{CoordinateNode, LocationNode, OffsetNode, PlaneNode, PointNode, PositionNode, VectorNode};
        use crate::gql_relay::{CoordinateEdge, LocationEdge, OffsetEdge, PlaneEdge, PointEdge, PositionEdge, VectorEdge};

        #[derive(Clone, Interface)]
        #[graphql(name = "Node", field(name = "id", ty = "crate::id::Id"))]
        pub enum NodeIface {
            Vector(Arc<VectorNode>),
            Point(Arc<PointNode>),
            Coordinate(Arc<CoordinateNode>),
            Offset(Arc<OffsetNode>),
            Plane(Arc<PlaneNode>),
            Position(Arc<PositionNode>),
            Location(Arc<LocationNode>),
        }

        #[derive(Clone, Interface)]
        #[graphql(name = "EntityEdge", field(name = "cursor", ty = "String"))]
        pub enum EntityEdgeIface {
            Vector(VectorEdge),
            Point(PointEdge),
            Coordinate(CoordinateEdge),
            Offset(OffsetEdge),
            Plane(PlaneEdge),
            Position(PositionEdge),
            Location(LocationEdge),
        }

        #[derive(Clone, Interface)]
        #[graphql(
            name = "Version",
            field(name = "id", ty = "crate::id::Id"),
            field(name = "hash", ty = "String"),
            field(name = "checkpoint", ty = "crate::gql_relay::CheckpointConnection"),
            field(name = "latestWipCheckpointAncestor", method = "latest_wip_checkpoint_ancestor", ty = "Option<Arc<crate::vcs::Checkpoint>>"),
            field(name = "savedChanges", method = "saved_changes", ty = "crate::gql_relay::ChangeConnection"),
            field(name = "unsavedChanges", method = "unsaved_changes", ty = "crate::gql_relay::ChangeConnection"),
            field(name = "kit", ty = "Arc<crate::kit::Kit>")
        )]
        pub enum VersionIface {
            TheKit(Arc<crate::vcs::TheKit>),
            Alternative(Arc<crate::vcs::Alternative>),
        }
    }
    //#endregion 🌐 interfaces

    /// @emoji 🧩 Executable schema (`Query`, `Mutation`, `Subscription`).
    pub type AppSchema = Schema<Query, Mutation, Subscription>;

    pub struct Query;

    #[Object]
    impl Query {
        /// @emoji 🧭 Canonical entry: first active [`crate::vcs::Session`] on this runtime.
        pub async fn session(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<crate::vcs::Session>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            rt.sessions.read().await.first().cloned().ok_or_else(|| async_graphql::Error::new("no session"))
        }

        pub async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.wip_graph.clone())
        }

        #[graphql(name = "authoritative")]
        pub async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.auth_graph.clone())
        }

        pub async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql_relay::ConflictConnection> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let list = rt.conflicts.read().await.clone();
            Ok(crate::gql_relay::ConflictConnection::from_conflicts(list).await)
        }

        /// @emoji 🔎 Relay-style global `node` lookup (WIP + authoritative + sessions + conflicts).
        pub async fn node(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::resolve_node(rt.as_ref(), &id).await)
        }

        /// @emoji 🔎 Alias of [`Query::node`] for SDL `entity` entry point (`hash` merkle id).
        pub async fn entity(&self, ctx: &Context<'_>, hash: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            self.node(ctx, hash).await
        }

        #[graphql(name = "kitStoreBundleJson")]
        pub async fn kit_store_bundle_json(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            rt.wip_graph.ensure_default_seed_state().await;
            let bundle = crate::kit_backbone::KitStoreBundleFile::from_graph(rt.wip_graph.as_ref()).await;
            serde_json::to_string_pretty(&bundle).map_err(|e| async_graphql::Error::new(e.to_string()))
        }
    }

    //#region 🎛️commands
    /// @emoji 🎛️ `Mutation.session` scope — holds kit command context on [`ParentRuntime`].
    pub struct SessionCommandNav;

    #[Object(name = "SessionCommandInput")]
    impl SessionCommandNav {
        async fn start(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let _ = rt.wip_graph.ensure_default_seed_state().await;
            Ok(rt.sessions.read().await.first().map(|s| s.id.clone()).ok_or_else(|| async_graphql::Error::new("no session"))?)
        }

        async fn end(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = ctx;
            Ok(Id::new().await)
        }

        async fn login(&self, ctx: &Context<'_>, username: String, password_hash: String, hub_url: Option<String>) -> async_graphql::Result<Id> {
            let _ = (ctx, username, password_hash, hub_url);
            Ok(Id::new().await)
        }

        async fn logout(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = ctx;
            Ok(Id::new().await)
        }

        #[graphql(name = "theKit")]
        async fn the_kit(&self) -> VersionCommandNav {
            VersionCommandNav
        }

        async fn alternative(&self, #[graphql(name = "id")] id: Id) -> AlternativeCommandNav {
            AlternativeCommandNav { alternative_id: id }
        }

        #[graphql(name = "startAlternative")]
        async fn start_alternative(&self, ctx: &Context<'_>, name: Option<String>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let n = name.unwrap_or_default();
            rt.wip_graph.create_alternative_from_tip(n, None).await.map_err(|e| async_graphql::Error::new(e.to_string()))
        }
    }

    pub struct VersionCommandNav;

    #[Object(name = "VersionCommandInput")]
    impl VersionCommandNav {
        #[graphql(name = "startNewChange")]
        async fn start_new_change(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let draft = rt.wip_graph.ensure_default_seed_state().await;
            let tx = rt.wip_graph.open_transaction(&draft.id).await;
            *rt.wip_kit_scope.write().await = Some((draft.id.clone(), tx.id.clone()));
            Ok(tx.id.clone())
        }

        #[graphql(name = "unsavedChange")]
        async fn unsaved_change(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<UnsavedChangeNav> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            if let Some((_, tx)) = rt.wip_kit_scope.read().await.as_ref() {
                if tx != &id {
                    return Err(async_graphql::Error::new("unsavedChange id does not match active change"));
                }
            }
            Ok(UnsavedChangeNav { change_id: id })
        }

        async fn save(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let scope = rt.wip_kit_scope.read().await.clone();
            let Some((draft_id, tx_id)) = scope else {
                return Err(async_graphql::Error::new("no active unsaved change"));
            };
            rt.wip_graph.commit_transaction(&draft_id, &tx_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            *rt.wip_kit_scope.write().await = None;
            Ok(Id::new().await)
        }

        #[graphql(name = "createCheckpoint")]
        async fn create_checkpoint(&self, ctx: &Context<'_>, message: String) -> async_graphql::Result<Id> {
            let _ = (ctx, message);
            Ok(Id::new().await)
        }
    }

    pub struct UnsavedChangeNav {
        pub change_id: Id,
    }

    #[Object(name = "UnsavedChangeCommandInput")]
    impl UnsavedChangeNav {
        async fn kit(&self) -> KitOperationNav {
            KitOperationNav { change_id: self.change_id.clone() }
        }

        async fn save(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let scope = rt.wip_kit_scope.read().await.clone();
            let Some((draft_id, tx_id)) = scope else {
                return Err(async_graphql::Error::new("no active unsaved change"));
            };
            if tx_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            rt.wip_graph.commit_transaction(&draft_id, &tx_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            *rt.wip_kit_scope.write().await = None;
            Ok(Id::new().await)
        }
    }

    pub struct AlternativeCommandNav {
        pub alternative_id: Id,
    }

    #[Object(name = "AlternativeCommandInput")]
    impl AlternativeCommandNav {
        async fn version(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = (ctx, &self.alternative_id);
            Ok(Id::new().await)
        }

        #[graphql(name = "integrateIntoTheKit")]
        async fn integrate_into_the_kit(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = (ctx, &self.alternative_id);
            Ok(Id::new().await)
        }
    }

    pub struct KitOperationNav {
        pub change_id: Id,
    }

    #[Object(name = "KitOperationInput")]
    impl KitOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyKitOperation { request_id: request_id.clone(), draft_id, transaction_id, operation: KitOperation::RenameKit { scope: Scope::Kit, input: Input::Name { name: new_name } } };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }

        #[graphql(name = "createTag")]
        async fn create_tag(&self, ctx: &Context<'_>, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let tag_id = Id::new().await;
            let request_id = Id::new().await;
            let tag = crate::meta::TagInput { name, description, icon, order, attributes: None };
            let cmd = Command::ApplyKitOperation {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                operation: KitOperation::CreateTag { scope: Scope::CreateTag { owner_id, tag_id: tag_id.clone(), attribute_ids: Vec::new() }, input: Input::Tag { tag } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn tag(&self, #[graphql(name = "id")] id: Id) -> TagOperationNav {
            TagOperationNav { change_id: self.change_id.clone(), tag_id: id }
        }

        #[graphql(name = "deleteTag")]
        async fn delete_tag(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "deleteTags")]
        async fn delete_tags(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }

        #[graphql(name = "createConcept")]
        async fn create_concept(&self, ctx: &Context<'_>, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let concept_id = Id::new().await;
            let request_id = Id::new().await;
            let concept = crate::meta::ConceptInput { name, description, icon, order, attributes: None };
            let cmd = Command::ApplyKitOperation {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                operation: KitOperation::CreateConcept { scope: Scope::CreateConcept { owner_id, concept_id: concept_id.clone(), attribute_ids: Vec::new() }, input: Input::Concept { concept } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn concept(&self, #[graphql(name = "id")] id: Id) -> ConceptOperationNav {
            ConceptOperationNav { change_id: self.change_id.clone(), concept_id: id }
        }

        #[graphql(name = "deleteConcept")]
        async fn delete_concept(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "deleteConcepts")]
        async fn delete_concepts(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }

        #[graphql(name = "createQuality")]
        async fn create_quality(&self, ctx: &Context<'_>, key: String, value: Option<String>, unit: Option<String>, definition: Option<String>, description: Option<String>, icon: Option<String>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let quality_id = Id::new().await;
            let request_id = Id::new().await;
            let quality = crate::meta::QualityInput { key, value, unit, definition, description, icon, attributes: None };
            let cmd = Command::ApplyKitOperation {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                operation: KitOperation::CreateQuality { scope: Scope::CreateQuality { owner_id, quality_id: quality_id.clone(), attribute_ids: Vec::new(), benchmark_ids: Vec::new() }, input: Input::Quality { quality } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn quality(&self, #[graphql(name = "id")] id: Id) -> QualityOperationNav {
            QualityOperationNav { change_id: self.change_id.clone(), quality_id: id }
        }

        #[graphql(name = "deleteQuality")]
        async fn delete_quality(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "deleteQualities")]
        async fn delete_qualities(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }

        #[graphql(name = "createType")]
        async fn create_type(&self, ctx: &Context<'_>, name: String, description: Option<String>, icon: Option<String>, image: Option<String>, unit: Option<String>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, name, description, icon, image, unit);
            Ok(Id::new().await)
        }

        async fn r#type(&self, #[graphql(name = "id")] id: Id) -> TypeOperationNav {
            TypeOperationNav { change_id: self.change_id.clone(), type_id: id }
        }

        #[graphql(name = "deleteType")]
        async fn delete_type(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "deleteTypes")]
        async fn delete_types(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }

        #[graphql(name = "createDesign")]
        async fn create_design(&self, ctx: &Context<'_>, name: String, description: Option<String>, icon: Option<String>, image: Option<String>, unit: Option<String>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, name, description, icon, image, unit);
            Ok(Id::new().await)
        }

        async fn design(&self, #[graphql(name = "id")] id: Id) -> DesignOperationNav {
            DesignOperationNav { change_id: self.change_id.clone(), design_id: id }
        }

        #[graphql(name = "deleteDesign")]
        async fn delete_design(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "deleteDesigns")]
        async fn delete_designs(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct TagOperationNav {
        pub change_id: Id,
        pub tag_id: Id,
    }

    #[Object(name = "TagOperationInput")]
    impl TagOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_name);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct ConceptOperationNav {
        pub change_id: Id,
        pub concept_id: Id,
    }

    #[Object(name = "ConceptOperationInput")]
    impl ConceptOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_name);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct QualityOperationNav {
        pub change_id: Id,
        pub quality_id: Id,
    }

    #[Object(name = "QualityOperationInput")]
    impl QualityOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newKey")] new_key: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_key);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct TypeOperationNav {
        pub change_id: Id,
        pub type_id: Id,
    }

    #[Object(name = "TypeOperationInput")]
    impl TypeOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_name);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
        #[graphql(name = "createPort")]
        async fn create_port(&self, ctx: &Context<'_>, code: Option<String>, label: Option<String>, description: Option<String>, icon: Option<String>, order: Option<i32>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, code, label, description, icon, order);
            Ok(Id::new().await)
        }
        async fn port(&self, #[graphql(name = "id")] id: Id) -> PortOperationNav {
            PortOperationNav { change_id: self.change_id.clone(), type_id: self.type_id.clone(), port_id: id }
        }
        #[graphql(name = "deletePort")]
        async fn delete_port(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "deletePorts")]
        async fn delete_ports(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
        #[graphql(name = "addConnector")]
        async fn add_connector(&self, ctx: &Context<'_>, code: String, description: Option<String>, icon: Option<String>, #[graphql(name = "portId")] port_id: Option<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, code, description, icon, port_id);
            Ok(Id::new().await)
        }
        async fn connector(&self, #[graphql(name = "id")] id: Id) -> ConnectorOperationNav {
            ConnectorOperationNav { change_id: self.change_id.clone(), type_id: self.type_id.clone(), connector_id: id }
        }
        #[graphql(name = "removeConnector")]
        async fn remove_connector(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeConnectors")]
        async fn remove_connectors(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct PortOperationNav {
        pub change_id: Id,
        pub type_id: Id,
        pub port_id: Id,
    }

    #[Object(name = "PortOperationInput")]
    impl PortOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newCode")] new_code: String, #[graphql(name = "newLabel")] new_label: Option<String>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_code, new_label);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct ConnectorOperationNav {
        pub change_id: Id,
        pub type_id: Id,
        pub connector_id: Id,
    }

    #[Object(name = "ConnectorOperationInput")]
    impl ConnectorOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newCode")] new_code: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_code);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_icon);
            Ok(Id::new().await)
        }
    }

    pub struct DesignOperationNav {
        pub change_id: Id,
        pub design_id: Id,
    }

    #[Object(name = "DesignOperationInput")]
    impl DesignOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_name);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        async fn flatten(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = (ctx, self);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
        #[graphql(name = "addFixedPiece")]
        async fn add_fixed_piece(&self, ctx: &Context<'_>, #[graphql(name = "blueprintId")] blueprint_id: Id, position: Position, name: Option<String>, description: Option<String>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let piece_id = Id::new().await;
            let cmd = Command::ApplyKitOperation {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                operation: KitOperation::CreateFixedPiece { scope: Scope::CreateFixedPiece { design_id: self.design_id.clone(), piece_id, blueprint_id, attribute_ids: Vec::new() }, input: Input::FixedPiece { position, name, description } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
        #[graphql(name = "addChildPieceWithParentConnection")]
        async fn add_child_piece_with_parent_connection(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "blueprintId")] blueprint_id: Id,
            #[graphql(name = "parentPieceId")] parent_piece_id: Id,
            #[graphql(name = "parentConnector")] parent_connector: String,
            #[graphql(name = "childConnector")] child_connector: String,
            name: Option<String>,
            description: Option<String>,
            position: Option<Position>,
            scale: Option<f64>,
        ) -> async_graphql::Result<Id> {
            let _ = (ctx, self, blueprint_id, parent_piece_id, parent_connector, child_connector, name, description, position, scale);
            Ok(Id::new().await)
        }
        #[graphql(name = "addHangingChildPieceWithParentConnection")]
        async fn add_hanging_child_piece_with_parent_connection(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "blueprintId")] blueprint_id: Id,
            #[graphql(name = "parentPieceId")] parent_piece_id: Id,
            #[graphql(name = "parentConnector")] parent_connector: String,
            #[graphql(name = "childConnector")] child_connector: String,
            position: Position,
            name: Option<String>,
            description: Option<String>,
            scale: Option<f64>,
        ) -> async_graphql::Result<Id> {
            let _ = (ctx, self, blueprint_id, parent_piece_id, parent_connector, child_connector, position, name, description, scale);
            Ok(Id::new().await)
        }
        async fn piece(&self, #[graphql(name = "id")] id: Id) -> PieceOperationNav {
            PieceOperationNav { change_id: self.change_id.clone(), design_id: self.design_id.clone(), piece_id: id }
        }
        async fn pieces(&self, ids: Vec<Id>) -> PiecesOperationNav {
            PiecesOperationNav { change_id: self.change_id.clone(), design_id: self.design_id.clone(), piece_ids: ids }
        }
        #[graphql(name = "deletePiece")]
        async fn delete_piece(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "deletePieces")]
        async fn delete_pieces(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
        #[graphql(name = "deletePiecesAndConnections")]
        async fn delete_pieces_and_connections(&self, ctx: &Context<'_>, #[graphql(name = "pieceIds")] piece_ids: Vec<Id>, #[graphql(name = "connectionIds")] connection_ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, piece_ids, connection_ids);
            Ok(Id::new().await)
        }
    }

    pub struct PieceOperationNav {
        pub change_id: Id,
        pub design_id: Id,
        pub piece_id: Id,
    }

    #[Object(name = "PieceOperationInput")]
    impl PieceOperationNav {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_name);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }
        async fn drag(&self, ctx: &Context<'_>, offset: Offset) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyKitOperation {
                request_id,
                draft_id,
                transaction_id,
                operation: KitOperation::DragPieceInDesign { scope: Scope::PieceInDesign { design_id: self.design_id.clone(), piece_id: self.piece_id.clone() }, input: Input::Offset { offset } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
        async fn r#move(&self, ctx: &Context<'_>, position: Position) -> async_graphql::Result<Id> {
            let _ = (ctx, self, position);
            Ok(Id::new().await)
        }
        async fn fix(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = (ctx, self);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeBlueprint")]
        async fn change_blueprint(&self, ctx: &Context<'_>, #[graphql(name = "blueprintId")] blueprint_id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, blueprint_id);
            Ok(Id::new().await)
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, key, value, definition);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, id);
            Ok(Id::new().await)
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<Id> {
            let _ = (ctx, self, ids);
            Ok(Id::new().await)
        }
    }

    pub struct PiecesOperationNav {
        pub change_id: Id,
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
    }

    #[Object(name = "PiecesOperationInput")]
    impl PiecesOperationNav {
        async fn drag(&self, ctx: &Context<'_>, offset: Offset) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let (draft_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyKitOperation {
                request_id,
                draft_id,
                transaction_id,
                operation: KitOperation::DragPiecesInDesign { scope: Scope::PiecesInDesign { design_id: self.design_id.clone(), piece_ids: self.piece_ids.clone() }, input: Input::Offset { offset } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
        async fn r#move(&self, ctx: &Context<'_>, offset: Offset) -> async_graphql::Result<Id> {
            let _ = (ctx, self, offset);
            Ok(Id::new().await)
        }
        async fn fix(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = (ctx, self);
            Ok(Id::new().await)
        }
        #[graphql(name = "changeBlueprint")]
        async fn change_blueprint(&self, ctx: &Context<'_>, #[graphql(name = "blueprintId")] blueprint_id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, self, blueprint_id);
            Ok(Id::new().await)
        }
    }
    //#endregion 🎛️commands

    pub struct Mutation;

    #[Object]
    impl Mutation {
        /// @emoji 🎛️ Kit-changing commands — navigate via nested fields per `target.schema.graphql` `#region Commands`.
        async fn session(&self) -> SessionCommandNav {
            SessionCommandNav
        }

        #[graphql(name = "hydrateKitStoreBundleJson")]
        async fn hydrate_kit_store_bundle_json(&self, ctx: &Context<'_>, json: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            crate::kit_backbone::KitStoreBundleFile::hydrate_into_graph(&rt.wip_graph, &json).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            rt.wip_graph.ensure_default_seed_state().await;
            Ok(rt.wip_graph.id.clone())
        }
    }

    pub struct Subscription;

    type EventJsonStream = Pin<Box<dyn Stream<Item = async_graphql::Result<Json<serde_json::Value>>> + Send>>;

    fn event_to_json(ev: &Event) -> serde_json::Value {
        let kind = match ev {
            Event::CommandSucceeded(_) => "commandSucceeded",
            Event::OperationSucceeded(_) => "operationSucceeded",
            Event::OperationFailed(_) => "operationFailed",
            Event::CreatedFixedPiece(_) => "createdFixedPiece",
            Event::FixedPiece(_) => "fixedPiece",
            Event::DraggedPiece(_) => "draggedPiece",
            Event::RenamedKit(_) => "kitRenamed",
            Event::ChangedDescription(_) => "changedDescription",
        };
        let payload = match ev {
            Event::CommandSucceeded(r) => serde_json::to_value(r).unwrap_or(serde_json::Value::Null),
            Event::OperationSucceeded(_) => serde_json::Value::Null,
            Event::OperationFailed(e) => serde_json::Value::String(e.to_string()),
            _ => serde_json::Value::Null,
        };
        serde_json::json!({ "kind": kind, "payload": payload })
    }

    #[Subscription]
    impl Subscription {
        async fn event(&self, ctx: &Context<'_>) -> async_graphql::Result<EventJsonStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let mut rx = bus.subscribe();
            Ok(Box::pin(stream! {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            let j = event_to_json(&ev);
                            yield Ok(Json(j));
                        }
                        Err(_) => break,
                    }
                }
            }))
        }
    }

    fn build_schema_sync_for(rt: Arc<ParentRuntime>) -> AppSchema {
        Schema::build(Query, Mutation, Subscription)
            .data(rt.clone())
            .data(rt.bus.clone())
            .register_output_type::<crate::kit::target_operations::CreatedQualityInput>()
            .register_output_type::<crate::kit::target_operations::CreatedQualitiesInput>()
            .register_output_type::<crate::kit::target_operations::RenamedQualityInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedQualityDescriptionInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedQualityIconInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributeToQualityInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributesToQualityInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributeFromQualityInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributesFromQualityInput>()
            .register_output_type::<crate::kit::target_operations::DeletedQualityInput>()
            .register_output_type::<crate::kit::target_operations::DeletedQualitiesInput>()
            .register_output_type::<crate::kit::target_operations::CreatedTagInput>()
            .register_output_type::<crate::kit::target_operations::CreatedTagsInput>()
            .register_output_type::<crate::kit::target_operations::RenamedTagInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedTagDescriptionInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedTagIconInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributeToTagInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributesToTagInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributeFromTagInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributesFromTagInput>()
            .register_output_type::<crate::kit::target_operations::DeletedTagInput>()
            .register_output_type::<crate::kit::target_operations::DeletedTagsInput>()
            .register_output_type::<crate::kit::target_operations::CreatedConceptInput>()
            .register_output_type::<crate::kit::target_operations::CreatedConceptsInput>()
            .register_output_type::<crate::kit::target_operations::RenamedConceptInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedConceptDescriptionInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedConceptIconInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributeToConceptInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributesToConceptInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributeFromConceptInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributesFromConceptInput>()
            .register_output_type::<crate::kit::target_operations::DeletedConceptInput>()
            .register_output_type::<crate::kit::target_operations::DeletedConceptsInput>()
            .register_output_type::<crate::kit::target_operations::CreatedPortInput>()
            .register_output_type::<crate::kit::target_operations::CreatedPortsInput>()
            .register_output_type::<crate::kit::target_operations::RenamedPortInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedPortDescriptionInput>()
            .register_output_type::<crate::kit::target_operations::UpdatedPortIconInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributeToPortInput>()
            .register_output_type::<crate::kit::target_operations::AddedAttributesToPortInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributeFromPortInput>()
            .register_output_type::<crate::kit::target_operations::RemovedAttributesFromPortInput>()
            .register_output_type::<crate::kit::target_operations::DeletedPortInput>()
            .register_output_type::<crate::kit::target_operations::DeletedPortsInput>()
            .register_output_type::<crate::gql::interfaces::NodeIface>()
            .register_output_type::<crate::gql::interfaces::EntityEdgeIface>()
            .register_output_type::<crate::gql::interfaces::VersionIface>()
            .finish()
    }

    /// 📜 Canonical SDL: static fragments from [`crate::sdl_registry`] plus embedded executable golden (`semio/graphql/target.schema.graphql`).
    pub async fn sdl() -> String {
        let mut acc = String::new();
        for frag in crate::sdl_registry::all_fragments() {
            acc.push_str(frag);
            acc.push('\n');
        }
        acc.push_str(include_str!("../graphql/target.schema.graphql"));
        normalize_target_sdl(&acc)
    }

    /// 🧮 Normalize SDL text for stable comparisons (trim ends, collapse blank-line runs).
    pub fn normalize_target_sdl(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut blank_run = 0u32;
        for line in s.lines() {
            let t = line.trim_end();
            if t.is_empty() {
                blank_run += 1;
                if blank_run > 1 {
                    continue;
                }
                out.push('\n');
            } else {
                blank_run = 0;
                out.push_str(t);
                out.push('\n');
            }
        }
        out.trim_end_matches('\n').to_string() + "\n"
    }

    /// 🧱 Build schema with parent runtime + bus.
    pub fn build_schema_for(rt: Arc<ParentRuntime>) -> AppSchema {
        build_schema_sync_for(rt)
    }

    /// 🧱 Default schema (fresh runtime).
    pub async fn build_schema() -> AppSchema {
        build_schema_sync_for(ParentRuntime::spawn().await)
    }
}

//#endregion 🌐 gql

//#region 🔌 wasm_bridge

#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    //! 🌐 `KitStoreHandle`: GraphQL executor + subscriptions over seeded [`crate::worker::ParentRuntime`] (WASM build).
    use std::sync::Arc;
    use std::sync::Mutex;

    use async_graphql::{Request, Variables};
    use serde::Deserialize;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;

    use crate::gql::{build_schema_for, AppSchema};
    use crate::worker::ParentRuntime;

    #[derive(Deserialize)]
    struct WireReq {
        query: String,
        #[serde(rename = "operationName")]
        #[allow(dead_code)]
        operation_name: Option<String>,
        variables: Option<serde_json::Value>,
    }

    fn request_from_wire(s: &str) -> Result<Request, JsValue> {
        let w: WireReq = serde_json::from_str(s).map_err(|e| JsValue::from_str(&format!("graphql json: {}", e)))?;
        let mut r = Request::new(w.query);
        if let Some(v) = w.variables {
            let vars = Variables::from_json(v);
            r = r.variables(vars);
        }
        Ok(r)
    }

    #[wasm_bindgen(start)]
    pub fn _start() {
        console_error_panic_hook::set_once();
    }

    /// 🛰️ Boot stub (workers may call explicitly); runtime is rooted by [`KitStoreHandle`].
    #[wasm_bindgen(js_name = boot)]
    pub fn boot() {}

    /// 📜 Schema SDL for tooling (`schema_sdl` export name matches existing pkg consumers).
    #[wasm_bindgen]
    pub fn schema_sdl() -> js_sys::Promise {
        future_to_promise(async move {
            let sdl = crate::gql::sdl().await;
            Ok(JsValue::from_str(&sdl))
        })
    }

    /// 🛰️ Boot the parent runtime inside the current (parent) web worker — placeholder for iframe hosts.
    #[wasm_bindgen(js_name = parent_boot)]
    pub fn parent_boot() -> js_sys::Promise {
        future_to_promise(async move {
            let _rt: Arc<ParentRuntime> = ParentRuntime::spawn().await;
            Ok(JsValue::TRUE)
        })
    }

    /// 🌐 Stateful GraphQL façade for `@semio/js` embedded worker + inline WASM.
    #[wasm_bindgen]
    pub struct KitStoreHandle {
        rt: Arc<ParentRuntime>,
        schema_mtx: Arc<Mutex<Option<AppSchema>>>,
    }

    #[wasm_bindgen]
    impl KitStoreHandle {
        /// 🧾 `KitStoreHandle.create(JSON.parse(kitDto))`.
        #[wasm_bindgen(js_name = create)]
        pub fn create(dto_js: JsValue) -> js_sys::Promise {
            future_to_promise(async move {
                let v: serde_json::Value = serde_wasm_bindgen::from_value(dto_js).map_err(|e| JsValue::from_str(&e.to_string()))?;
                let rt = ParentRuntime::spawn_wip_overlay_from_kit_dto(v).await.map_err(|e| JsValue::from_str(&e.message))?;
                Ok(JsValue::from(KitStoreHandle { rt, schema_mtx: Arc::new(Mutex::new(None)) }))
            })
        }

        #[wasm_bindgen(js_name = execute)]
        pub fn execute(&self, req_json: &str) -> js_sys::Promise {
            let req_str = req_json.to_string();
            let rt = self.rt.clone();
            let mtx = Arc::clone(&self.schema_mtx);
            future_to_promise(async move {
                let mut locked = mtx.lock().map_err(|_| JsValue::from_str("schema lock poisoned"))?;
                if locked.is_none() {
                    *locked = Some(build_schema_for(rt.clone()));
                }
                let schema = locked.as_ref().ok_or_else(|| JsValue::from_str("schema init failed"))?;
                let schema = schema.clone();
                drop(locked);

                let mut req = request_from_wire(&req_str)?;
                req = req.data(rt.clone()).data(rt.bus.clone());
                let resp = schema.execute(req).await;
                let json = serde_json::to_string(&async_graphql::Response::from(resp)).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Ok(JsValue::from_str(&json))
            })
        }

        #[wasm_bindgen(js_name = subscribe)]
        pub fn subscribe(&self, req_json: &str, on_event: &::js_sys::Function) -> js_sys::Promise {
            let cb = on_event.clone();
            let req_str = req_json.to_string();
            let rt = self.rt.clone();
            let mtx = Arc::clone(&self.schema_mtx);
            future_to_promise(async move {
                use futures_util::StreamExt;

                let mut locked = mtx.lock().map_err(|_| JsValue::from_str("schema lock poisoned"))?;
                if locked.is_none() {
                    *locked = Some(build_schema_for(rt.clone()));
                }
                let schema = locked.as_ref().ok_or_else(|| JsValue::from_str("schema init failed"))?;
                let schema = schema.clone();
                drop(locked);

                let mut req = request_from_wire(&req_str)?;
                req = req.data(rt.clone()).data(rt.bus.clone());
                let mut stream = schema.execute_stream(req);
                while let Some(resp) = stream.next().await {
                    let json = serde_json::to_string(&async_graphql::Response::from(resp)).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let msg = JsValue::from_str(&json);
                    if cb.call1(&JsValue::UNDEFINED, &msg).is_err() {
                        break;
                    }
                }
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = free)]
        pub fn free(self) {
            drop(self.rt);
        }
    }
}

//#endregion 🔌 wasm_bridge

//#region 🧪 tests

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use async_graphql::{Request, Variables};
    use futures_lite::future::block_on;
    use serde_json::json;

    use crate::gql::AppSchema;

    /// @emoji 📜 `gql::sdl()` tracks the canonical target SDL file.
    #[test]
    fn schema_matches_target_graphql_file() {
        let disk = include_str!("../graphql/target.schema.graphql");
        let from_fn = block_on(crate::gql::sdl());
        assert_eq!(crate::gql::normalize_target_sdl(disk), crate::gql::normalize_target_sdl(&from_fn));
    }

    /// @emoji 🌱 Opens an unsaved kit change via `Mutation.session.theKit.startNewChange` (replaces legacy flat bootstrap mutations).
    async fn graphql_start_new_change(schema: &AppSchema) -> String {
        let res = schema.execute(Request::new(r#"mutation { session { theKit { startNewChange } } }"#)).await;
        assert!(res.errors.is_empty(), "startNewChange: {:?}", res.errors);
        res.data.into_json().unwrap()["session"]["theKit"]["startNewChange"].as_str().expect("change id").to_string()
    }

    /// @emoji 🌱 GraphQL tests open a target-schema unsaved change; the internal draft anchor stays hidden.
    async fn graphql_seed_defaults_and_open_tx(schema: &AppSchema) -> (String, String) {
        let tx_id = graphql_start_new_change(schema).await;
        ("the-kit".to_string(), tx_id)
    }

    fn position_value() -> serde_json::Value {
        json!({
            "center": { "u": 0.0, "v": 0.0 },
            "plane": {
                "origin": { "x": 0.0, "y": 0.0, "z": 0.0 },
                "xAxis":  { "x": 1.0, "y": 0.0, "z": 0.0 },
                "yAxis":  { "x": 0.0, "y": 1.0, "z": 0.0 }
            }
        })
    }

    fn add_fixed_piece_vars(transaction_id: &str, design_id: &str) -> Variables {
        Variables::from_value(async_graphql::value!({
            "tx": transaction_id,
            "designId": design_id,
            "bp": "bp-new",
            "pos": position_value(),
        }))
    }

    const ADD_FIXED_PIECE_TO_DESIGN: &str = r#"
        mutation($tx: ID!, $designId: ID!, $bp: ID!, $pos: PositionInput!) {
            session {
                theKit {
                    unsavedChange(id: $tx) {
                        kit {
                            design(id: $designId) {
                                addFixedPiece(blueprintId: $bp, position: $pos)
                            }
                        }
                    }
                }
            }
        }
    "#;

    fn relay_wip_designs_have_piece() -> &'static str {
        "{ wip { theKit { kit { designs { edges { node { id pieces { edges { node { id position { center { u v } } } } } } } } } } } }"
    }

    fn relay_auth_designs_piece_ids() -> &'static str {
        "{ authoritative { theKit { kit { designs { edges { node { pieces { edges { node { id } } } } } } } } } }"
    }

    /// 📤 Writes the executable schema's SDL to `SEMIO_GRAPHQL_SCHEMA_OUT`; run via `npx nx build semio/graphql`.
    #[test]
    #[ignore = "writes the generated SDL to SEMIO_GRAPHQL_SCHEMA_OUT"]
    fn export_semio_graphql_schema_file() {
        let out = std::env::var("SEMIO_GRAPHQL_SCHEMA_OUT").expect("SEMIO_GRAPHQL_SCHEMA_OUT");
        let path = std::path::PathBuf::from(out);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create schema output parent");
        }
        std::fs::write(path, block_on(crate::gql::sdl())).expect("write generated GraphQL schema");
    }

    /// 🛡️ Guard test: the crate must contain exactly **one** `pub async fn emit_event` definition.
    #[test]
    fn single_emit_event_in_codebase() {
        let src = include_str!("lib.rs");
        let needle = concat!("pub async fn ", "emit_event(&self, ev: Event)");
        let count = src.matches(needle).count();
        assert_eq!(count, 1, "expected exactly one canonical emit_event definition in lib.rs, found {}", count);
    }

    /// 🛡️ [`crate::worker::ChildRuntime`] must record operations and rely on materialization replay (`Kit::apply_diff` inside `Graph::materialized_kit_for_draft`); it must not replace `parent_root_for_active_draft` or call `apply_diff` directly.
    #[test]
    fn worker_child_runtime_guard_no_direct_root_or_apply_diff() {
        let src = include_str!("lib.rs");
        let i = src.find("impl ChildRuntime").expect("ChildRuntime impl");
        let j = src[i..].find("//#endregion 🧵 worker").expect("worker end marker") + i;
        let worker = &src[i..j];
        assert!(!worker.contains("parent_root_for_active_draft.write()"), "ChildRuntime must not assign Graph::parent_root_for_active_draft");
        assert!(!worker.contains("apply_diff"), "ChildRuntime must not call Kit::apply_diff; use record_op_in_open_transaction + materialized_kit_for_draft");
    }

    #[test]
    fn kit_store_bundle_serialize_hydrate_round_trip_via_graphql() {
        // 📸 Renames then hydrates via [`crate::kit_backbone::KitStoreBundleFile`] (bundle GraphQL entry points were dropped from the target schema).
        block_on(async {
            let rt = crate::worker::ParentRuntime::spawn().await;
            let g = rt.wip_graph.clone();
            let draft_a = g.ensure_default_seed_state().await;
            let tx_a = g.open_transaction(&draft_a.id).await;
            let req = crate::id::Id::new().await;
            let _ = rt
                .dispatch_wip(crate::operation::Command::ApplyKitOperation {
                    request_id: req,
                    draft_id: draft_a.id.clone(),
                    transaction_id: tx_a.id.clone(),
                    operation: crate::operation::KitOperation::RenameKit { scope: crate::operation::Scope::Kit, input: crate::operation::Input::Name { name: "Hello Bundle".into() } },
                })
                .await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let schema_a = crate::gql::build_schema_for(rt.clone());
            let q_baseline = r#"{
                wip {
                    initialKit { name }
                    theKit { kit { name } }
                    checkpoints { edges { node { initial { name } kit { name } } } }
                }
            }"#;
            let res = schema_a.execute(q_baseline).await;
            assert!(res.errors.is_empty(), "baseline query errors: {:?}", res.errors);
            let vr = res.data.into_json().unwrap();
            assert_eq!(vr["wip"]["theKit"]["kit"]["name"].as_str(), Some("Hello Bundle"), "materialized wip.theKit.kit");
            assert_eq!(vr["wip"]["initialKit"]["name"].as_str(), Some("the kit"), "graph.initialKit stays immutable");
            let cp_initial = vr["wip"]["checkpoints"]["edges"][0]["node"]["initial"]["name"].as_str().expect("checkpoint.initial.name");
            assert_eq!(cp_initial, "the kit", "checkpoint.initial must not alias live rename");

            g.abort_transaction(&draft_a.id, &tx_a.id).await.expect("abort");
            let res = schema_a.execute(q_baseline).await;
            assert!(res.errors.is_empty(), "baseline after abort: {:?}", res.errors);
            let vr = res.data.into_json().unwrap();
            assert_eq!(vr["wip"]["theKit"]["kit"]["name"].as_str(), Some("the kit"), "materialized kit reverts after abort");
            assert_eq!(vr["wip"]["initialKit"]["name"].as_str(), Some("the kit"));
            assert_eq!(vr["wip"]["checkpoints"]["edges"][0]["node"]["initial"]["name"].as_str(), Some("the kit"));

            let tx_a2 = g.open_transaction(&draft_a.id).await;
            let req2 = crate::id::Id::new().await;
            let _ = rt
                .dispatch_wip(crate::operation::Command::ApplyKitOperation {
                    request_id: req2,
                    draft_id: draft_a.id.clone(),
                    transaction_id: tx_a2.id.clone(),
                    operation: crate::operation::KitOperation::RenameKit { scope: crate::operation::Scope::Kit, input: crate::operation::Input::Name { name: "Hello Bundle".into() } },
                })
                .await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let json_a = serde_json::to_string(&crate::kit_backbone::KitStoreBundleFile::from_graph(g.as_ref()).await).expect("serialize bundle");

            let v: serde_json::Value = serde_json::from_str(&json_a).expect("bundle parses");
            assert_eq!(v["schema"].as_str().unwrap(), crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
            for k in ["wip", "authoritative", "stage", "conflicts", "blobs"].iter() {
                assert!(v.get(*k).is_some(), "missing top-level key {k}");
            }
            assert!(v["wip"]["initialKit"].is_object(), "wip.initialKit projects the live kit");
            assert_eq!(v["wip"]["initialKit"]["name"].as_str().unwrap_or(""), "the kit");
            assert!(!v["wip"]["checkpoints"]["items"].as_array().unwrap().is_empty(), "seed checkpoint present");
            assert!(v["wip"].get("drafts").is_none(), "bundle must not persist drafts");
            assert!(v["wip"].get("transactions").is_none(), "bundle must not persist transactions");
            assert!(v["wip"].get("savedChanges").is_none(), "graph must not persist savedChanges");
            assert!(v["wip"].get("unsavedChanges").is_none(), "graph must not persist unsavedChanges");
            assert!(!v["wip"]["theKit"]["unsavedChanges"]["items"].as_array().unwrap().is_empty(), "default unsaved change present on the kit version");

            let rt_b = crate::worker::ParentRuntime::spawn().await;
            crate::kit_backbone::KitStoreBundleFile::hydrate_into_graph(&rt_b.wip_graph, &json_a).await.expect("hydrate");

            let json_b = serde_json::to_string(&crate::kit_backbone::KitStoreBundleFile::from_graph(rt_b.wip_graph.as_ref()).await).expect("serialize bundle b");
            let vb: serde_json::Value = serde_json::from_str(&json_b).expect("bundle b parses");
            assert_eq!(vb["wip"]["initialKit"]["name"].as_str().unwrap_or(""), "the kit", "immutable baseline survives bundle round-trip");
        });
    }

    #[test]
    fn create_alternative_from_tip_graphql() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            const M: &str = r#"mutation($n: String!) { session { startAlternative(name: $n) } }"#;
            let res = schema.execute(Request::new(M).variables(Variables::from_value(async_graphql::value!({ "n": "branch-a" })))).await;
            assert!(res.errors.is_empty(), "startAlternative errors: {:?}", res.errors);
            let id: String = res.data.into_json().unwrap()["session"]["startAlternative"].as_str().expect("alt id").to_string();

            let q = r#"{ wip { alternatives { edges { node { id name } } } } }"#;
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "alternatives query errors: {:?}", res.errors);
            let v = res.data.into_json().unwrap();
            let edges = v["wip"]["alternatives"]["edges"].as_array().expect("edges");
            assert!(edges.iter().any(|e| e["node"]["id"].as_str() == Some(id.as_str())), "expected new alternative id in wip.alternatives");
            assert!(edges.iter().any(|e| e["node"]["name"].as_str() == Some("branch-a")));
        });
    }

    #[test]
    fn transaction_open_commit_abort_lifecycle_on_wip_graph() {
        block_on(async {
            let rt = crate::worker::ParentRuntime::spawn().await;
            let g = &rt.wip_graph;
            let draft_id = crate::id::Id::from("draft-tx-test");
            let tx_a = g.open_transaction(&draft_id).await;
            let draft = g.ensure_draft(&draft_id).await;
            assert_eq!(draft.open_transaction.read().await.upgrade().map(|t| t.id.clone()), Some(tx_a.id.clone()));
            let ordered: Vec<crate::id::Id> = draft.transactions.read().await.iter().map(|t| t.id.clone()).collect();
            assert_eq!(ordered, vec![tx_a.id.clone()]);

            g.commit_transaction(&draft_id, &tx_a.id).await.expect("commit");
            assert!(draft.open_transaction.read().await.upgrade().is_none());
            assert!(draft.transactions.read().await.is_empty());

            let tx_b = g.open_transaction(&draft_id).await;
            g.abort_transaction(&draft_id, &tx_b.id).await.expect("abort");
            assert!(draft.open_transaction.read().await.upgrade().is_none());
            assert!(draft.transactions.read().await.is_empty());

            assert!(g.commit_transaction(&draft_id, &crate::id::Id::from("missing")).await.is_err());
            assert!(g.abort_transaction(&crate::id::Id::from("missing-draft"), &tx_b.id).await.is_err());
        });
    }

    #[test]
    fn create_tag_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;

            const M: &str = r#"
                mutation($tx: ID!, $name: String!) {
                    session {
                        theKit {
                            unsavedChange(id: $tx) {
                                kit {
                                    createTag(name: $name)
                                }
                            }
                        }
                    }
                }
            "#;
            let vars = async_graphql::value!({
                "tx": tx_id,
                "name": "alpha-tag",
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createTag errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { kit { tags { edges { node { name } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["wip"]["theKit"]["kit"]["tags"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
            assert!(names.iter().any(|n| n == "alpha-tag"), "tags missing new name: {:?}", names);
        });
    }

    #[test]
    fn create_concept_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;

            const M: &str = r#"
                mutation($tx: ID!, $name: String!) {
                    session {
                        theKit {
                            unsavedChange(id: $tx) {
                                kit {
                                    createConcept(name: $name)
                                }
                            }
                        }
                    }
                }
            "#;
            let vars = async_graphql::value!({
                "tx": tx_id,
                "name": "beta-concept",
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createConcept errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { kit { concepts { edges { node { name } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["wip"]["theKit"]["kit"]["concepts"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
            assert!(names.iter().any(|n| n == "beta-concept"), "concepts missing new name: {:?}", names);
        });
    }

    #[test]
    fn create_quality_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;

            const M: &str = r#"
                mutation($tx: ID!, $key: String!, $value: String!) {
                    session {
                        theKit {
                            unsavedChange(id: $tx) {
                                kit {
                                    createQuality(key: $key, value: $value)
                                }
                            }
                        }
                    }
                }
            "#;
            let vars = async_graphql::value!({
                "tx": tx_id,
                "key": "q1",
                "value": "v1",
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createQuality errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { kit { qualities { edges { node { key value } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let keys: Vec<String> = data["wip"]["theKit"]["kit"]["qualities"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["key"].as_str().map(String::from)).collect();
            assert!(keys.iter().any(|k| k == "q1"), "qualities missing new key: {:?}", keys);
        });
    }

    #[test]
    fn graph_op_registry_row_count() {
        assert_eq!(crate::operation::GRAPH_OP_REGISTRY_ROWS, 98);
    }

    #[test]
    fn create_fixed_piece_end_to_end() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;
            let res = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(add_fixed_piece_vars(&tx_id, "des1"))).await;
            assert!(res.errors.is_empty(), "mutation errors: {:?}", res.errors);

            // The wip child applies asynchronously; wait briefly for the event loop.
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = relay_wip_designs_have_piece();
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let edges = data["wip"]["theKit"]["kit"]["designs"]["edges"].as_array().expect("design edges");
            let any_piece = edges.iter().any(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.iter().any(|_| true)).unwrap_or(false));
            assert!(any_piece, "expected at least one piece in wip; got: {}", serde_json::to_string_pretty(&data).unwrap());
        });
    }

    #[test]
    fn wip_and_authoritative_are_isolated() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;
            let _ = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(add_fixed_piece_vars(&tx_id, "des1"))).await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = relay_auth_designs_piece_ids();
            let res = schema.execute(q).await;
            let data = res.data.into_json().unwrap();
            let edges = data["authoritative"]["theKit"]["kit"]["designs"]["edges"].as_array().expect("auth design edges");
            let all_empty = edges.iter().all(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.is_empty()).unwrap_or(true));
            assert!(all_empty, "authoritative leaked pieces: {}", serde_json::to_string_pretty(&data).unwrap());
        });
    }

    /// 🛡️ Traversal must share Arcs, not deep-copy entities. Resolves a deep path and asserts
    /// the live `Arc<Piece>` strong count grows only by the bounded number of resolver hops
    /// that touch it (not by the number of pieces in the design).
    #[test]
    fn no_deep_clone_on_traversal() {
        block_on(async {
            let rt = crate::worker::ParentRuntime::spawn().await;
            let schema = crate::gql::build_schema_for(rt.clone());

            let draft = rt.wip_graph.ensure_default_seed_state().await;
            let tx = rt.wip_graph.open_transaction(&draft.id).await;

            // Insert two pieces directly via the wip graph (no GraphQL plumbing).
            let position = crate::geom::Position::default();
            let blueprint_id = crate::id::Id::new().await;
            let p1 = rt.wip_graph.apply_create_fixed_piece(draft.id.clone(), tx.id.clone(), crate::id::Id::from("des1"), blueprint_id.clone(), position, None, None).await.expect("insert piece 1").0;
            let _p2 = rt.wip_graph.apply_create_fixed_piece(draft.id.clone(), tx.id.clone(), crate::id::Id::from("des1"), blueprint_id, position, None, None).await.expect("insert piece 2").0;

            // Baseline strong count for p1: held by the design's pieces Vec + our local handle = 2.
            let baseline = Arc::strong_count(&p1);

            let q = relay_wip_designs_have_piece();
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "{:?}", res.errors);

            // After the query, only short-lived Arc clones may have been taken; once the resolver
            // chain completes they must all be dropped, leaving us at the baseline (or temporarily +1
            // if any local Arc clone hasn't been dropped yet — but never +2 per piece in the design).
            let after = Arc::strong_count(&p1);
            assert!(after <= baseline + 1, "deep-clone detected: strong_count grew from {} to {} after a single deep query", baseline, after);
        });
    }

    fn relay_piece_count_wip(data: &serde_json::Value) -> usize {
        let Some(edges) = data["wip"]["theKit"]["kit"]["designs"]["edges"].as_array() else {
            return 0;
        };
        edges.iter().map(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.len()).unwrap_or(0)).sum()
    }

    /// 🛡️ Mutation visibility without re-snapshotting: read wip, mutate, read wip again, second
    /// read must reflect the mutation. Proves the resolver sees the live Arc, not a snapshot.
    #[test]
    fn mutation_visible_without_resnapshotting() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let (_, tx_id) = graphql_seed_defaults_and_open_tx(&schema).await;
            let q = relay_wip_designs_have_piece();

            let before = schema.execute(q).await;
            let before_data = before.data.into_json().unwrap();
            let before_pieces = relay_piece_count_wip(&before_data);

            let _ = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(add_fixed_piece_vars(&tx_id, "des1"))).await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let after = schema.execute(q).await;
            let after_data = after.data.into_json().unwrap();
            let after_pieces = relay_piece_count_wip(&after_data);

            assert_eq!(after_pieces, before_pieces + 1, "mutation not visible on re-read; before={} after={}", before_pieces, after_pieces);
        });
    }

    /// 🗃️ Delegates to [`crate::kit_graph_engine::projection_fingerprint_for_kit`] (single implementation).
    pub async fn stable_projection_fingerprint(kit: &Arc<crate::kit::Kit>) -> String {
        crate::kit_graph_engine::projection_fingerprint_for_kit(kit.as_ref()).await
    }

    #[test]
    fn kit_store_golden_ops_replay_matches_expected_invariants() {
        block_on(async {
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.operations.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.operations")).expect("parse operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let draft_id = crate::id::Id::from(ops_json["draftId"].as_str().expect("draftId"));
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            for rec in ops_json["operations"].as_array().expect("operations") {
                let kind = rec["kind"].as_str().expect("operation kind");
                let input = &rec["input"];
                match kind {
                    "createdFixedPiece" => {
                        let design_id = crate::id::Id::from(input["designId"].as_str().expect("designId"));
                        let blueprint_id = crate::id::Id::from(input["blueprintId"].as_str().expect("blueprintId"));
                        let position: crate::geom::Position = serde_json::from_value(input["position"].clone()).expect("position serde");
                        let name = input.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let description = input.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                        g.apply_create_fixed_piece(draft_id.clone(), tx_id.clone(), design_id, blueprint_id, position, name, description).await.expect("apply createFixedPiece");
                    }
                    other => panic!("unsupported golden operation kind: {other}"),
                }
            }

            let inv = &exp["invariants"];
            let kit = g.materialized_kit_for_draft(&draft_id).await;
            let ds = kit.designs.read().await;
            assert_eq!(ds.len(), inv["designCount"].as_u64().expect("designCount") as usize, "designCount");
            let mut total = 0usize;
            let mut centers: Vec<[f64; 2]> = Vec::new();
            for d in ds.iter() {
                for p in d.pieces.read().await.iter() {
                    total += 1;
                    let guard = p.position.read().await;
                    let n = guard.as_ref().expect("piece position");
                    let pv = n.snapshot_value().await;
                    centers.push([pv.center.u, pv.center.v]);
                }
            }
            centers.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap().then_with(|| a[1].partial_cmp(&b[1]).unwrap()));
            assert_eq!(total, inv["totalPieces"].as_u64().expect("totalPieces") as usize, "totalPieces");
            let expect_centers: Vec<[f64; 2]> = serde_json::from_value(inv["sortedPieceCenters"].clone()).expect("sortedPieceCenters shape");
            assert_eq!(centers.len(), expect_centers.len(), "centers len");
            for (got, want) in centers.iter().zip(expect_centers.iter()) {
                assert!((got[0] - want[0]).abs() < 1e-9, "center u");
                assert!((got[1] - want[1]).abs() < 1e-9, "center v");
            }

            let fp = stable_projection_fingerprint(&g.materialized_kit_for_draft(&draft_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint in kit-store.golden.expected.semio.json");
            assert_eq!(fp, exp_fp, "projectionFingerprint");
        });
    }

    /// 🪡 `kit_graph_engine::apply__operation_json` must replay the same golden operations as manual apply.
    #[test]
    fn kit_store_golden_ops_via__op_json_match_fingerprint() {
        block_on(async {
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.operations.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.operations")).expect("parse operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let draft_id = crate::id::Id::from(ops_json["draftId"].as_str().expect("draftId"));
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            for rec in ops_json["operations"].as_array().expect("operations") {
                let kind = rec["kind"].as_str().expect("operation kind");
                let payload = serde_json::to_string(rec.get("input").expect("input")).expect("payload json");
                let applied = crate::kit_graph_engine::apply__operation_json(&g, &draft_id, &tx_id, kind, &payload).await.expect("apply__operation_json");
                assert!(applied.created_piece.is_some(), "expected piece for {kind}");
                assert!(applied.diff.summary.as_ref().map(|s| !s.is_empty()).unwrap_or(false), "diff summary");
            }

            let fp = stable_projection_fingerprint(&g.materialized_kit_for_draft(&draft_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "projectionFingerprint via  json apply");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dev_json_backbone_persisted_ops_replay_matches_us001_projection_fingerprint() {
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("dev-kit.json");

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.operations.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read operations")).expect("parse golden operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored operations");
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);
            let bundle = crate::kit_backbone::KitStoreBundleFile::from_stored__ops(&stored);
            std::fs::write(&path, serde_json::to_string_pretty(&bundle).expect("serialize kit-store bundle")).expect("write kit-store bundle");

            let g = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::operation::BackboneStoreKind::DevJson, "wip", &g).await.expect("dev json mount+replay");

            let draft_id = crate::id::Id::from(golden_ops["draftId"].as_str().expect("draftId"));
            let fp = stable_projection_fingerprint(&g.materialized_kit_for_draft(&draft_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "dev-json backbone replay must match US-001 golden fingerprint");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn local_semio_sqlite_backbone_persisted_ops_replay_matches_us001_projection_fingerprint() {
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let proj_root = dir.path().join("workspace");
            std::fs::create_dir_all(&proj_root).expect("mkdir workspace");
            let proj_canon = proj_root.canonicalize().expect("canonical workspace");
            let uri_full = format!("file://{}", proj_canon.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.operations.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read operations")).expect("parse golden operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored operations");

            let g_bootstrap = crate::vcs::Graph::new().await;
            let _bones = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::operation::BackboneStoreKind::LocalDotSemio, "wip", &g_bootstrap).await.expect("bootstrap .semio layout");

            let db_path = proj_canon.join(".semio").join("wip.db");
            let conn = rusqlite::Connection::open(&db_path).expect("open wip.db");
            for operation in &stored {
                let input_json = serde_json::to_string(&operation.input).expect("input json");
                conn.execute("INSERT INTO _op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![operation.draft_id, operation.transaction_id, operation.kind, input_json]).expect("insert  operation row");
            }
            drop(conn);

            let g2 = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::operation::BackboneStoreKind::LocalDotSemio, "wip", &g2).await.expect("replay wip.db");

            let draft_id = crate::id::Id::from(golden_ops["draftId"].as_str().expect("draftId"));
            let fp = stable_projection_fingerprint(&g2.materialized_kit_for_draft(&draft_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "local .semio backbone replay must match US-001 golden fingerprint");
        });
    }

    #[test]
    fn kit_store_bundle_metabolism_new_has_contract_shape() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/metabolism.new.kit.semio.json");
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read metabolism.new bundle")).expect("parse");
        for k in ["schema", "wip", "authoritative", "stage", "conflicts", "blobs"] {
            assert!(v.get(k).is_some(), "metabolism.new.kit.semio.json missing `{k}`");
        }
        assert_eq!(v.get("schema").and_then(|s| s.as_str()), Some(crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA), "metabolism.new.kit.semio.json schema marker drift");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_template_round_trips_metabolism_top_level_keys() {
        // 🧾 The empty bundle template is byte-stable in its top-level shape: serialise → deserialise → keys match.
        let bundle = crate::kit_backbone::KitStoreBundleFile::template();
        let s = serde_json::to_string_pretty(&bundle).expect("serialize empty template");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parse template");
        for k in ["schema", "wip", "authoritative", "stage", "conflicts", "blobs"] {
            assert!(v.get(k).is_some(), "empty template missing `{k}`");
        }
        assert_eq!(v["schema"], crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
        for graph_key in ["wip", "authoritative", "stage"] {
            for inner in ["id", "hash", "authors", "initialKit", "theKit", "checkpoints", "alternatives"] {
                assert!(v[graph_key].get(inner).is_some(), "graph `{graph_key}` missing `{inner}`");
            }
            assert!(v[graph_key].get("savedChanges").is_none(), "graph `{graph_key}` must not own savedChanges");
            assert!(v[graph_key].get("unsavedChanges").is_none(), "graph `{graph_key}` must not own unsavedChanges");
            assert!(v[graph_key]["theKit"].get("savedChanges").is_some(), "graph `{graph_key}.theKit` missing `savedChanges`");
            assert!(v[graph_key]["theKit"].get("unsavedChanges").is_some(), "graph `{graph_key}.theKit` missing `unsavedChanges`");
            assert!(v[graph_key]["initialKit"].get("types").is_some(), "graph `{graph_key}.initialKit` missing `types`");
            assert!(v[graph_key]["initialKit"].get("designs").is_some(), "graph `{graph_key}.initialKit` missing `designs`");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn json_array_or_block_items_helpers_accept_legacy_or_block_lists() {
        let flat = serde_json::json!([{"id":"a"}]);
        let block = serde_json::json!({"hash": crate::kit_backbone::KIT_BUNDLE_HASH_STUB,"items":[{"id":"b"}]});
        assert_eq!(crate::kit_backbone::json_array_or_block_items_ref(&flat).unwrap().len(), 1);
        assert_eq!(crate::kit_backbone::json_array_or_block_items_ref(&block).unwrap()[0]["id"], "b");
        let mut m = block.clone();
        assert!(crate::kit_backbone::json_array_or_block_items_mut(&mut m).unwrap()[0].get("id").is_some());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_bundle_purge_unreferenced_blob_rows() {
        let mut bundle = crate::kit_backbone::KitStoreBundleFile::template();
        bundle.blobs.items.push(serde_json::json!({ "hash": "orphan_digest_deadbeef", "blob": "data:,x" }));
        bundle.wip.root = serde_json::json!({
            "id": "k-purge",
            "name": "K",
            "createdAt": "2020-01-01T00:00:00.000Z",
            "updatedAt": "2020-01-01T00:00:00.000Z",
            "files": [],
        });
        crate::kit_backbone::KitStoreBundleFile::purge_unreferenced_blobs(&mut bundle);
        assert!(bundle.blobs.items.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_bundle_hoist_and_materialize_file_blobs_round_trip() {
        let blob_txt = "data:application/octet-stream;base64,QQ==";
        let dig = crate::kit_backbone::KitStoreBundleFile::digest_kit_blob_wire(blob_txt);
        let mut bundle = crate::kit_backbone::KitStoreBundleFile::template();
        bundle.wip.root = serde_json::json!({
            "id": "k-blob",
            "name": "K",
            "createdAt": "2020-01-01T00:00:00.000Z",
            "updatedAt": "2020-01-01T00:00:00.000Z",
            "files": [{
                "id": "f-blob",
                "name": "a.bin",
                "blob": blob_txt,
                "createdAt": "2020-01-01T00:00:00.000Z",
                "updatedAt": "2020-01-01T00:00:00.000Z",
            }],
        });
        crate::kit_backbone::KitStoreBundleFile::hoist_inline_file_blobs_for_storage(&mut bundle);
        assert!(bundle.wip.root["files"][0].as_object().expect("file obj").get("blob").is_none());
        assert_eq!(bundle.wip.root["files"][0]["blobHash"].as_str().expect("blobHash"), dig);
        assert_eq!(bundle.blobs.items.len(), 1);
        assert_eq!(bundle.blobs.items[0]["hash"].as_str().expect("blob row hash"), dig);
        let mut merged = bundle.wip.root.clone();
        crate::kit_backbone::KitStoreBundleFile::merge_bundle_file_blobs_into_kit_json(&mut merged, &bundle.blobs.items);
        assert_eq!(merged["files"][0]["blob"].as_str().expect("merged blob"), blob_txt);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_initialize_with_unsaved_change_seeds_root_checkpoint_and_change() {
        // 🌱 Bundle bootstrap matches sketchpad "create dev kit (json)": one root, one checkpoint, one unsaved change on the version.
        let bundle = crate::kit_backbone::KitStoreBundleFile::initialize_with_unsaved_change("kit-id-1", "change-1", "ckpt-1");
        assert_eq!(bundle.schema, crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
        assert_eq!(bundle.wip.id, "kit-id-1");
        assert_eq!(bundle.authoritative.id, "kit-id-1");
        assert_eq!(bundle.stage.id, "kit-id-1");
        assert_eq!(bundle.wip.checkpoints.items.len(), 1, "first checkpoint anchored on root");
        assert_eq!(bundle.wip.checkpoints.items[0]["id"], "ckpt-1");
        assert_eq!(bundle.wip.checkpoints.items[0]["message"], "init");
        assert!(bundle.wip.the_kit.saved_changes.items.is_empty(), "no saved changes at bootstrap");
        assert_eq!(bundle.wip.the_kit.unsaved_changes.items.len(), 1, "one active unsaved change on the kit version");
        let change = &bundle.wip.the_kit.unsaved_changes.items[0];
        assert_eq!(change.id, "change-1");
        assert!(change.edits.items.is_empty(), "no edits recorded yet");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dev_json_backbone_round_trip_persists_metabolism_shape_on_disk() {
        // 🔁 Mount (no file) → append operation via attached backbone → re-read on-disk JSON → confirm metabolism top-level + version change path.
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("dev-kit.json");
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);

            let g = crate::vcs::Graph::new().await;
            let mut bone = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::operation::BackboneStoreKind::DevJson, "wip", &g).await.expect("mount empty bundle");
            let draft_id = crate::id::Id::from("draft-rs-1");
            let tx_id = crate::id::Id::from("tx-rs-1");
            bone.append__op(&draft_id, &tx_id, "kit.design.piece.createdFixedPiece", &serde_json::json!({"designId": "d-1", "blueprintId": "b-1"})).expect("append operation");

            let raw = std::fs::read_to_string(&path).expect("read on-disk bundle");
            let v: serde_json::Value = serde_json::from_str(&raw).expect("parse on-disk bundle");
            for k in ["schema", "wip", "authoritative", "stage", "conflicts", "blobs"] {
                assert!(v.get(k).is_some(), "on-disk bundle missing `{k}` after append");
            }
            assert_eq!(v["schema"], crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
            assert!(v["wip"].get("drafts").is_none(), "bundle must not persist drafts");
            assert!(v["wip"].get("transactions").is_none(), "bundle must not persist transactions");
            assert!(v["wip"].get("unsavedChanges").is_none(), "graph must not own unsavedChanges");
            let changes = v["wip"]["theKit"]["unsavedChanges"]["items"].as_array().expect("wip.theKit unsavedChanges items array");
            assert_eq!(changes.len(), 1, "single unsaved change on disk");
            assert_eq!(changes[0]["id"], "tx-rs-1");
            let edits = changes[0]["edits"]["items"].as_array().expect("edit items");
            assert_eq!(edits.len(), 1, "single edit on disk");
            let fwd = edits[0]["forwards"]["items"].as_array().expect("forwards items");
            assert_eq!(fwd.len(), 1, "single forward step on disk");
            assert_eq!(fwd[0]["kind"], "kit.design.piece.createdFixedPiece");
            assert_eq!(fwd[0]["input"]["designId"], "d-1");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_append_unsaved_edit_creates_change_and_edit_paths() {
        // ➕ Appending to a fresh template materialises wip.theKit.unsavedChanges[0].edits[0].forwards[0] without touching authoritative/stage.
        let mut bundle = crate::kit_backbone::KitStoreBundleFile::template();
        bundle.append_unsaved_edit("change-y", "kit.design.piece.createdFixedPiece", serde_json::json!({"hello": "world"}));
        assert_eq!(bundle.wip.the_kit.unsaved_changes.items.len(), 1, "wip.theKit unsaved change created");
        let change = &bundle.wip.the_kit.unsaved_changes.items[0];
        assert_eq!(change.id, "change-y");
        assert_eq!(change.edits.items.len(), 1, "edit created under change");
        let edit = &change.edits.items[0];
        assert_eq!(edit.forwards.items.len(), 1, "single forward step appended");
        let step = &edit.forwards.items[0];
        assert_eq!(step.kind, "kit.design.piece.createdFixedPiece");
        assert_eq!(step.input["hello"], "world");
        assert!(bundle.authoritative.the_kit.unsaved_changes.items.is_empty(), "authoritative untouched");
        assert!(bundle.stage.the_kit.unsaved_changes.items.is_empty(), "stage untouched");

        // Appending another step into the same change grows the same edit.
        bundle.append_unsaved_edit("change-y", "kit.design.piece.deletedFixedPieces", serde_json::json!({"pieceIds": []}));
        assert_eq!(bundle.wip.the_kit.unsaved_changes.items.len(), 1, "no extra change");
        assert_eq!(bundle.wip.the_kit.unsaved_changes.items[0].edits.items.len(), 1, "no extra edit");
        assert_eq!(bundle.wip.the_kit.unsaved_changes.items[0].edits.items[0].forwards.items.len(), 2, "two forward steps");

        // Flatten replays everything in order from wip version changes.
        let flat = bundle.wip__ops();
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].draft_id, "the-kit");
        assert_eq!(flat[0].transaction_id, "change-y");
        assert_eq!(flat[0].kind, "kit.design.piece.createdFixedPiece");
        assert_eq!(flat[1].kind, "kit.design.piece.deletedFixedPieces");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_projects_alternative_version_change_lanes() {
        block_on(async {
            let graph = crate::vcs::Graph::new().await;
            let alt_id = graph.create_alternative_from_tip("branch-a".to_string(), None).await.expect("alternative");
            let bundle = crate::kit_backbone::KitStoreBundleFile::from_graph(graph.as_ref()).await;
            assert!(bundle.wip.the_kit.saved_changes.items.is_empty());
            let alt = bundle.wip.alternatives.items.iter().find(|a| a.id == alt_id.as_str()).expect("alternative dto");
            assert_eq!(alt.name, "branch-a");
            assert!(alt.saved_changes.items.is_empty(), "alternative owns savedChanges");
            assert!(alt.unsaved_changes.items.is_empty(), "alternative owns unsavedChanges");
            let raw = serde_json::to_value(&bundle).expect("bundle json");
            assert!(raw["wip"].get("savedChanges").is_none(), "graph must not own savedChanges");
            assert!(raw["wip"]["alternatives"]["items"][0].get("savedChanges").is_some(), "alternative must own savedChanges");
            assert!(raw["wip"]["alternatives"]["items"][0].get("unsavedChanges").is_some(), "alternative must own unsavedChanges");
        });
    }

    #[test]
    fn normalized_kit_operation_create_tag_diff_and_backwards_use_scoped_ids() {
        block_on(async {
            let graph = crate::vcs::Graph::new().await;
            let kit = graph.parent_root_for_active_draft.read().await.clone();
            let owner_id = kit.workspace_kit_id().await;
            let tag_id = crate::id::Id::from("tag-scope-1");
            let attribute_id = crate::id::Id::from("attr-scope-1");
            let tag_input = crate::meta::TagInput {
                name: "alpha-tag".to_string(),
                description: Some("tag description".to_string()),
                icon: Some("tag-icon".to_string()),
                order: Some(3),
                attributes: Some(vec![crate::meta::AttributeInput { key: "material".to_string(), value: Some("steel".to_string()), definition: Some("visible material".to_string()) }]),
            };
            let create = crate::operation::KitOperation::CreateTag {
                scope: crate::operation::Scope::CreateTag { owner_id: owner_id.clone(), tag_id: tag_id.clone(), attribute_ids: vec![attribute_id.clone()] },
                input: crate::operation::Input::Tag { tag: tag_input.clone() },
            };

            let diff = create.to_diff(&kit).await.expect("createTag diff");
            let tags = diff.0.tags.as_ref().expect("tags collection diff");
            assert_eq!(tags.added.len(), 1, "single added tag row");
            let row = &tags.added[0];
            assert_eq!(row.get("id").and_then(|v| v.as_str()), Some(tag_id.as_str()));
            assert_eq!(row.get("ownerId").and_then(|v| v.as_str()), Some(owner_id.as_str()));
            assert_eq!(row.get("name").and_then(|v| v.as_str()), Some("alpha-tag"));

            let staged = kit.deep_clone().await;
            staged.apply_diff(&diff).await.expect("apply createTag diff on clone");

            let backwards = crate::operation::KitOperation::DeleteTag { scope: crate::operation::Scope::Tag { tag_id: tag_id.clone() }, input: crate::operation::Input::None }.to_backwards(&staged).await.expect("deleteTag backwards");
            assert_eq!(backwards.len(), 1);
            match &backwards[0] {
                crate::operation::KitOperation::CreateTag { scope, input } => {
                    let crate::operation::Scope::CreateTag { owner_id: o, tag_id: t, attribute_ids } = scope else {
                        panic!("expected CreateTag scope");
                    };
                    assert_eq!(o, &owner_id);
                    assert_eq!(t, &tag_id);
                    assert_eq!(attribute_ids, &vec![attribute_id.clone()]);
                    let crate::operation::Input::Tag { tag } = input else {
                        panic!("expected Tag input");
                    };
                    assert_eq!(tag.name, "alpha-tag");
                    assert_eq!(tag.description.as_deref(), Some("tag description"));
                }
                other => panic!("unexpected backwards operation: {:?}", other),
            }
        });
    }

    /// @emoji 📦 `metabolism.kit.diff.semio.json` deserializes as [`crate::operation::CanonicalKitDiff`] and round-trips through `serde_json::Value` without structural drift.
    #[test]
    fn canonical_kit_diff_metabolism_fixture_json_round_trip() {
        const FIXTURE: &str = include_str!("../assets/semio/metabolism.kit.diff.semio.json");
        let raw: serde_json::Value = serde_json::from_str(FIXTURE).expect("fixture parses as JSON");
        let parsed: crate::operation::CanonicalKitDiff = serde_json::from_value(raw.clone()).expect("fixture maps to CanonicalKitDiff");
        let round = serde_json::to_value(&parsed).expect("CanonicalKitDiff serializes");
        assert_eq!(round, raw, "canonical kit diff fixture must round-trip through CanonicalKitDiff serde");
    }

    //#region 🪪 merkle hashing
    #[test]
    fn merkle_node_str_sorts_child_digests_for_order_independence() {
        let a = crate::hash::merkle_node_str(&["semio:test:node", "id-1"], vec!["zzz".into(), "aaa".into(), "mmm".into()]);
        let b = crate::hash::merkle_node_str(&["semio:test:node", "id-1"], vec!["mmm".into(), "zzz".into(), "aaa".into()]);
        assert_eq!(a, b, "child digest order must not affect the parent hash");
    }

    #[test]
    fn merkle_collection_matches_tagged_empty_node() {
        let empty_coll = crate::hash::merkle_collection(Vec::new());
        let tagged = crate::hash::merkle_node_str(&["semio:relay:collection"], Vec::new());
        assert_eq!(empty_coll, tagged);
    }

    #[test]
    fn merkle_collection_is_order_independent() {
        let x = crate::hash::merkle_collection(vec!["3".into(), "1".into(), "2".into()]);
        let y = crate::hash::merkle_collection(vec!["2".into(), "3".into(), "1".into()]);
        assert_eq!(x, y);
    }

    #[test]
    fn file_serde_round_trips_blob_digest_as_hash() {
        let v = json!({
            "id": "019caa00-0000-7000-a000-000000000021",
            "url": "https://example.com/f",
            "hash": "sha256:abc",
            "mime": serde_json::Value::Null,
            "size": serde_json::Value::Null,
            "description": serde_json::Value::Null,
            "created": serde_json::Value::Null,
            "updated": serde_json::Value::Null
        });
        let f: crate::meta::File = serde_json::from_value(v.clone()).expect("File from JSON");
        assert_eq!(f.hash, "sha256:abc");
        let out = serde_json::to_value(&f).expect("File serde_json::to_value");
        assert_eq!(out.get("hash").and_then(|x| x.as_str()), Some("sha256:abc"));
    }

    #[test]
    fn geom_plane_compute_hash_stable() {
        block_on(async {
            let pl =
                crate::geom::entity::PlaneNode::from_value(crate::geom::Plane { origin: crate::geom::Point { x: 0.0, y: 0.0, z: 0.0 }, x_axis: crate::geom::Vector { x: 1.0, y: 0.0, z: 0.0 }, y_axis: crate::geom::Vector { x: 0.0, y: 1.0, z: 0.0 } });
            assert_eq!(pl.compute_hash().await, pl.compute_hash().await);
        });
    }

    /// @emoji 🛡️ Relay connection hashes must fold sorted child digests, not legacy id-join helpers.
    #[test]
    fn guard_gql_relay_has_no_legacy_id_join_hash() {
        let src = include_str!("lib.rs");
        let i = src.find("pub mod gql_relay").expect("gql_relay module");
        let j = src[i..].find("//#endregion 🪢 gql_relay").expect("gql_relay end") + i;
        let relay = &src[i..j];
        let needle = concat!("hash", "_ids");
        assert!(!relay.contains(needle), "gql_relay must not contain legacy id-join hash helper; use merkle_collection / entity compute_hash digests");
    }
    //#endregion 🪪 merkle hashing

    #[test]
    fn normalized_create_fixed_piece_replay_reuses_scoped_piece_id() {
        block_on(async {
            let graph = crate::vcs::Graph::new().await;
            let draft_id = crate::id::Id::from("draft-scoped-1");
            let operation = crate::operation::KitOperation::CreateFixedPiece {
                scope: crate::operation::Scope::CreateFixedPiece {
                    design_id: crate::id::Id::from("design-scoped-1"),
                    piece_id: crate::id::Id::from("piece-scoped-1"),
                    blueprint_id: crate::id::Id::from("blueprint-scoped-1"),
                    attribute_ids: Vec::new(),
                },
                input: crate::operation::Input::FixedPiece { position: crate::geom::Position::default(), name: Some("Scoped Piece".to_string()), description: Some("Persisted with explicit scope ids".to_string()) },
            };

            let payload = operation.payload_json().expect("payload json");
            let applied = crate::kit_graph_engine::apply__operation_json(&graph, &draft_id, &crate::id::Id::from("tx-scoped-1"), operation.kind(), &payload).await.expect("apply normalized createFixedPiece");

            let piece = applied.created_piece.expect("created piece");
            assert_eq!(piece.id, crate::id::Id::from("piece-scoped-1"));
            assert_eq!(piece.name.read().await.clone().as_deref(), Some("Scoped Piece"));

            let mat = graph.materialized_kit_for_draft(&draft_id).await;
            let design = mat.design_by_external_id(&crate::id::Id::from("design-scoped-1")).await.expect("design exists");
            assert!(design.piece_by_external_id(&crate::id::Id::from("piece-scoped-1")).await.is_some(), "piece should be addressable by scoped id");
        });
    }
}

//#endregion 🧪 tests
