import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️pile-wall-slope-and-uplift-checks/🦀️.rs"
phspec.build(F, "check_piles_walls_slopes_and_uplift", [
 ("check_pile_resistances", "Checks the axial resistance of every pile.", 7, 150),
 ("check_retaining_walls", "Checks the earth pressure, sliding and overturning of every retaining wall.", 159, 356),
 ("check_slopes", "Checks the overall stability of every slope.", 365, 436),
 ("check_uplift", "Checks the buoyancy uplift of every uplift case.", 445, 445),
], out=sys.argv[1])
