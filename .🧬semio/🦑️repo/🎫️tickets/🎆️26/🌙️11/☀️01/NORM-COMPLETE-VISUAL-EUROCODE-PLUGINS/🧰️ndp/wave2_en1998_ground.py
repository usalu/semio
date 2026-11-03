import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "en1997-en1998-target.md:351"
REC = "recollection of EN 1998-1 Table 3.1, not in the spec"
ROWS = [
    ("vs-a-min", "v_s,30 lower bound of ground type A", 800.0, "m/s", SPEC, "A: v_s,30 > 800 m/s (stated by the spec; the spec itself is unverified)"),
    ("vs-b-min", "v_s,30 lower bound of ground type B", 360.0, "m/s", SPEC, "B: 360 < v_s,30 <= 800 m/s"),
    ("vs-c-min", "v_s,30 lower bound of ground type C", 180.0, "m/s", SPEC, "C: 180 < v_s,30 <= 360 m/s, D below"),
    ("spt-b-min", "N_SPT lower bound of ground type B", 50.0, "blows/30cm", SPEC, "B: N_SPT > 50, used when v_s,30 is not given"),
    ("spt-c-min", "N_SPT lower bound of ground type C", 15.0, "blows/30cm", SPEC, "C: 15 <= N_SPT <= 50, D below"),
    ("cu-b-min", "c_u lower bound of ground type B", 250.0, "kPa", SPEC, "B: c_u > 250 kPa, used when v_s,30 and N_SPT are not given"),
    ("cu-c-min", "c_u lower bound of ground type C", 70.0, "kPa", SPEC, "C: 70 <= c_u <= 250 kPa, D below"),
    ("e.thickness-min-m", "Ground type E alluvium minimum thickness", 5.0, "m", SPEC, "E: surface alluvium 5 to 20 m (range stated by the spec)"),
    ("e.thickness-max-m", "Ground type E alluvium maximum thickness", 20.0, "m", SPEC, "E: surface alluvium 5 to 20 m"),
    ("e.substratum-vs-min", "Ground type E substratum v_s lower bound", 800.0, "m/s", REC, "E: alluvium of type C or D over a substratum with v_s > 800 m/s (the spec says only A-like)"),
    ("s1.thickness-min-m", "Ground type S1 soft clay minimum thickness", 10.0, "m", REC, "S1: soft clay or silt layer of at least 10 m with high plasticity index"),
    ("s1.plasticity-index-min", "Ground type S1 plasticity index lower bound", 40.0, "", REC, "S1: PI > 40"),
]
for key, sym, v, unit, ref, note in ROWS:
    i = put(d, "1998", f"en1.ground.{key}", "3.1 Table 3.1", sym, v, v, unit, note, part="1")
    d[i]["specRef"] = ref
    d[i]["source"]["locator"] = "Table 3.1"
save(d); print(len(d))
