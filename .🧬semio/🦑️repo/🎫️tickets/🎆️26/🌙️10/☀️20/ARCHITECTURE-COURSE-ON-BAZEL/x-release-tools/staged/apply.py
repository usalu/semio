#!/usr/bin/env python3
"""Applies the build-mode axis patch: the Bazel adapter, the build script's dry-run plan, the README mode paragraph and the 📥️mode test suite.

Two-phase and idempotent: every target is re-read and every edit computed in memory by unique anchors first (an edit already present is skipped,
a missing or ambiguous anchor refuses and nothing is written); only then are files replaced atomically, so hard links are never written through.
Usage: apply.py [--check]            full patch (README gets the post-patch wording)
       apply.py --readme-now         only restore the README wording to what is true before the patch lands (safe at any time)
"""
import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import mode_patch as m

def read(path): return open(path,encoding="utf8").read()
def replace_atomically(path,data,binary=False):
    temporary=path+".tmp-x-release"
    with open(temporary,"wb" if binary else "w",**({} if binary else {"encoding":"utf8"})) as handle: handle.write(data)
    os.replace(temporary,path)

def main():
    check="--check" in sys.argv
    if "--readme-now" in sys.argv:
        text=read(m.README); updated=m.readme(text,"now")
        print("readme: "+("already true" if updated==text else "restored to the wording that is true before the patch"))
        if updated!=text and not check: replace_atomically(m.README,updated)
        return
    plan=[]
    adapter=read(m.ADAPTER); plan.append((m.ADAPTER,adapter,m.patched(adapter)))
    script=read(m.BUILD_SCRIPT); plan.append((m.BUILD_SCRIPT,script,m.patched(script,m.SCRIPT_EDITS)))
    readme=read(m.README); plan.append((m.README,readme,m.readme(readme,"full")))
    tests=[name for name in os.listdir(m.STAGED_TESTS) if os.path.isfile(os.path.join(m.STAGED_TESTS,name))]
    stale=[name for name in tests if not os.path.exists(os.path.join(m.TESTS,name)) or open(os.path.join(m.TESTS,name),"rb").read()!=open(os.path.join(m.STAGED_TESTS,name),"rb").read()]
    for path,before,after in plan: print(f"{os.path.relpath(path,m.REPO)[-60:]}: {'applied' if before==after else 'pending'}")
    print(f"tests to install: {len(stale)} of {len(tests)}")
    if check: return
    for path,before,after in plan:
        if before!=after: replace_atomically(path,after)
    os.makedirs(m.TESTS,exist_ok=True)
    for name in stale: replace_atomically(os.path.join(m.TESTS,name),open(os.path.join(m.STAGED_TESTS,name),"rb").read(),binary=True)
    if not (m.applied(read(m.ADAPTER)) and m.applied(read(m.BUILD_SCRIPT),m.SCRIPT_EDITS)): raise SystemExit("refused: a target does not contain the patch after applying")
    print("done")

main()
