"""📋 Condense a layout report: one line per beat and distinct finding, ignoring tile-under-chip noise."""
import re
import sys
from pathlib import Path

report = Path(sys.argv[1]).read_text(encoding="utf-8").split("\n## ")
ignore = tuple(sys.argv[2:])
for block in report[1:]:
    lines = block.split("\n")
    key, items = lines[0].replace("## ", ""), [line for line in lines[1:] if line.startswith("- ") and line != "- none"]
    seen = {}
    for item in items:
        match = re.match(r"- (overlap|object under|line through|off-frame|in caption zone|[a-z ]+) '([^']*)'", item)
        label = (match.group(1), match.group(2)) if match else (item[:60], "")
        if any(word in label[1] for word in ignore):
            continue
        seen[label] = seen.get(label, 0) + 1
    if seen:
        print(f"[{key}] " + "; ".join(f"{kind} '{text}'" + (f" x{n}" if n > 1 else "") for (kind, text), n in seen.items()))
