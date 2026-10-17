#!/usr/bin/env python3
"""🩹️ Consumer repairs after the WP3/WP4 flips seen by the full checkpoint cp5, and the cfg(test) leak conversion (`#[cfg(test)]` definitions that tests outside the crate use become `cfg(any(test, feature = "artifact-app-testing"))`)."""
import re, os, sys
import ed
from lib import *

NEW = '#[cfg(any(test, feature = "artifact-app-testing"))]'
LEAKS = {
    "child": ["ChildAdmissionTestState"],
    "command": ["HISTORY_ROWS_BUILT"],
    "composition-publication": ["test_original_pending_child_group_closure"],
    "document-media": ["refusal_fault", "terminal_store_frame_bytes"],
    "job": ["test_artifact_owned_tool_job_context_identity_digest", "test_retained_factory_proof_join", "test_unregistered_tool_job_admission_rejected", "test_window_config_context_identity", "test_window_transient_context_identity"],
    "reactor-lifecycle": ["borrowed_patch", "has_unpublished", "phases", "receipt_is_issued", "insert", "is_empty", "cause"],
    "reactor-patches": ["in_flight_generations", "retain_unadmitted", "take_ready_patch"],
    "reactor-tasks": ["cancel_instance_tasks"],
    "retained-command": ["test_blank_mounted_operation", "test_original_mounted_child_frontier", "test_pending_emit_shape", "test_raw_allocation_close", "test_retained_structural_refusal"],
    "runtime-instances": ["test_admit_instance_actor", "test_push_instance"],
}


def convert(text, name):
    out, hits = [], 0
    lines = text.split("\n")
    for i, l in enumerate(lines):
        m = re.search(r"(?:\b(?:fn|struct|enum|const|type|trait)\s+|\bstatic\s+)" + re.escape(name) + r"\b", l)
        if not m or l.lstrip().startswith("//"):
            continue
        j = i - 1
        while j >= 0 and (lines[j].lstrip().startswith("#[") or lines[j].lstrip().startswith("///")):
            if lines[j].strip() == "#[cfg(test)]":
                ind = lines[j][: len(lines[j]) - len(lines[j].lstrip())]
                lines[j] = ind + NEW
                hits += 1
                lines[i] = re.sub(r"\bpub\((?:crate|super|self)\)\s+", "pub ", l)
                if lines[i] == l and not re.match(r"\s*pub\b", l):
                    lines[i] = re.sub(r"^(\s*)", r"\1pub ", l, count=1)
                break
            j -= 1
        else:
            continue
    # thread_local blocks
    t = "\n".join(lines)
    t2 = re.sub(r"#\[cfg\(test\)\](\s*thread_local!\s*\{[^}]*?\b" + re.escape(name) + r"\b)", NEW + r"\1", t, flags=re.S)
    if t2 != t:
        hits += 1
        t2 = re.sub(r"pub\(crate\) (static " + re.escape(name) + r"\b)", r"pub \1", t2)
    return t2, hits


def main():
    import manifest as MF, uo
    from cfgtest import crate_files
    reals = uo.real_crates()
    for K, names in LEAKS.items():
        files = sorted(crate_files(reals[K]))
        for name in names:
            done = 0
            for f in files:
                t = read(f)
                t2, h = convert(t, name)
                if h:
                    ed.put(f, t2)
                    done += h
                    print("converted", K, name, rel(f))
            if not done:
                print("MISS", K, name)


if __name__ == "__main__":
    main()
