#!/usr/bin/env python3
"""⚙️ Follow-ups of the sim split (lane P1, slice S1d).

- `EnergyCommitReservation` (the page sources a commit packet reserves) moves from `🧪️sim` to `🧪️sim/📡️wire`, beside the
  packet that carries it.
- The design-day and synthetic hourly weather synthesis moves from `🧪️sim` to `📍️site` (`design_day_hour`,
  `synthetic_hour`), the owner of weather records and design days.
- The wire laws' fixture path gains the extra directory level of the wire crate.
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "⚙️sim-items.py"), encoding="utf-8").read())
E = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine"
sim_p, wire_p, site_p = E + "/🧪️sim/🦀️.rs", E + "/🧪️sim/📡️wire/🦀️.rs", E + "/📍️site/🦀️.rs"
wt_p = E + "/🧪️sim/📡️wire/🧪️tests/🔬️unit/🦀️.rs"
sim, wire, site, wt = (open(p, encoding="utf-8").read() for p in (sim_p, wire_p, site_p, wt_p))
if "pub fn design_day_hour" in site: sys.exit("already applied")
def take(src, names):
    its = items(src); out = []
    for n, k, a, b in reversed(its):
        if n in names and k != "use":
            out.insert(0, src[a:b]); src = src[:a].rstrip("\n") + "\n\n" + src[b:].lstrip("\n")
    return src, out
sim, moved = take(sim, {"EnergyCommitReservation"})
moved = [re.sub(r"(?m)^struct EnergyCommitReservation", "pub struct EnergyCommitReservation", moved[0])] + [re.sub(r"(?m)^    fn ", "    pub fn ", t) for t in moved[1:]]
wire = wire.rstrip("\n") + "\n\n" + "\n\n".join(moved) + "\n"
sim, weather = take(sim, {"design_day_hour", "synthetic_hour"})
weather = [t.replace("fn synthetic_hour", "/// 🌤️ One synthetic hour of a smooth annual and daily temperature cycle, for runs without authored weather.\npub fn synthetic_hour", 1).replace("fn design_day_hour", "/// 🌡️ One design-day hour at a fixed dry-bulb temperature, sunny when the day is a cooling design day.\npub fn design_day_hour", 1) for t in weather]
anchor = "// #endregion 🔖️DesignDay\n"; assert site.count(anchor) == 1
site = site.replace(anchor, "\n".join(weather).rstrip("\n") + "\n" + anchor)
for name in ("design_day_hour", "synthetic_hour"):
    sim = re.sub(r"(?<![\w:])" + name + r"\(", "crate::site::" + name + "(", sim)
wt = wt.replace('include_str!("../../../../../../🧪️tests/🧫️fixtures/🧮️numerical-laws/🔣️.json")', 'include_str!("../../../../../../../🧪️tests/🧫️fixtures/🧮️numerical-laws/🔣️.json")')
wt = re.sub(r"(?<![\w:])design_day_hour\(", "crate::site::design_day_hour(", wt)
st_p = E + "/🧪️sim/🧪️tests/🔬️unit/🦀️.rs"; st = open(st_p, encoding="utf-8").read()
st = re.sub(r"(?<![\w:])design_day_hour\(", "crate::site::design_day_hour(", st)
for p, t in ((sim_p, sim), (wire_p, wire), (site_p, site), (wt_p, wt), (st_p, st)): open(p, "w", encoding="utf-8").write(t)
print("ok")
