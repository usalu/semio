#!/usr/bin/env python3
"""📚️ EX1: every example loader loads its example for real or refuses by code — never an empty, genesis or other document.

Applies AFTER T6 row 12 (fault localization: `with_parameter`, the framework fault catalog, `app_fault`, per-plugin `.fault`
declarations). Structural edits (functions located by signature and brace-matched, loaders by pattern), so the same script
plans against the live tree and against a post-row-12 overlay; `--dry-run` reports per edit `apply` / `applied` / `PROBLEM`.

ONE resolver (SDK, `🔌️plugin/🦀️.rs` module `app`):
- `example_snapshot::<S>(examples, id)`: the published example's body decoded through the snapshot's own codec; an id nothing
  publishes → `app.example.unknown` {example}; a body the codec cannot read → `app.example.unreadable` {example}.
- `editor_example_snapshot::<E>(id)`: the empty id is the editor's initial document (the shell's "default document"
  request), every other id → `example_snapshot` over `editor_examples::<E>()` (the picker's list: its own, else its subset's).
- laws (`artifact_app_laws`): `assert_examples_load::<S>` (catalog rows == published ids; every example decodes, is not the
  empty `Default` unless its row says so, round-trips DSL + pack; an unpublished id is refused by code) and
  `assert_set_active_example_loads::<A, M>` (every example loads through `setActiveExample` on a fresh registered fixture and
  writes the document unless it IS the genesis document; an unpublished id is refused `app.example.unknown`).
- language-agnostic fixture per plugin `🧫️fixtures/📚️example-catalog/🔣️.json` (schema `semio.example-catalog.v1`,
  `🔌️plugin/🧬️schema/📚️example-catalog/🔣️.json`).

Sections: `sdk`, `stdio` (folds LB2's `lb2-p18-example-refusal.py`: the 62 stdio loaders, their 11 unit-test files, the
editor-catalog law), then one section per plugin.

usage: python3 ex1-example-loaders.py [--dry-run | --write | --revert] [--root <tree>] [--only <section>…]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s15-ex1-backup/<root-hash>/`; created files listed in `created.txt` there.
"""
import hashlib, json, os, re, shutil, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
ARGS = sys.argv[1:]
TREE = ARGS[ARGS.index("--root") + 1] if "--root" in ARGS else REPO
ONLY = ARGS[ARGS.index("--only") + 1:] if "--only" in ARGS else None
BACKUP = f"{REPO}/.🧬semio/🌐hub/s15-ex1-backup/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
HERE = os.path.dirname(os.path.abspath(__file__))
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
SDK_DUMMY = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs"
CATALOG = "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json"
SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📚️example-catalog/🔣️.json"
PLUGINS = "✏️s/🔌️plugins"

problems, notes = [], []


class Plan:
    """🗂️ Edits (path → [fn(text) -> text]) and creations (path → text) of one run."""

    def __init__(self):
        self.edits, self.creates, self.deletes, self.copies = {}, {}, [], {}

    def edit(self, path, fn):
        self.edits.setdefault(path, []).append(fn)

    def create(self, path, text):
        self.creates[path] = text

    def delete(self, path):
        self.deletes.append(path)

    def copy(self, source, target, overwrite=False):
        """📦️ `target` gets `source`'s exact bytes (binary-safe; `source` may be deleted by the same run — read at staging). A
        move (`overwrite=False`) is applied once `target` exists; an overwrite once `source` is gone or the bytes agree."""
        self.copies[target] = (source, overwrite)


def read(path):
    with open(os.path.join(TREE, path), encoding="utf-8") as handle:
        return handle.read()


def once(text, old, new, label):
    """✂️ Replaces the one `old` by `new`; `new` already present and `old` absent is `applied`."""
    if new in text and (old not in text or old in new):
        notes.append(f"applied {label}")
        return text
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def item_span(text, start):
    """🧭️ `[start, end)` of the Rust item whose header starts at `start`: through the brace that closes its body."""
    open_at = text.index("{", start)
    depth, at = 0, open_at
    while at < len(text):
        char = text[at]
        if char == '"':
            at += 1
            while text[at] != '"':
                at += 2 if text[at] == "\\" else 1
        elif char == "'" and re.match(r"'(?:\\.|[^\\'])'", text[at:at + 4]):
            at += text[at + 1:at + 4].index("'") + 1
        elif char == "/" and text[at + 1:at + 2] == "/":
            at = text.index("\n", at) - 1
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                end = at + 1
                return start, end + (1 if text[end:end + 1] == "\n" else 0)
        at += 1
    raise ValueError("unbalanced item")


def replace_item(text, header, docstring_start, new, label, gone_marker):
    """🔁️ Replaces the item whose header line is `header` (with the doc comment starting at `docstring_start` above it) by
    `new`; `gone_marker` absent and `new` present is `applied`."""
    if header not in text:
        if new in text and gone_marker not in text:
            notes.append(f"applied {label}")
        else:
            problems.append(f"{label}: header not found")
        return text
    at = text.index(header)
    doc = text.rfind(docstring_start, 0, at)
    if doc < 0 or text[doc:at].count("\n") > 12:
        problems.append(f"{label}: doc comment not found above the header")
        return text
    start, end = item_span(text, at)
    return text[:doc] + new + text[end:]


def replace_fn(text, header, new, label):
    """🔁️ Replaces the fn whose header line starts with `header` (exact, incl. indentation), together with the contiguous
    `///` / `#[…]` / `// 🚫️async` lines right above it, by `new`; `new` already present and `header` absent is `applied`."""
    if header not in text:
        if new in text:
            notes.append(f"applied {label}")
        else:
            problems.append(f"{label}: header not found")
        return text
    at = text.index(header)
    if text.count(header) != 1:
        problems.append(f"{label}: {text.count(header)} headers")
        return text
    lines = text[:at].split("\n")[:-1]
    top = at
    for line in reversed(lines):
        if re.match(r"\s*(///|#\[|// 🚫️async)", line):
            top -= len(line) + 1
        else:
            break
    _, end = item_span(text, at)
    return text[:top] + new + text[end:]


def drop_fault(text, code, label):
    """🧹️ Removes the one-line `.fault("<code>", …)` declaration of a code nothing raises any more."""
    line = re.search(rf'\n[ \t]*\.fault\("{re.escape(code)}", [^\n]*', text)
    if line is None:
        notes.append(f"applied {label}")
        return text
    body = line.group(0)
    if body.rstrip().endswith(";"):
        head = text[: line.start()]
        return head + ";" + text[line.end():]
    return text[: line.start()] + text[line.end():]


def drop_use(text, name, label):
    """🧹️ Removes `name` from the one `use …::{…};` list (or `use …::name;`) that imports it — the import went unused."""
    body = re.sub(r"(?m)//.*$", "", re.sub(r"(?m)^[ \t]*use [^;]*;", "", text))
    if re.search(rf"(?<![\w:]){re.escape(name)}(?!\w)", body):
        notes.append(f"kept {label}: still used")
        return text
    for found in re.finditer(r"use ([\w:]+)::\{([^}]*)\};", text):
        items = [item.strip() for item in found.group(2).split(",") if item.strip()]
        if name in items:
            items.remove(name)
            replacement = f"use {found.group(1)}::{{{', '.join(items)}}};" if len(items) != 1 else f"use {found.group(1)}::{items[0]};"
            return text[: found.start()] + replacement + text[found.end():]
    single = re.search(rf"\n[ \t]*use [\w:]+::{re.escape(name)};", text)
    if single:
        return text[: single.start()] + text[single.end():]
    notes.append(f"applied {label}")
    return text


# ─── sdk ──────────────────────────────────────────────────────────────────────────────────────────────────────────────

SDK_RESOLVER = '''    /// 📚️ The document the example `example_id` of `examples` IS: its registered body decoded through the snapshot's own
    /// codec. The ONE resolver every `setActiveExample` loads through — an id `examples` does not publish is refused
    /// `app.example.unknown`, a body the codec cannot read `app.example.unreadable`, both naming the example; never an
    /// empty, genesis or other document in its place.
    pub fn example_snapshot<S>(examples: &[ExampleSource], example_id: &str) -> Result<S, Fault>
    where
        S: store::ArtifactDsl + protocol::FromValue,
    {
        let source = examples.iter().find(|source| source.id() == example_id).ok_or_else(|| unknown_example(example_id))?;
        example_body_snapshot(example_id, &source.document())
    }

    /// 📚️ The refusal of an example id no example list publishes — `app.example.unknown` naming the example. Raised by
    /// [`example_snapshot`], and by retained example work that walks pre-decoded example documents instead of decoding.
    pub fn unknown_example(example_id: &str) -> Fault {
        Fault::new(FaultOrigin::Framework, FaultCode::new("app.example.unknown"), format!("no registered example '{example_id}'")).with_parameter("example", example_id)
    }

    /// 📚️ The document `setActiveExample` loads in editor `E`: the empty id is the editor's own initial document (the
    /// shell's "default document" request); every other id resolves through [`example_snapshot`] over the examples the
    /// editor's picker offers ([`editor_examples`]).
    pub fn editor_example_snapshot<E: ArtifactEditor>(example_id: &str) -> Result<E::Snapshot, Fault> {
        if example_id.is_empty() {
            return Ok(E::initial_snapshot());
        }
        example_snapshot(&editor_examples::<E>(), example_id)
    }

    /// 📚️ [`editor_example_snapshot`] over the examples a subset declaration publishes, handed in by the subset itself: the
    /// empty id is the editor's initial document, every other id resolves through [`example_snapshot`] — without waiting for
    /// plugin assembly to register the subset's examples for [`editor_examples`].
    pub fn subset_example_snapshot<E: ArtifactEditor>(examples: &[ExampleSource], example_id: &str) -> Result<E::Snapshot, Fault> {
        if example_id.is_empty() {
            return Ok(E::initial_snapshot());
        }
        example_snapshot(examples, example_id)
    }

    /// 📚️ The examples editor `E`'s picker offers: its own, else the ones its subset declares.
    pub fn editor_examples<E: ArtifactEditor>() -> Vec<ExampleSource> {
        let examples = E::examples();
        if examples.is_empty() {
            return declarations::subset_examples(&E::DIALECT);
        }
        examples
    }

    /// 📚️ Decodes the registered body of the example `example_id` into the app snapshot. A `{` body is the JSON a deferred
    /// producer emits; anything else is the subset's own DSL.
    pub(crate) fn example_body_snapshot<S>(example_id: &str, body: &str) -> Result<S, Fault>
    where
        S: store::ArtifactDsl + protocol::FromValue,
    {
        if body.trim_start().starts_with('{') {
            dsl::os_pack::json::from_json_str(body).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("app.example.unreadable"), error.to_string()).with_parameter("example", example_id))
        } else {
            <S as store::ArtifactDsl>::parse_dsl(body).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("app.example.unreadable"), error.to_string()).with_parameter("example", example_id))
        }
    }
'''

SDK_LAWS = '''        /// 📚️ The rows of `app` in a `semio.example-catalog.v1` fixture (schema `🔌️plugin/🧬️schema/📚️example-catalog/🔣️.json`):
        /// the ids of the examples it publishes, each with whether its document IS the snapshot type's `Default`.
        fn example_catalog_rows(fixture: &str, app: &str, expected: impl Fn() -> String) -> Vec<(String, bool)> {
            let fixture: serde_json::Value = serde_json::from_str(fixture).expect("the example catalog is JSON");
            assert_eq!(fixture["schema"], "semio.example-catalog.v1", "the example catalog names its schema");
            let apps = fixture["apps"].as_array().expect("the example catalog lists its apps");
            let row = apps.iter().find(|row| row["app"] == app).unwrap_or_else(|| panic!("the example catalog lists no app {app}; its row is {}", expected()));
            let examples = row["examples"].as_array().expect("an app row lists its examples");
            examples.iter().map(|example| (example["id"].as_str().expect("an example row names its id").to_string(), example["document"] == "default")).collect()
        }

        /// 📚️ The catalog row `app` answers for `examples` — printed by a failing law so the fixture is corrected from it.
        fn example_catalog_row<S>(app: &str, examples: &[super::ExampleSource], dispose: &dyn Fn(S)) -> String
        where
            S: store::ArtifactDsl + protocol::FromValue + Default + PartialEq,
        {
            let rows = examples.iter().map(|source| {
                let document = match super::example_snapshot::<S>(examples, source.id()) {
                    Ok(document) => {
                        let is_default = document == S::default();
                        dispose(document);
                        if is_default { "default" } else { "distinct" }
                    }
                    Err(_) => "unreadable",
                };
                format!("{{\\"id\\": {}, \\"document\\": \\"{document}\\"}}", serde_json::Value::from(source.id()))
            });
            format!("{{\\"app\\": {}, \\"examples\\": [{}]}}", serde_json::Value::from(app), rows.collect::<Vec<_>>().join(", "))
        }

        /// ⚖️ LAW (resolver): the examples `app` publishes are exactly its catalog rows; each decodes through
        /// [`super::example_snapshot`] into a document that is not the empty `S::default()` (unless its row says it IS the
        /// default) and round-trips through both document codecs — DSL text and pack; an unpublished id is refused
        /// `app.example.unknown`. Answers the number of examples checked, so a caller pins it.
        pub fn assert_examples_load<S>(fixture: &str, app: &str, examples: &[super::ExampleSource], dispose: &dyn Fn(S)) -> usize
        where
            S: store::ArtifactDsl + store::ArtifactPack + protocol::FromValue + Default + PartialEq,
        {
            let expected = || example_catalog_row::<S>(app, examples, dispose);
            let rows = example_catalog_rows(fixture, app, expected);
            let mut published = examples.iter().map(|source| source.id().to_string()).collect::<Vec<_>>();
            let mut listed = rows.iter().map(|(id, _)| id.clone()).collect::<Vec<_>>();
            published.sort();
            listed.sort();
            assert!(published == listed, "{app} publishes exactly the examples its catalog lists; its row is {}", expected());
            for (id, is_default) in &rows {
                let document = super::example_snapshot::<S>(examples, id).unwrap_or_else(|fault| panic!("{app} example {id} loads: {} {}", fault.code.0, fault.message));
                assert!((document == S::default()) == *is_default, "{app} example {id} is {} the empty default document; its row is {}", if *is_default { "exactly" } else { "not" }, expected());
                let reparsed = <S as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&document)).unwrap_or_else(|error| panic!("{app} example {id} re-parses from its printed DSL: {error:?}"));
                assert!(reparsed == document, "{app} example {id} round-trips through its DSL codec");
                let reopened = <S as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&document)).unwrap_or_else(|error| panic!("{app} example {id} reopens from its pack: {error:?}"));
                assert!(reopened == document, "{app} example {id} round-trips through its pack codec");
                dispose(reparsed);
                dispose(reopened);
                dispose(document);
            }
            let refusal = super::example_snapshot::<S>(examples, "semio.example-catalog.unpublished").err().unwrap_or_else(|| panic!("{app} refuses an unpublished example id"));
            assert_eq!(refusal.code.0, "app.example.unknown", "{app} refuses an unpublished example id by code");
            rows.len()
        }

        /// ⚖️ LAW (handler): `setActiveExample` on a fresh registered fixture of `definition` loads every example its catalog
        /// lists — it settles without a refusal and writes the document unless the example IS the genesis document — and an
        /// unpublished id is refused `app.example.unknown` by code: never a silent no-op, never an empty document. An app
        /// that neither declares the verb nor publishes an example has no picker and answers 0.
        pub async fn assert_set_active_example_loads<A, M>(fixture: &str, definition: &semio_framework::AppDefinition, examples: &[super::ExampleSource], dispose: &dyn Fn(A::Snapshot)) -> usize
        where
            A: ArtifactApp + Default,
            M: super::SpaceMember + super::MemberFactory + Send + 'static,
        {
            use semio_framework::{window_kind_actions, DslValue};
            let rows = example_catalog_rows(fixture, &definition.id, || format!("the one the resolver law prints for {}", definition.id));
            let presenting = definition.window_kinds.iter().position(|window| window_kind_actions(definition, window).iter().any(|action| action.id == "setActiveExample"));
            if presenting.is_none() && rows.is_empty() {
                return 0;
            }
            let index = presenting.unwrap_or(0);
            let published = examples.iter().map(|source| source.id()).collect::<std::collections::BTreeSet<_>>();
            let form = window_kind_actions(definition, &definition.window_kinds[index]).into_iter().find(|action| action.id == "setActiveExample").and_then(|action| action.args.iter().find(|argument| argument.id == "exampleId").cloned());
            if let Some(argument) = form {
                if let semio_framework::ArgSchema::String { options, .. } = &argument.schema {
                    let offered = options.iter().map(|option| option.value.as_str()).filter(|value| !value.is_empty()).collect::<std::collections::BTreeSet<_>>();
                    assert!(options.is_empty() || offered == published, "{} offers exactly the examples it publishes (and at most the empty No example row) in its setActiveExample form: offers {offered:?}, publishes {published:?}", definition.id);
                }
                if let Some(default) = argument.default.as_ref().and_then(DslValue::as_str) {
                    assert!(published.contains(default), "{} defaults its setActiveExample form to a published example, not {default:?}", definition.id);
                }
            }
            let instances = definition.window_kinds.iter().enumerate().map(|(index, window)| semio_framework::ViewWindowInstance { id: format!("example-catalog-{index}"), window_kind_id: window.id.clone() }).collect::<Vec<_>>();
            let view = super::ViewModel {
                active_mode_id: Some(definition.default_mode_id.clone()),
                active_window_kind_id: Some(definition.window_kinds[index].id.clone()),
                window_id: Some(instances[index].id.clone()),
                window_instances: instances.clone(),
                ..Default::default()
            };
            let body_key = definition.window_kinds[index].body_key.clone();
            let genesis = A::initial_snapshot().await;
            let args = |id: &str| DslValue::object([("exampleId".to_string(), DslValue::String(id.to_string()))]);
            for (id, _) in &rows {
                let document = super::example_snapshot::<A::Snapshot>(examples, id).unwrap_or_else(|fault| panic!("{} example {id} loads: {} {}", definition.id, fault.code.0, fault.message));
                match declared_verb_dispatch::<A, M>(definition, None, "setActiveExample", &args(id), &view, &body_key).await {
                    DeclaredVerbOutcome::Settled(effect) => assert!(document == genesis || effect.touches_document(), "{} example {id} writes the document it names", definition.id),
                    other => panic!("{} example {id} loads through setActiveExample: {other:?}", definition.id),
                }
                dispose(document);
            }
            dispose(genesis);
            match declared_verb_dispatch::<A, M>(definition, None, "setActiveExample", &args("semio.example-catalog.unpublished"), &view, &body_key).await {
                DeclaredVerbOutcome::Refused { code, .. } => assert_eq!(code, "app.example.unknown", "{} refuses an unpublished example id by code", definition.id),
                other => panic!("{} refuses an unpublished example id by code, never a silent no-op: {other:?}", definition.id),
            }
            rows.len()
        }

        /// ⚖️ LAW (catalog): a plugin's example catalog lists exactly `editors` apps, each once — pinned by the plugin's own law
        /// module, so an editor that drops out of the catalog (or a row that names no editor) turns it red.
        pub fn assert_example_catalog_lists(fixture: &str, editors: usize) {
            let fixture: serde_json::Value = serde_json::from_str(fixture).expect("the example catalog is JSON");
            let apps = fixture["apps"].as_array().expect("the example catalog lists its apps");
            let distinct = apps.iter().map(|row| row["app"].as_str().expect("an app row names its app")).collect::<std::collections::BTreeSet<_>>();
            assert_eq!(distinct.len(), apps.len(), "the example catalog lists every app once");
            assert_eq!(apps.len(), editors, "the example catalog lists exactly the plugin's editors");
        }

        /// ⚖️ LAW (editor): both example laws for editor `E` — [`assert_examples_load`] over the examples its picker offers
        /// ([`super::editor_examples`]) and [`assert_set_active_example_loads`] on its registered `definition`, which must
        /// dispatch exactly as many examples as the resolver checked. Answers that number.
        pub async fn assert_editor_examples_load<E>(fixture: &str, definition: &semio_framework::AppDefinition) -> usize
        where
            E: ArtifactEditor,
            E::Snapshot: Default,
        {
            let examples = super::editor_examples::<E>();
            let published = assert_examples_load::<E::Snapshot>(fixture, &definition.id, &examples, &dispose_editor_snapshot::<E>);
            assert_eq!(assert_set_active_example_loads::<super::EditorApp<E>, E::Members>(fixture, definition, &examples, &dispose_editor_snapshot::<E>).await, published, "{} loads every example it publishes through setActiveExample", definition.id);
            published
        }

        /// 🧹️ Disposes one example document through `E`'s exact snapshot disposer when it declares one (a raster forest's owned
        /// maps must never reach `Drop` populated); any other document simply drops.
        fn dispose_editor_snapshot<E: ArtifactEditor>(document: E::Snapshot) {
            let Some(mut disposer) = E::build_snapshot_disposer() else { return };
            let mut slot = Some(std::sync::Arc::new(document));
            while !disposer.terminal_is_empty(&slot) {
                disposer.close_step(&mut slot, 64, 1 << 20).unwrap_or_else(|fault| panic!("an example document disposes: {} {}", fault.code.0, fault.message));
            }
        }

'''

SDK_DUMMY_LAW = '''
/// ⚖️ LAW: the ONE example resolver decodes a published example through the snapshot's own codec — DSL or a deferred
/// producer's JSON — and refuses by code, naming the example: `app.example.unknown` for an id nothing publishes,
/// `app.example.unreadable` for a body the codec cannot read; the example-catalog law holds a published example to its
/// catalog row, both codecs and the unpublished-id refusal.
#[test]
fn the_example_resolver_loads_or_refuses_by_code() {
    let label = || LocalizedLabel::native("Four", "Vier");
    let examples = [ExampleSource::new("four", label(), <DummySnapshot as store::ArtifactDsl>::print_dsl(&DummySnapshot { count: 4 }), "file"), ExampleSource::new("broken", label(), "{\\"count\\":", "file")];
    assert_eq!(crate::app::example_snapshot::<DummySnapshot>(&examples, "four").expect("a published example loads"), DummySnapshot { count: 4 });
    for (id, code) in [("missing", "app.example.unknown"), ("broken", "app.example.unreadable")] {
        let fault = crate::app::example_snapshot::<DummySnapshot>(&examples, id).expect_err("refused by code");
        assert_eq!(fault.code.0, code);
        assert_eq!(fault.parameters.as_slice().iter().map(|parameter| (parameter.name.as_str(), parameter.value.as_str())).collect::<Vec<_>>(), [("example", id)]);
    }
    let fixture = r#"{"schema": "semio.example-catalog.v1", "plugin": "test", "apps": [{"app": "s.test.dummy@1/*#editor", "examples": [{"id": "four", "document": "distinct"}]}]}"#;
    assert_eq!(crate::app::artifact_app_laws::assert_examples_load::<DummySnapshot>(fixture, DummyApp::APP_ID, &examples[..1], &drop::<DummySnapshot>), 1);
}
'''

