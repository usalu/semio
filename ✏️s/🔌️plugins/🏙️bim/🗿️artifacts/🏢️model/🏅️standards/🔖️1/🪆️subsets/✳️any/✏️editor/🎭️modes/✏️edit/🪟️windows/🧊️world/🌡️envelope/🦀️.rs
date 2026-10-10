//! 🌡️ The envelope overlay in the world window: when the window's `energy_overlay` setting is on, the `EnvelopeSurface` polygons of the conditioned spaces are added to the scene as one extra mesh and instance per building
//! placement, coloured by U-value or by boundary condition (see [`crate::render::envelope`]). Every surface gets a point marker whose text, in both languages, names its id, kind, boundary, area and U-value: the tooltip
//! of the pick and the accessible alternative of the overlay. In U-value mode and with one placement the framework heatmap paints the same scale and draws its on-screen legend; in every case the legend is also the title of the
//! annotation layer and the text of the overlay toggle. The overlay instances carry no interaction id: they are looked at, never picked.

use super::config::BimWorldWindowConfig;
use crate::editor::bim::terminology::BimLabels;
use crate::render::envelope::{self, Mode, Part};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::{Boundary, EnvelopeSurface, SurfaceKind};
use crate::{ModelInference, ModelSnapshot};
use semio_framework_plugin::{DslValue, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dScalarDomain, World3dScalarField, World3dScalarRange, World3dText, WORLD3D_ANNOTATIONS_MAX};

/// 🏷️ The id of the mesh and the instance of the overlay group `index`.
pub fn mesh_id(index: usize) -> String {
    format!("bim-energy-envelope/{index}")
}

/// 🎨️ The mode the window colours by: its `energy_mode` key, the U-value for an empty or unknown key.
pub fn mode(config: &BimWorldWindowConfig) -> Mode {
    Mode::parse(&config.energy_mode).unwrap_or_default()
}

/// 👁️ Whether the overlay draws the surfaces of a storey: not on a hidden storey, and on the isolated storey when there is one.
pub fn shown(config: &BimWorldWindowConfig, storey: &str) -> bool {
    (config.isolated_storey.is_empty() || config.isolated_storey == storey) && !config.hidden_storeys.iter().any(|hidden| hidden == storey)
}

//#region 🔖️Texts
fn measure(value: f64, german: bool) -> String {
    let text = format!("{value:.2}");
    if german {
        text.replace('.', ",")
    } else {
        text
    }
}

fn kind_text(labels: &BimLabels, kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::Wall => labels.kind_wall.as_str(),
        SurfaceKind::CurtainWall => labels.kind_curtain_wall.as_str(),
        SurfaceKind::Floor => labels.env_surface_floor.as_str(),
        SurfaceKind::Ceiling => labels.env_surface_ceiling.as_str(),
        SurfaceKind::Window => labels.env_surface_window.as_str(),
        SurfaceKind::Door => labels.env_surface_door.as_str(),
    }
}

/// 🔭️ The localized name of a boundary condition.
pub fn boundary_text(labels: &BimLabels, boundary: Boundary) -> &'static str {
    match boundary {
        Boundary::Exterior => labels.env_boundary_exterior.as_str(),
        Boundary::Ground => labels.env_boundary_ground.as_str(),
        Boundary::Adjacent => labels.env_boundary_adjacent.as_str(),
        Boundary::Adiabatic => labels.env_boundary_adiabatic.as_str(),
    }
}

/// 🏷️ What a pick or hover of a surface shows in one language: its id, kind with the construction (type) that gives it its layers, boundary (with the neighbour), net area and U-value, or that it states none.
pub fn surface_text(labels: &BimLabels, german: bool, surface: &EnvelopeSurface) -> String {
    let boundary = if surface.adjacent.is_empty() { boundary_text(labels, surface.boundary).to_string() } else { format!("{} {}", boundary_text(labels, surface.boundary), surface.adjacent) };
    let kind = if surface.construction.is_empty() { kind_text(labels, surface.kind).to_string() } else { format!("{} {}", kind_text(labels, surface.kind), surface.construction) };
    let text = if surface.u_value.is_some() { labels.env_text } else { labels.env_text_no_u };
    text.as_str()
        .replace("{id}", &surface.id)
        .replace("{kind}", &kind)
        .replace("{boundary}", &boundary)
        .replace("{area}", &measure(surface.area, german))
        .replace("{u}", &surface.u_value.map(|value| measure(value, german)).unwrap_or_default())
}

