#!/usr/bin/env python3
"""🎯️ Coordinator decision (session 12): every editor that edits a persisted document declares the artifact kind it
edits, so the hub's one open-target rule (`app_opens_kind`) pairs it (and its dialect's viewer) with that kind.
Adds `.artifact_kind(crate::artifact_kind())` right after the builder's `.document([...])` call and one law per editor.
Usage: editor-kind-declarations.py [--write] [artifact-substring …]"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
EDITORS = [
    ("🀄️wfc/🗿️artifacts/🖼️bitmap", '.document(["semio", "wfc", "bitmap"])', "create_bitmap_editor", "a WFC bitmap"),
    ("🀄️wfc/🗿️artifacts/🔲️grid2d", '.document(["semio", "wfc", "grid2d"])', "create_grid2d_editor", "a WFC 2D grid"),
    ("🀄️wfc/🗿️artifacts/◻️2d", '.document(["semio", "wfc", "2d"])', "create_wfc2d_editor", "a WFC 2D rule set"),
    ("🀄️wfc/🗿️artifacts/🧊️3d", '.document(["semio", "wfc", "3d"])', "create_wfc3d_editor", "a WFC 3D rule set"),
    ("🔱️trinity/🗿️artifacts/♻️rewriting", '.document(["semio", "trinity", "rewriting"])', "create_rewriting_app", "a Trinity rewrite rule"),
    ("🎪️demonstrator/🗿️artifacts/🎪️playground", '.document(["semio", "playground"])', "create_playground_editor", "a playground document"),
]
write = "--write" in sys.argv
only = [arg for arg in sys.argv[1:] if arg != "--write"]
problems = 0
for artifact, anchor, create, noun in EDITORS:
    if only and not any(part in artifact for part in only):
        continue
    editor = ROOT / artifact / SUBSET / "🦀️.rs"
    tests = ROOT / artifact / SUBSET / "🧪️tests/🔬️unit/🦀️.rs"
    source = editor.read_text(encoding="utf-8")
    line = next((l for l in source.splitlines(keepends=True) if anchor in l and ("Editor::builder" in l or l.strip() == anchor)), None)
    if line is None or source.count(anchor) != 1 or ".artifact_kind(crate::artifact_kind())" in source:
        print(f"PROBLEM {artifact}: anchor count {source.count(anchor)}"); problems += 1; continue
    following = source.split(line, 1)[1].splitlines()[0]
    indent = " " * (len(following) - len(following.lstrip()))
    source = source.replace(line, line + f"{indent}.artifact_kind(crate::artifact_kind())\n", 1)
    law_source = tests.read_text(encoding="utf-8")
    attribute = "#[semio_framework_async_macros::async_test]" if "async_test]" in law_source else "#[test]"
    asynchronous = "async " if attribute.endswith("async_test]") else ""
    if "fn the_editor_declares_the_artifact_kind_it_edits" in law_source:
        print(f"PROBLEM {artifact}: law exists"); problems += 1; continue
    law_source = law_source.rstrip("\n") + (
        "\n\n/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is\n"
        "/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)\n"
        f"/// pairs with this editor and the viewer of its dialect — so {noun} can be created and opened as a hub document.\n"
        f"{attribute}\n{asynchronous}fn the_editor_declares_the_artifact_kind_it_edits() {{\n"
        f"    assert_eq!({create}().artifact_kinds, vec![crate::artifact_kind()]);\n}}\n"
    )
    print(f"{artifact}: +declaration ({indent!r}) +law ({attribute})")
    if write:
        editor.write_text(source, encoding="utf-8")
        tests.write_text(law_source, encoding="utf-8")
print(f"problems={problems} write={write}")
