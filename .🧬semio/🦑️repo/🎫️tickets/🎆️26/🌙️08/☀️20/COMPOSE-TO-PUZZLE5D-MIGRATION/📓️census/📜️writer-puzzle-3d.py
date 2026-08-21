#!/usr/bin/env python3
"""Writer for the handcrafted puzzle 🧊️3d mutation fixtures (ticket 26/08/20).

Every case's before/after/mutation/diff/outcome payload and every assertion string below is
hand-authored from that mutation's own 🔺️diff/🦀️component.rs. This file is only the transcriber
that lays the bytes down on disk; it contains no per-mutation logic.
"""
import copy
import json
import os
import textwrap

ROOT = "/Users/ueli/Documents/semio"
TREE = os.path.join(
    ROOT,
    "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
)

DIFF_FIELDS = [
    "artifact", "schema", "domain", "meta", "objects", "attractions", "targetVolumes",
    "references", "selectedObjectIds", "selectedVortexIds", "selectedAttractionIds",
    "selectedTargetVolumeIds", "selectedReferenceIds", "activeUtilityId", "cameraPositionX",
    "cameraPositionY", "cameraPositionZ", "cameraTargetX", "cameraTargetY", "cameraTargetZ",
    "cameraZoom", "selectionMethod", "selectionModeDefault", "engagementInput", "gridVisible",
    "gridSnapEnabled", "gridSpacing", "overlapBudget", "fillCount", "brushCandidateIndex",
    "lodAutomatic", "lodDepthVariable", "lodManual", "proximityRadius", "locale",
    "runtimeExtrasJson", "hoveredObjectId", "hoveredVortexFullId", "hoveredKindId", "previewSeq",
]


def diff(**set_fields):
    return {name: set_fields.get(name) for name in DIFF_FIELDS}


def delta(added=None, removed=None, patched=None, reordered=None):
    return {
        "added": added or [],
        "removed": removed or [],
        "patched": patched or [],
        "reordered": reordered,
    }


def patch(entry_id, replacement):
    return [{"id": entry_id, "patch": {"replacement": replacement}}]


VORTEX_1 = {"id": "vortex-1", "vortexKind": "vortex-kind-a", "position": [1.0, 0.0, 0.0],
            "hidden": False, "locked": False}
VORTEX_SPARE = {"id": "vortex-spare", "vortexKind": "vortex-kind-a", "position": [0.0, 1.0, 0.0],
                "hidden": False, "locked": False}
VORTEX_2 = {"id": "vortex-2", "vortexKind": "vortex-kind-b", "position": [-1.0, 0.0, 0.0],
            "hidden": False, "locked": False}

OBJECT_A = {
    "id": "object-a",
    "label": "Alpha",
    "objectKind": "object-kind-a",
    "anchor": "fixed",
    "origin": [0.0, 0.0, 0.0],
    "orientation": [0.0, 0.0, 0.0, 1.0],
    "scale": 1.0,
    "meshUrl": "mesh://alpha",
    "vortices": [copy.deepcopy(VORTEX_1), copy.deepcopy(VORTEX_SPARE)],
    "hidden": False,
    "locked": False,
}

OBJECT_B = {
    "id": "object-b",
    "objectKind": "object-kind-b",
    "anchor": "fixed",
    "origin": [4.0, 0.0, 0.0],
    "vortices": [copy.deepcopy(VORTEX_2)],
    "hidden": False,
    "locked": False,
}

ATTRACTION_1 = {
    "id": "attraction-1",
    "attracting": "object-a:vortex-1",
    "attracted": "object-b:vortex-2",
    "gap": 1.0,
    "shift": 0.0,
    "rise": 0.0,
    "rotation": 0.0,
    "turn": 0.0,
    "tilt": 0.0,
    "x": 0.0,
    "y": 0.0,
}

VOLUME_1 = {
    "id": "volume-1",
    "origin": [0.0, 0.0, 0.0],
    "orientation": [0.0, 0.0, 0.0, 1.0],
    "scale": [2.0, 2.0, 2.0],
    "hidden": False,
    "locked": False,
}

REFERENCE_1 = {
    "id": "reference-1",
    "source": {"url": "asset://plan.png", "mediaKind": "image"},
    "origin": [0.0, 0.0, 0.0],
    "widthWorld": 10.0,
    "locked": False,
    "hidden": False,
}

COMPAT_AB = {"source": "vortex-kind-a", "target": "vortex-kind-b", "bidirectional": True,
             "important": False, "specificity": "vortex"}

BASE = {
    "schema": "puzzle.3d",
    "domain": "architecture",
    "meta": {"kindCompatibility": [copy.deepcopy(COMPAT_AB)]},
    "objects": [copy.deepcopy(OBJECT_A), copy.deepcopy(OBJECT_B)],
    "attractions": [copy.deepcopy(ATTRACTION_1)],
    "targetVolumes": [copy.deepcopy(VOLUME_1)],
    "references": [copy.deepcopy(REFERENCE_1)],
}

APPLIED = {"status": "applied"}
APPLIED_NOOP = {"status": "applied",
                "messages": [{"level": "warn", "code": "mutation.no-op"}]}


def base():
    return copy.deepcopy(BASE)


def edited(template, **edits):
    item = copy.deepcopy(template)
    for key, value in edits.items():
        if value is None and key in item:
            del item[key]
        elif value is not None:
            item[key] = value
    return item


def snap_with(collection, index, item):
    snap = base()
    snap[collection][index] = item
    return snap


CASES = []


def case(leaf, name, summary, mutation, diff_json, after, outcome, after_asserts, diff_asserts):
    CASES.append({
        "leaf": leaf, "name": name, "summary": summary, "mutation": mutation, "diff": diff_json,
        "before": base(), "after": after, "outcome": outcome,
        "after_asserts": after_asserts, "diff_asserts": diff_asserts,
    })


