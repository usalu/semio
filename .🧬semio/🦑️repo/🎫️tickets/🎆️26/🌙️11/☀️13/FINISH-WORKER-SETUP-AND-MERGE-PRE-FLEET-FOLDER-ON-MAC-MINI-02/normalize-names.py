"""🔤 Renames every name below a worktree that is not NFC to its NFC form (macOS tar extracts NFD), children before parents.

The fleet path policy shares NFC paths only; `bootstrap-settle` does this rename, the adopt path does not.
Usage: python3 -I normalize-names.py <worktree> [--apply]
"""
import os, sys, unicodedata

root, apply = sys.argv[1], "--apply" in sys.argv
SKIP = {".git", ".cache", "node_modules"}
renamed = 0
for directory, directories, files in os.walk(root, topdown=False):
    relative = os.path.relpath(directory, root)
    if relative != "." and relative.split(os.sep)[0] in SKIP:
        continue
    for name in directories + files:
        target = unicodedata.normalize("NFC", name)
        if target == name:
            continue
        renamed += 1
        print(os.path.join(relative, name))
        if apply:
            os.rename(os.path.join(directory, name), os.path.join(directory, target))
            if target not in os.listdir(directory):
                raise SystemExit(f"the volume kept the old form of {name!r} in {relative}")
print(f"{renamed} names {'renamed' if apply else 'would be renamed'}")
