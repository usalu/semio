#!/usr/bin/env python3
# 🩹 terra-hard-diagnostic-remove-bad-await.py
#
# Final cleanup round: purely diagnostic-driven removal of stray ".await" tokens left by the
# original buggy fixer, for shapes the syntactic sweeps didn't anticipate (".await" glued right
# before an opening paren, right after "{", inside argument lists, after char/string literals,
# doubled ".await?.await", etc). Reads `cargo check --message-format=json`. For each qualifying
# error, uses the primary span's OWN column_start/column_end (rustc points these AT the "await"
# keyword itself, or at the "." for "expected expression" errors) to locate the exact ".await" to
# delete — never a heuristic scan of the line. Byte-safe: column is a UTF-8 byte offset within the
# line, converted to a Python character offset via encode/decode on just that line.
import json
import sys
from collections import defaultdict

AWAIT = ".await"
BAD_MESSAGES = (
    "expected expression, found `.`",
    "expected `,` following `match` arm",
    "is not a future",
    "attempted to take value of method",
    "the `?` operator can only be applied to values that implement `Try`",
)


def char_offset_from_byte_col(line: str, byte_col: int) -> int:
    encoded = line.encode("utf-8")
    prefix_bytes = encoded[: byte_col - 1]
    return len(prefix_bytes.decode("utf-8"))


def main() -> int:
    apply = "--apply" in sys.argv
    json_path = sys.argv[1]

    # file -> line_no -> set of byte column_start values naming the token to delete
    sites = defaultdict(lambda: defaultdict(set))
    with open(json_path, encoding="utf-8") as f:
        for raw in f:
            raw = raw.strip()
            if not raw.startswith("{"):
                continue
            try:
                msg = json.loads(raw)
            except json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            inner = msg.get("message", {})
            if inner.get("level") != "error":
                continue
            text = inner.get("message", "")
            if not any(b in text for b in BAD_MESSAGES):
                continue
            for span in inner.get("spans", []):
                if not span.get("is_primary"):
                    continue
                if "🔨️modules" not in span["file_name"]:
                    continue
                if span["line_start"] != span["line_end"]:
                    continue
                line_texts = span.get("text", [])
                if not line_texts:
                    continue
                captured_line = line_texts[0]["text"]
                # find where the ".await" run nearest this span's highlighted range sits, using
                # the CAPTURED line snapshot (byte-accurate at diagnostic time) to search, then
                # re-locate the same run in the current file by content, not raw offset.
                sites[span["file_name"]][span["line_end"]].add((span["column_start"], span["column_end"], captured_line))

    total = 0
    for fname, by_line in sorted(sites.items()):
        with open(fname, encoding="utf-8") as f:
            lines = f.readlines()

        removed_this_file = 0
        for line_no, entries in by_line.items():
            idx = line_no - 1
            if idx >= len(lines):
                continue
            raw_line = lines[idx]
            has_nl = raw_line.endswith("\n")
            line = raw_line[:-1] if has_nl else raw_line

            removal_offsets = []
            for col_s, col_e, captured_line in entries:
                # locate ".await" in the CAPTURED line around the flagged byte column, then map
                # that same textual run to the CURRENT line by finding the run whose surrounding
                # bytes match (current line may already have prior runs removed from earlier in
                # this same pass, so search fresh each time using string content, not offset math)
                char_col = char_offset_from_byte_col(captured_line, col_s)
                # the flagged token is "await" (5 chars) or the "." itself; in both cases the
                # ".await" run starts at char_col-1 (if pointing at 'a') or char_col (if at '.')
                for start_guess in (char_col - 1, char_col):
                    if 0 <= start_guess and captured_line[start_guess:start_guess + 6] == AWAIT:
                        removal_offsets.append(start_guess)
                        break

            if not removal_offsets:
                continue

            # captured_line (pre-corruption-cleanup baseline for THIS run) should still literally
            # appear in `line` today only if nothing on this line has been touched yet this round;
            # since multiple diagnostics on one line share the same captured baseline text, just
            # verify current line still contains a ".await" at each offset in the captured text's
            # own coordinate frame is unsafe if the line already diverged. Simplest safe move:
            # since `line` at this point in the loop already reflects prior file state (from disk,
            # untouched by earlier rounds' successful removals unless this round runs twice), and
            # the captured_line IS what rustc just saw (i.e. matches `line` before any edit in
            # THIS invocation), removing by captured-line offsets applied to `line` is valid as
            # long as `line == captured_line` at loop entry for this line.
            captured_line_for_check = next(iter(entries))[2]
            if line != captured_line_for_check:
                print(f"SKIP {fname}:{line_no} — current line diverges from captured snapshot, needs hand review")
                print(f"    current : {line!r}")
                print(f"    captured: {captured_line_for_check!r}")
                continue

            for pos in sorted(set(removal_offsets), reverse=True):
                assert line[pos:pos + 6] == AWAIT, f"{fname}:{line_no} sanity check failed at {pos}"
                line = line[:pos] + line[pos + 6:]
                removed_this_file += 1

            lines[idx] = line + ("\n" if has_nl else "")

        print(f"{fname}: removed {removed_this_file}")
        total += removed_this_file
        if apply and removed_this_file:
            with open(fname, "w", encoding="utf-8") as f:
                f.writelines(lines)

    print(f"TOTAL removed: {total}")
    if not apply:
        print("dry-run only; pass --apply to write")
    return 0


if __name__ == "__main__":
    sys.exit(main())
