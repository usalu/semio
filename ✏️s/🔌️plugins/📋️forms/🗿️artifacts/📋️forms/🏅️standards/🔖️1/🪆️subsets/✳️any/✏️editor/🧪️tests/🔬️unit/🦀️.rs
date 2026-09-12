pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type FormsApp = VcsArtifactApp<EditorApp<FormsPlayApp>>;
    
    /// 🧪️ An app instance with its concrete command registry and retained job proofs.
    pub async fn forms_app() -> FormsApp {
        new_app_with_registry::<EditorApp<FormsPlayApp>>(forms_manifest_for_tests).await
    }
    
    /// 🚧️ SDK GAP (w0-f-report Gap 3): `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still take `fn() -> App` (the pre-migration manifest wrapper), unchanged for this ticket —
    /// `create_forms_app` now returns `AppDefinition`, so wrap it in a throwaway `App` (empty examples)
    /// rather than widen the framework test context signature.
    fn forms_manifest_for_tests() -> App {
        App { definition: create_forms_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
    /// `kind` default declared on `addQuestion` materializes host-side.
    pub async fn forms_app_with_registry() -> FormsApp {
        new_app_with_registry::<EditorApp<FormsPlayApp>>(forms_manifest_for_tests).await
    }
    
    pub async fn config(app: &FormsApp) -> FormsConfig {
        let files = app.config_pack().await.expect("config pack");
        store::parse_document_pack::<FormsConfig, FormsConfigMutation>(&files.pack, &files.spr).await.expect("config projection").snapshot
    }
    
    pub fn action_args(value: &serde_json::Value) -> dsl::DslValue {
        dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(&value.to_string()).expect("fixture JSON"))
    }
    
    pub async fn dispatch(app: &mut FormsApp, command: FormsCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut FormsApp, body_key: &str) -> String {
        let view = if body_key == FORMS_PLAY_BODY_TRY {
            ViewModel {
                window_id: Some("forms-try-test".into()),
                window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "forms-try-test".into(), window_kind_id: try_window::FORMS_PLAY_WINDOW_TRY.into() }],
                ..Default::default()
            }
        } else {
            ViewModel::default()
        };
        serde_json::to_string(&app.render(body_key, None, &view).await.expect("render").root).expect("rendered component JSON")
    }
    
    /// 🧩️ A host contribution registering `"buildingComponent"` as an extension question kind rendered
    /// by `forms-module-procedural` — shared by every test exercising the extension-question path.
    pub fn building_component_contributions() -> Vec<ProgramContributionEntry> {
        vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
                "forms.questionKind",
                semio_framework_os_kernel::DslValue::object([
                    ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                    ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                    ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
                ]),
            )),
        }]
    }
    
    /// 🧩️ A standalone `buildingComponent` question, for tests that exercise `render_extension_question`
    /// directly without going through a full document.
    pub fn building_component_question() -> FormQuestion {
        let mut question = add_question::question_shell("geometry".into(), "Geometry".into(), "buildingComponent".into());
        question.fixture_slug = Some("hexagonal-mushroom-column".into());
        question.params = Some(crate::schema::value_to_dsl(&dsl::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
        question
    }
}

use super::*;
use crate::editor::forms::unit_tests::context::{building_component_contributions, building_component_question, forms_app, forms_app_with_registry};
use crate::forms_steps;
use semio_framework_plugin::artifact_app_laws::meta;

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 28, "every FormsCommand row must be covered by every_command()");
}

