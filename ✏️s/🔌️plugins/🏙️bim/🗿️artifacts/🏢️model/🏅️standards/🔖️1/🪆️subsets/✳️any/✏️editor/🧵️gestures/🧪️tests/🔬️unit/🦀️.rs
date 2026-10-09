use super::*;
use crate::editor::bim::BimModelApp;
use crate::{ColumnType, DoorLeaves, DoorType, Layer, LayerFunction, Material, MaterialCategory, Profile, Rgb, RoofType, SlabType, Swing, WindowType};
use semio_framework_plugin::ArtifactEditor;

/// 🧰️ The fixtures every tool test shares: the demo model without its walls, extended by one type of each family, and a rig that drives a tool session pointer event by pointer event and
/// applies every emitted mutation through the central applier, so a test reads the model the gesture left behind.
pub(crate) mod fixture {
    use super::*;
    use crate::editor::bim::gestures::session::{Modifiers, Pointer, Step, Surface, ToolContext, ToolEvent};
    use crate::editor::bim::gestures::ToolSession;

    pub const PLAN: &str = crate::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID;

    fn layer(material: &str, thickness: f64) -> Layer {
        Layer { material: material.into(), thickness, function: LayerFunction::Structure }
    }

    /// 🏠️ Storeys `st-ground` (3 m) and `st-first` (2.8 m), wall type `wt-300`, and a type of every other family; no element yet.
    pub fn model() -> ModelSnapshot {
        let mut snapshot = BimModelApp::initial_snapshot();
        snapshot.walls.clear();
        let color = Rgb { r: 0.5, g: 0.5, b: 0.5 };
        for (id, category) in [("m-glass", MaterialCategory::Glass), ("m-steel", MaterialCategory::Metal), ("m-concrete", MaterialCategory::Concrete)] {
            snapshot.materials.insert(id.into(), Material { name: id.into(), category, color, density: 2400.0, conductivity: 1.0, specific_heat: 900.0 });
        }
        snapshot.slab_types.insert("sl-200".into(), SlabType { name: "Slab".into(), layers: vec![layer("m-concrete", 0.2)] });
        snapshot.roof_types.insert("rf-200".into(), RoofType { name: "Roof".into(), layers: vec![layer("m-concrete", 0.2)] });
        snapshot.column_types.insert("col-30".into(), ColumnType { name: "Column".into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material: "m-concrete".into() });
        snapshot.beam_types.insert("bm-20".into(), crate::BeamType { name: "Beam".into(), profile: Profile::Rectangle { width: 0.2, depth: 0.4 }, material: "m-concrete".into() });
        snapshot.window_types.insert("win-12".into(), WindowType { name: "Window".into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 2, material: "m-steel".into() });
        snapshot.door_types.insert("door-09".into(), DoorType { name: "Door".into(), width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: DoorLeaves::Single, swing: Swing::Left, material: "m-steel".into() });
        snapshot
    }

    /// 🧱️ [`model`] with the four walls of the demo room (8 by 6 metres) on the ground storey.
    pub fn room() -> ModelSnapshot {
        let mut snapshot = BimModelApp::initial_snapshot();
        let extras = model();
        snapshot.materials = extras.materials;
        snapshot.slab_types = extras.slab_types;
        snapshot.roof_types = extras.roof_types;
        snapshot.column_types = extras.column_types;
        snapshot.beam_types = extras.beam_types;
        snapshot.window_types = extras.window_types;
        snapshot.door_types = extras.door_types;
        snapshot
    }

    /// 🎛️ A tool session on a surface over a model; one pixel spans one centimetre, so a snap reaches ten and a pick eight centimetres.
    pub struct Rig {
        pub snapshot: ModelSnapshot,
        pub session: ToolSession,
        pub surface: Surface,
        pub selected: Vec<String>,
        pub library: Vec<String>,
        pub preview: Preview,
        operations: usize,
    }

    pub const PIXEL: f64 = 0.01;

    impl Rig {
        pub fn plan(utility: &str, snapshot: ModelSnapshot) -> Self {
            Self::on(utility, snapshot, Surface::Plan { storey: "st-ground".into() })
        }

