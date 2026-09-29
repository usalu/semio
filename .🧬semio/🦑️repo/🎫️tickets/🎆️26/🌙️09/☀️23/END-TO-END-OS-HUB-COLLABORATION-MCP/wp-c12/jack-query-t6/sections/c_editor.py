# ✏️ Section C of `c12-jack-query-document-patch.py`: the editor publishes the query on the document lane; the editor
# window's config lane is removed.
ED = EDITOR + "🦀️.rs"
CMD = EDITOR + "🎮️commands/"
JOB = CMD + "▶️run-query/🧵️job/🦀️.rs"
EDITOR_WINDOW = EDITOR + "🎭️modes/✏️edit/🪟️windows/📝️editor/"

# ── editor module ──
edit(ED, """//! `TrinityJackRuntime` field (selection, camera, query draft, LOD, …) lives in
//! exact window config/transient owners.""", """//! `TrinityJackRuntime` field (selection, camera, LOD, …) lives in exact window config/transient owners; the query is
//! document content (`JackSnapshot::query`), undoable and shared like the graph it runs against.""", "editor module doc")
edit(ED, "use crate::editor::jack::query_window_config::JackEditorWindowConfigOwner;\n", "", "editor import")
edit(ED, 'pub(crate) const TRINITY_JACK_DEFAULT_QUERY: &str = "' + DEFAULT_QUERY + '";\n', "", "editor default query moves to the root")
edit(ED, 'const JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS: &[&str] = &["nodeGraphViewport", "textEdit", "formatDocument", "setLodMode"];',
     'const JACK_RETAINED_WINDOW_CONFIG_TOOL_IDS: &[&str] = &["nodeGraphViewport", "setLodMode"];', "config route tool ids")
edit(ED, """    ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "formatDocument", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
""", "", "config route contracts")
edit(ED, """        TrinityJackCommand::TextEdit { text } => text.len(),
        TrinityJackCommand::FormatDocument => 1,
        TrinityJackCommand::SetLodMode { value } => value.len(),""", """        TrinityJackCommand::SetLodMode { value } => value.len(),""", "config route extent")
edit(ED, """        TrinityJackCommand::TextEdit { text } => commands::text_edit(text, context.and_then(|context| context.view_state.as_ref())),
        TrinityJackCommand::FormatDocument => {
            let context = context.ok_or_else(|| Fault::from("Jack query formatting requires retained window context"))?;
            let query = context.window_config.as_ref().and_then(|window| window.get::<JackEditorWindowConfigOwner>()).ok_or_else(|| Fault::from("Jack query formatting requires the exact editor-window config snapshot"))?;
            commands::format_document(&query.jack_query, context.view_state.as_ref())
        }
""", "", "config route reduce")
edit(ED, 'const JACK_RETAINED_DOCUMENT_TOOL_IDS: &[&str] = &["patchNodes", "deleteSelection", "setActiveExample", "setFixtureJson"];',
     'const JACK_RETAINED_DOCUMENT_TOOL_IDS: &[&str] = &["patchNodes", "deleteSelection", "setActiveExample", "setFixtureJson", "textEdit", "formatDocument"];', "document route tool ids")
edit(ED, """    ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];""", """    ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "formatDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
];""", "document route contracts")
edit(ED, """        TrinityJackCommand::SetFixtureJson { json } => json.len(),
        _ => return None,""", """        TrinityJackCommand::SetFixtureJson { json } => json.len(),
        TrinityJackCommand::TextEdit { text } if text.len() <= crate::JACK_QUERY_MAXIMUM_BYTES => text.len(),
        TrinityJackCommand::FormatDocument => 1,
        _ => return None,""", "document route extent")
edit(ED, """        TrinityJackCommand::SetFixtureJson { json } => commands::set_fixture_json(json),
        _ => return Err(Fault::from("jack-retained-document-route-mismatch")),""", """        TrinityJackCommand::SetFixtureJson { json } => commands::set_fixture_json(json),
        TrinityJackCommand::TextEdit { text } => commands::text_edit(text)?,
        TrinityJackCommand::FormatDocument => commands::format_document(&snapshot.query),
        _ => return Err(Fault::from("jack-retained-document-route-mismatch")),""", "document route reduce")
edit(ED, """        crate::editor::jack::window_config::register(registry)?;
        crate::editor::jack::query_window_config::register(registry)
""", """        crate::editor::jack::window_config::register(registry)
""", "window config owners")
edit(ED, """            TrinityJackCommand::TextEdit { text } => return commands::text_edit(text, view_state),""",
     """            TrinityJackCommand::TextEdit { text } => commands::text_edit(text)?,""", "direct reduce text edit")
