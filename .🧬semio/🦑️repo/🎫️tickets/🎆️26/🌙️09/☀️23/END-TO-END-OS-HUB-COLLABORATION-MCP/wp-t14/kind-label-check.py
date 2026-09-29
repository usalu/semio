#!/usr/bin/env python3
"""🏷️ T14 picker-label check over the generated plugin descriptors (`✏️s/🔌️plugins/*/🔣️.json`): every artifact kind a
creation picker offers (each editor/viewer app with an `io.artifactSchema`, resolved to its kind by schema among every
kind the package declares — an editor app's kinds and the manifest's; a viewer of the same dialect declares none) carries its own `label` with en + de, and no two distinct kinds of one package share an en or de label.
usage: kind-label-check.py"""
import glob
import json
import sys
from collections import defaultdict

problems, info, kinds = [], [], 0
for path in sorted(glob.glob("/Users/ueli/Documents/semio/✏️s/🔌️plugins/*/🔣️.json")):
    plugin = path.split("/")[-2]
    manifest = json.load(open(path, encoding="utf-8"))
    manifest = manifest.get("manifest", manifest)
    labels = {"en": defaultdict(set), "de": defaultdict(set)}
    for app in manifest.get("apps", []):
        schema = app.get("io", {}).get("artifactSchema")
        if app.get("role") not in ("editor", "viewer") or not schema:
            continue
        package_kinds = [k for other in manifest.get("apps", []) for k in (other.get("artifactKinds") or [])] + (manifest.get("artifactKinds") or [])
        kind = next((k for k in package_kinds if k.get("schema") == schema), None)
        if kind is None:
            if app["role"] == "editor":
                info.append(f"{plugin}: {app['dialect']['artifactKind']} ({schema}) declares no kind -> not creatable (W4's identity gate owns declared-vs-io mismatches)")
            continue
        if "label" not in kind:
            problems.append(f"{plugin}: kind {kind['id']} ({schema}) has no label")
            continue
        kinds += 1
        for locale in ("en", "de"):
            text = kind["label"]["native"].get(locale, "")
            if not text:
                problems.append(f"{plugin}: kind {kind['id']} has an empty {locale} label")
            labels[locale][text].add(app["dialect"]["artifactKind"])
    for locale, by_text in labels.items():
        for text, owners in by_text.items():
            if len(owners) > 1:
                problems.append(f"{plugin}: {locale} label {text!r} shared by {sorted(owners)}")
print("\n".join(info))
print("\n".join(problems[:60]))
print(f"picker kinds {kinds}; problems {len(problems)}")
sys.exit(1 if problems else 0)
