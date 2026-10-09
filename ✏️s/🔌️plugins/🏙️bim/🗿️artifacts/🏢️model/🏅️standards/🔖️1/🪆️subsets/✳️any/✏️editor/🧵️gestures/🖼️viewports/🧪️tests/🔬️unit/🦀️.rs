//! 🧪️ The sheet tools: standard scales, picking and moving viewports to the whole millimetre, scaling by the corner handle, nothing written for a press that did not travel, the place tool and the ghosts.

use super::*;
use crate::editor::bim::gestures::session::Modifiers;
use crate::standards::v1::subsets::any::io::export::sheets::testkit::{house_with_sheets, inferred};
use crate::{ModelInference, ModelSnapshot};

fn pointer(at: P) -> Pointer {
    Pointer { at, modifiers: Modifiers::default(), tolerance: 0.5 }
}

struct Rig {
    model: ModelSnapshot,
    inference: ModelInference,
}

fn rig() -> Rig {
    let model = house_with_sheets();
    let inference = inferred(&model);
    Rig { model, inference }
}

impl Rig {
    fn context(&self, selected: &'static [String]) -> ToolContext<'_> {
        let mut context = ToolContext::new(&self.model, &self.inference, Surface::Sheet { sheet: "sh-plans".into() }, "seed");
        context.selected = selected;
        context
    }

    fn window(&self, viewport: &str) -> PaperRect {
        self.inference.sheet_layouts["sh-plans"].viewports.iter().find(|placed| placed.viewport == viewport).expect("a placed viewport").window
    }
}

fn centre(window: &PaperRect) -> P {
    [window.x + window.width / 2.0, window.y + window.height / 2.0]
}

fn selected(ids: &[&str]) -> &'static [String] {
    Box::leak(ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().into_boxed_slice())
}

