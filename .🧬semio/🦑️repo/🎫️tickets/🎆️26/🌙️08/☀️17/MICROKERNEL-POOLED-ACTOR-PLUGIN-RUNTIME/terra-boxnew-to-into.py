#!/usr/bin/env python3
"""🔧 Diagnostic-driven (syntax-driven, not name-keyed) repair tool for the fem `dyn Element` packet.

`Box::new(X { .. })` (optionally followed by ` as Box<dyn Element>`) is a fully unambiguous syntactic
pattern — `Box::new` is a fixed std API name, not a first-party identifier that can collide with
anything else the way async fn names do (R10 is about NAME collisions with std method names; this
matches a literal, non-overloadable path). Every occurrence in this packet's scope constructs one of
the 13 concrete `Element` impls being folded into the `Elements` enum, so `Box::new(EXPR)` becomes
`EXPR.into()` (dropping any trailing `as Box<dyn Element>` cast) via a brace/paren/bracket-balanced
scan — never a regex over the inner expression, which could contain arbitrarily nested delimiters and
string/char literals.

Usage: python3 boxnew_to_into.py <file> [<file> ...]   — edits in place, prints one line per rewrite.
"""
import sys


def find_matching_paren(s: str, open_idx: int) -> int:
    """🔎 Given the index of an opening '(', returns the index of its matching ')', honoring nested
    (), {}, [], and skipping over string/char literal contents (including escapes)."""
    depth = 0
    i = open_idx
    n = len(s)
    while i < n:
        c = s[i]
        if c in "\"'":
            quote = c
            i += 1
            while i < n:
                if s[i] == "\\":
                    i += 2
                    continue
                if s[i] == quote:
                    i += 1
                    break
                i += 1
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError(f"unbalanced delimiters starting at {open_idx}")


def rewrite(text: str) -> tuple[str, int]:
    out = []
    i = 0
    n = len(text)
    needle = "Box::new("
    count = 0
    while True:
        idx = text.find(needle, i)
        if idx == -1:
            out.append(text[i:])
            break
        # 🚧 guard: only match a standalone call, not e.g. "SomeBox::new(" — check preceding char.
        if idx > 0 and (text[idx - 1].isalnum() or text[idx - 1] in "_:"):
            out.append(text[i : idx + len(needle)])
            i = idx + len(needle)
            continue
        out.append(text[i:idx])
        open_paren = idx + len("Box::new") - 0  # index of '(' -> idx + len("Box::new(") - 1
        open_paren = idx + len("Box::new")
        assert text[open_paren] == "("
        close_paren = find_matching_paren(text, open_paren)
        inner = text[open_paren + 1 : close_paren]
        rest_start = close_paren + 1
        # 🧹 drop an immediately-following ` as Box < dyn Element >` cast, tolerant of whitespace.
        j = rest_start
        cast_prefix = text[j:]
        stripped = cast_prefix.lstrip(" \t\n")
        leading_ws_len = len(cast_prefix) - len(stripped)
        if stripped.startswith("as Box<dyn Element>"):
            rest_start = j + leading_ws_len + len("as Box<dyn Element>")
        elif stripped.startswith("as Box < dyn Element >"):
            rest_start = j + leading_ws_len + len("as Box < dyn Element >")
        out.append(f"{inner.strip()}.into()")
        count += 1
        i = rest_start
    return "".join(out), count


def main(argv: list[str]) -> int:
    total = 0
    for path in argv:
        with open(path, encoding="utf-8") as fh:
            original = fh.read()
        new_text, count = rewrite(original)
        if count:
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(new_text)
        print(f"{path}: {count} rewrite(s)")
        total += count
    print(f"TOTAL: {total} rewrite(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
