#!/usr/bin/env python3
"""🔁️ SH2 space-home set re-base onto the live tree: for every listed path whose tree content moved away from the prepared base
(`s14-sh2-base`), the stage (`s14-sh2-stage`) becomes the clean three-way merge `git merge-file -p stage base tree` and the base
becomes the tree content; a conflicting merge stops without writing anything. Run `sh2-capture.py` afterwards."""
import os, subprocess, sys, tempfile

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-base")
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-stage")


def read(path):
    return open(path, encoding="utf-8").read() if os.path.exists(path) else None


def merge(current, old, new):
    with tempfile.TemporaryDirectory() as scratch:
        names = []
        for label, text in (("current", current), ("old", old), ("new", new)):
            path = os.path.join(scratch, label)
            open(path, "w", encoding="utf-8").write(text)
            names.append(path)
        result = subprocess.run(["git", "merge-file", "-p", *names], capture_output=True, text=True)
    return result.stdout, result.returncode == 0


def main():
    paths = [line.rstrip("\n") for line in open(os.path.join(HERE, "sh2-files.txt"), encoding="utf-8") if line.strip()]
    plans = []
    for relative in paths:
        tree, base, stage = read(os.path.join(REPO, relative)), read(os.path.join(BASE, relative)), read(os.path.join(STAGE, relative))
        if tree is None or base is None or stage is None or tree == base:
            continue
        merged, clean = merge(tree, base, stage)
        if not clean:
            print(f"conflict: {relative}")
            return 1
        plans.append((relative, tree, merged))
    for relative, tree, merged in plans:
        open(os.path.join(BASE, relative), "w", encoding="utf-8").write(tree)
        open(os.path.join(STAGE, relative), "w", encoding="utf-8").write(merged)
        print(f"rebased {relative}")
    print(f"rebased {len(plans)} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
