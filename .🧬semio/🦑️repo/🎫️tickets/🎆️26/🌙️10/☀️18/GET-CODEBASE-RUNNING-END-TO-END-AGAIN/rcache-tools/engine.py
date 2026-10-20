# shared by overlay.py and apply.py: loads edits.py, applies edits to text, lists staged files.
import hashlib, importlib.util, os, re, sys

ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(TOOLS, "staged")
sys.dont_write_bytecode = True


def load_edits():
    spec = importlib.util.spec_from_file_location("rcache_edits", os.path.join(TOOLS, "edits.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.EDITS


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def apply_one(text, edit):
    if "old" in edit:
        count = text.count(edit["old"])
        if count != 1:
            raise ValueError(f'{edit["file"]}: anchor found {count} times: {edit["old"][:90]!r}')
        return text.replace(edit["old"], edit["new"])
    start = text.count(edit["start"])
    if start != 1:
        raise ValueError(f'{edit["file"]}: region start found {start} times: {edit["start"][:90]!r}')
    begin = text.index(edit["start"])
    end = len(text) if edit["end"] is None else text.index(edit["end"], begin + len(edit["start"]))
    return text[:begin] + edit["new"] + text[end:]


def edited_files(edits):
    order = []
    for edit in edits:
        if edit["file"] not in order:
            order.append(edit["file"])
    return order


def staged_files():
    rows = []
    for base, _dirs, names in os.walk(STAGED):
        for name in names:
            path = os.path.join(base, name)
            rows.append(os.path.relpath(path, STAGED))
    return sorted(rows)


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()
