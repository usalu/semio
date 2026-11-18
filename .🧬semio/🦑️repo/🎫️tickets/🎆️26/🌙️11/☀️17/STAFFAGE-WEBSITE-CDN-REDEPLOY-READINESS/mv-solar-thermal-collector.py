"""Solar thermal collector 1:500 decision texts: the front shows two feet, no foot block any more (the shapes are in the first pass of this asset)."""
from mvlib import *
d = D("🏗️machinery/solar-thermal-collector")
dec = d.at("asset", "representations", "500", "decisions")
d.sub(dec, "one merged block on a foot block in the front view", "one merged block on two feet in the front view")
d.sub(dec, "ein zusammengeführter Block auf einem Fussblock in der Vorderansicht", "ein zusammengeführter Block auf zwei Füssen in der Vorderansicht")
d.sub(dec, "Fussblock und Plattenfläche in der Vorderansicht", "beide Füsse und Plattenfläche in der Vorderansicht")
d.commit()
