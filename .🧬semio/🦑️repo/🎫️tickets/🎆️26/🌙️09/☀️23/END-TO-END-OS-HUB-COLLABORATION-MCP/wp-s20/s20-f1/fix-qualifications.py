"""🧹️ S20 F1: removes the `unused_qualifications` the fault changes introduced — only the warnings whose removed path
prefix qualifies a fault item (`FaultCode`, `FaultOrigin`, `FaultParameter`, `Terminology`, `Locale`, the fault
manifest functions). Reads a cargo log, applies each help suggestion by column, right to left per line; a line that no
longer matches the log is left alone. Usage: python3 fix-qualifications.py <root> <cargo-log>"""
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(sys.argv[1])
LOG = Path(sys.argv[2]).read_text().split("\n")
MINE = re.compile(r"^(?:semio_framework|super|crate|dsl|protocol)::(?:[A-Za-z_]\w*::)*(FaultCode|FaultOrigin|FaultParameter|FaultParameters|FaultDefinition|Terminology|Locale|validate_fault_definitions|fault_text_parameters|fault_text|framework_fault_catalog|app_fault|app_refusal|FAULT_[A-Z_]+)\b")
edits: dict[Path, list[tuple[int, int, str, str]]] = defaultdict(list)
for index, line in enumerate(LOG):
    if not line.startswith("warning: unnecessary qualification"):
        continue
    location = re.search(r"--> (.+):(\d+):(\d+)$", LOG[index + 1])
    if not location:
        continue
    path = (ROOT / "🧰️framework" / location.group(1).split("🧰️framework/", 1)[1]) if "🧰️framework/" in location.group(1) else None
    if path is None:
        continue
    path = Path(str(path).replace("/📦️packages/🦀️rust/../..", "")).resolve() if "/../" in str(path) else path
    number, column = int(location.group(2)), int(location.group(3))
    minus = next((LOG[j] for j in range(index, index + 16) if re.match(rf"^{number} - ", LOG[j])), None)
    plus = next((LOG[j] for j in range(index, index + 16) if re.match(rf"^{number} \+ ", LOG[j])), None)
    if minus is None or plus is None:
        continue
    before, after = minus[len(f"{number} - "):], plus[len(f"{number} + "):]
    start = column - 1
    removed_length = len(before) - len(after)
    removed = before[start:start + removed_length]
    if not MINE.match(before[start:]):
        continue
    edits[path].append((number, start, removed, before))
changed = 0
for path, rows in edits.items():
    lines = path.read_text().split("\n")
    for number, start, removed, before in sorted(rows, key=lambda row: (row[0], -row[1])):
        current = lines[number - 1]
        if current[start:start + len(removed)] != removed:
            continue
        lines[number - 1] = current[:start] + current[start + len(removed):]
        changed += 1
    path.write_text("\n".join(lines))
print(f"removed {changed} fault-item qualifications in {len(edits)} files")
