use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::design::DesignStoreWeak;
use crate::geom::{Coord, Plane};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::prop::{PropFullDto, PropShallowDto, PropStore};
use crate::typ::{TypeIdDto, TypeStoreWeak};

pub type PieceStoreRef = Arc<RwLock<PieceStore>>;
pub type PieceStoreWeak = Weak<RwLock<PieceStore>>;

/// Placed instance of a [`crate::typ::TypeStore`] inside a [`crate::design::DesignStore`].
#[derive(Debug)]
pub struct PieceStore {
    pub guid: Guid,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub plane: Option<Plane>,
    pub center: Option<Coord>,
    pub scale: Option<f64>,
    pub mirror_plane: Option<Plane>,
    pub hidden: Option<bool>,
    pub locked: Option<bool>,
    pub color: Option<String>,
    pub props: Vec<PropStore>,
    pub attributes: Vec<AttributeStore>,
    pub type_ref: Option<TypeStoreWeak>,
    pub parent_design: DesignStoreWeak,
    hash_cache: OnceLock<String>,
    flat_plane: OnceLock<Plane>,
    flat_center: OnceLock<Coord>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PieceIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PieceMetadataDto {
    pub guid: Guid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plane: Option<Plane>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<Coord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorPlane")]
    pub mirror_plane: Option<Plane>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<TypeIdDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design: Option<crate::design::DesignIdDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PieceShallowDto {
    #[serde(flatten)]
    pub meta: PieceMetadataDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PieceFullDto {
    #[serde(flatten)]
    pub meta: PieceMetadataDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropFullDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeFullDto>,
}

impl PieceStore {
    pub fn new() -> Self {
        Self {
            guid: Guid::new_v7(),
            id: None,
            name: None,
            description: None,
            plane: None,
            center: None,
            scale: None,
            mirror_plane: None,
            hidden: None,
            locked: None,
            color: None,
            props: Vec::new(),
            attributes: Vec::new(),
            type_ref: None,
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
            flat_plane: OnceLock::new(),
            flat_center: OnceLock::new(),
        }
    }

    pub fn invalidate_pose_caches(&mut self) {
        self.flat_plane = OnceLock::new();
        self.flat_center = OnceLock::new();
    }

    fn bubble_pose_invalidation_to_design(&self) {
        if let Some(d) = self.parent_design.upgrade() {
            if let Ok(mut dw) = d.write() {
                dw.invalidate_piece_pose_caches();
            }
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
        self.invalidate_pose_caches();
    }

    pub fn set_plane(&mut self, plane: Option<Plane>) {
        self.plane = plane;
        self.invalidate_hash();
        self.bubble_pose_invalidation_to_design();
    }

    pub fn set_center(&mut self, center: Option<Coord>) {
        self.center = center;
        self.invalidate_hash();
        self.bubble_pose_invalidation_to_design();
    }

    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
        self.invalidate_hash();
    }

    pub fn set_type(&mut self, type_ref: Option<TypeStoreWeak>) {
        self.type_ref = type_ref;
        self.invalidate_hash();
    }

    /// World-space plane for this piece (identity layout; connection propagation is incremental).
    pub fn flat_plane(&self) -> Plane {
        *self.flat_plane.get_or_init(|| {
            let mut plane = self.plane.unwrap_or_else(Plane::world_xy);
            if let Some(design) = self.parent_design.upgrade() {
                if let Ok(d) = design.read() {
                    for c in &d.connections {
                        if let Ok(conn) = c.read() {
                            let touches = |side: &crate::side::SideStore| -> bool {
                                side.piece
                                    .upgrade()
                                    .and_then(|p| p.read().ok().map(|p| p.guid == self.guid))
                                    .unwrap_or(false)
                            };
                            if touches(&conn.connected) || touches(&conn.connecting) {
                                let other = if touches(&conn.connected) {
                                    &conn.connecting
                                } else {
                                    &conn.connected
                                };
                                if let Some(op) = other.piece.upgrade() {
                                    if let Ok(op_read) = op.read() {
                                        let other_plane = op_read.plane.unwrap_or_else(Plane::world_xy);
                                        plane = other_plane;
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
            plane
        })
    }

    /// World-space center for this piece (identity layout; connection propagation is incremental).
    pub fn flat_center(&self) -> Coord {
        *self.flat_center.get_or_init(|| {
            let mut center = self.center.unwrap_or_default();
            if let Some(design) = self.parent_design.upgrade() {
                if let Ok(d) = design.read() {
                    for c in &d.connections {
                        if let Ok(conn) = c.read() {
                            let touches = |side: &crate::side::SideStore| -> bool {
                                side.piece
                                    .upgrade()
                                    .and_then(|p| p.read().ok().map(|p| p.guid == self.guid))
                                    .unwrap_or(false)
                            };
                            if touches(&conn.connected) || touches(&conn.connecting) {
                                let other = if touches(&conn.connected) {
                                    &conn.connecting
                                } else {
                                    &conn.connected
                                };
                                if let Some(op) = other.piece.upgrade() {
                                    if let Ok(op_read) = op.read() {
                                        center = op_read.center.unwrap_or_default();
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
            center
        })
    }

    pub fn hash(&self) -> String {
        self.hash_cache
            .get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
            .clone()
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("piece")
            .str(self.guid.as_str())
            .opt_str(self.id.as_deref())
            .opt_str(self.name.as_deref())
            .opt_str(self.description.as_deref());
        if let Some(p) = &self.plane {
            p.hash_into(w);
        }
        if let Some(c) = &self.center {
            c.hash_into(w);
        }
        w.opt_f64(self.scale);
        if let Some(p) = &self.mirror_plane {
            p.hash_into(w);
        }
        w.opt_bool(self.hidden).opt_bool(self.locked).opt_str(self.color.as_deref());
        for p in &self.props {
            p.hash_into(w);
        }
        for a in &self.attributes {
            a.hash_into(w);
        }
        if let Some(t) = self.type_ref.as_ref().and_then(|t| t.upgrade()) {
            if let Ok(t) = t.read() {
                w.str(t.guid.as_str());
            }
        }
    }

    pub fn from_id_dto(d: PieceIdDto) -> Self {
        Self {
            guid: d.guid,
            id: None,
            name: None,
            description: None,
            plane: None,
            center: None,
            scale: None,
            mirror_plane: None,
            hidden: None,
            locked: None,
            color: None,
            props: Vec::new(),
            attributes: Vec::new(),
            type_ref: None,
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
            flat_plane: OnceLock::new(),
            flat_center: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: PieceMetadataDto) -> Self {
        Self {
            guid: d.guid,
            id: d.id,
            name: d.name,
            description: d.description,
            plane: d.plane,
            center: d.center,
            scale: d.scale,
            mirror_plane: d.mirror_plane,
            hidden: d.hidden,
            locked: d.locked,
            color: d.color,
            props: Vec::new(),
            attributes: Vec::new(),
            type_ref: None,
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
            flat_plane: OnceLock::new(),
            flat_center: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: PieceShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(d.meta);
        s.props = d.props.into_iter().map(PropStore::from_shallow_dto).collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_shallow_dto).collect();
        s
    }

    pub fn from_full_dto(d: PieceFullDto) -> Self {
        let mut s = Self::from_metadata_dto(d.meta);
        s.props = d.props.into_iter().map(PropStore::from_full_dto).collect();
        s.attributes = d.attributes.into_iter().map(AttributeStore::from_full_dto).collect();
        s
    }

    pub fn to_id_dto(&self) -> PieceIdDto {
        PieceIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> PieceMetadataDto {
        let r#type = self
            .type_ref
            .as_ref()
            .and_then(|t| t.upgrade())
            .and_then(|t| t.read().ok().map(|t| TypeIdDto { guid: t.guid.clone() }));
        let design = self
            .parent_design
            .upgrade()
            .and_then(|d| d.read().ok().map(|d| crate::design::DesignIdDto { guid: d.guid.clone() }));
        PieceMetadataDto {
            guid: self.guid.clone(),
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            plane: self.plane,
            center: self.center,
            scale: self.scale,
            mirror_plane: self.mirror_plane,
            hidden: self.hidden,
            locked: self.locked,
            color: self.color.clone(),
            r#type,
            design,
        }
    }

    pub fn to_shallow_dto(&self) -> PieceShallowDto {
        PieceShallowDto {
            meta: self.to_metadata_dto(),
            props: self.props.iter().map(PropStore::to_shallow_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
        }
    }

    pub fn to_full_dto(&self) -> PieceFullDto {
        PieceFullDto {
            meta: self.to_metadata_dto(),
            props: self.props.iter().map(PropStore::to_full_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
        }
    }
}

impl Default for PieceStore {
    fn default() -> Self {
        Self::new()
    }
}
