#!/usr/bin/env python3
"""Idempotent anchored applier for the short Bazel path change (see handoff-bazel-short-paths.md).

usage: apply.py --root <checkout-or-overlay-root> [--check]

Every edit is anchored on exact text. An edit is `applied` when its new text is already present, `pending`
when its old text occurs exactly once, and a hard failure (`anchor-missing`, `anchor-ambiguous`) otherwise, so
a moved anchor is reported instead of guessed. `--check` writes nothing. New files are copied from staged/
(short names, because the repository paths would exceed the 256 unit rule inside the ticket folder).
"""
import argparse, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits

parser = argparse.ArgumentParser()
parser.add_argument("--root", required=True)
parser.add_argument("--check", action="store_true")
parser.add_argument("--live", action="store_true", help="allow writing into a git checkout (only for the peer integrate)")
arguments = parser.parse_args()
root = os.path.abspath(arguments.root)
if not arguments.check and not arguments.live and os.path.exists(os.path.join(root, ".git")):
    sys.exit("refused: %s is a git checkout; verify in an overlay, or pass --live when integrating" % root)
failures = 0
report = []


def note(state, target, detail=""):
    global failures
    if state.startswith("anchor"):
        failures += 1
    report.append("%-15s %s %s" % (state, target, detail))


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    if os.path.commonpath([os.path.realpath(path), os.path.realpath(root)]) != os.path.realpath(root):
        sys.exit("refused: %s resolves outside %s" % (path, root))
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


for staged, target in edits.NEW_FILES:
    destination = os.path.join(root, target)
    source = os.path.join(HERE, "staged", staged)
    if os.path.exists(destination) and read(destination) == read(source):
        note("applied", target)
        continue
    note("pending", target, "(new file)" if not os.path.exists(destination) else "(differs)")
    if not arguments.check:
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        if os.path.islink(destination) or os.path.commonpath([os.path.realpath(os.path.dirname(destination)), os.path.realpath(root)]) != os.path.realpath(root):
            sys.exit("refused: %s resolves outside %s" % (destination, root))
        shutil.copyfile(source, destination)

for target, pairs in edits.TEXT_EDITS:
    path = os.path.join(root, target)
    if not os.path.exists(path):
        note("anchor-missing", target, "file does not exist")
        continue
    text = read(path)
    changed = False
    for label, old, new in pairs:
        shrinks = new in old
        done = (old not in text and (new == "" or new in text)) if shrinks else new in text
        if done:
            note("applied", target, label)
        elif text.count(old) == 1:
            note("pending", target, label)
            text = text.replace(old, new)
            changed = True
        else:
            note("anchor-missing" if text.count(old) == 0 else "anchor-ambiguous", target, label)
    if changed and not arguments.check:
        write(path, text)

for target, rules in edits.LINE_EDITS:
    path = os.path.join(root, target)
    if not os.path.exists(path):
        note("anchor-missing", target, "file does not exist")
        continue
    lines = read(path).split("\n")
    changed = False
    for label, kind, start, end, new in rules:
        hits = [index for index, line in enumerate(lines) if line.startswith(start)]
        if not hits:
            note("applied" if (new is None or any(line == new for line in lines)) else "anchor-missing", target, label)
        elif len(hits) > 1:
            note("anchor-ambiguous", target, label)
        else:
            first = hits[0]
            last = first
            if kind == "block":
                after = [index for index in range(first, len(lines)) if lines[index] == end]
                if not after:
                    note("anchor-missing", target, label + " (end)")
                    continue
                last = after[0]
            lines[first : last + 1] = [] if new is None else [new]
            note("pending", target, label)
            changed = True
    if changed and not arguments.check:
        write(path, "\n".join(lines))

for target, label, transform in edits.JSON_EDITS:
    path = os.path.join(root, target)
    if not os.path.exists(path):
        note("anchor-missing", target, "file does not exist")
        continue
    text = read(path)
    document = json.loads(text)
    try:
        changed = transform(document)
    except KeyError as error:
        note("anchor-missing", target, "%s: %s" % (label, error))
        continue
    if not changed:
        note("applied", target, label)
        continue
    note("pending", target, label)
    if not arguments.check:
        write(path, json.dumps(document, indent=2, ensure_ascii=False) + "\n")

print("\n".join(report))
print("failures: %d" % failures)
sys.exit(1 if failures else 0)
