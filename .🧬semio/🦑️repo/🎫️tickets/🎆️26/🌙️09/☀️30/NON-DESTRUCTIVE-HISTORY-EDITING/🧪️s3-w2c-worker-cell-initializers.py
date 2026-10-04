"""🧵️ S3-W2C: completes the peer's WorkerCell initializer refactor (commit 202c4b7b5b1) in the wgpu renderer.

Every `WorkerCell::new()` static becomes `WorkerCell::new(Default::default)`; the Scenes cell's test hook passes its
initializer. Exact, counted replacements only — the script refuses when a count differs from the expectation.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements")
EDITS = [
    ("🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs", "= WorkerCell::new();", "= WorkerCell::new(Default::default);", 14),
    ("⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs", "= WorkerCell::new();", "= WorkerCell::new(Default::default);", 5),
    ("🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "= crate::interpreter::WorkerCell::new();", "= crate::interpreter::WorkerCell::new(Default::default);", 2),
]
for relative, old, new, expected in EDITS:
    path = ROOT / relative
    text = path.read_text()
    found = text.count(old)
    if found != expected:
        sys.exit(f"{relative}: expected {expected} × {old!r}, found {found}")
    path.write_text(text.replace(old, new))
    print(f"{relative}: {found} replaced")
