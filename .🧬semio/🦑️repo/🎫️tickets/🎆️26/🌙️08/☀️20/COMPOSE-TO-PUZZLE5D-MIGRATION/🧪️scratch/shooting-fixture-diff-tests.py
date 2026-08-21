#!/usr/bin/env python3
"""Splices the three diff assertions into the 31 handcrafted 🎥️shooting test files.

Every doc comment, every extra assertion and every failure message below is written for ONE
mutation — the table is the handcrafted content, this script only places it.
"""
import os
import re

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

# leaf -> (case, label, produce_doc, [extra assertion lines], canonical_doc, applies_doc)
T = {}


def t(leaf, case, label, produce_doc, extras, canonical_doc, applies_doc):
    T[leaf] = (case, label, produce_doc, extras, canonical_doc, applies_doc)


L = '"{}: '  # convenience marker (unused, kept for readability)

t("🌱️create-asset", "appends-asset-detail", "create-asset/appends-asset-detail",
  "it proves `create-asset` ships the WHOLE new asset record in `assets.added` and never reaches\n/// into `shots`, `savedCameras` or either active cursor.",
  ['assert_eq!(committed["assets"]["added"][0]["id"], "asset-detail", "create-asset/appends-asset-detail: the new record travels in `assets.added`, by value");',
   'assert!(committed["assets"]["reordered"].is_null(), "create-asset/appends-asset-detail: a create carries no explicit ordering — the payload\'s `index` never reaches the diff");',
   'assert!(committed["shots"].is_null() && committed["savedCameras"].is_null() && committed["activeAssetId"].is_null(), "create-asset/appends-asset-detail: creating an asset must touch no other slot");'],
  "the committed create-asset delta round-trips through `ShootingDiff` unchanged.",
  "the `assets.added` entry alone is enough to rebuild the after-snapshot.")

t("🗑️delete-asset", "removes-trailing-asset-prop", "delete-asset/removes-trailing-asset-prop",
  "it proves `delete-asset` travels as a BARE ID in `assets.removed` — no record body, and above\n/// all no cascading patch into `shots` or the active cursors.",
  ['assert_eq!(committed["assets"]["removed"][0], "asset-prop", "delete-asset/removes-trailing-asset-prop: a delete is an id, never a record");',
   'assert!(committed["assets"]["patched"].as_array().expect("patched is an array").is_empty(), "delete-asset/removes-trailing-asset-prop: nothing is patched on the way out");',
   'assert!(committed["shots"].is_null() && committed["activeAssetId"].is_null(), "delete-asset/removes-trailing-asset-prop: the diff performs no referential cascade at all");'],
  "the committed delete-asset delta round-trips through `ShootingDiff` unchanged.",
  "the lone removed id is enough to rebuild the after-snapshot.")

t("✏️rename-asset", "renames-asset-hero-to-lead", "rename-asset/renames-asset-hero-to-lead",
  "it proves `rename-asset` writes a SPARSE `ShootingAssetPatch` with only `name` filled — the\n/// end-state test alone could not tell this apart from a whole-record replacement.",
  ['assert_eq!(committed["assets"]["patched"][0]["patch"]["name"], "Lead", "rename-asset/renames-asset-hero-to-lead: `name` is the one filled patch slot");',
   'assert!(committed["assets"]["patched"][0]["patch"]["url"].is_null() && committed["assets"]["patched"][0]["patch"]["origin"].is_null(), "rename-asset/renames-asset-hero-to-lead: url and transform slots stay null — this is a patch, not a replacement");',
   'assert!(committed["assets"]["added"].as_array().expect("added is an array").is_empty(), "rename-asset/renames-asset-hero-to-lead: a rename never re-adds the record");'],
  "the committed rename-asset patch round-trips through `ShootingDiff` unchanged.",
  "a one-slot patch is enough to rebuild the after-snapshot.")

