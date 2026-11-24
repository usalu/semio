#!/usr/bin/env python3
"""Applies only the lazy-activation catalog deltas to the committed generated catalog: procedural depends on the flow hub alone, and each flow extension carries its `on-node-kind:` activation event. Mirrors what the real generator emits for those rows; unrelated peer drift in the generator output is left alone."""
import json, re, sys
OUT = "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/"
EVENTS = {"bim": "bim", "brep": "brep", "dictionary": "dictionary", "draw": "draw", "list": "list", "logic": "logic", "math": "math", "primitive": "core", "text": "text"}
EVENT = {f"flow-extension-{slug}": [f"on-node-kind:{ns}"] for slug, ns in EVENTS.items()}

def patch_rows(rows):
    for row in rows:
        if row["pluginId"] == "procedural": row["dependsOn"] = ["flow"]
        if row["pluginId"] in EVENT: row["activationEvents"] = EVENT[row["pluginId"]]

def rewrite_json(name, locate):
    path = OUT + name
    text = open(path, encoding="utf8").read()
    data = json.loads(text)
    assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == text, f"{name} does not round trip byte-identically"
    locate(data)
    open(path, "w", encoding="utf8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")

rewrite_json("🔌️plugins.json", patch_rows)
rewrite_json("🗂️sources.json", lambda d: [row.__setitem__("dependsOn", ["flow"]) for row in (d if isinstance(d, list) else d.get("plugins", d.get("entries", []))) if row.get("pluginId") == "procedural"])
rewrite_json("🦀️catalog.json", lambda d: [row.__setitem__("activationEvents", EVENT[row["pluginId"]]) for key in ("plugins", "extensions") for row in d.get(key, []) if row.get("pluginId") in EVENT])

path = OUT + "🧩️plugins/🟦️.ts"
lines = open(path, encoding="utf8").read().split("\n")
for index, line in enumerate(lines):
    m = re.match(r'\t\{ pluginId: "([^"]+)"', line)
    if not m: continue
    pid = m.group(1)
    if pid == "procedural":
        new = re.sub(r'dependsOn: \[[^\]]*\]', 'dependsOn: ["flow"]', line, count=1)
    elif pid in EVENT:
        new = re.sub(r'activationEvents: \[[^\]]*\]', 'activationEvents: ' + json.dumps(EVENT[pid], ensure_ascii=False), line, count=1)
    else: continue
    assert new != line, pid
    lines[index] = new
open(path, "w", encoding="utf8").write("\n".join(lines))
print("patched")
