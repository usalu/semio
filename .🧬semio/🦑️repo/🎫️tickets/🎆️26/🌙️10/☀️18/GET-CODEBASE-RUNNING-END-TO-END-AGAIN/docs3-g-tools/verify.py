"""📓️ docs3-g laws for the staged doc batches, run against a MIRROR root (never the live tree).

usage: python3 -I verify.py <mirror root> [seed...]
The mirror root holds a copy of the guide as it was when the batches were built (`mirror.py create d3g-a --only <guide> --no-closure --no-nm`).
Laws: every batch is pending and clean-checks on the pristine guide (quietfc: prerequisite exit 3); in several seeded random orders every batch applies, re-applies as a no-op and
all batches report applied; the sorted lines of the result are identical for every order (blocks at one anchor may swap places); reverting in another random order restores the
original bytes; a moved anchor makes apply refuse with exit 2 and leaves the file byte-identical; EN and DE halves carry the same number of edits and the same SEMIO_ names."""
import hashlib
import json
import os
import random
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
root = os.path.abspath(sys.argv[1])
seeds = [int(x) for x in sys.argv[2:]] or [1, 2, 3, 4]
index = json.load(open(os.path.join(STAGED, "index.json"), encoding="utf8"))
guide = os.path.join(root, index["guide"])
names = [b["name"] for b in index["batches"]]
PREREQ = {"quietfc"}
failures = []


def sha():
    return hashlib.sha256(open(guide, "rb").read()).hexdigest()


def run(name, mode):
    process = subprocess.run([sys.executable, "-I", os.path.join(STAGED, name, "stage.py"), mode, "--root", root], capture_output=True, text=True)
    return process.returncode, process.stdout


def expect(condition, message):
    if not condition:
        failures.append(message)
        print("FAIL", message)


original = sha()
expect(original == index["baseSha256"], "mirror guide equals the staged base (%s vs %s)" % (original[:12], index["baseSha256"][:12]))
for name in names:
    code, out = run(name, "check")
    expect(code == (3 if name in PREREQ else 0), "%s: check on the pristine guide exits %d: %s" % (name, code, out.strip().splitlines()[-1] if out.strip() else ""))
    if name not in PREREQ:
        expect("result: pending" in out, "%s: pristine guide is pending" % name)
free = [n for n in names if n not in PREREQ]
results = []
for seed in seeds:
    order = free[:]
    random.Random(seed).shuffle(order)
    for name in order:
        code, out = run(name, "apply")
        expect(code == 0, "seed %d: apply %s exits %d: %s" % (seed, name, code, out.strip()[-200:]))
    after = sha()
    for name in free:
        code, out = run(name, "check")
        expect(code == 0 and "result: applied" in out, "seed %d: %s reports applied: %s" % (seed, name, out.strip().splitlines()[-1]))
    for name in free:
        code, out = run(name, "apply")
        expect(code == 0 and sha() == after, "seed %d: re-apply %s is a no-op" % (seed, name))
    results.append(sorted(open(guide, encoding="utf8").read().split("\n")))
    order = free[:]
    random.Random(seed + 100).shuffle(order)
    for name in order:
        code, out = run(name, "revert")
        expect(code == 0, "seed %d: revert %s exits %d: %s" % (seed, name, code, out.strip()[-200:]))
    expect(sha() == original, "seed %d: reverting every batch restores the original bytes" % seed)
    print("seed %d: %d batches applied, re-applied, reverted" % (seed, len(free)), flush=True)
expect(all(r == results[0] for r in results), "the applied guide has the same lines for every application order")
# partial states: apply half, check the others stay pending, revert half
half = free[::2]
for name in half:
    run(name, "apply")
for name in free:
    code, out = run(name, "check")
    expect(code == 0 and ("result: applied" if name in half else "result: pending") in out, "partial: %s state" % name)
for name in reversed(half):
    run(name, "revert")
expect(sha() == original, "partial apply then revert restores the original bytes")
# drift: a moved anchor refuses and writes nothing
backup = open(guide, "rb").read()
text = backup.decode("utf8")
for name in ("dashlock", "mirror", "bench2"):
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    edit = spec["edits"][0]
    needle = edit.get("old") or edit["anchor"]
    mutated = text.replace(needle, needle[:-4] + "XYZ" + needle[-1:], 1)
    open(guide, "wb").write(mutated.encode("utf8"))
    before = sha()
    code, out = run(name, "apply")
    expect(code == 2 and sha() == before, "drift: %s refuses with exit 2 and leaves the file untouched (exit %d)" % (name, code))
open(guide, "wb").write(backup)
# prerequisite: quietfc needs the code batch text
code, out = run("quietfc", "apply")
expect(code == 3 and sha() == original, "quietfc refuses without its code batch (exit 3)")
# parity of EN and DE
for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    for lang_edits in (spec["edits"],):
        en = [e for e in lang_edits if e["lang"] == "en"]
        de = [e for e in lang_edits if e["lang"] == "de"]
        expect(len(en) == len(de) and [e["id"] for e in en] == [e["id"] for e in de], "%s: EN and DE edit lists are parallel" % name)
        for a, b in zip(en, de):
            ta = a.get("block") or a.get("new") or ""
            tb = b.get("block") or b.get("new") or ""
            names_en = set(re.findall(r"SEMIO_[A-Z0-9_]+", ta))
            names_de = set(re.findall(r"SEMIO_[A-Z0-9_]+", tb))
            expect(names_en == names_de, "%s/%s: SEMIO_ names equal in EN and DE (%s vs %s)" % (name, a["id"], sorted(names_en ^ names_de), ""))
for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    for edit in spec["edits"]:
        block = edit.get("block")
        if block and block.startswith("|"):
            expect(block.count("|") == 3, "%s/%s: a table row has exactly three pipes (%d)" % (name, edit["id"], block.count("|")))
print("FAILURES: %d" % len(failures))
sys.exit(1 if failures else 0)
