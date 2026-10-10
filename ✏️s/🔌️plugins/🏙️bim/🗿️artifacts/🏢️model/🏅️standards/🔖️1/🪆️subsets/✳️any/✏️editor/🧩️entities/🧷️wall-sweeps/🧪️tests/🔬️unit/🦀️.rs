use super::*;
use crate::editor::bim::entities::{kind_holding, kind_of, storey_of};
use crate::editor::bim::terminology::BimLabels;

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

fn applied(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> ModelSnapshot {
    crate::mutations::apply_model_mutation(snapshot, mutation).unwrap_or_else(|refusal| panic!("{mutation:?} is refused: {refusal:?}"))
}

fn furnished() -> ModelSnapshot {
    let mut snapshot = demo();
    for (kind, id, parent) in [("slab-type", "sl-1", ""), ("slab", "slab-1", "st-ground"), ("roof-type", "rf-1", ""), ("roof", "roof-1", "st-ground"), ("ceiling-type", "ct-1", ""), ("ceiling", "ceil-1", "st-ground"), ("opening", "op-1", "w-south")] {
        let create = kind_of(kind).and_then(|row| row.create).expect("a creatable kind");
        let mutation = create(&snapshot, id, parent, "Sample").expect("creates");
        snapshot = applied(&snapshot, &mutation);
    }
    snapshot
}

fn field(kind: &str, key: &str) -> &'static FieldRow {
    kind_of(kind).and_then(|row| row.fields.iter().find(|field| field.key == key)).unwrap_or_else(|| panic!("{kind} has no row {key}"))
}

fn write(snapshot: &ModelSnapshot, kind: &str, key: &str, id: &str, value: &str) -> Option<ModelSnapshot> {
    field(kind, key).write.expect("an editable row")(snapshot, id, value).map(|mutation| applied(snapshot, &mutation))
}

fn read(snapshot: &ModelSnapshot, kind: &str, key: &str, id: &str) -> String {
    (field(kind, key).read)(snapshot, id).unwrap_or_else(|| panic!("{kind} {id} has no {key}"))
}

fn with_sweep(snapshot: &ModelSnapshot) -> ModelSnapshot {
    let create = kind_of("wall-sweep").and_then(|row| row.create).expect("wall sweep create");
    applied(snapshot, &create(snapshot, "sw-1", "w-south", "Baseboard").expect("creates"))
}

