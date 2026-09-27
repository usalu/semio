#!/usr/bin/env python3
"""🧷️ SH1 window-3 patch set: space-home reachability (bindSpaceFile / importSpace / deleteVirtualFileSystemNode as retained,
event-sourced routes), the two stale space-home test reds, the en1993 mutation grammar and the root/space TS checks.

Dry run by default; `--write` applies; `--root <dir>` targets a scratch overlay instead of the repo tree; `--only a,b`
selects manifest ids. Each payload is a whole-file pair (`payload/<id>.old` = the tree content it was prepared against,
`payload/<id>.new` = the prepared content, listed in `payload/manifest.json` by `sh1-capture.py`). A file equal to `.old`
is replaced; a file equal to `.new` counts as applied; a file a peer changed since is merged three-way with
`git merge-file -p` (read-only, stdout) and applied only when the merge is conflict-free. Every target is re-read right
before it is written; tree writes keep a backup under `.🧬semio/🌐hub/s13-sh1-backup/<stamp>/`.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")
BACKUP = os.path.join(REPO, ".🧬semio/🌐hub/s13-sh1-backup")


def read(path):
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def merge(current, old, new):
    with tempfile.TemporaryDirectory() as scratch:
        names = []
        for label, text in (("current", current), ("old", old), ("new", new)):
            path = os.path.join(scratch, label)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(text)
            names.append(path)
        result = subprocess.run(["git", "merge-file", "-p", names[0], names[1], names[2]], capture_output=True, text=True)
    return (result.stdout, result.returncode == 0)


def plan(root, entry):
    target = os.path.join(root, entry["path"])
    old, new = read(os.path.join(PAYLOAD, f"{entry['id']}.old")), read(os.path.join(PAYLOAD, f"{entry['id']}.new"))
    current = read(target)
    if entry["mode"] == "create":
        if current is None:
            return "apply", target, new
        return ("applied", target, None) if current == new else ("conflict: file exists with other content", target, None)
    if entry["mode"] == "delete":
        if current is None:
            return "applied", target, None
        return ("apply", target, None) if current == old else ("conflict: file differs from the prepared base", target, None)
    if current is None:
        return "conflict: file missing", target, None
    if current == new:
        return "applied", target, None
    if current == old:
        return "apply", target, new
    merged, clean = merge(current, old, new)
    return ("apply (3-way merge over a peer's change)", target, merged) if clean else ("conflict: 3-way merge has conflicts", target, None)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--root", default=REPO)
    parser.add_argument("--only", default="")
    arguments = parser.parse_args()
    root = os.path.abspath(arguments.root)
    with open(os.path.join(PAYLOAD, "manifest.json"), encoding="utf-8") as handle:
        manifest = json.load(handle)
    selected = [entry for entry in manifest if not arguments.only or entry["id"] in arguments.only.split(",")]
    results = [(entry, *plan(root, entry)) for entry in selected]
    for entry, state, target, _ in results:
        print(f"{state:>44}  {entry['id']}  {entry['path']}")
    conflicts = [result for result in results if result[1].startswith("conflict")]
    pending = [result for result in results if result[1].startswith("apply")]
    print(f"files={len(results)} apply={len(pending)} applied={sum(1 for result in results if result[1] == 'applied')} conflicts={len(conflicts)} root={root}")
    if conflicts:
        return 1
    if not arguments.write:
        return 0
    stamp = time.strftime("%Y%m%d-%H%M%S")
    for entry, _, target, _ in pending:
        fresh = plan(root, entry)
        if not fresh[0].startswith("apply"):
            print(f"re-read changed {entry['path']}: {fresh[0]}")
            return 1
        if root == REPO and os.path.exists(target):
            backup = os.path.join(BACKUP, stamp, entry["path"])
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            shutil.copy2(target, backup)
        if entry["mode"] == "delete":
            os.remove(target)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf-8") as handle:
            handle.write(fresh[2])
    print("written" + (f", backups {os.path.join(BACKUP, stamp)}" if root == REPO else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
