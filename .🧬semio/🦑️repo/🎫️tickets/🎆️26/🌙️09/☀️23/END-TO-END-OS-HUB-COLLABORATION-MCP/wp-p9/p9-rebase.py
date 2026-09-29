#!/usr/bin/env python3
"""🧬️ Re-bases P9's stage onto the live tree (session 14b): for every staged file the live tree moved away from, the new
base is the live file and the new stage is the live file with P9's hunks applied — hunks whose anchor the peer's
rustfmt pass re-wrapped are re-anchored on the live text by the region they replace (`REANCHOR`). `p9-regen.sh` then
turns base → stage into the patch set again. Usage: p9-rebase.py --dry-run | --write"""
import json
import re
import os
import sys
import types
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
STAGE = TREE / os.environ.get("P9_STAGE", ".🧬semio/🌐hub/s14-p9-stage")
PATCH = Path(__file__).parent / "patches/p9-agent-lane.py"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"


def patch_hunks():
    captured = []
    module = types.ModuleType("p9_patch")
    module.ROOT = TREE
    module.replace = lambda part, path, old, new, count=1: captured.append((str(path.relative_to(TREE)), old, new))
    module.create = lambda part, path, content: None
    module.finish = lambda doc: None
    sys.modules["p9_patch"] = module
    exec(compile(PATCH.read_text(encoding="utf-8"), str(PATCH), "exec"), {"__name__": "p9"})
    return captured


def region(live: str, start: str, end: str) -> str:
    """The unique live span from `start` through the first `end` after it."""
    assert live.count(start) == 1, f"start anchor found {live.count(start)}x: {start[:80]!r}"
    lo = live.index(start)
    hi = live.index(end, lo) + len(end)
    return live[lo:hi]


def reanchor_preview_head(live: str, old: str, new: str):
    """Hunk 12: the opBytes tail + `preview_retained_command` head, re-applied as three sub-edits on the live span, so
    whatever a peer placed between them (rustfmt's `operation_id` wrap, T14/G12's `authoring_seed` fn) is kept verbatim:
    the `jobSteps` output row, the doc + signature + `verb` head, and the keyed cancellation lease."""
    live_old = region(live, '                ("opBytes".into(), DslValue::String(priced.to_string())),\n', "            let completion = ArtifactToolCompletion::<A>::new();\n")
    ops_row = '                ("opBytes".into(), DslValue::String(priced.to_string())),\n'
    steps_row = next(line + "\n" for line in new.splitlines() if '("jobSteps".into()' in line)
    old_head = live_old[live_old.index("        /// 👁️ Builds the SAME retained job"):live_old.index("            let verb = A::command_id(&command).await.to_string();\n") + len("            let verb = A::command_id(&command).await.to_string();\n")]
    new_head = new[new.index("        /// 👁️ Builds the SAME app-owned job"):new.index("            let verb = admission.verb.clone();\n") + len("            let verb = admission.verb.clone();\n")]
    lease = new[new.index("            let lease ="):new.index(";\n", new.index("            let lease =")) + 2]
    completion = "            let completion = ArtifactToolCompletion::<A>::new();\n"
    live_new = live_old.replace(ops_row, ops_row + steps_row, 1).replace(old_head, new_head, 1)
    assert live_new.endswith(completion)
    live_new = live_new[: -len(completion)] + lease + completion
    assert live_new.count("jobSteps") == 1 and "preview_typed_command_job" in live_new and "preview_retained_command" not in live_new
    return live_old, live_new


def reanchor_preview_tail(live: str, old: str, new: str):
    """Hunk 14: the retained-payload downcast rustfmt now wraps as a method chain."""
    live_old = region(live, "                instance_operation_owner: self.instance_operation_owner.clone(),\n                output_chunks: ArtifactOutputChunks::new(proof.contract().max_output_bytes),\n", "            previewed.map(|(emit, _)| emit)\n        }\n")
    head = old.split("            let payload = spec", 1)[0]
    assert live_old.startswith(head), "hunk 14 head differs from the live region"
    assert "interactive-job.preview-unsupported" in live_old and "job.preview_emit()" in live_old
    return live_old, new


REANCHOR = {"interactive-job.preview-unsupported": reanchor_preview_tail, '("jobSteps".into()': reanchor_preview_head}

write = "--write" in sys.argv
by_file: dict = {}
for rel, old, new in patch_hunks():
    by_file.setdefault(rel, []).append((old, new))
for rel, hunks in by_file.items():
    base, live = (STAGE / "base" / rel).read_text(encoding="utf-8"), (TREE / rel).read_text(encoding="utf-8")
    if base == live:
        continue
    staged = live
    for old, new in hunks:
        if staged.count(old) == 1:
            staged = staged.replace(old, new)
            continue
        key = next((key for key in REANCHOR if key in new or key in old), None)
        assert key and rel == PLUGIN, f"{rel}: no re-anchor rule for {old[:100]!r}"
        live_old, live_new = REANCHOR[key](staged, old, new)
        assert staged.count(live_old) == 1
        staged = staged.replace(live_old, live_new)
        print(f"re-anchored {rel}: {len(live_old.splitlines())} live line(s) by rule {key!r}")
    print(f"{'rebased' if write else 'would rebase'} {rel}: base = live tree ({len(live.splitlines())} lines), stage = live + {len(hunks)} hunk(s)")
    if write:
        (STAGE / "base" / rel).write_text(live, encoding="utf-8")
        (STAGE / "stage" / rel).write_text(staged, encoding="utf-8")
