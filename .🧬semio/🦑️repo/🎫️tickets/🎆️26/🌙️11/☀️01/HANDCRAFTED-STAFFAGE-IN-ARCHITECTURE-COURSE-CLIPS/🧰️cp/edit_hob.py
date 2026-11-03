"""🔥 Hob heat budget: 865 W flame, 435 W metal, 1200 W into the pot."""
from pathlib import Path

F = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"
s = F.read_bytes().decode("utf-8")
pairs = [
    ('"heatW": 1500', '"heatW": 865'),
    (
        "puts 1500 W into the flame region (about 60 percent of the rated input leaves the flame directly as plume and radiation, the rest reaches the pan) and 260 W",
        "puts 865 W into the flame region (about 35 percent of the rated input leaves the flame directly as plume and radiation, the pot absorbs about 1200 W, 48 percent, as the cooking-pot asset books it) and 260 W",
    ),
    (
        "legt 1500 W in die Flammenregion (rund 60 Prozent der Nennbelastung verlassen die Flamme direkt als Fahne und Strahlung, der Rest erreicht den Topf) und 260 W",
        "legt 865 W in die Flammenregion (rund 35 Prozent der Nennbelastung verlassen die Flamme direkt als Fahne und Strahlung, der Topf nimmt etwa 1200 W auf, 48 Prozent, wie es das Asset cooking-pot bucht) und 260 W",
    ),
    (
        "1500 W into the flame region (plume and radiation), 435 W into plate, burner head and pan support and the rest into a pan above;",
        "865 W into the flame region (plume and radiation), 435 W into plate, burner head and pan support and 1200 W into the pot above (cooking-pot: 1155 W water, 20 W base, 25 W wall);",
    ),
    (
        "1500 W in die Flammenregion (Fahne und Strahlung), 435 W in Platte, Brennerkopf und Pfannenträger und der Rest in einen Topf darüber;",
        "865 W in die Flammenregion (Fahne und Strahlung), 435 W in Platte, Brennerkopf und Pfannenträger und 1200 W in den Topf darüber (cooking-pot: 1155 W Wasser, 20 W Boden, 25 W Wand);",
    ),
]
for old, new in pairs:
    assert s.count(old) == 1, old[:50]
    s = s.replace(old, new)
F.write_bytes(s.encode("utf-8"))