# ─── 🌱create-object ──────────────────────────────────────────────────────────────────────────
OBJECT_C = {
    "id": "object-c",
    "objectKind": "object-kind-c",
    "anchor": "fixed",
    "origin": [8.0, 0.0, 0.0],
    "vortices": [{"id": "vortex-3", "vortexKind": "vortex-kind-a", "position": [0.0, 0.0, 1.0],
                  "hidden": False, "locked": False}],
    "hidden": False,
    "locked": False,
}
_after = base()
_after["objects"].append(copy.deepcopy(OBJECT_C))
case(
    "🌱create-object", "appends-object-c",
    "A brand-new `object-c` is appended to `objects`; a `null` index means the builder emits no "
    "`reordered` order at all, so the object lands at the end of the collection.",
    {"mutation": "createObject", "object": copy.deepcopy(OBJECT_C), "index": None},
    diff(objects=delta(added=[copy.deepcopy(OBJECT_C)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.objects.len(), 3, "create-object/appends-object-c: object-c was not appended to the objects collection");',
        'assert_eq!(snapshot.objects[2].id, "object-c", "create-object/appends-object-c: a null index must append, never insert");',
        'assert_eq!(snapshot.attractions, before().attractions, "create-object/appends-object-c: a fresh object attracts nothing on its own");',
    ],
    [
        'assert_eq!(committed["objects"]["added"][0]["id"].as_str(), Some("object-c"), "create-object/appends-object-c: the diff must carry object-c in objects.added");',
        'assert!(committed["objects"]["reordered"].is_null(), "create-object/appends-object-c: a null index must leave reordered unset");',
        'assert!(committed["attractions"].is_null(), "create-object/appends-object-c: create-object must never touch the attractions delta");',
    ],
)

# ─── 🗑delete-object ──────────────────────────────────────────────────────────────────────────
_after = base()
_after["objects"] = [copy.deepcopy(OBJECT_B)]
_after["attractions"] = []
case(
    "🗑delete-object", "removes-object-a-and-severs-attraction",
    "Deleting `object-a` cascades through the `object:vortex` full-id form: `attraction-1` "
    "attracts from `object-a:vortex-1`, so the builder removes it in the very same diff.",
    {"mutation": "deleteObject", "id": "object-a"},
    diff(objects=delta(removed=["object-a"]), attractions=delta(removed=["attraction-1"])),
    _after, APPLIED,
    [
        'assert!(!snapshot.objects.iter().any(|object| object.id == "object-a"), "delete-object/removes-object-a-and-severs-attraction: object-a survived the delete");',
        'assert!(snapshot.attractions.is_empty(), "delete-object/removes-object-a-and-severs-attraction: attraction-1 hangs off object-a:vortex-1 and must be severed");',
        'assert_eq!(snapshot.target_volumes, before().target_volumes, "delete-object/removes-object-a-and-severs-attraction: deleting an object must not touch the target volumes");',
    ],
    [
        'assert_eq!(committed["objects"]["removed"][0].as_str(), Some("object-a"), "delete-object/removes-object-a-and-severs-attraction: the diff must remove object-a by id");',
        'assert_eq!(committed["attractions"]["removed"][0].as_str(), Some("attraction-1"), "delete-object/removes-object-a-and-severs-attraction: the severed attraction must be a removal, not a rewrite");',
        'assert!(committed["references"].is_null() && committed["targetVolumes"].is_null(), "delete-object/removes-object-a-and-severs-attraction: the cascade stops at attractions");',
    ],
)

# ─── 📍move-object ────────────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, origin=[1.0, 2.0, 3.0])
case(
    "📍move-object", "moves-object-a",
    "An absolute world reposition of `object-a`'s `origin`. The builder clones the object and "
    "rewrites that one `[f64; 3]` field, leaving the orientation and scale alone.",
    {"mutation": "moveObject", "id": "object-a", "newOrigin": [1.0, 2.0, 3.0]},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its own move");',
        'assert_eq!(object.origin, [1.0, 2.0, 3.0], "move-object/moves-object-a: object-a did not land on the committed origin");',
        'assert_eq!(object.orientation, before().objects[0].orientation, "move-object/moves-object-a: a move must not reorient the object");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["id"].as_str(), Some("object-a"), "move-object/moves-object-a: the diff must patch object-a and nothing else");',
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["origin"][2].as_f64(), Some(3.0), "move-object/moves-object-a: the replacement must carry the new z");',
        'assert!(committed["attractions"].is_null(), "move-object/moves-object-a: moving an object must not re-solve its attractions here");',
    ],
)

# ─── 🔃rotate-object ──────────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, orientation=[0.0, 0.0, 1.0, 0.0])
case(
    "🔃rotate-object", "half-turn-about-z",
    "Replaces `object-a`'s orientation quaternion with a half turn about Z. The payload is an "
    "`Option<[f64; 4]>` assigned wholesale, so `null` would clear the orientation entirely.",
    {"mutation": "rotateObject", "id": "object-a", "newOrientation": [0.0, 0.0, 1.0, 0.0]},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its rotation");',
        'assert_eq!(object.orientation, Some([0.0, 0.0, 1.0, 0.0]), "rotate-object/half-turn-about-z: object-a did not take the committed quaternion");',
        'assert_eq!(object.origin, before().objects[0].origin, "rotate-object/half-turn-about-z: a rotation must not translate the object");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["orientation"][2].as_f64(), Some(1.0), "rotate-object/half-turn-about-z: the replacement must carry the new quaternion");',
        'assert!(committed["targetVolumes"].is_null(), "rotate-object/half-turn-about-z: rotating an object never touches a target volume");',
    ],
)

# ─── 📏scale-object ───────────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, scale=[2.0, 1.0, 0.5])
case(
    "📏scale-object", "uniform-to-per-axis",
    "`object-a` carries the scalar form of `Puzzle3dScale` in the base; the payload swaps in the "
    "explicit `[x, y, z]` triple, so the wire shape changes from a bare number to an array.",
    {"mutation": "scaleObject", "id": "object-a", "newScale": [2.0, 1.0, 0.5]},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its scaling");',
        'assert_eq!(object.scale, Some(crate::artifacts::puzzle3d::Puzzle3dScale::Vec3([2.0, 1.0, 0.5])), "scale-object/uniform-to-per-axis: object-a did not take the per-axis scale");',
        'assert_eq!(object.mesh_url, before().objects[0].mesh_url, "scale-object/uniform-to-per-axis: scaling must not repoint the mesh");',
    ],
    [
        'assert!(committed["objects"]["patched"][0]["patch"]["replacement"]["scale"].is_array(), "scale-object/uniform-to-per-axis: the per-axis form must serialize as an array, not a scalar");',
        'assert!(committed["attractions"].is_null(), "scale-object/uniform-to-per-axis: scaling never rewrites an attraction");',
    ],
)

# ─── 🧱change-object-mesh ─────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, meshUrl="mesh://alpha-lod2")
case(
    "🧱change-object-mesh", "repoints-object-a-mesh",
    "Repoints `object-a` at a different mesh URL. Only `meshUrl` moves — the object keeps its "
    "kind, pose and every rim vortex.",
    {"mutation": "changeObjectMesh", "id": "object-a", "newMeshUrl": "mesh://alpha-lod2"},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its mesh swap");',
        'assert_eq!(object.mesh_url.as_deref(), Some("mesh://alpha-lod2"), "change-object-mesh/repoints-object-a-mesh: object-a kept its old mesh");',
        'assert_eq!(object.vortices, before().objects[0].vortices, "change-object-mesh/repoints-object-a-mesh: a mesh swap must not rebuild the rim vortices");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["meshUrl"].as_str(), Some("mesh://alpha-lod2"), "change-object-mesh/repoints-object-a-mesh: the replacement must carry the new mesh url");',
        'assert!(committed["meta"].is_null(), "change-object-mesh/repoints-object-a-mesh: a mesh swap is not a catalog edit");',
    ],
)

# ─── 🖋️edit-object-label ──────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, label="Alpha Prime")
case(
    "🖋️edit-object-label", "relabels-object-a",
    "Rewrites `object-a`'s human-facing label. The payload is an `Option<String>` assigned "
    "wholesale, so `null` would clear the label rather than leave it alone.",
    {"mutation": "editObjectLabel", "id": "object-a", "newLabel": "Alpha Prime"},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its relabel");',
        'assert_eq!(object.label.as_deref(), Some("Alpha Prime"), "edit-object-label/relabels-object-a: object-a kept its old label");',
        'assert_eq!(object.object_kind, before().objects[0].object_kind, "edit-object-label/relabels-object-a: a label is not a catalog reference");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["label"].as_str(), Some("Alpha Prime"), "edit-object-label/relabels-object-a: the replacement must carry the new label");',
        'assert_eq!(committed["objects"]["patched"].as_array().map(Vec::len), Some(1), "edit-object-label/relabels-object-a: exactly one object may be patched");',
    ],
)

