use super::*;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::testkit::{close_registered_fixture_app, meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

pub type Gis2dApp = VcsArtifactApp<EditorApp<Gis2dPlayApp>>;

/// 🧬️ Builds the real registered fixture and binds the instance addressed by [`meta`].
pub async fn app() -> Gis2dApp {
    let mut app = new_app_with_registry::<EditorApp<Gis2dPlayApp>>(gis2d_app_manifest_for_testkit).await;
    app.bind_instance_id(meta("local").instance_id).await;
    app
}

/// ✏️ Adapts `create_gis2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here.
pub fn gis2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_gis2d_app(), examples: Vec::new() }
}

/// 🪟️ Targets the real GIS Map window instance for render, config, and measure authority.
pub fn main_window_view() -> ViewModel {
    ViewModel { window_id: Some(map::GIS2D_PLAY_WINDOW_MAIN.into()), window_instances: vec![ViewWindowInstance { id: map::GIS2D_PLAY_WINDOW_MAIN.into(), window_kind_id: map::GIS2D_PLAY_WINDOW_MAIN.into() }], ..Default::default() }
}

/// 🧹️ Drives a GIS Map fixture to its exact terminal-empty ownership witness.
pub fn close(app: &mut Gis2dApp) {
    close_registered_fixture_app(app);
}

/// 📬️ Host-visible publication collected after one GIS Map command reaches quiescence.
pub struct Gis2dDispatchReceipt(semio_framework_plugin::testkit::TypedOperationFixtureReceipt);

impl Gis2dDispatchReceipt {
    pub fn artifact_publication_count(&self) -> usize {
        self.0.lanes.iter().filter(|lane| **lane == TypedOperationResultLane::Artifact).count()
    }

    pub fn effects(&self) -> &[Effect] {
        &self.0.effects
    }
}

/// 🎯️ Dispatches a typed GIS Map command and drives its retained publication to exact ACK.
pub async fn dispatch(app: &mut Gis2dApp, command: Gis2dCommand) -> Gis2dDispatchReceipt {
    let mut action_meta = meta("local");
    action_meta.view_state = Some(main_window_view());
    app.dispatch_typed(command, &action_meta).await.expect("dispatch admission");
    Gis2dDispatchReceipt(semio_framework_plugin::testkit::settle_registered_typed_operation(app, action_meta.instance_id).await.expect("dispatch publication"))
}

/// 🖼️ Renders and retires one GIS Map fixture component tree.
pub async fn render(app: &mut Gis2dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &main_window_view()).await.expect("render")).expect("render projection")
}

/// 📐️ Projects the main GIS Map window measures through the registered fixture.
pub async fn main_window_measures(app: &mut Gis2dApp) -> Vec<WindowMeasure> {
    app.window_measures(&main_window_view()).await.get(map::GIS2D_PLAY_WINDOW_MAIN).cloned().unwrap_or_default()
}

/// 🤝️ Applies disjoint commands to two registered, bound instances and proves convergence.
pub async fn assert_two_instances_converge<P>(channel: &str, command_a: Gis2dCommand, command_b: Gis2dCommand, probe: impl Fn(&Gis2dApp) -> P)
where
    P: PartialEq + std::fmt::Debug,
{
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    assert_eq!(instance_a.artifact_generation_now(), instance_b.artifact_generation_now(), "hot peers start at the same generation");
    assert_eq!(instance_a.snapshot().expect("initial a"), instance_b.snapshot().expect("initial b"), "hot peers start from the same independently materialized document");
    let document_a = instance_a.document_text().await.expect("initial document a");
    let document_b = instance_b.document_text().await.expect("initial document b");
    assert_eq!(document_a.dsl, document_b.dsl, "hot peers start from the same serialized initial snapshot");
    assert_eq!(document_a.ops, document_b.ops, "hot peers start with the same document identity and history cursor");
    let (backbone_a, backbone_b) = store::MemoryBackbone::pair(channel, channel).await;
    instance_a.attach_hot_backbone(store::Backbones::Memory(backbone_a)).await.expect("hot attach a after equal initial state proof");
    instance_b.attach_hot_backbone(store::Backbones::Memory(backbone_b)).await.expect("hot attach b after equal initial state proof");
    dispatch(&mut instance_a, command_a).await;
    dispatch(&mut instance_b, command_b).await;
    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");
    assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same snapshot");
    close(&mut instance_a);
    close(&mut instance_b);
}
