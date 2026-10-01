#!/usr/bin/env python3
"""🧫️ W3-T-LAYOUT authoring tool: writes the committed fixture quintets of the three layout frame-selection leaves
(`drag-frames`, `rotate-frames`, `scale-frames`), their Rust fixture tests and their crate-root test mounts.

The leaf semantics are implemented HERE, independently of the Rust leaves, in the same IEEE evaluation order, so the
committed after-snapshots and sparse diffs are what the Rust diff builders must reproduce bit for bit. Two third-party
checks guard the implementation before anything is written: `numpy` recomputes every moved frame as a matrix transform
(rotation matrix / scale matrix about the pivot) and must agree within 1e-9, and `jsonschema` validates every payload
against its leaf schema — every accepted vector must pass, every `mutation.invariant` vector must FAIL (design §11 negative
witness). Re-running reproduces every committed file byte for byte.

Usage: .venv/bin/python 🧪️w3-t-layout-author-vectors.py [--apply]
"""
import copy
import hashlib
import json
import math
import os
import re
import sys

import jsonschema
import numpy

REPO = "/Users/ueli/Documents/semio"
ARTIFACT = f"{REPO}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout"
SUBSET = f"{ARTIFACT}/🏅️standards/🔖️1/🪆️subsets/✳️any"
SUBSET_REL = "🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = f"{SUBSET}/🧬️schema/🧬️mutations"
FIXTURES = f"{SUBSET}/🧫️fixtures/🧬️mutations"
BASE = f"{FIXTURES}/🕹️move-frame/📍️moves-the-rect-frame/📸️snapshot/⬅️before/🔣️.json"
APPLY = "--apply" in sys.argv
LEAVES = {"drag-frames": ("✋️", "DragFrames"), "rotate-frames": ("🔃️", "RotateFrames"), "scale-frames": ("🗜️", "ScaleFrames")}
PAGE_PATCH_KEYS = ["name", "width", "height", "margin_top", "margin_right", "margin_bottom", "margin_left", "columns_count", "columns_gutter", "frame_added", "frame_removed", "frames_patched", "frame_layer", "frame_order", "guides", "layer_added", "layer_patched", "layer_removed", "overrides", "parent_page_id"]
FRAME_PATCH_KEYS = ["x", "y", "width", "height", "rotation", "fill", "stroke", "wrap_mode", "columns", "locked", "visible", "inset_height", "inset_width", "inset_x", "inset_y", "story_id", "thread_next"]
DIFF_KEYS = ["artifact", "schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFieldsJson", "backgroundDrawing", "referencedModel"]
written = []


#region 🧮️Semantics
def centre(bounds):
    return bounds["x"] + bounds["w"] * 0.5, bounds["y"] + bounds["h"] * 0.5


def moved_bounds(kind, payload, bounds):
    """🧮️ One frame's bounds after the leaf, in the Rust evaluation order (left to right, no fused operations)."""
    if kind == "drag-frames":
        return dict(bounds, x=bounds["x"] + payload["dx"], y=bounds["y"] + payload["dy"]), ["x", "y"]
    cx, cy = centre(bounds)
    if kind == "rotate-frames":
        sin, cos = math.sin(payload["angle"]), math.cos(payload["angle"])
        ox, oy = cx - payload["pivotX"], cy - payload["pivotY"]
        tx, ty = payload["pivotX"] + ox * cos - oy * sin, payload["pivotY"] + ox * sin + oy * cos
        return dict(bounds, x=tx - bounds["w"] * 0.5, y=ty - bounds["h"] * 0.5, rotation=bounds["rotation"] + payload["angle"]), ["x", "y", "rotation"]
    width, height = bounds["w"] * payload["sx"], bounds["h"] * payload["sy"]
    sx_, sy_ = payload["pivotX"] + (cx - payload["pivotX"]) * payload["sx"], payload["pivotY"] + (cy - payload["pivotY"]) * payload["sy"]
    return dict(bounds, x=sx_ - width * 0.5, y=sy_ - height * 0.5, w=width, h=height), ["x", "y", "width", "height"]