# ─── 🏗change-object-kind ─────────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, objectKind="object-kind-c")
case(
    "🏗change-object-kind", "reassigns-object-a-kind",
    "Repoints `object-a` at the `object-kind-c` catalog row. The builder writes only "
    "`objectKind`; it does not re-derive the vortices from the new kind's templates.",
    {"mutation": "changeObjectKind", "id": "object-a", "newObjectKind": "object-kind-c"},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its kind change");',
        'assert_eq!(object.object_kind.as_deref(), Some("object-kind-c"), "change-object-kind/reassigns-object-a-kind: object-a still points at its old catalog row");',
        'assert_eq!(object.vortices.len(), 2, "change-object-kind/reassigns-object-a-kind: a kind change must not re-derive the vortex list from the new kind");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["objectKind"].as_str(), Some("object-kind-c"), "change-object-kind/reassigns-object-a-kind: the replacement must carry the new kind");',
        'assert!(committed["meta"].is_null(), "change-object-kind/reassigns-object-a-kind: pointing at a catalog row must not rewrite the catalog itself");',
    ],
)

# ─── ⚓change-object-anchor ──────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, anchor="derived")
case(
    "⚓change-object-anchor", "fixed-to-derived",
    "Flips `object-a` from keeping its stored plane (`fixed`) to resetting to default XY on "
    "flatten (`derived`). `anchor` is a bare enum with no `Option`, so it is always present.",
    {"mutation": "changeObjectAnchor", "id": "object-a", "newAnchor": "derived"},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives its anchor flip");',
        'assert_eq!(object.anchor, crate::artifacts::puzzle3d::Puzzle3dObjectAnchor::Derived, "change-object-anchor/fixed-to-derived: object-a is still anchored fixed");',
        'assert_eq!(object.origin, before().objects[0].origin, "change-object-anchor/fixed-to-derived: flipping the anchor must not move the stored origin");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["anchor"].as_str(), Some("derived"), "change-object-anchor/fixed-to-derived: the replacement must carry the derived anchor");',
        'assert!(committed["attractions"].is_null(), "change-object-anchor/fixed-to-derived: the anchor flip must not re-solve the attractions here");',
    ],
)

# ─── 👁change-object-hidden ──────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, hidden=True)
case(
    "👁change-object-hidden", "hides-object-a",
    "Sets `object-a`'s `hidden` flag. Unlike puzzle2d's tri-state `visible`, puzzle3d's flag is a "
    "plain `bool` with a `#[serde(default)]`, so it is always present on the wire.",
    {"mutation": "changeObjectHidden", "id": "object-a", "newHidden": True},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives being hidden");',
        'assert!(object.hidden, "change-object-hidden/hides-object-a: object-a is still shown");',
        'assert!(!object.locked, "change-object-hidden/hides-object-a: hiding must not lock the object");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["hidden"].as_bool(), Some(true), "change-object-hidden/hides-object-a: the replacement must carry hidden=true");',
        'assert!(committed["attractions"].is_null(), "change-object-hidden/hides-object-a: hiding an object does not hide its attractions");',
    ],
)

# ─── 🔒change-object-locked ──────────────────────────────────────────────────────────────────
_item = edited(OBJECT_A, locked=True)
case(
    "🔒change-object-locked", "locks-object-a",
    "Sets `object-a`'s `locked` flag. Locking is a document edit here — it produces an ordinary "
    "object patch rather than a presence- or config-lane field.",
    {"mutation": "changeObjectLocked", "id": "object-a", "newLocked": True},
    diff(objects=delta(patched=patch("object-a", copy.deepcopy(_item)))),
    snap_with("objects", 0, _item), APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a survives being locked");',
        'assert!(object.locked, "change-object-locked/locks-object-a: object-a is still unlocked");',
        'assert!(!object.hidden, "change-object-locked/locks-object-a: locking must not hide the object");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["locked"].as_bool(), Some(true), "change-object-locked/locks-object-a: the replacement must carry locked=true");',
        'assert_eq!(committed["objects"]["removed"].as_array().map(Vec::is_empty), Some(true), "change-object-locked/locks-object-a: locking removes nothing");',
    ],
)

# ─── ➕add-object-vortex ─────────────────────────────────────────────────────────────────────
VORTEX_3 = {"id": "vortex-3", "vortexKind": "vortex-kind-b", "position": [0.0, 0.0, 1.0],
            "hidden": False, "locked": False}
_object_b = copy.deepcopy(OBJECT_B)
_object_b["vortices"].append(copy.deepcopy(VORTEX_3))
_after = base()
_after["objects"][1] = _object_b
case(
    "➕add-object-vortex", "appends-vortex-3-to-object-b",
    "Attaches a new rim vortex to `object-b`. A `null` index means the builder inserts at "
    "`vortices.len()`, i.e. appends; the whole owner object is republished as one object patch.",
    {"mutation": "addObjectVortex", "objectId": "object-b", "vortex": copy.deepcopy(VORTEX_3),
     "index": None},
    diff(objects=delta(patched=patch("object-b", copy.deepcopy(_object_b)))),
    _after, APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-b").expect("object-b survives gaining a vortex");',
        'assert_eq!(object.vortices.len(), 2, "add-object-vortex/appends-vortex-3-to-object-b: vortex-3 was not attached");',
        'assert_eq!(object.vortices[1].id, "vortex-3", "add-object-vortex/appends-vortex-3-to-object-b: a null index must append the vortex");',
        'assert_eq!(snapshot.objects[0], before().objects[0], "add-object-vortex/appends-vortex-3-to-object-b: only the owner object may change");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["id"].as_str(), Some("object-b"), "add-object-vortex/appends-vortex-3-to-object-b: the owner object is the patch target");',
        'assert_eq!(committed["objects"]["patched"][0]["patch"]["replacement"]["vortices"][1]["id"].as_str(), Some("vortex-3"), "add-object-vortex/appends-vortex-3-to-object-b: the replacement must carry the appended vortex");',
        'assert!(committed["attractions"].is_null(), "add-object-vortex/appends-vortex-3-to-object-b: a fresh vortex attracts nothing on its own");',
    ],
)

# ─── ➖remove-object-vortex ──────────────────────────────────────────────────────────────────
_object_b = copy.deepcopy(OBJECT_B)
_object_b["vortices"] = []
_after = base()
_after["objects"][1] = _object_b
_after["attractions"] = []
case(
    "➖remove-object-vortex", "removes-vortex-2-and-severs-attraction",
    "Detaching `vortex-2` from `object-b` cascades on the `object-b:vortex-2` full id: "
    "`attraction-1` is attracted to it, so the builder patches the owner AND removes the "
    "attraction in one diff.",
    {"mutation": "removeObjectVortex", "objectId": "object-b", "vortexId": "vortex-2"},
    diff(objects=delta(patched=patch("object-b", copy.deepcopy(_object_b))),
         attractions=delta(removed=["attraction-1"])),
    _after, APPLIED,
    [
        'let object = snapshot.objects.iter().find(|object| object.id == "object-b").expect("object-b survives losing a vortex");',
        'assert!(object.vortices.is_empty(), "remove-object-vortex/removes-vortex-2-and-severs-attraction: vortex-2 is still attached");',
        'assert!(snapshot.attractions.is_empty(), "remove-object-vortex/removes-vortex-2-and-severs-attraction: attraction-1 is attracted to object-b:vortex-2 and must be severed");',
    ],
    [
        'assert_eq!(committed["objects"]["patched"][0]["id"].as_str(), Some("object-b"), "remove-object-vortex/removes-vortex-2-and-severs-attraction: the owner object is patched, not removed");',
        'assert_eq!(committed["attractions"]["removed"][0].as_str(), Some("attraction-1"), "remove-object-vortex/removes-vortex-2-and-severs-attraction: the cascade must remove attraction-1 by id");',
        'assert!(committed["objects"]["removed"].as_array().map(Vec::is_empty).unwrap_or(false), "remove-object-vortex/removes-vortex-2-and-severs-attraction: removing a vortex must never remove its object");',
    ],
)