/// 🏷️ The legend of a mode in one language.
pub fn legend_text(labels: &BimLabels, german: bool, mode: Mode) -> String {
    match mode {
        Mode::UValue => labels
            .env_legend_u
            .as_str()
            .replace("{low}", &measure(envelope::U_MIN, german))
            .replace("{mid}", &measure((envelope::U_MIN + envelope::U_MAX) / 2.0, german))
            .replace("{high}", &measure(envelope::U_MAX, german)),
        Mode::Boundary => labels.env_legend_boundary.as_str().to_string(),
    }
}

fn both(text: impl Fn(&BimLabels, bool) -> String) -> World3dText {
    World3dText::new(text(&BimLabels::NATIVE_EN, false), text(&BimLabels::NATIVE_DE, true))
}

/// 🏷️ The legend of a mode in both languages, for the title of the annotation layer.
pub fn legend(mode: Mode) -> World3dText {
    both(|labels, german| legend_text(labels, german, mode))
}
//#endregion 🔖️Texts

//#region 🔖️Scene
/// 🧊️ What the overlay adds to the scene: meshes and instances (as the world window builds them), the markers with the legend, and the framework heatmap with its on-screen legend.
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    pub meshes: Vec<DslValue>,
    pub instances: Vec<DslValue>,
    pub annotations: Option<World3dAnnotationLayer>,
    pub scalar_field: Option<World3dScalarField>,
}

fn array(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|value| DslValue::float(*value)).collect())
}

fn instance(id: &str, placement: crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidPlacement, mode: Mode) -> DslValue {
    let (sin, cos) = (placement.rotation * 0.5).sin_cos();
    let label = both(|labels, german| legend_text(labels, german, mode));
    DslValue::object([
        ("id".to_string(), DslValue::String(id.to_string())),
        ("meshId".to_string(), DslValue::String(id.to_string())),
        ("position".to_string(), array(&[placement.x, placement.y, placement.z])),
        ("rotation".to_string(), array(&[0.0, 0.0, sin, cos])),
        ("scale".to_string(), array(&[1.0, 1.0, 1.0])),
        ("label".to_string(), DslValue::String(label.en)),
        ("selected".to_string(), DslValue::Bool(false)),
        ("hovered".to_string(), DslValue::Bool(false)),
    ])
}

fn marker(part: &Part<'_>) -> World3dAnnotation {
    let text = both(|labels, german| surface_text(labels, german, part.surface));
    World3dAnnotation::marker(part.surface.id.clone(), envelope::to_world(part.placement, envelope::centre(part.surface)), text)
}

/// 🌡️ The overlay of the window, `None` while the setting is off or no conditioned space on a shown storey has surfaces.
pub fn overlay(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimWorldWindowConfig) -> Option<Overlay> {
    if !config.energy_overlay {
        return None;
    }
    let mode = mode(config);
    let parts = envelope::parts(snapshot, inference, &|storey| shown(config, storey));
    if parts.is_empty() {
        return None;
    }
    let groups = envelope::by_placement(&parts);
    let mut overlay = Overlay::default();
    for (index, (placement, members)) in groups.iter().enumerate() {
        let id = mesh_id(index);
        let mesh = envelope::mesh_of(members, mode);
        if groups.len() == 1 && mode == Mode::UValue && !mesh.face_ids.is_empty() {
            let field = World3dScalarField::new(id.clone(), World3dScalarDomain::Face, envelope::u_values(members, &mesh), World3dColorRamp::Coolwarm, World3dScalarRange { min: envelope::U_MIN, max: envelope::U_MAX }, legend(mode)).with_unit("W/(m²·K)").with_ticks(envelope::TICKS as u8);
            overlay.scalar_field = Some(field);
        }
        overlay.meshes.push(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("data".to_string(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::from(mesh)))]));
        overlay.instances.push(instance(&id, *placement, mode));
    }
    let markers: Vec<World3dAnnotation> = parts.iter().take(WORLD3D_ANNOTATIONS_MAX).map(marker).collect();
    overlay.annotations = Some(World3dAnnotationLayer::new(markers).titled(legend(mode)));
    Some(overlay)
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
