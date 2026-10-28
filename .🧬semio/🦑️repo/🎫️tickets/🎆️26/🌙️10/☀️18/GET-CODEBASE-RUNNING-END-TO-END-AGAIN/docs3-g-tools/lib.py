"""📓️ docs3-g builder helpers: anchors are read from the guide at staging, never typed, so a typo cannot create a phantom anchor."""
import hashlib
import json
import os
import shutil

C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ROOT = C
while not os.path.isfile(os.path.join(ROOT, "AGENTS.md")):
    ROOT = os.path.dirname(ROOT)
GUIDE = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
TEXT = open(os.path.join(ROOT, GUIDE), "rb").read().decode("utf8")
BASE = hashlib.sha256(TEXT.encode("utf8")).hexdigest()
LINES = TEXT.split("\n")
TOOLS = os.path.join(C, "docs3-g-tools")
STAGED = os.path.join(TOOLS, "staged")
EDITS = []


def L(prefix):
    hits = [line for line in LINES if line.startswith(prefix)]
    assert len(hits) == 1, (prefix[:60], len(hits))
    return hits[0]


def LN(prefix, index):
    hits = [line for line in LINES if line.startswith(prefix)]
    assert len(hits) == 2 and hits[0] != hits[1], (prefix[:60], len(hits))
    return hits[index]


def S(fragment):
    assert TEXT.count(fragment) == 1, (fragment[:60], TEXT.count(fragment))
    return fragment


class Batch:
    def __init__(self, name, code, lane, order, summary, requires=()):
        self.name, self.code, self.lane, self.order, self.summary, self.requires = name, code, lane, order, summary, tuple(requires)
        self.edits = []
        EDITS.append(self)

    def replace(self, ident, lang, old, new, prereq=False):
        if not prereq:
            S(old)
        assert new not in TEXT or new == old, new[:60]
        edit = {"id": ident, "lang": lang, "kind": "replace", "old": old, "new": new}
        if prereq:
            edit["prereq"] = True
        self.edits.append(edit)

    def insert(self, ident, lang, anchor, block, where="after", prereq=False):
        assert TEXT.count(anchor) == 1, (anchor[:60], TEXT.count(anchor))
        position = TEXT.index(anchor)
        assert (where == "after" and TEXT[position + len(anchor):position + len(anchor) + 1] == "\n") or (where == "before" and TEXT[position - 1:position] == "\n"), ("anchor is not at a line edge", anchor[:60])
        assert block not in TEXT, block[:60]
        edit = {"id": ident, "lang": lang, "kind": "insert", "anchor": anchor, "where": where, "block": block}
        if prereq:
            edit["prereq"] = True
        self.edits.append(edit)

    def write(self):
        directory = os.path.join(STAGED, self.name)
        os.makedirs(directory, exist_ok=True)
        shutil.copyfile(os.path.join(TOOLS, "engine.py"), os.path.join(directory, "stage.py"))
        spec = {"name": self.name, "code": self.code, "lane": self.lane, "guide": GUIDE, "baseSha256": BASE, "edits": self.edits}
        with open(os.path.join(directory, "edits.json"), "w", encoding="utf8") as handle:
            json.dump(spec, handle, ensure_ascii=False, indent=1)
            handle.write("\n")
        with open(os.path.join(directory, "BASE.txt"), "w", encoding="utf8") as handle:
            handle.write("guide: %s\nsha256 at staging: %s\n%d edit(s): %s\n" % (GUIDE, BASE, len(self.edits), ", ".join("%s[%s]" % (e["id"], e["lang"]) for e in self.edits)))
        with open(os.path.join(directory, "README.md"), "w", encoding="utf8") as handle:
            handle.write("# docs3-g doc batch `%s`\n\nLands WITH the code batch `%s` (%s). %s\n\n```sh\npython3 -I stage.py check|apply|revert [--root DIR]\n```\n\nIdempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.\n" % (self.name, self.code, self.lane, self.summary))
