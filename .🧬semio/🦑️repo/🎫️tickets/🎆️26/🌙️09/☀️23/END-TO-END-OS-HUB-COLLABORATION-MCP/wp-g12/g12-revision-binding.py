#!/usr/bin/env python3
"""🔐️ G12 prepared set (window 3, end of T1): revision-bound verbs reachable by agents (coordinator decision 2026-09-28 22:0x).

Schema-first `ArgFormat::DocumentRevision` / `ArgFormat::TargetRevision` (`x-semio-format`), declared through
`ActionArgDef::document_revision` / `ActionArgDef::target_revision` — optional and hidden. The SDK's agent lane
(`preview_addressed_action`) fills an omitted one with the token the rendered binding would carry right now: the document's
canonical revision (`store.content_revision()`, the value every document-scoped verb compares against on that lane), or the
app's own `agent_target_revision` (target scope; `None` = refused by name). The shell lanes never fill: an omission there is
still refused by the app's own parser. The gateway (host, landed separately) admits an omission only behind a passed MCP
`expectedRevision`. Declarations switched: SDK text/tree (document) + document page (target) kits, stdio table cell +
structural (document), xlsx cell (target), epw row (target, new `row_revision_addressed_table_window_kind`), wav (document), zip ×2 /
docx ×3 / pdf page (target); agent target fills for zip ×2, pdf ×10, xlsx ×3, epw (docx paragraphs: none — an agent passes their token).

Laws: manifest (optional/hidden/format), SDK testkit `agent_preview` + csv (document) + zip ×2 (target) agent-lane laws, gateway
`an_omitted_revision_without_a_document_guard_is_refused_by_name_and_nothing_is_sent` +
`an_omitted_revision_behind_a_passed_document_guard_commits_and_a_stale_guard_is_a_conflict`.

usage: python3 g12-revision-binding.py [--dry-run|--write|--list] [--root <tree>]   (idempotent: an applied edit is detected by its new text)
"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv

MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
MANIFEST_TESTS = "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs"
PROJECTION = "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
GENERATED_TS = "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
STDIO_CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs"
WAV = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🦀️.rs"
ZIP_WINDOWS = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs",
]
DOCX_WINDOWS = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs",
]
PDF_PAGE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs"

EDITS = {}


def edit(path, name, old, new, count=1):
    EDITS.setdefault(path, []).append((name, old, new, count))


# ── manifest: schema-first formats + constructors ─────────────────────────────────────────────────────────────────────────
edit(MANIFEST, "ArgFormat variants", """    /// `ActionArgDef::surface_app`.
    SurfaceApp {
        roles: Vec<AppRole>,
        dialect_arg: String,
    },
}
""", """    /// `ActionArgDef::surface_app`.
    SurfaceApp {
        roles: Vec<AppRole>,
        dialect_arg: String,
    },
    /// 🔐️ The document's own revision token, as its rendered bindings carry it — see `ActionArgDef::document_revision`.
    DocumentRevision,
    /// 🔐️ One addressed target's own revision token (a page item, a cell, an entry), as its rendered binding carries it —
    /// see `ActionArgDef::target_revision`.
    TargetRevision,
}
""")
edit(MANIFEST, "apply_arg_format arms", """        ArgFormat::Terminology => "terminology",
        ArgFormat::ArtifactKind { roles } => {""", """        ArgFormat::Terminology => "terminology",
        ArgFormat::DocumentRevision => "documentRevision",
        ArgFormat::TargetRevision => "targetRevision",
        ArgFormat::ArtifactKind { roles } => {""")
edit(MANIFEST, "revision constructors", """        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::SurfaceApp { roles, dialect_arg: dialect_arg.into() })))
    }
""", """        Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::SurfaceApp { roles, dialect_arg: dialect_arg.into() })))
    }

    /// @emoji 🔐️ The document revision a rendered binding carries — hidden and optional: the shell lanes pass the binding's
    /// token (and refuse its absence in the app's own parser); the agent lane omits it and is admitted against the document's
    /// revision at admission, behind the MCP `expectedRevision` guard (ticket 26/09/23, G12 session 14c).
    pub fn document_revision(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        let mut def = Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::DocumentRevision)));
        def.presentation = Some(ArgPresentation::Hidden);
        def
    }

    /// @emoji 🔐️ One addressed target's revision a rendered binding carries — hidden and optional like
    /// [`Self::document_revision`]; the agent lane fills it from the app's own `agent_target_revision`.
    pub fn target_revision(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        let mut def = Self::with_schema(id, label, Self::plain_string(Some(ArgFormat::TargetRevision)));
        def.presentation = Some(ArgPresentation::Hidden);
        def
    }
