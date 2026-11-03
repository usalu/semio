"""🍞 pa: OI-26 - the toaster top plate releases 25 W (about 550 W/m2, 70 to 75 degrees C), the rest of its former 100 W leaves with the slot air - run once."""
from jio import load, save

path, data = load("toaster")
a = data["asset"]
th = a["thermal"]
next(r for r in th["regions"] if r["id"] == "top-plate")["operating"]["heatW"] = 25
em = next(e for e in th["emitters"] if e["id"] == "slot-air")
em["temperatureRiseK"] = 103

pairs = {
    "en": [
        ("100 W the top plate, 150 W the crust of the bread and a 470 W stream", "25 W the top plate, 150 W the crust of the bread and a 545 W stream"),
        ("100 W in the top plate, 150 W in the crust (browning, drying) and 470 W as hot slot air (0.0044 m3/s at 88 K rise: 470 W;", "25 W in the top plate, 150 W in the crust (browning, drying) and 545 W as hot slot air (0.0044 m3/s at 103 K rise: 545 W;"),
        ("the split is an authored estimate.", "the top plate of 0.045 m2 carries 555 W/m2, which at a combined convective and radiative coefficient of about 10 W/m2K is a rise of 55 K (about 75 degrees C), the usual touch temperature of a toaster top; the rest of the wire power rises through the slots; the split is an authored estimate."),
        ("the top plate (100 W), the bread crust (150 W) and the hot air through the slots (470 W)", "the top plate (25 W), the bread crust (150 W) and the hot air through the slots (545 W)"),
    ],
    "de": [
        ("100 W die Deckplatte, 150 W die Brotkruste, und ein Strom heißer Luft von 470 W", "25 W die Deckplatte, 150 W die Brotkruste, und ein Strom heißer Luft von 545 W"),
        ("100 W in der Deckplatte, 150 W in der Kruste (Bräunung, Trocknung) und 470 W als heiße Schlitzluft (0,0044 m3/s bei 88 K Anstieg: 470 W;", "25 W in der Deckplatte, 150 W in der Kruste (Bräunung, Trocknung) und 545 W als heiße Schlitzluft (0,0044 m3/s bei 103 K Anstieg: 545 W;"),
        ("die Aufteilung ist ein Schätzwert.", "die Deckplatte von 0,045 m2 trägt 555 W/m2, bei einem kombinierten Konvektions- und Strahlungskoeffizienten von etwa 10 W/m2K ein Anstieg von 55 K (etwa 75 Grad C), die übliche Berührtemperatur einer Toasterdecke; der Rest der Heizdrahtleistung steigt durch die Schlitze auf; die Aufteilung ist ein Schätzwert."),
        ("die Deckplatte (100 W), die Brotkruste (150 W) und die heiße Luft aus den Schlitzen (470 W)", "die Deckplatte (25 W), die Brotkruste (150 W) und die heiße Luft aus den Schlitzen (545 W)"),
    ],
}

def patch(holder):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)

patch(a["summary"])
patch(a["provenance"]["notes"])
patch(th["assumptions"][0])
save(path, data)
print("ok")
