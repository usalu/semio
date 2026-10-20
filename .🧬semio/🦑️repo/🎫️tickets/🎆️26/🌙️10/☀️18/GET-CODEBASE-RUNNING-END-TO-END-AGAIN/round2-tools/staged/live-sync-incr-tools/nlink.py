#!/usr/bin/env python3
"""Read-only single-link invariant over every tracked package and project manifest (the Source revision reader refuses nlink != 1).

usage: nlink.py [ROOT]   rc 0 all single-link, rc 1 offenders (first 20 named), rc 2 not a git work tree
Run it before and after every dependency sync, lock or apply; it needs git and python only, so it works on every platform.
"""
import os
import subprocess
import sys

NAMES = ("package.json", "\U0001F4CB️project.json")


def main():
    root = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
    run = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", *[":(glob)**/" + name for name in NAMES]], capture_output=True)
    if run.returncode != 0:
        print("not a git work tree: " + root)
        return 2
    count, offenders = 0, []
    for raw in run.stdout.split(b"\0"):
        if not raw:
            continue
        path = raw.decode("utf-8")
        try:
            links = os.lstat(os.path.join(root, path)).st_nlink
        except FileNotFoundError:
            continue
        count += 1
        if links != 1:
            offenders.append((path, links))
    print("%d tracked manifests, %d with nlink != 1" % (count, len(offenders)))
    for path, links in sorted(offenders)[:20]:
        print("  nlink=%d  %s" % (links, path))
    return 1 if offenders else 0


sys.exit(main())