# ─── 🔌replace-object-vortex ─────────────────────────────────────────────────────────────────
case(
    "🔌replace-object-vortex", "rekind-vortex-1-is-noop",
    "The builder clones the owner object and compares the clone against the original BEFORE "
    "writing the new vortex, so its `next == *object` guard always fires: every "
    "`replace-object-vortex` is a warned no-op with an empty diff. This fixture pins that actual "
    "behaviour, not the intent.",
    {"mutation": "replaceObjectVortex", "objectId": "object-a", "vortexId": "vortex-1",
     "newVortex": {"id": "vortex-1", "vortexKind": "vortex-kind-c", "position": [1.0, 0.0, 0.0],
                   "hidden": False, "locked": False}},
    diff(),
    base(), APPLIED_NOOP,
    [
        'assert_eq!(snapshot, before(), "replace-object-vortex/rekind-vortex-1-is-noop: the builder\'s clone-then-compare guard fires first, so nothing may change");',
        'let object = snapshot.objects.iter().find(|object| object.id == "object-a").expect("object-a is untouched");',
        'assert_eq!(object.vortices[0].vortex_kind.as_deref(), Some("vortex-kind-a"), "replace-object-vortex/rekind-vortex-1-is-noop: vortex-1 must keep its base kind");',
    ],
    [
        'assert!(committed["objects"].is_null(), "replace-object-vortex/rekind-vortex-1-is-noop: the no-op guard must leave the objects delta unset");',
        'assert!(committed.as_object().expect("the committed diff is a JSON object").values().all(serde_json::Value::is_null), "replace-object-vortex/rekind-vortex-1-is-noop: a no-op diff must carry no populated field at all");',
    ],
)

