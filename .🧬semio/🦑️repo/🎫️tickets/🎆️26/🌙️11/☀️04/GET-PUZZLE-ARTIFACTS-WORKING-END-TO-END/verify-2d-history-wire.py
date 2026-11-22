"""🔬️ Compare the Puzzle 2D wire fixture with the declared tags and SQLite's UTF-8 bytes."""

import json
from pathlib import Path
import re
import sqlite3

root = Path(__file__).resolve().parents[7]
puzzle = root / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
wire = puzzle / "🚪️io/💾️binary/🧬️mutations"
protocol = (wire / "📡️.protocol.semio").read_text(encoding="utf-8")
records = [(kind, int(tag)) for kind, tag in re.findall(r"^record ([a-z0-9-]+) tag=(\d+)$", protocol, re.MULTILINE)]
source = (puzzle / "🧬️schema/🧬️mutations/🦀️.rs").read_text(encoding="utf-8")
variants = source.split("pub enum Puzzle2dMutation {", 1)[1].split("\n}", 1)[0]
names = re.findall(r"^    (\w+)\(\w+\),$", variants, re.MULTILINE)
assert records == [(re.sub(r"(?<!^)(?=[A-Z])", "-", name).lower(), index) for index, name in enumerate(names)]
assert records

fixture = json.loads((wire / "🧪️tests/🔬️wire-format-guard/delete-node.json").read_text(encoding="utf-8"))
assert dict(records)[fixture["kind"]] == fixture["tag"]
db = sqlite3.connect(":memory:")
try:
    actual, = db.execute(
        """
        SELECT printf('%02x%02x01%02x%s01000600',
                      :format, :tag, length(CAST(:id AS BLOB)), lower(hex(:id)))
        """,
        fixture,
    ).fetchone()
finally:
    db.close()
assert actual == fixture["hex"], (actual, fixture["hex"])
print(f"{len(records)} authored mutation tags match their native ordinals; SQLite reproduced {actual}")
