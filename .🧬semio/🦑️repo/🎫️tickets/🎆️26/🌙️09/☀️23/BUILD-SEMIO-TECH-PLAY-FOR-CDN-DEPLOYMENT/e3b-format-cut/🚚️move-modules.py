#!/usr/bin/env python3
"""🚚️ Moves child module directories under a new parent module and rewrites every path that names them.

Each move takes a `#[path = "<dir>/🦀️.rs"] pub mod <name>;` mount out of its old parent file, moves `<dir>` (a plain
filesystem move) below the new parent's directory, mounts it in the new parent file, and rewrites `crate::<old>` and
`<crate ident>::<old>` to the new module path in every Rust file under the roots. Nothing is re-exported.

usage: 🚚️move-modules.py <spec.json> [--write] <root>...
spec: {"ident": crate ident, "moves": [{"from": "a::b::c", "to": "a::b::x::c", "from_parent": file, "to_parent": file,
        "dir": taxonomy dir name}]}   (files are repo-relative)
"""
import json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
SEP = r"\s*::\s*"


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    write = "--write" in sys.argv
    roots = [a for a in sys.argv[2:] if not a.startswith("--")]
    ident = spec["ident"]
    rewrites = []
    for move in spec["moves"]:
        old, new = move["from"], move["to"]
        rewrites.append((re.compile(r"(?<![\w:])(crate|" + re.escape(ident) + r")" + SEP + re.escape(old).replace(r"::", SEP) + r"(?![\w])"), new))
    files = []
    for base in roots:
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, base)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
            files += [os.path.join(dirpath, f) for f in filenames if f.endswith(".rs")]
    changed = {}
    for path in files:
        try:
            raw = open(path, encoding="utf-8").read()
        except (FileNotFoundError, UnicodeDecodeError):
            continue
        text = raw
        for pattern, new in rewrites:
            text = pattern.sub(lambda m: f"{m.group(1)}::{new}", text)
        if text != raw:
            changed[path] = text
    for move in spec["moves"]:
        name = move["from"].split("::")[-1]
        old_parent = os.path.join(REPO, move["from_parent"])
        new_parent = os.path.join(REPO, move["to_parent"])
        mount = re.compile(r'(?m)^(?P<attrs>(?:[ \t]*#\[(?!path)[^\n]*\]\n)*)[ \t]*#\[path\s*=\s*"' + re.escape(move["dir"]) + r'/🦀️\.rs"\]\n[ \t]*pub mod ' + re.escape(name) + r";\n")
        old_text = changed.get(old_parent, open(old_parent, encoding="utf-8").read())
        m = mount.search(old_text)
        if not m:
            raise SystemExit(f"mount of {name} not found in {move['from_parent']}")
        changed[old_parent] = old_text[:m.start()] + old_text[m.end():]
        new_text = changed.get(new_parent, open(new_parent, encoding="utf-8").read())
        block = f'{m.group("attrs")}#[path = "{move["dir"]}/🦀️.rs"]\npub mod {name};\n'
        mounts = list(re.finditer(r'(?m)^[ \t]*#\[path\s*=\s*"[^"]+"\]\n[ \t]*pub mod [A-Za-z_][A-Za-z0-9_]*;\n', new_text))
        if mounts:
            at = mounts[-1].end()
        else:
            uses = list(re.finditer(r"(?m)^use [^\n]*;\n", new_text))
            doc = re.match(r"(?://![^\n]*\n)*", new_text)
            at = uses[-1].end() if uses else doc.end()
            block = "\n" + block
        changed[new_parent] = new_text[:at] + block + new_text[at:]
        print(f"move {move['from']} -> {move['to']}")
    for path in sorted(changed):
        print(f"rewrite {os.path.relpath(path, REPO)}")
    if not write:
        return
    for path, text in changed.items():
        open(path, "w", encoding="utf-8").write(text)
    for move in spec["moves"]:
        source = os.path.join(os.path.dirname(os.path.join(REPO, move["from_parent"])), move["dir"])
        target = os.path.join(os.path.dirname(os.path.join(REPO, move["to_parent"])), move["dir"])
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.move(source, target)


main()
