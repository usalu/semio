//! semio: purely object-oriented, pointer-first in-memory graph.
//!
//! Every aggregate owns its children through `Arc<RwLock<T>>`; children hold
//! `Weak<RwLock<T>>` back-references to their parents. Content-addressable
//! hashes are computed lazily through `OnceLock` fingerprints on each entity.
//! GUIDs exist only as stable identity at serialization boundaries and in
//! DTO resolvers; the in-memory graph walks pointers.

#![allow(clippy::new_without_default)]

pub mod attribute;
pub mod author;
pub mod benchmark;
pub mod concept;
pub mod connection;
pub mod connector;
pub mod design;
pub mod diff;
pub mod error;
pub mod file;
pub mod folder;
pub mod geom;
pub mod group;
pub mod guid;
pub mod hash;
pub mod io;
pub mod kit;
pub mod layer;
pub mod piece;
pub mod port;
pub mod prop;
pub mod quality;
pub mod report;
pub mod representation;
pub mod session;
pub mod side;
pub mod stat;
pub mod tag;
pub mod typ;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use attribute::Attribute;
pub use author::Author;
pub use benchmark::Benchmark;
pub use concept::Concept;
pub use connection::{Connection, ConnectionDto, ConnectionRef, ConnectionWeak};
pub use connector::{Connector, ConnectorDto, ConnectorRef, ConnectorWeak};
pub use design::{Design, DesignDto, DesignRef, DesignWeak, FlattenedDesign};
pub use diff::{DesignChange, DesignDiff};
pub use error::{Result, SemioError};
pub use file::{File, FileDto, FileRef, FileWeak};
pub use folder::{Folder, FolderDto, FolderRef, FolderWeak};
pub use geom::{Camera, Coord, Location, Plane, Vector};
pub use group::{Group, GroupDto, GroupRef, GroupWeak};
pub use guid::Guid;
pub use hash::HashWriter;
pub use kit::{Kit, KitDto, KitRef};
pub use layer::{Layer, LayerDto, LayerRef, LayerWeak};
pub use piece::{FlattenedPiece, Piece, PieceDto, PieceRef, PieceWeak};
pub use port::{Port, PortDto, PortRef, PortWeak};
pub use prop::Prop;
pub use quality::{Quality, QualityDto, QualityRef, QualityWeak};
pub use report::{NoteSeverity, OperationNote, SemioReport, ValidationResult};
pub use representation::{Representation, RepresentationDto, RepresentationRef, RepresentationWeak};
pub use session::KitGraphSession;
pub use side::{Side, SideDto};
pub use stat::Stat;
pub use tag::Tag;
pub use typ::{Type, TypeDto, TypeRef, TypeWeak};
