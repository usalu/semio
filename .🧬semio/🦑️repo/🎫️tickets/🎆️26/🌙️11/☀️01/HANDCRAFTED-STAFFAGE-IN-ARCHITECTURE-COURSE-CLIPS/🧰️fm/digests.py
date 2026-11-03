"""🔐 SHA-256 of every file below a directory, sorted: `python3 digests.py <dir> > list.txt`."""

import hashlib
import sys

sys.stdout.reconfigure(encoding="utf-8")
from pathlib import Path

root = Path(sys.argv[1])
for path in sorted(p for p in root.rglob("*") if p.is_file()):
    print(hashlib.sha256(path.read_bytes()).hexdigest(), path.relative_to(root).as_posix())
