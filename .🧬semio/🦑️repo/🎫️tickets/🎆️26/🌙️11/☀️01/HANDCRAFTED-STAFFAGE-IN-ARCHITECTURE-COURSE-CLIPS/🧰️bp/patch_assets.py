"""🫁 bp: exact-text edits of the breathing and plume data of the three transport humans (textual, so the file layout stays untouched)."""
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


edit("standing-adult", [
    ('"id": "quiet-standing",\n            "title": {\n              "en": "Quiet standing",\n              "de": "Ruhiges Stehen"\n            },\n            "rateBpm": 14,\n            "tidalVolumeL": 0.4,',
     '"id": "quiet-standing",\n            "title": {\n              "en": "Quiet standing",\n              "de": "Ruhiges Stehen"\n            },\n            "rateBpm": 12,\n            "tidalVolumeL": 0.5,'),
    ('"directionDeg": -150,\n          "diameterM": 0.0095,', '"directionDeg": -150,\n          "diameterM": 0.015,'),
    ('"directionDeg": -105,\n          "diameterM": 0.0124,', '"directionDeg": -105,\n          "diameterM": 0.02,'),
    ('"directionDeg": 0,\n          "diameterM": 0.25,\n          "region": "hoodie",', '"directionDeg": 0,\n          "diameterM": 0.18,\n          "region": "hoodie",'),
])

edit("seated-office-worker", [
    ('"de": "Sitzende Büroarbeit"\n            },\n            "rateBpm": 15,\n            "tidalVolumeL": 0.55,', '"de": "Sitzende Büroarbeit"\n            },\n            "rateBpm": 12.5,\n            "tidalVolumeL": 0.55,'),
    ('"directionDeg": -130,\n          "diameterM": 0.009,', '"directionDeg": -130,\n          "diameterM": 0.015,'),
    ('"directionDeg": -105,\n          "diameterM": 0.0118,', '"directionDeg": -105,\n          "diameterM": 0.02,'),
    ('"directionDeg": 0,\n          "diameterM": 0.22,\n          "region": "shirt",', '"directionDeg": 0,\n          "diameterM": 0.16,\n          "region": "shirt",'),
])

edit("person-cooking", [
    ('"de": "Leichte Stehtätigkeit, etwas tiefer"\n            },\n            "rateBpm": 16,\n            "tidalVolumeL": 0.5,', '"de": "Leichte Stehtätigkeit, etwas tiefer"\n            },\n            "rateBpm": 15,\n            "tidalVolumeL": 0.5,'),
    ('"directionDeg": -150,\n          "diameterM": 0.0095,', '"directionDeg": -150,\n          "diameterM": 0.015,'),
    ('"directionDeg": -105,\n          "diameterM": 0.0124,', '"directionDeg": -105,\n          "diameterM": 0.02,'),
    ('"directionDeg": 0,\n          "diameterM": 0.25,\n          "region": "hoodie",', '"directionDeg": 0,\n          "diameterM": 0.18,\n          "region": "hoodie",'),
])
