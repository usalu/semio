#!/usr/bin/env python3
"""Census of the progress events of one `repo run <target> --dry-run` log (plain or graphtime.ts-prefixed).

usage: profile-log.py LOG
Counts owner observations (one `build:selected-declaration` event each), those with targets or inputs (`build:selected-normalization`), the per-phase event counts and, when lines carry a millisecond prefix, the span of every phase.
"""
import collections
import json
import re
import sys

PREFIX = re.compile(r"^\s*(\d+) (\[script\].*)$")
EVENT = re.compile(r'\{"owner":"repository:build","command":"([^"]+)","stage":"([^"]+)"')
ACTIVITY = re.compile(r'"resource":"([^"]+)"')


def main(path):
    count, first, last = collections.Counter(), {}, {}
    timed = False
    order = []
    with open(path, encoding="utf-8", errors="replace") as handle:
        for index, raw in enumerate(handle):
            at = None
            match = PREFIX.match(raw)
            line = raw
            if match:
                at, line, timed = int(match.group(1)), match.group(2), True
            event = EVENT.search(line)
            if not event:
                continue
            name = event.group(1)
            count[name] += 1
            if name not in first:
                first[name], order = (at, index), order + [name]
            last[name] = (at, index)
    observations = count["build:selected-declaration"]
    targeted = count["build:selected-normalization"]
    print("owner observations (selected-declaration): %d" % observations)
    print("  with targets or inputs (selected-normalization): %d" % targeted)
    print("  base only (declaration without normalization): %d" % (observations - targeted))
    print("%-42s %8s %s" % ("phase", "events", "first..last (line%s)" % (" / ms" if timed else "")))
    for name in order:
        a, b = first[name], last[name]
        span = "%d..%d" % (a[1], b[1]) + ("  %d..%d ms (%d)" % (a[0], b[0], b[0] - a[0]) if timed else "")
        print("%-42s %8d %s" % (name, count[name], span))


main(sys.argv[1])
