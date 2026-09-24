"""WP-T3 helper: remove an io leaf (module declaration block + its 🦀️.rs/🟦️.ts files) from a crate root.

usage: python3 leaf-tool.py remove <crate-root-🦀️.rs> <leaf-dir-relative-to-root> [...more leaf dirs]
A leaf dir is e.g. 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any
"""
import os
import shutil
import sys


def remove_block(text: str, leaf: str) -> str:
    lines = text.split("\n")
    target = f'#[path = "{leaf}/🦀️.rs"]'
    hits = [i for i, line in enumerate(lines) if line.strip() == target]
    if len(hits) != 1:
        raise SystemExit(f"expected exactly one declaration of {leaf}, found {len(hits)}")
    at = hits[0]
    start, end = at - 6, at + 5
    head = lines[start + 1].strip()
    indent = lines[start + 1][: len(lines[start + 1]) - len(lines[start + 1].lstrip())]
    ok = lines[start].strip() == '#[path = "."]' and head.startswith("pub mod ") and lines[end] == indent + "}"
    ok = ok and lines[at - 1].strip() == "pub mod any {" and lines[at + 1].strip() == "mod component;"
    if not ok:
        raise SystemExit(f"unexpected block shape around {leaf}:\n" + "\n".join(lines[start : end + 1]))
    return "\n".join(lines[:start] + lines[end + 1 :])


def main() -> None:
    command, root, *leaves = sys.argv[1:]
    if command != "remove":
        raise SystemExit("only `remove` is supported")
    base = os.path.dirname(root)
    text = open(root, encoding="utf8").read()
    for leaf in leaves:
        text = remove_block(text, leaf)
        directory = os.path.join(base, leaf)
        shutil.rmtree(directory)
        parent = os.path.dirname(directory)
        while parent != base and not os.listdir(parent):
            os.rmdir(parent)
            parent = os.path.dirname(parent)
        print(f"removed {leaf}")
    open(root, "w", encoding="utf8").write(text)


main()
