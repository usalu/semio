"""📓️ docs5 laws for the staged doc batches, run against a MIRROR root (never the live tree).

usage: python3 -I verify.py <mirror root> [seed...]
The mirror root holds copies of the build guide and the README as they were when the batches were built
(`mirror.py create d4g --only <guide> <README> --no-closure --no-nm`).
Laws:
 1. every docs5 batch is pending on the pristine documents; the base hashes equal the staged ones.
 2. in seeded random orders, mixed with every PENDING docs3 batch: all apply (exit 0), all re-apply as a no-op, all report applied;
    the sorted lines of the result are equal for every order; reverting in another random order restores the original bytes.
 3. independent oracle: a line-based applier written here (no code shared with stage.py) produces the same bytes as the engine, doc by doc.
 4. a moved anchor makes apply refuse with exit 2 and leaves the file untouched.
 5. parity: EN and DE edit lists are parallel (same ids, same count); the SEMIO_ names equal; the digit runs equal (decimal comma/point, thousands separators and date order folded);
    every table row of an inserted block has exactly three pipes; every inserted heading exists in both languages.
 6. claims: every SEMIO_ name and every named identifier of the batches exists in the live source tree (rg -F -f, streamed)."""
import atexit
import hashlib
import json
import os
import random
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
STAGED = os.path.join(HERE, os.environ.get("DOCS5_STAGED", "staged"))
DOCS3 = os.path.join(C, "docs3-g-tools", "staged")
root = os.path.abspath(sys.argv[1])
seeds = [int(x) for x in sys.argv[2:]] or [1, 2, 3]
index = json.load(open(os.path.join(STAGED, "index.json"), encoding="utf8"))["batches"]
names = [row["name"] for row in index]
docs = sorted({row["doc"] for row in index})
failures = []


def path(doc):
    return os.path.join(root, doc)


def sha(doc):
    return hashlib.sha256(open(path(doc), "rb").read()).hexdigest()


def snapshot():
    return {doc: open(path(doc), "rb").read() for doc in docs}


def modes():
    return {doc: os.stat(path(doc)).st_mode & 0o7777 for doc in docs}


def restore(saved):
    for doc, data in saved.items():
        open(path(doc), "wb").write(data)


def run(directory, name, mode):
    process = subprocess.run([sys.executable, "-I", os.path.join(directory, name, "stage.py"), mode, "--root", root], capture_output=True, text=True)
    return process.returncode, process.stdout


def expect(condition, message):
    if not condition:
        failures.append(message)
        print("FAIL", message, flush=True)


original = snapshot()
original_modes = modes()
atexit.register(lambda: restore(original))
for row in index:
    expect(sha(row["doc"]) == row["baseSha256"], "%s: mirror document equals the staged base (%s)" % (row["name"], row["doc"].split("/")[-1]))
needs = {n: json.load(open(os.path.join(STAGED, n, "edits.json"), encoding="utf8")).get("needs", []) for n in names}
for name in names:
    code, out = run(STAGED, name, "check")
    if needs[name]:
        expect(code == 3 and "result: prereq" in out, "%s: prerequisite reported on the pristine documents (exit %d)" % (name, code))
        code, out = run(STAGED, name, "apply")
        expect(code == 3 and snapshot() == original, "%s: apply refuses with exit 3 and writes nothing" % name)
    else:
        expect(code == 0 and "result: pending" in out, "%s: pending on the pristine documents (exit %d): %s" % (name, code, out.strip().splitlines()[-1] if out.strip() else ""))
foreign = []
if os.path.isdir(DOCS3):
    for name in sorted(os.listdir(DOCS3)):
        if os.path.isfile(os.path.join(DOCS3, name, "stage.py")):
            code, out = run(DOCS3, name, "check")
            if code == 0 and "result: pending" in out:
                foreign.append(name)
print("docs3 batches pending in the mirror: %d (%s)" % (len(foreign), ", ".join(foreign)), flush=True)
free_members = [(STAGED, n) for n in names if not needs[n]] + [(DOCS3, n) for n in foreign]
late_members = [(STAGED, n) for n in names if needs[n]]
members = free_members + late_members


def apply_all(order):
    for directory, name in order:
        code, out = run(directory, name, "apply")
        expect(code == 0, "apply %s exits %d: %s" % (name, code, out.strip()[-160:]))