        pub fn on(utility: &str, snapshot: ModelSnapshot, surface: Surface) -> Self {
            let kind = match surface {
                Surface::Plan { .. } => PLAN,
                Surface::World { .. } => crate::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID,
                Surface::Section { .. } => crate::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID,
            };
            Self { snapshot, session: ToolSession::new(utility, kind), surface, selected: Vec::new(), library: Vec::new(), preview: Preview::default(), operations: 0 }
        }

        pub fn pointer(&self, x: f64, y: f64, modifiers: Modifiers) -> Pointer {
            Pointer { at: [x, y], modifiers, tolerance: PIXEL }
        }

        /// ➡️ Sends one event; every mutation of the answer is applied to the rig's model.
        pub fn send(&mut self, event: ToolEvent) -> Step {
            self.drive(|session, context| session.advance(context, &event))
        }

        /// ⌨️ Moves the keyboard cursor by `delta` metres from `start` (or from where the pointer last was); every mutation of the answer is applied to the rig's model.
        pub fn nudge(&mut self, delta: [f64; 2], start: [f64; 2]) -> Step {
            self.drive(|session, context| session.nudge(context, delta, start))
        }

        /// ⌨️ Clicks at the keyboard cursor; every mutation of the answer is applied to the rig's model.
        pub fn place(&mut self, start: [f64; 2]) -> Step {
            self.drive(|session, context| session.place(context, start))
        }

        /// ⌨️ Types one line into the entry field the way a submit does; every mutation of the answer is applied to the rig's model.
        pub fn typed(&mut self, line: &str) -> Step {
            self.drive(|session, context| session.enter(context, line))
        }

        fn drive(&mut self, feed: impl FnOnce(&mut ToolSession, &mut ToolContext<'_>) -> (Step, Preview)) -> Step {
            self.operations += 1;
            let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &self.snapshot, |inference| inference.clone());
            let snapshot = self.snapshot.clone();
            let mut context = ToolContext::new(&snapshot, &inference, self.surface.clone(), &format!("op{}", self.operations));
            context.selected = &self.selected;
            context.library = &self.library;
            context.labels = Some(&crate::editor::bim::terminology::BimLabels::NATIVE_EN);
            let (step, preview) = feed(&mut self.session, &mut context);
            self.preview = preview;
            for mutation in &step.mutations {
                self.snapshot = crate::mutations::apply_model_mutation(&self.snapshot, mutation).expect("an emitted mutation applies");
            }
            step
        }

        pub fn down(&mut self, x: f64, y: f64) -> Step {
            self.send(ToolEvent::Down(self.pointer(x, y, Modifiers::default())))
        }

        pub fn mv(&mut self, x: f64, y: f64) -> Step {
            self.send(ToolEvent::Move(self.pointer(x, y, Modifiers::default())))
        }

        pub fn up(&mut self, x: f64, y: f64) -> Step {
            self.send(ToolEvent::Up(self.pointer(x, y, Modifiers::default())))
        }

        pub fn double(&mut self, x: f64, y: f64) -> Step {
            self.send(ToolEvent::Double(self.pointer(x, y, Modifiers::default())))
        }

        /// 🖱️ A press and its release: the step carries what either of them answered.
        pub fn click(&mut self, x: f64, y: f64) -> Step {
            let mut step = self.down(x, y);
            let release = self.up(x, y);
            step.mutations.extend(release.mutations);
            step.pick = step.pick.or(release.pick);
            step.refused = step.refused.or(release.refused);
            step
        }

        pub fn finish(&mut self) -> Step {
            self.send(ToolEvent::Finish)
        }

        pub fn escape(&mut self) -> Step {
            self.send(ToolEvent::Escape)
        }

        /// 🫧️ Whether the preview holds a mark of the shape.
        pub fn shows(&self, shape: crate::editor::bim::gestures::session::Shape) -> bool {
            self.preview.marks.iter().any(|mark| mark.shape == shape)
        }
    }
}

use fixture::*;
use session::{Modifiers, Shape, Surface, ToolEvent};

