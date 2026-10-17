# 🧱️ Rebuilds the pre-carve tree of one artifact from the working tree, the carve's apply log and its --backup directory: usage precarve.py <artifact-dir> <apply-log> <backup-dir> <out-dir>
import sys, os, shutil, re
art, log, backup, out = sys.argv[1:5]
root = "/Users/ueli/Documents/semio"
dst = os.path.join(out, art)
shutil.rmtree(out, ignore_errors=True)
shutil.copytree(os.path.join(root, art), dst, symlinks=True, ignore=shutil.ignore_patterns("node_modules", "target", "dist"))
created = 0; restored = 0
for line in open(log, encoding="utf-8"):
    m = re.match(r"^\s+(\+|~|-) (.+)$", line.rstrip("\n"))
    if not m or not m.group(2).startswith(art): continue
    kind, path = m.group(1), m.group(2)
    target = os.path.join(out, path)
    if kind == "+":
        if os.path.exists(target): os.remove(target); created += 1
    else:
        src = os.path.join(backup, path)
        if os.path.exists(src): shutil.copy2(src, target); restored += 1
# remove directories the carve created and that are now empty
for d, dirs, files in os.walk(dst, topdown=False):
    if not dirs and not files: os.rmdir(d)
print("removed", created, "restored", restored)
