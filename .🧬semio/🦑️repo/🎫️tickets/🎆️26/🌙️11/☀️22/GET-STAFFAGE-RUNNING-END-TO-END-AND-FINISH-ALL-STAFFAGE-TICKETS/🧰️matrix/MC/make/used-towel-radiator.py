"""♨️ Ops writer of `used-towel-radiator`: the radiator leans back by 10 degrees on its rack at rest and stands upright once it is lifted and hung, but its front and plan shapes were flat outlines of the leaning rest pose, so the upright radiator reached 96 to 102 mm further forward in the plan than in the side elevation. The uprights, the rails, the brackets and every other part whose flat outline is the rectangle of its side elevation twin become that twin swept over their width. Small round parts (bracket pins and rosettes, plugs, valve parts) and surface marks stay flat: they are carried at the place of their side elevation twin and a turn of 10 degrees changes their picture by 1.5 percent: `python3 make/used-towel-radiator.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("used-towel-radiator")
write("used-towel-radiator", solidify(a, {"radiator", "rails"}))