#[semio_framework_async_macros::async_test]
async fn every_utility_of_the_registry_arms_a_tool_that_owns_a_gesture() {
    for row in crate::editor::bim::utilities::UTILITIES.iter().filter(|row| row.id != "select") {
        let mut rig = Rig::plan(row.id, model());
        let before = rig.snapshot.clone();
        rig.mv(1.0, 1.0);
        assert_eq!(rig.snapshot, before, "{}: a move never writes", row.id);
        assert!(!rig.preview.marks.is_empty() || matches!(row.id, "move" | "rotate" | "window" | "door" | "opening" | "slab-walls" | "split-wall" | "copy" | "mirror" | "array" | "array-radial" | "offset" | "trim" | "extend" | "align" | "split"), "{} shows the snapped pointer", row.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn the_hotkeys_cover_the_specified_tools_and_each_arms_an_existing_command() {
    let bindings = crate::editor::bim::utilities::keybindings();
    for (key, utility) in [("w", "wall"), ("c", "column"), ("b", "beam"), ("s", "slab"), ("r", "roof"), ("n", "window"), ("d", "door"), ("t", "stair"), ("l", "railing"), ("p", "space"), ("g", "grid"), ("m", "measure"), ("v", "select"), ("e", "move"), ("q", "rotate"), ("shift+s", "slab-walls"), ("shift+w", "split-wall"), ("k", "copy"), ("shift+k", "mirror"), ("y", "array"), ("shift+y", "array-radial"), ("f", "offset"), ("x", "trim"), ("shift+x", "extend"), ("z", "align"), ("shift+z", "split")] {
        let (_, action) = bindings.iter().find(|(keys, _)| *keys == key).unwrap_or_else(|| panic!("no hotkey {key}"));
        let command = crate::editor::bim::BimModelApp::command_from_action(action, None).unwrap_or_else(|error| panic!("{action}: {}", error.message));
        let armed = crate::editor::bim::utilities::UTILITIES.iter().find(|row| row.arm == Some(*action)).map(|row| row.id);
        assert_eq!(armed, Some(utility), "{key} arms {utility}");
        assert!(crate::editor::bim::BIM_TOOL_IDS.contains(&command.command_id()));
    }
    let keys: Vec<&str> = bindings.iter().map(|(keys, _)| *keys).collect();
    let mut unique = keys.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), keys.len(), "no hotkey is bound twice");
    assert_eq!(GESTURE_KEYBINDINGS, &[("escape", "canvasEscape"), ("enter", "canvasCommitDraft")]);
}

#[semio_framework_async_macros::async_test]
async fn a_utility_switch_drops_the_gesture_in_progress_without_a_trace() {
    let mut owner = GestureOwner::default();
    let snapshot = model();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &snapshot, |inference| inference.clone());
    let mut context = session::ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-ground".into() }, "seed");
    let pointer = |x: f64| ToolEvent::Down(session::Pointer { at: [x, 0.0], modifiers: Modifiers::default(), tolerance: PIXEL });
    owner.advance("w1", PLAN, "wall", &mut context, &pointer(0.0));
    let (_, preview) = owner.advance("w1", PLAN, "wall", &mut context, &ToolEvent::Move(session::Pointer { at: [3.0, 0.0], modifiers: Modifiers::default(), tolerance: PIXEL }));
    assert!(preview.marks.iter().any(|mark| mark.shape == Shape::Label), "the chain shows its rubber band");
    let (step, preview) = owner.advance("w1", PLAN, "column", &mut context, &ToolEvent::Move(session::Pointer { at: [3.0, 0.0], modifiers: Modifiers::default(), tolerance: PIXEL }));
    assert!(step.mutations.is_empty() && !preview.marks.iter().any(|mark| mark.shape == Shape::Label), "the rubber band died with the utility");
    assert_eq!(owner.windows(), vec!["w1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn the_owner_closes_empty() {
    let mut owner = GestureOwner::default();
    let snapshot = model();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &snapshot, |inference| inference.clone());
    let mut context = session::ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-ground".into() }, "seed");
    owner.advance("w1", PLAN, "wall", &mut context, &ToolEvent::Escape);
    assert!(!semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner));
    assert!(matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, 1, 4096), Ok(semio_framework_plugin::PluginCloseStep::Complete)));
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner));
}

#[semio_framework_async_macros::async_test]
async fn the_preview_round_trips_through_the_transient_text_and_an_empty_one_is_no_text() {
    let mut rig = Rig::plan("wall", model());
    rig.click(0.0, 0.0);
    rig.mv(3.0, 0.0);
    let text = rig.preview.to_text();
    assert_eq!(session::Preview::from_text(&text), rig.preview);
    assert_eq!(session::Preview::default().to_text(), "");
    assert_eq!(session::Preview::from_text("not json"), session::Preview::default());
}

