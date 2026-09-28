#!/usr/bin/env python3
"""📊️ LB2 prepared patch p7 — the stdio structural table (csv/tsv) honours the process-wide `UiValue` arena.

Measured (S18 served matrix, 2026-09-28 17:2x, stdio csv/tsv): `set-cell` applied, but the undo render was refused
`stdio.table.cell-arguments: cell argument map capacity` (and the redo `ui.snapshot-details.arguments …` — p1). The table's
argument builders reported an arena refusal under their own codes, so the SDK row window (`tree_window_indexed_rows`),
which ends early on the framework's capacity refusal `ui.fixed-capacity`, could not catch it; and the structure controls
(add row / add column) were built AFTER the windowed body, so a body that drained the arena took them down too.

Hunks: contract `table_arena(stage)` = `ui.fixed-capacity` for every arena refusal of `window_kit_revisioned_cell_arguments`,
`window_kit_revision_arguments`, `window_kit_indexed_revision_arguments` (the UI-text bound of a revision keeps its own
code); `render_structural_table` takes the body as a closure and admits the structure controls first (children order
unchanged: controls, headers, body); csv + tsv pass their table as that closure. Law `🧪️tests/📊️table-arena-headroom`
(own test binary = own arena) + fixture/schema `🧫️fixtures/📊️table-arena-headroom` (python-jsonschema 4.25.1 valid): every
argument builder under an exhausted arena answers `ui.fixed-capacity`; under controls-only / a third / half / all of the
body's measured cost the table renders (en + de) with its controls, a stamped full extent, fewer (or all) rows, every cell
address resolving by serde_json RFC 6901, and the complete window returns with the credit. Independent of p1 (p1 adds
the proactive headroom check and the reactor follow-up render; both insert their own `[[test]]` row).

Usage: lb2-p7-table-arena.py [--dry-run | --write | --revert] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
BACKUP = Path(__file__).resolve().parent / "generated" / "p7-backup"
PAYLOAD = Path(__file__).resolve().parent / "payload" / "p7"
CONTRACT_DIR = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/"
CONTRACT = CONTRACT_DIR + "🦀️.rs"
MANIFEST = CONTRACT_DIR + "📦️packages/🦀️rust/Cargo.toml"
LAW = CONTRACT_DIR + "🧪️tests/📊️table-arena-headroom/🦀️.rs"
FIXTURE = CONTRACT_DIR + "🧫️fixtures/📊️table-arena-headroom/🔣️.json"
SCHEMA = CONTRACT_DIR + "🧫️fixtures/📊️table-arena-headroom/🧬️schema/🔣️.json"
CSV = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"
TSV = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"

HELPER_ANCHOR = "/// 📍️ Builds one complete revision-guarded ordinal cell address without materializing its table.\n"
HELPER = """/// 🎟️ A refusal of the process-wide `UiValue` arena is the framework's capacity refusal: the row window building this
/// address ends there with a shorter run instead of refusing the editor render (`tree_window_indexed_rows`).
fn table_arena(stage: &'static str) -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", format!("stdio table {stage} admission failed: the UiValue arena has no free credit"))
}

