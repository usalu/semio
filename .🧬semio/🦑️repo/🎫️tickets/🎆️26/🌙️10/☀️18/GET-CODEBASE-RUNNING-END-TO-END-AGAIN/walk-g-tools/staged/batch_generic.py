"""[gram2-g] generic staged batch: anchored edits (edits.json: {"file"?:..., "edits":[{file?,name,old,new}]}) plus new files from the flat tree/ (manifest.json maps the repository path to f<NNN>) (regular files only, never overwriting a different file). usage: batch_generic.py <batch-dir> apply|revert|status. BATCH_ROOT overrides the workspace root (scratch round trips). Idempotent; refuses when an anchor is missing or ambiguous."""
import hashlib, json, os, shutil, sys
HERE = os.path.abspath(sys.argv[1])
ROOT = os.environ.get("BATCH_ROOT") or os.path.abspath(os.path.join(HERE, *([".."] * 10)))
TREE = os.path.join(HERE, "tree")
SPEC = json.load(open(os.path.join(HERE, "edits.json"), encoding="utf8"))
for edit in SPEC["edits"]:
    edit.setdefault("file", SPEC.get("file"))


FLAT = json.load(open(os.path.join(TREE, "manifest.json"), encoding="utf8")) if os.path.isdir(TREE) else {}


def stored(name):
    return os.path.join(TREE, FLAT[name]["file"])


NEW_FILES = sorted(FLAT)


def path(rel):
    return os.path.join(ROOT, rel)


def sha(file):
    return hashlib.sha256(open(file, "rb").read()).hexdigest()


def state(edit):
    text = open(path(edit["file"]), encoding="utf8").read()
    return "new" if text.count(edit["new"]) == 1 else "old" if text.count(edit["old"]) == 1 else "bad"


def status():
    return {"edits": [state(e) for e in SPEC["edits"]], "new": {name: os.path.isfile(path(name)) and sha(path(name)) == sha(stored(name)) for name in NEW_FILES}}


def apply():
    states = [state(e) for e in SPEC["edits"]]
    if "bad" in states:
        sys.exit("anchor missing or ambiguous: " + ", ".join(e["name"] for e, s in zip(SPEC["edits"], states) if s == "bad"))
    for name in NEW_FILES:
        if os.path.isfile(path(name)) and sha(path(name)) != sha(stored(name)):
            sys.exit("refuse to overwrite a different " + name)
    for edit, st in zip(SPEC["edits"], states):
        if st == "old":
            text = open(path(edit["file"]), encoding="utf8").read()
            open(path(edit["file"]), "w", encoding="utf8").write(text.replace(edit["old"], edit["new"]))
    for name in NEW_FILES:
        os.makedirs(os.path.dirname(path(name)), exist_ok=True)
        shutil.copyfile(stored(name), path(name))


def revert():
    for edit in reversed(SPEC["edits"]):
        text = open(path(edit["file"]), encoding="utf8").read()
        if text.count(edit["new"]) == 1:
            open(path(edit["file"]), "w", encoding="utf8").write(text.replace(edit["new"], edit["old"]))
    for name in NEW_FILES:
        if os.path.isfile(path(name)) and sha(path(name)) == sha(stored(name)):
            os.remove(path(name))
            folder = os.path.dirname(path(name))
            while folder != ROOT and os.path.isdir(folder) and not os.listdir(folder):
                os.rmdir(folder)
                folder = os.path.dirname(folder)


{"apply": apply, "revert": revert, "status": lambda: print(status())}[sys.argv[2]]()
