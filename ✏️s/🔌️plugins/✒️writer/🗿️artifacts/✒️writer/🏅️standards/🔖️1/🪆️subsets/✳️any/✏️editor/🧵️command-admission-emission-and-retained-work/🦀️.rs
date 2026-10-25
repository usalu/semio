//! 🧵️ Command admission, emission and retained work: the hidden writer operation and view, the AST topology, the context menu items, the command tool ids and capacities, the command admission and emission, the retained command work with its job factory and the artifact store preparation. See [`super`].

use super::*;

//#region 🔖️Constants
/// 🙈️ An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
pub(super) fn writer_hidden_operation(id: &str, label: LocalizedLabel, icon_id: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, ActionKind::Mutation, icon_id) }
}

/// 🙈️ An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
pub(super) fn writer_hidden_view(id: &str, label: LocalizedLabel, icon_id: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, ActionKind::View, icon_id) }
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ `ast` domain topology from the jack AST's own parent links — `HierarchyProvider::Topology`, so
/// this is the framework's ONLY source of truth for that domain's membership/hierarchy (selection
/// pruning after a document edit, `selectAll`, range-select, transitive descendant-closure
/// hover/selection). Empty for a non-jack document (nothing to select) or a document with no AST.
pub(super) fn writer_ast_topology(document: &WriterSnapshot) -> DomainTopology {
    use crate::schema::{parse_jack_ast, JackAstNode};

    fn visit(node: &JackAstNode, parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        out.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: parent.map(str::to_string) });
        for child in &node.children {
            visit(child, Some(node.id.as_str()), out);
        }
    }

    let mut ordered = Vec::new();
    if document.language_id == "jack" {
        let root = parse_jack_ast(&writer_text(document));
        visit(&root, None, &mut ordered);
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️ContextMenu
/// 🖱️ On-demand writer text-editor context menu from caret/selection/completions context — grouped/
/// progressively disclosed (GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS): a handful of top-level
/// verbs plus taxonomy groups for the rest, with the destructive `cut` row kept trailing. `selectToken`/
/// `selectLine`/`selectAll`/`cut`/`copy`/`paste` are not declared `ActionDefinition`s (no palette/undo
/// entry makes sense for them), so they stay bespoke `.item(...)` rows per `Menu::of`'s escape hatch;
/// `requestCompletions`/`lintDocument`/`formatDocument`/`commitRename` are declared actions and resolve
/// through `.action(...)` against `registry`.
pub(super) fn writer_context_menu_items(registry: &AppActionRegistry, text: Option<&ContextMenuTextContext>, view_state: &semio_framework_plugin::ViewModel) -> Vec<ContextMenuItemSpec> {
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
    let can_suggest = text.is_some_and(|t| t.has_completions);
    let has_selection = text.is_some_and(|t| t.has_selection);
    let can_rename = text.is_some_and(|t| t.can_rename);
    let reason = semio_framework_plugin::nothing_selected().resolve(view_state.terminology, view_state.locale).to_string();
    let bespoke = |id: &str, label: &str, icon: &str, action: &str, disabled: bool| {
        let row = ContextMenuItemSpec { id: id.into(), label: Some(label.into()), icon: Some(icon.into()), action: Some(action.into()), ..Default::default() };
        if disabled {
            row.disabled_because(reason.clone())
        } else {
            row
        }
    };
    Menu::of(registry, view_state)
        .item(bespoke("writer-select-token", if is_de { "Token auswählen" } else { "Select token" }, "text-cursor", "selectToken", false))
        .item(bespoke("writer-copy", if is_de { "Kopieren" } else { "Copy" }, "copy", "copy", !has_selection))
        .item(bespoke("writer-paste", if is_de { "Einfügen" } else { "Paste" }, "clipboard", "paste", false))
        .group("selection", |m| {
            m.item(bespoke("writer-select-line", if is_de { "Zeile auswählen" } else { "Select line" }, "list-ordered", "selectLine", false)).item(bespoke(
                "writer-select-all",
                if is_de { "Alles auswählen" } else { "Select All" },
                "select-all",
                "selectAll",
                false,
            ))
        })
        .group("tools", |m| {
            let m = m.action("lintDocument");
            if can_suggest {
                m.action("requestCompletions")
            } else {
                m
            }
        })
        .group("transform", |m| {
            let m = m.action("formatDocument");
            if can_rename {
                m.action("commitRename")
            } else {
                m
            }
        })
        .item(ContextMenuItemSpec { destructive: Some(true), ..bespoke("writer-cut", if is_de { "Ausschneiden" } else { "Cut" }, "scissors", "cut", !has_selection) })
        .build()
}
//#endregion 🔖️ContextMenu

//#region 🧵️RetainedCommands
pub(super) const WRITER_COMMAND_TOOL_IDS: &[&str] = &[
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
];

const WRITER_COMMAND_PAYLOAD_SCHEMA: &str = "writer.writer.tool-command.v1";

pub(super) const MAX_WRITER_COMMAND_RAW_BYTES: usize = 4_096;

const MAX_WRITER_COMMAND_DECODED_ITEMS: usize = 4_096;

pub(super) const MAX_WRITER_COMMAND_WORK_ITEMS: usize = 64;

const MAX_WRITER_COMMAND_TEXT_BYTES: usize = 4_096;

const MAX_WRITER_COMMAND_URI_BYTES: usize = 1_024;

const MAX_WRITER_EXAMPLE_ID_BYTES: usize = 64;

fn writer_command_admitted(command: &WriterCommand, snapshot: &WriterSnapshot, window_transient: Option<&WriterMainWindowTransient>) -> bool {
    if matches!(command, WriterCommand::EngagementInput(payload) if payload.value.len() > MAX_WRITER_COMMAND_TEXT_BYTES) {
        return false;
    }
    if matches!(command, WriterCommand::TextEdit(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
        || matches!(command, WriterCommand::SetText(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
        || matches!(command, WriterCommand::CommitRename(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
    {
        return false;
    }
    if matches!(command, WriterCommand::EngagementSubmit(payload) if payload.value.as_deref().or_else(|| window_transient.map(|state| state.engagement_input.as_str())).is_none_or(|value| value.len() > MAX_WRITER_COMMAND_TEXT_BYTES))
    {
        return false;
    }
    if matches!(command, WriterCommand::SetActiveExample(payload) if payload.example_id.len() > MAX_WRITER_EXAMPLE_ID_BYTES) {
        return false;
    }
    if matches!(command, WriterCommand::LoadDocumentJson(payload) if payload.json.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
    {
        return false;
    }
    if matches!(command, WriterCommand::OpenDocument(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES || payload.uri.len() > MAX_WRITER_COMMAND_URI_BYTES) {
        return false;
    }
    let requires_text = match command {
        WriterCommand::TextEdit(_) | WriterCommand::SetText(_) | WriterCommand::FormatDocument(_) | WriterCommand::CommitRename(_) => true,
        WriterCommand::EngagementSubmit(payload) => {
            let Some(value) = payload.value.as_deref().or_else(|| window_transient.map(|state| state.engagement_input.as_str())) else { return false };
            engagement_token_matches(value.trim(), "format")
        }
        _ => false,
    };
    if requires_text && snapshot.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES {
        return false;
    }
    true
}

fn writer_command_emit(
    command: &WriterCommand,
    snapshot: &WriterSnapshot,
    view_state: Option<&semio_framework_plugin::ViewModel>,
    window_config: Option<&WriterMainWindowConfig>,
    window_transient: Option<&WriterMainWindowTransient>,
) -> Result<(Emit<WriterMutation, NoConfigMutation>, EphemeralEmit<EditorApp<WriterPlayApp>>), &'static str> {
    let text = snapshot.text.as_str();
    let mut emit = Emit::default();
    let mut ephemeral = EphemeralEmit::default();
    match command {
        WriterCommand::TextEdit(payload) => emit = Emit::mutations(vec![WriterMutation::EditText(crate::schema::mutations::EditText { text: payload.text.clone() })]),
        WriterCommand::SetText(payload) => emit = Emit::mutations(vec![WriterMutation::EditText(crate::schema::mutations::EditText { text: payload.text.clone() })]),
        WriterCommand::TextSplice(payload) => {
            let view = view_state.ok_or("Writer typing requires its concrete window context")?;
            let selection = crate::WriterEditorSelection { start: payload.anchor, end: payload.caret, splice: payload.seq };
            ephemeral.window_transient.push(
                main::transient::addressed(view, WriterMainWindowTransientMutation::SetEditorSelection(main::transient::SetEditorSelection { selection: Some(selection) })).map_err(|_| "Writer typing rejected its concrete window context")?,
            );
            emit = Emit::mutations(vec![crate::schema::mutations::splice_text(payload.splice())]);
        }
        WriterCommand::SetCamera(payload) => {
            let view = view_state.ok_or("Writer camera change requires its concrete window context")?;
            emit.window_config_mutations.push(main::config::addressed(view, WriterMainWindowConfigMutation::SetCamera(main::config::SetCamera { camera: payload.camera.clone() })).map_err(|_| "Writer camera change rejected its concrete window context")?);
        }
        WriterCommand::RequestCompletions(_) => {}
        WriterCommand::LintDocument(_) => {
            let view = view_state.ok_or("Writer lint requires its concrete window context")?;
            let state = window_transient.ok_or("Writer lint lost its exact window transient")?;
            ephemeral.window_transient.push(
                main::transient::addressed(view, WriterMainWindowTransientMutation::SetLintGeneration(main::transient::SetLintGeneration { value: state.lint_generation + 1 })).map_err(|_| "Writer lint rejected its concrete window context")?,
            );
        }
        WriterCommand::SetEditorSelection(payload) => {
            let view = view_state.ok_or("Writer selection requires its concrete window context")?;
            let selection = crate::WriterEditorSelection { start: payload.start, end: payload.end, splice: payload.splice };
            ephemeral.window_transient.push(
                main::transient::addressed(view, WriterMainWindowTransientMutation::SetEditorSelection(main::transient::SetEditorSelection { selection: Some(selection) })).map_err(|_| "Writer selection rejected its concrete window context")?,
            );
        }
        WriterCommand::ToggleLineNumbers(_) => {
            let view = view_state.ok_or("Writer settings require their concrete window context")?;
            let mut settings = window_config.ok_or("Writer settings lost their exact window config")?.editor_settings.clone();
            settings.show_line_numbers = !settings.show_line_numbers;
            emit.window_config_mutations.push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
        }
        WriterCommand::SetFontPx(payload) => {
            let view = view_state.ok_or("Writer settings require their concrete window context")?;
            let mut settings = window_config.ok_or("Writer settings lost their exact window config")?.editor_settings.clone();
            settings.font_px = payload.value;
            emit.window_config_mutations.push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
        }
        WriterCommand::SetLineHeight(payload) => {
            let view = view_state.ok_or("Writer settings require their concrete window context")?;
            let mut settings = window_config.ok_or("Writer settings lost their exact window config")?.editor_settings.clone();
            settings.line_height = payload.value;
            emit.window_config_mutations.push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
        }
        WriterCommand::SetTabSize(payload) => {
            let view = view_state.ok_or("Writer settings require their concrete window context")?;
            let mut settings = window_config.ok_or("Writer settings lost their exact window config")?.editor_settings.clone();
            settings.tab_size = payload.value.max(1);
            emit.window_config_mutations.push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
        }
        WriterCommand::EngagementInput(payload) => {
            let view = view_state.ok_or("Writer engagement input requires its concrete window context")?;
            if window_transient.is_none_or(|state| state.engagement_input != payload.value) {
                ephemeral.window_transient.push(
                    main::transient::addressed(view, WriterMainWindowTransientMutation::SetEngagementInput(main::transient::SetEngagementInput { value: payload.value.clone() }))
                        .map_err(|_| "Writer engagement input rejected its concrete window context")?,
                );
            }
        }
        WriterCommand::SetActiveExample(payload) => {
            emit.effects.push(reset_document_effect_now(&set_active_example::document_for_example_id(&payload.example_id)));
        }
        WriterCommand::OpenDocument(payload) => emit = open_document::emit(payload),
        WriterCommand::LoadDocumentJson(payload) => {
            if let Ok(document) = semio_framework_pack_json::from_json_str::<WriterSnapshot>(&payload.json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                emit.effects.push(reset_document_effect_now(&document));
            }
        }
        WriterCommand::FormatDocument(_) => {
            let formatted = crate::schema::format_writer_text(text, &snapshot.language_id);
            if formatted != text {
                emit.artifact_mutations.push(WriterMutation::EditText(crate::schema::mutations::EditText { text: formatted }));
            }
        }
        WriterCommand::CommitRename(payload) => {
            use crate::schema::{apply_jack_rename, jack_symbol_at_offset, JackSymbolKind};
            let selection = window_transient.and_then(|state| state.editor_selection.clone()).ok_or("Writer rename requires its concrete editor selection")?;
            if selection.start == selection.end {
                if let Some(symbol) = jack_symbol_at_offset(text, selection.start) {
                    if symbol.kind == JackSymbolKind::Variable {
                        emit.artifact_mutations.push(WriterMutation::EditText(crate::schema::mutations::EditText { text: apply_jack_rename(text, &symbol.occurrences, &payload.text) }));
                        return Ok((emit, ephemeral));
                    }
                }
            }
            if selection.start <= selection.end && selection.end <= text.len() {
                let mut updated = text.to_string();
                updated.replace_range(selection.start..selection.end, &payload.text);
                emit.artifact_mutations.push(WriterMutation::EditText(crate::schema::mutations::EditText { text: updated }));
            }
        }
        WriterCommand::EngagementSubmit(payload) => {
            let view = view_state.ok_or("Writer engagement requires its concrete window context")?;
            let state = window_transient.ok_or("Writer engagement lost its exact window transient")?;
            let value = payload.value.clone().unwrap_or_else(|| state.engagement_input.clone());
            let trimmed = value.trim();
            ephemeral
                .window_transient
                .push(main::transient::addressed(view, WriterMainWindowTransientMutation::SetEngagementInput(main::transient::SetEngagementInput { value: String::new() })).map_err(|_| "Writer engagement rejected its concrete window context")?);
            if engagement_token_matches(trimmed, "format") {
                let formatted = crate::schema::format_writer_text(text, &snapshot.language_id);
                if formatted != text {
                    emit.artifact_mutations.push(WriterMutation::EditText(crate::schema::mutations::EditText { text: formatted }));
                }
            } else if engagement_token_matches(trimmed, "lint") {
                ephemeral.window_transient.push(
                    main::transient::addressed(view, WriterMainWindowTransientMutation::SetLintGeneration(main::transient::SetLintGeneration { value: state.lint_generation + 1 }))
                        .map_err(|_| "Writer lint rejected its concrete window context")?,
                );
            } else if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
                let mut settings = window_config.ok_or("Writer engagement lost its exact window config")?.editor_settings.clone();
                settings.show_line_numbers = !settings.show_line_numbers;
                emit.window_config_mutations
                    .push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
            } else if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
                if let Ok(px) = rest.parse::<u32>() {
                    let mut settings = window_config.ok_or("Writer engagement lost its exact window config")?.editor_settings.clone();
                    settings.font_px = px;
                    emit.window_config_mutations
                        .push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
                }
            } else if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
                if let Ok(size) = rest.parse::<u32>() {
                    let mut settings = window_config.ok_or("Writer engagement lost its exact window config")?.editor_settings.clone();
                    settings.tab_size = size.max(1);
                    emit.window_config_mutations
                        .push(main::config::addressed(view, WriterMainWindowConfigMutation::SetEditorSettings(main::config::SetEditorSettings { settings })).map_err(|_| "Writer settings rejected their concrete window context")?);
                }
            }
        }
    }
    Ok((emit, ephemeral))
}

/// 🧮️ One bounded step per command: Writer reduces every tool to its mutations in a single turn, so the work holds no state of its own.
pub(super) struct WriterRetainedCommandWork {
    pub(super) tool_id: &'static str,
}

impl WriterRetainedCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { tool_id }
    }
}

fn writer_window_transient<'a>(context: Option<&'a semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<WriterPlayApp>>>) -> Option<&'a WriterMainWindowTransient> {
    context.and_then(|context| context.window_transient.as_ref()).and_then(|snapshot| snapshot.get::<WriterMainWindowTransientOwner>())
}

impl ArtifactCommandWork<EditorApp<WriterPlayApp>> for WriterRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &WriterCommand, snapshot: &WriterSnapshot, _interaction: &protocol::InteractionState, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<WriterPlayApp>>>) -> Option<usize> {
        writer_command_admitted(command, snapshot, writer_window_transient(context)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<WriterPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<WriterPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let window_transient = writer_window_transient(input.context);
        if !writer_command_admitted(input.command, input.snapshot, window_transient) {
            return Err(Fault::from("writer-command-admission"));
        }
        let view_state = input.context.and_then(|context| context.view_state.as_ref());
        let window_config = input.context.and_then(|context| context.window_config.as_ref()).and_then(|snapshot| snapshot.get::<WriterMainWindowConfigOwner>());
        let (emit, ephemeral) = writer_command_emit(input.command, input.snapshot, view_state, window_config, window_transient).map_err(Fault::from)?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

pub(super) struct WriterCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl WriterCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: WRITER_COMMAND_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for WriterCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<WriterPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<WriterPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WRITER_COMMAND_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_WRITER_COMMAND_RAW_BYTES, MAX_WRITER_COMMAND_DECODED_ITEMS, 1, MAX_WRITER_COMMAND_WORK_ITEMS, 2_000, 1, 1)
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: MAX_WRITER_COMMAND_RAW_BYTES, checkpoint_resume: true);

}

