use super::*;
use crate::editor::bim::unit_tests::context::view as window_view;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, german, run};
use semio_framework_ui_locale::Locale;

fn create(kind: &str, parent: &str, name: &str, selected: &[&str], german_view: bool) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    ctx.view = german_view.then(german);
    run(&snapshot, |doc, cfg| handle(&CreateView { kind: kind.into(), parent: parent.into(), name: name.into() }, doc, cfg, &mut ctx))
}

fn created(emit: &Emit<ModelMutation, NoConfigMutation>) -> Vec<View> {
    emit.artifact_mutations
        .iter()
        .map(|mutation| match mutation {
            ModelMutation::CreateView(payload) => payload.view.clone(),
            other => panic!("only views are created, got {other:?}"),
        })
        .collect()
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_extents_of_the_demo_are_its_walls_grown_by_half_their_thickness() {
    let [x0, y0, x1, y1] = extents(&demo(), "bldg-1").expect("walls give extents");
    assert!(close(x0, -0.15) && close(y0, -0.15) && close(x1, 8.15) && close(y1, 6.15), "{x0} {y0} {x1} {y1}");
    assert_eq!(extents(&crate::ModelSnapshot::default(), "bldg-1"), None);
}

#[semio_framework_async_macros::async_test]
async fn a_plan_is_made_for_the_storey_of_the_parent_or_the_selection_or_the_lowest_one() {
    let explicit = created(&create("plan", "st-first", "", &["st-ground"], false).expect("creates"));
    assert_eq!((explicit[0].kind, explicit[0].storey.as_deref()), (ViewKind::Plan, Some("st-first")));
    let selected = created(&create("plan", "", "", &["w-south"], false).expect("creates"));
    assert_eq!(selected[0].storey.as_deref(), Some("st-ground"), "a wall selects its storey");
    let ceiling = created(&create("ceiling-plan", "", "Reflected", &[], false).expect("creates"));
    assert_eq!((ceiling[0].kind, ceiling[0].name.as_str(), ceiling[0].storey.as_deref()), (ViewKind::CeilingPlan, "Reflected", Some("st-ground")));
}

#[semio_framework_async_macros::async_test]
async fn the_four_elevations_are_the_committed_ones_and_look_at_the_four_sides() {
    let emit = create("elevations", "", "", &[], false).expect("creates");
    let views = created(&emit);
    assert_eq!(views.len(), 4);
    assert!(views.iter().all(|view| view.kind == ViewKind::Elevation));
    let snapshot = demo();
    for (side, view) in ["south", "east", "north", "west"].iter().zip(&views) {
        let committed = &snapshot.views[&format!("v-elevation-{side}")];
        assert_eq!((view.plane, view.depth), (committed.plane, committed.depth), "the {side} elevation of the demo is what the command makes");
    }
    let south = views[0].plane.expect("a plane");
    let frame = crate::standards::v1::subsets::any::schema::inferences::view_linework::frame::Frame::of(&south).expect("a frame");
    assert!(close(frame.look[0], 0.0) && close(frame.look[1], 1.0), "the south elevation looks north");
    let after = applied(&snapshot, &emit);
    assert_eq!(after.views.len(), snapshot.views.len() + 4, "the names clash with the demo's elevations and are made unique");
    assert!(after.views.values().any(|view| view.name == "South (2)"));
}

#[semio_framework_async_macros::async_test]
async fn a_section_runs_through_the_middle_and_a_camera_orbits_the_building() {
    let section = created(&create("section", "", "Long", &[], false).expect("creates"));
    let committed = &demo().views["v-section-a"];
    assert_eq!((section[0].plane, section[0].depth, section[0].name.as_str()), (committed.plane, committed.depth, "Long"));
    let camera = created(&create("perspective", "", "", &[], false).expect("creates"));
    let orbit = camera[0].camera.expect("a camera");
    assert!(close(orbit.target.x, 4.0) && close(orbit.target.y, 3.0) && orbit.distance > 8.0);
    assert_eq!(camera[0].kind, ViewKind::Perspective);
}

#[semio_framework_async_macros::async_test]
async fn the_names_speak_the_addressed_locale() {
    let german_names: Vec<String> = created(&create("elevations", "", "", &[], true).expect("creates")).into_iter().map(|view| view.name).collect();
    assert_eq!(german_names, vec!["Süd", "Ost", "Nord", "West (2)"]);
    assert_eq!(created(&create("plan", "st-ground", "", &[], true).expect("creates"))[0].name, "Grundriss Ground");
}

#[semio_framework_async_macros::async_test]
async fn the_new_view_is_selected_and_bound_to_the_addressed_window() {
    let snapshot = demo();
    let addressed = window_view(Locale::En, &[("window", crate::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID)], Some("window"));
    let mut ctx = crate::editor::bim::BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&addressed), None, None);
    let emit = run(&snapshot, |doc, cfg| handle(&CreateView { kind: "section".into(), parent: String::new(), name: "Cross".into() }, doc, cfg, &mut ctx)).expect("creates");
    assert_eq!(emit.window_config_mutations.len(), 1, "a section window now shows the new section");
    assert_eq!(emit.effects.len(), 1, "the new view is selected");
    let mut plan_ctx = crate::editor::bim::BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&addressed), None, None);
    let plan = run(&snapshot, |doc, cfg| handle(&CreateView { kind: "plan".into(), parent: String::new(), name: "Another".into() }, doc, cfg, &mut plan_ctx)).expect("creates");
    assert!(plan.window_config_mutations.is_empty(), "a plan does not rebind a section window");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_kind_and_a_missing_building_are_refused_with_a_stable_code() {
    assert_eq!(create("teapot", "", "", &[], false).err().map(|fault| fault.code.0), Some("bim.view.kind-unknown".to_string()));
    let empty = crate::ModelSnapshot::default();
    let mut ctx = ctx(&[]);
    let refused = run(&empty, |doc, cfg| handle(&CreateView { kind: "section".into(), parent: String::new(), name: String::new() }, doc, cfg, &mut ctx));
    assert_eq!(refused.err().map(|fault| fault.code.0), Some("bim.view.building-missing".to_string()));
}
