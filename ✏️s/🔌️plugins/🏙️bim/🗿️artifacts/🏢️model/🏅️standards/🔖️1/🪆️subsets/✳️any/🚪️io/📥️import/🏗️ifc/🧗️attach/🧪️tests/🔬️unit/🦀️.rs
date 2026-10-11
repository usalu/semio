use crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{attic, house};
use crate::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::{Axis, Ceiling, CeilingType, Layer, LayerFunction, LocationLine, ModelSnapshot, Opening, OpeningKind, Phase, Point2, Profile, Slab, Slope, TopConstraint, Vertex, Wall, WallSide, WallSweep};

fn vertex(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

fn rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Vertex> {
    vec![vertex(x0, y0), vertex(x1, y0), vertex(x1, y1), vertex(x0, y1)]
}

/// 🧗️ The part of the house the import understands plus three walls on the ground storey: one under a level ceiling with a baseboard and a window with a reveal, one under a sloped ceiling (a brep) with a hand rail, one standing on a level slab.
pub fn with_depth() -> ModelSnapshot {
    let mut model = house();
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    let ground = model.storeys.iter().find(|(_, storey)| storey.level == 0).map(|(id, _)| id.clone()).expect("the ground storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    let (wall_type, slab_type, window_type) = (model.wall_types.keys().next().cloned().expect("a wall type"), model.slab_types.keys().next().cloned().expect("a slab type"), model.window_types.keys().next().cloned().expect("a window type"));
    model.ceiling_types.insert("ct-board".into(), CeilingType { name: "Board".into(), layers: vec![Layer { material: material.clone(), thickness: 0.0125, function: LayerFunction::Finish }] });
    let ceiling = |boundary: Vec<Vertex>, slope: Option<Slope>, name: &str| Ceiling { storey: ground.clone(), ceiling_type: "ct-board".into(), boundary, holes: Vec::new(), offset: 0.2, slope, name: name.into() };
    model.ceilings.insert("ce-flat".into(), ceiling(rectangle(100.0, 0.0, 108.0, 6.0), None, "Flat"));
    model.ceilings.insert("ce-raked".into(), ceiling(rectangle(100.0, 10.0, 108.0, 16.0), Some(Slope { direction: 0.0, angle: 0.1 }), "Raked"));
    model.slabs.insert("sl-foot".into(), Slab { storey: ground.clone(), slab_type, boundary: rectangle(100.0, 20.0, 108.0, 26.0), holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: "Foot".into() });
    let wall = |y: f64, top: TopConstraint, base_slab: Option<&str>| Wall {
        storey: ground.clone(),
        wall_type: wall_type.clone(),
        axis: Axis::Line { start: Point2 { x: 100.0, y }, end: Point2 { x: 108.0, y } },
        location: LocationLine::Center,
        base_offset: if base_slab.is_some() { 0.05 } else { 0.0 },
        top,
        phase: Phase::New,
        start_join: None,
        end_join: None,
        name: String::new(),
        base_slab: base_slab.map(str::to_string),
    };
    model.walls.insert("w-ceil".into(), wall(3.0, TopConstraint::Ceiling { ceiling: "ce-flat".into(), offset: 0.05 }, None));
    model.walls.insert("w-raked".into(), wall(13.0, TopConstraint::Ceiling { ceiling: "ce-raked".into(), offset: 0.0 }, None));
    model.walls.insert("w-foot".into(), wall(23.0, TopConstraint::StoreyTop { offset: 0.0 }, Some("sl-foot")));
    model.wall_sweeps.insert("ws-board".into(), WallSweep { host: "w-ceil".into(), side: WallSide::Left, profile: Profile::Rectangle { width: 0.02, depth: 0.1 }, height: 0.0, inset: 0.0, material: material.clone(), name: "Baseboard".into() });
    model.wall_sweeps.insert("ws-rail".into(), WallSweep { host: "w-raked".into(), side: WallSide::Right, profile: Profile::Rectangle { width: 0.05, depth: 0.04 }, height: 0.9, inset: 0.01, material: material.clone(), name: String::new() });
    model.openings.insert(
        "o-rev".into(),
        Opening { host: "w-ceil".into(), kind: OpeningKind::Window { window_type }, offset: 4.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: String::new(), reveal_depth: Some(0.1), reveal_material: Some(material) },
    );
    model
}

#[test]
fn walls_under_ceilings_and_on_slabs_get_their_attach_back_exactly() {
    let model = with_depth();
    let (bytes, written) = export_ifc2x3(&model).expect("the model exports");
    assert!(written.is_empty(), "{written:?}");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("target") && !note.contains("base slab")), "{notes:?}");
    for id in ["w-ceil", "w-raked", "w-foot"] {
        assert_eq!(back.walls[id], model.walls[id], "{id}");
    }
}

#[test]
fn export_import_export_of_attached_walls_sweeps_and_reveals_is_byte_stable() {
    let (first, _) = export_ifc2x3(&with_depth()).expect("first export");
    let (back, _) = import_ifc2x3(&first).expect("the import");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(first, second);
}

#[test]
fn a_wall_under_a_roof_follows_the_imported_roof_and_a_base_slab_that_is_not_imported_leaves_the_base_free() {
    let model = attic();
    let (bytes, _) = export_ifc2x3(&model).expect("the attic exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(!back.roofs.is_empty(), "the roofs travel in the file");
    for id in ["w-hip", "w-east", "w-west", "w-south", "w-north"] {
        assert_eq!(back.walls[id].top, model.walls[id].top, "{id}");
    }
    assert!(notes.iter().all(|note| !note.contains("Roof target")), "{notes:?}");
    let layout = inference::try_with_inference(None, &model, |inferred| inferred.wall_layout["w-hip"].clone()).expect("the attic infers");
    assert!(layout.height > 0.0);
    if back.slabs.contains_key("sl-slope") {
        assert_eq!(back.walls["w-base"].base_slab.as_deref(), Some("sl-slope"));
    } else {
        assert!(notes.iter().any(|note| note.contains("base slab sl-slope")), "{notes:?}");
        assert_eq!((back.walls["w-base"].base_slab.clone(), back.walls["w-base"].base_offset), (None, 0.0));
    }
}
