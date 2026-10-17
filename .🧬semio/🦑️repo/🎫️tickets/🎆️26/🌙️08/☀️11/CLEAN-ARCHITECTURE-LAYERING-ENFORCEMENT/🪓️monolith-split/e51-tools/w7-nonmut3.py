"""🔧 E51 wave-7 part 3: widenings found by the whole-workspace scan (flow-vcs, cad model, layout json cursor)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from widen import widen
widen("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs", "FlowVcsCensus", 81, "pub", ["items", "bytes", "depth"])
widen("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs", "FlowVcsSource", 100, "pub", ["value"])
widen("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs", "CadWorkingScene", 150, "pub", ["objects", "geometry", "building_objects", "building_geometry", "energy_objects", "energy_geometry", "structure_classic_objects", "structure_classic_geometry"])
widen("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/🦀️.rs", "TypedJsonCursor", 199, "pub", ["stack", "emitted_nodes", "fragment_bytes"])
