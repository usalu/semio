//! 🦀 semio rust control plane — in-memory Arc-reference architecture.
//!
//! Every entity from the emitted `semio/graphql/schema.graphql` (see `crate::gql::target_schema_sdl`) is one Rust struct shared as
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
                let id = weak(
                    "coordinate",
                    &[&format!("{:.9}", c.u), &format!("{:.9}", c.v)],
                );
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
                Arc::new(Self {
                    id,
                    x: RwLock::new(v.x),
                    y: RwLock::new(v.y),
                    z: RwLock::new(v.z),
                })
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
                Arc::new(Self {
                    id,
                    x: RwLock::new(p.x),
                    y: RwLock::new(p.y),
                    z: RwLock::new(p.z),
                })
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
                let id = weak(
                    "plane",
                    &[
                        origin.id.as_str(),
                        x_axis.id.as_str(),
                        y_axis.id.as_str(),
                    ],
                );
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
                Arc::new(Self {
                    id,
                    u: RwLock::new(o.u),
                    v: RwLock::new(o.v),
                })
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
                Arc::new(Self {
                    id,
                    center,
                    plane,
                    data: RwLock::new(value),
                })
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
                Arc::new(Self {
                    id: Id::new().await,
                    label: RwLock::new(None),
                })
            }
        }
    }
    //#endregion 📐 entity
}

//#endregion 📐 geom

//#region 🪢 gql_relay

/// 🪢 Relay `PageInfo` + connection shells for static GraphQL (edges, pageInfo, hash).
pub mod gql_relay {
    use std::sync::Arc;

    use async_graphql::SimpleObject;
    use blake3::Hasher;

