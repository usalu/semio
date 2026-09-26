#!/usr/bin/env python3
"""🎯️ Ticket-local measurement (not a law): replays the hub's one pairing rule (`app_opens_kind` +
`descriptor_open_targets`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`) over every staged descriptor
JSON projection (`dist/dev/🔌️plugin-modules/<plugin>/🔣️.json`) and lists editor surfaces that open no kind.
Usage: open-target-census.py [plugin …]"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules")


def surface_id(app):
    d = app["dialect"]
    return f'{d["artifactKind"]}@{d["standard"]}/{d["subset"]}#{app["role"]}'


def opens(manifest, app, kind):
    declares = lambda kinds: any(k["id"] == kind["id"] and k["schema"] == kind["schema"] for k in kinds)
    if declares(manifest["artifactKinds"]):
        return app["dialect"]["artifactKind"] == kind["id"]
    if app["role"] == "editor":
        return declares(app.get("artifactKinds", []))
    return declares(app.get("artifactKinds", [])) or any(e["role"] == "editor" and e["dialect"] == app["dialect"] and declares(e.get("artifactKinds", [])) for e in manifest["apps"])


def targets(descriptor):
    manifest = descriptor["manifest"]
    if descriptor.get("execution") not in ("isolated", "Isolated"):
        return []
    found = []
    for app in manifest["apps"]:
        if app["id"] != surface_id(app):
            continue
        editors = [e for e in manifest["apps"] if app["role"] == "viewer" and e["role"] == "editor" and e["dialect"] == app["dialect"]]
        seen = set()
        for kind in manifest["artifactKinds"] + app.get("artifactKinds", []) + [k for e in editors for k in e.get("artifactKinds", [])]:
            key = (kind["id"], kind["schema"])
            if key in seen or not opens(manifest, app, kind):
                continue
            seen.add(key)
            found.append((app["id"], app["role"], kind["id"]))
    return found


only = set(sys.argv[1:])
total_editors = total_open = 0
for path in sorted(ROOT.glob("*/🔣️.json")):
    descriptor = json.loads(path.read_text(encoding="utf-8"))
    plugin = descriptor["manifest"]["pluginId"]
    if only and plugin not in only:
        continue
    found = targets(descriptor)
    editors = [a for a in descriptor["manifest"]["apps"] if a["role"] == "editor" and a["id"] == surface_id(a)]
    opened = {surface for surface, _, _ in found}
    missing = [a["id"] for a in editors if a["id"] not in opened]
    total_editors += len(editors); total_open += len(editors) - len(missing)
    print(f"{plugin}: execution={descriptor.get('execution')} editors={len(editors)} opening={len(editors) - len(missing)} targets={len(found)}")
    for surface in missing:
        print(f"  NO KIND  {surface}")
print(f"TOTAL editors={total_editors} opening={total_open}")
