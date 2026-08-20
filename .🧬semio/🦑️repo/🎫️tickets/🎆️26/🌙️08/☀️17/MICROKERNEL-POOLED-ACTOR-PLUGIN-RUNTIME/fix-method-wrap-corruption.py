#!/usr/bin/env python3
"""🩹 Repair `RECEIVER.semio_framework_plugin::resolve_ready(METHOD(args))` corruption.

WHY
---
A second defect in `wrap-sync-closure-await.py` (found via a coordinator-requested audit,
2026-08-20, after the first CALLEE_CHARS bug was already fixed and repaired): for a METHOD call
`receiver.method(args)`, the callee backward-scan correctly stops at `method`'s own name (it must
— free-function calls like `v_raw::entries()` need exactly that), but the tool never checked
whether it had landed after a `.` — i.e. inside a receiver chain — before wrapping. Result:
`receiver.` was left OUTSIDE the `resolve_ready(...)` wrap:

    cat.semio_framework_plugin::resolve_ready(level(cat.codes[row]))

instead of

    semio_framework_plugin::resolve_ready(cat.level(cat.codes[row]))

`RECEIVER.semio_framework_plugin` is not a valid expression position (a struct-literal-adjacent
"expected a pattern, found an expression" parse error, or similar), so this is a hard compile
blocker, confirmed 96 sites / 38 files.

REPAIR
------
Byte-pattern repair keyed on the corruption's own deterministic, impossible-in-valid-Rust
fingerprint: `<receiver-chain>.semio_framework_plugin::resolve_ready(`. Move the receiver chain
(everything from the nearest non-identifier/non-dot boundary up to the literal `.` right before
`semio_framework_plugin`) inside the parens, in front of the argument it was standing next to.
"""
from __future__ import annotations
import os, re, sys

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

BRIDGE = "semio_framework_plugin::resolve_ready("
# receiver chain: one or more `IDENT.` segments immediately preceding the bridge call
PAT = re.compile(r'((?:[A-Za-z0-9_]+\.)+)' + re.escape(BRIDGE))


def process(path: str, apply: bool):
    with open(path, "r", encoding="utf-8") as fh:
        text = fh.read()
    matches = list(PAT.finditer(text))
    if not matches:
        return 0
    new_text = text
    for m in reversed(matches):
        receiver = m.group(1)  # e.g. "cat." or "self.field."
        new_text = new_text[:m.start()] + BRIDGE + receiver + new_text[m.end():]
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
