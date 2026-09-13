use super::*;
use crate::editor::architect::modes::edit::windows::{adjacency, graph, report};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn block_on_architect_windows<F: std::future::Future>(mut future: std::pin::Pin<Box<F>>) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn architect_window_ownership_matches_the_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).expect("neutral Architect window fixture");
    let app_config = crate::editor::architect::config::ArchitectConfig::default();
    assert_eq!(<crate::editor::architect::config::ArchitectConfig as ArtifactDsl>::envelope_id(), "architect.config");
    assert_eq!(crate::editor::architect::config::ArchitectConfig::parse_dsl(&app_config.print_dsl()).expect("Architect app-config DSL"), app_config);
    assert_eq!(crate::editor::architect::config::ArchitectConfig::decode_pack(&app_config.encode_pack()).expect("Architect app-config Pack"), app_config);
    let app_schema = crate::editor::architect::config::schema::app_schema_descriptor();
    assert_eq!(app_schema.id, "s.architect.architect");
    assert!(app_schema.config.rust.contains("s.architect.architect.config"));
    assert!(app_schema.config.typescript.contains("interface ArchitectConfig"));
    assert!(app_schema.config.graphql.contains("type ArchitectConfig"));
    assert!(app_schema.config.json_schema.contains("/app/architect/architect/config/schema.json"));
    assert!(app_schema.config.proto.contains("package semio.app.architect.architect;"));
    let register_base: ArchitectRegisterWindowConfig = dsl::json::from_json_str(&fixture["windows"]["register"]["left"]["config"].to_string()).expect("Register base");
    let register_expected: ArchitectRegisterWindowConfig = dsl::json::from_json_str(&fixture["expectedLeft"]["register"].to_string()).expect("Register expected");
    let register_mutation: ArchitectRegisterWindowConfigMutation = dsl::json::from_json_str(&fixture["leftMutations"]["register"].to_string()).expect("Register mutation");
    let register_after = register_mutation.diff(&register_base).diff().apply(&register_base).expect("Register diff");
    assert_eq!(register_after, register_expected);
    assert_eq!(register_mutation.inverse(&register_base)[0].diff(&register_after).diff().apply(&register_after).expect("Register inverse"), register_base);
    assert_eq!(ArchitectRegisterWindowConfig::parse_dsl(&register_after.print_dsl()).expect("Register DSL"), register_after);
    assert_eq!(ArchitectRegisterWindowConfig::decode_pack(&register_after.encode_pack()).expect("Register Pack"), register_after);
    assert_eq!(ArchitectRegisterWindowConfigMutation::parse_op(&register_mutation.print_op()).expect("Register text op"), register_mutation);
    assert_eq!(ArchitectRegisterWindowConfigMutation::decode_op(&register_mutation.encode_op().expect("Register binary op")).expect("Register binary decode"), register_mutation);

    let adjacency_base: adjacency::config::ArchitectAdjacencyWindowConfig = dsl::json::from_json_str(&fixture["windows"]["adjacency"]["left"]["config"].to_string()).expect("Adjacency base");
    let adjacency_expected: adjacency::config::ArchitectAdjacencyWindowConfig = dsl::json::from_json_str(&fixture["expectedLeft"]["adjacency"].to_string()).expect("Adjacency expected");
    let adjacency_mutation: adjacency::config::ArchitectAdjacencyWindowConfigMutation = dsl::json::from_json_str(&fixture["leftMutations"]["adjacency"].to_string()).expect("Adjacency mutation");
    let adjacency_after = adjacency_mutation.diff(&adjacency_base).diff().apply(&adjacency_base).expect("Adjacency diff");
    assert_eq!(adjacency_after, adjacency_expected);
    assert_eq!(adjacency_mutation.inverse(&adjacency_base)[0].diff(&adjacency_after).diff().apply(&adjacency_after).expect("Adjacency inverse"), adjacency_base);
    assert_eq!(adjacency::config::ArchitectAdjacencyWindowConfig::parse_dsl(&adjacency_after.print_dsl()).expect("Adjacency DSL"), adjacency_after);
    assert_eq!(adjacency::config::ArchitectAdjacencyWindowConfig::decode_pack(&adjacency_after.encode_pack()).expect("Adjacency Pack"), adjacency_after);
    assert_eq!(adjacency::config::ArchitectAdjacencyWindowConfigMutation::parse_op(&adjacency_mutation.print_op()).expect("Adjacency text op"), adjacency_mutation);
    assert_eq!(adjacency::config::ArchitectAdjacencyWindowConfigMutation::decode_op(&adjacency_mutation.encode_op().expect("Adjacency binary op")).expect("Adjacency binary decode"), adjacency_mutation);

    let graph_base: graph::config::ArchitectGraphWindowConfig = dsl::json::from_json_str(&fixture["windows"]["graph"]["left"]["config"].to_string()).expect("Graph base");
    let graph_expected: graph::config::ArchitectGraphWindowConfig = dsl::json::from_json_str(&fixture["expectedLeft"]["graph"].to_string()).expect("Graph expected");
    let graph_mutation: graph::config::ArchitectGraphWindowConfigMutation = dsl::json::from_json_str(&fixture["leftMutations"]["graph"].to_string()).expect("Graph mutation");
    let graph_after = graph_mutation.diff(&graph_base).diff().apply(&graph_base).expect("Graph diff");
    assert_eq!(graph_after, graph_expected);
    assert_eq!(graph_mutation.inverse(&graph_base)[0].diff(&graph_after).diff().apply(&graph_after).expect("Graph inverse"), graph_base);
    assert_eq!(graph::config::ArchitectGraphWindowConfig::parse_dsl(&graph_after.print_dsl()).expect("Graph DSL"), graph_after);
    assert_eq!(graph::config::ArchitectGraphWindowConfig::decode_pack(&graph_after.encode_pack()).expect("Graph Pack"), graph_after);
    assert_eq!(graph::config::ArchitectGraphWindowConfigMutation::parse_op(&graph_mutation.print_op()).expect("Graph text op"), graph_mutation);
    assert_eq!(graph::config::ArchitectGraphWindowConfigMutation::decode_op(&graph_mutation.encode_op().expect("Graph binary op")).expect("Graph binary decode"), graph_mutation);

    let report_base: report::config::ArchitectReportWindowConfig = dsl::json::from_json_str(&fixture["windows"]["report"]["left"]["config"].to_string()).expect("Report base");
    let report_expected: report::config::ArchitectReportWindowConfig = dsl::json::from_json_str(&fixture["expectedLeft"]["report"].to_string()).expect("Report expected");
    let report_mutation: report::config::ArchitectReportWindowConfigMutation = dsl::json::from_json_str(&fixture["leftMutations"]["report"].to_string()).expect("Report mutation");
    let report_after = report_mutation.diff(&report_base).diff().apply(&report_base).expect("Report diff");
    assert_eq!(report_after, report_expected);
    assert_eq!(report_mutation.inverse(&report_base)[0].diff(&report_after).diff().apply(&report_after).expect("Report inverse"), report_base);
    assert_eq!(report::config::ArchitectReportWindowConfig::parse_dsl(&report_after.print_dsl()).expect("Report DSL"), report_after);
    assert_eq!(report::config::ArchitectReportWindowConfig::decode_pack(&report_after.encode_pack()).expect("Report Pack"), report_after);
    assert_eq!(report::config::ArchitectReportWindowConfigMutation::parse_op(&report_mutation.print_op()).expect("Report text op"), report_mutation);
    assert_eq!(report::config::ArchitectReportWindowConfigMutation::decode_op(&report_mutation.encode_op().expect("Report binary op")).expect("Report binary decode"), report_mutation);
    eprintln!("[DEBUG] Architect Register, Adjacency, Graph, and Report configs matched the neutral fixture, inverse, DSL, Pack, text-op, and binary-op laws");
}