def numpy_bounds(kind, payload, bounds):
    """🔢️ The third-party cross-check: the same transform as homogeneous matrices about the pivot."""
    if kind == "drag-frames":
        return [bounds["x"] + payload["dx"], bounds["y"] + payload["dy"], bounds["w"], bounds["h"], bounds["rotation"]]
    pivot = numpy.array([payload["pivotX"], payload["pivotY"]])
    middle = numpy.array(centre(bounds))
    if kind == "rotate-frames":
        angle = payload["angle"]
        matrix = numpy.array([[numpy.cos(angle), -numpy.sin(angle)], [numpy.sin(angle), numpy.cos(angle)]])
        turned = pivot + matrix @ (middle - pivot)
        return [turned[0] - bounds["w"] / 2, turned[1] - bounds["h"] / 2, bounds["w"], bounds["h"], bounds["rotation"] + angle]
    matrix = numpy.diag([payload["sx"], payload["sy"]])
    size = matrix @ numpy.array([bounds["w"], bounds["h"]])
    scaled = pivot + matrix @ (middle - pivot)
    return [scaled[0] - size[0] / 2, scaled[1] - size[1] / 2, size[0], size[1], bounds["rotation"]]


def invariant(kind, payload):
    targets = payload["targets"]
    if not targets or len(set(targets)) != len(targets):
        return True
    return kind == "scale-frames" and (payload["sx"] <= 0.0 or payload["sy"] <= 0.0)


def outcome(kind, payload, before):
    """🧮️ `(after, diff | None, outcome)` of one leaf on `before` — the frozen 9-code vocabulary only."""
    targets = payload["targets"]
    if invariant(kind, payload):
        return before, None, {"code": "mutation.invariant", "path": targets, "status": "rejected"}
    page = next((page for page in before["pages"] if page["id"] == payload["pageId"]), None)
    if page is None:
        return before, None, {"code": "mutation.target-missing", "path": [payload["pageId"]], "status": "rejected"}
    layers = {layer["id"]: layer for layer in page["layers"]}
    locked = lambda frame: frame.get("locked") is True or layers.get(frame["layerId"], {}).get("locked", False)
    missing = [target for target in targets if not any(frame["id"] == target for frame in page["frames"])]
    held = [frame["id"] for frame in page["frames"] if frame["id"] in targets and locked(frame)]
    movable = [frame for frame in page["frames"] if frame["id"] in targets and not locked(frame)]
    if not movable:
        return before, None, {"code": "mutation.target-missing", "path": targets, "status": "rejected"}
    messages = [{"code": "mutation.partial", "level": "warn", "target": ids} for ids in (missing, held) if ids]
    after = copy.deepcopy(before)
    after_page = next(page for page in after["pages"] if page["id"] == payload["pageId"])
    patched = []
    for frame in movable:
        bounds, fields = moved_bounds(kind, payload, frame["bounds"])
        reference = numpy_bounds(kind, payload, frame["bounds"])
        assert numpy.allclose([bounds["x"], bounds["y"], bounds["w"], bounds["h"], bounds["rotation"]], reference, rtol=0.0, atol=1e-9), (kind, frame["id"], bounds, reference)
        if bounds == frame["bounds"]:
            continue
        next(item for item in after_page["frames"] if item["id"] == frame["id"])["bounds"] = bounds
        patch = {key: None for key in FRAME_PATCH_KEYS}
        for field in fields:
            patch[field] = bounds[{"width": "w", "height": "h"}.get(field, field)]
        patched.append({"frame_id": frame["id"], "patch": patch})
    if not patched:
        return before, {key: None for key in DIFF_KEYS}, {"messages": messages + [{"code": "mutation.no-op", "level": "warn", "target": targets}], "status": "no-op"}
    page_patch = {key: None for key in PAGE_PATCH_KEYS}
    page_patch["frames_patched"] = patched
    diff = {key: None for key in DIFF_KEYS}
    diff["pages"] = {"added": [], "removed": [], "patched": [{"id": payload["pageId"], "patch": page_patch}], "reordered": None}
    return after, diff, ({"messages": messages, "status": "applied"} if messages else {"status": "applied"})
#endregion 🧮️Semantics


#region 🧫️Vectors
def base(lock_text=False):
    document = json.load(open(BASE, encoding="utf-8"))
    if lock_text:
        next(frame for frame in document["pages"][0]["frames"] if frame["id"] == "frame-text")["locked"] = True
    return document


def drag(targets, dx, dy, page="page-1"):
    return {"pageId": page, "targets": targets, "dx": dx, "dy": dy}


def turn(targets, pivot, angle, page="page-1"):
    return {"pageId": page, "targets": targets, "pivotX": pivot[0], "pivotY": pivot[1], "angle": angle}


def scale(targets, pivot, sx, sy, page="page-1"):
    return {"pageId": page, "targets": targets, "pivotX": pivot[0], "pivotY": pivot[1], "sx": sx, "sy": sy}


