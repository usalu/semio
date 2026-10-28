#!/usr/bin/env python3
"""phases2.py <run.ts>...: one line per run with the hot-path phases in seconds (SEMIO_REPO_PROFILE tables), streaming, plus the plan-reuse lines."""
import re, sys
WANT = ["door.capacity", "source.capture", "boot.loaded", "dependencies.locks", "dependencies.ensure", "dependencies.verify", "dependencies.witness", "dependencies.recheck", "graph.load", "graph.pinned", "bazel.prepare", "bazel.modules", "bazel.publish", "bazel.reuse", "bazel.implementation", "bazel.planner", "bazel.info", "bazel.invoke"]
for path in sys.argv[1:]:
    acc = {}; total = 0.0; notes = []
    for line in open(path, encoding="utf8", errors="replace"):
        m = re.match(r"\s*[\d.]+ (\d+)\t(\d+)\t(\S+)\t([\d.]+)\t([\d.]+)\t([\d.]+)", line)
        if m and m.group(3) in WANT: acc[m.group(3)] = acc.get(m.group(3), 0.0) + float(m.group(5)) / 1000
        t = re.match(r"\s*[\d.]+ #total\t([\d.]+)", line)
        if t: total = max(total, float(t.group(1)) / 1000)
        if "plan reused" in line or "planning again" in line or "cannot be reused" in line or "Plan wiederverwendet" in line: notes.append(line.strip()[:200])
    print(path.split("/")[-1], " ".join("%s=%.1f" % (k, acc[k]) for k in WANT if k in acc), "total=%.1f" % total)
    for n in notes[:4]: print("   ", n)
