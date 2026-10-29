//! 🧰️ The fixtures every tool test shares: the demo model without its walls, extended by one type of each family, and a rig that drives a tool session pointer event by pointer event and
//! applies every emitted mutation through the central applier, so a test reads the model the gesture left behind.

use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family;
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::ViewModel;
use crate::editor::bim::BimModelApp;
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::ToolSession;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::{GestureKey, Modifiers, Pointer, Step, Surface, ToolContext, ToolEvent};
use semio_s_artifact_bim_model_storey_levels::editor::bim::marks::{Preview, Shape};
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{plan, section, sheet};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Beam, ColumnType, Component, DoorType, Family, FamilyParameter, FamilySolid, Material, ModelSnapshot, RoofType, Slab, SlabType, WindowType};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{DoorLeaves, ExprPoint3, FamilyCategory, Layer, LayerFunction, MaterialCategory, ParameterKind, Point2, Profile, Rgb, SolidShape, Swing};
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};

pub const PLAN: &str = semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID;

fn layer(material: &str, thickness: f64) -> Layer {
    Layer { material: material.into(), thickness, function: LayerFunction::Structure }
}

/// 🏠️ Storeys `st-ground` (3 m) and `st-first` (2.8 m), wall type `wt-300`, and a type of every other family; no element yet.
pub fn model() -> ModelSnapshot {
    let mut snapshot = BimModelApp::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    snapshot.walls.clear();
    let color = Rgb { r: 0.5, g: 0.5, b: 0.5 };
    for (id, category) in [("m-glass", MaterialCategory::Glass), ("m-steel", MaterialCategory::Metal), ("m-concrete", MaterialCategory::Concrete)] {
        snapshot.materials.insert(id.into(), Material { name: id.into(), category, color, density: 2400.0, conductivity: 1.0, specific_heat: 900.0 });
    }
    snapshot.slab_types.insert("sl-200".into(), SlabType { name: "Slab".into(), layers: vec![layer("m-concrete", 0.2)] });
    snapshot.roof_types.insert("rf-200".into(), RoofType { name: "Roof".into(), layers: vec![layer("m-concrete", 0.2)] });
    snapshot.column_types.insert("col-30".into(), ColumnType { name: "Column".into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material: "m-concrete".into() });
    snapshot.beam_types.insert("bm-20".into(), semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::BeamType { name: "Beam".into(), profile: Profile::Rectangle { width: 0.2, depth: 0.4 }, material: "m-concrete".into() });
    snapshot.window_types.insert("win-12".into(), WindowType { name: "Window".into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 2, material: "m-steel".into(), u_value: None, g_value: None, frame_fraction: None });
    snapshot.curtain_wall_types.clear();
    snapshot.curtain_wall_types.insert("cw-fa".into(), semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::CurtainWallType { name: "Facade".into(), u_grid: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::CurtainGrid::Spacing { spacing: 1.5 }, v_grid: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::CurtainGrid::Spacing { spacing: 1.5 }, interior_mullion: Profile::Rectangle { width: 0.05, depth: 0.1 }, border_mullion: Profile::Rectangle { width: 0.05, depth: 0.1 }, panel: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::CurtainPanel::Glass, panel_material: "m-glass".into(), mullion_material: "m-steel".into(), u_value: None, g_value: None, frame_fraction: None });
    snapshot.door_types.insert("door-09".into(), DoorType { name: "Door".into(), width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: DoorLeaves::Single, swing: Swing::Left, material: "m-steel".into(), u_value: None });
    snapshot
}

/// 🧱️ [`model`] with the four walls of the demo room (8 by 6 metres) on the ground storey.
pub fn room() -> ModelSnapshot {
    let mut snapshot = BimModelApp::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let extras = model();
    snapshot.materials = extras.materials;
    snapshot.slab_types = extras.slab_types;
    snapshot.roof_types = extras.roof_types;
    snapshot.column_types = extras.column_types;
    snapshot.beam_types = extras.beam_types;
    snapshot.window_types = extras.window_types;
    snapshot.door_types = extras.door_types;
    snapshot.curtain_wall_types = extras.curtain_wall_types;
    snapshot
}

