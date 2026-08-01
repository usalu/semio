from pathlib import Path
import re

base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")
puzzle = [x for x in base.iterdir() if "puzzle" in x.name][0]
d5 = [x for x in (puzzle / "🎛️app").iterdir() if "5d" in x.name][0]
p = d5 / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs"
t = p.read_text()

# Find impl Puzzle5dPlayApp block helpers still on &mut self
for m in re.finditer(r"fn (\w+)\(&mut self", t):
    line = t[: m.start()].count("\n") + 1
    print(f"before {line}:{m.group(0)}")

# Only change methods that are on Puzzle5dPlayApp (between impl Puzzle5dPlayApp and next top-level impl DocumentApp or similar)
# Safer: replace specific known helpers
names = [
    "drive_precompute",
    "apply_engine_brush_placement",
    "apply_board_brush_place",
    "apply_board_events_from_json",
]
n = 0
for name in names:
    pat = f"fn {name}(&mut self"
    if pat in t:
        t = t.replace(pat, f"fn {name}(&self")
        n += 1
        print("fixed", name)
print("count", n)

# any remaining &mut self in file for Puzzle5dPlayApp methods?
for m in re.finditer(r"fn (\w+)\(&mut self", t):
    line = t[: m.start()].count("\n") + 1
    print(f"remaining {line}:{t.splitlines()[line-1][:140]}")

p.write_text(t)
