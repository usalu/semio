#!/usr/bin/env python3
# Lists parity sample candidates per top-level area from the tracked project manifests (read-only).
import json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
AREAS = {"framework": "🧰️framework", "s-plugins": "✏️s", "hub": "🌎️hub", "teaching": "🎓️teaching", "mit-bestand": "♻️mit-bestand", "semio-tech": "🏢️semio-tech"}
out = {}
for area, prefix in AREAS.items():
    files = [f for f in subprocess.run(["git", "ls-files", "--", prefix], cwd=ROOT, capture_output=True, text=True).stdout.split("\n") if f.endswith("📋️project.json")]
    rows = []
    for f in files:
        try: p = json.load(open(os.path.join(ROOT, f), encoding="utf-8"))
        except Exception: continue
        for t, d in (p.get("targets") or {}).items():
            outs = d.get("outputs") or []
            rows.append({"project": p.get("name"), "target": t, "root": os.path.dirname(f), "cache": d.get("cache"), "outputs": outs, "dependsOn": d.get("dependsOn", []), "inputs": len(d.get("inputs") or []), "command": (d.get("options") or {}).get("command", "")[:120]})
    out[area] = rows
json.dump(out, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print({k: len(v) for k, v in out.items()})
