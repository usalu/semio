#!/usr/bin/env python3
"""📊️ Compares two runset outputs file by file (pass/fail counts and failing names without timings): python3 -I cmp.py BASE AFTER... (a file only in AFTER is skipped)."""
import re, sys
def read(path):
    rows, current = {}, None
    for line in open(path, encoding="utf8"):
        line = re.sub(r" \[[0-9.]+m?s\]", "", line.rstrip("\n"))
        m = re.match(r"^(.*?):\s+(\d+ pass )?(\d+ fail )?", line)
        if "pass" in line and "fail" in line and line.split(":")[0].endswith((".ts", ".mjs")):
            current = line.split(":")[0]
            rows[current] = [line.split(":", 1)[1].strip(), []]
        elif current and line.startswith("(fail)"):
            rows[current][1].append(line)
    return rows
base = read(sys.argv[1])
bad = 0
for after_path in sys.argv[2:]:
    for name, (counts, fails) in read(after_path).items():
        if name not in base:
            print("NEW  ", name[-60:], counts); continue
        if base[name][0] != counts or sorted(base[name][1]) != sorted(fails):
            bad += 1
            print("DIFF ", name[-70:], "base:", base[name][0], "after:", counts)
            for f in sorted(set(fails) - set(base[name][1])): print("   +", f[:150])
print("differences:", bad)