#[test]
fn architect_window_ownership_interactive_classification_matches_retained_owners() {
    use crate::editor::architect::{create_architect_app, ArchitectPlayApp, ArchitectWindowCommandJobFactory, ARCHITECT_WINDOW_TOOL_IDS};
    use semio_framework_plugin::{ArtifactEditor, ArtifactOwnedToolJobFactory, InteractiveJobClassification};

    let definition = create_architect_app();
    let actions = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).collect::<Vec<_>>();
    let batch_expected = [
        "addElement",
        "addRegisterItem",
        "applyTemplate",
        "exportProgram",
        "exportRegistersCsv",
        "importProgram",
        "importRegistersCsv",
        "nodeGraphEdit",
        "patchRegisterItem",
        "removeElement",
        "removeRegisterItem",
        "runAnalysis",
        "runReport",
        "runValidation",
        "search",
        "setAdjacencyField",
        "setAdjacencyKind",
    ]
    .into_iter()
    .collect::<std::collections::BTreeSet<_>>();
    let retained = ARCHITECT_WINDOW_TOOL_IDS.iter().copied().collect::<std::collections::BTreeSet<_>>();
    let declared = retained.union(&batch_expected).copied().collect::<std::collections::BTreeSet<_>>();
    let domain_actions = actions.iter().filter(|action| declared.contains(action.id.as_str())).collect::<Vec<_>>();
    assert_eq!(domain_actions.iter().map(|action| action.id.as_str()).collect::<std::collections::BTreeSet<_>>(), declared);
    let migrated = actions
        .iter()
        .filter(|action| declared.contains(action.id.as_str()))
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
        .map(|action| action.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let proofs = <ArchitectPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().into_iter().map(|proof| proof.tool_id()).collect::<std::collections::BTreeSet<_>>();
    let publications = <ArchitectWindowCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(migrated, retained);
    assert_eq!(proofs, retained);
    assert_eq!(publications, retained);
    let batch = actions
        .iter()
        .filter(|action| declared.contains(action.id.as_str()))
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::BatchOnlyPendingRewrite)
        .map(|action| action.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(batch, batch_expected);
    assert!(domain_actions.iter().all(|action| action.semantics.execution.interactive_job != InteractiveJobClassification::Unclassified));
    eprintln!("[DEBUG] Architect interactive catalog has three bounded retained WindowConfig actions and seventeen truthful batch-only pending rewrites");
}