# ─── 🔗connect-vortices ──────────────────────────────────────────────────────────────────────
ATTRACTION_2 = {
    "id": "attraction-2",
    "attracting": "object-a:vortex-spare",
    "attracted": "object-b:vortex-2",
    "gap": 0.0,
    "shift": 0.5,
    "rise": 0.0,
    "rotation": 0.0,
    "turn": 0.0,
    "tilt": 0.0,
    "x": 1.0,
    "y": 2.0,
}
_after = base()
_after["attractions"].append(copy.deepcopy(ATTRACTION_2))
case(
    "🔗connect-vortices", "adds-second-attraction",
    "Attracts `object-a:vortex-spare` to `object-b:vortex-2` as `attraction-2`. The builder "
    "synthesises the attraction from the payload's eight connection parameters and appends it.",
    {"mutation": "connectVortices", "id": "attraction-2", "attracting": "object-a:vortex-spare",
     "attracted": "object-b:vortex-2", "gap": 0.0, "shift": 0.5, "rise": 0.0, "rotation": 0.0,
     "turn": 0.0, "tilt": 0.0, "x": 1.0, "y": 2.0},
    diff(attractions=delta(added=[copy.deepcopy(ATTRACTION_2)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.attractions.len(), 2, "connect-vortices/adds-second-attraction: attraction-2 was not appended");',
        'let attraction = snapshot.attractions.iter().find(|attraction| attraction.id == "attraction-2").expect("attraction-2 exists after connecting");',
        'assert_eq!((attraction.attracting.as_str(), attraction.attracted.as_str()), ("object-a:vortex-spare", "object-b:vortex-2"), "connect-vortices/adds-second-attraction: attraction-2 joins the wrong vortices");',
        'assert_eq!((attraction.shift, attraction.x, attraction.y), (0.5, 1.0, 2.0), "connect-vortices/adds-second-attraction: the initial connection parameters were not carried over");',
    ],
    [
        'assert_eq!(committed["attractions"]["added"][0]["id"].as_str(), Some("attraction-2"), "connect-vortices/adds-second-attraction: the diff must carry attraction-2 in attractions.added");',
        'assert!(committed["objects"].is_null(), "connect-vortices/adds-second-attraction: connecting vortices must not republish either object");',
    ],
)

# ─── ✂️disconnect-vortices ───────────────────────────────────────────────────────────────────
_after = base()
_after["attractions"] = []
case(
    "✂️disconnect-vortices", "removes-attraction-1",
    "Severs `attraction-1`. The builder emits a real `attractions.removed` entry — never a "
    "whole-snapshot capture — and leaves both endpoint objects and their vortices in place.",
    {"mutation": "disconnectVortices", "id": "attraction-1"},
    diff(attractions=delta(removed=["attraction-1"])),
    _after, APPLIED,
    [
        'assert!(snapshot.attractions.is_empty(), "disconnect-vortices/removes-attraction-1: attraction-1 survived the disconnect");',
        'assert_eq!(snapshot.objects, before().objects, "disconnect-vortices/removes-attraction-1: disconnecting must leave both endpoint objects and their vortices intact");',
    ],
    [
        'assert_eq!(committed["attractions"]["removed"][0].as_str(), Some("attraction-1"), "disconnect-vortices/removes-attraction-1: the diff must remove attraction-1 by id");',
        'assert!(committed["objects"].is_null(), "disconnect-vortices/removes-attraction-1: a disconnect must not touch the objects delta");',
    ],
)

# ─── 🧮replace-attraction-geometry ───────────────────────────────────────────────────────────
_item = edited(ATTRACTION_1, gap=2.0, shift=1.0, rise=0.5, x=3.0, y=4.0)
case(
    "🧮replace-attraction-geometry", "repositions-attraction-1",
    "A whole-pose swap on `attraction-1`: the builder writes all eight connection parameters "
    "(gap/shift/rise/rotation/turn/tilt/x/y) from the payload in one gesture.",
    {"mutation": "replaceAttractionGeometry", "id": "attraction-1", "newGap": 2.0,
     "newShift": 1.0, "newRise": 0.5, "newRotation": 0.0, "newTurn": 0.0, "newTilt": 0.0,
     "newX": 3.0, "newY": 4.0},
    diff(attractions=delta(patched=patch("attraction-1", copy.deepcopy(_item)))),
    snap_with("attractions", 0, _item), APPLIED,
    [
        'let attraction = snapshot.attractions.iter().find(|attraction| attraction.id == "attraction-1").expect("attraction-1 survives its repose");',
        'assert_eq!((attraction.gap, attraction.shift, attraction.rise), (2.0, 1.0, 0.5), "replace-attraction-geometry/repositions-attraction-1: the connection offsets are wrong");',
        'assert_eq!((attraction.x, attraction.y), (3.0, 4.0), "replace-attraction-geometry/repositions-attraction-1: the diagram position is wrong");',
        'assert_eq!(attraction.attracting, before().attractions[0].attracting, "replace-attraction-geometry/repositions-attraction-1: a repose must not rewire the endpoints");',
    ],
    [
        'assert_eq!(committed["attractions"]["patched"][0]["id"].as_str(), Some("attraction-1"), "replace-attraction-geometry/repositions-attraction-1: the diff must patch attraction-1");',
        'assert_eq!(committed["attractions"]["patched"][0]["patch"]["replacement"]["rise"].as_f64(), Some(0.5), "replace-attraction-geometry/repositions-attraction-1: the replacement must carry the new rise");',
        'assert!(committed["objects"].is_null(), "replace-attraction-geometry/repositions-attraction-1: an attraction repose never republishes an object");',
    ],
)

# ─── 🌍create-target-volume ──────────────────────────────────────────────────────────────────
VOLUME_2 = {"id": "volume-2", "origin": [10.0, 0.0, 0.0], "hidden": False, "locked": False}
_after = base()
_after["targetVolumes"].append(copy.deepcopy(VOLUME_2))
case(
    "🌍create-target-volume", "appends-volume-2",
    "Adds a second fill-constraining box. `volume-2` carries neither orientation nor scale, so "
    "both `Option` fields stay absent from the wire form the builder puts in `added`.",
    {"mutation": "createTargetVolume", "targetVolume": copy.deepcopy(VOLUME_2), "index": None},
    diff(targetVolumes=delta(added=[copy.deepcopy(VOLUME_2)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.target_volumes.len(), 2, "create-target-volume/appends-volume-2: volume-2 was not appended");',
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-2").expect("volume-2 exists after creation");',
        'assert_eq!((volume.orientation, volume.scale), (None, None), "create-target-volume/appends-volume-2: an unposed volume must not gain a default orientation or scale");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["added"][0]["id"].as_str(), Some("volume-2"), "create-target-volume/appends-volume-2: the diff must carry volume-2 in targetVolumes.added");',
        'assert!(committed["objects"].is_null(), "create-target-volume/appends-volume-2: a target volume is not an object");',
    ],
)

# ─── 🪦delete-target-volume ──────────────────────────────────────────────────────────────────
_after = base()
_after["targetVolumes"] = []
case(
    "🪦delete-target-volume", "removes-volume-1",
    "Removes `volume-1`. Nothing references a target volume, so unlike `delete-object` this "
    "builder has no cascade at all — the diff touches exactly one collection.",
    {"mutation": "deleteTargetVolume", "id": "volume-1"},
    diff(targetVolumes=delta(removed=["volume-1"])),
    _after, APPLIED,
    [
        'assert!(snapshot.target_volumes.is_empty(), "delete-target-volume/removes-volume-1: volume-1 survived the delete");',
        'assert_eq!(snapshot.objects, before().objects, "delete-target-volume/removes-volume-1: deleting a fill constraint must not touch the placed objects");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["removed"][0].as_str(), Some("volume-1"), "delete-target-volume/removes-volume-1: the diff must remove volume-1 by id");',
        'assert!(committed["objects"].is_null() && committed["attractions"].is_null(), "delete-target-volume/removes-volume-1: a target volume delete has no cascade");',
    ],
)

# ─── 🚀move-target-volume ────────────────────────────────────────────────────────────────────
_item = edited(VOLUME_1, origin=[0.0, 0.0, 5.0])
case(
    "🚀move-target-volume", "lifts-volume-1",
    "Lifts `volume-1` five metres up the Z axis. Only `origin` moves; the box keeps its "
    "orientation quaternion and its per-axis scale.",
    {"mutation": "moveTargetVolume", "id": "volume-1", "newOrigin": [0.0, 0.0, 5.0]},
    diff(targetVolumes=delta(patched=patch("volume-1", copy.deepcopy(_item)))),
    snap_with("targetVolumes", 0, _item), APPLIED,
    [
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives its move");',
        'assert_eq!(volume.origin, [0.0, 0.0, 5.0], "move-target-volume/lifts-volume-1: volume-1 did not land on the committed origin");',
        'assert_eq!(volume.scale, before().target_volumes[0].scale, "move-target-volume/lifts-volume-1: a move must not resize the box");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["patched"][0]["id"].as_str(), Some("volume-1"), "move-target-volume/lifts-volume-1: the diff must patch volume-1");',
        'assert_eq!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["origin"][2].as_f64(), Some(5.0), "move-target-volume/lifts-volume-1: the replacement must carry the new z");',
    ],
)

# ─── 🌀rotate-target-volume ──────────────────────────────────────────────────────────────────
_item = edited(VOLUME_1, orientation=[0.0, 0.0, 1.0, 0.0])
case(
    "🌀rotate-target-volume", "half-turn-about-z",
    "Replaces `volume-1`'s orientation quaternion with a half turn about Z, reorienting the "
    "oriented box the fill sessions clip against.",
    {"mutation": "rotateTargetVolume", "id": "volume-1", "newOrientation": [0.0, 0.0, 1.0, 0.0]},
    diff(targetVolumes=delta(patched=patch("volume-1", copy.deepcopy(_item)))),
    snap_with("targetVolumes", 0, _item), APPLIED,
    [
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives its rotation");',
        'assert_eq!(volume.orientation, Some([0.0, 0.0, 1.0, 0.0]), "rotate-target-volume/half-turn-about-z: volume-1 did not take the committed quaternion");',
        'assert_eq!(volume.origin, before().target_volumes[0].origin, "rotate-target-volume/half-turn-about-z: a rotation must not translate the box");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["orientation"][2].as_f64(), Some(1.0), "rotate-target-volume/half-turn-about-z: the replacement must carry the new quaternion");',
        'assert!(committed["objects"].is_null(), "rotate-target-volume/half-turn-about-z: rotating a box never touches an object");',
    ],
)

# ─── 📐scale-target-volume ───────────────────────────────────────────────────────────────────
_item = edited(VOLUME_1, scale=0.5)
case(
    "📐scale-target-volume", "per-axis-to-uniform",
    "`volume-1` carries the `[x, y, z]` triple form of `Puzzle3dScale` in the base; the payload "
    "swaps in the scalar form, so the wire shape collapses from an array to a bare number.",
    {"mutation": "scaleTargetVolume", "id": "volume-1", "newScale": 0.5},
    diff(targetVolumes=delta(patched=patch("volume-1", copy.deepcopy(_item)))),
    snap_with("targetVolumes", 0, _item), APPLIED,
    [
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives its scaling");',
        'assert_eq!(volume.scale, Some(crate::artifacts::puzzle3d::Puzzle3dScale::Uniform(0.5)), "scale-target-volume/per-axis-to-uniform: volume-1 did not take the scalar scale");',
        'assert_eq!(volume.orientation, before().target_volumes[0].orientation, "scale-target-volume/per-axis-to-uniform: scaling must not reorient the box");',
    ],
    [
        'assert!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["scale"].is_number(), "scale-target-volume/per-axis-to-uniform: the uniform form must serialize as a bare number, not an array");',
        'assert!(committed["references"].is_null(), "scale-target-volume/per-axis-to-uniform: scaling a box never touches a reference plane");',
    ],
)

# ─── 🙈change-target-volume-hidden ───────────────────────────────────────────────────────────
_item = edited(VOLUME_1, hidden=True)
case(
    "🙈change-target-volume-hidden", "hides-volume-1",
    "Sets `volume-1`'s `hidden` flag so the fill constraint stops drawing while still "
    "constraining. The flag is a plain `bool`, always present on the wire.",
    {"mutation": "changeTargetVolumeHidden", "id": "volume-1", "newHidden": True},
    diff(targetVolumes=delta(patched=patch("volume-1", copy.deepcopy(_item)))),
    snap_with("targetVolumes", 0, _item), APPLIED,
    [
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives being hidden");',
        'assert!(volume.hidden, "change-target-volume-hidden/hides-volume-1: volume-1 is still shown");',
        'assert!(!volume.locked, "change-target-volume-hidden/hides-volume-1: hiding must not lock the box");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["hidden"].as_bool(), Some(true), "change-target-volume-hidden/hides-volume-1: the replacement must carry hidden=true");',
        'assert_eq!(committed["targetVolumes"]["removed"].as_array().map(Vec::is_empty), Some(true), "change-target-volume-hidden/hides-volume-1: hiding is not deleting");',
    ],
)

# ─── 🔐change-target-volume-locked ───────────────────────────────────────────────────────────
_item = edited(VOLUME_1, locked=True)
case(
    "🔐change-target-volume-locked", "locks-volume-1",
    "Sets `volume-1`'s `locked` flag so the transform gumball can no longer grab the box. The "
    "builder emits an ordinary target-volume patch.",
    {"mutation": "changeTargetVolumeLocked", "id": "volume-1", "newLocked": True},
    diff(targetVolumes=delta(patched=patch("volume-1", copy.deepcopy(_item)))),
    snap_with("targetVolumes", 0, _item), APPLIED,
    [
        'let volume = snapshot.target_volumes.iter().find(|volume| volume.id == "volume-1").expect("volume-1 survives being locked");',
        'assert!(volume.locked, "change-target-volume-locked/locks-volume-1: volume-1 is still unlocked");',
        'assert!(!volume.hidden, "change-target-volume-locked/locks-volume-1: locking must not hide the box");',
    ],
    [
        'assert_eq!(committed["targetVolumes"]["patched"][0]["patch"]["replacement"]["locked"].as_bool(), Some(true), "change-target-volume-locked/locks-volume-1: the replacement must carry locked=true");',
        'assert!(committed["objects"].is_null(), "change-target-volume-locked/locks-volume-1: locking a box must not lock any object");',
    ],
)

# ─── 🖼create-reference ──────────────────────────────────────────────────────────────────────
REFERENCE_2 = {
    "id": "reference-2",
    "source": {"url": "asset://elevation.png", "mediaKind": "image"},
    "origin": [0.0, 0.0, 3.0],
    "widthWorld": 6.0,
    "locked": False,
    "hidden": False,
}
_after = base()
_after["references"].append(copy.deepcopy(REFERENCE_2))
case(
    "🖼create-reference", "appends-reference-2",
    "Pins a second reference plane in world space. A `null` index means the builder emits no "
    "`reordered` order, so `reference-2` lands at the end of `references`.",
    {"mutation": "createReference", "reference": copy.deepcopy(REFERENCE_2), "index": None},
    diff(references=delta(added=[copy.deepcopy(REFERENCE_2)])),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.references.len(), 2, "create-reference/appends-reference-2: reference-2 was not appended");',
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-2").expect("reference-2 exists after creation");',
        'assert_eq!(reference.width_world, 6.0, "create-reference/appends-reference-2: the world width did not survive creation");',
    ],
    [
        'assert_eq!(committed["references"]["added"][0]["source"]["url"].as_str(), Some("asset://elevation.png"), "create-reference/appends-reference-2: the diff must carry the reference source");',
        'assert!(committed["references"]["reordered"].is_null(), "create-reference/appends-reference-2: a null index must leave reordered unset");',
    ],
)

