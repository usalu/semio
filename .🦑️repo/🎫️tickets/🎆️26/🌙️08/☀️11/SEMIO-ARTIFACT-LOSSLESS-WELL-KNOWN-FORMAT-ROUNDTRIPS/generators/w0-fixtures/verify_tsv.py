#!/usr/bin/env python3
"""Re-parse the TSV fixture byte-for-byte: split on \\n, split each line on
\\t, confirm consistent column count and byte-exact round-trip."""
import sys

path = sys.argv[1]
raw = open(path, "rb").read()
text = raw.decode("utf-8")

assert "\r" not in text, "unexpected CR -- fixture should be pure LF"
trailing_newline = text.endswith("\n")
body = text[:-1] if trailing_newline else text
lines = body.split("\n")

rows = [line.split("\t") for line in lines]
col_counts = {len(r) for r in rows}
print(f"Lines: {len(lines)}  (header + {len(lines)-1} data rows)")
print(f"Column counts seen: {col_counts}")
assert len(col_counts) == 1, f"inconsistent column counts across rows: {col_counts}"
ncols = col_counts.pop()
assert ncols == 5

for i, r in enumerate(rows):
    print(f"  row {i}: {r}")

# Byte-exact round-trip: rejoin with \t and \n and compare to original bytes
rejoined = ("\n".join("\t".join(r) for r in rows) + ("\n" if trailing_newline else "")).encode("utf-8")
assert rejoined == raw, "round-trip (split+rejoin) did not reproduce original bytes exactly"

print(f"\ntrailing_newline={trailing_newline}")
print("Byte-exact split/rejoin round-trip confirmed.")
print("\nALL TSV STRUCTURAL ASSERTIONS PASSED")
