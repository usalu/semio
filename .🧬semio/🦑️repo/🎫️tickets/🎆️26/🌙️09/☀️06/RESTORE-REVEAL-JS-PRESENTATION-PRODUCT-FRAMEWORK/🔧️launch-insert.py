import io, sys

ANCHOR = '      "command": "bun nx run @semio-tech/print-viz-kernel:test-exhaustive",\n      "cwd": "${workspaceFolder}"\n    },\n'

ENTRIES = [
    ("\U0001f9ea️test\U0001f3a4️presentation⚡️quick", "@semio-tech/presentation:test-quick"),
    ("\U0001f9ea️test\U0001f3a4️presentation\U0001f315️long", "@semio-tech/presentation:test-long"),
    ("\U0001f9ea️test\U0001f3a4️presentation\U0001f30c️exhaustive", "@semio-tech/presentation:test-exhaustive"),
    ("\U0001f9ea️test\U0001f3a4️presentation⚛️react⚡️quick", "@semio-tech/presentation-react:test-quick"),
    ("\U0001f9ea️test\U0001f3a4️presentation⚛️react\U0001f315️long", "@semio-tech/presentation-react:test-long"),
    ("\U0001f9ea️test\U0001f3a4️presentation⚛️react\U0001f30c️exhaustive", "@semio-tech/presentation-react:test-exhaustive"),
    ("\U0001f9ea️test\U0001f4fd️projektetage", "@semio-tech/mit-bestand-praesentation-projektetage:test"),
]

BLOCK = "".join(
    '    {\n'
    f'      "name": "{name}",\n'
    '      "type": "node-terminal",\n'
    '      "request": "launch",\n'
    f'      "command": "bun nx run {target}",\n'
    '      "cwd": "${workspaceFolder}"\n'
    '    },\n'
    for name, target in ENTRIES
)

for path in sys.argv[1:]:
    text = io.open(path, encoding="utf-8", newline="").read()
    if any(f'"name": "{name}"' in text for name, _ in ENTRIES):
        print(f"{path}: already present, skipped")
        continue
    count = text.count(ANCHOR)
    if count != 1:
        raise SystemExit(f"{path}: anchor found {count} times")
    text = text.replace(ANCHOR, ANCHOR + BLOCK)
    io.open(path, "w", encoding="utf-8", newline="").write(text)
    print(f"{path}: inserted {len(ENTRIES)} entries")