edit(ED, """            TrinityJackCommand::FormatDocument => {
                let query = crate::editor::jack::query_window_config::current(cfg).ok_or_else(|| Fault::from("Jack query formatting requires exact editor-window config"))?;
                return commands::format_document(&query.jack_query, view_state);
            }""", """            TrinityJackCommand::FormatDocument => commands::format_document(&snapshot.query),""", "direct reduce format")
edit(ED, """            TRINITY_JACK_PLAY_BODY_EDITOR => {
                let query = crate::editor::jack::query_window_config::current(cfg).cloned().unwrap_or_default();
                edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, snapshot, &query, None)
            }""", """            TRINITY_JACK_PLAY_BODY_EDITOR => edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, snapshot, None),""", "render editor body")
edit(ED, """            let query = crate::editor::jack::query_window_config::current(cfg).cloned().unwrap_or_default();
            let root = edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, doc.snapshot, &query, selection)?;""",
     """            let root = edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, doc.snapshot, selection)?;""", "render editor with selection")
edit(ED, 'ActionDefinition::new("textEdit", LocalizedLabel::native("Edit Jack Query", "Jack-Abfrage bearbeiten"), ActionKind::View, "typography")',
     'ActionDefinition::new("textEdit", LocalizedLabel::native("Edit Jack Query", "Jack-Abfrage bearbeiten"), ActionKind::Mutation, "typography")', "text edit kind")
edit(ED, 'ActionDefinition::new("formatDocument", LocalizedLabel::native("Format Jack Query", "Jack-Abfrage formatieren"), ActionKind::View, "typography")',
     'ActionDefinition::new("formatDocument", LocalizedLabel::native("Format Jack Query", "Jack-Abfrage formatieren"), ActionKind::Mutation, "typography")', "format kind")
edit(ED, '''.action_describe("formatDocument", LocalizedLabel::native("Reformats the Jack query in the query editor window; the graph document is not changed, and a query that does not parse is left as it is.", "Formatiert die Jack-Abfrage im Abfrage-Editorfenster neu; das Graphdokument ändert sich nicht, eine nicht lesbare Abfrage bleibt wie sie ist."))''',
     '''.action_describe("formatDocument", LocalizedLabel::native("Reformats the document's Jack query as one undoable edit; the graph is not changed, and a query that does not parse is left as it is.", "Formatiert die Jack-Abfrage des Dokuments als ein rückgängig machbarer Schritt neu; der Graph ändert sich nicht, eine nicht lesbare Abfrage bleibt wie sie ist."))''', "format description")
edit(ED, '''"Runs a Jack graph query (or the editor's current one) against the component graph and shows the matches in the given results window; CREATE, SET or DELETE clauses change the graph.", "Führt eine Jack-Graphabfrage (oder die aktuelle des Editors) auf dem Bauteilgraphen aus''',
     '''"Runs a Jack graph query (or the document's own query) against the component graph and shows the matches in the given results window; CREATE, SET or DELETE clauses change the graph.", "Führt eine Jack-Graphabfrage (oder die eigene Abfrage des Dokuments) auf dem Bauteilgraphen aus''', "run description")
edit(ED, '''"Puts one of the bundled example queries into the query editor, runs it and shows its matches in the given results window.", "Setzt eine der mitgelieferten Beispielabfragen in den Abfrage-Editor, führt sie aus''',
     '''"Makes one of the bundled example queries the document's query, runs it and shows its matches in the given results window.", "Macht eine der mitgelieferten Beispielabfragen zur Abfrage des Dokuments, führt sie aus''', "load example description")
edit(ED, '''"Replaces the whole component graph with a bundled fixture (the Nakagin capsule tower or the branch chain), by example id.", "Ersetzt den gesamten Bauteilgraphen durch eine mitgelieferte Fixture (den Nakagin Capsule Tower oder die Astkette), anhand der Beispiel-Id."''',
     '''"Replaces the whole document with a bundled fixture (the Nakagin capsule tower or the branch chain) and its example query, by example id.", "Ersetzt das gesamte Dokument durch eine mitgelieferte Fixture (den Nakagin Capsule Tower oder die Astkette) samt Beispielabfrage, anhand der Beispiel-Id."''', "set example description")

