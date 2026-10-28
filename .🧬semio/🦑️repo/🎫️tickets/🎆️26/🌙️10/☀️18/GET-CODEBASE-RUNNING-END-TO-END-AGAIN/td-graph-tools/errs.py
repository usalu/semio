"""🩺 [td-graph] prints each failing test of a bun test log with the error lines that precede it (no stack frames, no source excerpts). usage: errs.py <log> [max-lines-per-test]"""
import re, sys
log, cap = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 8
buf = []
for line in open(log, encoding="utf-8", errors="replace"):
    line = line.rstrip("\n")
    if line.startswith("(fail)") or line.startswith("(pass)"):
        if line.startswith("(fail)"):
            print("##", line[7:200])
            for b in buf[-cap:]:
                print("   ", b[:330])
        buf = []
    elif re.match(r"^\s*at |^\s*\d+ \||^\s*\^|^\s*$|^bun test|^🟦", line) is None:
        buf.append(line)
