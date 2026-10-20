#!/usr/bin/env python3
"""Adds the cache decision (metadata.semio.cache) to the three cache:false targets that peers added to the caching manifest without one (law finding `uncached-without-reason`).
usage: apply.py [--root DIR] [--write]   dry run by default; anchored per target block (each block must exist exactly once and carry a `semio` object); idempotent; all-or-nothing; temp file + rename."""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
root = args[args.index("--root") + 1] if "--root" in args else None
if root is None:
    root = HERE
    while root != os.path.dirname(root) and not (os.path.isfile(os.path.join(root, "bunfig.toml")) and os.path.isfile(os.path.join(root, "bun.lock"))):
        root = os.path.dirname(root)
PATH = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json")
DECISIONS = {
    "cache-retire-inactive": ("side-effecting", "mutates-host-store"),
    "cache-retire-inactive-preview": ("diagnostic", "measures-environment"),
    "test-cargo-retirement": ("live", "exercises-host-resources"),
}


def block(text, name):
    m = re.search(r'\n    "%s": \{\n' % re.escape(name), text)
    if not m:
        raise SystemExit("target block missing: " + name)
    end = re.search(r'\n    \},?\n', text[m.end():])
    return m.start() + 1, m.end() + end.end()


text = open(PATH, encoding="utf8").read()
out, pending = text, []
for name, (klass, reason) in DECISIONS.items():
    a, b = block(out, name)
    body = out[a:b]
    if '"cache": {' in body.split('"metadata"', 1)[-1] and '"class"' in body:
        continue
    anchor = '      "metadata": {\n        "semio": {\n'
    if body.count(anchor) != 1:
        raise SystemExit("anchor count %d in block %s" % (body.count(anchor), name))
    insert = '          "cache": {\n            "version": 1,\n            "class": "%s",\n            "reason": "%s"\n          },\n' % (klass, reason)
    out = out[:a] + body.replace(anchor, anchor + insert) + out[b:]
    pending.append(name)
document = json.loads(out)
for name, (klass, reason) in DECISIONS.items():
    decision = document["targets"][name]["metadata"]["semio"]["cache"]
    assert decision == {"version": 1, "class": klass, "reason": reason}, name
print("pending:" if pending else "applied:", ", ".join(pending or DECISIONS))
if pending and "--write" in args:
    temporary = PATH + ".cache-law.tmp"
    open(temporary, "w", encoding="utf8", newline="").write(out)
    os.replace(temporary, PATH)
    print("written")
elif pending:
    print("dry run: nothing written; pass --write")
