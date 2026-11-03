import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
N = "wave 2 v3, UNVERIFIED: value recalled from the standard, not checked against the printed edition"
rows = [
    ("a2.rail.twist_speed_1", "A2.4.4.2.2", "v1 twist band", 120.0, "km/h", "Line speed up to which the twist limit twist_120 applies"),
    ("a2.rail.twist_speed_2", "A2.4.4.2.2", "v2 twist band", 200.0, "km/h", "Line speed up to which the twist limit twist_200 applies; above it twist_max applies"),
    ("a2.rail.speed_margin", "A2.4.4.2.1", "verification speed factor", 1.2, "", "Dynamic verification up to this factor times the maximum line speed"),
    ("a2.rail.dynamic_speed", "A2.4.4.2.1", "dynamic analysis speed", 200.0, "km/h", "Line speed above which a dynamic analysis of the deck is required"),
    ("a2.rail.end_rotation_speed", "A2.4.4.2.3", "end rotation speed band", 160.0, "km/h", "Line speed separating the two end rotation limits (structure of the rule recalled, magnitude pending the printed table)"),
    ("a2.rail.end_rotation_low", "A2.4.4.2.3", "end rotation, v <= band", 0.0065, "rad", "Maximum deck end rotation up to the speed band (placeholder pending the printed table)"),
    ("a2.rail.end_rotation_high", "A2.4.4.2.3", "end rotation, v > band", 0.005, "rad", "Maximum deck end rotation above the speed band (placeholder pending the printed table)"),
    ("a2.rail.twist_length", "A2.4.4.2.2", "twist base length", 3.0, "m", "Base length of the track twist t in mm per 3 m"),
    ("a2.fw.speed_cap", "A2.2.2", "F*_W wind speed cap", 35.0, "m/s", "Limiting wind speed v* of the wind force F*_W combined with traffic: F*_W = F_Wk min(1, (v*/v_b)^2) (EN 1991-1-4 8.1 link)"),
    ("a2.wind_thermal.simultaneous", "A2.2.2", "wind and thermal simultaneous", 0.0, "", "0 = wind and thermal actions are never combined (mutually exclusive), 1 = simultaneous"),
]
for key, clause, symbol, value, unit, note in rows:
    put(d, "1990", key, clause, symbol, value, value, unit, f"{note} ({N})")
save(d)
