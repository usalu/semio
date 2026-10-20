import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🦀️.rs"
c1 = child("🔩", "joint-checks", "Joint checks: the bolted and welded joint checks of EN 1993-1-8.")
c2 = child("⏳", "fatigue-fire-and-thin-walled-checks", "Fatigue, fire and thin walled checks: the fatigue, fire, cold-formed, stainless and plated panel checks.")
c3 = child("🏛", "shell-and-structure-checks", "Shell and structure checks: the shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks.")
spec = {"file": F, "function": "check_special_elements", "modes": {"annex": "val", "params": "val"}, "phases": [
    ph("check_joints", "🔩 Checks every joint: bolted shear, bearing and tension and welded joint resistances.", 9, 9, c1),
    ph("check_fatigue_and_fire", "⏳ Checks the Palmgren-Miner fatigue details of EN 1993-1-9 and the fire resistance of EN 1993-1-2.", 254, 306, c2),
    ph("check_cold_formed_stainless_and_plated", "🧊 Checks the cold-formed members, the stainless members and the plated panels.", 411, 494, c2),
    ph("check_shell_and_member_properties", "🥫 Checks the shells and silos, the through-thickness properties, the tension components and the high-strength steels.", 539, 689, c3),
    ph("check_bridge_tower_pile_and_crane", "🌉 Checks the bridge, tower, pile and crane runway rules.", 751, 942, c3),
]}
dump(spec, sys.argv[1])
