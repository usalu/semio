//! 🎨️ Target styling of the CAD pick engine: the scene palette and the per-target material style. See [`super`].

use super::*;

//#region 🔖️Style
/// 🎨️ A colour slot on the spatial scene palette. The plugin never resolves theme hex — W1k's rule
/// is that token ids cross the wire and the renderer resolves them, so a style names its slot and
/// only an authored typology style carries a literal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpatialColorRef {
    Token(&'static str),
    Hex(String),
}

/// 🎨️ Palette slots the spatial canvas materials read, as the CSS variables React resolves.
pub struct SpatialScenePalette;

impl SpatialScenePalette {
    pub const SELECTED: &'static str = "--color-changed-selected";
    pub const SELECTED_EMISSIVE: &'static str = "--active-base";
    pub const HOVERED: &'static str = "--color-changed-hovered";
    pub const HOVERED_EMISSIVE: &'static str = "--hover-panel";
    pub const VERTEX: &'static str = "--foreground";
    pub const VERTEX_EMISSIVE: &'static str = "--hover-base";
    pub const EDGE: &'static str = "--border-normal-color";
    pub const EDGE_EMISSIVE: &'static str = "--hover-base";
    pub const OBJECT: &'static str = "--muted-foreground";
    pub const OBJECT_EMISSIVE: &'static str = "--hover-panel";
    pub const FACE: &'static str = "--accent-secondary";
    pub const FACE_EMISSIVE: &'static str = "--hover-window";
}

/// 🌫️ `WORLD_LOCKED_OPACITY_SCALE` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:178`) — how far a locked
/// entity's highlight is dimmed.
pub const WORLD_LOCKED_OPACITY_SCALE: f64 = 0.35;

/// 🎨️ The resolved highlight material for one pick target in one interaction state.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialTargetStyle {
    pub color: SpatialColorRef,
    pub emissive: SpatialColorRef,
    pub opacity: f64,
    pub line_width: f64,
}

/// 🎨️ Material props a typology style contributes to a committed mesh.
pub fn typology_style_to_material_props(style: &ResolvedTypologyStyle) -> (String, String, f64) {
    (style.color.clone(), style.color.clone(), style.opacity)
}

/// 🎨️ Per-solid typology style from object-row membership (React's `createSolidTypologyStyleResolver`).
pub fn solid_typology_style(objects: &[CadObject], geometry: &CadGeometry, model_definition_id: &str, solid_id: &str) -> Option<ResolvedTypologyStyle> {
    let index = build_geometry_typology_index(objects, geometry, model_definition_id);
    index.get(&format!("solid:{solid_id}")).map(|typology| resolve_typology_style(typology))
}

/// 🎨️ The exact selected/hovered/locked/typology/default cascade React's `targetStyle` applies.
pub fn target_style(target: &SpatialPickTarget, hovered: bool, selected: bool, typology_style: Option<&ResolvedTypologyStyle>, locked: bool) -> SpatialTargetStyle {
    let is_vertex = target.kind == SpatialPickTargetKind::Vertex;
    let is_typology_object = target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none();
    if selected {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::SELECTED), emissive: SpatialColorRef::Token(SpatialScenePalette::SELECTED_EMISSIVE), opacity: if is_vertex { 1.0 } else { 0.34 }, line_width: 9.0 };
    }
    if hovered {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::HOVERED), emissive: SpatialColorRef::Token(SpatialScenePalette::HOVERED_EMISSIVE), opacity: if is_vertex { 1.0 } else { 0.28 }, line_width: 8.0 };
    }
    if locked {
        let dim = (if is_vertex { 0.55 } else { 0.12 }) * WORLD_LOCKED_OPACITY_SCALE;
        if let Some(style) = typology_style {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: dim, line_width: 4.0 };
        }
        let (color, emissive, line_width) = if is_vertex {
            (SpatialScenePalette::VERTEX, SpatialScenePalette::VERTEX_EMISSIVE, 4.0)
        } else if target.kind == SpatialPickTargetKind::Edge {
            (SpatialScenePalette::EDGE, SpatialScenePalette::EDGE_EMISSIVE, 4.0)
        } else if is_typology_object {
            (SpatialScenePalette::OBJECT, SpatialScenePalette::OBJECT_EMISSIVE, 5.0)
        } else {
            (SpatialScenePalette::FACE, SpatialScenePalette::FACE_EMISSIVE, 4.0)
        };
        return SpatialTargetStyle { color: SpatialColorRef::Token(color), emissive: SpatialColorRef::Token(emissive), opacity: dim, line_width };
    }
    if let Some(style) = typology_style {
        if is_vertex {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: 1.0, line_width: 5.0 };
        }
        if target.kind == SpatialPickTargetKind::Edge {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.edge_color.clone()), emissive: SpatialColorRef::Hex(style.edge_color.clone()), opacity: 0.85, line_width: 5.0 };
        }
        if is_typology_object {
            return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: style.opacity, line_width: 7.0 };
        }
        return SpatialTargetStyle { color: SpatialColorRef::Hex(style.color.clone()), emissive: SpatialColorRef::Hex(style.color.clone()), opacity: style.opacity.min(0.42), line_width: 5.0 };
    }
    if is_vertex {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::VERTEX), emissive: SpatialColorRef::Token(SpatialScenePalette::VERTEX_EMISSIVE), opacity: 1.0, line_width: 5.0 };
    }
    if target.kind == SpatialPickTargetKind::Edge {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::EDGE), emissive: SpatialColorRef::Token(SpatialScenePalette::EDGE_EMISSIVE), opacity: 0.8, line_width: 5.0 };
    }
    if is_typology_object {
        return SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::OBJECT), emissive: SpatialColorRef::Token(SpatialScenePalette::OBJECT_EMISSIVE), opacity: 0.28, line_width: 7.0 };
    }
    SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::FACE), emissive: SpatialColorRef::Token(SpatialScenePalette::FACE_EMISSIVE), opacity: 0.16, line_width: 5.0 }
}

/// 🏷️ Chrome label for one typology toggle row, authored label first.
pub fn typology_toggle_row_label(typology_id: &str) -> String {
    spatial_typology_toggle_label(typology_id, load_typology(typology_id).map(|spec| spec.label.as_str()))
}
//#endregion 🔖️Style
