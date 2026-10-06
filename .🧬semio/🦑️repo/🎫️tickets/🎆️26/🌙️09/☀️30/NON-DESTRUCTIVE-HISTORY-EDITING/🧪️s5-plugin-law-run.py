"""🧮️ Runs every owner's filters on ONE prebuilt `semio-framework-plugin` lib test binary (coordinator 16:20) and writes the
table `📓️s5-plugin-law-run.md`: owner × filter × passed / failed × failures with their first message.

    python3 🧪️s5-plugin-law-run.py <test binary> [<owner>…]

One invocation per row, one after the other, from the crate's package directory with `RUST_MIN_STACK=268435456` and
`--test-threads=4`; each invocation's whole output is kept under `🗑️generated/s5-runtime/law-run/`. Naming owners re-runs
only their rows and rewrites the table from every kept output.
"""

import os
import pathlib
import re
import subprocess
import sys
import time

TICKET = pathlib.Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PACKAGE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"
LOGS = TICKET / "🗑️generated/s5-runtime/law-run"
TABLE = TICKET / "📓️s5-plugin-law-run.md"

NESTED = (
    "a_child_survives an_agent_transaction_carries_owned_child composite_gesture_produces_one_undo_group a_composed_document_crosses "
    "a_composed_documents_child_heads member_factory_closed_dialect member_factory_parent_snapshot_restore a_composed_checkpoint_commit "
    "a_checkpoint_pins_its_children child_content_publication_path child_snapshot_retirement_rejection child_root_maintenance "
    "child_publications_at_the_maximum_rate maximum_child_public_dispatch the_child_content_view_never_goes_stale group_undo_skips_a_foreign_tail "
    "created_children_survive_absorb retained_child_group_publishes retained_window_input_recursive retained_composed_replacement "
    "a_whole_document_media_import fixed_child_member_registry stale_child_member_admission incomplete_child_member_registry "
    "owned_document_ingress member_run_ member_backfill_answers a_history_edit_is_its_own_row an_overwrite_head_equals_a_fresh_fold "
    "every_fixture_scenario_reaches owned_child_emission"
)

ROWS = [
    ("RUNTIME", "transient", "transient_root"),
    ("RUNTIME", "s22", "an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves hostile_history_edit_input_is_answered_never_panicked_on a_blocking_mutation_without_editable_inputs"),
    ("RUNTIME", "actor", "an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route"),
    ("RUNTIME", "actor-store-half", "every_route_authors_as_its_acting_actor"),
    ("RUNTIME", "w2a", "time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body composed_child_history"),
    ("LOAD", "load", "folder_reload_route a_merge_archive_command document_backbone"),
    ("TOOLS", "gesture", "gesture_laws"),
    ("GATES", "ledger", "a_blocking_ledger_replay"),
    ("CHANNEL", "funnel", "typed_wire_values_leave_in_key_byte_order_whatever_their_types_declare"),
    ("NESTED", "nested", NESTED),
    ("AGNOSTIC", "acceptance", "history_edit_acceptance"),
    ("ALL", "time-travel-module", "time_travel"),
    ("EXTRA", "tool-run-family", "tool_run"),
]


def run(binary, owner, label, filters):
    started = time.time()
    done = subprocess.run([binary, "--test-threads=4", *filters.split()], cwd=PACKAGE, env={**os.environ, "RUST_MIN_STACK": "268435456", "CARGO_MANIFEST_DIR": str(PACKAGE)}, capture_output=True, text=True, errors="replace")
    text = f"$ {binary} --test-threads=4 {filters}\n{done.stdout}\n--- stderr ---\n{done.stderr}\nexit={done.returncode} seconds={time.time() - started:.1f}\n"
    (LOGS / f"{owner}-{label}.txt").write_text(text, encoding="utf-8")


def verdict(text):
    result = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored; \d+ measured; (\d+) filtered out", text)
    failed = re.findall(r"^test (\S+) \.\.\. FAILED$", text, re.M)
    messages = {}
    for name in failed:
        block = re.search(rf"---- {re.escape(name)} stdout ----\n(.*?)(?=\n---- |\nfailures:\n)", text, re.S)
        lines = [line.strip() for line in (block.group(1) if block else "").splitlines() if line.strip() and not line.startswith("note: ")]
        panic = next((index for index, line in enumerate(lines) if "panicked at" in line), None)
        message = " ".join(lines[panic + 1 : panic + 4]) if panic is not None else " ".join(lines[:3])
        messages[name] = message[:420]
    exit_code = re.search(r"^exit=(-?\d+)", text, re.M)
    return result, failed, messages, exit_code.group(1) if exit_code else "?"


def table(binary):
    lines = [
        "# 📓️ Plugin law run on the ONE shared test binary",
        "",
        f"Binary: `{binary}` (`cargo test -p semio-framework-plugin --lib --features artifact-app-testing --no-run`, built {time.strftime('%Y-%m-%d %H:%M', time.localtime(pathlib.Path(binary).stat().st_mtime))}).",
        f"Invocation per row: `cd {PACKAGE.relative_to(ROOT)} && RUST_MIN_STACK=268435456 <binary> --test-threads=4 <filters>`; whole outputs under `🗑️generated/s5-runtime/law-run/`.",
        "",
        "| owner | filters | passed | failed | failures (first message) |",
        "| --- | --- | ---: | ---: | --- |",
    ]
    totals = [0, 0]
    for owner, label, filters in ROWS:
        log = LOGS / f"{owner}-{label}.txt"
        if not log.exists():
            lines.append(f"| {owner} | `{label}` | – | – | not run |")
            continue
        result, failed, messages, exit_code = verdict(log.read_text(encoding="utf-8"))
        shown = filters if len(filters) < 120 else f"{filters[:117]}…"
        if result is None:
            lines.append(f"| {owner} | `{shown}` | ? | ? | NO VERDICT (exit {exit_code}): the binary ended without a result line — see `{log.name}` |")
            continue
        passed, count = int(result.group(1)), int(result.group(2))
        if owner not in ("ALL", "EXTRA"):
            totals[0] += passed
            totals[1] += count
        detail = "<br>".join(f"`{name.split('::')[-1]}` — {messages[name].replace('|', '¦')}" for name in failed) or ("no test matches" if passed == 0 else "")
        lines.append(f"| {owner} | `{shown}` | {passed} | {count} | {detail} |")
    lines += ["", f"Totals over the owner rows (the `ALL` regression row overlaps `w2a`; the `EXTRA` row is the whole tool-run family, of which NESTED's `member_run_` laws are a part): **{totals[0]} passed / {totals[1]} failed**.", ""]
    TABLE.write_text("\n".join(lines), encoding="utf-8")
    print("\n".join(lines[5:]))


def main():
    binary, owners = sys.argv[1], set(sys.argv[2:])
    LOGS.mkdir(parents=True, exist_ok=True)
    for owner, label, filters in ROWS:
        if not owners or owner in owners:
            run(binary, owner, label, filters)
    table(binary)


if __name__ == "__main__":
    main()
