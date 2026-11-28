#!/usr/bin/env python3
"""Rebuilds the H100 stage for `repo land` from H100.patch (the diff of every staged file against the live tree of 2026-11-27 19:15).
usage (from the checkout root): restage.py <stage dir>
Then: repo land check <stage dir>   and, after the seal:   repo land --wave "Wave 1127" <stage dir>
A file that changed live since the patch was cut makes `patch` reject its hunks: fix the .rej by hand or land with --rebase."""
import os, re, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__)); stage = sys.argv[1]
patch = open(os.path.join(here, "H100.patch"), encoding="utf8").read()
modified = [m.group(1) for m in re.finditer(r"^--- a/(.+)$", patch, re.M)]
subprocess.run(["repo", "land", "--lane", "H100", "--title", "Self-repairing caches: verify on read, quarantine, durable writes, doctor repair", "checkout", stage, *modified], check=True)
subprocess.run(["patch", "-p1", "-d", os.path.join(stage, "tree"), "-i", os.path.join(here, "H100.patch")], check=True)
