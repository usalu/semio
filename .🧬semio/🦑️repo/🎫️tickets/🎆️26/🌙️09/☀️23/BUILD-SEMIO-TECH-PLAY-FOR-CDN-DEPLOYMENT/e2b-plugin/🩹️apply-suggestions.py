#!/usr/bin/env python3
"""🩹️ Applies rustc's machine-applicable suggestions of chosen lints from one `🔁️fix-loop.py` JSON log to plugin files.

Only the named lint codes are applied (for example `unused_qualifications`), only inside `semio-framework-plugin`'s own
files, and overlapping edits are skipped. Usage: apply-suggestions.py <log tag> <lint>[,<lint>…]
"""
import json, os, sys
from collections import defaultdict
REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
log, lints = f"{REPO}/.🧬semio/🦑️repo/⚡️cache/e2b/logs/{sys.argv[1]}.json", set(sys.argv[2].split(","))
edits = defaultdict(set)
for line in open(log):
    try: j = json.loads(line)
    except ValueError: continue
    m = j.get("message") if j.get("reason") == "compiler-message" else None
    if not m or j["target"]["name"] != "semio_framework_plugin" or (m.get("code") or {}).get("code") not in lints: continue
    for c in m.get("children", []):
        for s in c.get("spans", []):
            if s.get("suggestion_applicability") == "MachineApplicable" and s.get("suggested_replacement") is not None:
                f = os.path.normpath(os.path.join(REPO, s["file_name"]))
                if f.startswith(PLUGIN): edits[f].add((s["byte_start"], s["byte_end"], s["suggested_replacement"]))
n = 0
for f, es in edits.items():
    data = open(f, "rb").read()
    last = None
    for a, b, r in sorted(es, key=lambda e: (-e[0], -e[1])):
        if last is not None and b > last: continue
        data = data[:a] + r.encode() + data[b:]; last = a; n += 1
    open(f, "wb").write(data)
print(f"[apply] {n} edits in {len(edits)} files")
