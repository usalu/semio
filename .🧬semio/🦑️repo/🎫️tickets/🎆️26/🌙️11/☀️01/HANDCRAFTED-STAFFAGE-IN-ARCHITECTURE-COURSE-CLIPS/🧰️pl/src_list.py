#!/usr/bin/env python3
"""🔎 List source-side (outside ticket) new and modified paths with their naming shape (read-only)."""
import collections
import os
import sys

GEN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "pl")
TICKET = ".🧬semio/"


def entries():
    raw = open(os.path.join(GEN, "status.bin"), "rb").read().decode("utf-8").split("\0")
    i = 0
    while i < len(raw) and raw[i]:
        xy, path = raw[i][:2], raw[i][3:]
        i += 1
        if xy[0] in "RC":
            i += 1
        yield xy, path


def main():
    ents = list(entries())
    src = [e for e in ents if not e[1].startswith(TICKET)]
    tk = [e for e in ents if e[1].startswith(TICKET)]
    print("source", len(src), dict(collections.Counter(e[0] for e in src)))
    print("ticket", len(tk), dict(collections.Counter(e[0] for e in tk)))
    shapes = collections.Counter()
    for xy, p in src:
        name = p.split("/")[-1]
        ext = name[name.find("."):] if "." in name else ""
        shapes[(xy, ext)] += 1
    for k, v in sorted(shapes.items(), key=lambda kv: -kv[1]):
        print("SHAPE", k, v)
    with open(os.path.join(GEN, "src-list.txt"), "w", encoding="utf-8") as fh:
        for xy, p in src:
            fh.write(xy + " " + p + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
