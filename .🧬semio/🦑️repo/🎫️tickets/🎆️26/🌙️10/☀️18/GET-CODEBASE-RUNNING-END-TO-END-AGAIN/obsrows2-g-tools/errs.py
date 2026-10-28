#!/usr/bin/env python3
"""[obsrows2-g] streams a bun test log and prints, per failed test, the first error line and the next detail lines (usage: errs.py <log> [lines])"""
import re, sys
lines = int(sys.argv[2]) if len(sys.argv) > 2 else 4
pending = []
for raw in open(sys.argv[1], encoding="utf8", errors="replace"):
    line = raw.rstrip("\n")
    if line.startswith("[script]"): continue
    if re.match(r"^(error|\w*Error)\b", line) and not re.match(r"^\d+ \|", line):
        pending = [line[:260]]; continue
    if pending and len(pending) < lines + 1 and not line.startswith("(fail)") and line.strip(): pending.append(re.sub(r"/private/(?:tmp|var/folders/[^ ]*?)/sm/[a-z0-9-]+/", "", line)[:230])
    if line.startswith("(fail)"):
        print(line[:150]); print("   " + "\n   ".join(pending) if pending else "   (no error line)"); pending = []
