use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef};
use crate::design::DesignStoreWeak;
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::geom::{Coord, Plane};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
use crate::typ::{TypeIdDto, TypeStoreRef, TypeStoreWeak};

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
    pub props: Vec<PropStoreRef>,
    pub attributes: Vec<AttributeStoreRef>,
    pub type_ref: Option<TypeStoreWeak>,
    pub parent_design: DesignStoreWeak,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
    flat_plane: Cache<Plane>,
    flat_center: Cache<Coord>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<PropShallowDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<AttributeShallowDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct PieceFullDto {
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
            flat_plane: Cache::default(),
            flat_center: Cache::default(),
        }
    }

    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
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
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
            flat_plane: Cache::default(),
            flat_center: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Piece, self.guid.clone())
    }

    pub(crate) fn apply_metadata_fields(&mut self, d: PieceMetadataDto) {
        self.guid = d.guid;
        self.id = d.id;
        self.name = d.name;
        self.description = d.description;
        self.plane = d.plane;
        self.center = d.center;
        self.scale = d.scale;
        self.mirror_plane = d.mirror_plane;
        self.hidden = d.hidden;
        self.locked = d.locked;
        self.color = d.color;
        self.hash_cache.invalidate();
        self.flat_plane.invalidate();
        self.flat_center.invalidate();
    }

    pub(crate) fn apply_full_dto(
        &mut self,
        d: PieceFullDto,
        design_weak: DesignStoreWeak,
        type_index: &HashMap<Guid, TypeStoreRef>,
    ) {
        self.apply_metadata_fields(PieceMetadataDto {
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
            r#type: d.r#type.clone(),
            design: d.design.clone(),
        });
        if let Some(tid) = d.r#type.as_ref().map(|t| t.guid.clone()) {
            if let Some(tr) = type_index.get(&tid) {
                self.type_ref = Some(Arc::downgrade(tr));
            }
        }
        self.parent_design = design_weak;
        self.props = d
            .props
            .into_iter()
            .map(|p| Arc::new(RwLock::new(PropStore::from_full_dto(p))))
            .collect();
        self.attributes = d
            .attributes
            .into_iter()
            .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
            .collect();
    }

    pub fn invalidate_flat_pose(&self) {
        self.flat_plane.invalidate();
        self.flat_center.invalidate();
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
        self.invalidate_flat_pose();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(d) = self.parent_design.upgrade() {
            if let Ok(dr) = d.read() {
                dr.invalidate_hash();
            }
        }
    }

    fn bubble_design_flatten(&self) {
        if let Some(d) = self.parent_design.upgrade() {
            if let Ok(d) = d.read() {
                d.invalidate_flatten();
            }
        }
    }

    pub fn set_plane(&mut self, plane: Option<Plane>) {
        if self.plane == plane {
            return;
        }
        self.plane = plane;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "plane",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_center(&mut self, center: Option<Coord>) {
        if self.center == center {
            return;
        }
        self.center = center;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "center",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_color(&mut self, color: Option<String>) {
        if self.color == color {
            return;
        }
        self.color = color;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "color",
        });
        self.invalidate_hash();
    }

    pub fn set_type_weak(&mut self, type_ref: Option<TypeStoreWeak>) {
        self.type_ref = type_ref;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "type",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_id(&mut self, id: Option<String>) {
        if self.id == id {
            return;
        }
        self.id = id;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "id",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_name(&mut self, name: Option<String>) {
        if self.name == name {
            return;
        }
        self.name = name;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "name",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        if self.description == description {
            return;
        }
        self.description = description;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "description",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_scale(&mut self, scale: Option<f64>) {
        if self.scale == scale {
            return;
        }
        self.scale = scale;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "scale",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_mirror_plane(&mut self, mirror_plane: Option<Plane>) {
        if self.mirror_plane == mirror_plane {
            return;
        }
        self.mirror_plane = mirror_plane;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "mirrorPlane",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_hidden(&mut self, hidden: Option<bool>) {
        if self.hidden == hidden {
            return;
        }
        self.hidden = hidden;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "hidden",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    pub fn set_locked(&mut self, locked: Option<bool>) {
        if self.locked == locked {
            return;
        }
        self.locked = locked;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "locked",
        });
        self.invalidate_hash();
        self.bubble_design_flatten();
    }

    /// World-space plane from design flatten cache.
    pub fn flat_plane(&self) -> Plane {
        self.flat_plane.get_or_init(|| {
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(d) = d.read() {
                    if let Some((pl, _)) = d.flatten_map().get(&self.guid) {
                        return *pl;
                    }
                }
            }
            self.plane.unwrap_or_else(Plane::world_xy)
        })
    }

    /// World-space center from design flatten cache.
    pub fn flat_center(&self) -> Coord {
        self.flat_center.get_or_init(|| {
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(d) = d.read() {
                    if let Some((_, ce)) = d.flatten_map().get(&self.guid) {
                        return *ce;
                    }
                }
            }
            self.center.unwrap_or_default()
        })
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
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
            if let Ok(p) = p.read() {
                p.hash_into(w);
            }
        }
        for a in &self.attributes {
            if let Ok(a) = a.read() {
                a.hash_into(w);
            }
        }
        if let Some(t) = self.type_ref.as_ref().and_then(|t| t.upgrade()) {
            if let Ok(t) = t.read() {
                w.str(t.guid.as_str());
            }
        }
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
        let m = self.to_metadata_dto();
        PieceShallowDto {
            guid: m.guid,
            id: m.id,
            name: m.name,
            description: m.description,
            plane: m.plane,
            center: m.center,
            scale: m.scale,
            mirror_plane: m.mirror_plane,
            hidden: m.hidden,
            locked: m.locked,
            color: m.color,
            r#type: m.r#type,
            design: m.design,
            props: self
                .props
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                .collect(),
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                .collect(),
        }
    }

    pub fn to_full_dto(&self) -> PieceFullDto {
        let m = self.to_metadata_dto();
        PieceFullDto {
            guid: m.guid,
            id: m.id,
            name: m.name,
            description: m.description,
            plane: m.plane,
            center: m.center,
            scale: m.scale,
            mirror_plane: m.mirror_plane,
            hidden: m.hidden,
            locked: m.locked,
            color: m.color,
            r#type: m.r#type,
            design: m.design,
            props: self
                .props
                .iter()
                .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                .collect(),
            attributes: self
                .attributes
                .iter()
                .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                .collect(),
        }
    }
}

impl Default for PieceStore {
    fn default() -> Self {
        Self::new()
    }
}
