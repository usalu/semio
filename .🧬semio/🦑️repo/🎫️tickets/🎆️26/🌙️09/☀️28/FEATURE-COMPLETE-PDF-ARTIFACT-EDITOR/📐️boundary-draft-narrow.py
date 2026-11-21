"""🎯️ Keeps the decimal tuple policy in the native and browser renderer owners."""

import json
from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
outer = ticket / "🗑️generated"
stage = outer / "boundary-draft"
output = outer / "boundary-draft-narrow"
output.mkdir(exist_ok=True)
generator = runpy.run_path(str(ticket / "📐️boundary-draft-stage.py"))
generator["stage"](Path.cwd(), Path(sys.argv[1]))
mapping = json.loads((stage / "overlay.json").read_text())
native = generator["RUST"]
private = native.replace("pub const ", "const ").replace("pub fn ", "fn ")
excluded = (
    "/🧩️component/🎚️slider-geometry-and-key-handling/🦀️.rs",
    "/🧩️component/🦀️.rs", "/🧩️component/🎨️basic-widget-props/🦀️.rs",
)
selected = {}
for path, target in mapping.items():
    if path.endswith(excluded):
        continue
    text = (stage / target).read_text()
    if path.endswith("/🧮️control-commit-rules/🦀️.rs"):
        for name in ["UI_DECIMAL_QUAD_COMMIT", "decimal_quad_text", "input_commits_on_blur"]:
            text = text.replace("ui_contract::" + name, name)
        text += "\n" + private
    if path.endswith("/🎛️widget-nodes/🦀️.rs"):
        text = text.replace("ui_contract::input_commits_on_blur(props.commit.as_ref().map(|value| value.as_str()))",
                            'matches!(props.commit.as_ref().map(|value| value.as_str()), Some("blur" | "blurDecimalQuad"))')
    text = text.replace("semio_ui_contract::UI_DECIMAL_QUAD_COMMIT", '"blurDecimalQuad"')
    (output / target).write_text(text)
    selected[path] = target
assert len(mapping) - len(selected) == 3
(output / "overlay.json").write_text(json.dumps(selected, ensure_ascii=False, indent=2) + "\n")
(output / "browser-smoke.ts").write_bytes((stage / "browser-smoke.ts").read_bytes())
print(f"[DEBUG] Staged {len(selected)} paths with unchanged shared Rust UI contract")