#[test]
fn the_nearest_standard_scale_is_found_on_the_logarithmic_scale() {
    for (wanted, scale) in [(90.0, 100), (60.0, 50), (1500.0, 1000), (0.4, 1), (160.0, 150), (180.0, 200), (110.0, 100), (240.0, 250)] {
        assert_eq!(nearest_scale(wanted), scale, "{wanted}");
    }
    assert!(SERIES.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn a_press_selects_the_viewport_under_the_pointer_and_a_drag_moves_it_to_the_whole_millimetre() {
    let rig = rig();
    let mut context = rig.context(&[]);
    let window = rig.window("vp-ground");
    let start = centre(&window);
    let mut tool = Arrange::default();
    let pressed = tool.event(&mut context, &ToolEvent::Down(pointer(start)));
    assert_eq!(pressed.pick.as_ref().map(|pick| pick.targets.clone()), Some(vec![("viewport".to_string(), "vp-ground".to_string())]));
    assert!(pressed.mutations.is_empty());
    tool.event(&mut context, &ToolEvent::Move(pointer([start[0] + 20.4, start[1] + 10.2])));
    assert!(!tool.preview(&context).is_empty());
    let released = tool.event(&mut context, &ToolEvent::Up(pointer([start[0] + 20.4, start[1] + 10.2])));
    let [ModelMutation::SetViewport(set)] = released.mutations.as_slice() else { panic!("one set-viewport: {released:?}") };
    assert_eq!(set.id, "vp-ground");
    assert_eq!((set.position, set.scale), (Some(crate::Point2 { x: 50.0, y: 40.0 }), None));
    assert!(tool.preview(&context).is_empty());
}

#[test]
fn shift_moves_in_tenths_of_a_millimetre() {
    let rig = rig();
    let mut context = rig.context(&[]);
    let start = centre(&rig.window("vp-ground"));
    let mut tool = Arrange::default();
    tool.event(&mut context, &ToolEvent::Down(pointer(start)));
    let fine = Pointer { modifiers: Modifiers { shift: true, ..Modifiers::default() }, ..pointer([start[0] + 20.44, start[1]]) };
    let released = tool.event(&mut context, &ToolEvent::Up(fine));
    let [ModelMutation::SetViewport(set)] = released.mutations.as_slice() else { panic!("one set-viewport") };
    assert_eq!(set.position.map(|position| position.x), Some(50.4));
}

#[test]
fn a_press_that_did_not_travel_writes_nothing() {
    let rig = rig();
    let mut context = rig.context(&[]);
    let start = centre(&rig.window("vp-ground"));
    let mut tool = Arrange::default();
    tool.event(&mut context, &ToolEvent::Down(pointer(start)));
    let released = tool.event(&mut context, &ToolEvent::Up(pointer([start[0] + 0.7, start[1] - 0.6])));
    assert!(released.mutations.is_empty() && released.refused.is_none());
}

#[test]
fn a_press_on_the_bare_paper_clears_the_selection() {
    let rig = rig();
    let mut context = rig.context(selected(&["vp-ground"]));
    let mut tool = Arrange::default();
    let pressed = tool.event(&mut context, &ToolEvent::Down(pointer([2.0, 2.0])));
    assert_eq!(pressed.pick.as_ref().map(|pick| (pick.targets.len(), pick.merge)), Some((0, "replace")));
}

#[test]
fn dragging_the_corner_handle_of_a_selected_viewport_scales_it_to_a_standard_scale() {
    let rig = rig();
    let mut context = rig.context(selected(&["vp-ground"]));
    let window = rig.window("vp-ground");
    let handle = [window.right(), window.bottom()];
    let mut tool = Arrange::default();
    let pressed = tool.event(&mut context, &ToolEvent::Down(pointer(handle)));
    assert!(pressed.pick.is_none(), "the handle press keeps the selection");
    let content = window.width - 10.0;
    let half = [window.x + 10.0 + content / 2.0, handle[1]];
    tool.event(&mut context, &ToolEvent::Move(pointer(half)));
    assert!(!tool.preview(&context).is_empty());
    let released = tool.event(&mut context, &ToolEvent::Up(pointer(half)));
    let [ModelMutation::SetViewport(set)] = released.mutations.as_slice() else { panic!("one set-viewport: {released:?}") };
    assert_eq!((set.id.as_str(), set.scale, set.position), ("vp-ground", Some(200), None));
}

#[test]
fn the_handles_belong_to_the_selected_viewports_only() {
    let rig = rig();
    let layout = &rig.inference.sheet_layouts["sh-plans"];
    assert_eq!(handles(layout, &[]).len(), 0);
    let marks = handles(layout, &["vp-first".to_string()]);
    assert_eq!(marks.len(), 1);
    let window = rig.window("vp-first");
    assert_eq!(marks[0].points, vec![window.right(), window.bottom()]);
}

#[test]
fn escape_drops_the_drag() {
    let rig = rig();
    let mut context = rig.context(&[]);
    let start = centre(&rig.window("vp-ground"));
    let mut tool = Arrange::default();
    tool.event(&mut context, &ToolEvent::Down(pointer(start)));
    tool.event(&mut context, &ToolEvent::Move(pointer([start[0] + 30.0, start[1]])));
    tool.event(&mut context, &ToolEvent::Escape);
    assert!(tool.preview(&context).is_empty());
    assert!(tool.event(&mut context, &ToolEvent::Up(pointer([start[0] + 30.0, start[1]]))).mutations.is_empty());
}

#[test]
fn a_click_with_the_place_tool_puts_the_selected_view_on_the_sheet_and_selects_the_viewport() {
    let rig = rig();
    let mut context = rig.context(selected(&["v-section-b"]));
    let mut tool = Place::default();
    tool.event(&mut context, &ToolEvent::Move(pointer([100.2, 120.7])));
    assert!(!tool.preview(&context).is_empty());
    let placed = tool.event(&mut context, &ToolEvent::Down(pointer([100.2, 120.7])));
    let [ModelMutation::CreateViewport(create)] = placed.mutations.as_slice() else { panic!("one create-viewport: {placed:?}") };
    assert_eq!((create.viewport.sheet.as_str(), create.viewport.view.as_str()), ("sh-plans", "v-section-b"));
    assert_eq!((create.viewport.position.x, create.viewport.position.y), (100.0, 121.0));
    assert_eq!(placed.pick.as_ref().map(|pick| pick.targets.clone()), Some(vec![("viewport".to_string(), create.id.clone())]));
}

#[test]
fn the_place_tool_takes_the_next_view_that_is_not_on_the_sheet_without_a_selected_one() {
    let rig = rig();
    let mut context = rig.context(&[]);
    let placed = Place::default().event(&mut context, &ToolEvent::Down(pointer([60.0, 60.0])));
    let [ModelMutation::CreateViewport(create)] = placed.mutations.as_slice() else { panic!("one create-viewport: {placed:?}") };
    assert!(!["v-plan-st-ground", "v-plan-st-first"].contains(&create.viewport.view.as_str()));
    assert_ne!(create.viewport.view, "v-3d");
}

#[test]
fn the_tools_do_nothing_off_a_sheet_surface() {
    let rig = rig();
    let mut context = ToolContext::new(&rig.model, &rig.inference, Surface::Plan { storey: "st-ground".into() }, "seed");
    assert_eq!(Arrange::default().event(&mut context, &ToolEvent::Down(pointer([40.0, 40.0]))), Step::default());
    assert_eq!(Place::default().event(&mut context, &ToolEvent::Down(pointer([40.0, 40.0]))), Step::refuse(VIEW_MISSING));
}
