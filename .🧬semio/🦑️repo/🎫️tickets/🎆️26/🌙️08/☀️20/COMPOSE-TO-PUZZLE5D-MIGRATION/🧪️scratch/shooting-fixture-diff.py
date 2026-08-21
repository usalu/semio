#!/usr/bin/env python3
"""Authoring aid for the 🎥️shooting `🔺️diff/🔣️component.json` files.

Not a harness: each entry below transcribes, by hand, exactly what that mutation's own
`🔺️diff/🦀️component.rs` constructs — nothing is derived by re-running the Rust builder.

`ShootingDiff` is `#[serde(rename_all = "camelCase", default)]` with NO `skip_serializing_if` on any
field, so serde emits all 19 fields, `null` for the untouched ones. The three collection deltas are
likewise `#[serde(rename_all = "camelCase", default)]` with no skips (`added`/`removed`/`patched`
always arrays, `reordered` null when unset), and the three `*Patch` records are
`#[serde(rename_all = "camelCase")]` with no skips either (every patch slot present, null when
untouched).
"""
import copy
import json
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

DIFF_FIELDS = [
    "artifact", "schema", "assets", "savedCameras", "scene", "shots", "activeShotId", "activeAssetId",
    "emblem", "selectedShotIds", "activeUtilityId", "defaultShotFormat", "defaultShotShape",
    "defaultAssetFormat", "centerModel", "fitRevision", "cameraDraftLabel", "camera", "locale",
]


def diff(**set_fields):
    out = {name: None for name in DIFF_FIELDS}
    for name, value in set_fields.items():
        assert name in out, name
        out[name] = value
    return out


def delta(added=None, removed=None, patched=None, reordered=None):
    return {"added": added or [], "removed": removed or [], "patched": patched or [], "reordered": reordered}


def asset_patch(name=None, url=None, origin=None, orientation=None, scale=None):
    return {"name": name, "url": url, "origin": origin, "orientation": orientation, "scale": scale}


def shot_patch(label=None, width=None, height=None, format=None, shape=None):
    return {"label": label, "width": width, "height": height, "format": format, "shape": shape}


def camera_patch(label=None, camera=None):
    return {"label": label, "camera": camera}


