#!/usr/bin/env python3
"""🧮 R9 de-asyncify pass — stdio's `entries()` composer-table accessors.

WHY
---
`insert-await.py`'s crate-wide fixpoint over `semio-s-plugin-stdio` aborts on E0728
(`await` outside `async`) because every `entries()` in stdio's `🚪️DerivedIoRegistry`
region (76 definitions, two uniform signatures: `-> &'static [ComposerEntry]` and
`-> &'static [&'static ComposerEntry]`) is `async fn entries()` calling
`ENTRIES.get_or_init(|| ...)`. `OnceLock::get_or_init` takes a SYNC `FnOnce`, so any
`.await` inside that closure is illegal — the classic R10 residue class 1
("`.await` inside a sync closure").

Verified before writing this script (see 📓️terra-stdio-await-report.md for the full
census): all 76 bodies are pure table construction — `vec![composer_entry_of::<T>(), ...]`
or a fold over child-module `entries()` — grepped for `std::fs`/`tokio`/`reqwest`/
`File::`/`TcpStream`/`sleep` across every body: **zero hits**. This is the R9 shape
("pure computation whose consumer is language-barred from being async") the ticket's
`terra-number-green` packet already established the precedent for.

WHAT IT DOES
------------
Strips exactly the `async ` keyword from the two known-uniform `entries()` signatures,
matched as a literal string (not a name-keyed call-site guess — this only ever touches
a `fn` DEFINITION whose full signature text is verified ahead of time, never a call
expression). Leaves every call site alone; the resulting `entries().await` /
`xxx::entries().await` fallout (E0277 "not a future") is cleaned up separately by the
ticket's own `remove-bad-await.py`, which is diagnostic-driven per R10.

Idempotent: a file with no matches is a no-op.
"""
from __future__ import annotations
import os, re, sys, json

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

SIGNATURES = [
    "pub async fn entries() -> &'static [ComposerEntry] {",
    "pub async fn entries() -> &'static [&'static ComposerEntry] {",
]

def main():
    apply = "--apply" in sys.argv
    report = []
    for dp, dn, fn in os.walk(SCOPE_ROOT):
        for f in fn:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            with open(p, "r", encoding="utf-8") as fh:
                lines = fh.readlines()
            hits = 0
            out = []
            for i, line in enumerate(lines):
                stripped = line.rstrip("\n")
                matched_sig = None
                for sig in SIGNATURES:
                    if stripped.strip() == sig.strip():
                        matched_sig = sig
                        break
                if matched_sig is None:
                    out.append(line)
                    continue
                indent = line[:len(line) - len(line.lstrip(" "))]
                prev = out[-1] if out else ""
                already_tagged = "🚫️async: E1" in prev
                if not already_tagged:
                    out.append(indent + "// 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9\n")
                new_line = line.replace("pub async fn entries()", "pub fn entries()")
                out.append(new_line)
                hits += 1
            new_text = "".join(out)
            if hits:
                report.append((os.path.relpath(p, REPO), hits))
                if apply:
                    with open(p, "w", encoding="utf-8") as fh:
                        fh.write(new_text)

    total = sum(h for _, h in report)
    print(f"{'APPLIED' if apply else 'DRY-RUN'}: {len(report)} files, {total} entries() defs de-asyncified")
    for rel, hits in report:
        print(f"  {hits}  {rel}")
    if not apply:
        print("\nRe-run with --apply to write.")

if __name__ == "__main__":
    main()