/// ⚖️ Every generated Forms command has one concrete retained-factory key, proof row, and exact
/// nonempty publication contract in the same declaration order.
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape, ToolJobFactory};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;

    assert_eq!(FormsCommand::TOOL_JOB_IDS, FORMS_RETAINED_TOOL_IDS);
    assert_eq!(<FormsPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), FORMS_RETAINED_TOOL_IDS.len());
    assert_eq!(FormsBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), FORMS_RETAINED_TOOL_IDS.len());
    assert_eq!(forms_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(forms_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);

    let factory = FormsBoundedCommandJobFactory::new("s.forms.forms@1/*#editor");
    let factory_ids: Vec<&str> = factory.keys().iter().map(|key| key.tool_id.as_str()).collect();
    assert_eq!(factory_ids, FORMS_RETAINED_TOOL_IDS);
    for (tool_id, contract) in FORMS_RETAINED_TOOL_IDS.iter().zip(FormsBoundedCommandJobFactory::PUBLICATION_CONTRACTS) {
        assert_eq!(*tool_id, contract.tool_id);
        assert!(!contract.lanes.is_empty(), "tool {tool_id} must declare a publication lane");
        assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1, "HostOnly must be exclusive for tool {tool_id}");
    }
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// kebab-cased command id for most rows, except the documented divergences copied VERBATIM from the
/// pre-migration `forms_protocol::FormsCommand`'s own `#[dsl(key = ..)]` attributes (host-pushed
/// `spec-json`/`active-example` keys — preserving these exactly is what makes the wire format
/// byte-identical across the migration; see TEMPLATE.md §5.1).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setContributions" => "contributions".to_string(),
            "setTryValue" => "try-value".to_string(),
            "setTryValues" => "try-values".to_string(),
            "setSpecJson" => "spec-json".to_string(),
            "setActiveExample" => "active-example".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<FormsCommand> {
    vec![
        FormsCommand::SetTryValue(set_try_value::SetTryValue { key: "q1".into(), value_json: Some("\"Ada\"".into()), ..Default::default() }),
        FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"name":"Ada"}"#.into(), ..Default::default() }),
        FormsCommand::ResetTry(reset_try::ResetTry::default()),
        FormsCommand::PreviousStep(previous_step::PreviousStep::default()),
        FormsCommand::NextStep(next_step::NextStep::default()),
        FormsCommand::Submit(submit::Submit::default()),
        FormsCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        FormsCommand::AddStep(add_step::AddStep {}),
        FormsCommand::PatchStep(patch_step::PatchStep { step_id: "s1".into(), field: "title".into(), value: "Renamed".into() }),
        FormsCommand::RemoveStep(remove_step::RemoveStep { step_id: "s1".into() }),
        FormsCommand::MoveStep(move_step::MoveStep { step_id: "s1".into(), index: 0 }),
        FormsCommand::UpdateForm(update_form::UpdateForm { title: "My Form".into() }),
        FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: Some("s1".into()) }),
        FormsCommand::RemoveQuestion(remove_question::RemoveQuestion { question_id: "q1".into() }),
        FormsCommand::PatchQuestions(patch_questions::PatchQuestions { question_ids: vec!["q1".into(), "q2".into()], field: "required".into(), value_json: "true".into(), param_key: None }),
        FormsCommand::PatchQuestionOptions(patch_question_options::PatchQuestionOptions { question_ids: vec!["q1".into()], option_value: "a".into(), field: "label".into(), value_json: "\"Option A\"".into() }),
        FormsCommand::AddQuestionOption(add_question_option::AddQuestionOption { question_id: "q1".into(), label: "New option".into() }),
        FormsCommand::RemoveQuestionOption(remove_question_option::RemoveQuestionOption { question_id: "q1".into(), option_value: "a".into() }),
        FormsCommand::PatchVectorField(patch_vector_field::PatchVectorField { question_id: "q1".into(), field_key: "x".into(), field: "value".into(), value_json: "1.0".into() }),
        FormsCommand::AddVectorField(add_vector_field::AddVectorField { question_id: "q1".into(), field_key: "w".into() }),
        FormsCommand::RemoveVectorField(remove_vector_field::RemoveVectorField { question_id: "q1".into(), field_key: "w".into() }),
        FormsCommand::MoveQuestion(move_question::MoveQuestion { question_id: "q1".into(), to_step_id: "s2".into(), target_id: Some("q2".into()), position: "before".into(), index: Some(0) }),
        FormsCommand::DropQuestionKind(drop_question_kind::DropQuestionKind { kind: "slider".into(), target_id: "step:s1".into(), drop_position: "inside".into() }),
        FormsCommand::SetSpecJson(set_spec_json::SetSpecJson { json: "{}".into() }),
        FormsCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
        FormsCommand::ExportFixture(export_fixture::ExportFixture {}),
        FormsCommand::SetTryValueStep(set_try_value_step::SetTryValueStep { app_id: "1".into(), document_id: "document".into(), operation_id: "1".into(), generation: 1, cursor: 64, target_index: 128, base_revision: "0".repeat(64), ..Default::default() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = dsl::os_pack::json::to_json_string(&create_forms_app());
    for id in [builder::FORMS_PLAY_WINDOW_BLUEPRINT, try_window::FORMS_PLAY_WINDOW_TRY] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(blueprint::FORMS_PLAY_MODE_BLUEPRINT), "mode missing from the manifest");
    for body in [FORMS_PLAY_BODY_DOCUMENT, FORMS_PLAY_BODY_CATALOGUE, FORMS_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("form.dictionary"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn app_has_blueprint_and_try_windows_only() {
    let definition = create_forms_app();
    assert_eq!(definition.window_kinds.len(), 2);
    assert_eq!(definition.window_kinds[0].id, builder::FORMS_PLAY_WINDOW_BLUEPRINT);
    assert_eq!(definition.window_kinds[1].id, try_window::FORMS_PLAY_WINDOW_TRY);
    assert_eq!(definition.modes[0].id, blueprint::FORMS_PLAY_MODE_BLUEPRINT);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The `fields` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
/// selection, and scoped to the blueprint (builder) window kind.
#[semio_framework_async_macros::async_test]
async fn fields_interaction_domain_is_declared_topology_and_transitive_on_the_blueprint_window() {
    let definition = create_forms_app();
    let fields = definition.interactions.iter().find(|interaction| interaction.id == FORMS_INTERACTION_FIELDS).expect("fields interaction domain declared");
    assert!(matches!(fields.hierarchy, HierarchyProvider::Topology));
    assert!(fields.hover.transitive, "fields hover must be transitive so a hovered step covers its questions");
    assert!(fields.selection.transitive, "fields selection must be transitive so a selected step covers its questions");
    let builder_window = definition.window_kinds.iter().find(|window| window.id == builder::FORMS_PLAY_WINDOW_BLUEPRINT).expect("blueprint window kind declared");
    assert!(builder_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == FORMS_INTERACTION_FIELDS), "blueprint window must reference the fields interaction domain");
}

/// 🌳️ `interaction_topology` walks the document's own step/question nesting into `TopologyNode.parent`
/// links — a step has no parent, every question's parent is its owning step's row id.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_walks_step_nesting_into_parent_links() {
    let document = crate::schema::building_component_spec();
    let config = FormsConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = FormsPlayApp::interaction_topology(&doc, &cfg);
    let fields = topology.domains.get(FORMS_INTERACTION_FIELDS).expect("fields domain present in topology");
    let steps = forms_steps(&document);
    let question_count: usize = steps.iter().map(|step| step.blocks.len()).sum();
    assert!(!steps.is_empty() && question_count > 0, "the building-component fixture must have steps and questions to make this assertion meaningful");
    assert_eq!(fields.ordered.len(), steps.len() + question_count, "topology must cover every step and every question");
}

/// 🌱️ A document with a step but no questions still contributes its (parent-less) section node —
/// only the field-granularity nodes are absent.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_has_a_section_node_and_no_field_nodes_for_a_document_with_no_questions() {
    let document = crate::schema::empty_forms_snapshot();
    let config = FormsConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = FormsPlayApp::interaction_topology(&doc, &cfg);
    let fields = topology.domains.get(FORMS_INTERACTION_FIELDS).expect("fields domain present in topology");
    assert!(!fields.ordered.is_empty(), "the empty document's own single step still contributes a section node");
    assert!(fields.ordered.iter().all(|node| node.granularity == FORMS_INTERACTION_GRANULARITY_SECTION), "an empty document has sections but no fields");
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn add_question_materializes_kind_default() {
    let mut app = forms_app_with_registry().await;
    let steps_before = forms_steps(&app.snapshot().expect("projection")).len();
    assert!(steps_before > 0, "seeded fixture has at least one step to receive the question");
    app.dispatch_typed(FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }), &meta("local")).await.expect("add question");
    let spec = app.snapshot().expect("projection");
    assert!(crate::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "text"), "kind default materialized from the registry");
}

