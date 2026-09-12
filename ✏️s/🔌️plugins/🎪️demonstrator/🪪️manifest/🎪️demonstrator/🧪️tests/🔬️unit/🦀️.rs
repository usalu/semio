use super::*;

fn test_bundle() -> Plugin<DemonstratorApps> {
    plugin().unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn bundle_keeps_its_plugin_identity() {
    let manifest = test_bundle().manifest;
    assert_eq!(manifest.plugin_id, PLUGIN_ID);
    assert_eq!(manifest.label, PLUGIN_LABEL);
    assert_eq!(manifest.version, PLUGIN_VERSION);
    assert_eq!(manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect::<Vec<_>>(), vec!["cad", "gis", "procedural", "process", "puzzle", "sourcing"]);
}

/// 🔗️ Ticket 26/09/05/S-END-TO-END lane H: the dependency list above is not free-standing prose —
/// every surface this bundle registers for an artifact kind it does NOT own must name that kind's
/// owner among its dependencies, or the host's `AppRouter` excludes the whole plugin
/// (`surface.contribution-not-permitted`) and none of its ten surfaces route. Derived from the
/// built manifest's own app dialects, so adding a borrowed app without its dependency fails here.
#[test]
fn every_borrowed_surface_is_backed_by_a_declared_dependency() {
    artifact_app_laws::assert_surface_dependencies_declared(&test_bundle().manifest);
}

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: playground's own two native
/// surfaces are registered first (right after `.artifact(...)`), then the six foreign plugins'
/// surfaces in their preserved order — gis's own W2 packet (P7) has landed, so `gis2d-play` is now
/// `s.gis.gismap@1/*#editor` (coordinator follow-up, done once gis's own report confirmed the new
/// module path, per that packet's ground truth). W4-FIX: `sourcing`/`process` were still on the
/// deleted `apps::`/`document_app` path (`s.demonstrator.playground@1/*#editor`-style ids never
/// applied to them) — now `.editor::<E>()` + `.viewer::<V>()` like every other foreign plugin here,
/// so each contributes TWO surfaces instead of one (8 foreign surfaces total, from 6 plugins).
#[test]
fn bundle_registers_its_own_and_the_six_foreign_demonstrator_surfaces() {
    let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
    assert_eq!(
        ids,
        vec![
            "s.demonstrator.playground@1/*#editor",
            "s.demonstrator.playground@1/*#viewer",
            "s.procedural.generation3d@1/*#editor",
            "s.cad.cad@1/*#editor",
            "s.puzzle.puzzle3d@1/*#editor",
            "s.sourcing.curation@1/*#editor",
            "s.sourcing.curation@1/*#viewer",
            "s.process.process3d@1/*#editor",
            "s.process.process3d@1/*#viewer",
            "s.gis.gismap@1/*#editor",
        ]
    );
}

#[test]
fn every_surface_declares_a_document_schema() {
    for app in test_bundle().manifest.apps {
        assert!(!app.io.document_schema.is_empty(), "app {} declares no document schema", app.id);
    }
}

#[test]
fn contribution_consumers_declare_the_hidden_app_command() {
    let consumers: Vec<String> = test_bundle().manifest.apps.iter().filter(|app| app.commands.iter().any(|command| command.id == "setContributions")).map(|app| app.id.clone()).collect();
    assert_eq!(consumers, vec!["s.cad.cad@1/*#editor", "s.sourcing.curation@1/*#editor", "s.process.process3d@1/*#editor"]);
    for app in test_bundle().manifest.apps {
        if let Some(command) = app.commands.iter().find(|command| command.id == "setContributions") {
            assert!(!command.in_palette, "host catalogue command leaked into {}'s palette", app.id);
            assert_eq!(command.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), vec!["json"]);
            assert_eq!(command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "host catalogue command in {} must be admitted as migrated interactive work", app.id);
        }
    }
    let procedural = test_bundle().manifest.apps.into_iter().find(|app| app.id == "s.procedural.generation3d@1/*#editor").expect("procedural surface");
    let tick = procedural.commands.iter().find(|command| command.id == "flowEvalTick").expect("recursive evaluation command");
    assert!(!tick.in_palette);
    assert!(tick.args.is_empty());
}

fn assert_tree_reconciles(tree: ComponentTree, generation: u64, label: &str) {
    assert!(!tree.root.key.is_empty(), "{label} must contain an authored root");
    let mut producer = semio_framework_ui_runtime::ComponentTreeProducer::try_new(tree.root, generation).expect("nonzero aggregate tree generation");
    for _ in 0..65_536 {
        match producer.step(generation, false, false) {
            semio_framework_ui_runtime::ComponentTreeProducerStep::MoreWork => {}
            semio_framework_ui_runtime::ComponentTreeProducerStep::Complete => {
                assert!(producer.take_complete().is_some(), "completed {label} tree transfers its exact owner");
                return;
            }
            semio_framework_ui_runtime::ComponentTreeProducerStep::Fault(fault) => panic!("{label} tree must enter retained reconciliation: {fault:?}"),
        }
    }
    panic!("{label} tree producer did not settle within its fixed bound");
}

#[semio_framework_async_macros::async_test]
async fn aggregate_runtime_renders_every_demonstrator_window() {
    let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::<DemonstratorApps>::new();
    semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, test_bundle());
    let apps: &[(&str, &[&str])] = &[
        ("s.procedural.generation3d@1/*#editor", &["procedural.play.main", "procedural.play.preview", "procedural.play.generations", "procedural.play.generate-form", "procedural.play.generate-preview"]),
        ("s.cad.cad@1/*#editor", &["cad.play.shape", "cad.play.building", "cad.play.energy", "cad.play.structure-classic"]),
        ("s.puzzle.puzzle3d@1/*#editor", &["puzzle3d.play.composite"]),
        ("s.sourcing.curation@1/*#editor", &["sourcing.pool", "sourcing.curated", "sourcing.preview", "sourcing.grid"]),
        ("s.process.process3d@1/*#editor", &["process.play.main"]),
        ("s.gis.gismap@1/*#editor", &["gis2d.play.composite"]),
    ];
    let mut generation = 1_u64;
    for (app_index, (app_id, body_keys)) in apps.iter().enumerate() {
        let instance_id = u32::try_from(app_index + 1).expect("six aggregate app instances");
        semio_framework_plugin::plugin_runtime::plugin_create_app_with_id(&runtime, instance_id, app_id).await.unwrap_or_else(|fault| panic!("aggregate app {app_id} opens: {fault:?}"));
        for body_key in *body_keys {
            let tree = semio_framework_plugin::plugin_runtime::plugin_render(&runtime, instance_id, body_key, "{}").await.unwrap_or_else(|fault| panic!("aggregate body {body_key} renders: {fault:?}"));
            assert_tree_reconciles(tree, generation, body_key);
            generation += 1;
        }
    }
}
