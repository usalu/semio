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
//! semantic diffs, and `projectionFingerprint` aligned with kit-store golden fixtures.

#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

//#region 🆔 id

pub mod id {
    //! 🆔 Immutable uuid-v7 wrapper used by every entity.
    use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
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

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "PlaneInput")]
    pub struct Plane {
        pub origin: Point,
        #[graphql(name = "xAxis")]
        #[serde(alias = "xAxis")]
        pub x_axis: Vector,
        #[graphql(name = "yAxis")]
        #[serde(alias = "yAxis")]
        pub y_axis: Vector,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, InputObject)]
    #[graphql(name = "PositionInput")]
    pub struct Position {
        pub center: Coordinate,
        pub plane: Plane,
    }

    //#region 📐 entity
    pub mod entity {
        //! 📐 `Arc` geometry nodes (target WeakEntity / Entity graph shapes); `#[Object]` impls live after [`crate::iface`].
        use std::sync::Arc;

        use async_lock::RwLock;

        use crate::hash::h;
        use crate::id::Id;

        use super::{Coordinate, Plane, Point, Position, Vector};

        fn weak(prefix: &str, parts: &[&str]) -> Id {
            Id::from(format!("semio:weak:{prefix}:{}", h(parts)))
        }

        /// @emoji 📍 Coordinate WeakEntity data node.
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
        }

        /// @emoji ↗ Vector WeakEntity data node.
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
        }

        /// @emoji ◆ Point WeakEntity data node.
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
        }

        /// @emoji ▭ Plane WeakEntity data node (owns origin + axes).
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
        }

        /// @emoji ↖ WeakEntity-style offset (piece drag input echo).
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
        }

        /// @emoji ⌖ Position WeakEntity root (center + plane); mirrors live [`super::Position`] DTO via RwLock sync.
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
        }

        /// @emoji 🧭 Placeholder StrongEntity shell for `Place` (full meta wiring lands with meta lift).
        pub struct PlaceNode {
            pub id: Id,
            pub label: RwLock<Option<String>>,
        }

        impl PlaceNode {
            pub async fn new() -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, label: RwLock::new(None) })
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
    use blake3::Hasher;

    use crate::id::Id;
    use crate::kit::design::piece::Piece;
    use crate::kit::design::Design;
    use crate::kit::r#type::Type;
    use crate::meta::{Author, Benchmark, Concept, File, Folder, Group, Layer, Prop, Quality, Stat, Tag};
    use crate::vcs::{Alternative, Checkpoint, Conflict};

    fn edge_cursor(i: usize) -> String {
        format!("e{i}")
    }

    fn hash_ids(ids: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut hasher = Hasher::new();
        for id in ids {
            hasher.update(id.as_ref().as_bytes());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
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
        pub fn from_designs(rows: Vec<Arc<Design>>) -> Self {
            let hash = hash_ids(rows.iter().map(|d| d.id.as_str()));
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
        pub fn from_pieces(rows: Vec<Arc<Piece>>) -> Self {
            let hash = hash_ids(rows.iter().map(|p| p.id.as_str()));
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
        pub fn from_types(rows: Vec<Arc<Type>>) -> Self {
            let hash = hash_ids(rows.iter().map(|t| t.id.as_str()));
            let edges = rows.into_iter().enumerate().map(|(i, t)| TypeEdge { cursor: edge_cursor(i), node: t }).collect();
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
        pub fn from_conflicts(rows: Vec<Arc<Conflict>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
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
        pub fn from_alternatives(rows: Vec<Arc<Alternative>>) -> Self {
            let hash = hash_ids(rows.iter().map(|a| a.id.as_str()));
            let edges = rows.into_iter().enumerate().map(|(i, a)| AlternativeEdge { cursor: edge_cursor(i), node: a }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
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
        pub fn from_checkpoints(rows: Vec<Arc<Checkpoint>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
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
        pub fn from_connections(rows: Vec<Arc<crate::kit::design::connection::Connection>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
            let edges = rows.into_iter().enumerate().map(|(i, node)| ConnectionEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    macro_rules! simple_conn {
        ($Conn:ident, $Edge:ident, $node:ty, $id_closure:expr) => {
            #[derive(Clone, SimpleObject)]
            pub struct $Edge {
                pub cursor: String,
                pub node: $node,
            }

            #[derive(Clone, SimpleObject)]
            pub struct $Conn {
                pub edges: Vec<$Edge>,
                #[graphql(name = "pageInfo")]
                pub page_info: std::sync::Arc<PageInfo>,
                pub hash: String,
            }

            impl $Conn {
                pub fn from_rows(rows: Vec<$node>) -> Self {
                    let id_fn = $id_closure;
                    let mut hasher = Hasher::new();
                    for r in &rows {
                        let id = id_fn(r);
                        hasher.update(id.as_str().as_bytes());
                        hasher.update(b"\x1f");
                    }
                    let hash = hasher.finalize().to_hex().to_string();
                    let edges = rows.into_iter().enumerate().map(|(i, node)| $Edge { cursor: edge_cursor(i), node }).collect();
                    Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
                }
            }
        };
    }

    simple_conn!(FileConnection, FileEdge, File, |f: &File| f.id.clone());
    simple_conn!(FolderConnection, FolderEdge, Folder, |f: &Folder| f.id.clone());
    simple_conn!(AuthorConnection, AuthorEdge, Author, |a: &Author| a.id.clone());
    simple_conn!(ConceptConnection, ConceptEdge, std::sync::Arc<Concept>, |c: &std::sync::Arc<Concept>| c.id.clone());
    simple_conn!(TagConnection, TagEdge, std::sync::Arc<Tag>, |t: &std::sync::Arc<Tag>| t.id.clone());
    simple_conn!(QualityConnection, QualityEdge, std::sync::Arc<Quality>, |q: &std::sync::Arc<Quality>| q.id.clone());
    simple_conn!(BenchmarkConnection, BenchmarkEdge, Benchmark, |b: &Benchmark| b.id.clone());
    simple_conn!(PropConnection, PropEdge, Prop, |p: &Prop| p.id.clone());
    simple_conn!(AttributeConnection, AttributeEdge, crate::meta::Attribute, |a: &crate::meta::Attribute| a.id.clone());
    simple_conn!(StatConnection, StatEdge, Stat, |s: &Stat| s.id.clone());
    simple_conn!(LayerConnection, LayerEdge, Layer, |l: &Layer| l.id.clone());
    simple_conn!(GroupConnection, GroupEdge, Group, |g: &Group| g.id.clone());
    simple_conn!(PositionNodeConnection, PositionNodeEdge, Arc<crate::geom::entity::PositionNode>, |p: &Arc<crate::geom::entity::PositionNode>| p.id.clone());

    /// @emoji 🪢 `entity_relay!` — delegates to [`simple_conn!`] for `Arc`/value nodes with an id extractor closure.
    macro_rules! entity_relay {
        ($Conn:ident, $Edge:ident, $Node:ty, $id_expr:expr) => {
            simple_conn!($Conn, $Edge, $Node, $id_expr);
        };
    }

    entity_relay!(VectorNodeConnection, VectorNodeEdge, Arc<crate::geom::entity::VectorNode>, |v: &Arc<crate::geom::entity::VectorNode>| v.id.clone());
    entity_relay!(CoordinateNodeConnection, CoordinateNodeEdge, Arc<crate::geom::entity::CoordinateNode>, |c: &Arc<crate::geom::entity::CoordinateNode>| c.id.clone());
    entity_relay!(PointNodeConnection, PointNodeEdge, Arc<crate::geom::entity::PointNode>, |p: &Arc<crate::geom::entity::PointNode>| p.id.clone());
    entity_relay!(PlaneNodeConnection, PlaneNodeEdge, Arc<crate::geom::entity::PlaneNode>, |p: &Arc<crate::geom::entity::PlaneNode>| p.id.clone());
    entity_relay!(OffsetNodeConnection, OffsetNodeEdge, Arc<crate::geom::entity::OffsetNode>, |o: &Arc<crate::geom::entity::OffsetNode>| o.id.clone());

    /// @emoji 🪜 `entity_diffs!` — expands modification / diff / diffs relay ladder (hook for codegen; invoke per entity family).
    macro_rules! entity_diffs {
        ($($_base:ident),* $(,)?) => {};
    }

    /// @emoji 🪢 `entity_owner!` — expands owner/owned union shells (hook for codegen).
    macro_rules! entity_owner {
        ($($_base:ident),* $(,)?) => {};
    }

    /// @emoji 🧷 Placeholder `Family` row until the kit family model is lifted into Arc entities.
    #[derive(Clone, Debug, Default, SimpleObject)]
    pub struct Family {
        pub id: Id,
    }

    simple_conn!(FamilyConnection, FamilyEdge, Family, |f: &Family| f.id.clone());
}

//#endregion 🪢 gql_relay

//#region 🏷️ meta

pub mod meta {
    //! 🏷️ Metadata: DTO [`SimpleObject`] shells plus Arc-backed [`Tag`]/[`Concept`]/[`Quality`] entities (SDL `Entity`).
    use std::sync::{Arc, Weak};

    use async_graphql::{InputObject, Object, SimpleObject};
    use async_lock::RwLock;
    use serde::{Deserialize, Serialize};

    use crate::id::Id;
    use crate::timestamp::Timestamp;

    //#region 🧾 graphql inputs
    /// @emoji 🧾 SDL `AttributeInput` — instantiates [`Attribute`] rows on entity create/update paths.
    #[derive(Clone, Debug, Default, InputObject)]
    pub struct AttributeInput {
        pub key: String,
        pub value: Option<String>,
        pub definition: Option<String>,
    }

    /// @emoji 🧾 SDL `TagInput`.
    #[derive(Clone, Debug, Default, InputObject)]
    pub struct TagInput {
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub order: Option<i32>,
        pub attributes: Option<Vec<AttributeInput>>,
    }

    /// @emoji 🧾 SDL `ConceptInput`.
    #[derive(Clone, Debug, Default, InputObject)]
    pub struct ConceptInput {
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub order: Option<i32>,
        pub attributes: Option<Vec<AttributeInput>>,
    }

    /// @emoji 🧾 SDL `QualityInput` (subset aligned to persisted kit fields).
    #[derive(Clone, Debug, Default, InputObject)]
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
    }

    /// @emoji ➕ Expand optional GraphQL attribute rows into minted [`Attribute`] entities.
    pub async fn attributes_from_inputs(inp: Option<Vec<AttributeInput>>) -> Vec<Attribute> {
        let mut v = Vec::new();
        for a in inp.into_iter().flatten() {
            v.push(a.into_attribute().await);
        }
        v
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Location {
        pub id: Id,
    }

    /// 🏷️ Hand union for `Location.owner` (Place | Kit | Design).
    #[derive(Clone, async_graphql::Union)]
    pub enum LocationOwner {
        Place(std::sync::Arc<crate::geom::entity::PlaceNode>),
        Kit(std::sync::Arc<crate::kit::Kit>),
        Design(std::sync::Arc<crate::kit::design::Design>),
    }

    #[Object(name = "Location")]
    impl Location {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> LocationOwner {
            LocationOwner::Kit(std::sync::Arc::new(crate::kit::Kit::default()))
        }
        #[graphql(name = "placeOwner")]
        pub async fn place_owner(&self) -> Option<std::sync::Arc<crate::geom::entity::PlaceNode>> {
            None
        }
        #[graphql(name = "kitOwner")]
        pub async fn kit_owner(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
            None
        }
        #[graphql(name = "designOwner")]
        pub async fn design_owner(&self) -> Option<std::sync::Arc<crate::kit::design::Design>> {
            None
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn longitude(&self) -> f64 {
            0.0
        }
        pub async fn latitude(&self) -> f64 {
            0.0
        }
        pub async fn altitude(&self) -> f64 {
            0.0
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct File {
        pub id: Id,
        pub url: String,
        pub mime: Option<String>,
        pub size: Option<i32>,
        pub hash: String,
        pub description: Option<String>,
        pub created: Option<Timestamp>,
        pub updated: Option<Timestamp>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Folder {
        pub id: Id,
        pub path: String,
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Author {
        pub id: Id,
        pub name: String,
        pub email: String,
        pub role: Option<String>,
        pub rank: Option<i32>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Attribute {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub definition: Option<String>,
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
            crate::hash::h(&[self.id.as_str(), self.key.as_str()])
        }
        pub async fn owner(&self) -> AttributeOwner {
            AttributeOwner::Kit(std::sync::Arc::new(crate::kit::Kit::default()))
        }
        #[graphql(name = "pieceOwner")]
        pub async fn piece_owner(&self) -> Option<std::sync::Arc<crate::kit::design::piece::Piece>> {
            None
        }
        #[graphql(name = "connectorOwner")]
        pub async fn connector_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Connector>> {
            None
        }
        #[graphql(name = "representationOwner")]
        pub async fn representation_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Representation>> {
            None
        }
        #[graphql(name = "connectionOwner")]
        pub async fn connection_owner(&self) -> Option<std::sync::Arc<crate::kit::design::connection::Connection>> {
            None
        }
        #[graphql(name = "kitOwner")]
        pub async fn kit_owner(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
            None
        }
        #[graphql(name = "designOwner")]
        pub async fn design_owner(&self) -> Option<std::sync::Arc<crate::kit::design::Design>> {
            None
        }
        #[graphql(name = "typeOwner")]
        pub async fn type_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
            None
        }
        #[graphql(name = "conceptOwner")]
        pub async fn concept_owner(&self) -> Option<std::sync::Arc<crate::meta::Concept>> {
            None
        }
        #[graphql(name = "tagOwner")]
        pub async fn tag_owner(&self) -> Option<std::sync::Arc<crate::meta::Tag>> {
            None
        }
        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
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

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Prop {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        #[graphql(skip)]
        #[serde(skip)]
        pub quality: Option<std::sync::Arc<Quality>>,
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

        pub async fn compute_hash(&self) -> String {
            let n = self.name.read().await;
            let d = self.description.read().await.clone().unwrap_or_default();
            crate::hash::h(&[self.id.as_str(), n.as_str(), d.as_str()])
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

        pub async fn compute_hash(&self) -> String {
            let n = self.name.read().await;
            let d = self.description.read().await.clone().unwrap_or_default();
            crate::hash::h(&[self.id.as_str(), n.as_str(), d.as_str()])
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

        pub async fn compute_hash(&self) -> String {
            let k = self.key.read().await;
            let v = self.value.read().await.clone().unwrap_or_default();
            crate::hash::h(&[self.id.as_str(), k.as_str(), v.as_str()])
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
    pub struct Stat {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Layer {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub color: Option<String>,
        pub order: Option<i32>,
        pub visible: Option<bool>,
        pub locked: Option<bool>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Group {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub color: Option<String>,
        pub icon: Option<String>,
        #[graphql(skip)]
        pub piece_ids: Vec<Id>,
    }
}

//#endregion 🏷️ meta

//#region 🪪 hash

pub mod hash {
    //! 🪪 Stable content hash helper used by every entity's `hash` resolver.
    use blake3::Hasher;

    pub fn h<S: AsRef<[u8]>>(parts: &[S]) -> String {
        let mut hasher = Hasher::new();
        for p in parts {
            hasher.update(p.as_ref());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
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
            #[graphql(name = "ownerEntity")]
            pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                None
            }
            #[graphql(name = "ownedEntities")]
            pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                Some(crate::iface::empty_owned_entity_connection())
            }
        }
        //#endregion 🛟 port

        //#region ⚓ connector
        pub struct Connector {
            pub id: Id,
            pub owner_type: Weak<Type>,
            pub code: RwLock<String>,
            pub description: RwLock<Option<String>>,
            /// @emoji 🔗 Resolved port via `Weak` (no id scan in GraphQL resolvers).
            pub port: RwLock<Weak<Port>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Connector {
            fn default() -> Self {
                Self { id: Id::default(), owner_type: Weak::new(), code: RwLock::new(String::new()), description: RwLock::new(None), port: RwLock::new(Weak::new()), qualities: RwLock::new(Vec::new()), attributes: RwLock::new(Vec::new()) }
            }
        }

        impl Connector {
            pub async fn new(owner_type: Weak<Type>, code: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_type, code: RwLock::new(code), description: RwLock::new(None), port: RwLock::new(Weak::new()), qualities: RwLock::new(Vec::new()), attributes: RwLock::new(Vec::new()) })
            }

            pub async fn compute_hash(&self) -> String {
                let code = self.code.read().await;
                let desc = self.description.read().await;
                h(&[self.id.as_str(), code.as_str(), desc.as_deref().unwrap_or("")])
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
            pub async fn code(&self) -> String {
                self.code.read().await.clone()
            }
            pub async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            pub async fn port(&self) -> Option<Arc<Port>> {
                self.port.read().await.upgrade()
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
            }
            pub async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
            }
            #[graphql(name = "ownerEntity")]
            pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                None
            }
            #[graphql(name = "ownedEntities")]
            pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                Some(crate::iface::empty_owned_entity_connection())
            }
        }
        //#endregion ⚓ connector

        //#region 💾 representation
        pub struct Representation {
            pub id: Id,
            pub owner_type: Weak<Type>,
            pub url: RwLock<String>,
            pub description: RwLock<Option<String>>,
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
                    url: RwLock::new(String::new()),
                    description: RwLock::new(None),
                    file: RwLock::new(None),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                }
            }
        }

        impl Representation {
            pub async fn new(owner_type: Weak<Type>, url: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_type, url: RwLock::new(url), description: RwLock::new(None), file: RwLock::new(None), tags: RwLock::new(Vec::new()), qualities: RwLock::new(Vec::new()), attributes: RwLock::new(Vec::new()) })
            }

            pub async fn compute_hash(&self) -> String {
                let url = self.url.read().await;
                let desc = self.description.read().await;
                h(&[self.id.as_str(), url.as_str(), desc.as_deref().unwrap_or("")])
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
            pub async fn url(&self) -> String {
                self.url.read().await.clone()
            }
            pub async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            pub async fn file(&self) -> Option<File> {
                self.file.read().await.clone()
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone())
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
            }
            pub async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
            }
            #[graphql(name = "ownerEntity")]
            pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                None
            }
            #[graphql(name = "ownedEntities")]
            pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                Some(crate::iface::empty_owned_entity_connection())
            }
        }
        //#endregion 💾 representation

        //#region 🏠 type
        pub struct Type {
            pub id: Id,
            pub owner_kit: Weak<crate::kit::Kit>,
            pub name: RwLock<String>,
            pub description: RwLock<Option<String>>,
            pub icon: RwLock<Option<String>>,
            pub image: RwLock<Option<String>>,
            pub unit: RwLock<Option<String>>,
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
                    description: RwLock::new(None),
                    icon: RwLock::new(None),
                    image: RwLock::new(None),
                    unit: RwLock::new(None),
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
                h(&[self.id.as_str(), name.as_str(), desc.as_deref().unwrap_or("")])
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
            pub async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> Option<String> {
                self.icon.read().await.clone()
            }
            pub async fn image(&self) -> Option<String> {
                self.image.read().await.clone()
            }
            pub async fn unit(&self) -> Option<String> {
                self.unit.read().await.clone()
            }
            pub async fn created(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            pub async fn updated(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            pub async fn connectors(&self) -> Vec<Arc<Connector>> {
                self.connectors.read().await.clone()
            }
            pub async fn connector(&self, id: Id) -> Option<Arc<Connector>> {
                self.refresh_connector_child_weak_maps().await;
                self.connector_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            pub async fn representations(&self) -> Vec<Arc<Representation>> {
                self.representations.read().await.clone()
            }
            pub async fn representation(&self, id: Id) -> Option<Arc<Representation>> {
                self.refresh_connector_child_weak_maps().await;
                self.representation_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            #[graphql(name = "bestRepresentation")]
            pub async fn best_representation(&self, tag_ids: Vec<Id>) -> Option<Arc<Representation>> {
                self.best_representation_for_tags(&tag_ids).await
            }
            pub async fn authors(&self) -> Vec<Author> {
                self.authors.read().await.clone()
            }
            pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
                crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone())
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone())
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
            }
            pub async fn props(&self) -> Vec<Prop> {
                self.props.read().await.clone()
            }
            pub async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
            }
            pub async fn stats(&self) -> Vec<Stat> {
                self.stats.read().await.clone()
            }
            #[graphql(name = "ownerEntity")]
            pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                None
            }
            #[graphql(name = "ownedEntities")]
            pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                Some(crate::iface::empty_owned_entity_connection())
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
                #[graphql(name = "replaceableBlueprint")]
                pub async fn replaceable_blueprint(&self) -> Vec<super::super::r#type::Blueprint> {
                    Vec::new()
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

                #[graphql(name = "ownerEntity")]
                pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                    None
                }

                #[graphql(name = "ownedEntities")]
                pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                    Some(crate::iface::empty_owned_entity_connection())
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
                pub piece: RwLock<Weak<super::piece::Piece>>,
                pub port: RwLock<Weak<super::super::r#type::Port>>,
                pub design_piece: RwLock<Weak<super::piece::Piece>>,
                pub connector: RwLock<Weak<super::super::r#type::Connector>>,
            }

            impl Default for Side {
                fn default() -> Self {
                    Self { id: Id::default(), piece: RwLock::new(Weak::new()), port: RwLock::new(Weak::new()), design_piece: RwLock::new(Weak::new()), connector: RwLock::new(Weak::new()) }
                }
            }

            impl Side {
                pub async fn new(piece: Weak<super::piece::Piece>) -> Arc<Self> {
                    Arc::new(Self { id: Id::new().await, piece: RwLock::new(piece), ..Default::default() })
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
                    h(&[self.id.as_str()])
                }
                pub async fn owner(&self) -> SideOwner {
                    SideOwner::Connection(Arc::new(Connection::default()))
                }
                #[graphql(name = "connectionOwner")]
                pub async fn connection_owner(&self) -> Option<Arc<Connection>> {
                    None
                }
                #[graphql(name = "ownerEntity")]
                pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                    None
                }
                #[graphql(name = "ownedEntities")]
                pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                    Some(crate::iface::empty_owned_entity_connection())
                }
                pub async fn piece(&self) -> Arc<super::piece::Piece> {
                    self.piece.read().await.upgrade().unwrap_or_default()
                }
                pub async fn port(&self) -> Option<Arc<super::super::r#type::Port>> {
                    self.port.read().await.upgrade()
                }
                #[graphql(name = "designPiece")]
                pub async fn design_piece(&self) -> Option<Arc<super::piece::Piece>> {
                    self.design_piece.read().await.upgrade()
                }
                pub async fn connector(&self) -> Option<Arc<super::super::r#type::Connector>> {
                    self.connector.read().await.upgrade()
                }
            }
            //#endregion ⛓️ side

            //#region 🔗 connection
            pub struct Connection {
                pub id: Id,
                pub owner_design: Weak<super::Design>,
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
                pub description: RwLock<Option<String>>,
                pub attributes: RwLock<Vec<Attribute>>,
            }

            impl Default for Connection {
                fn default() -> Self {
                    Self {
                        id: Id::default(),
                        owner_design: Weak::new(),
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
                        description: RwLock::new(None),
                        attributes: RwLock::new(Vec::new()),
                    }
                }
            }

            impl Connection {
                pub async fn compute_hash(&self) -> String {
                    let connected = self.connected.read().await;
                    let connecting = self.connecting.read().await;
                    let cp = connected.piece.read().await.upgrade().map(|p| p.id.0.clone()).unwrap_or_default();
                    let np = connecting.piece.read().await.upgrade().map(|p| p.id.0.clone()).unwrap_or_default();
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
                pub async fn description(&self) -> Option<String> {
                    self.description.read().await.clone()
                }
                pub async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
                }
                #[graphql(name = "ownerEntity")]
                pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                    None
                }
                #[graphql(name = "ownedEntities")]
                pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                    Some(crate::iface::empty_owned_entity_connection())
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

        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Group, Layer, Location, Prop, Quality, Stat};
        use crate::timestamp::Timestamp;

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
            pub location: RwLock<Option<Location>>,
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
                let plist = d_json.get("pieces").and_then(|p| p.as_array()).cloned().unwrap_or_default();
                let owner_des = Arc::downgrade(des);
                for pj in plist {
                    let pid = pj.get("id").and_then(|x| x.as_str()).ok_or_else(|| crate::error::SemioError::invalid("design piece missing id"))?;
                    let type_id_raw = pj.get("type").and_then(|t| t.get("id")).and_then(|x| x.as_str()).ok_or_else(|| crate::error::SemioError::invalid("design piece missing type.id"))?;
                    let ty = kit.types.read().await.iter().find(|t| t.id.as_str() == type_id_raw).cloned().ok_or_else(|| crate::error::SemioError::not_found("Type", type_id_raw))?;
                    let plane_val = pj.get("plane").cloned().unwrap_or_else(|| serde_json::json!({}));
                    let center_val = pj.get("center").cloned().unwrap_or_else(|| serde_json::json!({"u":0.0,"v":0.0}));
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
            pub async fn location(&self) -> Option<Location> {
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
                crate::gql_relay::PieceConnection::from_pieces(self.pieces.read().await.clone())
            }
            pub async fn piece(&self, id: Id) -> Option<Arc<piece::Piece>> {
                self.piece_by_external_id(&id).await
            }
            pub async fn connections(&self) -> crate::gql_relay::ConnectionConnection {
                crate::gql_relay::ConnectionConnection::from_connections(self.connections.read().await.clone())
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
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
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

            #[graphql(name = "ownerEntity")]
            pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
                crate::iface::owner_entity_arc_opt(self.owner_kit.upgrade().map(crate::iface::OwnerEntity::Kit))
            }

            #[graphql(name = "ownedEntities")]
            pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
                Some(crate::iface::empty_owned_entity_connection())
            }

            pub async fn references(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(Vec::new())
            }
            #[graphql(name = "referencedBy")]
            pub async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new())
            }
        }
        //#endregion 🏘 design
    }
    //#endregion 🏘 design

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
            let slot = self.resolve_tag_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs(input.attributes).await;
            let tag = Tag::new(slot, input.name, input.description, input.icon, input.order, attrs).await;
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
            let slot = self.resolve_concept_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs(input.attributes).await;
            let c = Concept::new(slot, input.name, input.description, input.icon, input.order, attrs).await;
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

        /// @emoji ➕ Create [`Quality`] under resolved owner (kit/type/representation/connector/design).
        pub async fn create_and_register_quality(self: &Arc<Self>, owner_id: &Id, input: crate::meta::QualityInput) -> Result<Arc<Quality>, crate::error::SemioError> {
            let slot = self.resolve_quality_owner_slot(owner_id).await?;
            let attrs = crate::meta::attributes_from_inputs(input.attributes).await;
            let q = Quality::new(slot, input.key, input.value, input.unit, input.definition, input.description, input.icon, Vec::new(), attrs).await;
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
                let types_arr = dto.get("types").and_then(|v| v.as_array()).cloned().unwrap_or_default();
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
            let design_arr_owned = dto.get("designs").and_then(|v| v.as_array()).cloned().unwrap_or_default();
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
        /// Owner [`crate::vcs::Graph`] is set by [`crate::worker::ChildRuntime::new`] via Weak.
        pub async fn owner(&self) -> KitOwner {
            match self.owner_graph.upgrade() {
                Some(g) => KitOwner::Graph(g),
                None => KitOwner::Graph(Arc::new(crate::vcs::Graph::default())),
            }
        }
        #[graphql(name = "graphOwner")]
        pub async fn graph_owner(&self) -> Option<Arc<crate::vcs::Graph>> {
            self.owner_graph.upgrade()
        }
        #[graphql(name = "checkpointOwner")]
        pub async fn checkpoint_owner(&self) -> Option<Arc<crate::vcs::Checkpoint>> {
            None
        }
        #[graphql(name = "alternativeOwner")]
        pub async fn alternative_owner(&self) -> Option<Arc<crate::vcs::Alternative>> {
            None
        }
        pub async fn checkpoint(&self) -> Option<Arc<crate::vcs::Checkpoint>> {
            None
        }
        pub async fn draft(&self) -> Option<Arc<crate::vcs::Draft>> {
            None
        }
        pub async fn transaction(&self) -> Option<Arc<crate::vcs::Transaction>> {
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
            crate::gql_relay::DesignConnection::from_designs(self.designs.read().await.clone())
        }
        #[graphql(name = "type")]
        pub async fn type_(&self, id: Id) -> Option<Arc<r#type::Type>> {
            self.type_by_external_id(&id).await
        }
        pub async fn types(&self) -> crate::gql_relay::TypeConnection {
            crate::gql_relay::TypeConnection::from_types(self.types.read().await.clone())
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
            crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone())
        }
        pub async fn tags(&self) -> crate::gql_relay::TagConnection {
            crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone())
        }
        pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
            crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
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

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            crate::iface::owner_entity_arc_opt(self.owner_graph.upgrade().map(crate::iface::OwnerEntity::Graph))
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }

        /// @emoji 📸 JSON string projection of `@semio/js` KitFullDto (camelCase ids + ordered pieces); WASM + Node pull this via GraphQL (`wip.theKit`).
        #[graphql(name = "fullSnapshot")]
        pub async fn full_snapshot(&self) -> String {
            serde_json::to_string(&self.kit_full_snapshot_value().await).unwrap_or_else(|_| "{}".to_string())
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
    #[graphql(name = "kitOwner")]
    pub async fn kit_owner(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
        match &*self.owner.read().await {
            crate::meta::TagOwnerSlot::Kit(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "typeOwner")]
    pub async fn type_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
        match &*self.owner.read().await {
            crate::meta::TagOwnerSlot::Type(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "representationOwner")]
    pub async fn representation_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Representation>> {
        match &*self.owner.read().await {
            crate::meta::TagOwnerSlot::Rep(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "ownerEntity")]
    pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
        let raw = match &*self.owner.read().await {
            crate::meta::TagOwnerSlot::Kit(w) => w.upgrade().map(crate::iface::OwnerEntity::Kit),
            crate::meta::TagOwnerSlot::Type(w) => w.upgrade().map(crate::iface::OwnerEntity::Type),
            crate::meta::TagOwnerSlot::Rep(w) => w.upgrade().map(crate::iface::OwnerEntity::Representation),
            crate::meta::TagOwnerSlot::Unset => None,
        };
        raw.map(crate::iface::owner_entity_arc)
    }
    #[graphql(name = "ownedEntities")]
    pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
        Some(crate::iface::empty_owned_entity_connection())
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
    #[graphql(name = "kitOwner")]
    pub async fn kit_owner(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
        match &*self.owner.read().await {
            crate::meta::ConceptOwnerSlot::Kit(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "typeOwner")]
    pub async fn type_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
        match &*self.owner.read().await {
            crate::meta::ConceptOwnerSlot::Type(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "ownerEntity")]
    pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
        let raw = match &*self.owner.read().await {
            crate::meta::ConceptOwnerSlot::Kit(w) => w.upgrade().map(crate::iface::OwnerEntity::Kit),
            crate::meta::ConceptOwnerSlot::Type(w) => w.upgrade().map(crate::iface::OwnerEntity::Type),
            crate::meta::ConceptOwnerSlot::Unset => None,
        };
        raw.map(crate::iface::owner_entity_arc)
    }
    #[graphql(name = "ownedEntities")]
    pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
        Some(crate::iface::empty_owned_entity_connection())
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
    #[graphql(name = "connectorOwner")]
    pub async fn connector_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Connector>> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Conn(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "representationOwner")]
    pub async fn representation_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Representation>> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Rep(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "typeOwner")]
    pub async fn type_owner(&self) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Type(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "designOwner")]
    pub async fn design_owner(&self) -> Option<std::sync::Arc<crate::kit::design::Design>> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Design(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "kitOwner")]
    pub async fn kit_owner(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
        match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Kit(w) => w.upgrade(),
            _ => None,
        }
    }
    #[graphql(name = "ownerEntity")]
    pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
        let raw = match &*self.owner.read().await {
            crate::meta::QualityOwnerSlot::Kit(w) => w.upgrade().map(crate::iface::OwnerEntity::Kit),
            crate::meta::QualityOwnerSlot::Design(w) => w.upgrade().map(crate::iface::OwnerEntity::Design),
            crate::meta::QualityOwnerSlot::Type(w) => w.upgrade().map(crate::iface::OwnerEntity::Type),
            crate::meta::QualityOwnerSlot::Rep(w) => w.upgrade().map(crate::iface::OwnerEntity::Representation),
            crate::meta::QualityOwnerSlot::Conn(w) => w.upgrade().map(crate::iface::OwnerEntity::Connector),
            crate::meta::QualityOwnerSlot::Unset => None,
        };
        raw.map(crate::iface::owner_entity_arc)
    }
    #[graphql(name = "ownedEntities")]
    pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
        Some(crate::iface::empty_owned_entity_connection())
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
    //! 🌿 Version-control entities — change/transaction/draft/checkpoint/alternative/graph/session/conflict.
    use std::sync::{Arc, Weak};

    use async_graphql::{Object, Union};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::hash::h;
    use crate::id::Id;
    use crate::kit::Kit;
    use crate::meta::Author;
    use crate::op;
    use crate::timestamp::Timestamp;

    //#region 🪪 change
    pub struct Change {
        pub id: Id,
        pub owner: RwLock<Option<ChangeOwnerRef>>,
        pub forwards: RwLock<Vec<op::OperationKind>>,
        pub backwards: RwLock<Vec<op::OperationKind>>,
    }

    /// 🔗 Untyped reference to one of the variants of the [`ChangeOwnerUnion`].
    #[derive(Clone)]
    pub enum ChangeOwnerRef {
        Transaction(Weak<Transaction>),
        Draft(Weak<Draft>),
        Checkpoint(Weak<Checkpoint>),
    }

    impl Default for Change {
        fn default() -> Self {
            Self { id: Id::default(), owner: RwLock::new(None), forwards: RwLock::new(Vec::new()), backwards: RwLock::new(Vec::new()) }
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
                Some(ChangeOwnerRef::Transaction(w)) => ChangeOwnerUnion::Transaction(w.upgrade().unwrap_or_default()),
                Some(ChangeOwnerRef::Draft(w)) => ChangeOwnerUnion::Draft(w.upgrade().unwrap_or_default()),
                Some(ChangeOwnerRef::Checkpoint(w)) => ChangeOwnerUnion::Checkpoint(w.upgrade().unwrap_or_default()),
                None => ChangeOwnerUnion::Transaction(Arc::default()),
            }
        }
        pub async fn forwards(&self) -> Vec<op::OperationKind> {
            self.forwards.read().await.clone()
        }
        pub async fn backwards(&self) -> Vec<op::OperationKind> {
            self.backwards.read().await.clone()
        }

        /// @emoji 🔗 Ordered semantic op record ids constituting the forwards side (bundle `semanticOpLog` ids) when persisted.
        #[graphql(name = "forwardSemanticOpRecordIds")]
        pub async fn forward_semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        /// @emoji 🔗 Ordered semantic op record ids for backwards / inverse application when persisted separately from `OperationKind`.
        #[graphql(name = "backwardSemanticOpRecordIds")]
        pub async fn backward_semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }

    #[derive(Clone, Union)]
    #[graphql(name = "ChangeOwner")]
    pub enum ChangeOwnerUnion {
        Transaction(Arc<Transaction>),
        Draft(Arc<Draft>),
        Checkpoint(Arc<Checkpoint>),
    }
    //#endregion 🪪 change

    //#region 💼 transaction
    pub struct Transaction {
        pub id: Id,
        pub owner_draft: Weak<Draft>,
        pub changes: RwLock<Vec<Arc<Change>>>,
    }

    impl Default for Transaction {
        fn default() -> Self {
            Self { id: Id::default(), owner_draft: Weak::new(), changes: RwLock::new(Vec::new()) }
        }
    }

    impl Transaction {
        pub async fn new(owner_draft: Weak<Draft>) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_draft, changes: RwLock::new(Vec::new()) })
        }
        pub async fn with_id(owner_draft: Weak<Draft>, id: Id) -> Arc<Self> {
            Arc::new(Self { id, owner_draft, changes: RwLock::new(Vec::new()) })
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn record(&self, change: Arc<Change>) {
            self.changes.write().await.push(change);
        }
    }

    #[Object(name = "Transaction")]
    impl Transaction {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<Arc<Draft>> {
            self.owner_draft.upgrade()
        }
        pub async fn changes(&self) -> Vec<Arc<Change>> {
            self.changes.read().await.clone()
        }

        /// @emoji 🔗 Transaction forwards op sequence as `semanticOpLog` ids (see `histories.transaction`).
        #[graphql(name = "semanticOpRecordIds")]
        pub async fn semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
    //#endregion 💼 transaction

    //#region 📝 draft
    pub struct Draft {
        pub id: Id,
        pub owner_alternative: Weak<Alternative>,
        pub parent_checkpoint: RwLock<Weak<Checkpoint>>,
        pub target_alternative: RwLock<Weak<Alternative>>,
        pub open_transaction: RwLock<Weak<Transaction>>,
        pub finalized_transactions: RwLock<Vec<Arc<Transaction>>>,
        pub redo_transactions: RwLock<Vec<Arc<Transaction>>>,
        pub transactions: RwLock<Vec<Arc<Transaction>>>,
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
        pub async fn ensure_transaction(self: &Arc<Self>, id: &Id) -> Arc<Transaction> {
            if let Some(t) = self.transactions.read().await.iter().find(|t| &t.id == id).cloned() {
                *self.open_transaction.write().await = Arc::downgrade(&t);
                return t;
            }
            let t = Transaction::with_id(Arc::downgrade(self), id.clone()).await;
            self.transactions.write().await.push(t.clone());
            *self.open_transaction.write().await = Arc::downgrade(&t);
            t
        }
    }

    #[Object(name = "Draft")]
    impl Draft {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<Arc<Alternative>> {
            self.owner_alternative.upgrade()
        }
        #[graphql(name = "parentCheckpoint")]
        pub async fn parent_checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.parent_checkpoint.read().await.upgrade()
        }
        #[graphql(name = "targetAlternative")]
        pub async fn target_alternative(&self) -> Option<Arc<Alternative>> {
            self.target_alternative.read().await.upgrade()
        }
        #[graphql(name = "openTransaction")]
        pub async fn open_transaction(&self) -> Option<Arc<Transaction>> {
            self.open_transaction.read().await.upgrade()
        }
        #[graphql(name = "finalizedTransactions")]
        pub async fn finalized_transactions(&self) -> Vec<Arc<Transaction>> {
            self.finalized_transactions.read().await.clone()
        }
        #[graphql(name = "redoTransactions")]
        pub async fn redo_transactions(&self) -> Vec<Arc<Transaction>> {
            self.redo_transactions.read().await.clone()
        }
        pub async fn changes(&self) -> Vec<Arc<Change>> {
            let mut out = Vec::new();
            for t in self.transactions.read().await.iter() {
                for c in t.changes.read().await.iter() {
                    out.push(c.clone());
                }
            }
            out
        }
        #[graphql(name = "canUndo")]
        pub async fn can_undo(&self, _steps: i32) -> bool {
            !self.finalized_transactions.read().await.is_empty()
        }
        #[graphql(name = "canRedo")]
        pub async fn can_redo(&self, _steps: i32) -> bool {
            !self.redo_transactions.read().await.is_empty()
        }

        /// @emoji 🔗 Stable transaction open order on this draft (bundle `histories.draft`).
        #[graphql(name = "orderedTransactionIds")]
        pub async fn ordered_transaction_ids(&self) -> Vec<Id> {
            self.transactions.read().await.iter().map(|t| t.id.clone()).collect()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }
        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
    //#endregion 📝 draft

    //#region 🪧 checkpoint
    pub struct Checkpoint {
        pub id: Id,
        pub timestamp: RwLock<Option<Timestamp>>,
        pub authors: RwLock<Vec<Author>>,
        pub root: RwLock<Option<Arc<Kit>>>,
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
                root: RwLock::new(None),
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
        pub async fn authors(&self) -> Vec<Author> {
            self.authors.read().await.clone()
        }
        pub async fn root(&self) -> Option<Arc<Kit>> {
            self.root.read().await.clone()
        }
        #[graphql(name = "parentCheckpoint")]
        pub async fn parent_checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.parent_checkpoint.read().await.upgrade()
        }
        pub async fn message(&self) -> Option<String> {
            self.message.read().await.clone()
        }
        #[graphql(name = "isRelease")]
        pub async fn is_release(&self) -> bool {
            *self.is_release.read().await
        }
        #[graphql(name = "changeCount")]
        pub async fn change_count(&self) -> i32 {
            *self.change_count.read().await
        }

        /// @emoji 🔗 Checkpoint-scoped semantic op ids (`histories.checkpoint` wraps op log ids without duplicating kit trees).
        #[graphql(name = "semanticOpRecordIds")]
        pub async fn semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
    //#endregion 🪧 checkpoint

    //#region 🌱 alternative
    pub struct Alternative {
        pub id: Id,
        pub owner_graph: Weak<Graph>,
        pub name: RwLock<String>,
        pub start: RwLock<Weak<Checkpoint>>,
        pub checkpoints: RwLock<Vec<Arc<Checkpoint>>>,
        pub kit: RwLock<Option<Arc<Kit>>>,
        pub draft: RwLock<Weak<Draft>>,
        pub transaction: RwLock<Weak<Transaction>>,
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
        pub async fn draft(&self) -> Option<Arc<Draft>> {
            self.draft.read().await.upgrade()
        }
        pub async fn transaction(&self) -> Option<Arc<Transaction>> {
            self.transaction.read().await.upgrade()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            crate::iface::owner_entity_arc_opt(self.owner_graph.upgrade().map(crate::iface::OwnerEntity::Graph))
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
    //#endregion 🌱 alternative

    //#region 🌐 graph
    /// @emoji 🔗 `Graph.owner` — target SDL `union GraphOwner = Session`.
    #[derive(Clone, Union)]
    pub enum GraphOwner {
        Session(Arc<Session>),
    }

    pub struct Graph {
        pub id: Id,
        pub owner_session: RwLock<Weak<Session>>,
        pub the_kit: Arc<Kit>,
        pub alternatives: RwLock<Vec<Arc<Alternative>>>,
        pub checkpoints: RwLock<Vec<Arc<Checkpoint>>>,
        pub releases: RwLock<Vec<Arc<Checkpoint>>>,
        pub drafts: RwLock<Vec<Arc<Draft>>>,
        /// @emoji 📜 Ordered operation spine for this graph head (static schema / replay).
        pub op_history: RwLock<Vec<Arc<crate::op::OperationIface>>>,
    }

    impl Default for Graph {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_session: RwLock::new(Weak::new()),
                the_kit: Arc::default(),
                alternatives: RwLock::new(Vec::new()),
                checkpoints: RwLock::new(Vec::new()),
                releases: RwLock::new(Vec::new()),
                drafts: RwLock::new(Vec::new()),
                op_history: RwLock::new(Vec::new()),
            }
        }
    }

    impl Graph {
        /// 🆕 Build a brand-new Graph wired with its `the_kit` Arc; `Kit::owner_graph` Weak is set
        /// in a second pass via [`Arc::new_cyclic`]-style assembly because both directions need an Arc.
        pub async fn new() -> Arc<Self> {
            let id = Id::new().await;
            Arc::new_cyclic(|weak_self: &Weak<Graph>| {
                let kit = crate::kit::Kit::new_sync(weak_self.clone(), "the kit".to_string());
                Self {
                    id,
                    owner_session: RwLock::new(Weak::new()),
                    the_kit: kit,
                    alternatives: RwLock::new(Vec::new()),
                    checkpoints: RwLock::new(Vec::new()),
                    releases: RwLock::new(Vec::new()),
                    drafts: RwLock::new(Vec::new()),
                    op_history: RwLock::new(Vec::new()),
                }
            })
        }

        /// 🛰️ WIP bootstrap for `@semio/js` WASM: builds an empty typed shell then overlays [`crate::kit::Kit::hydrate_from_kit_full_snapshot_json`].
        pub async fn new_overlay_from_kit_json(dto_json: serde_json::Value) -> Result<Arc<Self>, SemioError> {
            let g = Self::new().await;
            g.the_kit.hydrate_from_kit_full_snapshot_json(&dto_json).await?;
            if let Some(c) = dto_json.get("createdAt").and_then(|v| v.as_str()) {
                *g.the_kit.created.write().await = Some(crate::timestamp::Timestamp(c.to_string()));
            }
            if let Some(u) = dto_json.get("updatedAt").and_then(|v| v.as_str()) {
                *g.the_kit.updated.write().await = Some(crate::timestamp::Timestamp(u.to_string()));
            }
            Ok(g)
        }

        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }

        /// @emoji 🌱 Ensure the graph has a seed `Checkpoint` anchored on `the_kit` and a default `Draft`
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
                        root: RwLock::new(Some(self.the_kit.clone())),
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
                None => self
                    .drafts
                    .read()
                    .await
                    .iter()
                    .find(|d| d.owner_alternative.upgrade().is_none())
                    .cloned()
                    .ok_or_else(|| SemioError::invalid("no main-line draft"))?,
                Some(aid) => {
                    let alts = self.alternatives.read().await;
                    let alt = alts
                        .iter()
                        .find(|a| &a.id == aid)
                        .ok_or_else(|| SemioError::not_found("Alternative", aid.as_str()))?
                        .clone();
                    drop(alts);
                    alt.draft.read().await.upgrade().ok_or_else(|| SemioError::invalid("alternative has no draft"))?
                }
            };

            let parent_cp = source_draft
                .parent_checkpoint
                .read()
                .await
                .upgrade()
                .ok_or_else(|| SemioError::invalid("draft has no parent checkpoint"))?;

            let new_alt_id = Id::new().await;
            let new_alt = Arc::new(Alternative {
                id: new_alt_id.clone(),
                owner_graph: Arc::downgrade(self),
                name: RwLock::new(name),
                start: RwLock::new(Arc::downgrade(&parent_cp)),
                checkpoints: RwLock::new(vec![parent_cp.clone()]),
                kit: RwLock::new(parent_cp.root.read().await.clone()),
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
        pub async fn open_transaction(self: &Arc<Self>, draft_id: &Id) -> Arc<Transaction> {
            let draft = self.ensure_draft(draft_id).await;
            let tx_id = Id::new().await;
            let tx = Transaction::with_id(Arc::downgrade(&draft), tx_id).await;
            draft.transactions.write().await.push(tx.clone());
            *draft.open_transaction.write().await = Arc::downgrade(&tx);
            tx
        }

        /// @emoji ✅ Mark a transaction as finalized: moved from `transactions` into `finalized_transactions`; clears the draft's open pointer if it matched.
        pub async fn commit_transaction(self: &Arc<Self>, draft_id: &Id, transaction_id: &Id) -> Result<(), SemioError> {
            let draft = self
                .drafts
                .read()
                .await
                .iter()
                .find(|d| &d.id == draft_id)
                .cloned()
                .ok_or_else(|| SemioError::not_found("Draft", draft_id.as_str()))?;
            let tx = {
                let mut txs = draft.transactions.write().await;
                let pos = txs.iter().position(|t| &t.id == transaction_id).ok_or_else(|| SemioError::not_found("Transaction", transaction_id.as_str()))?;
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
            let draft = self
                .drafts
                .read()
                .await
                .iter()
                .find(|d| &d.id == draft_id)
                .cloned()
                .ok_or_else(|| SemioError::not_found("Draft", draft_id.as_str()))?;
            {
                let mut txs = draft.transactions.write().await;
                let pos = txs.iter().position(|t| &t.id == transaction_id).ok_or_else(|| SemioError::not_found("Transaction", transaction_id.as_str()))?;
                txs.remove(pos);
            }
            let open = draft.open_transaction.read().await.upgrade();
            if let Some(open_tx) = open {
                if &open_tx.id == transaction_id {
                    *draft.open_transaction.write().await = std::sync::Weak::new();
                }
            }
            Ok(())
        }

        /// 🪡 Hot path: mutate using an already-bound [`crate::kit_graph_engine::DesignHandle`] / design node (no further design-id scans).
        pub(crate) async fn apply_create_fixed_piece_on_design_node(
            self: &Arc<Self>,
            draft_id: Id,
            transaction_id: Id,
            design: Arc<crate::kit::design::Design>,
            blueprint_id: Id,
            position: crate::geom::Position,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<Arc<crate::kit::design::piece::Piece>, SemioError> {
            let blueprint_type = crate::kit::r#type::Type::new(Arc::downgrade(&self.the_kit), format!("type-{}", blueprint_id.as_str())).await;
            let blueprint = crate::kit::r#type::Blueprint::Type(blueprint_type);

            let piece = crate::kit::design::piece::Piece::new_fixed(Arc::downgrade(&design), blueprint, position).await;
            piece.set_name(name).await;
            piece.set_description(description).await;
            let _ = design.insert_piece(piece.clone()).await;

            let draft = self.ensure_draft(&draft_id).await;
            let _ = draft.ensure_transaction(&transaction_id).await;

            Ok(piece)
        }

        /// 🪡 Graph-mutating entry for `createFixedPiece`: one external id bind, then pointer-only core; returns deterministic ephemeral diff (not persisted).
        pub async fn apply_create_fixed_piece(
            self: &Arc<Self>,
            draft_id: Id,
            transaction_id: Id,
            design_id: Id,
            blueprint_id: Id,
            position: crate::geom::Position,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<(Arc<crate::kit::design::piece::Piece>, op::SemanticDiff), SemioError> {
            let fp_before = crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await;
            let (_handle, design) = self.the_kit.bind_external_design_id(&design_id).await;
            let piece = self.apply_create_fixed_piece_on_design_node(draft_id, transaction_id, design, blueprint_id.clone(), position, name.clone(), description.clone()).await?;
            let fp_after = crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await;
            let input = op::CreatedFixedPieceInput { design_id, blueprint_id, position, name, description };
            let payload_json = serde_json::to_string(&input).map_err(|e| SemioError::invalid(e.to_string()))?;
            let diff = crate::kit_graph_engine::deterministic_semantic_diff("createdFixedPiece", &payload_json, &fp_before, &fp_after);
            Ok((piece, diff))
        }

        /// @emoji ↔ Apply UV offset to a piece center (creates a default [`PositionNode`] when absent).
        pub async fn apply_drag_piece_in_design(self: &Arc<Self>, design_id: &Id, piece_id: &Id, offset: crate::geom::Offset) -> Result<(), SemioError> {
            use crate::geom::entity::PositionNode;
            let design = self.the_kit.design_by_external_id(design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
            let piece = design.piece_by_external_id(piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
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
            Ok(())
        }

        /// @emoji ↔ Batch drag: same offset for every piece id.
        pub async fn apply_drag_pieces_in_design(self: &Arc<Self>, design_id: &Id, piece_ids: &[Id], offset: crate::geom::Offset) -> Result<(), SemioError> {
            for pid in piece_ids {
                self.apply_drag_piece_in_design(design_id, pid, offset).await?;
            }
            Ok(())
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
        #[graphql(name = "sessionOwner")]
        pub async fn session_owner(&self) -> Option<Arc<Session>> {
            self.owner_session.read().await.upgrade()
        }
        #[graphql(name = "theKit")]
        pub async fn the_kit(&self) -> Option<Arc<Kit>> {
            Some(self.the_kit.clone())
        }
        pub async fn alternative(&self, id: Id) -> Option<Arc<Alternative>> {
            self.alternatives.read().await.iter().find(|a| a.id == id).cloned()
        }
        pub async fn alternatives(&self) -> crate::gql_relay::AlternativeConnection {
            crate::gql_relay::AlternativeConnection::from_alternatives(self.alternatives.read().await.clone())
        }
        pub async fn checkpoint(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.checkpoints.read().await.iter().find(|c| c.id == id).cloned()
        }
        pub async fn checkpoints(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.checkpoints.read().await.clone())
        }
        pub async fn release(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.releases.read().await.iter().find(|c| c.id == id).cloned()
        }
        pub async fn releases(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.releases.read().await.clone())
        }

        /// @emoji 📝 Lookup a draft on this graph by id (sketchpad uses this after `transactionOpen` / before `transactionCommit`).
        pub async fn draft(&self, id: Id) -> Option<Arc<Draft>> {
            self.drafts.read().await.iter().find(|d| d.id == id).cloned()
        }

        /// @emoji 📝 All drafts currently open on this graph (sketchpad probes `wip.drafts` to know which ones to commit / abort on close).
        pub async fn drafts(&self) -> Vec<Arc<Draft>> {
            self.drafts.read().await.clone()
        }

        /// @emoji 📜 Ordered semantic op log for this graph line (persisted bundle field); empty until store wiring lands.
        /// **Memoization:** none — each field resolution recomputes from current graph/backbone state once wired. **Invalidate:** backbone attach/replay, bundle tips, or any writer appending to the log (no stale cached slice).
        #[graphql(name = "semanticOpLog")]
        pub async fn semantic_op_log(&self, #[graphql(name = "limit")] limit: Option<i32>) -> Vec<op::SemanticOpRecord> {
            let _ = limit;
            Vec::new()
        }

        /// @emoji 🔢 Stable `projectionFingerprint` (sorted piece centers via [`crate::kit_graph_engine::projection_fingerprint_for_kit`]).
        /// **Memoization:** none — derived on every read from live piece centers. **Invalidate:** any semantic op or mutation changing piece geometry on this graph line.
        #[graphql(name = "projectionFingerprint")]
        pub async fn projection_fingerprint(&self) -> String {
            crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await
        }

        /// @emoji 📸 Hash of the materialized root kit aggregate for this graph head.
        /// **Memoization:** none — recomputed per request from the current [`Kit`] graph. **Invalidate:** any structural or identity change the kit hash subscribes to (mutations, replay).
        #[graphql(name = "rootSnapshotHash")]
        pub async fn root_snapshot_hash(&self) -> String {
            self.the_kit.compute_hash().await
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            crate::iface::owner_entity_arc_opt(self.owner_session.read().await.upgrade().map(crate::iface::OwnerEntity::Session))
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
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
        pub async fn drafts(&self) -> Vec<Arc<Draft>> {
            self.drafts.read().await.clone()
        }

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
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

    #[Object(name = "Conflict")]
    impl Conflict {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str(), self.reason.read().await.as_str()])
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

        #[graphql(name = "ownerEntity")]
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            None
        }

        #[graphql(name = "ownedEntities")]
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
    }
    //#endregion ⚠️ conflict

    //#region 📖 read-write version
    /// @emoji 📖 SDL `ReadVersionOwner = Conflict`.
    #[derive(Clone, Union)]
    #[graphql(name = "ReadVersionOwner")]
    pub enum ReadVersionOwner {
        Conflict(Arc<Conflict>),
    }

    /// @emoji 📖 Read-side version marker on a [`Conflict`].
    pub struct ReadVersion {
        pub id: Id,
        pub owner_conflict: Weak<Conflict>,
        pub checkpoint: RwLock<Option<Arc<Checkpoint>>>,
        pub change: RwLock<Option<Arc<Change>>>,
        pub operation: RwLock<Option<Arc<crate::op::OperationIface>>>,
    }

    impl Default for ReadVersion {
        fn default() -> Self {
            Self { id: Id::default(), owner_conflict: Weak::new(), checkpoint: RwLock::new(None), change: RwLock::new(None), operation: RwLock::new(None) }
        }
    }

    #[Object(name = "ReadVersion")]
    impl ReadVersion {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> ReadVersionOwner {
            ReadVersionOwner::Conflict(self.owner_conflict.upgrade().unwrap_or_default())
        }
        #[graphql(name = "conflictOwner")]
        pub async fn conflict_owner(&self) -> Option<Arc<Conflict>> {
            self.owner_conflict.upgrade()
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            crate::iface::owner_entity_arc_opt(self.owner_conflict.upgrade().map(crate::iface::OwnerEntity::Conflict))
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.checkpoint.read().await.clone()
        }
        pub async fn change(&self) -> Option<Arc<Change>> {
            self.change.read().await.clone()
        }
        pub async fn operation(&self) -> Option<Arc<crate::op::OperationIface>> {
            self.operation.read().await.clone()
        }
    }

    #[derive(Clone, Union)]
    #[graphql(name = "WriteVersionOwner")]
    pub enum WriteVersionOwner {
        Conflict(Arc<Conflict>),
    }

    /// @emoji ✏️ Write-side version marker on a [`Conflict`].
    pub struct WriteVersion {
        pub id: Id,
        pub owner_conflict: Weak<Conflict>,
        pub draft: RwLock<Option<Arc<Draft>>>,
        pub transaction: RwLock<Option<Arc<Transaction>>>,
        pub checkpoint: RwLock<Option<Arc<Checkpoint>>>,
        pub change: RwLock<Option<Arc<Change>>>,
        pub operation: RwLock<Option<Arc<crate::op::OperationIface>>>,
    }

    impl Default for WriteVersion {
        fn default() -> Self {
            Self { id: Id::default(), owner_conflict: Weak::new(), draft: RwLock::new(None), transaction: RwLock::new(None), checkpoint: RwLock::new(None), change: RwLock::new(None), operation: RwLock::new(None) }
        }
    }

    #[Object(name = "WriteVersion")]
    impl WriteVersion {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn owner(&self) -> WriteVersionOwner {
            WriteVersionOwner::Conflict(self.owner_conflict.upgrade().unwrap_or_default())
        }
        #[graphql(name = "conflictOwner")]
        pub async fn conflict_owner(&self) -> Option<Arc<Conflict>> {
            self.owner_conflict.upgrade()
        }
        pub async fn owner_entity(&self) -> Option<std::sync::Arc<crate::iface::OwnerEntity>> {
            crate::iface::owner_entity_arc_opt(self.owner_conflict.upgrade().map(crate::iface::OwnerEntity::Conflict))
        }
        pub async fn owned_entities(&self) -> Option<std::sync::Arc<crate::iface::OwnedEntityConnection>> {
            Some(crate::iface::empty_owned_entity_connection())
        }
        pub async fn draft(&self) -> Option<Arc<Draft>> {
            self.draft.read().await.clone()
        }
        pub async fn transaction(&self) -> Option<Arc<Transaction>> {
            self.transaction.read().await.clone()
        }
        pub async fn checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.checkpoint.read().await.clone()
        }
        pub async fn change(&self) -> Option<Arc<Change>> {
            self.change.read().await.clone()
        }
        pub async fn operation(&self) -> Option<Arc<crate::op::OperationIface>> {
            self.operation.read().await.clone()
        }
    }
    //#endregion 📖 read-write version
}

//#endregion 🌿 vcs

//#region 🧷 iface

/// 🧷 Cross-cutting GraphQL `OwnerEntity` / `OwnedEntity` unions and empty Relay shells (expanded as more entities register).
pub mod iface {
    use std::sync::Arc;

    use async_graphql::{Object, SimpleObject, Union};

    use crate::geom::entity::{CoordinateNode, OffsetNode, PlaceNode, PlaneNode, PointNode, PositionNode, VectorNode};
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
            Self { edges: Vec::new(), page_info: crate::gql_relay::PageInfo::default(), hash: crate::hash::h(&[""]) }
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
            let kid = g.the_kit.workspace_kit_id().await;
            if id == &kid || id == &g.the_kit.id {
                return Some(GqlNode::Kit(g.the_kit.clone()));
            }
            if let Some(t) = g.the_kit.find_tag(id).await {
                return Some(GqlNode::Tag(t));
            }
            if let Some(c) = g.the_kit.find_concept(id).await {
                return Some(GqlNode::Concept(c));
            }
            if let Some(q) = g.the_kit.find_quality(id).await {
                return Some(GqlNode::Quality(q));
            }
            let designs = g.the_kit.designs.read().await;
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
        let des = g.the_kit.design_by_external_id(design_id).await?;
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
            let u = *self.u.read().await;
            let v = *self.v.read().await;
            crate::hash::h(&[&format!("{u:.9}"), &format!("{v:.9}")])
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
            let x = *self.x.read().await;
            let y = *self.y.read().await;
            let z = *self.z.read().await;
            crate::hash::h(&[&format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")])
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
            let x = *self.x.read().await;
            let y = *self.y.read().await;
            let z = *self.z.read().await;
            crate::hash::h(&[&format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")])
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
            crate::hash::h(&[self.origin.id.as_str(), self.x_axis.id.as_str(), self.y_axis.id.as_str()])
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
            crate::hash::h(&[self.center.id.as_str(), self.plane.id.as_str()])
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
            let u = *self.u.read().await;
            let v = *self.v.read().await;
            crate::hash::h(&[&format!("{u:.9}"), &format!("{v:.9}")])
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

    #[Object(name = "Place")]
    impl PlaceNode {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
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

//#region ⚙️ op

pub mod op {
    //! ⚙️ Operation entities and their inputs. Operations carry `Arc<Entity>` payloads so the
    //! event bus broadcasts shared references, never deep-copied entity data.
    use std::sync::{Arc, Weak};

    use async_graphql::{InputObject, Interface, Object, OneofObject, Union};
    use serde::{Deserialize, Serialize};

    use crate::geom::{Offset, Position};
    use crate::id::Id;
    use crate::iface::{empty_owned_entity_connection, OwnedEntityConnection, OwnerEntity};
    use crate::vcs::Change;

    /// 🏷️ Hand union for `Operation.owner` (every operation is owned by a `Change`).
    #[derive(Clone, Union)]
    pub enum OperationOwner {
        Change(Arc<Change>),
    }

    impl Default for OperationOwner {
        fn default() -> Self {
            Self::Change(Arc::default())
        }
    }

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

    //#region 📜 semantic op record (kit bundle / op log contract)
    /// @emoji 📜 One persisted semantic op: stable id, kind string, JSON payload, monotonic sequence index.
    #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
    #[graphql(name = "SemanticOpRecord")]
    pub struct SemanticOpRecord {
        pub id: Id,
        #[graphql(name = "opKind")]
        pub op_kind: String,
        #[graphql(name = "payloadJson")]
        pub payload_json: String,
        pub sequence: i32,
    }
    //#endregion 📜 semantic op record (kit bundle / op log contract)

    //#region 📦 diff
    /// 📜 Ephemeral semantic diff payload for the kit engine; replay/apply diff carrier (not a GraphQL output type).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct SemanticDiff {
        pub id: Id,
        pub summary: Option<String>,
    }
    //#endregion 📦 diff

    //#region 🪄 operations
    pub struct CreatedFixedPiece {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: CreatedFixedPieceInput,
        pub diff: SemanticDiff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl CreatedFixedPiece {
        pub async fn new(input: CreatedFixedPieceInput, piece: Arc<crate::kit::design::piece::Piece>, diff: SemanticDiff) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_change: Weak::new(), input, diff, piece })
        }
    }

    impl Default for CreatedFixedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: CreatedFixedPieceInput::default(), diff: SemanticDiff::default(), piece: Arc::default() }
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
            Arc::new(OperationOwner::Change(self.owner_change.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "changeOwner")]
        pub async fn change_owner(&self) -> Option<Arc<Change>> {
            self.owner_change.upgrade()
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
        pub owner_change: Weak<Change>,
        pub input: FixedPieceInput,
        pub diff: SemanticDiff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl Default for FixedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: FixedPieceInput::default(), diff: SemanticDiff::default(), piece: Arc::default() }
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
            Arc::new(OperationOwner::Change(self.owner_change.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "changeOwner")]
        pub async fn change_owner(&self) -> Option<Arc<Change>> {
            self.owner_change.upgrade()
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
        pub owner_change: Weak<Change>,
        pub input: DraggedPieceInput,
        pub diff: SemanticDiff,
        pub pieces: Vec<Arc<crate::kit::design::piece::Piece>>,
    }

    impl Default for DraggedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: DraggedPieceInput::default(), diff: SemanticDiff::default(), pieces: Vec::new() }
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
            Arc::new(OperationOwner::Change(self.owner_change.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "changeOwner")]
        pub async fn change_owner(&self) -> Option<Arc<Change>> {
            self.owner_change.upgrade()
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
        pub owner_change: Weak<Change>,
        pub input: RenamedKitInput,
        pub diff: SemanticDiff,
        pub kit: Arc<crate::kit::Kit>,
    }

    impl Default for RenamedKit {
        fn default() -> Self {
            Self { id: Id::default(), request_id: Id::default(), owner_change: Weak::new(), input: RenamedKitInput::default(), diff: SemanticDiff::default(), kit: Arc::default() }
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
            Arc::new(OperationOwner::Change(self.owner_change.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "changeOwner")]
        pub async fn change_owner(&self) -> Option<Arc<Change>> {
            self.owner_change.upgrade()
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
        pub owner_change: Weak<Change>,
        pub input: ChangedDescriptionInput,
        pub diff: SemanticDiff,
        pub entity: Arc<crate::kit::Kit>,
    }

    impl Default for ChangedDescription {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: ChangedDescriptionInput::default(), diff: SemanticDiff::default(), entity: Arc::default() }
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
            Arc::new(OperationOwner::Change(self.owner_change.upgrade().unwrap_or_default()))
        }
        #[graphql(name = "changeOwner")]
        pub async fn change_owner(&self) -> Option<Arc<Change>> {
            self.owner_change.upgrade()
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
        field(name = "owner", ty = "std::sync::Arc<crate::op::OperationOwner>"),
        field(name = "changeOwner", method = "change_owner", ty = "Option<std::sync::Arc<crate::vcs::Change>>"),
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
        AddFixedPieceToDesign { request_id: Id, draft_id: Id, transaction_id: Id, design_id: Id, blueprint_id: Id, position: Position, name: Option<String>, description: Option<String> },
        FixPieceInDesign { request_id: Id, draft_id: Id, transaction_id: Id, design_id: Id, piece_id: Id },
        RenameKit { request_id: Id, draft_id: Id, transaction_id: Id, name: String },
        ChangeDescription { request_id: Id, draft_id: Id, transaction_id: Id, entity_id: Id, description: String },
        CreateTag { request_id: Id, draft_id: Id, transaction_id: Id, owner_id: Id, input: crate::meta::TagInput },
        CreateTags { request_id: Id, draft_id: Id, transaction_id: Id, owner_id: Id, inputs: Vec<crate::meta::TagInput> },
        RenameTag { request_id: Id, draft_id: Id, transaction_id: Id, tag_id: Id, name: String },
        DeleteTag { request_id: Id, draft_id: Id, transaction_id: Id, tag_id: Id },
        DeleteTags { request_id: Id, draft_id: Id, transaction_id: Id, tag_ids: Vec<Id> },
        CreateConcept { request_id: Id, draft_id: Id, transaction_id: Id, owner_id: Id, input: crate::meta::ConceptInput },
        CreateQuality { request_id: Id, draft_id: Id, transaction_id: Id, owner_id: Id, input: crate::meta::QualityInput },
        DragPieceInDesign { request_id: Id, draft_id: Id, transaction_id: Id, design_id: Id, piece_id: Id, offset: Offset },
        DragPiecesInDesign { request_id: Id, draft_id: Id, transaction_id: Id, design_id: Id, piece_ids: Vec<Id>, offset: Offset },
        BackboneAttach { request_id: Id, connection_uri: String, store_kind: BackboneStoreKind },
        BackboneDetach { request_id: Id, connection_uri: String },
    }

    impl Command {
        pub fn request_id(&self) -> &Id {
            match self {
                Command::AddFixedPieceToDesign { request_id, .. } => request_id,
                Command::FixPieceInDesign { request_id, .. } => request_id,
                Command::RenameKit { request_id, .. } => request_id,
                Command::ChangeDescription { request_id, .. } => request_id,
                Command::CreateTag { request_id, .. } => request_id,
                Command::CreateTags { request_id, .. } => request_id,
                Command::RenameTag { request_id, .. } => request_id,
                Command::DeleteTag { request_id, .. } => request_id,
                Command::DeleteTags { request_id, .. } => request_id,
                Command::CreateConcept { request_id, .. } => request_id,
                Command::CreateQuality { request_id, .. } => request_id,
                Command::DragPieceInDesign { request_id, .. } => request_id,
                Command::DragPiecesInDesign { request_id, .. } => request_id,
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

    /// @emoji 🧩 Declarative op row registration hook (`ops! { CreatedFixedPiece, … }`) — expand to typed op structs + history wiring.
    macro_rules! ops {
        ($($row:ident),* $(,)?) => {
            /// @emoji 🔢 Row count listed in `ops! { … }` (static registry grows toward ~100 SDL operations).
            pub const GRAPH_OP_REGISTRY_ROWS: usize = [$(stringify!($row)),*].len();
        };
    }

    ops! {
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

//#endregion ⚙️ op

//#region 🧩 kit graph engine

pub mod kit_graph_engine {
    //! 🧩 Core kit graph engine: internal handle-backed slots, deterministic ephemeral semantic diffs, async apply for bundle replay and multi-`Graph` states (`wip` / `authoritative`).
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use serde::Deserialize;

    use crate::error::SemioError;
    use crate::hash::h;
    use crate::id::Id;
    use crate::kit;
    use crate::op;
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

    //#region 📦 semantic diff
    /// @emoji 📦 Deterministic non-persisted diff from op kind + payload + projection fingerprint transition.
    pub fn deterministic_semantic_diff(op_kind: &str, payload_json: &str, projection_fp_before: &str, projection_fp_after: &str) -> op::SemanticDiff {
        let digest = h(&[op_kind, payload_json, projection_fp_before, projection_fp_after]);
        op::SemanticDiff { id: Id::from(format!("semio:diff:{digest}")), summary: Some(digest) }
    }
    //#endregion 📦 semantic diff

    //#region 🪡 semantic op apply
    /// @emoji 🧾 Output of [`apply_semantic_op_json`]: ephemeral diff + optional created entities.
    pub struct AppliedSemanticOp {
        pub diff: op::SemanticDiff,
        pub created_piece: Option<Arc<kit::design::piece::Piece>>,
    }

    #[derive(Debug, Deserialize)]
    struct CreatedFixedPiecePayload {
        #[serde(rename = "designId")]
        design_id: String,
        #[serde(rename = "blueprintId")]
        blueprint_id: String,
        position: crate::geom::Position,
        name: Option<String>,
        description: Option<String>,
    }

    /// @emoji 🪡 Async apply for one persisted semantic op (`kind` + JSON payload); routes through [`Graph::apply_create_fixed_piece`] for `createdFixedPiece`.
    pub async fn apply_semantic_op_json(graph: &Arc<Graph>, draft_id: &Id, transaction_id: &Id, op_kind: &str, payload_json: &str) -> Result<AppliedSemanticOp, SemioError> {
        match op_kind {
            "createdFixedPiece" => {
                let payload: CreatedFixedPiecePayload = serde_json::from_str(payload_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                let design_id = Id::from(payload.design_id.as_str());
                let blueprint_id = Id::from(payload.blueprint_id.as_str());
                let (piece, diff) = graph.apply_create_fixed_piece(draft_id.clone(), transaction_id.clone(), design_id, blueprint_id, payload.position, payload.name, payload.description).await?;
                Ok(AppliedSemanticOp { diff, created_piece: Some(piece) })
            }
            other => Err(SemioError::invalid(format!("unsupported semantic op kind `{other}`"))),
        }
    }
    //#endregion 🪡 semantic op apply
}

//#endregion 🧩 kit graph engine

//#region 🗄️ kit backbone persistence (native)

pub mod kit_backbone {
    //! @emoji 🗄️ Dev JSON + local `.semio/` kit backbones: atomic single-file writes, multi-db SQLite + blobs dir, replay via [`kit_graph_engine::apply_semantic_op_json`].
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
    use crate::op::BackboneStoreKind;
    use crate::vcs::Graph;

    //#region 🧾 wire format

    /// @emoji 🪪 On-disk schema marker stamped at the bundle root; matches `semio/assets/semio/metabolism.new.kit.semio.json`.
    pub const KIT_STORE_BUNDLE_SCHEMA: &str = "🎆26🌙06⬆️1";

    /// @emoji 🚧 Block-merkle hash sentinel reused everywhere until real hashing is wired (matches the literal `"…"` placeholders in the metabolism fixture).
    pub const HASH_PLACEHOLDER: &str = "…";

    /// @emoji 📜 `{hash, items: [T]}` envelope — the universal "block-hashed list" reused in every nested collection of the bundle.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct BlockHashedListDto<T> {
        pub hash: String,
        pub items: Vec<T>,
    }

    impl<T> Default for BlockHashedListDto<T> {
        fn default() -> Self {
            Self { hash: HASH_PLACEHOLDER.to_string(), items: Vec::new() }
        }
    }

    /// @emoji 🔗 `{id, hash}` typed reference to another node in the bundle (authors, qualities, ports, families, …).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct HashRefDto {
        pub id: String,
        pub hash: String,
    }

    /// @emoji 📦 Top-level on-disk kit store bundle (mirrors `metabolism.new.kit.semio.json`: `schema / wip / authoritative / stage / conflicts / blobs`).
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
        #[serde(default = "empty_root_value")]
        pub root: serde_json::Value,
        #[serde(default)]
        pub checkpoints: BlockHashedListDto<serde_json::Value>,
        #[serde(default)]
        pub alternatives: BlockHashedListDto<serde_json::Value>,
        #[serde(default)]
        pub drafts: BlockHashedListDto<DraftDto>,
    }

    /// @emoji 📝 Draft record: pointer to a checkpoint plus ordered transactions of forward / backward semantic ops.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DraftDto {
        pub id: String,
        pub hash: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub checkpoint: Option<HashRefDto>,
        #[serde(default)]
        pub transactions: BlockHashedListDto<TransactionDto>,
    }

    /// @emoji 💼 Transaction = ordered forward + backward semantic op steps (mirrors `Transaction.forwards` / `.backwards` in the bundle).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct TransactionDto {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub forwards: BlockHashedListDto<TransactionStepDto>,
        #[serde(default)]
        pub backwards: BlockHashedListDto<TransactionStepDto>,
    }

    /// @emoji 🪡 One semantic op step inside a transaction (id, op `kind`, optional human description, payload `input`).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct TransactionStepDto {
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
            "hash": HASH_PLACEHOLDER,
            "name": "",
            "types": { "hash": HASH_PLACEHOLDER, "items": [] },
            "designs": { "hash": HASH_PLACEHOLDER, "items": [] },
        })
    }

    impl GraphSnapshotDto {
        /// @emoji 🌱 Empty graph snapshot stamped with `kit_id` (used for fresh `wip` / `authoritative` / `stage` heads).
        pub fn empty(kit_id: &str) -> Self {
            Self {
                id: kit_id.to_string(),
                hash: HASH_PLACEHOLDER.to_string(),
                authors: BlockHashedListDto::default(),
                root: empty_root_value(),
                checkpoints: BlockHashedListDto::default(),
                alternatives: BlockHashedListDto::default(),
                drafts: BlockHashedListDto::default(),
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

        /// @emoji 🔁 Flatten every recorded `wip` draft transaction into ordered [`StoredSemanticOp`] records ready for replay.
        pub fn wip_semantic_ops(&self) -> Vec<StoredSemanticOp> {
            let mut out = Vec::new();
            for draft in &self.wip.drafts.items {
                for tx in &draft.transactions.items {
                    for step in &tx.forwards.items {
                        out.push(StoredSemanticOp {
                            draft_id: draft.id.clone(),
                            transaction_id: tx.id.clone(),
                            kind: step.kind.clone(),
                            input: step.input.clone(),
                        });
                    }
                }
            }
            out
        }

        /// @emoji 📸 Project the live `Graph` into a metabolism-shaped bundle ready for atomic write.
        /// `wip.id` mirrors the graph id; `wip.root` is the camelCase `KitFullDto` projection of `the_kit`;
        /// checkpoints / drafts / transactions echo the in-memory state so the on-disk file matches what
        /// sketchpad is currently editing. The dev who reads the file should see the same shape as
        /// `semio/assets/semio/metabolism.new.kit.semio.json`.
        pub async fn from_graph(graph: &crate::vcs::Graph) -> Self {
            let mut bundle = Self::template();
            let kit_dto = graph.the_kit.kit_full_snapshot_value().await;
            let gid = graph.id.as_str().to_string();
            bundle.wip.id = gid.clone();
            bundle.authoritative.id = gid.clone();
            bundle.stage.id = gid;
            bundle.wip.root = kit_dto;

            // 🪧 Project checkpoints (each anchored on a kit snapshot we currently leave at the placeholder hash).
            for cp in graph.checkpoints.read().await.iter() {
                let msg = cp.message.read().await.clone().unwrap_or_default();
                let ts = cp.timestamp.read().await.clone().map(|t| t.0).unwrap_or_else(|| HASH_PLACEHOLDER.to_string());
                bundle.wip.checkpoints.items.push(serde_json::json!({
                    "id": cp.id.as_str(),
                    "hash": HASH_PLACEHOLDER,
                    "timestamp": ts,
                    "message": msg,
                    "authors": { "hash": HASH_PLACEHOLDER, "items": [] },
                    "changes": { "hash": HASH_PLACEHOLDER, "items": [] },
                }));
            }

            // 📝 Project drafts and the transactions inside them (open + finalized — both visible on disk).
            for draft in graph.drafts.read().await.iter() {
                let mut txs: Vec<TransactionDto> = Vec::new();
                for tx in draft.transactions.read().await.iter() {
                    txs.push(TransactionDto { id: tx.id.as_str().to_string(), hash: HASH_PLACEHOLDER.to_string(), forwards: BlockHashedListDto::default(), backwards: BlockHashedListDto::default() });
                }
                for tx in draft.finalized_transactions.read().await.iter() {
                    txs.push(TransactionDto { id: tx.id.as_str().to_string(), hash: HASH_PLACEHOLDER.to_string(), forwards: BlockHashedListDto::default(), backwards: BlockHashedListDto::default() });
                }
                let parent = draft.parent_checkpoint.read().await.upgrade().map(|c| HashRefDto { id: c.id.as_str().to_string(), hash: HASH_PLACEHOLDER.to_string() });
                bundle.wip.drafts.items.push(DraftDto {
                    id: draft.id.as_str().to_string(),
                    hash: HASH_PLACEHOLDER.to_string(),
                    checkpoint: parent,
                    transactions: BlockHashedListDto { hash: HASH_PLACEHOLDER.to_string(), items: txs },
                });
            }

            bundle
        }

        /// @emoji 🩻 Hydrate the live graph from a previously-persisted bundle JSON. Today this only restores
        /// the kit projection (`wip.root`) into `the_kit`; the draft / transaction structure is preserved on
        /// disk via `from_graph` round-trip but op replay is owned by the existing semantic-op log path.
        pub async fn hydrate_into_graph(graph: &std::sync::Arc<crate::vcs::Graph>, json: &str) -> Result<Self, SemioError> {
            let bundle: Self = serde_json::from_str(json).map_err(|e| SemioError::invalid(format!("bundle parse: {e}")))?;
            if bundle.schema != KIT_STORE_BUNDLE_SCHEMA {
                return Err(SemioError::invalid(format!("bundle schema mismatch: {} != {}", bundle.schema, KIT_STORE_BUNDLE_SCHEMA)));
            }
            if !bundle.wip.root.is_null() && bundle.wip.root.is_object() {
                graph.the_kit.hydrate_from_kit_full_snapshot_json(&bundle.wip.root).await?;
            }
            Ok(bundle)
        }

        /// @emoji 🌱 Initialize an empty bundle with a non-empty `wip` head: empty root projection stamped with `kit_id`,
        /// a single seed checkpoint anchored on the empty kit, and a single empty draft pointing at that checkpoint.
        /// This is the "create dev kit" bootstrap state that sketchpad sees the moment a JSON file is opened/created.
        pub fn initialize_with_active_draft(kit_id: &str, draft_id: &str, checkpoint_id: &str) -> Self {
            let mut bundle = Self::template();
            bundle.wip.id = kit_id.to_string();
            bundle.authoritative.id = kit_id.to_string();
            bundle.stage.id = kit_id.to_string();
            // 🪧 First checkpoint anchors the kit at its initial empty projection (no changes yet).
            bundle.wip.checkpoints.items.push(serde_json::json!({
                "id": checkpoint_id,
                "hash": HASH_PLACEHOLDER,
                "timestamp": HASH_PLACEHOLDER,
                "message": "init",
                "authors": { "hash": HASH_PLACEHOLDER, "items": [] },
                "changes": { "hash": HASH_PLACEHOLDER, "items": [] },
            }));
            // 📝 Active draft on the seed checkpoint (no transactions yet).
            bundle.wip.drafts.items.push(DraftDto {
                id: draft_id.to_string(),
                hash: HASH_PLACEHOLDER.to_string(),
                checkpoint: Some(HashRefDto { id: checkpoint_id.to_string(), hash: HASH_PLACEHOLDER.to_string() }),
                transactions: BlockHashedListDto::default(),
            });
            bundle
        }

        /// @emoji 🪪 Locate the wip draft mutably (returns `Err` if not present).
        fn wip_draft_mut(&mut self, draft_id: &str) -> Result<&mut DraftDto, SemioError> {
            self.wip
                .drafts
                .items
                .iter_mut()
                .find(|d| d.id == draft_id)
                .ok_or_else(|| SemioError::not_found("Draft", draft_id))
        }

        /// @emoji 🟢 Open a new (empty) transaction inside the targeted `wip` draft and return its id; the draft is created if absent.
        pub fn open_transaction(&mut self, draft_id: &str) -> String {
            let drafts = &mut self.wip.drafts.items;
            let draft_idx = match drafts.iter().position(|d| d.id == draft_id) {
                Some(i) => i,
                None => {
                    drafts.push(DraftDto {
                        id: draft_id.to_string(),
                        hash: HASH_PLACEHOLDER.to_string(),
                        checkpoint: None,
                        transactions: BlockHashedListDto::default(),
                    });
                    drafts.len() - 1
                }
            };
            let tx_id = uuid::Uuid::now_v7().to_string();
            drafts[draft_idx].transactions.items.push(TransactionDto {
                id: tx_id.clone(),
                hash: HASH_PLACEHOLDER.to_string(),
                forwards: BlockHashedListDto::default(),
                backwards: BlockHashedListDto::default(),
            });
            tx_id
        }

        /// @emoji ✅ Mark a transaction as finalized inside its draft. Until checkpointing lands the row stays in
        /// `wip.drafts[*].transactions[*]`; the call is a no-op except for validating that the (draft, tx) pair exists.
        pub fn commit_transaction(&mut self, draft_id: &str, transaction_id: &str) -> Result<(), SemioError> {
            let draft = self.wip_draft_mut(draft_id)?;
            if draft.transactions.items.iter().any(|t| t.id == transaction_id) {
                Ok(())
            } else {
                Err(SemioError::not_found("Transaction", transaction_id))
            }
        }

        /// @emoji ⛔ Drop a transaction (and all its steps) from the targeted draft.
        pub fn abort_transaction(&mut self, draft_id: &str, transaction_id: &str) -> Result<(), SemioError> {
            let draft = self.wip_draft_mut(draft_id)?;
            let before = draft.transactions.items.len();
            draft.transactions.items.retain(|t| t.id != transaction_id);
            if draft.transactions.items.len() == before {
                return Err(SemioError::not_found("Transaction", transaction_id));
            }
            Ok(())
        }

        /// @emoji ➕ Append a single forward step to the targeted draft / transaction in `wip` (creating either if absent).
        pub fn append_wip_step(&mut self, draft_id: &str, transaction_id: &str, kind: &str, input: serde_json::Value) {
            let drafts = &mut self.wip.drafts.items;
            let draft_idx = match drafts.iter().position(|d| d.id == draft_id) {
                Some(i) => i,
                None => {
                    drafts.push(DraftDto {
                        id: draft_id.to_string(),
                        hash: HASH_PLACEHOLDER.to_string(),
                        checkpoint: None,
                        transactions: BlockHashedListDto::default(),
                    });
                    drafts.len() - 1
                }
            };
            let txs = &mut drafts[draft_idx].transactions.items;
            let tx_idx = match txs.iter().position(|t| t.id == transaction_id) {
                Some(i) => i,
                None => {
                    txs.push(TransactionDto {
                        id: transaction_id.to_string(),
                        hash: HASH_PLACEHOLDER.to_string(),
                        forwards: BlockHashedListDto::default(),
                        backwards: BlockHashedListDto::default(),
                    });
                    txs.len() - 1
                }
            };
            txs[tx_idx].forwards.items.push(TransactionStepDto {
                id: uuid::Uuid::now_v7().to_string(),
                hash: HASH_PLACEHOLDER.to_string(),
                kind: kind.to_string(),
                description: None,
                input,
            });
        }

        /// @emoji 🪪 Build a metabolism-shaped bundle from a flat ordered semantic op log (used by golden test fixtures and import paths).
        pub fn from_stored_semantic_ops(ops: &[StoredSemanticOp]) -> Self {
            let mut bundle = Self::template();
            for op in ops {
                bundle.append_wip_step(&op.draft_id, &op.transaction_id, &op.kind, op.input.clone());
            }
            bundle
        }
    }

    /// @emoji 📜 Internal value type used by replay + the SQLite local-`.semio/` path; not part of the on-disk dev-json wire format.
    #[derive(Clone, Debug)]
    pub struct StoredSemanticOp {
        pub draft_id: String,
        pub transaction_id: String,
        pub kind: String,
        pub input: serde_json::Value,
    }
    //#endregion 🧾 wire format

    //#region 🧭 paths + uri
    pub fn normalize_connection_uri(raw: &str) -> String {
        raw.trim().to_string()
    }

    pub fn filesystem_path_from_uri(uri: &str) -> Result<PathBuf, SemioError> {
        let u = uri.trim();
        let p = if let Some(r) = u.strip_prefix("file://") { r } else { u };
        if p.is_empty() {
            return Err(SemioError::invalid("empty backbone connectionUri"));
        }
        Ok(PathBuf::from(p))
    }

    fn resolve_local_semio_root(project_or_dot_semio: &Path) -> PathBuf {
        if project_or_dot_semio.file_name().and_then(|s| s.to_str()) == Some(".semio") {
            project_or_dot_semio.to_path_buf()
        } else {
            project_or_dot_semio.join(".semio")
        }
    }

    fn init_local_dot_semio_layout(semio_root: &Path) -> Result<(), SemioError> {
        std::fs::create_dir_all(semio_root).map_err(|e| SemioError::invalid(format!("create .semio root: {e}")))?;
        std::fs::create_dir_all(semio_root.join("blobs")).map_err(|e| SemioError::invalid(format!("create blobs dir: {e}")))?;
        let ddl = r#"
CREATE TABLE IF NOT EXISTS semantic_op_log (
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

    fn db_file_for_child(semio_root: &Path, child_label: &'static str) -> Result<PathBuf, SemioError> {
        let name = match child_label {
            "wip" => "wip.db",
            "auth" => "authoritative.db",
            other => return Err(SemioError::invalid(format!("unknown child label `{other}` for local backbone"))),
        };
        Ok(semio_root.join(name))
    }
    //#endregion 🧭 paths + uri

    //#region ✍️ atomic json
    fn atomic_write_bundle(path: &Path, doc: &KitStoreBundleFile) -> Result<(), SemioError> {
        let parent = path.parent().ok_or_else(|| SemioError::invalid("kit-store bundle path has no parent directory"))?;
        std::fs::create_dir_all(parent).map_err(|e| SemioError::invalid(format!("create kit-store bundle parent: {e}")))?;
        let tmp = path.with_extension("tmp.semio-write");
        let body = serde_json::to_string_pretty(doc).map_err(|e| SemioError::invalid(e.to_string()))?;
        std::fs::write(&tmp, body).map_err(|e| SemioError::invalid(format!("write temp kit-store bundle: {e}")))?;
        std::fs::rename(&tmp, path).map_err(|e| SemioError::invalid(format!("rename kit-store bundle: {e}")))?;
        Ok(())
    }

    fn read_or_init_bundle(path: &Path) -> Result<KitStoreBundleFile, SemioError> {
        if !path.exists() {
            return Ok(KitStoreBundleFile::template());
        }
        let s = std::fs::read_to_string(path).map_err(|e| SemioError::invalid(format!("read kit-store bundle: {e}")))?;
        serde_json::from_str(&s).map_err(|e| SemioError::invalid(format!("parse kit-store bundle: {e}")))
    }
    //#endregion ✍️ atomic json

    //#region 🔁 replay
    pub async fn replay_stored_ops(graph: &Arc<Graph>, ops: &[StoredSemanticOp]) -> Result<(), SemioError> {
        graph.the_kit.clear_piece_projections_for_backbone_replay().await;
        for op in ops {
            let draft_id = Id::from(op.draft_id.as_str());
            let transaction_id = Id::from(op.transaction_id.as_str());
            let payload = serde_json::to_string(&op.input).map_err(|e| SemioError::invalid(e.to_string()))?;
            crate::kit_graph_engine::apply_semantic_op_json(graph, &draft_id, &transaction_id, op.kind.as_str(), &payload).await?;
        }
        Ok(())
    }
    //#endregion 🔁 replay

    //#region 🧩 attached variants
    pub struct DevJsonAttached {
        path: PathBuf,
        connection_uri_normalized: String,
    }

    impl DevJsonAttached {
        /// @emoji 📥 Read the on-disk bundle (or fresh template if file is absent).
        pub fn read_bundle(&self) -> Result<KitStoreBundleFile, SemioError> {
            read_or_init_bundle(&self.path)
        }

        /// @emoji ➕ Append a forward semantic op step into the targeted `wip` draft / transaction and atomically rewrite the bundle.
        pub fn append_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            let mut doc = self.read_bundle()?;
            doc.append_wip_step(draft_id.as_str(), transaction_id.as_str(), kind, input.clone());
            atomic_write_bundle(&self.path, &doc)
        }
    }

    pub struct LocalAttached {
        #[allow(dead_code)]
        semio_root: PathBuf,
        db_path: PathBuf,
        connection_uri_normalized: String,
    }

    impl LocalAttached {
        pub fn append_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite append: {e}")))?;
            let input_json = serde_json::to_string(input).map_err(|e| SemioError::invalid(e.to_string()))?;
            conn.execute("INSERT INTO semantic_op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![draft_id.as_str(), transaction_id.as_str(), kind, input_json])
                .map_err(|e| SemioError::invalid(format!("sqlite insert: {e}")))?;
            Ok(())
        }

        fn load_ops(&self) -> Result<Vec<StoredSemanticOp>, SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite read: {e}")))?;
            let mut stmt = conn.prepare("SELECT draft_id, transaction_id, kind, input_json FROM semantic_op_log ORDER BY seq ASC").map_err(|e| SemioError::invalid(format!("sqlite prepare: {e}")))?;
            let mut rows = stmt.query([]).map_err(|e| SemioError::invalid(format!("sqlite query: {e}")))?;
            let mut out = Vec::new();
            while let Some(row) = rows.next().map_err(|e| SemioError::invalid(format!("sqlite row: {e}")))? {
                let draft_id: String = row.get(0).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let transaction_id: String = row.get(1).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let kind: String = row.get(2).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input_json: String = row.get(3).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input: serde_json::Value = serde_json::from_str(&input_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                out.push(StoredSemanticOp { draft_id, transaction_id, kind, input });
            }
            Ok(out)
        }
    }

    pub enum AttachedBackbone {
        DevJson(DevJsonAttached),
        Local(LocalAttached),
    }

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
            let ops: Vec<StoredSemanticOp> = match self {
                AttachedBackbone::DevJson(d) => d.read_bundle()?.wip_semantic_ops(),
                AttachedBackbone::Local(l) => l.load_ops()?,
            };
            replay_stored_ops(graph, &ops).await
        }

        pub fn append_semantic_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
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

    /// @emoji 🧪 Build [`StoredSemanticOp`] rows from `kit-store.golden.ops.semio.json` (US-001 fixture) for persistence tests.
    pub fn stored_ops_from_golden_ops_json(src: &serde_json::Value) -> Result<Vec<StoredSemanticOp>, SemioError> {
        let draft_id = src["draftId"].as_str().ok_or_else(|| SemioError::invalid("golden ops missing draftId"))?.to_string();
        let transaction_id = src["transactionId"].as_str().ok_or_else(|| SemioError::invalid("golden ops missing transactionId"))?.to_string();
        let arr = src["ops"].as_array().ok_or_else(|| SemioError::invalid("golden ops missing ops"))?;
        let mut out = Vec::new();
        for rec in arr {
            let kind = rec["kind"].as_str().ok_or_else(|| SemioError::invalid("op.kind"))?;
            let input = rec.get("input").cloned().ok_or_else(|| SemioError::invalid("op.input"))?;
            out.push(StoredSemanticOp { draft_id: draft_id.clone(), transaction_id: transaction_id.clone(), kind: kind.to_string(), input });
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
    use crate::op;

    /// 🌐 Broadcast envelope for every observable thing the control plane emits.
    #[derive(Clone)]
    pub enum Event {
        CommandSucceeded(op::CommandReceipt),
        OperationSucceeded(op::OperationKind),
        OperationFailed(SemioError),
        CreatedFixedPiece(Arc<op::CreatedFixedPiece>),
        FixedPiece(Arc<op::FixedPiece>),
        DraggedPiece(Arc<op::DraggedPiece>),
        RenamedKit(Arc<op::RenamedKit>),
        ChangedDescription(Arc<op::ChangedDescription>),
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
    use std::sync::{Arc, Weak};

    use async_channel::{Receiver, Sender};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::event::{Event, EventBus};
    use crate::id::Id;
    use crate::op::{BackboneStoreKind, Command, CommandReceipt, CreatedFixedPiece, CreatedFixedPieceInput, OperationIface, RenamedKit, RenamedKitInput, SemanticDiff};
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

        pub async fn record_created_fixed_piece_if_attached(&self, draft_id: &Id, transaction_id: &Id, payload: &serde_json::Value) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let mut guard = self.slot.write().await;
                if let Some(backbone) = guard.as_mut() {
                    backbone.append_semantic_op(draft_id, transaction_id, "createdFixedPiece", payload)?;
                }
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (draft_id, transaction_id, payload);
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

            Arc::new(Self { bus, wip: ChildPort { inbound: wip_tx }, auth: ChildPort { inbound: auth_tx }, wip_graph, auth_graph, sessions: RwLock::new(Vec::new()), conflicts: RwLock::new(Vec::new()) })
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

            Ok(Arc::new(Self { bus, wip: ChildPort { inbound: wip_tx }, auth: ChildPort { inbound: auth_tx }, wip_graph, auth_graph, sessions: RwLock::new(Vec::new()), conflicts: RwLock::new(Vec::new()) }))
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
                    Command::AddFixedPieceToDesign { .. } => "addFixedPieceToDesign",
                    Command::FixPieceInDesign { .. } => "fixPieceInDesign",
                    Command::RenameKit { .. } => "renameKit",
                    Command::ChangeDescription { .. } => "changeDescription",
                    Command::CreateTag { .. } => "createTag",
                    Command::CreateTags { .. } => "createTags",
                    Command::RenameTag { .. } => "renameTag",
                    Command::DeleteTag { .. } => "deleteTag",
                    Command::DeleteTags { .. } => "deleteTags",
                    Command::CreateConcept { .. } => "createConcept",
                    Command::CreateQuality { .. } => "createQuality",
                    Command::DragPieceInDesign { .. } => "dragPieceInDesign",
                    Command::DragPiecesInDesign { .. } => "dragPiecesInDesign",
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
                Command::AddFixedPieceToDesign { request_id: _, draft_id, transaction_id, design_id, blueprint_id, position, name, description } => {
                    let (piece, diff) = self.graph.apply_create_fixed_piece(draft_id.clone(), transaction_id.clone(), design_id.clone(), blueprint_id.clone(), position, name.clone(), description.clone()).await?;

                    let payload = serde_json::json!({
                        "designId": design_id.as_str(),
                        "blueprintId": blueprint_id.as_str(),
                        "position": serde_json::to_value(position).map_err(|e| SemioError::invalid(e.to_string()))?,
                        "name": name.clone(),
                        "description": description.clone(),
                    });
                    self.backbone.record_created_fixed_piece_if_attached(&draft_id, &transaction_id, &payload).await?;

                    let input = CreatedFixedPieceInput { design_id, blueprint_id, position, name, description };
                    let op = CreatedFixedPiece::new(input, piece, diff).await;
                    self.graph.op_history.write().await.push(Arc::new(OperationIface::CreatedFixedPiece(op.clone())));
                    self.bus.emit_event(Event::CreatedFixedPiece(op)).await;
                    Ok(())
                }
                Command::FixPieceInDesign { design_id, piece_id, .. } => {
                    let kit = &self.graph.the_kit;
                    let design = kit.design_by_external_id(&design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
                    let piece = design.piece_by_external_id(&piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
                    *piece.connection_kind.write().await = Some(crate::kit::design::piece::PieceConnectionKind::Fixed);
                    kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::RenameKit { request_id, name, .. } => {
                    if name.chars().count() > 256 {
                        return Err(SemioError::invalid(format!("Kit name too long: {} > 256", name.chars().count())));
                    }
                    *self.graph.the_kit.name.write().await = name.clone();
                    self.graph.the_kit.bump_touch_epoch().await;
                    let mut diff = SemanticDiff::default();
                    diff.id = Id::new().await;
                    diff.summary = Some("renameKit".to_string());
                    let op = Arc::new(RenamedKit { id: Id::new().await, request_id: request_id.clone(), owner_change: Weak::new(), input: RenamedKitInput { name }, diff, kit: self.graph.the_kit.clone() });
                    self.graph.op_history.write().await.push(Arc::new(OperationIface::RenamedKit(op.clone())));
                    self.bus.emit_event(Event::RenamedKit(op)).await;
                    Ok(())
                }
                Command::ChangeDescription { entity_id, description, .. } => {
                    let kit = &self.graph.the_kit;
                    let kid = kit.workspace_kit_id().await;
                    if entity_id == kid || entity_id == kit.id {
                        *kit.description.write().await = Some(description);
                    } else if let Some(t) = kit.type_by_external_id(&entity_id).await {
                        *t.description.write().await = Some(description);
                    } else if let Some(d) = kit.design_by_external_id(&entity_id).await {
                        *d.description.write().await = Some(description);
                    } else {
                        return Err(SemioError::not_found("DescriptionEntity", entity_id.as_str()));
                    }
                    kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::CreateTag { owner_id, input, .. } => {
                    self.graph.the_kit.create_and_register_tag(&owner_id, input).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::CreateTags { owner_id, inputs, .. } => {
                    for inp in inputs {
                        self.graph.the_kit.create_and_register_tag(&owner_id, inp).await?;
                    }
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::RenameTag { tag_id, name, .. } => {
                    let tag = self.graph.the_kit.find_tag(&tag_id).await.ok_or_else(|| SemioError::not_found("Tag", tag_id.as_str()))?;
                    *tag.name.write().await = name;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::DeleteTag { tag_id, .. } => {
                    self.graph.the_kit.delete_tag_by_id(&tag_id).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::DeleteTags { tag_ids, .. } => {
                    for id in tag_ids {
                        self.graph.the_kit.delete_tag_by_id(&id).await?;
                    }
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::CreateConcept { owner_id, input, .. } => {
                    let _ = self.graph.the_kit.create_and_register_concept(&owner_id, input).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::CreateQuality { owner_id, input, .. } => {
                    let _ = self.graph.the_kit.create_and_register_quality(&owner_id, input).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::DragPieceInDesign { design_id, piece_id, offset, .. } => {
                    self.graph.apply_drag_piece_in_design(&design_id, &piece_id, offset).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::DragPiecesInDesign { design_id, piece_ids, offset, .. } => {
                    self.graph.apply_drag_pieces_in_design(&design_id, &piece_ids, offset).await?;
                    self.graph.the_kit.bump_touch_epoch().await;
                    Ok(())
                }
                Command::BackboneAttach { connection_uri, store_kind, .. } => self.backbone.mount(&self.graph, self.label, &connection_uri, store_kind).await,
                Command::BackboneDetach { connection_uri, .. } => self.backbone.detach_matching(&connection_uri).await,
            }
        }
    }
}

//#endregion 🧵 worker

//#region 🌐 gql

pub mod gql {
    //! 🌐 Type-safe static GraphQL schema via `Schema::build` (embedded target SDL string for tooling).
    use async_graphql::{Context, Object, Schema, Subscription};
    use async_stream::stream;
    use futures_util::Stream;
    use std::pin::Pin;
    use std::sync::Arc;

    use crate::error::SemioError;
    use crate::event::{Event, EventBus};
    use crate::geom::{Offset, Position};
    use crate::id::Id;
    use crate::meta::{ConceptInput, QualityInput, TagInput};
    use crate::op::{Command, CommandReceipt, OperationKind, RenamedKit};
    use crate::vcs::Graph;
    use crate::worker::ParentRuntime;

    /// @emoji 🧩 Executable schema (`Query`, `Mutation`, `Subscription`).
    pub type AppSchema = Schema<Query, Mutation, Subscription>;

    pub struct Query;

    #[Object]
    impl Query {
        pub async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.wip_graph.clone())
        }

        #[graphql(name = "authoritative")]
        pub async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.auth_graph.clone())
        }

        pub async fn session(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<Arc<crate::vcs::Session>>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let sessions = rt.sessions.read().await;
            Ok(sessions.iter().find(|s| s.id == id).cloned())
        }

        pub async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql_relay::ConflictConnection> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let list = rt.conflicts.read().await.clone();
            Ok(crate::gql_relay::ConflictConnection::from_conflicts(list))
        }

        /// @emoji 🔎 Relay-style global `node` lookup (WIP + authoritative + sessions + conflicts).
        pub async fn node(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::resolve_node(rt.as_ref(), &id).await)
        }

        /// @emoji 🔎 Alias of [`Query::node`] for SDL `entity` entry point.
        pub async fn entity(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            self.node(ctx, id).await
        }

        /// @emoji 🧩 Resolve a piece within a design on the WIP graph line.
        #[graphql(name = "pieceInDesign")]
        pub async fn piece_in_design(&self, ctx: &Context<'_>, #[graphql(name = "designId")] design_id: Id, #[graphql(name = "pieceId")] piece_id: Id) -> async_graphql::Result<Option<Arc<crate::kit::design::piece::Piece>>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::piece_in_design_on_wip(rt.as_ref(), &design_id, &piece_id).await)
        }

        /// @emoji 🧩 Alternative-line piece kind (stub until alternatives are modeled in Rust).
        #[graphql(name = "alternativePieceKind")]
        pub async fn alternative_piece_kind(&self, ctx: &Context<'_>, #[graphql(name = "pieceId")] piece_id: Id) -> async_graphql::Result<Option<String>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::alternative_piece_kind(rt.as_ref(), &piece_id).await)
        }

        /// @emoji 📸 Serialize the live `wip` graph as a metabolism-shaped bundle JSON (matches `semio/assets/semio/metabolism.new.kit.semio.json`).
        /// Sketchpad calls this after every change and atomically writes the bytes to the dev-kit JSON file via the host adapter.
        #[graphql(name = "kitStoreBundleJson")]
        pub async fn kit_store_bundle_json(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let bundle = crate::kit_backbone::KitStoreBundleFile::from_graph(rt.wip_graph.as_ref()).await;
            serde_json::to_string_pretty(&bundle).map_err(|e| async_graphql::Error::new(e.to_string()))
        }
    }

    pub struct Mutation;

    #[Object]
    impl Mutation {
        /// @emoji ➕ Routes through [`ParentRuntime::dispatch_wip`] → child apply + event bus.
        #[graphql(name = "addFixedPieceToDesign")]
        pub async fn add_fixed_piece_to_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "blueprintId")] blueprint_id: Id,
            position: Position,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::AddFixedPieceToDesign { request_id: request_id.clone(), draft_id, transaction_id, design_id, blueprint_id, position, name: None, description: None };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "fixPieceInDesign")]
        pub async fn fix_piece_in_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceId")] piece_id: Id,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::FixPieceInDesign { request_id: request_id.clone(), draft_id, transaction_id, design_id, piece_id };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "renameKit")]
        pub async fn rename_kit(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id, name: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::RenameKit { request_id: request_id.clone(), draft_id, transaction_id, name };
            Ok(rt.dispatch_wip(cmd).await)
        }

        /// @emoji 🟢 Open a brand-new transaction inside the targeted `wip` draft (creating the draft if missing); returns the new transaction id.
        #[graphql(name = "transactionOpen")]
        pub async fn transaction_open(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let tx = rt.wip_graph.open_transaction(&draft_id).await;
            Ok(tx.id.clone())
        }

        /// @emoji ✅ Finalize a transaction inside the targeted `wip` draft; moves it to `finalizedTransactions`.
        #[graphql(name = "transactionCommit")]
        pub async fn transaction_commit(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id) -> async_graphql::Result<bool> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            rt.wip_graph.commit_transaction(&draft_id, &transaction_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            Ok(true)
        }

        /// @emoji ⛔ Drop an in-flight transaction inside the targeted `wip` draft (no-op for already-finalized ones).
        #[graphql(name = "transactionAbort")]
        pub async fn transaction_abort(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id) -> async_graphql::Result<bool> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            rt.wip_graph.abort_transaction(&draft_id, &transaction_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            Ok(true)
        }

        /// @emoji 🌱 Bootstrap the `wip` graph for a fresh dev kit: ensures a seed checkpoint anchored on `the_kit`
        /// and a default draft on that checkpoint. Idempotent — returns the active default draft id (existing or new).
        /// Sketchpad calls this once when a JSON file backbone is mounted so the on-disk bundle immediately
        /// shows "root + first checkpoint + first draft".
        #[graphql(name = "kitStoreInitializeDefaults")]
        pub async fn kit_store_initialize_defaults(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let draft = rt.wip_graph.ensure_default_seed_state().await;
            Ok(draft.id.clone())
        }

        /// @emoji 🌱 Create a new alternative branch from the current checkpoint tip (`sourceAlternativeId` omitted = fork from the kit main line).
        #[graphql(name = "createAlternativeFromTip")]
        pub async fn create_alternative_from_tip(
            &self,
            ctx: &Context<'_>,
            name: String,
            #[graphql(name = "sourceAlternativeId")] source_alternative_id: Option<Id>,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let id = rt
                .wip_graph
                .create_alternative_from_tip(name, source_alternative_id.as_ref())
                .await
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            Ok(id)
        }

        /// @emoji 🩻 Hydrate the `wip` graph from a previously-persisted metabolism-shaped bundle JSON.
        /// Restores the kit projection (`wip.root`); future passes also replay drafts / transactions through the op log.
        #[graphql(name = "kitStoreBundleHydrate")]
        pub async fn kit_store_bundle_hydrate(&self, ctx: &Context<'_>, json: String) -> async_graphql::Result<bool> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            crate::kit_backbone::KitStoreBundleFile::hydrate_into_graph(&rt.wip_graph, &json).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            Ok(true)
        }

        #[graphql(name = "changeDescription")]
        pub async fn change_description(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "entityId")] entity_id: Id,
            description: String,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::ChangeDescription { request_id: request_id.clone(), draft_id, transaction_id, entity_id, description };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "createTag")]
        pub async fn create_tag(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id, #[graphql(name = "ownerId")] owner_id: Id, tag: TagInput) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::CreateTag { request_id: request_id.clone(), draft_id, transaction_id, owner_id, input: tag };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "createTags")]
        pub async fn create_tags(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "ownerId")] owner_id: Id,
            tags: Vec<TagInput>,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::CreateTags { request_id: request_id.clone(), draft_id, transaction_id, owner_id, inputs: tags };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "renameTag")]
        pub async fn rename_tag(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id, #[graphql(name = "tagId")] tag_id: Id, name: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::RenameTag { request_id: request_id.clone(), draft_id, transaction_id, tag_id, name };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "deleteTag")]
        pub async fn delete_tag(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id, #[graphql(name = "tagId")] tag_id: Id) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::DeleteTag { request_id: request_id.clone(), draft_id, transaction_id, tag_id };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "deleteTags")]
        pub async fn delete_tags(&self, ctx: &Context<'_>, #[graphql(name = "draftId")] draft_id: Id, #[graphql(name = "transactionId")] transaction_id: Id, #[graphql(name = "tagIds")] tag_ids: Vec<Id>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::DeleteTags { request_id: request_id.clone(), draft_id, transaction_id, tag_ids };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "createConcept")]
        pub async fn create_concept(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "ownerId")] owner_id: Id,
            concept: ConceptInput,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::CreateConcept { request_id: request_id.clone(), draft_id, transaction_id, owner_id, input: concept };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "createQuality")]
        pub async fn create_quality(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "ownerId")] owner_id: Id,
            quality: QualityInput,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::CreateQuality { request_id: request_id.clone(), draft_id, transaction_id, owner_id, input: quality };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "dragPieceInDesign")]
        pub async fn drag_piece_in_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceId")] piece_id: Id,
            offset: Offset,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::DragPieceInDesign { request_id: request_id.clone(), draft_id, transaction_id, design_id, piece_id, offset };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "dragPiecesInDesign")]
        pub async fn drag_pieces_in_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceIds")] piece_ids: Vec<Id>,
            offset: Offset,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::DragPiecesInDesign { request_id: request_id.clone(), draft_id, transaction_id, design_id, piece_ids, offset };
            Ok(rt.dispatch_wip(cmd).await)
        }
    }

    pub struct Subscription;

    type CommandSucceededStream = Pin<Box<dyn Stream<Item = CommandReceipt> + Send>>;
    type OperationSucceededStream = Pin<Box<dyn Stream<Item = OperationKind> + Send>>;
    type OperationFailedStream = Pin<Box<dyn Stream<Item = SemioError> + Send>>;
    type KitRenamedStream = Pin<Box<dyn Stream<Item = Arc<RenamedKit>> + Send>>;

    #[Subscription]
    impl Subscription {
        #[graphql(name = "commandSucceeded")]
        pub async fn command_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<CommandSucceededStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let mut rx = bus.subscribe();
            Ok(Box::pin(stream! {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            if let Event::CommandSucceeded(r) = ev {
                                yield r;
                            }
                        }
                        Err(_) => break,
                    }
                }
            }))
        }

        #[graphql(name = "kitRenamed")]
        pub async fn kit_renamed(&self, ctx: &Context<'_>) -> async_graphql::Result<KitRenamedStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let mut rx = bus.subscribe();
            Ok(Box::pin(stream! {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            if let Event::RenamedKit(o) = ev {
                                yield o;
                            }
                        }
                        Err(_) => break,
                    }
                }
            }))
        }

        #[graphql(name = "operationSucceeded")]
        pub async fn operation_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationSucceededStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let mut rx = bus.subscribe();
            Ok(Box::pin(stream! {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            match ev {
                                Event::OperationSucceeded(k) => yield k,
                                Event::CreatedFixedPiece(o) => yield OperationKind::CreatedFixedPiece(o),
                                Event::FixedPiece(o) => yield OperationKind::FixedPiece(o),
                                Event::DraggedPiece(o) => yield OperationKind::DraggedPiece(o),
                                Event::RenamedKit(o) => yield OperationKind::RenamedKit(o),
                                Event::ChangedDescription(o) => yield OperationKind::ChangedDescription(o),
                                _ => {}
                            }
                        }
                        Err(_) => break,
                    }
                }
            }))
        }

        #[graphql(name = "operationFailed")]
        pub async fn operation_failed(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationFailedStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let mut rx = bus.subscribe();
            Ok(Box::pin(stream! {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            if let Event::OperationFailed(e) = ev {
                                yield e;
                            }
                        }
                        Err(_) => break,
                    }
                }
            }))
        }

        /// @emoji 🚨 Mirrors [`Subscription::operationFailed`] for SDL `error` consumers.
        pub async fn error(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationFailedStream> {
            self.operation_failed(ctx).await
        }
    }

    fn build_schema_sync_for(rt: Arc<ParentRuntime>) -> AppSchema {
        Schema::build(Query, Mutation, Subscription).data(rt.clone()).data(rt.bus.clone()).finish()
    }

    /// 📜 Executable SDL emitted by the in-Rust schema (code-first).
    pub async fn sdl() -> String {
        build_schema().await.sdl()
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

    fn add_fixed_piece_vars(design_id: &str) -> async_graphql::Value {
        async_graphql::value!({
            "draftId": "d1",
            "transactionId": "t1",
            "designId": design_id,
            "blueprintId": "bp-new",
            "position": position_value()
        })
    }

    const ADD_FIXED_PIECE_TO_DESIGN: &str = r#"
        mutation($draftId: ID!, $transactionId: ID!, $designId: ID!, $blueprintId: ID!, $position: PositionInput!) {
            addFixedPieceToDesign(draftId: $draftId, transactionId: $transactionId, designId: $designId, blueprintId: $blueprintId, position: $position)
        }
    "#;

    fn relay_wip_designs_have_piece() -> &'static str {
        "{ wip { theKit { designs { edges { node { id pieces { edges { node { id position { center { u v } } } } } } } } } } }"
    }

    fn relay_auth_designs_piece_ids() -> &'static str {
        "{ authoritative { theKit { designs { edges { node { pieces { edges { node { id } } } } } } } } }"
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

    #[test]
    fn kit_store_bundle_serialize_hydrate_round_trip_via_graphql() {
        // 📸🩻 Sketchpad path: Mutation.kitStoreInitializeDefaults → Query.kitStoreBundleJson → write to file →
        // re-mount → Mutation.kitStoreBundleHydrate → Query.kitStoreBundleJson again must yield the same root projection.
        block_on(async {
            let schema_a = crate::gql::build_schema().await;

            // Bootstrap defaults on graph A and rename the kit.
            let res = schema_a.execute(r#"mutation { kitStoreInitializeDefaults }"#).await;
            assert!(res.errors.is_empty(), "init defaults errors: {:?}", res.errors);
            let draft_a: String = res.data.into_json().unwrap()["kitStoreInitializeDefaults"].as_str().expect("draft id").to_string();
            assert!(!draft_a.is_empty());

            // Open a transaction and rename inside it.
            let res = schema_a.execute(Request::new(r#"mutation($d: ID!) { transactionOpen(draftId: $d) }"#).variables(Variables::from_value(async_graphql::value!({ "d": draft_a.clone() })))).await;
            let tx_a: String = res.data.into_json().unwrap()["transactionOpen"].as_str().expect("tx id").to_string();
            const RN: &str = r#"mutation($d: Id!, $t: Id!, $n: String!) { renameKit(draftId: $d, transactionId: $t, name: $n) }"#;
            let res = schema_a.execute(Request::new(RN).variables(Variables::from_value(async_graphql::value!({ "d": draft_a.clone(), "t": tx_a.clone(), "n": "Hello Bundle" })))).await;
            assert!(res.errors.is_empty(), "renameKit errors: {:?}", res.errors);
            std::thread::sleep(std::time::Duration::from_millis(150));

            // Serialize bundle to JSON.
            let res = schema_a.execute(r#"{ kitStoreBundleJson }"#).await;
            assert!(res.errors.is_empty(), "serialize errors: {:?}", res.errors);
            let json_a: String = res.data.into_json().unwrap()["kitStoreBundleJson"].as_str().expect("bundle json").to_string();

            // The serialized bundle has the metabolism-shaped top-level keys.
            let v: serde_json::Value = serde_json::from_str(&json_a).expect("bundle parses");
            assert_eq!(v["schema"].as_str().unwrap(), crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
            for k in ["wip", "authoritative", "stage", "conflicts", "blobs"].iter() {
                assert!(v.get(*k).is_some(), "missing top-level key {k}");
            }
            assert!(v["wip"]["root"].is_object(), "wip.root projects the live kit");
            assert_eq!(v["wip"]["root"]["name"].as_str().unwrap_or(""), "Hello Bundle");
            assert!(!v["wip"]["checkpoints"]["items"].as_array().unwrap().is_empty(), "seed checkpoint present");
            assert!(!v["wip"]["drafts"]["items"].as_array().unwrap().is_empty(), "default draft present");

            // Mount a fresh schema B (simulates re-opening sketchpad on the same file) and hydrate from json_a.
            let schema_b = crate::gql::build_schema().await;
            const HY: &str = r#"mutation($j: String!) { kitStoreBundleHydrate(json: $j) }"#;
            let res = schema_b.execute(Request::new(HY).variables(Variables::from_value(async_graphql::value!({ "j": json_a.clone() })))).await;
            assert!(res.errors.is_empty(), "hydrate errors: {:?}", res.errors);
            assert_eq!(res.data.into_json().unwrap()["kitStoreBundleHydrate"], true);

            // Re-serialize on B and check the kit name survived hydration.
            let res = schema_b.execute(r#"{ kitStoreBundleJson }"#).await;
            let json_b: String = res.data.into_json().unwrap()["kitStoreBundleJson"].as_str().expect("bundle json b").to_string();
            let vb: serde_json::Value = serde_json::from_str(&json_b).expect("bundle b parses");
            assert_eq!(vb["wip"]["root"]["name"].as_str().unwrap_or(""), "Hello Bundle", "kit name survives bundle round-trip");
        });
    }

    #[test]
    fn transaction_open_commit_abort_lifecycle_on_wip_graph() {
        // 🟢✅⛔ The full transaction lifecycle drives `wip.drafts[*].transactions / finalizedTransactions` through GraphQL.
        block_on(async {
            let schema = crate::gql::build_schema().await;

            const OPEN: &str = r#"mutation($draftId: ID!) { transactionOpen(draftId: $draftId) }"#;
            let res = schema.execute(Request::new(OPEN).variables(Variables::from_value(async_graphql::value!({ "draftId": "draft-tx-test" })))).await;
            assert!(res.errors.is_empty(), "transactionOpen errors: {:?}", res.errors);
            let tx_id_a: String = res.data.into_json().unwrap()["transactionOpen"].as_str().expect("tx id").to_string();

            // The newly opened transaction is visible on the draft via `wip.draft(id:)`.
            let q = r#"
                query($id: ID!) {
                    wip { draft(id: $id) {
                        id
                        openTransaction { id }
                        orderedTransactionIds
                    } }
                }
            "#;
            let res = schema.execute(Request::new(q).variables(Variables::from_value(async_graphql::value!({ "id": "draft-tx-test" })))).await;
            assert!(res.errors.is_empty(), "draft probe errors: {:?}", res.errors);
            let v = res.data.into_json().unwrap();
            assert_eq!(v["wip"]["draft"]["id"], "draft-tx-test");
            assert_eq!(v["wip"]["draft"]["openTransaction"]["id"], tx_id_a);
            let ordered: Vec<String> = v["wip"]["draft"]["orderedTransactionIds"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
            assert_eq!(ordered, vec![tx_id_a.clone()]);

            // Commit moves the transaction to finalized and clears the open pointer.
            const COMMIT: &str = r#"mutation($d: ID!, $t: ID!) { transactionCommit(draftId: $d, transactionId: $t) }"#;
            let res = schema.execute(Request::new(COMMIT).variables(Variables::from_value(async_graphql::value!({ "d": "draft-tx-test", "t": tx_id_a.clone() })))).await;
            assert!(res.errors.is_empty(), "transactionCommit errors: {:?}", res.errors);
            assert_eq!(res.data.into_json().unwrap()["transactionCommit"], true);

            let res = schema.execute(Request::new(q).variables(Variables::from_value(async_graphql::value!({ "id": "draft-tx-test" })))).await;
            let v = res.data.into_json().unwrap();
            assert!(v["wip"]["draft"]["openTransaction"].is_null(), "open transaction cleared after commit");
            let ordered: Vec<String> = v["wip"]["draft"]["orderedTransactionIds"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
            assert!(ordered.is_empty(), "committed tx removed from active transactions");

            // Abort: open another, abort it, gone.
            let res = schema.execute(Request::new(OPEN).variables(Variables::from_value(async_graphql::value!({ "draftId": "draft-tx-test" })))).await;
            let tx_id_b: String = res.data.into_json().unwrap()["transactionOpen"].as_str().expect("tx id b").to_string();
            const ABORT: &str = r#"mutation($d: ID!, $t: ID!) { transactionAbort(draftId: $d, transactionId: $t) }"#;
            let res = schema.execute(Request::new(ABORT).variables(Variables::from_value(async_graphql::value!({ "d": "draft-tx-test", "t": tx_id_b.clone() })))).await;
            assert!(res.errors.is_empty(), "transactionAbort errors: {:?}", res.errors);
            let res = schema.execute(Request::new(q).variables(Variables::from_value(async_graphql::value!({ "id": "draft-tx-test" })))).await;
            let v = res.data.into_json().unwrap();
            assert!(v["wip"]["draft"]["openTransaction"].is_null(), "open transaction cleared after abort");
            let ordered: Vec<String> = v["wip"]["draft"]["orderedTransactionIds"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
            assert!(ordered.is_empty(), "aborted tx removed");

            // Unknown ids surface as errors.
            let res = schema.execute(Request::new(COMMIT).variables(Variables::from_value(async_graphql::value!({ "d": "draft-tx-test", "t": "missing" })))).await;
            assert!(!res.errors.is_empty(), "commit unknown tx must error");
            let res = schema.execute(Request::new(ABORT).variables(Variables::from_value(async_graphql::value!({ "d": "missing-draft", "t": "x" })))).await;
            assert!(!res.errors.is_empty(), "abort on unknown draft must error");
        });
    }

    #[test]
    fn create_tag_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let kit_id = schema.execute("{ wip { theKit { id } } }").await.data.into_json().unwrap()["wip"]["theKit"]["id"].as_str().expect("kit id").to_string();

            const M: &str = r#"
                mutation($draftId: ID!, $transactionId: ID!, $ownerId: ID!, $tag: TagInput!) {
                    createTag(draftId: $draftId, transactionId: $transactionId, ownerId: $ownerId, tag: $tag)
                }
            "#;
            let vars = async_graphql::value!({
                "draftId": "d1",
                "transactionId": "t1",
                "ownerId": kit_id,
                "tag": { "name": "alpha-tag" }
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createTag errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { tags { edges { node { name } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["wip"]["theKit"]["tags"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
            assert!(names.iter().any(|n| n == "alpha-tag"), "tags missing new name: {:?}", names);
        });
    }

    #[test]
    fn create_concept_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let kit_id = schema.execute("{ wip { theKit { id } } }").await.data.into_json().unwrap()["wip"]["theKit"]["id"].as_str().expect("kit id").to_string();

            const M: &str = r#"
                mutation($draftId: ID!, $transactionId: ID!, $ownerId: ID!, $concept: ConceptInput!) {
                    createConcept(draftId: $draftId, transactionId: $transactionId, ownerId: $ownerId, concept: $concept)
                }
            "#;
            let vars = async_graphql::value!({
                "draftId": "d1",
                "transactionId": "t1",
                "ownerId": kit_id,
                "concept": { "name": "beta-concept" }
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createConcept errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { concepts { edges { node { name } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["wip"]["theKit"]["concepts"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
            assert!(names.iter().any(|n| n == "beta-concept"), "concepts missing new name: {:?}", names);
        });
    }

    #[test]
    fn create_quality_on_kit_graphql_roundtrip() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let kit_id = schema.execute("{ wip { theKit { id } } }").await.data.into_json().unwrap()["wip"]["theKit"]["id"].as_str().expect("kit id").to_string();

            const M: &str = r#"
                mutation($draftId: ID!, $transactionId: ID!, $ownerId: ID!, $quality: QualityInput!) {
                    createQuality(draftId: $draftId, transactionId: $transactionId, ownerId: $ownerId, quality: $quality)
                }
            "#;
            let vars = async_graphql::value!({
                "draftId": "d1",
                "transactionId": "t1",
                "ownerId": kit_id,
                "quality": { "key": "q1", "value": "v1" }
            });
            let res = schema.execute(Request::new(M).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "createQuality errors: {:?}", res.errors);

            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ wip { theKit { qualities { edges { node { key value } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let keys: Vec<String> = data["wip"]["theKit"]["qualities"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["key"].as_str().map(String::from)).collect();
            assert!(keys.iter().any(|k| k == "q1"), "qualities missing new key: {:?}", keys);
        });
    }

    #[test]
    fn graph_op_registry_row_count() {
        assert_eq!(crate::op::GRAPH_OP_REGISTRY_ROWS, 98);
    }

    #[test]
    fn create_fixed_piece_end_to_end() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let res = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1")))).await;
            assert!(res.errors.is_empty(), "mutation errors: {:?}", res.errors);

            // The wip child applies asynchronously; wait briefly for the event loop.
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = relay_wip_designs_have_piece();
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let edges = data["wip"]["theKit"]["designs"]["edges"].as_array().expect("design edges");
            let any_piece = edges.iter().any(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.iter().any(|_| true)).unwrap_or(false));
            assert!(any_piece, "expected at least one piece in wip; got: {}", serde_json::to_string_pretty(&data).unwrap());
        });
    }

    #[test]
    fn wip_and_authoritative_are_isolated() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let _ = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1")))).await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = relay_auth_designs_piece_ids();
            let res = schema.execute(q).await;
            let data = res.data.into_json().unwrap();
            let edges = data["authoritative"]["theKit"]["designs"]["edges"].as_array().expect("auth design edges");
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

            // Insert two pieces directly via the wip graph (no GraphQL plumbing).
            let position = crate::geom::Position::default();
            let blueprint_id = crate::id::Id::new().await;
            let p1 = rt.wip_graph.apply_create_fixed_piece(crate::id::Id::from("d1"), crate::id::Id::from("t1"), crate::id::Id::from("des1"), blueprint_id.clone(), position, None, None).await.expect("insert piece 1").0;
            let _p2 = rt.wip_graph.apply_create_fixed_piece(crate::id::Id::from("d1"), crate::id::Id::from("t1"), crate::id::Id::from("des1"), blueprint_id, position, None, None).await.expect("insert piece 2").0;

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
        let Some(edges) = data["wip"]["theKit"]["designs"]["edges"].as_array() else {
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
            let q = relay_wip_designs_have_piece();

            let before = schema.execute(q).await;
            let before_data = before.data.into_json().unwrap();
            let before_pieces = relay_piece_count_wip(&before_data);

            let _ = schema.execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1")))).await;
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
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.ops")).expect("parse ops");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let draft_id = crate::id::Id::from(ops_json["draftId"].as_str().expect("draftId"));
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            for rec in ops_json["ops"].as_array().expect("ops") {
                let kind = rec["kind"].as_str().expect("op kind");
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
                    other => panic!("unsupported golden op kind: {other}"),
                }
            }

            let inv = &exp["invariants"];
            let ds = g.the_kit.designs.read().await;
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

            let fp = stable_projection_fingerprint(&g.the_kit).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint in kit-store.golden.expected.semio.json");
            assert_eq!(fp, exp_fp, "projectionFingerprint");
        });
    }

    /// 🪡 `kit_graph_engine::apply_semantic_op_json` must replay the same golden ops as manual apply.
    #[test]
    fn kit_store_golden_ops_via_semantic_op_json_match_fingerprint() {
        block_on(async {
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.ops")).expect("parse ops");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let draft_id = crate::id::Id::from(ops_json["draftId"].as_str().expect("draftId"));
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            for rec in ops_json["ops"].as_array().expect("ops") {
                let kind = rec["kind"].as_str().expect("op kind");
                let payload = serde_json::to_string(rec.get("input").expect("input")).expect("payload json");
                let applied = crate::kit_graph_engine::apply_semantic_op_json(&g, &draft_id, &tx_id, kind, &payload).await.expect("apply_semantic_op_json");
                assert!(applied.created_piece.is_some(), "expected piece for {kind}");
                assert!(applied.diff.summary.as_ref().map(|s| !s.is_empty()).unwrap_or(false), "diff summary");
            }

            let fp = stable_projection_fingerprint(&g.the_kit).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "projectionFingerprint via semantic json apply");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dev_json_backbone_persisted_ops_replay_matches_us001_projection_fingerprint() {
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("dev-kit.json");

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read ops")).expect("parse golden ops");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored ops");
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);
            let bundle = crate::kit_backbone::KitStoreBundleFile::from_stored_semantic_ops(&stored);
            std::fs::write(&path, serde_json::to_string_pretty(&bundle).expect("serialize kit-store bundle")).expect("write kit-store bundle");

            let g = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::DevJson, "wip", &g).await.expect("dev json mount+replay");

            let fp = stable_projection_fingerprint(&g.the_kit).await;
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

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read ops")).expect("parse golden ops");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored ops");

            let g_bootstrap = crate::vcs::Graph::new().await;
            let _bones = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::LocalDotSemio, "wip", &g_bootstrap).await.expect("bootstrap .semio layout");

            let db_path = proj_canon.join(".semio").join("wip.db");
            let conn = rusqlite::Connection::open(&db_path).expect("open wip.db");
            for op in &stored {
                let input_json = serde_json::to_string(&op.input).expect("input json");
                conn.execute("INSERT INTO semantic_op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![op.draft_id, op.transaction_id, op.kind, input_json]).expect("insert semantic op row");
            }
            drop(conn);

            let g2 = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::LocalDotSemio, "wip", &g2).await.expect("replay wip.db");

            let fp = stable_projection_fingerprint(&g2.the_kit).await;
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
            for inner in ["id", "hash", "authors", "root", "checkpoints", "alternatives", "drafts"] {
                assert!(v[graph_key].get(inner).is_some(), "graph `{graph_key}` missing `{inner}`");
            }
            assert!(v[graph_key]["root"].get("types").is_some(), "graph `{graph_key}.root` missing `types`");
            assert!(v[graph_key]["root"].get("designs").is_some(), "graph `{graph_key}.root` missing `designs`");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_initialize_with_active_draft_seeds_root_checkpoint_and_draft() {
        // 🌱 Bundle bootstrap matches sketchpad "create dev kit (json)": one root, one checkpoint, one draft on that checkpoint.
        let bundle = crate::kit_backbone::KitStoreBundleFile::initialize_with_active_draft("kit-id-1", "draft-1", "ckpt-1");
        assert_eq!(bundle.schema, crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
        assert_eq!(bundle.wip.id, "kit-id-1");
        assert_eq!(bundle.authoritative.id, "kit-id-1");
        assert_eq!(bundle.stage.id, "kit-id-1");
        assert_eq!(bundle.wip.checkpoints.items.len(), 1, "first checkpoint anchored on root");
        assert_eq!(bundle.wip.checkpoints.items[0]["id"], "ckpt-1");
        assert_eq!(bundle.wip.checkpoints.items[0]["message"], "init");
        assert_eq!(bundle.wip.drafts.items.len(), 1, "one active draft on the seed checkpoint");
        let draft = &bundle.wip.drafts.items[0];
        assert_eq!(draft.id, "draft-1");
        assert_eq!(draft.checkpoint.as_ref().expect("draft anchored on checkpoint").id, "ckpt-1");
        assert!(draft.transactions.items.is_empty(), "no transactions opened yet");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_open_commit_abort_transaction_lifecycle() {
        // 🟢✅⛔ Full transaction lifecycle on an initialized bundle (no rs ↔ js round-trip yet, just data-shape level).
        let mut bundle = crate::kit_backbone::KitStoreBundleFile::initialize_with_active_draft("kit-1", "draft-1", "ckpt-1");

        // Open: draft already exists — adds a fresh transaction.
        let tx_a = bundle.open_transaction("draft-1");
        assert!(!tx_a.is_empty(), "open_transaction returns a non-empty id");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items.len(), 1);
        assert_eq!(bundle.wip.drafts.items[0].transactions.items[0].id, tx_a);

        // Open second transaction on same draft.
        let tx_b = bundle.open_transaction("draft-1");
        assert_ne!(tx_a, tx_b, "transaction ids are unique");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items.len(), 2);

        // Append a forward step into tx_b.
        bundle.append_wip_step("draft-1", &tx_b, "kit.renamedKit", serde_json::json!({"name": "Hello"}));
        assert_eq!(bundle.wip.drafts.items[0].transactions.items[1].forwards.items.len(), 1);

        // Commit tx_a: stays in draft (no checkpoint move yet).
        bundle.commit_transaction("draft-1", &tx_a).expect("commit known tx");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items.len(), 2, "commit keeps tx in draft pre-checkpointing");

        // Abort tx_b: removed from draft.
        bundle.abort_transaction("draft-1", &tx_b).expect("abort known tx");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items.len(), 1);
        assert_eq!(bundle.wip.drafts.items[0].transactions.items[0].id, tx_a);

        // Commit / abort errors are surfaced for unknown ids / drafts.
        assert!(bundle.commit_transaction("draft-1", "tx-missing").is_err());
        assert!(bundle.abort_transaction("draft-1", "tx-missing").is_err());
        assert!(bundle.commit_transaction("draft-missing", &tx_a).is_err());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dev_json_backbone_round_trip_persists_metabolism_shape_on_disk() {
        // 🔁 Mount (no file) → append op via attached backbone → re-read on-disk JSON → confirm metabolism top-level + drafts path.
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("dev-kit.json");
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);

            let g = crate::vcs::Graph::new().await;
            let mut bone = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::DevJson, "wip", &g).await.expect("mount empty bundle");
            let draft_id = crate::id::Id::from("draft-rs-1");
            let tx_id = crate::id::Id::from("tx-rs-1");
            bone.append_semantic_op(&draft_id, &tx_id, "kit.design.piece.createdFixedPiece", &serde_json::json!({"designId": "d-1", "blueprintId": "b-1"}))
                .expect("append op");

            let raw = std::fs::read_to_string(&path).expect("read on-disk bundle");
            let v: serde_json::Value = serde_json::from_str(&raw).expect("parse on-disk bundle");
            for k in ["schema", "wip", "authoritative", "stage", "conflicts", "blobs"] {
                assert!(v.get(k).is_some(), "on-disk bundle missing `{k}` after append");
            }
            assert_eq!(v["schema"], crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
            let drafts = v["wip"]["drafts"]["items"].as_array().expect("wip drafts items array");
            assert_eq!(drafts.len(), 1, "single draft on disk");
            assert_eq!(drafts[0]["id"], "draft-rs-1");
            let txs = drafts[0]["transactions"]["items"].as_array().expect("tx items");
            assert_eq!(txs.len(), 1, "single transaction on disk");
            assert_eq!(txs[0]["id"], "tx-rs-1");
            let fwd = txs[0]["forwards"]["items"].as_array().expect("forwards items");
            assert_eq!(fwd.len(), 1, "single forward step on disk");
            assert_eq!(fwd[0]["kind"], "kit.design.piece.createdFixedPiece");
            assert_eq!(fwd[0]["input"]["designId"], "d-1");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_append_wip_step_creates_draft_and_transaction_paths() {
        // ➕ Appending to a fresh template materialises wip.drafts[0].transactions[0].forwards[0] without touching authoritative/stage.
        let mut bundle = crate::kit_backbone::KitStoreBundleFile::template();
        bundle.append_wip_step("draft-x", "tx-y", "kit.design.piece.createdFixedPiece", serde_json::json!({"hello": "world"}));
        assert_eq!(bundle.wip.drafts.items.len(), 1, "wip draft created");
        let draft = &bundle.wip.drafts.items[0];
        assert_eq!(draft.id, "draft-x");
        assert_eq!(draft.transactions.items.len(), 1, "transaction created under draft");
        let tx = &draft.transactions.items[0];
        assert_eq!(tx.id, "tx-y");
        assert_eq!(tx.forwards.items.len(), 1, "single forward step appended");
        let step = &tx.forwards.items[0];
        assert_eq!(step.kind, "kit.design.piece.createdFixedPiece");
        assert_eq!(step.input["hello"], "world");
        assert!(bundle.authoritative.drafts.items.is_empty(), "authoritative untouched");
        assert!(bundle.stage.drafts.items.is_empty(), "stage untouched");

        // Appending another step into the same draft+tx grows the same transaction.
        bundle.append_wip_step("draft-x", "tx-y", "kit.design.piece.deletedFixedPieces", serde_json::json!({"pieceIds": []}));
        assert_eq!(bundle.wip.drafts.items.len(), 1, "no extra draft");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items.len(), 1, "no extra transaction");
        assert_eq!(bundle.wip.drafts.items[0].transactions.items[0].forwards.items.len(), 2, "two forward steps");

        // Flatten replays everything in order from wip drafts.
        let flat = bundle.wip_semantic_ops();
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].draft_id, "draft-x");
        assert_eq!(flat[0].transaction_id, "tx-y");
        assert_eq!(flat[0].kind, "kit.design.piece.createdFixedPiece");
        assert_eq!(flat[1].kind, "kit.design.piece.deletedFixedPieces");
    }
}

//#endregion 🧪 tests
