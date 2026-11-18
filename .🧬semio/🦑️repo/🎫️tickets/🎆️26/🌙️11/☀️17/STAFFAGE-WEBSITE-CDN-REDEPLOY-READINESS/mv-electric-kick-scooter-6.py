"""Electric kick scooter 1:200: the deck notch of the volume is as wide as the 45 mm tyre (half width 0.0225 m instead of 0.025 m), so the 2.5 mm slits beside the wheel that leak the pocket to the outside vanish."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
n = d.solid(200, "deck-body"); p = d.val("asset", "volumes", "200", "body", "0", "profile")
d.set(n, "profile", [[(0.0225 if x == 0.025 else -0.0225 if x == -0.025 else x), y] for x, y in p])
d.commit()
