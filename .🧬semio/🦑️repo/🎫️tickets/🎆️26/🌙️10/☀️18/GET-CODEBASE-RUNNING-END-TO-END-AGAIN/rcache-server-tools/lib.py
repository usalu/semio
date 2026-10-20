# shared by apply.py and mkoverlay.py: the staged set = new files + whole-file replacements (targets.py) + anchored edits (edits.py).
import hashlib, importlib.util, json, os, sys

ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(TOOLS, "staged")
sys.dont_write_bytecode = True


def load(name):
    spec = importlib.util.spec_from_file_location("rcs_" + name, os.path.join(TOOLS, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def units(path):
    return len(path.encode("utf-16-le")) // 2


def apply_edit(text, edit):
    """returns (new_text, state) with state in applied|already; raises ValueError on anchor problems."""
    kind = edit["kind"]
    if kind == "text":
        if edit["new"] and edit["old"] in edit["new"] and text.count(edit["new"]) >= 1:
            return text, "already"
        count = text.count(edit["old"])
        if count == 1:
            return text.replace(edit["old"], edit["new"]), "applied"
        if count == 0 and edit["new"] and text.count(edit["new"]) >= 1:
            return text, "already"
        raise ValueError(f'anchor found {count} times: {edit["old"][:80]!r}')
    if kind == "region":
        start = text.count(edit["start"])
        if start != 1:
            if start == 0 and (not edit["new"] or text.count(edit["new"]) == 1):
                return text, "already"
            raise ValueError(f'region start found {start} times: {edit["start"][:80]!r}')
        begin = text.index(edit["start"])
        if edit["end"] is None:
            end = len(text)
        else:
            if text.count(edit["end"], begin + len(edit["start"])) < 1:
                raise ValueError(f'region end missing: {edit["end"][:80]!r}')
            end = text.index(edit["end"], begin + len(edit["start"]))
        return text[:begin] + edit["new"] + text[end:], "applied"
    if kind == "sub":
        count = text.count(edit["old"])
        if count == edit["count"]:
            return text.replace(edit["old"], edit["new"]), "applied"
        if count == 0 and text.count(edit["new"]) >= 1:
            return text, "already"
        raise ValueError(f'substitution expected {edit["count"]} occurrences, found {count}: {edit["old"][:80]!r}')
    if kind == "json":
        value = json.loads(text)
        value = edit["fn"](value)
        out = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
        return out, ("already" if out == text else "applied")
    raise ValueError("unknown edit kind " + kind)


def apply_edits(text, edits):
    states = []
    for edit in edits:
        text, state = apply_edit(text, edit)
        states.append(state)
    return text, states


def plan(root=ROOT):
    """returns (writes, problems, notes). writes = [(relative, new_text)]"""
    targets = load("targets")
    edits = load("edits").EDITS
    base_file = os.path.join(TOOLS, "base-hashes.json")
    base = json.load(open(base_file, encoding="utf8")) if os.path.exists(base_file) else {}
    writes, problems, notes = [], [], []
    by_file = {}
    for edit in edits:
        by_file.setdefault(edit["file"], []).append(edit)
    for relative, mine in by_file.items():
        path = os.path.join(root, relative)
        if not os.path.isfile(path):
            problems.append(f"missing target: {relative}")
            continue
        text = read(path)
        try:
            updated, states = apply_edits(text, mine)
        except ValueError as error:
            problems.append(f"{relative}: {error}")
            continue
        if all(state == "already" for state in states) or updated == text:
            notes.append(f"already applied: {relative}")
            continue
        if relative in base and base[relative] != sha(text) and "already" not in states:
            notes.append(f"drift since staging (anchors still unique, accepted): {relative}")
        writes.append((relative, updated))
    for name, relative in targets.REPLACE.items():
        path = os.path.join(root, relative)
        content = read(os.path.join(STAGED, name))
        if not os.path.isfile(path):
            problems.append(f"missing replace target: {relative}")
            continue
        text = read(path)
        if text == content:
            notes.append(f"already present: {relative}")
            continue
        if relative in base and base[relative] != sha(text):
            problems.append(f"drift (content differs from staging time): {relative}")
            continue
        writes.append((relative, content))
    for name, relative in targets.NEW.items():
        path = os.path.join(root, relative)
        content = read(os.path.join(STAGED, name))
        if os.path.exists(path):
            if read(path) == content:
                notes.append(f"already present: {relative}")
                continue
            problems.append(f"new file exists with other content: {relative}")
            continue
        if units(os.path.join(ROOT, relative)) > 256:
            problems.append(f"path exceeds 256 UTF-16 units: {relative}")
            continue
        writes.append((relative, content))
    return writes, problems, notes