#[semio_framework_async_macros::async_test]
async fn pointers_convert_from_pixels_in_every_surface() {
    let mut ctx = crate::editor::bim::BimDispatchCtx::default();
    ctx.plan.viewport = store::Viewport2d { x: 0.0, y: 0.0, zoom: 40.0 };
    ctx.section.viewport = store::Viewport2d { x: 5.0, y: -1.5, zoom: 40.0 };
    let raw = Raw { x: 440.0, y: 260.0, width: 800.0, height: 600.0, ..Raw::default() };
    let plan = pointer_of(&ctx, &Surface::Plan { storey: String::new() }, 0.025, &raw);
    assert_eq!(plan.at, [1.0, 1.0], "the press at (440, 260) of an 800 by 600 plan is model (1, 1)");
    let section = pointer_of(&ctx, &Surface::Section { start: [0.0, 0.0], end: [10.0, 0.0] }, 0.025, &raw);
    assert_eq!(section.at, [6.0, 2.5], "section u runs along the line and v up: v = -(canvas y)");
}

#[semio_framework_async_macros::async_test]
async fn drawing_four_walls_placing_a_window_and_dragging_the_storey_top_re_infers_the_walls_and_the_window_frame() {
    let mut walls = Rig::plan("wall", model());
    for (x, y) in [(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)] {
        walls.down(x, y);
    }
    walls.down(0.0, 0.0);
    assert_eq!(walls.snapshot.walls.len(), 4, "five clicks draw four walls and close the loop");
    let mut windows = Rig::plan("window", walls.snapshot.clone());
    let placed = windows.down(3.0, 0.0);
    assert_eq!(placed.mutations.len(), 1);
    let opening = windows.snapshot.openings.keys().next().expect("the window").clone();
    let infer = |snapshot: &ModelSnapshot| crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, snapshot, |inference| inference.clone());
    let before = infer(&windows.snapshot);
    assert!(before.wall_layout.values().all(|layout| (layout.height - 3.0).abs() < 1e-9), "every wall starts as high as its 3 m storey");
    assert!((before.opening_frames[&opening].host_height - 3.0).abs() < 1e-9 && before.opening_frames[&opening].valid);
    let mut section = Rig::on("select", windows.snapshot.clone(), Surface::Section { start: [0.0, 0.0], end: [6.0, 0.0] });
    section.down(3.0, 3.0);
    section.mv(3.0, 3.5);
    let step = section.up(3.0, 3.5);
    assert!(matches!(step.mutations.as_slice(), [crate::ModelMutation::SetStoreyHeight(set)] if set.id == "st-ground" && (set.height - 3.5).abs() < 1e-9), "the handle writes exactly one set-storey-height, got {:?}", step.mutations);
    let after = infer(&section.snapshot);
    assert!(after.wall_layout.values().all(|layout| (layout.height - 3.5).abs() < 1e-9), "the four walls follow the storey");
    assert!((after.opening_frames[&opening].host_height - 3.5).abs() < 1e-9, "the window frame follows its host");
    assert_eq!(section.snapshot.walls, windows.snapshot.walls, "no wall was touched: the heights are inferred, never stored");
    assert!((after.storey_levels["st-first"].elevation - 3.5).abs() < 1e-9, "the storey above is lifted too");
}

const GESTURE_CASES: &str = include_str!("../../../../🧫️fixtures/🛠️gestures/🔣️.json");

fn point_of(value: &serde_json::Value) -> [f64; 2] {
    [value[0].as_f64().expect("x"), value[1].as_f64().expect("y")]
}

fn ring_of(value: &serde_json::Value) -> Vec<[f64; 2]> {
    value.as_array().expect("a ring").iter().map(point_of).collect()
}

fn same_ring(left: &[[f64; 2]], right: &[[f64; 2]]) -> bool {
    left.len() == right.len() && (0..left.len()).any(|shift| (0..left.len()).all(|index| plane::dist(left[(index + shift) % left.len()], right[index]) < 1e-9))
}

