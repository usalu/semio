#!/usr/bin/env python3
"""R8 session 12 one-off: typed UI retirement accrues a sub-page grant instead of stalling.

`PagedList::release_empty_page` frees a backing whole or not at all, so the typed retirement of a
`UiFixedList` under a grant narrower than its next backing answered `progressed: false` forever (25
ui-contract laws, puzzle3d B52), and the document ladder papered over it by RAISING the grant past the
caller's budget. The job crate decided this class once (`charge_payload_page`): accrue the grant until the
backing is paid, spend at most the grant per turn. The ledger lives in the one place a typed retirement
keeps state across turns — `UiTypedRetirementCursor` — as `paid: u32` (fits the cursor's padding: its size
is unchanged), threaded to every `UiTypedRetire::retire_typed` as `paid: &mut u32`.

Usage: python3 typed-retire-accrual.py <contract-root> [--apply]
"""
import os, re, sys

root = sys.argv[1]
apply = "--apply" in sys.argv
files = []
for base, _, names in os.walk(root):
    if "🧪️tests" in base:
        continue
    for name in names:
        if name == "🦀️.rs":
            files.append(os.path.join(base, name))

SIG = [
    ("value: &mut Option<UiValueRetirement>, bytes: usize, count: u8)", "value: &mut Option<UiValueRetirement>, paid: &mut u32, bytes: usize, count: u8)"),
    ("value: &mut Option<UiValueRetirement>, bytes: usize)", "value: &mut Option<UiValueRetirement>, paid: &mut u32, bytes: usize)"),
    ("_: &mut Option<UiValueRetirement>, _: usize)", "_: &mut Option<UiValueRetirement>, _: &mut u32, _: usize)"),
    ("_: &mut Option<UiValueRetirement>, maximum_bytes: usize)", "_: &mut Option<UiValueRetirement>, _: &mut u32, maximum_bytes: usize)"),
    ("_: &mut Option<UiValueRetirement>, bytes: usize)", "_: &mut Option<UiValueRetirement>, _: &mut u32, bytes: usize)"),
]
CALL = [
    (re.compile(r"\.retire_typed\((path|child_path), value, bytes\)"), r".retire_typed(\1, value, paid, bytes)"),
    (re.compile(r"field_step\(([^;]*?), path, value, bytes, (\d+)\)"), r"field_step(\1, path, value, paid, bytes, \2)"),
    (re.compile(r"let mut step = field\.retire_typed\(path, value, bytes\)\?;"), r"let mut step = field.retire_typed(path, value, paid, bytes)?;"),
]
total = 0
for path in files:
    text = open(path, encoding="utf8").read()
    if "retire_typed" not in text and "field_step(" not in text:
        continue
    out = text
    for old, new in SIG:
        out = out.replace(old, new)
    for pattern, repl in CALL:
        out = pattern.sub(repl, out)
    if out != text:
        n = sum(1 for a, b in zip(text.splitlines(), out.splitlines()) if a != b)
        total += n
        print(f"{n:4d} lines  {os.path.relpath(path, root)}")
        if apply:
            open(path, "w", encoding="utf8").write(out)
print("changed lines", total, "(applied)" if apply else "(dry run)")