# ─── 🚮delete-reference ──────────────────────────────────────────────────────────────────────
_after = base()
_after["references"] = []
case(
    "🚮delete-reference", "removes-reference-1",
    "Unpins `reference-1`. Like the target-volume delete, nothing references a reference plane, "
    "so the builder emits a single removal with no cascade.",
    {"mutation": "deleteReference", "id": "reference-1"},
    diff(references=delta(removed=["reference-1"])),
    _after, APPLIED,
    [
        'assert!(snapshot.references.is_empty(), "delete-reference/removes-reference-1: reference-1 survived the delete");',
        'assert_eq!(snapshot.objects, before().objects, "delete-reference/removes-reference-1: unpinning a reference plane must not touch the placed objects");',
    ],
    [
        'assert_eq!(committed["references"]["removed"][0].as_str(), Some("reference-1"), "delete-reference/removes-reference-1: the diff must remove reference-1 by id");',
        'assert!(committed["targetVolumes"].is_null(), "delete-reference/removes-reference-1: a reference delete has no cascade");',
    ],
)

# ─── 🎯move-reference ────────────────────────────────────────────────────────────────────────
_item = edited(REFERENCE_1, origin=[2.0, 0.0, 0.0])
case(
    "🎯move-reference", "slides-reference-1",
    "Slides `reference-1` two metres along X. Only `origin` moves; the plane keeps its source "
    "media and its world width.",
    {"mutation": "moveReference", "id": "reference-1", "newOrigin": [2.0, 0.0, 0.0]},
    diff(references=delta(patched=patch("reference-1", copy.deepcopy(_item)))),
    snap_with("references", 0, _item), APPLIED,
    [
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-1").expect("reference-1 survives its move");',
        'assert_eq!(reference.origin, [2.0, 0.0, 0.0], "move-reference/slides-reference-1: reference-1 did not land on the committed origin");',
        'assert_eq!(reference.width_world, before().references[0].width_world, "move-reference/slides-reference-1: a move must not rescale the plane");',
    ],
    [
        'assert_eq!(committed["references"]["patched"][0]["id"].as_str(), Some("reference-1"), "move-reference/slides-reference-1: the diff must patch reference-1");',
        'assert_eq!(committed["references"]["patched"][0]["patch"]["replacement"]["origin"][0].as_f64(), Some(2.0), "move-reference/slides-reference-1: the replacement must carry the new x");',
    ],
)

# ─── 📎resize-reference ──────────────────────────────────────────────────────────────────────
_item = edited(REFERENCE_1, widthWorld=20.0)
case(
    "📎resize-reference", "widens-reference-1",
    "Doubles `reference-1`'s `widthWorld`, the single metre-valued field that sets the plane's "
    "real-world scale. Its origin and source media are untouched.",
    {"mutation": "resizeReference", "id": "reference-1", "newWidthWorld": 20.0},
    diff(references=delta(patched=patch("reference-1", copy.deepcopy(_item)))),
    snap_with("references", 0, _item), APPLIED,
    [
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-1").expect("reference-1 survives its resize");',
        'assert_eq!(reference.width_world, 20.0, "resize-reference/widens-reference-1: the plane did not take the committed world width");',
        'assert_eq!(reference.origin, before().references[0].origin, "resize-reference/widens-reference-1: a resize must not move the plane");',
    ],
    [
        'assert_eq!(committed["references"]["patched"][0]["patch"]["replacement"]["widthWorld"].as_f64(), Some(20.0), "resize-reference/widens-reference-1: the replacement must carry the new world width");',
        'assert!(committed["objects"].is_null(), "resize-reference/widens-reference-1: resizing a plane never touches an object");',
    ],
)

