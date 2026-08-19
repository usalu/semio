#!/usr/bin/env python3
# 🩹 terra-hard-safe-await-fixer.py
#
# Corrected, byte-safe replacement for the earlier buggy terra-hard-unawaited-future-fixer.py
# (which used a whole-file UTF-8 BYTE offset as a Python STRING/codepoint index — wrong wherever
# multi-byte emoji preceded the target, which is constantly in this codebase).
#
# This version works PER LINE, which rustc's JSON diagnostics give directly (`line_end`, and the
# single-line `text[0].text` snapshot of that line's content AT CAPTURE TIME). For each diagnostic:
#   1. Read the file NOW, take the CURRENT line at `line_end`.
#   2. If it's byte-identical to the captured `text[0].text` -> untouched since capture; convert
#      `highlight_end` (a BYTE offset rustc reports WITHIN THE LINE) to a character offset by
#      encoding a prefix of the line and re-decoding, then splice ".await" in at that character
#      offset. This is exact because the conversion happens within a single line's own bytes, not
#      the whole file, and is verified against the captured text before touching anything.
#   3. If the current line differs from the captured text, it likely already carries a stray
#      ".await" dropped by the earlier buggy tool somewhere on this same line (edits never insert
#      newlines, so line numbers are stable even after that corruption). Recovers the original line
#      by diffing current vs. captured text and removing exactly the inserted ".await" run(s), then
#      proceeds as in case 2. Refuses (reports, does not guess) if the diff isn't cleanly explained
#      by whole ".await" insertions.
# Multiple diagnostics on the same line are applied insertion-point-descending within that line so
# earlier positions on the line stay valid.
import json
import sys
import argparse
from collections import defaultdict
from difflib import SequenceMatcher

AWAIT = ".await"


def strip_stray_await(current: str, captured: str):
    """Return (recovered_original, ok). ok is False if the diff isn't cleanly explained by
    whole '.await' insertions into `captured`."""
    if current == captured:
        return current, True
    sm = SequenceMatcher(None, captured, current, autojunk=False)
    recovered = []
    pos_in_captured = 0
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            recovered.append(current[j1:j2])
            pos_in_captured = i2
        elif tag == "insert":
            inserted = current[j1:j2]
            if inserted.replace(AWAIT, "") != "":
                return None, False
            # drop it — it's exactly one or more stray ".await" runs
        elif tag == "replace":
            return None, False
        elif tag == "delete":
            return None, False
    result = "".join(recovered)
    return (result, result == captured)


def char_offset_from_byte_col(line: str, byte_col: int) -> int:
    """rustc's column is a 1-indexed BYTE offset into the line's UTF-8 encoding. Convert to a
    0-indexed Python character offset by encoding a prefix and decoding it back."""
    encoded = line.encode("utf-8")
    prefix_bytes = encoded[: byte_col - 1]
    return len(prefix_bytes.decode("utf-8"))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("json_path")
    ap.add_argument("--scope", default="🔨️modules")
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    # file -> line_no(1-indexed) -> list of (highlight_end, captured_line_text)
    sites = defaultdict(lambda: defaultdict(list))
    with open(args.json_path, encoding="utf-8") as f:
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
            if inner.get("level") != "warning":
                continue
            text = inner.get("message", "")
            if "unused implementer of `Future` that must be used" not in text and "unused implementer of `dyn Future`" not in text:
                continue
            for span in inner.get("spans", []):
                if not span.get("is_primary"):
                    continue
                fname = span["file_name"]
                if args.scope not in fname:
                    continue
                if span["line_start"] != span["line_end"]:
                    print(f"SKIP multi-line span (needs hand review): {fname}:{span['line_start']}-{span['line_end']}")
                    continue
                line_no = span["line_end"]
                col_end = span["column_end"]
                line_texts = span.get("text", [])
                if not line_texts:
                    continue
                captured_line = line_texts[0]["text"]
                sites[fname][line_no].append((col_end, captured_line))

    total_ok = 0
    total_bad = 0
    for fname, by_line in sorted(sites.items()):
        with open(fname, encoding="utf-8") as f:
            lines = f.readlines()

        file_ok = 0
        file_bad = 0
        for line_no, entries in sorted(by_line.items()):
            idx = line_no - 1
            if idx >= len(lines):
                print(f"BAD {fname}:{line_no} out of range")
                file_bad += len(entries)
                continue
            current_line = lines[idx]
            # normalize: captured text has no trailing newline; current_line does
            current_no_nl = current_line[:-1] if current_line.endswith("\n") else current_line
            trailing_nl = current_line[len(current_no_nl):]

            # dedupe identical (col_end, captured) pairs on the same line
            seen = set()
            uniq_entries = []
            for col_end, captured in entries:
                key = (col_end, captured)
                if key not in seen:
                    seen.add(key)
                    uniq_entries.append(key)

            # all entries on one line should share the same captured baseline; recover once
            captured_candidates = {c for _, c in uniq_entries}
            if len(captured_candidates) != 1:
                print(f"BAD {fname}:{line_no} multiple distinct captured baselines: {captured_candidates}")
                file_bad += len(uniq_entries)
                continue
            captured = next(iter(captured_candidates))

            recovered, ok = strip_stray_await(current_no_nl, captured)
            if not ok:
                print(f"BAD {fname}:{line_no} diff not cleanly explained by stray '.await' — current={current_no_nl!r} captured={captured!r}")
                file_bad += len(uniq_entries)
                continue

            # insert at each col_end (descending) into `recovered`, which now equals `captured`
            col_ends = sorted({c for c, _ in uniq_entries}, reverse=True)
            out = recovered
            for col_end in col_ends:
                char_off = char_offset_from_byte_col(captured, col_end)
                if out[char_off - 6:char_off] == AWAIT:
                    continue  # already has it somehow
                out = out[:char_off] + AWAIT + out[char_off:]

            lines[idx] = out + trailing_nl
            file_ok += len(uniq_entries)

        total_ok += file_ok
        total_bad += file_bad
        print(f"{fname}: {file_ok} ok, {file_bad} bad (of {sum(len(v) for v in by_line.values())} raw sites on {len(by_line)} lines)")

        if args.apply and file_ok:
            with open(fname, "w", encoding="utf-8") as f:
                f.writelines(lines)

    print(f"\nTOTAL ok={total_ok} bad={total_bad}")
    if not args.apply:
        print("dry-run only; pass --apply to write")
    return 0


if __name__ == "__main__":
    sys.exit(main())
