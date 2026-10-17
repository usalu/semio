"""✂️ Second edit batch of slice ALIAS-SUBPATH-SWEEP (packages without subpaths, for one uniform alias style): every `@semio-tech/*` string alias becomes an exact-match regex.

Usage: python3 apply.py [--write]. Without --write it only checks that every anchor is unique. Each file is read fresh, every anchor must occur exactly once, a snapshot of the pre-edit text goes to `🗑️generated/ass-snap/`.
"""
import json
import os
import sys

R = "/Users/ueli/Documents/semio"
F = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SNAP = F + "/🗑️generated/ass-snap"
WRITE = "--write" in sys.argv


def rx(name):
    return "/^" + name.replace("/", "\\/") + "$/u"


def find_row(name):
    return (f'find: "{name}"', f"find: {rx(name)}")


def finds(*names):
    return [find_row(n) for n in names]


UI = ["@semio-tech/ui-react/test", "@semio-tech/ui-react/runtime"]
BUILDER_SET = UI + [
    "@semio-tech/ui-react",
    "@semio-tech/assets",
    "@semio-tech/ui-styling",
    "@semio-tech/canvas-react-renderer",
    "@semio-tech/infinite-world-r3f",
    "@semio-tech/framework-renderer-react",
    "@semio-tech/framework-os",
]


def single_object(name, expr):
    old = f'alias: {{\n      "{name}": {expr},\n    }},'
    new = f"alias: [{{ find: {rx(name)}, replacement: {expr} }}],"
    return [(old, new)]


EDITS = {
    "🧰️framework/🛍️products/🖥️server/🧪️tests/🎚️config/🟦️.ts": single_object("@semio-tech/framework-server", 'resolve(root, "🟦️.ts")'),
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🎚️config/🟦️.ts": single_object("@semio-tech/framework-os-shell", 'resolve(root, "🟦️.ts")'),
    "🧰️framework/🛍️products/❓️quiz/🧪️tests/🎚️config/🟦️.ts": single_object("@semio-tech/quiz", 'resolve(root, "🟦️.ts")'),
    "🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎚️config/🟦️.ts": single_object("@semio-tech/presentation", 'resolve(root, "🟦️.ts")'),
    "✏️s/🔌️plugins/🎞️animate/🧪️tests/🎚️config/🟦️.ts": finds("@semio-tech/presentation"),
}


def main():
    problems = 0
    plans = []
    for rel, rows in EDITS.items():
        path = R + "/" + rel
        if not os.path.isfile(path):
            print("MISSING", rel)
            problems += 1
            continue
        text = open(path, encoding="utf8").read()
        seen = set()
        for old, new in rows:
            if old in seen:
                print("DUPLICATE ROW", rel, old[:60])
                problems += 1
            seen.add(old)
            n = text.count(old)
            if n != 1:
                print(f"ANCHOR COUNT {n}:", rel[-60:], old[:80].replace("\n", "\\n"))
                problems += 1
        plans.append((rel, path, text, rows))
    if problems:
        print("problems:", problems)
        sys.exit(1)
    total = 0
    for rel, path, text, rows in plans:
        out = text
        for old, new in rows:
            out = out.replace(old, new)
            total += 1
        if WRITE:
            os.makedirs(SNAP, exist_ok=True)
            index = SNAP + "/index.json"
            table = json.load(open(index, encoding="utf8")) if os.path.exists(index) else {}
            if rel not in table:
                name = f"{len(table):03d}.txt"
                open(SNAP + "/" + name, "w", encoding="utf8").write(text)
                table[rel] = name
                json.dump(table, open(index, "w", encoding="utf8"), ensure_ascii=False, indent=1)
            fresh = open(path, encoding="utf8").read()
            assert fresh == text, "file changed between read and write: " + rel
            open(path, "w", encoding="utf8").write(out)
    print(("written" if WRITE else "checked"), len(plans), "files,", total, "rows")


main()
