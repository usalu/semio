"""🔥 tp: electric-fan-heater: cool the grille and cover to plastic-safe temperatures and move the heat to the jet air (stage 1 at 49 K, stage 2 at 50 K)."""
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-fan-heater/🔣️.json"
text = PATH.read_text(encoding="utf-8")
PAIRS = [
    ("while grille, top cover and housing release the remaining 186 W.", "while grille, top cover and housing release the remaining 88 W."),
    ("während Gitter, Haube und Gehäuse die restlichen 186 W abgeben.", "während Gitter, Haube und Gehäuse die restlichen 88 W abgeben."),
    ("1814 W as warm air (2 x 0.016 m3/s at 47 K rise), 60 W in the outlet grille, 93 W in the top cover and 33 W in the housing; the split is an authored estimate.",
     "1912 W as warm air (2 x 0.016 m3/s at 49 K and 50 K rise), 25 W in the outlet grille, 30 W in the top cover and 33 W in the housing; the split is an authored estimate that keeps the plastic cover near 70 degC and the grille bars near 75 degC."),
    ("1814 W als Warmluft (2 x 0,016 m3/s bei 47 K Anstieg), 60 W im Auslassgitter, 93 W in der Haube und 33 W im Gehäuse; die Aufteilung ist ein Schätzwert.",
     "1912 W als Warmluft (2 x 0,016 m3/s bei 49 K und 50 K Anstieg), 25 W im Auslassgitter, 30 W in der Haube und 33 W im Gehäuse; die Aufteilung ist ein Schätzwert, der die Kunststoffhaube bei etwa 70 degC und die Gitterstäbe bei etwa 75 degC hält."),
    ("air warmed by 47 K out of the front grille.", "air warmed by 49 K out of the front grille."),
    ("um 47 K erwärmte Luft aus dem vorderen Gitter.", "um 49 K erwärmte Luft aus dem vorderen Gitter."),
    ("air warmed by 47 K leaves the front grille as a warm jet", "air warmed by about 50 K leaves the front grille as a warm jet"),
    ("um 47 K erwärmte Luft verlassen das vordere Gitter", "um etwa 50 K erwärmte Luft verlassen das vordere Gitter"),
    ('"heatW": 93', '"heatW": 30'),
    ('"heatW": 60', '"heatW": 25'),
    ("their 2000 W leave as warm jet air (1814 W), through the outlet grille (60 W), top cover (93 W) and housing (33 W).",
     "their 2000 W leave as warm jet air (1912 W), through the outlet grille (25 W), top cover (30 W) and housing (33 W)."),
    ("ihre 2000 W verlassen das Gerät als warme Strahlluft (1814 W), durch das Auslassgitter (60 W), die Haube (93 W) und das Gehäuse (33 W).",
     "ihre 2000 W verlassen das Gerät als warme Strahlluft (1912 W), durch das Auslassgitter (25 W), die Haube (30 W) und das Gehäuse (33 W)."),
]
for old, new in PAIRS:
    count = text.count(old)
    if count != 1:
        print("MISS", count, old[:70])
        continue
    text = text.replace(old, new)
low, high = '"temperatureRiseK": 47,', '"temperatureRiseK": 49,'
first = text.index(low)
text = text[:first] + high + text[first + len(low):]
second = text.index(low)
text = text[:second] + '"temperatureRiseK": 50,' + text[second + len(low):]
PATH.write_text(text, encoding="utf-8", newline="\n")
print("done")
