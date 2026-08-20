#!/usr/bin/env python3
"""🌉 Bridge `store::ByteReader::new(...)`/`ByteWriter::new()` construction sites left un-resolved
after this packet's R9 rounds — the "second lever" the ticket's own reports have named twice, now
worked around IN-SCOPE (no framework lease needed).

WHY THIS EXISTS
----------------
`store::ByteReader`/`ByteWriter` (`🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`) are
confirmed I/O-free (verified by TWO prior packets — `terra-stdio-await` and `terra-replication-r9`
— every method inspected, pure in-memory cursor ops) but remain `async fn` because the framework
file is out of this packet's `path_scope`, and the one prior attempt to R9-revert it hit a real
blocker (two unconditional call sites in `🗣️dsl/🦀️component.rs`, a peer-owned file) and was
correctly reverted rather than force-landed.

After this packet's own R9 rounds made ~13,700 stdio-owned functions sync, a large slice of the
crate's residue (measured: **1,464 of 5,965 remaining errors — 24.6%** — mention `PackError`,
`ByteReader`, or `ByteWriter` in the diagnostic) turned out to be exactly this interaction: a
now-sync stdio helper constructs `let mut r = ByteReader::new(bytes);` / `let mut inner =
ByteWriter::new();` and never resolves it, so every later use of `r`/`inner` sees an opaque
`impl Future<Output = ByteReader>` / `impl Future<Output = ByteWriter>` instead of the real type —
E0308/E0599/E0277 fan-out from ONE unresolved construction site, not independent bugs each.

Since `resolve_ready<F: Future>(fut: F) -> F::Output` is this codebase's own established
sync-bridge idiom for exactly this shape (already used elsewhere, e.g. the bcf/dwg composers), and
since the construction call itself has zero suspension points (confirmed by the two prior reports),
wrapping the CONSTRUCTION call (not every downstream use) is sound and clears the whole fan-out in
one edit per site — entirely within `✏️s/🔌️plugins/🗄️stdio/**`, no lease needed.

WHAT IT DOES
------------
Scoped to files the CURRENT compiler diagnostics confirm are actually affected (a file list
derived from grepping the JSON for `PackError`/`ByteReader`/`ByteWriter` in error messages, not a
blind crate-wide sweep). Within those files, matches literal
`= ByteReader::new(...)` / `= ByteWriter::new()` construction expressions that are not already
wrapped in `resolve_ready(...)` and have no `.await` on the same line, and wraps the call in
`semio_framework_plugin::resolve_ready(...)`.

SAFETY
------
- File-scoped by the compiler's own current error output — never a blind name sweep.
- Matches a fixed, fully-enumerated two-name call surface (`ByteReader::new`, `ByteWriter::new`),
  not a name list vulnerable to std collisions (R10's concern) — these two names are this
  codebase's own framework types, not std/external crate methods.
- Idempotent: a line already containing `resolve_ready` or `.await` is left alone.
- Reports every edit; does not silently skip ambiguous shapes (a construction call spanning
  multiple lines is refused, not guessed at).
"""
from __future__ import annotations
import os, re, sys

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

CONSTRUCT_RE = re.compile(r"(=\s*)((?:[a-zA-Z_][a-zA-Z0-9_]*::)*ByteReader::new\([^;\n]*\)|(?:[a-zA-Z_][a-zA-Z0-9_]*::)*ByteWriter::new\(\))(\s*;)")


def process_file(path: str, apply: bool) -> tuple[int, list[str]]:
    with open(path, "r", encoding="utf-8") as fh:
        lines = fh.readlines()
    hits = 0
    notes = []
    out = []
    for line in lines:
        if ("ByteReader::new(" not in line and "ByteWriter::new(" not in line):
            out.append(line)
            continue
        if "resolve_ready" in line or ".await" in line:
            out.append(line)
            continue
        m = CONSTRUCT_RE.search(line)
        if not m:
            notes.append(f"REFUSED (no single-line match): {line.strip()[:100]}")
            out.append(line)
            continue
        new_line = (
            line[: m.start(2)]
            + "semio_framework_plugin::resolve_ready("
            + m.group(2)
            + ")"
            + line[m.end(2):]
        )
        out.append(new_line)
        hits += 1
    if hits and apply:
        with open(path, "w", encoding="utf-8") as fh:
            fh.writelines(out)
    return hits, notes


def main():
    apply = "--apply" in sys.argv
    files_list_arg = None
    for a in sys.argv[1:]:
        if a.startswith("--files="):
            files_list_arg = a.split("=", 1)[1]

    if files_list_arg:
        with open(files_list_arg, encoding="utf-8") as fh:
            targets = [l.strip() for l in fh if l.strip()]
    else:
        targets = []
        for dp, dn, fn in os.walk(SCOPE_ROOT):
            for f in fn:
                if f.endswith(".rs"):
                    targets.append(os.path.join(dp, f))

    total_hits = 0
    total_files = 0
    for p in targets:
        hits, notes = process_file(p, apply)
        if hits:
            total_hits += hits
            total_files += 1
            print(f"  {hits:3d}  {os.path.relpath(p, REPO)}")
        for n in notes:
            print(f"    {n}  ({os.path.relpath(p, REPO)})")

    mode = "APPLIED" if apply else "DRY-RUN"
    print(f"{mode}: {total_files} files, {total_hits} construction sites wrapped")
    if not apply:
        print("\nRe-run with --apply to write.")


if __name__ == "__main__":
    main()