#[test]
fn architect_window_ownership_report_handler_preserves_exact_invocation_identity() {
    use crate::editor::architect::commands::analysis::run_report;
    use crate::op::ProgramMutation;
    use semio_framework_plugin::{ArtifactView, HistoryView, ViewModel, ViewWindowInstance, WindowConfigOwner};

    fn authored_id(emit: &semio_framework_plugin::Emit<crate::op::ProgramMutation, crate::editor::architect::config::ArchitectConfigMutation>) -> crate::EntityId {
        let Some(ProgramMutation::CreateReportRecord(payload)) = emit.artifact_mutations.first() else { panic!("one authored ReportRecord") };
        assert_eq!(emit.artifact_mutations.len(), 1);
        payload.report_record.header.id.clone()
    }

    let program = crate::sample_plugin();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&program, &history);
    let all = ViewModel {
        window_instances: [
            ("architect-report-left", report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID),
            ("architect-report-right", report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID),
            ("architect-graph-left", graph::config::ArchitectGraphWindowConfigOwner::WINDOW_KIND_ID),
        ]
        .into_iter()
        .map(|(id, window_kind_id)| ViewWindowInstance { id: id.into(), window_kind_id: window_kind_id.into() })
        .collect(),
        ..Default::default()
    };
    let left = all.for_window_instance("architect-report-left").expect("left Report");
    let right = all.for_window_instance("architect-report-right").expect("right Report");
    let graph = all.for_window_instance("architect-graph-left").expect("Graph");
    let payload = run_report::RunReport { report_kind: "executiveSummary".into() };

    let unscoped = run_report::handle_with_view(&payload, &doc, None).expect("unscoped Report authoring");
    let unscoped_id = authored_id(&unscoped);
    assert!(unscoped.window_config_mutations.is_empty());
    let wrong_kind = run_report::handle_with_view(&payload, &doc, Some(&graph)).expect("Graph-scoped Report authoring");
    let wrong_kind_id = authored_id(&wrong_kind);
    assert!(wrong_kind.window_config_mutations.is_empty());

    let stale = ViewModel { window_id: Some("architect-report-gone".into()), window_instances: all.window_instances.clone(), ..Default::default() };
    assert!(run_report::handle_with_view(&payload, &doc, Some(&stale)).is_err());
    let right_emit = run_report::handle_with_view(&payload, &doc, Some(&right)).expect("right Report authoring");
    let right_id = authored_id(&right_emit);
    assert_eq!(right_emit.window_config_mutations.len(), 1);
    assert_eq!(right_emit.window_config_mutations[0].window_id(), "architect-report-right");
    assert_eq!(right_emit.window_config_mutations[0].window_kind_id(), report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID);
    let left_emit = run_report::handle_with_view(&payload, &doc, Some(&left)).expect("left Report authoring");
    let left_id = authored_id(&left_emit);
    assert_eq!(left_emit.window_config_mutations.len(), 1);
    assert_eq!(left_emit.window_config_mutations[0].window_id(), "architect-report-left");
    assert_eq!(left_emit.window_config_mutations[0].window_kind_id(), report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID);
    assert_eq!([unscoped_id, wrong_kind_id, right_id, left_id].into_iter().collect::<std::collections::HashSet<_>>().len(), 4);
    eprintln!("[DEBUG] Architect Report handler authored exact records without a window and from Graph, rejected stale identity, and addressed only each exact invoking Report window");
}

