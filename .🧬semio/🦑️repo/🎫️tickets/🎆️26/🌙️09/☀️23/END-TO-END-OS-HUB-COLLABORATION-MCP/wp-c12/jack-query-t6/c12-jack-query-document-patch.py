"""📝️ C12 T6 set — trinity jack's query becomes DOCUMENT content (coordinator decision (a), 2026-09-29 10:1x).

Root cause (C12 14c row 21): jack's `text_edit` published the query as a coalesced WINDOW-CONFIG mutation, and the framework's
`undo` only ever reaches the document store — so Ctrl+Z did nothing for typing in the query editor, and the query was neither
collaborative nor event-sourced. Now:
  * `JackSnapshot`/`JackArtifact` carry `query` (artifact state, required; every twin: Rust, TS, JSON Schema, GraphQL, proto,
    the `JackPackRecord` text/pack codec, the embedded-JSON import/export, the executor `Graph`); `JackDiff` carries
    `query: Option<String>`; the default query lives at the artifact root (`crate::TRINITY_JACK_DEFAULT_QUERY`);
  * one new document mutation leaf `🔎️set-query` / `TrinityGraphMutation::SetQuery { value }` (binary tag 8, text opcode
    `set-query`, diff = the new query, inverse = BASE's query, en/de label) in every language surface of the aggregate;
  * `textEdit` publishes `SetQuery` on the ARTIFACT lane under the typing coalesce key (one undo step per typing run),
    `formatDocument` publishes one `SetQuery`, `runQuery` reads the document's query, `loadExampleQuery` puts its query into
    the document with the graph writes of the run, `setActiveExample` loads the example with its preset query; both verbs move
    from the window-config retained route to the document route;
  * the editor window's config lane (`📝️editor/🎚️config`, `JackEditorWindowConfig`, its `SetQuery`, oracles, fixtures and the
    `mutate-trinity-jack-1-any-editor-edit-editor-config` scenario) is REMOVED — window config stays for per-window view state.
Idempotent; usage: python3 c12-jack-query-document-patch.py [--apply]   (default dry run; C12_REPO overrides the tree)"""
import json
import os
import shutil
import sys

REPO = os.environ.get("C12_REPO", "/Users/ueli/Documents/semio")
APPLY = "--apply" in sys.argv
JACK = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/"
ANY = JACK + "🏅️standards/🔖️1/🪆️subsets/✳️any/"
SCHEMA = ANY + "🧬️schema/"
MUT = SCHEMA + "🧬️mutations/"
EDITOR = ANY + "✏️editor/"
ROOT = JACK + "🦀️.rs"
DEFAULT_QUERY = "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label"

plan, problems, files = [], [], {}


def read(rel):
    if rel not in files:
        path = os.path.join(REPO, rel)
        files[rel] = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    return files[rel]


def edit(rel, old, new, label):
    text = read(rel)
    if text is None:
        problems.append((label, "missing file"))
    elif (new in text) if old in new else (old not in text):
        plan.append(f"present {label}")
    elif text.count(old) != 1:
        problems.append((label, text.count(old)))
    else:
        files[rel] = text.replace(old, new)
        plan.append(f"edit    {label}")


def create(rel, content, label):
    text = read(rel)
    if text == content:
        plan.append(f"present {label}")
    elif text is not None:
        problems.append((label, "exists with other content"))
    else:
        files[rel] = content
        plan.append(f"create  {label}")


removals = []


def remove(rel, label):
    if os.path.exists(os.path.join(REPO, rel)):
        removals.append(rel)
        plan.append(f"remove  {label}")
    else:
        plan.append(f"absent  {label}")


def json_fixture(rel, update, label):
    text = read(rel)
    if text is None:
        problems.append((label, "missing file"))
        return
    value = json.loads(text)
    indent = next((n for n in (2, 4) if json.dumps(value, indent=n, ensure_ascii=False) + "\n" == text), None)
    if indent is None:
        problems.append((label, "format"))
        return
    updated = update(json.loads(text))
    if updated == value:
        plan.append(f"present {label}")
        return
    files[rel] = json.dumps(updated, indent=indent, ensure_ascii=False) + "\n"
    plan.append(f"fixture {label}")


def replace_every(rel, old, new, label):
    text = read(rel)
    if text is None:
        problems.append((label, "missing file"))
    elif old not in text:
        plan.append(f"present {label}")
    else:
        files[rel] = text.replace(old, new)
        plan.append(f"edit    {label} ×{text.count(old)}")


def with_key(key, value, before=None):
    def update(record):
        if key in record:
            return record
        items = list(record.items())
        index = next((n for n, (name, _) in enumerate(items) if name == before), len(items))
        items.insert(index, (key, value))
        return dict(items)
    return update


# ── A. `query` joins the artifact, the snapshot and the diff ────────────────────────────────────────────────────────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "a_schema.py"), encoding="utf-8").read(), "a_schema.py", "exec"))
# ── B. the `set-query` document mutation ────────────────────────────────────────────────────────────────────────────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "b_mutation.py"), encoding="utf-8").read(), "b_mutation.py", "exec"))
# ── C. editor: typing, formatting, running and examples publish on the document lane; the window-config lane goes ────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "c_editor.py"), encoding="utf-8").read(), "c_editor.py", "exec"))
# ── E. exhaustive matches and bounded owners of the new variant ─────────────────────────────────────────────────────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "e_compile.py"), encoding="utf-8").read(), "e_compile.py", "exec"))
# ── D. fixtures and laws ────────────────────────────────────────────────────────────────────────────────────────────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "d_laws.py"), encoding="utf-8").read(), "d_laws.py", "exec"))
# ── F. the declared catalog and the Rust/Python differential ────────────────────────────────────────────────────────────
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", "f_differential.py"), encoding="utf-8").read(), "f_differential.py", "exec"))

print("\n".join(plan))
print(f"{len(problems)} problems {problems}", "mode=apply" if APPLY else "mode=dry-run")
if problems:
    sys.exit(1)
if APPLY:
    for rel, text in files.items():
        if text is None:
            continue
        path = os.path.join(REPO, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        if not os.path.exists(path) or open(path, encoding="utf-8").read() != text:
            open(path, "w", encoding="utf-8").write(text)
    for rel in removals:
        path = os.path.join(REPO, rel)
        shutil.rmtree(path) if os.path.isdir(path) else os.remove(path)