SCHEMA_JSON = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "semio.example-catalog.v1",
    "title": "Example Catalog",
    "description": "📚️ The examples every app of one plugin publishes — the navbar picker's list and what `setActiveExample` loads. `document` says whether the example IS the snapshot type's empty `Default` document (`default`) or a distinct one (`distinct`); the resolver law holds each app to its row.",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema", "plugin", "apps"],
    "properties": {
        "schema": {"const": "semio.example-catalog.v1"},
        "plugin": {"type": "string", "pattern": "^[a-z][a-z0-9-]*$"},
        "apps": {
            "type": "array",
            "minItems": 1,
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["app", "examples"],
                "properties": {
                    "app": {"type": "string", "pattern": "^s\\.[a-z0-9.-]+@[^/#]+/[^#]+#editor$"},
                    "examples": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "additionalProperties": False,
                            "required": ["id", "document"],
                            "properties": {"id": {"type": "string", "minLength": 1}, "document": {"enum": ["distinct", "default"]}},
                        },
                    },
                },
            },
        },
    },
}



SCHEMA_ORACLE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📚️example-catalog/🟦️.ts"
SCHEMA_ORACLE_TEXT = """/** 📚️ Every plugin's example catalog (`✏️s/🔌️plugins/<plugin>/🧫️fixtures/📚️example-catalog/🔣️.json`) against `semio.example-catalog.v1`
 * (`🧬️schema/📚️example-catalog/🔣️.json`); oracle: ajv — the third-party reading of the fixture the Rust law
 * `artifact_app_laws::assert_editor_examples_load` holds every editor to, plus the invariants `assert_example_catalog_lists` holds
 * (each app once, each example id once per app). */
import { describe, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import schema from "../../🧬️schema/📚️example-catalog/🔣️.json";

type Catalog = Readonly<{ schema: string; plugin: string; apps: Readonly<{ app: string; examples: Readonly<{ id: string; document: string }>[] }>[] }>;

const plugins = join(dirname(fileURLToPath(import.meta.url)), "../../../../../../../✏️s/🔌️plugins");
const catalogs = readdirSync(plugins)
  .map((plugin) => join(plugins, plugin, "🧫️fixtures/📚️example-catalog/🔣️.json"))
  .filter((path) => existsSync(path))
  .map((path) => JSON.parse(readFileSync(path, "utf8")) as Catalog);
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

describe("example catalogs (ajv)", () => {
  test("every plugin that ships editors ships its catalog", () => {
    expect(catalogs.length).toBeGreaterThanOrEqual(34);
  });
  for (const catalog of catalogs) {
    test(`${catalog.plugin} conforms to semio.example-catalog.v1`, () => {
      expect(validate(catalog) ? [] : validate.errors).toEqual([]);
    });
    test(`${catalog.plugin} lists each app once and each example once per app`, () => {
      const apps = catalog.apps.map((row) => row.app);
      expect(new Set(apps).size).toBe(apps.length);
      for (const row of catalog.apps) {
        const ids = row.examples.map((example) => example.id);
        expect(new Set(ids).size).toBe(ids.length);
      }
    });
  }
});
"""

UNREADABLE_EN = "The example {example} could not be read; choose another example or report the problem."
UNREADABLE_DE = "Das Beispiel {example} konnte nicht gelesen werden; wählen Sie ein anderes Beispiel oder melden Sie das Problem."


def catalog_unreadable(text):
    """🗂️ The catalog text of `app.example.unreadable` names the example the resolver refuses ({example})."""
    line = re.search(r'(?m)^.*"code": "app\.example\.unreadable".*$', text)
    if line is None:
        problems.append("catalog: app.example.unreadable entry not found")
        return text
    entry = line.group(0)
    if UNREADABLE_EN in entry and UNREADABLE_DE in entry:
        notes.append("applied catalog: app.example.unreadable names its example")
        return text
    entry = re.sub(r'"en": "(?:[^"\\]|\\.)*"', '"en": ' + json.dumps(UNREADABLE_EN, ensure_ascii=False), entry)
    entry = re.sub(r'"de": "(?:[^"\\]|\\.)*"', '"de": ' + json.dumps(UNREADABLE_DE, ensure_ascii=False), entry)
    return text[: line.start()] + entry + text[line.end():]


def sdk(plan):
    def resolver(text):
        return replace_item(
            text,
            "    pub(crate) fn catalogue_snapshot_from_document<S>(body: &str) -> Result<S, Fault>\n",
            "    /// 📚️ Turns a registered example body into the app snapshot.",
            SDK_RESOLVER,
            "sdk: resolver",
            "fn catalogue_snapshot_from_document<",
        )

    plan.edit(SDK, resolver)
    plan.edit(SDK, lambda text: once(
        text,
        "        async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> {\n            let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;\n",
        "        async fn dispatch_catalogue_example(&mut self, action: &str, example_id: &str, body: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> {\n            let snapshot = example_body_snapshot::<A::Snapshot>(example_id, body)?;\n",
        "sdk: catalogue dispatch decodes through the resolver's decoder",
    ))
    plan.edit(SDK, lambda text: once(
        text,
        "                    return self.dispatch_catalogue_example(action, &body, meta).await;\n",
        "                    return self.dispatch_catalogue_example(action, example_id, &body, meta).await;\n",
        "sdk: catalogue dispatch names its example",
    ))
    plan.edit(SDK, lambda text: once(
        text,
        "            let mut examples = E::examples();\n            if examples.is_empty() {\n                examples = declarations::subset_examples(&E::DIALECT);\n            }\n            if examples.is_empty() {\n                return None;\n            }\n",
        "            let examples = editor_examples::<E>();\n            if examples.is_empty() {\n                return None;\n            }\n",
        "sdk: catalogue reads the picker's list",
    ))
    plan.edit(SDK, lambda text: once(
        text,
        "pub use app::{activation_target, locale_from_str, ",
        "pub use app::{activation_target, editor_example_snapshot, editor_examples, example_snapshot, locale_from_str, subset_example_snapshot, unknown_example, ",
        "sdk: re-export the resolver",
    ))
    plan.edit(SDK, lambda text: once(
        text,
        "        /// 🤖️ The verbs whose agent lane diverges from their shell lane, in probe order",
        SDK_LAWS + "        /// 🤖️ The verbs whose agent lane diverges from their shell lane, in probe order",
        "sdk: example-catalog laws",
    ))
    plan.edit(SDK_DUMMY, lambda text: once(
        text,
        "\n#[semio_framework_async_macros::async_test]\nasync fn meta_carries_actor_and_local_instance_id() {",
        SDK_DUMMY_LAW + "\n#[semio_framework_async_macros::async_test]\nasync fn meta_carries_actor_and_local_instance_id() {",
        "sdk: resolver law",
    ))
    plan.edit(SDK_DUMMY, lambda text: once(
        text,
        "use crate::app::{\n    built_text_to_component_tree, ",
        "use crate::app::{\n    built_text_to_component_tree, ExampleSource, ",
        "sdk: resolver law imports ExampleSource",
    ))
    if os.path.isfile(os.path.join(TREE, CATALOG)):
        plan.edit(CATALOG, catalog_unreadable)
    else:
        problems.append(f"{CATALOG}: missing — this set applies after T6 row 12")
    plan.create(SCHEMA, json.dumps(SCHEMA_JSON, ensure_ascii=False, indent=2) + "\n")
    plan.create(SCHEMA_ORACLE, SCHEMA_ORACLE_TEXT)


# ─── stdio (folds LB2 p18) ────────────────────────────────────────────────────────────────────────────────────────────

STDIO = f"{PLUGINS}/🗄️stdio"
STDIO_ART = f"{STDIO}/🗿️artifacts"
STDIO_CATALOG_LAW = f"{STDIO}/🧪️tests/✏️editor-catalog/🦀️.rs"
STDIO_FIXTURE = f"{STDIO}/🧫️fixtures/📚️example-catalog/🔣️.json"
LOADER = re.compile(
    r"(?P<doc>(?:[ \t]*///[^\n]*\n)*)(?P<tag>[ \t]*// 🚫️async:[^\n]*\n)?"
    r"fn (?P<name>\w+_example_snapshot)\(example_id: &str\) -> (?P<snap>\w+) \{\s*"
    r"if example_id == (?P<mod>[\w:]+)::ID \{\s*<(?P=snap) as store::ArtifactDsl>::parse_dsl\((?P=mod)::PRIMARY_TEXT\)\.unwrap_or_default\(\)\s*"
    r"\} else \{\s*(?P=snap)::default\(\)\s*\}\s*\}\n"
)
RESOLVE = "semio_framework_plugin::editor_example_snapshot::<{editor}>"


def stdio_loader_files():
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "_example_snapshot(example_id: &str) -> ", STDIO_ART], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in found if path and "🧪️tests" not in path)


def stdio_editor(text, path):
    editors = re.findall(r"impl ArtifactEditor for (\w+) \{", text)
    if len(editors) != 1:
        problems.append(f"{path}: {len(editors)} ArtifactEditor impls")
        return None
    return editors[0]


def stdio_landed_files():
    """🧾️ Editors whose `setActiveExample` already resolves through the SDK resolver (a written tree)."""
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "load_example_effect(&semio_framework_plugin::editor_example_snapshot::<", STDIO_ART], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    landed = {}
    for path in sorted(path for path in found if path and "🧪️tests" not in path):
        editor = re.search(r"editor_example_snapshot::<(\w+)>\(example_id\)\?", read(path))
        landed[path] = (None, editor[1])
        notes.append(f"applied stdio loader {path}")
    return landed


def stdio(plan):
    names = STDIO_NAMES
    loaders = stdio_loader_files()
    STDIO_NAMES_LANDED.update(stdio_landed_files())
    landed = list(STDIO_NAMES_LANDED)
    if len(loaders) + len(landed) != 62:
        problems.append(f"{len(loaders)} stdio example loaders left + {len(landed)} landed, expected 62")
    for path in loaders:
        text = read(path)
        editor = stdio_editor(text, path)
        found = list(LOADER.finditer(text))
        if len(found) != 1 or editor is None:
            problems.append(f"{path}: {len(found)} loader fns")
            continue
        names[path] = (found[0]["name"], editor)

        def loader(text, path=path, name=found[0]["name"], editor=editor):
            match = LOADER.search(text)
            if match is None:
                problems.append(f"{path}: loader vanished")
                return text
            head, tail = text[: match.start()], text[match.end():]
            if head.endswith("\n\n") and tail.startswith("\n"):
                tail = tail[1:]
            text = head + tail
            call = f"load_example_effect(&{name}(example_id), "
            if text.count(call) < 1:
                problems.append(f"{path}: no setActiveExample call of {name}")
                return text
            return text.replace(call, f"load_example_effect(&{RESOLVE.format(editor=editor)}(example_id)?, ")

        plan.edit(path, loader)
    tests = 0
    for path, (name, editor) in sorted({**names, **STDIO_NAMES_LANDED}.items()):
        test = path.replace("/🦀️.rs", "/🧪️tests/🔬️unit/🦀️.rs")
        if not os.path.isfile(os.path.join(TREE, test)):
            continue
        body = read(test)
        if name is None or name + "(" not in body:
            if RESOLVE.format(editor=editor) in body:
                tests += 1
            continue
        tests += 1

        def unit(text, test=test, name=name, editor=editor):
            resolve = RESOLVE.format(editor=editor)
            text = re.sub(
                rf'    assert_eq!\({name}\(""\), \w+::default\(\)\);\n',
                f'    assert_eq!({resolve}("").expect("the empty id opens the genesis document"), <{editor} as semio_framework_plugin::ArtifactEditor>::initial_snapshot());\n',
                text,
            )
            text = re.sub(
                rf'    assert_eq!\({name}\("no-such-example"\), \w+::default\(\)\);\n',
                f'    assert_eq!({resolve}("no-such-example").expect_err("an unpublished example id is refused").code.0, "app.example.unknown");\n',
                text,
            )
            text = re.sub(rf"(?<![\w:]){name}\((crate::[\w:]+::ID)\)", lambda call: f'{resolve}({call[1]}).expect("the published example opens")', text)
            text = text.replace("/// unknown or empty id falls back to the genesis document.", "/// empty id opens the genesis document and an unpublished id is refused `app.example.unknown`.")
            raw = [call for call in re.finditer(rf"(?<![\w:]){name}\(", text)]
            if raw:
                problems.append(f"{test}: {len(raw)} {name} calls left")
            return text

        plan.edit(test, unit)
    if tests != 10:
        problems.append(f"{tests} stdio editor unit-test files load an example, expected 10")
    plan.edit(STDIO_CATALOG_LAW, lambda text: once(
        text,
        "fn fixture() -> serde_json::Value {\n",
        'const EXAMPLE_CATALOG: &str = include_str!("../../🧫️fixtures/📚️example-catalog/🔣️.json");\n\nfn fixture() -> serde_json::Value {\n',
        "stdio: editor-catalog law reads the example catalog",
    ))
    plan.edit(STDIO_CATALOG_LAW, lambda text: once(
        text,
        "async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition) {\n    stdio_packages_assembled();\n",
        "async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition)\nwhere\n    E::Snapshot: Default,\n{\n    stdio_packages_assembled();\n"
        "    artifact_app_laws::assert_editor_examples_load::<E>(EXAMPLE_CATALOG, &definition).await;\n",
        "stdio: every shipped editor loads every example it publishes",
    ))
    plan.edit(STDIO_CATALOG_LAW, lambda text: once(
        text,
        "    assert_eq!(fixture[\"editorCount\"].as_u64().unwrap() as usize, EDITOR_COUNT);\n}\n",
        "    assert_eq!(fixture[\"editorCount\"].as_u64().unwrap() as usize, EDITOR_COUNT);\n}\n\n"
        "/// 📚️ The example catalog lists every shipped editor exactly once.\n#[test]\nfn the_example_catalog_lists_every_editor_once() {\n"
        "    artifact_app_laws::assert_example_catalog_lists(EXAMPLE_CATALOG, EDITOR_COUNT);\n}\n",
        "stdio: the example catalog lists every editor once",
    ))
    fixture = os.path.join(HERE, "fixtures", "stdio.json")
    if os.path.isfile(fixture):
        plan.create(STDIO_FIXTURE, open(fixture, encoding="utf-8").read())
    else:
        problems.append(f"{fixture}: the stdio example catalog is not written yet")


# ─── laws (every non-stdio editor) ────────────────────────────────────────────────────────────────────────────────────

LAW_FILES = {"demonstrator": "🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️surface/🦀️.rs"}
PLUGIN_ASSEMBLY = {"demonstrator": "super::plugin()"}
SURFACE_MOUNT = (
    "\n//#region 🧪️SurfaceTests\n"
    "/// 🧪️ Holds every editor of this plugin to the laws of its published surface (the example catalog).\n"
    "#[cfg(test)]\n"
    "#[path = \"🧪️tests/🔬️surface/🦀️.rs\"]\n"
    "mod surface_tests;\n"
    "//#endregion 🧪️SurfaceTests\n"
)


def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def laws(plan):
    """📚️ Per plugin: its example catalog (`🧫️fixtures/📚️example-catalog/🔣️.json`, from `wp-ex1/laws/<plugin>.json`) and one
    law per editor in the plugin's surface tests calling `assert_editor_examples_load` on the ASSEMBLED plugin (subset
    examples are registered by the plugin's assembly). A plugin without surface tests gets them: the file, its mount in the
    plugin root and the `artifact-app-testing` dev feature."""
    for spec_path in sorted(os.listdir(os.path.join(HERE, "laws"))):
        spec = json.load(open(os.path.join(HERE, "laws", spec_path), encoding="utf-8"))
        plugin, slug = spec["dir"], spec["plugin"]
        fixture = f"{PLUGINS}/{plugin}/🧫️fixtures/📚️example-catalog/🔣️.json"
        catalog = {"schema": "semio.example-catalog.v1", "plugin": slug, "apps": spec["apps"]}
        plan.create(fixture, json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
        law_file = f"{PLUGINS}/{LAW_FILES.get(slug, plugin + '/🧪️tests/🔬️surface/🦀️.rs')}"
        include = os.path.relpath(os.path.join(TREE, fixture), os.path.dirname(os.path.join(TREE, law_file)))
        crate_root = f"{PLUGINS}/{plugin}/📦️packages/🦀️rust/🦀️.rs"
        nested = os.path.isfile(os.path.join(TREE, crate_root)) and re.search(r"^\s*(?:pub )?mod plugin;", read(crate_root), re.M)
        assembly = PLUGIN_ASSEMBLY.get(slug, "crate::plugin::plugin()" if nested else "crate::plugin()")
        tests = []
        for test in spec["tests"]:
            editor = test["editor"]
            create = f"{test['create']}().await" if test.get("async") else f"{test['create']}()"
            name = snake(editor.split("::")[-1])
            tests.append(
                f"/// 📚️ `{test['app']}` loads every example it publishes — through the ONE resolver and through `setActiveExample` on a\n"
                f"/// fresh registered fixture — and refuses an unpublished id `app.example.unknown` (catalog `🧫️fixtures/📚️example-catalog`).\n"
                f"#[semio_framework_async_macros::async_test]\n"
                f"async fn {name}_loads_every_example_it_publishes() {{\n"
                f"    example_catalog_assembled();\n"
                f"    semio_framework_plugin::artifact_app_laws::assert_editor_examples_load::<{editor}>(EXAMPLE_CATALOG, &{create}).await;\n"
                f"}}\n"
            )
        block = (
            "\n//#region 📚️ExampleCatalog\n"
            f'const EXAMPLE_CATALOG: &str = include_str!("{include}");\n\n'
            "/// 🧩️ Assembles the plugin once: assembly registers every subset's published examples, exactly as a host that loaded\n"
            "/// the plugin has them.\n"
            "fn example_catalog_assembled() {\n"
            "    static ASSEMBLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();\n"
            "    ASSEMBLED.get_or_init(|| {\n"
            f"        {assembly}.expect(\"the plugin assembles\");\n"
            "    });\n"
            "}\n\n"
            + "\n".join(tests)
            + "\n/// 📚️ The example catalog lists exactly this plugin's editors, each once.\n#[test]\nfn the_example_catalog_lists_every_editor_once() {\n"
            f"    semio_framework_plugin::artifact_app_laws::assert_example_catalog_lists(EXAMPLE_CATALOG, {len(tests)});\n}}\n"
            "//#endregion 📚️ExampleCatalog\n"
        )
        if os.path.isfile(os.path.join(TREE, law_file)) or law_file in plan.creates:

            def append(text, block=block, law_file=law_file):
                if "//#region 📚️ExampleCatalog" in text:
                    notes.append(f"applied law {law_file}")
                    return text
                return text.rstrip("\n") + "\n" + block

            plan.edit(law_file, append)
        else:
            plan.create(law_file, f"//! 🧪️ The published surface of every {slug} editor.\n" + block)
            plan.edit(f"{PLUGINS}/{plugin}/🦀️.rs", lambda text, law_file=law_file: text if "mod surface_tests;" in text else text.rstrip("\n") + "\n" + SURFACE_MOUNT)
            cargo = f"{PLUGINS}/{plugin}/📦️packages/🦀️rust/Cargo.toml"

            def dev_feature(text, cargo=cargo):
                if re.search(r"(?ms)^\[dev-dependencies\].*?semio-framework-plugin = \{[^}]*artifact-app-testing", text):
                    notes.append(f"applied {cargo}: artifact-app-testing")
                    return text
                line = re.search(r"(?m)^semio-framework-plugin = (\{[^\n]*features = \[\"component-guest\"\][^\n]*\})$", text)
                if line is None or "[dev-dependencies]\n" not in text:
                    problems.append(f"{cargo}: no semio-framework-plugin dependency line / dev-dependencies table")
                    return text
                dev = "semio-framework-plugin = " + line.group(1).replace('features = ["component-guest"]', 'features = ["component-guest", "artifact-app-testing"]')
                return text.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + dev + "\n", 1)

            plan.edit(cargo, dev_feature)


# ─── plugins, batch A (plain `setActiveExample` handlers) ─────────────────────────────────────────────────────────────

RESOLVE_DOC_TAIL = "any other id or an unreadable asset is refused by code."


def plugin_path(rel):
    return f"{PLUGINS}/{rel}"


def collapse(text):
    return re.sub(r"\n{3,}", "\n\n", text)


def handler(plan, rel, header, new, label, uses=(), after=None):
    """🔁️ One handler rewrite: `header`'s fn (with its doc block) → `new`; `uses` dropped from the imports."""
    path = plugin_path(rel)

    def edit(text):
        text = replace_fn(text, header, new, label)
        for name in uses:
            text = drop_use(text, name, f"{label}: use {name}")
        return collapse(after(text) if after else text)

    plan.edit(path, edit)


def faults(plan, rel, codes, label):
    """🧹️ The per-plugin example codes the ONE resolver replaced: their `.fault(…)` declarations go."""
    path = plugin_path(rel)
    plan.edit(path, lambda text: [text := drop_fault(text, code, f"{label}: .fault {code}") for code in codes][-1])


SUBSET_EXAMPLES = {'WriterPlayApp': 'crate::standards::v1::subsets::any::examples()', 'EquationPlayApp': 'crate::standards::v1::subsets::any::examples()', 'BitmapEditor': 'crate::examples::example_source_slice()', 'Grid2dEditor': 'crate::standards::v1::subsets::any::examples()', 'Grid3dEditor': 'crate::standards::v1::subsets::any::examples()', 'Wfc2dEditor': 'crate::standards::v1::subsets::any::examples()', 'Wfc3dEditor': 'crate::standards::v1::subsets::any::examples()', 'VcsPlayApp': 'crate::standards::v1::subsets::any::examples()', 'AnimatePresentationPlayApp': 'crate::standards::v1::subsets::any::examples()', 'SequencePlayApp': 'crate::standards::v1::subsets::any::examples()', 'Fem2dPlayApp': 'crate::standards::v1::subsets::any::examples()', 'Fem3dPlayApp': 'crate::standards::v1::subsets::any::examples()', 'ReasoningWiresPlayApp': 'crate::standards::v1::subsets::any::examples()', 'FormsPlayApp': 'crate::standards::v1::subsets::any::examples()', 'PlaybookPlayApp': 'crate::standards::v1::subsets::any::examples()', 'DagPlayApp': 'crate::standards::v1::subsets::any::examples()', 'DrawingPlayApp': 'crate::standards::v1::subsets::any::examples()', 'NotePlayApp': 'crate::standards::v1::subsets::any::examples()', 'Block2dPlayApp': 'crate::standards::v1::subsets::any::examples()', 'Block3dPlayApp': 'crate::standards::v1::subsets::any::examples()', 'Block5dPlayApp': 'crate::standards::v1::subsets::any::examples()', 'SourcingCurationApp': 'crate::standards::v1::subsets::any::examples()', 'TrinityJackPlayApp': 'crate::standards::v1::subsets::any::examples()', 'TrinityRewritingPlayApp': 'crate::standards::v1::subsets::any::examples()'}
"""📚️ Editors whose examples only their subset declaration publishes (no `ArtifactEditor::examples()`) → the subset's own list:
their handlers resolve through `subset_example_snapshot` over it, so a unit test resolves them without assembling the plugin."""


def resolve(editor, expr="&payload.example_id"):
    """🧭️ The ONE resolver call for `editor`'s examples: its subset's own list where only the subset publishes them."""
    listing = SUBSET_EXAMPLES.get(editor.split("::")[-1])
    if listing:
        return f"semio_framework_plugin::subset_example_snapshot::<{editor}>({listing}, {expr})"
    return f"semio_framework_plugin::editor_example_snapshot::<{editor}>({expr})"