const TABLE: &str = include_str!("../../../🧪️tests/🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

/// 🛋️ The demo room (walls, storeys) with the families of the table fixture (a table, three profiles, a broken generic one) and a basin (plumbing, one parameter `width`, one cuboid).
pub fn furnished() -> ModelSnapshot {
    let mut snapshot = room();
    let table: ModelSnapshot = from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes");
    snapshot.materials.extend(table.materials);
    snapshot.families = table.families;
    snapshot.family_parameters = table.family_parameters;
    snapshot.family_solids = table.family_solids;
    snapshot.families.insert("fam-basin".into(), Family { name: "Basin".into(), category: FamilyCategory::Plumbing });
    snapshot.family_parameters.insert("fam-basin.width".into(), FamilyParameter { family: "fam-basin".into(), name: "width".into(), kind: ParameterKind::Length, value: "0.5 m".into() });
    let offset = ExprPoint3 { x: "0 m".into(), y: "0 m".into(), z: "0 m".into() };
    let shape = SolidShape::Cuboid { x: "-width / 2".into(), y: "0 m".into(), z: "0 m".into(), width: "width".into(), depth: "0.4 m".into(), height: "0.2 m".into() };
    snapshot.family_solids.insert("s-basin".into(), FamilySolid { family: "fam-basin".into(), name: "Bowl".into(), shape, material: "\"m-steel\"".into(), visible: "true".into(), offset });
    snapshot
}

/// 🪑️ [`furnished`] with a table `c-table` and a basin `c-basin` mounted on the first wall, on the ground storey.
pub fn placed() -> ModelSnapshot {
    let mut snapshot = furnished();
    let wall = snapshot.walls.keys().next().cloned().expect("the room has walls");
    let storey = snapshot.walls[&wall].storey.clone();
    let component = |family: &str, host: Option<String>, name: &str| Component { storey: storey.clone(), family: family.into(), position: Point2 { x: 1.0, y: 1.0 }, elevation: 0.0, rotation: 0.0, mirrored: false, host, system: None, name: name.into() };
    snapshot.components.insert("c-table".into(), component("fam-table", None, "Table 1"));
    snapshot.components.insert("c-basin".into(), component("fam-basin", Some(wall), "Basin 1"));
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
            Surface::World { .. } => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID,
            Surface::Section { .. } => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID,
            Surface::Sheet { .. } => semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::WINDOW_KIND_ID,
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

    /// 🔑️ Offers a gesture key to the tool; every mutation of the answer is applied to the rig's model. The flag says whether the tool took the key.
    pub fn key(&mut self, key: semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::GestureKey) -> (bool, Step) {
        let mut taken = false;
        let step = self.drive(|session, context| {
            let (took, step, preview) = session.key(context, key);
            taken = took;
            (step, preview)
        });
        (taken, step)
    }

    fn drive(&mut self, feed: impl FnOnce(&mut ToolSession, &mut ToolContext<'_>) -> (Step, Preview)) -> Step {
        self.operations += 1;
        let inference = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &self.snapshot, |inference| inference.clone());
        let snapshot = self.snapshot.clone();
        let mut context = ToolContext::new(&snapshot, &inference, self.surface.clone(), &format!("op{}", self.operations));
        context.selected = &self.selected;
        context.library = &self.library;
        context.labels = Some(&semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels::NATIVE_EN);
        let (step, preview) = feed(&mut self.session, &mut context);
        self.preview = preview;
        for mutation in &step.mutations {
            self.snapshot = semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::apply_model_mutation(&self.snapshot, mutation).expect("an emitted mutation applies");
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
    pub fn shows(&self, shape: semio_s_artifact_bim_model_storey_levels::editor::bim::marks::Shape) -> bool {
        self.preview.marks.iter().any(|mark| mark.shape == shape)
    }
}

pub mod context {
    use crate::editor::bim::*;
    use crate::editor::bim::{BimCommand, BimModelApp};
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{Effect, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    /// 🧪️ The registered, MOUNTED fixture app every editor test drives, bound to [`BIM_TEST_INSTANCE`] so typed commands reach their retained routes and retired through the framework's
    /// exact close loop when it goes out of scope.
    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
    /// scope pays the caller's terminal retirement after the paid app close.
    pub struct BimApp(pub VcsArtifactApp<EditorApp<BimModelApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

    impl std::ops::Deref for BimApp {
        type Target = VcsArtifactApp<EditorApp<BimModelApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for BimApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for BimApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
            }
        }
    }

    pub const BIM_TEST_INSTANCE: u32 = 1;

    pub fn bim_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_bim_app(), examples: Vec::new() }
    }

    pub async fn bim_app() -> BimApp {
        let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = new_app_with_registry::<EditorApp<BimModelApp>>(bim_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(BIM_TEST_INSTANCE).await;
        BimApp(app, scope)
    }

    /// 🔁️ Dispatches `command` as the host does (no window addressed), drives its retained publication home and applies any `LoadDocument` effect.
    pub async fn dispatch(app: &mut BimApp, command: BimCommand) -> Dispatched {
        let mut result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, BIM_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let Effect::LoadDocument { pack, spr } = effect {
                let files = store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                semio_framework_plugin::artifact_app_laws::load_document(&mut app.0, &files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("test host applies load-document effect");
            }
        }
        Dispatched { result, lanes: settled.lanes }
    }

    /// 🪟️ Dispatches `command` addressed at the window instance `view` names, as the shell does for a window-scoped action.
    pub async fn dispatch_in(app: &mut BimApp, command: BimCommand, view: &ViewModel) -> Dispatched {
        let action_meta = semio_framework_plugin::ActionMeta { instance_id: BIM_TEST_INSTANCE, view_state: Some(view.clone()), ..meta("local") };
        let mut result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action_meta, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, BIM_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        Dispatched { result, lanes: settled.lanes }
    }

    pub struct Dispatched {
        pub result: InvocationResult,
        pub lanes: Vec<TypedOperationResultLane>,
    }

    impl Dispatched {
        pub fn edited_document(&self) -> bool {
            self.lanes.contains(&TypedOperationResultLane::Artifact)
        }
        pub fn wrote(&self, lane: TypedOperationResultLane) -> bool {
            self.lanes.contains(&lane)
        }
    }

    impl std::ops::Deref for Dispatched {
        type Target = InvocationResult;
        fn deref(&self) -> &Self::Target {
            &self.result
        }
    }

    pub async fn history_verb(app: &mut BimApp, verb: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, verb, BIM_TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }

    pub fn view(locale: semio_framework_ui_locale::Locale, windows: &[(&str, &str)], addressed: Option<&str>) -> ViewModel {
        ViewModel {
            window_id: addressed.map(str::to_string),
            window_instances: windows.iter().map(|(id, kind)| ViewWindowInstance { id: (*id).into(), window_kind_id: (*kind).into() }).collect(),
            ..ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
        }
    }

    pub async fn render_text(app: &mut BimApp, body_key: &str, view: &ViewModel) -> String {
        semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, view, app.1.cx()).await.expect("project and retire semantic tree")
    }

    pub async fn canvas_scene(app: &mut BimApp, body_key: &str, view: &ViewModel) -> semio_framework_plugin::Canvas2dScene {
        let json = render_text(app, body_key, view).await;
        semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&json).expect("assembled canvas scene")
    }

    pub async fn world_scene(app: &mut BimApp, view: &ViewModel) -> semio_framework_plugin::World3dScene {
        semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::BODY_KEY, view, app.1.cx(), |root| semio_framework_plugin::artifact_app_laws::built_surface_scene(root)).await.expect("render world").expect("assembled 3D scene")
    }
}

