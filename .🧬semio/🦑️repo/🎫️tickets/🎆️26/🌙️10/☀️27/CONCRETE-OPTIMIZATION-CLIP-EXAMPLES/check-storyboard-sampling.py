"""📏 Check narration clause lengths in storyboard-sampling.md."""
import sys
from pathlib import Path

path = Path(__file__).with_name("storyboard-sampling.md")
rows = [line.split("|") for line in path.read_text(encoding="utf-8").splitlines() if line.startswith("| ") and "`" in line.split("|")[2]]
total = 0
for cells in rows:
    cells = [c.strip() for c in cells]
    if len(cells) < 9 or not cells[1].isdigit():
        continue
    key, en, de = cells[2].strip("`"), cells[-3], cells[-2]
    total += 1
    flag = "  <-- TOO LONG" if len(en) > 95 or len(de) > 95 else ""
    print(f"{cells[1]:>2} {key:14} en {len(en):3} de {len(de):3}{flag}")
print("[DEBUG] clauses", total, file=sys.stderr)
