//! 🦀 semio rust control plane — greenfield skeleton.
//!
//! Layout matches `semio/graphql/target.schema.graphql`. Every GraphQL type is one Rust struct
//! with **two impl blocks**: a main Rust impl (all `pub fn` async) and an `#[Object]` impl
//! exposing the GraphQL resolvers. There is exactly **one** `emit_event` in the entire crate
//! ([`event::EventBus::emit_event`]); every mutation routes through it.
//!
//! Worker topology: a parent router hosts the GraphQL schema and dispatches commands to two
//! child workers (`wip` + `authoritative`), each owning its own [`vcs::Graph`]. On native targets
//! both children run as in-process async actors; on `wasm32` they live in dedicated web workers
//! wired through [`wasm_bridge`].

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
    //! 📐 Pure value geometry: vectors, points, planes, poses.
    use async_graphql::{InputObject, SimpleObject};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Vector", input_name = "VectorInput")]
    pub struct Vector {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Point", input_name = "PointInput")]
    pub struct Point {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Coordinate", input_name = "CoordinateInput")]
    pub struct Coordinate {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Offset", input_name = "OffsetInput")]
    pub struct Offset {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Plane", input_name = "PlaneInput")]
    pub struct Plane {
        pub origin: Point,
        #[graphql(name = "xAxis")]
        pub x_axis: Vector,
        #[graphql(name = "yAxis")]
        pub y_axis: Vector,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, SimpleObject, InputObject)]
    #[graphql(name = "Position", input_name = "PositionInput")]
    pub struct Position {
        pub center: Coordinate,
        pub plane: Plane,
    }
}

//#endregion 📐 geom

//#region 🏷️ meta

pub mod meta {
    //! 🏷️ Strong shared metadata entities (location, file, folder, author, attribute, …).
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
        // `pieces: [Piece!]!` is resolved at the Design level for the skeleton.
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
    //! 📦 Kit ↔ Type ↔ Design entity tree.

    //#region 🏠 type
    pub mod r#type {
        //! 🏠 Types, their connectors and representations.
        use async_graphql::Object;
        use serde::{Deserialize, Serialize};

        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Concept, Prop, Quality, Stat, Tag, File};
        use crate::timestamp::Timestamp;

        //#region ⚓ connector
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct Connector {
            pub id: Id,
            pub owner_type_id: Id,
            pub code: String,
            pub description: Option<String>,
            pub port_id: Option<Id>,
            pub qualities: Vec<Quality>,
            pub attributes: Vec<Attribute>,
        }

        impl Connector {
            pub async fn new(owner_type_id: Id, code: String) -> Self {
                Self { id: Id::new().await, owner_type_id, code, ..Default::default() }
            }

            pub async fn compute_hash(&self) -> String {
                h(&[self.id.as_str(), &self.code, self.description.as_deref().unwrap_or("")])
            }
        }

        #[Object(name = "Connector")]
        impl Connector {
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                Connector::compute_hash(self).await
            }
            /// Owner [`Type`] resolved by id at the [`crate::kit::Kit`] root.
            async fn owner(&self) -> Type {
                Type { id: self.owner_type_id.clone(), ..Default::default() }
            }
            async fn code(&self) -> String {
                self.code.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.clone()
            }
            #[graphql(name = "portId")]
            async fn port_id(&self) -> Option<Id> {
                self.port_id.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.clone()
            }
        }
        //#endregion ⚓ connector

        //#region 💾 representation
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct Representation {
            pub id: Id,
            pub owner_type_id: Id,
            pub url: String,
            pub description: Option<String>,
            pub file: Option<File>,
            pub tags: Vec<Tag>,
            pub qualities: Vec<Quality>,
            pub attributes: Vec<Attribute>,
        }

        impl Representation {
            pub async fn new(owner_type_id: Id, url: String) -> Self {
                Self { id: Id::new().await, owner_type_id, url, ..Default::default() }
            }
            pub async fn compute_hash(&self) -> String {
                h(&[self.id.as_str(), &self.url, self.description.as_deref().unwrap_or("")])
            }
        }

        #[Object(name = "Representation")]
        impl Representation {
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                Representation::compute_hash(self).await
            }
            async fn owner(&self) -> Type {
                Type { id: self.owner_type_id.clone(), ..Default::default() }
            }
            async fn url(&self) -> String {
                self.url.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.clone()
            }
            async fn file(&self) -> Option<File> {
                self.file.clone()
            }
            async fn tags(&self) -> Vec<Tag> {
                self.tags.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.clone()
            }
        }
        //#endregion 💾 representation

        //#region 🏠 type
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct Type {
            pub id: Id,
            pub owner_kit_id: Id,
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub image: Option<String>,
            pub unit: Option<String>,
            pub created: Option<Timestamp>,
            pub updated: Option<Timestamp>,
            pub connectors: Vec<Connector>,
            pub representations: Vec<Representation>,
            pub authors: Vec<Author>,
            pub concepts: Vec<Concept>,
            pub tags: Vec<Tag>,
            pub qualities: Vec<Quality>,
            pub props: Vec<Prop>,
            pub attributes: Vec<Attribute>,
            pub stats: Vec<Stat>,
        }

        impl Type {
            pub async fn new(owner_kit_id: Id, name: String) -> Self {
                Self { id: Id::new().await, owner_kit_id, name, ..Default::default() }
            }
            pub async fn compute_hash(&self) -> String {
                h(&[self.id.as_str(), &self.name, self.description.as_deref().unwrap_or("")])
            }
            pub async fn connector_by_id(&self, id: &Id) -> Option<Connector> {
                self.connectors.iter().find(|c| &c.id == id).cloned()
            }
            pub async fn representation_by_id(&self, id: &Id) -> Option<Representation> {
                self.representations.iter().find(|r| &r.id == id).cloned()
            }
            pub async fn best_representation_for_tags(&self, tag_ids: &[Id]) -> Option<Representation> {
                let want: std::collections::HashSet<&Id> = tag_ids.iter().collect();
                self.representations
                    .iter()
                    .max_by_key(|r| r.tags.iter().filter(|t| want.contains(&t.id)).count())
                    .cloned()
            }
        }

        #[Object(name = "Type")]
        impl Type {
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                Type::compute_hash(self).await
            }
            async fn owner(&self) -> crate::kit::Kit {
                crate::kit::Kit { id: self.owner_kit_id.clone(), ..Default::default() }
            }
            async fn name(&self) -> String { self.name.clone() }
            async fn description(&self) -> Option<String> { self.description.clone() }
            async fn icon(&self) -> Option<String> { self.icon.clone() }
            async fn image(&self) -> Option<String> { self.image.clone() }
            async fn unit(&self) -> Option<String> { self.unit.clone() }
            async fn created(&self) -> Option<Timestamp> { self.created.clone() }
            async fn updated(&self) -> Option<Timestamp> { self.updated.clone() }
            async fn connectors(&self) -> Vec<Connector> {
                self.connectors.clone()
            }
            async fn connector(&self, id: Id) -> Option<Connector> {
                Type::connector_by_id(self, &id).await
            }
            async fn representations(&self) -> Vec<Representation> {
                self.representations.clone()
            }
            async fn representation(&self, id: Id) -> Option<Representation> {
                Type::representation_by_id(self, &id).await
            }
            #[graphql(name = "bestRepresentation")]
            async fn best_representation(&self, tag_ids: Vec<Id>) -> Option<Representation> {
                Type::best_representation_for_tags(self, &tag_ids).await
            }
            async fn authors(&self) -> Vec<Author> {
                self.authors.clone()
            }
            async fn concepts(&self) -> Vec<Concept> {
                self.concepts.clone()
            }
            async fn tags(&self) -> Vec<Tag> {
                self.tags.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.clone()
            }
            async fn props(&self) -> Vec<Prop> {
                self.props.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.clone()
            }
            async fn stats(&self) -> Vec<Stat> {
                self.stats.clone()
            }
        }
        //#endregion 🏠 type

        // 🧩 Blueprint union (Type | Design) — shared by Piece.
        #[derive(Clone, Debug, async_graphql::Union)]
        #[graphql(name = "Blueprint")]
        pub enum Blueprint {
            Type(Type),
            Design(super::design::Design),
        }
    }
    //#endregion 🏠 type

    //#region 🏘 design
    pub mod design {
        //! 🏘 Designs and their pieces, connections, layers, groups.

        //#region ⭕ piece
        pub mod piece {
            //! ⭕ Piece (instance of a Type or Design within a Design).
            use async_graphql::{Enum, Object};
            use serde::{Deserialize, Serialize};

            use crate::geom::Position;
            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::{Attribute, Prop};

            #[derive(Clone, Copy, Debug, Eq, PartialEq, Enum, Serialize, Deserialize)]
            #[graphql(name = "PieceConnectionKind")]
            pub enum PieceConnectionKind {
                #[graphql(name = "FIXED")]
                Fixed,
                #[graphql(name = "CONNECTED")]
                Connected,
            }

            impl Default for PieceConnectionKind {
                fn default() -> Self {
                    Self::Fixed
                }
            }

            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct Piece {
                pub id: Id,
                pub owner_design_id: Id,
                pub name: Option<String>,
                pub description: Option<String>,
                pub pose: Option<Position>,
                pub scale: Option<f64>,
                pub blueprint_type_id: Option<Id>,
                pub blueprint_design_id: Option<Id>,
                pub connection_kind: Option<PieceConnectionKind>,
                pub parent_connection_id: Option<Id>,
                pub parent_piece_id: Option<Id>,
                pub child_piece_ids: Vec<Id>,
                pub child_connection_ids: Vec<Id>,
                pub depth: i32,
                pub path: Vec<Id>,
                pub props: Vec<Prop>,
                pub attributes: Vec<Attribute>,
            }

            impl Piece {
                pub async fn new_fixed(owner_design_id: Id, blueprint_type_id: Id, pose: Position) -> Self {
                    Self {
                        id: Id::new().await,
                        owner_design_id,
                        pose: Some(pose),
                        blueprint_type_id: Some(blueprint_type_id),
                        connection_kind: Some(PieceConnectionKind::Fixed),
                        ..Default::default()
                    }
                }

                pub async fn compute_hash(&self) -> String {
                    h(&[
                        self.id.as_str(),
                        self.name.as_deref().unwrap_or(""),
                        self.blueprint_type_id.as_ref().map(|i| i.as_str()).unwrap_or(""),
                        self.blueprint_design_id.as_ref().map(|i| i.as_str()).unwrap_or(""),
                    ])
                }

                pub async fn compute_flat_position(&self) -> Position {
                    // Skeleton: when fixed, flat == pose; for connected pieces full layout will land in a follow-up ticket.
                    self.pose.unwrap_or_default()
                }
            }

            #[Object(name = "Piece")]
            impl Piece {
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn hash(&self) -> String {
                    Piece::compute_hash(self).await
                }
                async fn owner(&self) -> super::Design {
                    super::Design { id: self.owner_design_id.clone(), ..Default::default() }
                }
                async fn name(&self) -> Option<String> {
                    self.name.clone()
                }
                async fn description(&self) -> Option<String> {
                    self.description.clone()
                }
                async fn pose(&self) -> Option<Position> {
                    self.pose
                }
                async fn scale(&self) -> Option<f64> {
                    self.scale
                }
                async fn blueprint(&self) -> Option<super::super::r#type::Blueprint> {
                    if let Some(tid) = &self.blueprint_type_id {
                        Some(super::super::r#type::Blueprint::Type(super::super::r#type::Type {
                            id: tid.clone(),
                            ..Default::default()
                        }))
                    } else if let Some(did) = &self.blueprint_design_id {
                        Some(super::super::r#type::Blueprint::Design(super::Design {
                            id: did.clone(),
                            ..Default::default()
                        }))
                    } else {
                        None
                    }
                }
                #[graphql(name = "connectionKind")]
                async fn connection_kind(&self) -> Option<PieceConnectionKind> {
                    self.connection_kind
                }
                #[graphql(name = "flatPosition")]
                async fn flat_position(&self) -> Position {
                    Piece::compute_flat_position(self).await
                }
                #[graphql(name = "replaceableBlueprint")]
                async fn replaceable_blueprint(&self) -> Vec<super::super::r#type::Blueprint> {
                    Vec::new()
                }
                #[graphql(name = "parentConnection")]
                async fn parent_connection(&self) -> Option<super::connection::Connection> {
                    self.parent_connection_id.as_ref().map(|id| super::connection::Connection {
                        id: id.clone(),
                        ..Default::default()
                    })
                }
                #[graphql(name = "childConnections")]
                async fn child_connections(&self) -> Vec<super::connection::Connection> {
                    self.child_connection_ids
                        .iter()
                        .map(|id| super::connection::Connection { id: id.clone(), ..Default::default() })
                        .collect()
                }
                #[graphql(name = "parentPiece")]
                async fn parent_piece(&self) -> Option<Piece> {
                    self.parent_piece_id.as_ref().map(|id| Piece { id: id.clone(), ..Default::default() })
                }
                #[graphql(name = "childPieces")]
                async fn child_pieces(&self) -> Vec<Piece> {
                    self.child_piece_ids
                        .iter()
                        .map(|id| Piece { id: id.clone(), ..Default::default() })
                        .collect()
                }
                async fn depth(&self) -> i32 {
                    self.depth
                }
                async fn path(&self) -> Vec<Piece> {
                    self.path.iter().map(|id| Piece { id: id.clone(), ..Default::default() }).collect()
                }
                async fn props(&self) -> Vec<Prop> {
                    self.props.clone()
                }
                async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.clone()
                }
            }
        }
        //#endregion ⭕ piece

        //#region 🔗 connection
        pub mod connection {
            //! 🔗 Connection between two piece sides + the Side value.
            use async_graphql::Object;
            use serde::{Deserialize, Serialize};

            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::Attribute;

            //#region ⛓️ side
            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct Side {
                pub id: Id,
                pub piece_id: Id,
                pub port_id: Option<Id>,
                pub design_piece_id: Option<Id>,
                pub connector_id: Option<Id>,
            }

            impl Side {
                pub async fn new(piece_id: Id) -> Self {
                    Self { id: Id::new().await, piece_id, ..Default::default() }
                }
            }

            #[Object(name = "Side")]
            impl Side {
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn piece(&self) -> super::piece::Piece {
                    super::piece::Piece { id: self.piece_id.clone(), ..Default::default() }
                }
                async fn port(&self) -> Option<super::super::r#type::Connector> {
                    self.port_id
                        .as_ref()
                        .map(|id| super::super::r#type::Connector { id: id.clone(), ..Default::default() })
                }
                #[graphql(name = "designPiece")]
                async fn design_piece(&self) -> Option<super::piece::Piece> {
                    self.design_piece_id
                        .as_ref()
                        .map(|id| super::piece::Piece { id: id.clone(), ..Default::default() })
                }
                async fn connector(&self) -> Option<super::super::r#type::Connector> {
                    self.connector_id
                        .as_ref()
                        .map(|id| super::super::r#type::Connector { id: id.clone(), ..Default::default() })
                }
            }
            //#endregion ⛓️ side

            //#region 🔗 connection
            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct Connection {
                pub id: Id,
                pub owner_design_id: Id,
                pub connected: Side,
                pub connecting: Side,
                pub gap: Option<f64>,
                pub shift: Option<f64>,
                pub rise: Option<f64>,
                pub rotation: Option<f64>,
                pub turn: Option<f64>,
                pub tilt: Option<f64>,
                pub u: Option<f64>,
                pub v: Option<f64>,
                pub description: Option<String>,
                pub attributes: Vec<Attribute>,
            }

            impl Connection {
                pub async fn compute_hash(&self) -> String {
                    h(&[self.id.as_str(), self.connected.piece_id.as_str(), self.connecting.piece_id.as_str()])
                }
            }

            #[Object(name = "Connection")]
            impl Connection {
                async fn id(&self) -> Id {
                    self.id.clone()
                }
                async fn hash(&self) -> String {
                    Connection::compute_hash(self).await
                }
                async fn owner(&self) -> super::Design {
                    super::Design { id: self.owner_design_id.clone(), ..Default::default() }
                }
                async fn connected(&self) -> Side {
                    self.connected.clone()
                }
                async fn connecting(&self) -> Side {
                    self.connecting.clone()
                }
                async fn gap(&self) -> Option<f64> {
                    self.gap
                }
                async fn shift(&self) -> Option<f64> {
                    self.shift
                }
                async fn rise(&self) -> Option<f64> {
                    self.rise
                }
                async fn rotation(&self) -> Option<f64> {
                    self.rotation
                }
                async fn turn(&self) -> Option<f64> {
                    self.turn
                }
                async fn tilt(&self) -> Option<f64> {
                    self.tilt
                }
                async fn u(&self) -> Option<f64> {
                    self.u
                }
                async fn v(&self) -> Option<f64> {
                    self.v
                }
                async fn description(&self) -> Option<String> {
                    self.description.clone()
                }
                async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.clone()
                }
            }
            //#endregion 🔗 connection
        }
        //#endregion 🔗 connection

        //#region 🏘 design
        use async_graphql::Object;
        use serde::{Deserialize, Serialize};

        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Concept, Group, Layer, Location, Prop, Quality, Stat, Tag};
        use crate::timestamp::Timestamp;

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct Design {
            pub id: Id,
            pub owner_kit_id: Id,
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub image: Option<String>,
            pub location: Option<Location>,
            pub unit: Option<String>,
            pub created: Option<Timestamp>,
            pub updated: Option<Timestamp>,
            pub pieces: Vec<piece::Piece>,
            pub connections: Vec<connection::Connection>,
            pub layers: Vec<Layer>,
            pub groups: Vec<Group>,
            pub authors: Vec<Author>,
            pub concepts: Vec<Concept>,
            pub tags: Vec<Tag>,
            pub qualities: Vec<Quality>,
            pub props: Vec<Prop>,
            pub attributes: Vec<Attribute>,
            pub stats: Vec<Stat>,
        }

        impl Design {
            pub async fn new(owner_kit_id: Id, name: String) -> Self {
                Self { id: Id::new().await, owner_kit_id, name, ..Default::default() }
            }

            pub async fn compute_hash(&self) -> String {
                h(&[self.id.as_str(), &self.name])
            }

            /// 🆕 Insert a piece into this design (returns the inserted clone).
            pub async fn insert_piece(&mut self, piece: piece::Piece) -> piece::Piece {
                self.pieces.push(piece.clone());
                piece
            }

            pub async fn piece_by_id(&self, id: &Id) -> Option<piece::Piece> {
                self.pieces.iter().find(|p| &p.id == id).cloned()
            }

            pub async fn connection_by_id(&self, id: &Id) -> Option<connection::Connection> {
                self.connections.iter().find(|c| &c.id == id).cloned()
            }
        }

        #[Object(name = "Design")]
        impl Design {
            async fn id(&self) -> Id {
                self.id.clone()
            }
            async fn hash(&self) -> String {
                Design::compute_hash(self).await
            }
            async fn owner(&self) -> super::Kit {
                super::Kit { id: self.owner_kit_id.clone(), ..Default::default() }
            }
            async fn name(&self) -> String {
                self.name.clone()
            }
            async fn description(&self) -> Option<String> {
                self.description.clone()
            }
            async fn icon(&self) -> Option<String> {
                self.icon.clone()
            }
            async fn image(&self) -> Option<String> {
                self.image.clone()
            }
            async fn location(&self) -> Option<Location> {
                self.location.clone()
            }
            async fn unit(&self) -> Option<String> {
                self.unit.clone()
            }
            async fn created(&self) -> Option<Timestamp> {
                self.created.clone()
            }
            async fn updated(&self) -> Option<Timestamp> {
                self.updated.clone()
            }
            async fn pieces(&self) -> Vec<piece::Piece> {
                self.pieces.clone()
            }
            async fn piece(&self, id: Id) -> Option<piece::Piece> {
                Design::piece_by_id(self, &id).await
            }
            async fn connections(&self) -> Vec<connection::Connection> {
                self.connections.clone()
            }
            async fn connection(&self, id: Id) -> Option<connection::Connection> {
                Design::connection_by_id(self, &id).await
            }
            async fn layers(&self) -> Vec<Layer> {
                self.layers.clone()
            }
            async fn groups(&self) -> Vec<Group> {
                self.groups.clone()
            }
            async fn authors(&self) -> Vec<Author> {
                self.authors.clone()
            }
            async fn concepts(&self) -> Vec<Concept> {
                self.concepts.clone()
            }
            async fn tags(&self) -> Vec<Tag> {
                self.tags.clone()
            }
            async fn qualities(&self) -> Vec<Quality> {
                self.qualities.clone()
            }
            async fn props(&self) -> Vec<Prop> {
                self.props.clone()
            }
            async fn attributes(&self) -> Vec<Attribute> {
                self.attributes.clone()
            }
            async fn stats(&self) -> Vec<Stat> {
                self.stats.clone()
            }
            #[graphql(name = "qualitySum")]
            async fn quality_sum(&self, _quality_id: Id) -> f64 {
                0.0
            }
            async fn references(&self) -> Vec<Design> {
                Vec::new()
            }
            #[graphql(name = "referencedBy")]
            async fn referenced_by(&self) -> Vec<piece::Piece> {
                Vec::new()
            }
        }
        //#endregion 🏘 design
    }
    //#endregion 🏘 design

    //#region 📦 kit
    use async_graphql::Object;
    use serde::{Deserialize, Serialize};

    use crate::hash::h;
    use crate::id::Id;
    use crate::meta::{Attribute, Author, Concept, File, Folder, Prop, Quality, Stat, Tag};
    use crate::timestamp::Timestamp;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Kit {
        pub id: Id,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub preview: Option<String>,
        pub remote: Option<String>,
        pub homepage: Option<String>,
        pub license: Option<String>,
        pub uri: Option<String>,
        pub created: Option<Timestamp>,
        pub updated: Option<Timestamp>,
        pub version: Option<String>,
        pub designs: Vec<design::Design>,
        pub types: Vec<r#type::Type>,
        pub files: Vec<File>,
        pub folders: Vec<Folder>,
        pub authors: Vec<Author>,
        pub concepts: Vec<Concept>,
        pub tags: Vec<Tag>,
        pub qualities: Vec<Quality>,
        pub props: Vec<Prop>,
        pub attributes: Vec<Attribute>,
        pub stats: Vec<Stat>,
    }

    impl Kit {
        pub async fn new(name: String) -> Self {
            Self { id: Id::new().await, name, ..Default::default() }
        }

        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str(), &self.name])
        }

        pub async fn design_by_id(&self, id: &Id) -> Option<design::Design> {
            self.designs.iter().find(|d| &d.id == id).cloned()
        }
        pub async fn type_by_id(&self, id: &Id) -> Option<r#type::Type> {
            self.types.iter().find(|t| &t.id == id).cloned()
        }
        pub async fn design_mut(&mut self, id: &Id) -> Option<&mut design::Design> {
            self.designs.iter_mut().find(|d| &d.id == id)
        }
    }

    #[Object(name = "Kit")]
    impl Kit {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            Kit::compute_hash(self).await
        }
        /// Owner [`crate::vcs::Graph`] (skeleton: filled in by [`crate::worker::ChildRuntime`]).
        async fn owner(&self) -> Option<crate::vcs::Graph> {
            None
        }
        async fn checkpoint(&self) -> Option<crate::vcs::Checkpoint> {
            None
        }
        async fn draft(&self) -> Option<crate::vcs::Draft> {
            None
        }
        async fn transaction(&self) -> Option<crate::vcs::Transaction> {
            None
        }
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn description(&self) -> Option<String> {
            self.description.clone()
        }
        async fn icon(&self) -> Option<String> {
            self.icon.clone()
        }
        async fn image(&self) -> Option<String> {
            self.image.clone()
        }
        async fn preview(&self) -> Option<String> {
            self.preview.clone()
        }
        async fn remote(&self) -> Option<String> {
            self.remote.clone()
        }
        async fn homepage(&self) -> Option<String> {
            self.homepage.clone()
        }
        async fn license(&self) -> Option<String> {
            self.license.clone()
        }
        async fn uri(&self) -> Option<String> {
            self.uri.clone()
        }
        async fn created(&self) -> Option<Timestamp> {
            self.created.clone()
        }
        async fn updated(&self) -> Option<Timestamp> {
            self.updated.clone()
        }
        async fn version(&self) -> Option<String> {
            self.version.clone()
        }
        async fn design(&self, id: Id) -> Option<design::Design> {
            Kit::design_by_id(self, &id).await
        }
        async fn designs(&self) -> Vec<design::Design> {
            self.designs.clone()
        }
        #[graphql(name = "type")]
        async fn type_(&self, id: Id) -> Option<r#type::Type> {
            Kit::type_by_id(self, &id).await
        }
        async fn types(&self) -> Vec<r#type::Type> {
            self.types.clone()
        }
        async fn files(&self) -> Vec<File> {
            self.files.clone()
        }
        async fn folders(&self) -> Vec<Folder> {
            self.folders.clone()
        }
        async fn authors(&self) -> Vec<Author> {
            self.authors.clone()
        }
        async fn concepts(&self) -> Vec<Concept> {
            self.concepts.clone()
        }
        async fn tags(&self) -> Vec<Tag> {
            self.tags.clone()
        }
        async fn qualities(&self) -> Vec<Quality> {
            self.qualities.clone()
        }
        async fn props(&self) -> Vec<Prop> {
            self.props.clone()
        }
        async fn attributes(&self) -> Vec<Attribute> {
            self.attributes.clone()
        }
        async fn stats(&self) -> Vec<Stat> {
            self.stats.clone()
        }
    }
    //#endregion 📦 kit
}

