#!/usr/bin/env python3
"""R2: writes edits.json (anchored literal edits) and baseline.json (sha256 of the live files they apply to)."""
import json,hashlib,os,sys
HERE=os.path.dirname(os.path.abspath(__file__));P=os.path.dirname(HERE);sys.path.insert(0,P)
import payload
LIVE=next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md"))
L=payload.LIB
SERVICE=L+"/🏡️resident/🏛️service/🌐️host/🟨️.mjs"
COMPOSE=L+"/🏡️resident/🏗️composition/🌐️host/🟨️.mjs"
HOST=L+"/🕸️graph/🕰️host/🟦️.ts"
INPUTS=L+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
E=[]
def lit(path,old,new,count=1):E.append({"path":path,"kind":"literal","old":old,"new":new,"count":count})
lit(SERVICE," function startJob(key,workspace,frame,indexKey){"," const settleAfterJobV1=async()=>{if(jobs.size>0||governor.inFlight>0)return;try{await ports.settle?.();}catch{}};\n function startJob(key,workspace,frame,indexKey){")
lit(SERVICE,"job.settled=run(job,workspace,frame,progress,indexKey).then(()=>finish({}),error=>finish({error}));","job.settled=run(job,workspace,frame,progress,indexKey).then(()=>finish({}),error=>finish({error})).then(settleAfterJobV1);")
lit(COMPOSE,'import {graphServiceHeapBytesV1,GRAPH_MEMORY_POLICY_V1} from "../../../🧠️memory/🟨️.mjs";','import {graphServiceHeapBytesV1,GRAPH_MEMORY_POLICY_V1} from "../../../🧠️memory/🟨️.mjs";\nimport {settleGraphMemoryV1} from "../../../🧠️memory/🧽️settle/🟨️.mjs";')
lit(COMPOSE,"...(admission?{admission}:{}),workers:{memory:workerMemory,active:workerBusy}","...(admission?{admission}:{}),settle:()=>settleGraphMemoryV1(),workers:{memory:workerMemory,active:workerBusy}")
lit(HOST,"graphStringInternerV1.clear();createMemoryPortsV1().collect();},\n    releaseCutoff:async","graphStringInternerV1.clear();createMemoryPortsV1().collect();if(options?.more===true)(await import(\"../../🧠️memory/🧽️settle/🟨️.mjs\")).reportRetainedGraphMemoryV1();},\n    releaseCutoff:async")
lit(INPUTS,"const walkFactsByDigestV1 = new Map(), walkMembersByDigestV1 = new Map(), layerListsV1 = new WeakMap();",'const walkFactsByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-facts", maxEntries: 65536, maxBytes: 65536 }), walkMembersByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-members", maxEntries: 65536, maxBytes: 65536 }), layerListsV1 = new WeakMap();')
lit(INPUTS,"closureStoresV1.clear(); closureCountsV1.derived","closureStoresV1.clear(); walkFactsByDigestV1.clear(); walkMembersByDigestV1.clear(); closureCountsV1.derived")
json.dump(E,open(P+"/edits.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
base={}
for p in sorted({e["path"] for e in E}):base[p]=hashlib.sha256(open(LIVE+"/"+p,"rb").read()).hexdigest()
json.dump(base,open(P+"/baseline.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(E),"edits,",len(base),"files")
