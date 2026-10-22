#!/usr/bin/env python3
"""Copy of the agent-session set without its edit of the product README: docs-1021a documents agent sessions in the same regions (gated by the feature state), and both writing there makes the docs set's earlier text vanish from the README."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/agent-session-tools", "agent-session")
spec = json.load(open(T + "/staged/edits.json", encoding="utf8"))
rows = spec["edits"] if isinstance(spec.get("edits"), list) else None
if rows is None:
    rows = spec
    raise SystemExit("maker: unexpected agent-session edits.json shape: " + str(list(spec)[:5]))
product = "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/README.md"
kept = [row for row in rows if row.get("file") != product]
assert len(rows) - len(kept) == 1, len(rows) - len(kept)
spec["edits"] = kept
json.dump(spec, open(T + "/staged/edits.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("agent-session copy:", T)
