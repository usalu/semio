//! 🎛️ Writer play app editor and constructor: the writer play app with its artifact editor implementation and the writer app constructor with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️WriterPlayApp
/// 🧪️ Writer editor runtime; document state stays in the artifact and concrete view state stays in
/// its exact main-window config/transient owners.
#[derive(Default)]
pub struct WriterPlayApp;

impl ArtifactEditor for WriterPlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🧬️ The loaded-parent child projection, read straight off the snapshot's own `#[child]` fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("writer.child-projection"), error.to_string()))
    }

    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = WriterCommand;

    const DIALECT: Dialect = crate::WRITER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;

    /// ⌨️ Writer's typing algebra (design §13.2): a typing run of the prose editor is ONE net `splice-text` — every typed splice
    /// composes into it (`TextSplice::then`), a run that erased what it typed is empty, a caret that jumped away splits; a
    /// whole-text delivery (`textEdit`) replaces the run's text.
    fn typing_fold(net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_plugin::TypingFold<Self::Mutation> {
        use semio_framework_plugin::{TextSpliceComposition, TypingFold, TEXT_SPLICE_CONTEXT_SCALARS};
        match (net, next) {
            ([WriterMutation::SpliceText(run)], [WriterMutation::SpliceText(typed)]) => match run.splice().then(&typed.splice(), TEXT_SPLICE_CONTEXT_SCALARS) {
                TextSpliceComposition::Composed(splice) => TypingFold::Net(vec![crate::schema::mutations::splice_text(splice)]),
                TextSpliceComposition::Cancelled => TypingFold::Net(Vec::new()),
                TextSpliceComposition::Disjoint => TypingFold::Split,
            },
            ([WriterMutation::EditText(_)], [WriterMutation::EditText(_)]) => TypingFold::Net(next.to_vec()),
            _ => TypingFold::Split,
        }
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_writer_child_pack(snapshot, slot, child_id)
    
})())
}

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(WriterArtifactPreparationFactory::shared())
    }

    fn build_config_store_disposer() -> ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<WriterMainWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<WriterMainWindowTransientOwner>()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<WriterPlayApp>,
        owner_file: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.writer.writer@1/*#editor",
        artifact_schema: "writer.document",
        factory: "WriterCommandJobFactory",
        factory_type: WriterCommandJobFactory,
        contract: ToolExecutionContract::resumable(4_096, 4_096, 1, 64, 2_000, 1, 1),
        tools: [
            "textEdit",
            "setText",
            "setActiveExample",
            "openDocument",
            "loadDocumentJson",
            "formatDocument",
            "commitRename",
            "setCamera",
            "requestCompletions",
            "lintDocument",
            "textSelect",
            "toggleLineNumbers",
            "setEditorSetting",
            "engagementInput",
            "engagementSubmit",
            "textSplice",
                    ]
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::owned::writer_envelope_decode_owner_bundle())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn reference_birth()->Option<fn()->WriterSnapshot>{Some(||{
        crate::schema::empty_writer_snapshot()
    })}

    fn io() -> Option<AppIo> {
        Some(writer_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &WriterCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `WriterCommand` — the bridge the React/wgpu
    /// shells still need because they speak `{action, args}` while dispatch is typed-only. Without it
    /// the trait default refuses EVERY id (`app.command.unsupported`), so the boot `setActiveExample`,
    /// every Actions-pane row and every chrome control died before reaching `handle`.
    /// `setEditorSetting` carries three payload types under one manifest id, so it selects on the
    /// `setting` key the way the `app_commands!` rows split on their wire keyword.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<WriterCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64));
        match action {
            "textEdit" => Ok(WriterCommand::TextEdit(text_edit::TextEdit { text: text_arg(&["text", "value"]).unwrap_or_default() })),
            "setText" => Ok(WriterCommand::SetText(set_text::SetText { text: text_arg(&["text", "value"]).unwrap_or_default() })),
            "openDocument" => Ok(WriterCommand::OpenDocument(open_document::OpenDocument { uri: text_arg(&["uri"]).unwrap_or_default(), text: text_arg(&["text"]).unwrap_or_default() })),
            "loadDocumentJson" => Ok(WriterCommand::LoadDocumentJson(load_document_json::LoadDocumentJson { json: text_arg(&["json", "value"]).unwrap_or_default() })),
            "setActiveExample" => Ok(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| "jack".into()) })),
            "formatDocument" => Ok(WriterCommand::FormatDocument(format_document::FormatDocument {})),
            "commitRename" => Ok(WriterCommand::CommitRename(commit_rename::CommitRename { text: text_arg(&["text", "value"]).unwrap_or_default() })),
            "setCamera" => {
                let value = args.and_then(|value| value.get("camera")).cloned().ok_or_else(|| Fault::from("writer setCamera requires a camera"))?;
                Ok(WriterCommand::SetCamera(set_camera::SetCamera { camera: semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid writer setCamera camera: {error}")))? }))
            }
            "requestCompletions" => Ok(WriterCommand::RequestCompletions(request_completions::RequestCompletions {})),
            "lintDocument" => Ok(WriterCommand::LintDocument(lint_document::LintDocument {})),
            "textSelect" => Ok(WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: number_arg(&["start"]).unwrap_or_default() as usize, end: number_arg(&["end"]).unwrap_or_default() as usize, splice: number_arg(&["splice"]).unwrap_or_default() as u64 })),
            "textSplice" => Ok(WriterCommand::TextSplice(text_splice::TextSplice {
                start: u32::try_from(number_arg(&["start"]).unwrap_or_default() as u64).map_err(|_| Fault::from("writer textSplice start exceeds u32"))?,
                deleted: text_arg(&["deleted"]).unwrap_or_default(),
                insert: text_arg(&["insert"]).unwrap_or_default(),
                before: text_arg(&["before"]).unwrap_or_default(),
                after: text_arg(&["after"]).unwrap_or_default(),
                seq: number_arg(&["seq"]).unwrap_or_default() as u64,
                anchor: number_arg(&["anchor"]).unwrap_or_default() as usize,
                caret: number_arg(&["caret"]).unwrap_or_default() as usize,
            })),
            "toggleLineNumbers" => Ok(WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {})),
            "setEditorSetting" => {
                let value = number_arg(&["value"]).unwrap_or_default() as u32;
                match text_arg(&["setting", "key"]).as_deref().unwrap_or("fontPx") {
                    "lineHeight" | "line-height" => Ok(WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value })),
                    "tabSize" | "tab-size" => Ok(WriterCommand::SetTabSize(set_tab_size::SetTabSize { value })),
                    "fontPx" | "font-px" => Ok(WriterCommand::SetFontPx(set_font_px::SetFontPx { value })),
                    other => Err(Fault::from(format!("writer setEditorSetting has no setting '{other}' (fontPx/lineHeight/tabSize)"))),
                }
            }
            "engagementInput" => Ok(WriterCommand::EngagementInput(engagement_input::EngagementInput { value: text_arg(&["value", "text"]).unwrap_or_default() })),
            "engagementSubmit" => Ok(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: text_arg(&["value", "text"]) })),
            other => Err(Fault::from(format!("writer: unhandled action id {other}"))),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(WriterCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !WRITER_COMMAND_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Writer command does not match its exact registered tool").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, WriterCommand::command_id, MAX_WRITER_COMMAND_RAW_BYTES, MAX_WRITER_COMMAND_WORK_ITEMS, false, |request| WriterRetainedCommandWork::new(request.command.command_id()))
    }

    fn handle(
        command: &WriterCommand,
        doc: &ArtifactView<'_, WriterSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WriterMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `ast` domain: `HierarchyProvider::Topology` from the jack AST's own parent links — see
    /// `writer_ast_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("ast".to_string(), writer_ast_topology(doc.snapshot));
        InteractionTopology { domains }
    
})())
}

    /// 🎞️ `"text:out"` exports the writer document's current text as one "chapter" payload (see
    /// `writer_chapter_payload`) — `playbook`'s `"chapters:in"` is the intended consumer. Falls through
    /// to the default whole-document-pack export for `"artifact:out"` (duplicated inline, not delegated
    /// — Rust traits have no `super` call for an overridden default).
    fn export_media(port: &str, doc: &ArtifactView<'_, WriterSnapshot>) -> Result<Media, MediaError> {
        if port == "text:out" {
            let payload = writer_chapter_payload(doc.snapshot);
            let json = serde_json::to_string(&payload).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json } });
        }
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let labels = writer_play_labels(view_state);
        let window_config = main::config::current(cfg).cloned().unwrap_or_default();
        let window_transient = WriterMainWindowTransient::default();
        let node = match body_key {
            WRITER_PLAY_BODY_MAIN => main::render(document, &window_config, &window_transient),
            WRITER_PLAY_BODY_ARTIFACT => document_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, WRITER_PLAY_BODY_ARTIFACT)),
            WRITER_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            WRITER_PLAY_BODY_INSPECTION => inspection_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, WRITER_PLAY_BODY_INSPECTION)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "writer unknown-body text admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, WriterSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, NoTransient>,
        _interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == WRITER_PLAY_BODY_MAIN {
            let config = main::config::current(cfg).cloned().unwrap_or_default();
            let transient = main::transient::current(transient).cloned().unwrap_or_default();
            let node = main::render(doc.snapshot, &config, &transient)?;
            return Ok(semio_framework_plugin::built_to_component_tree(node));
        }
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        use semio_framework_plugin::{WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus};

        let Some(config) = main::config::current(cfg) else { return HashMap::new() };
        let labels = writer_play_labels(view_state);
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: Some(vec![WindowEngagementOption {
                id: "writer-line-numbers".into(),
                label: Some(labels.line_numbers.into()),
                icon_id: Some("list-ordered".into()),
                pressed: Some(config.editor_settings.show_line_numbers),
                disabled: None,
                action: Some(writer_action("toggleLineNumbers", None)),
            }]),
            input: Some(WindowEngagementInput {
                id: Some("writer-engagement-input".into()),
                value: Some(String::new()),
                placeholder: Some(labels.engagement_placeholder.into()),
                disabled: None,
                on_change: Some(writer_action("engagementInput", None)),
                on_submit: Some(writer_action("engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "writer-editor-mode".into(), text: labels.editor_mode_status.into() }]),
            possible_engagements: Some(vec![
                WindowEngagementPossible { id: "writer-format".into(), label: labels.format.into(), detail: None, action: Some(writer_action("formatDocument", None)) },
                WindowEngagementPossible { id: "writer-lint".into(), label: labels.lint.into(), detail: None, action: Some(writer_action("lintDocument", None)) },
                WindowEngagementPossible { id: "writer-line-numbers".into(), label: labels.line_numbers.into(), detail: None, action: Some(writer_action("toggleLineNumbers", None)) },
            ]),
        };
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        HashMap::from([(window_id, engagement)])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, WriterSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let mut engagements = Self::window_engagements(doc, cfg, view_state);
        let Some(window_id) = view_state.window_id.as_ref() else { return HashMap::new() };
        let Some(window_transient) = main::transient::current(transient) else { return HashMap::new() };
        if let Some(input) = engagements.get_mut(window_id).and_then(|engagement| engagement.input.as_mut()) {
            input.value = Some(window_transient.engagement_input.clone());
        }
        engagements
    }

    fn window_measures(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(config) = main::config::current(cfg) else { return HashMap::new() };
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        HashMap::from([(window_id, main::window_measures(config, writer_play_labels(view_state)))])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let text = request.surface.as_ref().and_then(|surface| surface.text.as_ref());
        writer_context_menu_items(registry, text, view_state)
    }
}
//#endregion 🔖️WriterPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_writer_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::WRITER_DIALECT)
            .document(["semio", "writer"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("writer")
            .mode_def(edit::definition())
            .default_mode_id(edit::WRITER_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .action_with(ActionDefinition::new("formatDocument", LocalizedLabel::native("Format Document", "Dokument formatieren"), ActionKind::Mutation, "typography").with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("lintDocument", LocalizedLabel::native("Lint Document", "Dokument prüfen"), ActionKind::View).with_category("tools"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_with(writer_hidden_operation("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"), "typography"))
            .action_with(writer_hidden_operation("textSplice", LocalizedLabel::native("Type Text", "Text tippen"), "typography"))
            .action_with(ActionDefinition::new("setText", LocalizedLabel::native("Set Text", "Text festlegen"), ActionKind::Mutation, "sparkles").with_category("transform"))
            .action_with(writer_hidden_view("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), "camera"))
            .action_with(writer_hidden_operation("commitRename", LocalizedLabel::native("Commit Rename", "Umbenennung übernehmen"), "sparkles").with_category("transform"))
            .action_with(writer_hidden_operation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), "sparkles"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(writer_hidden_operation("openDocument", LocalizedLabel::native("Open Document", "Dokument öffnen"), "folder-open"))
            .action_with(writer_hidden_operation("loadDocumentJson", LocalizedLabel::native("Load Document JSON", "Dokument-JSON laden"), "sparkles"))
            .action_destructive("loadDocumentJson")
            .action_with(writer_hidden_view("requestCompletions", LocalizedLabel::native("Request Completions", "Vervollständigungen anfordern"), "typography").with_category("tools"))
            .action_with(writer_hidden_view("textSelect", LocalizedLabel::native("Select Editor Text", "Editor-Text auswählen"), "text-cursor"))
            .action_with(writer_hidden_view("toggleLineNumbers", LocalizedLabel::native("Toggle Line Numbers", "Zeilennummern umschalten"), "eye"))
            .action_with(writer_hidden_view("setEditorSetting", LocalizedLabel::native("Set Editor Setting", "Editor-Einstellung festlegen"), "eye"))
            .action_with(writer_hidden_view("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), "hand"))
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("textSplice", InteractiveJobClassification::Migrated)
            .action_interactive_job("setText", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("requestCompletions", InteractiveJobClassification::Migrated)
            .action_interactive_job("lintDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("textSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLineNumbers", InteractiveJobClassification::Migrated)
            .action_interactive_job("setEditorSetting", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("openDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadDocumentJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("formatDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("commitRename", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("jack", LocalizedLabel::native("Jack", "Jack")),
                    ActionArgOption::new("dag.jack", LocalizedLabel::native("Dag Jack", "Dag Jack")),
                ]).default_value(&"jack"),
            ])
            .action_args("setText", vec![ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).required()])
            .action_args("loadDocumentJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Fixture JSON", "Fixture-JSON"))])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .interaction(InteractionDefinition {
                id: WRITER_INTERACTION_AST.into(),
                label: LocalizedLabel::native("AST", "AST"),
                granularities: vec![GranularityDefinition { id: WRITER_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Single, SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(WRITER_PLAY_WINDOW_KIND, vec![InteractionRef::new(WRITER_INTERACTION_AST)])
            .io(writer_io())
            .action_describe("formatDocument", LocalizedLabel::native("Reformats the whole text with the formatter of the document's language; text that is already formatted writes nothing.", "Formatiert den gesamten Text mit dem Formatierer der Dokumentsprache neu; bereits formatierter Text schreibt nichts."))
            .action_describe("lintDocument", LocalizedLabel::native("Checks the text with the linter of the document's language and shows the diagnostics in the editor window; the text is not changed.", "Prüft den Text mit dem Linter der Dokumentsprache und zeigt die Befunde im Editorfenster an; der Text ändert sich nicht."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document with a bundled example (the Jack demo or the DAG Jack example), or with an empty document for any other id.", "Ersetzt das gesamte Dokument durch ein mitgeliefertes Beispiel (die Jack-Demo oder das DAG-Jack-Beispiel), bei jeder anderen Id durch ein leeres Dokument."))
            .action_describe("setText", LocalizedLabel::native("Replaces the document's entire text with the given text; the previous text is gone unless the edit is undone.", "Ersetzt den gesamten Text des Dokuments durch den angegebenen Text; der bisherige Text ist fort, sofern die Änderung nicht rückgängig gemacht wird."))
            .action_describe("commitRename", LocalizedLabel::native("With the caret on a Jack variable, renames every occurrence of it to the given text; otherwise replaces the editor's selected range with that text.", "Steht die Einfügemarke auf einer Jack-Variablen, werden alle ihre Vorkommen in den angegebenen Text umbenannt; sonst ersetzt der Text den ausgewählten Bereich des Editors."))
            .action_describe("openDocument", LocalizedLabel::native("Opens the given text under a URI as the writer document, detecting its language from the content or extension; the current document is replaced.", "Öffnet den angegebenen Text unter einer URI als Writer-Dokument und erkennt die Sprache aus Inhalt oder Dateiendung; das aktuelle Dokument wird ersetzt."))
            .action_describe("loadDocumentJson", LocalizedLabel::native("Loads a test fixture given as JSON as the whole writer document, replacing the current one; invalid JSON changes nothing.", "Lädt eine als JSON übergebene Test-Fixture als gesamtes Writer-Dokument und ersetzt das aktuelle; ungültiges JSON ändert nichts."))
            .action_audience("textEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("textSplice", semio_framework_plugin::CapabilityAudience::Input)
            .action_describe("textSplice", LocalizedLabel::native("Replaces one range of the text as its author saw it; the change lands between the text its author saw around it, even when others edited the text meanwhile.", "Ersetzt einen Textbereich so, wie ihn sein Autor sah; die Änderung landet zwischen dem Text, den der Autor darum herum sah, auch wenn andere den Text inzwischen bearbeitet haben."))
            .action_destructive("setText")
            .action_destructive("openDocument")
            .build_definition()
}
//#endregion 🔖️Manifest
