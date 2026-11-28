#!/usr/bin/env python3
# H56 (1)+(2): a selected read whose own memo is missing or stale lends the owner facts of other selections' memos.
# usage: DRY=1 apply-seed.py <build-root> <staged-root>
import sys,os,shutil
R=sys.argv[1]; S=sys.argv[2]
MEM={}
def rep(path,old,new,count=1):
    p=os.path.join(R,path)
    if p not in MEM: MEM[p]=open(p,encoding="utf-8").read()
    assert MEM[p].count(old)==count,(path,old[:70],MEM[p].count(old))
    MEM[p]=MEM[p].replace(old,new)
L="🕸️graph/📥️load/🟦️.ts"
rep(L,'import {readAdmissionMemoV1,writeAdmissionMemoV1,packAdmissionMemoV1,ownersOfAdmissionMemoV1} from "./🧠️admission-memo/🟦️.ts";','import {readAdmissionMemoV1,writeAdmissionMemoV1,packAdmissionMemoV1,ownersOfAdmissionMemoV1} from "./🧠️admission-memo/🟦️.ts";\nimport {seedFromSiblingMemosV1} from "./🌱️sibling-seed/🟦️.ts";')
rep(L,'    let journal:JournalCleanV1|undefined,journalMemo:string|undefined;','    let journal:JournalCleanV1|undefined,journalMemo:string|undefined;const usedSuffixes=new Set<string>();')
rep(L,'    if(remembered===undefined)lent=undefined;','    if(remembered===undefined)lent=undefined;\n    if(remembered===undefined&&selected&&scope&&!followUp){report.begin("memo-seed");try{const seeded=await seedFromSiblingMemosV1({directory:dirname(path),scope,format:loadedGraphAdmissionFormatV1,known:new Set(selectedAdmissionFactsV1.suffixes(scope)),seed:(suffix,facts,rows)=>selectedAdmissionFactsV1.seed(scope,suffix,facts,rows),signal:invocation?.control.signal});process.stderr.write("[graph-sibling-seed] "+JSON.stringify({version:1,...seeded})+"\\n");}catch(error){if(invocation?.control.signal.aborted===true)throw error;}}')
rep(L,'lookup=report.ownerLookup(owner.name,key);','lookup=report.ownerLookup(owner.name,key);usedSuffixes.add(suffix);')
rep(L,'owners:selectedAdmissionFactsV1.entries(scope)})','owners:selectedAdmissionFactsV1.entries(scope).filter(entry=>usedSuffixes.has(entry.suffix))})')
if os.environ.get("DRY")=="1":
    print("dry run: all anchors found")
else:
    for p,v in MEM.items():
        open(p+".tmp","w",encoding="utf-8").write(v); os.replace(p+".tmp",p)
    dest=os.path.join(R,"🕸️graph/📥️load/🌱️sibling-seed")
    for root,dirs,files in os.walk(os.path.join(S,"🌱️sibling-seed")):
        for name in files:
            rel=os.path.relpath(os.path.join(root,name),os.path.join(S,"🌱️sibling-seed"))
            target=os.path.join(dest,rel); os.makedirs(os.path.dirname(target),exist_ok=True)
            shutil.copyfile(os.path.join(root,name),target+".tmp"); os.replace(target+".tmp",target)
    print("ok")
