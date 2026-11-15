import json,sys
B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    t=open(B+path,encoding='utf8').read()
    for a,b in pairs:
        assert t.count(a)==1,(path,a[:70],t.count(a))
        t=t.replace(a,b)
    open(B+path,'w',encoding='utf8').write(t)
L="🏡️resident/🏗️composition/🌐️host/👷️slice-pool/"
t=open(B+L+"🧱️limit/🟨️.mjs",encoding='utf8').read()
if 'sliceBookingV1' in t: sys.exit("patched")
edit(L+"🧱️limit/🟨️.mjs",[('/** 📏 The policy of one load,','''/** 📐 The memory one load books and the least it starts with, sized from this machine: what it is declared or learned to need (`declaredMegabytes` when the host's own limit sets it, else the learned peak capped by the elasticity maximum, never below the unlearned guess), capped at `hostCapBasisPoints` of the physical memory (a booking sized for a larger machine than this one is never granted: 8192 MB on a 16 GB host that also runs the replica daemon, three sidecars and the user's applications), but never below `minimumMegabytes`, the least a load starts with; a load that cannot get its booking in full starts with what is free once that covers the minimum.
 * @see ../../../../../../🌡️host-capacity/🧪️tests/📉️shrink/🟦️.ts */
export function sliceBookingV1({declaredMegabytes,learnedMegabytes=0,unlearnedMegabytes=0,maximumMegabytes=0,hostMegabytes=0,hostCapBasisPoints=10000,minimumMegabytes=0}){
 const asked=declaredMegabytes??Math.max(Math.min(maximumMegabytes||Infinity,learnedMegabytes),unlearnedMegabytes),capped=hostMegabytes>0?Math.floor(hostMegabytes*hostCapBasisPoints/10000):Infinity,wanted=Math.max(0,Math.min(asked,Math.max(capped,minimumMegabytes)));
 return{wantedMegabytes:wanted,minimumMegabytes:Math.min(minimumMegabytes,wanted)};
}

/** 📏 The policy of one load,''')])
d=open(B+L+"🧱️limit/🟨️.d.mts",encoding='utf8').read()
d+="export function sliceBookingV1(input:{declaredMegabytes?:number;learnedMegabytes?:number;unlearnedMegabytes?:number;maximumMegabytes?:number;hostMegabytes?:number;hostCapBasisPoints?:number;minimumMegabytes?:number}):{wantedMegabytes:number;minimumMegabytes:number};\n"
open(B+L+"🧱️limit/🟨️.d.mts",'w',encoding='utf8').write(d)
edit(L+"🟨️.mjs",[
('import {graphChildHeapBytesV1} from "../../../../🧠️memory/🟨️.mjs";','import {GRAPH_MEMORY_POLICY_V1,graphChildHeapBytesV1} from "../../../../🧠️memory/🟨️.mjs";\nimport {totalmem} from "node:os";'),
('import {sliceLimitPolicyV1} from "./🧱️limit/🟨️.mjs";','import {sliceBookingV1,sliceLimitPolicyV1} from "./🧱️limit/🟨️.mjs";'),
('unlearnedMegabytes=0,graceMilliseconds,','unlearnedMegabytes=0,hostMegabytes=Math.floor(totalmem()/1048576),reserveFacts=()=>({floorMegabytes:0,limitSeconds:0}),graceMilliseconds,'),
('''const reserving=async(megabytes,signal,onProgress)=>{
  const started=performance.now(),beat=setInterval(()=>{note();onProgress?.(Math.round(performance.now()-started),{phase:"reservation",done:0,total:megabytes,running:[],decision:{key:"waiting",used:megabytes,budget:Math.round((headroom().availableBytes??0)/1048576)}});},heartbeatMilliseconds);beat.unref?.();
  try{return await reserve({megabytes,signal});}finally{clearInterval(beat);}''','''const reserving=async(megabytes,minimumMegabytes,signal,onProgress)=>{
  const started=performance.now(),beat=setInterval(()=>{const facts=reserveFacts(),spare=Math.max(0,Math.round((headroom().availableBytes??0)/1048576)-facts.floorMegabytes);note();onProgress?.(Math.round(performance.now()-started),{phase:"reservation",done:0,total:megabytes,running:[],decision:{key:"reserving",used:megabytes,budget:spare,minimum:minimumMegabytes,floor:facts.floorMegabytes,limit:facts.limitSeconds}});},heartbeatMilliseconds);beat.unref?.();
  try{return await reserve({megabytes,minimumMegabytes,signal});}finally{clearInterval(beat);}'''),
('const runWorker=(request,signal,onProgress,limit,reservation)=>new Promise','const runWorker=(request,signal,onProgress,limit,reservation,booked)=>new Promise'),
('{...process.env,BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1()),','{...process.env,BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1()),...reservation?.megabytes>0&&reservation.megabytes<booked?{[GRAPH_MEMORY_POLICY_V1.budgetEnvironment]:String(Math.max(GRAPH_MEMORY_POLICY_V1.minimumBudgetMegabytes,Math.floor(reservation.megabytes*GRAPH_MEMORY_POLICY_V1.grantShare)))}:{},'),
('''     const booked=policy?.memoryMegabytes??Math.max(Math.min(elasticity.maximumMegabytes||Infinity,learned.read()),unlearnedMegabytes);
     reservation=reserve===undefined||!(booked>0)?undefined:await reserving(booked,signal,onProgress);''','''     const{wantedMegabytes:booked,minimumMegabytes}=sliceBookingV1({declaredMegabytes:policy?.memoryMegabytes,learnedMegabytes:learned.read(),unlearnedMegabytes,maximumMegabytes:elasticity.maximumMegabytes,hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,minimumMegabytes:elasticity.minimumMegabytes});
     reservation=reserve===undefined||!(booked>0)?undefined:await reserving(booked,minimumMegabytes,signal,onProgress);'''),
('return await runWorker(request,signal,onProgress,limit,owned);','return await runWorker(request,signal,onProgress,limit,owned,booked);'),
])
# resident schema
import json
p=B+"🏡️resident/🧬️schema/🔣️.json";s=json.load(open(p,encoding='utf8'))
txt=open(p,encoding='utf8').read()
assert json.dumps(s,indent=1,ensure_ascii=False)+"\n"==txt
def find(o,key):
    if isinstance(o,dict):
        if key in o: yield o
        for v in o.values(): yield from find(v,key)
    elif isinstance(o,list):
        for v in o: yield from find(v,key)
el=[x for x in find(s,"sliceHostElasticity")];assert len(el)==1
el[0]["sliceHostElasticity"]["hostCapBasisPoints"]=4000
el[0]["sliceHostElasticity"]["minimumMegabytes"]=2048
m=s["definitions"]["Policy"]["const"]["messages"]
m["en"]["host-slice-owners-reserving"]="Reading the project graph slice ({seconds} s): waiting for memory: {used} MB wanted, {minimum} MB are enough to start, {budget} MB are free above the {floor} MB this machine keeps in reserve. It starts as soon as {minimum} MB are free; if nothing else is running and they are not free after {limit} s, it stops and says what to close."
m["de"]["host-slice-owners-reserving"]="Lese den Projektgraph-Ausschnitt ({seconds} s): wartet auf Speicher: {used} MB gewünscht, {minimum} MB genügen zum Start, {budget} MB sind über den {floor} MB frei, die dieser Rechner in Reserve hält. Der Start erfolgt, sobald {minimum} MB frei sind; läuft sonst nichts und sind sie nach {limit} s nicht frei, bricht es ab und nennt, was zu schließen ist."
open(p,'w',encoding='utf8').write(json.dumps(s,indent=1,ensure_ascii=False)+"\n")
print("ok")
