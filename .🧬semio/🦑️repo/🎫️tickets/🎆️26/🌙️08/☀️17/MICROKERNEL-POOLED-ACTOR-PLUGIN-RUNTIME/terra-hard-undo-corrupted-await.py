#!/usr/bin/env python3
# 🩹 terra-hard-undo-corrupted-await.py
#
# Reverses the exact, deterministic (but semantically WRONG) corruption produced by
# terra-hard-unawaited-future-fixer.py's first buggy run: that tool took rustc's `byte_end`
# (a UTF-8 BYTE offset) from --message-format=json and used it directly as a Python STRING
# (Unicode codepoint) index. In a codebase this heavy with multi-byte emoji, byte offset !=
# codepoint offset, so ".await" landed at drifted positions — sometimes inside doc comments,
# sometimes splitting a word.
#
# This is NOT a guess-and-hope-for-the-best patch. The original insertion was:
#   edits = sorted(set(byte_end_values), reverse=True)     # per file
#   out = text
#   for pos in edits:
#       out = out[:pos] + ".await" + out[pos:]
# For the i-th value (0-indexed, DESCENDING order) among N unique values in a file, its FINAL
# character position in the fully-edited string is exactly:
#   final_pos(i) = pos_i + 6 * (N - 1 - i)
# (i earlier/larger insertions never shift it since they land after it; N-1-i later/smaller
# insertions each shift it right by len(".await")==6.) This script recomputes that formula from
# the SAME original JSON capture, verifies the 6 characters at each computed position really are
# ".await", and removes exactly those characters — nothing else.
import json
import sys
from collections import defaultdict

AWAIT = ".await"


def main() -> int:
    json_path = sys.argv[1]
    apply = "--apply" in sys.argv[2:]

    ends_by_file = defaultdict(set)
    with open(json_path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line.startswith("{"):
                continue
            try:
                msg = json.loads(line)
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
                if "🔨️modules" not in span["file_name"]:
                    continue
                ends_by_file[span["file_name"]].add(span["byte_end"])

    total_verified = 0
    total_mismatch = 0
    for fname, ends in sorted(ends_by_file.items()):
        pos_desc = sorted(ends, reverse=True)
        n = len(pos_desc)
        final_positions = [pos_desc[i] + 6 * (n - 1 - i) for i in range(n)]

        with open(fname, encoding="utf-8") as f:
            text = f.read()

        verified = []
        mismatched = []
        for fp in final_positions:
            if text[fp:fp + 6] == AWAIT:
                verified.append(fp)
            else:
                mismatched.append(fp)

        print(f"{fname}: {len(verified)} verified / {len(mismatched)} mismatched (of {n})")
        if mismatched:
            for fp in mismatched[:5]:
                print(f"    mismatch at {fp}: {text[max(0,fp-20):fp+20]!r}")

        total_verified += len(verified)
        total_mismatch += len(mismatched)

        if apply and verified:
            # remove descending so earlier positions stay valid
            out = text
            for fp in sorted(verified, reverse=True):
                out = out[:fp] + out[fp + 6:]
            with open(fname, "w", encoding="utf-8") as f:
                f.write(out)
            print(f"  removed {len(verified)} corrupted '.await' insertions from {fname}")

    print(f"\nTOTAL verified={total_verified} mismatched={total_mismatch}")
    if not apply:
        print("dry-run only; pass --apply to write")
    return 0


if __name__ == "__main__":
    sys.exit(main())
