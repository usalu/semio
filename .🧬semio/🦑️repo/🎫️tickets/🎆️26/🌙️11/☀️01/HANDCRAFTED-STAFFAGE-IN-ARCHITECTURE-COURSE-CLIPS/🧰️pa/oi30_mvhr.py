"""🌬️ pa: OI-30 - the insulated MVHR casing runs about 1.5 K above the room (12 W, not 45 W) and the unit gets a steady normal-spin loop clip - run once."""
from jio import load, save

path, data = load("mvhr-unit")
a = data["asset"]
next(r for r in a["thermal"]["regions"] if r["id"] == "casing")["operating"]["heatW"] = 12

pairs = {
    "en": [
        (
            "The 75 W of the two fans end entirely as heat: 45 W at the casing (extract fan, electronics) and 30 W at the supply impeller;",
            "The 75 W of the two fans end entirely as heat: about 33 W warm the air streams (the 0.5 K of fan heat in the supply air), 30 W show at the supply impeller and 12 W at the casing (extract fan, electronics), which behind its 20 mm insulation of R 0.5 runs only about 1.5 K above the room;",
        ),
        (
            "the two fans take 75 W (45 W at the casing, 30 W at the supply impeller);",
            "the two fans take 75 W (about 33 W warm the air streams, 12 W show at the casing behind its insulation, which runs about 1.5 K above the room, and 30 W at the supply impeller);",
        ),
    ],
    "de": [
        (
            "Die 75 W der beiden Ventilatoren enden vollständig als Wärme: 45 W am Gehäuse (Abluftventilator, Elektronik) und 30 W am Zuluftlaufrad;",
            "Die 75 W der beiden Ventilatoren enden vollständig als Wärme: etwa 33 W erwärmen die Luftströme (die 0,5 K Ventilatorwärme in der Zuluft), 30 W zeigen sich am Zuluftlaufrad und 12 W am Gehäuse (Abluftventilator, Elektronik), das hinter seiner 20-mm-Dämmung mit R 0,5 nur etwa 1,5 K über dem Raum liegt;",
        ),
        (
            "die beiden Ventilatoren nehmen 75 W auf (45 W am Gehäuse, 30 W am Zuluftlaufrad);",
            "die beiden Ventilatoren nehmen 75 W auf (etwa 33 W erwärmen die Luftströme, 12 W zeigen sich am Gehäuse hinter seiner Dämmung, das etwa 1,5 K über dem Raum liegt, und 30 W am Zuluftlaufrad);",
        ),
    ],
}

def patch(holder):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)

patch(a["provenance"]["notes"])
patch(a["thermal"]["assumptions"][1])

spin = {
    "id": "normal-spin",
    "title": {"en": "Steady spin at the normal stage", "de": "Gleichmäßiger Lauf auf der Grundstufe"},
    "description": {
        "en": "The impeller turns once in three seconds at the normal stage (120 degrees per second, the rate of the spin-up clip); the picture is a slow-motion view: the real impeller turns at about 2000 revolutions per minute.",
        "de": "Das Laufrad dreht sich auf der Grundstufe in drei Sekunden einmal (120 Grad je Sekunde, die Rate des Hochlaufs); das Bild zeigt eine Zeitlupe: das echte Laufrad dreht mit etwa 2000 Umdrehungen je Minute.",
    },
    "durationS": 3,
    "fps": 15,
    "loop": True,
    "rootMotion": "none",
    "keys": [
        {"t": 0, "pose": "normal", "ease": "linear", "bones": {"impeller": {"angleDeg": 0}}},
        {"t": 3, "pose": "normal", "ease": "linear", "bones": {"impeller": {"angleDeg": 360}}},
    ],
    "contacts": [
        {"anchor": "wall-hook", "fromS": 0, "toS": 3},
        {"anchor": "wall-rest", "fromS": 0, "toS": 3},
    ],
    "phases": [{"id": "run", "fromS": 0, "toS": 3, "title": {"en": "Turning", "de": "Dreht"}}],
    "events": [],
    "tracks": [],
}
if not any(c["id"] == "normal-spin" for c in a["clips"]):
    index = next(i for i, c in enumerate(a["clips"]) if c["id"] == "boost-spin")
    a["clips"].insert(index + 1, spin)
save(path, data)
print("ok")