#[semio_framework_async_macros::async_test]
async fn a_new_sweep_is_a_baseboard_on_the_left_face_of_the_chosen_wall_else_the_first_wall() {
    let snapshot = demo();
    let Ok(ModelMutation::CreateWallSweep(created)) = create_wall_sweep(&snapshot, "sw-1", "w-east", "Baseboard") else { panic!("create wall sweep") };
    assert_eq!((created.wall_sweep.host.as_str(), created.wall_sweep.side, created.wall_sweep.height, created.wall_sweep.inset), ("w-east", WallSide::Left, 0.0, 0.0));
    assert_eq!(created.wall_sweep.profile, baseboard());
    let Ok(ModelMutation::CreateWallSweep(fallback)) = create_wall_sweep(&snapshot, "sw-2", "st-ground", "Baseboard") else { panic!("create wall sweep") };
    assert_eq!(Some(fallback.wall_sweep.host), snapshot.walls.keys().next().cloned());
    let mut bare = snapshot.clone();
    bare.walls.clear();
    assert_eq!(create_wall_sweep(&bare, "sw-3", "", "Baseboard").err(), Some("bim.create.wall-missing"));
    let mut unpainted = snapshot;
    unpainted.materials.clear();
    assert_eq!(create_wall_sweep(&unpainted, "sw-3", "w-south", "Baseboard").err(), Some("bim.create.material-missing"));
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_sits_under_its_wall_and_on_the_storey_of_its_wall() {
    let snapshot = with_sweep(&demo());
    let row = kind_holding(&snapshot, "sw-1").expect("a row holds the sweep");
    assert_eq!(row.kind, "wall-sweep");
    assert_eq!((row.parent)(&snapshot, "sw-1").as_deref(), Some("w-south"));
    assert_eq!(storey_of(&snapshot, "sw-1").as_deref(), Some("st-ground"));
}

#[semio_framework_async_macros::async_test]
async fn the_sweep_rows_write_sparse_set_wall_sweep_mutations_and_read_back_what_they_wrote() {
    let snapshot = with_sweep(&demo());
    for (key, value, expected) in [("side", "Right", "Right"), ("height", "0.15", "0.15"), ("inset", "0.005", "0.005"), ("host", "w-east", "w-east"), ("name", "Skirting", "Skirting")] {
        let changed = write(&snapshot, "wall-sweep", key, "sw-1", value).unwrap_or_else(|| panic!("{key} is editable"));
        assert_eq!(read(&changed, "wall-sweep", key, "sw-1"), expected, "{key}");
    }
    assert!(matches!(field("wall-sweep", "side").write.expect("side")(&snapshot, "sw-1", "Right"), Some(ModelMutation::SetWallSweep(set)) if set.side == Some(WallSide::Right) && set.host.is_none() && set.profile.is_none()));
    for (key, value) in [("side", "Up"), ("height", "high"), ("inset", ""), ("profile", "blob 1")] {
        assert!(write(&snapshot, "wall-sweep", key, "sw-1", value).is_none(), "{key} refuses '{value}'");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_profile_row_speaks_rect_circle_and_custom() {
    let snapshot = with_sweep(&demo());
    assert_eq!(read(&snapshot, "wall-sweep", "profile", "sw-1"), "rect 0.02 x 0.1");
    let round = write(&snapshot, "wall-sweep", "profile", "sw-1", "circle 0.04").expect("circle");
    assert_eq!(round.wall_sweeps["sw-1"].profile, Profile::Circle { diameter: 0.04 });
    let custom = write(&snapshot, "wall-sweep", "profile", "sw-1", "custom 0, 0; 0.03, 0; 0.03, 0.08; 0, 0.05").expect("custom");
    assert!(matches!(&custom.wall_sweeps["sw-1"].profile, Profile::Custom { outline } if outline.len() == 4));
    assert_eq!(read(&custom, "wall-sweep", "profile", "sw-1"), "custom 0, 0; 0.03, 0; 0.03, 0.08; 0, 0.05");
    let rectangle = write(&snapshot, "wall-sweep", "profile", "sw-1", "rect 0.03 x 0.12").expect("rect");
    assert_eq!(rectangle.wall_sweeps["sw-1"].profile, Profile::Rectangle { width: 0.03, depth: 0.12 });
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_row_reads_its_inferred_length_area_and_volume() {
    let snapshot = with_sweep(&demo());
    let inference = <crate::ModelInference as protocol::Inference<ModelSnapshot>>::infer(&snapshot).expect("infer");
    let row = kind_of("wall-sweep").expect("wall sweep");
    for key in ["length", "area", "volume"] {
        let shown = row.inferred.iter().find(|row| row.key == key).and_then(|row| (row.read)(&snapshot, &inference, "sw-1"));
        assert!(shown.is_some_and(|text| text.parse::<f64>().is_ok_and(|value| value > 0.0)), "{key} of the sweep shows");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_top_attach_row_follows_a_roof_a_slab_or_a_ceiling_keeps_the_offset_and_frees_the_top() {
    let snapshot = furnished();
    assert_eq!(read(&snapshot, "wall", "top_attach", "w-south"), "");
    let on_roof = write(&snapshot, "wall", "top_attach", "w-south", "roof-1").expect("attach to the roof");
    assert_eq!(on_roof.walls["w-south"].top, TopConstraint::Roof { roof: "roof-1".into(), offset: 0.0 });
    assert_eq!(read(&on_roof, "wall", "top_attach", "w-south"), "roof-1");
    let offset = write(&on_roof, "wall", "top", "w-south", "roof roof-1 0.25").expect("offset");
    let on_slab = write(&offset, "wall", "top_attach", "w-south", "slab-1").expect("attach to the slab");
    assert_eq!(on_slab.walls["w-south"].top, TopConstraint::Slab { slab: "slab-1".into(), offset: 0.25 }, "the offset stays");
    let on_ceiling = write(&on_slab, "wall", "top_attach", "w-south", "ceil-1").expect("attach to the ceiling");
    assert_eq!(on_ceiling.walls["w-south"].top, TopConstraint::Ceiling { ceiling: "ceil-1".into(), offset: 0.25 });
    let freed = write(&on_ceiling, "wall", "top_attach", "w-south", "").expect("free");
    assert_eq!(freed.walls["w-south"].top, TopConstraint::StoreyTop { offset: 0.0 });
    assert!(write(&snapshot, "wall", "top_attach", "w-south", "w-east").is_none(), "a wall is no surface");
}

#[semio_framework_async_macros::async_test]
async fn the_base_slab_row_follows_a_slab_and_frees_it() {
    let snapshot = furnished();
    assert_eq!(read(&snapshot, "wall", "base_slab", "w-south"), "");
    let followed = write(&snapshot, "wall", "base_slab", "w-south", "slab-1").expect("follow the slab");
    assert_eq!(followed.walls["w-south"].base_slab.as_deref(), Some("slab-1"));
    assert_eq!(read(&followed, "wall", "base_slab", "w-south"), "slab-1");
    let freed = write(&followed, "wall", "base_slab", "w-south", "").expect("free");
    assert_eq!(freed.walls["w-south"].base_slab, None);
    assert!(write(&snapshot, "wall", "base_slab", "w-south", "roof-1").is_none(), "a roof is no slab");
}

#[semio_framework_async_macros::async_test]
async fn the_attach_pickers_offer_the_surfaces_in_both_languages() {
    let snapshot = furnished();
    for (labels, roof_label, slab_label) in [(&BimLabels::NATIVE_EN, "Roof: Sample", "Slab: Sample"), (&BimLabels::NATIVE_DE, "Dach: Sample", "Decke: Sample")] {
        let choices = surface_choices(&snapshot, labels);
        assert_eq!(choices.first(), Some(&("roof-1".to_string(), roof_label.to_string())));
        assert!(choices.contains(&("slab-1".to_string(), slab_label.to_string())));
        assert!(choices.iter().any(|(id, _)| id == "ceil-1"));
        assert_eq!(slab_choices(&snapshot, labels), vec![("slab-1".to_string(), "Sample".to_string())]);
    }
}

#[semio_framework_async_macros::async_test]
async fn the_reveal_rows_set_the_depth_and_material_and_clear_them_with_empty_text() {
    let snapshot = furnished();
    let opening = "op-1".to_string();
    assert_eq!((read(&snapshot, "opening", "reveal_depth", &opening), read(&snapshot, "opening", "reveal_material", &opening)), (String::new(), String::new()));
    let deep = write(&snapshot, "opening", "reveal_depth", &opening, "0.08").expect("depth");
    assert_eq!(deep.openings[&opening].reveal_depth, Some(0.08));
    assert_eq!(read(&deep, "opening", "reveal_depth", &opening), "0.08");
    let material = snapshot.materials.keys().next().cloned().expect("a material");
    let painted = write(&deep, "opening", "reveal_material", &opening, &material).expect("material");
    assert_eq!(painted.openings[&opening].reveal_material.as_deref(), Some(material.as_str()));
    let cleared = write(&write(&painted, "opening", "reveal_depth", &opening, "").expect("clear depth"), "opening", "reveal_material", &opening, "").expect("clear material");
    assert_eq!((cleared.openings[&opening].reveal_depth, cleared.openings[&opening].reveal_material.clone()), (None, None));
    assert!(write(&snapshot, "opening", "reveal_depth", &opening, "deep").is_none());
}
