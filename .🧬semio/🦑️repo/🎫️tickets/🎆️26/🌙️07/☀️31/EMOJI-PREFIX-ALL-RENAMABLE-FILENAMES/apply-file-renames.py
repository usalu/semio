#!/usr/bin/env python3
"""Apply file-rename-map.json via os.rename (deepest old paths first)."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

MAP_PATH = os.path.join(HERE, "file-rename-map.json")


def main():
    with open(MAP_PATH, encoding="utf-8") as f:
        entries = json.load(f)
    entries.sort(key=lambda e: -e["old"].count("/"))
    errors = []
    for e in entries:
        old, new = e["old"], e["new"]
        if not os.path.isfile(old):
            errors.append(f"missing: {old}")
            continue
        if os.path.exists(new):
            errors.append(f"collision: {new}")
            continue
        os.rename(old, new)
    print(f"renamed {len(entries) - len(errors)} files, errors {len(errors)}")
    for err in errors[:30]:
        print(" ", err)
    if errors:
        sys.exit(1)


if __name__ == "__main__":
    main()
