"""🔁️ Lists every tracked JSON file holding an object with a repeated key: `python json-duplicate-keys.py <workspace>`."""
import json
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1])
files = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "*.json"], cwd=root, capture_output=True, check=True).stdout.decode("utf-8").split("\0")


def pairs(found, path):
    def hook(items):
        seen = set()
        for key, _ in items:
            if key in seen:
                found.append((path, key))
            seen.add(key)
        return dict(items)
    return hook


found = []
for name in filter(None, files):
    path = root / name
    try:
        json.loads(path.read_text(encoding="utf-8-sig"), object_pairs_hook=pairs(found, name))
    except (OSError, ValueError):
        pass
for path, key in found:
    print(f"{path}\t{key}")
print(f"{len(found)} duplicate key(s)")
