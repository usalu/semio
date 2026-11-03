"""🛢️ pa: OI-33 - pumpjack height 6.3 m (horsehead up, 5.5 m at mid-stroke) and a 30 kW motor at 93 % (1.8 kW at 24 kW shaft load); expansion valve 0.45 kg - run once."""
from jio import load, save


def patch(holder, pairs):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)


path, data = load("oil-pumpjack")
a = data["asset"]
a["dimensions"]["heightM"] = 6.3
regions = {r["id"]: r for r in a["thermal"]["regions"]}
regions["motor"]["operating"]["heatW"] = 1800
regions["gearbox"]["operating"]["heatW"] = 720

patch(a["provenance"]["notes"], {
    "en": [("and the 0.9 kW and 0.7 kW losses of gear reducer and motor are authored estimates for this class (about 20 kW rated motor power at 95 percent efficiency and 97 percent for the gears).", "and the 0.72 kW and 1.8 kW losses of gear reducer and motor are authored estimates for this class (a 30 kW prime mover at 93 percent efficiency carrying a mean shaft load of 24 kW loses 24 / 0.93 - 24 = 1.8 kW; the gears at 97 percent lose 0.72 kW). The unit is 6.3 m high at the top of the stroke with the horsehead up (the stopped pose) and 5.5 m at mid-stroke.")],
    "de": [("und die Verluste von 0,9 kW und 0,7 kW von Getriebe und Motor sind Schätzwerte für die", "und die Verluste von 0,72 kW und 1,8 kW von Getriebe und Motor sind Schätzwerte für die")],
})
patch(a["provenance"]["notes"], {"de": [("(etwa 20 kW Nennleistung des Motors bei 95 Prozent Wirkungsgrad und 97 Prozent für das Getriebe)", "(ein Antriebsmotor von 30 kW bei 93 Prozent Wirkungsgrad mit einer mittleren Wellenlast von 24 kW verliert 24 / 0,93 - 24 = 1,8 kW; das Getriebe mit 97 Prozent verliert 0,72 kW). Die Einheit ist am oberen Hubende mit hochstehendem Pferdekopf (Pose gestoppt) 6,3 m hoch und in Hubmitte 5,5 m")]})
patch(a["thermal"]["assumptions"][0], {
    "en": [("Motor and gear reducer lose 700 W and 900 W as heat while pumping;", "Motor and gear reducer lose 1800 W and 720 W as heat while pumping (30 kW motor at 93 percent efficiency under a mean shaft load of 24 kW, gears at 97 percent);")],
    "de": [("Motor und Getriebe verlieren beim Pumpen 700 W und 900 W als Wärme;", "Motor und Getriebe verlieren beim Pumpen 1800 W und 720 W als Wärme (30-kW-Motor mit 93 Prozent Wirkungsgrad bei einer mittleren Wellenlast von 24 kW, Getriebe mit 97 Prozent);")],
})
rep = a["representations"]["50"]["decisions"]["silhouette"]
patch(rep, {
    "en": [("A 9 m wide, 5.5 m tall conventional oil pumpjack, 18 cm by 11 cm on paper:", "A 9 m wide, 6.3 m tall conventional oil pumpjack (horsehead up), 18 cm by 13 cm on paper:")],
    "de": [("Eine 9 m breite und 5,5 m hohe klassische Erdölpumpe, auf Papier 18 cm mal 11 cm:", "Eine 9 m breite und 6,3 m hohe klassische Erdölpumpe (Pferdekopf oben), auf Papier 18 cm mal 13 cm:")],
})
save(path, data)

path, data = load("expansion-valve")
a = data["asset"]
a["dimensions"]["massKg"] = 0.45
masses = {"body": 0.3, "head": 0.08, "stem": 0.02, "bulb": 0.05}
for bone in a["rig"]["bones"]:
    if bone["id"] in masses:
        bone["mass"]["kg"] = masses[bone["id"]]
patch(a["provenance"]["notes"], {
    "en": [("The mass is 0.81 kg.", "The mass is 0.45 kg (a small forged brass thermostatic valve of 0.3 to 0.6 kg).")],
    "de": [("Die Masse beträgt 0,81 kg.", "Die Masse beträgt 0,45 kg (ein kleines geschmiedetes Thermostatventil aus Messing wiegt 0,3 bis 0,6 kg).")],
})
for item in a["thermal"]["assumptions"]:
    patch(item, {"en": [("the 0.8 kg is carried", "the 0.45 kg is carried")], "de": [("die 0,8 kg", "die 0,45 kg")]})
for item in a["structure"]["assumptions"]:
    patch(item, {"en": [("The 0.8 kg valve", "The 0.45 kg valve")], "de": [("Das 0,8 kg schwere Ventil", "Das 0,45 kg schwere Ventil")]})
save(path, data)
print("ok")
