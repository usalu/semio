"""🏷️ Gives every TS wire-envelope literal its `verb: null` (the envelope type now names the authoring verb)."""
import pathlib, re

R = pathlib.Path("/Users/ueli/Documents/semio")
FILES = [
    "🌎️hub/📦️packages/🦀️rust/📜️script.ts",
    "🌎️hub/🧪️tests/📈️document-growth/🟦️.ts",
    "🌎️hub/🧪️tests/🤖️agent-ceiling/🟦️.ts",
    "🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️testkit/📡️client-probe/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-archive-restore/🟦️.ts",
]
for rel in FILES:
    path = R / rel
    lines = path.read_text().split("\n")
    changed = 0
    for index, line in enumerate(lines):
        if "transaction: null" not in line or "verb: null" in line or (index + 1 < len(lines) and "verb: null" in lines[index + 1]):
            continue
        window = "\n".join(lines[max(0, index - 3):index + 1])
        if "timestamp:" not in window:
            continue
        inline = re.search(r"transaction: null(\s*\})", line)
        if inline:
            lines[index] = line.replace("transaction: null" + inline.group(1), "transaction: null, verb: null" + inline.group(1), 1)
        else:
            assert line.rstrip().endswith("transaction: null,"), (rel, index + 1, line)
            lines.insert(index + 1, line[: len(line) - len(line.lstrip())] + "verb: null,")
        changed += 1
    path.write_text("\n".join(lines))
    print(rel, changed)
