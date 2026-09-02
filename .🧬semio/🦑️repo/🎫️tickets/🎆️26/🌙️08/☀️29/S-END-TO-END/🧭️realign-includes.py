#!/usr/bin/env python3
"""🧭️ Repoints `include_str!`/`include_bytes!`/`#[path]` references at files the sweep has renamed.

Driven by git's own rename records (`git status --porcelain` `R  old -> new`) rather than by guessing
naming shapes, so it cannot invent a target: a reference is rewritten only when it currently resolves
to nothing AND git says that exact path was renamed to something that exists.

The taxonomy sweep moves files and updates their references in separate passes; in between, a crate
dies with `couldn't read …: No such file or directory`, which masks every real diagnostic behind it.
Idempotent.
"""
import io, os, re, subprocess

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)

renames = {}
porcelain = subprocess.run(["git", "status", "--porcelain", "-z"], capture_output=True).stdout.decode("utf-8")
fields = porcelain.split("\0")
i = 0
while i < len(fields):
    entry = fields[i]
    if not entry:
        i += 1
        continue
    status, path = entry[:2], entry[3:]
    if "R" in status and i + 1 < len(fields):
        renames[fields[i + 1]] = path   # porcelain -z emits NEW then OLD
        i += 2
    else:
        i += 1
print("git rename records: %d" % len(renames))

REF = re.compile(r'(include_str!\("|include_bytes!\("|#\[path = ")([^"]+)(")')
files = subprocess.run(
    ["grep", "-rlE", r'include_str!\("|include_bytes!\("|#\[path = "', "--include=*.rs", "🧰️framework", "✏️s"],
    capture_output=True).stdout.decode("utf-8").split("\n")

total, touched = 0, []
for path in [f for f in files if f and "/target/" not in f]:
    try:
        text = io.open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        continue
    base, fixes = os.path.dirname(path), 0

    def repair(match):
        global fixes
        rel = match.group(2)
        absolute = os.path.normpath(os.path.join(base, rel))
        if os.path.exists(absolute):
            return match.group(0)
        repo_rel = os.path.relpath(absolute, ROOT)
        moved = renames.get(repo_rel)
        if moved and os.path.exists(os.path.join(ROOT, moved)):
            fixes += 1
            return match.group(1) + os.path.relpath(os.path.join(ROOT, moved), base) + match.group(3)
        return match.group(0)

    repaired = REF.sub(repair, text)
    if fixes:
        io.open(path, "w", encoding="utf-8").write(repaired)
        total += fixes
        touched.append((path, fixes))

print("references repointed: %d across %d files" % (total, len(touched)))
for path, count in touched[:10]:
    print("  %3d  %s" % (count, path))
