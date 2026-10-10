//! 🧱️ Which element of the model holds a surface of the thermal envelope and which type gives it its layers. The inference decides (`EnvelopeSurface::holder` is the id of the wall, curtain wall, slab, roof, ceiling or opening
//! whose construction the surface has, `EnvelopeSurface::construction` the id of its type); this module only names the collection the id belongs to and reads the layer stack of the type, so the gbXML export takes the layers of a
//! construction and the IFC export the element a `ThermalTransmittance` belongs to without restating the rule.
//! 📎 https://www.iso.org/standard/65708.html

use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{EnvelopeSurface, SurfaceKind};
use crate::{Layer, ModelSnapshot};

/// 🧱️ The element a surface belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Holder<'a> {
    Wall(&'a str),
    CurtainWall(&'a str),
    Slab(&'a str),
    Roof(&'a str),
    Ceiling(&'a str),
    Window(&'a str),
    Door(&'a str),
}

impl<'a> Holder<'a> {
    /// 🔑️ The model id of the element.
    pub fn id(&self) -> &'a str {
        match *self {
            Self::Wall(id) | Self::CurtainWall(id) | Self::Slab(id) | Self::Roof(id) | Self::Ceiling(id) | Self::Window(id) | Self::Door(id) => id,
        }
    }
}

/// 🧱️ The element that holds `surface`; `None` for a surface the inference gave no holder or whose holder is not in the model.
pub fn holder_of<'a>(model: &ModelSnapshot, surface: &'a EnvelopeSurface) -> Option<Holder<'a>> {
    let id = surface.holder.as_str();
    match surface.kind {
        SurfaceKind::Wall => model.walls.contains_key(id).then_some(Holder::Wall(id)),
        SurfaceKind::CurtainWall => model.curtain_walls.contains_key(id).then_some(Holder::CurtainWall(id)),
        SurfaceKind::Window => model.openings.contains_key(id).then_some(Holder::Window(id)),
        SurfaceKind::Door => model.openings.contains_key(id).then_some(Holder::Door(id)),
        SurfaceKind::Floor | SurfaceKind::Ceiling => {
            if model.slabs.contains_key(id) {
                Some(Holder::Slab(id))
            } else if model.roofs.contains_key(id) {
                Some(Holder::Roof(id))
            } else {
                model.ceilings.contains_key(id).then_some(Holder::Ceiling(id))
            }
        }
    }
}

/// 🍰️ The type id and the layer stack of the construction of an opaque `surface` held by `holder`; windows, doors and curtain walls have none.
pub fn layers_of<'a>(model: &'a ModelSnapshot, surface: &EnvelopeSurface, holder: Holder<'_>) -> Option<(&'a str, &'a [Layer])> {
    let id = surface.construction.as_str();
    match holder {
        Holder::Wall(_) => model.wall_types.get_key_value(id).map(|(key, kind)| (key.as_str(), kind.layers.as_slice())),
        Holder::Slab(_) => model.slab_types.get_key_value(id).map(|(key, kind)| (key.as_str(), kind.layers.as_slice())),
        Holder::Roof(_) => model.roof_types.get_key_value(id).map(|(key, kind)| (key.as_str(), kind.layers.as_slice())),
        Holder::Ceiling(_) => model.ceiling_types.get_key_value(id).map(|(key, kind)| (key.as_str(), kind.layers.as_slice())),
        Holder::CurtainWall(_) | Holder::Window(_) | Holder::Door(_) => None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