t("🔗️change-asset-url", "points-asset-prop-at-v2-mesh", "change-asset-url/points-asset-prop-at-v2-mesh",
  "it proves `change-asset-url` fills only the `url` patch slot — in particular `format` is not a\n/// patch slot this mutation ever writes, so no url-derived format sneaks into the delta.",
  ['assert_eq!(committed["assets"]["patched"][0]["patch"]["url"], "/mesh/prop-v2.glb", "change-asset-url/points-asset-prop-at-v2-mesh: `url` is the one filled patch slot");',
   'assert!(committed["assets"]["patched"][0]["patch"]["name"].is_null(), "change-asset-url/points-asset-prop-at-v2-mesh: the display name slot stays null");',
   'assert_eq!(committed["assets"]["patched"][0]["id"], "asset-prop", "change-asset-url/points-asset-prop-at-v2-mesh: exactly one asset is addressed");'],
  "the committed change-asset-url patch round-trips through `ShootingDiff` unchanged.",
  "the single url patch is enough to rebuild the after-snapshot.")

t("🔀️reorder-assets", "moves-asset-hero-behind-asset-prop", "reorder-assets/moves-asset-hero-behind-asset-prop",
  "it proves `reorder-assets` ships a whole-list `reordered` id sequence and NOTHING else — no\n/// per-record patches, which is what guarantees the records are carried, not rewritten.",
  ['assert_eq!(committed["assets"]["reordered"][0], "asset-prop", "reorder-assets/moves-asset-hero-behind-asset-prop: the new order is a complete id sequence");',
   'assert_eq!(committed["assets"]["reordered"][1], "asset-hero", "reorder-assets/moves-asset-hero-behind-asset-prop: the moved asset is named last");',
   'assert!(committed["assets"]["patched"].as_array().expect("patched is an array").is_empty() && committed["assets"]["added"].as_array().expect("added is an array").is_empty(), "reorder-assets/moves-asset-hero-behind-asset-prop: reordering is pure permutation — no record is patched or re-added");'],
  "the committed reorder-assets sequence round-trips through `ShootingDiff` unchanged.",
  "the id sequence alone is enough to rebuild the after-snapshot.")

t("↔️drag-assets", "offsets-both-assets-and-skips-a-ghost", "drag-assets/offsets-both-assets-and-skips-a-ghost",
  "it proves the bulk drag fans out into ONE patch entry per RESOLVED asset — two entries, not\n/// three — each carrying an already-absolute origin, and no entry for the skipped ghost id.",
  ['assert_eq!(committed["assets"]["patched"].as_array().expect("patched is an array").len(), 2, "drag-assets/offsets-both-assets-and-skips-a-ghost: the unresolvable id contributes no patch entry");',
   'assert_eq!(committed["assets"]["patched"][0]["patch"]["origin"][0], 5.0, "drag-assets/offsets-both-assets-and-skips-a-ghost: the delta stores the RESOLVED absolute origin, not the relative offset");',
   'assert!(committed["assets"]["patched"][1]["patch"]["orientation"].is_null() && committed["assets"]["patched"][1]["patch"]["scale"].is_null(), "drag-assets/offsets-both-assets-and-skips-a-ghost: a drag fills only the `origin` slot");'],
  "the committed two-entry drag fan-out round-trips through `ShootingDiff` unchanged.",
  "the two origin patches are enough to rebuild the after-snapshot.")

t("🔄️rotate-assets", "spins-asset-hero-about-z", "rotate-assets/spins-asset-hero-about-z",
  "it proves the delta carries the COMPOSED quaternion, not the axis-angle the payload named —\n/// the multiplication happens in the diff builder, never at apply time.",
  ['assert_eq!(committed["assets"]["patched"][0]["patch"]["orientation"][3], (1.5f64 * 0.5).cos(), "rotate-assets/spins-asset-hero-about-z: the stored w is cos(angle/2), i.e. already composed");',
   'assert!(committed["assets"]["patched"][0]["patch"]["origin"].is_null(), "rotate-assets/spins-asset-hero-about-z: a rotation fills only the `orientation` slot");',
   'assert_eq!(committed["assets"]["patched"].as_array().expect("patched is an array").len(), 1, "rotate-assets/spins-asset-hero-about-z: only the addressed asset gets an entry");'],
  "the committed quaternion patch round-trips through `ShootingDiff` unchanged.",
  "the single orientation patch is enough to rebuild the after-snapshot.")

