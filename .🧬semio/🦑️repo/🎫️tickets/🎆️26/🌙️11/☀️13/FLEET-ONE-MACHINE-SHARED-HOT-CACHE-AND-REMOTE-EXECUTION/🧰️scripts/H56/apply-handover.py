#!/usr/bin/env python3
# H56 (4): the graph service hands over without waiting on idle old-generation holders.
# usage: DRY=1 apply-handover.py <build-root>   (the folder holding 🏡️resident/ and ⌨️command/)
import sys,os
R=sys.argv[1]
MEM={}
def rep(path,old,new,count=1):
    p=os.path.join(R,path)
    if p not in MEM: MEM[p]=open(p,encoding="utf-8").read()
    assert MEM[p].count(old)==count,(path,old[:70],MEM[p].count(old))
    MEM[p]=MEM[p].replace(old,new)
def flush():
    if os.environ.get("DRY")=="1": print("dry run: all anchors found"); return
    for p,v in MEM.items():
        open(p+".tmp","w",encoding="utf-8").write(v); os.replace(p+".tmp",p)
H="🏡️resident/🏛️service/🌐️host/🟨️.mjs"
rep(H,'import {processPidExistsV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs";','import {ownProcessIdentityV1,processHolderAliveV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs";')
rep(H,'const alive=pid=>processPidExistsV1(pid);','const alive=(pid,startMs)=>processHolderAliveV1({pid,startMs:Number.isSafeInteger(startMs)&&startMs>0?startMs:0});\nconst ownStartMsV1=()=>{try{return ownProcessIdentityV1().startMs;}catch{return 0;}};')
rep(H,'if(other.startedAt<since&&other.busy>0&&exists(other.pid))return{pid:other.pid,busy:other.busy};','if(other.startedAt<since&&other.busy>0&&exists(other.pid,other.startMs))return{pid:other.pid,busy:other.busy};')
rep(H,'if(other.startedAt>startedAt&&!other.stopping&&alive(other.pid))return true;','if(other.startedAt>startedAt&&!other.stopping&&alive(other.pid,other.startMs))return true;')
rep(H,'JSON.stringify({v:1,pid:process.pid,instance,startedAt,endpoint:runtime.endpoint,','JSON.stringify({v:1,pid:process.pid,startMs:ownStartMs,instance,startedAt,endpoint:runtime.endpoint,')
rep(H,' let lastBusy=0;\n',' let lastBusy=0;const ownStartMs=ownStartMsV1();\n')
rep(H,'export const supersededDrainDueV1=({supersededSinceMs,nowMs,lastProgressMs,connections,inFlight,graceMilliseconds,stallMilliseconds})=>supersededSinceMs!==undefined&&nowMs-supersededSinceMs>=graceMilliseconds&&(connections===0&&inFlight===0||inFlight>0&&nowMs-Math.max(supersededSinceMs,lastProgressMs)>=stallMilliseconds);',
'export const supersededDrainDueV1=({supersededSinceMs,nowMs,lastProgressMs,lastActivityMs=nowMs,connections,inFlight,graceMilliseconds,stallMilliseconds,idleConnectionMilliseconds=Infinity})=>supersededSinceMs!==undefined&&nowMs-supersededSinceMs>=graceMilliseconds&&(connections===0&&inFlight===0||connections>0&&inFlight===0&&nowMs-Math.max(supersededSinceMs,lastActivityMs)>=idleConnectionMilliseconds||inFlight>0&&nowMs-Math.max(supersededSinceMs,lastProgressMs)>=stallMilliseconds);')
rep(H,'lastProgressMs:lastProgressAt,connections:connections.size,inFlight:governor.inFlight,graceMilliseconds:policy.supersededGraceMilliseconds,stallMilliseconds:policy.reload.drainStallMilliseconds}))','lastProgressMs:lastProgressAt,lastActivityMs:lastActivity,connections:connections.size,inFlight:governor.inFlight,graceMilliseconds:policy.supersededGraceMilliseconds,stallMilliseconds:policy.reload.drainStallMilliseconds,idleConnectionMilliseconds:policy.supersededIdleConnectionMilliseconds}))')
rep(H,'/** 🔀️ Whether a service that a newer one superseded (since `supersededSinceMs`, debounced by `graceMilliseconds`) ends now: once idle (no connection, no plan in flight),','/** 🔀️ Whether a service that a newer one superseded (since `supersededSinceMs`, debounced by `graceMilliseconds`) ends now: once idle (no connection, no plan in flight; a connection without a plan and without traffic for `idleConnectionMilliseconds` does not hold it either, so an idle holder never blocks the hand-over),')
D="🏡️resident/🏛️service/🌐️host/🟨️.d.mts"
rep(D,'export function olderBusyServiceV1(options:{directory:string;tag:string;since:number;alive?:(pid:number)=>boolean;','export function olderBusyServiceV1(options:{directory:string;tag:string;since:number;alive?:(pid:number,startMs?:number)=>boolean;')
rep(D,'export function supersededDrainDueV1(input:{supersededSinceMs:number|undefined;nowMs:number;lastProgressMs:number;connections:number;inFlight:number;graceMilliseconds:number;stallMilliseconds:number}):boolean;','export function supersededDrainDueV1(input:{supersededSinceMs:number|undefined;nowMs:number;lastProgressMs:number;lastActivityMs?:number;connections:number;inFlight:number;graceMilliseconds:number;stallMilliseconds:number;idleConnectionMilliseconds?:number}):boolean;')
S="🏡️resident/🧬️schema/🔣️.json"
rep(S,'    "supersededGraceMilliseconds": 60000,\n','    "supersededGraceMilliseconds": 60000,\n    "supersededIdleConnectionMilliseconds": 300000,\n    "predecessorWaitMilliseconds": 60000,\n')
P="🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
rep(P,'predecessor=()=>undefined,','predecessor=()=>undefined,predecessorBoundMilliseconds=Infinity,')
rep(P,'   for(let older=predecessor();older!==undefined&&handing===undefined;older=predecessor()){','   for(let older=predecessor(),waitedSince=Date.now();older!==undefined&&handing===undefined&&Date.now()-waitedSince<predecessorBoundMilliseconds;older=predecessor()){')
PD="🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.d.mts"
rep(PD,'predecessor?:()=>Readonly<{pid:number;busy:number}>|undefined;','predecessor?:()=>Readonly<{pid:number;busy:number}>|undefined;predecessorBoundMilliseconds?:number;')
C="⌨️command/🌐️host/🟦️.ts"
rep(C,'slicePool=createSlicePoolV1({predecessor:()=>olderBusy({directory:residentRuntime.directory,tag:residentRuntime.tag,since:born}),','slicePool=createSlicePoolV1({predecessor:()=>olderBusy({directory:residentRuntime.directory,tag:residentRuntime.tag,since:born}),predecessorBoundMilliseconds:Number((await import("../../🏡️resident/🟨️.mjs")).HOST_POLICY_V1.predecessorWaitMilliseconds),')
flush()
print("ok",len(MEM),"files")
