"""💻 pp: raise the laptop full office load from 45 W to 60 W (17/14/14 W to 22.8/18.6/18.6 W) with matching texts."""
import glob
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = Path(glob.glob(str(ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/laptop/🔣️.json"))[0])
text = path.read_bytes().decode("utf-8")
pairs = [
    ("releases 45 W of electrical power", "releases 60 W of electrical power"),
    ("1.4 W asleep with the lid closed", "1.8 W asleep with the lid closed"),
    ("gibt er 45 W elektrische", "gibt er 60 W elektrische"),
    ("geschlossen im Ruhezustand 1,4 W.", "geschlossen im Ruhezustand 1,8 W."),
    ("full office load of 45 W", "full office load of 60 W"),
    ("volle Büroauslastung von 45 W", "volle Büroauslastung von 60 W"),
    ("nothing computed: 11 W", "nothing computed: 15 W"),
    ("keine Rechenlast: 11 W", "keine Rechenlast: 15 W"),
    ("sleeps at about 1.4 W", "sleeps at about 1.8 W"),
    ("ruht bei etwa 1,4 W", "ruht bei etwa 1,8 W"),
    ("Sleep (1.4 W)", "Sleep (1.8 W)"),
    ("Ruhezustand (1,4 W)", "Ruhezustand (1,8 W)"),
    ("Idle (11 W)", "Idle (15 W)"),
    ("Leerlauf (11 W)", "Leerlauf (15 W)"),
    ("Full office load (45 W)", "Full office load (60 W)"),
    ("Volle Büroauslastung (45 W)", "Volle Büroauslastung (60 W)"),
    ('"heatW": 17\n', '"heatW": 22.8\n'),
    ("release 17 W, 14 W and 14 W at level 1 (45 W together, class of 30 to 60 W)", "release 22.8 W, 18.6 W and 18.6 W at level 1 (60 W together: a laptop under load with its charger, class of 30 to 60 W)"),
    ("17 W, 14 W und 14 W ab (zusammen 45 W, Klasse von 30 bis 60 W)", "22,8 W, 18,6 W und 18,6 W ab (zusammen 60 W: ein Laptop unter Last mit Netzteil, Klasse von 30 bis 60 W)"),
    ("the underside stays near 47 degC", "the underside runs near 55 degC"),
    ("die Unterseite bleibt bei etwa 47 degC", "die Unterseite liegt bei etwa 55 degC"),
]
for old, new in pairs:
    assert old in text, old
    text = text.replace(old, new)
assert text.count('"heatW": 14\n') == 2
text = text.replace('"heatW": 14\n', '"heatW": 18.6\n')
path.write_bytes(text.encode("utf-8"))
print("ok")
