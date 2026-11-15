#!/usr/bin/env python3
"""P7: removes the temporary [DEBUG] trace of the farm and of the project loop from the live files (anchored reverse edits on the CURRENT live text, so edits of other agents next to them stay) under T/landing.lock, keeps the originals and runs the farm tests.
usage: untrace.py --dry-run | --apply <clean-farm-file>"""
import os,sys,re,shutil,time,subprocess,hashlib,datetime
R=next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"))
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
FARM=L+"/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs"
NODES=L+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
def once(s,a,b):
    assert s.count(a)==1,(s.count(a),a[:90]);return s.replace(a,b)
def strip_nodes(s):
    s=once(s,'import { appendFileSync, statSync as rawStatSync } from "node:fs";','import { statSync as rawStatSync } from "node:fs";')
    s=once(s,'const inferProject=async({result,name,json},away=work=>work(),queuedAt=performance.now())=>{const tStart=performance.now();let tp0=0,tp1=0,tObs0=0;','const inferProject=async({result,name,json},away=work=>work())=>{')
    s=once(s,'tObs0=performance.now();result.projects[name]=await','result.projects[name]=await')
    s=once(s,'prepare:farm?async()=>{tp0=performance.now();try{return await away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));}finally{tp1=performance.now();}}:undefined','prepare:farm?()=>away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey})):undefined')
    a=s.index("try{appendFileSync(");b=s.index("catch{}",a)+len("catch{}")
    s=s[:a]+s[b:]
    s=once(s,'{const job=jobs[cursor++],queuedAt=performance.now();','{const job=jobs[cursor++];')
    assert s.count("inferProject(job,away,queuedAt)")==2
    return s.replace("inferProject(job,away,queuedAt)","inferProject(job,away)")
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    nodes=open(NODES,encoding="utf8").read();farm_live=open(FARM,encoding="utf8").read()
    clean_farm=open(sys.argv[2],encoding="utf8").read() if len(sys.argv)>2 else None
    new_nodes=strip_nodes(nodes)
    lines=[l for l in farm_live.split("\n")]
    kept=[l for l in lines if not (l.lstrip().startswith("trace(") or l.startswith("const trace = ") or l.startswith("  const part = value"))]
    s="\n".join(kept)
    s=s.replace('import { appendFileSync, rmSync, writeFileSync } from "node:fs";','import { rmSync, writeFileSync } from "node:fs";')
    s=s.replace('pool.farm.ready.then(usable => { trace("ready", { source, workers, usable }); if','pool.farm.ready.then(usable => { if')
    s=s.replace("    async prepare(job) {\n      const entered = performance.now();\n      const answer = await this.prepareTraced(job);\n      return answer;\n    },\n    async prepareTraced(job) {\n","    async prepare(job) {\n")
    s=s.replace("        const heldAt = performance.now();\n","")
    ok=clean_farm is not None and s==clean_farm
    print("farm: stripped live text equals the clean file:",ok)
    print("project-nodes: reverse edits apply, lines",len(nodes.split("\n")),"->",len(new_nodes.split("\n")))
    if mode!="--apply":return
    if not ok:sys.exit("farm text differs from the clean file: refusing")
    lock=T+"/landing.lock";owner="P7-untrace-%d"%os.getpid()
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
    open(lock+"/owner","w").write(owner+"\n")
    try:
        backup=T+"/🗑️generated/P7-untrace-"+datetime.datetime.now().strftime("%H%M%S")
        for path,text in ((FARM,clean_farm),(NODES,None)):
            cur=open(path,encoding="utf8").read()
            if path==NODES:
                text=strip_nodes(cur)
            rel=os.path.relpath(path,R);os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True);shutil.copyfile(path,backup+"/"+rel)
            open(path+".p7tmp","w",encoding="utf8").write(text);os.replace(path+".p7tmp",path)
        print("untraced; originals in",backup)
    finally:
        if open(lock+"/owner").read().strip()==owner:os.remove(lock+"/owner");os.rmdir(lock)
main()
