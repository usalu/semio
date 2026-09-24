"""R8 one-off codemod: ends each failing flow VCS law by retiring what it opened (sessions + must-retire sources)."""
import re, sys
PATH = sys.argv[1]
LAWS = """retained_vcs_cancel_around_every_transfer_has_exact_resource_fingerprints
retained_vcs_cancel_during_adjacent_transfer_rolls_back_exact_document
retained_vcs_cancel_restores_each_document_replacement_boundary
retained_vcs_cancel_restores_each_split_publication_boundary
retained_vcs_cancel_restores_every_partially_retired_redo_owner
retained_vcs_every_mutating_control_rejects_partial_grants_without_state_change
retained_vcs_fixture_authority_malformed_and_grant_vectors_execute_exact_results
retained_vcs_fixture_byte_vectors_execute_exact_multibyte_max_and_max_plus_one_results
retained_vcs_fixture_cancel_and_fault_execute_all_twenty_four_exact_transfer_states
retained_vcs_malformed_sources_fail_before_transfer_with_exact_fingerprint
retained_vcs_panic_fault_preserves_exact_resources_and_next_close_progresses
retained_vcs_repeated_rejection_preserves_source_and_credits_then_valid_control_progresses
retained_vcs_replace_document_uses_persistent_owner_transfer_phases
retained_vcs_scan_and_shift_advance_only_one_semantic_unit_per_grant
retained_vcs_stale_aba_cancel_ack_and_incremental_close_are_fail_closed
retained_vcs_zero_fuel_deadline_and_interrupted_close_preserve_every_credit""".split()
SOURCE_RETIRE = {("retained_vcs_fixture_authority_malformed_and_grant_vectors_execute_exact_results", "widget_source"): "retire_widget_source",
                 ("retained_vcs_cancel_restores_each_document_replacement_boundary", "source"): "retire_snapshot_source",
                 ("retained_vcs_replace_document_uses_persistent_owner_transfer_phases", "source"): "retire_snapshot_source"}

def block_end(text, start):
    depth = 0
    i = start
    in_str = False
    while i < len(text):
        c = text[i]
        if in_str:
            if c == '\\':
                i += 2
                continue
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c == '{':
            depth += 1
        elif c == '}':
            if depth == 0:
                return i
            depth -= 1
        i += 1
    raise ValueError("unbalanced")

s = open(PATH, encoding="utf8").read()
for law in LAWS:
    start = s.index("fn " + law + "(")
    body_open = s.index("{\n", start) + 2
    end = block_end(s, body_open)
    body = s[body_open:end]
    inserts = []
    for m in re.finditer(r"\n(\s*)let mut (\w+)\s*=\s*(FlowRetainedVcs::new|flow_hostile_session|FlowVcsSource)", "\n" + body):
        indent, name, kind = m.group(1), m.group(2), m.group(3)
        if kind == "FlowVcsSource":
            call = SOURCE_RETIRE.get((law, name))
            if not call:
                continue
            statement = f"{call}(&mut {name});"
        else:
            statement = f"close_to_terminal(&mut {name});"
        absolute = body_open + m.start()
        close = block_end(s, absolute)
        line_start = s.rfind("\n", 0, close) + 1
        inserts.append((line_start, indent + statement + "\n", close))
    ends = {}
    for pos, text, close in inserts:
        ends.setdefault(pos, []).append(text)
    for pos in sorted(ends, reverse=True):
        texts = ends[pos]
        sources = [t for t in texts if "retire_" in t]
        sessions = [t for t in texts if "close_to_terminal" in t]
        if len(set(sessions)) != len(sessions):
            raise SystemExit(f"{law}: two sessions share one block; shadowing needs a manual close")
        s = s[:pos] + "".join(sessions + sources) + s[pos:]
    print(law, len(inserts))
open(PATH, "w", encoding="utf8").write(s)
