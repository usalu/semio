use crate::editor::bim::commands::{arm_utility, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, world_pointer_down};
use crate::editor::bim::modes::edit::windows::{plan, world};
use crate::editor::bim::unit_tests::context::{bim_app, canvas_scene, dispatch_in, history_verb, view};
use crate::editor::bim::BimCommand;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::{Effect, PluginApp, ViewModel};
use semio_framework_ui_locale::Locale;

const WINDOW: &str = "bim-plan";

fn on_a_big_stack<F: std::future::Future<Output = ()>>(test: impl FnOnce() -> F + Send + 'static) {
    std::thread::Builder::new()
        .name("bim-gesture-app".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let mut future = std::pin::pin!(test());
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            while future.as_mut().poll(&mut context).is_pending() {
                std::thread::yield_now();
            }
        })
        .expect("spawn")
        .join()
        .expect("the mounted gesture law holds");
}

fn armed(utility: &str) -> ViewModel {
    ViewModel { active_utility_id: Some(utility.into()), ..view(Locale::En, &[(WINDOW, plan::WINDOW_KIND_ID)], Some(WINDOW)) }
}

fn at(x: f64, y: f64) -> (f64, f64) {
    (400.0 + 40.0 * x, 300.0 - 40.0 * y)
}

fn press(x: f64, y: f64) -> BimCommand {
    let (x, y) = at(x, y);
    BimCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x, y, width: 800.0, height: 600.0, ..Default::default() })
}

fn hover(x: f64, y: f64) -> BimCommand {
    let (x, y) = at(x, y);
    BimCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x, y, width: 800.0, height: 600.0, ..Default::default() })
}

fn release(x: f64, y: f64) -> BimCommand {
    let (x, y) = at(x, y);
    BimCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x, y, width: 800.0, height: 600.0, ..Default::default() })
}

#[test]
fn two_clicks_of_the_wall_tool_write_one_wall_in_one_undoable_row_through_the_mounted_app() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        let wall = armed("wall");
        let first = dispatch_in(&mut app, press(10.0, 10.0), &wall).await;
        assert!(!first.edited_document(), "the first click only sets the start");
        dispatch_in(&mut app, release(10.0, 10.0), &wall).await;
        let second = dispatch_in(&mut app, press(14.0, 10.0), &wall).await;
        assert!(second.edited_document(), "the second click writes the wall");
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.walls.len(), 5);
        let created = snapshot.walls.values().find(|row| row.name.starts_with("Wall ") && row.storey == "st-ground" && crate::editor::bim::gestures::plane::axis_ends(&row.axis).0 == [10.0, 10.0]).expect("the drawn wall");
        assert_eq!((crate::editor::bim::gestures::plane::axis_ends(&created.axis).1, created.wall_type.as_str()), ([14.0, 10.0], "wt-300"));
        history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().expect("snapshot").walls.len(), 4, "the click is one history row and its inverse removes the wall");
    });
}

#[test]
fn a_move_shows_the_preview_in_the_window_transient_and_never_touches_the_document() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        let wall = armed("wall");
        dispatch_in(&mut app, press(10.0, 10.0), &wall).await;
        let before = app.document_pack().await.expect("pack");
        let moved = dispatch_in(&mut app, hover(13.0, 10.0), &wall).await;
        assert!(moved.wrote(TypedOperationResultLane::WindowTransient) && !moved.edited_document(), "a move writes the window transient only");
        let after = app.document_pack().await.expect("pack");
        assert!(before.pack == after.pack && before.spr == after.spr, "a preview never changes document bytes");
        let scene = canvas_scene(&mut app, plan::BODY_KEY, &wall).await;
        assert!(scene.layers_json.contains("preview:"), "the rubber band is painted over the plan");
        assert!(scene.layers_json.contains("3.00 m"), "with its length");
    });
}

#[test]
fn escape_cancels_the_chain_and_a_double_click_ends_it() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        let wall = armed("wall");
        dispatch_in(&mut app, press(10.0, 10.0), &wall).await;
        dispatch_in(&mut app, BimCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &wall).await;
        assert!(!dispatch_in(&mut app, press(14.0, 10.0), &wall).await.edited_document(), "after escape the next click starts a chain");
        let (x, y) = at(14.0, 10.0);
        dispatch_in(&mut app, BimCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick { x, y, width: 800.0, height: 600.0, ..Default::default() }), &wall).await;
        assert!(!dispatch_in(&mut app, press(18.0, 10.0), &wall).await.edited_document(), "after a double click the next click starts a chain");
        assert_eq!(app.snapshot().expect("snapshot").walls.len(), 4);
    });
}

#[test]
fn switching_the_utility_cancels_the_gesture_of_the_window() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        dispatch_in(&mut app, press(10.0, 10.0), &armed("wall")).await;
        assert!(!dispatch_in(&mut app, press(14.0, 10.0), &armed("beam")).await.edited_document(), "the beam tool starts its own gesture: the wall chain died with the utility");
        assert_eq!(app.snapshot().expect("snapshot").walls.len(), 4);
    });
}

#[test]
fn the_select_utility_selects_the_wall_under_the_press_and_a_hotkey_arms_a_utility() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        let selected = dispatch_in(&mut app, press(4.0, 0.0), &armed("select")).await;
        assert!(!selected.edited_document());
        let selection = app.interaction_state().await.selection.get(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        assert_eq!(selection, vec!["w-south".to_string()], "the press selected the wall through the framework's own interaction lane");
        let armed_wall = dispatch_in(&mut app, BimCommand::ArmWall(arm_utility::ArmWall {}), &armed("select")).await;
        assert!(armed_wall.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, utility_id } if window_id == WINDOW && utility_id == "wall")));
    });
}

#[test]
fn the_world_window_draws_with_the_ground_point_of_its_ray() {
    on_a_big_stack(|| async {
        let mut app = bim_app().await;
        let windows = [("bim-world", world::WINDOW_KIND_ID)];
        let wall = ViewModel { active_utility_id: Some("wall".into()), ..view(Locale::En, &windows, Some("bim-world")) };
        let down = |x: f64, y: f64| BimCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: vec![x, y, 0.0], ..Default::default() });
        assert!(!dispatch_in(&mut app, down(10.0, 10.0), &wall).await.edited_document());
        assert!(dispatch_in(&mut app, down(14.0, 10.0), &wall).await.edited_document(), "two ground points write a wall");
        assert_eq!(app.snapshot().expect("snapshot").walls.len(), 5);
    });
}
