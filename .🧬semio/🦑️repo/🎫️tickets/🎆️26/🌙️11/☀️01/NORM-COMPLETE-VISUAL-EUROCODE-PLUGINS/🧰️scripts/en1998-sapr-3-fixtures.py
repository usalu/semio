#!/usr/bin/env python3
"""Converts every JSON fixture of the EN 1998 artifact from site.seismicZone to site.sApR (zone0..3 -> 0, 1.0, 1.5, 2.0 m/s2 = 2.5 a_gR) preserving formatting."""
import os, re
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
ZONE = {"zone0": "0.0", "zone1": "1.0", "zone2": "1.5", "zone3": "2.0"}
n = 0
for dp, dn, fn in os.walk(ART):
    dn[:] = [d for d in dn if d not in ("target", "node_modules")]
    for f in fn:
        if not f.endswith(".json"): continue
        p = os.path.join(dp, f)
        s = open(p, encoding="utf-8").read()
        o = s
        s = re.sub(r'"seismicZone"(\s*:\s*)"(zone[0-3])"', lambda m: '"sApR"' + m.group(1) + ZONE[m.group(2)], s)
        s = re.sub(r'"newSeismicZone"(\s*:\s*)"(zone[0-3])"', lambda m: '"newSApR"' + m.group(1) + ZONE[m.group(2)], s)
        s = s.replace('"changeSiteSeismicZone"', '"changeSiteSApR"')
        if s != o:
            open(p, "w", encoding="utf-8").write(s); n += 1
print("fixtures converted", n)
