#!/usr/bin/env python3
"""Byte-copies (read + write, never link or clone) the tracked and untracked sources of the course closure into a mirror OUTSIDE the repository, so staged edits can run against a real tree. usage: mkmirror-copy.py <mirror-dir> [--sync]"""
import os, subprocess, sys

root = "/Users/ueli/Documents/semio"
prefixes = ("🧰️framework/", "🎓️teaching/")
mirror = sys.argv[1]
sync = "--sync" in sys.argv

def git(*args):
    out = subprocess.run(["git", *args], cwd=root, capture_output=True, check=True).stdout
    return [p for p in out.decode("utf8").split("\0") if p]

tracked = git("ls-files", "-z", "--", *[p.rstrip("/") for p in prefixes])
untracked = git("ls-files", "-z", "--others", "--exclude-standard", "--", *[p.rstrip("/") for p in prefixes])
generated = git("ls-files", "-z", "--others", "--ignored", "--exclude-standard", "--", *[f":(glob){p}**/🤖️generated/**" for p in prefixes])
top = [p for p in git("ls-files", "-z") if "/" not in p]
count = 0
for rel in sorted(set(tracked + untracked + generated + top)):
    source = os.path.join(root, rel)
    if not os.path.isfile(source) or os.path.islink(source):
        continue
    target = os.path.join(mirror, rel)
    if sync and os.path.exists(target):
        continue
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(source, "rb") as reader, open(target, "wb") as writer:
        writer.write(reader.read())
    os.chmod(target, os.stat(source).st_mode & 0o777)
    count += 1
modules = os.path.join(mirror, "node_modules")
os.makedirs(modules, exist_ok=True)
live = os.path.join(root, "node_modules")
for name in os.listdir(live):
    link = os.path.join(modules, name)
    if name == "@semio-tech" or os.path.lexists(link):
        continue
    os.symlink(os.path.realpath(os.path.join(live, name)), link)
scope = os.path.join(modules, "@semio-tech")
os.makedirs(scope, exist_ok=True)
for name in os.listdir(os.path.join(live, "@semio-tech")):
    link = os.path.join(scope, name)
    if not os.path.lexists(link):
        target = os.readlink(os.path.join(live, "@semio-tech", name))
        os.symlink(target.replace(root, mirror, 1) if target.startswith(root) else target, link)
print("copied", count)
