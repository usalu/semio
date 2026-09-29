"""🧮️ S20 faults overlay: `baseline` records the sha1 of every source file of the overlay at clone time; `diff` lists the
files the overlay changed, created or deleted since — the landing set L1 copies into the tree in ONE train.
Usage: python3 s20-overlay-diff.py baseline|diff
"""
import hashlib
import json
import os
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
BASELINE = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults.baseline.json")
SKIP = {"node_modules", "target", ".nx", "dist", "🗑️generated", ".git", "__pycache__", ".venv"}


def files() -> dict[str, str]:
    out: dict[str, str] = {}
    for root, dirs, names in os.walk(OVERLAY):
        dirs[:] = [d for d in dirs if d not in SKIP and not d.startswith("target-")]
        for name in names:
            path = Path(root) / name
            try:
                out[str(path.relative_to(OVERLAY))] = hashlib.sha1(path.read_bytes()).hexdigest()
            except OSError:
                continue
    return out


if sys.argv[1] == "baseline":
    snapshot = files()
    BASELINE.write_text(json.dumps(snapshot))
    print(f"baseline files={len(snapshot)}")
else:
    base = json.loads(BASELINE.read_text())
    now = files()
    changed = sorted(p for p in now if p in base and now[p] != base[p])
    created = sorted(p for p in now if p not in base)
    deleted = sorted(p for p in base if p not in now)
    print(json.dumps({"changed": changed, "created": created, "deleted": deleted}, ensure_ascii=False, indent=1))
