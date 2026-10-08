"""Collects distinct emoji-bearing path segments of the repo for the T-D width fixture.

Usage: python -I r2-td-fixture-segments.py <repo> <out.txt>
Picks up to three representatives per structure of the leading cluster (scalar count, plane, VS16, ZWJ, keycap, ASCII tail) so the fixture stays small while every
emoji family of the repo taxonomy is present; the oracle probe then attaches the expected widths.
"""
import subprocess
import sys

repo, out = sys.argv[1], sys.argv[2]
listing = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files"], cwd=repo, capture_output=True, encoding="utf-8", check=True).stdout.splitlines()
segments = set()
for path in listing:
    for part in path.split("/"):
        if any(ord(ch) > 0x7f for ch in part):
            segments.add(part)


def lead(part):
    cluster = part[:1]
    for ch in part[1:]:
        if ch in "️‍" or 0x1f3fb <= ord(ch) <= 0x1f3ff or 0xe0020 <= ord(ch) <= 0xe007f or ord(ch) == 0x20e3:
            cluster += ch
        else:
            break
    return cluster


picked = {}
for part in sorted(segments, key=lambda value: (len(value), value)):
    head = lead(part)
    key = (len(head), ord(head[0]) > 0xffff, "️" in head, "‍" in head, "⃣" in head, part[len(head):len(head) + 1].isascii())
    bucket = picked.setdefault(key, [])
    if len(bucket) < 3:
        bucket.append(part)
picked = {index: part for index, part in enumerate(part for bucket in picked.values() for part in bucket)}
with open(out, "w", encoding="utf-8", newline="\n") as handle:
    for part in sorted(picked.values()):
        handle.write(part + "\n")
print(len(segments), "segments,", len(picked), "representatives")
