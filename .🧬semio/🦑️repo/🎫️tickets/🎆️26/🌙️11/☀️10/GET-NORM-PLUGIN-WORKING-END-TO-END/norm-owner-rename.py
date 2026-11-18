#!/usr/bin/env python3
"""Renames the artifact identity owner segment of the moved norm families to their plugin id.

Usage: norm-owner-rename.py [--apply] ; without --apply it only lists the files it would change.
Forms rewritten per family F owned by plugin P (P_ is P with '-' as '_'):
  semio.s.norm.F -> semio.s.P_.F   (protobuf packages cannot carry hyphens, the protobuf twin maps '-' to '_')
  s.norm.F       -> s.P.F          (kind, dialect, schema, mutation, controller, activation ids)
  s/norm/F       -> s/P/F          (schema document $id paths)
"""
import os, re, sys

OWNERS = {
    "din18599": "norm-energy",
    "en1990": "norm-eurocode", "en1991": "norm-eurocode", "en1997": "norm-eurocode", "en1998": "norm-eurocode",
    "en1992": "norm-eurocode-structural", "en1993": "norm-eurocode-structural", "en1994": "norm-eurocode-structural",
    "en1995": "norm-eurocode-materials", "en1996": "norm-eurocode-materials", "en1999": "norm-eurocode-materials",
}
ROOT = "/Users/ueli/Documents/semio"
TREES = ["✏️s/🔌️plugins/📕️norm", "🌎️hub/🧩️compositions/📕️norm", "✏️s/🧑‍💻dev/🧩️catalog", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", "🏢️semio-tech", "🧪️tests", "🧫️fixtures"]
SKIP = {"node_modules", ".git", "target", ".🧾️store", "dist", "📤️dist"}
fams = "|".join(sorted(OWNERS, key=len, reverse=True))
proto = re.compile(r"semio\.s\.norm\.(%s)(?![A-Za-z0-9])" % fams)
dotted = re.compile(r"(?<![A-Za-z0-9_-])s\.norm\.(%s)(?![A-Za-z0-9])" % fams)
slashed = re.compile(r"(?<![A-Za-z0-9_-])s/norm/(%s)(?![A-Za-z0-9])" % fams)
quick = re.compile(r"s[./]norm[./](%s)" % fams)

def rewrite(text):
    text = proto.sub(lambda m: "semio.s.%s.%s" % (OWNERS[m.group(1)].replace("-", "_"), m.group(1)), text)
    text = dotted.sub(lambda m: "s.%s.%s" % (OWNERS[m.group(1)], m.group(1)), text)
    return slashed.sub(lambda m: "s/%s/%s" % (OWNERS[m.group(1)], m.group(1)), text)

apply = "--apply" in sys.argv
changed = []
for tree in TREES:
    for base, dirs, files in os.walk(os.path.join(ROOT, tree)):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for name in files:
            path = os.path.join(base, name)
            try:
                text = open(path, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError):
                continue
            if not quick.search(text):
                continue
            new = rewrite(text)
            if new != text:
                changed.append(path)
                if apply:
                    open(path, "w", encoding="utf-8").write(new)
print(("applied " if apply else "would change ") + str(len(changed)) + " files")
with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "norm-owner-rename-files.txt"), "w") as out:
    out.write("\n".join(os.path.relpath(p, ROOT) for p in changed) + "\n")
