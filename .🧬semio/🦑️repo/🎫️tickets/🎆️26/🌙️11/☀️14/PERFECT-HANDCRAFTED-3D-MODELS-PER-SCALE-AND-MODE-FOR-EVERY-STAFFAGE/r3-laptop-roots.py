"""🌱 Round 3, appliances/laptop: assembles the candidate asset document (corrected rig, anchors, envelope, materials, references, notes, handcrafted volumes, redrawn 2D) under 🗑️generated/r3-laptop/, so the tools can judge it before the catalog file is touched.

Usage: python3 r3-laptop-roots.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/laptop/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-laptop")


def module(name):
    """📦 Loads one of the sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_laptop_" + name, os.path.join(HERE, f"r3-laptop-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K, M, D = module("kit"), module("model"), module("draw")

REFERENCES = [
    {
        "id": "asus-zenbook-14-ux3402-2022",
        "citation": "ASUSTeK Computer Inc. (2022). Zenbook 14 OLED (UX3402) technical specifications: 14.0-inch 16:10 display, 313.6 x 220.6 x 16.9 mm, 1.39 kg, 75 Wh battery. https://www.asus.com/laptops/for-home/zenbook/zenbook-14-oled-ux3402/techspec/",
    },
    {
        "id": "apple-macbook-pro-14-2021",
        "citation": "Apple Inc. (2021). MacBook Pro (14-inch, 2021) technical specifications: 14.2-inch display, height 1.55 cm, width 31.26 cm, depth 22.12 cm, 1.6 kg. https://support.apple.com/kb/SP854",
    },
]
MATERIALS = [
    {
        "id": "keycap-plastic",
        "title": {"en": "Black ABS of key caps, keyboard bed and hinge cover", "de": "Schwarzes ABS der Tastenkappen, des Tastaturbetts und der Scharnierabdeckung"},
        "emissivity": 0.92,
        "solarAbsorptivity": 0.9,
        "densityKgM3": 1050,
        "conductivityWmK": 0.2,
        "specificHeatJKgK": 1400,
        "source": "literature",
        "reference": "iso-10456-2007",
    },
    {
        "id": "hinge-steel",
        "title": {"en": "Stainless steel of hinges and brackets", "de": "Edelstahl der Scharniere und Winkel"},
        "emissivity": 0.22,
        "solarAbsorptivity": 0.4,
        "densityKgM3": 7900,
        "conductivityWmK": 14.9,
        "specificHeatJKgK": 477,
        "source": "literature",
        "reference": "incropera-2018",
    },
]
GEOMETRY_NOTE = {
    "en": "Side elevation of a 14-inch aluminium clamshell laptop of the common thin-and-light class (ASUS Zenbook 14 OLED UX3402: 313.6 x 220.6 x 16.9 mm, 1.39 kg; Apple MacBook Pro 14-inch 2021: 312.6 x 221.2 x 15.5 mm, 1.6 kg): 0.313 m wide, 0.218 m deep, the chassis 10.5 mm thick on 2 mm rubber feet (the deck 12.5 mm above the desk, 13 mm rounded) and the lid 6.5 mm, 17 mm closed without the feet, 1.30 kg with battery. The bottom cover is a tub with a rounded bilge, the keyboard deck a plate with the keyboard well, the trackpad and a ledge at the rear, on which the 9 mm hinge barrel of the lid sits; the hinge axis lies 4.5 mm in front of the rear edge and 2 mm above the deck, and the gap under the barrel is the exhaust. The 14.0-inch 16:10 panel measures 301.6 x 188.5 mm behind a cover glass with a 3 mm rim. With the lid open at 105 degrees the envelope is 0.273 m long and 0.221 m high. All measures are rounded from that product class (authored estimate, no single product).",
    "de": "Seitenansicht eines 14-Zoll-Klapplaptops aus Aluminium der gängigen flachen Leichtklasse (ASUS Zenbook 14 OLED UX3402: 313,6 x 220,6 x 16,9 mm, 1,39 kg; Apple MacBook Pro 14 Zoll 2021: 312,6 x 221,2 x 15,5 mm, 1,6 kg): 0,313 m breit, 0,218 m tief, das Chassis 10,5 mm dick auf 2 mm hohen Gummifüßen (das Deck 12,5 mm über dem Tisch, gerundet 13 mm) und der Deckel 6,5 mm, geschlossen 17 mm ohne Füße, 1,30 kg mit Akku. Die Bodenschale ist eine Wanne mit gerundeter Kimm, das Tastaturdeck eine Platte mit Tastaturmulde, Touchpad und einem Absatz hinten, auf dem die 9-mm-Scharnierwalze des Deckels sitzt; die Scharnierachse liegt 4,5 mm vor der Hinterkante und 2 mm über dem Deck, und der Spalt unter der Walze ist der Abluftauslass. Das 14,0-Zoll-Panel im Format 16:10 misst 301,6 x 188,5 mm hinter einem Deckglas mit 3 mm Rand. Bei auf 105 Grad geöffnetem Deckel ist die Hülle 0,273 m lang und 0,221 m hoch. Alle Maße sind aus dieser Produktklasse gerundet (Schätzwert, kein einzelnes Produkt).",
}
OLD_NOTE_END = {"en": "(authored estimate, no single product).", "de": "(Schätzwert, kein einzelnes Produkt)."}
ANCHORS = {"feet-contact": (K.CENTRE_U, 0.0), "deck-top": (M.PLUME_U, K.DECK_V)}


def envelope():
    """📏 The envelope of the 1:50 body in the default pose: the lid opened by 105 degrees over the chassis."""
    corners = [K.lid_world(K.LID_CENTRE_U + ru, v) for v, _, ru in M.LID_50]
    return {"heightM": round(max(y for _, y in corners), 3), "widthM": round(K.CENTRE_U - min(x for x, _ in corners), 3)}


def candidate():
    """🧩 The catalog document with every round 3 change applied in memory."""
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    asset["rig"] = K.rig(asset["rig"])
    asset["anchors"] = [dict(a, x=ANCHORS[a["id"]][0], y=ANCHORS[a["id"]][1]) for a in asset["anchors"]]
    asset["dimensions"] = dict(asset["dimensions"], **envelope())
    asset["materials"] = [m for m in asset["materials"] if all(m["id"] != new["id"] for new in MATERIALS)] + MATERIALS
    refs = asset["provenance"]["references"]
    asset["provenance"]["references"] = [r for r in refs if all(r["id"] != new["id"] for new in REFERENCES)] + REFERENCES
    notes = asset["provenance"]["notes"]
    asset["provenance"]["notes"] = {lang: GEOMETRY_NOTE[lang] + notes[lang].split(OLD_NOTE_END[lang], 1)[1] for lang in ("en", "de")}
    asset["volumes"] = M.volumes()
    D.apply(asset)
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    print("candidate", os.path.join(OUT, "🔣️.json"), document["asset"]["dimensions"], {k: [sum(s.get("repeat", {}).get("count", 1) for s in v.get(l, [])) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
