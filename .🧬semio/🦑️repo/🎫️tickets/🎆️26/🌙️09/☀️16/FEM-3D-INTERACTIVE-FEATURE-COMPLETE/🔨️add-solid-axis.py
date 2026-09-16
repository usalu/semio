#!/usr/bin/env python3
"""🧭️ One-shot sweep: every committed fem3d JSON data fixture gets the new `FemSolid.axis` field (`"z"`, the
former implicit extrusion axis). A fixture whose solid objects are spelled in STRUCT order (`id, name,
outline, holes, baseZ, …, materialId`) — the `committed_json_is_canonical` law compares `to_value`'s
declaration order with the file verbatim — gets `axis` after `materialId`; a fixture spelled in sorted
key order (`baseZ` first) gets it before `baseZ`. JSON *schema* files (with a `required` array) are
skipped and edited by hand. Idempotent."""
import re, subprocess
ROOT = "/Users/ueli/Documents/semio"
A3 = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
files = subprocess.run(["bash", "-lc", f"cd '{ROOT}' && grep -rl '\"baseZ\"' '{A3}' --include='*.json'"], capture_output=True, text=True).stdout.split("\n")
touched = 0
for rel in [f for f in files if f]:
    path = f"{ROOT}/{rel}"
    text = open(path, encoding="utf-8").read()
    if '"required"' in text:
        continue
    lines = [line for line in text.split("\n") if not re.match(r'^\s*"axis": "z",?$', line)]
    out = []
    changed = False
    for index, line in enumerate(lines):
        base = re.match(r'^(\s*)"baseZ":', line)
        if base:
            struct_order = index > 0 and '"holes"' in lines[index - 1] or (index > 0 and lines[index - 1].strip() == "],")
            if not struct_order:
                out.append(f'{base.group(1)}"axis": "z",')
                changed = True
            out.append(line)
            continue
        material = re.match(r'^(\s*)"materialId": "[^"]*"(,?)$', line)
        if material and index + 1 < len(lines) and lines[index + 1].strip().startswith("}"):
            previous = lines[index - 1]
            if '"meshSize"' in previous:
                out.append(f'{material.group(1)}"materialId": {line.split(":",1)[1].strip().rstrip(",")},')
                out.append(f'{material.group(1)}"axis": "z"')
                changed = True
                continue
        out.append(line)
    if changed:
        open(path, "w", encoding="utf-8").write("\n".join(out))
        touched += 1
print(f"touched {touched} files")