# ─── 🖇replace-reference-source ──────────────────────────────────────────────────────────────
_item = edited(REFERENCE_1, source={"url": "asset://plan-v2.png", "mediaKind": "image"})
case(
    "🖇replace-reference-source", "repoints-reference-1-source",
    "Swaps the whole `source` block of `reference-1` — url and media kind together, one gesture — "
    "while leaving the plane pinned exactly where it was.",
    {"mutation": "replaceReferenceSource", "id": "reference-1",
     "newSource": {"url": "asset://plan-v2.png", "mediaKind": "image"}},
    diff(references=delta(patched=patch("reference-1", copy.deepcopy(_item)))),
    snap_with("references", 0, _item), APPLIED,
    [
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-1").expect("reference-1 survives its source swap");',
        'assert_eq!(reference.source.url, "asset://plan-v2.png", "replace-reference-source/repoints-reference-1-source: the plane still points at the old asset");',
        'assert_eq!(reference.origin, before().references[0].origin, "replace-reference-source/repoints-reference-1-source: a source swap must not move the plane");',
    ],
    [
        'assert_eq!(committed["references"]["patched"][0]["patch"]["replacement"]["source"]["url"].as_str(), Some("asset://plan-v2.png"), "replace-reference-source/repoints-reference-1-source: the replacement must carry the new source url");',
        'assert_eq!(committed["references"]["patched"].as_array().map(Vec::len), Some(1), "replace-reference-source/repoints-reference-1-source: exactly one reference may be patched");',
    ],
)

# ─── 👀change-reference-hidden ───────────────────────────────────────────────────────────────
_item = edited(REFERENCE_1, hidden=True)
case(
    "👀change-reference-hidden", "hides-reference-1",
    "Sets `reference-1`'s `hidden` flag so the tracing plane stops drawing. The flag is a plain "
    "`bool` on the reference record, always present on the wire.",
    {"mutation": "changeReferenceHidden", "id": "reference-1", "newHidden": True},
    diff(references=delta(patched=patch("reference-1", copy.deepcopy(_item)))),
    snap_with("references", 0, _item), APPLIED,
    [
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-1").expect("reference-1 survives being hidden");',
        'assert!(reference.hidden, "change-reference-hidden/hides-reference-1: reference-1 is still shown");',
        'assert!(!reference.locked, "change-reference-hidden/hides-reference-1: hiding must not lock the plane");',
    ],
    [
        'assert_eq!(committed["references"]["patched"][0]["patch"]["replacement"]["hidden"].as_bool(), Some(true), "change-reference-hidden/hides-reference-1: the replacement must carry hidden=true");',
        'assert_eq!(committed["references"]["removed"].as_array().map(Vec::is_empty), Some(true), "change-reference-hidden/hides-reference-1: hiding is not unpinning");',
    ],
)

# ─── 🗝change-reference-locked ───────────────────────────────────────────────────────────────
_item = edited(REFERENCE_1, locked=True)
case(
    "🗝change-reference-locked", "locks-reference-1",
    "Sets `reference-1`'s `locked` flag so the plane can no longer be dragged while tracing over "
    "it. The builder emits an ordinary reference patch.",
    {"mutation": "changeReferenceLocked", "id": "reference-1", "newLocked": True},
    diff(references=delta(patched=patch("reference-1", copy.deepcopy(_item)))),
    snap_with("references", 0, _item), APPLIED,
    [
        'let reference = snapshot.references.iter().find(|reference| reference.id == "reference-1").expect("reference-1 survives being locked");',
        'assert!(reference.locked, "change-reference-locked/locks-reference-1: reference-1 is still unlocked");',
        'assert!(!reference.hidden, "change-reference-locked/locks-reference-1: locking must not hide the plane");',
    ],
    [
        'assert_eq!(committed["references"]["patched"][0]["patch"]["replacement"]["locked"].as_bool(), Some(true), "change-reference-locked/locks-reference-1: the replacement must carry locked=true");',
        'assert!(committed["targetVolumes"].is_null(), "change-reference-locked/locks-reference-1: locking a plane must not lock a fill volume");',
    ],
)

# ─── 🌐change-domain ─────────────────────────────────────────────────────────────────────────
_after = base()
_after["domain"] = "engineering"
case(
    "🌐change-domain", "architecture-to-engineering",
    "`domain` is a document-root scalar, so the builder has no missing-target branch and no "
    "collection delta: it publishes the new string on the diff's own `domain` field.",
    {"mutation": "changeDomain", "newDomain": "engineering"},
    diff(domain="engineering"),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.domain, "engineering", "change-domain/architecture-to-engineering: the document domain did not change");',
        'assert_eq!(snapshot.objects, before().objects, "change-domain/architecture-to-engineering: a domain change must not touch any placed object");',
    ],
    [
        'assert_eq!(committed["domain"].as_str(), Some("engineering"), "change-domain/architecture-to-engineering: the diff must carry the new domain as a scalar field");',
        'assert!(committed["objects"].is_null() && committed["meta"].is_null(), "change-domain/architecture-to-engineering: a scalar document edit touches no collection and no meta block");',
    ],
)

# ─── 🤝connect-kind-compatibility ────────────────────────────────────────────────────────────
COMPAT_BC = {"source": "vortex-kind-b", "target": "vortex-kind-c", "bidirectional": False,
             "important": True, "specificity": "cable"}
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCompatibility"] = [copy.deepcopy(COMPAT_AB), copy.deepcopy(COMPAT_BC)]
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "🤝connect-kind-compatibility", "adds-vortex-kind-pair",
    "Appends a `vortex-kind-b -> vortex-kind-c` allowance to `meta.kindCompatibility`. The builder "
    "PUSHES the row and republishes the whole `meta` block — puzzle3d's diff has no meta delta.",
    {"mutation": "connectKindCompatibility", "source": "vortex-kind-b", "target": "vortex-kind-c",
     "bidirectional": False, "important": True, "specificity": "cable"},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'assert_eq!(snapshot.meta.kind_compatibility.len(), 2, "connect-kind-compatibility/adds-vortex-kind-pair: the new allowance was not appended");',
        'let row = &snapshot.meta.kind_compatibility[1];',
        'assert_eq!((row.source.as_str(), row.target.as_str()), ("vortex-kind-b", "vortex-kind-c"), "connect-kind-compatibility/adds-vortex-kind-pair: the row must be pushed at the end, in payload order");',
        'assert!(row.important && !row.bidirectional, "connect-kind-compatibility/adds-vortex-kind-pair: the payload flags must be carried onto the row verbatim");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCompatibility"].as_array().map(Vec::len), Some(2), "connect-kind-compatibility/adds-vortex-kind-pair: the republished meta must carry both rows");',
        'assert_eq!(committed["meta"]["kindCompatibility"][1]["specificity"].as_str(), Some("cable"), "connect-kind-compatibility/adds-vortex-kind-pair: the specificity must survive onto the new row");',
    ],
)

