#!/usr/bin/env python3
"""🧮 R9 de-asyncify pass — stdio's top-level codec/computation helper functions.

WHY THIS EXISTS
----------------
Hypothesis test result (see 📓️terra-stdio-green-report.md): the dominant error shape in
`semio-s-plugin-stdio` (7,242 E0271 "expected X to return Y, but it returns future" · 5,240
E0277 · 4,001 E0308, concentrated in the per-format `🔺️diff`/`🧬️mutations` schema modules
under `🗿️artifacts/**`) is R9/E1 transitivity, NOT missing `.await`. The universal-async codemod
made every free-standing `enc_*`/`dec_*`/`encode_*`/`decode_*`/`read_bin_*`/`write_bin_*`/
`split_*`/`strip_*`/`parse_*`-shaped helper in these files `async fn`, even though:
  - they perform zero I/O (grepped whole-file for `std::fs`/`tokio`/`reqwest`/`ureq`/`File::`/
    `TcpStream`/`spawn`/`sleep`/`SystemTime` — every file this script edits had zero hits before
    editing, or it is skipped), and
  - they are consumed as bare fn-item VALUES by generic `Fn`-bound higher-order combinators
    (`encode_option(&d.uri, |v| enc_str(v))`, `decode_option_option(extensions, dec_json)`) and by
    `format!`/`Display` — both hard language barriers no `.await` can cross (R9 rule 2).

The higher-level trait-method impls in these SAME files (`DiffCodec::print_diff/parse_diff/
encode_diff/decode_diff`, `MutationDiff::apply/absorb`, `DiffAlgebra::inverse/between/is_empty`)
must stay `async` — their trait declarations
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`) are themselves `async fn`,
so R9 does not apply to them. This script never touches them: it only ever rewrites a `fn`
DECLARATION line with **zero leading whitespace** — every trait-impl method in this codebase is
indented inside its `impl … { }` block, so column-0 reliably selects free-standing module-level
functions and never a trait method. (Verified by direct inspection before writing this: the gltf
diff file has 235 column-0 `async fn` lines and 83 indented ones, and every one of the 83 sampled
sits inside an `impl … for …` block.)

WHAT IT DOES
------------
1. Whole-file I/O gate: skips any file containing an I/O marker. Only edits files that gate clean.
2. Line-level, column-0-only match of `(pub(crate) )?(pub )?async fn NAME(` — a syntactic anchor,
   not a name list (R10-safe: it never guesses from an identifier, it strips exactly the literal
   token `async ` immediately preceding `fn` on a line that already begins at column 0).
3. Inserts (once) a docstring tag on the preceding line, in the ticket's own convention.
4. Never touches a call site. Fallout (`callee(...).await` now "not a future") is cleaned up by
   the ticket's own diagnostic-driven `remove-bad-await.py`, exactly as the `entries()` R9 pass
   (`terra-stdio-entries-r9.py`) and `terra-number-green` did before it.

SELF-CORRECTING SAFETY NET
---------------------------
A column-0 free function that itself `.await`s a genuinely-async call (rare — none observed in the
sampled files, but not proven absent everywhere) will, after this script strips its `async`, throw
**E0728 "await is only allowed inside async functions"** — a loud, self-revealing compile error,
never silent corruption (same property `wrap-sync-closure-await.py`'s bugs did NOT have, which is
why those needed hand repair). Any such function must be manually reverted (see the report's
"E0728 audit" section) — this script does not auto-revert, it only forward-applies.

USAGE
-----
    python3 terra-green-codec-r9.py --dry-run
    python3 terra-green-codec-r9.py --apply
    python3 terra-green-codec-r9.py --apply --root '🗿️artifacts/🧊️gltf'   # restrict to one subtree
"""
from __future__ import annotations
import argparse, os, re, sys

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

IO_MARKERS = [
    "std::fs", "tokio", "reqwest", "ureq", "File::", "TcpStream",
    "spawn", "sleep", "SystemTime",
]

TOPLEVEL_ASYNC_FN = re.compile(r"^((pub(\(crate\))?)\s+)?async fn ([a-zA-Z_][a-zA-Z0-9_]*)\(")

# 🧮 round-1.1: the simple regex above requires `NAME(` with nothing between the name and the
# opening paren, so it silently skipped every GENERIC free function — `async fn foo<T>(...)`,
# `async fn bar<'a>(...)`, `async fn deserialize<'de, D: Deserializer<'de>>(...)`. Found via a
# post-round-2 residue audit: 517 such column-0 declarations remained un-reverted crate-wide,
# including several serde `deserialize_with` helpers (textbook E1 — serde calls them synchronously
# by path) and several pure combinator callbacks passed to Fn-bound generics (the exact R9 shape
# this tool exists for). GENERIC_ASYNC_FN_PREFIX matches the fixed part; the optional `<...>` list
# is then scanned by hand with bracket-depth counting (regex `[^>]*` breaks on the one nested case,
# `T: Iterator<Item = f64>`) — refuses (does not guess) if the list isn't closed on the same line.
GENERIC_ASYNC_FN_PREFIX = re.compile(r"^((pub(\(crate\))?)\s+)?async fn ([a-zA-Z_][a-zA-Z0-9_]*)")

