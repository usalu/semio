#!/usr/bin/env python3
"""R24 bundle builder: compares a scratch overlay (`work`: the live files of the base snapshot plus the prepared packages R12 and R19 plus R24) with the base snapshot (`base`: the live files the overlay was started from) and writes R24-bundle.json: a unified diff per changed file (a file without a final newline is carried in full), every new file in full, and the sha256 of the baseline and of the result of each file.
usage: build-bundle.py <base-root> <work-root> <out.json>"""
import difflib, hashlib, json, os, sys

MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
ROOTS = [MODULES + "/🤝️shared-worktree", MODULES + "/📚️library/🏗️build"]
SKIP_DIRS = {"node_modules", ".git"}
SKIP_NAMES = {".DS_Store"}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def walk(root, top):
    for directory, names, files in os.walk(os.path.join(root, top)):
        names[:] = [name for name in names if name not in SKIP_DIRS]
        for file in files:
            if file in SKIP_NAMES:
                continue
            yield os.path.relpath(os.path.join(directory, file), root)


def main():
    base, work, out = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3]
    bundle = {"version": 1, "patches": {}, "full": {}, "new": {}, "baseline": {}, "result": {}}
    for top in ROOTS:
        for rel in sorted(set(walk(work, top))):
            new_path = os.path.join(work, rel)
            old_path = os.path.join(base, rel)
            data = open(new_path, "rb").read()
            if not os.path.exists(old_path):
                bundle["new"][rel] = data.decode("utf8")
                bundle["result"][rel] = sha(data)
                continue
            old = open(old_path, "rb").read()
            if old == data:
                continue
            if not old.endswith(b"\n") or not data.endswith(b"\n"):
                bundle["full"][rel] = data.decode("utf8")
                bundle["baseline"][rel] = sha(old)
                bundle["result"][rel] = sha(data)
                continue
            diff = "".join(difflib.unified_diff(old.decode("utf8").splitlines(True), data.decode("utf8").splitlines(True), "a/" + rel, "b/" + rel, n=3))
            bundle["patches"][rel] = diff
            bundle["baseline"][rel] = sha(old)
            bundle["result"][rel] = sha(data)
    json.dump(bundle, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("patches", len(bundle["patches"]), "full", len(bundle["full"]), "new", len(bundle["new"]))


main()
