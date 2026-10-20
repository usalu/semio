#!/usr/bin/env python3
"""Removes the short-path patch from a tree it was applied to by mistake, keeping every peer change.

usage: revert.py --root <checkout> [--check]
Files that the patch alone changed since HEAD are restored from `git show HEAD:<path>` (only after proving
HEAD + patch equals the file). Files that peers changed too (adapter, schema, test aggregator) get the exact
inverse of each anchored edit.
"""
import argparse, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits

parser = argparse.ArgumentParser()
parser.add_argument("--root", required=True)
parser.add_argument("--check", action="store_true")
arguments = parser.parse_args()
root = os.path.abspath(arguments.root)
failures = 0


def head(path):
    return subprocess.run(["git", "-C", root, "show", "HEAD:" + path], capture_output=True, check=True).stdout.decode("utf-8")


def read(path):
    with open(os.path.join(root, path), encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    if not arguments.check:
        with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
            handle.write(text)


def say(state, path, detail=""):
    print("%-14s %s %s" % (state, path[-70:], detail))


def patched_head(path):
    text = head(path)
    for target, pairs in edits.TEXT_EDITS:
        if target == path:
            for _label, old, new in pairs:
                text = text.replace(old, new) if old in text else text
    for target, rules in edits.LINE_EDITS:
        if target == path:
            lines = text.split("\n")
            for _label, kind, start, end, new in rules:
                hits = [i for i, line in enumerate(lines) if line.startswith(start)]
                if len(hits) == 1:
                    last = hits[0]
                    if kind == "block":
                        last = [i for i in range(hits[0], len(lines)) if lines[i] == end][0]
                    lines[hits[0] : last + 1] = [] if new is None else [new]
            text = "\n".join(lines)
    for target, _label, transform in edits.JSON_EDITS:
        if target == path:
            document = json.loads(text)
            transform(document)
            text = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
    return text


touched = {target for target, _ in edits.TEXT_EDITS} | {target for target, _ in edits.LINE_EDITS} | {target for target, _l, _t in edits.JSON_EDITS}
for path in sorted(touched):
    current = read(path)
    if patched_head(path) == current:
        say("restore-head", path)
        write(path, head(path))
        continue
    pairs = [pair for target, group in edits.TEXT_EDITS if target == path for pair in group]
    if pairs and not any(target == path for target, _ in edits.LINE_EDITS) and not any(target == path for target, _l, _t in edits.JSON_EDITS):
        for label, old, new in pairs:
            if current.count(new) == 1 and (old not in current or old in new):
                current = current.replace(new, old)
                say("inverse", path, label)
            elif old in current and new not in current:
                say("already-clean", path, label)
            else:
                failures += 1
                say("ANCHOR", path, label)
        write(path, current)
        continue
    if path.endswith("/🧬️schema/🔣️.json") and "/🏗️build/" in path:
        original = json.loads(head(path))["definitions"]
        document = json.loads(current)
        definitions = document["definitions"]
        for key in ("PlatformRules", "PlatformInput"):
            definitions[key] = original[key]
        definitions["PlatformProfile"]["properties"]["pathBudget"] = original["PlatformProfile"]["properties"]["pathBudget"]
        definitions["ActionPolicy"]["properties"]["sources"] = original["ActionPolicy"]["properties"]["sources"]
        del definitions["Actions"]["const"]["sources"]["alias"]
        for key in ("PathBudget", "PathSize", "PathClosure", "PathMeasurement", "PathBudgetReport"):
            definitions.pop(key, None)
        text = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
        if patched_head(path) != current:
            say("note", path, "schema also differs from HEAD by peer edits; inverse applied key-wise")
        say("inverse", path, "schema keys")
        write(path, text)
        continue
    failures += 1
    say("UNKNOWN", path, "neither HEAD+patch nor an anchored inverse applies")

for _staged, target in edits.NEW_FILES:
    if os.path.exists(os.path.join(root, target)):
        say("remove", target)
        if not arguments.check:
            os.remove(os.path.join(root, target))
directory = os.path.join(root, os.path.dirname(edits.NEW_FILES[0][1]))
if os.path.isdir(directory) and not os.listdir(directory) and not arguments.check:
    os.rmdir(directory)
print("failures: %d" % failures)
sys.exit(1 if failures else 0)