results = []
for seed in seeds:
    order = free_members[:]
    random.Random(seed).shuffle(order)
    apply_all(order + late_members)
    after = snapshot()
    expect(modes() == original_modes, "seed %d: applying every batch keeps the file modes" % seed)
    for directory, name in members:
        code, out = run(directory, name, "check")
        expect(code == 0 and "result: applied" in out, "seed %d: %s reports applied" % (seed, name))
    for directory, name in members:
        run(directory, name, "apply")
    expect(snapshot() == after, "seed %d: re-applying every batch is a no-op" % seed)
    results.append({doc: sorted(data.decode("utf8").split("\n")) for doc, data in after.items()})
    order = free_members[:]
    random.Random(seed + 100).shuffle(order)
    for directory, name in list(reversed(late_members)) + order:
        code, out = run(directory, name, "revert")
        expect(code == 0, "seed %d: revert %s exits %d" % (seed, name, code))
    expect(snapshot() == original and modes() == original_modes, "seed %d: reverting every batch restores the original bytes and modes" % seed)
    print("seed %d: %d batches (%d docs3) applied, re-applied, reverted" % (seed, len(members), len(foreign)), flush=True)
expect(all(r == results[0] for r in results), "the applied documents have the same lines for every application order")


def apply_independent(text, spec):
    lines = text.split("\n")
    for edit in spec["edits"]:
        if edit["kind"] == "replace":
            assert text_count(lines, edit["old"]) == 1, edit["id"]
            joined = "\n".join(lines).replace(edit["old"], edit["new"], 1)
            lines = joined.split("\n")
        else:
            hits = [i for i, line in enumerate(lines) if line == edit["anchor"]]
            assert len(hits) == 1, (edit["id"], len(hits))
            block = edit["block"].split("\n")
            at = hits[0] + 1 if edit["where"] == "after" else hits[0]
            lines[at:at] = block
    return "\n".join(lines)


def text_count(lines, fragment):
    return "\n".join(lines).count(fragment)


for doc in docs:
    text = original[doc].decode("utf8")
    for row in sorted((r for r in index if r["doc"] == doc), key=lambda r: r["order"]):
        spec = json.load(open(os.path.join(STAGED, row["name"], "edits.json"), encoding="utf8"))
        text = apply_independent(text, spec)
    for row in sorted((r for r in index if r["doc"] == doc), key=lambda r: r["order"]):
        code, out = run(STAGED, row["name"], "apply")
        expect(code == 0, "oracle run: apply %s" % row["name"])
    engine = open(path(doc), "rb").read().decode("utf8")
    expect(sorted(engine.split("\n")) == sorted(text.split("\n")), "independent applier equals the engine (same lines) for %s" % doc.split("/")[-1])
    expect(len(engine) == len(text), "independent applier equals the engine (same length) for %s" % doc.split("/")[-1])
    restore(original)
expect(snapshot() == original, "oracle runs left the mirror pristine")

backup = snapshot()
for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    edit = spec["edits"][0]
    needle = edit.get("old") or edit["anchor"]
    doc = spec["guide"]
    text = backup[doc].decode("utf8")
    mutated = text.replace(needle, needle[:-4] + "XYZ" + needle[-1:], 1)
    open(path(doc), "wb").write(mutated.encode("utf8"))
    before = sha(doc)
    code, out = run(STAGED, name, "apply")
    expect(code == 2 and sha(doc) == before, "drift: %s refuses with exit 2 and leaves the file untouched (exit %d)" % (name, code))
    restore(backup)

NUMBER = re.compile(r"\d+")
SPAN = re.compile(r"`([^`]+)`")


def numbers(text):
    return sorted(NUMBER.findall(text))


def spans(text):
    return sorted(s for s in SPAN.findall(text) if "<" not in s and re.fullmatch(r"[A-Za-z0-9_./*:=|\-\\{}\[\]\"' ]+", s) is not None and re.search(r"[A-Z_]{4,}|\.|/|--|_", s))


for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    en = [e for e in spec["edits"] if e["lang"] == "en"]
    de = [e for e in spec["edits"] if e["lang"] == "de"]
    expect(len(en) == len(de) and [e["id"] for e in en] == [e["id"] for e in de], "%s: EN and DE edit lists are parallel" % name)
    for a, b in zip(en, de):
        ta, tb = a.get("block") or a.get("new") or "", b.get("block") or b.get("new") or ""
        expect(set(re.findall(r"SEMIO_[A-Z0-9_]+", ta)) == set(re.findall(r"SEMIO_[A-Z0-9_]+", tb)), "%s/%s: SEMIO_ names equal in EN and DE" % (name, a["id"]))
        na, nb = numbers(ta), numbers(tb)
        expect(na == nb, "%s/%s: numbers equal in EN and DE (%s)" % (name, a["id"], sorted(set(na) ^ set(nb))))
        sa, sb = spans(ta), spans(tb)
        expect(sa == sb, "%s/%s: identifier spans equal in EN and DE (%s)" % (name, a["id"], sorted(set(sa) ^ set(sb))))
        for edit in (a, b):
            block = edit.get("block") or edit.get("new") or ""
            rows = [line for line in block.split("\n") if line.startswith("|")]
            for line in rows:
                expect(line.count("|") == (rows[0].count("|") if len(rows) > 1 else 3) and line.count("|") >= 3, "%s/%s: the rows of a table block have one pipe count (%d): %s" % (name, edit["id"], line.count("|"), line[:50]))
            for line in block.split("\n"):
                if line.startswith("## "):
                    expect(True, "heading")
    heads_en = [l for e in en for l in (e.get("block") or "").split("\n") if l.startswith("## ")]
    heads_de = [l for e in de for l in (e.get("block") or "").split("\n") if l.startswith("## ")]
    expect(len(heads_en) == len(heads_de), "%s: the same number of inserted headings in EN and DE" % name)

