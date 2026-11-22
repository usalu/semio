"""🔥 Extractor hood and fan heater: the heat sources that the body already models as real internal parts (blower motor; heating coils and fan motor) join the thermal list at every scale as references to those very solids."""
from rework_16_lib import load, save, put, decide

hood = load("kitchen-extractor-hood")
for volume in hood["volumes"].values():
    put(volume["thermal"], {"id": "th-blower-motor", "of": "blower-motor", "region": "canopy", "sectionFaces": ["+u", "-u", "+v", "-v", "+w", "-w"]})
    decide(volume, "Thermal: the heat source of the canopy is a real part inside it, the external-rotor motor of the centrifugal blower in the motor box above the filters; the two lamps under the canopy are the other sources.", "Thermik: Die Wärmequelle der Haube ist ein echtes Bauteil in ihrem Inneren, der Aussenläufermotor des Radialgebläses im Motorkasten über den Filtern; die zwei Leuchten unter der Haube sind die weiteren Quellen.")
save("kitchen-extractor-hood", hood)

heater = load("electric-fan-heater")
for volume in heater["volumes"].values():
    for ident, region in (("coil-stage-1", "grille"), ("coil-stage-2", "grille"), ("motor-stack", "casing")):
        put(volume["thermal"], {"id": f"th-{ident}", "of": ident, "region": region, "sectionFaces": ["+u", "-u", "+v", "-v", "+w", "-w"]})
    decide(volume, "Thermal: the heat sources are real parts inside the casing, the two resistance coil stages in the heater cassette behind the outlet grille and the shaded-pole motor of the axial fan behind them.", "Thermik: Die Wärmequellen sind echte Bauteile im Gehäuse, die zwei Heizwendelstufen in der Heizkassette hinter dem Ausblasgitter und der Spaltpolmotor des Axiallüfters dahinter.")
save("electric-fan-heater", heater)
