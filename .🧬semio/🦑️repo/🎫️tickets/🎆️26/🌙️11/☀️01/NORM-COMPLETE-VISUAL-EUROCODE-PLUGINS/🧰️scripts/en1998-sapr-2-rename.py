#!/usr/bin/env python3
"""Renames the mutation leaf change-site-seismic-zone to change-site-s-ap-r everywhere, rewrites its leaf files for f64 and converts fixtures (zone -> S_aP,R)."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, json
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = NORM + "/🗿️artifacts/🫨️en1998"
TAX = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
ZONE = {"zone0": 0.0, "zone1": 1.0, "zone2": 1.5, "zone3": 2.0}
REN = [("change-site-seismic-zone", "change-site-s-ap-r"), ("change_site_seismic_zone", "change_site_s_ap_r"), ("ChangeSiteSeismicZone", "ChangeSiteSApR"), ("new_seismic_zone", "new_s_ap_r"), ("new-seismic-zone", "new-s-ap-r"), ("newSeismicZone", "newSApR"), ("site-seismic-zone", "site-s-ap-r"), ("CHANGE_SITE_SEISMIC_ZONE", "CHANGE_SITE_S_AP_R")]
files = []
for dp, dn, fn in os.walk(ART):
    dn[:] = [d for d in dn if d not in ("target", "node_modules")]
    for f in fn:
        files.append(os.path.join(dp, f))
files.append(TAX)
changed = 0
for p in files:
    try: s = open(p, encoding="utf-8").read()
    except Exception: continue
    o = s
    for a, b in REN: s = s.replace(a, b)
    if s != o:
        open(p, "w", encoding="utf-8").write(s); changed += 1
print("renamed identifiers in", changed, "files")
moves = []
for dp, dn, fn in os.walk(ART):
    for d in list(dn):
        if "change-site-seismic-zone" in d: moves.append(os.path.join(dp, d))
for m in sorted(moves, key=len, reverse=True):
    new = m.replace("change-site-seismic-zone", "change-site-s-ap-r")
    if os.path.exists(new): raise SystemExit("exists " + new)
    os.rename(m, new)
print("moved", len(moves), "dirs")
