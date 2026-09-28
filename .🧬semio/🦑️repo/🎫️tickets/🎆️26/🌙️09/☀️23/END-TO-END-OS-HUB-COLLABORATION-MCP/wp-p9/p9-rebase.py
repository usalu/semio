#!/usr/bin/env python3
"""🧬️ Re-bases P9's stage onto the live tree (session 14b): for every staged file the live tree moved away from, the new
base is the live file and the new stage is the live file with P9's hunks applied — hunks whose anchor the peer's
rustfmt pass re-wrapped are re-anchored on the live text by the region they replace (`REANCHOR`). `p9-regen.sh` then
turns base → stage into the patch set again. Usage: p9-rebase.py --dry-run | --write"""
import json
import re
import sys
import types
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
STAGE = TREE / ".🧬semio/🌐hub/s14-p9-stage"
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
    """Hunk 12: the opBytes tail + `preview_retained_command` head, whose `operation_id` line rustfmt now wraps."""
    live_old = region(live, '                ("opBytes".into(), DslValue::String(priced.to_string())),\n', "            let completion = ArtifactToolCompletion::<A>::new();\n")
    wrapped = re.search(r"            let operation_id =\n                self\.admit_typed_operation_slot\(\)[^\n]*\n", live_old).group(0)
    one_line = re.search(r"            let operation_id = self\.admit_typed_operation_slot\(\)[^\n]*\n", new).group(0)
    assert one_line.split("= ", 1)[1].strip() == wrapped.split("=\n", 1)[1].strip(), "operation_id statement changed"
    live_new = new.replace(one_line, wrapped)
    live_new_old = old.replace(re.search(r"            let operation_id = self\.admit_typed_operation_slot\(\)[^\n]*\n", old).group(0), wrapped)
    assert live_new_old == live_old, "hunk 12 differs from the live region beyond the wrapped operation_id line"
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