#[semio_framework_async_macros::async_test]
async fn the_cases_the_third_party_oracle_wrote_replay_through_the_tools_to_the_same_numbers() {
    let cases: serde_json::Value = serde_json::from_str(GESTURE_CASES).expect("the committed cases");
    let each = |key: &str| cases[key].as_array().unwrap_or_else(|| panic!("cases {key}")).clone();
    for case in each("arcs") {
        let bulge = plane::bulge_through(point_of(&case["start"]), point_of(&case["through"]), point_of(&case["end"])).expect("an arc");
        assert!((bulge - case["bulge"].as_f64().expect("bulge")).abs() < 1e-9, "arc {case}: {bulge}");
    }
    for case in each("projections") {
        let axis = plane::axis_of(point_of(&case["axis"]["start"]), point_of(&case["axis"]["end"]), 0.0);
        let found = plane::project(&axis, point_of(&case["point"]));
        assert!((found.offset - case["offset"].as_f64().expect("offset")).abs() < 1e-9 && (found.distance - case["distance"].as_f64().expect("distance")).abs() < 1e-9, "projection {case}");
        assert_eq!(found.side.signum() as i64, case["side"].as_i64().expect("side"), "side of {case}");
    }
    for case in each("rectangles") {
        let ring = plane::rectangle(point_of(&case["a"]), point_of(&case["b"]));
        assert!(same_ring(&ring, &ring_of(&case["ring"])) && (plane::signed_area(&ring) - case["area"].as_f64().expect("area")).abs() < 1e-9, "rectangle {case}");
    }
    for case in each("counter_clockwise") {
        let ring = plane::counter_clockwise(ring_of(&case["ring"]));
        assert!(same_ring(&ring, &ring_of(&case["expected"])) && (plane::signed_area(&ring) - case["area"].as_f64().expect("area")).abs() < 1e-9, "orientation {case}");
    }
    for case in each("windows") {
        let offset = opening::fitted_offset(case["length"].as_f64().expect("length"), case["width"].as_f64().expect("width"), case["foot"].as_f64().expect("foot"));
        assert_eq!(offset.map(|value| (value * 1e9).round()), case["offset"].as_f64().map(|value| (value * 1e9).round()), "window {case}");
    }
    for case in each("heights") {
        let elevation = case["elevation"].as_f64().expect("elevation");
        let storey = if elevation == 0.0 { "st-ground" } else { "st-first" };
        let top = elevation + room().storeys[storey].height;
        let mut rig = Rig::on("select", room(), Surface::Section { start: [0.0, 0.0], end: [8.0, 0.0] });
        rig.down(4.0, top);
        let step = rig.up(4.0, case["pointer"].as_f64().expect("pointer"));
        let [crate::ModelMutation::SetStoreyHeight(set)] = step.mutations.as_slice() else { panic!("a set-storey-height for {case}, got {:?}", step.mutations) };
        assert!((set.height - case["height"].as_f64().expect("height")).abs() < 1e-9 && set.id == storey, "height {case}: {}", set.height);
    }
    for case in each("rotations") {
        let mut rig = Rig::plan("rotate", room());
        rig.selected = vec!["w-south".into()];
        let (pivot, reference, target) = (point_of(&case["pivot"]), point_of(&case["reference"]), point_of(&case["target"]));
        rig.click(pivot[0], pivot[1]);
        rig.click(reference[0], reference[1]);
        let step = rig.click(target[0], target[1]);
        let [crate::ModelMutation::RotateElements(turned)] = step.mutations.as_slice() else { panic!("a rotate-elements for {case}, got {:?}", step.mutations) };
        assert!((turned.angle - case["angle"].as_f64().expect("angle")).abs() < 1e-9, "rotation {case}: {}", turned.angle);
    }
}