def load(editor, expr="&payload.example_id"):
    return resolve(editor, expr) + "?"


A = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def plugins_a(plan):
    # ➗️ mathematical
    math = f"➗️mathematical/🗿️artifacts/➗️equation/{A}/✏️editor"
    handler(plan, f"{math}/🎮️commands/🎬️set-active-example/🦀️.rs",
            "pub fn emit(example_id: &str) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {",
            "/// 🧬️ The whole-document replacement this verb publishes. It has no `EquationMutation` representative — the taxonomy\n"
            "/// forbids a whole-document replace variant — so it lands as an `Effect::LoadDocument` outside undo history, the\n"
            "/// `🕸️dag`/`🏛️architect` shape. The document is what the ONE example resolver decodes from the published example's own\n"
            f"/// asset (the empty id — the navbar's No example row — is the editor's empty initial document); {RESOLVE_DOC_TAIL}\n"
            "///\n"
            "/// 🪨️ Free of any `ArtifactView`, because the retained work's `setActiveExample` branch runs BEFORE a scene owner exists —\n"
            "/// resolving one is precisely what this verb is there to make possible.\n"
            "pub fn emit(example_id: &str) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {\n"
            f"    let document = {load('crate::editor::equation::EquationPlayApp', 'example_id')};\n"
            "    Ok(Emit { effects: vec![reset_equation_document_effect(&document)], ..Default::default() })\n"
            "}\n",
            "mathematical: emit", uses=("app_fault",))
    faults(plan, f"{math}/🦀️.rs", ["equation.example-invalid"], "mathematical")
    # 🎬️ sequence
    seq = f"🎬️sequence/🗿️artifacts/🎬️sequence/{A}/✏️editor"
    handler(plan, f"{seq}/🎮️commands/📚️example/🦀️.rs",
            "    pub fn emit(example_id: &str) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {",
            "    /// 🧬️ The whole-document replacement this verb publishes. A composed-child document has no whole-snapshot mutation\n"
            "    /// representative, so it lands as an `Effect::LoadDocument` outside undo history — the `🧊️process3d`/`🕸️dag` shape. The\n"
            "    /// document is what the ONE example resolver decodes from the published example's own asset (the empty id — the\n"
            f"    /// navbar's No example row — is the editor's initial document); {RESOLVE_DOC_TAIL}\n"
            "    ///\n"
            "    /// 🪨️ Free of any `ArtifactView`: the retained work answers this verb before a working scene exists, which is precisely\n"
            "    /// what loading an example is there to make possible.\n"
            "    pub fn emit(example_id: &str) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {\n"
            f"        let document = {load('crate::editor::sequence::SequencePlayApp', 'example_id')};\n"
            "        Ok(Emit { effects: vec![reset_sequence_document_effect(&document)], ..Default::default() })\n"
            "    }\n",
            "sequence: emit", uses=("app_fault",))
    faults(plan, f"{seq}/🦀️.rs", ["sequence.example-invalid"], "sequence")
    # 🌊️ flow
    flow = f"🌊️flow/🗿️artifacts/🌊️flow/{A}/✏️editor"
    flow_handler = f"{flow}/🎮️commands/🎨️set-active-example/🦀️.rs"
    handler(plan, flow_handler,
            "pub fn set_active_example_edit(payload: &SetActiveExample, composed: &FlowSnapshot) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {",
            "/// 🎨️ Replaces the composed scene with the example the ONE resolver loads — a published example decoded from its own\n"
            "/// asset, or the editor's empty initial document for the empty id — and publishes it on the content child, the coordinate\n"
            "/// every reader resolves, so the example is exactly what the window, the next verb and a reload see. Nothing is published\n"
            f"/// when the scene already is that example; {RESOLVE_DOC_TAIL}\n"
            "pub fn set_active_example_edit(payload: &SetActiveExample, composed: &FlowSnapshot) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {\n"
            f"    let target = {load('crate::editor::flow::FlowPlayApp')};\n"
            "    let scene = crate::flow_working_scene(&target);\n"
            "    Ok(crate::editor::flow::flow_scene_publication(composed, &scene.widgets, &scene.synapses, &scene.layout))\n"
            "}\n",
            "flow: set_active_example_edit", uses=("app_fault", "demo"))
    handler(plan, flow_handler,
            "pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {",
            "/// 🎨️ Loads the example `payload` names through the ONE example resolver ([`set_active_example_edit`]).\n"
            "pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {\n"
            "    set_active_example_edit(payload, &crate::flow_composed_snapshot(doc.snapshot, &doc.children)?)\n"
            "}\n",
            "flow: handle")
    faults(plan, f"{flow}/🦀️.rs", ["flow.example-invalid", "flow.example-unknown"], "flow")
    # 🌿️ vcs
    vcs = f"🌿️vcs/🗿️artifacts/🌿️vcs/{A}"
    handler(plan, f"{vcs}/✏️editor/🎮️commands/📚️example/🦀️.rs",
            "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {",
            "    /// 🧬️ Whole-document replace has no `VcsDemoMutation` representative, so picking an example builds an\n"
            "    /// `Effect::LoadDocument` and therefore lands outside undo history. The document is what the ONE example resolver\n"
            "    /// decodes from the published example's own asset (the empty id — the navbar's No example row — is the editor's empty\n"
            f"    /// initial document); {RESOLVE_DOC_TAIL}\n"
            "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {\n"
            f"        let document = {load('crate::editor::vcs::VcsPlayApp')};\n"
            "        Ok(Emit { effects: vec![vcs_document_effect(&document)], ..Default::default() })\n"
            "    }\n",
            "vcs: handle",
            after=lambda text: text.replace("use crate::editor::vcs::vcs_example_document_effect;", "use crate::editor::vcs::vcs_document_effect;"))
    handler(plan, f"{vcs}/✏️editor/🦀️.rs",
            "pub fn vcs_example_document_effect() -> semio_framework_plugin::Effect {",
            "/// 🧬️ Whole-document replace is banned from the `Mutation` enum, so the example switch builds an\n"
            "/// `Effect::LoadDocument` — the same lane `🗒️note`/`✒️writer`/`🏛️architect` use. The spr comes from\n"
            "/// `store::empty_document_spr`, never from a minted `ArtifactEnvelope`: an envelope is a terminal\n"
            "/// store shell whose `Drop` asserts its bounded retirement authority detached every nested owner\n"
            "/// first, so building the effect that way panics (ticket 26/09/18, B1a fix #7).\n"
            "pub fn vcs_document_effect(document: &VcsSnapshot) -> semio_framework_plugin::Effect {\n"
            "    let pack = <VcsSnapshot as store::ArtifactPack>::encode_pack(document);\n"
            "    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr(VCS_APP_ID, VCS_DOCUMENT_SCHEMA));\n"
            "    semio_framework_plugin::Effect::LoadDocument { pack, spr }\n"
            "}\n",
            "vcs: document effect")
    plan.edit(plugin_path(f"{vcs}/📚️examples/🎬️demo/🦀️.rs"), lambda text: once(
        text,
        "\n/// 🧬️ The document this example IS, decoded from its own asset so the DSL file stays the single\n/// source of truth. `setActiveExample` packs this into the `Effect::LoadDocument` it emits.\npub fn snapshot() -> crate::VcsSnapshot {\n    <crate::VcsSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).unwrap_or_default()\n}\n",
        "",
        "vcs: demo leaf decodes through the resolver only",
    ) if "pub fn snapshot() -> crate::VcsSnapshot" in text or "vcs_document_effect" not in text else text)
    # 🎪️ demonstrator
    dem = f"🎪️demonstrator/🗿️artifacts/🎪️playground/{A}/✏️editor"
    handler(plan, f"{dem}/🎮️commands/🎨️set-active-example/🦀️.rs",
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<PlaygroundMutation, NoConfigMutation>, Fault> {",
            "/// 🎨️ Replaces the playground schema with the one of the example the ONE resolver loads — a published example decoded\n"
            f"/// from its own asset, or the editor's empty initial document for the empty id; {RESOLVE_DOC_TAIL}\n"
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<PlaygroundMutation, NoConfigMutation>, Fault> {\n"
            f"    let schema = {load('crate::editor::playground::PlaygroundEditor')}.schema;\n"
            "    Ok(Emit::mutations(vec![PlaygroundMutation::ChangeSchema(ChangeSchemaMutation { new_schema: schema })]))\n"
            "}\n",
            "demonstrator: handle", uses=("demo", "empty_playground_snapshot"))
    faults(plan, f"{dem}/🦀️.rs", ["playground.example.unparsable"], "demonstrator")
    # 🏛️ architect
    arch = f"🏛️architect/🗿️artifacts/🏛️program/{A}/✏️editor"
    handler(plan, f"{arch}/🎮️commands/📚️example/🦀️.rs",
            "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {",
            "    /// 🧬️ Whole-document replace has no `ProgramMutation` representative (banned outright by the taxonomy's forbidden\n"
            "    /// vocabulary), so picking an example builds a `Effect::LoadDocument` through `reset_document_effect` — the same lane\n"
            "    /// `importProgram`/`importRegistersCsv` use — and therefore lands outside undo history. The document is what the ONE\n"
            "    /// example resolver decodes from the published example's own asset (the empty id is the editor's initial document);\n"
            f"    /// {RESOLVE_DOC_TAIL}\n"
            "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {\n"
            f"        let document = {load('crate::editor::architect::ArchitectPlayApp')};\n"
            "        Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })\n"
            "    }\n",
            "architect: handle", uses=("sample_plugin",))
    # 💡️ reasoning
    wires = f"💡️reasoning/🗿️artifacts/🔌️wires/{A}/✏️editor"
    handler(plan, f"{wires}/🎮️commands/🧬️set-active-example/🦀️.rs",
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, crate::WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {",
            "/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see\n"
            "/// `📓️taxonomy.md`'s forbidden vocabulary), so loading a named example builds `editor::wires::reset_wires_document_effect`\n"
            "/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry. The document is what the\n"
            "/// ONE example resolver decodes from the published example's own asset (the empty id is the editor's empty initial\n"
            f"/// document); {RESOLVE_DOC_TAIL}\n"
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, crate::WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {\n"
            f"    let next = {load('crate::editor::wires::ReasoningWiresPlayApp')};\n"
            "    Ok(Emit { effects: vec![crate::editor::wires::reset_wires_document_effect(&next)], ..Default::default() })\n"
            "}\n",
            "reasoning: handle", uses=("empty_wires_snapshot", "metabolism_wires_example_snapshot", "app_fault"))
    faults(plan, f"{wires}/🦀️.rs", ["wires.example-invalid"], "reasoning")
    # 📖️ playbook / 📜️ imperative / 🕸️ dag / 🖍️ draw / 🗒️ note
    simple = [
        ("📖️playbook", f"📖️playbook/🗿️artifacts/📖️playbook/{A}/✏️editor", "🧬️set-active-example",
         "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {",
         load("crate::editor::playbook::PlaybookPlayApp"), "crate::editor::playbook::reset_playbook_document_effect(&next)", ["playbook-example-unparsable"], (), "the editor's empty initial document"),
        ("📜️imperative", f"📜️imperative/🗿️artifacts/📜️procedure/{A}/✏️editor", "🧬️set-active-example",
         "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {",
         f"if payload.example_id.is_empty() {{ ProcedureSnapshot::default() }} else {{ {load('crate::editor::procedure::ImperativePlayApp')} }}", "crate::editor::procedure::reset_procedure_document_effect(&next)", ["imperative-example-unparsable"], (), "the empty procedure"),
        ("🕸️dag", f"🕸️dag/🗿️artifacts/🕸️dag/{A}/✏️editor", "🧬️set-active-example",
         "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {",
         f"if payload.example_id.is_empty() {{ crate::empty_snapshot() }} else {{ {load('crate::editor::dag::DagPlayApp')} }}", "crate::editor::dag::reset_dag_document_effect(&next)", [], (), "the empty graph"),
        ("🗒️note", f"🗒️note/🗿️artifacts/🗒️note/{A}/✏️editor", "🗃️set-active-example",
         "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {",
         load("crate::editor::note::NotePlayApp"), "crate::editor::note::reset_document_effect(&next)", ["note.example.unknown"], ("semio_example_snapshot", "app_fault"), "the editor's empty initial document"),
    ]
    for plugin, base, leaf, header, next_expr, effect, codes, uses, empty in simple:
        handler(plan, f"{base}/🎮️commands/{leaf}/🦀️.rs", header,
                "/// 🧬️ Whole-document replace has no in-history mutation, so loading a named example emits an `Effect::LoadDocument`\n"
                "/// (outside undo history). The document is what the ONE example resolver decodes from the published example's own\n"
                f"/// asset (the empty id — the navbar's No example row — is {empty}); {RESOLVE_DOC_TAIL}\n"
                f"{header}\n"
                f"    let next = {next_expr};\n"
                f"    Ok(Emit {{ effects: vec![{effect}], ..Default::default() }})\n"
                "}\n",
                f"{plugin}: handle", uses=uses)
        if codes:
            faults(plan, f"{base}/🦀️.rs", codes, plugin)
    draw = f"🖍️draw/🗿️artifacts/🖍️drawing/{A}/✏️editor"
    handler(plan, f"{draw}/🎮️commands/🖼️set-active-example/🦀️.rs",
            "pub fn handle(\n    payload: &SetActiveExample,",
            "/// 🖼️ Loads the example the ONE resolver answers for `payload.example_id` — a published example decoded from its own\n"
            f"/// asset, or the editor's empty initial document for the empty id; {RESOLVE_DOC_TAIL}\n"
            "pub fn handle(\n"
            "    payload: &SetActiveExample,\n"
            "    _doc: &ArtifactView<'_, DrawingSnapshot>,\n"
            "    _cfg: &ConfigView<'_, NoConfig>,\n"
            "    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,\n"
            ") -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {\n"
            f"    let next = {load('crate::editor::drawing::DrawingPlayApp')};\n"
            "    Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&next)], ..Default::default() })\n"
            "}\n",
            "draw: handle", uses=("app_fault", "default_drawing_document", "examples", "ArtifactDsl"))
    faults(plan, f"{draw}/🦀️.rs", ["drawing.example.unknown", "drawing.example.parse"], "draw")
    # 🌍️ gis map + terrain
    for kind, snapshot, empty, derive, codes in (
        ("🗺️gismap", "GisMapSnapshot", "GisMapSnapshot::default()", "gis_map_snapshot_with_derived_children", ["gis.map.example.unknown", "gis.map.example.invalid"]),
        ("🏔️gisterrain", "GisTerrainSnapshot", "crate::schema::empty_gis_terrain_snapshot()", None, ["gis.terrain.example.unknown", "gis.terrain.example.invalid"]),
    ):
        base = f"🌍️gis/🗿️artifacts/{kind}/{A}/✏️editor"
        resolved = "semio_framework_plugin::example_snapshot(&example_catalogue(), example_id)?"
        handler(plan, f"{base}/🎮️commands/🎨️example/🦀️.rs",
                f"pub fn example_document(example_id: &str) -> Result<{snapshot}, Fault> {{",
                "/// 📚️ The document `setActiveExample` loads: the empty id — the navbar's No example row — is the empty document, every\n"
                f"/// other id the published example the ONE resolver decodes from its own asset; {RESOLVE_DOC_TAIL}\n"
                f"pub fn example_document(example_id: &str) -> Result<{snapshot}, Fault> {{\n"
                "    if example_id.is_empty() {\n"
                f"        return Ok({empty});\n"
                "    }\n"
                + (f"    Ok({derive}({resolved}))\n" if derive else f"    Ok({resolved})\n")
                + "}\n",
                f"gis {kind}: example_document", uses=("app_fault",))
        faults(plan, f"{base}/🦀️.rs", codes, f"gis {kind}")
    # 🏗️ fem 2d + 3d
    for kind, snapshot, editor, empty, effect in (
        ("◻️2d", "Fem2dSnapshot", "crate::editor::fem2d::Fem2dPlayApp", "crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot()", "crate::editor::fem2d::reset_document_effect(&document)"),
        ("🧊️3d", "Fem3dSnapshot", "crate::editor::fem3d::Fem3dPlayApp", "crate::standards::v1::subsets::any::schema::empty_fem3d_snapshot()", "crate::editor::fem3d::reset_document_effect(&document)"),
    ):
        mutation = "Fem2dMutation" if kind == "◻️2d" else "Fem3dMutation"
        header = f"pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, {snapshot}>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<{mutation}, NoConfigMutation>, Fault> {{"
        handler(plan, f"🏗️fem/🗿️artifacts/{kind}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs", header,
                "/// 📚️ Loads the example the ONE resolver answers for `payload.example_id` through the host document replacement effect\n"
                "/// (`Effect::LoadDocument`, outside undo history) — a published example decoded from its own asset, or the empty document\n"
                f"/// for the empty id (the navbar's No example row); {RESOLVE_DOC_TAIL}\n"
                f"{header}\n"
                f"    let document = if payload.example_id.is_empty() {{ {empty} }} else {{ {load(editor)} }};\n"
                f"    Ok(Emit {{ effects: vec![{effect}], ..Default::default() }})\n"
                "}\n",
                f"fem {kind}: handle", uses=("ArtifactDsl",) if kind == "◻️2d" else ())
    # 🌀️ procedural generation2d + generation3d (editor)
    g2 = f"🌀️procedural/🗿️artifacts/🌀️generation2d/{A}/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
    handler(plan, g2, "fn example_document(example_id: &str) -> Option<Generation2dSnapshot> {", "", "generation2d: example_document")
    handler(plan, g2,
            "pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {",
            "/// 🎨️ Loads the example the ONE resolver answers for `payload.example_id` (a published example decoded from its own\n"
            f"/// asset) or clears the graph for the empty id — the navbar's No example row; {RESOLVE_DOC_TAIL}\n"
            "pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {\n"
            f"    let target = if payload.example_id.is_empty() {{ empty_generation2d_snapshot() }} else {{ {load('crate::editor::generation2d::Generation2dPlayApp')} }};\n"
            "    let mut operations: Vec<Generation2dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation2d(GenerationMutation::Remove { id: generation.id.clone() })).collect();\n"
            "    operations.extend(generation2d_host_snapshot_operations(&doc.snapshot.host_snapshot, &target.host_snapshot));\n"
            "    let config = Generation2dConfig { show_mode: cfg.snapshot.show_mode.clone(), selected_generation_id: None };\n"
            "    target.retire_cold();\n"
            "    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation2dConfigMutation::Snapshot { config }], ..Default::default() })\n"
            "}\n",
            "generation2d: emit")
    g3 = f"🌀️procedural/🗿️artifacts/🧊️generation3d/{A}/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
    handler(plan, g3,
            "pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {",
            "/// 🎨️ Loads the picked example — or, for the EMPTY id, clears the graph.\n"
            "///\n"
            "/// 🕳️ The empty id is the picker's own `No example` row: `NavbarExampleSelect` normalizes its `__none__` sentinel to `\"\"`\n"
            "/// before dispatching. It used to resolve to `default_snapshot()`, which is the hexagonal mushroom column — itself one of\n"
            "/// the eight bundled examples — so the row that promises NO example silently loaded one (measured live on the served editor\n"
            "/// 2026-09-12). Every other id is the published example the ONE resolver decodes from its own asset; an unpublished id\n"
            "/// or an unreadable asset is refused by code (`app.example.unknown` / `app.example.unreadable`) instead of a silent\n"
            "/// no-op or an empty graph (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 26/09/23 EX1).\n"
            "pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {\n"
            "    let host_snapshot = &doc.snapshot.host_snapshot;\n"
            f"    let target = if payload.example_id.is_empty() {{ empty_generation3d_snapshot() }} else {{ {load('crate::editor::generation3d::Generation3dPlayApp')} }};\n"
            "    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();\n"
            "    operations.extend(generation3d_host_snapshot_operations(host_snapshot, &target.host_snapshot));\n"
            "    let config = config_after_document_load(cfg.snapshot, &target.host_snapshot.camera);\n"
            "    // 🧹️ The loaded example projection is dead once its operations and camera are read — close it\n"
            "    // through its explicit ladder, never leave the fixture's ordered layout root to drop glue.\n"
            "    target.retire_cold();\n"
            "    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::SetSnapshot(crate::editor::generation3d::config::SetSnapshot { config })], ..Default::default() })\n"
            "}\n",
            "generation3d: emit",
            after=lambda text: text.replace("use crate::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, example_snapshot, is_generation3d_example_id};", "use crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot;"))


# ─── plugins, batch B (handlers whose examples themselves need fixing) ─────────────────────────────────────────────────

def plugins_b(plan):
    # 📸️ remodel — the picker offered `demo-session` (a `.cmd.semio` replay, not a document): picking it parsed a command
    # script as a scene and silently did nothing. The registry lists document examples only; the session stays mounted
    # for its own replay tests.
    rem = f"📸️remodel/🗿️artifacts/📸️remodeling/{A}"
    registry = plugin_path(f"{rem}/✏️editor/📚️examples/🦀️.rs")
    plan.edit(registry, lambda text: once(
        text,
        "    RemodelingExample {\n"
        "        id: crate::examples::app_remodeling_demo_session::ID,\n"
        "        text: crate::examples::app_remodeling_demo_session::PRIMARY_TEXT,\n"
        "        icon: crate::examples::app_remodeling_demo_session::ICON,\n"
        "        label_en: \"Demo Session\",\n"
        "        label_de: \"Demo-Sitzung\",\n"
        "    },\n",
        "",
        "remodel: the picker lists document examples only",
    ))
    plan.edit(registry, lambda text: once(
        text,
        "/// 📚️ One registered example: the id the `setActiveExample` action carries, the committed text, and the\n"
        "/// picker chrome. `text` is `.dsl.semio` document text for a document example and `.cmd.semio` replay\n"
        "/// text for a session example — [`REMODELING_EXAMPLE_BOOT_ID`] names the one that boots the editor.\n",
        "/// 📚️ One registered example: the id the `setActiveExample` action carries, the committed `.dsl.semio` document\n"
        "/// text, and the picker chrome — [`REMODELING_EXAMPLE_BOOT_ID`] names the one that boots the editor. A replay session\n"
        "/// (`✏️editor/📚️examples/🎬️demo-session`, `.cmd.semio`) is not a document and is not registered here.\n",
        "remodel: registry row doc",
    ))
    plan.edit(registry, lambda text: once(
        text,
        "/// 🚀️ The example the editor boots on when its committed text parses — otherwise\n"
        "/// `default_remodeling_scene()` stands in, exactly the fallback shape `🧱️block`'s\n"
        "/// `default_block2d_snapshot()` uses.\n",
        "/// 🚀️ The example the editor boots on.\n",
        "remodel: boot id doc",
    ))
    handler(plan, f"{rem}/✏️editor/📚️examples/🦀️.rs",
            "pub fn boot_snapshot() -> crate::RemodelingSnapshot {",
            "/// 🚀️ The boot document: the registered boot example as the ONE example resolver decodes it. The example-catalog law\n"
            "/// holds every registered example to decode, so a committed text that stops parsing is a red law, never an empty boot.\n"
            "pub fn boot_snapshot() -> crate::RemodelingSnapshot {\n"
            "    semio_framework_plugin::example_snapshot(example_source_slice(), REMODELING_EXAMPLE_BOOT_ID).expect(\"the remodeling boot example decodes — pinned by the example-catalog law\")\n"
            "}\n",
            "remodel: boot snapshot")
    handler(plan, f"{rem}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
            "pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {",
            "/// 🎬️ Replaces the document's declared state with the named example's, as the ONE example resolver decodes it from the\n"
            "/// example's committed text; the empty id — the navbar's No example row — restores the artifact's default scene. Any\n"
            "/// other id or a committed text that no longer parses is refused by code, never a silent no-op; picking the example the\n"
            "/// document already is writes nothing.\n"
            "pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {\n"
            "    let next = if payload.example_id.is_empty() { crate::default_remodeling_scene() } else { semio_framework_plugin::example_snapshot(example_source_slice(), &payload.example_id)? };\n"
            "    let mut mutations = example_media_operations(&payload.example_id, doc.snapshot);\n"
            "    mutations.extend(replace_document_operations(doc.snapshot, &next));\n"
            "    match mutations.is_empty() {\n"
            "        true => Ok(Emit::default()),\n"
            "        false => Ok(Emit::mutations(mutations)),\n"
            "    }\n"
            "}\n",
            "remodel: handle",
            after=lambda text: text.replace("use crate::editor::remodeling::examples::example_text;", "use crate::editor::remodeling::examples::example_source_slice;"))
    handler(plan, f"{rem}/👁️viewer/🦀️.rs",
            "    fn initial_snapshot() -> RemodelingSnapshot {",
            "    /// 🚀️ Boots on the subset's committed `demo` example — the same document the editor boots on — so the two surfaces of\n"
            "    /// one dialect open the same scene. The example-catalog law holds that example to decode, so a text that stops\n"
            "    /// parsing is a red law, never an empty viewer.\n"
            "    fn initial_snapshot() -> RemodelingSnapshot {\n"
            "        crate::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect(\"the remodeling demo example decodes — pinned by the example-catalog law\")\n"
            "    }\n",
            "remodel: viewer boot", uses=("default_remodeling_scene",))
    writer_b(plan)
    raster_b(plan)
    block_b(plan)
    writer_tests_b(plan)
    wfc_b(plan)
    norm_b(plan)


def writer_b(plan):
    """✒️ writer — its form offered `jack` (default) and `dag.jack` while it published only `demo`: `setActiveExample("jack")`
    opened an EMPTY document. `dag-jack` becomes a published example (its asset moves beside the demo's), the form offers
    exactly the published examples, and the retained job completes an unpublished id as the resolver's typed refusal."""
    base = f"✒️writer/🗿️artifacts/✒️writer/{A}"
    old_asset = plugin_path(f"{base}/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio")
    new_asset = plugin_path(f"{base}/🖼️assets/🕸️dag-jack/🗣️.dsl.semio")
    if os.path.isfile(os.path.join(TREE, old_asset)):
        plan.create(new_asset, read(old_asset))
        plan.delete(old_asset)
    plan.create(plugin_path(f"{base}/📚️examples/🕸️dag-jack/🦀️.rs"),
                "//! 📚️ Example `dag-jack`.\n\n"
                "use semio_framework_plugin::{ExampleSource, LocalizedLabel};\n\n"
                "pub const ID: &str = \"dag-jack\";\n"
                "pub fn label() -> LocalizedLabel {\n"
                "    LocalizedLabel::native(\"DAG Jack\", \"DAG-Jack\")\n"
                "}\n"
                "pub const ICON: &str = \"file\";\n"
                "pub const PRIMARY_TEXT: &str = include_str!(\"../../🖼️assets/🕸️dag-jack/🗣️.dsl.semio\");\n"
                "pub fn source() -> ExampleSource {\n"
                "    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)\n"
                "}\n")
    plan.edit(plugin_path("✒️writer/🗿️artifacts/✒️writer/🦀️.rs"), lambda text: once(
        text,
        "        #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs\"]\n        mod component;\n        pub use component::*;\n    }\n",
        "        #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs\"]\n        mod component;\n        pub use component::*;\n    }\n"
        "    #[path = \".\"]\n    pub mod dag_jack {\n        #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🕸️dag-jack/🦀️.rs\"]\n        mod component;\n        pub use component::*;\n    }\n",
        "writer: mount the dag-jack example",
    ))
    plan.edit(plugin_path(f"{base}/🦀️.rs"), lambda text: once(
        text,
        "fn examples() -> &'static [ExampleSource] {\n    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();\n    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()\n}\n",
        "/// 📚️ The examples this subset publishes — the navbar picker's list, the `setActiveExample` form's options and what the\n"
        "/// ONE example resolver loads.\n"
        "pub fn examples() -> &'static [ExampleSource] {\n    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();\n    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source(), crate::examples::dag_jack::source()]).as_slice()\n}\n",
        "writer: publish dag-jack",
    ))
    text_rs = plugin_path(f"{base}/🚪️io/📸️snapshot/📝️text/🦀️.rs")
    plan.edit(text_rs, lambda text: once(
        text,
        'pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio");',
        'pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🕸️dag-jack/🗣️.dsl.semio");',
        "writer: dag-jack asset path",
    ))
    plan.edit(text_rs, lambda text: once(text, "    parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())", "    parse_dsl(JACK_EXAMPLE_TEXT).expect(\"the jack example decodes — pinned by the example-catalog law\")", "writer: jack boot"))
    plan.edit(text_rs, lambda text: once(text, "    parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())", "    parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect(\"the dag-jack example decodes — pinned by the example-catalog law\")", "writer: dag-jack document"))
    plan.edit(text_rs, lambda text: drop_use(text, "schema", "writer: text use schema"))
    plan.edit(plugin_path(f"{base}/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"), lambda text: once(text, '("📚️examples/🎬️demo/🖼️assets/🧪️dag-example", ', '("🖼️assets/🕸️dag-jack", ', "writer: demo leaf law names the moved asset"))
    editor_rs = plugin_path(f"{base}/✏️editor/🦀️.rs")
    plan.edit(editor_rs, lambda text: once(
        text,
        "            .action_args(\"setActiveExample\", vec![\n"
        "                ActionArgDef::select(\"exampleId\", LocalizedLabel::native(\"Example\", \"Beispiel\"), vec![\n"
        "                    ActionArgOption::new(\"jack\", LocalizedLabel::native(\"Jack\", \"Jack\")),\n"
        "                    ActionArgOption::new(\"dag.jack\", LocalizedLabel::native(\"Dag Jack\", \"Dag Jack\")),\n"
        "                ]).default_value(&\"jack\"),\n"
        "            ])\n",
        "            .action_args(\"setActiveExample\", vec![\n"
        "                ActionArgDef::select(\"exampleId\", LocalizedLabel::native(\"Example\", \"Beispiel\"), crate::standards::v1::subsets::any::examples().iter().map(|source| ActionArgOption::new(source.id(), source.label().clone())).collect())\n"
        "                    .default_value(&crate::examples::demo::ID),\n"
        "            ])\n",
        "writer: the form offers the published examples",
    ))
    plan.edit(editor_rs, lambda text: once(text, '.unwrap_or_else(|| "jack".into()) })),', '.unwrap_or_else(|| crate::examples::demo::ID.into()) })),', "writer: the form default is the published demo"))
    plan.edit(editor_rs, lambda text: once(
        text,
        '.action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document with a bundled example (the Jack demo or the DAG Jack example), or with an empty document for any other id.", "Ersetzt das gesamte Dokument durch ein mitgeliefertes Beispiel (die Jack-Demo oder das DAG-Jack-Beispiel), bei jeder anderen Id durch ein leeres Dokument."))',
        '.action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document with a bundled example (the Jack demo or the DAG Jack example); an id that is not one of them is refused.", "Ersetzt das gesamte Dokument durch ein mitgeliefertes Beispiel (die Jack-Demo oder das DAG-Jack-Beispiel); eine andere Id wird abgelehnt."))',
        "writer: describe",
    ))
    plan.edit(editor_rs, lambda text: once(
        text,
        "    fn emit(&mut self) -> Result<(Emit<WriterMutation, NoConfigMutation>, EphemeralEmit<EditorApp<WriterPlayApp>>), &'static str> {\n",
        "    fn emit(&mut self) -> Result<(Emit<WriterMutation, NoConfigMutation>, EphemeralEmit<EditorApp<WriterPlayApp>>), WriterEmitRefusal> {\n",
        "writer: emit answers typed refusals",
    ))
    plan.edit(editor_rs, lambda text: once(
        text,
        "            WriterCommand::SetActiveExample(payload) => {\n                emit.effects.push(reset_document_effect_now(&set_active_example::document_for_example_id(&payload.example_id)));\n            }\n",
        "            WriterCommand::SetActiveExample(payload) => {\n                let document = semio_framework_plugin::subset_example_snapshot::<WriterPlayApp>(crate::standards::v1::subsets::any::examples(), &payload.example_id).map_err(WriterEmitRefusal::Refused)?;\n                emit.effects.push(reset_document_effect_now(&document));\n            }\n",
        "writer: the retained job loads through the resolver",
    ))
    plan.edit(editor_rs, lambda text: once(
        text,
        "            let (emit, ephemeral) = match self.emit() {\n                Ok(output) => output,\n                Err(_) => return Self::fault(),\n            };\n            if let Err(rejected) = completion.complete(Ok(emit), ephemeral) {\n",
        "            let (outcome, ephemeral) = match self.emit() {\n                Ok((emit, ephemeral)) => (Ok(emit), ephemeral),\n                Err(WriterEmitRefusal::Refused(fault)) => (Err(fault), EphemeralEmit::default()),\n                Err(WriterEmitRefusal::Lost(_)) => return Self::fault(),\n            };\n            if let Err(rejected) = completion.complete(outcome, ephemeral) {\n",
        "writer: a refusal completes the operation by code",
    ))
    plan.edit(editor_rs, lambda text: once(
        text,
        "struct WriterCommandToolJob {",
        "/// 🛑️ Why a writer command job produced no emit: it lost one of its own owners (a job fault) or the command was refused by\n"
        "/// code — completed as the operation's outcome, so the person reads the refusal's declared text.\n"
        "enum WriterEmitRefusal {\n    Lost(&'static str),\n    Refused(semio_framework_plugin::Fault),\n}\n\n"
        "impl From<&'static str> for WriterEmitRefusal {\n    fn from(detail: &'static str) -> Self {\n        Self::Lost(detail)\n    }\n}\n\n"
        "struct WriterCommandToolJob {",
        "writer: emit refusal type",
    ))
    handler(plan, f"{base}/✏️editor/🎮️commands/🧺️set-active-example/🦀️.rs",
            "pub fn document_for_example_id(example_id: &str) -> WriterSnapshot {", "", "writer: document_for_example_id",
            uses=("dag_jack_example_document", "jack_example_document", "empty_writer_snapshot"))
    handler(plan, f"{base}/✏️editor/🎮️commands/🧺️set-active-example/🦀️.rs",
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {",
            "/// 📚️ Loads the example the ONE resolver answers for `payload.example_id` — `demo` (the Jack document) or `dag-jack`,\n"
            "/// decoded from their own assets, or the editor's empty initial document for the empty id (the navbar's No example row);\n"
            "/// any other id or an unreadable asset is refused by code.\n"
            "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {\n"
            f"    let document = {load('crate::editor::writer::WriterPlayApp')};\n"
            "    Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() })\n"
            "}\n",
            "writer: handle")


def raster_b(plan):
    """🖨️ raster — an id it does not register was a silent no-op; the boot document fell back to the empty scaffold when the
    committed carrier stopped parsing. Both go through the ONE resolver / the law now."""
    base = f"🖨️raster/🗿️artifacts/🖨️raster/{A}"
    schema = plugin_path(f"{base}/🧬️schema/🦀️.rs")
    plan.edit(schema, lambda text: once(
        text,
        "/// source of truth instead of being restated in Rust. Falls back to [`empty_raster_document`] when\n/// the carrier does not parse — the same shape `block2d`'s `default_block2d_snapshot` uses.\n",
        "/// source of truth instead of being restated in Rust. The example-catalog law holds that carrier to decode, so a carrier\n/// that stops parsing is a red law, never an empty boot.\n",
        "raster: default document doc",
    ))
    plan.edit(schema, lambda text: once(
        text,
        "    super::snapshot::text::parse_dsl(crate::examples::art_raster_demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_raster_document())\n",
        "    super::snapshot::text::parse_dsl(crate::examples::art_raster_demo::PRIMARY_TEXT).expect(\"the raster demo example decodes — pinned by the example-catalog law\")\n",
        "raster: default document decodes",
    ))
    plan.edit(schema, lambda text: once(
        text,
        "\n/// 📚️ The committed example document behind one registered example id, or `None` when the id is not\n/// one this subset registers — the lookup `🎮️commands/🎬️set-active-example` resolves against.\npub fn raster_example_document(example_id: &str) -> Option<RasterSnapshot> {\n    (example_id == crate::examples::art_raster_demo::ID).then(default_raster_document)\n}\n",
        "",
        "raster: the example lookup is the SDK resolver",
    ))
    command = f"{base}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"
    plan.edit(plugin_path(command), lambda text: once(
        text,
        "/// 🎬️ Loads one registered example over the open document. An id this subset does not register is a\n/// no-op rather than a fault — the navbar switcher is free-text on the wire, and an unknown id must\n/// not destroy the open document.\n",
        "/// 🎬️ Loads one published example over the open document, as the ONE example resolver decodes it from its committed\n/// carrier; the empty id — the navbar's No example row — is the editor's empty initial document. Any other id or a\n/// carrier that no longer decodes is refused by code, which leaves the open document untouched.\n",
        "raster: handle doc",
    ))
    plan.edit(plugin_path(command), lambda text: once(
        text,
        "    let Some(mut example) = raster_example_document(&payload.example_id) else {\n        return Ok(Emit::default());\n    };\n",
        "    let mut example = semio_framework_plugin::editor_example_snapshot::<crate::editor::raster::RasterPlayApp>(&payload.example_id)?;\n",
        "raster: handle loads through the resolver",
    ))
    plan.edit(plugin_path(command), lambda text: once(
        text,
        "use crate::standards::v1::subsets::any::schema::{layer_node_id, raster_example_document};",
        "use crate::standards::v1::subsets::any::schema::layer_node_id;",
        "raster: imports",
    ))
    boot = plugin_path(f"{base}/🧬️schema/🧪️tests/🔬️boot-document/🦀️.rs")
    plan.edit(boot, lambda text: once(
        text,
        "/// 📚️ `raster_example_document` resolves exactly the ids this subset registers, and nothing else.\n#[semio_framework_async_macros::async_test]\nasync fn only_a_registered_example_id_resolves_to_a_document() {\n    let registered = raster_example_document(crate::examples::art_raster_demo::ID).expect(\"the demo example id resolves\");\n    let expected = default_raster_document();\n    assert_eq!(registered, expected);\n    assert!(raster_example_document(\"not-a-real-example\").is_none());\n",
        "/// 📚️ The ONE example resolver loads exactly the ids this subset publishes, and refuses every other id by code.\n#[semio_framework_async_macros::async_test]\nasync fn only_a_registered_example_id_resolves_to_a_document() {\n    let registered = semio_framework_plugin::editor_example_snapshot::<crate::editor::raster::RasterPlayApp>(crate::examples::art_raster_demo::ID).expect(\"the demo example id resolves\");\n    let expected = default_raster_document();\n    assert_eq!(registered, expected);\n    assert_eq!(semio_framework_plugin::editor_example_snapshot::<crate::editor::raster::RasterPlayApp>(\"not-a-real-example\").expect_err(\"an unpublished id is refused\").code.0, \"app.example.unknown\");\n",
        "raster: boot-document law",
    ))
    plan.edit(plugin_path(f"{base}/✏️editor/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(
        text,
        "    let example = raster_example_document(crate::examples::art_raster_demo::ID).expect(\"the demo example document\");\n",
        "    let example = semio_framework_plugin::editor_example_snapshot::<crate::editor::raster::RasterPlayApp>(crate::examples::art_raster_demo::ID).expect(\"the demo example document\");\n",
        "raster: handler law",
    ))


def block_b(plan):
    """🧱️ block 2d/3d/5d — an id outside the fixtures or a fixture that stopped parsing was a silent no-op; the boot document
    fell back to the empty snapshot. Handlers load through the ONE resolver; boots are pinned by the example-catalog law."""
    for dim, editor in (("◻️2d", "crate::editor::block2d::Block2dPlayApp"), ("🧊️3d", "crate::editor::block3d::Block3dPlayApp"), ("🖐️5d", "crate::editor::block5d::Block5dPlayApp")):
        n = {"◻️2d": "2", "🧊️3d": "3", "🖐️5d": "5"}[dim]
        base = f"🧱️block/🗿️artifacts/{dim}/{A}"
        header = f"pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block{n}dSnapshot>, _cfg: &ConfigView<'_, Block{n}dConfig>) -> Result<Emit<Block{n}dMutation, Block{n}dConfigMutation>, Fault> {{"
        handler(plan, f"{base}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs", header,
                "/// 🎬️ Loads the published example `payload.id` names — as the ONE example resolver decodes it from its committed fixture —\n"
                "/// as the minimal ordered batch of semantic mutations from the open document ([`replace_document_operations`]); the\n"
                "/// empty id — the navbar's No example row — loads the empty document. Any other id or an unreadable fixture is refused by\n"
                "/// code, never a silent no-op.\n"
                f"{header}\n"
                f"    let example = if payload.id.is_empty() {{ crate::standards::v1::subsets::any::schema::empty_block{n}d_snapshot() }} else {{ {load(editor, '&payload.id')} }};\n"
                "    Ok(Emit::mutations(replace_document_operations(doc.snapshot, &example)))\n"
                "}\n",
                f"block {dim}: handle")
    t3 = plugin_path(f"🧱️block/🗿️artifacts/🧊️3d/{A}/🧬️schema/📸️snapshot/📝️text/🦀️.rs")
    plan.edit(t3, lambda text: once(
        text,
        "/// so the `World3d` surface renders a real mesh instead of an empty scene on first paint. Falls back to\n/// the empty document if the embedded fixture ever stops parsing — a boot must never fault on a fixture.\npub fn block3d_boot_snapshot() -> Block3dSnapshot {\n    parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()\n}\n",
        "/// so the `World3d` surface renders a real mesh instead of an empty scene on first paint. The example-catalog law holds the\n/// embedded fixture to decode, so a fixture that stops parsing is a red law, never an empty boot.\npub fn block3d_boot_snapshot() -> Block3dSnapshot {\n    parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).expect(\"the block3d boot example decodes — pinned by the example-catalog law\")\n}\n",
        "block 3d: boot",
    ))
    for dim, n in (("🖐️5d", "5"), ("◻️2d", "2")):
        schema = plugin_path(f"🧱️block/🗿️artifacts/{dim}/{A}/🧬️schema/🦀️.rs")
        plan.edit(schema, lambda text, n=n: once(
            text,
            f"/// example fixture (the same DSL text `setActiveExample` loads), falling back to the empty snapshot\n/// only when that fixture fails to parse. Shared by `Block{n}dPlayApp` and `Block{n}dViewer`.\n",
            f"/// example fixture (the same DSL text `setActiveExample` loads), which the example-catalog law holds to decode.\n/// Shared by `Block{n}dPlayApp` and `Block{n}dViewer`.\n",
            f"block {n}d: boot doc",
        ))
        plan.edit(schema, lambda text, n=n: once(
            text,
            f"    super::snapshot::text::parse_dsl(super::snapshot::text::BLOCK{n}D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_block{n}d_snapshot())\n",
            f"    super::snapshot::text::parse_dsl(super::snapshot::text::BLOCK{n}D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).expect(\"the block{n}d boot example decodes — pinned by the example-catalog law\")\n",
            f"block {n}d: boot",
        ))


def writer_tests_b(plan):
    """✒️ writer laws follow the published ids: `dag-jack` loads its fixture, an unpublished id is refused by code."""
    tests = plugin_path(f"✒️writer/🗿️artifacts/✒️writer/{A}/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs")
    plan.edit(tests, lambda text: once(
        text,
        'WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "dag.jack".into() })',
        "WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::dag_jack::ID.into() })",
        "writer: dag-jack law dispatches the published id",
    ))
    plan.edit(tests, lambda text: once(
        text,
        "/// loads is the jack fixture. The dispatched id used to be `\"jack\"`, which\n/// `document_for_example_id` has never published — it fell through to the empty document, so this law\n/// proved the opposite of its own name.\n",
        "/// loads is the jack fixture. The dispatched id used to be `\"jack\"`, which this app has never published — it fell\n/// through to the empty document, so this law proved the opposite of its own name.\n",
        "writer: jack law doc",
    ))
    plan.edit(tests, lambda text: once(
        text,
        "/// 🪹 An id this app does not publish resets to the EMPTY document rather than faulting (the navbar\n"
        "/// dispatches whatever its combobox holds). The EMPTY id is not that case: it means \"load my default\n"
        "/// example\", exactly like `📽️animate`'s own `setActiveExample` — so this law now files an id the app\n"
        "/// genuinely does not publish.\n"
        "#[semio_framework_async_macros::async_test]\n"
        "async fn set_active_example_falls_back_to_empty_document() {\n"
        "    let mut app = app_with_jack().await;\n"
        "    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: \"not-a-published-example\".into() })).await;\n"
        "    assert!(result.mutations.is_empty());\n"
        "    let projection = loaded_document(&result);\n"
        "    assert_eq!(projection.id, \"empty\");\n"
        "    assert_eq!(writer_text(&projection), \"\");\n"
        "}\n",
        "/// 🪹 An id this app does not publish is refused by code — never an EMPTY document in its place (it used to reset the\n"
        "/// open document to the empty one, silently). The dispatch half is the example-catalog law in `🧪️tests/🔬️surface`.\n"
        "#[semio_framework_async_macros::async_test]\n"
        "async fn set_active_example_refuses_an_unpublished_id() {\n"
        "    let fault = semio_framework_plugin::subset_example_snapshot::<crate::editor::writer::WriterPlayApp>(crate::standards::v1::subsets::any::examples(), \"not-a-published-example\").expect_err(\"an unpublished example is refused\");\n"
        "    assert_eq!(fault.code.0, \"app.example.unknown\");\n"
        "}\n",
        "writer: unpublished id law",
    ))


def wfc_b(plan):
    """🀄️ wfc — bitmap answered an unknown id with a silent no-op; wfc3d/grid3d/wfc2d/grid2d refused with four private codes.
    All five load through the ONE resolver (`app.example.unknown` / `app.example.unreadable`)."""
    w = "🀄️wfc/🗿️artifacts"
    wfc3d = f"{w}/🧊️3d/{A}/✏️editor/🦀️.rs"
    handler(plan, wfc3d, "pub fn example_snapshot(example_id: &str) -> Result<Wfc3dSnapshot, Fault> {",
            "/// 📚️ The document an example id loads: the empty id — the navbar's No example row — is the boot example\n"
            "/// (`two-room-corridor`); every other id is the published example the ONE resolver decodes from its committed asset, and\n"
            "/// an id this artifact never published is refused by code instead of silently opening a blank document.\n"
            "pub fn example_snapshot(example_id: &str) -> Result<Wfc3dSnapshot, Fault> {\n"
            "    if example_id.is_empty() {\n"
            "        return Ok(crate::examples::two_room_corridor::snapshot());\n"
            "    }\n"
            "    semio_framework_plugin::subset_example_snapshot::<Wfc3dEditor>(crate::standards::v1::subsets::any::examples(), example_id)\n"
            "}\n",
            "wfc3d: example_snapshot", uses=("app_fault",))
    faults(plan, wfc3d, ["wfc3d.example.unknown"], "wfc3d")
    grid3d = f"{w}/🧱️grid3d/{A}/✏️editor/🦀️.rs"
    handler(plan, grid3d, "pub fn example_snapshot(example_id: &str) -> Option<Grid3dSnapshot> {", "", "grid3d: example_snapshot")
    plan.edit(plugin_path(grid3d), lambda text: once(
        text,
        "            let Some(next_document) = example_snapshot(example_id) else {\n                return Err(app_fault(\"wfc.grid3d.example.unknown\").with_parameter(\"example\", example_id));\n            };\n",
        "            let next_document = semio_framework_plugin::subset_example_snapshot::<Grid3dEditor>(crate::standards::v1::subsets::any::examples(), example_id)?;\n",
        "grid3d: handler loads through the resolver",
    ))
    plan.edit(plugin_path(grid3d), lambda text: drop_use(text, "app_fault", "grid3d: use app_fault"))
    faults(plan, grid3d, ["wfc.grid3d.example.unknown"], "grid3d")
    plan.edit(plugin_path(f"{w}/🧱️grid3d/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(
        text,
        "        assert!(example_snapshot(id).is_some(), \"example '{id}' must resolve\");\n    }\n    assert!(example_snapshot(\"nope\").is_none());\n",
        "        assert!(semio_framework_plugin::subset_example_snapshot::<Grid3dEditor>(crate::standards::v1::subsets::any::examples(), id).is_ok(), \"example '{id}' must resolve\");\n    }\n    assert_eq!(semio_framework_plugin::subset_example_snapshot::<Grid3dEditor>(crate::standards::v1::subsets::any::examples(), \"nope\").expect_err(\"an unpublished id is refused\").code.0, \"app.example.unknown\");\n",
        "grid3d: picker law",
    ))
    wfc2d = f"{w}/◻️2d/{A}/✏️editor/🦀️.rs"
    handler(plan, wfc2d, "pub fn wfc2d_example_document(example_id: &str) -> Option<Wfc2dSnapshot> {", "", "wfc2d: example document")
    plan.edit(plugin_path(wfc2d), lambda text: once(
        text,
        "            let Some(next) = wfc2d_example_document(example_id) else {\n                return Err(app_fault(\"wfc2d.example.unknown\").with_parameter(\"example\", example_id));\n            };\n",
        "            let next = semio_framework_plugin::subset_example_snapshot::<Wfc2dEditor>(crate::standards::v1::subsets::any::examples(), example_id)?;\n",
        "wfc2d: handler loads through the resolver",
    ))
    plan.edit(plugin_path(wfc2d), lambda text: drop_use(text, "app_fault", "wfc2d: use app_fault"))
    faults(plan, wfc2d, ["wfc2d.example.unknown"], "wfc2d")
    plan.edit(plugin_path(f"{w}/◻️2d/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(text, '    assert_eq!(fault.code.0, "wfc2d.example.unknown");', '    assert_eq!(fault.code.0, "app.example.unknown");', "wfc2d: unknown-id law code"))
    grid2d = f"{w}/🔲️grid2d/{A}/✏️editor/🦀️.rs"
    handler(plan, grid2d, "pub fn example_document(example_id: &str) -> Option<Grid2dSnapshot> {", "", "grid2d: example document")
    plan.edit(plugin_path(grid2d), lambda text: once(
        text,
        "                let next = example_document(example_id).ok_or_else(|| app_fault(\"wfc-grid2d-unknown-example\").with_parameter(\"example\", example_id))?;\n",
        "                let next = semio_framework_plugin::subset_example_snapshot::<Grid2dEditor>(crate::standards::v1::subsets::any::examples(), example_id)?;\n",
        "grid2d: handler loads through the resolver",
    ))
    plan.edit(plugin_path(grid2d), lambda text: drop_use(text, "app_fault", "grid2d: use app_fault"))
    faults(plan, grid2d, ["wfc-grid2d-unknown-example"], "grid2d")
    plan.edit(plugin_path(f"{w}/🔲️grid2d/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(
        text,
        "        assert!(example_document(source.id()).is_some(), \"{} is offered by the picker but resolves to no document\", source.id());\n    }\n    assert_eq!(example_document(\"pipes\"), Some(crate::examples::grid2d::pipes::document()));\n    assert!(example_document(\"not-an-example\").is_none());\n",
        "        assert!(semio_framework_plugin::subset_example_snapshot::<Grid2dEditor>(crate::standards::v1::subsets::any::examples(), source.id()).is_ok(), \"{} is offered by the picker but resolves to no document\", source.id());\n    }\n    assert_eq!(semio_framework_plugin::subset_example_snapshot::<Grid2dEditor>(crate::standards::v1::subsets::any::examples(), \"pipes\").expect(\"the pipes example loads\"), crate::examples::grid2d::pipes::document());\n    assert_eq!(semio_framework_plugin::subset_example_snapshot::<Grid2dEditor>(crate::standards::v1::subsets::any::examples(), \"not-an-example\").expect_err(\"an unpublished id is refused\").code.0, \"app.example.unknown\");\n",
        "grid2d: switcher law",
    ))
    bitmap = f"{w}/🖼️bitmap/{A}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"
    handler(plan, bitmap, "pub fn example_snapshot(example_id: &str) -> Option<BitmapSnapshot> {", "", "bitmap: example_snapshot")
    plan.edit(plugin_path(bitmap), lambda text: once(
        text,
        "    let Some(next) = example_snapshot(example_id) else { return Ok(Emit::default()) };\n",
        "    let next = semio_framework_plugin::subset_example_snapshot::<crate::editor::bitmap::BitmapEditor>(crate::examples::example_source_slice(), example_id)?;\n",
        "bitmap: handler loads through the resolver",
    ))
    plan.edit(plugin_path(bitmap), lambda text: once(
        text,
        "/// 🎬️ Replaces the open document's declared state with the named example's.\n",
        "/// 🎬️ Replaces the open document's declared state with the named example's, as the ONE example resolver decodes it from\n/// its committed asset (the empty id is the boot example); any other id or an unreadable asset is refused by code.\n",
        "bitmap: handler doc",
    ))
    btests = plugin_path(f"{w}/🖼️bitmap/{A}/✏️editor/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs")
    plan.edit(btests, lambda text: once(text, "    let next = example_snapshot(example_id).expect(\"a bundled example\");\n", "    let next = semio_framework_plugin::subset_example_snapshot::<crate::editor::bitmap::BitmapEditor>(crate::examples::example_source_slice(), example_id).expect(\"a bundled example\");\n", "bitmap: replay law"))
    plan.edit(btests, lambda text: once(
        text,
        "fn an_unknown_example_id_is_a_no_op_rather_than_a_fault() {\n    assert!(example_snapshot(\"no-such-example\").is_none());\n",
        "fn an_unknown_example_id_is_refused_by_code() {\n    assert_eq!(semio_framework_plugin::subset_example_snapshot::<crate::editor::bitmap::BitmapEditor>(crate::examples::example_source_slice(), \"no-such-example\").expect_err(\"an unpublished id is refused\").code.0, \"app.example.unknown\");\n",
        "bitmap: unknown-id law",
    ))


NORM_ROSTER = re.compile(
    r"    let Some\(snapshot\) = crate::app_surface::roster_example_snapshot\(<(?P<editor>[\w:]+) as ArtifactEditor>::examples\(\), &payload\.example_id\)\? else \{\n        return Ok\(Emit::default\(\)\);\n    \};\n"
)


def norm_b(plan):
    """📕️ norm — 13 roster editors answered an id outside the roster with a silent no-op, en1990/din18599 with their own
    silent no-op maps. All 15 load through the ONE resolver; `roster_example_snapshot` and its code go."""
    root = f"{PLUGINS}/📕️norm/🗿️artifacts"
    for artifact in sorted(os.listdir(os.path.join(TREE, root))) if os.path.isdir(os.path.join(TREE, root)) else []:
        path = f"{root}/{artifact}/{A}/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
        if not os.path.isfile(os.path.join(TREE, path)) or artifact in ("⚖️en1990", "⚡️din18599"):
            continue

        def roster(text, path=path):
            match = NORM_ROSTER.search(text)
            if match is None:
                if "semio_framework_plugin::editor_example_snapshot::<" in text:
                    notes.append(f"applied norm roster {path}")
                else:
                    problems.append(f"{path}: roster load not found")
                return text
            load_line = f"    let snapshot = if payload.example_id.is_empty() {{ Default::default() }} else {{ semio_framework_plugin::editor_example_snapshot::<{match['editor']}>(&payload.example_id)? }};\n"
            text = text[: match.start()] + load_line + text[match.end():]
            text = text.replace("/// 🎨️ Replaces the live document with the named roster example, or clears it when the id is empty.\n",
                                "/// 🎨️ Replaces the live document with the named example as the ONE example resolver decodes it, or clears it when the\n/// id is empty; any other id or an unreadable asset is refused by code.\n")
            return drop_use(text, "ArtifactEditor", f"{path}: use ArtifactEditor")

        plan.edit(path, roster)
    for artifact, snapshot, editor, wrap in (
        ("⚖️en1990", "En1990Snapshot", "crate::editor::en1990::En1990PlayApp", "text"),
        ("⚡️din18599", "Din18599Snapshot", "crate::editor::din18599::Din18599PlayApp", "text: crate::document::escape_op_text_field(&text)"),
    ):
        header = f"pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, {snapshot}>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<{snapshot.replace('Snapshot', 'Mutation')}, NoConfigMutation>, Fault> {{"
        handler(plan, f"📕️norm/🗿️artifacts/{artifact}/{A}/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs", header,
                "/// 🎨️ Replaces the live document with the named example as the ONE example resolver decodes it from its `PRIMARY_TEXT`,\n"
                "/// or clears it when the id is empty; any other id or an unreadable asset is refused by code.\n"
                f"{header}\n"
                f"    let document = if payload.example_id.is_empty() {{ {snapshot}::default() }} else {{ {load(editor)} }};\n"
                f"    let text = <{snapshot} as store::ArtifactDsl>::print_dsl(&document);\n"
                f"    set_snapshot::handle(&set_snapshot::ReplaceSnapshot {{ {wrap} }}, doc, cfg)\n"
                "}\n",
                f"norm {artifact}: handle")
    surface = f"{PLUGINS}/📕️norm/🖥️app-surface/🦀️.rs"
    plan.edit(surface, lambda text: (lambda cut: drop_use(cut, "ExampleSource", "norm: use ExampleSource"))(
        once(text, text[text.index("//#region 🔖️Examples\n"):text.index("//#endregion 🔖️Examples\n") + len("//#endregion 🔖️Examples\n") + 1] if "//#region 🔖️Examples\n" in text and "pub fn roster_example_snapshot" in text else "\x00roster-gone", "", "norm: roster_example_snapshot is the SDK resolver")))
    faults(plan, "📕️norm/🖥️app-surface/🦀️.rs", ["norm.set-active-example-invalid"], "norm")


# ─── examples-stdio (every stdio editor publishes a real example; the law found 16 semio subsets decoding the base scaffold
#     and 26 editors publishing none) ──────────────────────────────────────────────────────────────────────────────────

SEMIO = f"{PLUGINS}/🗄️stdio/🗿️artifacts/🧿️semio"
SEMIO_SUBSETS = f"{SEMIO}/🏅️standards/🔖️v1/🪆️subsets"
SEMIO_OWN_EXAMPLES = {
    "🌊️flow": ["crate::v1_subsets_flow_examples_pipeline"],
    "🎞️animation": ["crate::v1_subsets_animation_examples_walk"],
    "🎬️video": ["crate::v1_subsets_video_examples_clip"],
    "🏛️model": ["crate::v1_subsets_model_examples_building"],
    "📊️table": ["crate::standards::v1::subsets::table::examples::sheet"],
    "📐️cad": ["crate::v1_subsets_cad_examples_drawing"],
    "📑️document": ["crate::v1_subsets_document_examples_memo"],
    "📦️object": ["crate::standards::v1::subsets::object::examples::crate_"],
    "📽️presentation": ["crate::v1_subsets_presentation_examples_deck"],
    "🔊️audio": ["crate::v1_subsets_audio_examples_tone"],
    "🔢️value": ["crate::v1_subsets_value_examples_graph"],
    "🕸️graph": ["crate::standards::v1::subsets::graph::examples::wires"],
    "🧰️kit": ["crate::standards::v1::subsets::kit::examples::furniture"],
    "🔤️text": ["crate::v1_subsets_text_examples_note"],
    "🖊️drawing": ["crate::v1_subsets_drawing_examples_sketch"],
    "🖼️image": ["crate::v1_subsets_image_examples_swatch"],
    "✉️base": ["crate::v1_subsets_base_examples_envelope"],
}
SEMIO_NEW_LEAVES = {
    "🔤️text": ("📃️note", "v1_subsets_text_examples_note", "note", "Note", "Notiz", None, "stdio.semio.text"),
    "🖊️drawing": ("✏️sketch", "v1_subsets_drawing_examples_sketch", "sketch", "Sketch", "Skizze", "🧫️fixtures/🖊️mutate-semio-drawing/🗣️.dsl.semio", "stdio.semio.drawing"),
    "🖼️image": ("🎨️swatch", "v1_subsets_image_examples_swatch", "swatch", "Swatch", "Farbfeld", "🧫️fixtures/🖼️mutate-semio-image/🗣️.dsl.semio", "stdio.semio.image"),
}
STDIO_DEMO_ONLY_EDITORS = [
    ("📖️pdf", ["Pdf14Editor", "Pdf14AEditor", "Pdf14XEditor", "Pdf17Editor", "Pdf17AEditor", "Pdf17EEditor", "Pdf17HEditor", "Pdf17UaEditor", "Pdf17VtEditor", "Pdf17XEditor"]),
    ("📜️docx", ["DocxEditor", "DocxStrictEditor", "DocxTransitionalEditor"]),
    ("📕️xlsx", ["XlsxEditor", "XlsxStrictEditor", "XlsxTransitionalEditor"]),
    ("📽️pptx", ["PptxEditor", "PptxStrictEditor", "PptxTransitionalEditor"]),
    ("🎒️zip", ["ZipAnyEditor", "ZipIso21320Editor"]),
    ("💾️binary", ["BinaryEditor"]),
    ("🗜️deflate", ["DeflateEditor"]),
    ("🌦️epw", ["EpwEditor"]),
]


def publish_examples(text, editor, modules, label):
    """📚️ `impl ArtifactEditor for <editor>` publishes exactly `modules`' examples (its `examples()` added or replaced)."""
    body = "vec![" + ", ".join(f"{module}::source()" for module in modules) + "]"
    block = f"    /// 📚️ The examples this editor's picker offers and `setActiveExample` loads.\n    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {{\n        {body}\n    }}\n"
    head = f"impl ArtifactEditor for {editor} {{\n"
    if head not in text:
        problems.append(f"{label}: no `{head.strip()}`")
        return text
    start = text.index(head) + len(head)
    _, end = item_span(text, text.index(head))
    impl = text[start:end]
    existing = re.search(r"(?:[ \t]*///[^\n]*\n)*[ \t]*fn examples\(\) -> Vec<[\w:]*ExampleSource> \{\n[^\n]*\n[ \t]*\}\n", impl)
    if existing:
        if body in existing.group(0):
            notes.append(f"applied {label}")
            return form_follows(text, modules)
        return form_follows(text[:start] + impl[: existing.start()] + block + impl[existing.end():] + text[end:], modules)
    return form_follows(text[:start] + block + text[start:], modules)


DEMO_FORM = "set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID)"


def form_follows(text, modules):
    """🧾️ The editor's `setActiveExample` form offers exactly the examples it publishes (default: the first)."""
    if modules == ["crate::examples::demo"] or DEMO_FORM not in text:
        return text
    options = ", ".join(f"({module}::ID, {module}::label())" for module in modules)
    return text.replace(DEMO_FORM, f"set_active_example_args(&[{options}], {modules[0]}::ID)")


SEMIO_BASE_EXAMPLES = f"{SEMIO_SUBSETS}/✉️base/📚️examples"


def semio_base_leaves(plan, subset, leaf):
    """🧿️ semio base — its `📚️examples` held two leaves no base document is: `📃️note`, the genuine `stdio.semio.text` demo
    (`print_dsl(demo_text_snapshot())`, pinned by the text honesty law), which moves to the text subset that owns it, and
    `🎬️demo`, a hex scaffold that parsed as no document at all, which goes (the base editor publishes its 13 own leaves)."""
    target = f"{SEMIO_SUBSETS}/{subset}/📚️examples/{leaf}"
    for asset in ("🦀️.rs", "🟦️.ts", "🖼️assets/🗣️.dsl.semio", "🖼️assets/🎒️.pack.semio", "🧪️tests/🔬️unit/🦀️.rs"):
        plan.copy(f"{SEMIO_BASE_EXAMPLES}/📃️note/{asset}", f"{target}/{asset}")
        plan.delete(f"{SEMIO_BASE_EXAMPLES}/📃️note/{asset}")
        plan.delete(f"{SEMIO_BASE_EXAMPLES}/🎬️demo/{asset}")
    plan.edit(f"{SEMIO}/🦀️.rs", lambda text: once(
        text,
        "#[path = \".\"]\npub mod examples {\n    #[path = \"🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/🎬️demo/🦀️.rs\"]\n    pub mod demo;\n"
        "    #[path = \"🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/📃️note/🦀️.rs\"]\n    pub mod note;\n}\n\n",
        "", "semio: the base examples module goes (note moved, scaffold demo removed)"))
    text_io = f"{SEMIO_SUBSETS}/🔤️text/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs"
    for kind in ("include_str!(\"../../../../✉️base/📚️examples/📃️note/🖼️assets/🗣️.dsl.semio\")", "include_bytes!(\"../../../../✉️base/📚️examples/📃️note/🖼️assets/🎒️.pack.semio\")"):
        plan.edit(text_io, lambda text, kind=kind: once(text, kind, kind.replace("../../../../✉️base/📚️examples/", "../../../📚️examples/"), "semio text: honesty law reads its own leaf"))
    plan.edit(f"{SEMIO_SUBSETS}/📊️table/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs", lambda text: once(text, "from `✉️base/📚️examples/📃️note/…`", "from `🔤️text/📚️examples/📃️note/…`", "semio table: doc names the note's home"))


MP4_DEMO = f"{PLUGINS}/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/📚️examples/🎬️demo"
MP4_DEMO_RS = """//! 📚️ Example `demo` for `stdio.mp4` — the real 43 KB `logo.mp4` (h264, 410×140, 1441 frames). Its document is derived from
//! those bytes by the artifact's own codec (`decode_mp4`, printed in the snapshot DSL) whenever a consumer asks for it, so
//! no generated text can drift from the file (the scaffold's hex dump of the bytes never parsed as a document).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const SOURCE_MP4: &[u8] = include_bytes!("🖼️assets/🎬️.mp4");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::deferred(ID, label(), ICON, ".mp4", SOURCE_MP4, document_dsl)
}

/// 🎬️ The demo document: the real file decoded by the mp4 codec, printed in the snapshot's own DSL.
fn document_dsl() -> String {
    let snapshot = crate::standards::isobmff::subsets::any::io::decode_mp4(SOURCE_MP4).expect("the demo mp4 decodes — pinned by the example-catalog law");
    store::ArtifactDsl::print_dsl(&snapshot)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
"""
MP4_DEMO_TEST = """use super::*;

/// ⚖️ LAW: the demo document IS the real file — the published body parses back to exactly what the codec decodes from it.
#[semio_framework_async_macros::async_test]
async fn demo_document_is_the_decoded_real_file() {
    let decoded = crate::standards::isobmff::subsets::any::io::decode_mp4(SOURCE_MP4).expect("the demo mp4 decodes");
    let published = <crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot as store::ArtifactDsl>::parse_dsl(&source().document()).expect("the published demo document parses");
    assert_eq!(published, decoded);
}
"""


def mp4_demo(plan):
    """🎥️ mp4 — its demo published a hex dump of the real file's bytes as if it were the snapshot DSL (`app.example.unreadable`:
    expected Text, found Absent) next to an empty pack. The demo now derives its document from the real `🎬️.mp4` through the
    codec (a deferred example body), and the two fake assets go."""
    plan.edit(f"{MP4_DEMO}/🦀️.rs", lambda text: MP4_DEMO_RS if "scaffolded by W1b" in text else (notes.append("applied mp4 demo leaf") or text))
    plan.edit(f"{MP4_DEMO}/🧪️tests/🔬️unit/🦀️.rs", lambda text: MP4_DEMO_TEST if "demo_source_nonempty" in text else (notes.append("applied mp4 demo test") or text))
    for asset in ("🗣️.dsl.semio", "🎒️.pack.semio"):
        plan.delete(f"{MP4_DEMO}/🖼️assets/{asset}")


IFC = f"{PLUGINS}/🗄️stdio/🗿️artifacts/🏗️ifc"
IFC_SHARED_DEMO = f"{IFC}/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo"
IFC4_DEMO = f"{IFC}/🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo"
IFC2X3_FIXTURE = f"{IFC}/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧫️fixtures/🎬️demo"
IFC_LEAF = """//! 📚️ Example `demo` for {what}

use semio_framework_plugin::{{ExampleSource, LocalizedLabel}};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {{
    LocalizedLabel::native("Demo", "Demo")
}}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {{
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
"""
IFC4_WHAT = ("stdio.ifc 4 — `demo_ifc_snapshot()`, a real minimal IFC4 exchange structure. Its assets are written only by\n"
             "//! the IFC4 generator `zzz_write_demo_fixtures` and pinned by its `fixture_honesty_law` (`🚪️io` tests).")
IFC2X3_WHAT = ("stdio.ifc 2x3 — `demo_ifc2x3_snapshot()`, this standard's own demo document. Its assets are\n"
               "//! written only by the 2x3 generator `zzz_write_demo_fixtures` and pinned by its `fixture_honesty_law` (`🚪️io` tests).")


def ifc_split(plan):
    """🏗️ ifc — ONE `demo` leaf under the 2x3 base subset held IFC4's `stdio.ifc.dsl` document and all five editors published it:
    the four 2x3 editors decoded an IFC4 Part-21 body as their `stdio.ifc.2x3` snapshot (`app.example.unreadable`), while the
    2x3 standard's real demo lived as a test-only fixture beside it. Each standard now publishes its own leaf: the IFC4 demo
    moves to the IFC4 subset (`examples::ifc4`), the 2x3 leaf carries the 2x3 generator's output (`examples::ifc2x3`), both
    honesty laws and generators point at their own leaf."""
    for asset in ("🖼️assets/🗣️.dsl.semio", "🖼️assets/🎒️.pack.semio", "🖼️assets/🧪️example/🏗️.ifc", "🟦️.ts", "🧪️tests/🔬️unit/🦀️.rs"):
        plan.copy(f"{IFC_SHARED_DEMO}/{asset}", f"{IFC4_DEMO}/{asset}")
    plan.create(f"{IFC4_DEMO}/🦀️.rs", IFC_LEAF.format(what=IFC4_WHAT))
    for asset in ("🗣️.dsl.semio", "🎒️.pack.semio"):
        plan.copy(f"{IFC2X3_FIXTURE}/{asset}", f"{IFC_SHARED_DEMO}/🖼️assets/{asset}", overwrite=True)
        plan.delete(f"{IFC2X3_FIXTURE}/{asset}")
    plan.delete(f"{IFC_SHARED_DEMO}/🖼️assets/🧪️example/🏗️.ifc")
    plan.edit(f"{IFC_SHARED_DEMO}/🦀️.rs", lambda text: IFC_LEAF.format(what=IFC2X3_WHAT) if text.startswith("//! 📚️ Example demo for stdio.ifc.\n") else (notes.append("applied ifc 2x3 leaf") or text))
    mount = ("pub mod examples {\n    #[path = \".\"]\n    pub mod demo {\n        #[path = \"🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs\"]\n"
             "        mod component;\n        pub use component::*;\n    }\n}\n")
    mounts = ("pub mod examples {\n    #[path = \".\"]\n    pub mod ifc4 {\n        #[path = \"🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs\"]\n"
              "        mod component;\n        pub use component::*;\n    }\n"
              "    #[path = \".\"]\n    pub mod ifc2x3 {\n        #[path = \"🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs\"]\n"
              "        mod component;\n        pub use component::*;\n    }\n}\n")
    plan.edit(f"{IFC}/🦀️.rs", lambda text: once(text, mount, mounts, "ifc: one example leaf per standard"))
    editors = {"4️⃣4/🪆️subsets/✳️any": "ifc4", "🔖️2x3/🪆️subsets/🧱️base": "ifc2x3", "🔖️2x3/🪆️subsets/🏢️cobie": "ifc2x3", "🔖️2x3/🪆️subsets/🧮️sav": "ifc2x3", "🔖️2x3/🪆️subsets/🤝️cv20": "ifc2x3"}
    for subset, module in editors.items():
        plan.edit(f"{IFC}/🏅️standards/{subset}/✏️editor/🦀️.rs", lambda text, module=module, subset=subset: text.replace("crate::examples::demo::", f"crate::examples::{module}::") if "crate::examples::demo::" in text else (notes.append(f"applied ifc {subset} editor") or text))
    io4 = f"{IFC}/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs"
    plan.edit(io4, lambda text: once(text, 'include_str!("../../../../../../🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")', 'include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")', "ifc4: honesty law reads its own leaf (dsl)"))
    plan.edit(io4, lambda text: once(text, 'include_bytes!("../../../../../../🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio")', 'include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio")', "ifc4: honesty law reads its own leaf (pack)"))
    plan.edit(io4, lambda text: once(text, '.join("../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets")', '.join("../../🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets")', "ifc4: generator writes its own leaf"))
    plan.edit(f"{IFC}/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", lambda text: once(
        text, '"/../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/🧪️example/🏗️.ifc"', '"/../../🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️example/🏗️.ifc"', "ifc4: retention law reads its own leaf's native file"))
    io2 = f"{IFC}/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs"
    plan.edit(io2, lambda text: once(
        text,
        "        // 🧫️ This standard's OWN fixtures. `📚️examples/🎬️demo/🖼️assets/` beside them is the\n"
        "        // ARTIFACT-level `demo` example (`ifc::examples::demo`), whose text is IFC4's\n"
        "        // `stdio.ifc.dsl` Part-21 body — the registered `s.stdio.ifc` codec's, asserted by `4`'s own\n"
        "        // `fixture_honesty_law`. `Ifc2x3Snapshot` prints a different DSL entirely\n"
        "        // (`stdio.ifc.2x3`, the tagged value encoding), so pointing this law at that shared asset\n"
        "        // only ever asserted one standard's fixture against another standard's printer.\n",
        "", "ifc 2x3: honesty law comment (its leaf is its own now)"))
    plan.edit(io2, lambda text: once(text, 'include_str!("../../../🧫️fixtures/🎬️demo/🗣️.dsl.semio")', 'include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")', "ifc 2x3: honesty law reads its leaf (dsl)"))
    plan.edit(io2, lambda text: once(text, 'include_bytes!("../../../🧫️fixtures/🎬️demo/🎒️.pack.semio")', 'include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio")', "ifc 2x3: honesty law reads its leaf (pack)"))
    plan.edit(io2, lambda text: once(text, '.join("../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧫️fixtures/🎬️demo");\n        std::fs::create_dir_all(&assets).expect("create 🧫️fixtures/🎬️demo");\n',
                                     '.join("../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets");\n', "ifc 2x3: generator writes its leaf"))


EPW_DEMO = f"{PLUGINS}/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo"
EPW_DEMO_RS = """//! 📚️ Example `demo` for `stdio.epw` — a real EnergyPlus weather file (Hannover, 8 header records + 24 hourly rows). Its
//! document is derived from the file by the artifact's own codec (`decode_epw`, printed in the snapshot DSL) whenever a
//! consumer asks for it, so no generated text can drift from the file (the scaffold's hex dump never parsed as a document).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const SOURCE_EPW: &str = include_str!("🖼️assets/🌦️.epw");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::deferred(ID, label(), ICON, ".epw", SOURCE_EPW.as_bytes(), document_dsl)
}

/// 🌦️ The demo document: the real weather file decoded by the epw codec, printed in the snapshot's own DSL.
fn document_dsl() -> String {
    let snapshot = crate::standards::energyplus::subsets::any::io::decode_epw(SOURCE_EPW).expect("the demo weather file decodes — pinned by the example-catalog law");
    store::ArtifactDsl::print_dsl(&snapshot)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
"""
EPW_DEMO_TEST = """use super::*;

/// ⚖️ LAW: the demo document IS the real weather file — the published body parses back to exactly what the codec decodes.
#[semio_framework_async_macros::async_test]
async fn demo_document_is_the_decoded_real_file() {
    let decoded = crate::standards::energyplus::subsets::any::io::decode_epw(SOURCE_EPW).expect("the demo weather file decodes");
    let published = <crate::EpwSnapshot as store::ArtifactDsl>::parse_dsl(&source().document()).expect("the published demo document parses");
    assert_eq!(published, decoded);
}
"""


def epw_demo(plan):
    """🌦️ epw — its demo published a hex dump of a real weather file as if it were the snapshot DSL (`app.example.unreadable`:
    expected at least 8 header lines) next to an empty pack. The file itself becomes the asset (`🌦️.epw`, the dump's exact
    bytes), the demo derives its document from it through the codec, and the two fake assets go."""
    hex_asset = f"{EPW_DEMO}/🖼️assets/🗣️.dsl.semio"
    if os.path.isfile(os.path.join(TREE, hex_asset)):
        plan.create(f"{EPW_DEMO}/🖼️assets/🌦️.epw", bytes.fromhex(read(hex_asset).strip()))
    plan.edit(f"{EPW_DEMO}/🦀️.rs", lambda text: EPW_DEMO_RS if "scaffolded by W1b" in text else (notes.append("applied epw demo leaf") or text))
    plan.edit(f"{EPW_DEMO}/🧪️tests/🔬️unit/🦀️.rs", lambda text: EPW_DEMO_TEST if "demo_source_nonempty" in text else (notes.append("applied epw demo test") or text))
    for asset in ("🗣️.dsl.semio", "🎒️.pack.semio"):
        plan.delete(f"{EPW_DEMO}/🖼️assets/{asset}")


def examples_stdio(plan):
    mp4_demo(plan)
    epw_demo(plan)
    ifc_split(plan)
    for artifact, editors in STDIO_DEMO_ONLY_EDITORS:
        root = f"{PLUGINS}/🗄️stdio/🗿️artifacts/{artifact}"
        found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "impl ArtifactEditor for ", root], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
        for path in sorted(path for path in found if path and "🧪️tests" not in path):
            text = read(path)
            for editor in editors:
                if f"impl ArtifactEditor for {editor} {{" in text:
                    plan.edit(path, lambda text, editor=editor, path=path: publish_examples(text, editor, ["crate::examples::demo"], f"{path}: {editor} publishes its demo"))
    for subset, modules in SEMIO_OWN_EXAMPLES.items():
        path = f"{SEMIO_SUBSETS}/{subset}/✏️editor/🦀️.rs"
        editor = re.search(r"impl ArtifactEditor for (\w+) \{", read(path)).group(1)
        plan.edit(path, lambda text, editor=editor, modules=modules, subset=subset: publish_examples(text, editor, modules, f"semio {subset}: publishes its own examples"))
    gif89a = f"{PLUGINS}/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs"
    plan.edit(gif89a, lambda text: publish_examples(text, "Gif89aEditor", ["crate::examples::dancing"], "gif89a: publishes the real GIF89a example (`dancing`), not the GIF87a demo"))
    for subset, module in (("🧊️brep", "crate::v1_subsets_brep_examples_solid"), ("🔺️mesh", "crate::standards::v1::subsets::mesh::examples::cube")):
        path = f"{SEMIO_SUBSETS}/{subset}/✏️editor/🦀️.rs"
        editor = re.search(r"impl ArtifactEditor for (\w+) \{", read(path)).group(1)
        plan.edit(path, lambda text, editor=editor, module=module, subset=subset: publish_examples(text, editor, [module], f"semio {subset}: publishes its example"))
    mounts = []
    for subset, (leaf, module, ident, en, de, fixture, schema) in SEMIO_NEW_LEAVES.items():
        leaf_path = f"{SEMIO_SUBSETS}/{subset}/📚️examples/{leaf}/🦀️.rs"
        if fixture is None:
            semio_base_leaves(plan, subset, leaf)
            mounts.append(f"#[path = \"🏅️standards/🔖️v1/🪆️subsets/{subset}/📚️examples/{leaf}/🦀️.rs\"]\npub mod {module};\n")
            continue
        plan.create(leaf_path,
                    f"//! 📚️ Example \"{ident}\" for `{schema}` — the subset's committed document fixture, published so the picker offers a\n"
                    f"//! real `{schema}` document (the editor used to decode the base scaffold and opened empty).\n\n"
                    "use semio_framework_plugin::{ExampleSource, LocalizedLabel};\n\n"
                    f"pub const ID: &str = \"{ident}\";\n"
                    f"pub fn label() -> LocalizedLabel {{\n    LocalizedLabel::native(\"{en}\", \"{de}\")\n}}\n"
                    "pub const ICON: &str = \"file\";\n"
                    f"pub const PRIMARY_TEXT: &str = include_str!(\"../../{fixture}\");\n"
                    "pub fn source() -> ExampleSource {\n    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)\n}\n")
        mounts.append(f"#[path = \"🏅️standards/🔖️v1/🪆️subsets/{subset}/📚️examples/{leaf}/🦀️.rs\"]\npub mod {module};\n")
    plan.edit(f"{SEMIO}/🦀️.rs", lambda text: once(text, "pub mod v1_subsets_video_examples_clip;\n", "pub mod v1_subsets_video_examples_clip;\n" + "".join(mounts), "semio: mount the text/drawing/image examples"))


def example_leaf(plan, subset, leaf, ident, en, de, asset, doc):
    """📚️ One published example leaf `<subset>/📚️examples/<leaf>/` — its Rust source (id, label, icon, asset text) and the
    TypeScript mirror of its id/label/icon; `asset` is relative to the subset root."""
    plan.create(f"{subset}/📚️examples/{leaf}/🦀️.rs",
                f"//! 📚️ Example `{ident}`{doc}\n\n"
                "use semio_framework_plugin::{ExampleSource, LocalizedLabel};\n\n"
                f"pub const ID: &str = \"{ident}\";\n"
                f"pub fn label() -> LocalizedLabel {{\n    LocalizedLabel::native(\"{en}\", \"{de}\")\n}}\n"
                "pub const ICON: &str = \"file\";\n"
                f"pub const PRIMARY_TEXT: &str = include_str!(\"../../{asset}\");\n"
                "pub fn source() -> ExampleSource {\n    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)\n}\n")
    plan.create(f"{subset}/📚️examples/{leaf}/🟦️.ts",
                f"/** 📚️ Example `{ident}`. */\n"
                f"export const id = \"{ident}\";\n"
                f"export const label = {{ en: \"{en}\", de: \"{de}\" }} as const;\n"
                "export const icon = \"file\";\n")


def example_mount(module, leaf_rel):
    return f"    #[path = \".\"]\n    pub mod {module} {{\n        #[path = \"{leaf_rel}\"]\n        mod component;\n        pub use component::*;\n    }}\n"


def relabel_leaf(plan, subset, leaf, en, de, label):
    """🏷️ A demo leaf that IS a named example carries that name in the picker (Rust + TypeScript mirror)."""
    plan.edit(f"{subset}/📚️examples/{leaf}/🦀️.rs", lambda text: once(text, 'LocalizedLabel::native("Demo", "Demo")', f'LocalizedLabel::native("{en}", "{de}")', f"{label}: demo label (rs)"))
    plan.edit(f"{subset}/📚️examples/{leaf}/🟦️.ts", lambda text: once(text, 'export const label = { en: "Demo", de: "Demo" } as const;', f'export const label = {{ en: "{en}", de: "{de}" }} as const;', f"{label}: demo label (ts)"))


SUBSET_ROOTS = ["✒️writer/🗿️artifacts/✒️writer", "➗️mathematical/🗿️artifacts/➗️equation", "🀄️wfc/🗿️artifacts/◻️2d", "🀄️wfc/🗿️artifacts/🔲️grid2d",
                "🀄️wfc/🗿️artifacts/🧊️3d", "🀄️wfc/🗿️artifacts/🧱️grid3d", "🌿️vcs/🗿️artifacts/🌿️vcs", "🎞️animate/🗿️artifacts/🎬️presentation",
                "🎬️sequence/🗿️artifacts/🎬️sequence", "🏗️fem/🗿️artifacts/◻️2d", "🏗️fem/🗿️artifacts/🧊️3d", "💡️reasoning/🗿️artifacts/🔌️wires",
                "📖️playbook/🗿️artifacts/📖️playbook", "🕸️dag/🗿️artifacts/🕸️dag", "🗒️note/🗿️artifacts/🗒️note", "🧱️block/🗿️artifacts/◻️2d",
                "🧱️block/🗿️artifacts/🧊️3d", "🧱️block/🗿️artifacts/🖐️5d", "🔱️trinity/🗿️artifacts/🔌️jack", "🔱️trinity/🗿️artifacts/♻️rewriting"]
SUBSET_EXAMPLES_DOC = "/// 📚️ The examples this subset publishes — the navbar picker's list and the ONE list its `setActiveExample` resolves against.\n"


def subset_examples_public(plan):
    """📚️ Every subset whose examples only its declaration publishes exposes that ONE list (`pub fn examples()`), so its
    `setActiveExample` handler resolves against it directly; forms' list moves out of `subset()` to module level."""
    for artifact in SUBSET_ROOTS:
        root = next((f"{artifact}/🏅️standards/🔖️1/🪆️subsets/{subset}/🦀️.rs" for subset in ("✳️any", "🌐️any") if os.path.isfile(os.path.join(TREE, plugin_path(f"{artifact}/🏅️standards/🔖️1/🪆️subsets/{subset}/🦀️.rs")))), None)
        if root is None:
            problems.append(f"{artifact}: no subset root")
            continue

        def publish(text, artifact=artifact):
            if "\npub fn examples() -> &'static [ExampleSource] {" in text:
                notes.append(f"applied {artifact}: pub fn examples")
                return text
            head = "\nfn examples() -> &'static [ExampleSource] {"
            if text.count(head) != 1:
                problems.append(f"{artifact}: expected 1 `fn examples()`, found {text.count(head)}")
                return text
            before = text[: text.index(head) + 1]
            doc = "" if before.rstrip("\n").split("\n")[-1].startswith("///") else SUBSET_EXAMPLES_DOC
            return before + doc + "pub " + text[text.index(head) + 1:]

        plan.edit(plugin_path(root), publish)
    forms = plugin_path(f"📋️forms/🗿️artifacts/📋️forms/{A}/🦀️.rs")
    inner = ("    use semio_framework_plugin::ExampleSource;\n    use std::sync::OnceLock;\n\n"
             "    fn examples() -> &'static [ExampleSource] {\n        static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();\n"
             "        EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()\n    }\n\n")
    hoisted = (SUBSET_EXAMPLES_DOC + "pub fn examples() -> &'static [semio_framework_plugin::ExampleSource] {\n"
               "    static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();\n"
               "    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source(), crate::examples::contact::source(), crate::examples::onboarding::source()]).as_slice()\n}\n\n")
    plan.edit(forms, lambda text: once(once(text, inner, "    use std::sync::OnceLock;\n\n", "forms: the subset's examples leave subset()"),
                                       "pub fn subset<A: crate::FormsApplication>()", hoisted + "pub fn subset<A: crate::FormsApplication>()", "forms: pub fn examples()") if inner in text else (notes.append("applied forms: pub fn examples") or text))


def cad_c(plan):
    """📐️ cad — its form offered only the concrete forest (`hexagonal-cut-concrete-forest-left`, plus a `forest-left` alias the
    handler accepted) while the navbar published only `demo`; the handler kept its own codes. The forest becomes a published
    example — a leaf deriving its document from the authored model JSON the play scene is built from — the form offers exactly
    the published examples, and every other id resolves through the ONE resolver."""
    subset = plugin_path(f"📐️cad/🗿️artifacts/📐️cad/{A}")
    leaf = f"{subset}/📚️examples/🌲️hexagonal-cut-concrete-forest-left"
    plan.create(f"{leaf}/🦀️.rs",
                "//! 📚️ Example `hexagonal-cut-concrete-forest-left` — the reused hexagonal-cut concrete forest piece with its shape,\n"
                "//! building, energy and classic-structure models, derived from the authored model JSON the play scene is built from.\n\n"
                "use semio_framework_plugin::{ExampleSource, LocalizedLabel};\n\n"
                "pub const ID: &str = \"hexagonal-cut-concrete-forest-left\";\n"
                "pub fn label() -> LocalizedLabel {\n    LocalizedLabel::native(\"Hexagonal Cut Concrete Forest Left\", \"Sechseckig geschnittener Betonwald links\")\n}\n"
                "pub const ICON: &str = \"file\";\n"
                "pub const SOURCE_JSON: &str = include_str!(\"../🖼️assets/🎮️play/🔣️.json\");\n"
                "pub fn source() -> ExampleSource {\n    ExampleSource::deferred(ID, label(), ICON, \".json\", SOURCE_JSON.as_bytes(), document_json)\n}\n\n"
                "/// 🌲️ The persisted document of the forest play scene, as the JSON the example resolver decodes.\n"
                "fn document_json() -> String {\n    dsl::os_pack::json::to_json_string(&crate::standards::v1::subsets::any::schema::inferences::forest_play_scene())\n}\n")
    plan.create(f"{leaf}/🟦️.ts",
                "/** 📚️ Example `hexagonal-cut-concrete-forest-left`. */\n"
                "export const id = \"hexagonal-cut-concrete-forest-left\";\n"
                "export const label = { en: \"Hexagonal Cut Concrete Forest Left\", de: \"Sechseckig geschnittener Betonwald links\" } as const;\n"
                "export const icon = \"file\";\n")
    pad = " " * 20
    demo = (f"{pad}#[path = \".\"]\n{pad}pub mod demo {{\n{pad}    #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs\"]\n"
            f"{pad}    mod component;\n{pad}    pub use component::*;\n{pad}}}\n")
    forest = (f"{pad}#[path = \".\"]\n{pad}pub mod hexagonal_cut_concrete_forest_left {{\n"
              f"{pad}    #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🦀️.rs\"]\n"
              f"{pad}    mod component;\n{pad}    pub use component::*;\n{pad}}}\n")
    plan.edit(plugin_path("📐️cad/🗿️artifacts/📐️cad/🦀️.rs"), lambda text: once(text, demo, demo + forest, "cad: mount the forest example"))
    editor = f"{subset}/✏️editor/🦀️.rs"
    plan.edit(editor, lambda text: once(
        text,
        "impl ArtifactEditor for CadPlayApp {\n    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.\n    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {\n        vec![crate::examples::demo::source()]\n    }\n",
        "impl ArtifactEditor for CadPlayApp {\n    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.\n    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {\n        vec![crate::examples::demo::source(), crate::examples::hexagonal_cut_concrete_forest_left::source()]\n    }\n",
        "cad: publish the forest example"))
    plan.edit(editor, lambda text: once(
        text,
        '            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![\n'
        '                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links")),\n'
        '            ]).required()])\n',
        '            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![\n'
        "                ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label()),\n"
        "                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, crate::examples::hexagonal_cut_concrete_forest_left::label()),\n"
        "            ]).required()])\n",
        "cad: form options are the published examples"))
    faults(plan, f"📐️cad/🗿️artifacts/📐️cad/{A}/✏️editor/🦀️.rs", ["cad.example.invalid", "cad.example.unknown"], "cad")
    inferences = f"{subset}/🧬️schema/💡️inferences/🦀️.rs"
    plan.edit(inferences, lambda text: once(text, '    pub const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";\n',
                                            "    pub const CAD_EXAMPLE_FOREST_LEFT: &str = crate::examples::hexagonal_cut_concrete_forest_left::ID;\n", "cad: the forest id is the published example's"))
    plan.edit(inferences, lambda text: once(text, '    const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../📚️examples/🖼️assets/🎮️play/🔣️.json");\n',
                                            "    const FOREST_LEFT_MODEL_JSON: &str = crate::examples::hexagonal_cut_concrete_forest_left::SOURCE_JSON;\n", "cad: one include of the forest model JSON"))
    header = "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {"
    handler(plan, f"📐️cad/🗿️artifacts/📐️cad/{A}/✏️editor/🎮️commands/🗺️model-definition/🦀️.rs", header,
            "    /// 🗃️ Loads the named example as ONE whole-document replacement: the empty id is the app's default document (the\n"
            "    /// navbar's No example row), the concrete forest its full play scene (with the pane materialization its model JSON\n"
            "    /// derives, which the persisted document does not carry), every other published example what the ONE resolver decodes\n"
            "    /// from its asset; any other id or an unreadable asset is refused by code.\n"
            f"{header}\n"
            "        let example_id = payload.example_id.as_str();\n"
            "        let scene = if example_id.is_empty() {\n            default_document()\n        } else if example_id == CAD_EXAMPLE_FOREST_LEFT {\n            forest_play_scene()\n        } else {\n"
            f"            {resolve('crate::editor::cad::CadPlayApp', 'example_id')}?\n        }};\n"
            "        let runtime = CadPlayRuntime { active_example_id: (!example_id.is_empty()).then(|| example_id.to_string()), ..CadPlayRuntime::default() };\n"
            "        let mut emit = Emit { effects: vec![reset_document_effect(&scene)], ..Default::default() };\n"
            "        emit.config_mutations = vec![preview_transition_snapshot_of(&runtime, cfg.snapshot, ctx)?];\n"
            "        Ok(emit)\n"
            "    }\n",
            "cad: handle", uses=("app_fault",))
    plan.edit(plugin_path(f"📐️cad/🗿️artifacts/📐️cad/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(text, '    assert_eq!(fault.code.0, "cad.example.unknown", "{fault:?}");', '    assert_eq!(fault.code.0, "app.example.unknown", "{fault:?}");', "cad: unknown-example law code"))


def batch_c_tests(plan):
    """🧪️ Unit tests that pinned the old silent behaviour: animate's unknown id was a no-op, forms loaded its contact form as `default`."""
    animate = plugin_path(f"🎞️animate/🗿️artifacts/🎬️presentation/{A}/✏️editor/🎮️commands/📥️set-source/🧪️tests/🔬️unit/🦀️.rs")
    plan.edit(animate, lambda text: once(
        text,
        "async fn set_active_example_unknown_id_is_a_no_op() {\n",
        "async fn set_active_example_refuses_an_unpublished_id() {\n",
        "animate: unknown-id test name"))
    plan.edit(animate, lambda text: once(
        text,
        "    let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: \"other\".into() }, &doc, &cfg, &mut ctx).expect(\"handle\");\n"
        "    assert!(emit.effects.is_empty());\n    assert!(emit.artifact_mutations.is_empty());\n",
        "    let fault = set_active_example::handle(&set_active_example::SetActiveExample { example_id: \"other\".into() }, &doc, &cfg, &mut ctx).expect_err(\"an unpublished example is refused\");\n"
        "    assert_eq!(fault.code.0, \"app.example.unknown\");\n",
        "animate: unknown id is refused by code"))
    forms = f"📋️forms/🗿️artifacts/📋️forms/{A}/✏️editor"
    for rel in ("🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️unit/🦀️.rs", "🎮️commands/❓️add-question/🧪️tests/🔬️unit/🦀️.rs"):
        plan.edit(plugin_path(f"{forms}/{rel}"), lambda text, rel=rel: once(text, "SetActiveExample { example_id: \"default\".into() }", "SetActiveExample { example_id: crate::examples::contact::ID.into() }", f"forms: {rel.split('/')[1]} loads the contact example"))


TRINITY_BRANCH_QUERY = "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b"


def trinity_c(plan):
    """🔱️ trinity — jack answered `nakagin`/`nakagin-capsule-tower`/`branch-chain`/`demo` from its own table (the catalogue
    panel's fixture rows sent the first ones, the navbar the last) and ANY other id was a silent no-op; rewriting answered
    `default`/`blank`/`demo` and was a silent no-op otherwise. `branch-chain` becomes a published jack example — the nakagin
    graph with its whole-graph query, an asset of its own instead of a query patched over the demo after loading — the
    catalogue rows send the published ids, and both verbs resolve through the ONE resolver (the empty id is the editor's
    initial document)."""
    jack = plugin_path(f"🔱️trinity/🗿️artifacts/🔌️jack/{A}")
    demo_asset = f"{jack}/🖼️assets/🎬️demo/🗣️.dsl.semio"
    branch_asset = f"{jack}/🖼️assets/🔗️branch-chain/🗣️.dsl.semio"
    if os.path.isfile(os.path.join(TREE, demo_asset)):
        demo = read(demo_asset)
        default_query = 'query="MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = \'b\' AND b.name != \'b\' RETURN a.name, b.name, b.label"'
        if demo.count(default_query) == 1:
            plan.create(branch_asset, demo.replace(default_query, f'query="{TRINITY_BRANCH_QUERY}"'))
        else:
            problems.append("trinity: the demo asset's query is not the default query")
    example_leaf(plan, jack, "🔗️branch-chain", "branch-chain", "Branch Chain", "Zweigkette", "🖼️assets/🔗️branch-chain/🗣️.dsl.semio",
                 " — the nakagin capsule tower graph with the query that returns every connected piece pair as a graph.")
    plan.edit(plugin_path("🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs"), lambda text: once(
        text, "\npub mod examples {\n",
        "\npub mod examples {\n    #[path = \".\"]\n    pub mod branch_chain {\n        #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔗️branch-chain/🦀️.rs\"]\n        mod component;\n        pub use component::*;\n    }\n",
        "trinity jack: mount the branch-chain example"))
    plan.edit(f"{jack}/🦀️.rs", lambda text: once(
        text, "    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()\n",
        "    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source(), crate::examples::branch_chain::source()]).as_slice()\n",
        "trinity jack: publish branch-chain"))
    handler_file = f"🔱️trinity/🗿️artifacts/🔌️jack/{A}/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs"

    def jack_handler(text):
        text = once(text, '        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",\n',
                    f'        crate::examples::branch_chain::ID => "{TRINITY_BRANCH_QUERY}",\n', "trinity jack: preset query of the published id")
        start = text.index("/// 🎬️ The ids this verb answers.")
        return text[:start].replace("use crate::JackSnapshot;\n", "") + (
            "/// 🎬️ Loads the example the ONE resolver answers for `example_id` over the examples this subset publishes — the\n"
            "/// navbar picker and the catalogue panel's fixture rows send exactly those ids — or the initial document for the empty\n"
            f"/// id; {RESOLVE_DOC_TAIL}\n"
            "pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {\n"
            f"    let next = {resolve('crate::editor::jack::TrinityJackPlayApp', 'example_id')}?;\n"
            "    Ok(Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() })\n"
            "}\n")

    plan.edit(plugin_path(handler_file), lambda text: jack_handler(text) if "fn fixture_dsl_for_preset" in text else (notes.append("applied trinity jack handler") or text))
    plan.edit(plugin_path(handler_file), lambda text: once(text, "use semio_framework_plugin::{Emit, NoConfigMutation};\nuse store::ArtifactDsl;\n", "use semio_framework_plugin::{Emit, Fault, NoConfigMutation};\n", "trinity jack: handler imports"))
    jack_editor = plugin_path(f"🔱️trinity/🗿️artifacts/🔌️jack/{A}/✏️editor/🦀️.rs")
    plan.edit(jack_editor, lambda text: once(text, 'pub(crate) const BRANCH_FIXTURE_DSL: &str = include_str!("../🖼️assets/🎬️demo/🗣️.dsl.semio");\n', "", "trinity jack: the branch fixture alias of the demo asset goes"))
    plan.edit(jack_editor, lambda text: (text.replace("TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),", "TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id)?,")
                                         if text.count("TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),") == 2
                                         else (problems.append("trinity jack: expected 2 set_active_example dispatch sites") if "commands::set_active_example(example_id)?," not in text else notes.append("applied trinity jack dispatch")) or text))
    catalogue = plugin_path(f"🔱️trinity/🗿️artifacts/🔌️jack/{A}/✏️editor/📌️panels/📚️catalogue/🦀️.rs")
    plan.edit(plugin_path(f"🔱️trinity/🗿️artifacts/🔌️jack/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(
        text,
        "/// example this subset registers — `fixture_dsl_for_preset` knew `nakagin`/`branch-chain` only, so\n",
        "/// example this subset registered — the verb's own id table knew `nakagin`/`branch-chain` only, so\n",
        "trinity jack: test doc names no removed fn"))
    plan.edit(catalogue, lambda text: once(
        text,
        'const FIXTURES: [FixturePreset; 2] = [FixturePreset { id: "nakagin", label: "Nakagin — Table" }, FixturePreset { id: "branch-chain", label: "Branch — Graph" }];',
        'const FIXTURES: [FixturePreset; 2] = [FixturePreset { id: crate::examples::demo::ID, label: "Nakagin — Table" }, FixturePreset { id: crate::examples::branch_chain::ID, label: "Branch — Graph" }];',
        "trinity jack: catalogue fixture rows send the published ids"))
    rewriting = f"🔱️trinity/🗿️artifacts/♻️rewriting/{A}"
    rw_handler = plugin_path(f"{rewriting}/✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs")

    def rewriting_handler(text):
        start = text.index("/// 🎬️ The ids this verb answers.")
        return text[:start].replace("use crate::RewritingSnapshot;\n", "").replace("use semio_framework_plugin::{Emit, NoConfigMutation};\nuse store::ArtifactDsl;\n", "use semio_framework_plugin::{Emit, Fault, NoConfigMutation};\n") + (
            "/// 🧬️ Whole-document swap, so it routes through `Effect::LoadDocument` exactly as `reset_rule` does — `SetState` is\n"
            "/// forbidden vocabulary in the mutation enum. The document is what the ONE resolver answers for `example_id` over the\n"
            "/// examples this subset publishes (the empty id is the blank rule, the editor's initial document); any other id or an\n"
            "/// unreadable asset is refused by code.\n"
            "pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {\n"
            f"    let next = {resolve('crate::editor::rewriting::TrinityRewritingPlayApp', 'example_id')}?;\n"
            "    Ok(Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], ..Default::default() })\n"
            "}\n")

    plan.edit(rw_handler, lambda text: rewriting_handler(text) if "fn set_active_example_document" in text else (notes.append("applied trinity rewriting handler") or text))
    plan.edit(plugin_path("🔱️trinity/🗿️artifacts/♻️rewriting/🦀️.rs"), lambda text: once(text, "pub(crate) use set_active_example_leaf::{set_active_example, set_active_example_document};", "pub(crate) use set_active_example_leaf::set_active_example;", "trinity rewriting: re-export"))
    rw_editor = plugin_path(f"{rewriting}/✏️editor/🦀️.rs")
    plan.edit(rw_editor, lambda text: once(text, "TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),", "TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example(example_id)?,", "trinity rewriting: document dispatch"))
    plan.edit(rw_editor, lambda text: once(text, "TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example(example_id),", "TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example(example_id)?,", "trinity rewriting: retained dispatch"))
    rw_tests = plugin_path(f"{rewriting}/✏️editor/🧪️tests/🔬️unit/🦀️.rs")
    plan.edit(rw_tests, lambda text: once(
        text,
        "    let loaded = crate::editor::rewriting::commands::set_active_example_document(crate::examples::demo::ID).expect(\"the demo example resolves to a document\");\n",
        f"    let loaded = {resolve('crate::editor::rewriting::TrinityRewritingPlayApp', 'crate::examples::demo::ID')}.expect(\"the demo example resolves to a document\");\n",
        "trinity rewriting: test resolves through the resolver"))
    plan.edit(rw_tests, lambda text: once(
        text,
        "async fn set_active_example_ignores_an_unregistered_id() {\n    let mut app = new_app().await;\n"
        "    app.dispatch_typed(TrinityRewritingCommand::SetActiveExample { example_id: \"not-an-example\".into() }, &meta(\"local\")).await.expect(\"set active example\");\n"
        "    let receipt = settle(&mut app).await;\n    assert!(receipt.effects.is_empty(), \"an unknown example id loads nothing\");\n}\n",
        "async fn set_active_example_refuses_an_unregistered_id() {\n"
        "    let fault = crate::editor::rewriting::commands::set_active_example(\"not-an-example\").err().expect(\"an unknown example id is refused\");\n"
        "    assert_eq!(fault.code.0, \"app.example.unknown\");\n}\n",
        "trinity rewriting: unknown id is refused by code"))


def puzzle_c(plan):
    """🧩️ puzzle — its three retained example works walk pre-decoded example documents (a whole decode per step would break
    the per-step budget), so they cannot resolve through `example_snapshot`: they answer exactly the ids their subsets publish
    and refuse every other id with the SDK's ONE `unknown_example` refusal in their first step. They accepted the aliases
    `concrete`/`nakagin`(/`capsule`), and an unknown id loaded the EMPTY document (2d), faulted untyped as "exceeds capacity"
    (3d) or completed as a "too large" notice (5d)."""
    base = "🧩️puzzle/🗿️artifacts"
    unknown_extent = ("        /// 📏️ An id no example publishes costs one unit: the first step refuses it by code.\n")
    # ◻️ 2d
    p2 = plugin_path(f"{base}/◻️2d/{A}/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs")
    plan.edit(p2, lambda text: once(
        text,
        "pub(crate) fn canonical_example_id(example_id: &str) -> &'static str {\n    match example_id {\n"
        "        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID | \"concrete\" => PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,\n"
        "        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID | \"nakagin\" => PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID,\n        _ => \"\",\n    }\n}\n\n"
        "pub(crate) fn target(example_id: &str) -> &'static Puzzle2dSnapshot {\n    match example_id {\n"
        "        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID => &CONCRETE_FOREST,\n        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID => &NAKAGIN,\n        _ => &EMPTY,\n    }\n}\n",
        "/// 🛍️ The warmed document `example_id` loads: the empty document for the empty id (the navbar's No example row), each\n"
        "/// published example its own; any other id is refused `app.example.unknown`.\n"
        "pub(crate) fn target(example_id: &str) -> Result<&'static Puzzle2dSnapshot, semio_framework_plugin::Fault> {\n    match example_id {\n"
        "        \"\" => Ok(&EMPTY),\n        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID => Ok(&CONCRETE_FOREST),\n        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID => Ok(&NAKAGIN),\n"
        "        other => Err(semio_framework_plugin::unknown_example(other)),\n    }\n}\n",
        "puzzle2d: target answers the published ids"))
    e2 = plugin_path(f"{base}/◻️2d/{A}/✏️editor/🦀️.rs")
    plan.edit(e2, lambda text: once(
        text,
        "    fn target(command: &Puzzle2dCommand) -> &'static crate::Puzzle2dSnapshot {\n"
        "        let id = set_active_example::canonical_example_id(command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\"));\n"
        "        set_active_example::target(id)\n    }\n",
        "    fn target(command: &Puzzle2dCommand) -> Result<&'static crate::Puzzle2dSnapshot, Fault> {\n"
        "        set_active_example::target(command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\"))\n    }\n",
        "puzzle2d: work target"))
    plan.edit(e2, lambda text: once(
        text,
        "    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {\n        let target = Self::target(command);\n",
        unknown_extent + "    fn extent(&self, command: &Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {\n        let Ok(target) = Self::target(command) else { return Some(1) };\n",
        "puzzle2d: extent"))
    plan.edit(e2, lambda text: once(
        text,
        ") -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {\n        let target = Self::target(command);\n        match self.stage {\n            Puzzle2dExampleStage::ClearEdges => {",
        ") -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {\n        let target = Self::target(command)?;\n        match self.stage {\n            Puzzle2dExampleStage::ClearEdges => {",
        "puzzle2d: step refuses an unknown id"))
    # 🧊 3d
    e3 = plugin_path(f"{base}/🧊️3d/{A}/✏️editor/🦀️.rs")
    plan.edit(e3, lambda text: once(
        text,
        "    fn target(command: &Puzzle3dCommand) -> Option<&'static Puzzle3dFixture> {\n"
        "        match command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\") {\n"
        "            \"\" => Some(&EMPTY_EXAMPLE_FIXTURE),\n"
        "            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | \"concrete\" => Some(&CONCRETE_FOREST_EXAMPLE_FIXTURE),\n"
        "            PUZZLE3D_EXAMPLE_NAKAGIN | \"nakagin\" => Some(&NAKAGIN_EXAMPLE_FIXTURE),\n            _ => None,\n        }\n    }\n\n"
        "    /// 🏷️ The CANONICAL example id this command loads, for the alias the picker may have sent\n"
        "    /// (`concrete`/`nakagin`). Persisted on the shared config so `export_fixture` can name its\n"
        "    /// download after the example instead of one constant filename.\n"
        "    fn canonical_example_id(command: &Puzzle3dCommand) -> Option<&'static str> {\n"
        "        match command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\") {\n"
        "            \"\" => Some(\"\"),\n"
        "            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | \"concrete\" => Some(PUZZLE3D_EXAMPLE_CONCRETE_FOREST),\n"
        "            PUZZLE3D_EXAMPLE_NAKAGIN | \"nakagin\" => Some(PUZZLE3D_EXAMPLE_NAKAGIN),\n            _ => None,\n        }\n    }\n",
        "    /// 🏷️ The example id this command names — persisted on the shared config so `export_fixture` names its download after\n"
        "    /// the example instead of one constant filename.\n"
        "    fn example_id(command: &Puzzle3dCommand) -> &str {\n"
        "        command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\")\n    }\n\n"
        "    /// 🧩️ The warmed fixture the command loads: the empty fixture for the empty id (the navbar's No example row), each\n"
        "    /// published example its own; any other id is refused `app.example.unknown`.\n"
        "    fn target(command: &Puzzle3dCommand) -> Result<&'static Puzzle3dFixture, Fault> {\n"
        "        match Self::example_id(command) {\n"
        "            \"\" => Ok(&EMPTY_EXAMPLE_FIXTURE),\n"
        "            PUZZLE3D_EXAMPLE_CONCRETE_FOREST => Ok(&CONCRETE_FOREST_EXAMPLE_FIXTURE),\n"
        "            PUZZLE3D_EXAMPLE_NAKAGIN => Ok(&NAKAGIN_EXAMPLE_FIXTURE),\n"
        "            other => Err(semio_framework_plugin::unknown_example(other)),\n        }\n    }\n",
        "puzzle3d: target answers the published ids"))
    plan.edit(e3, lambda text: once(
        text,
        "    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {\n        let target = Self::target(command)?;\n",
        unknown_extent + "    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {\n        let Ok(target) = Self::target(command) else { return Some(1) };\n",
        "puzzle3d: extent"))
    plan.edit(e3, lambda text: once(
        text,
        "        let Some(target) = Self::target(command) else { return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit::default())) };\n",
        "        let target = Self::target(command)?;\n",
        "puzzle3d: step refuses an unknown id"))
    plan.edit(e3, lambda text: once(
        text,
        "                let example_id = Self::canonical_example_id(command).unwrap_or_default();\n",
        "                let example_id = Self::example_id(command);\n",
        "puzzle3d: the persisted id is the published id"))
    t3 = plugin_path(f"{base}/🧊️3d/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs")
    plan.edit(t3, lambda text: once(
        text,
        "/// from no example at all keeps the app-generic `puzzle-3d.json`. Both picker aliases (`concrete`,\n"
        "/// `nakagin`) resolve to the canonical id, so the filename never depends on how the row was spelled.\n",
        "/// from no example at all keeps the app-generic `puzzle-3d.json`.\n",
        "puzzle3d: export test doc"))
    plan.edit(t3, lambda text: once(
        text,
        "    dispatch(&mut app, \"setActiveExample\", Some(&json!({ \"exampleId\": \"nakagin\" })), None).await.expect(\"load nakagin by alias\");\n"
        "    assert_eq!(exported_filename(&mut app).await, \"nakagin-capsule-tower.json\", \"the picker alias must still export the canonical example id\");\n",
        "    dispatch(&mut app, \"setActiveExample\", Some(&json!({ \"exampleId\": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect(\"load nakagin\");\n"
        "    assert_eq!(exported_filename(&mut app).await, \"nakagin-capsule-tower.json\", \"Nakagin must export under its own id\");\n",
        "puzzle3d: export test loads the published nakagin id"))
    plan.edit(t3, lambda text: once(
        text,
        "    dispatch(&mut app, \"setActiveExample\", Some(&json!({ \"exampleId\": \"nakagin\" })), None).await.expect(\"load nakagin\");\n",
        "    dispatch(&mut app, \"setActiveExample\", Some(&json!({ \"exampleId\": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect(\"load nakagin\");\n",
        "puzzle3d: segmented export test loads the published nakagin id"))
    # 🖐 5d
    e5 = plugin_path(f"{base}/🖐️5d/{A}/✏️editor/🦀️.rs")
    plan.edit(e5, lambda text: once(
        text,
        "    fn target(command: &Puzzle5dCommand) -> Option<&'static Puzzle5dDocument> {\n"
        "        let example_id = command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\");\n"
        "        match example_id {\n"
        "            \"\" => Some(&EMPTY_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_CONCRETE_FOREST | \"concrete\" => Some(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_NAKAGIN | \"nakagin\" => Some(&NAKAGIN_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_CAPSULE_DREAM | \"capsule\" => Some(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),\n            _ => None,\n        }\n    }\n",
        "    /// 🧩️ The warmed document the command loads: the empty document for the empty id (the navbar's No example row), each\n"
        "    /// published example its own; any other id is refused `app.example.unknown`.\n"
        "    fn target(command: &Puzzle5dCommand) -> Result<&'static Puzzle5dDocument, Fault> {\n"
        "        match command.args().and_then(|args| args.get(\"exampleId\")).and_then(Value::as_str).unwrap_or(\"\") {\n"
        "            \"\" => Ok(&EMPTY_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_CONCRETE_FOREST => Ok(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_NAKAGIN => Ok(&NAKAGIN_EXAMPLE_DOCUMENT),\n"
        "            PUZZLE5D_EXAMPLE_CAPSULE_DREAM => Ok(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),\n"
        "            other => Err(semio_framework_plugin::unknown_example(other)),\n        }\n    }\n",
        "puzzle5d: target answers the published ids"))
    plan.edit(e5, lambda text: once(text, "        let target = Self::target(command)?;\n        let rows = [\n", "        let target = Self::target(command).ok()?;\n        let rows = [\n", "puzzle5d: units of a published example"))
    plan.edit(e5, lambda text: once(
        text,
        "            if Self::target(command).is_none() || Self::units(command, snapshot).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {\n",
        "            Self::target(command)?;\n"
        "            if Self::units(command, snapshot).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {\n",
        "puzzle5d: admission refuses an unknown id by code"))
    plan.edit(e5, lambda text: once(
        text,
        "        let Some(target) = Self::target(command) else {\n            self.stage = Puzzle5dSetActiveExampleStage::Complete;\n"
        "            return Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(puzzle5d_notice_emit(self.view_state.as_ref(), |labels| labels.example_too_large.as_str())));\n        };\n",
        "        let target = Self::target(command)?;\n",
        "puzzle5d: step target"))


def process3d_c(plan):
    """🏭️ process3d — its form offered `timber-beam-joinery` (default), `drilled-plate` and `concrete-forest` while it published
    `demo` (the timber joinery) and `concrete-forest`; ANY other id loaded the timber document. The drilled plate becomes a
    published example (its inline fixture text moves to its own asset), the demo carries its real name, the form offers
    exactly the published examples, and the example constants name the published ids."""
    p3 = plugin_path(f"🏭️process/🗿️artifacts/🧊️process3d/{A}")
    text_rs = f"{p3}/🧬️schema/📸️snapshot/📝️text/🦀️.rs"
    head = "pub const PROCESS_3D_PLATE_EXAMPLE_TEXT: &str = r#\""
    source = read(text_rs) if os.path.isfile(os.path.join(TREE, text_rs)) else ""
    if head in source:
        start = source.index(head) + len(head)
        plan.create(f"{p3}/🖼️assets/🔩️drilled-plate/🗣️.dsl.semio", source[start: source.index('"#;', start)])
        plan.edit(text_rs, lambda text: text[: text.index(head)] + 'pub const PROCESS_3D_PLATE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🔩️drilled-plate/🗣️.dsl.semio");' + text[text.index('"#;', text.index(head)) + 3:])
    else:
        notes.append("applied process3d: plate fixture asset")
    example_leaf(plan, p3, "🔩️drilled-plate", "drilled-plate", "Drilled Plate", "Gebohrte Platte", "🖼️assets/🔩️drilled-plate/🗣️.dsl.semio",
                 " — a plate drilled twice, the process timeline paused mid-way (`resolved-up-to=2`).")
    relabel_leaf(plan, p3, "🎬️demo", "Timber Beam Joinery", "Holzbalkenverbindung", "process3d")
    root = plugin_path("🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs")
    anchor = example_mount("concrete_forest", "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️.rs")
    plan.edit(root, lambda text: once(text, anchor, example_mount("drilled_plate", "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️drilled-plate/🦀️.rs") + anchor, "process3d: mount the drilled-plate example"))
    editor = f"{p3}/✏️editor/🦀️.rs"
    plan.edit(editor, lambda text: once(text, "        vec![crate::examples::demo::source(), crate::examples::concrete_forest::source()]\n", "        vec![crate::examples::demo::source(), crate::examples::drilled_plate::source(), crate::examples::concrete_forest::source()]\n", "process3d: publish the drilled plate"))
    plan.edit(editor, lambda text: once(
        text,
        'pub const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";\npub const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";\n/// 🌲️ Shares its id with `crate::examples::concrete_forest::ID`, the registered example the react\n/// shell\'s picker dispatches back through `setActiveExample`.\npub const PROCESS3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";\n',
        "/// 🪵️ The published timber-beam-joinery example (the subset's demo).\npub const PROCESS3D_EXAMPLE_TIMBER: &str = crate::examples::demo::ID;\n"
        "/// 🔩️ The published drilled-plate example.\npub const PROCESS3D_EXAMPLE_PLATE: &str = crate::examples::drilled_plate::ID;\n"
        "/// 🌲️ The published concrete-forest example.\npub const PROCESS3D_EXAMPLE_CONCRETE_FOREST: &str = crate::examples::concrete_forest::ID;\n",
        "process3d: example constants name the published ids"))
    plan.edit(editor, lambda text: once(
        text,
        '                    ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, LocalizedLabel::native("Timber Beam Joinery", "Holzbalkenverbindung")),\n'
        '                    ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, LocalizedLabel::native("Drilled Plate", "Gebohrte Platte")),\n'
        '                    ActionArgOption::new(PROCESS3D_EXAMPLE_CONCRETE_FOREST, LocalizedLabel::native("Concrete Forest", "Betonwald")),\n',
        "                    ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, crate::examples::demo::label()),\n"
        "                    ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, crate::examples::drilled_plate::label()),\n"
        "                    ActionArgOption::new(PROCESS3D_EXAMPLE_CONCRETE_FOREST, crate::examples::concrete_forest::label()),\n",
        "process3d: form options are the published examples"))


def forms_c(plan):
    """📋️ forms — its form offered `default` (Contact), `onboarding` and `building-component` beside the blank form while it
    published only `demo` (the building component); the handler kept its own id table and codes. The contact and onboarding
    templates become published examples beside their existing assets, the demo carries its real name, and the form offers
    the blank form plus exactly the published examples (default: the contact form, as before)."""
    subset = plugin_path(f"📋️forms/🗿️artifacts/📋️forms/{A}")
    example_leaf(plan, subset, "📇️contact", "contact", "Contact", "Kontakt", "🖼️assets/📇️contact/🗣️.dsl.semio", " — a one-step contact form.")
    example_leaf(plan, subset, "🌱️onboarding", "onboarding", "Onboarding", "Einführung", "🖼️assets/🌱️onboarding/🗣️.dsl.semio", " — a multi-step product onboarding form.")
    relabel_leaf(plan, subset, "🎬️demo", "Building Component", "Baukomponente", "forms")
    root = plugin_path("📋️forms/🗿️artifacts/📋️forms/🦀️.rs")
    anchor = example_mount("demo", "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs")
    plan.edit(root, lambda text: once(text, "pub mod examples {\n" + anchor, "pub mod examples {\n" + anchor
                                      + example_mount("contact", "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📇️contact/🦀️.rs")
                                      + example_mount("onboarding", "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌱️onboarding/🦀️.rs"), "forms: mount the contact + onboarding examples"))
    plan.edit(f"{subset}/✏️editor/🦀️.rs", lambda text: once(
        text,
        '                    ActionArgOption::new("default", LocalizedLabel::native("Contact", "Kontakt")),\n'
        '                    ActionArgOption::new("onboarding", LocalizedLabel::native("Onboarding", "Einführung")),\n'
        '                    ActionArgOption::new("building-component", LocalizedLabel::native("Building Component", "Baukomponente")),\n'
        '                ]).default_value(&"default"),\n',
        "                    ActionArgOption::new(crate::examples::contact::ID, crate::examples::contact::label()),\n"
        "                    ActionArgOption::new(crate::examples::onboarding::ID, crate::examples::onboarding::label()),\n"
        "                    ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label()),\n"
        "                ]).default_value(&crate::examples::contact::ID),\n",
        "forms: form options are the published examples"))


# ─── plugins, batch C (process3d, energy, shooting, forms, sourcing, animate + every boot that fell back) ─────────────

LAW_PINNED = "— pinned by the example-catalog law"


def plugins_c(plan):
    subset_examples_public(plan)
    process3d_c(plan)
    cad_c(plan)
    trinity_c(plan)
    puzzle_c(plan)
    batch_c_tests(plan)
    # 🏭️ process3d — ANY unknown id loaded the timber default; the three fixtures fell back to each other / Default.
    p3 = f"🏭️process/🗿️artifacts/🧊️process3d/{A}"
    header = "    pub fn handle(\n        payload: &SetActiveExample,\n        _doc: &ArtifactView<'_, Process3dSnapshot>,"
    handler(plan, f"{p3}/✏️editor/🎮️commands/🗿️artifact/🦀️.rs", header,
            "    /// 📄️ Loads the example the ONE resolver answers for `payload.example_id` — a published example decoded from its own\n"
            "    /// asset, or the empty document for the empty id (the navbar's No example row) — as one whole-document\n"
            "    /// `Effect::LoadDocument`; any other id or an unreadable asset is refused by code.\n"
            "    pub fn handle(\n        payload: &SetActiveExample,\n        _doc: &ArtifactView<'_, Process3dSnapshot>,\n        _cfg: &ConfigView<'_, Process3dConfig>,\n        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,\n    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {\n"
            f"        let snapshot = if payload.example_id.is_empty() {{ Process3dSnapshot::default() }} else {{ {load('crate::editor::process3d::Process3dPlayApp')} }};\n"
            "        Ok(Emit { effects: vec![crate::editor::process3d::reset_process3d_document_effect(&snapshot)], ..Default::default() })\n"
            "    }\n",
            "process3d: handle", uses=("plate_document", "concrete_forest_document", "default_document"))
    schema = f"{p3}/🧬️schema/🦀️.rs"
    for fixture, name in (("TIMBER_EXAMPLE_DSL).unwrap_or_default()", "timber"), ("PLATE_EXAMPLE_DSL).unwrap_or_else(|_| default_document())", "plate"), ("CONCRETE_FOREST_EXAMPLE_DSL).unwrap_or_else(|_| default_document())", "concrete forest")):
        plan.edit(plugin_path(schema), lambda text, fixture=fixture, name=name: once(text, fixture, fixture.split(".unwrap_or")[0] + f".expect(\"the process3d {name} fixture decodes {LAW_PINNED}\")", f"process3d: {name} fixture"))
    # 🔋️ energy — the example switch loads the published example through the resolver, as the snapshot it IS.
    energy = f"🔋️energy/🗿️artifacts/🔋️model/{A}/✏️editor/🦀️.rs"
    plan.edit(plugin_path(energy), lambda text: once(
        text,
        "            let loaded = example_model(example_id).ok_or_else(|| app_fault(\"energy.model.example.unknown\").with_parameter(\"example\", example_id))?;\n"
        "            return Ok(Emit { effects: vec![load_document_effect(&loaded)], description: Some(format!(\"Load example {example_id}\")), ..Default::default() });\n",
        f"            let loaded = {load('EnergyModelEditor', 'example_id')};\n"
        "            return Ok(Emit { effects: vec![load_snapshot_effect(loaded)], description: Some(format!(\"Load example {example_id}\")), ..Default::default() });\n",
        "energy: the example switch loads through the resolver",
    ))
    plan.edit(plugin_path(energy), lambda text: once(
        text,
        "fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {\n    let snapshot = crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None);\n    let pack",
        "fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {\n    load_snapshot_effect(crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None))\n}\n\n"
        "/// 📦️ The whole-document `Effect::LoadDocument` of one energy model snapshot: its pack and an edit-free op log.\n"
        "fn load_snapshot_effect(snapshot: EnergyModelSnapshot) -> semio_framework_plugin::kernel::Effect {\n    let pack",
        "energy: load effect of a snapshot",
    ))
    handler(plan, energy, "fn example_model(example_id: &str) -> Option<crate::model::Model> {", "", "energy: example_model")
    plan.edit(plugin_path(energy), lambda text: (lambda line: text[: line.start()] + text[line.end():] if line else (notes.append("applied energy: .fault energy.model.example.unknown") or text))(re.search(r"\n[ \t]*builder = builder\.fault\(\"energy\.model\.example\.unknown\", [^\n]*", text)))
    plan.edit(plugin_path(f"🔋️energy/🗿️artifacts/🔋️model/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(text, '"energy.model.example.unknown"', '"app.example.unknown"', "energy: unknown-example law code"))
    plan.edit(plugin_path(f"🔋️energy/🗿️artifacts/🔋️model/{A}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"), lambda text: once(
        text,
        '        assert!(example_model(source.id()).is_some(), "example {} has no model behind its picker row", source.id());\n',
        '        assert!(semio_framework_plugin::editor_example_snapshot::<EnergyModelEditor>(source.id()).is_ok(), "example {} resolves to no document behind its picker row", source.id());\n',
        "energy: picker rows resolve through the resolver"))
    # 🎥️ shooting — an unknown id was a silent no-op, a stale forest fixture another.
    shooting = f"🎥️shooting/🗿️artifacts/🎥️shooting/{A}"
    header = "    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {"
    handler(plan, f"{shooting}/✏️editor/🎮️commands/📄️document/🦀️.rs", header,
            "    /// 🎬️ Loads the example the ONE resolver answers for `payload.example_id` — a published example decoded from its own\n"
            "    /// asset, or the empty document for the empty id (the navbar's No example row); any other id or an unreadable asset is\n"
            "    /// refused by code.\n"
            f"{header}\n"
            f"        let next = if payload.example_id.is_empty() {{ crate::empty_shooting_snapshot() }} else {{ {load('crate::editor::shooting::ShootingPlayApp')} }};\n"
            "        Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&next)], ..Default::default() })\n"
            "    }\n",
            "shooting: handle")
    plan.edit(plugin_path(f"{shooting}/🧬️schema/🦀️.rs"), lambda text: once(
        text,
        "SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| crate::empty_shooting_snapshot())",
        f"SHOOTING_EXAMPLE_TEXT).expect(\"the shooting example decodes {LAW_PINNED}\")",
        "shooting: default snapshot"))
    # 📋️ forms — three bundled templates, one published; an unknown id refused under a private code.
    forms = f"📋️forms/🗿️artifacts/📋️forms/{A}"
    forms_c(plan)
    header = "pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {"
    handler(plan, f"{forms}/✏️editor/🎮️commands/📥️set-active-example/🦀️.rs", header,
            "/// 📥️ Replaces the form design with the published example the ONE resolver decodes from its template text, or the empty\n"
            "/// design for the empty id (the navbar's No example row); any other id or an unreadable template is refused by code.\n"
            f"{header}\n"
            f"    let next = if payload.example_id.is_empty() {{ empty_forms_snapshot() }} else {{ {load('crate::editor::forms::FormsPlayApp')} }};\n"
            "    Ok(Emit { artifact_mutations: replace_design_operations(doc.snapshot, &next), ..Default::default() })\n"
            "}\n",
            "forms: handle", uses=("app_fault",), after=lambda text: once(text, "use crate::document_dsl as forms_dsl;\n", "", "forms: handle: use forms_dsl") if text.count("forms_dsl") == 1 else text)
    faults(plan, f"{forms}/✏️editor/🦀️.rs", ["forms.template.invalid", "forms.template.unknown"], "forms")
    # 🪵️ sourcing — the ONE resolver instead of its own lookup + codes.
    sourcing = f"🪵️sourcing/🗿️artifacts/🗂️curation/{A}"
    header = "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {"
    handler(plan, f"{sourcing}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs", header,
            "/// 🎬️ Loads the published example the ONE resolver decodes from its committed stock text, or the empty curation for the\n"
            "/// empty id (the navbar's No example row); any other id or an unreadable text is refused by code.\n"
            f"{header}\n"
            f"    let next = if payload.example_id.is_empty() {{ <CurationSnapshot as store::ArtifactDsl>::parse_dsl(crate::document_dsl::EMPTY_CURATION_TEXT).expect(\"the empty curation decodes\") }} else {{ {load('crate::editor::sourcing::SourcingCurationApp')} }};\n"
            "    Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() })\n"
            "}\n",
            "sourcing: handle", uses=("app_fault", "EMPTY_EXAMPLE_ID"))
    faults(plan, f"{sourcing}/✏️editor/🦀️.rs", ["sourcing.example.unknown", "sourcing.example.unparsable"], "sourcing")
    # 🎞️ animate — "reset to demo" for the demo id and the empty id, a silent no-op for every other id.
    animate = f"🎞️animate/🗿️artifacts/🎬️presentation/{A}/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"
    header = "pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {"
    handler(plan, animate, header,
            "/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned outright — see\n"
            "/// `📓️taxonomy.md`'s forbidden vocabulary), so loading an example builds `editor::animate::reset_presentation_document_effect`\n"
            "/// (a `Effect::LoadDocument`, outside undo history). The document is what the ONE example resolver decodes from the\n"
            f"/// published example's own asset (the empty id is the editor's initial document); {RESOLVE_DOC_TAIL}\n"
            f"{header}\n"
            f"    let next = {load('crate::editor::animate::AnimatePresentationPlayApp')};\n"
            "    Ok(Emit { effects: vec![crate::editor::animate::reset_presentation_document_effect(&next), interaction_select_effect(&[], \"replace\")], ..Default::default() })\n"
            "}\n",
            "animate: handle", uses=("demo_presentation_snapshot",))
    # 📌️ boots that fell back to an empty/default document when their bundled example stopped decoding
    boots = [
        (f"🌀️procedural/🗿️artifacts/🧊️generation3d/{A}/🧬️schema/🦀️.rs", "Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()", "the generation3d hex-column example decodes"),
        (f"🌀️procedural/🗿️artifacts/🧊️generation3d/{A}/🧬️schema/🦀️.rs", "    let snapshot = example_snapshot(example_id).unwrap_or_default();", "every published generation3d example decodes"),
        (f"🌀️procedural/🗿️artifacts/🌀️generation2d/{A}/🧬️schema/🦀️.rs", "GENERATION2D_EXAMPLE_TEXT).unwrap_or_default()", "the generation2d example decodes"),
        (f"🌍️gis/🗿️artifacts/🗺️gismap/{A}/🧬️schema/🦀️.rs", "REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_snapshot())", "the reuse map example decodes"),
        (f"🌍️gis/🗿️artifacts/🏔️gisterrain/{A}/🧬️schema/🦀️.rs", "REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_terrain_snapshot())", "the reuse terrain example decodes"),
        (f"🗒️note/🗿️artifacts/🗒️note/{A}/🧬️schema/🦀️.rs", "SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_snapshot())", "the semio note example decodes"),
        (f"🖍️draw/🗿️artifacts/🖍️drawing/{A}/🧬️schema/🦀️.rs", "SEMIO_DRAW_EXAMPLE_TEXT).unwrap_or_else(|_| empty_drawing_snapshot())", "the semio drawing example decodes"),
        (f"🔱️trinity/🗿️artifacts/🔌️jack/{A}/✏️editor/🦀️.rs", "JackSnapshot::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap_or_else(|_| crate::empty_trinity_graph_fixture())", "the nakagin fixture decodes"),
        ("🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "from_json_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture())", "the concrete forest example decodes"),
        ("🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "from_json_str(NAKAGIN_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture())", "the nakagin example decodes"),
        ("🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🦀️.rs", "<TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_txt_snapshot())", "the txt demo example decodes"),
        ("🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs", "<CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_csv_snapshot())", "the csv demo example decodes"),
    ]
    for rel, old, message in boots:
        plan.edit(plugin_path(rel), lambda text, old=old, message=message: once(text, old, old.split(".unwrap_or")[0] + f".expect(\"{message} {LAW_PINNED}\")" + (";" if old.endswith(";") else ""), f"boot: {message}"))
    fem3d = f"🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"
    handler(plan, fem3d, "pub fn fem3d_boot_snapshot() -> Fem3dSnapshot {",
            "/// 🚀️ The document every `fem3d` surface boots with — the bundled `concrete-forest` (Betonwald) example so\n"
            "/// Entwerfen-mit-Bestand surfaces and the demonstrator Statik pane paint the reuse story on first frame. Shared by\n"
            "/// `Fem3dPlayApp::initial_snapshot` and `Fem3dViewer::initial_snapshot` (the viewer must never import through the sibling\n"
            "/// editor module, so the shared boot document lives here). The example-catalog law holds the example to decode.\n"
            "pub fn fem3d_boot_snapshot() -> Fem3dSnapshot {\n"
            f"    parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect(\"the concrete-forest example decodes {LAW_PINNED}\")\n"
            "}\n",
            "fem3d: boot")
    for rel, old, new in (
        (f"🌀️procedural/🗿️artifacts/🌀️generation2d/{A}/🧬️schema/🦀️.rs", "/// 📄️ The `procedural2d-play` \"default\" document — parsed from the bundled `.generation2d` example\n/// host_snapshot, falling back to the empty document if the fixture ever fails to parse.\n", "/// 📄️ The `procedural2d-play` \"default\" document — parsed from the bundled `.generation2d` example host_snapshot, which\n/// the example-catalog law holds to decode.\n"),
        (f"🗒️note/🗿️artifacts/🗒️note/{A}/🧬️schema/🦀️.rs", "/// every \"semio\" example call site (`setActiveExample`, tests). Falls back to the empty document if the\n/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.\n", "/// every \"semio\" example call site (`setActiveExample`, tests). The example-catalog law holds the fixture to decode.\n"),
    ):
        plan.edit(plugin_path(rel), lambda text, old=old, new=new: once(text, old, new, f"boot doc {rel.split('/')[0]}"))


STDIO_NAMES, STDIO_NAMES_LANDED = {}, {}
SECTIONS = {"sdk": sdk, "stdio": stdio, "laws": laws, "plugins-a": plugins_a, "plugins-b": plugins_b, "examples-stdio": examples_stdio, "plugins-c": plugins_c}


# ─── run ──────────────────────────────────────────────────────────────────────────────────────────────────────────────

def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in ARGS), "--dry-run")
    if mode == "--revert":
        created = os.path.join(BACKUP, "created.txt")
        for root, _, files in os.walk(BACKUP):
            for file in files:
                source = os.path.join(root, file)
                path = os.path.relpath(source, BACKUP)
                if path == "created.txt":
                    continue
                os.makedirs(os.path.dirname(os.path.join(TREE, path)), exist_ok=True)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        if os.path.isfile(created):
            for path in open(created, encoding="utf-8").read().split("\n"):
                if path and os.path.isfile(os.path.join(TREE, path)):
                    os.remove(os.path.join(TREE, path))
                    print("removed", path)
                    parent = os.path.dirname(os.path.join(TREE, path))
                    while parent != TREE and os.path.isdir(parent) and not os.listdir(parent):
                        os.rmdir(parent)
                        parent = os.path.dirname(parent)
        return
    plan = Plan()
    for name, section in SECTIONS.items():
        if ONLY is None or name in ONLY:
            section(plan)
    staged = {}
    for path, fns in plan.edits.items():
        if not os.path.isfile(os.path.join(TREE, path)):
            problems.append(f"{path}: missing")
            continue
        before = read(path)
        after = before
        for fn in fns:
            after = fn(after)
        if after != before:
            staged[path] = (before, after)
    creates = {}
    for path, text in plan.creates.items():
        full = os.path.join(TREE, path)
        if os.path.isfile(full):
            current = open(full, "rb").read() if isinstance(text, bytes) else open(full, encoding="utf-8").read()
            if current == text:
                notes.append(f"applied create {path}")
            else:
                problems.append(f"{path}: exists with other content")
            continue
        creates[path] = text
    copies = {}
    for target, (source, overwrite) in plan.copies.items():
        full, origin = os.path.join(TREE, target), os.path.join(TREE, source)
        if not overwrite and os.path.isfile(full):
            notes.append(f"applied copy {target}")
            continue
        if not os.path.isfile(origin):
            if os.path.isfile(full):
                notes.append(f"applied copy {target}")
            else:
                problems.append(f"{source}: missing (copy source)")
            continue
        payload = open(origin, "rb").read()
        if os.path.isfile(full) and open(full, "rb").read() == payload:
            notes.append(f"applied copy {target}")
            continue
        copies[target] = payload
    deletes = []
    for path in plan.deletes:
        if os.path.isfile(os.path.join(TREE, path)):
            deletes.append(path)
        else:
            notes.append(f"applied delete {path}")
    if "--list-crates" in ARGS:
        crates = set()
        for path in list(staged) + list(creates) + list(copies):
            directory = os.path.dirname(os.path.join(TREE, path))
            while directory.startswith(TREE) and directory != TREE:
                cargo = os.path.join(directory, "📦️packages", "🦀️rust", "Cargo.toml")
                if os.path.isfile(cargo):
                    name = re.search(r'^name = "([^"]+)"', open(cargo, encoding="utf-8").read(), re.M)
                    crates.add(name.group(1))
                    break
                directory = os.path.dirname(directory)
        print(" ".join(sorted(crates)))
        return
    for note in notes:
        print("  ", note)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files to edit, {len(creates)} to create, {len(copies)} to copy, {len(deletes)} to delete, {len(notes)} already applied, {len(problems)} problems  (root {TREE})")
    if mode == "--write" and not problems:
        os.makedirs(BACKUP, exist_ok=True)
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                with open(backup, "w", encoding="utf-8") as handle:
                    handle.write(before)
            with open(os.path.join(TREE, path), "w", encoding="utf-8") as handle:
                handle.write(after)
        with open(os.path.join(BACKUP, "created.txt"), "a", encoding="utf-8") as handle:
            for target, payload in copies.items():
                full = os.path.join(TREE, target)
                if os.path.isfile(full):
                    backup = os.path.join(BACKUP, target)
                    os.makedirs(os.path.dirname(backup), exist_ok=True)
                    if not os.path.exists(backup):
                        shutil.copyfile(full, backup)
                else:
                    handle.write(target + "\n")
                os.makedirs(os.path.dirname(full), exist_ok=True)
                with open(full, "wb") as out:
                    out.write(payload)
        for path in deletes:
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                shutil.copyfile(os.path.join(TREE, path), backup)
            os.remove(os.path.join(TREE, path))
            parent = os.path.dirname(os.path.join(TREE, path))
            while parent != TREE and not os.listdir(parent):
                os.rmdir(parent)
                parent = os.path.dirname(parent)
        with open(os.path.join(BACKUP, "created.txt"), "a", encoding="utf-8") as handle:
            for path, text in creates.items():
                full = os.path.join(TREE, path)
                os.makedirs(os.path.dirname(full), exist_ok=True)
                with (open(full, "wb") if isinstance(text, bytes) else open(full, "w", encoding="utf-8")) as out:
                    out.write(text)
                handle.write(path + "\n")
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
