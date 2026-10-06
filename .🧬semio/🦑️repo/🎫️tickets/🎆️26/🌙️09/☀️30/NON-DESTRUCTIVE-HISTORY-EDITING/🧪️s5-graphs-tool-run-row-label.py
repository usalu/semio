"""🏷️ S5-GRAPHS-WIRES — wave `tool-run-row-label` (follow-up of `tool-run-settle`, law run of 10-05 17:20: 39 / 3).

`label` (plugin runtime, under the `landing` lock): the history row of a finalized MEMBER tool run was labelled from its
first child leaf ("Set count to 1 (+9)") instead of the tool ("Toy member fill"), the one assertion
`member_run_finalize_is_one_member_edit_carrying_the_run_transaction_and_one_undo_removes_it` still fails on. The row's
transaction and its tool-run label are already resolved from the child edit; the label arm that uses them matched only
rows with a parent edit. A tool run's row is the tool's in whichever store its one edit lands (design §21.1).

`diagnostic` (member law file, test-only, STAGED — it is not covered by the coordinator's `--lib` train, so it lands only
together with a rebuild of the shared plugin test binary): the member pump's failure also prints one step of the
child-root retirement ring, whose `Blocked { reason }` names what the ring waits for. It is the next piece of evidence
for `member_run_holds_at_most_the_member_ceiling_and_reports_the_cap` (abort after a 4096-op run never settles while
`interactive-job.child-root-retirement-saturated` persists).

    python3 🧪️s5-graphs-tool-run-row-label.py --check   label|diagnostic
    python3 🧪️s5-graphs-tool-run-row-label.py --apply   label|diagnostic
    python3 🧪️s5-graphs-tool-run-row-label.py --restore label|diagnostic

Fails closed: explicit files, every anchor exactly once, `--restore` refuses a file that is no longer what `--apply` wrote.
"""

import pathlib
import sys

TICKET = pathlib.Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
BACKUP = TICKET / "🗑️generated/s5-graphs-wires/tool-run/before"
PARTS = {
    "label": (
        PLUGIN / "🦀️.rs",
        "row-label.plugin.rs",
        """                (Some(_), _) if tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
""",
        """                _ if tool_run_label.is_some() => tool_run_label.expect("tool run label checked above"),
""",
    ),
    "diagnostic": (
        PLUGIN / "🧪️tests/🧪️tool-run-member/🦀️.rs",
        "row-label.member-laws.rs",
        """    let admission = app.admit_child_content_publication().err().map(|fault| fault.code.0);
    panic!("{what} never settled; state {:?}; member ops {}; ledger work {}; step reasons {reasons:?}; child-content admission refusal {admission:?}", app.tool_runs.state(), app.tool_runs.member_ops().len(), app.tool_runs.has_pending_work());
""",
        """    let admission = app.admit_child_content_publication().err().map(|fault| fault.code.0);
    let ring = app.child_root_retirement_step(1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP).map_err(|fault| fault.code.0);
    panic!("{what} never settled; state {:?}; member ops {}; ledger work {}; step reasons {reasons:?}; child-content admission refusal {admission:?}; child-root ring step {ring:?}", app.tool_runs.state(), app.tool_runs.member_ops().len(), app.tool_runs.has_pending_work());
""",
    ),
}


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) == 3 else ""
    part = sys.argv[2] if len(sys.argv) == 3 else ""
    if mode not in ("--check", "--apply", "--restore") or part not in PARTS:
        raise SystemExit("usage: 🧪️s5-graphs-tool-run-row-label.py --check | --apply | --restore  label|diagnostic")
    path, name, before, after = PARTS[part]
    if not path.is_file():
        raise SystemExit("[tool-run-row-label] REFUSED: %s is missing" % path)
    source = path.read_text()
    if mode == "--restore":
        backup = BACKUP / name
        if not backup.is_file() or source.count(after) != 1 or backup.read_text().replace(before, after) != source:
            raise SystemExit("[tool-run-row-label] REFUSED: %s changed since --apply (or has no backup); restore the one anchor by hand" % part)
        path.write_text(backup.read_text())
        print("[tool-run-row-label] restored %s" % part)
        return
    if source.count(before) != 1 or source.count(after) != 0:
        raise SystemExit("[tool-run-row-label] REFUSED, nothing written: anchor of %s occurs %d times (expected 1), result already present %d times" % (part, source.count(before), source.count(after)))
    if mode == "--apply":
        BACKUP.mkdir(parents=True, exist_ok=True)
        (BACKUP / name).write_text(source)
        path.write_text(source.replace(before, after))
    print("[tool-run-row-label] %s (%s): 1 edit in 1 file" % ("applied" if mode == "--apply" else "would apply", part))


main()
