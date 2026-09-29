"""🧾️ S18 §15: rewrites the `## Session 15` status table rows of `📓️wp-s18.md` by row number and appends log lines.
usage: python3 s18-15-report.py <rows.json> — {"rows": {"<#>": "<full table row>"}, "log": ["- …"]}"""
import json
import sys
from pathlib import Path

REPORT = Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-s18.md")
spec = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
text = REPORT.read_text(encoding="utf-8")
start = text.index("## Session 15\n")
end = text.index("## Session 14c\n")
section = text[start:end]
lines = section.split("\n")
table_end = max(index for index, line in enumerate(lines) if line.startswith("| "))
for number, row in spec.get("rows", {}).items():
    matches = [index for index, line in enumerate(lines) if line.startswith(f"| {number} |")]
    if matches:
        lines[matches[0]] = row
    else:
        lines.insert(table_end + 1, row)
        table_end += 1
section = "\n".join(lines)
if spec.get("log"):
    section = section.rstrip("\n") + "\n" + "\n".join(spec["log"]) + "\n\n"
REPORT.write_text(text[:start] + section + text[end:], encoding="utf-8")
print("ok")
