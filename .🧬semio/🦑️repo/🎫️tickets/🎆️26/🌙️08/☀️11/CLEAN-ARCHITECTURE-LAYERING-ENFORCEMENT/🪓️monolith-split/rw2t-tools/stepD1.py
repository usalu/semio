"""📦 rw2t step D1: manifests of the law hosts (scene-ink-tests, scene-render) and the scene-state testkit edge of every crate whose laws read scene state."""
from mf import *
def fp(suffix, folder, feats=None):
    f = f', features = [{", ".join(chr(34) + x + chr(34) for x in feats)}]' if feats else ""
    return f'{FP}{suffix} = {{ path = "../../../{folder}/📦️packages/🦀️rust"{f} }}'
UI = "../../../../../../../../../../🔨️modules/🖱️ui"
ink = mpath("🧶️scene-ink-tests")
add_lines(ink, "dependencies", [
    fp("canvas2d-host", "🔄️canvas2d-host"),
    fp("frame-actions", "🛎️frame-actions", ["testkit"]),
    fp("host-base", "🏛️host-base"),
    fp("scene-feeds", "📰️scene-feeds"),
    fp("scene-render", "🪞️scene-render"),
    fp("scene-tables", "🧮️scene-tables", ["testkit"]),
    fp("ui-engine", "🖱️ui-engine"),
    f'semio-framework-ui-action = {{ path = "{UI}/🎬️action/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-draw-contract = {{ path = "{UI}/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-scene-plane = {{ path = "{UI}/🎬️scene/🎬️scenes/🖼️plane/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-scene-views = {{ path = "{UI}/🎬️scene/🎬️scenes/📄️views/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-text = {{ path = "{UI}/🔤️typography/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-theme = {{ path = "{UI}/🎨️styling/🎭️theme/📦️packages/🦀️rust" }}',
    f'ui_wgpu = {{ path = "{UI}/📦️packages/🦀️rust", features = ["wgpu-engine"], package = "semio-framework-ui" }}',
    'base64 = "0.22.1"',
    'image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }',
])
set_features(ink, FP + "scene-ink", ["testkit"])
set_features(ink, FP + "scene-state", ["testkit"])
set_features(ink, FP + "contracts", ["testkit"])
rend = mpath("🪞️scene-render")
add_lines(rend, "dev-dependencies", [
    f'semio-framework-ui-draw-contract = {{ path = "{UI}/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📦️packages/🦀️rust" }}',
    f'semio-framework-ui-text = {{ path = "{UI}/🔤️typography/📦️packages/🦀️rust" }}',
    f'ui_contract = {{ path = "{UI}/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }}',
])
