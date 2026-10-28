"""🐍 Independent model of the Nx graph backing rule over real files: `python3 -I 🐍️.py <directory>` prints {name: {"admitted": bool, "sparse": bool}} for every entry of the directory (lstat, never followed): admitted = one regular file with one link, sparse = fewer allocated bytes than its size."""
import json, os, stat, sys

directory = sys.argv[1]
rows = {}
for name in sorted(os.listdir(directory)):
    row = os.lstat(os.path.join(directory, name))
    blocks = getattr(row, "st_blocks", None)
    rows[name] = {"admitted": stat.S_ISREG(row.st_mode) and row.st_nlink == 1, "sparse": blocks is not None and blocks * 512 < row.st_size}
print(json.dumps(rows, sort_keys=True))