t("↕️scale-assets", "doubles-asset-hero-scale", "scale-assets/doubles-asset-hero-scale",
  "it proves the delta carries the PRODUCT `[4, 4, 4]`, not the factors `[2, 2, 2]` — the\n/// multiplication is resolved against the base inside the diff builder.",
  ['assert_eq!(committed["assets"]["patched"][0]["patch"]["scale"][0], 4.0, "scale-assets/doubles-asset-hero-scale: the delta stores the resolved product, not the factor");',
   'assert!(committed["assets"]["patched"][0]["patch"]["origin"].is_null() && committed["assets"]["patched"][0]["patch"]["orientation"].is_null(), "scale-assets/doubles-asset-hero-scale: a scale fills only the `scale` slot");',
   'assert!(committed["shots"].is_null() && committed["savedCameras"].is_null(), "scale-assets/doubles-asset-hero-scale: a transform never leaves the `assets` collection");'],
  "the committed scale patch round-trips through `ShootingDiff` unchanged.",
  "the single scale patch is enough to rebuild the after-snapshot.")

t("📸️create-shot", "appends-shot-macro", "create-shot/appends-shot-macro",
  "it proves `create-shot` ships the whole shot record in `shots.added` — and that the record's\n/// absent `background`/`cameraId` are OMITTED by serde rather than serialized as null.",
  ['assert_eq!(committed["shots"]["added"][0]["id"], "shot-macro", "create-shot/appends-shot-macro: the new record travels in `shots.added`, by value");',
   'assert!(committed["shots"]["added"][0].get("cameraId").is_none(), "create-shot/appends-shot-macro: `ShootingShot.camera_id` skips serializing when None, so the key is absent, not null");',
   'assert!(committed["assets"].is_null() && committed["savedCameras"].is_null(), "create-shot/appends-shot-macro: creating a shot must touch no other collection");'],
  "the committed create-shot delta round-trips through `ShootingDiff` unchanged.",
  "the `shots.added` entry alone is enough to rebuild the after-snapshot.")

t("🚮️delete-shot", "removes-trailing-shot-close", "delete-shot/removes-trailing-shot-close",
  "it proves `delete-shot` is a bare id in `shots.removed` and that `savedCameras` is left NULL —\n/// the strongest possible statement that no camera is garbage-collected.",
  ['assert_eq!(committed["shots"]["removed"][0], "shot-close", "delete-shot/removes-trailing-shot-close: a delete is an id, never a record");',
   'assert!(committed["savedCameras"].is_null(), "delete-shot/removes-trailing-shot-close: the saved-camera collection is not even opened");',
   'assert!(committed["activeShotId"].is_null(), "delete-shot/removes-trailing-shot-close: the active-shot cursor is not repaired by this diff");'],
  "the committed delete-shot delta round-trips through `ShootingDiff` unchanged.",
  "the lone removed id is enough to rebuild the after-snapshot.")

t("🏷️rename-shot", "relabels-shot-close-to-detail", "rename-shot/relabels-shot-close-to-detail",
  "it proves the `ShootingShotPatch` has `label` filled and its four sibling slots null — and note\n/// the patch type has no `background`/`cameraId` slot at all, so a relabel structurally cannot\n/// disturb either.",
  ['assert_eq!(committed["shots"]["patched"][0]["patch"]["label"], "Detail", "rename-shot/relabels-shot-close-to-detail: `label` is the one filled patch slot");',
   'assert!(committed["shots"]["patched"][0]["patch"]["width"].is_null() && committed["shots"]["patched"][0]["patch"]["height"].is_null(), "rename-shot/relabels-shot-close-to-detail: the pixel-dimension slots stay null");',
   'assert!(committed["shots"]["patched"][0]["patch"].get("cameraId").is_none(), "rename-shot/relabels-shot-close-to-detail: `ShootingShotPatch` carries no camera slot, so a relabel cannot rebind a shot");'],
  "the committed rename-shot patch round-trips through `ShootingDiff` unchanged.",
  "a one-slot patch is enough to rebuild the after-snapshot.")