#[semio_framework_async_macros::async_test]
async fn initial_document_seeds_building_component_fixture() {
    let app = forms_app().await;
    let spec = app.snapshot().expect("projection");
    assert!(!crate::schema::flatten_questions(&spec).is_empty());
    assert!(crate::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "buildingComponent"));
}

#[semio_framework_async_macros::async_test]
async fn extension_question_falls_back_without_contribution() {
    let node = render_extension_question(&building_component_question(), &Object::new(), &[], "try", true);
    let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
    assert!(json.contains("Extension unavailable"));
}

#[semio_framework_async_macros::async_test]
async fn extension_question_emits_external_slot_when_contribution_registered() {
    let node = render_extension_question(&building_component_question(), &Object::new(), &building_component_contributions(), "try", true);
    let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
    assert!(json.contains("\"type\":\"extension\""));
    assert!(json.contains("forms-module-procedural"));
}

/// 🗂️ The open `forms.questionKind` topic shape must resolve the extension question.
#[semio_framework_async_macros::async_test]
async fn extension_question_emits_external_slot_when_topic_contribution_registered() {
    let topic_only = vec![ProgramContributionEntry {
        plugin_id: "forms-module-procedural".into(),
        topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
            "forms.questionKind",
            semio_framework_os_kernel::DslValue::object([
                ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
            ]),
        )),
    }];
    let node = render_extension_question(&building_component_question(), &Object::new(), &topic_only, "try", true);
    let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
    assert!(json.contains("\"type\":\"extension\""));
    assert!(json.contains("forms-module-procedural"));
}

