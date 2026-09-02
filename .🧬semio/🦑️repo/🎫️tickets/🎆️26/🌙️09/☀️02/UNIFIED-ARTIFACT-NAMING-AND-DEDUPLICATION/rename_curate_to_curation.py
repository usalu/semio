#!/usr/bin/env python3
import re
import sys

TOKEN_A = re.compile(r'Curate(?![a-z])')
TOKEN_B = re.compile(r'(?<![A-Za-z])CURATE(?![a-z])')
TOKEN_C = re.compile(r'(?<![A-Za-z])curate(?![a-z])')

LITERAL_PRE = [
    ("sourcingcuratecfg", "sourcingcurationcfg"),
    ("sourcingcurate.presence", "sourcingcuration.presence"),
    ("Kuratieren", "Kuratierung"),
]

def transform(text):
    for a, b in LITERAL_PRE:
        text = text.replace(a, b)
    text = TOKEN_A.sub("Curation", text)
    text = TOKEN_B.sub("CURATION", text)
    text = TOKEN_C.sub("curation", text)
    return text

def main(paths):
    changed = []
    for p in paths:
        p = p.rstrip("\n")
        if not p:
            continue
        try:
            with open(p, "r", encoding="utf-8") as f:
                orig = f.read()
        except (UnicodeDecodeError, IsADirectoryError, FileNotFoundError) as e:
            print(f"SKIP {p}: {e}", file=sys.stderr)
            continue
        new = transform(orig)
        if new != orig:
            with open(p, "w", encoding="utf-8") as f:
                f.write(new)
            changed.append(p)
    for p in changed:
        print(p)

if __name__ == "__main__":
    main([l for l in sys.stdin.read().split("\n")])