""")
edit(MANIFEST_TESTS, "revision argument law", """//#region 🎯️ActionSemanticsFixture
#[test]
fn catalog_icons_depend_only_on_action_kind_and_explicit_icons_stay_owned() {""", """//#region 🔐️RevisionArguments
/// 🔐️ A revision a rendered binding carries is optional and hidden, and tells a client what it is: the agent lane omits it
/// and is admitted against the document's (or the target's) revision at admission (ticket 26/09/23, G12 session 14c).
#[test]
fn revision_arguments_are_optional_hidden_and_name_their_scope() {
    for (def, format) in [(ActionArgDef::document_revision("revision", LocalizedLabel::data("Revision")), "documentRevision"), (ActionArgDef::target_revision("revision", LocalizedLabel::data("Revision")), "targetRevision")] {
        assert!(!def.required);
        assert_eq!(def.presentation, Some(super::ArgPresentation::Hidden));
        assert_eq!(def.json_schema().get("x-semio-format"), Some(&DslValue::String(format.to_string())));
        assert!(super::missing_required_args(&[def.clone()], &DslValue::Object(Vec::new())).is_empty(), "an omitted revision is never a missing argument");
    }
}
//#endregion 🔐️RevisionArguments

//#region 🎯️ActionSemanticsFixture
#[test]
fn catalog_icons_depend_only_on_action_kind_and_explicit_icons_stay_owned() {""")

TS_UNION_OLD = """export type ArgFormat = { "kind": "artifactRef" } | { "kind": "windowId" } | { "kind": "entityId", entityKind: string, } | { "kind": "iconId" } | { "kind": "color" } | { "kind": "uri" } | { "kind": "json" } | { "kind": "locale" } | { "kind": "terminology" } | { "kind": "artifactKind", roles: Array<AppRole>, } | { "kind": "surfaceApp", roles: Array<AppRole>, dialectArg: string, };"""
TS_UNION_NEW = TS_UNION_OLD[: -len(";")] + """ | { "kind": "documentRevision" } | { "kind": "targetRevision" };"""
edit(PROJECTION, "ArgFormat metadata union", TS_UNION_OLD, TS_UNION_NEW)
edit(GENERATED_TS, "generated ArgFormat union", TS_UNION_OLD, TS_UNION_NEW)

# ── SDK: kit declarations ──────────────────────────────────────────────────────────────────────────────────────────────────
edit(SDK, "TextWindowKit textEdit revision", """.with_args(vec![ActionArgDef::text("revision", LocalizedLabel::native("Revision", "Revision")), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0).required()])""",
     """.with_args(vec![ActionArgDef::document_revision("revision", LocalizedLabel::native("Revision", "Revision")), ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")).min_length(0).required()])""")
edit(SDK, "TreeWindowKit set-node revision", """                        ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                        ActionArgDef::text("revision", LocalizedLabel::native("Revision", "Revision")).required(),""", """                        ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                        ActionArgDef::document_revision("revision", LocalizedLabel::native("Revision", "Revision")),""")
edit(SDK, "DocumentWindowKit page revision", """                ActionArgDef::index("item", LocalizedLabel::native("Text Item", "Textelement")).required(),
                ActionArgDef::text("revision", LocalizedLabel::native("Revision", "Revision")).required(),""", """                ActionArgDef::index("item", LocalizedLabel::native("Text Item", "Textelement")).required(),
                ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")),""")

# ── SDK: the agent-lane resolver hooks ─────────────────────────────────────────────────────────────────────────────────────
edit(SDK, "ArtifactApp::agent_target_revision", """        /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): resolves this app's typed `Command` from a
        /// `ui_contract::UiIntent` — the one entry point `plugin_runtime::plugin_dispatch_intents`""", """        /// @emoji 🔐️ The agent lane's fill for an omitted [`ActionArgDef::target_revision`] argument: the token the rendered
        /// binding of the target `args` address carries in `doc` right now, or `None` when this app resolves no such target —
        /// the agent lane then refuses the call by name. The shell lanes never call it (ticket 26/09/23, G12 session 14c).
        async fn agent_target_revision(_action: &str, _args: &DslValue, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
            Ok(None)
        }
        /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): resolves this app's typed `Command` from a
        /// `ui_contract::UiIntent` — the one entry point `plugin_runtime::plugin_dispatch_intents`""")
edit(SDK, "ArtifactEditor::agent_target_revision", """        /// 🎯️ M1: `ArtifactApp::command_from_intent`'s sibling default for a hand-authored
        /// `ArtifactEditor` — same version-mismatch Fault, same `merge_ui_values` bridge to""", """        /// 🔐️ `ArtifactApp::agent_target_revision`'s sibling for a hand-authored `ArtifactEditor`; `EditorApp<E>` delegates here.
        fn agent_target_revision(_action: &str, _args: &DslValue, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
            Ok(None)
        }
        /// 🎯️ M1: `ArtifactApp::command_from_intent`'s sibling default for a hand-authored
        /// `ArtifactEditor` — same version-mismatch Fault, same `merge_ui_values` bridge to""")
edit(SDK, "EditorApp delegation", """        async fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
            E::command_from_action(action, args)
        }
        async fn command_from_intent(intent: &UiIntent) -> Result<Self::Command, Fault> {
            E::command_from_intent(intent).await
        }""", """        async fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
            E::command_from_action(action, args)
        }
        async fn agent_target_revision(action: &str, args: &DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
            E::agent_target_revision(action, args, doc)
        }
        async fn command_from_intent(intent: &UiIntent) -> Result<Self::Command, Fault> {
            E::command_from_intent(intent).await
        }""")
edit(SDK, "agent-lane fill call", """            let mut arguments = invocation.arguments.clone();
            arguments.insert("windowId".into(), DslValue::String(address.window_instance_id.clone()));
            let args = DslValue::Object(arguments.into_iter().collect());
            let command = A::command_from_action(&address.action_id, Some(&args)).await?;""", """            let mut arguments = invocation.arguments.clone();
            arguments.insert("windowId".into(), DslValue::String(address.window_instance_id.clone()));
            self.fill_agent_revisions(address, &mut arguments).await?;
            let args = DslValue::Object(arguments.into_iter().collect());
            let command = A::command_from_action(&address.action_id, Some(&args)).await?;""")
edit(SDK, "agent-lane fill", """        /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
        /// **without applying any of them**.""", """        /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
        /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
        /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
        /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
        /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
        /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
        /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
        async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut std::collections::BTreeMap<String, DslValue>) -> Result<(), Fault> {
            let declared = match address.window_kind_id.as_str() {
                "*" => self.registry.get(&address.action_id).map(|action| action.args.clone()),
                kind => self.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
            }
            .unwrap_or_default();
            let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
            for arg in omitted.iter() {
                let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
                let token = match format {
                    semio_framework::manifest::ArgFormat::DocumentRevision => self.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                    semio_framework::manifest::ArgFormat::TargetRevision => {
                        self.refresh_cache().await?;
                        let render_operation = self.live_render_operation();
                        let parent_document_id = self.store.envelope().id.clone();
                        let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                        let (_, snapshot, _, history) = self.cache.as_ref().expect("cache refreshed above");
                        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_content_root), render_operation, parent_document_id, None).await;
                        A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                            Fault::new(
                                FaultOrigin::Framework,
                                FaultCode::new("agent.target-revision-unresolved"),
                                format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                            )
                        })?
                    }
                    _ => continue,
                };
                arguments.insert(arg.id.clone(), DslValue::String(token));
            }
            Ok(())
        }

        /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
        /// **without applying any of them**.""")

# ── stdio declarations ────────────────────────────────────────────────────────────────────────────────────────────────────
edit(STDIO_CONTRACT, "table cell (document)", """        ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(),
        ActionArgDef::text("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")).required(),
        ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),""", """        ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(),
        ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")),
        ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),""")
edit(STDIO_CONTRACT, "structural table verbs (document)", """    let revision = || ActionArgDef::text("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")).required();""",
     """    let revision = || ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision"));""")
edit(STDIO_CONTRACT, "xlsx cell (target)", """        ActionArgDef::text("revision", LocalizedLabel::native("Cell revision", "Zellrevision")).required(),""",
     """        ActionArgDef::target_revision("revision", LocalizedLabel::native("Cell revision", "Zellrevision")),""")
edit(WAV, "wav edit (document)", """    ActionArgDef::text("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")).required()""",
     """    ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision"))""")
for path in ZIP_WINDOWS:
    edit(path, "zip entry (target)", """            semio_framework_plugin::ActionArgDef::text("revision", LocalizedLabel::native("Saved revision", "Gespeicherte Revision")).required(),""",
         """            semio_framework_plugin::ActionArgDef::target_revision("revision", LocalizedLabel::native("Saved revision", "Gespeicherte Revision")),""")
for path in DOCX_WINDOWS:
    edit(path, "docx paragraph (target)", """                ActionArgDef::text("revision", LocalizedLabel::native("Revision", "Revision")).required(),""",
         """                ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")),""")
edit(PDF_PAGE, "pdf text item (target)", """ActionArgDef::text("revision", LocalizedLabel::native("Revision", "Revision")).min_length(1).required(), text()]""",
     """ActionArgDef::target_revision("revision", LocalizedLabel::native("Revision", "Revision")).min_length(1), text()]""")

# ── stdio target-scoped agent fills (the token each target's rendered binding carries) ─────────────────────────────────────
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
ZIP_EDITING = ARTIFACTS + "🎒️zip/✏️editor/🦀️.rs"
ZIP_EDITORS = [ARTIFACTS + "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs", ARTIFACTS + "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs"]
PDF_ROOT = ARTIFACTS + "📖️pdf/🦀️.rs"
PDF_EDITORS = [ARTIFACTS + "📖️pdf/🏅️standards/" + tail + "/✏️editor/🦀️.rs" for tail in ["7️⃣1.7/🪆️subsets/📐️e", "7️⃣1.7/🪆️subsets/⚕️h", "7️⃣1.7/🪆️subsets/🖨️x", "7️⃣1.7/🪆️subsets/🧱️base", "7️⃣1.7/🪆️subsets/🧾️vt", "7️⃣1.7/🪆️subsets/♿️ua", "7️⃣1.7/🪆️subsets/🗄️a", "4️⃣1.4/🪆️subsets/🖨️x", "4️⃣1.4/🪆️subsets/🧱️base", "4️⃣1.4/🪆️subsets/🗄️a"]]
XLSX_BASE = ARTIFACTS + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs"
XLSX_EDITORS = [ARTIFACTS + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs", ARTIFACTS + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs"]
EPW = ARTIFACTS + "🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs"
EPW_MAIN = ARTIFACTS + "🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"
EDITOR_ANCHOR = """    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {"""


def editor_hook(path, call):
    edit(path, "agent_target_revision hook", EDITOR_ANCHOR, f"""    fn agent_target_revision(_action: &str, args: &dsl::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {{
        {call}(doc.snapshot, args)
    }}
""" + EDITOR_ANCHOR)


edit(ZIP_EDITING, "zip agent target revision", """/// 🔐️ Guards an archive draft against changes to its persisted text.
pub fn text_revision(value: &str) -> String {""", """/// 🔐️ The token an agent's omitted `revision` is admitted against: the addressed field's saved text, as its draft binding
/// carries it (`draft`); a missing or ambiguous entry is refused exactly as the edit itself would refuse it.
pub fn agent_target_revision(snapshot: &ZipSnapshot, args: &dsl::DslValue) -> Result<Option<String>, Fault> {
    let node_id = semio_s_artifact_stdio_contract::window_kit_required_text_argument(Some(args), "nodeId")?;
    if node_id == COMMENT_NODE_ID {
        return Ok(Some(text_revision(&snapshot.comment)));
    }
    let mut named = snapshot.entries.iter().filter(|entry| entry_node_id(&entry.name) == node_id);
    match (named.next(), named.next()) {
        (Some(entry), None) => Ok(Some(text_revision(&entry.name))),
        (Some(_), Some(_)) => Err(fault("stdio.zip.target-ambiguous", "entries with identical names must be disambiguated before renaming")),
        (None, _) => Err(fault("stdio.zip.target-missing", "the archive entry changed or no longer exists")),
    }
}

/// 🔐️ Guards an archive draft against changes to its persisted text.
pub fn text_revision(value: &str) -> String {""")
for path in ZIP_EDITORS:
    editor_hook(path, "crate::editor::editing::agent_target_revision")

edit(PDF_ROOT, "pdf agent target revision", """/// 📄️ Builds one exact, reversible PDF page-text replacement or rejects an invalid/stale target.""", """/// 🔐️ The token an agent's omitted `set-page` revision is admitted against: the addressed page text's own token, as its draft
/// binding carries it; a missing page is refused exactly as the edit itself would refuse it.
pub fn page_text_agent_revision(snapshot: &PdfSnapshot, args: &dsl::DslValue) -> Result<Option<String>, semio_framework_plugin::Fault> {
    let page_index = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "page")? as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pdf.set-page.stale-target"), format!("PDF page {page_index} no longer exists"))
    })?;
    Ok(Some(semio_framework_plugin::app::DocumentWindowKit::text_revision(&target.text())))
}

/// 📄️ Builds one exact, reversible PDF page-text replacement or rejects an invalid/stale target.""")
for path in PDF_EDITORS:
    editor_hook(path, "crate::page_text_agent_revision")

edit(XLSX_BASE, "xlsx agent target revision", """fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxEditorCommand) -> Result<Emit<XlsxMutation>, Fault> {""", """/// 🔐️ The token an agent's omitted `set-cell` revision is admitted against: the addressed cell's own token, as its rendered
/// binding carries it; a stale address is refused exactly as the edit itself would refuse it.
pub(crate) fn xlsx_agent_target_revision(snapshot: &XlsxSnapshot, args: &dsl::DslValue) -> Result<Option<String>, Fault> {
    let sheet_name = semio_s_artifact_stdio_contract::window_kit_required_text_argument(Some(args), "sheetName")?;
    let row = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "row")?;
    let column = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "column")?;
    xlsx_cell_address(snapshot, &sheet_name, row, column).map(|address| Some(address.revision)).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.cell-stale"), message))
}

fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxEditorCommand) -> Result<Emit<XlsxMutation>, Fault> {""")
editor_hook(XLSX_BASE, "xlsx_agent_target_revision")
for path in XLSX_EDITORS:
    editor_hook(path, "crate::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_agent_target_revision")

edit(EPW, "epw agent target revision", """fn epw_set_cell_emit(snapshot: &EpwSnapshot, row: u32, column: &str, revision: &str, value: &str) -> Result<Emit<EpwMutation>, Fault> {""", """/// 🔐️ The token an agent's omitted `set-cell` revision is admitted against: the addressed record's row token, as its rendered
/// binding carries it; a row out of range is refused exactly as the edit itself would refuse it.
fn epw_agent_target_revision(snapshot: &EpwSnapshot, args: &dsl::DslValue) -> Result<Option<String>, Fault> {
    let row = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "row")?;
    let record = snapshot
        .records
        .get(row as usize)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.row-range"), format!("EPW row {row} is outside the {} available records", snapshot.records.len())))?;
    Ok(Some(epw_row_revision(record)))
}

fn epw_set_cell_emit(snapshot: &EpwSnapshot, row: u32, column: &str, revision: &str, value: &str) -> Result<Emit<EpwMutation>, Fault> {""")
editor_hook(EPW, "epw_agent_target_revision")
edit(STDIO_CONTRACT, "row-revision table kind", """pub const ADD_TABLE_ROW_ACTION_ID: &str = "add-row";""", """/// 🔐️ Declares a table cell address guarded by its row's own revision — a target token, not the document's.
pub fn row_revision_addressed_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::{ActionArgDef, LocalizedLabel};
    let mut definition = revision_addressed_table_window_kind();
    let action = definition.actions.iter_mut().find(|action| action.id == "set-cell").expect("the revision-addressed table declares set-cell");
    let revision = action.args.iter_mut().find(|argument| argument.id == "revision").expect("set-cell declares revision");
    *revision = ActionArgDef::target_revision("revision", LocalizedLabel::native("Row revision", "Zeilenrevision"));
    definition
}

pub const ADD_TABLE_ROW_ACTION_ID: &str = "add-row";""")
edit(EPW_MAIN, "epw row-revision table", """..semio_s_artifact_stdio_contract::revision_addressed_table_window_kind() }""", """..semio_s_artifact_stdio_contract::row_revision_addressed_table_window_kind() }""")

# ── laws: SDK agent preview testkit + csv (document scope) + zip (target scope) ────────────────────────────────────────────
CSV_TESTS = ARTIFACTS + "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
ZIP_TESTS = ARTIFACTS + "🎒️zip/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
edit(SDK, "testkit agent_preview", """        /// 🤖️ Dispatches one declared verb exactly as an agent does — the owner-qualified invocation the""", """        /// 🤖️ Previews `verb` with `args` exactly as the MCP gateway's `action_prepare` does — the agent lane's typed command
        /// frame, addressed to the first window kind of `definition` that declares the verb — and answers how many document ops
        /// the preview would commit (ticket 26/09/23, G12 session 14c).
        pub async fn agent_preview<A, M>(app: &mut VcsArtifactApp<A, M>, definition: &semio_framework::AppDefinition, verb: &str, args: &semio_framework::DslValue) -> Result<usize, super::Fault>
        where
            A: ArtifactApp + Default,
            M: super::SpaceMember + super::MemberFactory + Send + 'static,
        {
            let window_kind_id = definition.window_kinds.iter().find(|window| semio_framework::window_kind_actions(definition, window).iter().any(|action| action.id == verb)).map_or_else(|| "*".to_string(), |window| window.id.clone());
            let arguments = match args {
                semio_framework::DslValue::Object(entries) => entries.iter().cloned().collect(),
                _ => std::collections::BTreeMap::new(),
            };
            let invocation = super::ManifestActionInvocation {
                address: semio_framework::manifest::ActionAddress { plugin_id: String::new(), app_id: app.app.instance_id().await.to_string(), mode_id: definition.default_mode_id.clone(), window_kind_id, window_instance_id: "agent-preview".to_string(), action_id: verb.to_string() },
                arguments,
            };
            let result = app.preview_addressed_action(&invocation, &meta("agent")).await?;
            Ok(result.output.get("documentOps").and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0))
        }

        /// 🤖️ Dispatches one declared verb exactly as an agent does — the owner-qualified invocation the""")
edit(CSV_TESTS, "csv agent revision law", """//#endregion 🪟️KitVerbLaws""", """/// ⚖️ LAW (agent lane, ticket 26/09/23 G12 session 14c): an agent's `set-cell` without a `revision` is admitted against the
/// document's revision — it previews the edit exactly as with the token the rendered cell carries, a stale token is still
/// refused — while the shell lane keeps refusing the omission.
#[semio_framework_async_macros::async_test]
async fn an_agent_set_cell_without_a_revision_previews_against_the_document_revision_and_the_shell_lane_still_requires_it() {
    let source = csv_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&source).await;
    let definition = create_csv_editor();
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let address = |revision: Option<String>| {
        dsl::DslValue::object(
            [("row".to_string(), dsl::DslValue::Number(dsl::Number::UInt(0))), ("column".to_string(), dsl::DslValue::Number(dsl::Number::UInt(0))), ("value".to_string(), dsl::DslValue::String("Zeta".into()))]
                .into_iter()
                .chain(revision.map(|revision| ("revision".to_string(), dsl::DslValue::String(revision)))),
        )
    };
    let omitted = semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(None)).await.expect("the agent lane admits an omitted revision");
    let bound = semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(Some(revision))).await.expect("the rendered cell's revision");
    assert!(omitted > 0 && omitted == bound, "omitted {omitted} vs bound {bound} document ops");
    assert!(semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(Some("stale".into()))).await.is_err(), "a stale token is refused on the agent lane too");
    assert!(dispatch_settled(&mut app, "set-cell", address(None)).await.is_err(), "the shell lane still requires the rendered cell's revision");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws""")
edit(ZIP_TESTS, "zip agent revision law", """#[semio_framework_async_macros::async_test]
async fn base_archive_draft_uses_the_registered_retained_factory() {""", """/// ⚖️ LAW (agent lane, ticket 26/09/23 G12 session 14c): an agent's archive rename without a `revision` is admitted against the
/// addressed entry's own token — the same edit as with the token its draft binding carries, a stale token still refused —
/// while the shell lane keeps refusing the omission.
async fn agent_archive_rename_law<E: ArtifactEditor<Snapshot = ZipSnapshot, Mutation = ZipMutation>>(definition: semio_framework_plugin::AppDefinition) {
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};
    let row = fixture();
    let original: ZipSnapshot = dsl::os_pack::json::from_json_str(&row["snapshot"].to_string()).unwrap();
    let registered = definition.clone();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<E>, _>(async { semio_framework_plugin::App { definition: registered, examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, crate::STDIO_ZIP_DOCUMENT_SCHEMA) else { panic!("archive fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let name = row["rename"]["name"].as_str().unwrap();
    let value = row["rename"]["value"].as_str().unwrap();
    let address = |revision: Option<String>| {
        dsl::DslValue::object(
            [("nodeId".to_string(), dsl::DslValue::String(entry_node_id(name))), ("value".to_string(), dsl::DslValue::String(value.into()))].into_iter().chain(revision.map(|revision| ("revision".to_string(), dsl::DslValue::String(revision)))),
        )
    };
    assert_eq!(agent_target_revision(&original, &address(None)).unwrap(), Some(text_revision(name)), "the fill is the token the draft binding carries");
    let omitted = artifact_app_laws::agent_preview(&mut app, &definition, "set-node", &address(None)).await.expect("the agent lane admits an omitted revision");
    let bound = artifact_app_laws::agent_preview(&mut app, &definition, "set-node", &address(Some(text_revision(name)))).await.expect("the draft binding's revision");
    assert!(omitted > 0 && omitted == bound, "omitted {omitted} vs bound {bound} document ops");
    assert!(artifact_app_laws::agent_preview(&mut app, &definition, "set-node", &address(Some(text_revision("stale")))).await.is_err(), "a stale token is refused on the agent lane too");
    assert!(app.handle_action("set-node", Some(&address(None)), &artifact_app_laws::meta("local")).await.is_err(), "the shell lane still requires the draft binding's revision");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn base_archive_agent_rename_is_admitted_against_the_entry_token() {
    agent_archive_rename_law::<crate::editor::zip::base::ZipAnyEditor>(crate::editor::zip::base::create_zip_any_editor()).await;
}

#[semio_framework_async_macros::async_test]
async fn iso_archive_agent_rename_is_admitted_against_the_entry_token() {
    agent_archive_rename_law::<crate::editor::zip::iso21320::ZipIso21320Editor>(crate::editor::zip::iso21320::create_zip_iso21320_editor()).await;
}

#[semio_framework_async_macros::async_test]
async fn base_archive_draft_uses_the_registered_retained_factory() {""")

# ── gateway: an omitted revision only behind a document-level stale-write guard ────────────────────────────────────────────
DISPATCH = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs"
DISPATCH_TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs"
edit(DISPATCH, "revision guard helpers", """//#region 🔖️PublicReports
/// 📨️ `action.invoke`'s combined input""", """//#region 🔖️RevisionGuard
/// 🏷️ The typed refusal of a one-shot invoke that omits a revision argument and names no `expectedRevision`.
pub const REVISION_GUARD_REQUIRED_FAULT_CODE: &str = "revision.guard-required";

/// 🔐️ The revision arguments `capability` declares (`x-semio-format` `documentRevision` / `targetRevision`) that `input` omits.
/// The plugin's agent lane admits each against the token its rendered binding carries at admission — only behind a
/// document-level stale-write guard: a prepared handle (its prepare-time baseline is re-checked at invoke) or an explicit
/// `expectedRevision` (ticket 26/09/23, G12 session 14c).
pub fn omitted_revision_arguments(capability: &crate::catalog::CapabilityDefinition, input: &serde_json::Value) -> Vec<String> {
    capability
        .input_schema
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .map(|properties| {
            properties
                .iter()
                .filter(|(name, schema)| matches!(schema.get("x-semio-format").and_then(serde_json::Value::as_str), Some("documentRevision" | "targetRevision")) && input.get(name.as_str()).is_none())
                .map(|(name, _)| name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// 🚫️ Nothing would guard such a write against a document that moved since the agent read it, so it is refused by name.
fn revision_guard_required(capability_id: &str, omitted: &[String]) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("capability {capability_id} omits its revision argument(s) {} and names no expectedRevision — nothing would guard this write against a document that moved since it was read", omitted.join(", "))).with_details(serde_json::json!({
        "faultCode": REVISION_GUARD_REQUIRED_FAULT_CODE,
        "omitted": omitted,
        "remedy": {
            "en": "Prepare the action first (action_prepare, then action_invoke with its handle) or pass the document's expectedRevision.",
            "de": "Bereite die Aktion zuerst vor (action_prepare, dann action_invoke mit ihrem Handle) oder gib die expectedRevision des Dokuments mit.",
        },
    }))
}
//#endregion 🔖️RevisionGuard

//#region 🔖️PublicReports
/// 📨️ `action.invoke`'s combined input""")
edit(DISPATCH, "revision guard at one-shot invoke", """            let input = request.input.clone().unwrap_or_else(|| serde_json::json!({}));
            let prepared = self.prepare(catalog, principal, session, &capability_id, input, instance, now_ms)?;""", """            let input = request.input.clone().unwrap_or_else(|| serde_json::json!({}));
            if request.expected_revision.is_none() {
                let omitted = catalog.get(&capability_id).map(|capability| omitted_revision_arguments(capability, &input)).unwrap_or_default();
                if !omitted.is_empty() {
                    let error = revision_guard_required(&capability_id, &omitted);
                    self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &capability_id, raw_input: &input }, AuditDecision::Denied { code: error.code }, None, None, "revision_guard_required", Some(error.clone()), None, now_ms);
                    return Err(error);
                }
            }
            let prepared = self.prepare(catalog, principal, session, &capability_id, input, instance, now_ms)?;""")
edit(DISPATCH_TESTS, "revision guard laws", """#[test]
fn stale_expected_revision_is_a_revision_conflict_with_no_mutation_sent() {""", """//#region 🔖️RevisionGuardLaws
fn revision_bound_capability() -> CapabilityDefinition {
    let mut capability = synthetic_capability("table.set-cell", &["artifact.write"], ApprovalMode::Never, false);
    capability.input_schema = serde_json::json!({ "type": "object", "properties": { "row": { "type": "integer" }, "revision": { "type": "string", "x-semio-format": "documentRevision" } }, "required": ["row"] });
    capability
}

/// 🔐️ A one-shot invoke that omits its revision argument and names no `expectedRevision` is refused by name before anything
/// reaches the guest — no lane writes last-writer-wins (ticket 26/09/23, G12 session 14c).
#[test]
fn an_omitted_revision_without_a_document_guard_is_refused_by_name_and_nothing_is_sent() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = single_capability_catalog(revision_bound_capability());
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("table.set-cell".into()), input: Some(serde_json::json!({ "row": 0 })), ..Default::default() }, 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PreconditionFailed);
    assert_eq!(error.details.get("faultCode").and_then(serde_json::Value::as_str), Some(REVISION_GUARD_REQUIRED_FAULT_CODE));
    assert!(error.details.pointer("/remedy/de").and_then(serde_json::Value::as_str).is_some_and(|remedy| !remedy.is_empty()));
    assert!(channel.frame_log().is_empty(), "nothing may reach the guest: {:?}", channel.frame_log());
}

/// 🔐️ Behind a passed document-level guard the omission is admitted — the prepared handle (its baseline re-checked at invoke)
/// and an explicit `expectedRevision` — and a stale `expectedRevision` is a revision conflict with no mutation sent.
#[test]
fn an_omitted_revision_behind_a_passed_document_guard_commits_and_a_stale_guard_is_a_conflict() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = single_capability_catalog(revision_bound_capability());
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    let request = |expected_revision: Option<RevisionStamp>| InvokeRequest { capability_id: Some("table.set-cell".into()), input: Some(serde_json::json!({ "row": 0 })), expected_revision, ..Default::default() };
    let prepared = adapter.prepare(&catalog, &principal, &session, "table.set-cell", serde_json::json!({ "row": 0 }), 0, 0).unwrap();
    let via_handle = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();
    assert_eq!(via_handle.status, InvocationStatus::Succeeded);
    let current = adapter.prepare(&catalog, &principal, &session, "table.set-cell", serde_json::json!({ "row": 0 }), 0, 2).unwrap().expected_revision.expect("a baseline");
    let guarded = adapter.invoke(&catalog, &principal, &session, request(Some(current.clone())), 0, 3).unwrap();
    assert_eq!(guarded.status, InvocationStatus::Succeeded);
    let before_stale = channel.frame_log().len();
    let error = adapter.invoke(&catalog, &principal, &session, request(Some(current)), 0, 4).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::RevisionConflict);
    assert!(!channel.frame_log()[before_stale..].iter().any(|(_, command)| matches!(command, AppCommand::TransactionPrepare { .. } | AppCommand::TransactionCommit { .. })), "a stale guard sent a mutation");
}
//#endregion 🔖️RevisionGuardLaws

#[test]
fn stale_expected_revision_is_a_revision_conflict_with_no_mutation_sent() {""")


def main():
    if "--list" in sys.argv:
        print("\n".join(EDITS))
        return
    problems, changed = [], {}
    for path, edits in EDITS.items():
        file = ROOT / path
        if not file.exists():
            problems.append(f"{path}: missing")
            continue
        text = file.read_text(encoding="utf-8")
        patched = text
        for name, old, new, count in edits:
            if patched.count(new) >= count and new not in old:
                print(f"  {path.split('/')[-4:]} {name}: applied")
                continue
            if patched.count(old) != count:
                problems.append(f"{path} {name}: anchor count {patched.count(old)} (want {count})")
                continue
            patched = patched.replace(old, new)
            print(f"  {path.split('/')[-4:]} {name}: pending")
        if patched != text:
            changed[file] = patched
    print(f"root {ROOT}: {len(changed)} file(s) to change, {len(problems)} problem(s)")
    for problem in problems:
        print(f"  PROBLEM {problem}")
    if problems:
        sys.exit(1)
    if WRITE:
        for file, text in changed.items():
            file.write_text(text, encoding="utf-8")
        print(f"wrote {len(changed)} file(s)")
    elif not changed:
        print("nothing to do (applied)")


main()
