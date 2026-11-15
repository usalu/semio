# -*- coding: utf-8 -*-
"""R30 payload builder: payload/base = the live files now, payload/new = base with the edits below (every edit must match exactly once). usage: build_payload.py"""
import os,shutil,json
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload";S=HERE+"/src";LIVE="/Users/Shared/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/";W=B+"🪪️graph-state/🙋️wanted/";C=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/"
CG=B+"🧱️job-limits/🐧️cgroup/🟦️.ts";MJS=B+"🧠️memory/🟨️.mjs";DMT=B+"🧠️memory/🟨️.d.mts";READ=W+"📖️read/🟦️.ts";WSCH=W+"🧬️schema/🔣️.json";PRE=B+"🕸️graph/🤝️prefetch/🟦️.ts";HOST=B+"🕸️graph/🕰️host/🟦️.ts";RSCH=C+"🧬️schema/🔣️.json"
NEWTESTS={W+"🧪️tests/🐧️linux-host/🟦️.ts":"linux-host.test.ts",C+"🧪️tests/📏️roster-bound/🟦️.ts":"roster-bound.test.ts"}
def rd(p):return open(p,encoding="utf8").read()
def sub(text,old,new,count=1):
    assert text.count(old)==count,(old[:80],text.count(old));return text.replace(old,new)
CGROUP_NEW=rd(S+"/cgroup-snippet.txt")
def e_cg(t):return sub(t,"export {posix as cgroupPathV1};",CGROUP_NEW+"export {posix as cgroupPathV1};")
MEM_NEW=rd(S+"/memory-snippet.txt")
def e_mjs(t):
    t=sub(t,'import{cgroupMemoryHeadroomV1}from"../🧱️job-limits/🐧️cgroup/🟦️.ts";','import{cgroupMachineLimitBytesV1,cgroupMemoryHeadroomV1}from"../🧱️job-limits/🐧️cgroup/🟦️.ts";')
    return sub(t,"/** 🎫️ Partitions the qualified live reservation",MEM_NEW+"\n/** 🎫️ Partitions the qualified live reservation")
def e_dmt(t):return sub(t,"export function hostMemoryMegabytesV1():number;","export function hostMemoryMegabytesV1():number;\nexport type FleetHostReadingV1=Readonly<{physicalMegabytes:number;limitMegabytes:number|null;hostMegabytes:number;availableMegabytes:number|null}>;\nexport function fleetHostReadingV1(ports?:Readonly<{ports?:unknown;total?:()=>number;environment?:Readonly<Record<string,string|undefined>>;limit?:(ports?:unknown)=>number|undefined;available?:()=>number|undefined}>):FleetHostReadingV1;")
def e_read(t):
    t=sub(t,"estimate?:(name:string)=>OwnerEstimateV1;infer?","estimate?:(name:string)=>OwnerEstimateV1;readings?:Readonly<Record<string,string|number>>;infer?")
    return sub(t," input.say(heavyRest.length>0?ownerCostTextV1(\"plan\""," if(input.readings!==undefined)input.say(wantedTextV1(\"readings\",language,input.readings));\n input.say(heavyRest.length>0?ownerCostTextV1(\"plan\"")
def e_wsch(t):
    t=sub(t,'          "plan": "graph: {stale} stale projects,','          "readings": "graph: memory readings: machine {host} MB (physical {physical} MB, cgroup limit {limit}), available {available}",\n          "plan": "graph: {stale} stale projects,')
    return sub(t,'          "plan": "Graph: {stale} veraltete Projekte,','          "readings": "Graph: Speicherwerte: Maschine {host} MB (physisch {physical} MB, cgroup-Grenze {limit}), verfügbar {available}",\n          "plan": "Graph: {stale} veraltete Projekte,')
def e_pre(t):
    t=sub(t,"reading={hostMegabytes:(input.hostMegabytes??memory.hostMemoryMegabytesV1)()},","machine=input.hostMegabytes===undefined?memory.fleetHostReadingV1():undefined,reading={hostMegabytes:machine===undefined?input.hostMegabytes!():machine.hostMegabytes},")
    return sub(t,"return await fleetReadV1({stale:input.request.dirty,admittedMegabytes,grantMegabytes,","return await fleetReadV1({stale:input.request.dirty,admittedMegabytes,grantMegabytes,...machine===undefined?{}:{readings:{host:machine.hostMegabytes,physical:machine.physicalMegabytes,limit:machine.limitMegabytes===null?(input.language==\"en\"?\"none\":\"keine\"):machine.limitMegabytes+\" MB\",available:machine.availableMegabytes===null?(input.language==\"en\"?\"unknown\":\"unbekannt\"):machine.availableMegabytes+\" MB\"}},")
def e_host(t):return sub(t,"reading={hostMegabytes:memory.hostMemoryMegabytesV1()}","reading={hostMegabytes:memory.fleetHostReadingV1().hostMegabytes}")
def e_rsch(t):
    t=sub(t,'''            "names": {
              "type": "array",
              "maxItems": 16,''','''            "names": {
              "type": "array",
              "maxItems": 24,''')
    return sub(t,'''              "roles": {
                "type": "array",
                "maxItems": 16,''','''              "roles": {
                "type": "array",
                "maxItems": 24,''')
EDITS={CG:e_cg,MJS:e_mjs,DMT:e_dmt,READ:e_read,WSCH:e_wsch,PRE:e_pre,HOST:e_host,RSCH:e_rsch}
if os.path.isdir(P):shutil.rmtree(P)
names=[]
for rel,fn in EDITS.items():
    t=rd(LIVE+rel);os.makedirs(os.path.dirname(P+"/base/"+rel),exist_ok=True);os.makedirs(os.path.dirname(P+"/new/"+rel),exist_ok=True)
    open(P+"/base/"+rel,"w",encoding="utf8",newline="").write(t);open(P+"/new/"+rel,"w",encoding="utf8",newline="").write(fn(t));names.append(rel)
for rel,src in NEWTESTS.items():
    os.makedirs(os.path.dirname(P+"/new/"+rel),exist_ok=True);open(P+"/new/"+rel,"w",encoding="utf8",newline="").write(rd(S+"/"+src));names.append(rel)
open(P+"/files.txt","w",encoding="utf8").write("\n".join(names)+"\n");print(len(names),"files")
