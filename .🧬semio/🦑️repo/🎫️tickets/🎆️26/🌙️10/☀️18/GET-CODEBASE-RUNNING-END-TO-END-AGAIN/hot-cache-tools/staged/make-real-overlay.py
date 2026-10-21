#!/usr/bin/env python3
"""🏗 make-real-overlay.py REPO OUT: a real copy (no symlinked sources) of the tracked files the authority core needs, so physically captured modules and the real launch checks run unchanged.
Copies the tracked files of the framework modules and of the repo product plus the tracked root files, and links node_modules; reads the repository, writes only OUT."""
import os, shutil, subprocess, sys
repo, out = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
if os.path.exists(out):
    shutil.rmtree(out)
os.makedirs(out)
subtrees = ["\U0001F9F0️framework/\U0001F528️modules", "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo"]
listed = subprocess.run(["git", "-C", repo, "ls-files", "-z", "--"] + subtrees, capture_output=True, check=True).stdout.split(b"\0")
root_files = subprocess.run(["git", "-C", repo, "ls-files", "-z", "--"], capture_output=True, check=True).stdout.split(b"\0")
files = [f.decode() for f in listed if f] + [f.decode() for f in root_files if f and b"/" not in f]
copied = 0
for relative in files:
    source = os.path.join(repo, relative)
    if not os.path.isfile(source) or os.path.islink(source):
        continue
    target = os.path.join(out, relative)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copy2(source, target)
    copied += 1
if os.path.isdir(os.path.join(repo, "node_modules")):
    os.symlink(os.path.join(repo, "node_modules"), os.path.join(out, "node_modules"))
for base, directories, _ in os.walk(os.path.join(repo, subtrees[0])):
    if "node_modules" in directories:
        relative = os.path.relpath(os.path.join(base, "node_modules"), repo)
        target = os.path.join(out, relative)
        if not os.path.exists(target):
            os.makedirs(os.path.dirname(target), exist_ok=True)
            os.symlink(os.path.join(repo, relative), target)
    directories[:] = [d for d in directories if d != "node_modules"]
print("copied %d tracked files" % copied)
