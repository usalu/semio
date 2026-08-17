#!/usr/bin/env python3
"""B2: physically apply rename-map-v2.json (deepest-first, so a directory's own
path is still valid when its turn comes -- ancestors haven't moved yet, only
descendants have) then rename entry files (lib.rs/main.rs/bin.rs/index.ts/index.tsx
-> 📦️-prefixed) under the (now-renamed) scope roots."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(HERE, "rename-map-v2.json"), encoding="utf-8") as f:
    RENAMES = json.load(f)

NEW_ROOTS = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"]
EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}
ENTRY_RENAMES = {
    "lib.rs": "📦️lib.rs",
    "main.rs": "📦️main.rs",
    "bin.rs": "📦️bin.rs",
    "index.ts": "📦️index.ts",
    "index.tsx": "📦️index.tsx",
}


def apply_dir_renames():
    moved = 0
    errors = []
    for r in RENAMES:
        old, new = r["old"], r["new"]
        if not os.path.exists(old):
            errors.append(("MISSING_OLD", old, new))
            continue
        if os.path.exists(new):
            errors.append(("TARGET_EXISTS", old, new))
            continue
        os.makedirs(os.path.dirname(new) or ".", exist_ok=True)
        os.rename(old, new)
        moved += 1
    return moved, errors


def apply_entry_renames():
    moved = 0
    for root in NEW_ROOTS:
        if not os.path.isdir(root):
            continue
        for dirpath, dirs, files in os.walk(root):
            dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS and not d.startswith(".")]
            for fn in files:
                if fn in ENTRY_RENAMES:
                    old = os.path.join(dirpath, fn)
                    new = os.path.join(dirpath, ENTRY_RENAMES[fn])
                    if os.path.exists(new):
                        print("SKIP entry (target exists):", old)
                        continue
                    os.rename(old, new)
                    moved += 1
    return moved


def main():
    moved, errors = apply_dir_renames()
    print(f"directories moved: {moved}")
    print(f"directory errors: {len(errors)}")
    for e in errors[:50]:
        print(" ", e)
    entry_moved = apply_entry_renames()
    print(f"entry files moved: {entry_moved}")


if __name__ == "__main__":
    main()
