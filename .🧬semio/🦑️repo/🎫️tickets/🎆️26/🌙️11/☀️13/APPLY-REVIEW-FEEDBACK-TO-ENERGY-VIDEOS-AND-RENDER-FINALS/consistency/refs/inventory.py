import collections
import re
import sys
from pathlib import Path

ENERGY = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
PATTERN = re.compile(
    r"(?:DIN(?:[ /](?:EN|V|TS|SPEC))*(?: ISO)?|VDI|ISO|EN(?: ISO)?|IEST-RP-CC)\s?\d[\d.]*(?:-\d+)?(?: Beiblatt \d| Blatt \d)?(?:-\d)?(?::\d{4}(?:-\d{2})?)?"
    r"|GModG|GEG|IWU|Bohne|Willems|Maas|Pettenkofer|Bundesgesundheitsbl\.?|BAFA|dena|Quelle|Source:"
)
counter = collections.Counter()
where = collections.defaultdict(list)
for path in ENERGY.rglob("*"):
    parts = set(path.parts)
    if not path.is_file() or parts & {".venv", "__pycache__", "site-packages", ".ruff_cache", "node_modules"}:
        continue
    if path.suffix not in {".py", ".json", ".md"}:
        continue
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except Exception:
        continue
    for number, line in enumerate(lines, 1):
        for match in PATTERN.finditer(line):
            text = match.group(0)
            counter[text] += 1
            where[text].append(f"{path.relative_to(ENERGY)}:{number}")
only = sys.argv[1] if len(sys.argv) > 1 else None
for text, count in sorted(counter.items()):
    if only and only not in text:
        continue
    print(f"{count:4} {text!r}  {where[text][0]}")
