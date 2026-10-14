#!/usr/bin/env python3
"""🧱️ Extracts top-level inline `pub mod NAME { .. }` modules of one Rust file into taxonomy child files.

Each named module's body (dedented one level) becomes `<dir>/🦀️.rs` next to the file and the inline module becomes
`#[path = "<dir>/🦀️.rs"] pub mod NAME;`. A textual `include!("<test>")` inside the body is re-pointed from the new
file; with a `tests` map entry the included test file is moved to `<dir>/<new relative path>` first.

usage: 🧱️inline-extract.py <spec.json> plan|apply
spec: {"file": repo-rel file, "modules": {NAME: dir}, "tests": {NAME: "🧪️tests/🔬️unit/🦀️.rs"}}
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec_path, mode = sys.argv[1], sys.argv[2]
spec = json.load(open(spec_path, encoding="utf-8"))
path = os.path.join(REPO, spec["file"])
base = os.path.dirname(path)
src = open(path, encoding="utf-8").read()
lines = src.split("\n")
out, i, writes, moves = [], 0, {}, []
while i < len(lines):
    m = re.match(r"^pub mod (\w+) \{\s*$", lines[i])
    if not m or m.group(1) not in spec["modules"]:
        out.append(lines[i]); i += 1; continue
    name, directory = m.group(1), spec["modules"][m.group(1)]
    depth, j = 0, i
    while True:
        depth += lines[j].count("{") - lines[j].count("}")
        if depth == 0 and j > i:
            break
        j += 1
    body = [line[4:] if line.startswith("    ") else line.lstrip() if not line.strip() else line for line in lines[i + 1:j]]
    while body and not body[0].strip():
        body.pop(0)
    target = os.path.join(base, directory, "🦀️.rs")
    text = "\n".join(body).rstrip() + "\n"
    def repoint(inc):
        old = os.path.normpath(os.path.join(base, inc.group(1)))
        new = old
        if name in spec.get("tests", {}):
            new = os.path.join(base, directory, spec["tests"][name])
            moves.append((old, new))
        return f'include!("{os.path.relpath(new, os.path.dirname(target))}")'
    text = re.sub(r'include!\("([^"]+)"\)', repoint, text)
    writes[target] = text
    out.append(f'#[path = "{directory}/🦀️.rs"]')
    out.append(f"pub mod {name};")
    print(f"{name}: {j - i + 1} lines -> {os.path.relpath(target, REPO)}")
    i = j + 1
writes[path] = "\n".join(out)
if mode == "apply":
    for old, new in moves:
        os.makedirs(os.path.dirname(new), exist_ok=True)
        os.rename(old, new)
        try:
            os.removedirs(os.path.dirname(old))
        except OSError:
            pass
    for target, text in writes.items():
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(text)
