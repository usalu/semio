//! semio: purely object-oriented, pointer-first in-memory graph.
//!
//! Every aggregate owns its children through `Arc<RwLock<T>>`; children hold
//! `Weak<RwLock<T>>` back-references to their parents. Content-addressable
//! hashes are computed lazily through interior-mutable `Cache` on each entity.
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
pub mod events;
pub(crate) mod event_wire;
pub(crate) mod flatten_math;
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

#[cfg(test)]
mod tests;

mod async_kit;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use attribute::{
    AttributeFullDto, AttributeIdDto, AttributeMetadataDto, AttributeShallowDto, AttributeStore,
    AttributeStoreRef, AttributeStoreWeak,
};
pub use author::{
    AuthorFullDto, AuthorIdDto, AuthorMetadataDto, AuthorShallowDto, AuthorStore, AuthorStoreRef, AuthorStoreWeak,
};
pub use benchmark::{
    BenchmarkFullDto, BenchmarkIdDto, BenchmarkMetadataDto, BenchmarkShallowDto, BenchmarkStore,
    BenchmarkStoreRef, BenchmarkStoreWeak,
};
pub use concept::{
    ConceptFullDto, ConceptIdDto, ConceptMetadataDto, ConceptShallowDto, ConceptStore, ConceptStoreRef,
    ConceptStoreWeak,
};
pub use connection::{
    ConnectionFullDto, ConnectionIdDto, ConnectionMetadataDto, ConnectionShallowDto, ConnectionStore,
    ConnectionStoreRef, ConnectionStoreWeak,
};
pub use connector::{
    ConnectorFullDto, ConnectorIdDto, ConnectorMetadataDto, ConnectorShallowDto, ConnectorStore,
    ConnectorStoreRef, ConnectorStoreWeak,
};
pub use design::{
    DesignFullDto, DesignIdDto, DesignMetadataDto, DesignShallowDto, DesignStore, DesignStoreRef, DesignStoreWeak,
};
pub use diff::{DesignChange, DesignDiff};
pub use error::{Result, SemioError};
pub use events::{EntityKind, EntityRef, EventBus, KitEvent};
pub use file::{FileFullDto, FileIdDto, FileMetadataDto, FileShallowDto, FileStore, FileStoreRef, FileStoreWeak};
pub use folder::{
    FolderFullDto, FolderIdDto, FolderMetadataDto, FolderShallowDto, FolderStore, FolderStoreRef, FolderStoreWeak,
};
pub use geom::{Camera, Coord, Location, Plane, Vector};
pub use group::{
    GroupFullDto, GroupIdDto, GroupMetadataDto, GroupShallowDto, GroupStore, GroupStoreRef, GroupStoreWeak,
};
pub use guid::Guid;
pub use hash::{Cache, HashWriter};
pub use kit::{
    KitFullDto, KitIdDto, KitMetadataDto, KitShallowDto, KitStore, KitStoreRef, KitStoreWeak,
};
pub use layer::{
    LayerFullDto, LayerIdDto, LayerMetadataDto, LayerShallowDto, LayerStore, LayerStoreRef, LayerStoreWeak,
};
pub use piece::{
    PieceFullDto, PieceIdDto, PieceMetadataDto, PieceShallowDto, PieceStore, PieceStoreRef, PieceStoreWeak,
};
pub use port::{
    PortFullDto, PortIdDto, PortMetadataDto, PortShallowDto, PortStore, PortStoreRef, PortStoreWeak,
};
pub use prop::{PropFullDto, PropIdDto, PropMetadataDto, PropShallowDto, PropStore, PropStoreRef, PropStoreWeak};
pub use quality::{
    QualityFullDto, QualityIdDto, QualityMetadataDto, QualityShallowDto, QualityStore, QualityStoreRef,
    QualityStoreWeak,
};
pub use report::{NoteSeverity, OperationNote, SemioReport, ValidationResult};
pub use representation::{
    RepresentationFullDto, RepresentationIdDto, RepresentationMetadataDto, RepresentationShallowDto,
    RepresentationStore, RepresentationStoreRef, RepresentationStoreWeak,
};
pub use session::KitGraphSession;
pub use side::{
    SideFullDto, SideIdDto, SideMetadataDto, SideShallowDto, SideStore, SideStoreRef, SideStoreWeak,
};
pub use stat::{StatFullDto, StatIdDto, StatMetadataDto, StatShallowDto, StatStore, StatStoreRef, StatStoreWeak};
pub use tag::{TagFullDto, TagIdDto, TagMetadataDto, TagShallowDto, TagStore, TagStoreRef, TagStoreWeak};
pub use typ::{TypeFullDto, TypeIdDto, TypeMetadataDto, TypeShallowDto, TypeStore, TypeStoreRef, TypeStoreWeak};
