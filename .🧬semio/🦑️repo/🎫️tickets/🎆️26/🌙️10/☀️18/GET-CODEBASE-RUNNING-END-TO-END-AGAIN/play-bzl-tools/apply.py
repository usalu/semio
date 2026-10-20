#!/usr/bin/env python3
"""Applies the play-bzl manifest edits: --dry-run (default) or --write. All-or-nothing, anchored per target block, refuses on drift."""
import hashlib, json, re, sys, os
ROOT = "/Users/ueli/Documents/semio"
PLAY = "🏢️semio-tech/🎡️play/📋️project.json"
SDEV = "✏️s/🧑‍💻dev/📋️project.json"
CACHING = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json"
DRIVER = '"cache": { "version": 1, "class": "driver-owned", "reason": "freshness-owned-by-driver" }'
def decision(reason): return '"cache": { "version": 1, "class": "deterministic", "reason": "%s" }' % reason
def block(text, name):
    m = re.search(r'\n    "%s": \{\n' % re.escape(name), text)
    if not m: raise SystemExit("target block missing: " + name)
    end = re.search(r'\n    \},?\n', text[m.end():])
    return m.start() + 1, m.end() + end.end()
def sub_in_block(text, name, old, new):
    a, b = block(text, name)
    body = text[a:b]
    if body.count(old) != 1: raise SystemExit("anchor %r occurs %d times in block %s" % (old, body.count(old), name))
    return text[:a] + body.replace(old, new) + text[b:]
def play(text):
    text = sub_in_block(text, "prepare-prototype", '"cache": false,', '"cache": true,')
    text = sub_in_block(text, "prepare-prototype", DRIVER, decision("check-without-outputs"))
    text = sub_in_block(text, "activate-prototype", '"cache": false,', '"cache": true,')
    text = sub_in_block(text, "activate-prototype", DRIVER, decision("pure-function-of-inputs"))
    return text
def sdev(text):
    return sub_in_block(text, "generate", '      "cache": true,\n', '      "cache": true,\n      "inputs": [\n        "sharedGlobals"\n      ],\n')
def caching(text):
    text = sub_in_block(text, "generator-inputs", '      "cache": true,\n', '      "cache": false,\n')
    return sub_in_block(text, "generator-inputs", '        "command": "bun \\"./🔏️inputs/📜️script.ts\\" registry-catalog"\n      }\n', '        "command": "bun \\"./🔏️inputs/📜️script.ts\\" registry-catalog"\n      },\n      "metadata": {\n        "semio": {\n          "cache": { "version": 1, "class": "diagnostic", "reason": "measures-environment" }\n        }\n      }\n')
EDITS = [(PLAY, play), (SDEV, sdev), (CACHING, caching)]
def main():
    write = "--write" in sys.argv
    bases = json.load(open(os.path.join(os.path.dirname(__file__), "bases.json"))) if os.path.exists(os.path.join(os.path.dirname(__file__), "bases.json")) else {}
    out = {}
    for path, fn in EDITS:
        text = open(os.path.join(ROOT, path), encoding="utf8").read()
        sha = hashlib.sha256(text.encode()).hexdigest()
        if bases.get(path) and bases[path] != sha and "--accept-drift" not in sys.argv:
            raise SystemExit("drift: %s base %s now %s (rerun with --accept-drift if anchors still hold)" % (path, bases[path][:12], sha[:12]))
        new = fn(text)
        json.loads(new)
        out[path] = (text, new)
    for path, (old, new) in out.items():
        print(("write " if write else "dry-run ") + path, "bytes %d -> %d" % (len(old.encode()), len(new.encode())))
    if write:
        for path, (old, new) in out.items():
            open(os.path.join(ROOT, path), "w", encoding="utf8").write(new)
main()