t("📏️change-shot-width", "widens-shot-close-to-1024", "change-shot-width/widens-shot-close-to-1024",
  "it proves the aspect independence at DELTA level: `width` is filled and `height` is explicitly\n/// null in the same patch, so no proportional resize can hide behind a matching end state.",
  ['assert_eq!(committed["shots"]["patched"][0]["patch"]["width"], 1024, "change-shot-width/widens-shot-close-to-1024: `width` is the one filled patch slot");',
   'assert!(committed["shots"]["patched"][0]["patch"]["height"].is_null(), "change-shot-width/widens-shot-close-to-1024: `height` is null IN THE DELTA — the proof there is no aspect coupling");',
   'assert!(committed["shots"]["patched"][0]["patch"]["shape"].is_null(), "change-shot-width/widens-shot-close-to-1024: the mask shape slot stays null");'],
  "the committed width patch round-trips through `ShootingDiff` unchanged.",
  "the single width patch is enough to rebuild the after-snapshot.")

t("📐️change-shot-height", "heightens-shot-close-to-768", "change-shot-height/heightens-shot-close-to-768",
  "it proves the mirror-image sparsity of its width sibling: `height` filled, `width` explicitly\n/// null in the same patch.",
  ['assert_eq!(committed["shots"]["patched"][0]["patch"]["height"], 768, "change-shot-height/heightens-shot-close-to-768: `height` is the one filled patch slot");',
   'assert!(committed["shots"]["patched"][0]["patch"]["width"].is_null(), "change-shot-height/heightens-shot-close-to-768: `width` is null IN THE DELTA — the proof there is no aspect coupling");',
   'assert!(committed["shots"]["patched"][0]["patch"]["format"].is_null(), "change-shot-height/heightens-shot-close-to-768: the render-format slot stays null");'],
  "the committed height patch round-trips through `ShootingDiff` unchanged.",
  "the single height patch is enough to rebuild the after-snapshot.")

t("🖼️change-shot-format", "switches-shot-wide-to-svg", "change-shot-format/switches-shot-wide-to-svg",
  "it proves `format` and `shape` are independent patch slots: switching to a vector format fills\n/// `format` and leaves `shape` null.",
  ['assert_eq!(committed["shots"]["patched"][0]["patch"]["format"], "svg", "change-shot-format/switches-shot-wide-to-svg: `format` is the one filled patch slot");',
   'assert!(committed["shots"]["patched"][0]["patch"]["shape"].is_null(), "change-shot-format/switches-shot-wide-to-svg: the mask shape is a separate slot and stays null");',
   'assert_eq!(committed["shots"]["patched"][0]["id"], "shot-wide", "change-shot-format/switches-shot-wide-to-svg: exactly one shot is addressed");'],
  "the committed format patch round-trips through `ShootingDiff` unchanged.",
  "the single format patch is enough to rebuild the after-snapshot.")

t("✂️change-shot-shape", "rounds-shot-wide-to-ellipse", "change-shot-shape/rounds-shot-wide-to-ellipse",
  "it proves the converse of its format sibling: `shape` filled, `format` null — the mask outline\n/// moves without touching the encoder.",
  ['assert_eq!(committed["shots"]["patched"][0]["patch"]["shape"], "ellipse", "change-shot-shape/rounds-shot-wide-to-ellipse: `shape` is the one filled patch slot");',
   'assert!(committed["shots"]["patched"][0]["patch"]["format"].is_null(), "change-shot-shape/rounds-shot-wide-to-ellipse: the render format is a separate slot and stays null");',
   'assert!(committed["shots"]["patched"][0]["patch"]["label"].is_null(), "change-shot-shape/rounds-shot-wide-to-ellipse: the caption slot stays null");'],
  "the committed shape patch round-trips through `ShootingDiff` unchanged.",
  "the single shape patch is enough to rebuild the after-snapshot.")

