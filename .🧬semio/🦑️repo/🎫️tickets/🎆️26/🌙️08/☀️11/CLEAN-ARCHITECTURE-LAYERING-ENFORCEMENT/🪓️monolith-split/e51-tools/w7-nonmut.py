"""🔧 E51 wave-7 non-mutation fixes: widenings and one owner constructor, all through the journalled editors."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from widen import widen
from ed import edit
F = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/"
widen(F + "🖥️host/🏠️flow-host/🦀️.rs", "FlowBaselineLeases", 39, "pub", ["current"])
widen(F + "🖥️host/🧾️eval-session-state/🦀️.rs", "SessionPreviewRetention", 408, "pub(super)", ["phase", "slot", "comparison", "matched"])
widen(F + "🕸️wasm/🧩️domain-adapter/🦀️.rs", "FlowSurface", 39, "pub", ["id", "state"])
widen(F + "🕸️wasm/🧩️domain-adapter/🦀️.rs", "FlowProgramFeature", 677, "pub", ["action"])
widen(F + "🕸️wasm/🎬️actions-a/🦀️.rs", "FlowAction2524", 575, "pub", ["items"])
widen(F + "🕸️wasm/🎞️actions-b/🦀️.rs", "FlowAction2599", 1570, "pub", ["output"])
widen("🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/📚️dictionary/🦀️.rs", "Dictionary", 21, "pub", ["pairs"], hidden=True)