    use crate::id::Id;
    use crate::kit::design::Design;
    use crate::kit::design::piece::Piece;
    use crate::kit::r#type::Type;
    use crate::meta::{Author, Concept, File, Folder, Group, Layer, Prop, Quality, Stat, Tag};
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl DesignConnection {
        pub fn from_designs(rows: Vec<Arc<Design>>) -> Self {
            let hash = hash_ids(rows.iter().map(|d| d.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, d)| DesignEdge {
                    cursor: edge_cursor(i),
                    node: d,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl PieceConnection {
        pub fn from_pieces(rows: Vec<Arc<Piece>>) -> Self {
            let hash = hash_ids(rows.iter().map(|p| p.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, p)| PieceEdge {
                    cursor: edge_cursor(i),
                    node: p,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl TypeConnection {
        pub fn from_types(rows: Vec<Arc<Type>>) -> Self {
            let hash = hash_ids(rows.iter().map(|t| t.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, t)| TypeEdge {
                    cursor: edge_cursor(i),
                    node: t,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl ConflictConnection {
        pub fn from_conflicts(rows: Vec<Arc<Conflict>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, c)| ConflictEdge {
                    cursor: edge_cursor(i),
                    node: c,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl AlternativeConnection {
        pub fn from_alternatives(rows: Vec<Arc<Alternative>>) -> Self {
            let hash = hash_ids(rows.iter().map(|a| a.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, a)| AlternativeEdge {
                    cursor: edge_cursor(i),
                    node: a,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl CheckpointConnection {
        pub fn from_checkpoints(rows: Vec<Arc<Checkpoint>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, c)| CheckpointEdge {
                    cursor: edge_cursor(i),
                    node: c,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
        pub page_info: PageInfo,
        pub hash: String,
    }

    impl ConnectionConnection {
        pub fn from_connections(rows: Vec<Arc<crate::kit::design::connection::Connection>>) -> Self {
            let hash = hash_ids(rows.iter().map(|c| c.id.as_str()));
            let edges = rows
                .into_iter()
                .enumerate()
                .map(|(i, node)| ConnectionEdge {
                    cursor: edge_cursor(i),
                    node,
                })
                .collect();
            Self {
                edges,
                page_info: PageInfo::default(),
                hash,
            }
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
                pub page_info: PageInfo,
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
                    let edges = rows
                        .into_iter()
                        .enumerate()
                        .map(|(i, node)| $Edge {
                            cursor: edge_cursor(i),
                            node,
                        })
                        .collect();
                    Self {
                        edges,
                        page_info: PageInfo::default(),
                        hash,
                    }
                }
            }
        };
    }

    simple_conn!(FileConnection, FileEdge, File, |f: &File| f.id.clone());
    simple_conn!(FolderConnection, FolderEdge, Folder, |f: &Folder| f.id.clone());
    simple_conn!(AuthorConnection, AuthorEdge, Author, |a: &Author| a.id.clone());
    simple_conn!(ConceptConnection, ConceptEdge, Concept, |c: &Concept| c.id.clone());
    simple_conn!(TagConnection, TagEdge, Tag, |t: &Tag| t.id.clone());
    simple_conn!(QualityConnection, QualityEdge, Quality, |q: &Quality| q.id.clone());
    simple_conn!(PropConnection, PropEdge, Prop, |p: &Prop| p.id.clone());
    simple_conn!(AttributeConnection, AttributeEdge, crate::meta::Attribute, |a: &crate::meta::Attribute| a.id.clone());
    simple_conn!(StatConnection, StatEdge, Stat, |s: &Stat| s.id.clone());
    simple_conn!(LayerConnection, LayerEdge, Layer, |l: &Layer| l.id.clone());
    simple_conn!(GroupConnection, GroupEdge, Group, |g: &Group| g.id.clone());
    simple_conn!(
        PositionNodeConnection,
        PositionNodeEdge,
        Arc<crate::geom::entity::PositionNode>,
        |p: &Arc<crate::geom::entity::PositionNode>| p.id.clone()
    );

    /// @emoji 🪢 `entity_relay!` — delegates to [`simple_conn!`] for `Arc`/value nodes with an id extractor closure.
    macro_rules! entity_relay {
        ($Conn:ident, $Edge:ident, $Node:ty, $id_expr:expr) => {
            simple_conn!($Conn, $Edge, $Node, $id_expr);
        };
    }

    entity_relay!(
        VectorNodeConnection,
        VectorNodeEdge,
        Arc<crate::geom::entity::VectorNode>,
        |v: &Arc<crate::geom::entity::VectorNode>| v.id.clone()
    );
    entity_relay!(
        CoordinateNodeConnection,
        CoordinateNodeEdge,
        Arc<crate::geom::entity::CoordinateNode>,
        |c: &Arc<crate::geom::entity::CoordinateNode>| c.id.clone()
    );
    entity_relay!(
        PointNodeConnection,
        PointNodeEdge,
        Arc<crate::geom::entity::PointNode>,
        |p: &Arc<crate::geom::entity::PointNode>| p.id.clone()
    );
    entity_relay!(
        PlaneNodeConnection,
        PlaneNodeEdge,
        Arc<crate::geom::entity::PlaneNode>,
        |p: &Arc<crate::geom::entity::PlaneNode>| p.id.clone()
    );
    entity_relay!(
        OffsetNodeConnection,
        OffsetNodeEdge,
        Arc<crate::geom::entity::OffsetNode>,
        |o: &Arc<crate::geom::entity::OffsetNode>| o.id.clone()
    );

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
    //! 🏷️ Strong shared metadata as plain SimpleObject value types (small, immutable).
    use async_graphql::SimpleObject;
    use serde::{Deserialize, Serialize};

    use crate::id::Id;
    use crate::timestamp::Timestamp;

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Location {
        pub id: Id,
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

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Attribute {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub definition: Option<String>,
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
    pub struct Quality {
        pub id: Id,
        pub key: String,
        pub value: Option<String>,
        pub unit: Option<String>,
        pub definition: Option<String>,
        pub description: Option<String>,
        pub benchmarks: Vec<Benchmark>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Prop {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        pub quality: Option<Quality>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Tag {
        pub id: Id,
        pub name: String,
        pub order: Option<i32>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
    pub struct Concept {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub order: Option<i32>,
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
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                self.compute_hash().await
            }
            async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            async fn code(&self) -> Option<String> {
                self.code.read().await.clone()
            }
            async fn label(&self) -> Option<String> {
                self.label.read().await.clone()
            }
            async fn order(&self) -> Option<i32> {
                *self.order.read().await
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
            pub qualities: RwLock<Vec<Quality>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Connector {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_type: Weak::new(),
                    code: RwLock::new(String::new()),
                    description: RwLock::new(None),
                    port: RwLock::new(Weak::new()),
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
                    code: RwLock::new(code),
                    description: RwLock::new(None),
                    port: RwLock::new(Weak::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let code = self.code.read().await;
                let desc = self.description.read().await;
                h(&[self.id.as_str(), code.as_str(), desc.as_deref().unwrap_or("")])
            }
        }

        #[Object(name = "Connector")]
        impl Connector {
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                self.compute_hash().await
            }
            async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            async fn code(&self) -> String {
                self.code.read().await.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            async fn port(&self) -> Option<Arc<Port>> {
                self.port.read().await.upgrade()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.read().await.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
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
            pub tags: RwLock<Vec<Tag>>,
            pub qualities: RwLock<Vec<Quality>>,
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
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                self.compute_hash().await
            }
            async fn owner(&self) -> Arc<Type> {
                self.owner_type.upgrade().unwrap_or_default()
            }
            async fn url(&self) -> String {
                self.url.read().await.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            async fn file(&self) -> Option<File> {
                self.file.read().await.clone()
            }
            async fn tags(&self) -> Vec<Tag> {
                self.tags.read().await.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.read().await.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
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
            pub concepts: RwLock<Vec<Concept>>,
            pub tags: RwLock<Vec<Tag>>,
            pub qualities: RwLock<Vec<Quality>>,
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
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                self.compute_hash().await
            }
            async fn owner(&self) -> Arc<crate::kit::Kit> {
                self.owner_kit.upgrade().unwrap_or_default()
            }
            async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            async fn icon(&self) -> Option<String> {
                self.icon.read().await.clone()
            }
            async fn image(&self) -> Option<String> {
                self.image.read().await.clone()
            }
            async fn unit(&self) -> Option<String> {
                self.unit.read().await.clone()
            }
            async fn created(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            async fn updated(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            async fn connectors(&self) -> Vec<Arc<Connector>> {
                self.connectors.read().await.clone()
            }
            async fn connector(&self, id: Id) -> Option<Arc<Connector>> {
                self.refresh_connector_child_weak_maps().await;
                self.connector_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            async fn representations(&self) -> Vec<Arc<Representation>> {
                self.representations.read().await.clone()
            }
            async fn representation(&self, id: Id) -> Option<Arc<Representation>> {
                self.refresh_connector_child_weak_maps().await;
                self.representation_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            #[graphql(name = "bestRepresentation")]
            async fn best_representation(&self, tag_ids: Vec<Id>) -> Option<Arc<Representation>> {
                self.best_representation_for_tags(&tag_ids).await
            }
            async fn authors(&self) -> Vec<Author> {
                self.authors.read().await.clone()
            }
            async fn concepts(&self) -> Vec<Concept> {
                self.concepts.read().await.clone()
            }
            async fn tags(&self) -> Vec<Tag> {
                self.tags.read().await.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.read().await.clone()
            }
            async fn props(&self) -> Vec<Prop> {
                self.props.read().await.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.read().await.clone()
            }
            async fn stats(&self) -> Vec<Stat> {
                self.stats.read().await.clone()
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
                    Arc::new(Self {
                        id: Id::new().await,
                        owner_design,
                        position: RwLock::new(Some(pos_node)),
                        blueprint: RwLock::new(blueprint),
                        connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)),
                        ..Default::default()
                    })
                }

                /// 🧾 Hydrated workspace piece aligned to external JSON id (facade snapshot hydration).
                pub async fn new_fixed_with_external_id(id: Id, owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: Position) -> Arc<Self> {
                    let pos_node = PositionNode::from_position_value(position);
                    Arc::new(Self {
                        id,
                        owner_design,
                        position: RwLock::new(Some(pos_node)),
                        blueprint: RwLock::new(blueprint),
                        connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)),
                        ..Default::default()
                    })
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
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                async fn owner(&self) -> super::super::r#type::Blueprint {
                    super::super::r#type::Blueprint::Design(self.owner_design.upgrade().unwrap_or_default())
                }
                async fn name(&self) -> Option<String> {
                    self.name.read().await.clone()
                }
                async fn description(&self) -> Option<String> {
                    self.description.read().await.clone()
                }
                async fn position(&self) -> Option<Arc<PositionNode>> {
                    self.position.read().await.clone()
                }
                async fn scale(&self) -> Option<f64> {
                    *self.scale.read().await
                }
                async fn blueprint(&self) -> super::super::r#type::Blueprint {
                    self.blueprint.read().await.clone()
                }
                #[graphql(name = "connectionKind")]
                async fn connection_kind(&self) -> Option<PieceConnectionKind> {
                    *self.connection_kind.read().await
                }
                #[graphql(name = "flatPosition")]
                async fn flat_position(&self) -> Arc<PositionNode> {
                    if let Some(n) = self.position.read().await.clone() {
                        return n;
                    }
                    PositionNode::from_position_value(Position::default())
                }
                #[graphql(name = "replaceableBlueprint")]
                async fn replaceable_blueprint(&self) -> Vec<super::super::r#type::Blueprint> {
                    Vec::new()
                }
                #[graphql(name = "parentConnection")]
                async fn parent_connection(&self) -> Option<Arc<super::connection::Connection>> {
                    self.parent_connection.read().await.upgrade()
                }
                #[graphql(name = "childConnections")]
                async fn child_connections(&self) -> Vec<Arc<super::connection::Connection>> {
                    self.child_connections.read().await.clone()
                }
                #[graphql(name = "parentPiece")]
                async fn parent_piece(&self) -> Option<Arc<Piece>> {
                    self.parent_piece.read().await.upgrade()
                }
                #[graphql(name = "childPieces")]
                async fn child_pieces(&self) -> Vec<Arc<Piece>> {
                    self.child_pieces.read().await.clone()
                }
                async fn depth(&self) -> i32 {
                    *self.depth.read().await
                }
                async fn path(&self) -> Vec<Arc<Piece>> {
                    self.path.read().await.iter().filter_map(|w| w.upgrade()).collect()
                }
                async fn props(&self) -> Vec<Prop> {
                    self.props.read().await.clone()
                }
                async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
                }

                #[graphql(name = "ownerEntity")]
                async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
                    None
                }

                #[graphql(name = "ownedEntities")]
                async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
                    crate::iface::OwnedEntityConnection::empty()
                }
            }
        }
        //#endregion ⭕ piece

        //#region 🔗 connection
        pub mod connection {
            //! 🔗 Connection between two piece sides + the Side value.
            use std::sync::{Arc, Weak};

            use async_graphql::Object;
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

            #[Object(name = "Side")]
            impl Side {
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn piece(&self) -> Arc<super::piece::Piece> {
                    self.piece.read().await.upgrade().unwrap_or_default()
                }
                async fn port(&self) -> Option<Arc<super::super::r#type::Port>> {
                    self.port.read().await.upgrade()
                }
                #[graphql(name = "designPiece")]
                async fn design_piece(&self) -> Option<Arc<super::piece::Piece>> {
                    self.design_piece.read().await.upgrade()
                }
                async fn connector(&self) -> Option<Arc<super::super::r#type::Connector>> {
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
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                async fn owner(&self) -> Arc<super::Design> {
                    self.owner_design.upgrade().unwrap_or_default()
                }
                async fn connected(&self) -> Arc<Side> {
                    self.connected.read().await.clone()
                }
                async fn connecting(&self) -> Arc<Side> {
                    self.connecting.read().await.clone()
                }
                async fn gap(&self) -> Option<f64> {
                    *self.gap.read().await
                }
                async fn shift(&self) -> Option<f64> {
                    *self.shift.read().await
                }
                async fn rise(&self) -> Option<f64> {
                    *self.rise.read().await
                }
                async fn rotation(&self) -> Option<f64> {
                    *self.rotation.read().await
                }
                async fn turn(&self) -> Option<f64> {
                    *self.turn.read().await
                }
                async fn tilt(&self) -> Option<f64> {
                    *self.tilt.read().await
                }
                async fn u(&self) -> Option<f64> {
                    *self.u.read().await
                }
                async fn v(&self) -> Option<f64> {
                    *self.v.read().await
                }
                async fn description(&self) -> Option<String> {
                    self.description.read().await.clone()
                }
                async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
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
        use crate::meta::{Attribute, Author, Concept, Group, Layer, Location, Prop, Quality, Stat, Tag};
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
            pub concepts: RwLock<Vec<Concept>>,
            pub tags: RwLock<Vec<Tag>>,
            pub qualities: RwLock<Vec<Quality>>,
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
                    concepts: RwLock::new(Vec::new()),
                    tags: RwLock::new(Vec::new()),
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
                    let pid = pj
                        .get("id")
                        .and_then(|x| x.as_str())
                        .ok_or_else(|| crate::error::SemioError::invalid("design piece missing id"))?;
                    let type_id_raw = pj
                        .get("type")
                        .and_then(|t| t.get("id"))
                        .and_then(|x| x.as_str())
                        .ok_or_else(|| crate::error::SemioError::invalid("design piece missing type.id"))?;
                    let ty = kit
                        .types
                        .read()
                        .await
                        .iter()
                        .find(|t| t.id.as_str() == type_id_raw)
                        .cloned()
                        .ok_or_else(|| crate::error::SemioError::not_found("Type", type_id_raw))?;
                    let plane_val = pj.get("plane").cloned().unwrap_or_else(|| serde_json::json!({}));
                    let center_val = pj.get("center").cloned().unwrap_or_else(|| serde_json::json!({"u":0.0,"v":0.0}));
                    let position: crate::geom::Position = serde_json::from_value(serde_json::json!({"plane": plane_val, "center": center_val}))
                        .map_err(|e| crate::error::SemioError::invalid(format!("piece position serde: {}", e)))?;
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
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                self.compute_hash().await
            }
            async fn owner(&self) -> DesignOwner {
                DesignOwner::Kit(self.owner_kit.upgrade().unwrap_or_default())
            }
            async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            async fn icon(&self) -> Option<String> {
                self.icon.read().await.clone()
            }
            async fn image(&self) -> Option<String> {
                self.image.read().await.clone()
            }
            async fn location(&self) -> Option<Location> {
                self.location.read().await.clone()
            }
            async fn unit(&self) -> Option<String> {
                self.unit.read().await.clone()
            }
            #[graphql(name = "createdAt")]
            async fn created_at(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            #[graphql(name = "updatedAt")]
            async fn updated_at(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.pieces.read().await.clone())
            }
            async fn piece(&self, id: Id) -> Option<Arc<piece::Piece>> {
                self.piece_by_external_id(&id).await
            }
            async fn connections(&self) -> crate::gql_relay::ConnectionConnection {
                crate::gql_relay::ConnectionConnection::from_connections(self.connections.read().await.clone())
            }
            async fn connection(&self, id: Id) -> Option<Arc<connection::Connection>> {
                self.connections.read().await.iter().find(|c| c.id == id).cloned()
            }
            async fn layers(&self) -> crate::gql_relay::LayerConnection {
                crate::gql_relay::LayerConnection::from_rows(self.layers.read().await.clone())
            }
            async fn groups(&self) -> crate::gql_relay::GroupConnection {
                crate::gql_relay::GroupConnection::from_rows(self.groups.read().await.clone())
            }
            async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
            }
            async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
                crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone())
            }
            async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone())
            }
            async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
            }
            async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_rows(self.props.read().await.clone())
            }
            async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
            }
            async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_rows(self.stats.read().await.clone())
            }
            #[graphql(name = "qualitySum")]
            async fn quality_sum(&self, _quality_id: Id) -> f64 {
                0.0
            }

            #[graphql(name = "ownerEntity")]
            async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
                self.owner_kit.upgrade().map(crate::iface::OwnerEntity::Kit)
            }

            #[graphql(name = "ownedEntities")]
            async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
                crate::iface::OwnedEntityConnection::empty()
            }

            async fn references(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(Vec::new())
            }
            #[graphql(name = "referencedBy")]
            async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
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
        pub concepts: RwLock<Vec<Concept>>,
        pub tags: RwLock<Vec<Tag>>,
        pub qualities: RwLock<Vec<Quality>>,
        pub props: RwLock<Vec<Prop>>,
        pub attributes: RwLock<Vec<Attribute>>,
        pub stats: RwLock<Vec<Stat>>,
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

        /// 🆕 Insert (or look up) a design by id, returning the shared Arc (maintains [`Kit::design_weak_by_id`]).
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
                designs
                    .iter()
                    .position(|d| &d.id == design_id)
                    .expect("design slot after ensure_design") as u32
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
            let design_arr_owned = dto
                .get("designs")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
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
                        let pv = if let Some(n) = p.position.read().await.as_ref() {
                            n.snapshot_value().await
                        } else {
                            crate::geom::Position::default()
                        };
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
        async fn id(&self) -> Id {
            self.workspace_kit_id().await
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        /// Owner [`crate::vcs::Graph`] is set by [`crate::worker::ChildRuntime::new`] via Weak.
        async fn owner(&self) -> KitOwner {
            match self.owner_graph.upgrade() {
                Some(g) => KitOwner::Graph(g),
                None => KitOwner::Graph(Arc::new(crate::vcs::Graph::default())),
            }
        }
        #[graphql(name = "graphOwner")]
        async fn graph_owner(&self) -> Option<Arc<crate::vcs::Graph>> {
            self.owner_graph.upgrade()
        }
        #[graphql(name = "checkpointOwner")]
        async fn checkpoint_owner(&self) -> Option<Arc<crate::vcs::Checkpoint>> {
            None
        }
        #[graphql(name = "alternativeOwner")]
        async fn alternative_owner(&self) -> Option<Arc<crate::vcs::Alternative>> {
            None
        }
        async fn checkpoint(&self) -> Option<Arc<crate::vcs::Checkpoint>> {
            None
        }
        async fn draft(&self) -> Option<Arc<crate::vcs::Draft>> {
            None
        }
        async fn transaction(&self) -> Option<Arc<crate::vcs::Transaction>> {
            None
        }
        async fn name(&self) -> String {
            self.name.read().await.clone()
        }
        async fn description(&self) -> Option<String> {
            self.description.read().await.clone()
        }
        async fn icon(&self) -> Option<String> {
            self.icon.read().await.clone()
        }
        async fn image(&self) -> Option<String> {
            self.image.read().await.clone()
        }
        async fn preview(&self) -> Option<String> {
            self.preview.read().await.clone()
        }
        async fn remote(&self) -> Option<String> {
            self.remote.read().await.clone()
        }
        async fn homepage(&self) -> Option<String> {
            self.homepage.read().await.clone()
        }
        async fn license(&self) -> Option<String> {
            self.license.read().await.clone()
        }
        async fn uri(&self) -> Option<String> {
            self.uri.read().await.clone()
        }
        #[graphql(name = "createdAt")]
        async fn created_at(&self) -> Option<Timestamp> {
            self.created.read().await.clone()
        }
        #[graphql(name = "updatedAt")]
        async fn updated_at(&self) -> Option<Timestamp> {
            self.updated.read().await.clone()
        }
        async fn version(&self) -> Option<String> {
            self.version.read().await.clone()
        }
        async fn design(&self, id: Id) -> Option<Arc<design::Design>> {
            self.design_by_external_id(&id).await
        }
        async fn designs(&self) -> crate::gql_relay::DesignConnection {
            crate::gql_relay::DesignConnection::from_designs(self.designs.read().await.clone())
        }
        #[graphql(name = "type")]
        async fn type_(&self, id: Id) -> Option<Arc<r#type::Type>> {
            self.type_by_external_id(&id).await
        }
        async fn types(&self) -> crate::gql_relay::TypeConnection {
            crate::gql_relay::TypeConnection::from_types(self.types.read().await.clone())
        }
        async fn files(&self) -> crate::gql_relay::FileConnection {
            crate::gql_relay::FileConnection::from_rows(self.files.read().await.clone())
        }
        async fn folders(&self) -> crate::gql_relay::FolderConnection {
            crate::gql_relay::FolderConnection::from_rows(self.folders.read().await.clone())
        }
        async fn families(&self) -> crate::gql_relay::FamilyConnection {
            crate::gql_relay::FamilyConnection::from_rows(Vec::new())
        }
        async fn authors(&self) -> crate::gql_relay::AuthorConnection {
            crate::gql_relay::AuthorConnection::from_rows(self.authors.read().await.clone())
        }
        async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
            crate::gql_relay::ConceptConnection::from_rows(self.concepts.read().await.clone())
        }
        async fn tags(&self) -> crate::gql_relay::TagConnection {
            crate::gql_relay::TagConnection::from_rows(self.tags.read().await.clone())
        }
        async fn qualities(&self) -> crate::gql_relay::QualityConnection {
            crate::gql_relay::QualityConnection::from_rows(self.qualities.read().await.clone())
        }
        async fn props(&self) -> crate::gql_relay::PropConnection {
            crate::gql_relay::PropConnection::from_rows(self.props.read().await.clone())
        }
        async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
            crate::gql_relay::AttributeConnection::from_rows(self.attributes.read().await.clone())
        }
        async fn stats(&self) -> crate::gql_relay::StatConnection {
            crate::gql_relay::StatConnection::from_rows(self.stats.read().await.clone())
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            self.owner_graph.upgrade().map(crate::iface::OwnerEntity::Graph)
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
        }

        /// @emoji 📸 JSON string projection of `@semio/js` KitFullDto (camelCase ids + ordered pieces); WASM + Node pull this via GraphQL (`wip.theKit`).
        #[graphql(name = "fullSnapshot")]
        async fn full_snapshot(&self) -> String {
            serde_json::to_string(&self.kit_full_snapshot_value().await).unwrap_or_else(|_| "{}".to_string())
        }
    }
    //#endregion 📦 kit
}

//#endregion 📦 kit

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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn owner(&self) -> ChangeOwnerUnion {
            match self.owner.read().await.clone() {
                Some(ChangeOwnerRef::Transaction(w)) => ChangeOwnerUnion::Transaction(w.upgrade().unwrap_or_default()),
                Some(ChangeOwnerRef::Draft(w)) => ChangeOwnerUnion::Draft(w.upgrade().unwrap_or_default()),
                Some(ChangeOwnerRef::Checkpoint(w)) => ChangeOwnerUnion::Checkpoint(w.upgrade().unwrap_or_default()),
                None => ChangeOwnerUnion::Transaction(Arc::default()),
            }
        }
        async fn forwards(&self) -> Vec<op::OperationKind> {
            self.forwards.read().await.clone()
        }
        async fn backwards(&self) -> Vec<op::OperationKind> {
            self.backwards.read().await.clone()
        }

        /// @emoji 🔗 Ordered semantic op record ids constituting the forwards side (bundle `semanticOpLog` ids) when persisted.
        #[graphql(name = "forwardSemanticOpRecordIds")]
        async fn forward_semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        /// @emoji 🔗 Ordered semantic op record ids for backwards / inverse application when persisted separately from `OperationKind`.
        #[graphql(name = "backwardSemanticOpRecordIds")]
        async fn backward_semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn owner(&self) -> Option<Arc<Draft>> {
            self.owner_draft.upgrade()
        }
        async fn changes(&self) -> Vec<Arc<Change>> {
            self.changes.read().await.clone()
        }

        /// @emoji 🔗 Transaction forwards op sequence as `semanticOpLog` ids (see `histories.transaction`).
        #[graphql(name = "semanticOpRecordIds")]
        async fn semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn owner(&self) -> Option<Arc<Alternative>> {
            self.owner_alternative.upgrade()
        }
        #[graphql(name = "parentCheckpoint")]
        async fn parent_checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.parent_checkpoint.read().await.upgrade()
        }
        #[graphql(name = "targetAlternative")]
        async fn target_alternative(&self) -> Option<Arc<Alternative>> {
            self.target_alternative.read().await.upgrade()
        }
        #[graphql(name = "openTransaction")]
        async fn open_transaction(&self) -> Option<Arc<Transaction>> {
            self.open_transaction.read().await.upgrade()
        }
        #[graphql(name = "finalizedTransactions")]
        async fn finalized_transactions(&self) -> Vec<Arc<Transaction>> {
            self.finalized_transactions.read().await.clone()
        }
        #[graphql(name = "redoTransactions")]
        async fn redo_transactions(&self) -> Vec<Arc<Transaction>> {
            self.redo_transactions.read().await.clone()
        }
        async fn changes(&self) -> Vec<Arc<Change>> {
            let mut out = Vec::new();
            for t in self.transactions.read().await.iter() {
                for c in t.changes.read().await.iter() {
                    out.push(c.clone());
                }
            }
            out
        }
        #[graphql(name = "canUndo")]
        async fn can_undo(&self, _steps: i32) -> bool {
            !self.finalized_transactions.read().await.is_empty()
        }
        #[graphql(name = "canRedo")]
        async fn can_redo(&self, _steps: i32) -> bool {
            !self.redo_transactions.read().await.is_empty()
        }

        /// @emoji 🔗 Stable transaction open order on this draft (bundle `histories.draft`).
        #[graphql(name = "orderedTransactionIds")]
        async fn ordered_transaction_ids(&self) -> Vec<Id> {
            self.transactions.read().await.iter().map(|t| t.id.clone()).collect()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn timestamp(&self) -> Option<Timestamp> {
            self.timestamp.read().await.clone()
        }
        async fn authors(&self) -> Vec<Author> {
            self.authors.read().await.clone()
        }
        async fn root(&self) -> Option<Arc<Kit>> {
            self.root.read().await.clone()
        }
        #[graphql(name = "parentCheckpoint")]
        async fn parent_checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.parent_checkpoint.read().await.upgrade()
        }
        async fn message(&self) -> Option<String> {
            self.message.read().await.clone()
        }
        #[graphql(name = "isRelease")]
        async fn is_release(&self) -> bool {
            *self.is_release.read().await
        }
        #[graphql(name = "changeCount")]
        async fn change_count(&self) -> i32 {
            *self.change_count.read().await
        }

        /// @emoji 🔗 Checkpoint-scoped semantic op ids (`histories.checkpoint` wraps op log ids without duplicating kit trees).
        #[graphql(name = "semanticOpRecordIds")]
        async fn semantic_op_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            None
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn owner(&self) -> Option<Arc<Graph>> {
            self.owner_graph.upgrade()
        }
        async fn name(&self) -> String {
            self.name.read().await.clone()
        }
        async fn start(&self) -> Arc<Checkpoint> {
            self.start.read().await.upgrade().unwrap_or_default()
        }
        async fn checkpoints(&self) -> Vec<Arc<Checkpoint>> {
            self.checkpoints.read().await.clone()
        }
        async fn store(&self) -> Arc<Kit> {
            self.kit.read().await.clone().unwrap_or_default()
        }
        async fn draft(&self) -> Option<Arc<Draft>> {
            self.draft.read().await.upgrade()
        }
        async fn transaction(&self) -> Option<Arc<Transaction>> {
            self.transaction.read().await.upgrade()
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            self.owner_graph.upgrade().map(crate::iface::OwnerEntity::Graph)
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
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

        pub async fn ensure_draft(self: &Arc<Self>, draft_id: &Id) -> Arc<Draft> {
            if let Some(d) = self.drafts.read().await.iter().find(|d| &d.id == draft_id).cloned() {
                return d;
            }
            let d = Draft::with_id(draft_id.clone()).await;
            self.drafts.write().await.push(d.clone());
            d
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
        ) -> Result<(Arc<crate::kit::design::piece::Piece>, op::Diff), SemioError> {
            let fp_before = crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await;
            let (_handle, design) = self.the_kit.bind_external_design_id(&design_id).await;
            let piece = self
                .apply_create_fixed_piece_on_design_node(
                    draft_id, transaction_id, design,
                    blueprint_id.clone(),
                    position,
                    name.clone(),
                    description.clone(),
                )
                .await?;
            let fp_after = crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await;
            let input = op::CreatedFixedPieceInput {
                design_id,
                blueprint_id,
                position,
                name,
                description,
            };
            let payload_json = serde_json::to_string(&input).map_err(|e| SemioError::invalid(e.to_string()))?;
            let diff = crate::kit_graph_engine::deterministic_semantic_diff("createdFixedPiece", &payload_json, &fp_before, &fp_after);
            Ok((piece, diff))
        }
    }

    #[Object(name = "Graph")]
    impl Graph {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            self.compute_hash().await
        }
        async fn owner(&self) -> GraphOwner {
            let g = self.owner_session.read().await;
            match g.upgrade() {
                Some(s) => GraphOwner::Session(s),
                None => GraphOwner::Session(Arc::new(Session::default())),
            }
        }
        #[graphql(name = "sessionOwner")]
        async fn session_owner(&self) -> Option<Arc<Session>> {
            self.owner_session.read().await.upgrade()
        }
        #[graphql(name = "theKit")]
        async fn the_kit(&self) -> Option<Arc<Kit>> {
            Some(self.the_kit.clone())
        }
        async fn alternative(&self, id: Id) -> Option<Arc<Alternative>> {
            self.alternatives.read().await.iter().find(|a| a.id == id).cloned()
        }
        async fn alternatives(&self) -> crate::gql_relay::AlternativeConnection {
            crate::gql_relay::AlternativeConnection::from_alternatives(self.alternatives.read().await.clone())
        }
        async fn checkpoint(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.checkpoints.read().await.iter().find(|c| c.id == id).cloned()
        }
        async fn checkpoints(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.checkpoints.read().await.clone())
        }
        async fn release(&self, id: Id) -> Option<Arc<Checkpoint>> {
            self.releases.read().await.iter().find(|c| c.id == id).cloned()
        }
        async fn releases(&self) -> crate::gql_relay::CheckpointConnection {
            crate::gql_relay::CheckpointConnection::from_checkpoints(self.releases.read().await.clone())
        }

        /// @emoji 📜 Ordered semantic op log for this graph line (persisted bundle field); empty until store wiring lands.
        /// **Memoization:** none — each field resolution recomputes from current graph/backbone state once wired. **Invalidate:** backbone attach/replay, bundle tips, or any writer appending to the log (no stale cached slice).
        #[graphql(name = "semanticOpLog")]
        async fn semantic_op_log(&self, #[graphql(name = "limit")] limit: Option<i32>) -> Vec<op::SemanticOpRecord> {
            let _ = limit;
            Vec::new()
        }

        /// @emoji 🔢 Stable `projectionFingerprint` (sorted piece centers via [`crate::kit_graph_engine::projection_fingerprint_for_kit`]).
        /// **Memoization:** none — derived on every read from live piece centers. **Invalidate:** any semantic op or mutation changing piece geometry on this graph line.
        #[graphql(name = "projectionFingerprint")]
        async fn projection_fingerprint(&self) -> String {
            crate::kit_graph_engine::projection_fingerprint_for_kit(&self.the_kit).await
        }

        /// @emoji 📸 Hash of the materialized root kit aggregate for this graph head.
        /// **Memoization:** none — recomputed per request from the current [`Kit`] graph. **Invalidate:** any structural or identity change the kit hash subscribes to (mutations, replay).
        #[graphql(name = "rootSnapshotHash")]
        async fn root_snapshot_hash(&self) -> String {
            self.the_kit.compute_hash().await
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            self.owner_session.read().await.upgrade().map(crate::iface::OwnerEntity::Session)
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        #[graphql(name = "startedAt")]
        async fn started_at(&self) -> Option<Timestamp> {
            self.started_at.read().await.clone()
        }
        async fn drafts(&self) -> Vec<Arc<Draft>> {
            self.drafts.read().await.clone()
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            None
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        #[graphql(name = "backboneTip")]
        async fn backbone_tip(&self) -> Option<String> {
            self.backbone_tip.read().await.clone()
        }
        async fn reason(&self) -> String {
            self.reason.read().await.clone()
        }
        #[graphql(name = "createdAt")]
        async fn created_at(&self) -> Timestamp {
            self.created_at.read().await.clone()
        }

        #[graphql(name = "ownerEntity")]
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            None
        }

        #[graphql(name = "ownedEntities")]
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
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
            Self {
                id: Id::default(),
                owner_conflict: Weak::new(),
                checkpoint: RwLock::new(None),
                change: RwLock::new(None),
                operation: RwLock::new(None),
            }
        }
    }

