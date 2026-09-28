#!/usr/bin/env python3
"""🧱️ SH2 space-home set staging helper: `add <path…>` records repo paths in `sh2-files.txt` and seeds the base (current tree content, or absent
for a new file) and the stage (a copy of the base) for each; `rebase` refreshes the base of every listed path from the tree
(stage untouched). Paths are repo-relative."""
import os, shutil, sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-base")
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-stage")
LIST = os.path.join(HERE, "sh2-files.txt")


def listed():
    return [line.rstrip("\n") for line in open(LIST, encoding="utf-8") if line.strip()] if os.path.exists(LIST) else []


def seed(path, stage_too):
    source = os.path.join(REPO, path)
    for root, copy in ((BASE, True), (STAGE, stage_too)):
        if not copy:
            continue
        target = os.path.join(root, path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if os.path.exists(source):
            shutil.copy2(source, target)
        elif os.path.exists(target) and root == BASE:
            os.remove(target)


def main():
    command, paths = sys.argv[1], sys.argv[2:]
    current = listed()
    if command == "add":
        for path in paths:
            if path not in current:
                current.append(path)
                seed(path, not os.path.exists(os.path.join(STAGE, path)))
        with open(LIST, "w", encoding="utf-8") as handle:
            handle.write("\n".join(current) + "\n")
    elif command == "rebase":
        for path in current:
            seed(path, False)
    print(f"{len(current)} path(s) listed")


if __name__ == "__main__":
    main()
