#!/usr/bin/env python3
"""📚 BD12 item 3 follow-up: moves the 27 orphan statement docs that were hoisted above `poll_kernel_turn` onto the phase functions that own the code they describe. Text is moved verbatim; the pre-edit files are copied to bd12-tools/orig/docs first."""
import os, re, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn"
POLL = T + "/🌀️poll-kernel-turn/🦀️.rs"
ROUTE = {
    "📨️event-dispatch": {"dispatch_turn_event": [7], "ui_intent_event": [4, 15], "job_completed_event": [8], "advance_original_receiving": [5, 6, 13, 14]},
    "⌨️command-ingress-turn": {"drive_command_ingress": [10, 11, 12]},
    "🔌️extension-invocation-and-command-ingress": {"drive_extension_turn": [0, 9]},
    "🪜️entry-lifecycle-and-retirement": {"drive_turn_retirement": [1, 2, 3], "step_pending_patch_close": [16]},
    "🎨️dirty-surface-render": {"render_dirty_surfaces": [17, 18, 19, 20]},
    "🔦️more-work-sources-and-tracing": {"drive_turn_executors": [21, 22, 23], "settle_turn_work": [25]},
    "📬️task-resumes-and-wire-effects": {"drain_task_resumes": [24]},
}
KEEP = [26]
lines = open(POLL, encoding="utf-8").read().split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith("///"))
end = start
while lines[end].startswith("///"):
    end += 1
paras, cur = [], []
for l in lines[start:end]:
    if l.strip() == "///":
        if cur: paras.append(cur); cur = []
    else:
        cur.append(l)
if cur: paras.append(cur)
assert len(paras) == 27, len(paras)
used = sorted(i for fns in ROUTE.values() for v in fns.values() for i in v) + KEEP
assert sorted(used) == list(range(27)), sorted(used)
os.makedirs(HERE + "/orig/docs", exist_ok=True)
for child, fns in ROUTE.items():
    path = f"{T}/{child}/🦀️.rs"
    shutil.copyfile(path, HERE + "/orig/docs/" + child + ".rs") if not os.path.exists(HERE + "/orig/docs/" + child + ".rs") else None
    src = open(path, encoding="utf-8").read().split("\n")
    for fn, idxs in fns.items():
        rx = re.compile(r"^(?:pub(?:\([a-z]+\))? )?(?:async )?fn " + fn + r"\b")
        hits = [i for i, l in enumerate(src) if rx.match(l)]
        assert len(hits) == 1, (child, fn, hits)
        j = hits[0] - 1
        while src[j].lstrip().startswith("#["):
            j -= 1
        assert src[j].startswith("///"), (child, fn, src[j][:60])
        block = []
        for i in idxs:
            block += ["///"] + paras[i]
        src[j + 1:j + 1] = block
    open(path, "w", encoding="utf-8").write("\n".join(src))
summary = "/// 🌀️ Runs one reactor turn that polls the kernel: admits the retained input, drives the retirement ladders, serves the one selected event, ingresses commands and cold pairs, renders dirty surfaces, reconciles, runs the executors, settles the more-work verdict and publishes the turn result. Each phase is a function of its own child module."
rest = lines[:start] + [summary, "///"] + paras[26] + lines[end:]
open(POLL, "w", encoding="utf-8").write("\n".join(rest))
print("routed", len(used), "paragraphs")
