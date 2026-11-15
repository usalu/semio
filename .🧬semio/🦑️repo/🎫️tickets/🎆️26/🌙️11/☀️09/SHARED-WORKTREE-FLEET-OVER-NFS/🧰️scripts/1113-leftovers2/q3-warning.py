#!/usr/bin/env python3
"""q3: attest: the `blobs` fleet warning (kind, schema, texts) and its test. usage: q3-warning.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:70],s.count(old))
    return s.replace(old,new)
def edit(path,fn):
    p=root+"/"+path
    s=open(p,encoding="utf8").read(); s=fn(s); open(p,"w",encoding="utf8").write(s)
edit("🩺️attest/🟦️.ts",lambda s:sub(sub(s,'"code-fallback"|"guards-unknown";detail:string}>','"code-fallback"|"guards-unknown"|"blobs";detail:string}>'),
 ''' for(const origin of origins){const fallback=attestations[origin]!.fallback;''',
 ''' for(const origin of origins){const item=attestations[origin]!,wanted=item.failures?.find(failure=>failure.kind==="blob-wanted"&&item.wallMs-failure.lastMs<blobsWarnMilliseconds);if(wanted!==undefined)warnings.push({originId:origin,kind:"blobs",detail:String(wanted.count)});}
 for(const origin of origins){const fallback=attestations[origin]!.fallback;'''))
edit("🩺️attest/🟦️.ts",lambda s:sub(s,'const capabilities=(origins:','const blobsWarnMilliseconds=120_000;\nconst capabilities=(origins:'))
edit("🩺️attest/🧬️schema/🔣️.json",lambda s:sub(s,'"code-fallback", "guards-unknown"]','"code-fallback", "guards-unknown", "blobs"]'))
edit("🔁️replica/📖️texts/🔣️.json",lambda s:sub(s,'''    "fleet.ok": {''','''    "fleet.warning.blobs": {
      "en": "  warning: {origin} cannot read {detail} blob(s) of the store (missing or damaged) and has asked the fleet to put them back",
      "de": "  Warnung: {origin} kann {detail} Blob(s) des Stores nicht lesen (fehlend oder beschädigt) und hat das Fleet gebeten, sie wiederherzustellen"
    },
    "fleet.ok": {'''))
def tests(s):
    anchor='test("the attestation of code that predates the guards is present with unknown guards'
    new='''test("a replica that cannot read blobs is named by a blobs warning while the failure is recent, and the verdict stays ok",async()=>{
 const store=temp("store"),now=1_700_000_000_000,common={version:1 as const,groupId:"g1",platform:"linux",wallMs:now,hlc:"1700000000000-000000-a1",sync:true,code:"d".repeat(64),cursors:{},files:1,manifest:"1".repeat(64),head:null,indexTree:null,view:null,queue:{incoming:0},guards:null,latency:silent};
 await writeAttestationV1(store,{...common,originId:"a1",failures:[{kind:"blob-wanted",count:3,lastMs:now-5000}]});
 await writeAttestationV1(store,{...common,originId:"a2",failures:[{kind:"blob-wanted",count:1,lastMs:now-600_000}]});
 await writeAttestationV1(store,{...common,originId:"a3"});
 const report=await fleetReportV1(store,now);
 expect(report.warnings.filter(item=>item.kind==="blobs")).toEqual([{originId:"a1",kind:"blobs",detail:"3"}]);
 expect(validator("Report")(report)).toBe(true);
});

'''
    return sub(s,anchor,new+anchor)
edit("🩺️attest/🧪️tests/🟦️.ts",tests)
print("ok")
