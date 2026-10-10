//! 🎨️ The PBR materials of the glTF export: the colour of a model material, translucent glazing, and a neutral colour per element family for faces that name no material.
//! The colour of a face group is the one the 3D viewer paints (`group_color`), so the exported scene looks like the editor.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#reference-material

use super::document::GltfMaterial;
use crate::render::world::{family_color, group_color};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{parts, SolidFamily, SolidGroup};
use crate::{MaterialCategory, ModelSnapshot};
use std::collections::BTreeMap;

/// 🔑️ What a face group is made of: a model material, glazing without a material, or nothing (the colour of its element family).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaterialKey {
    Model(String),
    Glass,
    System(String),
    Family(SolidFamily),
}

/// 🔑️ The material key of a face group of an element of `family`.
pub fn key_of(snapshot: &ModelSnapshot, family: SolidFamily, group: &SolidGroup) -> MaterialKey {
    if snapshot.materials.contains_key(&group.material) {
        MaterialKey::Model(group.material.clone())
    } else if group.part == parts::GLASS {
        MaterialKey::Glass
    } else if family == SolidFamily::Mep && crate::standards::v1::subsets::any::schema::inferences::mep::part_colour(&group.part).is_some() {
        MaterialKey::System(group.part.clone())
    } else {
        MaterialKey::Family(family)
    }
}

const GLASS_ALPHA: f32 = 0.35;

fn family_name(family: SolidFamily) -> &'static str {
    match family {
        SolidFamily::Wall => "Wall",
        SolidFamily::CurtainWall => "Curtain wall",
        SolidFamily::Window => "Window",
        SolidFamily::Door => "Door",
        SolidFamily::Column => "Column",
        SolidFamily::Beam => "Beam",
        SolidFamily::Slab => "Slab",
        SolidFamily::Roof => "Roof",
        SolidFamily::Stair => "Stair",
        SolidFamily::Ramp => "Ramp",
        SolidFamily::Railing => "Railing",
        SolidFamily::Ceiling => "Ceiling",
        SolidFamily::WallSweep => "Wall sweep",
        SolidFamily::Component => "Component",
        SolidFamily::Mep => "MEP element",
    }
}

fn definition(snapshot: &ModelSnapshot, key: &MaterialKey, family: SolidFamily, group: &SolidGroup) -> GltfMaterial {
    let color = group_color(snapshot, family, group);
    match key {
        MaterialKey::Model(id) => {
            let material = &snapshot.materials[id];
            match material.category {
                MaterialCategory::Glass => GltfMaterial { name: material.name.clone(), color: [color[0], color[1], color[2], GLASS_ALPHA], metallic: 0.0, roughness: 0.05, blend: true },
                MaterialCategory::Metal => GltfMaterial { name: material.name.clone(), color, metallic: 1.0, roughness: 0.4, blend: false },
                _ => GltfMaterial { name: material.name.clone(), color, metallic: 0.0, roughness: 0.9, blend: false },
            }
        }
        MaterialKey::Glass => GltfMaterial { name: "Glazing".into(), color, metallic: 0.0, roughness: 0.05, blend: true },
        MaterialKey::System(system) => GltfMaterial { name: format!("MEP {system}"), color, metallic: 0.0, roughness: 0.8, blend: false },
        MaterialKey::Family(family) => GltfMaterial { name: family_name(*family).into(), color: family_color(*family), metallic: 0.0, roughness: 0.9, blend: false },
    }
}

/// 🎨️ The materials met while exporting, numbered in order of first use.
#[derive(Debug, Default)]
pub struct Palette {
    numbers: BTreeMap<MaterialKey, usize>,
    pub materials: Vec<GltfMaterial>,
}

impl Palette {
    /// 🔢️ The glTF material index of a face group, defining the material on first use.
    pub fn index_of(&mut self, snapshot: &ModelSnapshot, family: SolidFamily, group: &SolidGroup) -> usize {
        let key = key_of(snapshot, family, group);
        if let Some(index) = self.numbers.get(&key) {
            return *index;
        }
        let index = self.materials.len();
        self.materials.push(definition(snapshot, &key, family, group));
        self.numbers.insert(key, index);
        index
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
