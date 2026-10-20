#!/usr/bin/env python3
"""Applies the staged build guide (EN and DE) and the statements it makes stale to the product README.

Dry run by default. All-or-nothing: the whole new text is computed in memory, checked, and replaced in one atomic rename.
Every edit is anchored: its `old` text must occur exactly once (pending) or its `new` text exactly once (already applied);
anything else is drift and the run refuses. A README whose hash differs from the staged base is drift too, unless --rebase
(allowed only when every anchor still matches exactly once). Cross-platform: Python standard library only.

Usage:
  apply.py                 dry run: print the plan, write nothing
  apply.py --write         apply (backup in ./backup, record in ./applied.json)
  apply.py --rebase        accept a README that changed since staging when all anchors still match
  apply.py --undo          restore the backup while the README still equals the applied result
  apply.py --root DIR      workspace root (default: derived from this script's location)
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
args = sys.argv[1:]
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
WRITE, REBASE, UNDO = "--write" in args, "--rebase" in args, "--undo" in args
EDITS = json.load(open(os.path.join(HERE, "staged", "edits.json"), encoding="utf8"))
README = os.path.join(ROOT, EDITS["readme"])
BACKUP = os.path.join(HERE, "backup")
RECORD = os.path.join(HERE, "applied.json")
sha = lambda b: hashlib.sha256(b).hexdigest()


def die(message, code=2):
    print("refused: " + message)
    sys.exit(code)


def broken_tables(text):
    broken, block, fence = 0, [], False
    for l in text.split("\n") + [""]:
        if l.startswith("```"):
            fence = not fence
        if not fence and l.startswith("|"):
            block.append(l)
        else:
            if block and len({len(re.split(r"(?<!\\)\|", b.strip().strip("|"))) for b in block}) != 1:
                broken += 1
            block = []
    return broken


def fences(text):
    return sum(1 for l in text.split("\n") if l.startswith("```"))


if not os.path.isfile(README):
    die("README not found: " + README)
before = open(README, "rb").read()
text = before.decode("utf8")

if UNDO:
    record = json.load(open(RECORD, encoding="utf8")) if os.path.isfile(RECORD) else die("nothing recorded to undo")
    if sha(before) != record["after"]:
        die("the README changed after the apply; undo would discard those changes")
    original = open(os.path.join(BACKUP, "README." + record["before"][:16] + ".md"), "rb").read()
    if sha(original) != record["before"]:
        die("backup does not match its recorded hash")
    temporary = README + ".docs-build.tmp"
    open(temporary, "wb").write(original)
    os.replace(temporary, README)
    os.remove(RECORD)
    print("restored README to " + record["before"][:16])
    sys.exit(0)

CATALOG = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json")
catalog = open(CATALOG, encoding="utf8").read() if os.path.isfile(CATALOG) else ""
pending, applied, missing, deferred = [], [], [], []
result = text
for edit in EDITS["edits"]:
    if edit.get("requires") and ('"name": "' + edit["requires"] + '"') not in catalog:
        deferred.append(edit["id"])
    elif result.count(edit.get("probe", edit["new"])) == 1 and (("probe" in edit) or result.count(edit["new"]) == 1):
        applied.append(edit["id"])
    elif result.count(edit["old"]) == 1:
        result = result.replace(edit["old"], edit["new"], 1)
        pending.append(edit)
    else:
        missing.append(edit["id"])
print(f"README {sha(before)[:16]} (staged base {EDITS['base'][:16]}): {len(pending)} pending, {len(applied)} applied, {len(deferred)} deferred, {len(missing)} missing")
if deferred:
    print("deferred until the operation catalogue offers them: " + ", ".join(deferred))
if missing:
    die("anchors that match neither as old nor as new text (drift): " + ", ".join(missing))
if not pending:
    print("already applied")
    sys.exit(0)
previous = json.load(open(RECORD, encoding="utf8")) if os.path.isfile(RECORD) else None
known = {EDITS["base"]} | ({previous["after"]} if previous else set())
if sha(before) not in known and not REBASE:
    die("the README changed since staging; every anchor still matches, rerun with --rebase to apply anyway")

if result.count("## ") == 0 or fences(result) % 2 != fences(text) % 2:
    die("the result would unbalance code fences")
if broken_tables(result) > broken_tables(text):
    die("the result would add a table with differing column counts")
for marker in ("Build tools guide\n", "### Build-Werkzeuge im Alltag\n"):
    if result.count(marker) != 1:
        die("expected exactly one " + repr(marker.strip()))
after = result.encode("utf8")
print(f"result {sha(after)[:16]}: {len(after) - len(before):+d} bytes, {result.count(chr(10)) - text.count(chr(10)):+d} lines")
for edit in pending:
    print("  " + edit["id"])
if not WRITE:
    print("dry run: nothing written (use --write)")
    sys.exit(0)

os.makedirs(BACKUP, exist_ok=True)
open(os.path.join(BACKUP, "README." + sha(before)[:16] + ".md"), "wb").write(before)
if open(README, "rb").read() != before:
    die("the README changed while the plan was computed; rerun")
temporary = README + ".docs-build.tmp"
with open(temporary, "wb") as handle:
    handle.write(after)
    handle.flush()
    os.fsync(handle.fileno())
os.replace(temporary, README)
if sha(open(README, "rb").read()) != sha(after):
    die("written README does not match the plan", 3)
first = previous["before"] if previous and previous["after"] == sha(before) else sha(before)
json.dump({"before": first, "after": sha(after), "edits": [e["id"] for e in pending]}, open(RECORD, "w", encoding="utf8"))
print("applied " + str(len(pending)) + " edits")