# ── root: query bound, module tree ──
edit(ROOT, 'pub const TRINITY_JACK_DEFAULT_QUERY: &str = "' + DEFAULT_QUERY + '";\n', 'pub const TRINITY_JACK_DEFAULT_QUERY: &str = "' + DEFAULT_QUERY + '";\n' + """
/// 📏️ Largest query text a jack document holds: one `set-query` stays inside the 4 KiB document-mutation admission
/// (`TRINITY_JACK_ARTIFACT_MUTATION_MAXIMUM_BYTES`) and the store initializer's 4 KiB owned-field bound.
pub const JACK_QUERY_MAXIMUM_BYTES: usize = 3_584;
""", "root query bound")
edit(ROOT, """        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🦀️.rs"]
        pub mod query_window_config;

""", "", "root module query_window_config")
remove(EDITOR_WINDOW + "🎚️config", "editor window config lane")
remove(ANY + "🧪️tests/🎚️mutate-trinity-jack-1-any-editor-edit-editor-config", "editor config scenario")
BRIDGE = "✏️s/🔌️plugins/🔱️trinity/🏭️bridge/🦀️.rs"
edit(BRIDGE, """    ("JackEditorWindowConfigMutation", descriptors::<semio_s_artifact_trinity_jack::editor::jack::query_window_config::JackEditorWindowConfig, semio_s_artifact_trinity_jack::editor::jack::query_window_config::JackEditorWindowConfigMutation>),
""", "", "bridge aggregate")
edit(BRIDGE, """    ("s.trinity.jack", "1", "any", "✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config", "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config", ""),
""", "", "bridge coordinate")

# ── commands ──
edit(CMD + "✏️text-edit/🦀️.rs", """use crate::editor::jack::query_window_config::{self, JackEditorWindowConfigMutation, SetQuery};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};

/// ⌨️ The coalesce key of a typing burst in the query editor: every keystroke amends the one window-config edit the burst
/// opened, so typing is one undo step and never spends the store's fixed applied-edit ledger one keystroke at a time (ticket
/// 26/09/23 F1: the 65th typed character was refused with `batched publication requires preinstalled fixed applied and
/// revision capacity` and the query stopped saving).
pub(crate) const JACK_QUERY_TYPING_COALESCE_KEY: &str = "jack-query-typing";

pub(crate) fn text_edit(text: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::from("Jack text edit requires an exact editor window"))?;
    let mutation = query_window_config::addressed(view, JackEditorWindowConfigMutation::SetQuery(SetQuery { value: text.to_string() }))?;
    Ok(Emit { window_config_mutations: vec![mutation], coalesce_key: Some(JACK_QUERY_TYPING_COALESCE_KEY.into()), ..Default::default() })
}""", """use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// ⌨️ The coalesce key of a typing burst in the query editor: every keystroke amends the one document edit the burst opened,
/// so a typing run is ONE undo step and never spends the store's fixed applied-edit ledger one keystroke at a time (ticket
/// 26/09/23 F1: the 65th typed character was refused with `batched publication requires preinstalled fixed applied and
/// revision capacity` and the query stopped saving).
pub(crate) const JACK_QUERY_TYPING_COALESCE_KEY: &str = "jack-query-typing";

/// ⌨️ One keystroke of the query editor: the whole query text as one `set-query` document mutation, coalesced into the run.
pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    if text.len() > crate::JACK_QUERY_MAXIMUM_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.query-too-large"), format!("a Jack query holds at most {} bytes", crate::JACK_QUERY_MAXIMUM_BYTES)));
    }
    Ok(Emit { artifact_mutations: vec![set_query(text.to_string())], coalesce_key: Some(JACK_QUERY_TYPING_COALESCE_KEY.into()), ..Default::default() })
}""", "command text edit")
edit(CMD + "✨️format-document/🦀️.rs", """use crate::core;
use crate::editor::jack::query_window_config::{self, JackEditorWindowConfigMutation, SetQuery};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};

pub(crate) fn format_document(jack_query: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::from("Jack query formatting requires an exact editor window"))?;
    match core::format(jack_query) {
        Ok(formatted) => Ok(Emit { window_config_mutations: vec![query_window_config::addressed(view, JackEditorWindowConfigMutation::SetQuery(SetQuery { value: formatted }))?], ..Default::default() }),
        Err(_) => Ok(Emit::default()),
    }
}""", """use crate::core;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, NoConfigMutation};

/// ✨️ Reformats the document's query as one undoable `set-query`; a query that does not parse, or is already formatted,
/// changes nothing.
pub(crate) fn format_document(jack_query: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match core::format(jack_query) {
        Ok(formatted) if formatted != jack_query && formatted.len() <= crate::JACK_QUERY_MAXIMUM_BYTES => Emit { artifact_mutations: vec![set_query(formatted)], ..Default::default() },
        _ => Emit::default(),
    }
}""", "command format")
edit(CMD + "🎯️set-active-example/🦀️.rs", "        _ => crate::editor::jack::TRINITY_JACK_DEFAULT_QUERY,", "        _ => crate::TRINITY_JACK_DEFAULT_QUERY,", "example preset default query")
edit(CMD + "🎯️set-active-example/🦀️.rs", """    match fixture_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(next) => {""", """    match fixture_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(parsed) => {
            let next = JackSnapshot { query: preset_query(example_id).into(), ..parsed };""", "example loads its preset query")

