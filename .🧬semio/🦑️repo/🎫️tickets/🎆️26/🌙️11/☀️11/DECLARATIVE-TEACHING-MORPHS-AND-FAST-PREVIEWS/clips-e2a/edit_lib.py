"""One-shot exact-string edit helper for the E2a ventilation migration (deleted when the batch closes).

An edit is (old, new) with exactly one match, or (anchor, old, new): the first match of `old` after the first `anchor` text.
"""

import sys
from pathlib import Path


def apply(path, edits):
    p = Path(path)
    text = p.read_bytes().decode("utf-8")
    crlf = "\r\n" in text
    if crlf:
        text = text.replace("\r\n", "\n")
    for edit in edits:
        if len(edit) == 3:
            anchor, old, new = edit
            at = text.find(anchor)
            if at < 0:
                print(f"ANCHOR MISSING: {anchor[:90]!r}")
                sys.exit(1)
            i = text.find(old, at)
            if i < 0:
                print(f"EDIT FAILED (no match after anchor): {old[:90]!r}")
                sys.exit(1)
            text = text[:i] + new + text[i + len(old):]
            continue
        old, new = edit
        n = text.count(old)
        if n != 1:
            print(f"EDIT FAILED ({n} matches): {old[:90]!r}")
            sys.exit(1)
        text = text.replace(old, new)
    if crlf:
        text = text.replace("\n", "\r\n")
    p.write_bytes(text.encode("utf-8"))
    print(f"ok {len(edits)} edits -> {p.name}")
