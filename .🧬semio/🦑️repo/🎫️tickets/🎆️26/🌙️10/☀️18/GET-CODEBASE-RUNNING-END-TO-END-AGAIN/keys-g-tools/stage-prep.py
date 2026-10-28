"""python3 stage-prep.py : copies the live precision engine directory (engine, schema, rules, tests) to C/keys-g-tools/stage/🎯️precision/
and rewrites every relative import that leaves the directory to an absolute path, so staged edits can be unit-tested before the one live batch.
Streams file by file; never touches the live tree."""
import os, re, shutil
ROOT = "/Users/ueli/Documents/semio"
LIVE = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision"
STAGE = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/keys-g-tools/stage/🎯️precision"
SPEC = re.compile(r'((?:from|import)\s*\(?\s*)(["\'])(\.{1,2}/[^"\']+)\2')
if os.path.exists(STAGE):
    shutil.rmtree(STAGE)
for here, dirs, files in os.walk(LIVE):
    dirs[:] = [d for d in dirs if d not in ("node_modules",)]
    for name in files:
        src = os.path.join(here, name)
        dst = os.path.join(STAGE, os.path.relpath(src, LIVE))
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        if not name.endswith((".ts", ".mjs")):
            shutil.copy2(src, dst)
            continue
        text = open(src, encoding="utf-8").read()
        def fix(m):
            target = os.path.normpath(os.path.join(os.path.dirname(src), m.group(3)))
            if target.startswith(LIVE + os.sep) or target == LIVE:
                return m.group(0)
            return f"{m.group(1)}{m.group(2)}{target}{m.group(2)}"
        open(dst, "w", encoding="utf-8").write(SPEC.sub(fix, text))
print("staged", STAGE)