//#endregion 📦 kit

//#region 🌿 vcs

pub mod vcs {
    //! 🌿 Version-control entities — change/transaction/draft/checkpoint/alternative/graph/session/conflict.
    use async_graphql::{Object, Union};
    use serde::{Deserialize, Serialize};

    use crate::error::SemioError;
    use crate::hash::h;
    use crate::id::Id;
    use crate::kit::Kit;
    use crate::meta::Author;
    use crate::op;
    use crate::timestamp::Timestamp;

    //#region 🪪 change
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Change {
        pub id: Id,
        pub owner: Option<ChangeOwnerRef>,
        pub forwards: Vec<op::OperationKind>,
        pub backwards: Vec<op::OperationKind>,
    }

    /// 🔗 Untyped reference to one of the variants of the [`ChangeOwnerUnion`].
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ChangeOwnerRef {
        Transaction(Id),
        Draft(Id),
        Checkpoint(Id),
    }

    impl Change {
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
            Change::compute_hash(self).await
        }
        async fn owner(&self) -> ChangeOwnerUnion {
            match &self.owner {
                Some(ChangeOwnerRef::Transaction(id)) => ChangeOwnerUnion::Transaction(Transaction { id: id.clone(), ..Default::default() }),
                Some(ChangeOwnerRef::Draft(id)) => ChangeOwnerUnion::Draft(Draft { id: id.clone(), ..Default::default() }),
                Some(ChangeOwnerRef::Checkpoint(id)) => ChangeOwnerUnion::Checkpoint(Checkpoint { id: id.clone(), ..Default::default() }),
                None => ChangeOwnerUnion::Transaction(Transaction::default()),
            }
        }
        async fn forwards(&self) -> Vec<op::OperationKind> {
            self.forwards.clone()
        }
        async fn backwards(&self) -> Vec<op::OperationKind> {
            self.backwards.clone()
        }
    }

    #[derive(Clone, Debug, Union)]
    #[graphql(name = "ChangeOwner")]
    pub enum ChangeOwnerUnion {
        Transaction(Transaction),
        Draft(Draft),
        Checkpoint(Checkpoint),
    }
    //#endregion 🪪 change

    //#region 💼 transaction
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Transaction {
        pub id: Id,
        pub owner_draft_id: Option<Id>,
        pub changes: Vec<Change>,
    }

    impl Transaction {
        pub async fn new() -> Self {
            Self { id: Id::new().await, ..Default::default() }
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn record(&mut self, change: Change) {
            self.changes.push(change);
        }
    }

    #[Object(name = "Transaction")]
    impl Transaction {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            Transaction::compute_hash(self).await
        }
        async fn owner(&self) -> Option<Draft> {
            self.owner_draft_id.as_ref().map(|id| Draft { id: id.clone(), ..Default::default() })
        }
        async fn changes(&self) -> Vec<Change> {
            self.changes.clone()
        }
    }
    //#endregion 💼 transaction

    //#region 📝 draft
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Draft {
        pub id: Id,
        pub owner_alternative_id: Option<Id>,
        pub parent_checkpoint_id: Option<Id>,
        pub target_alternative_id: Option<Id>,
        pub open_transaction_id: Option<Id>,
        pub finalized_transaction_ids: Vec<Id>,
        pub redo_transaction_ids: Vec<Id>,
        pub transactions: Vec<Transaction>,
    }

    impl Draft {
        pub async fn new() -> Self {
            Self { id: Id::new().await, ..Default::default() }
        }
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }
        pub async fn open_transaction_mut(&mut self) -> Option<&mut Transaction> {
            let oid = self.open_transaction_id.clone()?;
            self.transactions.iter_mut().find(|t| t.id == oid)
        }
        pub async fn open_or_start_transaction(&mut self) -> &mut Transaction {
            if self.open_transaction_id.is_none() {
                let tx = Transaction::new().await;
                self.open_transaction_id = Some(tx.id.clone());
                self.transactions.push(tx);
            }
            let oid = self.open_transaction_id.clone().unwrap();
            self.transactions.iter_mut().find(|t| t.id == oid).unwrap()
        }
    }

    #[Object(name = "Draft")]
    impl Draft {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            Draft::compute_hash(self).await
        }
        async fn owner(&self) -> Option<Alternative> {
            self.owner_alternative_id.as_ref().map(|id| Alternative { id: id.clone(), ..Default::default() })
        }
        #[graphql(name = "parentCheckpoint")]
        async fn parent_checkpoint(&self) -> Option<Checkpoint> {
            self.parent_checkpoint_id.as_ref().map(|id| Checkpoint { id: id.clone(), ..Default::default() })
        }
        #[graphql(name = "targetAlternative")]
        async fn target_alternative(&self) -> Option<Alternative> {
            self.target_alternative_id.as_ref().map(|id| Alternative { id: id.clone(), ..Default::default() })
        }
        #[graphql(name = "openTransaction")]
        async fn open_transaction(&self) -> Option<Transaction> {
            let oid = self.open_transaction_id.clone()?;
            self.transactions.iter().find(|t| t.id == oid).cloned()
        }
        #[graphql(name = "finalizedTransactions")]
        async fn finalized_transactions(&self) -> Vec<Transaction> {
            self.finalized_transaction_ids
                .iter()
                .filter_map(|id| self.transactions.iter().find(|t| &t.id == id).cloned())
                .collect()
        }
        #[graphql(name = "redoTransactions")]
        async fn redo_transactions(&self) -> Vec<Transaction> {
            self.redo_transaction_ids
                .iter()
                .filter_map(|id| self.transactions.iter().find(|t| &t.id == id).cloned())
                .collect()
        }
        async fn changes(&self) -> Vec<Change> {
            self.transactions.iter().flat_map(|t| t.changes.clone()).collect()
        }
        #[graphql(name = "canUndo")]
        async fn can_undo(&self, _steps: i32) -> bool {
            !self.finalized_transaction_ids.is_empty()
        }
        #[graphql(name = "canRedo")]
        async fn can_redo(&self, _steps: i32) -> bool {
            !self.redo_transaction_ids.is_empty()
        }
    }
    //#endregion 📝 draft

    //#region 🪧 checkpoint
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Checkpoint {
        pub id: Id,
        pub timestamp: Option<Timestamp>,
        pub authors: Vec<Author>,
        pub root: Option<Kit>,
        pub parent_checkpoint_id: Option<Id>,
        pub message: Option<String>,
        pub is_release: bool,
        pub change_count: i32,
    }

    impl Checkpoint {
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
            Checkpoint::compute_hash(self).await
        }
        async fn timestamp(&self) -> Option<Timestamp> {
            self.timestamp.clone()
        }
        async fn authors(&self) -> Vec<Author> {
            self.authors.clone()
        }
        async fn root(&self) -> Option<Kit> {
            self.root.clone()
        }
        #[graphql(name = "parentCheckpoint")]
        async fn parent_checkpoint(&self) -> Option<Checkpoint> {
            self.parent_checkpoint_id.as_ref().map(|id| Checkpoint { id: id.clone(), ..Default::default() })
        }
        async fn message(&self) -> Option<String> {
            self.message.clone()
        }
        #[graphql(name = "isRelease")]
        async fn is_release(&self) -> bool {
            self.is_release
        }
        #[graphql(name = "changeCount")]
        async fn change_count(&self) -> i32 {
            self.change_count
        }
    }
    //#endregion 🪧 checkpoint

    //#region 🌱 alternative
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Alternative {
        pub id: Id,
        pub owner_graph_id: Option<Id>,
        pub name: String,
        pub start_checkpoint_id: Option<Id>,
        pub checkpoint_ids: Vec<Id>,
        pub kit: Option<Kit>,
        pub draft_id: Option<Id>,
        pub transaction_id: Option<Id>,
    }

    impl Alternative {
        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str(), &self.name])
        }
    }

    #[Object(name = "Alternative")]
    impl Alternative {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            Alternative::compute_hash(self).await
        }
        async fn owner(&self) -> Option<Graph> {
            self.owner_graph_id.as_ref().map(|id| Graph { id: id.clone(), ..Default::default() })
        }
        async fn name(&self) -> String {
            self.name.clone()
        }
        async fn start(&self) -> Checkpoint {
            self.start_checkpoint_id
                .as_ref()
                .map(|id| Checkpoint { id: id.clone(), ..Default::default() })
                .unwrap_or_default()
        }
        async fn checkpoints(&self) -> Vec<Checkpoint> {
            self.checkpoint_ids.iter().map(|id| Checkpoint { id: id.clone(), ..Default::default() }).collect()
        }
        async fn store(&self) -> Kit {
            self.kit.clone().unwrap_or_default()
        }
        async fn draft(&self) -> Option<Draft> {
            self.draft_id.as_ref().map(|id| Draft { id: id.clone(), ..Default::default() })
        }
        async fn transaction(&self) -> Option<Transaction> {
            self.transaction_id.as_ref().map(|id| Transaction { id: id.clone(), ..Default::default() })
        }
    }
    //#endregion 🌱 alternative

    //#region 🌐 graph
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Graph {
        pub id: Id,
        pub owner_session_id: Option<Id>,
        pub the_kit: Kit,
        pub alternatives: Vec<Alternative>,
        pub checkpoints: Vec<Checkpoint>,
        pub releases: Vec<Checkpoint>,
        pub drafts: Vec<Draft>,
    }

    impl Graph {
        pub async fn new() -> Self {
            Self { id: Id::new().await, the_kit: Kit::new("the kit".to_string()).await, ..Default::default() }
        }

        pub async fn compute_hash(&self) -> String {
            h(&[self.id.as_str()])
        }

        pub async fn ensure_design(&mut self, design_id: &Id) -> &mut crate::kit::design::Design {
            if self.the_kit.design_by_id(design_id).await.is_none() {
                let owner_kit_id = self.the_kit.id.clone();
                self.the_kit.designs.push(crate::kit::design::Design {
                    id: design_id.clone(),
                    owner_kit_id,
                    name: format!("design-{}", design_id.as_str()),
                    ..Default::default()
                });
            }
            self.the_kit.design_mut(design_id).await.expect("ensure_design: just inserted")
        }

        pub async fn ensure_draft(&mut self, draft_id: &Id) -> &mut Draft {
            if !self.drafts.iter().any(|d| &d.id == draft_id) {
                self.drafts.push(Draft { id: draft_id.clone(), ..Default::default() });
            }
            self.drafts.iter_mut().find(|d| &d.id == draft_id).expect("ensure_draft: just inserted")
        }

        /// 🪡 The single graph-mutating entry point for `createFixedPiece`. Returns the new piece.
        pub async fn apply_create_fixed_piece(
            &mut self,
            draft_id: Id,
            transaction_id: Id,
            design_id: Id,
            blueprint_id: Id,
            pose: crate::geom::Position,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<crate::kit::design::piece::Piece, SemioError> {
            let mut piece = crate::kit::design::piece::Piece::new_fixed(design_id.clone(), blueprint_id, pose).await;
            piece.name = name;
            piece.description = description;
            let inserted = {
                let design = self.ensure_design(&design_id).await;
                design.insert_piece(piece).await
            };
            let draft = self.ensure_draft(&draft_id).await;
            if draft.open_transaction_id.is_none() {
                draft.open_transaction_id = Some(transaction_id.clone());
                draft.transactions.push(Transaction { id: transaction_id, ..Default::default() });
            }
            Ok(inserted)
        }
    }

    #[Object(name = "Graph")]
    impl Graph {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        async fn hash(&self) -> String {
            Graph::compute_hash(self).await
        }
        async fn owner(&self) -> Option<Session> {
            self.owner_session_id.as_ref().map(|id| Session { id: id.clone(), ..Default::default() })
        }
        #[graphql(name = "theKit")]
        async fn the_kit(&self) -> Option<Kit> {
            Some(self.the_kit.clone())
        }
        async fn alternative(&self, id: Id) -> Option<Alternative> {
            self.alternatives.iter().find(|a| a.id == id).cloned()
        }
        async fn alternatives(&self) -> Vec<Alternative> {
            self.alternatives.clone()
        }
        async fn checkpoint(&self, id: Id) -> Option<Checkpoint> {
            self.checkpoints.iter().find(|c| c.id == id).cloned()
        }
        async fn checkpoints(&self) -> Vec<Checkpoint> {
            self.checkpoints.clone()
        }
        async fn release(&self, id: Id) -> Option<Checkpoint> {
            self.releases.iter().find(|c| c.id == id).cloned()
        }
        async fn releases(&self) -> Vec<Checkpoint> {
            self.releases.clone()
        }
    }
    //#endregion 🌐 graph

    //#region 👤 session
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Session {
        pub id: Id,
        pub started_at: Option<Timestamp>,
        pub draft_ids: Vec<Id>,
    }

    impl Session {
        pub async fn new() -> Self {
            Self { id: Id::new().await, ..Default::default() }
        }
    }

    #[Object(name = "Session")]
    impl Session {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        #[graphql(name = "startedAt")]
        async fn started_at(&self) -> Option<Timestamp> {
            self.started_at.clone()
        }
        async fn drafts(&self) -> Vec<Draft> {
            self.draft_ids.iter().map(|id| Draft { id: id.clone(), ..Default::default() }).collect()
        }
    }
    //#endregion 👤 session

    //#region ⚠️ conflict
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Conflict {
        pub id: Id,
        pub backbone_tip: Option<String>,
        pub reason: String,
        pub created_at: Timestamp,
    }

    #[Object(name = "Conflict")]
    impl Conflict {
        async fn id(&self) -> Id {
            self.id.clone()
        }
        #[graphql(name = "backboneTip")]
        async fn backbone_tip(&self) -> Option<String> {
            self.backbone_tip.clone()
        }
        async fn reason(&self) -> String {
            self.reason.clone()
        }
        #[graphql(name = "createdAt")]
        async fn created_at(&self) -> Timestamp {
            self.created_at.clone()
        }
    }
    //#endregion ⚠️ conflict
}

