"""📏 Check storyboard narration clauses against the 95 character caption budget."""

import re
import sys
from pathlib import Path

LIMIT = 95
sys.stdout.reconfigure(encoding="utf-8")
text = (Path(__file__).with_name("storyboard-extrema.md")).read_text(encoding="utf-8")
failed = 0
count = 0
for line in text.splitlines():
    cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
    if line.startswith("| `") and len(cells) >= 3:
        count += 1
        key = cells[0]
        for language, clause in (("en", cells[1]), ("de", cells[2])):
            size = len(re.sub(r"\{[^}]*\}", "000000", clause))
            if size > LIMIT:
                failed += 1
                print(f"[DEBUG] {key} {language} {size}: {clause}")
print(f"[DEBUG] {count} clauses, {failed} too long")
sys.exit(1 if failed else 0)