t("🔃️reorder-shots", "moves-shot-close-to-front", "reorder-shots/moves-shot-close-to-front",
  "it proves the storyboard permutation ships as a `shots.reordered` id sequence with `activeShotId`\n/// left NULL — the delta itself is the proof the cursor does not follow the new index 0.",
  ['assert_eq!(committed["shots"]["reordered"][0], "shot-close", "reorder-shots/moves-shot-close-to-front: the promoted shot heads the sequence");',
   'assert!(committed["activeShotId"].is_null(), "reorder-shots/moves-shot-close-to-front: the active-shot cursor slot is untouched by a reorder");',
   'assert!(committed["shots"]["patched"].as_array().expect("patched is an array").is_empty(), "reorder-shots/moves-shot-close-to-front: reordering is pure permutation");'],
  "the committed reorder-shots sequence round-trips through `ShootingDiff` unchanged.",
  "the id sequence alone is enough to rebuild the after-snapshot.")

t("📷️replace-shot-camera", "rewrites-cam-wide-through-shot-wide", "replace-shot-camera/rewrites-cam-wide-through-shot-wide",
  "it is the only place the write-through is visible: the payload names a SHOT, the delta patches a\n/// SAVED CAMERA, and `shots` is left NULL entirely.",
  ['assert_eq!(committed["savedCameras"]["patched"][0]["id"], "cam-wide", "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the delta is keyed by the dereferenced CAMERA id, not the payload\'s shot id");',
   'assert!(committed["shots"].is_null(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the `shots` collection is not opened at all");',
   'assert!(committed["savedCameras"]["patched"][0]["patch"]["label"].is_null(), "replace-shot-camera/rewrites-cam-wide-through-shot-wide: the patch fills `camera` and leaves `label` null");'],
  "the committed write-through patch round-trips through `ShootingDiff` unchanged.",
  "the saved-camera patch alone is enough to rebuild the after-snapshot.")

t("🎥️create-saved-camera", "appends-saved-camera-top", "create-saved-camera/appends-saved-camera-top",
  "it proves the new pose enters via `savedCameras.added` and that `shots` stays NULL — no shot is\n/// rebound onto the camera that was just created.",
  ['assert_eq!(committed["savedCameras"]["added"][0]["id"], "cam-top", "create-saved-camera/appends-saved-camera-top: the new record travels in `savedCameras.added`, by value");',
   'assert_eq!(committed["savedCameras"]["added"][0]["camera"]["fov"], 50.0, "create-saved-camera/appends-saved-camera-top: the whole pose rides along inside the record");',
   'assert!(committed["shots"].is_null(), "create-saved-camera/appends-saved-camera-top: no shot is rebound onto the new camera");'],
  "the committed create-saved-camera delta round-trips through `ShootingDiff` unchanged.",
  "the `savedCameras.added` entry alone is enough to rebuild the after-snapshot.")

t("🧹️delete-saved-camera", "removes-trailing-cam-close", "delete-saved-camera/removes-trailing-cam-close",
  "it proves the delete is a bare id in `savedCameras.removed` and that `shots` stays NULL — no\n/// dangling `cameraId` is cleared, which is exactly the behaviour to pin down.",
  ['assert_eq!(committed["savedCameras"]["removed"][0], "cam-close", "delete-saved-camera/removes-trailing-cam-close: a delete is an id, never a record");',
   'assert!(committed["shots"].is_null(), "delete-saved-camera/removes-trailing-cam-close: no shot\'s `cameraId` is cleared — the reference is deliberately left one-way");',
   'assert!(committed["savedCameras"]["patched"].as_array().expect("patched is an array").is_empty(), "delete-saved-camera/removes-trailing-cam-close: nothing is patched on the way out");'],
  "the committed delete-saved-camera delta round-trips through `ShootingDiff` unchanged.",
  "the lone removed id is enough to rebuild the after-snapshot.")

