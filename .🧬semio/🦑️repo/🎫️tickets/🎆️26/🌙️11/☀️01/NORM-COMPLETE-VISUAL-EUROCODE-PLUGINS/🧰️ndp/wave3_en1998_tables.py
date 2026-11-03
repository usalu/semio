import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
N = "as used by the artifact"
def row(key, clause, symbol, v, note, unit=""):
    put(d, "1998", f"en1.{key}", clause, symbol, v, v, unit, f"{note}; {N}", part="1")
for cls, v in (("i", 0.8), ("ii", 1.0), ("iii", 1.2), ("iv", 1.4)):
    row(f"gamma-i.{cls}", "4.2.5 / NA", f"gamma_I class {cls.upper()}", v, "Importance factor of importance class (CC1 to CC4 map to I to IV, unknown classes to II)")
for cls, v in (("dch", .25), ("dcm", .25), ("low", .20)):
    row(f"rc.column-min-b.{cls}", "5.4.3.2.1 / 5.5.3.2.1", f"RC column minimum width {cls.upper()}", v, "Simplified minimum column width", "m")
for role, vals in (("column", (.01, .01, .01)), ("beam", (.0035, .0025, .0025)), ("other", (.002, .002, .002))):
    for cls, v in zip(("dch", "dcm", "low"), vals):
        row(f"rc.rho-min.{role}.{cls}", "5.4.3 / 5.5.3", f"rho_min {role} {cls.upper()}", v, "Simplified minimum longitudinal reinforcement ratio")
        row(f"rc.rho-max.{role}.{cls}", "5.4.3 / 5.5.3", f"rho_max {role} {cls.upper()}", .04, "Simplified maximum longitudinal reinforcement ratio")
for cls, v in (("dch", .12), ("dcm", .08), ("low", 0.0)):
    row(f"rc.omega-wd-min.{cls}", "5.4.3.2.2 / 5.5.3.2.2", f"omega_wd min {cls.upper()}", v, "Minimum mechanical volumetric ratio of confining hoops")
for band, v in (("none", 0.0), ("lt0-5", .02), ("lt0-7", .03), ("ge0-7", .04)):
    row(f"masonry-wall-ratio.{band}", "NA.12 (unverified)", f"masonry wall area ratio a_gR band {band}", v, "Simple masonry wall-area ratio by reference acceleration band (none without seismic action, below 0.5, below 0.7 m/s2, else)")
row("masonry-wall-ratio.importance-iii-iv", "NA.12 (unverified)", "masonry wall ratio factor importance III/IV", 1.25, "Multiplier of the wall-area ratio for importance classes III and IV")
row("plan-gap-ratio", "4.4.2.2 plan envelope", "plan gap ratio limit", 0.05, "Gap ratio at which the plan envelope utilisation is 1")
save(d); print(len(d))
