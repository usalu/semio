use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::attribute::Attribute;
use crate::geom::{Coord, Plane};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::prop::Prop;
use crate::typ::TypeWeak;

pub type PieceRef = Arc<RwLock<Piece>>;
pub type PieceWeak = Weak<RwLock<Piece>>;

/// Placed instance of a [`crate::typ::Type`] inside a [`crate::design::Design`].
#[derive(Debug)]
pub struct Piece {
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
    pub props: Vec<Prop>,
    pub attributes: Vec<Attribute>,
    pub type_ref: Option<TypeWeak>,
    pub parent_design: Weak<RwLock<crate::design::Design>>,
    hash_cache: OnceLock<String>,
    flatten_cache: OnceLock<FlattenedPiece>,
}

/// Result of flattening a piece (world-space pose after connection resolution).
#[derive(Clone, Debug, Default)]
pub struct FlattenedPiece {
    pub plane: Plane,
    pub center: Coord,
}

impl Piece {
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
            flatten_cache: OnceLock::new(),
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
        self.flatten_cache = OnceLock::new();
    }

    pub fn invalidate_flatten(&mut self) {
        self.flatten_cache = OnceLock::new();
    }

    pub fn set_plane(&mut self, plane: Option<Plane>) {
        self.plane = plane;
        self.invalidate_hash();
    }

    pub fn set_center(&mut self, center: Option<Coord>) {
        self.center = center;
        self.invalidate_hash();
    }

    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
        self.invalidate_hash();
    }

    pub fn set_type(&mut self, type_ref: Option<TypeWeak>) {
        self.type_ref = type_ref;
        self.invalidate_hash();
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
        if let Some(p) = &self.plane { p.hash_into(w); }
        if let Some(c) = &self.center { c.hash_into(w); }
        w.opt_f64(self.scale);
        if let Some(p) = &self.mirror_plane { p.hash_into(w); }
        w.opt_bool(self.hidden).opt_bool(self.locked).opt_str(self.color.as_deref());
        for p in &self.props { p.hash_into(w); }
        for a in &self.attributes { a.hash_into(w); }
        if let Some(t) = self.type_ref.as_ref().and_then(|t| t.upgrade()) {
            if let Ok(t) = t.read() {
                w.str(t.guid.as_str());
            }
        }
    }
}

impl Default for Piece {
    fn default() -> Self { Self::new() }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PieceDto {
    #[serde(default)]
    pub guid: Option<Guid>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Prop>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typeGuid")]
    pub type_guid: Option<Guid>,
}

impl From<&Piece> for PieceDto {
    fn from(p: &Piece) -> Self {
        PieceDto {
            guid: Some(p.guid.clone()),
            id: p.id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
            plane: p.plane,
            center: p.center,
            scale: p.scale,
            mirror_plane: p.mirror_plane,
            hidden: p.hidden,
            locked: p.locked,
            color: p.color.clone(),
            props: p.props.clone(),
            attributes: p.attributes.clone(),
            type_guid: p
                .type_ref
                .as_ref()
                .and_then(|t| t.upgrade())
                .and_then(|t| t.read().ok().map(|t| t.guid.clone())),
        }
    }
}

impl Piece {
    pub fn from_dto(d: PieceDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
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
            props: d.props,
            attributes: d.attributes,
            type_ref: None,
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
            flatten_cache: OnceLock::new(),
        }
    }
}
