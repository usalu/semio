"""[gram4-g] self-contained staged batch tool: anchored edits (edits.json: {"edits":[{file,name,old,new}]}) plus new files from tree/ (never overwriting a different file), with the sha256 of every edited file before the batch (bases.json, written by `bases`). usage: stage.py <batch dir> bases|status|check|apply|revert  (BATCH_ROOT overrides the workspace root for scratch copies / mirrors; exit 0 ok, 2 refused). `check`: every anchor appears exactly once (old) or the batch is applied (new) and the base hashes match; `apply`/`revert` are idempotent and atomic per file (all anchors are verified before the first write)."""
import hashlib, json, os, sys
HERE = os.path.abspath(sys.argv[1])
ROOT = os.environ.get("BATCH_ROOT") or "/Users/ueli/Documents/semio"
TREE = os.path.join(HERE, "tree")
SPEC = json.load(open(os.path.join(HERE, "edits.json"), encoding="utf8"))["edits"]
BASES = os.path.join(HERE, "bases.json")


def path(rel):
    return os.path.join(ROOT, rel)


def sha(file):
    return hashlib.sha256(open(file, "rb").read()).hexdigest()


def new_files():
    found = []
    for base, dirs, names in os.walk(TREE):
        found += [os.path.relpath(os.path.join(base, n), TREE) for n in names if not os.path.islink(os.path.join(base, n))]
    return sorted(found) if os.path.isdir(TREE) else []


def state(edit):
    text = open(path(edit["file"]), encoding="utf8").read()
    return "new" if text.count(edit["new"]) == 1 and edit["new"] != edit["old"] and text.count(edit["old"]) == 0 else "old" if text.count(edit["old"]) == 1 else "bad"


def states():
    return [state(e) for e in SPEC]


def bases():
    return json.load(open(BASES, encoding="utf8")) if os.path.exists(BASES) else {}


def check():
    sts, bad = states(), []
    for e, s in zip(SPEC, sts):
        if s == "bad":
            bad.append("anchor missing or ambiguous: " + e["name"])
    if "old" in sts and "new" in sts:
        bad.append("batch partly applied")
    if "old" in sts:
        for file, digest in bases().items():
            if sha(path(file)) != digest:
                bad.append("base changed: " + file)
        for name in new_files():
            if os.path.isfile(path(name)) and sha(path(name)) != sha(os.path.join(TREE, name)):
                bad.append("would overwrite a different " + name)
    return sts, bad


def write_bases():
    files = sorted({e["file"] for e in SPEC})
    json.dump({f: sha(path(f)) for f in files}, open(BASES, "w", encoding="utf8"), ensure_ascii=False, indent=1)


def apply():
    sts, bad = check()
    if bad:
        sys.exit("refused: " + "; ".join(bad))
    texts = {}
    for e, s in zip(SPEC, sts):
        if s == "old":
            t = texts.get(e["file"]) or open(path(e["file"]), encoding="utf8").read()
            texts[e["file"]] = t.replace(e["old"], e["new"], 1) if t.count(e["old"]) == 1 else sys.exit("anchor lost: " + e["name"])
    for file, t in texts.items():
        open(path(file), "w", encoding="utf8").write(t)
    for name in new_files():
        os.makedirs(os.path.dirname(path(name)), exist_ok=True)
        open(path(name), "wb").write(open(os.path.join(TREE, name), "rb").read())


def revert():
    texts = {}
    for e in reversed(SPEC):
        t = texts.get(e["file"]) or open(path(e["file"]), encoding="utf8").read()
        if t.count(e["new"]) == 1:
            texts[e["file"]] = t.replace(e["new"], e["old"], 1)
        elif t.count(e["old"]) != 1:
            sys.exit("refused: neither the old nor the new text is present once: " + e["name"])
    for file, t in texts.items():
        open(path(file), "w", encoding="utf8").write(t)
    for name in new_files():
        if os.path.isfile(path(name)) and sha(path(name)) == sha(os.path.join(TREE, name)):
            os.remove(path(name))
            folder = os.path.dirname(path(name))
            while folder != ROOT and os.path.isdir(folder) and not os.listdir(folder):
                os.rmdir(folder)
                folder = os.path.dirname(folder)


cmd = sys.argv[2]
if cmd == "bases":
    write_bases()
elif cmd == "status":
    sts, bad = check()
    print(json.dumps({"edits": sts, "problems": bad, "newFiles": len(new_files())}, ensure_ascii=False))
elif cmd == "check":
    sts, bad = check()
    print(("OK " if not bad else "REFUSED ") + json.dumps({"state": sorted(set(sts)), "problems": bad}, ensure_ascii=False))
    sys.exit(0 if not bad else 2)
elif cmd == "apply":
    apply()
elif cmd == "revert":
    revert()
else:
    sys.exit("usage: stage.py <batch dir> bases|status|check|apply|revert")