BOTH = ["frame-rect", "frame-text"]
CASES = [
    ("drag-frames", "✋️", "drags-both-frames", False, drag(BOTH, 16.0, -8.0), "The rect and the text frame of page 1 both move 16 right and 8 up; their extents and rotations stay."),
    ("drag-frames", "⚠️", "skips-a-locked-and-a-missing-frame", True, drag(["frame-ghost", "frame-rect", "frame-text"], 5.0, 2.5), "Only the rect moves; the absent `frame-ghost` and the locked text frame are skipped with one Warning-level `mutation.partial` per reason."),
    ("drag-frames", "🚫️", "rejects-missing-frames", False, drag(["frame-ghost"], 1.0, 1.0), "No target is a frame of page 1: Error-level `mutation.target-missing`, nothing moves."),
    ("drag-frames", "⏸️", "keeps-a-zero-offset", False, drag(["frame-rect"], 0.0, 0.0), "A zero offset moves nothing: Warning-level `mutation.no-op` and the default diff."),
    ("drag-frames", "🔁️", "refuses-a-repeated-frame", False, drag(["frame-rect", "frame-rect"], 4.0, 0.0), "A target named twice is what the schema's `uniqueItems` forbids: a Fatal `mutation.invariant`, nothing moves."),
    ("rotate-frames", "🔃️", "orbits-both-frames-a-quarter-turn", False, turn(BOTH, (75.0, 100.0), math.pi / 2), "A quarter turn about the centroid of both frame centres: each centre orbits the pivot and each rotation grows by the angle; the extents stay."),
    ("rotate-frames", "🌀️", "turns-the-rect-about-its-centre", False, turn(["frame-rect"], (50.0, 50.0), 0.5), "A turn about the rect's own centre leaves its origin where it is and only grows its rotation."),
    ("rotate-frames", "⚠️", "skips-a-locked-and-a-missing-frame", True, turn(["frame-rect", "frame-text", "frame-ghost"], (50.0, 50.0), 0.25), "Only the rect turns; the locked text frame and the absent `frame-ghost` are skipped with one Warning-level `mutation.partial` per reason."),
    ("rotate-frames", "🚫️", "rejects-a-missing-page", False, turn(["frame-rect"], (50.0, 50.0), 1.0, page="page-ghost"), "The addressed page does not exist: Error-level `mutation.target-missing` at the page, nothing turns."),
    ("rotate-frames", "⏸️", "keeps-a-zero-angle", False, turn(BOTH, (75.0, 100.0), 0.0), "A zero angle turns nothing: Warning-level `mutation.no-op` and the default diff."),
    ("rotate-frames", "🔁️", "refuses-a-repeated-frame", False, turn(["frame-text", "frame-text"], (100.0, 150.0), 0.5), "A target named twice is what the schema's `uniqueItems` forbids: a Fatal `mutation.invariant`, nothing turns."),
    ("scale-frames", "🗜️", "doubles-both-frames-about-their-centroid", False, scale(BOTH, (75.0, 100.0), 2.0, 2.0), "Doubling about the centroid of both frame centres moves each centre twice as far from the pivot and doubles each extent."),
    ("scale-frames", "↔️", "stretches-the-rect-sideways", False, scale(["frame-rect"], (50.0, 50.0), 1.5, 1.0), "Stretching the rect by 1.5 along x about its own centre widens it symmetrically; its height and centre stay."),
    ("scale-frames", "⚠️", "skips-a-locked-and-a-missing-frame", True, scale(["frame-text", "frame-ghost", "frame-rect"], (50.0, 50.0), 0.5, 0.5), "Only the rect shrinks; the absent `frame-ghost` and the locked text frame are skipped with one Warning-level `mutation.partial` per reason."),
    ("scale-frames", "🚫️", "rejects-missing-frames", False, scale(["frame-ghost"], (0.0, 0.0), 2.0, 2.0), "No target is a frame of page 1: Error-level `mutation.target-missing`, nothing scales."),
    ("scale-frames", "⏸️", "keeps-unit-factors", False, scale(["frame-rect"], (50.0, 50.0), 1.0, 1.0), "Unit factors scale nothing: Warning-level `mutation.no-op` and the default diff."),
    ("scale-frames", "🫓️", "refuses-a-zero-factor", False, scale(["frame-rect"], (50.0, 50.0), 0.0, 1.0), "A zero factor would collapse the rect to a line; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`, nothing scales."),
]
#endregion 🧫️Vectors


#region ✍️Writers
def dump(path, value):
    text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    if os.path.exists(path) and open(path, encoding="utf-8").read() == text:
        return
    if APPLY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
    written.append(path)