impl ArtifactOwnedToolJobFactory for WriterCommandJobFactory {
    type Owner = EditorApp<WriterPlayApp>;
    const TOOL_IDS: &'static [&'static str] = WRITER_COMMAND_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setText", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "textSplice", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "formatDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitRename", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "requestCompletions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "lintDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "textSelect", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLineNumbers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setEditorSetting", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "engagementSubmit",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient],
        },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ArtifactStorePreparation
const WRITER_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 32_768;

fn writer_snapshot_retained_bytes(snapshot: &WriterSnapshot) -> usize {
    snapshot.schema.len().saturating_add(snapshot.id.len()).saturating_add(snapshot.language_id.len()).saturating_add(snapshot.uri.len()).saturating_add(snapshot.text.len())
}

fn admit_writer_artifact_mutation(mutation: &WriterMutation) -> Result<(), String> {
    let bytes = match mutation {
        WriterMutation::EditText(payload) => payload.text.len(),
        WriterMutation::SpliceText(payload) => payload.deleted.len() + payload.insert.len() + payload.before.len() + payload.after.len(),
        _ => return Err("Writer retained Artifact preparation only admits the exact EditText and SpliceText cohort".into()),
    };
    if bytes > MAX_WRITER_COMMAND_TEXT_BYTES {
        return Err("Writer text edit exceeds its fixed retained preparation envelope".into());
    }
    Ok(())
}

