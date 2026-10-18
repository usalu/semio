#!/usr/bin/env python3
"""🧩 BD12 item 3 installer: copies the staged phase children into the live reactor turn folder, appends the three phase groups to their existing children and rewrites the poll-kernel-turn child and the root wiring.
Refuses to run when a live file differs from the pre-edit snapshot hash (a peer touched it)."""
import hashlib, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn"
STAGE, SNAP = HERE + "/stage", HERE + "/orig/rt"
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
touched = ["🌀️poll-kernel-turn/🦀️.rs", "🦀️.rs", "🔌️extension-invocation-and-command-ingress/🦀️.rs", "🧊️cold-pair-loads-and-process-pool/🦀️.rs", "🔦️more-work-sources-and-tracing/🦀️.rs"]
for rel in touched:
    assert sha(f"{LIVE}/{rel}") == sha(f"{SNAP}/{rel}"), ("live file changed since the snapshot", rel)
new_children = ["🪜️entry-lifecycle-and-retirement", "📨️event-dispatch", "⌨️command-ingress-turn", "🎨️dirty-surface-render", "🔚️turn-finish"]
for c in new_children:
    assert not os.path.exists(f"{LIVE}/{c}"), c
    os.makedirs(f"{LIVE}/{c}")
    shutil.copyfile(f"{STAGE}/{c}/🦀️.rs", f"{LIVE}/{c}/🦀️.rs")
shutil.copyfile(f"{STAGE}/🌀️poll-kernel-turn/🦀️.rs", f"{LIVE}/🌀️poll-kernel-turn/🦀️.rs")
for c in ["🔌️extension-invocation-and-command-ingress", "🧊️cold-pair-loads-and-process-pool", "🔦️more-work-sources-and-tracing"]:
    text = open(f"{LIVE}/{c}/🦀️.rs", encoding="utf-8").read()
    if not text.endswith("\n"): text += "\n"
    text += open(f"{STAGE}/{c}/append.rs", encoding="utf-8").read()
    open(f"{LIVE}/{c}/🦀️.rs", "w", encoding="utf-8").write(text)
root = open(f"{LIVE}/🦀️.rs", encoding="utf-8").read()
anchor = '#[path = "🌀️poll-kernel-turn/🦀️.rs"]\nmod poll_turn;\nuse poll_turn::*;\n'
assert root.count(anchor) == 1
wiring = "".join(f'#[path = "{c}/🦀️.rs"]\nmod {m};\nuse {m}::*;\n\n' for c, m in [("🪜️entry-lifecycle-and-retirement", "entry_lifecycle_and_retirement"), ("📨️event-dispatch", "event_dispatch"), ("⌨️command-ingress-turn", "command_ingress_turn"), ("🎨️dirty-surface-render", "dirty_surface_render"), ("🔚️turn-finish", "turn_finish")])
root = root.replace(anchor, wiring + anchor)
open(f"{LIVE}/🦀️.rs", "w", encoding="utf-8").write(root)
print("installed")