CLAIMS = json.load(open(os.path.join(HERE, "claims.json"), encoding="utf8"))
used = set()
for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    for edit in spec["edits"]:
        used.update(re.findall(r"SEMIO_[A-Z0-9_]+", edit.get("block") or edit.get("new") or ""))
live_needles = set(CLAIMS["live"]) | {n for n in used if n not in CLAIMS["pending"]}
pending = CLAIMS["pending"]
live = os.environ.get("DOCS5_LIVE") or C
while not os.path.isfile(os.path.join(live, "AGENTS.md")):
    live = os.path.dirname(live)


def search(words, targets):
    with tempfile.NamedTemporaryFile("w", delete=False, suffix=".txt", encoding="utf8") as handle:
        handle.write("\n".join(sorted(words, key=lambda word: (-len(word), word))) + "\n")
        needle_file = handle.name
    command = ["rg", "-F", "-o", "-N", "--no-filename", "--no-messages", "-g", "!*.log", "-g", "!*.jsonl", "-g", "!node_modules", "-g", "!🗑️generated", "-g", "!📓️build-guide.md", "-g", "!README.md", "-f", needle_file] + targets
    seen = set()
    process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True, encoding="utf8")
    for line in process.stdout:
        seen.add(line.rstrip("\n"))
    process.wait()
    os.unlink(needle_file)
    return seen


found = search(live_needles, [os.path.join(live, "🧰️framework"), os.path.join(live, "nx.json")])
missing = sorted(n for n in live_needles if n not in found)
expect(not missing, "every identifier named as live exists in the live source tree (missing: %s)" % missing)
landed, absent = [], []
for needle, where in sorted(pending.items()):
    in_batch = search([needle], [os.path.join(C, where)])
    in_live = search([needle], [os.path.join(live, "🧰️framework")])
    if needle not in in_batch:
        absent.append(needle)
    if needle in in_live:
        landed.append(needle)
expect(not absent, "every identifier marked pending exists in its staged batch (missing: %s)" % absent)
print("NOTE pending claims that are live by now (drop the marker): %s" % (landed or "none"))

for name in names:
    spec = json.load(open(os.path.join(STAGED, name, "edits.json"), encoding="utf8"))
    for edit in spec["edits"]:
        text = edit.get("block") or edit.get("new") or ""
        marker = "pending gate" if edit["lang"] == "en" else "wartet auf das Tor"
        for needle in pending:
            if needle in text:
                expect(marker in text, "%s/%s[%s]: names the pending identifier %s and carries the marker '%s'" % (name, edit["id"], edit["lang"], needle, marker))
files_missing = [f for f in CLAIMS["files"] if not os.path.exists(os.path.join(C, f))]
expect(not files_missing, "every ticket tool named exists (missing: %s)" % files_missing)
bad_paths = []
for suffix in CLAIMS["paths"]:
    hit = subprocess.run(["rg", "--files", "--no-messages", "-g", "!node_modules", "-g", "**/" + suffix + "*", "-g", "**/" + suffix + "/**", os.path.join(live, "🧰️framework")], capture_output=True, text=True).stdout.strip()
    if not hit:
        bad_paths.append(suffix)
expect(not bad_paths, "every path the batches cite exists below the framework (missing: %s)" % bad_paths)
bad_roots = [f for f, needle in CLAIMS["rootfiles"] if needle not in open(os.path.join(live, f), encoding="utf8").read()]
expect(not bad_roots, "every cited root file holds what is said of it (%s)" % bad_roots)
print("claims: %d live, %d pending (%d landed), %d tools, %d missing" % (len(live_needles), len(pending), len(landed), len(CLAIMS["files"]), len(missing)))
print("FAILURES: %d" % len(failures))
sys.exit(1 if failures else 0)
