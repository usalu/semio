"""🪑️ Author 14, office swivel chair: the decisions of 1:50, 1:100 and 1:200 say that the castors now carry in the structure list.

Run from the ticket folder after rework-14-office-swivel-chair-structure.py: python3 rework-14-office-swivel-chair-decisions.py
"""
import json

PATH = "volumes/furniture/office-swivel-chair.json"
EDITS = {
    "50": [
        ("one contact pad under each castor;", "between stem and floor every castor carries with its neck, swivel head, centre web and both wheels (rim and tyre), so the path from the arm to the contact pad under each castor is closed;"),
        ("je ein Kontaktpolster unter jeder Rolle;", "zwischen Stift und Boden trägt jede Rolle mit Hals, Drehkopf, Mittelsteg und beiden Rädern (Felge und Reifen), sodass der Pfad vom Arm bis zum Kontaktpolster unter jeder Rolle geschlossen ist;"),
        ("Cushion, pad, hoods, levers and knob carry nothing and are absent.", "Cushion, pad, castor hoods, levers and knob carry nothing and are absent."),
        ("Polster, Hauben, Hebel und Spannrad tragen nichts und fehlen.", "Polster, Rollenhauben, Hebel und Spannrad tragen nichts und fehlen."),
    ],
    "100": [
        ("tubes for standpipe and piston,", "the swivel head and the two wheels of every castor as members down to its contact pad, tubes for standpipe and piston,"),
        ("Rohre für Standrohr und Kolben,", "Drehkopf und beide Räder jeder Rolle als Stäbe bis zu ihrem Auflager, Rohre für Standrohr und Kolben,"),
    ],
    "200": [
        ("Structure: hub, arms and bosses,", "Structure: hub, arms and bosses, the wheel body of every castor under its boss,"),
        ("Tragwerk: Nabe, Arme und Augen,", "Tragwerk: Nabe, Arme und Augen, der Radkörper jeder Rolle unter seinem Auge,"),
    ],
}

document = json.load(open(PATH))
for scale, edits in EDITS.items():
    decision = document["volumes"][scale]["decisions"][1]
    for old, new in edits:
        language = "en" if old in decision["en"] else "de"
        if new in decision[language]:
            continue
        assert old in decision[language], old
        decision[language] = decision[language].replace(old, new)
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
