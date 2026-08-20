#!/usr/bin/env python3
"""🧮 R9 de-asyncify pass, round 2 — INHERENT `impl` methods (e.g. `DwgBitReader`/`DwgBitWriter`
bit-cursor methods) in already-verified-I/O-free stdio files.

WHY THIS EXISTS
----------------
`terra-green-codec-r9.py` only touched column-0 (module-top-level, non-nested) `async fn`s — a
deliberately conservative first pass, so it never risked stripping `async` off a trait-impl method
whose trait DECLARES it async. After that pass + `remove-bad-await.py`/`terra-remove-e0728-await.py`
fixpoint, `semio-s-plugin-stdio` fell 18,758 → 17,402, and the dominant residue shape is now
"no method named `read_bl`/`write_bs`/... found for opaque type `impl Future<Output = DwgBitReader>`"
— the exact same R9 shape, one level deeper: bit-cursor readers/writers (`DwgBitReader::read_bl`,
`DwgBitWriter::write_bs`, etc.) are INHERENT methods (`impl DwgBitReader { async fn read_bl(&mut
self) -> ... }`, no `for` clause — nobody else's trait constrains their signature), pure in-memory
bit-twiddling over an already-loaded buffer, zero I/O, in files this ticket already grepped clean.

WHAT IT DOES
------------
Walks the SAME file set (I/O-marker-gated, see `terra-green-codec-r9.py`'s docstring for the exact
marker list and why file-level gating is the evidence bar this ticket's precedent uses). Within each
gated file, tracks brace nesting with a small per-line tokenizer that strips string/char literal
content and `//`/`/* */` comments before counting `{`/`}` (so a `format!("[{}]", …)` does not
desync the depth counter — the #1 risk in a naive brace counter, and exactly the kind of mistake a
prior packet's whole-file rewrite script made). For every enclosing-block stack frame it records
one of: `impl_inherent` (`impl Type {` — no `for`), `trait_impl` (`impl Trait for Type {`),
`trait_decl` (`trait Trait {`), or `other` (fn bodies, `mod`, `if`/`match`/closures, …).

An `async fn` line is eligible for reversion **only if no `trait_impl` or `trait_decl` frame
encloses it** — i.e. it is either module-top-level or inside a plain inherent `impl Type { }` block.
This is strictly more permissive than round 1's column-0-only rule, but never touches a method whose
signature is externally fixed by a trait declaration.

Same edit shape as round 1: rewrite `async fn` → `fn` on that one line, insert an R9 tag comment
above it (once), never touch a call site.

SELF-CORRECTING SAFETY NET
---------------------------
Same property as round 1: a wrongly-reverted trait method throws an immediate, function-named
compile error (signature mismatch against the trait) — loud, not silent. A brace-depth desync would
show up as either a wrong (over- or under-) reversion count relative to a manual `grep -c` sanity
check, or as parse errors (R18: parse errors self-reveal). Both are checked in this packet's report
before this tool's output is trusted.

USAGE
-----
    python3 terra-green-codec-r9-inherent.py --dry-run
    python3 terra-green-codec-r9-inherent.py --apply
"""
from __future__ import annotations
import argparse, os, re, sys

REPO = "/Users/ueli/Documents/semio"
SCOPE_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio")

IO_MARKERS = [
    "std::fs", "tokio", "reqwest", "ureq", "File::", "TcpStream",
    "spawn", "sleep", "SystemTime",
]

ASYNC_FN_RE = re.compile(r"^(\s*)((pub(\(crate\))?)\s+)?async fn ([a-zA-Z_][a-zA-Z0-9_]*)\(")
# 🧮 same round-1.1 generic-parameter gap (see terra-green-codec-r9.py), fixed here identically:
# `ASYNC_FN_RE` requires `NAME(` immediately, silently skipping `async fn foo<T>(...)`. Matched
# with a bracket-depth scan, not a regex, because one real site nests (`T: Iterator<Item = f64>`).
ASYNC_FN_PREFIX_RE = re.compile(r"^(\s*)((pub(\(crate\))?)\s+)?async fn ([a-zA-Z_][a-zA-Z0-9_]*)")
IMPL_RE = re.compile(r"^\s*(unsafe\s+)?impl(<[^>]*>)?\s+")
TRAIT_RE = re.compile(r"^\s*(pub(\(crate\))?\s+)?trait\s+")
MOD_RE = re.compile(r"^\s*(pub(\(crate\))?\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*$")


def match_async_fn(line: str):
    """🔎 Returns a (match, name) pair if `line` is `[indent][pub[(crate)]] async fn NAME[<...>](`.
    `match` exposes `.group(1)` (indent) for callers that need it, same as a plain `re.Match`."""
    m = ASYNC_FN_RE.match(line)
    if m:
        return m, m.group(5)
    m = ASYNC_FN_PREFIX_RE.match(line)
    if not m:
        return None, None
    rest = line[m.end():]
    if not rest.startswith("<"):
        return None, None
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
        return None, None
    if rest[close_idx + 1:].startswith("("):
        return m, m.group(5)
    return None, None

TAG = "// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9"


def has_io_marker(text: str) -> str | None:
    for m in IO_MARKERS:
        if m in text:
            return m
    return None


