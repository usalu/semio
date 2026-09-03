"""🐍️ `s.shooting.shooting`'s second, independent implementation of its own 31-kind mutation
vocabulary.

`s.shooting.shooting` is a semio-NATIVE render-scene document — its `shooting.shooting.dsl` grammar,
with its typed table columns and `deg`-suffixed angle literals, is defined by this repository alone.
This subset's own no-oracle decision (`shooting-render-scene-mutation-semantics`) argues rather than
assumes the third-party negative: glTF 2.0, USD and Collada are named and declined on the one
structural point that matters — none of them models a SHOT, and eleven of the thirty-one kinds
address one. The reference is therefore a second IMPLEMENTATION, written from this subset's own
committed `../../🧬️schema/📸️snapshot/🔣️.json` document shape, each mutation's own committed
`(mutation, after)` leaf fixture (all thirty-one leaves share the SAME committed before-document,
confirmed byte-identical across all thirty-one copies by this feature's own description — SHA-1
`6441b72754e5c649b2b07a2f2b244313467f85a0`), and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
verb table. It imports nothing from the Rust it judges and transliterates none of it.

📐 Two conventions this vocabulary uses that are NOT read off the Rust implementation, because they
are forced by the committed vectors themselves or by the payload schemas' own declared shape, not by
an arbitrary code choice:
  - `create-<singular>{..., index}`: EVERY committed `create-*` vector's `index` field disagrees with
    where its own committed after-document actually places the new member (`create-asset`'s vector
    carries `index: 0` but the new asset lands LAST, at position 2 of 2 existing) — the only reading
    consistent with all three `create-*` vectors at once is APPEND-ONLY, `index` recorded but not
    acted on. `delete-*`'s inverse therefore also always appends (this is why every committed
    `delete-*` vector removes the TRAILING member — an append-only re-creation can only land back on
    the original position when that position was last).
  - `replace-shot-camera{shot_id, new_camera}` patches the SAVED CAMERA the named shot's `cameraId`
    resolves to, not the shot itself — read off the one committed vector, where the diff lands on
    `savedCameras[0]` (`cam-wide`, `shot-wide`'s own `cameraId`) rather than on `shots[0]`, and
    confirmed by the schema: `ReplaceShotCamera` carries no camera-VALUE field on the shot at all,
    only `shots[].cameraId`, a reference.

🧭 `rotate-assets`' quaternion composition and `scale-assets`' scale composition are ordinary
mathematics with no implementation freedom (Hamilton product of unit quaternions in `[x, y, z, w]`
order; component-wise scale multiply) — this reference derives them from that standard definition,
not from the Rust arithmetic. The one non-mathematical convention — which operand is composed on
which side (new orientation = axis-angle delta multiplied on the LEFT of the current orientation) —
is a real implementation choice this codebase makes, read off `apply_shooting_mutation`'s own
mathematics documentation, not invented; it is associative with its own inverse regardless of which
side is chosen, so the one committed vector (which starts from an identity orientation) cannot by
itself distinguish the two conventions, and this note records that rather than concealing it. `None`
`orientation`/`scale` reads as identity `[0,0,0,1]` / `[1,1,1]` before composing, per each mutation's
own payload schema (`orientation`/`scale` are optional array fields).

🚧 Scope: `drag-assets`/`rotate-assets`/`scale-assets` silently skip an addressed id that does not
exist among `assets` (the committed `drag-assets` vector addresses a THIRD, absent id — `asset-ghost`
— alongside two real ones, and only the two real ones move) — this reference reproduces that
skip-on-miss behaviour for the document transformation, not the `mutation.partial`/`mutation.
target-missing` diagnostic codes those same paths also raise, which are outside this reference's
claim (it is compared on the projected DOCUMENT, not on diagnostics).
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"

# 📸️ The ONE before-document all thirty-one leaves share (confirmed byte-identical across all
# thirty-one copies by this feature's own description) — read once, from where the domain already
# keeps it, rather than a thirty-second copy.
_BASE_URI = f"{_ROOT}/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️.json"

# 🗺️ kind -> (dir, fixture, wire tag). The wire tag is the `"mutation"` field's own committed value.
VECTORS = {
    "create-asset": ("🌱️create-asset", "appends-asset-detail", "createAsset"),
    "delete-asset": ("🗑️delete-asset", "removes-trailing-asset-prop", "deleteAsset"),
    "rename-asset": ("✏️rename-asset", "renames-asset-hero-to-lead", "renameAsset"),
    "change-asset-url": ("🔗️change-asset-url", "points-asset-prop-at-v2-mesh", "changeAssetUrl"),
    "reorder-assets": ("🔀️reorder-assets", "moves-asset-hero-behind-asset-prop", "reorderAssets"),
    "drag-assets": ("↔️drag-assets", "offsets-both-assets-and-skips-a-ghost", "dragAssets"),
    "rotate-assets": ("🔄️rotate-assets", "spins-asset-hero-about-z", "rotateAssets"),
    "scale-assets": ("↕️scale-assets", "doubles-asset-hero-scale", "scaleAssets"),
    "create-shot": ("📸️create-shot", "appends-shot-macro", "createShot"),
    "delete-shot": ("🚮️delete-shot", "removes-trailing-shot-close", "deleteShot"),
    "rename-shot": ("🏷️rename-shot", "relabels-shot-close-to-detail", "renameShot"),
    "change-shot-width": ("📏️change-shot-width", "widens-shot-close-to-1024", "changeShotWidth"),
    "change-shot-height": ("📐️change-shot-height", "heightens-shot-close-to-768", "changeShotHeight"),
    "change-shot-format": ("🖼️change-shot-format", "switches-shot-wide-to-svg", "changeShotFormat"),
    "change-shot-shape": ("✂️change-shot-shape", "rounds-shot-wide-to-ellipse", "changeShotShape"),
    "reorder-shots": ("🔃️reorder-shots", "moves-shot-close-to-front", "reorderShots"),
    "replace-shot-camera": ("📷️replace-shot-camera", "rewrites-cam-wide-through-shot-wide", "replaceShotCamera"),
    "create-saved-camera": ("🎥️create-saved-camera", "appends-saved-camera-top", "createSavedCamera"),
    "delete-saved-camera": ("🧹️delete-saved-camera", "removes-trailing-cam-close", "deleteSavedCamera"),
    "rename-saved-camera": ("🪪️rename-saved-camera", "relabels-cam-close-to-tight", "renameSavedCamera"),
    "replace-saved-camera-view": ("🎞️replace-saved-camera-view", "repositions-cam-close-view", "replaceSavedCameraView"),
    "reorder-saved-cameras": ("🔁️reorder-saved-cameras", "moves-cam-close-to-front", "reorderSavedCameras"),
    "set-active-shot": ("🎯️set-active-shot", "activates-shot-close", "setActiveShot"),
    "set-active-asset": ("📌️set-active-asset", "activates-asset-prop", "setActiveAsset"),
    "change-scene-sun-enabled": ("☀️change-scene-sun-enabled", "switches-scene-sun-off", "changeSceneSunEnabled"),
    "change-scene-sun-azimuth": ("🧭️change-scene-sun-azimuth", "turns-scene-sun-to-315-degrees", "changeSceneSunAzimuth"),
    "change-scene-sun-elevation": ("🌅️change-scene-sun-elevation", "raises-scene-sun-to-60-degrees", "changeSceneSunElevation"),
    "change-scene-sun-intensity": ("💡️change-scene-sun-intensity", "dims-scene-sun-to-half", "changeSceneSunIntensity"),
    "change-scene-ambient-intensity": ("🔅️change-scene-ambient-intensity", "dims-scene-ambient-to-quarter", "changeSceneAmbientIntensity"),
    "change-scene-shadow-enabled": ("🌑️change-scene-shadow-enabled", "switches-scene-shadows-off", "changeSceneShadowEnabled"),
    "change-scene-material-roughness": ("🪨️change-scene-material-roughness", "polishes-scene-material-to-quarter", "changeSceneMaterialRoughness"),
}

WIRE_TAG_TO_KIND = {tag: kind for kind, (_dir, _fixture, tag) in VECTORS.items()}


def _leaf_root(kind: str) -> str:
    dirname, fixture, _tag = VECTORS[kind]
    return f"{_ROOT}/{dirname}/🧪️tests/{fixture}"


def _read_json(ctx: Context, uri: str):
    return json.loads(ctx.fixture_bytes(uri))


def unwrap(wire):
    """📨 The internally-tagged form every committed vector uses: `{"mutation": "<tag>", ...fields}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Fixtures


# region 🔖️Addressing
def _find(items, item_id, key="id"):
    for index, item in enumerate(items):
        if item.get(key) == item_id:
            return index, item
    raise AssertionError(f"no member with {key}={item_id!r} among {[i.get(key) for i in items]!r}")
# endregion 🔖️Addressing


# region 🔖️Rotation / scale math — standard definitions, not read off the Rust arithmetic
def quat_from_axis_angle(ax, ay, az, angle):
    """🧭 Unit quaternion for a rotation of `angle` radians about `(ax, ay, az)`, `[x, y, z, w]`."""
    length = math.sqrt(ax * ax + ay * ay + az * az)
    if length < 1e-8:
        return [0.0, 0.0, 0.0, 1.0]
    half = angle * 0.5
    s = math.sin(half)
    return [ax / length * s, ay / length * s, az / length * s, math.cos(half)]


def quat_mul(a, b):
    """🧭 Hamilton product `a * b`, both `[x, y, z, w]` — the standard quaternion composition."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
        aw * bw - ax * bx - ay * by - az * bz,
    ]


def _reciprocal(value):
    return 1.0 if abs(value) < 1e-8 else 1.0 / value
# endregion 🔖️Rotation / scale math


# region 🔖️Vocabulary — forward appliers
def apply_create_asset(doc, p):
    after = copy.deepcopy(doc)
    after["assets"].append(copy.deepcopy(p["asset"]))
    return after


def apply_delete_asset(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["assets"], p["id"])
    after["assets"].pop(idx)
    return after


def apply_rename_asset(doc, p):
    after = copy.deepcopy(doc)
    _, a = _find(after["assets"], p["id"])
    a["name"] = p["new_name"]
    return after


def apply_change_asset_url(doc, p):
    after = copy.deepcopy(doc)
    _, a = _find(after["assets"], p["id"])
    a["url"] = p["new_url"]
    return after


def apply_reorder_assets(doc, p):
    after = copy.deepcopy(doc)
    idx, item = _find(after["assets"], p["id"])
    after["assets"].pop(idx)
    after["assets"].insert(min(p["to_index"], len(after["assets"])), item)
    return after


def apply_drag_assets(doc, p):
    after = copy.deepcopy(doc)
    ids = set(p["asset_ids"])
    for a in after["assets"]:
        if a["id"] in ids:
            a["origin"] = [a["origin"][0] + p["dx"], a["origin"][1] + p["dy"], a["origin"][2] + p["dz"]]
    return after


def apply_rotate_assets(doc, p):
    after = copy.deepcopy(doc)
    ids = set(p["asset_ids"])
    delta = quat_from_axis_angle(p["ax"], p["ay"], p["az"], p["angle"])
    for a in after["assets"]:
        if a["id"] in ids:
            current = a.get("orientation") or [0.0, 0.0, 0.0, 1.0]
            a["orientation"] = quat_mul(delta, current)
    return after


def apply_scale_assets(doc, p):
    after = copy.deepcopy(doc)
    ids = set(p["asset_ids"])
    for a in after["assets"]:
        if a["id"] in ids:
            current = a.get("scale") or [1.0, 1.0, 1.0]
            a["scale"] = [current[0] * p["sx"], current[1] * p["sy"], current[2] * p["sz"]]
    return after


def apply_create_shot(doc, p):
    after = copy.deepcopy(doc)
    after["shots"].append(copy.deepcopy(p["shot"]))
    return after


def apply_delete_shot(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["shots"], p["id"])
    after["shots"].pop(idx)
    return after


def apply_rename_shot(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["shots"], p["id"])
    s["label"] = p["new_label"]
    return after


def apply_change_shot_width(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["shots"], p["id"])
    s["width"] = p["new_width"]
    return after


def apply_change_shot_height(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["shots"], p["id"])
    s["height"] = p["new_height"]
    return after


def apply_change_shot_format(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["shots"], p["id"])
    s["format"] = p["new_format"]
    return after


def apply_change_shot_shape(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["shots"], p["id"])
    s["shape"] = p["new_shape"]
    return after


def apply_reorder_shots(doc, p):
    after = copy.deepcopy(doc)
    idx, item = _find(after["shots"], p["id"])
    after["shots"].pop(idx)
    after["shots"].insert(min(p["to_index"], len(after["shots"])), item)
    return after


def apply_replace_shot_camera(doc, p):
    """📷 Patches the SAVED CAMERA the named shot's `cameraId` resolves to — not the shot."""
    after = copy.deepcopy(doc)
    _, shot = _find(after["shots"], p["shot_id"])
    camera_id = shot.get("cameraId")
    assert camera_id, f"shot {p['shot_id']!r} has no saved camera to replace"
    _, cam = _find(after["savedCameras"], camera_id)
    cam["camera"] = copy.deepcopy(p["new_camera"])
    return after


def apply_create_saved_camera(doc, p):
    after = copy.deepcopy(doc)
    after["savedCameras"].append(copy.deepcopy(p["saved_camera"]))
    return after


def apply_delete_saved_camera(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["savedCameras"], p["id"])
    after["savedCameras"].pop(idx)
    return after


def apply_rename_saved_camera(doc, p):
    after = copy.deepcopy(doc)
    _, c = _find(after["savedCameras"], p["id"])
    c["label"] = p["new_label"]
    return after


def apply_replace_saved_camera_view(doc, p):
    after = copy.deepcopy(doc)
    _, c = _find(after["savedCameras"], p["id"])
    c["camera"] = copy.deepcopy(p["new_camera"])
    return after


def apply_reorder_saved_cameras(doc, p):
    after = copy.deepcopy(doc)
    idx, item = _find(after["savedCameras"], p["id"])
    after["savedCameras"].pop(idx)
    after["savedCameras"].insert(min(p["to_index"], len(after["savedCameras"])), item)
    return after


def apply_set_active_shot(doc, p):
    after = copy.deepcopy(doc)
    after["activeShotId"] = p["shot_id"]
    return after


def apply_set_active_asset(doc, p):
    after = copy.deepcopy(doc)
    after["activeAssetId"] = p["asset_id"]
    return after


def apply_change_scene_sun_enabled(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["sun"]["enabled"] = p["new_enabled"]
    return after


def apply_change_scene_sun_azimuth(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["sun"]["azimuth"] = p["new_azimuth"]
    return after


def apply_change_scene_sun_elevation(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["sun"]["elevation"] = p["new_elevation"]
    return after


def apply_change_scene_sun_intensity(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["sun"]["intensity"] = p["new_intensity"]
    return after


def apply_change_scene_ambient_intensity(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["ambient"]["intensity"] = p["new_intensity"]
    return after


def apply_change_scene_shadow_enabled(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["shadow"]["enabled"] = p["new_enabled"]
    return after


def apply_change_scene_material_roughness(doc, p):
    after = copy.deepcopy(doc)
    after["scene"]["material"]["roughness"] = p["new_roughness"]
    return after


APPLIERS = {
    "create-asset": apply_create_asset,
    "delete-asset": apply_delete_asset,
    "rename-asset": apply_rename_asset,
    "change-asset-url": apply_change_asset_url,
    "reorder-assets": apply_reorder_assets,
    "drag-assets": apply_drag_assets,
    "rotate-assets": apply_rotate_assets,
    "scale-assets": apply_scale_assets,
    "create-shot": apply_create_shot,
    "delete-shot": apply_delete_shot,
    "rename-shot": apply_rename_shot,
    "change-shot-width": apply_change_shot_width,
    "change-shot-height": apply_change_shot_height,
    "change-shot-format": apply_change_shot_format,
    "change-shot-shape": apply_change_shot_shape,
    "reorder-shots": apply_reorder_shots,
    "replace-shot-camera": apply_replace_shot_camera,
    "create-saved-camera": apply_create_saved_camera,
    "delete-saved-camera": apply_delete_saved_camera,
    "rename-saved-camera": apply_rename_saved_camera,
    "replace-saved-camera-view": apply_replace_saved_camera_view,
    "reorder-saved-cameras": apply_reorder_saved_cameras,
    "set-active-shot": apply_set_active_shot,
    "set-active-asset": apply_set_active_asset,
    "change-scene-sun-enabled": apply_change_scene_sun_enabled,
    "change-scene-sun-azimuth": apply_change_scene_sun_azimuth,
    "change-scene-sun-elevation": apply_change_scene_sun_elevation,
    "change-scene-sun-intensity": apply_change_scene_sun_intensity,
    "change-scene-ambient-intensity": apply_change_scene_ambient_intensity,
    "change-scene-shadow-enabled": apply_change_scene_shadow_enabled,
    "change-scene-material-roughness": apply_change_scene_material_roughness,
}
# endregion 🔖️Vocabulary — forward appliers


# region 🔖️Vocabulary — inverse rule
def inverse_mutation(kind, base, payload):
    """↩️ Every inverse is computed from BASE — the one shared committed before-document — never
    from the payload. Returns `(wire_tag, inverse_payload)`."""
    if kind == "create-asset":
        return "deleteAsset", {"id": payload["asset"]["id"]}
    if kind == "delete-asset":
        idx, asset = _find(base["assets"], payload["id"])
        return "createAsset", {"asset": asset, "index": idx}
    if kind == "rename-asset":
        _, a = _find(base["assets"], payload["id"])
        return "renameAsset", {"id": payload["id"], "new_name": a["name"]}
    if kind == "change-asset-url":
        _, a = _find(base["assets"], payload["id"])
        return "changeAssetUrl", {"id": payload["id"], "new_url": a["url"]}
    if kind == "reorder-assets":
        idx, _a = _find(base["assets"], payload["id"])
        return "reorderAssets", {"id": payload["id"], "to_index": idx}
    if kind == "drag-assets":
        return "dragAssets", {"asset_ids": payload["asset_ids"], "dx": -payload["dx"], "dy": -payload["dy"], "dz": -payload["dz"]}
    if kind == "rotate-assets":
        return "rotateAssets", {"asset_ids": payload["asset_ids"], "ax": payload["ax"], "ay": payload["ay"], "az": payload["az"], "angle": -payload["angle"]}
    if kind == "scale-assets":
        return "scaleAssets", {"asset_ids": payload["asset_ids"], "sx": _reciprocal(payload["sx"]), "sy": _reciprocal(payload["sy"]), "sz": _reciprocal(payload["sz"])}
    if kind == "create-shot":
        return "deleteShot", {"id": payload["shot"]["id"]}
    if kind == "delete-shot":
        idx, shot = _find(base["shots"], payload["id"])
        return "createShot", {"shot": shot, "index": idx}
    if kind == "rename-shot":
        _, s = _find(base["shots"], payload["id"])
        return "renameShot", {"id": payload["id"], "new_label": s["label"]}
    if kind == "change-shot-width":
        _, s = _find(base["shots"], payload["id"])
        return "changeShotWidth", {"id": payload["id"], "new_width": s["width"]}
    if kind == "change-shot-height":
        _, s = _find(base["shots"], payload["id"])
        return "changeShotHeight", {"id": payload["id"], "new_height": s["height"]}
    if kind == "change-shot-format":
        _, s = _find(base["shots"], payload["id"])
        return "changeShotFormat", {"id": payload["id"], "new_format": s["format"]}
    if kind == "change-shot-shape":
        _, s = _find(base["shots"], payload["id"])
        return "changeShotShape", {"id": payload["id"], "new_shape": s["shape"]}
    if kind == "reorder-shots":
        idx, _s = _find(base["shots"], payload["id"])
        return "reorderShots", {"id": payload["id"], "to_index": idx}
    if kind == "replace-shot-camera":
        _, shot = _find(base["shots"], payload["shot_id"])
        _, cam = _find(base["savedCameras"], shot["cameraId"])
        return "replaceShotCamera", {"shot_id": payload["shot_id"], "new_camera": cam["camera"]}
    if kind == "create-saved-camera":
        return "deleteSavedCamera", {"id": payload["saved_camera"]["id"]}
    if kind == "delete-saved-camera":
        idx, cam = _find(base["savedCameras"], payload["id"])
        return "createSavedCamera", {"saved_camera": cam, "index": idx}
    if kind == "rename-saved-camera":
        _, c = _find(base["savedCameras"], payload["id"])
        return "renameSavedCamera", {"id": payload["id"], "new_label": c["label"]}
    if kind == "replace-saved-camera-view":
        _, c = _find(base["savedCameras"], payload["id"])
        return "replaceSavedCameraView", {"id": payload["id"], "new_camera": c["camera"]}
    if kind == "reorder-saved-cameras":
        idx, _c = _find(base["savedCameras"], payload["id"])
        return "reorderSavedCameras", {"id": payload["id"], "to_index": idx}
    if kind == "set-active-shot":
        return "setActiveShot", {"shot_id": base["activeShotId"]}
    if kind == "set-active-asset":
        return "setActiveAsset", {"asset_id": base["activeAssetId"]}
    if kind == "change-scene-sun-enabled":
        return "changeSceneSunEnabled", {"new_enabled": base["scene"]["sun"]["enabled"]}
    if kind == "change-scene-sun-azimuth":
        return "changeSceneSunAzimuth", {"new_azimuth": base["scene"]["sun"]["azimuth"]}
    if kind == "change-scene-sun-elevation":
        return "changeSceneSunElevation", {"new_elevation": base["scene"]["sun"]["elevation"]}
    if kind == "change-scene-sun-intensity":
        return "changeSceneSunIntensity", {"new_intensity": base["scene"]["sun"]["intensity"]}
    if kind == "change-scene-ambient-intensity":
        return "changeSceneAmbientIntensity", {"new_intensity": base["scene"]["ambient"]["intensity"]}
    if kind == "change-scene-shadow-enabled":
        return "changeSceneShadowEnabled", {"new_enabled": base["scene"]["shadow"]["enabled"]}
    if kind == "change-scene-material-roughness":
        return "changeSceneMaterialRoughness", {"new_roughness": base["scene"]["material"]["roughness"]}
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary — inverse rule


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_json(ctx, _BASE_URI)
        root = _leaf_root(kind)
        _dir, _fixture, wire_tag = VECTORS[kind]
        actual_tag, payload = unwrap(_read_json(ctx, f"{root}/🦠️mutation/🔣️.json"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after = APPLIERS[kind](base, payload)
        expected_after = _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json")
        assert after == expected_after, f"mutate-{kind}: {after} != committed after-document {expected_after}"
        raw = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=raw)

    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_json(ctx, _BASE_URI)
        root = _leaf_root(kind)
        _dir, _fixture, wire_tag = VECTORS[kind]
        actual_tag, payload = unwrap(_read_json(ctx, f"{root}/🦠️mutation/🔣️.json"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        after = APPLIERS[kind](base, payload)
        assert after != base, f"inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"
        inv_tag, inv_payload = inverse_mutation(kind, base, payload)
        inv_kind = WIRE_TAG_TO_KIND[inv_tag]
        restored = APPLIERS[inv_kind](after, inv_payload)
        assert restored == base, f"inverse-{kind}: {restored} != committed shared before-document {base}"
        raw = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=raw)

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables.
    Oracle role only — `identity-round-trip` stays subject-only, exactly as the Rust adapter already
    treats it: the real committed artifact is `.dsl.semio` text only, and decoding it needs this
    subset's own codec, which this reference does not carry."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
