"""Electric kick scooter 1:200: the deck notch of the volume ends 12 mm behind the rear wheel and leaves a pocket the plan drawing does not show; the notch end moves into the wheel at x = -0.34 m (the second pass moves -0.322 to -0.34 after the first pass moved -0.31 to -0.322)."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
n = d.solid(200, "deck-body"); p = d.val("asset", "volumes", "200", "body", "0", "profile")
d.set(n, "profile", [[x, (-0.34 if y in (-0.31, -0.322) else y)] for x, y in p])
d.commit()
