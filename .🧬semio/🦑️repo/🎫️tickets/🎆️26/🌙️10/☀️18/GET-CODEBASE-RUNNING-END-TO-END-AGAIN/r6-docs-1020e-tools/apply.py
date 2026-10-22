#!/usr/bin/env python3
"""Applies the staged concurrency / incremental / shared-cache guide (EN, DE) and the statements it makes stale to the product README.

Dry run by default. All-or-nothing: the whole new text is computed in memory, checked, and replaced in one atomic rename.
Every edit is anchored. An edit is `applied` when its `new` text occurs exactly once, `pending` when its `old` text occurs exactly once, and a
`refresh` when the text this tool wrote earlier (recorded in applied.json) occurs exactly once and differs from the new text, which is how a feature
that went live since the last apply flips its state words. Anything else is drift and the run refuses. A README whose hash differs from the staged base
(or from our last result) is drift too, unless --rebase (allowed only when every anchor still matches exactly once). The feature states recorded in
edits.json are recomputed from the live tree and must be equal, otherwise the run refuses: rerun make-edits.py. Python standard library only.

Usage:
  apply.py                  dry run: print the plan, write nothing
  apply.py --write          apply (backup in ./backup, record in ./applied.json)
  apply.py --rebase         accept a README that changed since staging when all anchors still match
  apply.py --undo           restore the backup while the README still equals the applied result
  apply.py --root DIR       workspace root (default: derived from this script's location)
  apply.py --readme FILE    README path (default: inside the root), used by the rehearsal
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
args = sys.argv[1:]
import lib

README = lib.README
if "--readme" in args:
    README = os.path.abspath(args[args.index("--readme") + 1])
WRITE, REBASE, UNDO = "--write" in args, "--rebase" in args, "--undo" in args
EDITS = json.load(open(os.path.join(HERE, "staged", "edits.json"), encoding="utf8"))
BACKUP, RECORD = lib.BACKUP, lib.RECORD
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
record = json.load(open(RECORD, encoding="utf8")) if os.path.isfile(RECORD) else None

if UNDO:
    if record is None:
        die("nothing recorded to undo")
    if sha(before) != record["after"]:
        die("the README changed after the apply; undo would discard those changes")
    original = open(os.path.join(BACKUP, "README." + record["before"][:16] + ".md"), "rb").read()
    if sha(original) != record["before"]:
        die("backup does not match its recorded hash")
    temporary = README + ".docs-1020e.tmp"
    open(temporary, "wb").write(original)
    os.replace(temporary, README)
    os.remove(RECORD)
    print("restored README to " + record["before"][:16])
    sys.exit(0)

states, suspicious = lib.probe_states()
if suspicious:
    die("probe paths with a missing parent directory: " + ", ".join(suspicious))
changed = sorted(k for k in states if bool(states[k]) != bool(EDITS["probes"].get(k)))
if changed:
    die("feature states moved since make-edits.py ran (" + ", ".join(f"{k}={'live' if states[k] else 'staged'}" for k in changed) + "); rerun make-edits.py")

history = {} if sha(before) == EDITS["base"] else (record or {}).get("history", {})
pending, refresh, applied, missing = [], [], [], []
result = text
for edit in EDITS["edits"]:
    new, old, previous = edit["new"], edit["old"], history.get(edit["id"])
    if result.count(new) == 1:
        applied.append(edit["id"])
    elif previous:
        if previous != new and result.count(previous) == 1:
            result = result.replace(previous, new, 1)
            refresh.append(edit["id"])
        else:
            missing.append(edit["id"])
    elif result.count(old) == 1:
        result = result.replace(old, new, 1)
        pending.append(edit["id"])
    else:
        missing.append(edit["id"])
print(f"README {sha(before)[:16]} (staged base {EDITS['base'][:16]}): {len(pending)} pending, {len(refresh)} refresh, {len(applied)} applied, {len(missing)} missing, {len(EDITS['deferred'])} deferred")
if EDITS["deferred"]:
    print("deferred until their feature is live: " + ", ".join(d["id"] for d in EDITS["deferred"]))
if missing:
    die("anchors that match neither as old nor as new text (drift): " + ", ".join(missing))
if not pending and not refresh:
    print("already applied")
    sys.exit(0)
known = {EDITS["base"]} | ({record["after"]} if record else set())
if sha(before) not in known and not REBASE:
    die("the README changed since staging; every anchor still matches, rerun with --rebase to apply anyway")
if fences(result) % 2 != fences(text) % 2 or fences(result) % 2:
    die("the result would unbalance code fences")
if broken_tables(result) > broken_tables(text):
    die("the result would add a table with differing column counts")
for marker in ("### 👥️ Many agents in one folder\n", "#### Viele Agenten in einem Ordner\n"):
    if result.count(marker) != 1:
        die("expected exactly one " + repr(marker.strip()))
after = result.encode("utf8")
print(f"result {sha(after)[:16]}: {len(after) - len(before):+d} bytes, {result.count(chr(10)) - text.count(chr(10)):+d} lines")
for identifier in pending + refresh:
    print("  " + identifier + (" (refresh)" if identifier in refresh else ""))
if not WRITE:
    print("dry run: nothing written (use --write)")
    sys.exit(0)

os.makedirs(BACKUP, exist_ok=True)
open(os.path.join(BACKUP, "README." + sha(before)[:16] + ".md"), "wb").write(before)
if open(README, "rb").read() != before:
    die("the README changed while the plan was computed; rerun")
temporary = README + ".docs-1020e.tmp"
with open(temporary, "wb") as handle:
    handle.write(after)
    handle.flush()
    os.fsync(handle.fileno())
os.replace(temporary, README)
if sha(open(README, "rb").read()) != sha(after):
    die("written README does not match the plan", 3)
first = record["before"] if record and record["after"] == sha(before) else sha(before)
history.update({e["id"]: e["new"] for e in EDITS["edits"]})
json.dump({"before": first, "after": sha(after), "history": history, "states": EDITS["probes"]}, open(RECORD, "w", encoding="utf8"), ensure_ascii=False)
print("applied " + str(len(pending)) + " edits, refreshed " + str(len(refresh)))
