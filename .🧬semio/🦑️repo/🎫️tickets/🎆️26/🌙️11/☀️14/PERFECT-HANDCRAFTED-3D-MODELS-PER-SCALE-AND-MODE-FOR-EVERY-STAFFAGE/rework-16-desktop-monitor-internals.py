"""🖥️ Desktop monitor: the heat sources as real parts inside the housing of the thermal list, the LED light bar at the lower panel edge (1:50) and the shielded power and scaler board in the rear bulge (1:50, 1:100)."""
from rework_16_lib import load, save, put, box, thermal, decide

doc = load("desktop-monitor")
fine = doc["volumes"]["50"]
put(fine["thermal"], thermal(box("th-light-bar", "panel", "housing-plastic", [0.008, 0.0075, 0.5], [-0.155, -0.031, 0.0], tone=0.85), "panel-front", inner=True), after="th-front-glass")
put(fine["thermal"], thermal(box("th-power-board", "panel", "housing-plastic", [0.13, 0.014, 0.2], [-0.02, -0.016, 0.0], tone=0.3), "housing", inner=True), after="th-housing")
decide(fine, "Thermal: the two heat sources are real parts inside the housing, the LED light bar (500 x 8 mm) along the lower edge of the panel behind the bezel and the shielded power and scaler board (200 x 130 x 14 mm) in the rear bulge under the vents.", "Thermik: Die beiden Wärmequellen sind echte Bauteile im Gehäuse, die LED-Lichtleiste (500 x 8 mm) an der Unterkante des Panels hinter der Blende und die abgeschirmte Netzteil- und Scalerplatine (200 x 130 x 14 mm) in der hinteren Ausbuchtung unter den Lüftungsschlitzen.")
mid = doc["volumes"]["100"]
put(mid["thermal"], thermal(box("th-power-board", "panel", "housing-plastic", [0.13, 0.015, 0.2], [-0.02, -0.016, 0.0], tone=0.3), "housing", inner=True), after="th-housing")
decide(mid, "Thermal: the shielded power and scaler board (200 x 130 x 15 mm) is a real part inside the rear bulge; the 8 mm light bar is below the smallest feature and is carried by the front stack.", "Thermik: Die abgeschirmte Netzteil- und Scalerplatine (200 x 130 x 15 mm) ist ein echtes Bauteil in der hinteren Ausbuchtung; die 8 mm starke Lichtleiste liegt unter dem kleinsten Merkmal und wird vom Frontstapel getragen.")
save("desktop-monitor", doc)
