#!/usr/bin/env python3
"""🔄️ Gives layout's `rotate-frame` (added 09-23 without evidence) its committed specification vector and case rows:
the quintet under `🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates-the-rect-frame`, the leaf-local vector test, the
catalog vector, the two `Examples` rows of `📐️mutate-layout-1`, and both adapters (the Rust subject/oracle table and the
independent Python reference: forward applier plus inverse rule). The before-document is move-frame's committed one.
Usage: layout-rotate-frame.py [--write]"""
import copy, json, sys
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
crate = root / "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout"
subset = crate / "🏅️standards/🔖️1/🪆️subsets/✳️any"
leaf = "🔄️rotate-frame"
scenario = "🌀️rotates-the-rect-frame"
bundle = subset / "🧫️fixtures/🧬️mutations" / leaf / scenario
move = subset / "🧫️fixtures/🧬️mutations/🕹️move-frame/📍️moves-the-rect-frame"
write = "--write" in sys.argv
edits = []


def put(path, text):
    edits.append(path)
    if write:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


def once(text, old, new, what):
    if text.count(old) != 1:
        raise SystemExit(f"{what}: expected exactly one anchor, found {text.count(old)}")
    return text.replace(old, new)


def dump(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


before = json.loads((move / "📸️snapshot/⬅️before/🔣️.json").read_text(encoding="utf-8"))
after = copy.deepcopy(before)
frame = next(f for p in after["pages"] if p["id"] == "page-1" for f in p["frames"] if f["id"] == "frame-rect")
frame["bounds"]["rotation"] = 15.0
diff = json.loads((move / "🔺️diff/🔣️.json").read_text(encoding="utf-8"))
patch = diff["pages"]["patched"][0]["patch"]["frame_patched"]["patch"]
patch["x"], patch["y"], patch["rotation"] = None, None, 15.0
put(bundle / "📸️snapshot/⬅️before/🔣️.json", (move / "📸️snapshot/⬅️before/🔣️.json").read_text(encoding="utf-8"))
put(bundle / "📸️snapshot/➡️after/🔣️.json", dump(after))
put(bundle / "🦠️mutation/🔣️.json", dump({"RotateFrame": {"page_id": "page-1", "frame_id": "frame-rect", "new_rotation": 15.0}}))
put(bundle / "🔺️diff/🔣️.json", dump(diff))
put(bundle / "🎯️outcome/🔣️.json", dump({"status": "applied"}))

test = (subset / "🧬️schema/🧬️mutations/🕹️move-frame/🧪️tests/📍️moves-the-rect-frame/🦀️.rs").read_text(encoding="utf-8")
head, _, _ = test.partition("/// ▶️")
head = head.replace("move-frame", "rotate-frame").replace("📍️moves-the-rect-frame", scenario).replace("moves-the-rect-frame", "rotates-the-rect-frame").replace("🕹️move-frame", leaf)
head = head.replace("Proves the bounds origin moves while the extent and rotation stay fixed.", "Proves the frame's rotation turns while its origin and extent stay fixed.")
body = '''/// ▶️ `rotate-frame` writes `bounds.rotation`; origin, width and height are untouched.
#[semio_framework_async_macros::async_test]
async fn turns_the_rotation_only() {
    let after = applied();
    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!(bounds.rotation, 15.0, "rotate-frame must write the payload rotation into the frame bounds");
    assert_eq!((bounds.x, bounds.y, bounds.width, bounds.height), (20.0, 30.0, 60.0, 40.0), "rotate-frame must neither move nor resize the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().rotation, 0.0, "rotate-frame must not rotate sibling frames");
    assert_eq!(after, expected_after(), "rotate-frame/rotates-the-rect-frame: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `rotate-frame` carrying the rotation captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_turns_the_rect_frame_back() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "rotate-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::RotateFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_rotation, 0.0, "the inverse must carry the pre-rotation angle");
        }
        other => panic!("rotate-frame must invert to rotate-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("rotate-frame/rotates-the-rect-frame: inverse step applies");
    }
    assert_eq!(snapshot, base, "rotate-frame/rotates-the-rect-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rotate-frame/rotates-the-rect-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rotate-frame/rotates-the-rect-frame: committed mutation JSON is not canonical");
}

/// 🔺️ The produced diff is the committed `🔺️diff`, and the committed outcome is the produced one.
#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff_and_outcome() {
    let outcome = mutation().diff(&before());
    let produced: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(outcome.diff())).expect("diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rotate-frame/rotates-the-rect-frame: produced diff differs from the committed 🔺️diff");
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    assert!(outcome.messages().is_empty(), "rotate-frame/rotates-the-rect-frame: an applied rotation raises no diagnostic");
}
'''
put(subset / "🧬️schema/🧬️mutations" / leaf / "🧪️tests" / scenario / "🦀️.rs", head + body)

lib = (crate / "🦀️.rs").read_text(encoding="utf-8")
lib = once(lib, '''                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-frame/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }''', '''                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-frame/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-frame/🧪️tests/🌀️rotates-the-rect-frame/🦀️.rs"]
                            mod tests_rotates_the_rect_frame;
                        }''', "crate root mount")
put(crate / "🦀️.rs", lib)

oracles_path = subset / "🔮️oracles/🔣️.json"
oracles_text = oracles_path.read_text(encoding="utf-8")
oracles = json.loads(oracles_text)
catalog = oracles["mutationCatalogs"][0]
if not any(v["mutationId"] == "rotate-frame" for v in catalog["vectors"]):
    catalog["vectors"].append({"mutationId": "rotate-frame", "sourceMutationDirectoryName": leaf, "mutationDirectoryName": leaf, "scenarios": [{"id": "rotates-the-rect-frame", "directoryName": scenario}]})
put(oracles_path, json.dumps(oracles, ensure_ascii=False, indent=1 if oracles_text.startswith('{\n "') else 2) + "\n")

case = subset / "🧪️tests/📐️mutate-layout-1"
feature = (case / "🥒️.feature").read_text(encoding="utf-8")
row_old = "      | move-frame             | 🕹️move-frame             | 📍️moves-the-rect-frame                            |\n"
row_new = row_old + "      | rotate-frame           | 🔄️rotate-frame           | 🌀️rotates-the-rect-frame                          |\n"
if feature.count(row_old) != 2:
    raise SystemExit("feature: expected the move-frame row in both Examples tables")
put(case / "🥒️.feature", feature.replace(row_old, row_new))

adapter = (case / "🦀️.rs").read_text(encoding="utf-8")
anchor = '''        "resize-frame" => ('''
arm = '''        "rotate-frame" => (
            include_str!("../../🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates-the-rect-frame/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates-the-rect-frame/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates-the-rect-frame/📸️snapshot/➡️after/🔣️.json"),
        ),
'''
adapter = once(adapter, anchor, arm + anchor, "rust adapter fixture arm")
adapter = once(adapter, '''    "resize-frame",\n''', '''    "resize-frame",\n    "rotate-frame",\n''', "rust adapter KINDS")
put(case / "🦀️.rs", adapter)

python = (case / "🐍️.py").read_text(encoding="utf-8")
python = once(python, '''    "resize-frame": (f"{_ROOT}/📏resize-frame/📐️resizes-the-rect-frame", "ResizeFrame"),\n''', '''    "resize-frame": (f"{_ROOT}/📏resize-frame/📐️resizes-the-rect-frame", "ResizeFrame"),\n    "rotate-frame": (f"{_ROOT}/🔄️rotate-frame/🌀️rotates-the-rect-frame", "RotateFrame"),\n''', "python vectors")
python = once(python, '''def apply_resize_frame(doc, p):''', '''def apply_rotate_frame(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["bounds"]["rotation"] = p["new_rotation"]
    return after


def apply_resize_frame(doc, p):''', "python applier")
python = once(python, '''    "move-frame": apply_move_frame,\n''', '''    "move-frame": apply_move_frame,\n    "rotate-frame": apply_rotate_frame,\n''', "python appliers")
python = once(python, '''    if kind == "resize-frame":''', '''    if kind == "rotate-frame":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "RotateFrame", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_rotation": frame["bounds"]["rotation"]}
    if kind == "resize-frame":''', "python inverse rule")
python = python.replace("own 25-kind mutation vocabulary", "own 26-kind mutation vocabulary")
put(case / "🐍️.py", python)

print(("wrote" if write else "would write"), len(edits), "file(s)")
for path in edits:
    print(" ", path.relative_to(root))
