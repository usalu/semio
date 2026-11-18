"""Scissor lift: the vent outlet is one smallest feature thick (30, 15 and 7.5 mm) and its origin sits so that it protrudes 5 mm beyond the chassis end at every scale."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/scissor-lift")
for sc, f in ((200, 0.03), (100, 0.015), (50, 0.0075)):
    n = d.solid(sc, "motor-vent-outlet"); o = d.obj(n)
    d.setv(sp.child(n, "profile"), [o["profile"][0], [f, o["profile"][1][1]]])
    d.setv(sp.child(n, "origin"), [round(0.93 - f, 4), o["origin"][1], o["origin"][2]])
d.commit()
