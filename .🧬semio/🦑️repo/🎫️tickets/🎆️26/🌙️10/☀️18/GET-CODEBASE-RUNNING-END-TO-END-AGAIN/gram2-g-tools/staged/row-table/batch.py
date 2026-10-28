"""[gram2-g] staged batch `row-table`: observation rows get one number per fact from a verified hash table instead of a permanent key string per row (half the bytes per row). usage: batch.py apply|revert|status|check  Anchored edits from edits.json (each `old` exactly once, idempotent), plus the new leaf module and its law copied from the flat tree/ (manifest.json maps the repository path to f<NNN>) (never overwriting a different file)."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("BATCH_ROOT") or os.path.abspath(os.path.join(HERE, *([".."] * 10)))
TREE = os.path.join(HERE, "tree")
SPEC = json.load(open(os.path.join(HERE, "edits.json"), encoding="utf8"))
FLAT = json.load(open(os.path.join(TREE, "manifest.json"), encoding="utf8")) if os.path.isdir(TREE) else {}


def stored(name):
    return os.path.join(TREE, FLAT[name]["file"])


NEW_FILES = sorted(FLAT)


def path(rel):
    return os.path.join(ROOT, rel)


def sha(file):
    return hashlib.sha256(open(file, "rb").read()).hexdigest()


def edit_state(text):
    return [("new" if text.count(e["new"]) == 1 else "old" if text.count(e["old"]) == 1 else "bad") for e in SPEC["edits"]]


def status():
    text = open(path(SPEC["file"]), encoding="utf8").read()
    files = {name: os.path.isfile(path(name)) and sha(path(name)) == sha(stored(name)) for name in NEW_FILES}
    return {"edits": edit_state(text), "leaf": files}


def apply():
    file = path(SPEC["file"])
    text = open(file, encoding="utf8").read()
    states = edit_state(text)
    if "bad" in states:
        sys.exit("anchor missing or ambiguous: " + ", ".join(e["name"] for e, s in zip(SPEC["edits"], states) if s == "bad"))
    for edit, state in zip(SPEC["edits"], states):
        if state == "old":
            text = text.replace(edit["old"], edit["new"])
    for name in NEW_FILES:
        target = path(name)
        source = stored(name)
        if os.path.isfile(target) and sha(target) != sha(source):
            sys.exit("refuse to overwrite a different " + target)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(source, target)
    open(file, "w", encoding="utf8").write(text)


def revert():
    file = path(SPEC["file"])
    text = open(file, encoding="utf8").read()
    for edit in reversed(SPEC["edits"]):
        if text.count(edit["new"]) == 1:
            text = text.replace(edit["new"], edit["old"])
    open(file, "w", encoding="utf8").write(text)
    for name in NEW_FILES:
        if os.path.isfile(path(name)) and sha(path(name)) == sha(stored(name)):
            os.remove(path(name))
            folder = os.path.dirname(path(name))
            while folder != ROOT and os.path.isdir(folder) and not os.listdir(folder):
                os.rmdir(folder)
                folder = os.path.dirname(folder)


{"apply": apply, "revert": revert, "status": lambda: print(status())}[sys.argv[1]]()