def strip_noise(line: str, in_block_comment: bool) -> tuple[str, bool]:
    """🧹 Replace string/char-literal and comment content with spaces so brace counting is safe."""
    out = []
    i = 0
    n = len(line)
    in_str = False
    in_char = False
    while i < n:
        c = line[i]
        if in_block_comment:
            if c == "*" and i + 1 < n and line[i + 1] == "/":
                in_block_comment = False
                out.append("  ")
                i += 2
                continue
            out.append(" ")
            i += 1
            continue
        if in_str:
            out.append(" ")
            if c == "\\" and i + 1 < n:
                out.append(" ")
                i += 2
                continue
            if c == '"':
                in_str = False
            i += 1
            continue
        if in_char:
            out.append(" ")
            if c == "\\" and i + 1 < n:
                out.append(" ")
                i += 2
                continue
            if c == "'":
                in_char = False
            i += 1
            continue
        if c == "/" and i + 1 < n and line[i + 1] == "/":
            out.append(" " * (n - i))
            break
        if c == "/" and i + 1 < n and line[i + 1] == "*":
            in_block_comment = True
            out.append("  ")
            i += 2
            continue
        if c == '"':
            in_str = True
            out.append(" ")
            i += 1
            continue
        if c == "'":
            # could be a char literal or a lifetime ('a) — heuristic: char literal is 'x' or '\x'
            # closing within a few chars. Lifetimes are followed by an identifier then NOT a quote.
            # Safe approximation: only enter char-literal mode if we see a closing quote within 4 chars.
            j = i + 1
            closed = False
            steps = 0
            while j < n and steps < 4:
                if line[j] == "\\":
                    j += 2
                    steps += 2
                    continue
                if line[j] == "'":
                    closed = True
                    break
                j += 1
                steps += 1
            if closed:
                in_char = True
                out.append(" ")
                i += 1
                continue
            else:
                out.append(c)
                i += 1
                continue
        out.append(c)
        i += 1
    return "".join(out), in_block_comment


def classify_opener(pre_brace_text_accum: str) -> str:
    t = pre_brace_text_accum
    if TRAIT_RE.match(t):
        return "trait_decl"
    if IMPL_RE.match(t):
        return "trait_impl" if re.search(r"\bfor\b", t) else "impl_inherent"
    m = MOD_RE.match(t)
    if m and "test" in m.group(3).lower():
        return "test_mod"
    return "other"


def preceded_by_test_attr(prior_raw_lines: list[str]) -> bool:
    """🧪 Walk backward over attribute/doc-comment/blank lines immediately above a fn and refuse to
    touch it if any carries `test` — covers `#[test]`, `#[async_test]`,
    `#[semio_framework_async_macros::async_test]`, `#[tokio::test]`, etc. Those macros almost always
    require their input to stay a literal `async fn`; R9 must never touch a test harness entry point
    (R4 clause 5's territory, not this tool's)."""
    for line in reversed(prior_raw_lines):
        s = line.strip()
        if s == "":
            continue
        if s.startswith("///") or s.startswith("//!") or s.startswith("//"):
            continue
        if s.startswith("#["):
            if "test" in s.lower():
                return True
            continue
        # any other non-attribute, non-comment, non-blank line ends the lookback window
        break
    return False


def process_file(path: str, apply: bool):
    with open(path, "r", encoding="utf-8") as fh:
        text = fh.read()
    marker = has_io_marker(text)
    if marker:
        return 0, marker, []

    lines = text.split("\n")
    stack: list[str] = []  # one entry per currently-open '{', classification
    pending_header = ""    # accumulates text since the last '{' or ';' for classification
    in_block_comment = False

    out_lines = []
    hits = 0
    names = []

    for raw_line in lines:
        clean, in_block_comment = strip_noise(raw_line, in_block_comment)

        m, fn_name = match_async_fn(raw_line)
        if m:
            blocked = any(k in ("trait_impl", "trait_decl", "test_mod") for k in stack)
            blocked = blocked or preceded_by_test_attr(out_lines)
            if not blocked:
                prev = out_lines[-1] if out_lines else ""
                if "🚫️async:" not in prev:
                    indent = m.group(1)
                    out_lines.append(indent + TAG)
                new_line = raw_line.replace("async fn ", "fn ", 1)
                out_lines.append(new_line)
                hits += 1
                names.append(fn_name)
                # still need to process braces on this (rewritten) line for nesting — same braces either way
                raw_line_for_braces = clean
            else:
                out_lines.append(raw_line)
                raw_line_for_braces = clean
        else:
            out_lines.append(raw_line)
            raw_line_for_braces = clean

        # walk characters of the cleaned line to update stack + pending_header
        idx = 0
        for ch in raw_line_for_braces:
            if ch == "{":
                kind = classify_opener(pending_header)
                stack.append(kind)
                pending_header = ""
            elif ch == "}":
                if stack:
                    stack.pop()
                pending_header = ""
            elif ch == ";":
                pending_header = ""
            else:
                pending_header += ch
        pending_header += "\n"

    new_text = "\n".join(out_lines)
    if hits and apply:
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(new_text)
    return hits, None, names


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--root", default="")
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
    print(f"{mode}: {total_files} files, {total_hits} inherent-impl async fns de-asyncified")
    print(f"Skipped (I/O marker present): {len(skipped)} files")
    print()
    for rel, hits, names in sorted(edited_report, key=lambda t: -t[1]):
        print(f"  {hits:4d}  {rel}")
    if not apply:
        print("\nRe-run with --apply to write.")


if __name__ == "__main__":
    main()
