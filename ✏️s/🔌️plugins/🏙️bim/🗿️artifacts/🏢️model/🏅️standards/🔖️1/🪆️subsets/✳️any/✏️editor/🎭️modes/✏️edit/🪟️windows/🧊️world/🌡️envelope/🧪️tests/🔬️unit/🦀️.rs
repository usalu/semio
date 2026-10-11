use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::EnvelopeSpace;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference;
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;

fn corner(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

fn surface(id: &str, kind: SurfaceKind, boundary: Boundary, u_value: Option<f64>, polygon: Vec<Vec3>) -> EnvelopeSurface {
    EnvelopeSurface { id: id.into(), kind, boundary, u_value, area: 12.0, gross_area: 12.0, polygon, ..EnvelopeSurface::default() }
}

fn model() -> (ModelSnapshot, ModelInference) {
    let snapshot = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    let mut inference = with_inference(None, &snapshot, Clone::clone);
    let wall = vec![corner(0.0, 0.0, 0.0), corner(4.0, 0.0, 0.0), corner(4.0, 0.0, 3.0), corner(0.0, 0.0, 3.0)];
    let floor = vec![corner(0.0, 0.0, 0.0), corner(0.0, 3.0, 0.0), corner(4.0, 3.0, 0.0), corner(4.0, 0.0, 0.0)];
    let space = EnvelopeSpace {
        space: "sp-office".into(),
        storey: "st-ground".into(),
        conditioned: true,
        surfaces: vec![surface("sp-office/w1", SurfaceKind::Wall, Boundary::Exterior, Some(0.3), wall), surface("sp-office/f1", SurfaceKind::Floor, Boundary::Ground, None, floor)],
        ..EnvelopeSpace::default()
    };
    inference.energy_envelopes.insert("sp-office".into(), space);
    (snapshot, inference)
}

fn on(mode: &str) -> BimWorldWindowConfig {
    BimWorldWindowConfig { energy_overlay: true, energy_mode: mode.into(), ..BimWorldWindowConfig::default() }
}

#[semio_framework_async_macros::async_test]
async fn the_overlay_is_off_by_default_and_without_conditioned_surfaces() {
    let (snapshot, inference) = model();
    assert!(overlay(&snapshot, &inference, &BimWorldWindowConfig::default()).is_none());
    let mut bare = inference.clone();
    bare.energy_envelopes.clear();
    assert!(overlay(&snapshot, &bare, &on("u_value")).is_none());
    let mut idle = inference.clone();
    idle.energy_envelopes.get_mut("sp-office").expect("space").conditioned = false;
    assert!(overlay(&snapshot, &idle, &on("u_value")).is_none(), "an unconditioned space has no overlay");
}

#[semio_framework_async_macros::async_test]
async fn by_u_value_the_overlay_is_one_mesh_painted_by_the_framework_heatmap_with_its_legend() {
    let (snapshot, inference) = model();
    let overlay = overlay(&snapshot, &inference, &on("u_value")).expect("an overlay");
    assert_eq!((overlay.meshes.len(), overlay.instances.len()), (1, 1));
    let field = overlay.scalar_field.expect("the heatmap");
    assert_eq!((field.mesh_id.as_str(), field.domain, field.ramp), ("bim-energy-envelope/0", World3dScalarDomain::Face, World3dColorRamp::Coolwarm));
    assert_eq!((field.range.min, field.range.max, field.legend.ticks, field.legend.unit.as_deref()), (0.0, 2.0, 5, Some("W/(m²·K)")));
    assert_eq!(field.values, [Some(0.3), Some(0.3), None, None], "two triangles for the wall, two for the floor without a U-value");
    assert!(field.legend.title.en.contains("U-value") && field.legend.title.de.contains("U-Wert"));
    assert_eq!(field.color_of(None), envelope::NEUTRAL, "the heatmap and the vertex colours agree on the neutral grey");
}

#[semio_framework_async_macros::async_test]
async fn by_boundary_the_vertex_colours_decide_and_there_is_no_heatmap() {
    let (snapshot, inference) = model();
    let overlay = overlay(&snapshot, &inference, &on("boundary")).expect("an overlay");
    assert!(overlay.scalar_field.is_none());
    let layer = overlay.annotations.expect("markers");
    assert!(layer.title.as_ref().is_some_and(|title| title.en.contains("boundary") && title.de.contains("Randbedingung")));
    let (snapshot, inference) = model();
    let parts = envelope::parts(&snapshot, &inference, &|_| true);
    let mesh = envelope::mesh_of(&parts, Mode::Boundary);
    let [r, g, b] = envelope::boundary_rgb(Boundary::Exterior);
    assert_eq!(mesh.colors[..3], [f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0]);
}

#[semio_framework_async_macros::async_test]
async fn every_surface_has_a_marker_that_names_id_kind_boundary_area_and_u_value_in_both_languages() {
    let (snapshot, inference) = model();
    let layer = overlay(&snapshot, &inference, &on("u_value")).and_then(|overlay| overlay.annotations).expect("markers");
    assert_eq!(layer.items.len(), 2);
    let texts: Vec<(&str, &World3dText)> = layer.items.iter().filter_map(|item| if let World3dAnnotation::Marker(marker) = item { Some((marker.id.as_str(), &marker.text)) } else { None }).collect();
    assert_eq!(texts.len(), 2);
    let (wall, floor) = (texts[0].1, texts[1].1);
    assert_eq!(wall.en, "sp-office/w1: Wall, Exterior, 12.00 m², U 0.30 W/(m²·K)");
    assert_eq!(wall.de, "sp-office/w1: Wand, Außen, 12,00 m², U 0,30 W/(m²·K)");
    assert_eq!(floor.en, "sp-office/f1: Floor, Ground, 12.00 m², no U-value");
    assert_eq!(floor.de, "sp-office/f1: Boden, Erdreich, 12,00 m², kein U-Wert");
    assert!(layer.validate().is_ok());
}

#[semio_framework_async_macros::async_test]
async fn the_marker_sits_at_the_world_position_of_the_centre_of_the_polygon() {
    let (snapshot, inference) = model();
    let layer = overlay(&snapshot, &inference, &on("u_value")).and_then(|overlay| overlay.annotations).expect("markers");
    let World3dAnnotation::Marker(first) = &layer.items[0] else { panic!("a marker") };
    let parts = envelope::parts(&snapshot, &inference, &|_| true);
    assert_eq!(first.position, envelope::to_world(parts[0].placement, [2.0, 0.0, 1.5]));
}

#[semio_framework_async_macros::async_test]
async fn a_hidden_or_not_isolated_storey_drops_its_surfaces_and_the_instances_are_not_pickable() {
    let (snapshot, inference) = model();
    let hidden = BimWorldWindowConfig { hidden_storeys: vec!["st-ground".into()], ..on("u_value") };
    assert!(overlay(&snapshot, &inference, &hidden).is_none());
    let isolated = BimWorldWindowConfig { isolated_storey: "st-first".into(), ..on("u_value") };
    assert!(overlay(&snapshot, &inference, &isolated).is_none());
    let here = BimWorldWindowConfig { isolated_storey: "st-ground".into(), ..on("u_value") };
    let overlay = overlay(&snapshot, &inference, &here).expect("the isolated storey itself stays");
    let text = semio_framework_pack_json::to_json_string(&overlay.instances);
    assert!(text.contains("bim-energy-envelope/0") && !text.contains("interactionId"), "{text}");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_mode_key_falls_back_to_the_u_value_and_no_text_keeps_a_placeholder() {
    assert_eq!(mode(&on("hatch")), Mode::UValue);
    assert_eq!((mode(&on("boundary")), mode(&BimWorldWindowConfig::default())), (Mode::Boundary, Mode::UValue));
    for mode in Mode::ALL {
        let text = legend(mode);
        assert!(!text.en.contains('{') && !text.de.contains('{'), "{text:?}");
    }
    assert_eq!(legend(Mode::UValue).de, "Hülle nach U-Wert in W/(m²·K): 0,00 blau, 1,00 grau, 2,00 und mehr rot; grau: kein U-Wert");
}

#[semio_framework_async_macros::async_test]
async fn the_world_scene_gains_the_overlay_only_while_the_setting_is_on_and_still_renders() {
    use crate::editor::bim::modes::edit::windows::world;
    let (snapshot, inference) = model();
    let off = world::scene(&snapshot, &inference, &BimWorldWindowConfig::default(), &[], &[], 1);
    assert!(!off.meshes_json.contains("bim-energy-envelope") && off.annotations.is_none() && off.scalar_field.is_none());
    let scene = world::scene(&snapshot, &inference, &on("u_value"), &[], &[], 1);
    assert!(scene.meshes_json.contains("bim-energy-envelope/0") && scene.instances_json.contains("bim-energy-envelope/0"));
    assert!(scene.instances_json.contains("w-south"), "the model itself is still drawn");
    assert!(scene.annotations.is_some() && scene.scalar_field.is_some());
    let boundary = world::scene(&snapshot, &inference, &on("boundary"), &[], &[], 1);
    assert!(boundary.annotations.is_some() && boundary.scalar_field.is_none());
    for config in [on("u_value"), on("boundary")] {
        assert!(world::render(&snapshot, &inference, &config, &[], &[], 1).is_ok(), "the scene with the overlay is a valid surface");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_hover_names_the_construction_that_gives_the_surface_its_layers() {
    let wall = EnvelopeSurface { construction: "wt-300".into(), adjacent: "sp-hall".into(), ..surface("sp-office/w2", SurfaceKind::Wall, Boundary::Adjacent, Some(0.5), Vec::new()) };
    assert_eq!(surface_text(&BimLabels::NATIVE_EN, false, &wall), "sp-office/w2: Wall wt-300, Adjacent space sp-hall, 12.00 m², U 0.50 W/(m²·K)");
    assert_eq!(surface_text(&BimLabels::NATIVE_DE, true, &wall), "sp-office/w2: Wand wt-300, Angrenzender Raum sp-hall, 12,00 m², U 0,50 W/(m²·K)");
}
