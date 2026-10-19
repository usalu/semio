use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::{EnvelopeSurface, SurfaceKind};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::holders::{holder_of, Holder, layers_of};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{LayerFunction, Phase, Point2, RoofShape, Vertex};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Roof, RoofType, Slab, SlabType};

fn corner(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

fn layer(thickness: f64) -> Layer {
    Layer { material: "m".into(), thickness, function: LayerFunction::Structure }
}

fn model() -> ModelSnapshot {
    let mut model = ModelSnapshot::default();
    model.slab_types.insert("st".into(), SlabType { name: "Slab".into(), layers: vec![layer(0.1), layer(0.2)] });
    model.slabs.insert("s-1".into(), Slab { storey: "g".into(), slab_type: "st".into(), boundary: vec![corner(0.0, 0.0), corner(5.0, 0.0), corner(5.0, 5.0)], holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: String::new() });
    model.roof_types.insert("rt".into(), RoofType { name: "Roof".into(), layers: vec![layer(0.2)] });
    model.roofs.insert("r-1".into(), Roof { storey: "g".into(), roof_type: "rt".into(), footprint: vec![corner(0.0, 0.0), corner(5.0, 0.0), corner(5.0, 5.0)], shape: RoofShape::Flat, overhang: 0.0, base_offset: 0.0, phase: Phase::New, name: String::new() });
    model
}

fn surface(kind: SurfaceKind, holder: &str, construction: &str) -> EnvelopeSurface {
    EnvelopeSurface { id: "x".into(), kind, holder: holder.into(), construction: construction.into(), ..EnvelopeSurface::default() }
}

#[test]
fn a_floor_is_held_by_the_slab_the_inference_names_and_a_ceiling_by_the_slab_or_the_roof() {
    let model = model();
    assert_eq!(holder_of(&model, &surface(SurfaceKind::Floor, "s-1", "st")), Some(Holder::Slab("s-1")));
    assert_eq!(holder_of(&model, &surface(SurfaceKind::Ceiling, "r-1", "rt")), Some(Holder::Roof("r-1")));
    assert_eq!(holder_of(&model, &surface(SurfaceKind::Ceiling, "s-1", "st")), Some(Holder::Slab("s-1")));
}

#[test]
fn a_holder_that_is_not_in_the_model_is_none() {
    let model = model();
    assert_eq!(holder_of(&model, &surface(SurfaceKind::Wall, "w-1", "wt")), None);
    assert_eq!(holder_of(&model, &surface(SurfaceKind::Floor, "nope", "st")), None);
    assert_eq!(holder_of(&model, &surface(SurfaceKind::CurtainWall, "cw-1", "cwt")), None);
}

#[test]
fn the_layers_come_from_the_type_the_inference_names() {
    let model = model();
    let floor = surface(SurfaceKind::Floor, "s-1", "st");
    assert_eq!(layers_of(&model, &floor, Holder::Slab("s-1")).map(|(id, layers)| (id, layers.len())), Some(("st", 2)));
    let roof = surface(SurfaceKind::Ceiling, "r-1", "rt");
    assert_eq!(layers_of(&model, &roof, Holder::Roof("r-1")).map(|(id, layers)| (id, layers.len())), Some(("rt", 1)));
    assert_eq!(layers_of(&model, &floor, Holder::Window("o")), None, "a window has no layer stack");
    assert_eq!(layers_of(&model, &surface(SurfaceKind::Floor, "s-1", "missing"), Holder::Slab("s-1")), None);
}

#[test]
fn the_id_of_a_holder_is_the_id_it_was_made_from() {
    assert_eq!(Holder::CurtainWall("cw").id(), "cw");
    assert_eq!(Holder::Ceiling("c").id(), "c");
}