t("🪪️rename-saved-camera", "relabels-cam-close-to-tight", "rename-saved-camera/relabels-cam-close-to-tight",
  "it proves the `ShootingSavedCameraPatch`'s two slots are used as `{label: Some, camera: None}` —\n/// the explicit null `camera` is what keeps the stored pose out of the write.",
  ['assert_eq!(committed["savedCameras"]["patched"][0]["patch"]["label"], "Tight", "rename-saved-camera/relabels-cam-close-to-tight: `label` is the filled patch slot");',
   'assert!(committed["savedCameras"]["patched"][0]["patch"]["camera"].is_null(), "rename-saved-camera/relabels-cam-close-to-tight: the `camera` slot is explicitly null in the delta");',
   'assert!(committed["shots"].is_null(), "rename-saved-camera/relabels-cam-close-to-tight: relabelling a camera never opens the shot collection");'],
  "the committed rename-saved-camera patch round-trips through `ShootingDiff` unchanged.",
  "a one-slot patch is enough to rebuild the after-snapshot.")

t("🎞️replace-saved-camera-view", "repositions-cam-close-view", "replace-saved-camera-view/repositions-cam-close-view",
  "it proves the mirror of its rename sibling: `{label: None, camera: Some}` — and that the camera\n/// slot carries the WHOLE pose, so nothing is merged with the old one.",
  ['assert_eq!(committed["savedCameras"]["patched"][0]["patch"]["camera"]["zoom"], 4.0, "replace-saved-camera-view/repositions-cam-close-view: the whole replacement pose is in the delta");',
   'assert!(committed["savedCameras"]["patched"][0]["patch"]["label"].is_null(), "replace-saved-camera-view/repositions-cam-close-view: the `label` slot is explicitly null in the delta");',
   'assert_eq!(committed["savedCameras"]["patched"][0]["id"], "cam-close", "replace-saved-camera-view/repositions-cam-close-view: the delta is keyed by the payload\'s own camera id");'],
  "the committed pose replacement round-trips through `ShootingDiff` unchanged.",
  "the single camera patch is enough to rebuild the after-snapshot.")

t("🔁️reorder-saved-cameras", "moves-cam-close-to-front", "reorder-saved-cameras/moves-cam-close-to-front",
  "it proves the library permutation ships as a `savedCameras.reordered` id sequence with `shots`\n/// left NULL — the delta is the proof that id-keyed shot bindings need no repair.",
  ['assert_eq!(committed["savedCameras"]["reordered"][0], "cam-close", "reorder-saved-cameras/moves-cam-close-to-front: the promoted camera heads the sequence");',
   'assert!(committed["shots"].is_null(), "reorder-saved-cameras/moves-cam-close-to-front: no shot binding is rewritten to chase the new index");',
   'assert!(committed["savedCameras"]["patched"].as_array().expect("patched is an array").is_empty(), "reorder-saved-cameras/moves-cam-close-to-front: reordering is pure permutation");'],
  "the committed reorder-saved-cameras sequence round-trips through `ShootingDiff` unchanged.",
  "the id sequence alone is enough to rebuild the after-snapshot.")

t("🎯️set-active-shot", "activates-shot-close", "set-active-shot/activates-shot-close",
  "it proves the cursor move is a bare document-root scalar: `activeShotId` filled, and ALL THREE\n/// collection slots null — a cursor move can never smuggle a collection edit.",
  ['assert_eq!(committed["activeShotId"], "shot-close", "set-active-shot/activates-shot-close: the scalar slot carries the new cursor");',
   'assert!(committed["assets"].is_null() && committed["shots"].is_null() && committed["savedCameras"].is_null(), "set-active-shot/activates-shot-close: no collection delta is opened at all");',
   'assert!(committed["activeAssetId"].is_null(), "set-active-shot/activates-shot-close: the sibling asset cursor slot stays null");'],
  "the committed scalar delta round-trips through `ShootingDiff` unchanged.",
  "the single scalar is enough to rebuild the after-snapshot.")

