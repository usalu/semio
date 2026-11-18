"""Tower crane 1:200: the head sheave (0.5 m) and the jib tip sheave (0.6 m) are in the volume and in every 1:100 drawing but in no 1:200 drawing; they are drawn as the extruded discs of 1:100."""
from mvlib import *
import json
d = D("🏗️machinery/tower-crane")
src = D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    last = d.at("asset", "representations", "200", "geometry", view)["items"][-1]
    for sid in ("jib-tip-sheave", "head-sheave"):
        o = src.obj(src.shape(100, view, sid)); o["z"] = 1.9
        o = {k: o[k] for k in o}
        d.insert_after(last, o)
d.commit()
