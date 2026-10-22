#!/usr/bin/env python3
"""apply-check.py : proves apply.py on clones of the reference overlay (round 6 + perf-slo): dry run changes nothing, write applies, the second run is `already`, groups apply alone, every kind of drift is refused with exit code 2 and nothing written."""
import json,os,shutil,subprocess,sys
C=os.environ.get("TICKET","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN");APPLY=C+"/boot-fast-tools/staged/apply.py";REF="/private/tmp/boot-fast/ovref";W="/private/tmp/boot-fast/ovapply"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";LAUNCH="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/"
def clone():
    shutil.rmtree(W,ignore_errors=True);subprocess.run(["cp","-cR",REF,W],check=True)
def run(*a):
    r=subprocess.run(["python3",APPLY,"--root",W,*a],capture_output=True,text=True);return r.returncode,r.stdout+r.stderr
def tree():
    out={}
    for dp,dn,fn in os.walk(W):
        dn[:]=[d for d in dn if d not in("node_modules",)]
        for f in fn:
            p=os.path.join(dp,f)
            if not os.path.islink(p):out[p]=os.stat(p).st_mtime_ns
    return out
ok=True
def check(name,cond,detail=""):
    global ok;print(("ok   " if cond else "FAIL ")+name+(" "+detail if not cond else ""));ok=ok and cond
clone();before=tree();rc,o=run();check("dry run exits 0 and says dry run",rc==0 and "dry run" in o);check("dry run writes nothing",tree()==before)
rc,o=run("--write");check("write applies 23 changes",rc==0 and "applied" in o and "23 change(s)" in o,o[-200:])
rc,o=run();check("second run is all already",rc==0 and "0 change(s)" in o,o[-200:])
rc,o=run("--write");check("second write changes nothing",rc==0 and "0 change(s)" in o)
for group,expect in (("door",10),("validator",3),("caller",2),("taxonomy",2),("slo",3)):
    clone();rc,o=run("--write","--group",group);check(f"group {group} alone applies {expect}",rc==0 and f"{expect} change(s)" in o,o[-200:])
clone();rc,o=run("--write","--group","launcher");check("launcher alone is refused without the door script",rc==2 and "need the door script" in o)
clone();rc,o=run("--write","--group","door");rc,o=run("--write","--group","launcher");check("launcher after door applies",rc==0 and "2 change(s)" in o,o[-200:])
clone();run("--write","--group","door");rc,o=run("--write","--group","manifest");check("manifest after door applies and names the door",rc==0 and "1 change(s)" in o and "🚪️door/📜️script.ts repo" in open(W+"/package.json",encoding="utf8").read(),o[-200:])
def tamper(name,rel,change,expect,group="all"):
    clone();p=os.path.join(W,rel);t=open(p,encoding="utf8").read();n=change(t);assert n!=t,name;open(p,"w",encoding="utf8").write(n);before=tree()
    rc,o=run("--write","--group",group);check(f"drift refused: {name}",rc==2 and expect in o and tree()==before,o[-300:])
tamper("caller anchor gone",B+"📥️caller/🟦️.ts",lambda t:t.replace('import {issueRepositoryCargoExecutionBackendV1} from','import {issueRepositoryCargoExecutionBackendV2} from'),"caller: static cargo import")
tamper("caller anchor twice",B+"📥️caller/🟦️.ts",lambda t:t+'\nimport {isStoreRepairSelectionV1} from "../🧾️store-receipt/🟨️.mjs";\n',"anchor occurs 2 times")
tamper("invocation taxonomy read changed",B+"🧭️invocation/🟦️.ts",lambda t:t.replace("JSON.parse(readFileSync(join(root, ","JSON.parse(readFileSync(join(workspace, "),"invocation: taxonomy read")
tamper("launcher entry is something else",LAUNCH+"🗂️registry/🔣️.json",lambda t:t.replace("📚️library/🏗️build/📜️script.ts","📚️library/🏗️build/🚀️other.ts"),"build entry is")
tamper("launcher registry layout changed",LAUNCH+"🗂️registry/🔣️.json",lambda t:t.replace('\n "version"','\n   "version"',1) if '\n "version"' in t else t+" ","canonical JSON layout")
tamper("Go entry-contract line changed",LAUNCH+"🧪️tests/🏗️entry-contract/🐹️.go",lambda t:t.replace('"📚️library", "🏗️build", "📜️script.ts")','"📚️library", "🏗️build", "📜️other.ts")'),"launcher entry-contract test")
tamper("package.json repo script is bun form","package.json",lambda t:t.replace('"repo": "repo ./','"repo": "bun ./'),"root package.json: repo script")
tamper("closures row repo-cli missing",B+"📥️source/🛗️zero-touch/🗂️closures/🔣️.json",lambda t:t.replace('"id": "repo-cli"','"id": "repo-cli-x"'),"row repo-cli is missing")
tamper("slo table lacks cli-help-p95",B+"📏️incremental/🏎️bench/🎯️slo/🔣️.json",lambda t:t.replace('"id": "cli-help-p95"','"id": "cli-help-p96"'),"row cli-help-p95 is missing")
tamper("a create target exists with other content",B+"🚪️door/🔣️.json",lambda t:t,"x") if False else None
clone();os.makedirs(os.path.join(W,B+"🚪️door"),exist_ok=True);open(os.path.join(W,B+"🚪️door/🔣️.json"),"w").write("{}\n");before=tree();rc,o=run("--write","--group","door");check("a create target with other content is refused",rc==2 and "exists with other content" in o and tree()==before)
clone();rc,o=run("--root","/nonexistent/"+"x"*300,"--write") if False else (0,"")
print("ALL OK" if ok else "FAILED");sys.exit(0 if ok else 1)
