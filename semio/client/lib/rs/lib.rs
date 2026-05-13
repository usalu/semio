//! 🦀 semio rust control plane — in-memory Arc-reference architecture (code-first GraphQL).

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
    ( $( $region:ident : [ $( $ty:ty ),* $(,)? ] ),* $(,)? ) => {
        $(
            $(
                impl crate::sdl_registry::HasSdlFragment for $ty {
                    const SDL_FRAGMENT: &'static str = "";
                }
            )*
        )*
        pub(crate) fn push_all_fragments(out: &mut Vec<&'static str>) {
            $(
                $(
                    out.push(<$ty as crate::sdl_registry::HasSdlFragment>::SDL_FRAGMENT);
                )*
            )*
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
    geom: [
        crate::geom::entity::Vector,
        crate::geom::entity::Point,
        crate::geom::entity::Coordinate,
        crate::geom::entity::Offset,
        crate::geom::entity::Plane,
        crate::geom::entity::Position,
        crate::geom::entity::Location,
        crate::geom::entity::Place,
    ],
    meta: [
        crate::meta::Attribute,
        crate::meta::Author,
        crate::meta::File,
        crate::meta::Folder,
        crate::meta::Prop,
        crate::meta::Benchmark,
        crate::meta::Quality,
        crate::meta::Tag,
        crate::meta::Concept,
        crate::meta::Stat,
        crate::meta::Layer,
        crate::meta::Group,
        crate::gql_relay::Family,
    ],
    type_: [
        crate::kit::r#type::Type,
        crate::kit::r#type::Port,
        crate::kit::r#type::Connector,
        crate::kit::r#type::Representation,
    ],
    design: [
        crate::kit::design::Design,
        crate::kit::design::piece::Piece,
        crate::kit::design::connection::Side,
        crate::kit::design::connection::Connection,
        crate::kit::design::Clump,
    ],
    kit: [crate::kit::Kit],
    vcs: [
        crate::vcs::Edit,
        crate::vcs::Change,
        crate::vcs::Checkpoint,
        crate::vcs::TheKit,
        crate::vcs::Alternative,
        crate::vcs::Graph,
        crate::vcs::Session,
        crate::vcs::Conflict,
    ],
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

/// @emoji 🪢 `entity_relay_sync!` — relay Edge/Connection for `SimpleObject` entities with sync child digests (`compute_entity_hash`, …).
#[macro_export]
macro_rules! entity_relay_sync {
    ($Conn:ident, $Edge:ident, $Node:ty, $hash_fn:expr) => {
        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Edge {
            pub cursor: String,
            pub node: $Node,
        }

        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Conn {
            pub edges: Vec<$Edge>,
            #[graphql(name = "pageInfo")]
            pub page_info: std::sync::Arc<$crate::gql_relay::PageInfo>,
            pub hash: String,
        }

        impl $Conn {
            pub fn from_entities(entities: Vec<$Node>) -> Self {
                let mut child_hashes = Vec::with_capacity(entities.len());
                for r in &entities {
                    child_hashes.push($hash_fn(r));
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = entities.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        }
    };
}

/// @emoji 🪢 `entity_full_family!` — relay Edge/Connection for geometry (`VectorEdge`…`LocationEdge`).
#[macro_export]
macro_rules! entity_full_family {
    (
        $_base:ident,
        $node:ty,
        relay = ($conn:ident, $edge:ident)
    ) => {
        $crate::entity_relay!($conn, $edge, $node);
    };
}

/// @emoji 🪢 `entity_relay!` — relay Edge/Connection for `Arc` graph nodes with async `compute_hash` digests.
#[macro_export]
macro_rules! entity_relay {
    ($Conn:ident, $Edge:ident, $Node:ty) => {
        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Edge {
            pub cursor: String,
            pub node: $Node,
        }

        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Conn {
            pub edges: Vec<$Edge>,
            #[graphql(name = "pageInfo")]
            pub page_info: std::sync::Arc<$crate::gql_relay::PageInfo>,
            pub hash: String,
        }

        impl $Conn {
            pub async fn from_entities(entities: Vec<$Node>) -> Self {
                let mut child_hashes = Vec::with_capacity(entities.len());
                for r in &entities {
                    child_hashes.push(r.compute_hash().await);
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = entities.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        }
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

/// @emoji 🧾 `entity_input!` — GraphQL `InputObject` with explicit SDL `name` (no serde; control plane is GraphQL-native).
#[macro_export]
macro_rules! entity_input {
    (
        $(#[$sm:meta])*
        $vis:vis struct $Name:ident as $gql:literal {
            $($(#[$fm:meta])* $fvis:vis $field:ident : $ftype:ty),* $(,)?
        }
    ) => {
        $(#[$sm])*
        #[derive(Clone, Debug, Default, PartialEq, async_graphql::InputObject)]
        #[graphql(name = $gql)]
        $vis struct $Name {
            $($(#[$fm])* $fvis $field : $ftype),*
        }
    };
}

/// @emoji 🏷️ `entity_family!` — `SimpleObject` + sync `compute_entity_hash` + `ComplexObject::hash` resolver shell.
#[macro_export]
macro_rules! entity_family {
    (
        $(#[$sm:meta])*
        $vis:vis struct $Name:ident {
            $($(#[$fm:meta])* $fvis:vis $field:ident : $ftype:ty),* $(,)?
        }
        hash = |$this:ident| $body:block
        $(, extra = ($($extra:item)+))?
    ) => {
        $(#[$sm])*
        #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
        #[graphql(complex)]
        $vis struct $Name {
            $($(#[$fm])* $fvis $field : $ftype),*
        }
        impl $Name {
            pub fn compute_entity_hash(&self) -> String {
                let $this = self;
                $body
            }
        }
        #[async_graphql::ComplexObject]
        impl $Name {
            pub async fn hash(&self) -> String {
                self.compute_entity_hash()
            }
            $($($extra)+)?
        }
    };
}

/// @emoji 🏷️ `meta_arc_titled_entity!` — shared Arc/RwLock tag/concept entity (`new`, `new_with_id`, `compute_hash`, `Default`).
#[macro_export]
macro_rules! meta_arc_titled_entity {
    (
        $(#[$sm:meta])*
        $N:ident,
        $Owner:ty,
        $tag:literal
    ) => {
        $(#[$sm])*
        #[derive(Debug)]
        pub struct $N {
            pub id: crate::id::Id,
            pub owner: async_lock::RwLock<$Owner>,
            pub name: async_lock::RwLock<String>,
            pub description: async_lock::RwLock<Option<String>>,
            pub icon: async_lock::RwLock<Option<String>>,
            pub order: async_lock::RwLock<Option<i32>>,
            pub attributes: async_lock::RwLock<Vec<crate::meta::Attribute>>,
        }

        impl $N {
            pub async fn new(
                owner: $Owner,
                name: String,
                description: Option<String>,
                icon: Option<String>,
                order: Option<i32>,
                attributes: Vec<crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id: crate::id::Id::new().await,
                    owner: async_lock::RwLock::new(owner),
                    name: async_lock::RwLock::new(name),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    order: async_lock::RwLock::new(order),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub fn new_with_id(
                owner: $Owner,
                id: crate::id::Id,
                name: String,
                description: Option<String>,
                icon: Option<String>,
                order: Option<i32>,
                attributes: Vec<crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id,
                    owner: async_lock::RwLock::new(owner),
                    name: async_lock::RwLock::new(name),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    order: async_lock::RwLock::new(order),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let n = self.name.read().await;
                let d = self.description.read().await.clone().unwrap_or_default();
                let ic = self.icon.read().await.clone().unwrap_or_default();
                let ord = self.order.read().await.map(|o| o.to_string()).unwrap_or_default();
                let attrs = self.attributes.read().await;
                let mut child_hashes: Vec<String> = attrs.iter().map(crate::meta::Attribute::compute_entity_hash).collect();
                child_hashes.sort();
                crate::hash::merkle_node_str(&[$tag, self.id.as_str(), n.as_str(), d.as_str(), ic.as_str(), ord.as_str()], child_hashes)
            }
        }

        impl Default for $N {
            fn default() -> Self {
                Self {
                    id: crate::id::Id::default(),
                    owner: async_lock::RwLock::new(<$Owner as Default>::default()),
                    name: async_lock::RwLock::new(String::new()),
                    description: async_lock::RwLock::new(None),
                    icon: async_lock::RwLock::new(None),
                    order: async_lock::RwLock::new(None),
                    attributes: async_lock::RwLock::new(Vec::new()),
                }
            }
        }
    };
}

/// @emoji 🏷️ `meta_quality_entity!` — Arc/RwLock quality entity (`new`, `new_with_id`, `compute_hash`, `Default`).
#[macro_export]
macro_rules! meta_quality_entity {
    () => {
        /// @emoji 🏷️ SDL `Quality` entity (benchmarks stay value-typed [`Benchmark`] entities).
        #[derive(Debug)]
        pub struct Quality {
            pub id: crate::id::Id,
            pub owner: async_lock::RwLock<crate::meta::QualityOwnerSlot>,
            pub key: async_lock::RwLock<String>,
            pub value: async_lock::RwLock<Option<String>>,
            pub unit: async_lock::RwLock<Option<String>>,
            pub definition: async_lock::RwLock<Option<String>>,
            pub description: async_lock::RwLock<Option<String>>,
            pub icon: async_lock::RwLock<Option<String>>,
            pub benchmarks: async_lock::RwLock<Vec<crate::meta::Benchmark>>,
            pub attributes: async_lock::RwLock<Vec<crate::meta::Attribute>>,
        }

        impl Quality {
            pub async fn new(
                owner: crate::meta::QualityOwnerSlot,
                key: String,
                value: Option<String>,
                unit: Option<String>,
                definition: Option<String>,
                description: Option<String>,
                icon: Option<String>,
                benchmarks: Vec<crate::meta::Benchmark>,
                attributes: Vec<crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id: crate::id::Id::new().await,
                    owner: async_lock::RwLock::new(owner),
                    key: async_lock::RwLock::new(key),
                    value: async_lock::RwLock::new(value),
                    unit: async_lock::RwLock::new(unit),
                    definition: async_lock::RwLock::new(definition),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    benchmarks: async_lock::RwLock::new(benchmarks),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub fn new_with_id(
                owner: crate::meta::QualityOwnerSlot,
                id: crate::id::Id,
                key: String,
                value: Option<String>,
                unit: Option<String>,
                definition: Option<String>,
                description: Option<String>,
                icon: Option<String>,
                benchmarks: Vec<crate::meta::Benchmark>,
                attributes: Vec<crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id,
                    owner: async_lock::RwLock::new(owner),
                    key: async_lock::RwLock::new(key),
                    value: async_lock::RwLock::new(value),
                    unit: async_lock::RwLock::new(unit),
                    definition: async_lock::RwLock::new(definition),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    benchmarks: async_lock::RwLock::new(benchmarks),
                    attributes: async_lock::RwLock::new(attributes),
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
                let mut child_hashes: Vec<String> = bm.iter().map(crate::meta::Benchmark::compute_entity_hash).collect();
                child_hashes.extend(av.iter().map(crate::meta::Attribute::compute_entity_hash));
                child_hashes.sort();
                crate::hash::merkle_node_str(&["semio:meta:Quality", self.id.as_str(), k.as_str(), v.as_str(), u.as_str(), def.as_str(), desc.as_str(), ic.as_str()], child_hashes)
            }
        }

        impl Default for Quality {
            fn default() -> Self {
                Self {
                    id: crate::id::Id::default(),
                    owner: async_lock::RwLock::new(crate::meta::QualityOwnerSlot::default()),
                    key: async_lock::RwLock::new(String::new()),
                    value: async_lock::RwLock::new(None),
                    unit: async_lock::RwLock::new(None),
                    definition: async_lock::RwLock::new(None),
                    description: async_lock::RwLock::new(None),
                    icon: async_lock::RwLock::new(None),
                    benchmarks: async_lock::RwLock::new(Vec::new()),
                    attributes: async_lock::RwLock::new(Vec::new()),
                }
            }
        }
    };
}

//#endregion 🧬 entity_dsl

//#region 🆔 id

pub mod id {
    //! 🆔 Immutable uuid-v7 wrapper used by every entity.
    use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
    use std::fmt;

    /// @emoji 🆔 Opaque node identifier (uuidv7 string); GraphQL wire name `ID` per target schema.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
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

    /// @emoji 🆔 Wire name `ID` matches relay + [`semio/graphql/target.schema.graphql`](../../graphql/target.schema.graphql) `scalar`/Node ids.
    #[Scalar(name = "ID")]
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

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    use thiserror::Error;

    #[derive(Clone, Debug, Error, SimpleObject)]
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
    //! 📐 Geometry: wire [`VectorInput`], [`PositionInput`], … for GraphQL kit inputs; canonical live weak entities live in [`entity`] as `Arc` graph nodes with one Rust kind per SDL weak entity.
    use async_graphql::InputObject;

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "VectorInput")]
    pub struct VectorInput {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "PointInput")]
    pub struct PointInput {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "CoordinateInput")]
    pub struct CoordinateInput {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "OffsetInput")]
    pub struct OffsetInput {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, InputObject)]
    #[graphql(name = "PlaneInput")]
    pub struct PlaneInput {
        pub origin: PointInput,
        #[graphql(name = "xAxis")]
        pub x_axis: VectorInput,
        #[graphql(name = "yAxis")]
        pub y_axis: VectorInput,
    }

    impl Default for PlaneInput {
        /// @emoji ◭ World XY plane through origin; hydrates kit JSON that omits plane axes.
        fn default() -> Self {
            Self { origin: PointInput::default(), x_axis: VectorInput { x: 1.0, y: 0.0, z: 0.0 }, y_axis: VectorInput { x: 0.0, y: 1.0, z: 0.0 } }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, InputObject)]
    #[graphql(name = "PositionInput")]
    pub struct PositionInput {
        pub center: CoordinateInput,
        pub plane: PlaneInput,
    }

    impl Default for PositionInput {
        fn default() -> Self {
            Self { center: CoordinateInput::default(), plane: PlaneInput::default() }
        }
    }

    /// @emoji 🌍 Wire `LocationInput` (lon/lat/alt) for [`entity::Location`].
    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "LocationInput")]
    pub struct LocationInput {
        pub longitude: f64,
        pub latitude: f64,
        pub altitude: f64,
    }

    //#region 📐 entity
    pub mod entity {
        //! 📐 `Arc` geometry nodes (target WeakEntity / Entity graph shapes); `#[Object]` impls live after [`crate::interface`].
        use std::sync::Arc;

        use async_lock::RwLock;

        use crate::hash::{h, merkle_node_str};
        use crate::id::Id;

        use super::{CoordinateInput, PlaneInput, PointInput, PositionInput, VectorInput};

        fn weak(prefix: &str, parts: &[&str]) -> Id {
            Id::from(format!("semio:weak:{prefix}:{}", h(parts)))
        }

        /// @emoji 📍 Canonical weak `Coordinate` (live u/v under `RwLock`).
        #[derive(Debug)]
        pub struct Coordinate {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl Coordinate {
            pub fn from_input(c: CoordinateInput) -> Arc<Self> {
                let id = weak("coordinate", &[&format!("{:.9}", c.u), &format!("{:.9}", c.v)]);
                Arc::new(Self { id, u: RwLock::new(c.u), v: RwLock::new(c.v) })
            }

            /// @emoji 🪪 Merkle leaf: id + live u/v (matches [`super::CoordinateInput`] payload).
            pub async fn compute_hash(&self) -> String {
                let u = *self.u.read().await;
                let v = *self.v.read().await;
                merkle_node_str(&["semio:geom:Coordinate", self.id.as_str(), &format!("{u:.9}"), &format!("{v:.9}")], Vec::new())
            }
        }

        /// @emoji ↗ Canonical weak `Vector`.
        #[derive(Debug)]
        pub struct Vector {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl Vector {
            pub fn from_input(v: VectorInput) -> Arc<Self> {
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

        /// @emoji ◆ Canonical weak `Point`.
        #[derive(Debug)]
        pub struct Point {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl Point {
            pub fn from_input(p: PointInput) -> Arc<Self> {
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

        /// @emoji ▭ Canonical weak `Plane` (owns origin + axes).
        #[derive(Debug)]
        pub struct Plane {
            pub id: Id,
            pub origin: Arc<Point>,
            pub x_axis: Arc<Vector>,
            pub y_axis: Arc<Vector>,
        }

        impl Plane {
            pub fn from_input(pl: PlaneInput) -> Arc<Self> {
                let origin = Point::from_input(pl.origin);
                let x_axis = Vector::from_input(pl.x_axis);
                let y_axis = Vector::from_input(pl.y_axis);
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

        /// @emoji ↖ Canonical weak `Offset` (piece drag input echo).
        #[derive(Debug)]
        pub struct Offset {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl Offset {
            pub fn from_input(o: super::OffsetInput) -> Arc<Self> {
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

        /// @emoji ⌖ Canonical weak `Position` (center + plane); live state only in child locks.
        #[derive(Debug)]
        pub struct Position {
            pub id: Id,
            pub center: Arc<Coordinate>,
            pub plane: Arc<Plane>,
        }

        impl Position {
            pub fn from_position_input(value: PositionInput) -> Arc<Self> {
                let center = Coordinate::from_input(value.center);
                let plane = Plane::from_input(value.plane);
                let id = weak("position", &[center.id.as_str(), plane.id.as_str()]);
                Arc::new(Self { id, center, plane })
            }

            /// @emoji 📸 Wire [`super::PositionInput`] from live center + plane child locks (single source of truth).
            pub async fn snapshot_input(&self) -> PositionInput {
                let u = *self.center.u.read().await;
                let v = *self.center.v.read().await;
                let ox = *self.plane.origin.x.read().await;
                let oy = *self.plane.origin.y.read().await;
                let oz = *self.plane.origin.z.read().await;
                let xx = *self.plane.x_axis.x.read().await;
                let xy = *self.plane.x_axis.y.read().await;
                let xz = *self.plane.x_axis.z.read().await;
                let yx = *self.plane.y_axis.x.read().await;
                let yy = *self.plane.y_axis.y.read().await;
                let yz = *self.plane.y_axis.z.read().await;
                PositionInput { center: CoordinateInput { u, v }, plane: PlaneInput { origin: PointInput { x: ox, y: oy, z: oz }, x_axis: VectorInput { x: xx, y: xy, z: xz }, y_axis: VectorInput { x: yx, y: yy, z: yz } } }
            }

            /// @emoji 🪪 Merkle node: live position scalars plus sorted digests of center + plane arcs.
            pub async fn compute_hash(&self) -> String {
                let p = self.snapshot_input().await;
                let flat = format!(
                    "{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}",
                    p.center.u, p.center.v, p.plane.origin.x, p.plane.origin.y, p.plane.origin.z, p.plane.x_axis.x, p.plane.x_axis.y, p.plane.x_axis.z, p.plane.y_axis.x, p.plane.y_axis.y, p.plane.y_axis.z,
                );
                let mut ch = vec![self.center.compute_hash().await, self.plane.compute_hash().await];
                ch.sort();
                merkle_node_str(&["semio:geom:Position", self.id.as_str(), flat.as_str()], ch)
            }
        }

        /// @emoji 🌍 Canonical weak `Location` (lon/lat/alt).
        #[derive(Debug)]
        pub struct Location {
            pub id: Id,
            pub longitude: RwLock<f64>,
            pub latitude: RwLock<f64>,
            pub altitude: RwLock<f64>,
        }

        impl Location {
            pub fn from_location_input(loc: super::LocationInput) -> Arc<Self> {
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

        /// @emoji 🧭 Placeholder shell for `Place` (full meta wiring lands with meta lift).
        #[derive(Debug)]
        pub struct Place {
            pub id: Id,
            pub label: RwLock<Option<String>>,
        }

        impl Place {
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
        impl Default for Coordinate {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for Vector {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for Point {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for Plane {
            fn default() -> Self {
                Self { id: Id::default(), origin: Arc::new(Point::default()), x_axis: Arc::new(Vector::default()), y_axis: Arc::new(Vector::default()) }
            }
        }

        impl Default for Offset {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for Position {
            fn default() -> Self {
                Self { id: Id::default(), center: Arc::new(Coordinate::default()), plane: Arc::new(Plane::default()) }
            }
        }

        impl Default for Location {
            fn default() -> Self {
                Self { id: Id::default(), longitude: RwLock::new(0.0), latitude: RwLock::new(0.0), altitude: RwLock::new(0.0) }
            }
        }

        impl Default for Place {
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

    pub(crate) fn edge_cursor(i: usize) -> String {
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

    crate::entity_relay!(DesignConnection, DesignEdge, Arc<Design>);
    impl DesignConnection {
        pub async fn from_designs(entities: Vec<Arc<Design>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(PieceConnection, PieceEdge, Arc<Piece>);
    impl PieceConnection {
        pub async fn from_pieces(entities: Vec<Arc<Piece>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(TypeConnection, TypeEdge, Arc<Type>);
    impl TypeConnection {
        pub async fn from_types(entities: Vec<Arc<Type>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(ConnectorConnection, ConnectorEdge, Arc<Connector>);
    impl ConnectorConnection {
        pub async fn from_connectors(entities: Vec<Arc<Connector>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(RepresentationConnection, RepresentationEdge, Arc<Representation>);
    impl RepresentationConnection {
        pub async fn from_representations(entities: Vec<Arc<Representation>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(SideConnection, SideEdge, Arc<Side>);
    impl SideConnection {
        pub async fn from_sides(entities: Vec<Arc<Side>>) -> Self {
            Self::from_entities(entities).await
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
        pub async fn from_blueprints(entities: Vec<crate::kit::r#type::Blueprint>) -> Self {
            let mut child_hashes = Vec::with_capacity(entities.len());
            for b in &entities {
                let h = match b {
                    crate::kit::r#type::Blueprint::Type(t) => t.compute_hash().await,
                    crate::kit::r#type::Blueprint::Design(d) => d.compute_hash().await,
                };
                child_hashes.push(h);
            }
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, node)| BlueprintEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    crate::entity_relay!(ConflictConnection, ConflictEdge, Arc<Conflict>);
    impl ConflictConnection {
        pub async fn from_conflicts(entities: Vec<Arc<Conflict>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(AlternativeConnection, AlternativeEdge, Arc<Alternative>);
    impl AlternativeConnection {
        pub async fn from_alternatives(entities: Vec<Arc<Alternative>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(ChangeConnection, ChangeEdge, Arc<Change>);
    impl ChangeConnection {
        pub async fn from_changes(entities: Vec<Arc<Change>>) -> Self {
            Self::from_entities(entities).await
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    crate::entity_relay!(EditConnection, EditEdge, Arc<crate::vcs::Edit>);
    impl EditConnection {
        pub async fn from_edits(entities: Vec<Arc<crate::vcs::Edit>>) -> Self {
            Self::from_entities(entities).await
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationEdge {
        pub cursor: String,
        pub node: Arc<crate::operation::OperationInterface>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationConnection {
        pub edges: Vec<OperationEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl OperationConnection {
        pub fn from_interface_entities(entities: Vec<Arc<crate::operation::OperationInterface>>) -> Self {
            let child_hashes: Vec<String> = entities.iter().map(|o| h(&[o.entity_id().as_str()])).collect();
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, o)| OperationEdge { cursor: edge_cursor(i), node: o }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    crate::entity_relay!(CheckpointConnection, CheckpointEdge, Arc<Checkpoint>);
    impl CheckpointConnection {
        pub async fn from_checkpoints(entities: Vec<Arc<Checkpoint>>) -> Self {
            Self::from_entities(entities).await
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
        pub async fn from_connections(entities: Vec<Arc<crate::kit::design::connection::Connection>>) -> Self {
            let mut child_hashes = Vec::with_capacity(entities.len());
            for c in &entities {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, node)| ConnectionEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    crate::entity_relay_sync!(FileConnection, FileEdge, File, |f: &File| f.compute_entity_hash());
    crate::entity_relay_sync!(FolderConnection, FolderEdge, Folder, |f: &Folder| f.compute_entity_hash());
    crate::entity_relay_sync!(AuthorConnection, AuthorEdge, Author, |a: &Author| a.compute_entity_hash());
    crate::entity_relay!(ConceptConnection, ConceptEdge, std::sync::Arc<Concept>);
    crate::entity_relay!(TagConnection, TagEdge, std::sync::Arc<Tag>);
    crate::entity_relay!(QualityConnection, QualityEdge, std::sync::Arc<Quality>);
    crate::entity_relay!(PortConnection, PortEdge, std::sync::Arc<crate::kit::r#type::Port>);
    crate::entity_relay!(PlaceConnection, PlaceEdge, std::sync::Arc<crate::geom::entity::Place>);
    crate::entity_relay_sync!(BenchmarkConnection, BenchmarkEdge, Benchmark, |b: &Benchmark| b.compute_entity_hash());
    crate::entity_relay_sync!(PropConnection, PropEdge, Prop, |p: &Prop| p.compute_entity_hash());
    crate::entity_relay_sync!(AttributeConnection, AttributeEdge, crate::meta::Attribute, |a: &crate::meta::Attribute| a.compute_entity_hash());
    crate::entity_relay_sync!(StatConnection, StatEdge, Stat, |s: &Stat| s.compute_entity_hash());
    crate::entity_relay_sync!(LayerConnection, LayerEdge, Layer, |l: &Layer| l.compute_entity_hash());
    crate::entity_relay_sync!(GroupConnection, GroupEdge, Group, |g: &Group| g.compute_entity_hash());

    crate::entity_full_family!(Vector, Arc<crate::geom::entity::Vector>, relay = (VectorConnection, VectorEdge));
    crate::entity_full_family!(Point, Arc<crate::geom::entity::Point>, relay = (PointConnection, PointEdge));
    crate::entity_full_family!(Coordinate, Arc<crate::geom::entity::Coordinate>, relay = (CoordinateConnection, CoordinateEdge));
    crate::entity_full_family!(Offset, Arc<crate::geom::entity::Offset>, relay = (OffsetConnection, OffsetEdge));
    crate::entity_full_family!(Plane, Arc<crate::geom::entity::Plane>, relay = (PlaneConnection, PlaneEdge));
    crate::entity_full_family!(Position, Arc<crate::geom::entity::Position>, relay = (PositionConnection, PositionEdge));
    crate::entity_full_family!(Location, Arc<crate::geom::entity::Location>, relay = (LocationConnection, LocationEdge));

    crate::entity_family! {
        /// @emoji 🧷 Kit [`Family`] SDL shell — Artifact [`name`]/[`description`]/[`icon`] are persisted kit fields.
        pub struct Family {
            pub id: Id,
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &["semio:meta:Family", this.id.as_str(), this.name.as_str(), this.description.as_deref().unwrap_or(""), this.icon.as_deref().unwrap_or("")],
                Vec::new(),
            )
        }
    }

    crate::entity_relay_sync!(FamilyConnection, FamilyEdge, Family, |f: &Family| f.compute_entity_hash());
}

//#endregion 🪢 gql_relay

//#region 🏷️ meta

pub mod meta {
    //! 🏷️ Metadata: DTO [`SimpleObject`] shells plus Arc-backed [`Tag`]/[`Concept`]/[`Quality`] entities (SDL `Entity`).
    use std::sync::Weak;

    use async_graphql::Object;

    use crate::id::Id;
    use crate::timestamp::Timestamp;

    //#region 🧾 graphql inputs
    crate::entity_input! {
        /// @emoji 🧾 SDL `AttributeInput` — instantiates [`Attribute`] entities on entity create/update paths.
        pub struct AttributeInput as "AttributeInput" {
            pub key: String,
            pub value: Option<String>,
            pub definition: Option<String>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾 SDL `TagInput`.
        pub struct TagInput as "TagInput" {
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub order: Option<i32>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾 SDL `ConceptInput`.
        pub struct ConceptInput as "ConceptInput" {
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub order: Option<i32>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾 SDL `QualityInput` (subset aligned to persisted kit fields).
        pub struct QualityInput as "QualityInput" {
            pub key: String,
            pub value: Option<String>,
            pub unit: Option<String>,
            pub definition: Option<String>,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
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

    /// @emoji ➕ Expand optional GraphQL attribute entities into minted [`Attribute`] entities.
    pub async fn attributes_from_inputs(inp: Option<Vec<AttributeInput>>) -> Vec<Attribute> {
        let mut v = Vec::new();
        for a in inp.into_iter().flatten() {
            v.push(a.into_attribute().await);
        }
        v
    }

    /// @emoji 🪪 Rebuild optional GraphQL attribute entities using the ids already recorded in operation scope.
    pub fn attributes_from_inputs_with_ids(inp: Option<Vec<AttributeInput>>, ids: &[Id]) -> Result<Vec<Attribute>, crate::error::SemioError> {
        let attrs = inp.unwrap_or_default();
        if attrs.len() != ids.len() {
            return Err(crate::error::SemioError::invalid(format!("attribute id count mismatch: expected {}, got {}", attrs.len(), ids.len())));
        }
        Ok(attrs.into_iter().zip(ids.iter().cloned()).map(|(attr, id)| attr.into_attribute_with_id(id)).collect())
    }

    crate::entity_family! {
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
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "semio:meta:File",
                    this.id.as_str(),
                    this.url.as_str(),
                    this.mime.as_deref().unwrap_or(""),
                    &this.size.map(|sz| sz.to_string()).unwrap_or_default(),
                    this.hash.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.created.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                    this.updated.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                ],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn tag(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<Tag>> {
                None
            }
            pub async fn quality(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<Quality>> {
                None
            }
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
        )
    }

    crate::entity_family! {
        pub struct Folder {
            pub id: Id,
            pub path: String,
            pub description: Option<String>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(&["semio:meta:Folder", this.id.as_str(), this.path.as_str(), this.description.as_deref().unwrap_or("")], Vec::new())
        }
        , extra = (
            pub async fn file(&self, #[graphql(name = "id")] _id: Id) -> Option<File> {
                None
            }
            #[graphql(name = "subFolder")]
            pub async fn sub_folder(&self, #[graphql(name = "id")] _id: Id) -> Option<Folder> {
                None
            }
            pub async fn family(&self, #[graphql(name = "id")] _id: Id) -> Option<crate::gql_relay::Family> {
                None
            }
            #[graphql(name = "type")]
            pub async fn type_(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
                None
            }
            pub async fn design(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<crate::kit::design::Design>> {
                None
            }
        )
    }

    crate::entity_family! {
        pub struct Author {
            pub id: Id,
            pub name: String,
            pub email: String,
            pub role: Option<String>,
            pub rank: Option<i32>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "semio:meta:Author",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.email.as_str(),
                    this.role.as_deref().unwrap_or(""),
                    &this.rank.map(|r| r.to_string()).unwrap_or_default(),
                ],
                Vec::new(),
            )
        }
    }

    #[derive(Clone, Debug, Default)]
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

    crate::entity_family! {
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
        hash = |this| {
            let min = this.min.map(|v| format!("{v:.9}")).unwrap_or_default();
            let max = this.max.map(|v| format!("{v:.9}")).unwrap_or_default();
            let minx = this.min_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let maxx = this.max_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(&["semio:meta:Benchmark", this.id.as_str(), this.name.as_str(), min.as_str(), max.as_str(), minx, maxx], Vec::new())
        }
    }

    crate::entity_family! {
        pub struct Prop {
            pub id: Id,
            pub key: String,
            pub value: String,
            pub unit: Option<String>,
            #[graphql(skip)]
            pub quality: Option<std::sync::Arc<Quality>>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(&["semio:meta:Prop", this.id.as_str(), this.key.as_str(), this.value.as_str(), this.unit.as_deref().unwrap_or("")], Vec::new())
        }
        , extra = (
            /// @emoji 🔎 SDL `Prop.attribute(id)` — props carry no attribute bag yet; reserved for kit snapshots.
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
        )
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

    crate::meta_arc_titled_entity!(Tag, TagOwnerSlot, "semio:meta:Tag");

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

    crate::meta_arc_titled_entity!(Concept, ConceptOwnerSlot, "semio:meta:Concept");

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

    crate::meta_quality_entity!();

    crate::entity_family! {
        pub struct Stat {
            pub id: Id,
            pub key: String,
            pub value: String,
            pub unit: Option<String>,
            pub description: Option<String>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &["semio:meta:Stat", this.id.as_str(), this.key.as_str(), this.value.as_str(), this.unit.as_deref().unwrap_or(""), this.description.as_deref().unwrap_or("")],
                Vec::new(),
            )
        }
        , extra = (
            /// @emoji 🔎 SDL `Stat.attribute(id)` — stats carry no attribute bag yet; reserved for kit snapshots.
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
        )
    }

    crate::entity_family! {
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
        hash = |this| {
            let vis = this.visible.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let lck = this.locked.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(
                &[
                    "semio:meta:Layer",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.icon.as_str(),
                    this.color.as_deref().unwrap_or(""),
                    &this.order.map(|o| o.to_string()).unwrap_or_default(),
                    vis,
                    lck,
                ],
                Vec::new(),
            )
        }
    }

    crate::entity_family! {
        pub struct Group {
            pub id: Id,
            pub name: String,
            pub description: Option<String>,
            pub color: Option<String>,
            pub icon: Option<String>,
            #[graphql(skip)]
            pub piece_ids: Vec<Id>,
        }
        hash = |this| {
            let mut ids: Vec<String> = this.piece_ids.iter().map(|i| i.as_str().to_string()).collect();
            ids.sort();
            let joined = ids.join("\x1e");
            crate::hash::merkle_node_str(
                &["semio:meta:Group", this.id.as_str(), this.name.as_str(), this.description.as_deref().unwrap_or(""), this.color.as_deref().unwrap_or(""), this.icon.as_deref().unwrap_or(""), joined.as_str()],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
            pub async fn piece(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<crate::kit::design::piece::Piece>> {
                None
            }
        )
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

            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<crate::meta::Attribute> {
                None
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
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
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
                crate::gql_relay::TagConnection::from_entities(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }

            pub async fn tag(&self, id: Id) -> Option<Arc<Tag>> {
                self.tags.read().await.iter().find(|t| t.id == id).cloned()
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
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
            pub async fn ports(&self) -> crate::gql_relay::PortConnection {
                crate::gql_relay::PortConnection::from_entities(self.ports.read().await.clone()).await
            }
            pub async fn port(&self, id: Id) -> Option<Arc<Port>> {
                self.refresh_connector_child_weak_maps().await;
                self.port_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
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
                crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
            }
            pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
                crate::gql_relay::ConceptConnection::from_entities(self.concepts.read().await.clone()).await
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_entities(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_entities(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_entities(self.stats.read().await.clone())
            }

            pub async fn author(&self, id: Id) -> Option<Author> {
                self.authors.read().await.iter().find(|a| a.id == id).cloned()
            }

            pub async fn concept(&self, id: Id) -> Option<Arc<Concept>> {
                self.concepts.read().await.iter().find(|c| c.id == id).cloned()
            }

            pub async fn tag(&self, id: Id) -> Option<Arc<Tag>> {
                self.tags.read().await.iter().find(|t| t.id == id).cloned()
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn prop(&self, id: Id) -> Option<Prop> {
                self.props.read().await.iter().find(|p| p.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
            }

            pub async fn stat(&self, id: Id) -> Option<Stat> {
                self.stats.read().await.iter().find(|s| s.id == id).cloned()
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

            use crate::geom::entity::Position as PositionEntity;
            use crate::geom::PositionInput;
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
                pub position: RwLock<Option<Arc<PositionEntity>>>,
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
                pub async fn new_fixed(owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: PositionInput) -> Arc<Self> {
                    let pos_node = PositionEntity::from_position_input(position);
                    Arc::new(Self { id: Id::new().await, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                /// 🧾 Hydrated workspace piece aligned to external JSON id (facade snapshot hydration).
                pub async fn new_fixed_with_external_id(id: Id, owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: PositionInput) -> Arc<Self> {
                    let pos_node = PositionEntity::from_position_input(position);
                    Arc::new(Self { id, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                pub async fn set_name(&self, name: Option<String>) {
                    *self.name.write().await = name;
                }
                pub async fn set_description(&self, description: Option<String>) {
                    *self.description.write().await = description;
                }
                pub async fn set_position(&self, position: Option<PositionInput>) {
                    let mut g = self.position.write().await;
                    *g = position.map(PositionEntity::from_position_input);
                }

                pub async fn compute_hash(&self) -> String {
                    let name = self.name.read().await;
                    h(&[self.id.as_str(), name.as_deref().unwrap_or("")])
                }

                pub async fn compute_flat_position(&self) -> PositionInput {
                    if let Some(n) = self.position.read().await.as_ref() {
                        return n.snapshot_input().await;
                    }
                    PositionInput::default()
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
                pub async fn position(&self) -> Option<Arc<PositionEntity>> {
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
                pub async fn flat_position(&self) -> Arc<PositionEntity> {
                    if let Some(n) = self.position.read().await.clone() {
                        return n;
                    }
                    PositionEntity::from_position_input(PositionInput::default())
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

                pub async fn prop(&self, id: Id) -> Option<Prop> {
                    self.props.read().await.iter().find(|p| p.id == id).cloned()
                }

                pub async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
                }

                pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                    self.attributes.read().await.iter().find(|a| a.id == id).cloned()
                }

                #[graphql(name = "childConnection")]
                pub async fn child_connection(&self, id: Id) -> Option<Arc<super::connection::Connection>> {
                    self.child_connections.read().await.iter().find(|c| c.id == id).cloned()
                }

                #[graphql(name = "childPiece")]
                pub async fn child_piece(&self, id: Id) -> Option<Arc<Piece>> {
                    self.child_pieces.read().await.iter().find(|p| p.id == id).cloned()
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
                    crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
                }

                pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                    self.attributes.read().await.iter().find(|a| a.id == id).cloned()
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

        use crate::geom::entity::Location;
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
            pub location: RwLock<Option<Arc<Location>>>,
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
            pub async fn location(&self) -> Option<Arc<Location>> {
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
                crate::gql_relay::LayerConnection::from_entities(self.layers.read().await.clone())
            }
            pub async fn groups(&self) -> crate::gql_relay::GroupConnection {
                crate::gql_relay::GroupConnection::from_entities(self.groups.read().await.clone())
            }
            pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_entities(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_entities(self.stats.read().await.clone())
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

        /// @emoji 🧬 Deep-clone this kit graph (dev-backbone `initialKit` projection round-trip) for immutable graph `initialKit` baselines / operation replay.
        pub async fn deep_clone(self: &Arc<Self>) -> Arc<Kit> {
            let snap = crate::kit_backbone::initial_kit_projection_value(self).await;
            let owner = self.owner_graph.clone();
            let nm = self.name.read().await.clone();
            let entity = Kit::new_sync(owner, nm);
            let _ = crate::kit_backbone::hydrate_kit_from_initial_projection_value(&entity, &snap).await;
            entity
        }

        /// @emoji 📦 Single mutation entry: walks canonical [`crate::operation::CanonicalKitDiff`] from [`crate::operation::Operation::to_diff`].
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
            if let Some(t) = &d.types {
                self.apply_types_collection_diff(t).await?;
            }
            if let Some(ds) = &d.designs {
                self.apply_designs_collection_diff(ds).await?;
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
                if *v {
                    return Err(crate::error::SemioError::invalid("kit diff `files` subtree apply not implemented"));
                }
            }
            if let Some(v) = &d.folders {
                if *v {
                    return Err(crate::error::SemioError::invalid("kit diff `folders` subtree apply not implemented"));
                }
            }
            if let Some(v) = &d.authors {
                if *v {
                    return Err(crate::error::SemioError::invalid("kit diff `authors` subtree apply not implemented"));
                }
            }
            self.bump_touch_epoch().await;
            Ok(())
        }

        async fn apply_types_collection_diff(self: &Arc<Self>, t: &crate::operation::TypesCollectionDiff) -> Result<(), crate::error::SemioError> {
            for r in &t.removed {
                let id = r.id.clone();
                let mut tys = self.types.write().await;
                tys.retain(|ty| ty.id != id);
                drop(tys);
                self.type_weak_by_id.write().await.remove(&id);
            }
            for entity in &t.modified {
                let tid = entity.type_ref.id.clone();
                let Some(ty) = self.type_by_external_id(&tid).await else {
                    continue;
                };
                let diff = &entity.diff;
                if let Some(s) = &diff.name {
                    *ty.name.write().await = s.clone();
                }
                if diff.description.is_some() {
                    *ty.description.write().await = diff.description.clone().unwrap_or_default();
                }
                if diff.icon.is_some() {
                    *ty.icon.write().await = diff.icon.clone().unwrap_or_default();
                }
                if diff.image.is_some() {
                    *ty.image.write().await = diff.image.clone().unwrap_or_default();
                }
                if diff.unit.is_some() {
                    *ty.unit.write().await = diff.unit.clone().unwrap_or_default();
                }
            }
            if !t.added.is_empty() {
                return Err(crate::error::SemioError::invalid("kit diff `types.added` apply not implemented"));
            }
            Ok(())
        }

        async fn apply_designs_collection_diff(self: &Arc<Self>, d: &crate::operation::DesignsCollectionDiff) -> Result<(), crate::error::SemioError> {
            for r in &d.removed {
                let id = r.id.clone();
                let mut ds = self.designs.write().await;
                ds.retain(|des| des.id != id);
                drop(ds);
                self.design_weak_by_id.write().await.remove(&id);
            }
            for entity in &d.modified {
                let design_id = entity.design.id.clone();
                let diff = &entity.diff;
                if let Some(design) = self.design_by_external_id(&design_id).await {
                    let sc = &diff.scalars;
                    if let Some(s) = &sc.name {
                        *design.name.write().await = s.clone();
                    }
                    if sc.description.is_some() {
                        *design.description.write().await = sc.description.clone();
                    }
                    if sc.icon.is_some() {
                        *design.icon.write().await = sc.icon.clone();
                    }
                    if sc.image.is_some() {
                        *design.image.write().await = sc.image.clone();
                    }
                }
                if let Some(pc) = &diff.pieces {
                    for pr in &pc.removed {
                        let piece_id = pr.id.clone();
                        let design = self.design_by_external_id(&design_id).await.ok_or_else(|| crate::error::SemioError::not_found("Design", design_id.as_str()))?;
                        design.delete_piece_by_external_id(&piece_id).await?;
                    }
                    for piece_row in &pc.added {
                        self.apply_design_piece_added_row(&design_id, piece_row).await?;
                    }
                    for prow in &pc.modified {
                        self.apply_design_piece_patch(&design_id, &prow.piece.id, &prow.diff).await?;
                    }
                }
            }
            if !d.added.is_empty() {
                return Err(crate::error::SemioError::invalid("kit diff `designs.added` apply not implemented"));
            }
            Ok(())
        }

        async fn apply_design_piece_added_row(self: &Arc<Self>, design_id: &Id, entity: &crate::operation::PieceAddedRow) -> Result<(), crate::error::SemioError> {
            let piece_id = entity.id.clone();
            let blueprint_id = entity.blueprint_id.clone();
            let position = entity.pose;
            let name = entity.name.clone();
            let description = entity.description.clone();
            let (_handle, design) = self.bind_external_design_id(design_id).await;
            let blueprint_type = crate::kit::r#type::Type::new(Arc::downgrade(self), format!("type-{}", blueprint_id.as_str())).await;
            let blueprint = crate::kit::r#type::Blueprint::Type(blueprint_type);
            let piece = crate::kit::design::piece::Piece::new_fixed_with_external_id(piece_id, Arc::downgrade(&design), blueprint, position).await;
            piece.set_name(name).await;
            piece.set_description(description).await;
            let _ = design.insert_piece(piece).await;
            Ok(())
        }

        async fn apply_design_piece_patch(self: &Arc<Self>, design_id: &Id, piece_id: &Id, pdiff: &crate::operation::PiecePatch) -> Result<(), crate::error::SemioError> {
            use crate::geom::entity::Position as GeomPosition;
            let design = self.design_by_external_id(design_id).await.ok_or_else(|| crate::error::SemioError::not_found("Design", design_id.as_str()))?;
            let piece = design.piece_by_external_id(piece_id).await.ok_or_else(|| crate::error::SemioError::not_found("Piece", piece_id.as_str()))?;
            if pdiff.fix_piece {
                *piece.connection_kind.write().await = Some(crate::kit::design::piece::PieceConnectionKind::Fixed);
                return Ok(());
            }
            if let Some(off) = pdiff.drag {
                let du = off.u;
                let dv = off.v;
                let pos_slot = piece.position.read().await.clone();
                if let Some(pos) = pos_slot {
                    let u = *pos.center.u.read().await + du;
                    let v = *pos.center.v.read().await + dv;
                    *pos.center.u.write().await = u;
                    *pos.center.v.write().await = v;
                } else {
                    let n = GeomPosition::from_position_input(crate::geom::PositionInput::default());
                    *n.center.u.write().await = du;
                    *n.center.v.write().await = dv;
                    *piece.position.write().await = Some(n);
                }
                return Ok(());
            }
            if let Some(position) = pdiff.pose {
                let n = GeomPosition::from_position_input(position);
                *piece.position.write().await = Some(n);
                return Ok(());
            }
            if let Some(n) = &pdiff.name {
                piece.set_name(Some(n.clone())).await;
            }
            if pdiff.description.is_some() {
                piece.set_description(pdiff.description.clone()).await;
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
            for entity in &t.added {
                self.apply_create_tag_scoped(&entity.owner_id, &entity.id, &entity.attribute_ids, entity.tag.clone()).await?;
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
            for entity in &c.added {
                self.apply_create_concept_scoped(&entity.owner_id, &entity.id, &entity.attribute_ids, entity.concept.clone()).await?;
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
            for entity in &q.added {
                self.apply_create_quality_scoped(&entity.owner_id, &entity.id, &entity.attribute_ids, &entity.benchmark_ids, entity.quality.clone()).await?;
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
        pub async fn bind_external_design_id(self: &Arc<Self>, design_id: &Id) -> (crate::kit_graph_engine::DesignSlot, Arc<design::Design>) {
            let design = self.ensure_design(design_id).await;
            let slot = {
                let designs = self.designs.read().await;
                designs.iter().position(|d| &d.id == design_id).expect("design slot after ensure_design") as u32
            };
            (crate::kit_graph_engine::DesignSlot(slot), design)
        }

        /// @emoji 🔁 Clears every **layout** node’s placed pieces and piece slot maps so [`crate::kit_backbone`] can replay without duplicating projections; kit metadata and empty layout shells stay resident (detach leaves this graph materialized in memory).
        pub async fn clear_piece_projections_for_backbone_replay(self: &Arc<Self>) {
            let designs = self.designs.read().await;
            for design in designs.iter() {
                design.pieces.write().await.clear();
                design.piece_weak_by_external_id.write().await.clear();
            }
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
            crate::gql_relay::FileConnection::from_entities(self.files.read().await.clone())
        }
        pub async fn folders(&self) -> crate::gql_relay::FolderConnection {
            crate::gql_relay::FolderConnection::from_entities(self.folders.read().await.clone())
        }
        pub async fn families(&self) -> crate::gql_relay::FamilyConnection {
            crate::gql_relay::FamilyConnection::from_entities(Vec::new())
        }
        pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
            crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
        }
        pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
            crate::gql_relay::ConceptConnection::from_entities(self.concepts.read().await.clone()).await
        }
        pub async fn tags(&self) -> crate::gql_relay::TagConnection {
            crate::gql_relay::TagConnection::from_entities(self.tags.read().await.clone()).await
        }
        pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
            crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
        }
        pub async fn props(&self) -> crate::gql_relay::PropConnection {
            crate::gql_relay::PropConnection::from_entities(self.props.read().await.clone())
        }
        pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
            crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
        }
        pub async fn stats(&self) -> crate::gql_relay::StatConnection {
            crate::gql_relay::StatConnection::from_entities(self.stats.read().await.clone())
        }

        pub async fn file(&self, id: Id) -> Option<File> {
            self.files.read().await.iter().find(|f| f.id == id).cloned()
        }

        pub async fn folder(&self, id: Id) -> Option<Folder> {
            self.folders.read().await.iter().find(|f| f.id == id).cloned()
        }

        pub async fn family(&self, id: Id) -> Option<crate::gql_relay::Family> {
            let _ = id;
            None
        }

        pub async fn author(&self, id: Id) -> Option<Author> {
            self.authors.read().await.iter().find(|a| a.id == id).cloned()
        }

        pub async fn concept(&self, id: Id) -> Option<Arc<Concept>> {
            self.concept_by_id.read().await.get(&id).cloned()
        }

        pub async fn tag(&self, id: Id) -> Option<Arc<Tag>> {
            self.tag_by_id.read().await.get(&id).cloned()
        }

        pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
            self.quality_by_id.read().await.get(&id).cloned()
        }

        pub async fn prop(&self, id: Id) -> Option<Prop> {
            self.props.read().await.iter().find(|p| p.id == id).cloned()
        }

        pub async fn attribute(&self, id: Id) -> Option<Attribute> {
            self.attributes.read().await.iter().find(|a| a.id == id).cloned()
        }

        pub async fn stat(&self, id: Id) -> Option<Stat> {
            self.stats.read().await.iter().find(|s| s.id == id).cloned()
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
        crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
    }

    pub async fn attribute(&self, id: crate::id::Id) -> Option<crate::meta::Attribute> {
        self.attributes.read().await.iter().find(|a| a.id == id).cloned()
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
        crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
    }

    pub async fn attribute(&self, id: crate::id::Id) -> Option<crate::meta::Attribute> {
        self.attributes.read().await.iter().find(|a| a.id == id).cloned()
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
        crate::gql_relay::BenchmarkConnection::from_entities(self.benchmarks.read().await.clone())
    }
    pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
        crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
    }

    pub async fn attribute(&self, id: crate::id::Id) -> Option<crate::meta::Attribute> {
        self.attributes.read().await.iter().find(|a| a.id == id).cloned()
    }
}

//#endregion 🏷️ meta graphql

//#region 🌿 vcs

pub mod vcs {
    //! 🌿 Version-control entities — [`Change`](../../graphql/target.schema.graphql), [`Edit`](../../graphql/target.schema.graphql), [`Checkpoint`](../../graphql/target.schema.graphql), [`Alternative`](../../graphql/target.schema.graphql), [`Graph`](../../graphql/target.schema.graphql), [`Session`](../../graphql/target.schema.graphql), [`TheKit`](../../graphql/target.schema.graphql) ([`Workspace`](../../graphql/target.schema.graphql)), [`Conflict`](../../graphql/target.schema.graphql).
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Weak};

    use async_graphql::{Context, InputObject, Object, Union};
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
        /// @emoji 📜 Forward [`operation::Operation`] steps (materialized via `Kit::apply_diff`; persisted `kitDiff` in bundles is derived from each op's `to_diff` at projection time).
        pub forwards: RwLock<Vec<operation::Operation>>,
        /// @emoji 📜 Backward companion operations for explicit undo/redo (same pipeline).
        pub backwards: RwLock<Vec<operation::Operation>>,
    }

    /// 🔗 Weak owner matching SDL `Alternative | Checkpoint` for persisted [`Change`] entities.
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

        /// @emoji 🔗 Ordered  operation record ids constituting the forwards side (bundle `OperationLog` ids) when persisted.
        #[graphql(name = "forwardOperationRecordIds")]
        pub async fn forward__operation_record_ids(&self) -> Vec<Id> {
            Vec::new()
        }

        /// @emoji 🔗 Ordered  operation record ids for backwards / inverse application when persisted separately from `OperationKind`.
        #[graphql(name = "backwardOperationRecordIds")]
        pub async fn backward__operation_record_ids(&self) -> Vec<Id> {
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
    /// @emoji 🔗 [`Edit`] owner: [`TheKit`] lives on [`Graph`]; [`Alternative`] is its own [`Workspace`](../../graphql/target.schema.graphql) (SDL `Workspace`).
    pub enum EditOwner {
        TheKit(Weak<Graph>),
        Alternative(Weak<Alternative>),
    }

    pub struct Edit {
        pub id: Id,
        pub owner: EditOwner,
        pub changes: RwLock<Vec<Arc<Change>>>,
        pub forward_interface_operations: RwLock<Vec<Arc<operation::OperationInterface>>>,
        pub backward_interface_operations: RwLock<Vec<Arc<operation::OperationInterface>>>,
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
                owner: EditOwner::TheKit(Weak::new()),
                changes: RwLock::new(Vec::new()),
                forward_interface_operations: RwLock::new(Vec::new()),
                backward_interface_operations: RwLock::new(Vec::new()),
                sequence_number: RwLock::new(0),
                started_at: RwLock::new(None),
                finished_at: RwLock::new(None),
                description: RwLock::new(String::new()),
                origin: RwLock::new(String::new()),
            }
        }
    }

    impl Edit {
        pub async fn new(owner: EditOwner) -> Arc<Self> {
            Self::with_id(owner, Id::new().await, 0).await
        }
        pub async fn with_id(owner: EditOwner, id: Id, sequence_number: i32) -> Arc<Self> {
            Arc::new(Self {
                id,
                owner,
                changes: RwLock::new(Vec::new()),
                forward_interface_operations: RwLock::new(Vec::new()),
                backward_interface_operations: RwLock::new(Vec::new()),
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

    /// @emoji 🧾 Flatten [`Edit`] rows into target-schema [`Change`](../../graphql/target.schema.graphql) entities for a [`Workspace`](../../graphql/target.schema.graphql).
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
            match &self.owner {
                EditOwner::Alternative(wa) => wa.upgrade().map(EditOwnerUnion::Alternative),
                EditOwner::TheKit(wg) => {
                    let g = wg.upgrade()?;
                    let cp = g.the_kit_parent_checkpoint.read().await.upgrade()?;
                    Some(EditOwnerUnion::Checkpoint(cp))
                }
            }
        }
        pub async fn forwards(&self) -> crate::gql_relay::OperationConnection {
            crate::gql_relay::OperationConnection::from_interface_entities(self.forward_interface_operations.read().await.clone())
        }
        pub async fn backwards(&self) -> crate::gql_relay::OperationConnection {
            crate::gql_relay::OperationConnection::from_interface_entities(self.backward_interface_operations.read().await.clone())
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
        pub workspace_id: Option<Id>,
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
    /// @emoji 📖 Placeholder version entity for `OwnerEntity` (`ReadVersion` in target SDL).
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

    /// @emoji 📖 Placeholder version entity for `OwnerEntity` (`WriteVersion` in target SDL).
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
            crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
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

        pub async fn edits(&self) -> crate::gql_relay::EditConnection {
            crate::gql_relay::EditConnection::empty()
        }

        pub async fn edit(&self, id: Id) -> Option<Arc<Edit>> {
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
        /// @emoji 🧭 SDL [`TheKit`](../../graphql/target.schema.graphql) — [`Workspace`] on [`Graph`] with `savedChanges` / `unsavedChanges` / `kit`.
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
                Some(g) => g.saved_change_connection_for_the_kit().await,
                None => crate::gql_relay::ChangeConnection::empty(),
            }
        }
        #[graphql(name = "unsavedChanges")]
        pub async fn unsaved_changes(&self) -> crate::gql_relay::ChangeConnection {
            match self.graph().await {
                Some(g) => g.unsaved_change_connection_for_the_kit().await,
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
        pub open_edit: RwLock<Weak<Edit>>,
        pub saved_edits: RwLock<Vec<Arc<Edit>>>,
        pub redo_edits: RwLock<Vec<Arc<Edit>>>,
        pub unsaved_edits: RwLock<Vec<Arc<Edit>>>,
        pub change_seq: AtomicU64,
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
                open_edit: RwLock::new(Weak::new()),
                saved_edits: RwLock::new(Vec::new()),
                redo_edits: RwLock::new(Vec::new()),
                unsaved_edits: RwLock::new(Vec::new()),
                change_seq: AtomicU64::new(0),
            }
        }
    }

    impl Alternative {
        pub async fn compute_hash(&self) -> String {
            let name = self.name.read().await;
            h(&[self.id.as_str(), name.as_str()])
        }

        /// @emoji ✏️ Ensure an unsaved [`Edit`] exists for `edit_id` on this [`Alternative`](../../graphql/target.schema.graphql) (SDL `unsavedChanges`).
        pub async fn ensure_unsaved_edit(self: &Arc<Self>, edit_id: &Id) -> Arc<Edit> {
            if let Some(t) = self.unsaved_edits.read().await.iter().find(|t| &t.id == edit_id).cloned() {
                *self.open_edit.write().await = Arc::downgrade(&t);
                return t;
            }
            let seq = self.unsaved_edits.read().await.len() as i32;
            let t = Edit::with_id(EditOwner::Alternative(Arc::downgrade(self)), edit_id.clone(), seq).await;
            self.unsaved_edits.write().await.push(t.clone());
            *self.open_edit.write().await = Arc::downgrade(&t);
            t
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
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(self.saved_edits.read().await.clone()).await).await
        }
        #[graphql(name = "unsavedChanges")]
        pub async fn unsaved_changes(&self) -> crate::gql_relay::ChangeConnection {
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(self.unsaved_edits.read().await.clone()).await).await
        }
        pub async fn kit(&self) -> Arc<Kit> {
            match self.owner_graph.upgrade() {
                Some(g) => g.materialized_kit_for_workspace(&self.id).await,
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

    /// @emoji 📦 Cached materialized [`Kit`] for a [`Workspace`](../../graphql/target.schema.graphql) (`workspace_id` + `change_seq`).
    pub struct MaterializedSlot {
        pub workspace_id: Id,
        pub change_seq: u64,
        pub kit: Arc<Kit>,
    }

    pub struct Graph {
        pub id: Id,
        pub owner_session: RwLock<Weak<Session>>,
        pub self_weak: std::sync::Mutex<std::sync::Weak<Graph>>,
        pub initial_kit: RwLock<Arc<Kit>>,
        /// @emoji 🏗️ Mutable working [`Kit`] used while replaying [`Operation`]s for materialized [`TheKit.kit`](../../graphql/target.schema.graphql) / [`Alternative.kit`](../../graphql/target.schema.graphql).
        pub mutable_kit: RwLock<Arc<Kit>>,
        pub materialized_cache: RwLock<Option<MaterializedSlot>>,
        pub alternatives: RwLock<Vec<Arc<Alternative>>>,
        pub checkpoints: RwLock<Vec<Arc<Checkpoint>>>,
        pub releases: RwLock<Vec<Arc<Checkpoint>>>,
        pub the_kit_parent_checkpoint: RwLock<Weak<Checkpoint>>,
        pub the_kit_open_edit: RwLock<Weak<Edit>>,
        pub the_kit_saved_edits: RwLock<Vec<Arc<Edit>>>,
        pub the_kit_redo_edits: RwLock<Vec<Arc<Edit>>>,
        pub the_kit_unsaved_edits: RwLock<Vec<Arc<Edit>>>,
        pub the_kit_workspace_seq: AtomicU64,
        pub op_history: RwLock<Vec<Arc<crate::operation::OperationInterface>>>,
    }

    impl Default for Graph {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_session: RwLock::new(Weak::new()),
                self_weak: std::sync::Mutex::new(Weak::new()),
                initial_kit: RwLock::new(Arc::default()),
                mutable_kit: RwLock::new(Arc::default()),
                materialized_cache: RwLock::new(None),
                alternatives: RwLock::new(Vec::new()),
                checkpoints: RwLock::new(Vec::new()),
                releases: RwLock::new(Vec::new()),
                the_kit_parent_checkpoint: RwLock::new(Weak::new()),
                the_kit_open_edit: RwLock::new(Weak::new()),
                the_kit_saved_edits: RwLock::new(Vec::new()),
                the_kit_redo_edits: RwLock::new(Vec::new()),
                the_kit_unsaved_edits: RwLock::new(Vec::new()),
                the_kit_workspace_seq: AtomicU64::new(0),
                op_history: RwLock::new(Vec::new()),
            }
        }
    }

    impl Graph {
        /// 🆕 Build a brand-new Graph; seeds [`Graph::mutable_kit`] from a deep-cloned empty [`Kit`] so [`Graph::initial_kit`] baselines never alias live mutation.
        pub async fn new() -> Arc<Self> {
            let id = Id::new().await;
            let g = Arc::new_cyclic(|weak_self: &Weak<Graph>| {
                let kit = crate::kit::Kit::new_sync(weak_self.clone(), "the kit".to_string());
                Self {
                    id,
                    owner_session: RwLock::new(Weak::new()),
                    self_weak: std::sync::Mutex::new(weak_self.clone()),
                    initial_kit: RwLock::new(Arc::default()),
                    mutable_kit: RwLock::new(kit.clone()),
                    materialized_cache: RwLock::new(None),
                    alternatives: RwLock::new(Vec::new()),
                    checkpoints: RwLock::new(Vec::new()),
                    releases: RwLock::new(Vec::new()),
                    the_kit_parent_checkpoint: RwLock::new(Weak::new()),
                    the_kit_open_edit: RwLock::new(Weak::new()),
                    the_kit_saved_edits: RwLock::new(Vec::new()),
                    the_kit_redo_edits: RwLock::new(Vec::new()),
                    the_kit_unsaved_edits: RwLock::new(Vec::new()),
                    the_kit_workspace_seq: AtomicU64::new(0),
                    op_history: RwLock::new(Vec::new()),
                }
            });
            let baseline = g.mutable_kit.read().await.clone().deep_clone().await;
            *g.initial_kit.write().await = baseline.clone();
            *g.mutable_kit.write().await = baseline;
            g
        }

        /// @emoji 🔗 Upgrade `&Graph` to [`Arc`] via the cyclic weak slot (panics if weak is unset).
        pub fn arc_here(&self) -> Arc<Graph> {
            self.self_weak.lock().ok().and_then(|slot| slot.upgrade()).expect("Graph.self_weak upgrade")
        }

        /// @emoji 🪪 Map persisted bundle anchor `the-kit` onto this graph's [`TheKit`](../../graphql/target.schema.graphql) [`Workspace`](../../graphql/target.schema.graphql) id ([`Graph::id`]).
        pub async fn resolve_workspace_id(self: &Arc<Self>, workspace_ref: &Id) -> Id {
            if workspace_ref.as_str() == "the-kit" {
                self.ensure_default_checkpoint_for_the_kit().await;
                self.id.clone()
            } else {
                workspace_ref.clone()
            }
        }

        /// @emoji 📦 [`TheKit.kit`](../../graphql/target.schema.graphql) — materialized [`Kit`] for the graph's [`TheKit`](../../graphql/target.schema.graphql) [`Workspace`](../../graphql/target.schema.graphql).
        pub async fn materialized_head_kit(self: &Arc<Self>) -> Arc<Kit> {
            self.ensure_default_checkpoint_for_the_kit().await;
            self.materialized_kit_for_workspace(&self.id).await
        }

        /// @emoji 📦 Same as [`Graph::materialized_head_kit`] but callable from `&Graph` resolvers.
        pub async fn materialized_head_kit_from_ref(&self) -> Arc<Kit> {
            self.arc_here().materialized_head_kit().await
        }

        /// @emoji 🧊 Invalidate lazily materialized kit cache (abort / record operation).
        pub async fn invalidate_materialized_cache(self: &Arc<Self>) {
            *self.materialized_cache.write().await = None;
        }

        /// @emoji 🧾 SDL `TheKit.savedChanges` — [`ChangeConnection`](../../graphql/target.schema.graphql) for this graph's [`TheKit`](../../graphql/target.schema.graphql).
        pub async fn saved_change_connection_for_the_kit(self: &Arc<Self>) -> crate::gql_relay::ChangeConnection {
            self.ensure_default_checkpoint_for_the_kit().await;
            let txs = self.the_kit_saved_edits.read().await.clone();
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(txs).await).await
        }

        /// @emoji 🧾 SDL `TheKit.unsavedChanges` — [`ChangeConnection`](../../graphql/target.schema.graphql) for this graph's [`TheKit`](../../graphql/target.schema.graphql).
        pub async fn unsaved_change_connection_for_the_kit(self: &Arc<Self>) -> crate::gql_relay::ChangeConnection {
            self.ensure_default_checkpoint_for_the_kit().await;
            let txs = self.the_kit_unsaved_edits.read().await.clone();
            crate::gql_relay::ChangeConnection::from_changes(changes_from_edits(txs).await).await
        }

        /// @emoji 📎 Ordered saved then unsaved [`Edit`] rows for a [`Workspace`](../../graphql/target.schema.graphql) id ([`TheKit`](../../graphql/target.schema.graphql) = [`Graph::id`], [`Alternative`](../../graphql/target.schema.graphql) = [`Alternative::id`]).
        pub async fn workspace_saved_and_unsaved_edits(self: &Arc<Self>, workspace_id: &Id) -> Option<(Vec<Arc<Edit>>, Vec<Arc<Edit>>)> {
            let ws = self.resolve_workspace_id(workspace_id).await;
            if ws == self.id {
                return Some((self.the_kit_saved_edits.read().await.clone(), self.the_kit_unsaved_edits.read().await.clone()));
            }
            for a in self.alternatives.read().await.iter() {
                if a.id == ws {
                    return Some((a.saved_edits.read().await.clone(), a.unsaved_edits.read().await.clone()));
                }
            }
            None
        }

        async fn workspace_is_the_kit(self: &Arc<Self>, workspace_id: &Id) -> bool {
            self.resolve_workspace_id(workspace_id).await == self.id
        }

        async fn workspace_alternative(self: &Arc<Self>, workspace_id: &Id) -> Option<Arc<Alternative>> {
            let ws = self.resolve_workspace_id(workspace_id).await;
            self.alternatives.read().await.iter().find(|a| a.id == ws).cloned()
        }

        /// @emoji ✏️ Ensure an unsaved [`Edit`] exists on [`TheKit`](../../graphql/target.schema.graphql) (SDL `unsavedChanges`).
        pub async fn ensure_the_kit_unsaved_edit(self: &Arc<Self>, edit_id: &Id) -> Arc<Edit> {
            if let Some(t) = self.the_kit_unsaved_edits.read().await.iter().find(|t| &t.id == edit_id).cloned() {
                *self.the_kit_open_edit.write().await = Arc::downgrade(&t);
                return t;
            }
            let seq = self.the_kit_unsaved_edits.read().await.len() as i32;
            let t = Edit::with_id(EditOwner::TheKit(Arc::downgrade(self)), edit_id.clone(), seq).await;
            self.the_kit_unsaved_edits.write().await.push(t.clone());
            *self.the_kit_open_edit.write().await = Arc::downgrade(&t);
            t
        }

        /// @emoji 📦 Deterministic materialized [`Kit`] for a [`Workspace`](../../graphql/target.schema.graphql): clone [`Graph::mutable_kit`] and replay recorded [`Operation`] forwards (matches SDL `Workspace.kit` computation).
        pub async fn materialized_kit_for_workspace(self: &Arc<Self>, workspace_id: &Id) -> Arc<Kit> {
            let ws = self.resolve_workspace_id(workspace_id).await;
            let (saved, unsaved, seq) = if ws == self.id {
                (
                    self.the_kit_saved_edits.read().await.clone(),
                    self.the_kit_unsaved_edits.read().await.clone(),
                    self.the_kit_workspace_seq.load(Ordering::Relaxed),
                )
            } else if let Some(a) = self.workspace_alternative(&ws).await {
                (
                    a.saved_edits.read().await.clone(),
                    a.unsaved_edits.read().await.clone(),
                    a.change_seq.load(Ordering::Relaxed),
                )
            } else {
                return self.mutable_kit.read().await.clone();
            };
            {
                let cache = self.materialized_cache.read().await;
                if let Some(slot) = cache.as_ref() {
                    if slot.workspace_id == ws && slot.change_seq == seq {
                        return slot.kit.clone();
                    }
                }
            }
            let base = self.mutable_kit.read().await.clone();
            let mat = base.deep_clone().await;
            let mut edits: Vec<Arc<Edit>> = Vec::new();
            edits.extend(saved);
            edits.extend(unsaved);
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
            *self.materialized_cache.write().await = Some(MaterializedSlot { workspace_id: ws.clone(), change_seq: seq, kit: mat.clone() });
            mat
        }

        /// @emoji 📦 [`Kit`] materialized for `workspace_id` through `target_edit` / `change_idx` / `forward_idx` (used when persisting each operation's `kitDiff` beside its input).
        pub async fn kit_materialized_for_workspace_before_operation_step(
            self: &Arc<Self>,
            workspace_id: &Id,
            target_edit: &Arc<Edit>,
            change_idx: usize,
            forward_idx: usize,
        ) -> Arc<Kit> {
            let (saved, unsaved) = self.workspace_saved_and_unsaved_edits(workspace_id).await.unwrap_or_else(|| (Vec::new(), Vec::new()));
            let base = self.mutable_kit.read().await.clone();
            let mat = base.deep_clone().await;
            let mut edits: Vec<Arc<Edit>> = Vec::new();
            edits.extend(saved);
            edits.extend(unsaved);
            for ed in edits {
                let same_ed = Arc::ptr_eq(&ed, target_edit);
                let changes = ed.changes.read().await.clone();
                for (ci, c_arc) in changes.iter().enumerate() {
                    let forwards = c_arc.forwards.read().await.clone();
                    if same_ed && ci == change_idx {
                        for (fi, op) in forwards.into_iter().enumerate() {
                            if fi >= forward_idx {
                                return mat;
                            }
                            if let Ok(d) = op.to_diff(&mat).await {
                                let _ = mat.apply_diff(&d).await;
                            }
                        }
                        return mat;
                    }
                    for op in forwards {
                        if let Ok(d) = op.to_diff(&mat).await {
                            let _ = mat.apply_diff(&d).await;
                        }
                    }
                }
            }
            mat
        }

        /// @emoji 📝 Append one forward operation plus backward operations onto the open [`Edit`]'s tail [`Change`], bumping the workspace cache epoch.
        pub async fn record_operation_in_open_transaction(
            self: &Arc<Self>,
            workspace_id: &Id,
            edit_id: &Id,
            forward: crate::operation::Operation,
            backwards: Vec<crate::operation::Operation>,
        ) -> Result<(), SemioError> {
            let ws = self.resolve_workspace_id(workspace_id).await;
            let tx = if ws == self.id {
                let _ = self.ensure_the_kit_unsaved_edit(edit_id).await;
                self.the_kit_unsaved_edits.read().await.iter().find(|t| &t.id == edit_id).cloned().ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?
            } else if let Some(alt) = self.workspace_alternative(&ws).await {
                let _ = alt.ensure_unsaved_edit(edit_id).await;
                alt.unsaved_edits.read().await.iter().find(|t| &t.id == edit_id).cloned().ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?
            } else {
                return Err(SemioError::not_found("Workspace", workspace_id.as_str()));
            };
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
            let change_owner = match &tx.owner {
                EditOwner::Alternative(wa) => wa.upgrade().map(|a| ChangeOwnerRef::Alternative(Arc::downgrade(&a))),
                EditOwner::TheKit(wg) => match wg.upgrade() {
                    Some(gr) => gr.the_kit_parent_checkpoint.read().await.upgrade().map(|cp| ChangeOwnerRef::Checkpoint(Arc::downgrade(&cp))),
                    None => None,
                },
            };
            *change.owner.write().await = change_owner;
            change.forwards.write().await.push(forward);
            change.backwards.write().await.extend(backwards);
            if ws == self.id {
                self.the_kit_workspace_seq.fetch_add(1, Ordering::Relaxed);
            } else if let Some(alt) = self.workspace_alternative(&ws).await {
                alt.change_seq.fetch_add(1, Ordering::Relaxed);
            }
            self.invalidate_materialized_cache().await;
            Ok(())
        }

        /// @emoji 🌱 Ensure a seed [`Checkpoint`] exists and [`TheKit`](../../graphql/target.schema.graphql) is anchored (idempotent).
        pub async fn ensure_default_checkpoint_for_the_kit(self: &Arc<Self>) {
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
            let mut anch = self.the_kit_parent_checkpoint.write().await;
            if anch.upgrade().is_none() {
                *anch = Arc::downgrade(&checkpoint);
            }
        }

        /// @emoji 🌱 Fork a new named [`Alternative`](../../graphql/target.schema.graphql) from [`TheKit`](../../graphql/target.schema.graphql) or another alternative's tip [`Checkpoint`](../../graphql/target.schema.graphql).
        pub async fn create_alternative_from_tip(self: &Arc<Self>, name: String, source_alternative_id: Option<&Id>) -> Result<Id, SemioError> {
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(SemioError::invalid("alternative name required"));
            }

            self.ensure_default_checkpoint_for_the_kit().await;

            let parent_cp = match source_alternative_id {
                None => self.the_kit_parent_checkpoint.read().await.upgrade().ok_or_else(|| SemioError::invalid("theKit workspace has no parent checkpoint"))?,
                Some(aid) => {
                    let alt = {
                        let alts = self.alternatives.read().await;
                        alts.iter().find(|a| &a.id == aid).ok_or_else(|| SemioError::not_found("Alternative", aid.as_str()))?.clone()
                    };
                    let parent_from_alt = {
                        let start_guard = alt.start.read().await;
                        start_guard.upgrade().ok_or_else(|| SemioError::invalid("alternative has no start checkpoint"))?
                    };
                    parent_from_alt
                }
            };

            let parent_initial_kit = self.initial_kit.read().await.clone();

            let new_alt_id = Id::new().await;
            let new_alt = Arc::new(Alternative {
                id: new_alt_id.clone(),
                owner_graph: Arc::downgrade(self),
                name: RwLock::new(name),
                start: RwLock::new(Arc::downgrade(&parent_cp)),
                checkpoints: RwLock::new(vec![parent_cp.clone()]),
                kit: RwLock::new(Some(parent_initial_kit)),
                open_edit: RwLock::new(Weak::new()),
                saved_edits: RwLock::new(Vec::new()),
                redo_edits: RwLock::new(Vec::new()),
                unsaved_edits: RwLock::new(Vec::new()),
                change_seq: AtomicU64::new(0),
            });

            self.alternatives.write().await.push(new_alt);

            Ok(new_alt_id)
        }

        /// @emoji 🟢 Open a new unsaved [`Edit`] on `workspace_id` (SDL `unsavedChanges`).
        pub async fn open_transaction(self: &Arc<Self>, workspace_id: &Id) -> Arc<Edit> {
            let tx_id = Id::new().await;
            if self.workspace_is_the_kit(workspace_id).await {
                let _ = self.ensure_default_checkpoint_for_the_kit().await;
                let seq = self.the_kit_unsaved_edits.read().await.len() as i32;
                let tx = Edit::with_id(EditOwner::TheKit(Arc::downgrade(self)), tx_id, seq).await;
                self.the_kit_unsaved_edits.write().await.push(tx.clone());
                *self.the_kit_open_edit.write().await = Arc::downgrade(&tx);
                return tx;
            }
            let alt = self.workspace_alternative(workspace_id).await.expect("Workspace not found for open_transaction");
            let seq = alt.unsaved_edits.read().await.len() as i32;
            let tx = Edit::with_id(EditOwner::Alternative(Arc::downgrade(&alt)), tx_id, seq).await;
            alt.unsaved_edits.write().await.push(tx.clone());
            *alt.open_edit.write().await = Arc::downgrade(&tx);
            tx
        }

        /// @emoji ✅ Commit an unsaved [`Edit`]: move it from `unsavedChanges` to `savedChanges` on that [`Workspace`](../../graphql/target.schema.graphql).
        pub async fn commit_transaction(self: &Arc<Self>, workspace_id: &Id, edit_id: &Id) -> Result<(), SemioError> {
            if self.workspace_is_the_kit(workspace_id).await {
                let tx = {
                    let mut txs = self.the_kit_unsaved_edits.write().await;
                    let pos = txs.iter().position(|t| &t.id == edit_id).ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?;
                    txs.remove(pos)
                };
                self.the_kit_saved_edits.write().await.push(tx);
                let open = self.the_kit_open_edit.read().await.upgrade();
                if let Some(open_tx) = open {
                    if &open_tx.id == edit_id {
                        *self.the_kit_open_edit.write().await = std::sync::Weak::new();
                    }
                }
                return Ok(());
            }
            let alt = self.workspace_alternative(workspace_id).await.ok_or_else(|| SemioError::not_found("Workspace", workspace_id.as_str()))?;
            let tx = {
                let mut txs = alt.unsaved_edits.write().await;
                let pos = txs.iter().position(|t| &t.id == edit_id).ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?;
                txs.remove(pos)
            };
            alt.saved_edits.write().await.push(tx);
            let open = alt.open_edit.read().await.upgrade();
            if let Some(open_tx) = open {
                if &open_tx.id == edit_id {
                    *alt.open_edit.write().await = std::sync::Weak::new();
                }
            }
            Ok(())
        }

        /// @emoji ⛔ Drop an unsaved [`Edit`] from `unsavedChanges` on that [`Workspace`](../../graphql/target.schema.graphql).
        pub async fn abort_transaction(self: &Arc<Self>, workspace_id: &Id, edit_id: &Id) -> Result<(), SemioError> {
            if self.workspace_is_the_kit(workspace_id).await {
                {
                    let mut txs = self.the_kit_unsaved_edits.write().await;
                    let pos = txs.iter().position(|t| &t.id == edit_id).ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?;
                    txs.remove(pos);
                }
                let open = self.the_kit_open_edit.read().await.upgrade();
                if let Some(open_tx) = open {
                    if &open_tx.id == edit_id {
                        *self.the_kit_open_edit.write().await = std::sync::Weak::new();
                    }
                }
                self.the_kit_workspace_seq.fetch_add(1, Ordering::Relaxed);
                self.invalidate_materialized_cache().await;
                return Ok(());
            }
            let alt = self.workspace_alternative(workspace_id).await.ok_or_else(|| SemioError::not_found("Workspace", workspace_id.as_str()))?;
            {
                let mut txs = alt.unsaved_edits.write().await;
                let pos = txs.iter().position(|t| &t.id == edit_id).ok_or_else(|| SemioError::not_found("Edit", edit_id.as_str()))?;
                txs.remove(pos);
            }
            let open = alt.open_edit.read().await.upgrade();
            if let Some(open_tx) = open {
                if &open_tx.id == edit_id {
                    *alt.open_edit.write().await = std::sync::Weak::new();
                }
            }
            alt.change_seq.fetch_add(1, Ordering::Relaxed);
            self.invalidate_materialized_cache().await;
            Ok(())
        }

        /// @emoji 📍 Materialized [`Kit`] at a [`KitReadPointInput`] anchor ([`TheKit`](../../graphql/target.schema.graphql), [`Checkpoint`](../../graphql/target.schema.graphql), [`Alternative`](../../graphql/target.schema.graphql)).
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
                return Ok(self.materialized_kit_for_workspace(&alt.id).await);
            }
            if let Some(did) = p.workspace_id.clone() {
                let _ = (p.draft_alternative_id.clone(), p.draft_transaction_id.clone(), p.draft_operation_id.clone(), p.draft_change_id.clone());
                return Ok(self.materialized_kit_for_workspace(&did).await);
            }
            Ok(self.materialized_head_kit().await)
        }

        /// @emoji 🔧 Apply `createFixedPiece` via [`Graph::record_operation_in_open_transaction`] (tests / golden replay).
        pub async fn apply_create_fixed_piece(
            self: &Arc<Self>,
            workspace_id: Id,
            edit_id: Id,
            design_id: Id,
            blueprint_id: Id,
            position: crate::geom::PositionInput,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<(Arc<crate::kit::design::piece::Piece>,), SemioError> {
            let piece_id = Id::new().await;
            let forward = crate::operation::Operation::CreateFixedPiece {
                scope: crate::operation::Scope::CreateFixedPiece { design_id: design_id.clone(), piece_id: piece_id.clone(), blueprint_id, attribute_ids: Vec::new() },
                input: crate::operation::Input::FixedPiece { position, name, description },
            };
            let before = self.materialized_kit_for_workspace(&workspace_id).await;
            let backwards = forward.to_backwards(&before).await?;
            self.record_operation_in_open_transaction(&workspace_id, &edit_id, forward, backwards).await?;
            let after = self.materialized_kit_for_workspace(&workspace_id).await;
            let piece = after.design_by_external_id(&design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?.piece_by_external_id(&piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
            Ok((piece,))
        }

        /// 🛰️ WIP bootstrap: hydrate [`Graph::mutable_kit`] from initial kit projection JSON via [`crate::kit_backbone::graph_new_overlay_from_initial_projection_json`].
        pub async fn new_overlay_from_initial_kit_projection_json(json: serde_json::Value) -> Result<Arc<Self>, SemioError> {
            crate::kit_backbone::graph_new_overlay_from_initial_projection_json(json).await
        }

        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
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
        pub async fn the_kit(&self) -> crate::gql::interfaces::VersionInterface {
            crate::gql::interfaces::VersionInterface::TheKit(TheKit::new(Arc::downgrade(&self.arc_here())))
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
    }

    impl Default for Session {
        fn default() -> Self {
            Self { id: Id::default(), started_at: RwLock::new(None) }
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

        /// @emoji 🌐 Same navigation as WIP [`Graph`] — resolved via [`crate::worker::ParentStore::wip_graph`] for the active runtime.
        pub async fn alternatives(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql_relay::AlternativeConnection> {
            let rt = ctx.data::<Arc<crate::worker::ParentStore>>()?;
            Ok(crate::gql_relay::AlternativeConnection::from_alternatives(rt.wip_graph.alternatives.read().await.clone()).await)
        }

        pub async fn alternative(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<Arc<Alternative>>> {
            let rt = ctx.data::<Arc<crate::worker::ParentStore>>()?;
            let alts = rt.wip_graph.alternatives.read().await;
            Ok(alts.iter().find(|a| a.id == id).cloned())
        }

        #[graphql(name = "theKit")]
        pub async fn the_kit(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql::interfaces::VersionInterface> {
            let rt = ctx.data::<Arc<crate::worker::ParentStore>>()?;
            Ok(crate::gql::interfaces::VersionInterface::TheKit(TheKit::new(Arc::downgrade(&rt.wip_graph.arc_here()))))
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

//#region 🧷 interface

/// 🧷 Cross-cutting GraphQL `OwnerEntity` / `OwnedEntity` unions and empty Relay shells (expanded as more entities register).
pub mod interface {
    use std::sync::Arc;

    use async_graphql::{Object, SimpleObject, Union};

    use crate::geom::entity::{Coordinate, Location, Offset, Place, Plane, Point, Position, Vector};
    use crate::hash::merkle_collection;
    use crate::id::Id;
    use crate::kit::design::piece::Piece;
    use crate::kit::design::Design;
    use crate::kit::Kit;
    use crate::vcs::{Alternative, Checkpoint, Conflict, Edit, Graph, ReadVersion, Session, WriteVersion};

    /// @emoji 🔗 SDL `OwnerEntity` subset (grow toward full target union).
    #[derive(Clone, Union)]
    pub enum OwnerEntity {
        Edit(Arc<Edit>),
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
        Position(Arc<Position>),
        Coordinate(Arc<Coordinate>),
        Plane(Arc<Plane>),
        Point(Arc<Point>),
        Vector(Arc<Vector>),
        Place(Arc<Place>),
        Offset(Arc<Offset>),
        Location(Arc<Location>),
    }

    /// @emoji 🔗 SDL `OwnedEntity` subset for non-empty `owns` edges.
    #[derive(Clone, Union)]
    pub enum OwnedEntity {
        Kit(Arc<Kit>),
        Design(Arc<Design>),
        Piece(Arc<Piece>),
        Position(Arc<Position>),
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
    pub fn owner_arc(e: OwnerEntity) -> Arc<OwnerEntity> {
        Arc::new(e)
    }

    /// 🏷️ Map an `Option<OwnerEntity>` resolver value into the `Arc<OwnerEntity>` shape.
    pub fn owner_arc_opt(o: Option<OwnerEntity>) -> Option<Arc<OwnerEntity>> {
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
    pub async fn resolve_node(rt: &crate::worker::ParentStore, id: &Id) -> Option<GqlNode> {
        for g in [&rt.wip_graph, &rt.auth_graph] {
            if &g.id == id {
                return Some(GqlNode::Graph(g.clone()));
            }
            let kit = g.mutable_kit.read().await.clone();
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
    pub async fn piece_in_design_on_wip(rt: &crate::worker::ParentStore, design_id: &Id, piece_id: &Id) -> Option<Arc<Piece>> {
        let g = &rt.wip_graph;
        let kit = g.mutable_kit.read().await.clone();
        let des = kit.design_by_external_id(design_id).await?;
        des.piece_by_external_id(piece_id).await
    }

    /// @emoji 📍 `alternativePieceKind` stub (returns `None` until alternative graph model is wired).
    pub async fn alternative_piece_kind(_rt: &crate::worker::ParentStore, _piece_id: &Id) -> Option<String> {
        None
    }

    /// @emoji 📍 WeakEntity + entity shell for [`Coordinate`] (SDL `Coordinate`).
    #[Object(name = "Coordinate")]
    impl Coordinate {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
        }
        pub async fn u(&self) -> f64 {
            *self.u.read().await
        }
        pub async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    #[Object(name = "Vector")]
    impl Vector {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
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
    impl Point {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
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
    impl Plane {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
        }
        pub async fn origin(&self) -> Arc<Point> {
            self.origin.clone()
        }
        #[graphql(name = "xAxis")]
        pub async fn x_axis(&self) -> Arc<Vector> {
            self.x_axis.clone()
        }
        #[graphql(name = "yAxis")]
        pub async fn y_axis(&self) -> Arc<Vector> {
            self.y_axis.clone()
        }
    }

    #[Object(name = "Position")]
    impl Position {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
        }
        pub async fn center(&self) -> Arc<Coordinate> {
            self.center.clone()
        }
        pub async fn plane(&self) -> Arc<Plane> {
            self.plane.clone()
        }
    }

    #[Object(name = "Offset")]
    impl Offset {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
        }
        pub async fn u(&self) -> f64 {
            *self.u.read().await
        }
        pub async fn v(&self) -> f64 {
            *self.v.read().await
        }
    }

    /// @emoji 🌍 WeakEntity shell for [`Location`] (SDL `Location`).
    #[Object(name = "Location")]
    impl Location {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
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
    impl Place {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_hash().await
        }
        pub async fn owner(&self) -> Option<std::sync::Arc<crate::interface::OwnerEntity>> {
            None
        }
        pub async fn owns(&self) -> Option<std::sync::Arc<crate::interface::OwnedEntityConnection>> {
            Some(crate::interface::empty_owned_entity_connection())
        }
    }
}

//#endregion 🧷 interface

//#region ⚙️ operation

pub mod operation {
    //! ⚙️ Operation entities and their inputs. Operations carry `Arc<Entity>` payloads so the
    //! event bus broadcasts shared references, never deep-copied entity data.
    use std::sync::{Arc, Weak};

    use async_graphql::{Interface, Object, Union};

    use crate::error::SemioError;
    use crate::geom::{OffsetInput, PositionInput};
    use crate::hash::h;
    use crate::id::Id;
    use crate::interface::{empty_owned_entity_connection, OwnedEntityConnection, OwnerEntity};
    use crate::meta::{ConceptInput, QualityInput, TagInput};
    use crate::vcs::Edit;

    //#region 🧭 normalized operation contract
    //#region 🔖 canonical_kit_diff
    /// @emoji 📦 `Id` reference wrapper for kit diff collection entities.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct IdRef {
        pub id: Id,
    }

    /// @emoji 📦 Sparse `tags` triple (`metabolism.kit.diff.semio.json`).
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TagsCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<TagModifiedRow>,
        pub added: Vec<TagAddedRow>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct TagModifiedRow {
        pub tag: IdRef,
        pub diff: TagPatch,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TagPatch {
        pub name: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
    }

    /// @emoji 📦 Row for `tags.added[]` (owner + ids + GraphQL [`TagInput`]).
    #[derive(Clone, Debug, PartialEq)]
    pub struct TagAddedRow {
        pub owner_id: Id,
        pub id: Id,
        pub attribute_ids: Vec<Id>,
        pub tag: TagInput,
    }

    /// @emoji 📦 Sparse `concepts` triple.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConceptsCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<ConceptModifiedRow>,
        pub added: Vec<ConceptAddedRow>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ConceptModifiedRow {
        pub concept: IdRef,
        pub diff: ConceptPatch,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConceptPatch {
        pub name: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ConceptAddedRow {
        pub owner_id: Id,
        pub id: Id,
        pub attribute_ids: Vec<Id>,
        pub concept: ConceptInput,
    }

    /// @emoji 📦 Sparse `qualities` triple (kit-level).
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct QualitiesCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<QualityModifiedRow>,
        pub added: Vec<QualityAddedRow>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct QualityModifiedRow {
        pub quality: IdRef,
        pub diff: QualityPatch,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct QualityPatch {
        pub description: Option<String>,
        pub icon: Option<String>,
        pub key: Option<String>,
        pub value: Option<String>,
        pub unit: Option<String>,
        pub definition: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct QualityAddedRow {
        pub owner_id: Id,
        pub id: Id,
        pub attribute_ids: Vec<Id>,
        pub benchmark_ids: Vec<Id>,
        pub quality: QualityInput,
    }

    //#region 🔖canonical_kit_types_designs_mod
    /// @emoji 📦 Sparse `types` triple.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TypesCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<TypeModifiedRow>,
        pub added: Vec<TypeScalarDiff>,
    }

    /// @emoji 📦 One `types.modified[]` entity.
    #[derive(Clone, Debug, PartialEq)]
    pub struct TypeModifiedRow {
        pub type_ref: IdRef,
        pub diff: TypeScalarDiff,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TypeScalarDiff {
        pub name: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub unit: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PiecesCollectionDiff {
        pub removed: Vec<IdRef>,
        pub added: Vec<PieceAddedRow>,
        pub modified: Vec<PieceModifiedRow>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct PieceAddedRow {
        pub id: Id,
        pub blueprint_id: Id,
        pub name: Option<String>,
        pub description: Option<String>,
        pub scale: f64,
        pub pose: PositionInput,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct PieceModifiedRow {
        pub piece: IdRef,
        pub diff: PiecePatch,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PiecePatch {
        pub fix_piece: bool,
        pub drag: Option<OffsetInput>,
        pub pose: Option<PositionInput>,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DesignScalarDiff {
        pub name: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DesignDiff {
        pub scalars: DesignScalarDiff,
        pub pieces: Option<PiecesCollectionDiff>,
    }

    /// @emoji 📦 Sparse `designs` triple.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DesignsCollectionDiff {
        pub removed: Vec<IdRef>,
        pub modified: Vec<DesignModifiedRow>,
        pub added: Vec<DesignDiff>,
    }

    /// @emoji 📦 One `designs.modified[]` entity.
    #[derive(Clone, Debug, PartialEq)]
    pub struct DesignModifiedRow {
        pub design: IdRef,
        pub diff: DesignDiff,
    }

    //#endregion 🔖canonical_kit_types_designs_mod

    /// @emoji 📦 `files` / `folders` / `authors`: `None` omitted; `Some(false)` trivial; `Some(true)` means unsupported non-empty subtree.
    pub type KitAuxSubtree = Option<bool>;

    /// @emoji 📦 Canonical sparse kit diff aligned with metabolism fixtures (typed collections; aux subtrees are tri-state placeholders).
    #[derive(Clone, Debug, Default, PartialEq)]
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
        pub types: Option<TypesCollectionDiff>,
        pub designs: Option<DesignsCollectionDiff>,
        pub tags: Option<TagsCollectionDiff>,
        pub concepts: Option<ConceptsCollectionDiff>,
        pub qualities: Option<QualitiesCollectionDiff>,
        pub files: KitAuxSubtree,
        pub folders: KitAuxSubtree,
        pub authors: KitAuxSubtree,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
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

    /// @emoji 🚫 Empty payload marker for scope-only operations.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct NoInput;

    /// @emoji 🪪 Rename-kit scope; the kit id is implicit from the target graph line.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RenameKitScope;

    /// @emoji 🪪 Single entity scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct EntityScope {
        pub entity_id: Id,
    }

    /// @emoji 🪪 Single tag scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TagScope {
        pub tag_id: Id,
    }

    /// @emoji 🪪 Multi-tag scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TagsScope {
        pub tag_ids: Vec<Id>,
    }

    /// @emoji 🪪 Single concept scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConceptScope {
        pub concept_id: Id,
    }

    /// @emoji 🪪 Single quality scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct QualityScope {
        pub quality_id: Id,
    }

    /// @emoji 🪪 Create-tag scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateTagScope {
        pub owner_id: Id,
        pub tag_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Batch create-tag scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateTagsScope {
        pub owner_id: Id,
        pub tag_ids: Vec<Id>,
        pub attribute_ids: Vec<Vec<Id>>,
    }

    /// @emoji 🪪 Create-concept scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateConceptScope {
        pub owner_id: Id,
        pub concept_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Create-quality scope with owner id plus all pre-minted ids.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateQualityScope {
        pub owner_id: Id,
        pub quality_id: Id,
        pub attribute_ids: Vec<Id>,
        pub benchmark_ids: Vec<Id>,
    }

    /// @emoji 🪪 Create-piece scope with the parent design id and the pre-minted piece id.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateFixedPieceScope {
        pub design_id: Id,
        pub piece_id: Id,
        pub blueprint_id: Id,
        pub attribute_ids: Vec<Id>,
    }

    /// @emoji 🪪 Single piece-in-design scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PieceInDesignScope {
        pub design_id: Id,
        pub piece_id: Id,
    }

    /// @emoji 🪪 Multi-piece-in-design scope.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PiecesInDesignScope {
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
    }

    /// @emoji ✏️ Rename-kit payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RenameKitInput {
        pub name: String,
    }

    /// @emoji ✏️ Rename-tag payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RenameTagInput {
        pub name: String,
    }

    /// @emoji ✏️ Generic description payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ChangeDescriptionInput {
        pub description: Option<String>,
    }

    /// @emoji ✏️ Generic icon payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ChangeIconInput {
        pub icon: Option<String>,
    }

    /// @emoji ✏️ Generic image payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ChangeImageInput {
        pub image: Option<String>,
    }

    /// @emoji ✏️ Batch tag payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateTagsInput {
        pub tags: Vec<TagInput>,
    }

    /// @emoji ✏️ Fixed-piece creation payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CreateFixedPieceInput {
        pub position: PositionInput,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    /// @emoji ✏️ Piece drag payload.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DragPieceInput {
        pub offset: OffsetInput,
    }

    /// @emoji 🧭 Shared scope payload: every distinct id-shape used across [`Operation`] commands.
    #[derive(Clone, Debug, PartialEq)]
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
    #[derive(Clone, Debug, PartialEq)]
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
        FixedPiece { position: PositionInput, name: Option<String>, description: Option<String> },
        Offset { offset: OffsetInput },
    }

    /// @emoji 🧩 Normalized  operation surface: every variant is `{ scope: Scope, input: Input }`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Operation {
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

    impl Operation {
        pub fn kind(&self) -> &'static str {
            match self {
                Operation::RenameKit { .. } => "renameKit",
                Operation::ChangeDescription { .. } => "changeDescription",
                Operation::ChangeIcon { .. } => "changeIcon",
                Operation::ChangeImage { .. } => "changeImage",
                Operation::CreateTag { .. } => "createTag",
                Operation::CreateTags { .. } => "createTags",
                Operation::DeleteTag { .. } => "deleteTag",
                Operation::DeleteTags { .. } => "deleteTags",
                Operation::RenameTag { .. } => "renameTag",
                Operation::CreateConcept { .. } => "createConcept",
                Operation::DeleteConcept { .. } => "deleteConcept",
                Operation::CreateQuality { .. } => "createQuality",
                Operation::DeleteQuality { .. } => "deleteQuality",
                Operation::CreateFixedPiece { .. } => "createFixedPiece",
                Operation::DeletePieceInDesign { .. } => "deletePieceInDesign",
                Operation::DragPieceInDesign { .. } => "dragPieceInDesign",
                Operation::DragPiecesInDesign { .. } => "dragPiecesInDesign",
                Operation::FixPieceInDesign { .. } => "fixPieceInDesign",
            }
        }

        /// @emoji 🔢 Stable digest for deterministic diff ids (no JSON serde).
        pub fn stable_payload_digest(&self) -> String {
            fn pos_fp(p: &PositionInput) -> String {
                format!(
                    "c:{}:{}|p:o:{}:{}:{}|xa:{}:{}:{}|ya:{}:{}:{}",
                    p.center.u, p.center.v, p.plane.origin.x, p.plane.origin.y, p.plane.origin.z, p.plane.x_axis.x, p.plane.x_axis.y, p.plane.x_axis.z, p.plane.y_axis.x, p.plane.y_axis.y, p.plane.y_axis.z,
                )
            }
            match self {
                Operation::CreateFixedPiece { scope, input } => {
                    let (d, p, b, ac) = match scope {
                        Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids } => (design_id.as_str(), piece_id.as_str(), blueprint_id.as_str(), attribute_ids.len()),
                        _ => ("", "", "", 0usize),
                    };
                    let Input::FixedPiece { position, name, description } = input else {
                        return h(&[self.kind(), "bad-input"]);
                    };
                    h(&[self.kind(), d, p, b, &ac.to_string(), &pos_fp(position), name.as_deref().unwrap_or(""), description.as_deref().unwrap_or("")])
                }
                Operation::RenameKit { input, .. } => match input {
                    Input::Name { name } => h(&[self.kind(), name.as_str()]),
                    _ => h(&[self.kind(), "bad-input"]),
                },
                Operation::DragPieceInDesign { scope, input } | Operation::DragPiecesInDesign { scope, input } => {
                    let (d, pid) = match scope {
                        Scope::PieceInDesign { design_id, piece_id } => (design_id.as_str(), piece_id.as_str()),
                        Scope::PiecesInDesign { design_id, piece_ids } => {
                            let mut s = piece_ids.iter().map(|x| x.as_str().to_string()).collect::<Vec<_>>();
                            s.sort();
                            return match input {
                                Input::Offset { offset } => h(&[self.kind(), design_id.as_str(), &s.join(","), &format!("{}:{}", offset.u, offset.v)]),
                                _ => h(&[self.kind(), "bad-input"]),
                            };
                        }
                        _ => ("", ""),
                    };
                    match input {
                        Input::Offset { offset } => h(&[self.kind(), d, pid, &format!("{}:{}", offset.u, offset.v)]),
                        _ => h(&[self.kind(), "bad-input"]),
                    }
                }
                _ => h(&[self.kind(), "minimal"]),
            }
        }

        /// Pure: read pre-state and produce a structural diff without mutating the kit.
        pub async fn to_diff(&self, kit: &Arc<crate::kit::Kit>) -> Result<KitDiff, SemioError> {
            match self {
                Operation::RenameKit { scope, input } => {
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
                Operation::ChangeDescription { scope, input } => {
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
                            types: Some(TypesCollectionDiff { modified: vec![TypeModifiedRow { type_ref: IdRef { id: entity_id.clone() }, diff: TypeScalarDiff { description: description.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(DesignsCollectionDiff {
                                modified: vec![DesignModifiedRow { design: IdRef { id: entity_id.clone() }, diff: DesignDiff { scalars: DesignScalarDiff { description: description.clone(), ..Default::default() }, pieces: None } }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("DescriptionEntity", entity_id.as_str()))
                }
                Operation::ChangeIcon { scope, input } => {
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
                            types: Some(TypesCollectionDiff { modified: vec![TypeModifiedRow { type_ref: IdRef { id: entity_id.clone() }, diff: TypeScalarDiff { icon: icon.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(DesignsCollectionDiff {
                                modified: vec![DesignModifiedRow { design: IdRef { id: entity_id.clone() }, diff: DesignDiff { scalars: DesignScalarDiff { icon: icon.clone(), ..Default::default() }, pieces: None } }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("IconEntity", entity_id.as_str()))
                }
                Operation::ChangeImage { scope, input } => {
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
                            types: Some(TypesCollectionDiff { modified: vec![TypeModifiedRow { type_ref: IdRef { id: entity_id.clone() }, diff: TypeScalarDiff { image: image.clone(), ..Default::default() } }], ..Default::default() }),
                            ..Default::default()
                        }));
                    }
                    if kit.design_by_external_id(entity_id).await.is_some() {
                        return Ok(KitDiff(CanonicalKitDiff {
                            designs: Some(DesignsCollectionDiff {
                                modified: vec![DesignModifiedRow { design: IdRef { id: entity_id.clone() }, diff: DesignDiff { scalars: DesignScalarDiff { image: image.clone(), ..Default::default() }, pieces: None } }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                    Err(SemioError::not_found("ImageEntity", entity_id.as_str()))
                }
                Operation::CreateTag { scope, input } => {
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
                    Ok(KitDiff(CanonicalKitDiff {
                        tags: Some(TagsCollectionDiff { added: vec![TagAddedRow { owner_id: owner_id.clone(), id: tag_id.clone(), attribute_ids: attribute_ids.clone(), tag: tag.clone() }], ..Default::default() }),
                        ..Default::default()
                    }))
                }
                Operation::CreateTags { scope, input } => {
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
                        added.push(TagAddedRow { owner_id: owner_id.clone(), id: tag_ids[index].clone(), attribute_ids: attribute_ids[index].clone(), tag: tag.clone() });
                    }
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { added, ..Default::default() }), ..Default::default() }))
                }
                Operation::DeleteTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("deleteTag expects Scope::Tag"));
                    };
                    ensure_tag(kit, tag_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { tags: Some(TagsCollectionDiff { removed: vec![IdRef { id: tag_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                Operation::DeleteTags { scope, .. } => {
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
                Operation::RenameTag { scope, input } => {
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
                Operation::CreateConcept { scope, input } => {
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
                    Ok(KitDiff(CanonicalKitDiff {
                        concepts: Some(ConceptsCollectionDiff { added: vec![ConceptAddedRow { owner_id: owner_id.clone(), id: concept_id.clone(), attribute_ids: attribute_ids.clone(), concept: concept.clone() }], ..Default::default() }),
                        ..Default::default()
                    }))
                }
                Operation::DeleteConcept { scope, .. } => {
                    let Scope::Concept { concept_id } = scope else {
                        return Err(SemioError::invalid("deleteConcept expects Scope::Concept"));
                    };
                    ensure_concept(kit, concept_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { concepts: Some(ConceptsCollectionDiff { removed: vec![IdRef { id: concept_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                Operation::CreateQuality { scope, input } => {
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
                    Ok(KitDiff(CanonicalKitDiff {
                        qualities: Some(QualitiesCollectionDiff {
                            added: vec![QualityAddedRow { owner_id: owner_id.clone(), id: quality_id.clone(), attribute_ids: attribute_ids.clone(), benchmark_ids: benchmark_ids.clone(), quality: quality.clone() }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
                Operation::DeleteQuality { scope, .. } => {
                    let Scope::Quality { quality_id } = scope else {
                        return Err(SemioError::invalid("deleteQuality expects Scope::Quality"));
                    };
                    ensure_quality(kit, quality_id).await?;
                    Ok(KitDiff(CanonicalKitDiff { qualities: Some(QualitiesCollectionDiff { removed: vec![IdRef { id: quality_id.clone() }], ..Default::default() }), ..Default::default() }))
                }
                Operation::CreateFixedPiece { scope, input } => {
                    let Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    let Input::FixedPiece { position, name, description } = input else {
                        return Err(SemioError::invalid("createFixedPiece expects Input::FixedPiece"));
                    };
                    if !attribute_ids.is_empty() {
                        return Err(SemioError::invalid("piece attribute ids are not supported yet"));
                    }
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(DesignsCollectionDiff {
                            modified: vec![DesignModifiedRow {
                                design: IdRef { id: design_id.clone() },
                                diff: DesignDiff {
                                    scalars: DesignScalarDiff::default(),
                                    pieces: Some(PiecesCollectionDiff {
                                        removed: vec![],
                                        added: vec![PieceAddedRow { id: piece_id.clone(), blueprint_id: blueprint_id.clone(), name: name.clone(), description: description.clone(), scale: 1.0, pose: *position }],
                                        modified: vec![],
                                    }),
                                },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
                Operation::DeletePieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("deletePieceInDesign expects Scope::PieceInDesign"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(DesignsCollectionDiff {
                            modified: vec![DesignModifiedRow {
                                design: IdRef { id: design_id.clone() },
                                diff: DesignDiff { scalars: DesignScalarDiff::default(), pieces: Some(PiecesCollectionDiff { removed: vec![IdRef { id: piece_id.clone() }], added: vec![], modified: vec![] }) },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
                Operation::DragPieceInDesign { scope, input } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Scope::PieceInDesign"));
                    };
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Input::Offset"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(DesignsCollectionDiff {
                            modified: vec![DesignModifiedRow {
                                design: IdRef { id: design_id.clone() },
                                diff: DesignDiff {
                                    scalars: DesignScalarDiff::default(),
                                    pieces: Some(PiecesCollectionDiff { removed: vec![], added: vec![], modified: vec![PieceModifiedRow { piece: IdRef { id: piece_id.clone() }, diff: PiecePatch { drag: Some(*offset), ..Default::default() } }] }),
                                },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
                Operation::DragPiecesInDesign { scope, input } => {
                    let Scope::PiecesInDesign { design_id, piece_ids } = scope else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Scope::PiecesInDesign"));
                    };
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Input::Offset"));
                    };
                    let mut modified = Vec::new();
                    for piece_id in piece_ids {
                        ensure_piece(kit, design_id, piece_id).await?;
                        modified.push(PieceModifiedRow { piece: IdRef { id: (*piece_id).clone() }, diff: PiecePatch { drag: Some(*offset), ..Default::default() } });
                    }
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(DesignsCollectionDiff {
                            modified: vec![DesignModifiedRow { design: IdRef { id: design_id.clone() }, diff: DesignDiff { scalars: DesignScalarDiff::default(), pieces: Some(PiecesCollectionDiff { removed: vec![], added: vec![], modified }) } }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
                Operation::FixPieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("fixPieceInDesign expects Scope::PieceInDesign"));
                    };
                    ensure_piece(kit, design_id, piece_id).await?;
                    Ok(KitDiff(CanonicalKitDiff {
                        designs: Some(DesignsCollectionDiff {
                            modified: vec![DesignModifiedRow {
                                design: IdRef { id: design_id.clone() },
                                diff: DesignDiff {
                                    scalars: DesignScalarDiff::default(),
                                    pieces: Some(PiecesCollectionDiff { removed: vec![], added: vec![], modified: vec![PieceModifiedRow { piece: IdRef { id: piece_id.clone() }, diff: PiecePatch { fix_piece: true, ..Default::default() } }] }),
                                },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }))
                }
            }
        }

        /// Pure: read pre-state and return the ordered list of backward operations.
        pub async fn to_backwards(&self, kit: &Arc<crate::kit::Kit>) -> Result<Vec<Operation>, SemioError> {
            match self {
                Operation::RenameKit { .. } => Ok(vec![Operation::RenameKit { scope: Scope::Kit, input: Input::Name { name: kit.name.read().await.clone() } }]),
                Operation::ChangeDescription { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeDescription expects Scope::Entity"));
                    };
                    Ok(vec![Operation::ChangeDescription { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Description { description: entity_description(kit, entity_id).await? } }])
                }
                Operation::ChangeIcon { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeIcon expects Scope::Entity"));
                    };
                    Ok(vec![Operation::ChangeIcon { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Icon { icon: entity_icon(kit, entity_id).await? } }])
                }
                Operation::ChangeImage { scope, .. } => {
                    let Scope::Entity { entity_id } = scope else {
                        return Err(SemioError::invalid("changeImage expects Scope::Entity"));
                    };
                    Ok(vec![Operation::ChangeImage { scope: Scope::Entity { entity_id: entity_id.clone() }, input: Input::Image { image: entity_image(kit, entity_id).await? } }])
                }
                Operation::CreateTag { scope, .. } => {
                    let Scope::CreateTag { tag_id, .. } = scope else {
                        return Err(SemioError::invalid("createTag expects Scope::CreateTag"));
                    };
                    Ok(vec![Operation::DeleteTag { scope: Scope::Tag { tag_id: tag_id.clone() }, input: Input::None }])
                }
                Operation::CreateTags { scope, .. } => {
                    let Scope::CreateTags { tag_ids, .. } = scope else {
                        return Err(SemioError::invalid("createTags expects Scope::CreateTags"));
                    };
                    Ok(vec![Operation::DeleteTags { scope: Scope::Tags { tag_ids: tag_ids.clone() }, input: Input::None }])
                }
                Operation::DeleteTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("deleteTag expects Scope::Tag"));
                    };
                    let tag = ensure_tag(kit, tag_id).await?;
                    let owner_id = tag_owner_id(kit, &tag).await?;
                    let attributes = tag.attributes.read().await.clone();
                    Ok(vec![Operation::CreateTag {
                        scope: Scope::CreateTag { owner_id, tag_id: tag.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect() },
                        input: Input::Tag { tag: tag_input_from_entity(&tag).await },
                    }])
                }
                Operation::DeleteTags { scope, .. } => {
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
                    Ok(vec![Operation::CreateTags { scope: Scope::CreateTags { owner_id: owner_id.unwrap_or_default(), tag_ids: out_tag_ids, attribute_ids }, input: Input::Tags { tags } }])
                }
                Operation::RenameTag { scope, .. } => {
                    let Scope::Tag { tag_id } = scope else {
                        return Err(SemioError::invalid("renameTag expects Scope::Tag"));
                    };
                    let tag = ensure_tag(kit, tag_id).await?;
                    let name = {
                        let guard = tag.name.read().await;
                        guard.clone()
                    };
                    drop(tag);
                    Ok(vec![Operation::RenameTag { scope: Scope::Tag { tag_id: tag_id.clone() }, input: Input::Name { name } }])
                }
                Operation::CreateConcept { scope, .. } => {
                    let Scope::CreateConcept { concept_id, .. } = scope else {
                        return Err(SemioError::invalid("createConcept expects Scope::CreateConcept"));
                    };
                    Ok(vec![Operation::DeleteConcept { scope: Scope::Concept { concept_id: concept_id.clone() }, input: Input::None }])
                }
                Operation::DeleteConcept { scope, .. } => {
                    let Scope::Concept { concept_id } = scope else {
                        return Err(SemioError::invalid("deleteConcept expects Scope::Concept"));
                    };
                    let concept = ensure_concept(kit, concept_id).await?;
                    let owner_id = concept_owner_id(kit, &concept).await?;
                    let attributes = concept.attributes.read().await.clone();
                    Ok(vec![Operation::CreateConcept {
                        scope: Scope::CreateConcept { owner_id, concept_id: concept.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect() },
                        input: Input::Concept { concept: concept_input_from_entity(&concept).await },
                    }])
                }
                Operation::CreateQuality { scope, .. } => {
                    let Scope::CreateQuality { quality_id, .. } = scope else {
                        return Err(SemioError::invalid("createQuality expects Scope::CreateQuality"));
                    };
                    Ok(vec![Operation::DeleteQuality { scope: Scope::Quality { quality_id: quality_id.clone() }, input: Input::None }])
                }
                Operation::DeleteQuality { scope, .. } => {
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
                    Ok(vec![Operation::CreateQuality {
                        scope: Scope::CreateQuality { owner_id, quality_id: quality.id.clone(), attribute_ids: attributes.iter().map(|attribute| attribute.id.clone()).collect(), benchmark_ids: Vec::new() },
                        input: Input::Quality { quality: quality_input_from_entity(&quality).await },
                    }])
                }
                Operation::CreateFixedPiece { scope, .. } => {
                    let Scope::CreateFixedPiece { design_id, piece_id, .. } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    Ok(vec![Operation::DeletePieceInDesign { scope: Scope::PieceInDesign { design_id: design_id.clone(), piece_id: piece_id.clone() }, input: Input::None }])
                }
                Operation::DeletePieceInDesign { scope, .. } => {
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
                    Ok(vec![Operation::CreateFixedPiece { scope: Scope::CreateFixedPiece { design_id: design_id.clone(), piece_id, blueprint_id, attribute_ids }, input: Input::FixedPiece { position, name, description } }])
                }
                Operation::DragPieceInDesign { scope, input } => {
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Input::Offset"));
                    };
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(SemioError::invalid("dragPieceInDesign expects Scope::PieceInDesign"));
                    };
                    Ok(vec![Operation::DragPieceInDesign { scope: Scope::PieceInDesign { design_id: design_id.clone(), piece_id: piece_id.clone() }, input: Input::Offset { offset: OffsetInput { u: -offset.u, v: -offset.v } } }])
                }
                Operation::DragPiecesInDesign { scope, input } => {
                    let Input::Offset { offset } = input else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Input::Offset"));
                    };
                    let Scope::PiecesInDesign { design_id, piece_ids } = scope else {
                        return Err(SemioError::invalid("dragPiecesInDesign expects Scope::PiecesInDesign"));
                    };
                    Ok(vec![Operation::DragPiecesInDesign { scope: Scope::PiecesInDesign { design_id: design_id.clone(), piece_ids: piece_ids.clone() }, input: Input::Offset { offset: OffsetInput { u: -offset.u, v: -offset.v } } }])
                }
                Operation::FixPieceInDesign { scope, .. } => {
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
    #[derive(Clone, Debug, Default)]
    pub struct CreatedFixedPieceInput {
        pub design_id: Id,
        pub blueprint_id: Id,
        pub position: PositionInput,
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
        pub async fn position(&self) -> Arc<crate::geom::entity::Position> {
            crate::geom::entity::Position::from_position_input(self.position)
        }
        pub async fn name(&self) -> Option<String> {
            self.name.clone()
        }
        pub async fn description(&self) -> Option<String> {
            self.description.clone()
        }
    }

    #[derive(Clone, Debug, Default)]
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

    #[derive(Clone, Debug, Default)]
    pub struct DraggedPieceInput {
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
        pub offset: OffsetInput,
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
        pub async fn offset(&self) -> Arc<crate::geom::entity::Offset> {
            crate::geom::entity::Offset::from_input(self.offset)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct RenamedKitInput {
        pub name: String,
    }

    #[Object(name = "RenamedKitInput")]
    impl RenamedKitInput {
        pub async fn name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Clone, Debug, Default)]
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

    //#endregion 🧾 inputs

    /// @emoji 🗄️ Backbone materialization: dev JSON file, local `.semio/` SQLite+blobs, or remote hub (reserved).
    #[derive(Clone, Copy, Debug, Eq, PartialEq, async_graphql::Enum)]
    #[graphql(name = "BackboneKind")]
    pub enum BackboneKind {
        #[graphql(name = "DEV")]
        Dev,
        #[graphql(name = "LOCAL")]
        Local,
        #[graphql(name = "REMOTE")]
        Remote,
    }

    impl BackboneKind {
        /// @emoji 🗺️ Parse `dev://`, `local://`, `remote://`, or legacy `file://` (dev file path).
        pub fn from_uri(raw: &str) -> Result<(Self, String), crate::error::SemioError> {
            let u = raw.trim();
            if let Some(rest) = u.strip_prefix("file://") {
                return Ok((Self::Dev, rest.trim().to_string()));
            }
            if let Some(rest) = u.strip_prefix("dev://") {
                return Ok((Self::Dev, rest.trim().to_string()));
            }
            if let Some(rest) = u.strip_prefix("local://") {
                return Ok((Self::Local, rest.trim().to_string()));
            }
            if let Some(rest) = u.strip_prefix("remote://") {
                return Ok((Self::Remote, rest.trim().to_string()));
            }
            Err(crate::error::SemioError::invalid("backbone uri must start with dev://, local://, remote://, or file://"))
        }
    }
    //#endregion 🧭 graph workspace + backbone store kind (readable/writable selectors)

    //#region 📜  operation record (kit bundle / operation log contract)
    /// @emoji 📜 One persisted  operation: stable id, kind string, JSON payload, monotonic sequence index.
    #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
    #[graphql(name = "OperationRecord")]
    pub struct OperationRecord {
        pub id: Id,
        #[graphql(name = "operationKind")]
        pub operation_kind: String,
        #[graphql(name = "payloadJson")]
        pub payload_json: String,
        pub sequence: i32,
    }
    //#endregion 📜  operation record (kit bundle / operation log contract)

    //#region 📦 diff
    /// 📜 Ephemeral operation-side payload (id + summary) for [`CreatedFixedPiece`] / siblings — **not** the geometric `interface Diff` from `semio/graphql/target.schema.graphql` (`VectorDiff`, `PositionDiff`, …).
    #[derive(Clone, Debug, Default)]
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
        pub async fn owner(&self) -> Option<Arc<OwnerEntity>> {
            self.owner_edit.upgrade().map(|e| Arc::new(OwnerEntity::Edit(e)))
        }
        #[graphql(name = "owns")]
        pub async fn owns(&self) -> Option<Arc<OwnedEntityConnection>> {
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
        pub async fn owner(&self) -> Option<Arc<OwnerEntity>> {
            self.owner_edit.upgrade().map(|e| Arc::new(OwnerEntity::Edit(e)))
        }
        #[graphql(name = "owns")]
        pub async fn owns(&self) -> Option<Arc<OwnedEntityConnection>> {
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
        pub async fn owner(&self) -> Option<Arc<OwnerEntity>> {
            self.owner_edit.upgrade().map(|e| Arc::new(OwnerEntity::Edit(e)))
        }
        #[graphql(name = "owns")]
        pub async fn owns(&self) -> Option<Arc<OwnedEntityConnection>> {
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
        pub async fn owner(&self) -> Option<Arc<OwnerEntity>> {
            self.owner_edit.upgrade().map(|e| Arc::new(OwnerEntity::Edit(e)))
        }
        #[graphql(name = "owns")]
        pub async fn owns(&self) -> Option<Arc<OwnedEntityConnection>> {
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
        pub async fn owner(&self) -> Option<Arc<OwnerEntity>> {
            self.owner_edit.upgrade().map(|e| Arc::new(OwnerEntity::Edit(e)))
        }
        #[graphql(name = "owns")]
        pub async fn owns(&self) -> Option<Arc<OwnedEntityConnection>> {
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
        field(name = "owner", method = "owner", ty = "Option<std::sync::Arc<crate::interface::OwnerEntity>>"),
        field(name = "owns", method = "owns", ty = "Option<std::sync::Arc<crate::interface::OwnedEntityConnection>>")
    )]
    pub enum OperationInterface {
        CreatedFixedPiece(Arc<CreatedFixedPiece>),
        FixedPiece(Arc<FixedPiece>),
        DraggedPiece(Arc<DraggedPiece>),
        RenamedKit(Arc<RenamedKit>),
        ChangedDescription(Arc<ChangedDescription>),
    }

    impl Default for OperationInterface {
        fn default() -> Self {
            Self::CreatedFixedPiece(Arc::new(CreatedFixedPiece::default()))
        }
    }

    impl OperationInterface {
        /// @emoji 🪪 Stable entity id for relay operation edges / merkle shells.
        pub fn entity_id(&self) -> Id {
            match self {
                OperationInterface::CreatedFixedPiece(o) => o.id.clone(),
                OperationInterface::FixedPiece(o) => o.id.clone(),
                OperationInterface::DraggedPiece(o) => o.id.clone(),
                OperationInterface::RenamedKit(o) => o.id.clone(),
                OperationInterface::ChangedDescription(o) => o.id.clone(),
            }
        }
    }
    //#endregion 🪄 operations

    //#region 📡 commands
    /// 📡 Internal command envelope passed parent → child runtime over the work queue.
    #[derive(Clone, Debug)]
    pub enum Command {
        ApplyOperation { request_id: Id, workspace_id: Id, transaction_id: Id, operation: Operation },
        BackboneAttach { request_id: Id, connection_uri: String },
        BackboneDetach { request_id: Id, connection_uri: String },
    }

    impl Command {
        pub fn request_id(&self) -> &Id {
            match self {
                Command::ApplyOperation { request_id, .. } => request_id,
                Command::BackboneAttach { request_id, .. } => request_id,
                Command::BackboneDetach { request_id, .. } => request_id,
            }
        }
    }

    /// ✅ Lightweight signal that a command was accepted (used by `commandSucceeded`).
    #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
    #[graphql(name = "Command")]
    pub struct CommandReceipt {
        #[graphql(name = "requestId")]
        pub request_id: Id,
        pub kind: String,
    }
    //#endregion 📡 commands

    /// @emoji 🧩 Declarative operation entity registration hook (`operations! { CreatedFixedPiece, … }`) — expand to typed operation structs + history wiring.
    macro_rules! operations {
        ($($entity:ident),* $(,)?) => {
            /// @emoji 🔢 Row count listed in `operations! { … }` (static registry grows toward ~100 SDL operations).
            pub const GRAPH_OPERATION_REGISTRY_ROWS: usize = [$(stringify!($entity)),*].len();
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

//#region 🧮 kit graph engine

pub mod kit_graph_engine {
    //! @emoji 🧮 Core kit graph engine: deterministic projection fingerprints, ephemeral operation diffs, and typed [`crate::operation::Operation`] apply for replay (no serde on the control-plane surface).
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use crate::error::SemioError;
    use crate::hash::h;
    use crate::id::Id;
    use crate::kit;
    use crate::operation::{self, Operation, Scope};
    use crate::vcs::Graph;

    //#region 🔖 design_slot
    /// @emoji 🧷 Opaque internal design list index; external [`Id`] maps only in [`kit::Kit::bind_external_design_id`].
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct DesignSlot(pub u32);
    //#endregion 🔖 design_slot

    //#region 🔖 projection_fingerprint
    /// @emoji 🔢 Stable `projectionFingerprint`: [`h`] over sorted piece centers keyed by design id (matches golden `kit-store.golden.expected`).
    pub async fn projection_fingerprint_for_kit(kit: &kit::Kit) -> String {
        let designs = kit.designs.read().await;
        let mut by_design: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        for d in designs.iter() {
            let mut pts: Vec<(f64, f64)> = Vec::new();
            for p in d.pieces.read().await.iter() {
                if let Some(node) = p.position.read().await.as_ref() {
                    let pos = node.snapshot_input().await;
                    pts.push((pos.center.u, pos.center.v));
                }
            }
            pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.partial_cmp(&b.1).unwrap()));
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
    //#endregion 🔖 projection_fingerprint

    //#region 🔖 deterministic_diff
    /// @emoji 📦 Deterministic non-persisted diff from operation kind + stable payload digest + projection fingerprint transition.
    pub fn deterministic__diff(op_kind: &str, payload_digest: &str, projection_fp_before: &str, projection_fp_after: &str) -> operation::Diff {
        let digest = h(&[op_kind, payload_digest, projection_fp_before, projection_fp_after]);
        operation::Diff { id: Id::from(format!("semio:diff:{digest}")), summary: Some(digest) }
    }
    //#endregion 🔖 deterministic_diff

    //#region 🔖 apply_kit_operation
    /// @emoji 🧾 Output of [`apply_kit_operation`]: ephemeral diff + optional created entities.
    pub struct AppliedOperation {
        pub diff: operation::Diff,
        pub created_piece: Option<Arc<kit::design::piece::Piece>>,
    }

    /// @emoji 🧩 Record one forward [`Operation`] (plus backwards) on `graph` and return deterministic projection metadata.
    pub async fn apply_kit_operation(graph: &Arc<Graph>, workspace_id: &Id, transaction_id: &Id, operation: Operation) -> Result<AppliedOperation, SemioError> {
        let ws = graph.resolve_workspace_id(workspace_id).await;
        let op_kind = operation.kind();
        let payload_digest = operation.stable_payload_digest();
        let created_piece_ids = match &operation {
            Operation::CreateFixedPiece { scope, .. } => match scope {
                Scope::CreateFixedPiece { design_id, piece_id, .. } => Some((design_id.clone(), piece_id.clone())),
                _ => None,
            },
            _ => None,
        };
        let before = graph.materialized_kit_for_workspace(&ws).await;
        let backwards = operation.to_backwards(&before).await?;
        graph.record_operation_in_open_transaction(&ws, transaction_id, operation, backwards).await?;
        let after = graph.materialized_kit_for_workspace(&ws).await;
        let fp_before = projection_fingerprint_for_kit(before.as_ref()).await;
        let fp_after = projection_fingerprint_for_kit(after.as_ref()).await;
        let diff = deterministic__diff(op_kind, &payload_digest, &fp_before, &fp_after);
        let created_piece = if let Some((design_id, piece_id)) = created_piece_ids {
            match after.design_by_external_id(&design_id).await {
                Some(des) => des.piece_by_external_id(&piece_id).await,
                None => None,
            }
        } else {
            None
        };
        Ok(AppliedOperation { diff, created_piece })
    }
    //#endregion 🔖 apply_kit_operation
}

//#endregion 🧮 kit graph engine

//#region 🗄️ kit backbone persistence (native)

pub mod kit_backbone {
    //! @emoji 🗄️ Dev JSON + local `.semio/` kit backbones: atomic single-file writes, multi-db SQLite + blobs dir, replay via [`kit_graph_engine::apply_kit_operation`].
    //! 🌐 The bundle wire format (`DevBackboneBundleDoc` + DTOs + `from_graph` / `hydrate_into_graph`) is wasm-compatible —
    //! sketchpad's WASM runtime serializes / hydrates the metabolism-shaped JSON directly. The SQLite + filesystem-IO parts
    //! (atomic writes, `DevJsonAttached`, `LocalAttached`) are native-only and gated below.

    use std::sync::Arc;

    #[cfg(not(target_arch = "wasm32"))]
    use std::path::{Path, PathBuf};

    #[cfg(not(target_arch = "wasm32"))]
    use rusqlite::Connection;

    use crate::error::SemioError;
    use crate::id::Id;
    use crate::vcs::Graph;

    //#region 🧾 wire format

    /// @emoji 🪪 On-disk schema marker stamped at the bundle root; matches `semio/assets/semio/metabolism.new.kit.semio.json`.
    pub const KIT_STORE_BUNDLE_SCHEMA: &str = "🎆26🌙06⬆️1";

    /// @emoji 🧾 Blake3 hex (empty-input digest) used on the wire until per-entity merkle is filled.
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

    //#region 🔖 dev_backbone_kit_operation_json
    fn position_input_to_json(p: &crate::geom::PositionInput) -> serde_json::Value {
        serde_json::json!({
            "center": { "u": p.center.u, "v": p.center.v },
            "plane": {
                "origin": { "x": p.plane.origin.x, "y": p.plane.origin.y, "z": p.plane.origin.z },
                "xAxis": { "x": p.plane.x_axis.x, "y": p.plane.x_axis.y, "z": p.plane.x_axis.z },
                "yAxis": { "x": p.plane.y_axis.x, "y": p.plane.y_axis.y, "z": p.plane.y_axis.z },
            }
        })
    }

    //#region 🔖 dev_backbone_canonical_kit_diff_wire_json
    /// @emoji 📦 Serializes [`crate::operation::CanonicalKitDiff`] for `kitDiff` on persisted operation steps (sparse object; aligns with `metabolism.kit.diff.semio.json` collection keys).
    pub(crate) fn canonical_kit_diff_to_wire_json(d: &crate::operation::CanonicalKitDiff) -> serde_json::Value {
        use serde_json::{Map, Value};
        let mut root = Map::new();
        let opt_s = |m: &mut Map<String, Value>, k: &str, v: &Option<String>| {
            if let Some(s) = v {
                m.insert(k.to_string(), Value::String(s.clone()));
            }
        };
        opt_s(&mut root, "name", &d.name);
        opt_s(&mut root, "version", &d.version);
        opt_s(&mut root, "description", &d.description);
        opt_s(&mut root, "icon", &d.icon);
        opt_s(&mut root, "image", &d.image);
        opt_s(&mut root, "remote", &d.remote);
        opt_s(&mut root, "homepage", &d.homepage);
        opt_s(&mut root, "license", &d.license);
        opt_s(&mut root, "preview", &d.preview);
        if let Some(t) = &d.types {
            root.insert("types".to_string(), types_collection_diff_wire(t));
        }
        if let Some(ds) = &d.designs {
            root.insert("designs".to_string(), designs_collection_diff_wire(ds));
        }
        if let Some(t) = &d.tags {
            root.insert("tags".to_string(), tags_collection_diff_wire(t));
        }
        if let Some(c) = &d.concepts {
            root.insert("concepts".to_string(), concepts_collection_diff_wire(c));
        }
        if let Some(q) = &d.qualities {
            root.insert("qualities".to_string(), qualities_collection_diff_wire(q));
        }
        if let Some(v) = d.files {
            root.insert("files".to_string(), Value::Bool(v));
        }
        if let Some(v) = d.folders {
            root.insert("folders".to_string(), Value::Bool(v));
        }
        if let Some(v) = d.authors {
            root.insert("authors".to_string(), Value::Bool(v));
        }
        Value::Object(root)
    }

    fn types_collection_diff_wire(t: &crate::operation::TypesCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": t.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "modified": t.modified.iter().map(|entity| json!({
                "type": { "id": entity.type_ref.id.as_str() },
                "diff": type_scalar_diff_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
            "added": t.added.iter().map(type_scalar_diff_wire).collect::<Vec<Value>>(),
        })
    }

    fn type_scalar_diff_wire(d: &crate::operation::TypeScalarDiff) -> serde_json::Value {
        use serde_json::{Map, Value};
        let mut m = Map::new();
        if let Some(ref s) = d.name {
            m.insert("name".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.description {
            m.insert("description".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.icon {
            m.insert("icon".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.image {
            m.insert("image".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.unit {
            m.insert("unit".into(), Value::String(s.clone()));
        }
        Value::Object(m)
    }

    fn designs_collection_diff_wire(d: &crate::operation::DesignsCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": d.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "modified": d.modified.iter().map(|entity| json!({
                "design": { "id": entity.design.id.as_str() },
                "diff": design_diff_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
            "added": d.added.iter().map(design_diff_wire).collect::<Vec<Value>>(),
        })
    }

    fn design_scalar_diff_wire(d: &crate::operation::DesignScalarDiff) -> serde_json::Value {
        use serde_json::{Map, Value};
        let mut m = Map::new();
        if let Some(ref s) = d.name {
            m.insert("name".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.description {
            m.insert("description".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.icon {
            m.insert("icon".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = d.image {
            m.insert("image".into(), Value::String(s.clone()));
        }
        Value::Object(m)
    }

    fn design_diff_wire(d: &crate::operation::DesignDiff) -> serde_json::Value {
        use serde_json::Value;
        let mut o = serde_json::Map::new();
        o.insert("scalars".to_string(), design_scalar_diff_wire(&d.scalars));
        if let Some(p) = &d.pieces {
            o.insert("pieces".to_string(), pieces_collection_diff_wire(p));
        }
        Value::Object(o)
    }

    fn pieces_collection_diff_wire(p: &crate::operation::PiecesCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": p.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "added": p.added.iter().map(piece_added_row_wire).collect::<Vec<Value>>(),
            "modified": p.modified.iter().map(|entity| json!({
                "piece": { "id": entity.piece.id.as_str() },
                "diff": piece_patch_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
        })
    }

    fn piece_added_row_wire(entity: &crate::operation::PieceAddedRow) -> serde_json::Value {
        use serde_json::json;
        let mut o = serde_json::Map::new();
        o.insert("id".to_string(), json!(entity.id.as_str()));
        o.insert("blueprint_id".to_string(), json!(entity.blueprint_id.as_str()));
        o.insert("scale".to_string(), json!(entity.scale));
        o.insert("pose".to_string(), position_input_to_json(&entity.pose));
        if let Some(ref n) = entity.name {
            o.insert("name".to_string(), json!(n));
        }
        if let Some(ref n) = entity.description {
            o.insert("description".to_string(), json!(n));
        }
        serde_json::Value::Object(o)
    }

    fn piece_patch_wire(p: &crate::operation::PiecePatch) -> serde_json::Value {
        use serde_json::{json, Map, Value};
        let mut m = Map::new();
        if p.fix_piece {
            m.insert("fix_piece".into(), Value::Bool(true));
        }
        if let Some(off) = &p.drag {
            m.insert("drag".into(), json!({ "u": off.u, "v": off.v }));
        }
        if let Some(pos) = &p.pose {
            m.insert("pose".into(), position_input_to_json(pos));
        }
        if let Some(ref n) = p.name {
            m.insert("name".into(), Value::String(n.clone()));
        }
        if let Some(ref n) = p.description {
            m.insert("description".into(), Value::String(n.clone()));
        }
        Value::Object(m)
    }

    fn tag_patch_wire(p: &crate::operation::TagPatch) -> serde_json::Value {
        use serde_json::{Map, Value};
        let mut m = Map::new();
        if let Some(ref s) = p.name {
            m.insert("name".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.description {
            m.insert("description".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.icon {
            m.insert("icon".into(), Value::String(s.clone()));
        }
        Value::Object(m)
    }

    fn tag_input_wire(t: &crate::meta::TagInput) -> serde_json::Value {
        serde_json::json!({
            "name": t.name,
            "description": t.description,
            "icon": t.icon,
            "order": t.order,
            "attributes": t.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
        })
    }

    fn tags_collection_diff_wire(t: &crate::operation::TagsCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": t.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "modified": t.modified.iter().map(|entity| json!({
                "tag": { "id": entity.tag.id.as_str() },
                "diff": tag_patch_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
            "added": t.added.iter().map(|entity| json!({
                "owner_id": entity.owner_id.as_str(),
                "id": entity.id.as_str(),
                "attribute_ids": entity.attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                "tag": tag_input_wire(&entity.tag),
            })).collect::<Vec<Value>>(),
        })
    }

    fn concept_patch_wire(p: &crate::operation::ConceptPatch) -> serde_json::Value {
        tag_patch_wire(&crate::operation::TagPatch { name: p.name.clone(), description: p.description.clone(), icon: p.icon.clone() })
    }

    fn concept_input_wire(c: &crate::meta::ConceptInput) -> serde_json::Value {
        serde_json::json!({
            "name": c.name,
            "description": c.description,
            "icon": c.icon,
            "order": c.order,
            "attributes": c.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
        })
    }

    fn concepts_collection_diff_wire(t: &crate::operation::ConceptsCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": t.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "modified": t.modified.iter().map(|entity| json!({
                "concept": { "id": entity.concept.id.as_str() },
                "diff": concept_patch_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
            "added": t.added.iter().map(|entity| json!({
                "owner_id": entity.owner_id.as_str(),
                "id": entity.id.as_str(),
                "attribute_ids": entity.attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                "concept": concept_input_wire(&entity.concept),
            })).collect::<Vec<Value>>(),
        })
    }

    fn quality_patch_wire(p: &crate::operation::QualityPatch) -> serde_json::Value {
        use serde_json::{Map, Value};
        let mut m = Map::new();
        if let Some(ref s) = p.description {
            m.insert("description".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.icon {
            m.insert("icon".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.key {
            m.insert("key".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.value {
            m.insert("value".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.unit {
            m.insert("unit".into(), Value::String(s.clone()));
        }
        if let Some(ref s) = p.definition {
            m.insert("definition".into(), Value::String(s.clone()));
        }
        Value::Object(m)
    }

    fn quality_input_wire(q: &crate::meta::QualityInput) -> serde_json::Value {
        serde_json::json!({
            "key": q.key,
            "value": q.value,
            "unit": q.unit,
            "definition": q.definition,
            "description": q.description,
            "icon": q.icon,
            "attributes": q.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
        })
    }

    fn qualities_collection_diff_wire(t: &crate::operation::QualitiesCollectionDiff) -> serde_json::Value {
        use serde_json::{json, Value};
        json!({
            "removed": t.removed.iter().map(|r| json!({ "id": r.id.as_str() })).collect::<Vec<Value>>(),
            "modified": t.modified.iter().map(|entity| json!({
                "quality": { "id": entity.quality.id.as_str() },
                "diff": quality_patch_wire(&entity.diff),
            })).collect::<Vec<Value>>(),
            "added": t.added.iter().map(|entity| json!({
                "owner_id": entity.owner_id.as_str(),
                "id": entity.id.as_str(),
                "attribute_ids": entity.attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                "benchmark_ids": entity.benchmark_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                "quality": quality_input_wire(&entity.quality),
            })).collect::<Vec<Value>>(),
        })
    }
    //#endregion 🔖 dev_backbone_canonical_kit_diff_wire_json

    fn kit_scope_json(s: &crate::operation::Scope) -> serde_json::Value {
        use crate::operation::Scope;
        match s {
            Scope::Kit => serde_json::json!("Kit"),
            Scope::Entity { entity_id } => serde_json::json!({ "Entity": { "entity_id": entity_id.as_str() } }),
            Scope::Tag { tag_id } => serde_json::json!({ "Tag": { "tag_id": tag_id.as_str() } }),
            Scope::Tags { tag_ids } => serde_json::json!({ "Tags": { "tag_ids": tag_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>() } }),
            Scope::Concept { concept_id } => serde_json::json!({ "Concept": { "concept_id": concept_id.as_str() } }),
            Scope::Quality { quality_id } => serde_json::json!({ "Quality": { "quality_id": quality_id.as_str() } }),
            Scope::CreateTag { owner_id, tag_id, attribute_ids } => serde_json::json!({
                "CreateTag": {
                    "owner_id": owner_id.as_str(),
                    "tag_id": tag_id.as_str(),
                    "attribute_ids": attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                }
            }),
            Scope::CreateTags { owner_id, tag_ids, attribute_ids } => serde_json::json!({
                "CreateTags": {
                    "owner_id": owner_id.as_str(),
                    "tag_ids": tag_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                    "attribute_ids": attribute_ids.iter().map(|v| v.iter().map(|i| i.as_str()).collect::<Vec<_>>()).collect::<Vec<_>>(),
                }
            }),
            Scope::CreateConcept { owner_id, concept_id, attribute_ids } => serde_json::json!({
                "CreateConcept": {
                    "owner_id": owner_id.as_str(),
                    "concept_id": concept_id.as_str(),
                    "attribute_ids": attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                }
            }),
            Scope::CreateQuality { owner_id, quality_id, attribute_ids, benchmark_ids } => serde_json::json!({
                "CreateQuality": {
                    "owner_id": owner_id.as_str(),
                    "quality_id": quality_id.as_str(),
                    "attribute_ids": attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                    "benchmark_ids": benchmark_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                }
            }),
            Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids } => serde_json::json!({
                "CreateFixedPiece": {
                    "design_id": design_id.as_str(),
                    "piece_id": piece_id.as_str(),
                    "blueprint_id": blueprint_id.as_str(),
                    "attribute_ids": attribute_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                }
            }),
            Scope::PieceInDesign { design_id, piece_id } => serde_json::json!({ "PieceInDesign": { "design_id": design_id.as_str(), "piece_id": piece_id.as_str() } }),
            Scope::PiecesInDesign { design_id, piece_ids } => serde_json::json!({
                "PiecesInDesign": {
                    "design_id": design_id.as_str(),
                    "piece_ids": piece_ids.iter().map(|i| i.as_str()).collect::<Vec<_>>(),
                }
            }),
        }
    }

    fn kit_attribute_input_json(a: &crate::meta::AttributeInput) -> serde_json::Value {
        serde_json::json!({
            "key": a.key,
            "value": a.value,
            "definition": a.definition,
        })
    }

    fn kit_input_json(i: &crate::operation::Input) -> serde_json::Value {
        use crate::operation::Input;
        match i {
            Input::None => serde_json::json!("None"),
            Input::Name { name } => serde_json::json!({ "Name": { "name": name } }),
            Input::Description { description } => serde_json::json!({ "Description": { "description": description } }),
            Input::Icon { icon } => serde_json::json!({ "Icon": { "icon": icon } }),
            Input::Image { image } => serde_json::json!({ "Image": { "image": image } }),
            Input::Tag { tag } => serde_json::json!({
                "Tag": {
                    "tag": {
                        "name": tag.name,
                        "description": tag.description,
                        "icon": tag.icon,
                        "order": tag.order,
                        "attributes": tag.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
                    }
                }
            }),
            Input::Tags { tags } => serde_json::json!({
                "Tags": {
                    "tags": tags.iter().map(|t| serde_json::json!({
                        "name": t.name,
                        "description": t.description,
                        "icon": t.icon,
                        "order": t.order,
                        "attributes": t.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
                    })).collect::<Vec<_>>()
                }
            }),
            Input::Concept { concept } => serde_json::json!({
                "Concept": {
                    "concept": {
                        "name": concept.name,
                        "description": concept.description,
                        "icon": concept.icon,
                        "order": concept.order,
                        "attributes": concept.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
                    }
                }
            }),
            Input::Quality { quality } => serde_json::json!({
                "Quality": {
                    "quality": {
                        "key": quality.key,
                        "value": quality.value,
                        "unit": quality.unit,
                        "definition": quality.definition,
                        "description": quality.description,
                        "icon": quality.icon,
                        "attributes": quality.attributes.as_ref().map(|v| v.iter().map(kit_attribute_input_json).collect::<Vec<_>>()),
                    }
                }
            }),
            Input::FixedPiece { position, name, description } => serde_json::json!({
                "FixedPiece": {
                    "position": position_input_to_json(position),
                    "name": name,
                    "description": description,
                }
            }),
            Input::Offset { offset } => serde_json::json!({ "Offset": { "offset": { "u": offset.u, "v": offset.v } } }),
        }
    }

    pub(crate) fn kit_operation_step_input_json(op: &crate::operation::Operation) -> serde_json::Value {
        use crate::operation::{Operation, Scope};
        let pair = |scope: &Scope, input: &crate::operation::Input| serde_json::json!({ "scope": kit_scope_json(scope), "input": kit_input_json(input) });
        match op {
            Operation::RenameKit { scope, input } => serde_json::json!({ "RenameKit": pair(scope, input) }),
            Operation::ChangeDescription { scope, input } => serde_json::json!({ "ChangeDescription": pair(scope, input) }),
            Operation::ChangeIcon { scope, input } => serde_json::json!({ "ChangeIcon": pair(scope, input) }),
            Operation::ChangeImage { scope, input } => serde_json::json!({ "ChangeImage": pair(scope, input) }),
            Operation::CreateTag { scope, input } => serde_json::json!({ "CreateTag": pair(scope, input) }),
            Operation::CreateTags { scope, input } => serde_json::json!({ "CreateTags": pair(scope, input) }),
            Operation::DeleteTag { scope, input } => serde_json::json!({ "DeleteTag": pair(scope, input) }),
            Operation::DeleteTags { scope, input } => serde_json::json!({ "DeleteTags": pair(scope, input) }),
            Operation::RenameTag { scope, input } => serde_json::json!({ "RenameTag": pair(scope, input) }),
            Operation::CreateConcept { scope, input } => serde_json::json!({ "CreateConcept": pair(scope, input) }),
            Operation::DeleteConcept { scope, input } => serde_json::json!({ "DeleteConcept": pair(scope, input) }),
            Operation::CreateQuality { scope, input } => serde_json::json!({ "CreateQuality": pair(scope, input) }),
            Operation::DeleteQuality { scope, input } => serde_json::json!({ "DeleteQuality": pair(scope, input) }),
            Operation::CreateFixedPiece { scope, input } => serde_json::json!({ "CreateFixedPiece": pair(scope, input) }),
            Operation::DeletePieceInDesign { scope, input } => serde_json::json!({ "DeletePieceInDesign": pair(scope, input) }),
            Operation::DragPieceInDesign { scope, input } => serde_json::json!({ "DragPieceInDesign": pair(scope, input) }),
            Operation::DragPiecesInDesign { scope, input } => serde_json::json!({ "DragPiecesInDesign": pair(scope, input) }),
            Operation::FixPieceInDesign { scope, input } => serde_json::json!({ "FixPieceInDesign": pair(scope, input) }),
        }
    }

    fn id_from_str(s: &str) -> crate::id::Id {
        crate::id::Id::from(s)
    }

    pub(crate) fn position_input_from_json(v: &serde_json::Value) -> Result<crate::geom::PositionInput, SemioError> {
        let f2 = |o: &serde_json::Map<String, serde_json::Value>, a: &str, b: &str| -> Result<(f64, f64), SemioError> {
            let u = o.get(a).and_then(|x| x.as_f64()).ok_or_else(|| SemioError::invalid("position field"))?;
            let v = o.get(b).and_then(|x| x.as_f64()).ok_or_else(|| SemioError::invalid("position field"))?;
            Ok((u, v))
        };
        let f3 = |o: &serde_json::Map<String, serde_json::Value>| -> Result<(f64, f64, f64), SemioError> {
            let x = o.get("x").and_then(|x| x.as_f64()).ok_or_else(|| SemioError::invalid("position field"))?;
            let y = o.get("y").and_then(|x| x.as_f64()).ok_or_else(|| SemioError::invalid("position field"))?;
            let z = o.get("z").and_then(|x| x.as_f64()).ok_or_else(|| SemioError::invalid("position field"))?;
            Ok((x, y, z))
        };
        let center_o = v.get("center").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("position.center"))?;
        let (u, vv) = f2(center_o, "u", "v")?;
        let plane_o = v.get("plane").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("position.plane"))?;
        let origin = plane_o.get("origin").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("plane.origin"))?;
        let (ox, oy, oz) = f3(origin)?;
        let xa = plane_o.get("xAxis").or_else(|| plane_o.get("x_axis")).and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("plane.xAxis"))?;
        let (xx, xy, xz) = f3(xa)?;
        let ya = plane_o.get("yAxis").or_else(|| plane_o.get("y_axis")).and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("plane.yAxis"))?;
        let (yx, yy, yz) = f3(ya)?;
        Ok(crate::geom::PositionInput {
            center: crate::geom::CoordinateInput { u, v: vv },
            plane: crate::geom::PlaneInput { origin: crate::geom::PointInput { x: ox, y: oy, z: oz }, x_axis: crate::geom::VectorInput { x: xx, y: xy, z: xz }, y_axis: crate::geom::VectorInput { x: yx, y: yy, z: yz } },
        })
    }

    fn kit_scope_from_json(v: &serde_json::Value) -> Result<crate::operation::Scope, SemioError> {
        use crate::operation::Scope;
        if let Some(s) = v.as_str() {
            if s == "Kit" {
                return Ok(Scope::Kit);
            }
        }
        let o = v.as_object().ok_or_else(|| SemioError::invalid("scope object"))?;
        let (k, inner) = o.iter().next().ok_or_else(|| SemioError::invalid("empty scope"))?;
        let m = inner.as_object().ok_or_else(|| SemioError::invalid("scope inner"))?;
        Ok(match k.as_str() {
            "Entity" => Scope::Entity { entity_id: id_from_str(m.get("entity_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("entity_id"))?) },
            "Tag" => Scope::Tag { tag_id: id_from_str(m.get("tag_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("tag_id"))?) },
            "Tags" => {
                let arr = m.get("tag_ids").and_then(|x| x.as_array()).ok_or_else(|| SemioError::invalid("tag_ids"))?;
                let mut tag_ids = Vec::with_capacity(arr.len());
                for x in arr {
                    tag_ids.push(id_from_str(x.as_str().ok_or_else(|| SemioError::invalid("tag id"))?));
                }
                Scope::Tags { tag_ids }
            }
            "Concept" => Scope::Concept { concept_id: id_from_str(m.get("concept_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("concept_id"))?) },
            "Quality" => Scope::Quality { quality_id: id_from_str(m.get("quality_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("quality_id"))?) },
            "CreateTag" => Scope::CreateTag {
                owner_id: id_from_str(m.get("owner_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("owner_id"))?),
                tag_id: id_from_str(m.get("tag_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("tag_id"))?),
                attribute_ids: m.get("attribute_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
            },
            "CreateTags" => Scope::CreateTags {
                owner_id: id_from_str(m.get("owner_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("owner_id"))?),
                tag_ids: m.get("tag_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
                attribute_ids: m
                    .get("attribute_ids")
                    .and_then(|x| x.as_array())
                    .map(|outer| outer.iter().map(|inner| inner.as_array().map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default()).collect())
                    .unwrap_or_default(),
            },
            "CreateConcept" => Scope::CreateConcept {
                owner_id: id_from_str(m.get("owner_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("owner_id"))?),
                concept_id: id_from_str(m.get("concept_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("concept_id"))?),
                attribute_ids: m.get("attribute_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
            },
            "CreateQuality" => Scope::CreateQuality {
                owner_id: id_from_str(m.get("owner_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("owner_id"))?),
                quality_id: id_from_str(m.get("quality_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("quality_id"))?),
                attribute_ids: m.get("attribute_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
                benchmark_ids: m.get("benchmark_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
            },
            "CreateFixedPiece" => Scope::CreateFixedPiece {
                design_id: id_from_str(m.get("design_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("design_id"))?),
                piece_id: id_from_str(m.get("piece_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("piece_id"))?),
                blueprint_id: id_from_str(m.get("blueprint_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("blueprint_id"))?),
                attribute_ids: m.get("attribute_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
            },
            "PieceInDesign" => Scope::PieceInDesign {
                design_id: id_from_str(m.get("design_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("design_id"))?),
                piece_id: id_from_str(m.get("piece_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("piece_id"))?),
            },
            "PiecesInDesign" => Scope::PiecesInDesign {
                design_id: id_from_str(m.get("design_id").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("design_id"))?),
                piece_ids: m.get("piece_ids").and_then(|x| x.as_array()).map(|a| a.iter().map(|x| id_from_str(x.as_str().unwrap_or(""))).collect()).unwrap_or_default(),
            },
            other => return Err(SemioError::invalid(format!("unknown scope `{other}`"))),
        })
    }

    fn attribute_input_from_json(v: &serde_json::Value) -> Result<crate::meta::AttributeInput, SemioError> {
        let m = v.as_object().ok_or_else(|| SemioError::invalid("AttributeInput"))?;
        Ok(crate::meta::AttributeInput {
            key: m.get("key").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            value: m.get("value").and_then(|x| x.as_str()).map(|s| s.to_string()),
            definition: m.get("definition").and_then(|x| x.as_str()).map(|s| s.to_string()),
        })
    }

    fn kit_input_from_json(v: &serde_json::Value) -> Result<crate::operation::Input, SemioError> {
        use crate::operation::Input;
        if let Some(s) = v.as_str() {
            if s == "None" {
                return Ok(Input::None);
            }
        }
        let o = v.as_object().ok_or_else(|| SemioError::invalid("input object"))?;
        let (k, inner) = o.iter().next().ok_or_else(|| SemioError::invalid("empty input"))?;
        Ok(match k.as_str() {
            "Name" => {
                let m = inner.as_object().ok_or_else(|| SemioError::invalid("Name"))?;
                Input::Name { name: m.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string() }
            }
            "Description" => {
                let m = inner.as_object().ok_or_else(|| SemioError::invalid("Description"))?;
                Input::Description { description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()) }
            }
            "Icon" => {
                let m = inner.as_object().ok_or_else(|| SemioError::invalid("Icon"))?;
                Input::Icon { icon: m.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()) }
            }
            "Image" => {
                let m = inner.as_object().ok_or_else(|| SemioError::invalid("Image"))?;
                Input::Image { image: m.get("image").and_then(|x| x.as_str()).map(|s| s.to_string()) }
            }
            "Tag" => {
                let m = inner.get("tag").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("tag"))?;
                let attrs = m.get("attributes").and_then(|x| x.as_array()).map(|a| a.iter().map(attribute_input_from_json).collect::<Result<Vec<_>, _>>()).transpose()?;
                Input::Tag {
                    tag: crate::meta::TagInput {
                        name: m.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                        description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        icon: m.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        order: m.get("order").and_then(|x| x.as_i64()).map(|i| i as i32),
                        attributes: attrs,
                    },
                }
            }
            "Tags" => {
                let arr = inner.get("tags").and_then(|x| x.as_array()).ok_or_else(|| SemioError::invalid("tags"))?;
                let mut tags = Vec::new();
                for t in arr {
                    let m = t.as_object().ok_or_else(|| SemioError::invalid("tag entity"))?;
                    let attrs = m.get("attributes").and_then(|x| x.as_array()).map(|a| a.iter().map(attribute_input_from_json).collect::<Result<Vec<_>, _>>()).transpose()?;
                    tags.push(crate::meta::TagInput {
                        name: m.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                        description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        icon: m.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        order: m.get("order").and_then(|x| x.as_i64()).map(|i| i as i32),
                        attributes: attrs,
                    });
                }
                Input::Tags { tags }
            }
            "Concept" => {
                let m = inner.get("concept").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("concept"))?;
                let attrs = m.get("attributes").and_then(|x| x.as_array()).map(|a| a.iter().map(attribute_input_from_json).collect::<Result<Vec<_>, _>>()).transpose()?;
                Input::Concept {
                    concept: crate::meta::ConceptInput {
                        name: m.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                        description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        icon: m.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        order: m.get("order").and_then(|x| x.as_i64()).map(|i| i as i32),
                        attributes: attrs,
                    },
                }
            }
            "Quality" => {
                let m = inner.get("quality").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("quality"))?;
                let attrs = m.get("attributes").and_then(|x| x.as_array()).map(|a| a.iter().map(attribute_input_from_json).collect::<Result<Vec<_>, _>>()).transpose()?;
                Input::Quality {
                    quality: crate::meta::QualityInput {
                        key: m.get("key").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                        value: m.get("value").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        unit: m.get("unit").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        definition: m.get("definition").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        icon: m.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()),
                        attributes: attrs,
                    },
                }
            }
            "FixedPiece" => {
                let m = inner.as_object().ok_or_else(|| SemioError::invalid("FixedPiece"))?;
                let position = position_input_from_json(m.get("position").ok_or_else(|| SemioError::invalid("position"))?)?;
                Input::FixedPiece { position, name: m.get("name").and_then(|x| x.as_str()).map(|s| s.to_string()), description: m.get("description").and_then(|x| x.as_str()).map(|s| s.to_string()) }
            }
            "Offset" => {
                let m = inner.get("offset").and_then(|x| x.as_object()).ok_or_else(|| SemioError::invalid("offset"))?;
                Input::Offset { offset: crate::geom::OffsetInput { u: m.get("u").and_then(|x| x.as_f64()).unwrap_or(0.0), v: m.get("v").and_then(|x| x.as_f64()).unwrap_or(0.0) } }
            }
            other => return Err(SemioError::invalid(format!("unknown input `{other}`"))),
        })
    }

    pub(crate) fn kit_operation_from_step_json(v: &serde_json::Value) -> Result<crate::operation::Operation, SemioError> {
        use crate::operation::Operation;
        let o = v.as_object().ok_or_else(|| SemioError::invalid("kit operation"))?;
        let (k, inner) = o.iter().next().ok_or_else(|| SemioError::invalid("empty kit operation"))?;
        let body = inner.as_object().ok_or_else(|| SemioError::invalid("kit operation body"))?;
        let scope = kit_scope_from_json(body.get("scope").ok_or_else(|| SemioError::invalid("scope"))?)?;
        let input = kit_input_from_json(body.get("input").ok_or_else(|| SemioError::invalid("input"))?)?;
        Ok(match k.as_str() {
            "RenameKit" => Operation::RenameKit { scope, input },
            "ChangeDescription" => Operation::ChangeDescription { scope, input },
            "ChangeIcon" => Operation::ChangeIcon { scope, input },
            "ChangeImage" => Operation::ChangeImage { scope, input },
            "CreateTag" => Operation::CreateTag { scope, input },
            "CreateTags" => Operation::CreateTags { scope, input },
            "DeleteTag" => Operation::DeleteTag { scope, input },
            "DeleteTags" => Operation::DeleteTags { scope, input },
            "RenameTag" => Operation::RenameTag { scope, input },
            "CreateConcept" => Operation::CreateConcept { scope, input },
            "DeleteConcept" => Operation::DeleteConcept { scope, input },
            "CreateQuality" => Operation::CreateQuality { scope, input },
            "DeleteQuality" => Operation::DeleteQuality { scope, input },
            "CreateFixedPiece" => Operation::CreateFixedPiece { scope, input },
            "DeletePieceInDesign" => Operation::DeletePieceInDesign { scope, input },
            "DragPieceInDesign" => Operation::DragPieceInDesign { scope, input },
            "DragPiecesInDesign" => Operation::DragPiecesInDesign { scope, input },
            "FixPieceInDesign" => Operation::FixPieceInDesign { scope, input },
            other => return Err(SemioError::invalid(format!("unknown kit operation `{other}`"))),
        })
    }

    async fn legacy_created_fixed_piece_to_kit_op(input: &serde_json::Value) -> Result<crate::operation::Operation, SemioError> {
        let design_id = id_from_str(input.get("designId").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("designId"))?);
        let blueprint_id = id_from_str(input.get("blueprintId").and_then(|x| x.as_str()).ok_or_else(|| SemioError::invalid("blueprintId"))?);
        let position = position_input_from_json(input.get("position").ok_or_else(|| SemioError::invalid("position"))?)?;
        let name = input.get("name").and_then(|x| x.as_str()).map(|s| s.to_string());
        let description = input.get("description").and_then(|x| x.as_str()).map(|s| s.to_string());
        let piece_id = crate::id::Id::new().await;
        Ok(crate::operation::Operation::CreateFixedPiece {
            scope: crate::operation::Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, attribute_ids: Vec::new() },
            input: crate::operation::Input::FixedPiece { position, name, description },
        })
    }

    pub(crate) async fn kit_operation_from_stored(kind: &str, input: &serde_json::Value) -> Result<crate::operation::Operation, SemioError> {
        if kind == "createdFixedPiece" {
            return legacy_created_fixed_piece_to_kit_op(input).await;
        }
        kit_operation_from_step_json(input)
    }
    //#endregion 🔖 dev_backbone_kit_operation_json

    //#region 🔖 dev_backbone_initial_kit_projection
    pub(crate) async fn initial_kit_projection_value(kit: &std::sync::Arc<crate::kit::Kit>) -> serde_json::Value {
        use crate::kit::r#type::Blueprint;
        let kid = kit.workspace_kit_id().await;
        let name = kit.name.read().await.clone();
        let types_items: Vec<serde_json::Value> = {
            let tys = kit.types.read().await;
            let mut out = Vec::with_capacity(tys.len());
            for t in tys.iter() {
                let tid = t.id.as_str();
                let nm = t.name.read().await.clone();
                out.push(serde_json::json!({"id": tid, "name": nm, "connectors": []}));
            }
            out
        };
        let design_items: Vec<serde_json::Value> = {
            let des = kit.designs.read().await;
            let mut out = Vec::with_capacity(des.len());
            for d in des.iter() {
                let did = d.id.as_str();
                let dn = d.name.read().await.clone();
                let pieces: Vec<serde_json::Value> = {
                    let pcs = d.pieces.read().await;
                    let mut pj = Vec::with_capacity(pcs.len());
                    for p in pcs.iter() {
                        let ty_id = match &*p.blueprint.read().await {
                            Blueprint::Type(ty) => ty.id.as_str().to_string(),
                            _ => String::new(),
                        };
                        let pos = p.compute_flat_position().await;
                        let pv = position_input_to_json(&pos);
                        let scale = p.scale.read().await.unwrap_or(1.0);
                        let nm = p.name.read().await.clone().unwrap_or_default();
                        pj.push(serde_json::json!({
                            "id": p.id.as_str(),
                            "name": nm,
                            "type": { "id": ty_id },
                            "plane": pv.get("plane").cloned().unwrap_or_else(|| serde_json::json!({})),
                            "center": pv.get("center").cloned().unwrap_or_else(|| serde_json::json!({"u":0.0,"v":0.0})),
                            "scale": scale,
                            "color": "#000000",
                            "props": [],
                            "attributes": [],
                        }));
                    }
                    pj
                };
                out.push(serde_json::json!({
                    "id": did,
                    "name": dn,
                    "pieces": { "hash": crate::kit_backbone::KIT_BUNDLE_HASH_STUB, "items": pieces },
                    "connections": { "hash": crate::kit_backbone::KIT_BUNDLE_HASH_STUB, "items": [] },
                }));
            }
            out
        };
        serde_json::json!({
            "id": kid.as_str(),
            "name": name,
            "types": { "hash": crate::kit_backbone::KIT_BUNDLE_HASH_STUB, "items": types_items },
            "designs": { "hash": crate::kit_backbone::KIT_BUNDLE_HASH_STUB, "items": design_items },
        })
    }

    pub(crate) async fn hydrate_kit_from_initial_projection_value(kit: &std::sync::Arc<crate::kit::Kit>, json: &serde_json::Value) -> Result<(), crate::error::SemioError> {
        if let Some(n) = json.get("name").and_then(|v| v.as_str()) {
            *kit.name.write().await = n.to_string();
        }
        if let Some(id_override) = json.get("id").and_then(|v| v.as_str()) {
            *kit.snapshot_external_kit_id.write().await = Some(Id::from(id_override));
        } else {
            *kit.snapshot_external_kit_id.write().await = None;
        }

        {
            let mut tys = kit.types.write().await;
            let mut tw = kit.type_weak_by_id.write().await;
            tys.clear();
            tw.clear();
            let types_arr = json.get("types").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
            let owner = std::sync::Arc::downgrade(kit);
            for t in &types_arr {
                let Some(ts) = t.get("id").and_then(|x| x.as_str()) else { continue };
                let nm = t.get("name").and_then(|x| x.as_str()).unwrap_or("");
                let entity = crate::kit::r#type::Type::new_with_external_id(owner.clone(), ts.into(), nm.to_string()).await;
                tw.insert(entity.id.clone(), std::sync::Arc::downgrade(&entity));
                tys.push(entity);
            }
        }

        let owner = std::sync::Arc::downgrade(kit);
        let design_arr_owned = json.get("designs").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
        let mut appended: Vec<std::sync::Arc<crate::kit::design::Design>> = Vec::new();
        for d in &design_arr_owned {
            let Some(ds) = d.get("id").and_then(|x| x.as_str()) else { continue };
            let dn = d.get("name").and_then(|x| x.as_str()).unwrap_or(ds);
            let des = crate::kit::design::Design::with_id(owner.clone(), ds.into(), dn.to_string()).await;
            hydrate_design_pieces_from_snapshot_value(&des, kit, d).await?;
            appended.push(des);
        }
        {
            let mut designs_slot = kit.designs.write().await;
            let mut weak_map = kit.design_weak_by_id.write().await;
            designs_slot.clear();
            weak_map.clear();
            for des in appended {
                let did = des.id.clone();
                weak_map.insert(did, std::sync::Arc::downgrade(&des));
                designs_slot.push(des);
            }
        }

        Ok(())
    }

    /// @emoji 🪢 Hydrates [`crate::kit::design::Design`] pieces from one `designs[]` entity (`pieces` block or array).
    pub(crate) async fn hydrate_design_pieces_from_snapshot_value(des: &std::sync::Arc<crate::kit::design::Design>, kit: &std::sync::Arc<crate::kit::Kit>, d_json: &serde_json::Value) -> Result<(), crate::error::SemioError> {
        use std::collections::HashMap;
        {
            let mut pcs = des.pieces.write().await;
            pcs.clear();
        }
        *des.piece_weak_by_external_id.write().await = HashMap::new();
        let plist = d_json.get("pieces").and_then(crate::kit_backbone::json_array_or_block_items_ref).cloned().unwrap_or_default();
        let owner_des = std::sync::Arc::downgrade(des);
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
            let position = position_input_from_json(&serde_json::json!({ "plane": plane_val, "center": center_val }))?;
            let scale = pj.get("scale").and_then(|s| s.as_f64()).unwrap_or(1.0);
            let nm_opt = pj.get("name").and_then(|x| x.as_str());
            let bp = crate::kit::r#type::Blueprint::Type(ty.clone());
            let piece = crate::kit::design::piece::Piece::new_fixed_with_external_id(pid.into(), owner_des.clone(), bp, position).await;
            if let Some(nm) = nm_opt {
                piece.set_name(Some(nm.to_string())).await;
            }
            *piece.scale.write().await = Some(scale);
            let _ = des.insert_piece(piece).await;
        }
        Ok(())
    }

    pub async fn graph_new_overlay_from_initial_projection_json(json: serde_json::Value) -> Result<std::sync::Arc<crate::vcs::Graph>, crate::error::SemioError> {
        let g = crate::vcs::Graph::new().await;
        {
            let mut slot = g.mutable_kit.write().await;
            hydrate_kit_from_initial_projection_value(&*slot, &json).await?;
            if let Some(c) = json.get("createdAt").and_then(|v| v.as_str()) {
                *slot.created.write().await = Some(crate::timestamp::Timestamp(c.to_string()));
            }
            if let Some(u) = json.get("updatedAt").and_then(|v| v.as_str()) {
                *slot.updated.write().await = Some(crate::timestamp::Timestamp(u.to_string()));
            }
            let cloned = slot.deep_clone().await;
            *slot = cloned;
        }
        {
            let ini = g.mutable_kit.read().await.deep_clone().await;
            *g.initial_kit.write().await = ini;
        }
        Ok(g)
    }
    //#endregion 🔖 dev_backbone_initial_kit_projection

    /// @emoji 📜 `{hash, items: [T]}` envelope — the universal "block-hashed list" reused in every nested collection of the bundle.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct BlockHashedList<T> {
        pub hash: String,
        pub items: Vec<T>,
    }

    impl<T> Default for BlockHashedList<T> {
        fn default() -> Self {
            Self { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: Vec::new() }
        }
    }

    /// @emoji 🔗 `{id, hash}` typed reference to another node in the bundle (authors, qualities, ports, families, …).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct HashRef {
        pub id: String,
        pub hash: String,
    }

    /// @emoji 📦 Top-level on-disk kit store bundle (mirrors `metabolism.new.kit.semio.json`: `schema / wip / authoritative / stage / conflicts / blobs`; each graph snapshot holds kit seed JSON under `initialKit`).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevBackboneBundleDoc {
        pub schema: String,
        pub wip: DevBackboneGraphHead,
        pub authoritative: DevBackboneGraphHead,
        pub stage: DevBackboneGraphHead,
        #[serde(default)]
        pub conflicts: BlockHashedList<serde_json::Value>,
        #[serde(default)]
        pub blobs: BlockHashedList<serde_json::Value>,
    }

    /// @emoji 🌐 One graph snapshot (head pointer used as `wip` / `authoritative` / `stage` heads in the bundle).
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevBackboneGraphHead {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub authors: BlockHashedList<HashRef>,
        /// @emoji 📦 Wire key `initialKit` — persisted kit seed for this snapshot (GraphQL `Graph.theKit` is the live materialization, not this JSON name).
        #[serde(rename = "initialKit", default = "empty_initial_kit_value")]
        pub initial_kit: serde_json::Value,
        #[serde(rename = "theKit", default)]
        pub the_kit: DevBackboneTheKitHead,
        #[serde(default)]
        pub checkpoints: BlockHashedList<serde_json::Value>,
        #[serde(default)]
        pub alternatives: BlockHashedList<DevBackboneAltHead>,
    }

    /// @emoji 🧭 Main kit version entity; version-scoped changes live here, not on the graph snapshot.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevBackboneTheKitHead {
        pub id: String,
        pub hash: String,
        #[serde(rename = "savedChanges", default)]
        pub saved_changes: BlockHashedList<VersionChange>,
        #[serde(rename = "unsavedChanges", default)]
        pub unsaved_changes: BlockHashedList<VersionChange>,
    }

    impl Default for DevBackboneTheKitHead {
        fn default() -> Self {
            Self { id: "the-kit".to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), saved_changes: BlockHashedList::default(), unsaved_changes: BlockHashedList::default() }
        }
    }

    /// @emoji 🌿 Alternative version entity; each alternative owns its own version-scoped changes.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct DevBackboneAltHead {
        pub id: String,
        pub hash: String,
        pub name: String,
        #[serde(rename = "savedChanges", default)]
        pub saved_changes: BlockHashedList<VersionChange>,
        #[serde(rename = "unsavedChanges", default)]
        pub unsaved_changes: BlockHashedList<VersionChange>,
    }

    /// @emoji 🧾 Version change record containing ordered edits directly on `the kit` or an alternative.
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct VersionChange {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub edits: BlockHashedList<VersionEdit>,
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
    pub struct VersionEdit {
        pub id: String,
        pub hash: String,
        #[serde(default)]
        pub forwards: BlockHashedList<OperationStep>,
        #[serde(default)]
        pub backwards: BlockHashedList<OperationStep>,
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
    pub struct OperationStep {
        pub id: String,
        pub hash: String,
        pub kind: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default)]
        pub input: serde_json::Value,
        /// @emoji 📦 Canonical [`crate::operation::CanonicalKitDiff`] JSON persisted beside `input` for audit and tooling (replay still uses `input`).
        #[serde(rename = "kitDiff", default, skip_serializing_if = "Option::is_none")]
        pub kit_diff: Option<serde_json::Value>,
    }

    /// @emoji 🌱 Empty `initialKit` projection placeholder until [`Kit`] emits a real metabolism-shaped snapshot.
    fn empty_initial_kit_value() -> serde_json::Value {
        serde_json::json!({
            "hash": KIT_BUNDLE_HASH_STUB,
            "name": "",
            "types": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
            "designs": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
        })
    }

    impl DevBackboneGraphHead {
        /// @emoji 🌱 Empty graph snapshot stamped with `kit_id` (used for fresh `wip` / `authoritative` / `stage` heads).
        pub fn empty(kit_id: &str) -> Self {
            Self {
                id: kit_id.to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                authors: BlockHashedList::default(),
                initial_kit: empty_initial_kit_value(),
                the_kit: DevBackboneTheKitHead::default(),
                checkpoints: BlockHashedList::default(),
                alternatives: BlockHashedList::default(),
            }
        }
    }

    impl DevBackboneBundleDoc {
        /// @emoji 🌱 Fresh empty bundle stamped with [`KIT_STORE_BUNDLE_SCHEMA`]; kit ids fill in once the live kit projects into `initialKit`.
        pub fn template() -> Self {
            Self {
                schema: KIT_STORE_BUNDLE_SCHEMA.to_string(),
                wip: DevBackboneGraphHead::empty(""),
                authoritative: DevBackboneGraphHead::empty(""),
                stage: DevBackboneGraphHead::empty(""),
                conflicts: BlockHashedList::default(),
                blobs: BlockHashedList::default(),
            }
        }

        /// @emoji 🔁 Flatten every recorded `wip` version edit into ordered [`StoredOperation`] records ready for replay.
        pub fn wip__operations(&self) -> Vec<StoredOperation> {
            let mut out = Vec::new();
            Self::push__operations_from_version_changes(&mut out, self.wip.the_kit.saved_changes.items.iter().chain(self.wip.the_kit.unsaved_changes.items.iter()), "the-kit");
            for alternative in &self.wip.alternatives.items {
                Self::push__operations_from_version_changes(&mut out, alternative.saved_changes.items.iter().chain(alternative.unsaved_changes.items.iter()), alternative.id.as_str());
            }
            out
        }

        fn push__operations_from_version_changes<'a>(out: &mut Vec<StoredOperation>, changes: impl Iterator<Item = &'a VersionChange>, fallback_workspace_id: &str) {
            for change in changes {
                let workspace_id = change.origin.clone().unwrap_or_else(|| fallback_workspace_id.to_string());
                for edit in &change.edits.items {
                    for step in &edit.forwards.items {
                        out.push(StoredOperation { workspace_id: workspace_id.clone(), transaction_id: change.id.clone(), kind: step.kind.clone(), input: step.input.clone(), kit_diff: step.kit_diff.clone() });
                    }
                }
            }
        }

        /// @emoji 📸 Project one live change into a bundle edit; each step's `kitDiff` is computed from that op's `to_diff` against a [`Kit`] materialized for the same [`Workspace`](../../graphql/target.schema.graphql) / [`Edit`](../../graphql/target.schema.graphql) cursor as full materialization.
        async fn edit_from_runtime_change(
            graph: &std::sync::Arc<crate::vcs::Graph>,
            workspace_id: &crate::id::Id,
            tx: &std::sync::Arc<crate::vcs::Edit>,
            change_idx: usize,
            ch: &std::sync::Arc<crate::vcs::Change>,
            sequence_number: i32,
        ) -> VersionEdit {
            let mut forward_items: Vec<OperationStep> = Vec::new();
            let mut backward_items: Vec<OperationStep> = Vec::new();
            let forwards_list = ch.forwards.read().await.clone();
            for (fi, op) in forwards_list.iter().enumerate() {
                let kit_cursor = graph.kit_materialized_for_workspace_before_operation_step(workspace_id, tx, change_idx, fi).await;
                let kit_diff = match op.to_diff(&kit_cursor).await {
                    Ok(d) => Some(crate::kit_backbone::canonical_kit_diff_to_wire_json(&d.0)),
                    Err(_) => None,
                };
                forward_items.push(OperationStep { id: Id::new().await.as_str().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), kind: op.kind().to_string(), description: None, input: kit_operation_step_input_json(op), kit_diff });
            }
            let kit_bw = graph.kit_materialized_for_workspace_before_operation_step(workspace_id, tx, change_idx, forwards_list.len()).await;
            for op in ch.backwards.read().await.iter() {
                let kit_diff = match op.to_diff(&kit_bw).await {
                    Ok(d) => {
                        let w = Some(crate::kit_backbone::canonical_kit_diff_to_wire_json(&d.0));
                        let _ = kit_bw.apply_diff(&d).await;
                        w
                    }
                    Err(_) => None,
                };
                backward_items.push(OperationStep { id: Id::new().await.as_str().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), kind: op.kind().to_string(), description: None, input: kit_operation_step_input_json(op), kit_diff });
            }
            VersionEdit {
                id: ch.id.as_str().to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                forwards: BlockHashedList { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: forward_items },
                backwards: BlockHashedList { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: backward_items },
                sequence_number,
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                finished_at: Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()),
                description: None,
                origin: None,
            }
        }

        /// @emoji 📸 Project one live write session into a version change with edits.
        async fn change_from_runtime_edit(graph: &std::sync::Arc<crate::vcs::Graph>, workspace_id: &crate::id::Id, tx: &std::sync::Arc<crate::vcs::Edit>, saved: bool) -> VersionChange {
            let mut edits = Vec::new();
            for (idx, ch) in tx.changes.read().await.iter().enumerate() {
                edits.push(Self::edit_from_runtime_change(graph, workspace_id, tx, idx, ch, (idx + 1) as i32).await);
            }
            VersionChange {
                id: tx.id.as_str().to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                edits: BlockHashedList { hash: KIT_BUNDLE_HASH_STUB.to_string(), items: edits },
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                saved_at: if saved { Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()) } else { None },
                description: None,
                origin: None,
            }
        }

        async fn change_lists_for_workspace(graph: &std::sync::Arc<crate::vcs::Graph>, workspace_id: &crate::id::Id) -> (BlockHashedList<VersionChange>, BlockHashedList<VersionChange>) {
            let mut saved = BlockHashedList::default();
            let mut unsaved = BlockHashedList::default();
            if let Some((saved_edits, unsaved_edits)) = graph.workspace_saved_and_unsaved_edits(workspace_id).await {
                for tx in saved_edits {
                    saved.items.push(Self::change_from_runtime_edit(graph, workspace_id, &tx, true).await);
                }
                for tx in unsaved_edits {
                    unsaved.items.push(Self::change_from_runtime_edit(graph, workspace_id, &tx, false).await);
                }
            }
            (saved, unsaved)
        }

        /// @emoji 📸 Project the live `Graph` into a metabolism-shaped bundle ready for atomic write.
        /// `wip.id` mirrors the graph id; `wip.initialKit` is the immutable [`Graph::initial_kit`] baseline (SDL `Graph.initialKit`); head materialization stays on `theKit.kit` / version changes.
        pub async fn from_graph(graph: &crate::vcs::Graph) -> Self {
            let mut bundle = Self::template();
            let g = graph.arc_here();
            let initial = crate::kit_backbone::initial_kit_projection_value(&*g.initial_kit.read().await).await;
            let gid = graph.id.as_str().to_string();
            bundle.wip.id = gid.clone();
            bundle.authoritative.id = gid.clone();
            bundle.stage.id = gid.clone();
            bundle.wip.the_kit.id = gid.clone();
            bundle.authoritative.the_kit.id = gid.clone();
            bundle.stage.the_kit.id = gid;
            bundle.wip.initial_kit = initial.clone();
            bundle.authoritative.initial_kit = initial.clone();
            bundle.stage.initial_kit = initial;

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

            g.ensure_default_checkpoint_for_the_kit().await;
            let (saved, unsaved) = Self::change_lists_for_workspace(&g, &g.id).await;
            bundle.wip.the_kit.saved_changes.items.extend(saved.items);
            bundle.wip.the_kit.unsaved_changes.items.extend(unsaved.items);
            for alternative in graph.alternatives.read().await.iter() {
                let (saved_changes, unsaved_changes) = Self::change_lists_for_workspace(&g, &alternative.id).await;
                bundle.wip.alternatives.items.push(DevBackboneAltHead { id: alternative.id.as_str().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), name: alternative.name.read().await.clone(), saved_changes, unsaved_changes });
            }

            Self::hoist_inline_file_blobs_for_storage(&mut bundle);
            bundle
        }

        //#region 📦 bundle file blobs (content-addressed outside kit projection JSON)

        /// @emoji 🔢 Blake3 hex digest of the UTF-8 blob wire (`data:` URL or raw); identical bytes ⇒ identical digest ⇒ one entity in [`DevBackboneBundleDoc::blobs`].
        pub(crate) fn digest_kit_blob_wire(wire: &str) -> String {
            blake3::hash(wire.as_bytes()).to_hex().to_string()
        }

        /// @emoji 📦 Hoist each `files[].blob` into [`DevBackboneBundleDoc::blobs`] keyed by [`digest_kit_blob_wire`], set `files[].blobHash`, strip inline payload (shared digest dedupes across graph `initialKit` projections).
        pub fn hoist_inline_file_blobs_for_storage(bundle: &mut DevBackboneBundleDoc) {
            let mut seen_digest = std::collections::HashSet::<String>::new();
            let mut collected: Vec<serde_json::Value> = Vec::new();
            Self::take_file_blobs_from_kit_json_into(&mut bundle.wip.initial_kit, &mut seen_digest, &mut collected);
            bundle.blobs.items.extend(collected);
            Self::purge_unreferenced_blobs(bundle);
        }

        /// @emoji 🧹 Drop [`blobs`] entities whose digest is not referenced by any `files[].blobHash` on `wip` / `authoritative` / `stage` `initialKit` snapshots.
        pub fn purge_unreferenced_blobs(bundle: &mut DevBackboneBundleDoc) {
            let refs = Self::referenced_blob_hashes_from_bundle(bundle);
            bundle.blobs.items.retain(|b| b.get("hash").and_then(|x| x.as_str()).map(|h| refs.contains(h)).unwrap_or(false));
        }

        fn referenced_blob_hashes_from_bundle(bundle: &DevBackboneBundleDoc) -> std::collections::HashSet<String> {
            let mut s = std::collections::HashSet::new();
            Self::collect_blob_hashes_from_kit_projection(&bundle.wip.initial_kit, &mut s);
            Self::collect_blob_hashes_from_kit_projection(&bundle.authoritative.initial_kit, &mut s);
            Self::collect_blob_hashes_from_kit_projection(&bundle.stage.initial_kit, &mut s);
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
            let mut wip_initial_kit_json = bundle.wip.initial_kit.clone();
            Self::merge_bundle_file_blobs_into_kit_json(&mut wip_initial_kit_json, &bundle.blobs.items);
            if !wip_initial_kit_json.is_null() && wip_initial_kit_json.is_object() {
                {
                    let w = graph.mutable_kit.write().await;
                    crate::kit_backbone::hydrate_kit_from_initial_projection_value(&*w, &wip_initial_kit_json).await?;
                }
                let ini = graph.mutable_kit.read().await.deep_clone().await;
                *graph.initial_kit.write().await = ini;
            }
            Ok(bundle)
        }

        /// @emoji 🌱 Initialize an empty bundle with a non-empty `wip` head: empty `initialKit` projection stamped with `kit_id`,
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
            bundle.wip.the_kit.unsaved_changes.items.push(VersionChange {
                id: change_id.to_string(),
                hash: KIT_BUNDLE_HASH_STUB.to_string(),
                edits: BlockHashedList::default(),
                started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                saved_at: None,
                description: None,
                origin: None,
            });
            bundle
        }

        /// @emoji ➕ Append a single forward operation to an unsaved version change (creating the change/edit if absent).
        pub fn append_unsaved_edit(&mut self, change_id: &str, kind: &str, input: serde_json::Value) {
            self.append_unsaved_edit_with_origin(change_id, None, kind, input, None);
        }

        /// @emoji ➕ Append a forward operation to an unsaved version change and keep an optional replay origin anchor plus optional persisted [`crate::operation::CanonicalKitDiff`] JSON.
        pub fn append_unsaved_edit_with_origin(&mut self, change_id: &str, origin: Option<String>, kind: &str, input: serde_json::Value, kit_diff: Option<serde_json::Value>) {
            let changes = &mut self.wip.the_kit.unsaved_changes.items;
            let change_idx = match changes.iter().position(|c| c.id == change_id) {
                Some(i) => i,
                None => {
                    changes.push(VersionChange {
                        id: change_id.to_string(),
                        hash: KIT_BUNDLE_HASH_STUB.to_string(),
                        edits: BlockHashedList::default(),
                        started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                        saved_at: None,
                        description: None,
                        origin,
                    });
                    changes.len() - 1
                }
            };
            if changes[change_idx].edits.items.is_empty() {
                changes[change_idx].edits.items.push(VersionEdit {
                    id: uuid::Uuid::now_v7().to_string(),
                    hash: KIT_BUNDLE_HASH_STUB.to_string(),
                    forwards: BlockHashedList::default(),
                    backwards: BlockHashedList::default(),
                    sequence_number: 1,
                    started_at: KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string(),
                    finished_at: Some(KIT_BUNDLE_CHECKPOINT_TIMESTAMP_STUB.to_string()),
                    description: None,
                    origin: None,
                });
            }
            changes[change_idx].edits.items[0].forwards.items.push(OperationStep { id: uuid::Uuid::now_v7().to_string(), hash: KIT_BUNDLE_HASH_STUB.to_string(), kind: kind.to_string(), description: None, input, kit_diff });
        }

        /// @emoji 🪪 Build a metabolism-shaped bundle from a flat ordered  operation log (used by golden test fixtures and import paths).
        pub fn from_stored__operations(operations: &[StoredOperation]) -> Self {
            let mut bundle = Self::template();
            for operation in operations {
                bundle.append_unsaved_edit_with_origin(&operation.transaction_id, Some(operation.workspace_id.clone()), &operation.kind, operation.input.clone(), operation.kit_diff.clone());
            }
            bundle
        }
    }

    /// @emoji 📜 Internal value type used by replay + the SQLite local-`.semio/` path; not part of the on-disk dev-json wire format.
    #[derive(Clone, Debug)]
    pub struct StoredOperation {
        pub workspace_id: String,
        pub transaction_id: String,
        pub kind: String,
        pub input: serde_json::Value,
        pub kit_diff: Option<serde_json::Value>,
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
CREATE TABLE IF NOT EXISTS _operation_log (
  seq INTEGER PRIMARY KEY AUTOINCREMENT,
  draft_id TEXT NOT NULL,
  transaction_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  input_json TEXT NOT NULL,
  kit_diff_json TEXT
);
CREATE TABLE IF NOT EXISTS conflict_stub (
  id INTEGER PRIMARY KEY
);
"#;
        for name in ["wip.db", "staged.db", "authoritative.db", "conflicts.db"] {
            let db = semio_root.join(name);
            let conn = Connection::open(&db).map_err(|e| SemioError::invalid(format!("open {name}: {e}")))?;
            conn.execute_batch(ddl).map_err(|e| SemioError::invalid(format!("init {name}: {e}")))?;
            ensure_operation_log_kit_diff_json_column(&conn).map_err(|e| SemioError::invalid(format!("migrate {name}: {e}")))?;
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn ensure_operation_log_kit_diff_json_column(conn: &Connection) -> Result<(), SemioError> {
        let mut stmt = conn.prepare("PRAGMA table_info(_operation_log)").map_err(|e| SemioError::invalid(format!("pragma: {e}")))?;
        let mut has = false;
        let mut entities = stmt.query([]).map_err(|e| SemioError::invalid(format!("pragma query: {e}")))?;
        while let Some(entity) = entities.next().map_err(|e| SemioError::invalid(format!("pragma entity: {e}")))? {
            let name: String = entity.get(1).map_err(|e| SemioError::invalid(format!("pragma name: {e}")))?;
            if name == "kit_diff_json" {
                has = true;
                break;
            }
        }
        if !has {
            conn.execute("ALTER TABLE _operation_log ADD COLUMN kit_diff_json TEXT", []).map_err(|e| SemioError::invalid(format!("alter add kit_diff_json: {e}")))?;
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
    fn atomic_write_bundle(path: &Path, doc: &DevBackboneBundleDoc) -> Result<(), SemioError> {
        let parent = path.parent().ok_or_else(|| SemioError::invalid("kit-store bundle path has no parent directory"))?;
        std::fs::create_dir_all(parent).map_err(|e| SemioError::invalid(format!("create kit-store bundle parent: {e}")))?;
        let tmp = path.with_extension("tmp.semio-write");
        let body = serde_json::to_string_pretty(doc).map_err(|e| SemioError::invalid(e.to_string()))?;
        std::fs::write(&tmp, body).map_err(|e| SemioError::invalid(format!("write temp kit-store bundle: {e}")))?;
        std::fs::rename(&tmp, path).map_err(|e| SemioError::invalid(format!("rename kit-store bundle: {e}")))?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_or_init_bundle(path: &Path) -> Result<DevBackboneBundleDoc, SemioError> {
        if !path.exists() {
            return Ok(DevBackboneBundleDoc::template());
        }
        let s = std::fs::read_to_string(path).map_err(|e| SemioError::invalid(format!("read kit-store bundle: {e}")))?;
        serde_json::from_str(&s).map_err(|e| SemioError::invalid(format!("parse kit-store bundle: {e}")))
    }
    //#endregion ✍️ atomic json

    //#region 🔁 replay
    pub async fn replay_stored_operations(graph: &Arc<Graph>, operations: &[StoredOperation]) -> Result<(), SemioError> {
        graph.mutable_kit.read().await.clear_piece_projections_for_backbone_replay().await;
        for operation in operations {
            let workspace_id = Id::from(operation.workspace_id.as_str());
            let transaction_id = Id::from(operation.transaction_id.as_str());
            let op = kit_operation_from_stored(operation.kind.as_str(), &operation.input).await?;
            crate::kit_graph_engine::apply_kit_operation(graph, &workspace_id, &transaction_id, op).await?;
        }
        Ok(())
    }
    //#endregion 🔁 replay

    //#region 🧩 attached variants (native only)
    #[cfg(not(target_arch = "wasm32"))]
    pub struct DevBackboneAttached {
        path: PathBuf,
        connection_uri_normalized: String,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl DevBackboneAttached {
        /// @emoji 📥 Read the on-disk bundle (or fresh template if file is absent).
        pub fn read_bundle(&self) -> Result<DevBackboneBundleDoc, SemioError> {
            read_or_init_bundle(&self.path)
        }

        /// @emoji ➕ Append a forward  operation step into the targeted unsaved version change and atomically rewrite the bundle.
        pub fn append_operation(&mut self, workspace_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value, kit_diff: Option<&serde_json::Value>) -> Result<(), SemioError> {
            let mut doc = self.read_bundle()?;
            let _ = workspace_id;
            doc.append_unsaved_edit_with_origin(transaction_id.as_str(), None, kind, input.clone(), kit_diff.cloned());
            atomic_write_bundle(&self.path, &doc)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub struct LocalBackboneAttached {
        #[allow(dead_code)]
        semio_root: PathBuf,
        db_path: PathBuf,
        connection_uri_normalized: String,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl LocalBackboneAttached {
        pub fn append_operation(&mut self, workspace_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value, kit_diff: Option<&serde_json::Value>) -> Result<(), SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite append: {e}")))?;
            ensure_operation_log_kit_diff_json_column(&conn)?;
            let input_json = serde_json::to_string(input).map_err(|e| SemioError::invalid(e.to_string()))?;
            let kit_json = match kit_diff {
                Some(v) => Some(serde_json::to_string(v).map_err(|e| SemioError::invalid(e.to_string()))?),
                None => None,
            };
            conn.execute("INSERT INTO _operation_log (draft_id, transaction_id, kind, input_json, kit_diff_json) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![workspace_id.as_str(), transaction_id.as_str(), kind, input_json, kit_json])
                .map_err(|e| SemioError::invalid(format!("sqlite insert: {e}")))?;
            Ok(())
        }

        fn load_operations(&self) -> Result<Vec<StoredOperation>, SemioError> {
            let conn = Connection::open(&self.db_path).map_err(|e| SemioError::invalid(format!("sqlite read: {e}")))?;
            ensure_operation_log_kit_diff_json_column(&conn)?;
            let mut stmt = conn.prepare("SELECT draft_id, transaction_id, kind, input_json, kit_diff_json FROM _operation_log ORDER BY seq ASC").map_err(|e| SemioError::invalid(format!("sqlite prepare: {e}")))?;
            let mut entities = stmt.query([]).map_err(|e| SemioError::invalid(format!("sqlite query: {e}")))?;
            let mut out = Vec::new();
            while let Some(entity) = entities.next().map_err(|e| SemioError::invalid(format!("sqlite entity: {e}")))? {
                let draft_id_col: String = entity.get(0).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let transaction_id: String = entity.get(1).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let kind: String = entity.get(2).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input_json: String = entity.get(3).map_err(|e| SemioError::invalid(format!("sqlite col: {e}")))?;
                let input: serde_json::Value = serde_json::from_str(&input_json).map_err(|e| SemioError::invalid(e.to_string()))?;
                let kit_diff: Option<serde_json::Value> = match entity.get::<_, Option<String>>(4) {
                    Ok(Some(s)) if !s.is_empty() => Some(serde_json::from_str(&s).map_err(|e| SemioError::invalid(e.to_string()))?),
                    _ => None,
                };
                out.push(StoredOperation { workspace_id: draft_id_col, transaction_id, kind, input, kit_diff });
            }
            Ok(out)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub enum AttachedBackbone {
        Dev(DevBackboneAttached),
        Local(LocalBackboneAttached),
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl AttachedBackbone {
        pub async fn mount_and_replay(connection_uri: &str, child_label: &'static str, graph: &Arc<Graph>) -> Result<Self, SemioError> {
            let norm = normalize_connection_uri(connection_uri);
            let (bone_kind, remainder) = crate::operation::BackboneKind::from_uri(&norm)?;
            let path = filesystem_path_from_uri(remainder.trim())?;
            let mut this = match bone_kind {
                crate::operation::BackboneKind::Dev => Self::Dev(DevBackboneAttached { path, connection_uri_normalized: norm }),
                crate::operation::BackboneKind::Local => {
                    let semio_root = resolve_local_semio_root(&path);
                    init_local_dot_semio_layout(&semio_root)?;
                    let db_path = db_file_for_child(&semio_root, child_label)?;
                    Self::Local(LocalBackboneAttached { semio_root, db_path, connection_uri_normalized: norm })
                }
                crate::operation::BackboneKind::Remote => {
                    return Err(SemioError::invalid("remote backbone attach is not implemented yet"));
                }
            };
            this.replay_into_graph(graph).await?;
            Ok(this)
        }

        pub async fn replay_into_graph(&mut self, graph: &Arc<Graph>) -> Result<(), SemioError> {
            let operations: Vec<StoredOperation> = match self {
                AttachedBackbone::Dev(d) => d.read_bundle()?.wip__operations(),
                AttachedBackbone::Local(l) => l.load_operations()?,
            };
            replay_stored_operations(graph, &operations).await
        }

        pub fn append__operation(&mut self, workspace_id: &Id, transaction_id: &Id, kind: &str, input: &serde_json::Value, kit_diff: Option<&serde_json::Value>) -> Result<(), SemioError> {
            match self {
                AttachedBackbone::Dev(d) => d.append_operation(workspace_id, transaction_id, kind, input, kit_diff),
                AttachedBackbone::Local(l) => l.append_operation(workspace_id, transaction_id, kind, input, kit_diff),
            }
        }

        pub fn normalized_connection_uri(&self) -> &str {
            match self {
                AttachedBackbone::Dev(d) => d.connection_uri_normalized.as_str(),
                AttachedBackbone::Local(l) => l.connection_uri_normalized.as_str(),
            }
        }
    }
    //#endregion 🧩 attached variants

    /// @emoji 📑 US-001 golden JSON: top-level `operations` array, or legacy key `ops` (see `kit-store.golden.ops.semio.json`).
    pub fn golden_operation_records_ref(src: &serde_json::Value) -> Result<&Vec<serde_json::Value>, SemioError> {
        src.get("operations").and_then(|v| v.as_array()).or_else(|| src.get("ops").and_then(|v| v.as_array())).ok_or_else(|| SemioError::invalid("golden operations missing `operations` or `ops` array"))
    }

    /// @emoji 🧪 Build [`StoredOperation`] entities from `kit-store.golden.ops.semio.json` (US-001 fixture) for persistence tests.
    pub fn stored_operations_from_golden_operations_json(src: &serde_json::Value) -> Result<Vec<StoredOperation>, SemioError> {
        let workspace_id = src["draftId"].as_str().ok_or_else(|| SemioError::invalid("golden operations missing draftId"))?.to_string();
        let transaction_id = src["transactionId"].as_str().ok_or_else(|| SemioError::invalid("golden operations missing transactionId"))?.to_string();
        let arr = golden_operation_records_ref(src)?;
        let mut out = Vec::new();
        for rec in arr {
            let kind = rec["kind"].as_str().ok_or_else(|| SemioError::invalid("operation.kind"))?;
            let input = rec.get("input").cloned().ok_or_else(|| SemioError::invalid("operation.input"))?;
            let kit_diff = rec.get("kitDiff").cloned();
            out.push(StoredOperation { workspace_id: workspace_id.clone(), transaction_id: transaction_id.clone(), kind: kind.to_string(), input, kit_diff });
        }
        Ok(out)
    }
}

//#endregion 🗄️ kit backbone persistence (native)

//#region 📣 event

pub mod event {
    //! 📣 The single emit point of the entire crate. Variants carry Arc-shared payloads.
    use std::sync::{Arc, Mutex};

    use async_broadcast::{InactiveReceiver, Receiver, Sender};
    use async_lock::Mutex as AsyncMutex;

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

    impl Event {
        /// @emoji 🧭 Command / aggregate outcome events re-emit every live subscription root.
        pub fn invalidates_all_subscription_paths(&self) -> bool {
            matches!(self, Self::CommandSucceeded(_) | Self::OperationSucceeded(_) | Self::OperationFailed(_))
        }

        /// @emoji 🎯 `watched` comes from subscription lookahead; empty ⇒ match all paths.
        pub fn matches_watched_paths(&self, watched: &[String]) -> bool {
            if watched.is_empty() {
                return true;
            }
            if self.invalidates_all_subscription_paths() {
                return true;
            }
            let touched = self.canonical_touched_paths();
            watched.iter().any(|w| touched.iter().any(|t| t == w))
        }

        /// @emoji 📍 Canonical dotted paths this event may invalidate (wip / authoritative mirrors).
        pub fn canonical_touched_paths(&self) -> Vec<String> {
            match self {
                Self::CommandSucceeded(_) | Self::OperationSucceeded(_) | Self::OperationFailed(_) => Vec::new(),
                Self::RenamedKit(_) => vec!["wip:theKit:kit:name".into(), "authoritative:theKit:kit:name".into()],
                Self::ChangedDescription(_) => vec!["wip:theKit:kit:description".into(), "authoritative:theKit:kit:description".into()],
                Self::CreatedFixedPiece(_) | Self::FixedPiece(_) | Self::DraggedPiece(_) => {
                    vec!["wip:theKit:kit".into(), "wip:theKit:kit:designs".into(), "authoritative:theKit:kit".into(), "authoritative:theKit:kit:designs".into()]
                }
            }
        }
    }

    /// 📣 The bus. Holds the only `emit_event` function in the crate.
    pub struct EventBus {
        tx: AsyncMutex<Sender<Event>>,
        keep_alive: InactiveReceiver<Event>,
        /// @emoji 🧷 Per-subscription [`Event`] fan-out keyed by watched canonical paths.
        path_sinks: Mutex<Vec<(Vec<String>, async_channel::Sender<Event>)>>,
    }

    impl EventBus {
        pub fn new(capacity: usize) -> Arc<Self> {
            let (mut tx, rx) = async_broadcast::broadcast(capacity);
            tx.set_overflow(true);
            // No active receivers? still proceed (drop the message) instead of awaiting one.
            tx.set_await_active(false);
            Arc::new(Self { tx: AsyncMutex::new(tx), keep_alive: rx.deactivate(), path_sinks: Mutex::new(Vec::new()) })
        }

        /// 📣 The **only** `emit_event` in the entire crate. All other code paths must call this.
        pub async fn emit_event(&self, ev: Event) {
            let sinks: Vec<(Vec<String>, async_channel::Sender<Event>)> = self.path_sinks.lock().unwrap().iter().map(|(p, t)| (p.clone(), t.clone())).collect();
            for (paths, tx) in sinks {
                if paths.is_empty() || ev.matches_watched_paths(&paths) {
                    let _ = tx.send(ev.clone()).await;
                }
            }
            let txb = self.tx.lock().await;
            let _ = txb.broadcast_direct(ev).await;
        }

        /// 🔔 New subscriber receiver (unfiltered broadcast).
        pub fn subscribe(&self) -> Receiver<Event> {
            self.keep_alive.activate_cloned()
        }

        /// @emoji 🔔 Path-filtered channel: emits only [`Event`] values matching `watched` canonical paths.
        pub fn subscribe_paths(&self, watched: &[String]) -> async_channel::Receiver<Event> {
            let (tx, rx) = async_channel::unbounded();
            self.path_sinks.lock().unwrap().push((watched.to_vec(), tx));
            rx
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
    use crate::operation::{Command, CommandReceipt, CreatedFixedPiece, CreatedFixedPieceInput, Diff, Input, Operation, OperationInterface, RenamedKit, RenamedKitInput as OperationRenamedKitInput, Scope};
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

        pub async fn mount(&self, graph: &Arc<Graph>, child_label: &'static str, uri: &str) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let bone = crate::kit_backbone::AttachedBackbone::mount_and_replay(uri, child_label, graph).await?;
                *self.slot.write().await = Some(bone);
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (graph, child_label, uri);
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

        pub async fn record_kit_operation_if_attached(&self, workspace_id: &Id, transaction_id: &Id, operation: &crate::operation::Operation, kit_diff_wire: Option<serde_json::Value>) -> Result<(), SemioError> {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let mut guard = self.slot.write().await;
                if let Some(backbone) = guard.as_mut() {
                    let payload = crate::kit_backbone::kit_operation_step_input_json(operation);
                    let kd = kit_diff_wire.as_ref();
                    backbone.append__operation(workspace_id, transaction_id, operation.kind(), &payload, kd)?;
                }
                Ok(())
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (workspace_id, transaction_id, operation, kit_diff_wire);
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
    pub struct ParentStore {
        pub bus: Arc<EventBus>,
        pub wip: ChildPort,
        pub auth: ChildPort,
        pub wip_graph: Arc<Graph>,
        pub auth_graph: Arc<Graph>,
        pub sessions: RwLock<Vec<Arc<Session>>>,
        pub conflicts: RwLock<Vec<Arc<Conflict>>>,
        pub wip_kit_scope: RwLock<Option<(Id, Id)>>,
    }

    impl ParentStore {
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
        pub async fn spawn_wip_overlay_from_initial_kit_projection_json(json: serde_json::Value) -> Result<Arc<Self>, crate::error::SemioError> {
            let bus = EventBus::new(1024);

            let wip_graph = Graph::new_overlay_from_initial_kit_projection_json(json).await?;
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
        let fut = async move { ChildStore { label, graph, bus, inbox, backbone: BackboneNativeCell::new() }.run().await };
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
    pub struct ChildStore {
        pub label: &'static str,
        pub graph: Arc<Graph>,
        pub bus: Arc<EventBus>,
        pub inbox: Receiver<Command>,
        pub backbone: BackboneNativeCell,
    }

    impl ChildStore {
        pub async fn run(self) {
            while let Ok(cmd) = self.inbox.recv().await {
                let request_id = cmd.request_id().clone();
                let kind = match &cmd {
                    Command::ApplyOperation { operation, .. } => operation.kind(),
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
                Command::ApplyOperation { request_id, workspace_id, transaction_id, operation } => self.apply_kit_operation(request_id, workspace_id, transaction_id, operation).await,
                Command::BackboneAttach { connection_uri, .. } => self.backbone.mount(&self.graph, self.label, &connection_uri).await,
                Command::BackboneDetach { connection_uri, .. } => self.backbone.detach_matching(&connection_uri).await,
            }
        }

        async fn apply_kit_operation(&self, request_id: Id, workspace_id: Id, transaction_id: Id, operation: Operation) -> Result<(), SemioError> {
            let graph = self.graph.clone();
            let ws = graph.resolve_workspace_id(&workspace_id).await;
            let before_kit = graph.materialized_kit_for_workspace(&ws).await;
            let kit_diff = operation.to_diff(&before_kit).await?;
            let backwards = operation.to_backwards(&before_kit).await?;
            let forward = operation.clone();
            let kit_wire = crate::kit_backbone::canonical_kit_diff_to_wire_json(&kit_diff.0);
            graph.record_operation_in_open_transaction(&ws, &transaction_id, forward, backwards).await?;
            self.backbone.record_kit_operation_if_attached(&workspace_id, &transaction_id, &operation, Some(kit_wire)).await?;
            let after_kit = graph.materialized_kit_for_workspace(&ws).await;

            let tx_edit = graph
                .workspace_saved_and_unsaved_edits(&ws)
                .await
                .and_then(|(s, u)| s.into_iter().chain(u).find(|t| t.id == transaction_id))
                .ok_or_else(|| SemioError::not_found("Edit", transaction_id.as_str()))?;

            match &operation {
                Operation::RenameKit { input, .. } => {
                    let Input::Name { name } = input else {
                        return Err(SemioError::invalid("renameKit expects Input::Name"));
                    };
                    let mut diff = Diff::default();
                    diff.id = Id::new().await;
                    diff.summary = Some("renameKit".to_string());
                    let op_evt = Arc::new(RenamedKit { id: Id::new().await, request_id, owner_edit: Arc::downgrade(&tx_edit), input: OperationRenamedKitInput { name: name.clone() }, diff, kit: after_kit.clone() });
                    let interface = Arc::new(OperationInterface::RenamedKit(op_evt.clone()));
                    graph.op_history.write().await.push(interface.clone());
                    tx_edit.forward_interface_operations.write().await.push(interface);
                    self.bus.emit_event(Event::RenamedKit(op_evt)).await;
                }
                Operation::CreateFixedPiece { scope, input } => {
                    let persisted = operation.clone();
                    let Scope::CreateFixedPiece { design_id, piece_id, blueprint_id, .. } = scope else {
                        return Err(SemioError::invalid("createFixedPiece expects Scope::CreateFixedPiece"));
                    };
                    let Input::FixedPiece { position, name, description } = input else {
                        return Err(SemioError::invalid("createFixedPiece expects Input::FixedPiece"));
                    };
                    let design = after_kit.design_by_external_id(design_id).await.ok_or_else(|| SemioError::not_found("Design", design_id.as_str()))?;
                    let piece = design.piece_by_external_id(piece_id).await.ok_or_else(|| SemioError::not_found("Piece", piece_id.as_str()))?;
                    let payload_digest = persisted.stable_payload_digest();
                    let fp_before = crate::kit_graph_engine::projection_fingerprint_for_kit(before_kit.as_ref()).await;
                    let fp_after = crate::kit_graph_engine::projection_fingerprint_for_kit(after_kit.as_ref()).await;
                    let diff = crate::kit_graph_engine::deterministic__diff("createFixedPiece", &payload_digest, &fp_before, &fp_after);
                    let created_input = CreatedFixedPieceInput { design_id: design_id.clone(), blueprint_id: blueprint_id.clone(), position: position.clone(), name: name.clone(), description: description.clone() };
                    let op_evt = Arc::new(CreatedFixedPiece { id: Id::new().await, owner_edit: Arc::downgrade(&tx_edit), input: created_input, diff, piece });
                    let interface = Arc::new(OperationInterface::CreatedFixedPiece(op_evt.clone()));
                    graph.op_history.write().await.push(interface.clone());
                    tx_edit.forward_interface_operations.write().await.push(interface);
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
    use async_graphql::{Context, Lookahead, Object, Schema, Subscription};
    use async_stream::stream;
    use futures_util::Stream;
    use std::pin::Pin;
    use std::sync::Arc;

    use crate::event::EventBus;
    use crate::geom::{OffsetInput, PositionInput};
    use crate::id::Id;
    use crate::operation::{Command, Input, Scope};
    use crate::vcs::Graph;
    use crate::worker::ParentStore;

    //#region 📡 subscription_paths
    /// @emoji 📡 Flattens subscription selection into canonical `root:field:...` strings for [`crate::event::Event::matches_watched_paths`].
    pub(crate) fn collect_subscription_field_paths(root: &str, ctx: &Context<'_>) -> Vec<String> {
        let la = ctx.look_ahead();
        let inner = la.field(root);
        let focus = if inner.exists() { inner } else { la };
        let mut out = Vec::new();
        collect_la_paths(root, &focus, &mut out);
        out
    }

    fn collect_la_paths(prefix: &str, look: &Lookahead<'_>, acc: &mut Vec<String>) {
        let fields = look.selection_fields();
        if fields.is_empty() {
            acc.push(prefix.to_string());
            return;
        }
        for sf in fields {
            let name = sf.name();
            let path = format!("{prefix}:{name}");
            let nested = Lookahead::from(sf);
            if nested.selection_fields().is_empty() {
                acc.push(path);
            } else {
                collect_la_paths(&path, &nested, acc);
            }
        }
    }
    //#endregion 📡 subscription_paths

    //#region 🌐 interfaces
    /// @emoji 🌐 SDL `Node` + `EntityEdge` interfaces (geometry variants). `EntityConnection` + `Entity`/`WeakEntity`/… need resolver-aligned field types (register after `page_info`/`Arc` story settles).
    pub mod interfaces {
        use std::sync::Arc;

        use async_graphql::Interface;

        use crate::geom::entity::{Coordinate, Location, Offset, Plane, Point, Position, Vector};
        use crate::gql_relay::{CoordinateEdge, LocationEdge, OffsetEdge, PlaneEdge, PointEdge, PositionEdge, VectorEdge};

        #[derive(Clone, Interface)]
        #[graphql(name = "Node", field(name = "id", ty = "crate::id::Id"))]
        pub enum NodeInterface {
            Vector(Arc<Vector>),
            Point(Arc<Point>),
            Coordinate(Arc<Coordinate>),
            Offset(Arc<Offset>),
            Plane(Arc<Plane>),
            Position(Arc<Position>),
            Location(Arc<Location>),
        }

        #[derive(Clone, Interface)]
        #[graphql(name = "EntityEdge", field(name = "cursor", ty = "String"))]
        pub enum EntityEdgeInterface {
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
        pub enum VersionInterface {
            TheKit(Arc<crate::vcs::TheKit>),
            Alternative(Arc<crate::vcs::Alternative>),
        }
    }
    //#endregion 🌐 interfaces

    /// @emoji 🧩 Executable schema (`Query`, `Mutation`, `Subscription`).
    pub type AppSchema = Schema<Query, Mutation, Subscription>;

    /// @emoji 🏪 Explicit target-schema Store root for session, graph heads, and conflict reads.
    pub struct Store;

    #[Object(name = "Store")]
    impl Store {
        /// @emoji 🧭 First active [`crate::vcs::Session`] on this runtime.
        pub async fn session(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<crate::vcs::Session>> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            rt.sessions.read().await.first().cloned().ok_or_else(|| async_graphql::Error::new("no session"))
        }

        /// @emoji 🌐 Writable in-progress graph head.
        pub async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Arc<Graph>> {
            Ok(ctx.data::<Arc<ParentStore>>()?.wip_graph.clone())
        }

        /// @emoji 🧾 Authoritative graph head when available.
        #[graphql(name = "authoritative")]
        pub async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Arc<Graph>>> {
            Ok(Some(ctx.data::<Arc<ParentStore>>()?.auth_graph.clone()))
        }

        /// @emoji ⚔️ Current conflict registry as a relay connection.
        pub async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<crate::gql_relay::ConflictConnection> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let list = rt.conflicts.read().await.clone();
            Ok(crate::gql_relay::ConflictConnection::from_conflicts(list).await)
        }
    }

    pub struct Query;

    #[Object]
    impl Query {
        /// @emoji 🏪 Canonical target-schema read root.
        pub async fn store(&self) -> Store {
            Store
        }

        /// @emoji 🔎 Relay-style global `node` lookup (WIP + authoritative + sessions + conflicts).
        pub async fn node(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Option<crate::interface::GqlNode>> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(crate::interface::resolve_node(rt.as_ref(), &id).await)
        }

        /// @emoji 🔎 Alias of [`Query::node`] for SDL `entity` entry point (`hash` merkle id).
        pub async fn entity(&self, ctx: &Context<'_>, hash: Id) -> async_graphql::Result<Option<crate::interface::GqlNode>> {
            self.node(ctx, hash).await
        }
    }

    //#region 🎛️commands
    /// @emoji 🎛️ `Mutation.session` scope — holds kit command context on [`ParentStore`].
    pub struct SessionCommand;

    #[Object(name = "SessionCommand")]
    impl SessionCommand {
        async fn start(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let _ = rt.wip_graph.ensure_default_checkpoint_for_the_kit().await;
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

        async fn backbone(&self) -> BackboneCommand {
            BackboneCommand
        }

        #[graphql(name = "theKit")]
        async fn the_kit(&self) -> VersionCommand {
            VersionCommand
        }

        async fn alternative(&self, #[graphql(name = "id")] id: Id) -> AlternativeCommand {
            AlternativeCommand { alternative_id: id }
        }

        #[graphql(name = "startAlternative")]
        async fn start_alternative(&self, ctx: &Context<'_>, name: Option<String>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let n = name.unwrap_or_default();
            rt.wip_graph.create_alternative_from_tip(n, None).await.map_err(|e| async_graphql::Error::new(e.to_string()))
        }
    }

    /// @emoji 🗄️ GraphQL entry for `session.backbone.*` kit persistence commands.
    pub struct BackboneCommand;

    #[derive(Clone, async_graphql::SimpleObject)]
    #[graphql(name = "BackboneStatus")]
    pub struct BackboneStatus {
        #[graphql(name = "attachedUri")]
        pub attached_uri: Option<String>,
        pub kind: Option<crate::operation::BackboneKind>,
    }

    #[Object(name = "BackboneCommand")]
    impl BackboneCommand {
        async fn attach(&self, ctx: &Context<'_>, uri: String) -> async_graphql::Result<Id> {
            let _ = crate::operation::BackboneKind::from_uri(&uri).map_err(|e| async_graphql::Error::new(e.message))?;
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let request_id = Id::new().await;
            Ok(rt.dispatch_wip(Command::BackboneAttach { request_id, connection_uri: uri }).await)
        }

        async fn detach(&self, ctx: &Context<'_>, uri: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let request_id = Id::new().await;
            Ok(rt.dispatch_wip(Command::BackboneDetach { request_id, connection_uri: uri }).await)
        }

        async fn status(&self, ctx: &Context<'_>) -> async_graphql::Result<BackboneStatus> {
            let _ = ctx;
            Ok(BackboneStatus { attached_uri: None, kind: None })
        }

        #[graphql(name = "setActiveCheckpoint")]
        async fn set_active_checkpoint(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<Id> {
            let _ = (ctx, id);
            Ok(Id::new().await)
        }

        #[graphql(name = "syncNow")]
        async fn sync_now(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let _ = ctx;
            Ok(Id::new().await)
        }
    }

    pub struct VersionCommand;

    #[Object(name = "VersionCommand")]
    impl VersionCommand {
        #[graphql(name = "startNewChange")]
        async fn start_new_change(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            rt.wip_graph.ensure_default_checkpoint_for_the_kit().await;
            let ws = rt.wip_graph.id.clone();
            let tx = rt.wip_graph.open_transaction(&ws).await;
            *rt.wip_kit_scope.write().await = Some((ws, tx.id.clone()));
            Ok(tx.id.clone())
        }

        #[graphql(name = "unsavedChange")]
        async fn unsaved_change(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<UnsavedChangeCommand> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            if let Some((_, tx)) = rt.wip_kit_scope.read().await.as_ref() {
                if tx != &id {
                    return Err(async_graphql::Error::new("unsavedChange id does not match active change"));
                }
            }
            Ok(UnsavedChangeCommand { change_id: id })
        }

        async fn save(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let scope = rt.wip_kit_scope.read().await.clone();
            let Some((workspace_id, tx_id)) = scope else {
                return Err(async_graphql::Error::new("no active unsaved change"));
            };
            rt.wip_graph.commit_transaction(&workspace_id, &tx_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            *rt.wip_kit_scope.write().await = None;
            Ok(Id::new().await)
        }

        #[graphql(name = "createCheckpoint")]
        async fn create_checkpoint(&self, ctx: &Context<'_>, message: String) -> async_graphql::Result<Id> {
            let _ = (ctx, message);
            Ok(Id::new().await)
        }
    }

    pub struct UnsavedChangeCommand {
        pub change_id: Id,
    }

    #[Object(name = "UnsavedChangeCommand")]
    impl UnsavedChangeCommand {
        async fn kit(&self) -> OperationInput {
            OperationInput { change_id: self.change_id.clone() }
        }

        async fn save(&self, ctx: &Context<'_>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let scope = rt.wip_kit_scope.read().await.clone();
            let Some((workspace_id, tx_id)) = scope else {
                return Err(async_graphql::Error::new("no active unsaved change"));
            };
            if tx_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            rt.wip_graph.commit_transaction(&workspace_id, &tx_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
            *rt.wip_kit_scope.write().await = None;
            Ok(Id::new().await)
        }
    }

    pub struct AlternativeCommand {
        pub alternative_id: Id,
    }

    #[Object(name = "AlternativeCommand")]
    impl AlternativeCommand {
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

    pub struct OperationInput {
        pub change_id: Id,
    }

    #[Object(name = "OperationInput")]
    impl OperationInput {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyOperation { request_id: request_id.clone(), workspace_id, transaction_id, operation: crate::operation::Operation::RenameKit { scope: Scope::Kit, input: Input::Name { name: new_name } } };
            Ok(rt.dispatch_wip(cmd).await)
        }

        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<Id> {
            let _ = (ctx, self, new_description);
            Ok(Id::new().await)
        }

        #[graphql(name = "createTag")]
        async fn create_tag(&self, ctx: &Context<'_>, name: String, description: Option<String>, icon: Option<String>, order: Option<i32>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let tag_id = Id::new().await;
            let request_id = Id::new().await;
            let tag = crate::meta::TagInput { name, description, icon, order, attributes: None };
            let cmd = Command::ApplyOperation {
                request_id: request_id.clone(),
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::CreateTag { scope: Scope::CreateTag { owner_id, tag_id: tag_id.clone(), attribute_ids: Vec::new() }, input: Input::Tag { tag } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn tag(&self, #[graphql(name = "id")] id: Id) -> TagOperationInput {
            TagOperationInput { change_id: self.change_id.clone(), tag_id: id }
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
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let concept_id = Id::new().await;
            let request_id = Id::new().await;
            let concept = crate::meta::ConceptInput { name, description, icon, order, attributes: None };
            let cmd = Command::ApplyOperation {
                request_id: request_id.clone(),
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::CreateConcept { scope: Scope::CreateConcept { owner_id, concept_id: concept_id.clone(), attribute_ids: Vec::new() }, input: Input::Concept { concept } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn concept(&self, #[graphql(name = "id")] id: Id) -> ConceptOperationInput {
            ConceptOperationInput { change_id: self.change_id.clone(), concept_id: id }
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
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch for kit operation"));
            }
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let owner_id = kit.workspace_kit_id().await;
            let quality_id = Id::new().await;
            let request_id = Id::new().await;
            let quality = crate::meta::QualityInput { key, value, unit, definition, description, icon, attributes: None };
            let cmd = Command::ApplyOperation {
                request_id: request_id.clone(),
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::CreateQuality { scope: Scope::CreateQuality { owner_id, quality_id: quality_id.clone(), attribute_ids: Vec::new(), benchmark_ids: Vec::new() }, input: Input::Quality { quality } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }

        async fn quality(&self, #[graphql(name = "id")] id: Id) -> QualityOperationInput {
            QualityOperationInput { change_id: self.change_id.clone(), quality_id: id }
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

        async fn r#type(&self, #[graphql(name = "id")] id: Id) -> TypeOperationInput {
            TypeOperationInput { change_id: self.change_id.clone(), type_id: id }
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

        async fn design(&self, #[graphql(name = "id")] id: Id) -> DesignOperationInput {
            DesignOperationInput { change_id: self.change_id.clone(), design_id: id }
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

    pub struct TagOperationInput {
        pub change_id: Id,
        pub tag_id: Id,
    }

    #[Object(name = "TagOperationInput")]
    impl TagOperationInput {
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

    pub struct ConceptOperationInput {
        pub change_id: Id,
        pub concept_id: Id,
    }

    #[Object(name = "ConceptOperationInput")]
    impl ConceptOperationInput {
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

    pub struct QualityOperationInput {
        pub change_id: Id,
        pub quality_id: Id,
    }

    #[Object(name = "QualityOperationInput")]
    impl QualityOperationInput {
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

    pub struct TypeOperationInput {
        pub change_id: Id,
        pub type_id: Id,
    }

    #[Object(name = "TypeOperationInput")]
    impl TypeOperationInput {
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
        async fn port(&self, #[graphql(name = "id")] id: Id) -> PortOperationInput {
            PortOperationInput { change_id: self.change_id.clone(), type_id: self.type_id.clone(), port_id: id }
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
        async fn connector(&self, #[graphql(name = "id")] id: Id) -> ConnectorOperationInput {
            ConnectorOperationInput { change_id: self.change_id.clone(), type_id: self.type_id.clone(), connector_id: id }
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

    pub struct PortOperationInput {
        pub change_id: Id,
        pub type_id: Id,
        pub port_id: Id,
    }

    #[Object(name = "PortOperationInput")]
    impl PortOperationInput {
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

    pub struct ConnectorOperationInput {
        pub change_id: Id,
        pub type_id: Id,
        pub connector_id: Id,
    }

    #[Object(name = "ConnectorOperationInput")]
    impl ConnectorOperationInput {
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

    pub struct DesignOperationInput {
        pub change_id: Id,
        pub design_id: Id,
    }

    #[Object(name = "DesignOperationInput")]
    impl DesignOperationInput {
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
        async fn add_fixed_piece(&self, ctx: &Context<'_>, #[graphql(name = "blueprintId")] blueprint_id: Id, position: PositionInput, name: Option<String>, description: Option<String>) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let piece_id = Id::new().await;
            let cmd = Command::ApplyOperation {
                request_id: request_id.clone(),
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::CreateFixedPiece {
                    scope: Scope::CreateFixedPiece { design_id: self.design_id.clone(), piece_id, blueprint_id, attribute_ids: Vec::new() },
                    input: Input::FixedPiece { position, name, description },
                },
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
            position: Option<PositionInput>,
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
            position: PositionInput,
            name: Option<String>,
            description: Option<String>,
            scale: Option<f64>,
        ) -> async_graphql::Result<Id> {
            let _ = (ctx, self, blueprint_id, parent_piece_id, parent_connector, child_connector, position, name, description, scale);
            Ok(Id::new().await)
        }
        async fn piece(&self, #[graphql(name = "id")] id: Id) -> PieceOperationInput {
            PieceOperationInput { change_id: self.change_id.clone(), design_id: self.design_id.clone(), piece_id: id }
        }
        async fn pieces(&self, ids: Vec<Id>) -> PiecesOperationInput {
            PiecesOperationInput { change_id: self.change_id.clone(), design_id: self.design_id.clone(), piece_ids: ids }
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

    pub struct PieceOperationInput {
        pub change_id: Id,
        pub design_id: Id,
        pub piece_id: Id,
    }

    #[Object(name = "PieceOperationInput")]
    impl PieceOperationInput {
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
        async fn drag(&self, ctx: &Context<'_>, offset: OffsetInput) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyOperation {
                request_id,
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::DragPieceInDesign { scope: Scope::PieceInDesign { design_id: self.design_id.clone(), piece_id: self.piece_id.clone() }, input: Input::Offset { offset } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
        async fn r#move(&self, ctx: &Context<'_>, position: PositionInput) -> async_graphql::Result<Id> {
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

    pub struct PiecesOperationInput {
        pub change_id: Id,
        pub design_id: Id,
        pub piece_ids: Vec<Id>,
    }

    #[Object(name = "PiecesOperationInput")]
    impl PiecesOperationInput {
        async fn drag(&self, ctx: &Context<'_>, offset: OffsetInput) -> async_graphql::Result<Id> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let (workspace_id, transaction_id) = rt.wip_kit_scope.read().await.clone().ok_or_else(|| async_graphql::Error::new("no active kit scope"))?;
            if transaction_id != self.change_id {
                return Err(async_graphql::Error::new("change id mismatch"));
            }
            let request_id = Id::new().await;
            let cmd = Command::ApplyOperation {
                request_id,
                workspace_id,
                transaction_id,
                operation: crate::operation::Operation::DragPiecesInDesign { scope: Scope::PiecesInDesign { design_id: self.design_id.clone(), piece_ids: self.piece_ids.clone() }, input: Input::Offset { offset } },
            };
            Ok(rt.dispatch_wip(cmd).await)
        }
        async fn r#move(&self, ctx: &Context<'_>, offset: OffsetInput) -> async_graphql::Result<Id> {
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
        async fn session(&self) -> SessionCommand {
            SessionCommand
        }
    }

    pub struct Subscription;

    type StoreStream = Pin<Box<dyn Stream<Item = async_graphql::Result<Store>> + Send>>;
    type SessionStream = Pin<Box<dyn Stream<Item = async_graphql::Result<Arc<crate::vcs::Session>>> + Send>>;
    type GraphStream = Pin<Box<dyn Stream<Item = async_graphql::Result<Arc<Graph>>> + Send>>;
    type ConflictStream = Pin<Box<dyn Stream<Item = async_graphql::Result<crate::gql_relay::ConflictConnection>> + Send>>;
    type NodeStream = Pin<Box<dyn Stream<Item = async_graphql::Result<Option<crate::interface::GqlNode>>> + Send>>;

    #[Subscription]
    impl Subscription {
        /// @emoji 📡 Live-query mirror of [`Query::store`] — re-emits the explicit Store root on matching event-bus ticks.
        async fn store(&self, ctx: &Context<'_>) -> async_graphql::Result<StoreStream> {
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let watched = collect_subscription_field_paths("store", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    yield Ok(Store);
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::session`] — re-emits on each outbound [`EventBus`] tick.
        async fn session(&self, ctx: &Context<'_>) -> async_graphql::Result<SessionStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let watched = collect_subscription_field_paths("session", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    let out = {
                        let g = rt.sessions.read().await;
                        g.first().cloned().ok_or_else(|| async_graphql::Error::new("no session"))
                    };
                    match out {
                        Ok(s) => yield Ok(s),
                        Err(e) => {
                            yield Err(e);
                            break;
                        }
                    }
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::wip`].
        async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<GraphStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let watched = collect_subscription_field_paths("wip", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    yield Ok(rt.wip_graph.clone());
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::authoritative`].
        #[graphql(name = "authoritative")]
        async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<GraphStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let watched = collect_subscription_field_paths("authoritative", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    yield Ok(rt.auth_graph.clone());
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::conflicts`].
        async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<ConflictStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let watched = collect_subscription_field_paths("conflicts", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    let list = rt.conflicts.read().await.clone();
                    let entity = crate::gql_relay::ConflictConnection::from_conflicts(list).await;
                    yield Ok(entity);
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::node`].
        async fn node(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<NodeStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let id_capture = id.clone();
            let watched = collect_subscription_field_paths("node", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    let out = crate::interface::resolve_node(rt.as_ref(), &id_capture).await;
                    yield Ok(out);
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }

        /// @emoji 📡 Live-query mirror of [`Query::entity`].
        async fn entity(&self, ctx: &Context<'_>, hash: Id) -> async_graphql::Result<NodeStream> {
            let rt = ctx.data::<Arc<ParentStore>>()?.clone();
            let bus = ctx.data::<Arc<EventBus>>()?.clone();
            let hash_capture = hash.clone();
            let watched = collect_subscription_field_paths("entity", ctx);
            let filtered = !watched.is_empty();
            Ok(Box::pin(stream! {
                let mut broadcast_rx = if filtered { None } else { Some(bus.subscribe()) };
                let path_rx = if filtered { Some(bus.subscribe_paths(&watched)) } else { None };
                loop {
                    let out = crate::interface::resolve_node(rt.as_ref(), &hash_capture).await;
                    yield Ok(out);
                    if let Some(ref prx) = path_rx {
                        if prx.recv().await.is_err() {
                            break;
                        }
                    } else if let Some(ref mut brx) = broadcast_rx {
                        match brx.recv().await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    } else {
                        break;
                    }
                }
            }))
        }
    }

    fn build_schema_sync_for(rt: Arc<ParentStore>) -> AppSchema {
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
            .register_output_type::<crate::gql::interfaces::NodeInterface>()
            .register_output_type::<crate::gql::interfaces::EntityEdgeInterface>()
            .register_output_type::<crate::gql::interfaces::VersionInterface>()
            .register_output_type::<crate::gql::BackboneStatus>()
            .finish()
    }

    /// 📜 Canonical SDL: non-empty [`crate::sdl_registry::HasSdlFragment::SDL_FRAGMENT`] prefixes (code-first ladder, plan W0–W8) plus embedded golden [`include_str!("../graphql/schema.graphql")`] until fragments subsume the file.
    pub async fn sdl() -> String {
        let mut acc = String::new();
        for frag in crate::sdl_registry::all_fragments() {
            if frag.is_empty() {
                continue;
            }
            acc.push_str(frag);
            acc.push('\n');
        }
        acc.push_str(include_str!("../../../schema/graphql/schema.graphql"));
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
    pub fn build_schema_for(rt: Arc<ParentStore>) -> AppSchema {
        build_schema_sync_for(rt)
    }

    /// 🧱 Default schema (fresh runtime).
    pub async fn build_schema() -> AppSchema {
        build_schema_sync_for(ParentStore::spawn().await)
    }
}

//#endregion 🌐 gql

//#region 🔌 wasm_bridge

#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    //! 🌐 `KitStoreHandle`: GraphQL executor + subscriptions over seeded [`crate::worker::ParentStore`] (WASM build).
    use std::sync::Arc;
    use std::sync::Mutex;

    use async_graphql::{Request, Variables};
    use base64::Engine;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::future_to_promise;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    use crate::gql::{build_schema_for, AppSchema};
    use crate::worker::ParentStore;

    async fn fetch_text_url(url: &str) -> Result<String, crate::error::SemioError> {
        let win = web_sys::window().ok_or_else(|| crate::error::SemioError::invalid("no window"))?;
        let resp_val = JsFuture::from(win.fetch_with_str(url)).await.map_err(|e| crate::error::SemioError::invalid(format!("fetch: {e:?}")))?;
        let resp: Response = resp_val.dyn_into().map_err(|_| crate::error::SemioError::invalid("bad response"))?;
        if !resp.ok() {
            return Err(crate::error::SemioError::invalid(format!("http {}", resp.status())));
        }
        let text = JsFuture::from(resp.text().map_err(|e| crate::error::SemioError::invalid(format!("text(): {e:?}")))?).await.map_err(|e| crate::error::SemioError::invalid(format!("text await: {e:?}")))?;
        Ok(text.as_string().unwrap_or_default())
    }

    async fn bootstrap_runtime_from_json_value(v: serde_json::Value) -> Result<Arc<ParentStore>, crate::error::SemioError> {
        if v.get("schema").and_then(|s| s.as_str()) == Some(crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA) {
            let rt = ParentStore::spawn().await;
            let s = serde_json::to_string(&v).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
            crate::kit_backbone::DevBackboneBundleDoc::hydrate_into_graph(&rt.wip_graph, &s).await?;
            Ok(rt)
        } else {
            ParentStore::spawn_wip_overlay_from_initial_kit_projection_json(v).await
        }
    }

    async fn bootstrap_runtime_from_open_uri(uri: &str) -> Result<Arc<ParentStore>, crate::error::SemioError> {
        let u = uri.trim();
        if u.is_empty() || u == "dev://empty" {
            return Ok(ParentStore::spawn().await);
        }
        if let Some(b64) = u.strip_prefix("dev+json:") {
            let bytes = base64::engine::general_purpose::STANDARD.decode(b64.trim()).map_err(|e| crate::error::SemioError::invalid(format!("base64: {e}")))?;
            let v: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
            return bootstrap_runtime_from_json_value(v).await;
        }
        let (_k, tail) = crate::operation::BackboneKind::from_uri(u)?;
        let tail = tail.trim();
        if tail.starts_with("http://") || tail.starts_with("https://") || tail.starts_with("blob:") {
            let txt = fetch_text_url(tail).await?;
            let v: serde_json::Value = serde_json::from_str(&txt).map_err(|e| crate::error::SemioError::invalid(e.to_string()))?;
            return bootstrap_runtime_from_json_value(v).await;
        }
        Err(crate::error::SemioError::invalid("KitStoreHandle.create: use dev://empty, dev+json:<standard-base64>, or dev://<http(s)|blob-url> returning kit or bundle JSON"))
    }

    fn graphql_execute_request_from_str(s: &str) -> Result<Request, JsValue> {
        let v: serde_json::Value = serde_json::from_str(s).map_err(|e| JsValue::from_str(&format!("graphql json: {}", e)))?;
        let query = v.get("query").and_then(|x| x.as_str()).ok_or_else(|| JsValue::from_str("graphql json: missing query"))?.to_string();
        let mut r = Request::new(query);
        if let Some(vars) = v.get("variables") {
            if !vars.is_null() {
                let gql_vars = Variables::from_json(vars.clone());
                r = r.variables(gql_vars);
            }
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
            let _rt: Arc<ParentStore> = ParentStore::spawn().await;
            Ok(JsValue::TRUE)
        })
    }

    /// 🌐 Stateful GraphQL façade for `@semio/js` embedded worker + inline WASM.
    #[wasm_bindgen]
    pub struct KitStoreHandle {
        rt: Arc<ParentStore>,
        schema_mtx: Arc<Mutex<Option<AppSchema>>>,
    }

    #[wasm_bindgen]
    impl KitStoreHandle {
        /// 🧾 `KitStoreHandle.create(uri)` — `dev://empty`, `dev+json:<base64>`, or `dev://<fetchable-url>` returning kit/bundle JSON.
        #[wasm_bindgen(js_name = create)]
        pub fn create(uri: String) -> js_sys::Promise {
            future_to_promise(async move {
                let rt = bootstrap_runtime_from_open_uri(&uri).await.map_err(|e| JsValue::from_str(&e.message))?;
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

                let mut req = graphql_execute_request_from_str(&req_str)?;
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

                let mut req = graphql_execute_request_from_str(&req_str)?;
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

    /// @emoji 📜 `gql::sdl()` equals normalized `target.schema.graphql` (non-empty [`crate::sdl_registry::HasSdlFragment::SDL_FRAGMENT`] values prefix the embedded golden per macro-driven refactor).
    #[test]
    fn schema_matches_target_graphql_file() {
        let disk = include_str!("../../../schema/graphql/target.schema.graphql");
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
        "{ store { wip { theKit { kit { designs { edges { node { id pieces { edges { node { id position { center { u v } } } } } } } } } } } } }"
    }

    fn relay_auth_designs_piece_ids() -> &'static str {
        "{ store { authoritative { theKit { kit { designs { edges { node { pieces { edges { node { id } } } } } } } } } } }"
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

    /// 🛡️ [`crate::worker::ChildStore`] must record operations and rely on materialization replay (`Kit::apply_diff` inside `Graph::materialized_kit_for_workspace`); it must not replace `mutable_kit` or call `apply_diff` directly.
    #[test]
    fn worker_child_runtime_guard_no_direct_mutable_kit_write_or_apply_diff() {
        let src = include_str!("lib.rs");
        let i = src.find("impl ChildStore").expect("ChildStore impl");
        let j = src[i..].find("//#endregion 🧵 worker").expect("worker end marker") + i;
        let worker = &src[i..j];
        assert!(!worker.contains("mutable_kit.write()"), "ChildStore must not assign Graph::mutable_kit");
        assert!(!worker.contains("apply_diff"), "ChildStore must not call Kit::apply_diff; use record_operation_in_open_transaction + materialized_kit_for_workspace");
    }

    #[test]
    fn kit_store_bundle_serialize_hydrate_round_trip_via_graphql() {
        // 📸 Renames then hydrates via [`crate::kit_backbone::DevBackboneBundleDoc`] (bundle GraphQL entry points were dropped from the target schema).
        block_on(async {
            let rt = crate::worker::ParentStore::spawn().await;
            let g = rt.wip_graph.clone();
            g.ensure_default_checkpoint_for_the_kit().await;
            let ws_a = g.id.clone();
            let tx_a = g.open_transaction(&ws_a).await;
            let req = crate::id::Id::new().await;
            let _ = rt
                .dispatch_wip(crate::operation::Command::ApplyOperation {
                    request_id: req,
                    workspace_id: ws_a.clone(),
                    transaction_id: tx_a.id.clone(),
                    operation: crate::operation::Operation::RenameKit { scope: crate::operation::Scope::Kit, input: crate::operation::Input::Name { name: "Hello Bundle".into() } },
                })
                .await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let schema_a = crate::gql::build_schema_for(rt.clone());
            let q_baseline = r#"{
                store {
                    wip {
                        initialKit { name }
                        theKit { kit { name } }
                        checkpoints { edges { node { initial { name } kit { name } } } }
                    }
                }
            }"#;
            let res = schema_a.execute(q_baseline).await;
            assert!(res.errors.is_empty(), "baseline query errors: {:?}", res.errors);
            let vr = res.data.into_json().unwrap();
            assert_eq!(vr["store"]["wip"]["theKit"]["kit"]["name"].as_str(), Some("Hello Bundle"), "materialized wip.theKit.kit");
            assert_eq!(vr["store"]["wip"]["initialKit"]["name"].as_str(), Some("the kit"), "graph.initialKit stays immutable");
            let cp_initial = vr["store"]["wip"]["checkpoints"]["edges"][0]["node"]["initial"]["name"].as_str().expect("checkpoint.initial.name");
            assert_eq!(cp_initial, "the kit", "checkpoint.initial must not alias live rename");

            g.abort_transaction(&ws_a, &tx_a.id).await.expect("abort");
            let res = schema_a.execute(q_baseline).await;
            assert!(res.errors.is_empty(), "baseline after abort: {:?}", res.errors);
            let vr = res.data.into_json().unwrap();
            assert_eq!(vr["store"]["wip"]["theKit"]["kit"]["name"].as_str(), Some("the kit"), "materialized kit reverts after abort");
            assert_eq!(vr["store"]["wip"]["initialKit"]["name"].as_str(), Some("the kit"));
            assert_eq!(vr["store"]["wip"]["checkpoints"]["edges"][0]["node"]["initial"]["name"].as_str(), Some("the kit"));

            let tx_a2 = g.open_transaction(&ws_a).await;
            let req2 = crate::id::Id::new().await;
            let _ = rt
                .dispatch_wip(crate::operation::Command::ApplyOperation {
                    request_id: req2,
                    workspace_id: ws_a.clone(),
                    transaction_id: tx_a2.id.clone(),
                    operation: crate::operation::Operation::RenameKit { scope: crate::operation::Scope::Kit, input: crate::operation::Input::Name { name: "Hello Bundle".into() } },
                })
                .await;
            std::thread::sleep(std::time::Duration::from_millis(150));

            let json_a = serde_json::to_string(&crate::kit_backbone::DevBackboneBundleDoc::from_graph(g.as_ref()).await).expect("serialize bundle");

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
            let fwd0 = &v["wip"]["theKit"]["unsavedChanges"]["items"][0]["edits"]["items"][0]["forwards"]["items"][0];
            assert_eq!(fwd0["kind"].as_str(), Some("renameKit"), "bundle forward step kind");
            let kd = fwd0.get("kitDiff").expect("bundle must persist kitDiff beside operation input");
            assert_eq!(kd["name"].as_str(), Some("Hello Bundle"), "renameKit kitDiff wire carries new name");

            let rt_b = crate::worker::ParentStore::spawn().await;
            crate::kit_backbone::DevBackboneBundleDoc::hydrate_into_graph(&rt_b.wip_graph, &json_a).await.expect("hydrate");

            let json_b = serde_json::to_string(&crate::kit_backbone::DevBackboneBundleDoc::from_graph(rt_b.wip_graph.as_ref()).await).expect("serialize bundle b");
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

            let q = r#"{ store { wip { alternatives { edges { node { id name } } } } } }"#;
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "alternatives query errors: {:?}", res.errors);
            let v = res.data.into_json().unwrap();
            let edges = v["store"]["wip"]["alternatives"]["edges"].as_array().expect("edges");
            assert!(edges.iter().any(|e| e["node"]["id"].as_str() == Some(id.as_str())), "expected new alternative id in wip.alternatives");
            assert!(edges.iter().any(|e| e["node"]["name"].as_str() == Some("branch-a")));
        });
    }

    #[test]
    fn transaction_open_commit_abort_lifecycle_on_wip_graph() {
        block_on(async {
            let rt = crate::worker::ParentStore::spawn().await;
            let g = &rt.wip_graph;
            g.ensure_default_checkpoint_for_the_kit().await;
            let workspace_id = g.id.clone();
            let tx_a = g.open_transaction(&workspace_id).await;
            assert_eq!(g.the_kit_open_edit.read().await.upgrade().map(|t| t.id.clone()), Some(tx_a.id.clone()));
            let ordered: Vec<crate::id::Id> = g.the_kit_unsaved_edits.read().await.iter().map(|t| t.id.clone()).collect();
            assert_eq!(ordered, vec![tx_a.id.clone()]);

            g.commit_transaction(&workspace_id, &tx_a.id).await.expect("commit");
            assert!(g.the_kit_open_edit.read().await.upgrade().is_none());
            assert!(g.the_kit_unsaved_edits.read().await.is_empty());
            assert_eq!(g.the_kit_saved_edits.read().await.len(), 1);

            let tx_b = g.open_transaction(&workspace_id).await;
            g.abort_transaction(&workspace_id, &tx_b.id).await.expect("abort");
            assert!(g.the_kit_open_edit.read().await.upgrade().is_none());
            assert!(g.the_kit_unsaved_edits.read().await.is_empty());

            assert!(g.commit_transaction(&workspace_id, &crate::id::Id::from("missing")).await.is_err());
            assert!(g.abort_transaction(&crate::id::Id::from("missing-workspace"), &tx_b.id).await.is_err());
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

            let q = "{ store { wip { theKit { kit { tags { edges { node { name } } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["store"]["wip"]["theKit"]["kit"]["tags"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
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

            let q = "{ store { wip { theKit { kit { concepts { edges { node { name } } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let names: Vec<String> = data["store"]["wip"]["theKit"]["kit"]["concepts"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["name"].as_str().map(String::from)).collect();
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

            let q = "{ store { wip { theKit { kit { qualities { edges { node { key value } } } } } } } }";
            let res = schema.execute(q).await;
            assert!(res.errors.is_empty(), "query errors: {:?}", res.errors);
            let data = res.data.into_json().unwrap();
            let keys: Vec<String> = data["store"]["wip"]["theKit"]["kit"]["qualities"]["edges"].as_array().unwrap().iter().filter_map(|e| e["node"]["key"].as_str().map(String::from)).collect();
            assert!(keys.iter().any(|k| k == "q1"), "qualities missing new key: {:?}", keys);
        });
    }

    #[test]
    fn graph_operation_registry_row_count() {
        assert_eq!(crate::operation::GRAPH_OPERATION_REGISTRY_ROWS, 98);
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
            let edges = data["store"]["wip"]["theKit"]["kit"]["designs"]["edges"].as_array().expect("design edges");
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
            let edges = data["store"]["authoritative"]["theKit"]["kit"]["designs"]["edges"].as_array().expect("auth design edges");
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
            let rt = crate::worker::ParentStore::spawn().await;
            let schema = crate::gql::build_schema_for(rt.clone());

            rt.wip_graph.ensure_default_checkpoint_for_the_kit().await;
            let workspace_id = rt.wip_graph.id.clone();
            let tx = rt.wip_graph.open_transaction(&workspace_id).await;

            // Insert two pieces directly via the wip graph (no GraphQL plumbing).
            let position = crate::geom::PositionInput::default();
            let blueprint_id = crate::id::Id::new().await;
            let p1 = rt.wip_graph.apply_create_fixed_piece(workspace_id.clone(), tx.id.clone(), crate::id::Id::from("des1"), blueprint_id.clone(), position, None, None).await.expect("insert piece 1").0;
            let _p2 = rt.wip_graph.apply_create_fixed_piece(workspace_id.clone(), tx.id.clone(), crate::id::Id::from("des1"), blueprint_id, position, None, None).await.expect("insert piece 2").0;

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
        let Some(edges) = data["store"]["wip"]["theKit"]["kit"]["designs"]["edges"].as_array() else {
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
    fn kit_store_golden_operations_replay_matches_expected_invariants() {
        block_on(async {
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.ops")).expect("parse operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let workspace_id = g.id.clone();
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            let golden_ops = crate::kit_backbone::golden_operation_records_ref(&ops_json).expect("operations|ops");
            for rec in golden_ops {
                let kind = rec["kind"].as_str().expect("operation kind");
                let input = &rec["input"];
                match kind {
                    "createdFixedPiece" => {
                        let design_id = crate::id::Id::from(input["designId"].as_str().expect("designId"));
                        let blueprint_id = crate::id::Id::from(input["blueprintId"].as_str().expect("blueprintId"));
                        let position = crate::kit_backbone::position_input_from_json(&input["position"]).expect("position from golden json");
                        let name = input.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let description = input.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                        g.apply_create_fixed_piece(workspace_id.clone(), tx_id.clone(), design_id, blueprint_id, position, name, description).await.expect("apply createFixedPiece");
                    }
                    other => panic!("unsupported golden operation kind: {other}"),
                }
            }

            let inv = &exp["invariants"];
            let kit = g.materialized_kit_for_workspace(&workspace_id).await;
            let ds = kit.designs.read().await;
            assert_eq!(ds.len(), inv["designCount"].as_u64().expect("designCount") as usize, "designCount");
            let mut total = 0usize;
            let mut centers: Vec<[f64; 2]> = Vec::new();
            for d in ds.iter() {
                for p in d.pieces.read().await.iter() {
                    total += 1;
                    let guard = p.position.read().await;
                    let n = guard.as_ref().expect("piece position");
                    let pv = n.snapshot_input().await;
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

            let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&workspace_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint in kit-store.golden.expected.semio.json");
            assert_eq!(fp, exp_fp, "projectionFingerprint");
        });
    }

    /// 🪡 `kit_graph_engine::apply_kit_operation` must replay the same golden operations as manual apply.
    #[test]
    fn kit_store_golden_operations_via_kit_graph_engine_match_fingerprint() {
        block_on(async {
            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let ops_json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read kit-store.golden.ops")).expect("parse operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read kit-store.golden.expected")).expect("parse expected");

            let g = crate::vcs::Graph::new().await;
            let workspace_id = g.id.clone();
            let tx_id = crate::id::Id::from(ops_json["transactionId"].as_str().expect("transactionId"));
            let golden_ops = crate::kit_backbone::golden_operation_records_ref(&ops_json).expect("operations|ops");
            for rec in golden_ops {
                let kind = rec["kind"].as_str().expect("operation kind");
                let input = rec.get("input").expect("input");
                let op = crate::kit_backbone::kit_operation_from_stored(kind, input).await.expect("kit_operation_from_stored");
                let applied = crate::kit_graph_engine::apply_kit_operation(&g, &workspace_id, &tx_id, op).await.expect("apply_kit_operation");
                assert!(applied.created_piece.is_some(), "expected piece for {kind}");
                assert!(applied.diff.summary.as_ref().map(|s| !s.is_empty()).unwrap_or(false), "diff summary");
            }

            let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&workspace_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "projectionFingerprint via typed apply");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dev_json_backbone_persisted_operations_replay_matches_us001_projection_fingerprint() {
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let path = dir.path().join("dev-kit.json");

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read operations")).expect("parse golden operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let g = crate::vcs::Graph::new().await;
            let legacy_workspace = golden_ops["draftId"].as_str().expect("draftId");
            let graph_workspace = g.id.as_str().to_string();
            let mut stored = crate::kit_backbone::stored_operations_from_golden_operations_json(&golden_ops).expect("golden → stored operations");
            for op in &mut stored {
                if op.workspace_id == legacy_workspace {
                    op.workspace_id = graph_workspace.clone();
                }
            }
            let uri_full = format!("file://{}", path.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_full);
            let bundle = crate::kit_backbone::DevBackboneBundleDoc::from_stored__operations(&stored);
            std::fs::write(&path, serde_json::to_string_pretty(&bundle).expect("serialize kit-store bundle")).expect("write kit-store bundle");

            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g).await.expect("dev json mount+replay");

            let workspace_id = g.id.clone();
            let fp = stable_projection_fingerprint(&g.materialized_kit_for_workspace(&workspace_id).await).await;
            let exp_fp = exp["projectionFingerprint"].as_str().expect("projectionFingerprint");
            assert_eq!(fp, exp_fp, "dev-json backbone replay must match US-001 golden fingerprint");
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn local_semio_sqlite_backbone_persisted_operations_replay_matches_us001_projection_fingerprint() {
        block_on(async {
            let dir = tempfile::tempdir().expect("temp dir");
            let proj_root = dir.path().join("workspace");
            std::fs::create_dir_all(&proj_root).expect("mkdir workspace");
            let proj_canon = proj_root.canonicalize().expect("canonical workspace");
            let uri_local = format!("local://{}", proj_canon.display());
            let norm = crate::kit_backbone::normalize_connection_uri(&uri_local);

            let path_ops = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.ops.semio.json");
            let path_exp = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/semio/kit-store.golden.expected.semio.json");
            let golden_ops: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_ops).expect("read operations")).expect("parse golden operations");
            let exp: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path_exp).expect("read expected")).expect("parse golden expected");

            let g_bootstrap = crate::vcs::Graph::new().await;
            let _bones = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g_bootstrap).await.expect("bootstrap .semio layout");

            let g2 = crate::vcs::Graph::new().await;
            let legacy_workspace = golden_ops["draftId"].as_str().expect("draftId");
            let graph_workspace = g2.id.as_str().to_string();
            let mut stored = crate::kit_backbone::stored_operations_from_golden_operations_json(&golden_ops).expect("golden → stored operations");
            for op in &mut stored {
                if op.workspace_id == legacy_workspace {
                    op.workspace_id = graph_workspace.clone();
                }
            }

            let db_path = proj_canon.join(".semio").join("wip.db");
            let conn = rusqlite::Connection::open(&db_path).expect("open wip.db");
            for operation in &stored {
                let input_json = serde_json::to_string(&operation.input).expect("input json");
                conn.execute(
                    "INSERT INTO _operation_log (draft_id, transaction_id, kind, input_json, kit_diff_json) VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![operation.workspace_id, operation.transaction_id, operation.kind, input_json, operation.kit_diff.as_ref().map(|v| serde_json::to_string(v).expect("kit diff json"))],
                )
                .expect("insert  operation entity");
            }
            drop(conn);

            crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g2).await.expect("replay wip.db");

            let workspace_id = g2.id.clone();
            let fp = stable_projection_fingerprint(&g2.materialized_kit_for_workspace(&workspace_id).await).await;
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
        let bundle = crate::kit_backbone::DevBackboneBundleDoc::template();
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
    fn kit_bundle_purge_unreferenced_blob_entities() {
        let mut bundle = crate::kit_backbone::DevBackboneBundleDoc::template();
        bundle.blobs.items.push(serde_json::json!({ "hash": "orphan_digest_deadbeef", "blob": "data:,x" }));
        bundle.wip.initial_kit = serde_json::json!({
            "id": "k-purge",
            "name": "K",
            "createdAt": "2020-01-01T00:00:00.000Z",
            "updatedAt": "2020-01-01T00:00:00.000Z",
            "files": [],
        });
        crate::kit_backbone::DevBackboneBundleDoc::purge_unreferenced_blobs(&mut bundle);
        assert!(bundle.blobs.items.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_bundle_hoist_and_materialize_file_blobs_round_trip() {
        let blob_txt = "data:application/octet-stream;base64,QQ==";
        let dig = crate::kit_backbone::DevBackboneBundleDoc::digest_kit_blob_wire(blob_txt);
        let mut bundle = crate::kit_backbone::DevBackboneBundleDoc::template();
        bundle.wip.initial_kit = serde_json::json!({
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
        crate::kit_backbone::DevBackboneBundleDoc::hoist_inline_file_blobs_for_storage(&mut bundle);
        assert!(bundle.wip.initial_kit["files"][0].as_object().expect("file obj").get("blob").is_none());
        assert_eq!(bundle.wip.initial_kit["files"][0]["blobHash"].as_str().expect("blobHash"), dig);
        assert_eq!(bundle.blobs.items.len(), 1);
        assert_eq!(bundle.blobs.items[0]["hash"].as_str().expect("blob entity hash"), dig);
        let mut merged = bundle.wip.initial_kit.clone();
        crate::kit_backbone::DevBackboneBundleDoc::merge_bundle_file_blobs_into_kit_json(&mut merged, &bundle.blobs.items);
        assert_eq!(merged["files"][0]["blob"].as_str().expect("merged blob"), blob_txt);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_initialize_with_unsaved_change_seeds_checkpoint_and_change() {
        // 🌱 Bundle bootstrap matches sketchpad "create dev kit (json)": one `initialKit`, one checkpoint, one unsaved change on the version.
        let bundle = crate::kit_backbone::DevBackboneBundleDoc::initialize_with_unsaved_change("kit-id-1", "change-1", "ckpt-1");
        assert_eq!(bundle.schema, crate::kit_backbone::KIT_STORE_BUNDLE_SCHEMA);
        assert_eq!(bundle.wip.id, "kit-id-1");
        assert_eq!(bundle.authoritative.id, "kit-id-1");
        assert_eq!(bundle.stage.id, "kit-id-1");
        assert_eq!(bundle.wip.checkpoints.items.len(), 1, "first checkpoint anchored on initial kit");
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
            let mut bone = crate::kit_backbone::AttachedBackbone::mount_and_replay(&norm, "wip", &g).await.expect("mount empty bundle");
            let workspace_id = crate::id::Id::from("draft-rs-1");
            let tx_id = crate::id::Id::from("tx-rs-1");
            bone.append__operation(&workspace_id, &tx_id, "kit.design.piece.createdFixedPiece", &serde_json::json!({"designId": "d-1", "blueprintId": "b-1"}), None).expect("append operation");

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
        let mut bundle = crate::kit_backbone::DevBackboneBundleDoc::template();
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
        let flat = bundle.wip__operations();
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].workspace_id, "the-kit");
        assert_eq!(flat[0].transaction_id, "change-y");
        assert_eq!(flat[0].kind, "kit.design.piece.createdFixedPiece");
        assert_eq!(flat[1].kind, "kit.design.piece.deletedFixedPieces");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn kit_store_bundle_projects_alternative_workspace_changes() {
        block_on(async {
            let graph = crate::vcs::Graph::new().await;
            let alt_id = graph.create_alternative_from_tip("branch-a".to_string(), None).await.expect("alternative");
            let bundle = crate::kit_backbone::DevBackboneBundleDoc::from_graph(graph.as_ref()).await;
            assert!(bundle.wip.the_kit.saved_changes.items.is_empty());
            let alt = bundle.wip.alternatives.items.iter().find(|a| a.id == alt_id.as_str()).expect("alternative json");
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
            let kit = graph.mutable_kit.read().await.clone();
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
            let create = crate::operation::Operation::CreateTag {
                scope: crate::operation::Scope::CreateTag { owner_id: owner_id.clone(), tag_id: tag_id.clone(), attribute_ids: vec![attribute_id.clone()] },
                input: crate::operation::Input::Tag { tag: tag_input.clone() },
            };

            let diff = create.to_diff(&kit).await.expect("createTag diff");
            let tags = diff.0.tags.as_ref().expect("tags collection diff");
            assert_eq!(tags.added.len(), 1, "single added tag entity");
            let entity = &tags.added[0];
            assert_eq!(entity.id, tag_id);
            assert_eq!(entity.owner_id, owner_id);
            assert_eq!(entity.tag.name, "alpha-tag");

            let staged = kit.deep_clone().await;
            staged.apply_diff(&diff).await.expect("apply createTag diff on clone");

            let backwards = crate::operation::Operation::DeleteTag { scope: crate::operation::Scope::Tag { tag_id: tag_id.clone() }, input: crate::operation::Input::None }.to_backwards(&staged).await.expect("deleteTag backwards");
            assert_eq!(backwards.len(), 1);
            match &backwards[0] {
                crate::operation::Operation::CreateTag { scope, input } => {
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

    /// @emoji 📦 `metabolism.kit.diff.semio.json` parses as JSON and exposes expected top-level contract keys (typed [`crate::operation::CanonicalKitDiff`] lives on the GraphQL control plane only).
    #[test]
    fn canonical_kit_diff_metabolism_fixture_has_contract_keys() {
        const FIXTURE: &str = include_str!("../../../assets/fixtures/metabolism.kit.diff.semio.json");
        let raw: serde_json::Value = serde_json::from_str(FIXTURE).expect("fixture parses as JSON");
        assert_eq!(raw.get("name").and_then(|v| v.as_str()), Some("Metabolism Modified"));
        assert!(raw.get("types").is_some(), "fixture must include types collection");
        assert!(raw.get("designs").is_some(), "fixture must include designs collection");
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
    fn file_blob_digest_field_is_exposed_on_entity() {
        let f = crate::meta::File { id: crate::id::Id::from("019caa00-0000-7000-a000-000000000021"), url: "https://example.com/f".to_string(), mime: None, size: None, hash: "sha256:abc".to_string(), description: None, created: None, updated: None };
        assert_eq!(f.hash, "sha256:abc");
    }

    #[test]
    fn geom_plane_compute_hash_stable() {
        block_on(async {
            let pl = crate::geom::entity::Plane::from_input(crate::geom::PlaneInput {
                origin: crate::geom::PointInput { x: 0.0, y: 0.0, z: 0.0 },
                x_axis: crate::geom::VectorInput { x: 1.0, y: 0.0, z: 0.0 },
                y_axis: crate::geom::VectorInput { x: 0.0, y: 1.0, z: 0.0 },
            });
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
            let workspace_id = graph.id.clone();
            let operation = crate::operation::Operation::CreateFixedPiece {
                scope: crate::operation::Scope::CreateFixedPiece {
                    design_id: crate::id::Id::from("design-scoped-1"),
                    piece_id: crate::id::Id::from("piece-scoped-1"),
                    blueprint_id: crate::id::Id::from("blueprint-scoped-1"),
                    attribute_ids: Vec::new(),
                },
                input: crate::operation::Input::FixedPiece { position: crate::geom::PositionInput::default(), name: Some("Scoped Piece".to_string()), description: Some("Persisted with explicit scope ids".to_string()) },
            };

            let applied = crate::kit_graph_engine::apply_kit_operation(&graph, &workspace_id, &crate::id::Id::from("tx-scoped-1"), operation).await.expect("apply normalized createFixedPiece");

            let piece = applied.created_piece.expect("created piece");
            assert_eq!(piece.id, crate::id::Id::from("piece-scoped-1"));
            assert_eq!(piece.name.read().await.clone().as_deref(), Some("Scoped Piece"));

            let mat = graph.materialized_kit_for_workspace(&workspace_id).await;
            let design = mat.design_by_external_id(&crate::id::Id::from("design-scoped-1")).await.expect("design exists");
            assert!(design.piece_by_external_id(&crate::id::Id::from("piece-scoped-1")).await.is_some(), "piece should be addressable by scoped id");
        });
    }
}

//#endregion 🧪 tests
