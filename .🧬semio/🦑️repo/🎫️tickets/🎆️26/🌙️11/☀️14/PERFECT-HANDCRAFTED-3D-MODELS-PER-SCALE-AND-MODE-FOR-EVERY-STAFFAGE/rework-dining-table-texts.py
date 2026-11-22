"""📝 Dining table, second round pass 1: notes, drawing decisions and German volume decisions follow the rebuilt slide."""
import json
from pathlib import Path

T = Path(__file__).resolve().parent
SOURCE = T / "volumes/furniture/dining-table.json"
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/dining-table/🔣️.json")
CATALOG_TEXT = [
    ("(tube in tube): a 25 x 12 mm tube of 2 mm sheet on each long apron and a stationary 21 x 8 mm centre bar per side running in the tubes of both halves",
     "(flat in rail): a 67.5 x 12 mm slotted C-rail of 2 mm sheet on the inner face of each long apron and a stationary 63.5 x 8 mm centre flat on edge per side running in the rails of both halves; it alone carries the tipping moment of the two-legged halves"),
    ("(Rohr in Rohr): je Längszarge ein Rohr 25 x 12 mm aus 2 mm Blech und je Seite ein feststehender Mittelstab 21 x 8 mm, der in den Rohren beider Hälften läuft",
     "(Steg in Schiene): je Längszarge innen eine geschlitzte C-Schiene 67,5 x 12 mm aus 2 mm Blech und je Seite ein feststehender, hochkant stehender Mittelsteg 63,5 x 8 mm, der in den Schienen beider Hälften läuft; er allein trägt das Kippmoment der zweibeinigen Hälften"),
    ("the 8 mm centre bar as a 17 mm filled strip", "the centre flat of the slide at its real 63.5 mm depth (1.3 mm) as a filled strip"),
    ("der 8-mm-Mittelstab als 17 mm hoher gefüllter Streifen", "der Mittelsteg des Auszugs in seiner wahren Höhe von 63,5 mm (1,3 mm) als gefüllter Streifen"),
]
SOURCE_TEXT = [
    ("je Seite ein Stahlauszug 18 x 20 mm", "je Seite ein Stahlauszug 18 x 65 mm"),
    ("je Seite ein 30-mm-Auszugsstab", "je Seite ein Auszug 30 x 70 mm hinter der Zarge; die Hubquerträger sind glatte Flachstähle ohne ihre Langlöcher (unter 30 mm)"),
    ("Zwei feststehende Stahl-Auszugsschienen tragen den querliegenden Hubmechanismus.",
     "Zwei feststehende Stahl-Auszugsschienen, 66 mm hoch hinter den Zargen, tragen die querliegenden Hubquerträger als glatte Flachstähle; die Hubmuttern (50 mm) liegen unter dem Merkmal und entfallen."),
]


def apply(path, pairs):
    text = path.read_text()
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"[DEBUG] {old[:50]!r} found {text.count(old)} times in {path.name}")
        text = text.replace(old, new)
    json.loads(text)
    path.write_text(text)


apply(CATALOG, CATALOG_TEXT)
apply(SOURCE, SOURCE_TEXT)
print("[DEBUG] texts updated")
