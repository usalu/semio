#!/usr/bin/env python3
"""🧮️ SH2 composed dry run: copies the live tree's content of every path either prepared set touches into a scratch root
(`.🧬semio/🌐hub/s14-sh2-compose`, recreated), applies the 38-file space-home set there (`sh2-apply.py --write --root`) and then
dry-runs the B1 local-catalog set on top (`b1-apply.py --root`) — the window-3 landing order, measured on today's tree without
touching it. `--rebase` additionally re-bases B1 onto the composed content: base := composed file, stage := the clean three-way
merge (B1 then applies as plain `apply` after the 38-file set)."""
import json, os, shutil, subprocess, sys, tempfile

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
SH2 = os.path.dirname(HERE)
ROOT = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-compose")
B1_BASE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-b1-base")
B1_STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-b1-stage")


def paths(manifest):
    return [entry["path"] for entry in json.load(open(manifest, encoding="utf-8"))]


def run(*command):
    result = subprocess.run(command, capture_output=True, text=True)
    print(result.stdout.rstrip())
    if result.stderr.strip():
        print(result.stderr.rstrip())
    return result.returncode


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
    shutil.rmtree(ROOT, ignore_errors=True)
    union = sorted(set(paths(os.path.join(SH2, "payload/manifest.json"))) | set(paths(os.path.join(HERE, "payload/manifest.json"))))
    for relative in union:
        source = os.path.join(REPO, relative)
        if os.path.exists(source):
            os.makedirs(os.path.dirname(os.path.join(ROOT, relative)), exist_ok=True)
            shutil.copy2(source, os.path.join(ROOT, relative))
    print(f"== compose root {ROOT}: {len(union)} path(s)")
    print("== 38-file set --write")
    if run("python3", os.path.join(SH2, "sh2-apply.py"), "--write", "--root", ROOT) != 0:
        return 1
    print("== B1 dry run on the composed root")
    status = run("python3", os.path.join(HERE, "b1-apply.py"), "--root", ROOT)
    if "--rebase" not in sys.argv:
        return status
    rebased = 0
    for entry in json.load(open(os.path.join(HERE, "payload/manifest.json"), encoding="utf-8")):
        if entry["mode"] != "whole":
            continue
        relative = entry["path"]
        composed = open(os.path.join(ROOT, relative), encoding="utf-8").read()
        old = open(os.path.join(B1_BASE, relative), encoding="utf-8").read()
        new = open(os.path.join(B1_STAGE, relative), encoding="utf-8").read()
        if composed == old:
            continue
        merged, clean = merge(composed, old, new)
        if not clean:
            print(f"rebase refused (conflict): {relative}")
            return 1
        open(os.path.join(B1_BASE, relative), "w", encoding="utf-8").write(composed)
        open(os.path.join(B1_STAGE, relative), "w", encoding="utf-8").write(merged)
        rebased += 1
    print(f"== rebased {rebased} B1 file(s) onto the composed tree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
