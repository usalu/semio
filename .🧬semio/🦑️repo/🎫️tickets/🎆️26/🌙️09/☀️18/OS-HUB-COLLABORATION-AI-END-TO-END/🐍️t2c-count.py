#!/usr/bin/env python3
"""📊️ Splits a `tsc --pretty false` capture into T2-owned (`🧰️framework/🛍️products/🦑️repo/**`) and foreign
diagnostics, printing the owned total, the per-file and per-code histograms and, with `--list`, every
owned diagnostic line.
"""

import collections
import pathlib
import re
import sys

OWNED = "🧰️framework/🛍️products/🦑️repo/"
LINE = re.compile(r"^(?P<path>[^\s(].*?)\((?P<row>\d+),(?P<col>\d+)\): error (?P<code>TS\d+): (?P<text>.*)$")


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: 🐍️t2c-count.py <capture.txt> [--list] [--file <substring>]", file=sys.stderr)
        return 2
    capture = pathlib.Path(sys.argv[1])
    want_list = "--list" in sys.argv
    needle = None
    if "--file" in sys.argv:
        needle = sys.argv[sys.argv.index("--file") + 1]
    owned, foreign = [], []
    for raw in capture.read_text(encoding="utf8").splitlines():
        match = LINE.match(raw)
        if match is None:
            continue
        (owned if match.group("path").startswith(OWNED) else foreign).append(match)
    print(f"total={len(owned) + len(foreign)} owned={len(owned)} foreign={len(foreign)}")
    files = collections.Counter(m.group("path") for m in owned)
    codes = collections.Counter(m.group("code") for m in owned)
    print("codes: " + " ".join(f"{code}={count}" for code, count in codes.most_common()))
    for path, count in files.most_common(40):
        print(f"{count:4d}  {path.removeprefix(OWNED)}")
    if want_list:
        print("---")
        for match in owned:
            if needle is not None and needle not in match.group("path"):
                continue
            print(
                f"{match.group('path').removeprefix(OWNED)}:{match.group('row')} "
                f"{match.group('code')} {match.group('text')}"
            )
    foreign_roots = collections.Counter(m.group("path").split("/")[0] for m in foreign)
    print("foreign: " + " ".join(f"{root}={count}" for root, count in foreign_roots.most_common()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
