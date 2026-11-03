"""🔥 pa: OI-22 - a condensing boiler loses 2 to 3 % of its input up the flue - run once."""
from jio import load, save

path, data = load("gas-boiler")
a = data["asset"]
em = next(e for e in a["thermal"]["emitters"] if e["id"] == "flue-gas")
em["volumeFlowM3S"] = 0.0077
em["temperatureRiseK"] = 46

swaps = {
    "en": [
        ("the flue gas leaves at 38 K above the room.", "the flue gas carries about 430 W (2.1 % of the input) at 46 K above the room."),
        (
            "The flue gas of a condensing boiler leaves at about 59 degrees C (38 K above the room): 0.0072 m3/s at 20 kW with a water vapour excess of 0.07 kg/kg after condensation, estimated from the natural-gas combustion with excess air 1.2 (Turns).",
            "The flue gas of a condensing boiler at full output with hot-water return leaves at about 67 degrees C (46 K above the room): 0.0077 m3/s at 20 kW with a water vapour excess of 0.07 kg/kg after condensation, estimated from the natural-gas combustion with excess air 1.2 (Turns). Its sensible heat is about 430 W, 2.1 % of the 20 kW input, inside the 2 to 3 % stack loss of condensing boilers (the temperature rise and the flow both scale with the burner level, so the loss falls faster than the input when modulated).",
        ),
    ],
    "de": [
        ("das Abgas tritt mit 38 K über der Raumtemperatur aus.", "das Abgas führt etwa 430 W (2,1 % der Zufuhr) mit 46 K über der Raumtemperatur ab."),
        (
            "Das Abgas eines Brennwertkessels tritt mit etwa 59 Grad C (38 K über dem Raum) aus: 0,0072 m3/s bei 20 kW",
            "Das Abgas eines Brennwertkessels bei voller Leistung mit Warmwasser-Rücklauf tritt mit etwa 67 Grad C (46 K über dem Raum) aus: 0,0077 m3/s bei 20 kW",
        ),
        (
            "geschätzt aus der Erdgasverbrennung mit Luftüberschuss 1,2 (Turns). Die Flamme",
            "geschätzt aus der Erdgasverbrennung mit Luftüberschuss 1,2 (Turns). Seine fühlbare Wärme beträgt etwa 430 W, 2,1 % der Zufuhr von 20 kW, innerhalb der Abgasverluste von 2 bis 3 % bei Brennwertkesseln (Temperaturerhöhung und Volumenstrom skalieren beide mit der Brennerstufe, daher fällt der Verlust bei Modulation schneller als die Zufuhr). Die Flamme",
        ),
    ],
}
for lang, pairs in swaps.items():
    for key in ("summary",):
        for old, new in pairs:
            if old in a[key][lang]:
                a[key][lang] = a[key][lang].replace(old, new)
    for old, new in pairs:
        if old in a["provenance"]["notes"][lang]:
            a["provenance"]["notes"][lang] = a["provenance"]["notes"][lang].replace(old, new)

pairs_assume = {
    "en": ("the 20 kW of fuel go into the heating water and the flue gas.", "the 20 kW of fuel go into the heating water and the flue gas (about 430 W, 2.1 %, up the flue)."),
    "de": ("die 20 kW Brennstoff gehen in das Heizwasser und das Abgas.", "die 20 kW Brennstoff gehen in das Heizwasser und das Abgas (etwa 430 W, 2,1 %, über den Abzug)."),
}
for lang, (old, new) in pairs_assume.items():
    a["thermal"]["assumptions"][0][lang] = a["thermal"]["assumptions"][0][lang].replace(old, new)

save(path, data)
print("ok")