//#endregion 🌿 vcs

//#region ⚙️ op

pub mod op {
    //! ⚙️ Operation entities and their inputs.
    use async_graphql::{InputObject, Interface, Object, OneofObject, Union};
    use serde::{Deserialize, Serialize};

    use crate::geom::{Position, Offset};
    use crate::id::Id;
    use crate::vcs::Change;

    //#region 🧾 inputs as graphql types (objects, not InputObjects — schema declares them as `type`)
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CreatedFixedPieceInput {
        pub design_id: Id,
        pub blueprint_id: Id,
        pub pose: Position,
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
        async fn pose(&self) -> Position {
            self.pose
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
        async fn offset(&self) -> Offset {
            self.offset
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

    /// 🧾 The schema's `union OperationInput = …` (oneof).
    #[derive(Clone, Debug, Union)]
    #[graphql(name = "OperationInput")]
    pub enum OperationInputUnion {
        RenamedKit(RenamedKitInput),
        ChangedDescription(ChangedDescriptionInput),
        CreatedFixedPiece(CreatedFixedPieceInput),
        FixedPiece(FixedPieceInput),
        DraggedPiece(DraggedPieceInput),
    }
    //#endregion 🧾 inputs

    //#region 📦 placeholder Diff scalar (filled in a follow-up ticket)
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Diff {
        pub id: Id,
    }

    #[Object(name = "Diff")]
    impl Diff {
        async fn id(&self) -> Id {
            self.id.clone()
        }
    }
    //#endregion 📦 diff

    //#region 🪄 operations
    macro_rules! op_struct {
        ($name:ident, $input:ident, $payload_field:ident: $payload_ty:ty) => {
            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct $name {
                pub id: Id,
                pub owner_change_id: Id,
                pub input: $input,
                pub diff: Diff,
                pub $payload_field: $payload_ty,
            }
        };
        ($name:ident, $input:ident) => {
            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct $name {
                pub id: Id,
                pub owner_change_id: Id,
                pub input: $input,
                pub diff: Diff,
            }
        };
    }

    op_struct!(CreatedFixedPiece, CreatedFixedPieceInput, piece: crate::kit::design::piece::Piece);
    op_struct!(FixedPiece, FixedPieceInput, piece: crate::kit::design::piece::Piece);
    op_struct!(DraggedPiece, DraggedPieceInput, pieces: Vec<crate::kit::design::piece::Piece>);
    op_struct!(RenamedKit, RenamedKitInput, kit: crate::kit::Kit);

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ChangedDescription {
        pub id: Id,
        pub owner_change_id: Id,
        pub input: ChangedDescriptionInput,
        pub diff: Diff,
        pub entity_id: Id,
    }

    impl CreatedFixedPiece {
        /// 🆕 Pure forward fn (skeleton): builds the op + the new piece from the input.
        pub async fn forward(input: CreatedFixedPieceInput) -> (Self, crate::kit::design::piece::Piece) {
            let mut piece = crate::kit::design::piece::Piece::new_fixed(
                input.design_id.clone(),
                input.blueprint_id.clone(),
                input.pose,
            )
            .await;
            piece.name = input.name.clone();
            piece.description = input.description.clone();
            let op = Self {
                id: Id::new().await,
                owner_change_id: Id::default(),
                input,
                diff: Diff::default(),
                piece: piece.clone(),
            };
            (op, piece)
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
        async fn owner(&self) -> Change {
            Change { id: self.owner_change_id.clone(), ..Default::default() }
        }
        async fn input(&self) -> CreatedFixedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn piece(&self) -> crate::kit::design::piece::Piece {
            self.piece.clone()
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
        async fn owner(&self) -> Change {
            Change { id: self.owner_change_id.clone(), ..Default::default() }
        }
        async fn input(&self) -> FixedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn piece(&self) -> crate::kit::design::piece::Piece {
            self.piece.clone()
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
        async fn owner(&self) -> Change {
            Change { id: self.owner_change_id.clone(), ..Default::default() }
        }
        async fn input(&self) -> DraggedPieceInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn pieces(&self) -> Vec<crate::kit::design::piece::Piece> {
            self.pieces.clone()
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
        async fn owner(&self) -> Change {
            Change { id: self.owner_change_id.clone(), ..Default::default() }
        }
        async fn input(&self) -> RenamedKitInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        async fn kit(&self) -> crate::kit::Kit {
            self.kit.clone()
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
        async fn owner(&self) -> Change {
            Change { id: self.owner_change_id.clone(), ..Default::default() }
        }
        async fn input(&self) -> ChangedDescriptionInput {
            self.input.clone()
        }
        async fn diff(&self) -> Diff {
            self.diff.clone()
        }
        // The schema declares `entity: Entity!` — surfaced as a Kit fallback for the skeleton.
        async fn entity(&self) -> crate::kit::Kit {
            crate::kit::Kit { id: self.entity_id.clone(), ..Default::default() }
        }
    }

    /// 🌗 Sum-type carrying any operation through the event bus / change log.
    #[derive(Clone, Debug, Serialize, Deserialize, Union)]
    #[graphql(name = "OperationKind")]
    pub enum OperationKind {
        CreatedFixedPiece(CreatedFixedPiece),
        FixedPiece(FixedPiece),
        DraggedPiece(DraggedPiece),
        RenamedKit(RenamedKit),
        ChangedDescription(ChangedDescription),
    }

    /// 🪄 Marker type for `interface Operation` — surfaced via [`OperationKind`].
    #[derive(Clone, Debug, Interface)]
    #[graphql(
        name = "Operation",
        field(name = "id", ty = "Id"),
        field(name = "hash", ty = "String"),
        field(name = "owner", ty = "Change"),
        field(name = "diff", ty = "Diff")
    )]
    pub enum OperationIface {
        CreatedFixedPiece(CreatedFixedPiece),
        FixedPiece(FixedPiece),
        DraggedPiece(DraggedPiece),
        RenamedKit(RenamedKit),
        ChangedDescription(ChangedDescription),
    }

    /// 🧾 OneOf input-object surface for batched submissions (mirrors the schema's `union OperationInput`).
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
        pub pose: Position,
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

    //#region 📡 command surface (parent → child workers)
    /// 📡 Internal command envelope passed parent → child runtime over the work queue.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Command {
        CreateFixedPiece {
            request_id: Id,
            draft_id: Id,
            transaction_id: Id,
            design_id: Id,
            blueprint_id: Id,
            pose: Position,
            name: Option<String>,
            description: Option<String>,
        },
        FixPiece {
            request_id: Id,
            draft_id: Id,
            transaction_id: Id,
            design_id: Id,
            piece_id: Id,
        },
        RenameKit {
            request_id: Id,
            draft_id: Id,
            transaction_id: Id,
            name: String,
        },
        ChangeDescription {
            request_id: Id,
            draft_id: Id,
            transaction_id: Id,
            description: String,
        },
    }

    impl Command {
        pub fn request_id(&self) -> &Id {
            match self {
                Command::CreateFixedPiece { request_id, .. } => request_id,
                Command::FixPiece { request_id, .. } => request_id,
                Command::RenameKit { request_id, .. } => request_id,
                Command::ChangeDescription { request_id, .. } => request_id,
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
    //#endregion 📡 command surface
}

//#endregion ⚙️ op

//#region 📣 event

pub mod event {
    //! 📣 The single emit point of the entire crate.
    use std::sync::Arc;

    use async_broadcast::{InactiveReceiver, Receiver, Sender};
    use async_lock::Mutex;
    use serde::{Deserialize, Serialize};

    use crate::error::SemioError;
    use crate::op;

    /// 🌐 Broadcast envelope for every observable thing the control plane emits.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum KitEvent {
        CommandSucceeded(op::CommandReceipt),
        OperationSucceeded(op::OperationKind),
        OperationFailed(SemioError),
        CreatedFixedPiece(op::CreatedFixedPiece),
        FixedPiece(op::FixedPiece),
        DraggedPiece(op::DraggedPiece),
        RenamedKit(op::RenamedKit),
        ChangedDescription(op::ChangedDescription),
    }

    /// 📣 The bus. Holds the only `emit_event` function in the crate.
    pub struct EventBus {
        tx: Mutex<Sender<KitEvent>>,
        keep_alive: InactiveReceiver<KitEvent>,
    }

    impl EventBus {
        pub fn new(capacity: usize) -> Arc<Self> {
            let (mut tx, rx) = async_broadcast::broadcast(capacity);
            tx.set_overflow(true);
            Arc::new(Self { tx: Mutex::new(tx), keep_alive: rx.deactivate() })
        }

        /// 📣 The **only** `emit_event` in the entire crate. All other code paths must call this.
        pub async fn emit_event(&self, ev: KitEvent) {
            let tx = self.tx.lock().await;
            let _ = tx.broadcast_direct(ev).await;
        }

        /// 🔔 New subscriber receiver.
        pub fn subscribe(&self) -> Receiver<KitEvent> {
            self.keep_alive.activate_cloned()
        }
    }
}

//#endregion 📣 event

//#region 🧵 worker

pub mod worker {
    //! 🧵 Parent router + two child runtimes (wip + authoritative).
    //!
    //! Native: both children are spawned on a shared [`async_executor::Executor`].
    //! Wasm: each child lives in a dedicated [`web_sys::Worker`]; messages cross via [`crate::wasm_bridge`].
    use std::sync::Arc;

    use async_channel::{Receiver, Sender};
    use async_lock::RwLock;

    use crate::error::SemioError;
    use crate::event::{EventBus, KitEvent};
    use crate::id::Id;
    use crate::op::{Command, CommandReceipt, CreatedFixedPiece, CreatedFixedPieceInput};
    use crate::vcs::{Conflict, Graph, Session};

    /// 🚪 Per-child handle held by the parent: send commands in, no out (events flow through the shared bus).
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
        pub wip_graph: Arc<RwLock<Graph>>,
        pub auth_graph: Arc<RwLock<Graph>>,
        pub sessions: Arc<RwLock<Vec<Session>>>,
        pub conflicts: Arc<RwLock<Vec<Conflict>>>,
    }

    impl ParentRuntime {
        /// 🛰️ Spawn parent + two child runtimes (in-process on native).
        pub async fn spawn() -> Arc<Self> {
            let bus = EventBus::new(1024);

            let wip_graph = Arc::new(RwLock::new(Graph::new().await));
            let auth_graph = Arc::new(RwLock::new(Graph::new().await));

            let (wip_tx, wip_rx) = async_channel::unbounded::<Command>();
            let (auth_tx, auth_rx) = async_channel::unbounded::<Command>();

            spawn_child("wip", wip_graph.clone(), bus.clone(), wip_rx);
            spawn_child("auth", auth_graph.clone(), bus.clone(), auth_rx);

            Arc::new(Self {
                bus,
                wip: ChildPort { inbound: wip_tx },
                auth: ChildPort { inbound: auth_tx },
                wip_graph,
                auth_graph,
                sessions: Arc::new(RwLock::new(vec![])),
                conflicts: Arc::new(RwLock::new(vec![])),
            })
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

        pub async fn snapshot_wip_graph(&self) -> Graph {
            self.wip_graph.read().await.clone()
        }
        pub async fn snapshot_auth_graph(&self) -> Graph {
            self.auth_graph.read().await.clone()
        }
    }

    fn spawn_child(label: &'static str, graph: Arc<RwLock<Graph>>, bus: Arc<EventBus>, inbox: Receiver<Command>) {
        let fut = async move { ChildRuntime { label, graph, bus, inbox }.run().await };
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
        pub graph: Arc<RwLock<Graph>>,
        pub bus: Arc<EventBus>,
        pub inbox: Receiver<Command>,
    }

    impl ChildRuntime {
        pub async fn run(self) {
            while let Ok(cmd) = self.inbox.recv().await {
                let request_id = cmd.request_id().clone();
                let kind = match &cmd {
                    Command::CreateFixedPiece { .. } => "createFixedPiece",
                    Command::FixPiece { .. } => "fixPiece",
                    Command::RenameKit { .. } => "renameKit",
                    Command::ChangeDescription { .. } => "changeDescription",
                };
                self.bus
                    .emit_event(KitEvent::CommandSucceeded(CommandReceipt {
                        request_id: request_id.clone(),
                        kind: kind.to_string(),
                    }))
                    .await;

                if let Err(e) = self.apply(cmd).await {
                    let err = e.with_request(request_id);
                    self.bus.emit_event(KitEvent::OperationFailed(err)).await;
                }
            }
        }

        async fn apply(&self, cmd: Command) -> Result<(), SemioError> {
            match cmd {
                Command::CreateFixedPiece {
                    request_id,
                    draft_id,
                    transaction_id,
                    design_id,
                    blueprint_id,
                    pose,
                    name,
                    description,
                } => {
                    let mut graph = self.graph.write().await;
                    let piece = graph
                        .apply_create_fixed_piece(
                            draft_id.clone(),
                            transaction_id,
                            design_id.clone(),
                            blueprint_id.clone(),
                            pose,
                            name.clone(),
                            description.clone(),
                        )
                        .await?;
                    drop(graph);

                    let op = CreatedFixedPiece {
                        id: request_id,
                        owner_change_id: Id::default(),
                        input: CreatedFixedPieceInput {
                            design_id,
                            blueprint_id,
                            pose,
                            name,
                            description,
                        },
                        diff: Default::default(),
                        piece,
                    };
                    self.bus.emit_event(KitEvent::CreatedFixedPiece(op)).await;
                    Ok(())
                }
                Command::FixPiece { .. } | Command::RenameKit { .. } | Command::ChangeDescription { .. } => {
                    // Skeleton stubs — wired through commandSucceeded only.
                    Ok(())
                }
            }
        }
    }
}

//#endregion 🧵 worker

//#region 🌐 gql

pub mod gql {
    //! 🌐 GraphQL roots: Query / Mutation / Subscription + schema builder.
    use std::sync::Arc;

    use async_graphql::{Context, EmptySubscription, Object, Schema, Subscription};
    use async_stream::stream;
    use futures_util::Stream;

    use crate::error::SemioError;
    use crate::event::{EventBus, KitEvent};
    use crate::geom::Position;
    use crate::id::Id;
    use crate::op::{
        ChangedDescription, Command, CommandReceipt, CreatedFixedPiece, DraggedPiece, FixedPiece,
        OperationIface, RenamedKit,
    };
    use crate::vcs::{Conflict, Graph, Session};
    use crate::worker::ParentRuntime;

    fn rt<'a>(ctx: &'a Context<'_>) -> async_graphql::Result<&'a Arc<ParentRuntime>> {
        ctx.data::<Arc<ParentRuntime>>()
    }
    fn bus<'a>(ctx: &'a Context<'_>) -> async_graphql::Result<&'a Arc<EventBus>> {
        ctx.data::<Arc<EventBus>>()
    }

    pub struct Query;

    #[Object]
    impl Query {
        async fn session(&self, ctx: &Context<'_>) -> async_graphql::Result<Session> {
            let rt = rt(ctx)?;
            let mut sessions = rt.sessions.write().await;
            if sessions.is_empty() {
                sessions.push(Session::new().await);
            }
            Ok(sessions[0].clone())
        }

        async fn wip(&self, ctx: &Context<'_>) -> async_graphql::Result<Graph> {
            Ok(rt(ctx)?.snapshot_wip_graph().await)
        }

        async fn authoritative(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Graph>> {
            Ok(Some(rt(ctx)?.snapshot_auth_graph().await))
        }

        #[graphql(deprecation = "Use authoritative")]
        async fn authorative(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Graph>> {
            Ok(Some(rt(ctx)?.snapshot_auth_graph().await))
        }

        async fn conflicts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Conflict>> {
            Ok(rt(ctx)?.conflicts.read().await.clone())
        }
    }

    pub struct Mutation;

    #[Object]
    impl Mutation {
        #[graphql(name = "renameKit")]
        async fn rename_kit(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            name: String,
        ) -> async_graphql::Result<Id> {
            let rt = rt(ctx)?;
            let request_id = Id::new().await;
            rt.dispatch_wip(Command::RenameKit { request_id: request_id.clone(), draft_id, transaction_id, name }).await;
            Ok(request_id)
        }

        #[graphql(name = "changeDescription")]
        async fn change_description(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            description: String,
        ) -> async_graphql::Result<Id> {
            let rt = rt(ctx)?;
            let request_id = Id::new().await;
            rt.dispatch_wip(Command::ChangeDescription {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                description,
            })
            .await;
            Ok(request_id)
        }

        #[graphql(name = "createFixedPiece")]
        async fn create_fixed_piece(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            pose: Position,
            name: Option<String>,
            description: Option<String>,
        ) -> async_graphql::Result<Id> {
            let rt = rt(ctx)?;
            let request_id = Id::new().await;
            // Skeleton: the GraphQL `createFixedPiece` does not yet take an explicit blueprint id;
            // we mint one so the piece always has a blueprint reference. A follow-up ticket adds the arg.
            let blueprint_id = Id::new().await;
            rt.dispatch_wip(Command::CreateFixedPiece {
                request_id: request_id.clone(),
                draft_id,
                transaction_id,
                design_id,
                blueprint_id,
                pose,
                name,
                description,
            })
            .await;
            Ok(request_id)
        }

        #[graphql(name = "fixPiece")]
        async fn fix_piece(
            &self,
            ctx: &Context<'_>,
            #[graphql(name = "draftId")] draft_id: Id,
            #[graphql(name = "transactionId")] transaction_id: Id,
            #[graphql(name = "designId")] design_id: Id,
            #[graphql(name = "pieceId")] piece_id: Id,
        ) -> async_graphql::Result<Id> {
            let rt = rt(ctx)?;
            let request_id = Id::new().await;
            rt.dispatch_wip(Command::FixPiece { request_id: request_id.clone(), draft_id, transaction_id, design_id, piece_id }).await;
            Ok(request_id)
        }
    }

    pub struct SubscriptionRoot;

    macro_rules! sub_filter {
        ($self:ident, $ctx:ident, $variant:ident, $ty:ty) => {{
            let bus = bus($ctx)?.clone();
            let mut rx = bus.subscribe();
            let s: std::pin::Pin<Box<dyn Stream<Item = $ty> + Send>> = Box::pin(stream! {
                while let Ok(ev) = rx.recv().await {
                    if let KitEvent::$variant(value) = ev { yield value; }
                }
            });
            Ok(s)
        }};
    }

    type SubStream<T> = std::pin::Pin<Box<dyn Stream<Item = T> + Send>>;

    #[Subscription]
    impl SubscriptionRoot {
        #[graphql(name = "commandSucceeded")]
        async fn command_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<CommandReceipt>> {
            sub_filter!(self, ctx, CommandSucceeded, CommandReceipt)
        }

        #[graphql(name = "operationSucceeded")]
        async fn operation_succeeded(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<OperationIface>> {
            let bus = bus(ctx)?.clone();
            let mut rx = bus.subscribe();
            let s: SubStream<OperationIface> = Box::pin(stream! {
                while let Ok(ev) = rx.recv().await {
                    match ev {
                        KitEvent::CreatedFixedPiece(o) => yield OperationIface::CreatedFixedPiece(o),
                        KitEvent::FixedPiece(o) => yield OperationIface::FixedPiece(o),
                        KitEvent::DraggedPiece(o) => yield OperationIface::DraggedPiece(o),
                        KitEvent::RenamedKit(o) => yield OperationIface::RenamedKit(o),
                        KitEvent::ChangedDescription(o) => yield OperationIface::ChangedDescription(o),
                        _ => {}
                    }
                }
            });
            Ok(s)
        }

        #[graphql(name = "operationFailed")]
        async fn operation_failed(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<SemioError>> {
            sub_filter!(self, ctx, OperationFailed, SemioError)
        }

        #[graphql(name = "kitRenamed")]
        async fn kit_renamed(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<RenamedKit>> {
            sub_filter!(self, ctx, RenamedKit, RenamedKit)
        }

        #[graphql(name = "descriptionChanged")]
        async fn description_changed(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<ChangedDescription>> {
            sub_filter!(self, ctx, ChangedDescription, ChangedDescription)
        }

        #[graphql(name = "createdFixedPiece")]
        async fn created_fixed_piece(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<CreatedFixedPiece>> {
            sub_filter!(self, ctx, CreatedFixedPiece, CreatedFixedPiece)
        }

        #[graphql(name = "fixedPiece")]
        async fn fixed_piece(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<FixedPiece>> {
            sub_filter!(self, ctx, FixedPiece, FixedPiece)
        }

        #[graphql(name = "draggedPiece")]
        async fn dragged_piece(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<DraggedPiece>> {
            sub_filter!(self, ctx, DraggedPiece, DraggedPiece)
        }

        async fn error(&self, ctx: &Context<'_>) -> async_graphql::Result<SubStream<SemioError>> {
            sub_filter!(self, ctx, OperationFailed, SemioError)
        }
    }

    /// 🧱 Build the schema with the parent runtime + event bus injected as data.
    pub async fn build_schema() -> Schema<Query, Mutation, SubscriptionRoot> {
        let rt = ParentRuntime::spawn().await;
        let bus = rt.bus.clone();
        Schema::build(Query, Mutation, SubscriptionRoot).data(rt).data(bus).finish()
    }

    /// 📜 Convenience SDL for tests / tooling.
    pub async fn sdl() -> String {
        build_schema().await.sdl()
    }

    // Suppress unused import warnings on cfg combinations.
    #[allow(dead_code)]
    fn _unused() -> EmptySubscription {
        EmptySubscription
    }
}

//#endregion 🌐 gql

//#region 🔌 wasm_bridge

#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    //! 🔌 Wires `ChildPort` ↔ `web_sys::Worker` postMessage on wasm32 targets.
    //!
    //! The skeleton exports two entry points so a JS host can spawn the parent and route messages
    //! through `postMessage`. The actual GraphQL request execution runs through the schema built
    //! by [`crate::gql::build_schema`].
    use std::sync::Arc;

    use wasm_bindgen::prelude::*;

    use crate::worker::ParentRuntime;

    #[wasm_bindgen(start)]
    pub fn _start() {
        console_error_panic_hook::set_once();
    }

    /// 🛰️ Boot the parent runtime inside the current (parent) web worker.
    #[wasm_bindgen]
    pub async fn parent_boot() -> JsValue {
        let rt: Arc<ParentRuntime> = ParentRuntime::spawn().await;
        let _ = rt; // The Arc is leaked into JS land via a global registry in a follow-up ticket.
        JsValue::TRUE
    }

    /// 📜 Schema SDL for tooling.
    #[wasm_bindgen]
    pub async fn schema_sdl() -> String {
        crate::gql::sdl().await
    }
}

//#endregion 🔌 wasm_bridge

//#region 🧪 tests

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use async_graphql::{Request, Variables};
    use futures_lite::future::block_on;
    use futures_util::StreamExt;
    use serde_json::json;

    #[test]
    fn parses_target_schema() {
        let sdl = block_on(crate::gql::sdl());
        for t in [
            "type Query",
            "type Mutation",
            "type Piece",
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
            "type CreatedFixedPiece",
            "interface Operation",
            "scalar Timestamp",
        ] {
            assert!(sdl.contains(t), "schema missing `{}`. SDL was:\n{}", t, sdl);
        }
    }

    /// 🛡️ Guard test: the crate must contain exactly **one** `pub async fn emit_event` definition.
    /// Matches the canonical signature on `EventBus`; everything else must route through it.
    #[test]
    fn single_emit_event_in_codebase() {
        let src = include_str!("lib.rs");
        let needle = concat!("pub async fn ", "emit_event(&self, ev: KitEvent)");
        let count = src.matches(needle).count();
        assert_eq!(count, 1, "expected exactly one canonical emit_event definition in lib.rs, found {}", count);
    }

    #[test]
    fn create_fixed_piece_end_to_end() {
        block_on(async {
            let schema = crate::gql::build_schema().await;

            let mutation = r#"
                mutation($draftId: ID!, $txId: ID!, $designId: ID!, $pose: PositionInput!) {
                    createFixedPiece(draftId: $draftId, transactionId: $txId, designId: $designId, pose: $pose)
                }
            "#;
            let pose = json!({
                "center": { "u": 0.0, "v": 0.0 },
                "plane": {
                    "origin": { "x": 0.0, "y": 0.0, "z": 0.0 },
                    "xAxis":  { "x": 1.0, "y": 0.0, "z": 0.0 },
                    "yAxis":  { "x": 0.0, "y": 1.0, "z": 0.0 }
                }
            });
            let vars: async_graphql::Value = async_graphql::value!({
                "draftId": "d1",
                "txId": "t1",
                "designId": "des1",
                "pose": pose
            });
            let res = schema.execute(Request::new(mutation).variables(Variables::from_value(vars))).await;
            assert!(res.errors.is_empty(), "mutation errors: {:?}", res.errors);

            // Subscribe and re-issue to validate that subscriptions deliver the event.
            let sub_doc = "subscription { createdFixedPiece { id piece { id } } }";
            let mut stream = schema.execute_stream(sub_doc);
            // Issue another createFixedPiece while subscribed.
            let mutation2 = r#"
                mutation($draftId: ID!, $txId: ID!, $designId: ID!, $pose: PositionInput!) {
                    createFixedPiece(draftId: $draftId, transactionId: $txId, designId: $designId, pose: $pose)
                }
            "#;
            let vars2: async_graphql::Value = async_graphql::value!({
                "draftId": "d1",
                "txId": "t1",
                "designId": "des1",
                "pose": pose
            });
            let schema2 = schema.clone();
            // Spawn the second mutation on a thread so it runs after we start polling the stream.
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(50));
                block_on(async move {
                    let _ = schema2.execute(Request::new(mutation2).variables(Variables::from_value(vars2))).await;
                });
            });

            let next = futures_lite::future::or(
                async { stream.next().await },
                async {
                    futures_timer_block(std::time::Duration::from_secs(2)).await;
                    None
                },
            )
            .await;
            assert!(next.is_some(), "did not receive createdFixedPiece subscription event");

            // wip should now contain the piece.
            let q = "{ wip { theKit { designs { id pieces { id } } } } }";
            let res = schema.execute(q).await;
            let data = res.data.into_json().unwrap();
            let designs = &data["wip"]["theKit"]["designs"];
            assert!(
                designs.as_array().map(|a| a.iter().any(|d| !d["pieces"].as_array().unwrap().is_empty())).unwrap_or(false),
                "expected at least one piece in wip; got: {}",
                serde_json::to_string_pretty(&data).unwrap()
            );
        });
    }

    #[test]
    fn wip_and_authoritative_are_isolated() {
        block_on(async {
            let schema = crate::gql::build_schema().await;
            let mutation = r#"
                mutation($draftId: ID!, $txId: ID!, $designId: ID!, $pose: PositionInput!) {
                    createFixedPiece(draftId: $draftId, transactionId: $txId, designId: $designId, pose: $pose)
                }
            "#;
            let pose = json!({
                "center": { "u": 0.0, "v": 0.0 },
                "plane": {
                    "origin": { "x": 0.0, "y": 0.0, "z": 0.0 },
                    "xAxis":  { "x": 1.0, "y": 0.0, "z": 0.0 },
                    "yAxis":  { "x": 0.0, "y": 1.0, "z": 0.0 }
                }
            });
            let vars: async_graphql::Value = async_graphql::value!({
                "draftId": "d1",
                "txId": "t1",
                "designId": "des1",
                "pose": pose
            });
            let _ = schema.execute(Request::new(mutation).variables(Variables::from_value(vars))).await;
            // Wait briefly for the wip child to apply.
            std::thread::sleep(std::time::Duration::from_millis(150));

            let q = "{ authoritative { theKit { designs { pieces { id } } } } }";
            let res = schema.execute(q).await;
            let data = res.data.into_json().unwrap();
            let designs = &data["authoritative"]["theKit"]["designs"];
            // Authoritative should not have any pieces.
            assert!(
                designs.as_array().map(|a| a.iter().all(|d| d["pieces"].as_array().unwrap().is_empty())).unwrap_or(true),
                "authoritative leaked pieces: {}",
                serde_json::to_string_pretty(&data).unwrap()
            );
        });
    }

    /// Tiny std-only timer (avoids pulling another crate just for tests).
    async fn futures_timer_block(dur: std::time::Duration) {
        let (tx, rx) = async_channel::bounded::<()>(1);
        std::thread::spawn(move || {
            std::thread::sleep(dur);
            let _ = tx.try_send(());
        });
        let _ = rx.recv().await;
    }
}

//#endregion 🧪 tests