#[test]
fn architect_window_ownership_runtime_isolates_renders_reloads_and_closes() {
    std::thread::Builder::new()
        .name("architect-window-ownership-law".into())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            eprintln!("[DEBUG] Architect window ownership runtime entered the 2 MiB thread");
            block_on_architect_windows(Box::pin(async {
                use crate::editor::architect::commands::adjacency::set_adjacency_filter;
                use crate::editor::architect::commands::graph::node_graph_viewport;
                use crate::editor::architect::commands::register::select_register;
                use crate::editor::architect::{create_architect_app, ArchitectCommand, ArchitectPlayApp};
                use semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt;
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

                eprintln!("[DEBUG] Architect window ownership runtime began its heap-pinned future");

                type ArchitectApp = VcsArtifactApp<EditorApp<ArchitectPlayApp>>;

                async fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand, view: Option<&ViewModel>) -> Result<TypedOperationFixtureReceipt, String> {
                    let meta = ActionMeta { instance_id: 93, view_state: view.cloned(), ..artifact_app_laws::meta("architect-window-ownership") };
                    app.dispatch_typed(command, &meta).await.map_err(|error| format!("{error:?}"))?;
                    artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{error:?}"))
                }

                async fn register_config(app: &mut ArchitectApp, view: &ViewModel) -> Result<ArchitectRegisterWindowConfig, String> {
                    artifact_app_laws::capture_fixture_window_config::<ArchitectRegisterWindowConfigOwner, _, _>(app, view)
                        .await
                        .map_err(|error| format!("{error:?}"))?
                        .ok_or_else(|| "missing Architect Register window config".into())
                }

                async fn adjacency_config(app: &mut ArchitectApp, view: &ViewModel) -> Result<adjacency::config::ArchitectAdjacencyWindowConfig, String> {
                    artifact_app_laws::capture_fixture_window_config::<adjacency::config::ArchitectAdjacencyWindowConfigOwner, _, _>(app, view)
                        .await
                        .map_err(|error| format!("{error:?}"))?
                        .ok_or_else(|| "missing Architect Adjacency window config".into())
                }

                async fn graph_config(app: &mut ArchitectApp, view: &ViewModel) -> Result<graph::config::ArchitectGraphWindowConfig, String> {
                    artifact_app_laws::capture_fixture_window_config::<graph::config::ArchitectGraphWindowConfigOwner, _, _>(app, view)
                        .await
                        .map_err(|error| format!("{error:?}"))?
                        .ok_or_else(|| "missing Architect Graph window config".into())
                }

                async fn report_config(app: &mut ArchitectApp, view: &ViewModel) -> Result<report::config::ArchitectReportWindowConfig, String> {
                    artifact_app_laws::capture_fixture_window_config::<report::config::ArchitectReportWindowConfigOwner, _, _>(app, view)
                        .await
                        .map_err(|error| format!("{error:?}"))?
                        .ok_or_else(|| "missing Architect Report window config".into())
                }

                async fn render(app: &mut ArchitectApp, body_key: &str, view: &ViewModel) -> Result<String, String> {
                    let tree = app.render(body_key, None, view).await.map_err(|error| format!("{error:?}"))?;
                    artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)
                }

                let manifest = || App { definition: create_architect_app(), examples: Vec::new() };
                let all = ViewModel {
                    window_instances: [
                        ("architect-register-left", ArchitectRegisterWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-register-right", ArchitectRegisterWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-adjacency-left", adjacency::config::ArchitectAdjacencyWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-adjacency-right", adjacency::config::ArchitectAdjacencyWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-graph-left", graph::config::ArchitectGraphWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-graph-right", graph::config::ArchitectGraphWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-report-left", report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID),
                        ("architect-report-right", report::config::ArchitectReportWindowConfigOwner::WINDOW_KIND_ID),
                    ]
                    .into_iter()
                    .map(|(id, window_kind_id)| ViewWindowInstance { id: id.into(), window_kind_id: window_kind_id.into() })
                    .collect(),
                    ..Default::default()
                };
                let register_left = all.for_window_instance("architect-register-left").expect("left Register");
                let register_right = all.for_window_instance("architect-register-right").expect("right Register");
                let adjacency_left = all.for_window_instance("architect-adjacency-left").expect("left Adjacency");
                let adjacency_right = all.for_window_instance("architect-adjacency-right").expect("right Adjacency");
                let graph_left = all.for_window_instance("architect-graph-left").expect("left Graph");
                let graph_right = all.for_window_instance("architect-graph-right").expect("right Graph");
                let report_left = all.for_window_instance("architect-report-left").expect("left Report");
                let report_right = all.for_window_instance("architect-report-right").expect("right Report");
                eprintln!("[DEBUG] Architect window ownership runtime is constructing the first registered app");
                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<ArchitectPlayApp>>(manifest).await);
                eprintln!("[DEBUG] Architect window ownership runtime constructed the first registered app");
                app.bind_instance_id(93).await;
                let outcome: Result<(), String> = async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    let register_receipt = dispatch(&mut app, ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "requirements".into() }), Some(&register_left)).await?;
                    let adjacency_receipt = dispatch(
                        &mut app,
                        ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: Some("preferred".into()) }),
                        Some(&adjacency_left),
                    )
                    .await?;
                    let graph_receipt = dispatch(
                        &mut app,
                        ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: -20.0, y: 12.5, zoom: 1.75 } }),
                        Some(&graph_left),
                    )
                    .await?;
                    for receipt in [&register_receipt, &adjacency_receipt, &graph_receipt] {
                        if receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count() != 1 {
                            return Err("Architect exact window command did not publish one WindowConfig lane".into());
                        }
                    }
                    if register_config(&mut app, &register_left).await?.active_register != "requirements"
                        || register_config(&mut app, &register_right).await? != ArchitectRegisterWindowConfig::default()
                        || adjacency_config(&mut app, &adjacency_left).await?.adjacency_kind_filter != Some(crate::registers::AdjacencyKind::Preferred)
                        || adjacency_config(&mut app, &adjacency_right).await? != adjacency::config::ArchitectAdjacencyWindowConfig::default()
                        || graph_config(&mut app, &graph_left).await?.viewport != (semio_framework_os_kernel::Viewport2d { x: -20.0, y: 12.5, zoom: 1.75 })
                        || graph_config(&mut app, &graph_right).await? != graph::config::ArchitectGraphWindowConfig::default()
                    {
                        return Err("Architect Register, Adjacency, or Graph config crossed exact same-kind windows".into());
                    }
                    let document_after_window_commands = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after_window_commands.pack || document_before.spr != document_after_window_commands.spr {
                        return Err("Architect exact window commands changed document bytes".into());
                    }

                    let rendered_register = render(&mut app, crate::editor::architect::modes::edit::windows::register::ARCHITECT_BODY_REGISTER, &register_left).await?;
                    let rendered_adjacency = render(&mut app, adjacency::ARCHITECT_BODY_ADJACENCY, &adjacency_left).await?;
                    let rendered_graph = render(&mut app, graph::ARCHITECT_BODY_GRAPH, &graph_left).await?;
                    let rendered_report = render(&mut app, report::ARCHITECT_BODY_REPORT, &report_left).await?;
                    if rendered_register.is_empty() || rendered_adjacency.is_empty() || rendered_graph.is_empty() || !rendered_report.contains("Generate a report in this window") {
                        return Err("Architect exact windows did not render their owned state".into());
                    }
                    let app_config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if app_config_before.pack != app_config_after.pack || app_config_before.spr != app_config_after.spr {
                        return Err("Architect exact window and report commands changed app cache bytes".into());
                    }

                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<ArchitectPlayApp>>(manifest).await);
                    reopened.bind_instance_id(94).await;
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let restored_left_register = register_config(&mut reopened, &register_left).await?;
                    let restored_right_register = register_config(&mut reopened, &register_right).await?;
                    let restored_left_adjacency = adjacency_config(&mut reopened, &adjacency_left).await?;
                    let restored_right_adjacency = adjacency_config(&mut reopened, &adjacency_right).await?;
                    let restored_left_graph = graph_config(&mut reopened, &graph_left).await?;
                    let restored_right_graph = graph_config(&mut reopened, &graph_right).await?;
                    let restored_left_report = report_config(&mut reopened, &report_left).await?;
                    let restored_right_report = report_config(&mut reopened, &report_right).await?;
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                    if restored_left_register.active_register != "requirements"
                        || restored_right_register != ArchitectRegisterWindowConfig::default()
                        || restored_left_adjacency.adjacency_kind_filter != Some(crate::registers::AdjacencyKind::Preferred)
                        || restored_right_adjacency != adjacency::config::ArchitectAdjacencyWindowConfig::default()
                        || restored_left_graph.viewport != (semio_framework_os_kernel::Viewport2d { x: -20.0, y: 12.5, zoom: 1.75 })
                        || restored_right_graph != graph::config::ArchitectGraphWindowConfig::default()
                        || restored_left_report != report::config::ArchitectReportWindowConfig::default()
                        || restored_right_report != report::config::ArchitectReportWindowConfig::default()
                    {
                        return Err("Architect exact window configs changed during reopen".into());
                    }
                    let stale = ViewModel { window_id: Some("architect-register-gone".into()), window_instances: all.window_instances.clone(), ..Default::default() };
                    if addressed(&stale, "risks".into()).is_ok() {
                        return Err("Architect Register accepted stale window identity".into());
                    }
                    if addressed(&graph_left, "risks".into()).is_ok() {
                        return Err("Architect Register accepted a different window kind".into());
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = &outcome {
                    eprintln!("[DEBUG] Architect exact-window runtime failure before close: {error}");
                }
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                outcome.expect("Architect exact-window ownership runtime law");
                eprintln!("[DEBUG] Architect runtime dispatched three bounded retained window commands, isolated eight exact windows, rendered all four owner kinds, restored every owner pack, preserved document and app-cache bytes, and closed terminal-empty");
            }))
        })
        .expect("spawn Architect window ownership law")
        .join()
        .expect("Architect window ownership law thread");
}