/// 🧰️ Pure-handler fixtures for the command nodes: the demo model behind a history-free view, no mounted app.
pub mod support {
    use crate::editor::bim::*;
    use crate::editor::bim::BimModelApp;
    use semio_framework_plugin::ArtifactView;
    use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
    use semio_framework_plugin::ConfigView;
    use semio_framework_plugin::Emit;
    use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
    use semio_framework_plugin::NoConfig;
    use semio_framework_plugin::NoConfigMutation;
    use semio_framework_plugin::ViewModel;
    use semio_framework_plugin::HistoryView;

    pub fn demo() -> ModelSnapshot {
        BimModelApp::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births")
    }

    pub fn english() -> ViewModel {
        ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }

    pub fn german() -> ViewModel {
        ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native)
    }

    pub fn ctx(selected: &[&str]) -> BimDispatchCtx {
        BimDispatchCtx { selected: selected.iter().map(|id| id.to_string()).collect(), utility: semio_s_artifact_bim_model_kit::editor::bim::utilities::DEFAULT_UTILITY.into(), ..BimDispatchCtx::default() }
    }

    /// 🧪️ Runs `body` against `snapshot` the way a handler sees it.
    pub fn run<R>(snapshot: &ModelSnapshot, body: impl FnOnce(&ArtifactView<'_, ModelSnapshot>, &ConfigView<'_, NoConfig>) -> R) -> R {
        let history = HistoryView::empty();
        let doc = ArtifactView::new(snapshot, &history);
        body(&doc, &ConfigView { snapshot: &NoConfig::default(), window: None })
    }

    /// 🔁️ The snapshot after the emitted mutations are applied one after the other through the central applier.
    pub fn applied(snapshot: &ModelSnapshot, emit: &Emit<ModelMutation, NoConfigMutation>) -> ModelSnapshot {
        emit.artifact_mutations.iter().fold(snapshot.clone(), |state, mutation| semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::apply_model_mutation(&state, mutation).expect("the emitted mutation applies"))
    }
}
