#!/usr/bin/env python3
"""Builds batch `handshake` from the CURRENT live executor: python3 -I build.py [--root DIR]  (rewrites staged/handshake/{files,base,manifest.json})"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); STAGE = os.path.join(HERE, "staged", "handshake")
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
EX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts"
TD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️handshake/"
def sha(b): return hashlib.sha256(b).hexdigest()
text = open(os.path.join(ROOT, EX), encoding="utf8").read(); base_bytes = text.encode()
def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80]); text = text.replace(old, new)
start = text.index("/** ⏱️ The handshake clock of one action birth")
end = text.index("/** 🐣️ Obtains unchanged current authority")
text = text[:start] + open(os.path.join(HERE, "handshake.part.ts"), encoding="utf8").read() + "\n" + text[end:]
once('import {withRepositoryCacheStoreV1}', 'import {CONTENTION_POLICY_V1,budgetStretchV1,createBudgetClockV1,sharedHostContentionV1,type ContentionSourceV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🟦️.ts";\nimport {withRepositoryCacheStoreV1}')
once('started=performance.now(),initial=()=>{if(performance.now()-started>=protocol.maximumHandshakeMilliseconds)throw refuseBuildV1("cancelled","bazel.birth-handshake-deadline-exhausted");},', 'started=performance.now(),handshake=bazelBirthHandshakeV1(protocol.maximumHandshakeMilliseconds,started),initial=()=>handshake.check(),')
once(',handshake=bazelBirthHandshakeV1(protocol.maximumHandshakeMilliseconds,envelope.deadlineEpochMilliseconds,started,fail);\n', ';handshake.arm(envelope.deadlineEpochMilliseconds,fail);\n')
new_bytes = text.encode()
SC = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
sched = open(os.path.join(ROOT, SC), encoding="utf8").read(); sched_base = sched.encode()
PORT = '"🧰️framework/🔨️modules/🏃️process/📖️texts/🟦️.ts"'
if PORT not in sched:
    anchor = '"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🟦️.ts"'
    assert sched.count(anchor) == 1, sched.count(anchor)
    i = sched.index(anchor); indent = sched[sched.rindex("\n", 0, i) + 1:i]
    sched = sched.replace(anchor, anchor + ",\n" + indent + PORT)
sched_bytes = sched.encode()
rows = {EX: {"file": "executor.ts", "base": sha(base_bytes), "new": sha(new_bytes)}, SC: {"file": "scheduling.json", "base": sha(sched_base), "new": sha(sched_bytes)}}
shutil.rmtree(os.path.join(STAGE, "files"), ignore_errors=True); shutil.rmtree(os.path.join(STAGE, "base"), ignore_errors=True)
os.makedirs(os.path.join(STAGE, "files")); os.makedirs(os.path.join(STAGE, "base"))
open(os.path.join(STAGE, "files", "executor.ts"), "wb").write(new_bytes); open(os.path.join(STAGE, "base", "executor.ts"), "wb").write(base_bytes)
open(os.path.join(STAGE, "files", "scheduling.json"), "wb").write(sched_bytes); open(os.path.join(STAGE, "base", "scheduling.json"), "wb").write(sched_base)
for name, file in (("🟦️.ts", "test.ts"), ("🔣️.json", "fixture.json")):
    data = open(os.path.join(HERE, file), "rb").read(); rel = TD + name; live = os.path.join(ROOT, rel)
    old = open(live, "rb").read() if os.path.exists(live) else None
    open(os.path.join(STAGE, "files", file), "wb").write(data)
    if old is not None: open(os.path.join(STAGE, "base", file), "wb").write(old)
    rows[rel] = {"file": file, "base": sha(old) if old is not None else None, "new": sha(data)}
json.dump(rows, open(os.path.join(STAGE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=2)
print("built", {k[-40:]: (v["base"] or "none")[:8] + ">" + v["new"][:8] for k, v in rows.items()})
