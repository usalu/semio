#!/usr/bin/env python3
# 🩹 terra-hard-final-corruption-sweep.py
#
# Last-resort residue cleaner for leftover stray ".await" left by the original buggy
# terra-hard-unawaited-future-fixer.py run, on files (grammar.rs in particular) where the
# byte-offset-as-codepoint-index drift was severe enough that the insertion landed on a
# DIFFERENT line than the one its diagnostic named — so neither the exact-formula reversal nor
# the per-diagnostic-line safe fixer could find it (both are keyed to a specific known line).
#
# This scan is purely SYNTACTIC, not diagnostic-driven, because by this point there is no
# diagnostic naming these stray positions. Two unambiguous corruption shapes, neither of which
# is ever legal Rust:
#   (a) ".await" with a word character (letter/digit/underscore) IMMEDIATELY after it — a
#       legal ".await" is always followed by whitespace or an operator/punctuation
#       (`.`/`?`/`;`/`,`/`)`/newline/etc.), never directly by more identifier text.
#   (b) A line whose only non-whitespace content up to ".await" is that ".await" itself (i.e.
#       the line, trimmed, STARTS with ".await"), where the PREVIOUS non-blank line's trimmed
#       tail ends in `;`, `{`, or `}` — meaning the previous statement was already complete, so
#       ".await" has no receiver at all. (A legitimate multi-line chain's continuation line
#       "    .await.method()" always follows a PREVIOUS line ending in an open call like
#       "foo(args)" with NO trailing `;` — that shape is deliberately excluded here.)
# Reports every match with context; only removes the literal ".await" (6 chars) at each
# confirmed site, in descending order, and re-reads before writing.
import re
import sys

WORD_AFTER = re.compile(r"\.await[A-Za-z0-9_]")


def find_sites(text: str):
    lines = text.split("\n")
    sites = []  # (line_idx, col_in_line) 0-indexed both

    # shape (a): word char immediately after .await
    for i, line in enumerate(lines):
        for m in re.finditer(r"\.await(?=[A-Za-z0-9_])", line):
            sites.append((i, m.start()))

    # shape (b): line starts (after whitespace) with .await, previous non-blank line ends in ; { }
    for i, line in enumerate(lines):
        stripped = line.lstrip()
        if not stripped.startswith(".await"):
            continue
        col = len(line) - len(stripped)
        # confirm not already caught by shape (a) at the same spot
        if (i, col) in sites:
            continue
        j = i - 1
        while j >= 0 and lines[j].strip() == "":
            j -= 1
        if j < 0:
            continue
        prev_tail = lines[j].rstrip()
        if prev_tail.endswith(";") or prev_tail.endswith("{") or prev_tail.endswith("}"):
            sites.append((i, col))

    return lines, sorted(set(sites))


def main() -> int:
    apply = "--apply" in sys.argv
    paths = [a for a in sys.argv[1:] if not a.startswith("--")]

    for path in paths:
        with open(path, encoding="utf-8") as f:
            text = f.read()
        lines, sites = find_sites(text)
        if not sites:
            print(f"{path}: clean")
            continue
        print(f"{path}: {len(sites)} corrupted '.await' sites")
        for i, col in sites:
            print(f"  line {i+1}: {lines[i]!r}")

        if apply:
            # remove in descending (line, col) order so earlier positions on a line stay valid
            for i, col in sorted(sites, reverse=True):
                line = lines[i]
                assert line[col:col + 6] == ".await", f"sanity check failed at {path}:{i+1}:{col}"
                lines[i] = line[:col] + line[col + 6:]
            with open(path, "w", encoding="utf-8") as f:
                f.write("\n".join(lines))
            print(f"  removed {len(sites)} sites from {path}")

    if not apply:
        print("\ndry-run only; pass --apply to write")
    return 0


if __name__ == "__main__":
    sys.exit(main())