/// 🔒️ Keeps the Artifact lane's deliberate contract on top of the shared bounded preparation: only the exact `EditText`/`SpliceText` cohort
/// every Writer tool emits, at most `MAX_WRITER_COMMAND_TEXT_BYTES` per edit, against a base of at most `WRITER_ARTIFACT_STORE_MAXIMUM_BYTES`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct WriterArtifactPreparationFactory {
    #[factory_child]
    inner: Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<WriterSnapshot, WriterMutation>>,
}

impl WriterArtifactPreparationFactory {
    pub fn shared() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<WriterSnapshot, WriterMutation>> {
        Arc::new(Self { inner: store::cold_mutation_apply_preparation_factory::<WriterSnapshot, WriterMutation>() })
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<WriterSnapshot, WriterMutation> for WriterArtifactPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<WriterMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<WriterMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        self.inner.begin_batch_digest(edit, grant)
    }

    fn operation_wire_source<'a>(&self, mutation: &'a WriterMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        self.inner.operation_wire_source(mutation)
    }

    fn operation_schema_parts<'a>(&'a self, mutation: &'a WriterMutation) -> Option<(&'a str, &'a str)> {
        self.inner.operation_schema_parts(mutation)
    }

    fn preflight(&self, mutation: &WriterMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Writer Artifact preparation rejected its lane".into());
        }
        admit_writer_artifact_mutation(mutation)?;
        self.inner.preflight(mutation, lane)
    }

    fn begin_demand(&self, mutation: &WriterMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        self.inner.begin_demand(mutation, lane)
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<WriterSnapshot, WriterMutation, WriterMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<WriterSnapshot, WriterMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<WriterSnapshot, WriterMutation, WriterMutation>)> {
        if writer_snapshot_retained_bytes(request.base.get()) > WRITER_ARTIFACT_STORE_MAXIMUM_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Writer Artifact base exceeds its fixed retained preparation envelope"), request));
        }
        self.inner.begin(request, grant)
    }

    fn stamped_clock(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.inner.stamped_clock()
    }

    fn stamped_mutation_id(&self) -> Option<&protocol::MutationId> {
        self.inner.stamped_mutation_id()
    }
}
//#endregion 📬️ArtifactStorePreparation
