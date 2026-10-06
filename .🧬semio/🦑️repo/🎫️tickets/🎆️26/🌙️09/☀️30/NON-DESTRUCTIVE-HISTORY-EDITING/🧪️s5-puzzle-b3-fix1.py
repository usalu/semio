"""🩹️ S5-PUZZLE wave B3, fix-forward 1 — all or nothing. `python3 <this file> --check | --apply`

- F16 withdrawn from the wave: with `HierarchyProvider::Topology` a selection made while a history edit is open no longer
  settles (`history_edit_runtime_tests`, step `{"select": ["late"]}`) and the editor loses its reference control after a
  pick (`select_tool_history_tests`, `useSelection`). The domain is `Flat` again and the unused topology goes with it.
- F24 law states what the guest guarantees: a transient flush commits nothing (the framework's command log still upserts
  the verb's own un-applied, op-less row).
- `the_board_emit_carries_the_transaction_and_the_parametric_leaf` dispatches from a window view, as every action must.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ED = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
E = f"{ED}/🦀️.rs"
LAW = f"{ED}/🧪️tests/🧪️board-tools/🦀️.rs"
CORPUS = f"{ED}/🧫️fixtures/🧫️board-tools/🔣️.json"
TRANSACTIONS = f"{ED}/🧪️tests/🧪️select-tool-transactions/🦀️.rs"
texts = {}


def text(path):
    return texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))


def sub(path, old, new):
    found = text(path).count(old)
    if found != 1:
        sys.exit(f"{path}: anchor resolves {found} times, expected 1:\n{old[:140]}")
    texts[path] = texts[path].replace(old, new)


def cut(path, first, last, replacement=""):
    lines = text(path).split("\n")
    begin = [index for index, line in enumerate(lines) if first in line]
    if len(begin) != 1:
        sys.exit(f"{path}: cut start resolves {len(begin)} times:\n{first}")
    end = next((index for index in range(begin[0] + 1, len(lines)) if lines[index] == last), None)
    if end is None:
        sys.exit(f"{path}: cut end not found:\n{last}")
    texts[path] = "\n".join(lines[: begin[0]] + ([replacement] if replacement else []) + lines[end + 1 :])


sub(E, "/// whose topology the app supplies (`interaction_topology`): nodes, their handles, edges and target regions.\n", "/// `Flat` hierarchy (no parent/child structure was ever modeled for it).\n")
sub(E, "hierarchy: HierarchyProvider::Topology,", "hierarchy: HierarchyProvider::Flat,")
cut(E, "    /// 🗺️ The `vortex` domain's whole pickable universe, in board order", "    }")
lines = text(E).split("\n")
texts[E] = "\n".join(line for index, line in enumerate(lines) if not (line == "" and index > 0 and lines[index - 1] == "" and index + 1 < len(lines) and lines[index + 1].startswith("    /// 🪧️ A history-edit reference chip")))
sub(E, "/// tool, a transient flush is no history row, and the `vortex` topology names every board entity.\n", "/// tool, and a transient flush commits nothing.\n")
sub(LAW, "//! nothing and a flush of transient rows alone leave zero trace.", "//! nothing and a flush of transient rows alone commit nothing.")
cut(LAW, "/// 🫥️ LAW (F24): a flush of transient kinds alone", "}", """/// 🫥️ LAW (F24): a flush of transient kinds alone — a hover, a selection, a candidate page, nothing — commits nothing,
/// however a host batches it: no applied row, no printed op, no inline mutation. The framework's command log still upserts
/// the verb's own row for the typed operation (un-applied, without ops); a host that lists such rows shows them.
#[test]
fn a_flush_of_transient_rows_commits_nothing() {
    let corpus = corpus();
    let mut app = select_tool_history_tests::seeded_app(&corpus["board"]);
    for case in corpus["cases"].as_array().expect("cases").iter().filter(|case| case["transient"] == true) {
        let result = flush(&mut app, &case["rows"]);
        let upserts: Vec<(u64, bool, usize)> = result.history_patch.as_ref().map(|patch| patch.upserts.iter().map(|entry| (entry.seq, entry.applied, entry.op_lines.len())).collect()).unwrap_or_default();
        assert!(upserts.iter().all(|(_, applied, ops)| !applied && *ops == 0), "{}: a transient flush applies nothing and prints no op: {upserts:?}", case["name"]);
        assert_eq!(committed_edits(&result), 0, "{}: no edit", case["name"]);
        assert!(result.mutations.is_empty(), "{}: and no inline mutation", case["name"]);
    }
    close_app(&mut app);
}""")
lines = text(LAW).split("\n")
begin = [index for index, line in enumerate(lines) if line.startswith("/// 🗺️ LAW (F16)")]
if len(begin) != 1:
    sys.exit(f"{LAW}: the topology law resolves {len(begin)} times")
texts[LAW] = "\n".join(lines[: begin[0]]).rstrip("\n") + "\n"
sub(CORPUS, "`transient: true` marks a flush of transient kinds alone, which upserts no history row at all (F24).", "`transient: true` marks a flush of transient kinds alone, which commits nothing (F24).")
sub(TRANSACTIONS, "    let emit_for = |seed: &str| puzzle2d_dispatch_emit(", "    let view = window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID);\n    let emit_for = |seed: &str| puzzle2d_dispatch_emit(")
sub(TRANSACTIONS, "overview::WINDOW_KIND_ID, None, select_utility::UTILITY_ID, &protocol::DomainSelection::default(), seed,", "overview::WINDOW_KIND_ID, Some(&view), select_utility::UTILITY_ID, &protocol::DomainSelection::default(), seed,")

mode = sys.argv[1] if len(sys.argv) > 1 else ""
if mode not in ("--check", "--apply"):
    sys.exit(__doc__)
for path, body in texts.items():
    before = (ROOT / path).read_text(encoding="utf-8")
    print(f"{len(body.splitlines()) - len(before.splitlines()):+4d} lines  {path.split('/')[-2]}/{path.split('/')[-1]}")
    if mode == "--apply":
        (ROOT / path).write_text(body, encoding="utf-8")
print("applied" if mode == "--apply" else "checked")