//#region 🔖️ModifyPreviews
fn modify_scene() -> ModelSnapshot {
    use crate::{Axis, Beam, Phase, Point2, Slab, Vertex, Wall};
    let mut snapshot = room();
    let line = |start: (f64, f64), end: (f64, f64)| Axis::Line { start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } };
    let wall = |axis: Axis, name: &str| Wall { axis, name: name.into(), ..snapshot.walls["w-south"].clone() };
    let (long, short) = (wall(line((0.0, 3.0), (10.0, 3.0)), "Long"), wall(line((1.0, 4.0), (5.0, 4.0)), "Short"));
    snapshot.walls.insert("w-long".into(), long);
    snapshot.walls.insert("w-short".into(), short);
    snapshot.beams.insert("b-1".into(), Beam { storey: "st-ground".into(), beam_type: "bm-20".into(), start: Point2 { x: 0.0, y: 2.0 }, end: Point2 { x: 8.0, y: 2.0 }, top_offset: 0.0, phase: Phase::New, name: "Beam".into() });
    let corner = |x: f64, y: f64| Vertex { point: Point2 { x, y }, bulge: 0.0 };
    snapshot.slabs.insert("sl-1".into(), Slab { storey: "st-ground".into(), slab_type: "sl-200".into(), boundary: vec![corner(0.0, 0.0), corner(8.0, 0.0), corner(8.0, 6.0), corner(0.0, 6.0)], holes: Vec::new(), offset: 0.0, slope: None, phase: Phase::New, name: "Floor".into() });
    snapshot
}

#[derive(Clone, Copy)]
enum Act {
    Move(f64, f64),
    Click(f64, f64),
    Press(f64, f64),
}

#[semio_framework_async_macros::async_test]
async fn every_modify_gesture_previews_live_in_the_plan_the_3d_window_and_the_transient_and_writes_nothing_until_the_last_click() {
    use Act::{Click, Move, Press};
    let scripts: [(&str, Vec<&str>, Vec<Act>); 10] = [
        ("copy", vec!["w-south"], vec![Move(1.0, 1.0), Click(1.0, 1.0), Move(1.0, 4.0)]),
        ("mirror", vec!["w-south"], vec![Move(10.0, -1.0), Click(10.0, -1.0), Move(10.0, 7.0)]),
        ("array", vec!["w-south"], vec![Click(1.0, 2.0), Click(1.0, 3.0), Move(1.0, 5.1)]),
        ("array-radial", vec!["w-south"], vec![Click(4.0, 3.0), Click(5.0, 3.0), Move(4.0, 4.0)]),
        ("offset", vec![], vec![Move(4.0, 0.0), Press(4.0, 0.0), Move(4.0, 1.52)]),
        ("trim", vec![], vec![Move(8.0, 1.5), Press(8.0, 1.5), Move(3.0, 3.0)]),
        ("extend", vec![], vec![Move(8.0, 1.5), Press(8.0, 1.5), Move(4.9, 4.0)]),
        ("align", vec!["w-north"], vec![Move(0.5, 0.0)]),
        ("split", vec![], vec![Move(2.0, 2.0)]),
        ("split", vec![], vec![Move(3.0, 2.0), Press(3.0, 2.0), Move(3.0, 4.0)]),
    ];
    for (index, (utility, selected, acts)) in scripts.into_iter().enumerate() {
        let mut scene = modify_scene();
        if index != 8 {
            scene.beams.clear();
        }
        if index != 9 {
            scene.slabs.clear();
        }
        let mut rig = Rig::plan(utility, scene);
        rig.selected = selected.iter().map(|id| id.to_string()).collect();
        let before = rig.snapshot.clone();
        let mut shown = Vec::new();
        for act in acts {
            let step = match act {
                Move(x, y) => rig.mv(x, y),
                Click(x, y) => rig.click(x, y),
                Press(x, y) => rig.down(x, y),
            };
            assert!(step.mutations.is_empty(), "{utility}: only the closing click writes");
            shown.push(rig.preview.marks.len());
        }
        assert_eq!(rig.snapshot, before, "{utility}: the document stays untouched while the gesture shows");
        assert!(shown.iter().all(|marks| *marks > 0), "{utility}: every step of the gesture shows something, {shown:?}");
        assert_eq!(session::Preview::from_text(&rig.preview.to_text()), rig.preview, "{utility}: the preview survives the window transient");
        assert!(!overlay::plan_records(&rig.snapshot, &rig.selected, utility, &rig.preview, 1.0).is_empty(), "{utility}: the plan paints it");
        let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &rig.snapshot, Clone::clone);
        assert!(!world::preview_items(&rig.snapshot, &inference, &world::config::BimWorldWindowConfig::default(), &rig.preview).is_empty(), "{utility}: the 3D window paints it");
    }
}
//#endregion 🔖️ModifyPreviews
