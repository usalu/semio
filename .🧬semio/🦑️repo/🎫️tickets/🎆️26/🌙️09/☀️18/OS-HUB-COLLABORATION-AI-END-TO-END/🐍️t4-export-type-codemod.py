#!/usr/bin/env python3
"""🔤️ Span-keyed TS1205 codemod: insert `type ` before each re-exported type name tsc names.

Driven purely by tsc's own (line, col) spans, so it can never hit an unrelated identifier.
tsc columns are 1-based UTF-16 code units; this maps them back to Python code-point indices.
"""
import io, re, sys, collections

IDENT = re.compile(r"[A-Za-z_$][A-Za-z0-9_$]*")

def u16_col_to_index(line: str, col: int) -> int:
    """📐️ Converts a 1-based UTF-16 column into a 0-based Python string index."""
    want = col - 1
    units = 0
    for i, ch in enumerate(line):
        if units == want:
            return i
        units += 2 if ord(ch) > 0xFFFF else 1
    if units == want:
        return len(line)
    raise ValueError(f"column {col} does not land on a character boundary")

def main(capture: str, apply: bool) -> int:
    pat = re.compile(r"^(.*?)\((\d+),(\d+)\): error TS1205:")
    hits = collections.defaultdict(list)
    for raw in io.open(capture, encoding="utf-8"):
        m = pat.match(raw)
        if m:
            hits[m.group(1)].append((int(m.group(2)), int(m.group(3))))
    changed = 0
    for path, spans in sorted(hits.items()):
        try:
            text = io.open(path, encoding="utf-8").read()
        except FileNotFoundError:
            print(f"SKIP missing {path}")
            continue
        lines = text.split("\n")
        for ln, col in sorted(set(spans), reverse=True):
            line = lines[ln - 1]
            idx = u16_col_to_index(line, col)
            tail = line[idx:]
            if "export" not in line:
                print(f"GUARD {path}:{ln}:{col} -> line is not an export clause, skipped (peer drift?)")
                continue
            if not re.match(r"[A-Za-z_$]", tail):
                print(f"SKIP {path}:{ln}:{col} -> not an identifier start: {tail[:40]!r}")
                continue
            if tail.startswith("type "):
                continue
            lines[ln - 1] = line[:idx] + "type " + tail
            changed += 1
            if not apply:
                name = re.match(IDENT, tail).group(0)
                print(f"{path}:{ln}:{col} -> type {name}")
        if apply:
            io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    print(f"{'applied' if apply else 'would apply'} {changed} insertions across {len(hits)} files")
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1], "--apply" in sys.argv))
