import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib_spec import *
F = ROOT + "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🧱️evaluate-wall/🦀️.rs"
c1 = child("📏", "wall-geometry-and-material", "Wall geometry and material phases: the wall paths, the slenderness and the material conformance of the wall evaluation.")
c2 = child("🏋", "load-case-checks", "Load case checks: the per load case compression, shear, flexure and concentrated load checks of the wall evaluation.")
c3 = child("🔥", "fire-durability-and-reinforcement", "Fire, durability and reinforcement phases: the fire, bed joint, reinforced masonry and basement checks of the wall evaluation.")
types = {"annex": "AnnexChoice", "masonry_class": "MasonryClass", "default_situation": "DesignSituation", "storeys": "u32", "out": "Vec<CheckResult>", "lambda": "f64", "a": "f64", "f_k": "f64", "t": "f64"}
for n in "t support fb mortar mortar_f mu bearing as_h unit_h density phi_inf basement".split():
    types["path_" + n] = "String"
modes = {k: "val" for k in ["annex", "masonry_class", "default_situation", "storeys", "lambda", "a", "f_k", "t"]}
modes["out"] = "mut"
spec = {"file": F, "function": "evaluate_wall", "types": types, "modes": modes, "keep": [13, 130, 806], "phases": [
    ph("wall_paths_and_basics", "📏 Resolves the wall field paths, the slenderness, the area, the characteristic strength and the thickness.", 14, 30, c1, struct="WallBasics", struct_doc="📐 The wall field paths and the derived geometry the wall evaluation phases share."),
    ph("check_slenderness", "📐 Checks the slenderness of the wall.", 33, 33, c1),
    ph("check_material_conformance", "🧱 Checks the material conformance of unit, mortar and density.", 65, 65, c1),
    ph("check_load_cases", "🏋 Checks every load case of the wall: compression, shear, lateral flexure and concentrated load.", 146, 146, c2),
    ph("check_fire", "🔥 Checks the fire resistance of the wall with the combination of actions.", 469, 469, c3),
    ph("check_durability", "💧 Checks the mortar class required by the exposure and the bed joint.", 516, 516, c3),
    ph("check_reinforced_masonry", "🪢 Checks the reinforced masonry rules.", 581, 581, c3),
    ph("check_simplified_and_basement", "🏚 Checks the simplified EN 1996-3 rules and the basement earth pressure.", 648, 648, c3),
]}
dump(spec, sys.argv[1])
