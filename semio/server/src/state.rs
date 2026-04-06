// #region 🔖Header
// [👤semio📚server💻semio-session🔖state](repo://p/u/semio/b/l/server/f/state.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// In-memory session state loaded from and persisted to PostgreSQL.
// #endregion 🔖Header

use std::collections::BTreeMap;
use uuid::Uuid;

use crate::domain::*;

// #region 🔖SessionState
// SessionState MUST hold the full typed in-memory state for one session.

#[derive(Debug, Clone)]
pub struct SessionState {
    pub session_id: SessionId,
    pub domain_version: DomainVersion,
    pub semio_version: SemioVersion,
    pub status: SessionStatus,
    pub kit: KitState,
    pub authors: BTreeMap<Uuid, AuthorState>,
    pub locations: BTreeMap<Uuid, LocationState>,
    pub folders: BTreeMap<Uuid, FolderState>,
    pub files: BTreeMap<Uuid, FileState>,
    pub tags: BTreeMap<Uuid, TagState>,
    pub concepts: BTreeMap<Uuid, ConceptState>,
    pub ports: BTreeMap<Uuid, PortState>,
    pub qualities: BTreeMap<Uuid, QualityState>,
    pub types: BTreeMap<Uuid, TypeState>,
    pub designs: BTreeMap<Uuid, DesignState>,
    pub semio_people: BTreeMap<(Uuid, String), SemioPersonState>,
}

// #endregion 🔖SessionState

// #region 🔖Entity States
// Entity States MUST mirror the canonical DB rows in typed Rust structs.

#[derive(Debug, Clone)]
pub struct KitState {
    pub kit_id: Uuid,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub preview: Option<String>,
    pub remote: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct AuthorState {
    pub author_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct LocationState {
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct FolderState {
    pub folder_id: Uuid,
    pub name: String,
    pub parent_folder_id: Option<Uuid>,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct FileState {
    pub file_id: Uuid,
    pub name: String,
    pub remote: Option<String>,
    pub folder_id: Option<Uuid>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub blob: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct TagState {
    pub tag_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct ConceptState {
    pub concept_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct PortState {
    pub port_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub compatible_port_ids: Vec<Uuid>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct QualityState {
    pub quality_id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub unit: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct TypeState {
    pub type_id: Uuid,
    pub name: String,
    pub parent_type_id: Option<Uuid>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub folder: Option<String>,
    pub unit: Option<String>,
    pub stock: Option<i32>,
    pub is_abstract: Option<bool>,
    pub virtual_type: Option<bool>,
    pub location_id: Option<Uuid>,
    pub connectors: BTreeMap<Uuid, ConnectorState>,
    pub models: BTreeMap<Uuid, ModelState>,
    pub props: BTreeMap<Uuid, PropState>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct ConnectorState {
    pub connector_id: Uuid,
    pub name: Option<String>,
    pub t: f64,
    pub point: [f64; 3],
    pub direction: [f64; 3],
    pub description: Option<String>,
    pub port_id: Option<Uuid>,
    pub mandatory: Option<bool>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct ModelState {
    pub model_id: Uuid,
    pub file_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct PropState {
    pub prop_id: Uuid,
    pub quality_id: Uuid,
    pub value: String,
    pub unit: Option<String>,
    pub lifecycle: Lifecycle,
}

// #endregion 🔖Entity States

// #region 🔖Design State
// Design State MUST hold nested design entities.

#[derive(Debug, Clone)]
pub struct DesignState {
    pub design_id: Uuid,
    pub name: String,
    pub parent_design_id: Option<Uuid>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub folder: Option<String>,
    pub unit: Option<String>,
    pub is_abstract: Option<bool>,
    pub can_scale: Option<bool>,
    pub can_mirror: Option<bool>,
    pub active_layer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub pieces: BTreeMap<Uuid, PieceState>,
    pub connections: BTreeMap<Uuid, ConnectionState>,
    pub layers: BTreeMap<Uuid, LayerState>,
    pub groups: BTreeMap<Uuid, GroupState>,
    pub stats: BTreeMap<Uuid, StatState>,
    pub props: BTreeMap<Uuid, PropState>,
    pub lifecycle: Lifecycle,
}

// #endregion 🔖Design State

// #region 🔖Piece, Connection, Layer, Group, Stat States

#[derive(Debug, Clone)]
pub struct PieceState {
    pub piece_id: Uuid,
    pub name: Option<String>,
    pub type_id: Option<Uuid>,
    pub design_ref_id: Option<Uuid>,
    pub plane: Option<PlaneState>,
    pub center: Option<[f64; 2]>,
    pub scale: Option<f64>,
    pub mirror_plane: Option<PlaneState>,
    pub is_hidden: Option<bool>,
    pub is_locked: Option<bool>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct PlaneState {
    pub origin: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
}

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub connection_id: Uuid,
    pub connected_piece_id: Uuid,
    pub connected_design_piece_id: Option<Uuid>,
    pub connected_connector_id: Option<Uuid>,
    pub connecting_piece_id: Uuid,
    pub connecting_design_piece_id: Option<Uuid>,
    pub connecting_connector_id: Option<Uuid>,
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub u: Option<f64>,
    pub v: Option<f64>,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct LayerState {
    pub layer_id: Uuid,
    pub path: String,
    pub is_hidden: Option<bool>,
    pub is_locked: Option<bool>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct GroupState {
    pub group_id: Uuid,
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub piece_ids: Vec<Uuid>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct StatState {
    pub stat_id: Uuid,
    pub quality_id: Uuid,
    pub unit: Option<String>,
    pub min: Option<f64>,
    pub min_excluded: Option<bool>,
    pub max: Option<f64>,
    pub max_excluded: Option<bool>,
    pub lifecycle: Lifecycle,
}

// #endregion 🔖Piece, Connection, Layer, Group, Stat States

// #region 🔖Semio Person State

#[derive(Debug, Clone)]
pub struct SemioPersonState {
    pub person_id: Uuid,
    pub frontend_id: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub is_present: bool,
    pub cursor: Option<[f64; 2]>,
    pub look: Option<LookState>,
    pub selected_piece_ids: Vec<Uuid>,
    pub selected_design_ids: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct LookState {
    pub position: [f64; 3],
    pub forward: [f64; 3],
    pub up: [f64; 3],
}

// #endregion 🔖Semio Person State
