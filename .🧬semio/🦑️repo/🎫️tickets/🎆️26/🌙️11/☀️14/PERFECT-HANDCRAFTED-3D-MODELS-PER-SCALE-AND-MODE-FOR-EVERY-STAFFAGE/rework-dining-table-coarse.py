"""🔩 Dining table, second round pass 1: lift mechanism modelled per scale and the decisions texts follow the rebuilt slide."""
import json
from pathlib import Path

SOURCE = Path(__file__).resolve().parent / "volumes/furniture/dining-table.json"
TEXT = {
    "50": [("a 25 x 12 mm steel tube of 2 mm sheet, and in the tubes the two stationary 21 x 8 mm centre bars of the telescopic slide (tube-in-tube)",
            "a 67.5 x 12 mm slotted steel C-rail of 2 mm sheet on the inner apron face, and in the rails the two stationary 63.5 x 8 mm centre flats on edge of the telescopic slide; the section is as deep as the apron allows because each half stands on two legs only and the slide alone carries its tipping moment"),
           ("ein Stahlrohr 25 x 12 mm aus 2 mm Blech und in den Rohren die zwei feststehenden Mittelstäbe 21 x 8 mm des Teleskopauszugs (Rohr in Rohr)",
            "eine geschlitzte Stahl-C-Schiene 67,5 x 12 mm aus 2 mm Blech an der Zargeninnenseite und in den Schienen die zwei feststehenden hochkant stehenden Mittelstege 63,5 x 8 mm des Teleskopauszugs; der Querschnitt ist so hoch, wie die Zarge erlaubt, weil jede Hälfte nur auf zwei Beinen steht und allein der Auszug ihr Kippmoment trägt")],
    "100": [("one 18 x 20 mm steel slide", "one 18 x 65 mm steel slide")],
    "200": [("one 30 mm slide bar per side", "one 30 x 70 mm slide per side behind the apron; the lift crossheads are plain flats without their slots (under 30 mm)")],
    "500": [("The two stationary steel slide rails connect to the transverse lift mechanism.",
             "The two stationary steel slide rails, 66 mm deep behind the aprons, connect to the transverse lift crossheads, which are plain flats; the lifting nuts (50 mm) are below the feature and omitted.")],
}

document = json.loads(SOURCE.read_text())
for scale, volume in document["volumes"].items():
    for decision in volume["decisions"]:
        for language in decision:
            for old, new in TEXT[scale]:
                decision[language] = decision[language].replace(old, new)
    if scale in ("200", "500"):
        for solid in volume["body"]:
            if solid["id"].startswith("lift-crosshead"):
                solid.pop("holes", None)
    if scale == "500":
        gone = {solid["id"] for solid in volume["body"] if solid["id"].startswith("lift-nut")}
        for name in ("body", "thermal", "structure"):
            volume[name] = [solid for solid in volume[name] if solid["id"] not in gone and solid.get("of") not in gone]
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("[DEBUG] coarse lift mechanism simplified")