/// 🗂️ `catalogue_kinds` must surface topic-contributed kinds.
#[semio_framework_async_macros::async_test]
async fn catalogue_kinds_includes_topic_contributed_kinds() {
    let contributions = vec![ProgramContributionEntry {
        plugin_id: "forms-module-procedural".into(),
        topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
            "forms.questionKind",
            semio_framework_os_kernel::DslValue::object([
                ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
            ]),
        )),
    }];
    let labels = forms_play_labels(&semio_framework_plugin::ViewModel::default());
    let kinds = catalogue_kinds(&contributions, labels);
    assert!(kinds.iter().any(|(kind, label, _)| kind == "buildingComponent" && label == "Building Component"));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::forms::unit_tests::context::render;
    let mut app = forms_app().await;
    assert!(render(&mut app, "forms.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits() {
    semio_framework_plugin::artifact_app_laws::assert_two_instances_converge::<EditorApp<FormsPlayApp>, (usize, usize)>(
        "mem://forms-convergence",
        FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }),
        FormsCommand::AddStep(add_step::AddStep {}),
        |app| {
            let projection = app.snapshot().expect("materialize projection");
            let steps = forms_steps(&projection);
            (steps.len(), steps[0].blocks.len())
        },
    )
    .await;
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
#[semio_framework_async_macros::async_test]
async fn export_media_dictionary_out_returns_default_values() {
    let app = forms_app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = <FormsPlayApp as ArtifactEditor>::export_media("dictionary:out", &doc).expect("export dictionary:out");
    assert_eq!(media.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "form.dictionary");
    let parsed: Value = dsl::os_pack::json::parse(&json).expect("valid json dictionary");
    assert!(parsed.as_object().is_some());
}

#[semio_framework_async_macros::async_test]
async fn export_media_document_out_round_trips_through_pack() {
    let app = forms_app().await;
    let document = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = <FormsPlayApp as ArtifactEditor>::export_media("document:out", &doc).expect("export document:out");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, FORMS_DOCUMENT_SCHEMA);
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
    let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    assert_eq!(decoded, document);
}

#[semio_framework_async_macros::async_test]
async fn forms_io_exposes_dictionary_out_port() {
    let io = FormsPlayApp::io().expect("forms declares io");
    assert!(io.ports.iter().any(|port| port.id == "dictionary:out"));
}

/// 🔌️ Relocated from the deleted artifact `⚙️engine`'s own `forms_io()` unit test (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — asserts the full port shape, not just
/// presence, alongside `forms_io_exposes_dictionary_out_port` above.
#[semio_framework_async_macros::async_test]
async fn forms_io_declares_dictionary_out_port() {
    let io = forms_io();
    assert_eq!(io.document_schema, FORMS_DOCUMENT_SCHEMA);
    let dictionary_out = io.ports.iter().find(|port| port.id == "dictionary:out").expect("dictionary:out declared");
    assert_eq!(dictionary_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(dictionary_out.kind_id.as_deref(), Some("form.dictionary"));
    assert_eq!(dictionary_out.multiplicity, semio_framework::PortMultiplicity::Many);
    let all_ports = io.all_ports().await;
    assert!(all_ports.iter().any(|port| port.id == "document:in"));
    assert!(all_ports.iter().any(|port| port.id == "document:out"));
}
//#endregion 🔖️MediaPorts
