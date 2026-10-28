#!/usr/bin/env python3
"""Applies the staged change journal after the measurement window: copies `staged/changes` to the filesystem observation module and `staged/journal` to the resident member, and adds the new tests and type checks to the observation test script. Dry run by default; `--write` applies. Refuses to overwrite a file that differs from the staged one unless it is absent."""
import filecmp
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
OBSERVATION = os.path.join(REPO, "🧰️framework/🔨️modules/📁️filesystem/🧾️observation")
RESIDENT = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident")
COPIES = [(os.path.join(HERE, "staged/changes"), os.path.join(OBSERVATION, "📜️changes")), (os.path.join(HERE, "staged/journal"), os.path.join(RESIDENT, "📜️journal"))]
SCRIPT = os.path.join(OBSERVATION, "📜️script.ts")
EDITS = [
    ('resolve(this.root,"🔁️churn/🧪️tests/🧾️refusals/🟦️.ts")],this.repoRoot,"filesystem:rooted-descriptors"',
     'resolve(this.root,"🔁️churn/🧪️tests/🧾️refusals/🟦️.ts"),resolve(this.root,"📜️changes/🧪️tests/🟦️.ts")],this.repoRoot,"filesystem:rooted-descriptors"'),
    ('resolve(this.root,"🔁️churn/🟦️.ts"),resolve(this.root,"🧪️tests/🟦️.ts")],this.repoRoot,"tool:owner"',
     'resolve(this.root,"🔁️churn/🟦️.ts"),resolve(this.root,"📜️changes/🟦️.ts"),resolve(this.root,"📜️changes/🧭️client/🟦️.ts"),resolve(this.root,"📜️changes/🌳️tree/🟦️.ts"),resolve(this.root,"🧪️tests/🟦️.ts")],this.repoRoot,"tool:owner"'),
]


def main():
    write = "--write" in sys.argv
    problems, actions = [], []
    for source, target in COPIES:
        for base, dirs, names in os.walk(source):
            dirs[:] = [d for d in dirs if d != "__pycache__"]
            for name in names:
                path = os.path.join(base, name)
                destination = os.path.join(target, os.path.relpath(path, source))
                if len(destination) > 250:
                    problems.append("path too long: " + destination)
                if os.path.exists(destination):
                    if not filecmp.cmp(path, destination, shallow=False):
                        problems.append("differs: " + destination)
                else:
                    actions.append((path, destination))
    text = open(SCRIPT, encoding="utf8").read()
    edited = text
    for old, new in EDITS:
        if new in edited:
            continue
        if edited.count(old) != 1:
            problems.append("anchor not found exactly once: " + old[:60])
        else:
            edited = edited.replace(old, new)
    print("%d files to copy, script %s, %d problems" % (len(actions), "edited" if edited != text else "unchanged", len(problems)))
    for problem in problems:
        print("PROBLEM", problem)
    if problems or not write:
        return 1 if problems else 0
    for path, destination in actions:
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        with open(path, "rb") as handle, open(destination, "wb") as out:
            out.write(handle.read())
    if edited != text:
        open(SCRIPT, "w", encoding="utf8").write(edited)
    print("applied")
    return 0


if __name__ == "__main__":
    sys.exit(main())
