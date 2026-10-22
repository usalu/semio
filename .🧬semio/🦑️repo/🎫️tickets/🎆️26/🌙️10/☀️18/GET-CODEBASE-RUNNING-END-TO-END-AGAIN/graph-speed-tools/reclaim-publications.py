#!/usr/bin/env python3
"""Frees the graph image headroom that the optional-cache quota never counts: the per-process producer publications `selections/<scope>/inference/p-<pid>/` of processes that no longer run (25-28 MB each). Cache only, recomputable; the quota refuses every optional publication once the image has less than 1 GiB free.

usage: reclaim-publications.py [--write] [--stale-scopes]   (dry run by default; --stale-scopes also removes the scope-level publication/receipt files of all but the 2 newest scopes)"""
import os, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
base = os.path.join(ROOT, ".cache", "b", "g", "m", "inference", "selections")
alive = {line.split()[0] for line in subprocess.run(["ps", "-axo", "pid"], capture_output=True, text=True).stdout.splitlines()[1:] if line.split()}
targets = []
scopes = sorted((d for d in os.listdir(base) if os.path.isdir(os.path.join(base, d))), key=lambda d: -os.path.getmtime(os.path.join(base, d)))
for scope in scopes:
    inference = os.path.join(base, scope, "inference")
    if not os.path.isdir(inference):
        continue
    for name in os.listdir(inference):
        if name.startswith("p-") and name[2:].isdigit() and name[2:] not in alive:
            targets.append(os.path.join(inference, name))
if "--stale-scopes" in sys.argv:
    for scope in scopes[2:]:
        inference = os.path.join(base, scope, "inference")
        for name in ("emoji-project-json.publication.json", "emoji-project-json.receipt.json"):
            if os.path.exists(os.path.join(inference, name)):
                targets.append(os.path.join(inference, name))


def size(path):
    if os.path.isfile(path):
        return os.path.getsize(path)
    return sum(os.path.getsize(os.path.join(d, f)) for d, _, fs in os.walk(path) for f in fs)


total = sum(size(t) for t in targets)
free = shutil.disk_usage(base).free
print("%d targets, %.0f MB; image free now %.0f MB, the optional quota needs 1 GiB = 1073.7 MB free plus the record size" % (len(targets), total / 1e6, free / 1e6))
if "--write" in sys.argv:
    for target in targets:
        shutil.rmtree(target) if os.path.isdir(target) else os.remove(target)
    print("removed; image free now %.0f MB" % (shutil.disk_usage(base).free / 1e6))
else:
    print("DRY RUN (use --write)")
