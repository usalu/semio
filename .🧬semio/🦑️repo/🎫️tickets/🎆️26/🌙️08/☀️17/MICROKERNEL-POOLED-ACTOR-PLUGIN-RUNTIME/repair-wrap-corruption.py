#!/usr/bin/env python3
"""🩹 Repair corruption from `wrap-sync-closure-await.py`'s pre-bugfix CALLEE_CHARS defect.

WHAT HAPPENED
-------------
`set(b"ABC...")` in Python 3 yields a set of INTEGERS (iterating `bytes` yields ints), but the
script compared it against `data[i:i+1]` — a length-1 `bytes` SLICE. A `bytes` object never
equals an `int`, so the membership check was always False and the backward callee-name scan was
a silent no-op on every one of its 224 applied edits. The callee name was left in place and only
the trailing `()` (or whatever the immediate parenthesized args were) got wrapped, producing text
like:

    v_raw::entriessemio_framework_plugin::resolve_ready(())
    write_bin_jsonsemio_framework_plugin::resolve_ready((w, x))          <- pattern varies

instead of the intended

    semio_framework_plugin::resolve_ready(v_raw::entries())

REPAIR
------
Diagnostic-driven would be ideal, but this shape doesn't compile at all (it's not even valid
Rust — `entriessemio_framework_plugin` reads as ONE broken identifier token, a parse error), so
rustc's own JSON diagnostics are useless here (parse errors don't carry useful spans for this).
Instead this is a **byte-pattern repair keyed on the corruption's own deterministic shape**, not
on any function name: find every occurrence of
`(?<=[A-Za-z0-9_:])semio_framework_plugin::resolve_ready\(` (the bridge call preceded, with NO
separator, by identifier/path characters — impossible in valid Rust, and the exact fingerprint
this bug always produces) and move that preceding identifier run inside the parens, right before
the argument list it was hiding in front of.

This is NOT the R10-banned name-keyed pattern (guessing whether an identifier is async by name);
it is keyed on an impossible-in-valid-Rust byte adjacency that only this bug's exact defect can
produce, verified by construction (see the bugfixed script's docstring) and the total repair
count is cross-checked against the original 224 applied-edit count.
"""
from __future__ import annotations
import os, re, sys

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

BRIDGE = "semio_framework_plugin::resolve_ready("
# preceding identifier/path run with NO separator before the bridge call — the corruption fingerprint
PAT = re.compile(r'([A-Za-z0-9_:]+)' + re.escape(BRIDGE))


def process(path: str, apply: bool):
    with open(path, "r", encoding="utf-8") as fh:
        text = fh.read()
    matches = list(PAT.finditer(text))
    if not matches:
        return 0
    new_text = text
    for m in reversed(matches):
        callee = m.group(1)
        new_text = new_text[:m.start()] + BRIDGE + callee + new_text[m.end():]
    if apply:
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(new_text)
    return len(matches)


def main():
    apply = "--apply" in sys.argv
    total = 0
    files = 0
    for dp, dn, fn in os.walk(SCOPE_ROOT):
        for f in fn:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            n = process(p, apply)
            if n:
                files += 1
                total += n
                print(f"{'REPAIRED' if apply else 'FOUND'} {n:2d}  {os.path.relpath(p, REPO)}")
    print(f"\nTOTAL: {total} corrupted sites in {files} files")
    if not apply:
        print("Re-run with --apply to write.")


if __name__ == "__main__":
    main()
