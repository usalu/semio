use semio_framework_job::{Generation, StepBudget, StepContext};
use semio_framework_plugin::{plugin_app_close_prelude::SurfaceId, testkit::new_app_with_registry, App, EditorApp, PluginApp, ViewModel};
use semio_framework_ui_runtime::{ComponentTreeProducer, ComponentTreeProducerStep, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconciler, TreeNode};
use semio_s_plugin_procedural::editor::procedural3d::{create_procedural3d_app, Procedural3dCommand, Procedural3dPlayApp};

fn count_nodes(node: &TreeNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn measure_surface(app: &mut EditorApp<Procedural3dPlayApp>, body_key: &str, generation: u64) {
    let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
    let nodes = count_nodes(&tree.root);
    let mut producer = ComponentTreeProducer::try_new(tree.root, generation).expect("producer admission");
    let mut producer_steps = 0;
    loop {
        producer_steps += 1;
        match producer.step(generation, false, false) {
            ComponentTreeProducerStep::MoreWork => {}
            ComponentTreeProducerStep::Complete => break,
            ComponentTreeProducerStep::Fault(fault) => panic!("producer fault after {producer_steps} steps: {fault:?}"),
        }
    }
    let produced = producer.take_complete().expect("complete tree");
    let surface = SurfaceId::try_from(format!("1:{body_key}")).expect("surface");
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new(surface), produced, generation).expect("reconcile admission");
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut reconcile_steps = 0;
    loop {
        reconcile_steps += 1;
        let mut context = StepContext::new(operation, Generation(generation), StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_ms, &mut preview_sequence);
        match job.drive_one(&mut context) {
            SurfaceReconcileJobStep::MoreWork => {}
            SurfaceReconcileJobStep::Ready => break,
            SurfaceReconcileJobStep::Fault => panic!("reconcile fault after {reconcile_steps} steps: {:?}", job.fault()),
        }
    }
    let (_, ready) = match job.take_ready() {
        Ok(ready) => ready,
        Err(_) => panic!("ready reconciliation was not available"),
    };
    let ready = ready.expect("initial patch");
    let (patch, _) = ready.publish().expect("publish patch");
    println!("surface={body_key} nodes={nodes} producer_steps={producer_steps} reconcile_steps={reconcile_steps} patch_ops={} revision={} base={}", patch.ops.len(), patch.revision.0, patch.base_revision.0);
}

fn main() {
    let definition = create_procedural3d_app();
    println!("generated ids={:?}", Procedural3dCommand::TOOL_JOB_IDS);
    for action in definition.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| Procedural3dCommand::TOOL_JOB_IDS.contains(&action.id.as_str())) {
        println!("action {} classification={:?}", action.id, action.semantics.execution.interactive_job);
    }
    for command in &definition.commands {
        println!("command {} classification={:?}", command.id, command.semantics.execution.interactive_job);
    }
    fn manifest() -> App {
        App { definition: create_procedural3d_app(), examples: Vec::new() }
    }
    let mut app = semio_framework::io::resolve_ready(new_app_with_registry::<EditorApp<Procedural3dPlayApp>>(manifest));
    for (index, body_key) in ["procedural.play.main", "procedural.play.preview", "procedural.play.generations", "procedural.play.generate-form", "procedural.play.generate-preview"].into_iter().enumerate() {
        measure_surface(&mut app, body_key, index as u64 + 1);
    }
    std::process::exit(0);
}
