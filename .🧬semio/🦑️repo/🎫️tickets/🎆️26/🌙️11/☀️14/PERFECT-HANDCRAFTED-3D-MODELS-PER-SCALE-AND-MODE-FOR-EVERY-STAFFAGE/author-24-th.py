"""Thermal area lines: python3 author-24-th.py <slug> [scale...]"""
import sys
sys.path.insert(0, ".")
from author_24_lib import *
slug = sys.argv[1]
for sc in sys.argv[2:] or ["50", "100", "200", "500"]:
    for m in measurements(slug, int(sc), "volumes-thermal-coverage"):
        if m["name"].startswith(("areaDeviation", "thermal.areaDeviation", "thermal.regionsWithout", "thermal.outside", "thermal.uncovered")):
            print(sc, ("ok  " if m["ok"] else "FAIL"), m["name"], round(m["value"], 3), "<", m["limit"], (m.get("note") or "")[:110])