# ── query window ──
edit(EDITOR_WINDOW + "🦀️.rs", """use crate::editor::jack::query_window_config::JackEditorWindowConfig;
""", "", "editor window import")
edit(EDITOR_WINDOW + "🦀️.rs", """pub(crate) fn render(surface_id: &str, _controller_id: &str, snapshot: &JackSnapshot, cfg: &JackEditorWindowConfig, selection: Option<&JackEditorSelection>) -> UiAssemblyResult<BuiltNode> {
    let query = &cfg.jack_query;""", """pub(crate) fn render(surface_id: &str, _controller_id: &str, snapshot: &JackSnapshot, selection: Option<&JackEditorSelection>) -> UiAssemblyResult<BuiltNode> {
    let query = &snapshot.query;""", "editor window render")

# ── run-query job ──
edit(JOB, "use crate::editor::jack::query_window_config::{JackEditorWindowConfigMutation, JackEditorWindowConfigOwner, SetQuery};\n", "use crate::standards::v1::subsets::any::schema::mutations::set_query;\n", "job imports")
edit(JOB, "const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient];",
     "const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient];", "job lanes")
edit(JOB, """    let editor_config = request
        .context
        .window_config
        .as_ref()
        .filter(|window| window.window_id() == editor_window_id)
        .and_then(|window| window.get::<JackEditorWindowConfigOwner>())
        .ok_or_else(|| Fault::from("Jack query execution requires the exact originating editor-window config snapshot"))?;
    let source = match request.command.as_ref() {
        TrinityJackCommand::RunQuery { query, .. } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&editor_config.jack_query),""",
     """    let source = match request.command.as_ref() {
        TrinityJackCommand::RunQuery { query, .. } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&request.snapshot.query),""", "job source")
edit(JOB, """    let work = Box::new(JackQueryWork::new(tool, source.to_string(), editor_window_id.to_string(), results_window_id.to_string(), operation.operation_id, operation.generation));""",
     """    let adopts_query = matches!(request.command.as_ref(), TrinityJackCommand::LoadExampleQuery { .. }) && source != request.snapshot.query && source.len() <= crate::JACK_QUERY_MAXIMUM_BYTES;
    let work = Box::new(JackQueryWork::new(tool, source.to_string(), adopts_query, editor_window_id.to_string(), results_window_id.to_string(), operation.operation_id, operation.generation));""", "job adopts query")
edit(JOB, """    source: Option<String>,
    editor_window_id: Option<String>,""", """    source: Option<String>,
    adopts_query: bool,
    editor_window_id: Option<String>,""", "job work field")
edit(JOB, """    fn new(tool: &'static str, source: String, editor_window_id: String, results_window_id: String, operation_id: u64, generation: u64) -> Self {
        Self { tool, source: Some(source), editor_window_id""", """    fn new(tool: &'static str, source: String, adopts_query: bool, editor_window_id: String, results_window_id: String, operation_id: u64, generation: u64) -> Self {
        Self { tool, source: Some(source), adopts_query, editor_window_id""", "job work new")
edit(JOB, """            emit: Emit {
                artifact_mutations: mutations,
                window_config_mutations: vec![semio_framework_plugin::WindowConfigMutation::of::<JackEditorWindowConfigOwner>(
                    self.editor_window_id.as_ref().expect("editor window id is retained"),
                    JackEditorWindowConfigMutation::SetQuery(SetQuery { value: self.source.as_ref().expect("query source is retained").clone() }),
                )],
                ..Default::default()
            },""", """            emit: Emit {
                artifact_mutations: self.adopts_query.then(|| set_query(self.source.as_ref().expect("query source is retained").clone())).into_iter().chain(mutations).collect(),
                ..Default::default()
            },""", "job completion publishes the adopted query")
edit(ED, """    fn handle(
        command: &TrinityJackCommand,
        doc: &ArtifactView<'_, JackSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,""", """    fn handle(
        command: &TrinityJackCommand,
        doc: &ArtifactView<'_, JackSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,""", "handle no longer reads config")
