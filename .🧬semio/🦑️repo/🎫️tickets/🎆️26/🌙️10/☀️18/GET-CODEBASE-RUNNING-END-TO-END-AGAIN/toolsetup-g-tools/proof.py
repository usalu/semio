#!/usr/bin/env python3
"""`proof.py <run.log>...`: streams each log (line by line, constant memory) and prints the plan-reuse verdict lines, the Bazel summary and the phase-profile rows of the biggest phases."""
import re, sys

KEEP = re.compile(r"plan reused|cannot be reused later|planning again|plan reuse check|^INFO: \d+ process|^INFO: Elapsed|^exit=|\[capacity\] admitted")
PROFILE = re.compile(r"^(\d+)\t(\d+)\t([a-z][a-z0-9.:/-]*)\t([\d.]+)\t([\d.]+)\t([\d.]+)")
for path in sys.argv[1:]:
    print("==", path.split("/")[-1])
    rows = []
    with open(path, encoding="utf8", errors="replace") as handle:
        for line in handle:
            line = line.rstrip("\n")
            if KEEP.search(line):
                print(re.sub(r"\(semio-source:[^)]*\)", "", line)[:400])
            m = PROFILE.match(line)
            if m:
                rows.append((float(m.group(5)), m.group(3), float(m.group(6))))
    for duration, phase, own in sorted(rows, reverse=True)[:14]:
        print("  %9.1f ms  self %9.1f  %s" % (duration, own, phase))
