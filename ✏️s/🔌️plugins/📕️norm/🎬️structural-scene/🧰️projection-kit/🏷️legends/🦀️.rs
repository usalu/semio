//! 🏷️ Legends and layers: the layer table, the utilisation legend and the role legend every scene ends with.

use crate::*;
use std::collections::BTreeSet;

/// 🗂️ The layer ids in use: those of `entities` plus `annotation_layer` when any annotation exists.
pub fn used_layers<'a>(entities: &'a [SceneEntity], annotations: &[Annotation], annotation_layer: &'a str) -> BTreeSet<&'a str> {
    entities.iter().map(|e| e.layer.as_str()).chain(annotations.iter().map(|_| annotation_layer)).collect()
}

/// 🗂️ The `(id, English, German)` rows of `table` that are in use, in table order; layers listed in `hidden` start invisible.
pub fn layers(table: &[(&str, &str, &str)], used: &BTreeSet<&str>, hidden: &[&str]) -> Vec<Layer> {
    table.iter().filter(|(id, _, _)| used.contains(id)).map(|&(id, en, de)| Layer { id: id.to_string(), name: LocalizedCopy::new(en, de), visible: !hidden.contains(&id), locked: false }).collect()
}

/// 🌡️ The utilisation legend on the viridis ramp from 0 to 1.2.
pub fn utilisation_legend(title: LocalizedCopy) -> Legend {
    Legend { id: "utilisation".into(), title, scale: LegendScale::Utilisation { map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.2 } } }
}

/// 🎨️ The role legend: one entry per distinct legend role; `label_of` gives the `(English, German)` label and the role representing the entry, or `None` to skip an entity.
pub fn role_legend(entities: &[SceneEntity], label_of: impl Fn(&MaterialRole) -> Option<((&'static str, &'static str), MaterialRole)>) -> Vec<LegendRole> {
    let mut entries: Vec<LegendRole> = Vec::new();
    for entity in entities {
        let Some(((en, de), role)) = label_of(&entity.appearance.role) else { continue };
        if !entries.iter().any(|e| e.role == role) {
            entries.push(LegendRole { role, label: LocalizedCopy::new(en, de) });
        }
    }
    entries
}
