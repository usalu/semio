"""👆️ Writes the swipe sections printed by swipe_vectors.py into the layered overview fixture: a section that is missing is appended, one
that differs is replaced in place, one that is equal is left alone. Run: python insert_vectors.py"""
import json
import pathlib
import subprocess
import sys

here = pathlib.Path(__file__).parent
fixture = here.parents[6] / "🧰️framework" / "🔨️modules" / "🖱️ui" / "🧫️fixtures" / "🥞️layered-overview" / "🔣️.json"
printed = subprocess.run([sys.executable, str(here / "swipe_vectors.py")], capture_output=True, text=True, encoding="utf-8", check=True).stdout
blocks: dict[str, list[str]] = {}
for line in printed.splitlines():
    if line.startswith('  "'):
        key = line.split('"')[1]
        blocks[key] = [line]
    else:
        blocks[key].append(line)
for key, lines in blocks.items():
    lines[-1] = lines[-1].rstrip(",")
wanted = {key: json.loads("{" + "\n".join(lines) + "}")[key] for key, lines in blocks.items()}

lines = fixture.read_text(encoding="utf-8").splitlines()
current = json.loads("\n".join(lines))
changed = []
for key, vectors in wanted.items():
    if key in current and current[key] == vectors:
        continue
    changed.append(key)
    if key not in current:
        continue
    start = lines.index(f'  "{key}": [')
    end = next(index for index in range(start, len(lines)) if lines[index] in ("  ],", "  ]"))
    trailing = lines[end].endswith(",")
    lines[start : end + 1] = [line + ("," if index == len(blocks[key]) - 1 and trailing else "") for index, line in enumerate(blocks[key])]
missing = [key for key in changed if key not in current]
if missing:
    closing = lines.index("}", len(lines) - 2)
    lines[closing - 1] = lines[closing - 1] + ","
    appended = []
    for position, key in enumerate(missing):
        appended += [line + ("," if index == len(blocks[key]) - 1 and position < len(missing) - 1 else "") for index, line in enumerate(blocks[key])]
    lines[closing:closing] = appended
fixture.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
assert {key: json.loads(fixture.read_text(encoding="utf-8"))[key] for key in wanted} == wanted
print(f"[DEBUG] replaced or added {changed} in {fixture}")