# ─── 💔disconnect-kind-compatibility ─────────────────────────────────────────────────────────
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCompatibility"] = []
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "💔disconnect-kind-compatibility", "removes-vortex-kind-pair",
    "Revokes the `vortex-kind-a -> vortex-kind-b` allowance. The builder retains every row whose "
    "source/target pair does not match, then republishes `meta` wholesale.",
    {"mutation": "disconnectKindCompatibility", "source": "vortex-kind-a",
     "target": "vortex-kind-b"},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'assert!(snapshot.meta.kind_compatibility.is_empty(), "disconnect-kind-compatibility/removes-vortex-kind-pair: the allowance was not revoked");',
        'assert_eq!(snapshot.attractions, before().attractions, "disconnect-kind-compatibility/removes-vortex-kind-pair: revoking a kind allowance must not sever an existing attraction");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCompatibility"].as_array().map(Vec::is_empty), Some(true), "disconnect-kind-compatibility/removes-vortex-kind-pair: the republished meta must carry an empty table");',
        'assert!(committed["attractions"].is_null(), "disconnect-kind-compatibility/removes-vortex-kind-pair: no attraction cascade may appear in this diff");',
    ],
)

# ─── 📚replace-kind-catalogs ─────────────────────────────────────────────────────────────────
CATALOGS = {
    "objects": [],
    "vortices": [
        {
            "id": "vortex-kind-a",
            "compatibleWith": ["vortex-kind-b"],
            "description": "Alpha port",
            "icon": "circle",
            "color": "#3366ff",
            "defaultCableKind": "cable-kind-a",
        }
    ],
    "cables": [],
    "attractions": [],
}
_meta = copy.deepcopy(BASE["meta"])
_meta["kindCatalogs"] = copy.deepcopy(CATALOGS)
_after = base()
_after["meta"] = copy.deepcopy(_meta)
case(
    "📚replace-kind-catalogs", "installs-vortex-kind-catalog",
    "A whole-value swap of the typed catalog bundle: the base carries `None`, so the payload's "
    "objects/vortices/cables/attractions bundle is installed as one manifest-import gesture.",
    {"mutation": "replaceKindCatalogs", "newCatalogs": copy.deepcopy(CATALOGS)},
    diff(meta=copy.deepcopy(_meta)),
    _after, APPLIED,
    [
        'let catalogs = snapshot.meta.kind_catalogs.as_ref().expect("replace-kind-catalogs/installs-vortex-kind-catalog: the catalog bundle was not installed");',
        'assert_eq!(catalogs.vortices.len(), 1, "replace-kind-catalogs/installs-vortex-kind-catalog: the vortex-kind catalog is empty");',
        'assert_eq!(catalogs.vortices[0].default_cable_kind, "cable-kind-a", "replace-kind-catalogs/installs-vortex-kind-catalog: the catalog row did not survive the swap");',
        'assert_eq!(snapshot.meta.kind_compatibility, before().meta.kind_compatibility, "replace-kind-catalogs/installs-vortex-kind-catalog: installing catalogs must not rewrite the compatibility table");',
    ],
    [
        'assert_eq!(committed["meta"]["kindCatalogs"]["vortices"][0]["id"].as_str(), Some("vortex-kind-a"), "replace-kind-catalogs/installs-vortex-kind-catalog: the diff must publish the catalogs on meta");',
        'assert!(committed["objects"].is_null(), "replace-kind-catalogs/installs-vortex-kind-catalog: a catalog swap must not republish any placed object");',
    ],
)


HEADER = '''//! 🧪️ `{leaf_slug}` fixture — `{case}`.
//!
{summary}
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::puzzle3d::mutations::{{apply_puzzle3d_mutation, inverse_puzzle3d_mutation}};
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Puzzle3dSnapshot {{
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Puzzle3dSnapshot {{
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Puzzle3dMutation {{
    serde_json::from_str(MUTATION).expect("mutation decodes")
}}
'''

BODY = '''
/// ▶️ The committed `{leaf_slug}` payload carries `before` to exactly the committed `after`, and
/// lands the change this case is named for.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_puzzle3d_mutation(&mut snapshot, &mutation()).expect("{leaf_slug} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{leaf_slug}/{case}: applied state differs from committed after-snapshot");
{after_asserts}
}}

/// ↩️ Applying `{leaf_slug}` then the inverse it derives from `before` restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_puzzle3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_puzzle3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_puzzle3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{leaf_slug}/{case}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed `{leaf_slug}` payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Puzzle3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed {{label}} JSON is not canonical");
    }}
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome matches what `{leaf_slug}` actually produces on this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_puzzle3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => assert!(applied, "{leaf_slug}/{case}: declared applied but the mutation was rejected"),
        "rejected" => {{
            assert!(!applied, "{leaf_slug}/{case}: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "{leaf_slug}/{case}: rejected mutation must leave the snapshot untouched");
        }}
        other => panic!("{leaf_slug}/{case}: unknown outcome status {{other:?}}"),
    }}
{outcome_messages}}}

/// 🔺️ The sparse delta `{leaf_slug}` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields this mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let base = before();
    let outcome = <Puzzle3dMutation as protocol::Mutation<Puzzle3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{leaf_slug}/{case}: produced diff differs from the committed 🔺️diff/🔣️component.json");
{diff_asserts}
}}

/// 🔣️ The committed `{leaf_slug}` diff is itself canonical and decodes to `Puzzle3dDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{leaf_slug}/{case}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed `{leaf_slug}` diff directly to `before` yields the committed `after` —
/// the diff is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::puzzle3d::diff::Puzzle3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::puzzle3d::diff::Puzzle3dDiff as protocol::MutationDiff<Puzzle3dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{leaf_slug}/{case}: committed diff did not carry before to after");
}}
'''

NOOP_OUTCOME_CHECK = '''    let messages = outcome.get("messages").and_then(serde_json::Value::as_array).expect("{leaf_slug}/{case}: this case declares a warn no-op and must list it");
    assert_eq!(messages[0]["code"].as_str(), Some("mutation.no-op"), "{leaf_slug}/{case}: the declared message must be the no-op warning the builder raises");
'''


def write(path, payload):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def main():
    for entry in CASES:
        case_dir = os.path.join(TREE, entry["leaf"], "🧪️tests", entry["name"])
        write(os.path.join(case_dir, "📸️snapshot/⬅️before/🔣️component.json"), entry["before"])
        write(os.path.join(case_dir, "📸️snapshot/➡️after/🔣️component.json"), entry["after"])
        write(os.path.join(case_dir, "🦠️mutation/🔣️component.json"), entry["mutation"])
        write(os.path.join(case_dir, "🔺️diff/🔣️component.json"), entry["diff"])
        write(os.path.join(case_dir, "🎯️outcome/🔣️component.json"), entry["outcome"])

        leaf_slug = entry["leaf"]
        while leaf_slug and not leaf_slug[0].isalpha():
            leaf_slug = leaf_slug[1:]
        summary = "\n".join("//! " + line for line in textwrap.wrap(entry["summary"], 96))
        outcome_messages = ""
        if entry["outcome"].get("messages"):
            outcome_messages = NOOP_OUTCOME_CHECK.format(leaf_slug=leaf_slug, case=entry["name"])
        text = HEADER.format(leaf_slug=leaf_slug, case=entry["name"], summary=summary) + BODY.format(
            leaf_slug=leaf_slug,
            case=entry["name"],
            outcome_messages=outcome_messages,
            after_asserts="\n".join("    " + line for line in entry["after_asserts"]),
            diff_asserts="\n".join("    " + line for line in entry["diff_asserts"]),
        )
        with open(os.path.join(case_dir, "🦀️component.rs"), "w", encoding="utf-8") as handle:
            handle.write(text)
        print(f"{entry['leaf']}\t{entry['name']}")


if __name__ == "__main__":
    main()
