#!/usr/bin/env python3
"""Authoring aid for the 🎥️shooting mutation fixtures.

Not a test harness: every case below carries its own hand-derived `after` edit, read straight off
that mutation's `🔺️diff/🦀️component.rs`. This file only writes the bytes out with stable formatting.
"""
import copy
import json
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

BASE = {
    "schema": "shooting.shooting",
    "assets": [
        {
            "id": "asset-hero",
            "name": "Hero",
            "url": "/mesh/hero.glb",
            "format": "glb",
            "origin": [1.0, 2.0, 3.0],
            "orientation": [0.0, 0.0, 0.0, 1.0],
            "scale": [2.0, 2.0, 2.0],
        },
        {
            "id": "asset-prop",
            "name": "Prop",
            "url": "/mesh/prop.glb",
            "format": "glb",
            "origin": [0.0, 0.0, 0.0],
            "orientation": None,
            "scale": None,
        },
    ],
    "savedCameras": [
        {"id": "cam-wide", "label": "Wide", "camera": {"position": [10.0, -10.0, 6.0], "target": [0.0, 0.0, 1.0], "zoom": 1.0, "fov": 50.0}},
        {"id": "cam-close", "label": "Close", "camera": {"position": [2.0, -2.0, 1.5], "target": [0.0, 0.0, 1.0], "zoom": 2.0, "fov": 35.0}},
    ],
    "scene": {
        "background": "#101014",
        "sun": {"enabled": True, "azimuth": 45.0, "elevation": 35.0, "intensity": 2.4, "color": "#ffffff"},
        "ambient": {"intensity": 1.15, "color": "#ffffff"},
        "shadow": {"enabled": True, "opacity": 0.35, "softness": 1.0},
        "material": {"color": "#9aa0ab", "metalness": 0.0, "roughness": 1.0, "emissive": "#000000", "emissiveIntensity": 0.0},
    },
    "shots": [
        {"id": "shot-wide", "label": "Wide", "width": 512, "height": 512, "format": "png", "shape": "rectangle", "background": "#ffffff", "cameraId": "cam-wide"},
        {"id": "shot-close", "label": "Close", "width": 256, "height": 256, "format": "svg", "shape": "ellipse"},
    ],
    "activeShotId": "shot-wide",
    "activeAssetId": "asset-hero",
}

APPLIED = {"status": "applied"}


def base():
    return copy.deepcopy(BASE)


CASES = []


def case(leaf, name, mutation, after, outcome=None):
    CASES.append((leaf, name, mutation, after, outcome or APPLIED))


# ── assets ────────────────────────────────────────────────────────────────────────────────────────
a = base()
a["assets"].append({"id": "asset-detail", "name": "Detail", "url": "/mesh/detail.glb", "format": "glb", "origin": [0.0, 4.0, 0.0], "orientation": None, "scale": None})
case("🌱️create-asset", "appends-asset-detail",
     {"mutation": "createAsset", "asset": {"id": "asset-detail", "name": "Detail", "url": "/mesh/detail.glb", "format": "glb", "origin": [0.0, 4.0, 0.0], "orientation": None, "scale": None}, "index": 0}, a)

a = base()
del a["assets"][1]
case("🗑️delete-asset", "removes-trailing-asset-prop", {"mutation": "deleteAsset", "id": "asset-prop"}, a)

a = base()
a["assets"][0]["name"] = "Lead"
case("✏️rename-asset", "renames-asset-hero-to-lead", {"mutation": "renameAsset", "id": "asset-hero", "new_name": "Lead"}, a)

a = base()
a["assets"][1]["url"] = "/mesh/prop-v2.glb"
case("🔗️change-asset-url", "points-asset-prop-at-v2-mesh", {"mutation": "changeAssetUrl", "id": "asset-prop", "new_url": "/mesh/prop-v2.glb"}, a)

a = base()
a["assets"] = [a["assets"][1], a["assets"][0]]
case("🔀️reorder-assets", "moves-asset-hero-behind-asset-prop", {"mutation": "reorderAssets", "id": "asset-hero", "to_index": 1}, a)