def write_text(path, text):
    if os.path.exists(path) and open(path, encoding="utf-8").read() == text:
        return
    if APPLY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
    written.append(path)


def module_of(slug):
    return "tests_" + slug.replace("-", "_")


RUST_HEAD = '''//! 🧪️ `{kind}` fixture — `{folder}`.
//!
//! {story}
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/{leaf}/{folder}/` (contract D1), authored by
//! the independent implementation `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-layout-author-vectors.py`.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{{Mutation, MutationDiff}};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/🦠️mutation/🔣️.json");
{diff_const}
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {{
    dsl::os_pack::from_json_str(BEFORE).expect("{kind}/{slug}: before snapshot decodes")
}}
fn expected_after() -> LayoutSnapshot {{
    dsl::os_pack::from_json_str(AFTER).expect("{kind}/{slug}: after snapshot decodes")
}}
fn mutation() -> LayoutMutation {{
    dsl::os_pack::from_json_str(MUTATION).expect("{kind}/{slug}: mutation decodes")
}}
fn outcome() -> serde_json::Value {{
    serde_json::from_str(OUTCOME).expect("{kind}/{slug}: outcome decodes")
}}
fn applied() -> LayoutSnapshot {{
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("{kind}/{slug}: the diff applies to its committed before-snapshot")
}}

/// 🗣️ `(level, code, target)` of every message `{kind}` raises on the committed base.
fn produced_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {{
    mutation().diff(&before()).messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {{
    let level = |text: &str| match text {{
        "info" => protocol::Severity::Info,
        "warn" => protocol::Severity::Warning,
        "error" => protocol::Severity::Error,
        "fatal" => protocol::Severity::Fatal,
        other => panic!("{kind}/{slug}: unknown message level {{other:?}}"),
    }};
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    let outcome = outcome();
    if outcome["status"].as_str() == Some("rejected") {{
        let fatal = outcome["code"].as_str() == Some("mutation.invariant");
        return vec![(if fatal {{ protocol::Severity::Fatal }} else {{ protocol::Severity::Error }}, outcome["code"].as_str().expect("a code").to_string(), strings(&outcome["path"]))];
    }}
    outcome.get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {{
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    }})
}}

/// 🔣️ Both committed snapshots and the committed payload are canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: LayoutSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "{kind}/{slug}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "{kind}/{slug}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — status, codes, levels and addressed targets — is exactly what `{kind}`'s diff raises.
#[test]
fn declared_outcome_holds() {{
    assert_eq!(produced_messages(), declared_messages(), "{kind}/{slug}: the produced messages differ from the declared outcome");
}}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    assert_eq!(applied(), expected_after(), "{kind}/{slug}: applied state differs from the committed after-snapshot");
}}
'''

RUST_MOVING = '''
/// 🔺️ The sparse delta `{kind}` produces is exactly the committed diff: WHICH frames of the page it patches, in page
/// order, and which bounds fields of each.
#[test]
fn produces_committed_diff() {{
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "{kind}/{slug}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🩹 The committed diff decodes to `LayoutDiff`, re-encodes byte for byte, and carries `before` to `after` on its own.
#[test]
fn committed_diff_is_canonical_and_complete() {{
    let decoded: crate::LayoutDiff = dsl::os_pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("committed diff re-encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff reparses"), "{kind}/{slug}: committed diff JSON is not canonical");
    assert_eq!(decoded.apply(&before()).expect("committed diff applies"), expected_after(), "{kind}/{slug}: committed diff did not carry before to after");
}}
'''

RUST_APPLIED = '''
/// ↩️ Applying the payload then every step of the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is the absolute setters of the base bounds, never a negated parameter.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let inverse = mutation().inverse(&base);
    assert!(!inverse.is_empty(), "{kind}/{slug}: a moving vector must have something to undo");
    let mut snapshot = applied();
    assert_ne!(snapshot, base, "{kind}/{slug}: an applied vector must move a frame");
    for step in &inverse {{
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("an inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{slug}: the inverse did not restore the before-snapshot");
}}
'''

RUST_UNMOVED = '''
/// 🧾️ A vector that moves nothing leaves the committed `after` equal to `before` and has nothing to undo{absent}.
#[test]
fn moves_nothing_and_has_nothing_to_undo() {{
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a vector that moves nothing commits two equal snapshots");
    assert!(mutation().inverse(&before()).is_empty(), "{kind}/{slug}: nothing moved, so nothing is undone");{absent_check}
}}
'''


