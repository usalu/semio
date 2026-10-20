#!/usr/bin/env python3
"""Staged removal of the retired gismap `inference-presentation-check` package script (package.json is behind the HARD FREEZE). Usage: script-removal.py [--apply]."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("SEMIO_ROOT") or HERE
while ROOT != os.path.dirname(ROOT) and not (os.path.isfile(os.path.join(ROOT, "bunfig.toml")) and os.path.isfile(os.path.join(ROOT, "bun.lock"))):
    ROOT = os.path.dirname(ROOT)
PJ = "/".join(["✏️s", "\U0001F50C️plugins", "\U0001F30D️gis", "\U0001F5FF️artifacts", "\U0001F5FA️gismap", "\U0001F4E6️packages", "\U0001F7E6️typescript", "package.json"])
path = os.path.join(ROOT, PJ)
text = open(path, encoding="utf-8").read()
data = json.loads(text)
if "inference-presentation-check" not in data["scripts"]:
    print("already removed"); sys.exit(0)
del data["scripts"]["inference-presentation-check"]
out = json.dumps(data, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")
if "--apply" in sys.argv[1:]:
    open(path, "w", encoding="utf-8").write(out); print("removed (written)")
else:
    print("would remove inference-presentation-check (dry run)")
