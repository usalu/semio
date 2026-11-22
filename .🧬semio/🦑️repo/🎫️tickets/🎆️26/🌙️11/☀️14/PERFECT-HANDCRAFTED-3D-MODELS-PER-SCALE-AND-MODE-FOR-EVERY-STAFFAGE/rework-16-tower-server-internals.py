"""🖥️ Tower server: the power supply and the CPU tower cooler as real heat sources inside the case of the thermal list at 1:200 and 1:500, where only the skins existed."""
from rework_16_lib import load, save, put, box, thermal, decide

doc = load("tower-server")
for scale in ("200", "500"):
    volume = doc["volumes"][scale]
    put(volume["thermal"], thermal(box("th-psu-case", "case", "chassis-steel", [0.08, 0.135, 0.15], [0.385, 0.13, 0.0], tone=0.45), "psu-vent", inner=True))
    put(volume["thermal"], thermal(box("th-heat-sink", "case", "chassis-steel", [0.12, 0.09, 0.12], [0.275, 0.1, 0.01], tone=0.6), "cpu-grille", inner=True))
    decide(volume, "Thermal: the two heat sources are real parts inside the hollow skins, the ATX power supply (150 x 135 x 80 mm) under the top at the rear and the CPU tower cooler with its fan (120 x 120 x 90 mm) on the mainboard wall behind the side grille.", "Thermik: Die beiden Wärmequellen sind echte Bauteile in den hohlen Häuten, das ATX-Netzteil (150 x 135 x 80 mm) hinten unter dem Deckel und der CPU-Turmkühler mit Lüfter (120 x 120 x 90 mm) an der Mainboardwand hinter dem Seitengitter.")
save("tower-server", doc)
