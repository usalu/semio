"""🫁 bp: second pass: nose jets at about 45 degrees below the head axis, plume diameters for 0.2 m/s source velocity."""
from pathlib import Path

root = Path(__file__).resolve().parents[8]
base = root / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"


def edit(asset: str, pairs: list[tuple[str, str]]) -> None:
    path = base / asset / "🔣️.json"
    text = path.read_bytes().decode("utf8")
    for old, new in pairs:
        assert text.count(old) == 1, (asset, old, text.count(old))
        text = text.replace(old, new)
    path.write_bytes(text.encode("utf8"))


edit("standing-adult", [('"directionDeg": -150,\n          "diameterM": 0.015,', '"directionDeg": -135,\n          "diameterM": 0.015,'), ('"diameterM": 0.18,\n          "region": "hoodie",', '"diameterM": 0.16,\n          "region": "hoodie",')])
edit("seated-office-worker", [('"directionDeg": -130,\n          "diameterM": 0.015,', '"directionDeg": -105,\n          "diameterM": 0.015,'), ('"diameterM": 0.16,\n          "region": "shirt",', '"diameterM": 0.14,\n          "region": "shirt",')])
edit("person-cooking", [('"directionDeg": -150,\n          "diameterM": 0.015,', '"directionDeg": -135,\n          "diameterM": 0.015,'), ('"diameterM": 0.18,\n          "region": "hoodie",', '"diameterM": 0.16,\n          "region": "hoodie",')])