""" + HELPER_ANCHOR
SITES = [
    ('PluginAssemblyError::new("stdio.table.cell-arguments", "cell argument map capacity")', 'table_arena("cell argument map")'),
    ('PluginAssemblyError::new("stdio.table.cell-arguments", "row argument capacity")', 'table_arena("row argument")'),
    ('PluginAssemblyError::new("stdio.table.cell-arguments", "column argument capacity")', 'table_arena("column argument")'),
    ('PluginAssemblyError::new("stdio.table.cell-arguments", "revision argument capacity")', 'table_arena("cell revision argument")'),
    ('PluginAssemblyError::new("stdio.table.revision-arguments", "argument map capacity")', 'table_arena("structure argument map")'),
    ('PluginAssemblyError::new("stdio.table.revision-arguments", "revision argument capacity")', 'table_arena("structure revision argument")'),
    ('PluginAssemblyError::new("stdio.table.indexed-arguments", "argument map capacity")', 'table_arena("indexed argument map")'),
    ('PluginAssemblyError::new("stdio.table.indexed-arguments", "index argument capacity")', 'table_arena("index argument")'),
    ('PluginAssemblyError::new("stdio.table.indexed-arguments", "revision argument capacity")', 'table_arena("indexed revision argument")'),
]
SIGNATURE_OLD = """/// 🧱️ Adds localized structural controls and a windowed header editor around a table body.
pub fn render_structural_table(
    table: semio_framework_plugin::BuiltNode,
    header_count: usize,
    mut header: impl FnMut(usize) -> String,
    editable_headers: bool,
    controller_id: &str,
    revision: &str,
    locale: semio_framework_plugin::Locale,
    windows: &semio_framework_plugin::TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
"""
SIGNATURE_NEW = """/// 🧱️ Adds localized structural controls and a windowed header editor around a table body. The controls are admitted
/// before `table` renders its windowed body: the body takes whatever `UiValue` arena credit is left and ends its window
/// short on the arena's capacity refusal, so a starved body never takes the controls down.
pub fn render_structural_table(
    header_count: usize,
    mut header: impl FnMut(usize) -> String,
    editable_headers: bool,
    controller_id: &str,
    revision: &str,
    locale: semio_framework_plugin::Locale,
    windows: &semio_framework_plugin::TreeWindows<'_>,
    table: impl FnOnce() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
"""
PUSH_OLD = "    children.push(table);\n"
PUSH_NEW = "    children.push(table()?);\n"


def caller(editable):
    head_old = "    let table = TableWindowKit::render_indexed_matrix_with_id(\n"
    head_new = "    let table = || TableWindowKit::render_indexed_matrix_with_id(\n"
    tail_old = f"    )?;\n    semio_s_artifact_stdio_contract::render_structural_table(table, width, column_name, {editable}, controller_id, revision, locale, windows)\n"
    tail_new = f"    );\n    semio_s_artifact_stdio_contract::render_structural_table(width, column_name, {editable}, controller_id, revision, locale, windows, table)\n"
    return [("table closure head", head_old, head_new), ("structural call", tail_old, tail_new)]


TEST_ANCHOR = "[dev-dependencies]\n"
TEST_ROWS = """[[test]]
name = "table_arena_headroom"
path = "../../🧪️tests/📊️table-arena-headroom/🦀️.rs"

"""

problems, changed = [], {}
if "--revert" in sys.argv:
    for backup in sorted(path for path in BACKUP.rglob("*") if path.is_file()):
        rel = backup.relative_to(BACKUP)
        if backup.name.endswith(".absent"):
            target = ROOT / str(rel)[: -len(".absent")]
            if target.exists():
                target.unlink()
                print("removed", target.relative_to(ROOT))
                parent = target.parent
                while parent != ROOT and not any(parent.iterdir()):
                    parent.rmdir()
                    parent = parent.parent
        else:
            (ROOT / rel).write_bytes(backup.read_bytes())
            print("restored", rel)
    sys.exit(0)


def text(rel):
    return changed[rel] if rel in changed else (ROOT / rel).read_text(encoding="utf-8")


def replace_once(rel, label, old, new):
    current = text(rel)
    if new in current and old not in current:
        print(f"already applied: {rel} ({label})")
        return
    if current.count(old) != 1:
        problems.append(f"{rel}: expected 1x {label}, found {current.count(old)}")
        return
    changed[rel] = current.replace(old, new, 1)


replace_once(CONTRACT, "arena helper", HELPER_ANCHOR, HELPER)
for old, new in SITES:
    replace_once(CONTRACT, old[len("PluginAssemblyError::new("):][:60], old, new)
replace_once(CONTRACT, "structural signature", SIGNATURE_OLD, SIGNATURE_NEW)
replace_once(CONTRACT, "body push", PUSH_OLD, PUSH_NEW)
for rel, editable in [(CSV, "true"), (TSV, "false")]:
    for label, old, new in caller(editable):
        replace_once(rel, label, old, new)
manifest = text(MANIFEST)
if 'name = "table_arena_headroom"' in manifest:
    print(f"already applied: {MANIFEST}")
elif manifest.count(TEST_ANCHOR) != 1:
    problems.append(f"{MANIFEST}: expected 1x [dev-dependencies]")
else:
    changed[MANIFEST] = manifest.replace(TEST_ANCHOR, TEST_ROWS + TEST_ANCHOR, 1)
for rel, source in [(LAW, "law.rs"), (FIXTURE, "fixture.json"), (SCHEMA, "schema.json")]:
    payload = (PAYLOAD / source).read_text(encoding="utf-8")
    target = ROOT / rel
    if target.exists():
        if target.read_text(encoding="utf-8") == payload:
            print(f"already applied: {rel}")
        else:
            problems.append(f"{rel} exists with different content")
        continue
    changed[rel] = payload
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} problems={len(problems)} write={WRITE}")
for rel in changed:
    print("  ", rel)
if WRITE and not problems:
    for rel, content in changed.items():
        target = ROOT / rel
        backup = BACKUP / (rel if target.exists() else rel + ".absent")
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(target.read_bytes() if target.exists() else b"")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