NEW_ASSET = {"id": "asset-detail", "name": "Detail", "url": "/mesh/detail.glb", "format": "glb", "origin": [0.0, 4.0, 0.0], "orientation": None, "scale": None}
NEW_SHOT = {"id": "shot-macro", "label": "Macro", "width": 128, "height": 128, "format": "png", "shape": "rectangle"}
NEW_CAMERA = {"id": "cam-top", "label": "Top", "camera": {"position": [0.0, 0.0, 20.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0, "fov": 50.0}}
SHOT_WIDE_POSE = {"position": [3.0, -3.0, 2.0], "target": [0.0, 0.0, 0.5], "zoom": 1.5, "fov": 40.0}
CAM_CLOSE_POSE = {"position": [1.0, -1.0, 0.75], "target": [0.0, 0.0, 1.0], "zoom": 4.0, "fov": 20.0}

DIFFS = {
    # 📦️ assets — every one of these touches `assets` and leaves the other 18 diff slots null.
    ("🌱️create-asset", "appends-asset-detail"): diff(assets=delta(added=[NEW_ASSET])),
    ("🗑️delete-asset", "removes-trailing-asset-prop"): diff(assets=delta(removed=["asset-prop"])),
    ("✏️rename-asset", "renames-asset-hero-to-lead"): diff(assets=delta(patched=[{"id": "asset-hero", "patch": asset_patch(name="Lead")}])),
    ("🔗️change-asset-url", "points-asset-prop-at-v2-mesh"): diff(assets=delta(patched=[{"id": "asset-prop", "patch": asset_patch(url="/mesh/prop-v2.glb")}])),
    ("🔀️reorder-assets", "moves-asset-hero-behind-asset-prop"): diff(assets=delta(reordered=["asset-prop", "asset-hero"])),
    ("↔️drag-assets", "offsets-both-assets-and-skips-a-ghost"): diff(assets=delta(patched=[
        {"id": "asset-hero", "patch": asset_patch(origin=[5.0, 1.0, 3.5])},
        {"id": "asset-prop", "patch": asset_patch(origin=[4.0, -1.0, 0.5])},
    ])),
    ("🔄️rotate-assets", "spins-asset-hero-about-z"): diff(assets=delta(patched=[
        {"id": "asset-hero", "patch": asset_patch(orientation=[0.0, 0.0, 0.6816387600233341, 0.7316888688738209])},
    ])),
    ("↕️scale-assets", "doubles-asset-hero-scale"): diff(assets=delta(patched=[
        {"id": "asset-hero", "patch": asset_patch(scale=[4.0, 4.0, 4.0])},
    ])),
    # 📸️ shots
    ("📸️create-shot", "appends-shot-macro"): diff(shots=delta(added=[NEW_SHOT])),
    ("🚮️delete-shot", "removes-trailing-shot-close"): diff(shots=delta(removed=["shot-close"])),
    ("🏷️rename-shot", "relabels-shot-close-to-detail"): diff(shots=delta(patched=[{"id": "shot-close", "patch": shot_patch(label="Detail")}])),
    ("📏️change-shot-width", "widens-shot-close-to-1024"): diff(shots=delta(patched=[{"id": "shot-close", "patch": shot_patch(width=1024)}])),
    ("📐️change-shot-height", "heightens-shot-close-to-768"): diff(shots=delta(patched=[{"id": "shot-close", "patch": shot_patch(height=768)}])),
    ("🖼️change-shot-format", "switches-shot-wide-to-svg"): diff(shots=delta(patched=[{"id": "shot-wide", "patch": shot_patch(format="svg")}])),
    ("✂️change-shot-shape", "rounds-shot-wide-to-ellipse"): diff(shots=delta(patched=[{"id": "shot-wide", "patch": shot_patch(shape="ellipse")}])),
    ("🔃️reorder-shots", "moves-shot-close-to-front"): diff(shots=delta(reordered=["shot-close", "shot-wide"])),
    # 🎥️ saved cameras — note `replace-shot-camera` is addressed by SHOT id but writes `savedCameras`.
    ("📷️replace-shot-camera", "rewrites-cam-wide-through-shot-wide"): diff(savedCameras=delta(patched=[
        {"id": "cam-wide", "patch": camera_patch(camera=SHOT_WIDE_POSE)},
    ])),
    ("🎥️create-saved-camera", "appends-saved-camera-top"): diff(savedCameras=delta(added=[NEW_CAMERA])),
    ("🧹️delete-saved-camera", "removes-trailing-cam-close"): diff(savedCameras=delta(removed=["cam-close"])),
    ("🪪️rename-saved-camera", "relabels-cam-close-to-tight"): diff(savedCameras=delta(patched=[
        {"id": "cam-close", "patch": camera_patch(label="Tight")},
    ])),
    ("🎞️replace-saved-camera-view", "repositions-cam-close-view"): diff(savedCameras=delta(patched=[
        {"id": "cam-close", "patch": camera_patch(camera=CAM_CLOSE_POSE)},
    ])),
    ("🔁️reorder-saved-cameras", "moves-cam-close-to-front"): diff(savedCameras=delta(reordered=["cam-close", "cam-wide"])),
    # 🎯️ active selection — bare document-root scalars, no collection delta at all.
    ("🎯️set-active-shot", "activates-shot-close"): diff(activeShotId="shot-close"),
    ("📌️set-active-asset", "activates-asset-prop"): diff(activeAssetId="asset-prop"),
}

# ☀️ scene — every scene leaf clones the WHOLE `ShootingSceneLighting`, edits one field of the clone
# and ships it as `scene`. The committed diff's `scene` is therefore byte-identical to the after
# snapshot's `scene`, which is exactly the coarseness this file is meant to expose.
SCENE_CASES = [
    ("☀️change-scene-sun-enabled", "switches-scene-sun-off"),
    ("🧭️change-scene-sun-azimuth", "turns-scene-sun-to-315-degrees"),
    ("🌅️change-scene-sun-elevation", "raises-scene-sun-to-60-degrees"),
    ("💡️change-scene-sun-intensity", "dims-scene-sun-to-half"),
    ("🔅️change-scene-ambient-intensity", "dims-scene-ambient-to-quarter"),
    ("🌑️change-scene-shadow-enabled", "switches-scene-shadows-off"),
    ("🪨️change-scene-material-roughness", "polishes-scene-material-to-quarter"),
]


def main():
    for leaf, case in SCENE_CASES:
        after = json.load(open(os.path.join(ROOT, leaf, "🧪️tests", case, "📸️snapshot/➡️after/🔣️component.json"), encoding="utf-8"))
        DIFFS[(leaf, case)] = diff(scene=copy.deepcopy(after["scene"]))

    assert len(DIFFS) == 31, len(DIFFS)
    for (leaf, case), payload in DIFFS.items():
        path = os.path.join(ROOT, leaf, "🧪️tests", case, "🔺️diff/🔣️component.json")
        assert os.path.isdir(os.path.dirname(os.path.dirname(path))), path
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(payload, handle, indent=2, ensure_ascii=False)
            handle.write("\n")
    print(f"wrote {len(DIFFS)} diff files")


if __name__ == "__main__":
    main()