def rust_test(kind, leaf, folder, slug, story, status):
    rejected = status == "rejected"
    diff_const = f'const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/🔺️diff/🚫️.absent");' if rejected else f'const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{folder}/🔺️diff/🔣️.json");'
    text = RUST_HEAD.format(kind=kind, leaf=leaf, folder=folder, slug=slug, story=story, diff_const=diff_const)
    if not rejected:
        text += RUST_MOVING.format(kind=kind, slug=slug)
    if status == "applied":
        text += RUST_APPLIED.format(kind=kind, slug=slug)
    else:
        absent = ", and its D6 sentinel `🔺️diff/🚫️.absent` stays empty while its diff is the default one" if rejected else ""
        check = f'\n    assert!(DIFF_ABSENT.is_empty(), "{kind}/{slug}: the D6 sentinel must stay empty");\n    assert_eq!(mutation().diff(&before()).diff(), &crate::LayoutDiff::default(), "{kind}/{slug}: a refusal carries the default diff");' if rejected else ""
        text += RUST_UNMOVED.format(kind=kind, slug=slug, absent=absent, absent_check=check)
    return text


def mount(cases):
    path = f"{ARTIFACT}/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    blocks = []
    for kind, (emoji, _variant) in LEAVES.items():
        module = kind.replace("-", "_")
        lines = [
            "                        #[path = \".\"]",
            f"                        pub mod {module} {{",
            f"                            #[path = \"{SUBSET_REL}/🧬️schema/🧬️mutations/{emoji}{kind}/🦀️.rs\"]",
            "                            mod component;",
            "                            pub use component::*;",
        ]
        for case_kind, case_emoji, slug, *_ in cases:
            if case_kind != kind:
                continue
            lines += [
                "                            #[cfg(test)]",
                f"                            #[path = \"{SUBSET_REL}/🧬️schema/🧬️mutations/{emoji}{kind}/🧪️tests/{case_emoji}{slug}/🦀️.rs\"]",
                f"                            mod {module_of(slug)};",
            ]
        lines.append("                        }")
        blocks.append("\n".join(lines))
    region = "\n".join(blocks) + "\n"
    start_marker = "                        #[path = \".\"]\n                        pub mod drag_frames {\n"
    anchor = "                        pub mod reorder_frame {\n"
    if start_marker in text:
        start = text.index(start_marker)
        end = text.index("                        #[path = \".\"]\n", text.index("                        pub mod scale_frames {\n"))
        after = text[:start] + region + text[end:]
    else:
        at = text.index(anchor)
        close = text.index("                        }\n", at) + len("                        }\n")
        after = text[:close] + region + text[close:]
    write_text(path, after)
#endregion ✍️Writers


def main():
    schemas = {kind: json.load(open(f"{MUTATIONS}/{emoji}{kind}/🧬️schema/🔣️.json", encoding="utf-8")) for kind, (emoji, _variant) in LEAVES.items()}
    for kind, emoji, slug, lock_text, payload, story in CASES:
        leaf = f"{LEAVES[kind][0]}{kind}"
        folder = f"{emoji}{slug}"
        before = base(lock_text)
        after, diff, result = outcome(kind, payload, before)
        valid = jsonschema.Draft7Validator(schemas[kind]).is_valid(payload)
        is_invariant = result.get("code") == "mutation.invariant"
        if valid == is_invariant:
            raise SystemExit(f"[w3-t-layout] {kind}/{slug}: schema validity {valid} contradicts outcome {result}")
        root = f"{FIXTURES}/{leaf}/{folder}"
        dump(f"{root}/📸️snapshot/⬅️before/🔣️.json", before)
        dump(f"{root}/📸️snapshot/➡️after/🔣️.json", after)
        dump(f"{root}/🦠️mutation/🔣️.json", {LEAVES[kind][1]: payload})
        dump(f"{root}/🎯️outcome/🔣️.json", result)
        if diff is None:
            write_text(f"{root}/🔺️diff/🚫️.absent", "")
        else:
            dump(f"{root}/🔺️diff/🔣️.json", diff)
        write_text(f"{MUTATIONS}/{leaf}/🧪️tests/{folder}/🦀️.rs", rust_test(kind, leaf, folder, slug, story, result["status"]))
    mount(CASES)
    for path in written:
        print(f"[w3-t-layout] {'wrote' if APPLY else 'would write'} {os.path.relpath(path, REPO)}")
    print(f"[w3-t-layout] {len(CASES)} vectors checked against numpy and jsonschema; {len(written)} files {'written' if APPLY else 'pending'}")


if __name__ == "__main__":
    main()
