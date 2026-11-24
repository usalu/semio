import sys,os
lib=sys.argv[1]
def edit(rel,pairs):
    p=os.path.join(lib,rel);s=open(p,encoding='utf8').read()
    for old,new in pairs:
        if new in s: continue
        assert s.count(old)==1,(rel,old[:60],s.count(old))
        s=s.replace(old,new)
    open(p,'w',encoding='utf8').write(s)
A="🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts"
edit(A,[
('import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";',
 'import {readRemoteCachePolicyV1} from "../../../🌐️remote-cache/🟦️.ts";\nimport type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";'),
(' let bytes=0;\n for(const [path,real] of [...spec.files]',
 ' let bytes=0;\n const entryLimit=(()=>{const server=readRemoteCachePolicyV1().server;return Math.min(server.maximumEntryBytes,Math.floor(server.maximumBytes/2));})();\n for(const real of spec.files.values()){let size=0;try{size=statSync(real).size;}catch{continue;}if(size>entryLimit)return{declined:"inputs-too-large",detail:`${real.split(/[\\\\/]/u).slice(-2).join("/")} is ${size} bytes, the content store takes at most ${entryLimit} per entry`};}\n for(const [path,real] of [...spec.files]'),
])
L="⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts"
edit(L,[
('export async function acquireSlotV1(root:string,count:number,options:Readonly<{signal?:AbortSignal;onWait?:(waitedMs:number)=>void}>={}):Promise<ClaimV1>{',
 'export async function acquireSlotV1(root:string,count:number,options:Readonly<{signal?:AbortSignal;onWait?:(waitedMs:number)=>void;since?:number}>={}):Promise<ClaimV1>{'),
('const started=Date.now(),name=String(started).padStart(15,"0")+"-"+randomBytes(4).toString("hex")+"-"+process.pid,ticket=join(tickets,name);',
 'const started=Date.now(),name=String(Math.min(started,options.since??started)).padStart(15,"0")+"-"+randomBytes(4).toString("hex")+"-"+process.pid,ticket=join(tickets,name);'),
])
S="⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts"
edit(S,[
('slot=await acquireSlotV1(settings.root,settings.slots,{onWait:waited=>event("slot-wait",null,0,waited)});',
 'slot=await acquireSlotV1(settings.root,settings.slots,{since:started,onWait:waited=>event("slot-wait",null,0,waited)});'),
('held=waitMilliseconds>0?await acquireSlotV1(settings.root,settings.slots,{signal:AbortSignal.any([controller.signal,AbortSignal.timeout(waitMilliseconds)])})',
 'held=waitMilliseconds>0?await acquireSlotV1(settings.root,settings.slots,{since:started,signal:AbortSignal.any([controller.signal,AbortSignal.timeout(waitMilliseconds)])})'),
])
