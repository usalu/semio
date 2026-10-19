use semio_s_artifact_bim_model_storey_levels::editor::bim::marks::{Mark, Preview, Style};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane::P;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_storey_levels::editor::bim::marks::Shape;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::snap::{snap, SnapHit, SnapKind, SnapRequest};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_entities::editor::bim::entities::id_taken;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::IdMint;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_ui_locale::LabelText;
#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::{Modifiers, ToolContext, Surface, Step, REJECTED, length_label};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::{model, room};

#[semio_framework_async_macros::async_test]
async fn the_held_modifiers_map_to_the_framework_merge_modes() {
    let merge = |shift, ctrl, meta| Modifiers { shift, ctrl, meta }.merge();
    assert_eq!([merge(false, false, false), merge(true, false, false), merge(false, true, false), merge(false, false, true), merge(true, true, false)], ["replace", "additive", "subtractive", "subtractive", "invertive"]);
}

#[semio_framework_async_macros::async_test]
async fn a_context_reads_the_storey_the_building_and_the_selected_library_type() {
    let snapshot = model();
    let inference = ModelInference::default();
    let mut context = ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-first".into() }, "seed");
    assert_eq!((context.storey(), context.building()), (Some("st-first"), Some("bldg-1")));
    assert_eq!(ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "gone".into() }, "seed").storey(), None, "a storey that no longer exists is no storey");
    assert_eq!(ToolContext::new(&snapshot, &inference, Surface::Section { start: [0.0, 0.0], end: [1.0, 0.0] }, "seed").storey(), None);
    assert_eq!(context.library_type(&snapshot.wall_types).as_deref(), Some("wt-300"));
    let library = vec!["win-12".to_string(), "wt-300".to_string()];
    context.library = &library;
    assert_eq!(context.library_type(&snapshot.wall_types).as_deref(), Some("wt-300"));
    assert_eq!(context.library_type(&snapshot.door_types).as_deref(), Some("door-09"), "no selected door type: the first one");
}

#[semio_framework_async_macros::async_test]
async fn minted_ids_carry_the_authoring_seed_and_never_collide_with_the_model_or_each_other() {
    let snapshot = room();
    let inference = ModelInference::default();
    let mut context = ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-ground".into() }, "ab-12");
    let first = context.mint("wall");
    let second = context.mint("wall");
    assert!(first.starts_with("wall-ab12-") && first != second);
    assert!(!snapshot.walls.contains_key(&first));
}

#[semio_framework_async_macros::async_test]
async fn the_model_is_asked_before_a_tool_proposes_a_mutation() {
    let snapshot = model();
    let inference = ModelInference::default();
    let context = ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-ground".into() }, "seed");
    let storey = |id: &str| ModelMutation::SetStoreyHeight(semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight { id: id.into(), height: 3.5 });
    assert!(context.accepts(&storey("st-ground")));
    assert!(!context.accepts(&storey("nowhere")));
    assert_eq!(Step::write(&context, storey("nowhere")).refused, Some(REJECTED));
    assert_eq!(Step::write(&context, storey("st-ground")).mutations.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn a_preview_is_text_and_back_and_a_mark_reads_its_corners() {
    let mark = Mark::path(&[[0.0, 1.0], [2.0, 3.0]], true, Style::Ghost);
    assert_eq!(mark.corners(), vec![[0.0, 1.0], [2.0, 3.0]]);
    let preview = Preview::of(vec![mark, Mark::label([1.0, 1.0], "1.00 m")]);
    assert_eq!(Preview::from_text(&preview.to_text()), preview);
    assert!(Preview::default().is_empty() && !preview.is_empty());
    assert_eq!(length_label([0.0, 0.0], [3.0, 4.0]).text, "5.00 m");
}
