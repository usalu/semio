#!/usr/bin/env python3
"""Independent mutation-kind catalog check for EN 1997 Wave C."""
import json
from pathlib import Path
oracle = Path(__file__).resolve().parents[2] / "🔮️oracles" / "🔣️.json"
data = json.loads(oracle.read_text())
kinds = [m["kind"] for m in data["mutations"]]
assert len(kinds) == 20, kinds
print(json.dumps({"ok": True, "kinds": len(kinds)}))
