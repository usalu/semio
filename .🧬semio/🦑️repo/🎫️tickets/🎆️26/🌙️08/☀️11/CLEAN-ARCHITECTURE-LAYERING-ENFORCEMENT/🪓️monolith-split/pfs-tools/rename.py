"""usage: rename.py <parent-dir> <old-folder> <new-folder> <file>...   moves <parent>/<old> to <parent>/<new> (refusing if the target exists) and replaces the folder name in the listed files (exactly, count printed)."""
import os, sys
parent, old, new, *files = sys.argv[1:]
src, dst = os.path.join(parent, old), os.path.join(parent, new)
assert os.path.isdir(src), src
assert not os.path.exists(dst), dst
os.rename(src, dst)
for f in files:
    s = open(f, encoding="utf-8").read()
    n = s.count(old)
    if n:
        open(f, "w", encoding="utf-8").write(s.replace(old, new))
    print(f"{n}x {os.path.basename(os.path.dirname(f))}/{os.path.basename(f)}")
