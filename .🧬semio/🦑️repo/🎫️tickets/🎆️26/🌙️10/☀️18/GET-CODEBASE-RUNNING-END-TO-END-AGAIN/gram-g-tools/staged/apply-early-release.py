#!/usr/bin/env python3
"""[gram-g] STAGED (apply after 11:30, one compile-atomic batch): the generation builder releases the inference caches right after the partial inference is shaped (before fact index patch/pack/write). One anchor in 🕰️generations/🔌️load/🟨️.mjs + new law 🕊️early-release. usage: apply-early-release.py [--write]"""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
LOAD = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load"
HERE = os.path.dirname(os.path.abspath(__file__))
old = "announceWithheldProjectsV1(host,part);return shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});}"
new = "announceWithheldProjectsV1(host,part);const shaped=shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});await host.release?.();return shaped;}"
text = open(LOAD + "/🟨️.mjs", encoding="utf8").read()
if new in text and old not in text:
    print("already applied")
else:
    if text.count(old) != 1:
        sys.exit("anchor drift in load module")
    text = text.replace(old, new)
    if "--write" in sys.argv:
        open(LOAD + "/🟨️.mjs", "w", encoding="utf8").write(text)
        os.makedirs(LOAD + "/🧪️tests/🕊️early-release", exist_ok=True)
        shutil.copy(HERE + "/files/early-release.test.ts", LOAD + "/🧪️tests/🕊️early-release/🟦️.ts")
print("written" if "--write" in sys.argv else "check ok (nothing written)")