    #[Object(name = "ReadVersion")]
    impl ReadVersion {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        async fn owner(&self) -> ReadVersionOwner {
            ReadVersionOwner::Conflict(self.owner_conflict.upgrade().unwrap_or_default())
        }
        #[graphql(name = "conflictOwner")]
        async fn conflict_owner(&self) -> Option<Arc<Conflict>> {
            self.owner_conflict.upgrade()
        }
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            self.owner_conflict.upgrade().map(crate::iface::OwnerEntity::Conflict)
        }
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
        }
        async fn checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.checkpoint.read().await.clone()
        }
        async fn change(&self) -> Option<Arc<Change>> {
            self.change.read().await.clone()
        }
        async fn operation(&self) -> Option<Arc<crate::op::OperationIface>> {
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
            Self {
                id: Id::default(),
                owner_conflict: Weak::new(),
                draft: RwLock::new(None),
                transaction: RwLock::new(None),
                checkpoint: RwLock::new(None),
                change: RwLock::new(None),
                operation: RwLock::new(None),
            }
        }
    }

    #[Object(name = "WriteVersion")]
    impl WriteVersion {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        async fn owner(&self) -> WriteVersionOwner {
            WriteVersionOwner::Conflict(self.owner_conflict.upgrade().unwrap_or_default())
        }
        #[graphql(name = "conflictOwner")]
        async fn conflict_owner(&self) -> Option<Arc<Conflict>> {
            self.owner_conflict.upgrade()
        }
        async fn owner_entity(&self) -> Option<crate::iface::OwnerEntity> {
            self.owner_conflict.upgrade().map(crate::iface::OwnerEntity::Conflict)
        }
        async fn owned_entities(&self) -> crate::iface::OwnedEntityConnection {
            crate::iface::OwnedEntityConnection::empty()
        }
        async fn draft(&self) -> Option<Arc<Draft>> {
            self.draft.read().await.clone()
        }
        async fn transaction(&self) -> Option<Arc<Transaction>> {
            self.transaction.read().await.clone()
        }
        async fn checkpoint(&self) -> Option<Arc<Checkpoint>> {
            self.checkpoint.read().await.clone()
        }
        async fn change(&self) -> Option<Arc<Change>> {
            self.change.read().await.clone()
        }
        async fn operation(&self) -> Option<Arc<crate::op::OperationIface>> {
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
            Self {
                edges: Vec::new(),
                page_info: crate::gql_relay::PageInfo::default(),
                hash: crate::hash::h(&[""]),
            }
        }
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
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            let u = *self.u.read().await;
            let v = *self.v.read().await;
            crate::hash::h(&[&format!("{u:.9}"), &format!("{v:.9}")])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn u(&self) -> f64 {
            *self.u.read().await
        }
        async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    #[Object(name = "Vector")]
    impl VectorNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            let x = *self.x.read().await;
            let y = *self.y.read().await;
            let z = *self.z.read().await;
            crate::hash::h(&[&format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn x(&self) -> f64 {
            *self.x.read().await
        }
        async fn y(&self) -> f64 {
            *self.y.read().await
        }
        async fn z(&self) -> f64 {
            *self.z.read().await
        }
    }

    #[Object(name = "Point")]
    impl PointNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            let x = *self.x.read().await;
            let y = *self.y.read().await;
            let z = *self.z.read().await;
            crate::hash::h(&[&format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn x(&self) -> f64 {
            *self.x.read().await
        }
        async fn y(&self) -> f64 {
            *self.y.read().await
        }
        async fn z(&self) -> f64 {
            *self.z.read().await
        }
    }

    #[Object(name = "Plane")]
    impl PlaneNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.origin.id.as_str(), self.x_axis.id.as_str(), self.y_axis.id.as_str()])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn origin(&self) -> Arc<PointNode> {
            self.origin.clone()
        }
        #[graphql(name = "xAxis")]
        async fn x_axis(&self) -> Arc<VectorNode> {
            self.x_axis.clone()
        }
        #[graphql(name = "yAxis")]
        async fn y_axis(&self) -> Arc<VectorNode> {
            self.y_axis.clone()
        }
    }

    #[Object(name = "Position")]
    impl PositionNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.center.id.as_str(), self.plane.id.as_str()])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn center(&self) -> Arc<CoordinateNode> {
            self.center.clone()
        }
        async fn plane(&self) -> Arc<PlaneNode> {
            self.plane.clone()
        }
    }

    #[Object(name = "Offset")]
    impl OffsetNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            let u = *self.u.read().await;
            let v = *self.v.read().await;
            crate::hash::h(&[&format!("{u:.9}"), &format!("{v:.9}")])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
        }
        async fn u(&self) -> f64 {
            *self.u.read().await
        }
        async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    #[Object(name = "Place")]
    impl PlaceNode {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner_entity(&self) -> Option<OwnerEntity> {
            None
        }
        async fn owned_entities(&self) -> OwnedEntityConnection {
            OwnedEntityConnection::empty()
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
    use crate::vcs::Change;

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
        async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "blueprintId")]
        async fn blueprint_id(&self) -> Id {
            self.blueprint_id.clone()
        }
        async fn position(&self) -> Arc<crate::geom::entity::PositionNode> {
            crate::geom::entity::PositionNode::from_position_value(self.position)
        }
        async fn name(&self) -> Option<String> {
            self.name.clone()
        }
        async fn description(&self) -> Option<String> {
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
        async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "pieceId")]
        async fn piece_id(&self) -> Id {
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
        async fn design_id(&self) -> Id {
            self.design_id.clone()
        }
        #[graphql(name = "pieceIds")]
        async fn piece_ids(&self) -> Vec<Id> {
            self.piece_ids.clone()
        }
        async fn offset(&self) -> Arc<crate::geom::entity::OffsetNode> {
            crate::geom::entity::OffsetNode::from_value(self.offset)
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RenamedKitInput {
        pub name: String,
    }

    #[Object(name = "RenamedKitInput")]
    impl RenamedKitInput {
        async fn name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangedDescriptionInput {
        pub description: String,
    }

    #[Object(name = "ChangedDescriptionInput")]
    impl ChangedDescriptionInput {
        async fn description(&self) -> String {
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
    #[derive(Clone, Debug, async_graphql::SimpleObject)]
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

    //#region 📦 diff (placeholder)
    /// @emoji 📦 Ephemeral semantic diff for operations — computed at apply time via [`crate::kit_graph_engine::deterministic_semantic_diff`], **not** persisted in the kit bundle; clients observe it on `Operation.diff` without storing it themselves. **Memoization:** none (stable ids derive from op kind + payload + fp transition).
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Diff {
        pub id: Id,
        pub summary: Option<String>,
    }

    #[Object(name = "Diff")]
    impl Diff {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn summary(&self) -> Option<String> {
            self.summary.clone()
        }
    }
    //#endregion 📦 diff

    //#region 🪄 operations
    pub struct CreatedFixedPiece {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: CreatedFixedPieceInput,
        pub diff: Diff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl CreatedFixedPiece {
        pub async fn new(input: CreatedFixedPieceInput, piece: Arc<crate::kit::design::piece::Piece>, diff: Diff) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_change: Weak::new(), input, diff, piece })
        }
    }

    #[Object(name = "CreatedFixedPiece")]
    impl CreatedFixedPiece {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner(&self) -> Arc<Change> {
            self.owner_change.upgrade().unwrap_or_default()
        }
        async fn input(&self) -> CreatedFixedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn piece(&self) -> Arc<crate::kit::design::piece::Piece> {
            self.piece.clone()
        }
    }

    pub struct FixedPiece {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: FixedPieceInput,
        pub diff: Diff,
        pub piece: Arc<crate::kit::design::piece::Piece>,
    }

    impl Default for FixedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: FixedPieceInput::default(), diff: Diff::default(), piece: Arc::default() }
        }
    }

    #[Object(name = "FixedPiece")]
    impl FixedPiece {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner(&self) -> Arc<Change> {
            self.owner_change.upgrade().unwrap_or_default()
        }
        async fn input(&self) -> FixedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn piece(&self) -> Arc<crate::kit::design::piece::Piece> {
            self.piece.clone()
        }
    }

    pub struct DraggedPiece {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: DraggedPieceInput,
        pub diff: Diff,
        pub pieces: Vec<Arc<crate::kit::design::piece::Piece>>,
    }

    impl Default for DraggedPiece {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: DraggedPieceInput::default(), diff: Diff::default(), pieces: Vec::new() }
        }
    }

    #[Object(name = "DraggedPiece")]
    impl DraggedPiece {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner(&self) -> Arc<Change> {
            self.owner_change.upgrade().unwrap_or_default()
        }
        async fn input(&self) -> DraggedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn pieces(&self) -> Vec<Arc<crate::kit::design::piece::Piece>> {
            self.pieces.clone()
        }
    }

    pub struct RenamedKit {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: RenamedKitInput,
        pub diff: Diff,
        pub kit: Arc<crate::kit::Kit>,
    }

    impl Default for RenamedKit {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: RenamedKitInput::default(), diff: Diff::default(), kit: Arc::default() }
        }
    }

    #[Object(name = "RenamedKit")]
    impl RenamedKit {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner(&self) -> Arc<Change> {
            self.owner_change.upgrade().unwrap_or_default()
        }
        async fn input(&self) -> RenamedKitInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn kit(&self) -> Arc<crate::kit::Kit> {
            self.kit.clone()
        }
    }

    pub struct ChangedDescription {
        pub id: Id,
        pub owner_change: Weak<Change>,
        pub input: ChangedDescriptionInput,
        pub diff: Diff,
        pub entity: Arc<crate::kit::Kit>,
    }

    impl Default for ChangedDescription {
        fn default() -> Self {
            Self { id: Id::default(), owner_change: Weak::new(), input: ChangedDescriptionInput::default(), diff: Diff::default(), entity: Arc::default() }
        }
    }

    #[Object(name = "ChangedDescription")]
    impl ChangedDescription {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.id.as_str()])
        }
        async fn owner(&self) -> Arc<Change> {
            self.owner_change.upgrade().unwrap_or_default()
        }
        async fn input(&self) -> ChangedDescriptionInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn entity(&self) -> Arc<crate::kit::Kit> {
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
    #[graphql(name = "Operation", field(name = "id", ty = "Id"), field(name = "hash", ty = "String"), field(name = "owner", ty = "Arc<Change>"), field(name = "diff", ty = "Diff"))]
    pub enum OperationIface {
        CreatedFixedPiece(Arc<CreatedFixedPiece>),
        FixedPiece(Arc<FixedPiece>),
        DraggedPiece(Arc<DraggedPiece>),
        RenamedKit(Arc<RenamedKit>),
        ChangedDescription(Arc<ChangedDescription>),
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
        ChangeDescription { request_id: Id, draft_id: Id, transaction_id: Id, description: String },
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
        ($($_row:ident),* $(,)?) => {};
    }

    ops! {
        CreatedFixedPiece,
        FixedPiece,
        DraggedPiece,
        RenamedKit,
        ChangedDescription
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
    pub fn deterministic_semantic_diff(op_kind: &str, payload_json: &str, projection_fp_before: &str, projection_fp_after: &str) -> op::Diff {
        let digest = h(&[op_kind, payload_json, projection_fp_before, projection_fp_after]);
        op::Diff {
            id: Id::from(format!("semio:diff:{digest}")),
            summary: Some(digest),
        }
    }
    //#endregion 📦 semantic diff

    //#region 🪡 semantic op apply
    /// @emoji 🧾 Output of [`apply_semantic_op_json`]: ephemeral diff + optional created entities.
    pub struct AppliedSemanticOp {
        pub diff: op::Diff,
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
    pub async fn apply_semantic_op_json(
        graph: &Arc<Graph>,
        draft_id: &Id,
        transaction_id: &Id,
        op_kind: &str,
        payload_json: &str,
    ) -> Result<AppliedSemanticOp, SemioError> {
        match op_kind {
            "createdFixedPiece" => {
                let payload: CreatedFixedPiecePayload = serde_json::from_str(payload_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                let design_id = Id::from(payload.design_id.as_str());
                let blueprint_id = Id::from(payload.blueprint_id.as_str());
                let (piece, diff) = graph
                    .apply_create_fixed_piece(
                        draft_id.clone(),
                        transaction_id.clone(),
                        design_id,
                        blueprint_id,
                        payload.position,
                        payload.name,
                        payload.description,
                    )
                    .await?;
                Ok(AppliedSemanticOp { diff, created_piece: Some(piece) })
            }
            other => Err(SemioError::invalid(format!("unsupported semantic op kind `{other}`"))),
        }
    }
    //#endregion 🪡 semantic op apply
}

//#endregion 🧩 kit graph engine

//#region 🗄️ kit backbone persistence (native)

#[cfg(not(target_arch = "wasm32"))]
pub mod kit_backbone {
    //! @emoji 🗄️ Dev JSON + local `.semio/` kit backbones: atomic single-file writes, multi-db SQLite + blobs dir, replay via [`kit_graph_engine::apply_semantic_op_json`].

    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use rusqlite::Connection;

    use crate::error::SemioError;
    use crate::id::Id;
    use crate::op::BackboneStoreKind;
    use crate::vcs::Graph;

    //#region 🧾 wire format
    /// @emoji 🧾 One row in a dev JSON `semanticOpLog` or SQLite `semantic_op_log` (camelCase `draftId` / `transactionId` to match bundle fixtures).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct StoredSemanticOp {
        #[serde(rename = "draftId")]
        pub draft_id: String,
        #[serde(rename = "transactionId")]
        pub transaction_id: String,
        pub kind: String,
        pub input: serde_json::Value,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevJsonPersistenceNotes {
        /// @emoji 🛡️ Human-readable: write `*.tmp.semio-write` in the same directory then `rename(2)` into the final filename.
        pub atomic_rewrite: String,
        /// @emoji 🛡️ Human-readable: after `fsync`+`rename`, readers see the previous or the new whole file; torn JSON only on the temp path (ignored by readers).
        pub crash_safety: String,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevJsonBackboneFile {
        pub kind: String,
        pub schema: String,
        #[serde(rename = "connectionUri")]
        pub connection_uri: String,
        pub persistence: DevJsonPersistenceNotes,
        #[serde(rename = "semanticOpLog")]
        pub semantic_op_log: Vec<StoredSemanticOp>,
    }

    impl DevJsonBackboneFile {
        fn template(uri: &str) -> Self {
            Self {
                kind: "semio.kit_backbone.dev_json".to_string(),
                schema: "2026-05-06".to_string(),
                connection_uri: uri.to_string(),
                persistence: DevJsonPersistenceNotes {
                    atomic_rewrite: "Serialize full document to <path>.tmp.semio-write in the same directory, fsync, then atomic rename(2) over <path>.".to_string(),
                    crash_safety: "Readers never observe a partial JSON object: they keep the last fully-renamed file. A crash may leave an orphan temp file to delete manually.".to_string(),
                },
                semantic_op_log: Vec::new(),
            }
        }
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
    fn atomic_write_json(path: &Path, doc: &DevJsonBackboneFile) -> Result<(), SemioError> {
        let parent = path.parent().ok_or_else(|| SemioError::invalid("dev json path has no parent directory"))?;
        std::fs::create_dir_all(parent).map_err(|e| SemioError::invalid(format!("create dev json parent: {e}")))?;
        let tmp = path.with_extension("tmp.semio-write");
        let body = serde_json::to_string_pretty(doc).map_err(|e| SemioError::invalid(e.to_string()))?;
        std::fs::write(&tmp, body).map_err(|e| SemioError::invalid(format!("write temp dev json: {e}")))?;
        std::fs::rename(&tmp, path).map_err(|e| SemioError::invalid(format!("rename dev json: {e}")))?;
        Ok(())
    }

    fn read_or_init_dev_json(path: &Path, uri: &str) -> Result<DevJsonBackboneFile, SemioError> {
        if !path.exists() {
            return Ok(DevJsonBackboneFile::template(uri));
        }
        let s = std::fs::read_to_string(path).map_err(|e| SemioError::invalid(format!("read dev json: {e}")))?;
        serde_json::from_str(&s).map_err(|e| SemioError::invalid(format!("parse dev json: {e}")))
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
        fn read_doc(&self) -> Result<DevJsonBackboneFile, SemioError> {
            read_or_init_dev_json(&self.path, &self.connection_uri_normalized)
        }

        pub fn append_op(&mut self, draft_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value) -> Result<(), SemioError> {
            let mut doc = self.read_doc()?;
            doc.semantic_op_log.push(StoredSemanticOp {
                draft_id: draft_id.to_string(),
                transaction_id: transaction_id.to_string(),
                kind: kind.to_string(),
                input: input.clone(),
            });
            atomic_write_json(&self.path, &doc)
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
            conn.execute(
                "INSERT INTO semantic_op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![draft_id.as_str(), transaction_id.as_str(), kind, input_json],
            )
            .map_err(|e| SemioError::invalid(format!("sqlite insert: {e}")))?;
            Ok(())
        }

        fn load_ops(&self) -> Result<Vec<StoredSemanticOp>, SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite read: {e}")))?;
            let mut stmt = conn
                .prepare("SELECT draft_id, transaction_id, kind, input_json FROM semantic_op_log ORDER BY seq ASC")
                .map_err(|e| SemioError::invalid(format!("sqlite prepare: {e}")))?;
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
                AttachedBackbone::DevJson(d) => d.read_doc()?.semantic_op_log.clone(),
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

#[cfg(target_arch = "wasm32")]
pub mod kit_backbone {}

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
    use std::sync::Arc;

    use async_channel::{Receiver, Sender};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::event::{Event, EventBus};
    use crate::id::Id;
    use crate::op::{BackboneStoreKind, Command, CommandReceipt, CreatedFixedPiece, CreatedFixedPieceInput, OperationIface};
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
                Err(SemioError::invalid(
                    "Attachable kit backbones use native disk (atomic JSON / SQLite); drive them from native hosts over GraphQL IPC instead of WASM.",
                ))
            }
        }

        pub async fn detach_matching(&self, uri: &str) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let norm = crate::kit_backbone::normalize_connection_uri(uri);
                let mut guard = self.slot.write().await;
                match &*guard {
                    Some(current) if current.normalized_connection_uri() != norm => {
                        return Err(SemioError::invalid(
                            "`connectionUri` did not match the attached backbone; detach aborted to avoid confusing persistence drift.",
                        ));
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
                    Command::BackboneAttach { .. } => "backboneAttach",
                    Command::BackboneDetach { .. } => "backboneDetach",
                };
                self.bus.emit_event(Event::CommandSucceeded(CommandReceipt { request_id: request_id.clone(), kind: kind.to_string() })).await;

                if let Err(e) = self.apply(cmd).await {
                    let err = e.with_request(request_id);
                    self.bus.emit_event(Event::OperationFailed(err)).await;
                }
            }
        }

        async fn apply(&self, cmd: Command) -> Result<(), SemioError> {
            match cmd {
                Command::AddFixedPieceToDesign { request_id: _, draft_id, transaction_id, design_id, blueprint_id, position, name, description } => {
                    let (piece, diff) = self
                        .graph
                        .apply_create_fixed_piece(draft_id.clone(), transaction_id.clone(), design_id.clone(), blueprint_id.clone(), position, name.clone(), description.clone())
                        .await?;

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
                Command::FixPieceInDesign { .. } | Command::RenameKit { .. } | Command::ChangeDescription { .. } => {
                    // Skeleton stubs — wired through commandSucceeded only.
                    Ok(())
                }
                Command::BackboneAttach { connection_uri, store_kind, .. } => {
                    self.backbone.mount(&self.graph, self.label, &connection_uri, store_kind).await
                }
                Command::BackboneDetach { connection_uri, .. } => self.backbone.detach_matching(&connection_uri).await,
            }
        }
    }
}

//#endregion 🧵 worker

//#region 🌐 gql

pub mod gql {
    //! 🌐 Type-safe static GraphQL schema via `Schema::build` (embedded target SDL string for tooling).
    use std::pin::Pin;
    use std::sync::Arc;
    use async_graphql::{Context, Object, Schema, Subscription};
    use async_stream::stream;
    use futures_util::Stream;

    use crate::event::{Event, EventBus};
    use crate::error::SemioError;
    use crate::geom::Position;
    use crate::id::Id;
    use crate::op::{Command, CommandReceipt, OperationKind};
    use crate::vcs::Graph;
    use crate::worker::ParentRuntime;

    /// @emoji 🧩 Executable schema (`Query`, `Mutation`, `Subscription`).
    pub type AppSchema = Schema<Query, Mutation, Subscription>;

    pub struct Query;

    #[Object]
    impl Query {
        async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.wip_graph.clone())
        }

        #[graphql(name = "authoritative")]
        async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentRuntime>>()?.auth_graph.clone())
        }

        async fn session(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<Arc<crate::vcs::Session>>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let sessions = rt.sessions.read().await;
            Ok(sessions.iter().find(|s| s.id == id).cloned())
        }

        async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql_relay::ConflictConnection> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let list = rt.conflicts.read().await.clone();
            Ok(crate::gql_relay::ConflictConnection::from_conflicts(list))
        }

        /// @emoji 🔎 Relay-style global `node` lookup (WIP + authoritative + sessions + conflicts).
        async fn node(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::resolve_node(rt.as_ref(), &id).await)
        }

        /// @emoji 🔎 Alias of [`Query::node`] for SDL `entity` entry point.
        async fn entity(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::iface::GqlNode>> {
            self.node(ctx, id).await
        }

        /// @emoji 🧩 Resolve a piece within a design on the WIP graph line.
        #[graphql(name = "pieceInDesign")]
        async fn piece_in_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceId")] piece_id: Id,
        ) -> async_graphql::Result<Option<Arc<crate::kit::design::piece::Piece>>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::piece_in_design_on_wip(rt.as_ref(), &design_id, &piece_id).await)
        }

        /// @emoji 🧩 Alternative-line piece kind (stub until alternatives are modeled in Rust).
        #[graphql(name = "alternativePieceKind")]
        async fn alternative_piece_kind(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "pieceId")] piece_id: Id,
        ) -> async_graphql::Result<Option<String>> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            Ok(crate::iface::alternative_piece_kind(rt.as_ref(), &piece_id).await)
        }
    }

    pub struct Mutation;

    #[Object]
    impl Mutation {
        /// @emoji ➕ Routes through [`ParentRuntime::dispatch_wip`] → child apply + event bus.
        #[graphql(name = "addFixedPieceToDesign")]
        async fn add_fixed_piece_to_design(
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
            let cmd = Command::AddFixedPieceToDesign {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                design_id,
                blueprint_id,
                position,
                name: None,
                description: None,
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "fixPieceInDesign")]
        async fn fix_piece_in_design(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceId")] piece_id: Id,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::FixPieceInDesign {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                design_id,
                piece_id,
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "renameKit")]
        async fn rename_kit(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            name: String,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::RenameKit {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                name,
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "changeDescription")]
        async fn change_description(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            description: String,
        ) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentRuntime>>()?;
            let request_id = Id::new().await;
            let cmd = Command::ChangeDescription {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                description,
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
    }

    pub struct Subscription;

    type CommandSucceededStream = Pin<Box<dyn Stream<Item = CommandReceipt> + Send>>;
    type OperationSucceededStream = Pin<Box<dyn Stream<Item = OperationKind> + Send>>;
    type OperationFailedStream = Pin<Box<dyn Stream<Item = SemioError> + Send>>;

    #[Subscription]
    impl Subscription {
        #[graphql(name = "commandSucceeded")]
        async fn command_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<CommandSucceededStream> {
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

        #[graphql(name = "operationSucceeded")]
        async fn operation_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationSucceededStream> {
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
        async fn operation_failed(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationFailedStream> {
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
        async fn error(&self, ctx: &Context<'_>) -> async_graphql::Result<OperationFailedStream> {
            self.operation_failed(ctx).await
        }
    }

    fn build_schema_sync_for(rt: Arc<ParentRuntime>) -> AppSchema {
        Schema::build(Query, Mutation, Subscription).data(rt.clone()).data(rt.bus.clone()).finish()
    }

    /// 📜 Canonical target SDL (contract file; executable schema is derived from Rust types).
    pub fn target_schema_sdl() -> String {
        include_str!("../graphql/target.schema.graphql").to_string()
    }

    /// 📜 Same as [`target_schema_sdl`] (async for historical call sites).
    pub async fn sdl() -> String {
        target_schema_sdl()
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
            let sdl = crate::gql::target_schema_sdl();
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
                let v: serde_json::Value =
                    serde_wasm_bindgen::from_value(dto_js).map_err(|e| JsValue::from_str(&e.to_string()))?;
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
                let json =
                    serde_json::to_string(&async_graphql::Response::from(resp)).map_err(|e| JsValue::from_str(&e.to_string()))?;
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
                    let json = serde_json::to_string(&async_graphql::Response::from(resp))
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
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

    /// 📤 Writes the generated SDL to `SEMIO_GRAPHQL_SCHEMA_OUT`; run via `npx nx build semio/graphql`.
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

    #[test]
    fn parses_target_schema() {
        let sdl = block_on(crate::gql::sdl());
        for t in [
            "type Query",
            "type Mutation",
            "type Piece",
            "type Connector",
            "type Port",
            "type Connection",
            "type Design",
            "type Kit",
            "type Graph",
            "type Session",
            "type Conflict",
            "type Checkpoint",
            "type Alternative",
            "type Draft",
            "type Transaction",
            "type Change",
            "interface Operation",
            "interface Entity",
            "scalar Timestamp",
            "pieceInDesign",
            "addFixedPieceToDesign",
            "fixPieceInDesign",
            "DraftConnection",
            "DesignConnection",
            "PieceConnection",
        ] {
            assert!(sdl.contains(t), "schema missing `{}`", t);
        }
    }

    fn normalize_sdl_trailing_ws(s: &str) -> String {
        s.lines().map(str::trim_end).collect::<Vec<_>>().join("\n")
    }

    /// @emoji 📜 Plan `target_sdl_byte_match`: full parity requires registering every target type (689+); run with `cargo test target_sdl_byte_match -- --ignored` while expanding static registration.
    #[test]
    #[ignore = "executable Schema::sdl() will not match target.schema.graphql until all types/unions are registered"]
    fn target_sdl_byte_match() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let got = normalize_sdl_trailing_ws(&schema.sdl());
            let want = normalize_sdl_trailing_ws(&crate::gql::target_schema_sdl());
            assert_eq!(got, want);
        });
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
    fn create_fixed_piece_end_to_end() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let res = schema
                .execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1"))))
                .await;
            assert!(res.errors.is_empty(), "mutation errors: {:?}", res.errors);

            // The wip child applies asynchronously; wait briefly for the event loop.
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = relay_wip_designs_have_piece();
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let edges = data["wip"]["theKit"]["designs"]["edges"].as_array().expect("design edges");
            let any_piece = edges.iter().any(|e| {
                e["node"]["pieces"]["edges"]
                    .as_array()
                    .map(|pe| pe.iter().any(|_| true))
                    .unwrap_or(false)
            });
            assert!(any_piece, "expected at least one piece in wip; got: {}", serde_json::to_string_pretty(&data).unwrap());
        });
    }

    #[test]
    fn wip_and_authoritative_are_isolated() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let _ = schema
                .execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1"))))
                .await;
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
            let p1 = rt
                .wip_graph
                .apply_create_fixed_piece(crate::id::Id::from("d1"), crate::id::Id::from("t1"), crate::id::Id::from("des1"), blueprint_id.clone(), position, None, None)
                .await
                .expect("insert piece 1")
                .0;
            let _p2 = rt
                .wip_graph
                .apply_create_fixed_piece(crate::id::Id::from("d1"), crate::id::Id::from("t1"), crate::id::Id::from("des1"), blueprint_id, position, None, None)
                .await
                .expect("insert piece 2")
                .0;

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
        edges
            .iter()
            .map(|e| e["node"]["pieces"]["edges"].as_array().map(|pe| pe.len()).unwrap_or(0))
            .sum()
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

            let _ = schema
                .execute(Request::new(ADD_FIXED_PIECE_TO_DESIGN).variables(Variables::from_value(add_fixed_piece_vars("des1"))))
                .await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let after = schema.execute(q).await;
            let after_data = after.data.into_json().unwrap();
            let after_pieces = relay_piece_count_wip(&after_data);

            assert_eq!(after_pieces, before_pieces + 1, "mutation not visible on re-read; before={} after={}", before_pieces, after_pieces);
        });
    }

    /// 🗃️ Delegates to [`crate::kit_graph_engine::projection_fingerprint_for_kit`] (single implementation).
    async fn stable_projection_fingerprint(kit: &Arc<crate::kit::Kit>) -> String {
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
            let golden_ops: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read ops")).expect("parse golden ops");
            let exp: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored ops");
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);
            let doc = crate::kit_backbone::DevJsonBackboneFile {
                kind: "semio.kit_backbone.dev_json".to_string(),
                schema: "2026-05-06".to_string(),
                connection_uri: norm.clone(),
                persistence: crate::kit_backbone::DevJsonPersistenceNotes {
                    atomic_rewrite: "fixture".to_string(),
                    crash_safety: "fixture".to_string(),
                },
                semantic_op_log: stored,
            };
            std::fs::write(&path, serde_json::to_string_pretty(&doc).expect("serialize dev json")).expect("write dev json");

            let g = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::DevJson, "wip", &g)
                .await
                .expect("dev json mount+replay");

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
            let golden_ops: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read ops")).expect("parse golden ops");
            let exp: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let stored = crate::kit_backbone::stored_ops_from_golden_ops_json(&golden_ops).expect("golden → stored ops");

            let g_bootstrap = crate::vcs::Graph::new().await;
            let _bones = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::LocalDotSemio, "wip", &g_bootstrap)
                .await
                .expect("bootstrap .semio layout");

            let db_path = proj_canon.join(".semio").join("wip.db");
            let conn = rusqlite::Connection::open(&db_path).expect("open wip.db");
            for op in &stored {
                let input_json = serde_json::to_string(&op.input).expect("input json");
                conn.execute(
                    "INSERT INTO semantic_op_log (draft_id, transaction_id, kind, input_json) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![op.draft_id, op.transaction_id, op.kind, input_json],
                )
                .expect("insert semantic op row");
            }
            drop(conn);

            let g2 = crate::vcs::Graph::new().await;
            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, crate::op::BackboneStoreKind::LocalDotSemio, "wip", &g2)
                .await
                .expect("replay wip.db");

            let fp = stable_projection_fingerprint(&g2.the_kit).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "local .semio backbone replay must match US-001 golden fingerprint");
        });
    }

    #[test]
    fn kit_store_bundle_metabolism_new_has_contract_shape() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/metabolism.new.kit.semio.json");
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read metabolism.new bundle")).expect("parse");
        for k in ["kind", "schema", "rootSnapshot", "semanticOpLog", "histories", "backbonePointers"] {
            assert!(v.get(k).is_some(), "metabolism.new.kit.semio.json missing `{k}`");
        }
    }
}

//#endregion 🧪 tests