t("📌️set-active-asset", "activates-asset-prop", "set-active-asset/activates-asset-prop",
  "it proves the asset cursor writes `activeAssetId` and NOT `activeShotId` — two same-shaped\n/// scalar slots that only the delta can tell apart.",
  ['assert_eq!(committed["activeAssetId"], "asset-prop", "set-active-asset/activates-asset-prop: the scalar slot carries the new cursor");',
   'assert!(committed["activeShotId"].is_null(), "set-active-asset/activates-asset-prop: the sibling shot cursor slot stays null");',
   'assert!(committed["assets"].is_null(), "set-active-asset/activates-asset-prop: validating the id against `assets` does not mean patching `assets`");'],
  "the committed scalar delta round-trips through `ShootingDiff` unchanged.",
  "the single scalar is enough to rebuild the after-snapshot.")

t("☀️change-scene-sun-enabled", "switches-scene-sun-off", "change-scene-sun-enabled/switches-scene-sun-off",
  "it exposes this family's deliberate COARSENESS: the scene leaves ship the whole cloned\n/// `ShootingSceneLighting`, so the delta names ambient/shadow/material too — the guarantee is that\n/// their values are carried unchanged, not that they are absent.",
  ['assert_eq!(committed["scene"]["sun"]["enabled"], false, "change-scene-sun-enabled/switches-scene-sun-off: the edited field inside the cloned scene");',
   'assert_eq!(committed["scene"]["sun"]["intensity"], 2.4, "change-scene-sun-enabled/switches-scene-sun-off: the sun\'s other settings ride along at their BASE values");',
   'assert!(committed["assets"].is_null() && committed["shots"].is_null(), "change-scene-sun-enabled/switches-scene-sun-off: coarse within `scene`, but it never leaves it");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("🧭️change-scene-sun-azimuth", "turns-scene-sun-to-315-degrees", "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees",
  "it pins the unwrapped bearing INSIDE the cloned scene block, and pins that the sun's enabled\n/// flag and elevation ride along at their base values rather than being recomputed.",
  ['assert_eq!(committed["scene"]["sun"]["azimuth"], 315.0, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: the bearing is in the delta unwrapped, not normalized to -45");',
   'assert_eq!(committed["scene"]["sun"]["elevation"], 35.0, "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: elevation rides along at its BASE value");',
   'assert!(committed["activeShotId"].is_null() && committed["camera"].is_null(), "change-scene-sun-azimuth/turns-scene-sun-to-315-degrees: turning the sun touches no cursor and no config camera");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("🌅️change-scene-sun-elevation", "raises-scene-sun-to-60-degrees", "change-scene-sun-elevation/raises-scene-sun-to-60-degrees",
  "it pins the new elevation inside the cloned scene block while the azimuth it is so easily\n/// confused with sits beside it at its base value.",
  ['assert_eq!(committed["scene"]["sun"]["elevation"], 60.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the edited field inside the cloned scene");',
   'assert_eq!(committed["scene"]["sun"]["azimuth"], 45.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the compass bearing rides along at its BASE value");',
   'assert_eq!(committed["scene"]["material"]["roughness"], 1.0, "change-scene-sun-elevation/raises-scene-sun-to-60-degrees: the material block is cloned verbatim, not re-derived");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("💡️change-scene-sun-intensity", "dims-scene-sun-to-half", "change-scene-sun-intensity/dims-scene-sun-to-half",
  "it pins the KEY light's new strength and — crucially — the ambient fill's untouched strength in\n/// the same block, the only place those two identically-named payload fields are distinguishable.",
  ['assert_eq!(committed["scene"]["sun"]["intensity"], 1.2, "change-scene-sun-intensity/dims-scene-sun-to-half: the SUN\'s intensity is the edited field");',
   'assert_eq!(committed["scene"]["ambient"]["intensity"], 1.15, "change-scene-sun-intensity/dims-scene-sun-to-half: the AMBIENT intensity rides along at its base value");',
   'assert_eq!(committed["scene"]["sun"]["enabled"], true, "change-scene-sun-intensity/dims-scene-sun-to-half: dimming is not disabling, even at delta level");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("🔅️change-scene-ambient-intensity", "dims-scene-ambient-to-quarter", "change-scene-ambient-intensity/dims-scene-ambient-to-quarter",
  "it is the mirror of the sun-intensity fixture: the AMBIENT intensity is the edited field and the\n/// sun's own intensity rides along untouched.",
  ['assert_eq!(committed["scene"]["ambient"]["intensity"], 0.25, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the AMBIENT intensity is the edited field");',
   'assert_eq!(committed["scene"]["sun"]["intensity"], 2.4, "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the SUN\'s intensity rides along at its base value");',
   'assert_eq!(committed["scene"]["ambient"]["color"], "#ffffff", "change-scene-ambient-intensity/dims-scene-ambient-to-quarter: the ambient tint is cloned, not reset");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("🌑️change-scene-shadow-enabled", "switches-scene-shadows-off", "change-scene-shadow-enabled/switches-scene-shadows-off",
  "it pins the shadow toggle inside the cloned scene block together with the sun's own flag, which\n/// stays TRUE in the same delta — the two toggles are not chained.",
  ['assert_eq!(committed["scene"]["shadow"]["enabled"], false, "change-scene-shadow-enabled/switches-scene-shadows-off: the edited field inside the cloned scene");',
   'assert_eq!(committed["scene"]["sun"]["enabled"], true, "change-scene-shadow-enabled/switches-scene-shadows-off: the sun that casts the shadow stays on in the same delta");',
   'assert_eq!(committed["scene"]["shadow"]["opacity"], 0.35, "change-scene-shadow-enabled/switches-scene-shadows-off: opacity rides along so the old look returns on re-enable");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")

t("🪨️change-scene-material-roughness", "polishes-scene-material-to-quarter", "change-scene-material-roughness/polishes-scene-material-to-quarter",
  "it pins the one PBR knob that has a mutation, and pins that its neighbours in the same struct —\n/// `metalness`, `color`, `emissive` — are cloned rather than defaulted.",
  ['assert_eq!(committed["scene"]["material"]["roughness"], 0.25, "change-scene-material-roughness/polishes-scene-material-to-quarter: the edited field inside the cloned scene");',
   'assert_eq!(committed["scene"]["material"]["metalness"], 0.0, "change-scene-material-roughness/polishes-scene-material-to-quarter: metalness has no mutation and must be cloned, not defaulted");',
   'assert_eq!(committed["scene"]["material"]["emissiveIntensity"], 0.0, "change-scene-material-roughness/polishes-scene-material-to-quarter: the whole material struct rides along, camelCased by the diff\'s own serde attrs");'],
  "the committed whole-scene block round-trips through `ShootingDiff` unchanged.",
  "the cloned scene block is enough to rebuild the after-snapshot.")


BLOCK = '''
/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — {produce_doc}
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️component.json");
{extras}
}}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — {canonical_doc}
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff straight to `before` yields `after` — {applies_doc}
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
'''


def main():
    touched = 0
    for leaf, (case, label, produce_doc, extras, canonical_doc, applies_doc) in T.items():
        path = os.path.join(ROOT, leaf, "🧪️tests", case, "🦀️component.rs")
        src = open(path, encoding="utf-8").read()
        assert "const DIFF:" not in src, path

        src = src.replace(
            "use crate::artifacts::shooting::ShootingSnapshot;",
            "use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};",
            1,
        )
        src = src.replace(
            'const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");',
            'const DIFF: &str = include_str!("🔺️diff/🔣️component.json");\nconst OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");',
            1,
        )
        assert "ShootingDiff, ShootingSnapshot" in src and "const DIFF:" in src, path

        block = BLOCK.format(
            produce_doc=produce_doc,
            label=label,
            extras="\n".join("    " + line for line in extras),
            canonical_doc=canonical_doc,
            applies_doc=applies_doc,
        )
        src = src.rstrip("\n") + "\n" + block
        open(path, "w", encoding="utf-8").write(src)
        touched += 1
    assert touched == 31, touched
    print(f"spliced {touched} test files")


if __name__ == "__main__":
    main()
