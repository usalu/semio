#!/usr/bin/env python3
"""🧬️ Span-keyed identifier rename driven by tsc's own (line, column) diagnostics.

Reads a `tsc --pretty false` capture on stdin, keeps the diagnostics whose message names `<from>`
as the offending property, and rewrites exactly that token at exactly that position. Columns are
UTF-16 code units, so they are mapped back to code-point indices before slicing — mandatory here,
because every path segment and many literals in this tree are astral emoji. Any position whose
token is not `<from>` is refused, so a stale capture can never corrupt a file.
"""
import io, re, sys, collections

DIAG = re.compile(r"^(.+?)\((\d+),(\d+)\): error (TS\d+): (.*)$")


def utf16_to_index(line: str, col: int) -> int:
    """🧭️ Maps a 1-based UTF-16 column onto a 0-based code-point index."""
    units = 0
    for index, char in enumerate(line):
        if units == col - 1:
            return index
        units += 2 if ord(char) > 0xFFFF else 1
    return len(line) if units == col - 1 else -1


def main(argv: list[str]) -> int:
    frm, to, apply = argv[1], argv[2], "--apply" in argv
    codes = {a for a in argv[3:] if a.startswith("TS")}
    hits = collections.defaultdict(list)
    for raw in sys.stdin:
        m = DIAG.match(raw.rstrip("\n"))
        if not m:
            continue
        path, line, col, code, message = m.group(1), int(m.group(2)), int(m.group(3)), m.group(4), m.group(5)
        if codes and code not in codes:
            continue
        if f"'{frm}'" not in message:
            continue
        hits[path].append((line, col))
    changed = refused = 0
    for path, positions in sorted(hits.items()):
        lines = io.open(path, encoding="utf-8").read().split("\n")
        for line, col in sorted(set(positions), reverse=True):
            text = lines[line - 1]
            index = utf16_to_index(text, col)
            if index < 0 or not text.startswith(frm, index) or (index + len(frm) < len(text) and (text[index + len(frm)].isalnum() or text[index + len(frm)] == "_")):
                print(f"REFUSED {path}({line},{col})")
                refused += 1
                continue
            lines[line - 1] = text[:index] + to + text[index + len(frm) :]
            changed += 1
        if apply:
            io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    print(f"{'applied' if apply else 'would apply'} {changed} renames, refused {refused}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
