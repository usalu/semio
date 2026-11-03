import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "EN 1991-2 Table 4.4a (groups of traffic loads) as used by EN 1990 Annex A2.2.2 / Table A2.4"
OWN = [("gr1a", "road_traffic", "LM1 tandem system"), ("gr1a", "road_udl", "LM1 uniform load"), ("gr1a", "road_pedestrian", "footway and cycle track load"), ("gr1b", "road_single_axle", "LM2 single axle"), ("gr2", "road_horizontal", "braking and centrifugal forces"), ("gr3", "road_gr3_pedestrian", "pedestrian load"), ("gr4", "road_gr4_crowd", "LM4 crowd loading"), ("gr5", "road_special", "LM3 special vehicles")]
for group, category, label in OWN:
    put(d, "1990", f"a2.group.{group}.{category}", "Table A2.4", f"c {group} {category}", 1.0, 1.0, "", f"{label} acts in {group} on its declared characteristic value (factor 1.0). {SPEC}. UNVERIFIED: the factor 1.0 is the structural reading 'own component at its declared value', not a printed Table 4.4a cell.", locator="EN 1991-2 Table 4.4a")
for group, category, label in [("gr2", "road_traffic", "LM1 as accompanying component"), ("gr5", "road_traffic", "LM1 as accompanying component")]:
    put(d, "1990", f"a2.group.{group}.{category}", "Table A2.4", f"c {group} {category}", None, None, "", f"{label} of {group}: Table 4.4a assigns a reduced (frequent) value; the printed coefficient is NOT keyed here (UNVERIFIED, left unset so no value is invented). {SPEC}.", locator="EN 1991-2 Table 4.4a")
for i in (1, 2, 3):
    e = d[f"ndp.en1990.kfi.{i}"]
    e["specRef"] = "spec l.48 [N]: K_FI(RC1..3) = 0.9/1.0/1.1 (Table B3); K_FI is derived from the reliability class, never stored (wave 2 v7)"
    e["evidence"] = "ndp-public (requests.md 'K_FI 0.9/1.0/1.1 confirmed') and spec l.48 agree; kept UNVERIFIED because no installed library or printed edition is cited (blue-prints has no EN 1990)"
save(d)
print(len(d))