a = base()
a["assets"][0]["origin"] = [5.0, 1.0, 3.5]
a["assets"][1]["origin"] = [4.0, -1.0, 0.5]
case("↔️drag-assets", "offsets-both-assets-and-skips-a-ghost",
     {"mutation": "dragAssets", "asset_ids": ["asset-hero", "asset-prop", "asset-ghost"], "dx": 4.0, "dy": -1.0, "dz": 0.5}, a,
     {"status": "applied", "messages": [{"level": "warn", "code": "mutation.partial"}]})

a = base()
a["assets"][0]["orientation"] = [0.0, 0.0, 0.6816387600233341, 0.7316888688738209]
case("🔄️rotate-assets", "spins-asset-hero-about-z",
     {"mutation": "rotateAssets", "asset_ids": ["asset-hero"], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.5}, a)

a = base()
a["assets"][0]["scale"] = [4.0, 4.0, 4.0]
case("↕️scale-assets", "doubles-asset-hero-scale",
     {"mutation": "scaleAssets", "asset_ids": ["asset-hero"], "sx": 2.0, "sy": 2.0, "sz": 2.0}, a)

# ── shots ─────────────────────────────────────────────────────────────────────────────────────────
a = base()
a["shots"].append({"id": "shot-macro", "label": "Macro", "width": 128, "height": 128, "format": "png", "shape": "rectangle"})
case("📸️create-shot", "appends-shot-macro",
     {"mutation": "createShot", "shot": {"id": "shot-macro", "label": "Macro", "width": 128, "height": 128, "format": "png", "shape": "rectangle"}, "index": None}, a)

a = base()
del a["shots"][1]
case("🚮️delete-shot", "removes-trailing-shot-close", {"mutation": "deleteShot", "id": "shot-close"}, a)

a = base()
a["shots"][1]["label"] = "Detail"
case("🏷️rename-shot", "relabels-shot-close-to-detail", {"mutation": "renameShot", "id": "shot-close", "new_label": "Detail"}, a)

a = base()
a["shots"][1]["width"] = 1024
case("📏️change-shot-width", "widens-shot-close-to-1024", {"mutation": "changeShotWidth", "id": "shot-close", "new_width": 1024}, a)

a = base()
a["shots"][1]["height"] = 768
case("📐️change-shot-height", "heightens-shot-close-to-768", {"mutation": "changeShotHeight", "id": "shot-close", "new_height": 768}, a)

a = base()
a["shots"][0]["format"] = "svg"
case("🖼️change-shot-format", "switches-shot-wide-to-svg", {"mutation": "changeShotFormat", "id": "shot-wide", "new_format": "svg"}, a)

a = base()
a["shots"][0]["shape"] = "ellipse"
case("✂️change-shot-shape", "rounds-shot-wide-to-ellipse", {"mutation": "changeShotShape", "id": "shot-wide", "new_shape": "ellipse"}, a)

a = base()
a["shots"] = [a["shots"][1], a["shots"][0]]
case("🔃️reorder-shots", "moves-shot-close-to-front", {"mutation": "reorderShots", "id": "shot-close", "to_index": 0}, a)

# ── cameras ───────────────────────────────────────────────────────────────────────────────────────
a = base()
a["savedCameras"][0]["camera"] = {"position": [3.0, -3.0, 2.0], "target": [0.0, 0.0, 0.5], "zoom": 1.5, "fov": 40.0}
case("📷️replace-shot-camera", "rewrites-cam-wide-through-shot-wide",
     {"mutation": "replaceShotCamera", "shot_id": "shot-wide", "new_camera": {"position": [3.0, -3.0, 2.0], "target": [0.0, 0.0, 0.5], "zoom": 1.5, "fov": 40.0}}, a)