TAG = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"


def has_io_marker(text: str) -> str | None:
    for m in IO_MARKERS:
        if m in text:
            return m
    return None


def preceded_by_test_attr(prior_raw_lines: list[str]) -> bool:
    """🧪 round-1.1 fix, ported verbatim from terra-green-codec-r9-inherent.py: this tool's FIRST
    run (before this check existed) wrongly stripped `async` from 20
    `#[semio_framework_async_macros::async_test]`-attributed test fns, because test files in this
    crate are not indented the way production code is, so column-0 alone was not a sufficient
    proxy for "not a trait/impl method". Found and hand-repaired once already (see
    📓️terra-stdio-green-report.md); this check exists so a SECOND run of this tool (e.g. after a
    regex fix that widens what it matches) cannot reintroduce the same 20 sites. Walks backward
    over attribute/doc-comment/blank lines and refuses if any attribute contains `test` OTHER than
    `cfg(test)` itself (a conditional-compilation attribute, not a test-harness entry point — fine
    to revert past)."""
    for line in reversed(prior_raw_lines):
        s = line.strip()
        if s == "":
            continue
        if s.startswith("///") or s.startswith("//!") or s.startswith("//"):
            continue
        if s.startswith("#["):
            if "test" in s.lower() and "cfg(test)" not in s and "cfg(not(test" not in s:
                return True
            continue
        break
    return False


def match_toplevel_async_fn(line: str) -> str | None:
    """🔎 Returns the fn name if `line` is `[pub[(crate)]] async fn NAME[<...>](` at column 0."""
    m = TOPLEVEL_ASYNC_FN.match(line)
    if m:
        return m.group(4)
    m = GENERIC_ASYNC_FN_PREFIX.match(line)
    if not m:
        return None
    rest = line[m.end():]
    if not rest.startswith("<"):
        return None  # plain regex already covers name-immediately-paren; this path is generics-only
    depth = 0
    close_idx = None
    for i, ch in enumerate(rest):
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
            if depth == 0:
                close_idx = i
                break
    if close_idx is None:
        return None  # unterminated on this line (multi-line generic list) — refuse, don't guess
    if rest[close_idx + 1:].startswith("("):
        return m.group(4)
    return None


def process_file(path: str, apply: bool) -> tuple[int, str | None, list[str]]:
    with open(path, "r", encoding="utf-8") as fh:
        text = fh.read()
    marker = has_io_marker(text)
    if marker:
        return 0, marker, []

    lines = text.split("\n")
    out = []
    hits = 0
    names = []
    for line in lines:
        name = match_toplevel_async_fn(line)
        if name is None:
            out.append(line)
            continue
        if preceded_by_test_attr(out):
            out.append(line)
            continue
        prev = out[-1] if out else ""
        already_tagged = "🚫️async:" in prev
        if not already_tagged:
            out.append(TAG.rstrip("\n"))
        new_line = line.replace("async fn ", "fn ", 1)
        out.append(new_line)
        hits += 1
        names.append(name)

    if hits and apply:
        with open(path, "w", encoding="utf-8") as fh:
            fh.write("\n".join(out))
    return hits, None, names


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--root", default="", help="restrict to a path fragment under stdio scope")
    args = ap.parse_args()
    apply = args.apply

    walk_root = SCOPE_ROOT
    if args.root:
        walk_root = os.path.join(SCOPE_ROOT, args.root)

    total_hits = 0
    total_files = 0
    skipped = []
    edited_report = []

    for dp, dn, fn in os.walk(walk_root):
        for f in fn:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            hits, marker, names = process_file(p, apply)
            if marker:
                skipped.append((os.path.relpath(p, REPO), marker))
                continue
            if hits:
                total_hits += hits
                total_files += 1
                edited_report.append((os.path.relpath(p, REPO), hits, names))

    mode = "APPLIED" if apply else "DRY-RUN"
    print(f"{mode}: {total_files} files, {total_hits} top-level async fns de-asyncified")
    print(f"Skipped (I/O marker present): {len(skipped)} files")
    for rel, marker in skipped:
        print(f"  SKIP [{marker}]  {rel}")
    print()
    for rel, hits, names in sorted(edited_report, key=lambda t: -t[1]):
        print(f"  {hits:4d}  {rel}")
    if not apply:
        print("\nRe-run with --apply to write.")


if __name__ == "__main__":
    main()
