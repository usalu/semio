#!/usr/bin/env python3
"""Structural cross-check for the 🎥️shooting fixtures while cargo is unusable.

Re-implements `ShootingDiff::apply` / `apply_identified_delta` (🔺️diff/📝️text/🦀️component.rs)
in Python and asserts `apply(committed_diff, before) == after` for all 31 cases. This is a
transcription check on the committed JSON, NOT a claim that the Rust tests pass.
"""
import copy
import json
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

ASSET_PATCH = {"name": "name", "url": "url", "origin": "origin", "orientation": "orientation", "scale": "scale"}
SHOT_PATCH = {"label": "label", "width": "width", "height": "height", "format": "format", "shape": "shape"}
CAMERA_PATCH = {"label": "label", "camera": "camera"}


def apply_patch(item, patch, slots):
    for slot, field in slots.items():
        value = patch.get(slot)
        if value is not None:
            item[field] = copy.deepcopy(value)


def apply_delta(items, delta, slots):
    nxt = copy.deepcopy(items)
    for rid in delta.get("removed") or []:
        pos = next(i for i, e in enumerate(nxt) if e["id"] == rid)
        nxt.pop(pos)
    for added in delta.get("added") or []:
        assert not any(e["id"] == added["id"] for e in nxt), added["id"]
        nxt.append(copy.deepcopy(added))
    for entry in delta.get("patched") or []:
        item = next(e for e in nxt if e["id"] == entry["id"])
        apply_patch(item, entry["patch"], slots)
    order = delta.get("reordered")
    if order is not None:
        assert len(order) == len(nxt), (order, [e["id"] for e in nxt])
        nxt = [next(e for e in nxt if e["id"] == oid) for oid in order]
    return nxt


def apply_diff(before, diff):
    nxt = copy.deepcopy(before)
    if diff.get("schema") is not None:
        nxt["schema"] = diff["schema"]
    if diff.get("assets") is not None:
        nxt["assets"] = apply_delta(nxt["assets"], diff["assets"], ASSET_PATCH)
    if diff.get("savedCameras") is not None:
        nxt["savedCameras"] = apply_delta(nxt["savedCameras"], diff["savedCameras"], CAMERA_PATCH)
    if diff.get("scene") is not None:
        nxt["scene"] = copy.deepcopy(diff["scene"])
    if diff.get("shots") is not None:
        nxt["shots"] = apply_delta(nxt["shots"], diff["shots"], SHOT_PATCH)
    if diff.get("activeShotId") is not None:
        nxt["activeShotId"] = diff["activeShotId"]
    if diff.get("activeAssetId") is not None:
        nxt["activeAssetId"] = diff["activeAssetId"]
    return nxt


DIFF_FIELDS = ["artifact", "schema", "assets", "savedCameras", "scene", "shots", "activeShotId", "activeAssetId",
               "emblem", "selectedShotIds", "activeUtilityId", "defaultShotFormat", "defaultShotShape",
               "defaultAssetFormat", "centerModel", "fitRevision", "cameraDraftLabel", "camera", "locale"]

failures = []
checked = 0
for leaf in sorted(os.listdir(ROOT)):
    tests = os.path.join(ROOT, leaf, "🧪️tests")
    if not os.path.isdir(tests):
        continue
    for case in sorted(os.listdir(tests)):
        d = os.path.join(tests, case)
        load = lambda rel: json.load(open(os.path.join(d, rel), encoding="utf-8"))
        before, after, diff = load("📸️snapshot/⬅️before/🔣️component.json"), load("📸️snapshot/➡️after/🔣️component.json"), load("🔺️diff/🔣️component.json")
        checked += 1
        if list(diff.keys()) != DIFF_FIELDS:
            failures.append((leaf, case, "diff field set/order mismatch"))
        for name, delta in (("assets", diff["assets"]), ("savedCameras", diff["savedCameras"]), ("shots", diff["shots"])):
            if delta is not None and list(delta.keys()) != ["added", "removed", "patched", "reordered"]:
                failures.append((leaf, case, f"{name} delta field set mismatch"))
            if delta is None:
                continue
            slots = {"assets": ASSET_PATCH, "savedCameras": CAMERA_PATCH, "shots": SHOT_PATCH}[name]
            for entry in delta["patched"]:
                if list(entry.keys()) != ["id", "patch"]:
                    failures.append((leaf, case, f"{name} patch entry field set mismatch"))
                if list(entry["patch"].keys()) != list(slots.keys()):
                    failures.append((leaf, case, f"{name} patch slot set mismatch: {list(entry['patch'].keys())}"))
        if sum(1 for f in DIFF_FIELDS if diff[f] is not None) == 0:
            failures.append((leaf, case, "diff sets nothing"))
        produced = apply_diff(before, diff)
        if produced != after:
            failures.append((leaf, case, "apply(diff, before) != after"))

print(f"checked {checked} cases")
print("failures:", failures if failures else "none")
