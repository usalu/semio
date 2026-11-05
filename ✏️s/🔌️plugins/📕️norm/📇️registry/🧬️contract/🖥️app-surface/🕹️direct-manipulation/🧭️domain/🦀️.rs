//! 🧭️ Interaction domain generator: `norm.<variant>` with one granularity per scene entity kind plus `check` and `loadCase` for cross-selection. See [`super`].

use crate::viewport3d::picking::{granularity, GRANULARITY_CHECK, GRANULARITY_LOAD_CASE};
use semio_plugin::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
use semio_ui_locale::LocalizedLabel;
use semio_s_plugin_norm_structural_scene::EntityKind;

pub const POINTER_CHANNEL: &str = "pointer";
pub const LIST_CHANNEL: &str = "list";
pub const UTILITY_TRANSFORM: &str = "transform";

/// 🆔️ Interaction domain id of an artifact variant.
pub fn interaction_domain_id(variant: &str) -> String {
    format!("norm.{variant}")
}

fn kind_label(kind: EntityKind) -> (LocalizedLabel, &'static str) {
    match kind {
        EntityKind::Member => (LocalizedLabel::native("Member", "Stab"), "minus"),
        EntityKind::Plate => (LocalizedLabel::native("Plate", "Platte"), "square"),
        EntityKind::Shell => (LocalizedLabel::native("Shell", "Schale"), "circle"),
        EntityKind::Solid => (LocalizedLabel::native("Solid", "Volumenkörper"), "box"),
        EntityKind::Fastener => (LocalizedLabel::native("Fastener", "Verbindungsmittel"), "wrench"),
        EntityKind::Support => (LocalizedLabel::native("Support", "Lager"), "anchor"),
        EntityKind::Load => (LocalizedLabel::native("Load", "Last"), "arrow-down"),
        EntityKind::Surface => (LocalizedLabel::native("Surface", "Fläche"), "layers"),
        EntityKind::Group => (LocalizedLabel::native("Group", "Gruppe"), "group"),
    }
}

/// 🕹️ The interaction definition generated from the entity kinds an artifact projects: tree rows, viewport picks, check rows and the inspector share one vocabulary.
pub fn viewport_interaction_definition(variant: &str, label: LocalizedLabel, kinds: &[EntityKind]) -> InteractionDefinition {
    let mut granularities: Vec<GranularityDefinition> = kinds.iter().map(|k| { let (label, icon) = kind_label(*k); GranularityDefinition { id: granularity(*k).into(), label, icon_id: icon.into() } }).collect();
    granularities.push(GranularityDefinition { id: GRANULARITY_CHECK.into(), label: LocalizedLabel::native("Check", "Nachweis"), icon_id: "check".into() });
    granularities.push(GranularityDefinition { id: GRANULARITY_LOAD_CASE.into(), label: LocalizedLabel::native("Load case", "Lastfall"), icon_id: "list".into() });
    InteractionDefinition {
        id: interaction_domain_id(variant),
        label,
        granularities,
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![POINTER_CHANNEL.into(), LIST_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Single, SelectionMode::Multiple],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}