a = base()
a["savedCameras"].append({"id": "cam-top", "label": "Top", "camera": {"position": [0.0, 0.0, 20.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0, "fov": 50.0}})
case("🎥️create-saved-camera", "appends-saved-camera-top",
     {"mutation": "createSavedCamera", "saved_camera": {"id": "cam-top", "label": "Top", "camera": {"position": [0.0, 0.0, 20.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0, "fov": 50.0}}, "index": None}, a)

a = base()
del a["savedCameras"][1]
case("🧹️delete-saved-camera", "removes-trailing-cam-close", {"mutation": "deleteSavedCamera", "id": "cam-close"}, a)

a = base()
a["savedCameras"][1]["label"] = "Tight"
case("🪪️rename-saved-camera", "relabels-cam-close-to-tight", {"mutation": "renameSavedCamera", "id": "cam-close", "new_label": "Tight"}, a)

a = base()
a["savedCameras"][1]["camera"] = {"position": [1.0, -1.0, 0.75], "target": [0.0, 0.0, 1.0], "zoom": 4.0, "fov": 20.0}
case("🎞️replace-saved-camera-view", "repositions-cam-close-view",
     {"mutation": "replaceSavedCameraView", "id": "cam-close", "new_camera": {"position": [1.0, -1.0, 0.75], "target": [0.0, 0.0, 1.0], "zoom": 4.0, "fov": 20.0}}, a)

a = base()
a["savedCameras"] = [a["savedCameras"][1], a["savedCameras"][0]]
case("🔁️reorder-saved-cameras", "moves-cam-close-to-front", {"mutation": "reorderSavedCameras", "id": "cam-close", "to_index": 0}, a)

# ── active selection ──────────────────────────────────────────────────────────────────────────────
a = base()
a["activeShotId"] = "shot-close"
case("🎯️set-active-shot", "activates-shot-close", {"mutation": "setActiveShot", "shot_id": "shot-close"}, a)

a = base()
a["activeAssetId"] = "asset-prop"
case("📌️set-active-asset", "activates-asset-prop", {"mutation": "setActiveAsset", "asset_id": "asset-prop"}, a)

# ── scene ─────────────────────────────────────────────────────────────────────────────────────────
a = base()
a["scene"]["sun"]["enabled"] = False
case("☀️change-scene-sun-enabled", "switches-scene-sun-off", {"mutation": "changeSceneSunEnabled", "new_enabled": False}, a)

a = base()
a["scene"]["sun"]["azimuth"] = 315.0
case("🧭️change-scene-sun-azimuth", "turns-scene-sun-to-315-degrees", {"mutation": "changeSceneSunAzimuth", "new_azimuth": 315.0}, a)

a = base()
a["scene"]["sun"]["elevation"] = 60.0
case("🌅️change-scene-sun-elevation", "raises-scene-sun-to-60-degrees", {"mutation": "changeSceneSunElevation", "new_elevation": 60.0}, a)

a = base()
a["scene"]["sun"]["intensity"] = 1.2
case("💡️change-scene-sun-intensity", "dims-scene-sun-to-half", {"mutation": "changeSceneSunIntensity", "new_intensity": 1.2}, a)

a = base()
a["scene"]["ambient"]["intensity"] = 0.25
case("🔅️change-scene-ambient-intensity", "dims-scene-ambient-to-quarter", {"mutation": "changeSceneAmbientIntensity", "new_intensity": 0.25}, a)

a = base()
a["scene"]["shadow"]["enabled"] = False
case("🌑️change-scene-shadow-enabled", "switches-scene-shadows-off", {"mutation": "changeSceneShadowEnabled", "new_enabled": False}, a)

a = base()
a["scene"]["material"]["roughness"] = 0.25
case("🪨️change-scene-material-roughness", "polishes-scene-material-to-quarter", {"mutation": "changeSceneMaterialRoughness", "new_roughness": 0.25}, a)


def write(path, payload):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2, ensure_ascii=False)
        handle.write("\n")


def main():
    leaves = {name for name in os.listdir(ROOT) if os.path.isdir(os.path.join(ROOT, name))}
    seen = set()
    for leaf, name, mutation, after, outcome in CASES:
        assert leaf in leaves, f"unknown leaf {leaf!r}"
        seen.add(leaf)
        root = os.path.join(ROOT, leaf, "🧪️tests", name)
        write(os.path.join(root, "📸️snapshot/⬅️before/🔣️component.json"), BASE)
        write(os.path.join(root, "📸️snapshot/➡️after/🔣️component.json"), after)
        write(os.path.join(root, "🦠️mutation/🔣️component.json"), mutation)
        write(os.path.join(root, "🎯️outcome/🔣️component.json"), outcome)
    missing = {leaf for leaf in leaves if os.path.exists(os.path.join(ROOT, leaf, "🦠️mutation/🦀️component.rs"))} - seen
    assert not missing, f"leaves without a case: {sorted(missing)}"
    print(f"wrote {len(CASES)} cases")


if __name__ == "__main__":
    main()
