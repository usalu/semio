"""📓️ docs4 builder helpers: anchors are read from the document at staging, never typed, so a typo cannot create a phantom anchor. One `Doc` per repository document, one `Batch` per topic."""
import hashlib
import json
import os
import shutil

C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ROOT = C
while not os.path.isfile(os.path.join(ROOT, "AGENTS.md")):
    ROOT = os.path.dirname(ROOT)
PRODUCT = "🧰️framework/🛍️products/🦑️repo"
TOOLS = os.path.join(C, "docs4-tools")
STAGED = os.path.join(TOOLS, os.environ.get("DOCS4_STAGED", "staged"))
BATCHES = []


class Doc:
    def __init__(self, rel):
        self.rel = rel
        self.text = open(os.path.join(ROOT, rel), "rb").read().decode("utf8")
        self.sha = hashlib.sha256(self.text.encode("utf8")).hexdigest()
        self.lines = self.text.split("\n")

    def L(self, prefix):
        hits = [line for line in self.lines if line.startswith(prefix)]
        assert len(hits) == 1, (prefix[:60], len(hits))
        return hits[0]

    def LN(self, prefix, index):
        hits = [line for line in self.lines if line.startswith(prefix)]
        assert len(hits) == 2 and hits[0] != hits[1], (prefix[:60], len(hits))
        return hits[index]

    def S(self, fragment):
        assert self.text.count(fragment) == 1, (fragment[:60], self.text.count(fragment))
        return fragment


GUIDE = Doc(PRODUCT + "/📓️build-guide.md")
README = Doc(PRODUCT + "/README.md")


class Batch:
    def __init__(self, name, doc, code, order, summary):
        self.name, self.doc, self.code, self.order, self.summary = name, doc, code, order, summary
        self.edits = []
        self.needs = None
        BATCHES.append(self)

    def replace(self, ident, lang, old, new, needs=None):
        assert new not in self.doc.text and new != old, new[:60]
        if needs is None:
            self.doc.S(old)
            self.edits.append({"id": ident, "lang": lang, "kind": "replace", "old": old, "new": new})
            return
        provided = sum(edit.get("block", edit.get("new", "")).count(old) for batch in BATCHES if batch.name in needs for edit in batch.edits)
        assert provided == 1 and self.doc.text.count(old) == 0, (old[:60], provided)
        self.edits.append({"id": ident, "lang": lang, "kind": "replace", "old": old, "new": new, "prereq": True})

    def insert(self, ident, lang, anchor, block, where="after"):
        text = self.doc.text
        assert text.count(anchor) == 1, (anchor[:60], text.count(anchor))
        position = text.index(anchor)
        assert (where == "after" and text[position + len(anchor):position + len(anchor) + 1] == "\n") or (where == "before" and text[position - 1:position] == "\n"), ("anchor is not at a line edge", anchor[:60])
        assert block not in text, block[:60]
        self.edits.append({"id": ident, "lang": lang, "kind": "insert", "anchor": anchor, "where": where, "block": block})

    def write(self):
        directory = os.path.join(STAGED, self.name)
        os.makedirs(directory, exist_ok=True)
        shutil.copyfile(os.path.join(TOOLS, "engine.py"), os.path.join(directory, "stage.py"))
        spec = {"name": self.name, "code": self.code, "lane": "docs4", "guide": self.doc.rel, "baseSha256": self.doc.sha, "needs": self.needs or [], "edits": self.edits}
        with open(os.path.join(directory, "edits.json"), "w", encoding="utf8") as handle:
            json.dump(spec, handle, ensure_ascii=False, indent=1)
            handle.write("\n")
        with open(os.path.join(directory, "BASE.txt"), "w", encoding="utf8") as handle:
            handle.write("document: %s\nsha256 at staging: %s\n%d edit(s): %s\n" % (self.doc.rel, self.doc.sha, len(self.edits), ", ".join("%s[%s]" % (e["id"], e["lang"]) for e in self.edits)))
        with open(os.path.join(directory, "README.md"), "w", encoding="utf8") as handle:
            handle.write("# docs4 doc batch `%s`\n\nDescribes behaviour that is live (code: %s). %s%s\n\n```sh\npython3 -I stage.py check|apply|revert [--root DIR]\n```\n\nIdempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.\n" % (self.name, self.code, self.summary, ("  Needs the doc batches %s applied first (exit 3 until then); revert it before them." % ", ".join(self.needs)) if getattr(self, "needs", None) else ""))
        with open(os.path.join(directory, "verify.sh"), "w", encoding="utf8") as handle:
            handle.write('#!/bin/sh\n# usage: sh verify.sh <mirror root>   (laws of the whole docs4 set on a MIRROR; never the live tree)\nexec python3 -I "$(dirname "$0")/../../verify.py" "$1"\n')
