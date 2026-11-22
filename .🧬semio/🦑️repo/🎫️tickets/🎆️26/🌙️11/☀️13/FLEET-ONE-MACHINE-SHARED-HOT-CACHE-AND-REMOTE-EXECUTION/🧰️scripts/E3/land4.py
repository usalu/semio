import sys,json
B=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:80],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
RC=B+'🌐️remote-cache/'
edit(RC+'🧾️provenance/🗂️evidence/🟦️.ts',[
('export type EvidenceV1=Readonly<{id:string;atMs:number;sort:string;','export type EvidenceV1=Readonly<{id:string;atMs:number;outcome:"refused"|"superseded";sort:string;'),
('/** 🗂️ What the server keeps of one refused write so that both sides can be compared later: who, what class, which outputs differ, and the files. */','/** 🗂️ What the server keeps of one write that met another result for its key, so that both sides can be compared later: who, what class of result, which outputs differ, and the files. `outcome` says which result the server serves: `refused` keeps the first result (the `kept` side) and refuses the new one, `superseded` replaces the first by the new one (the `refused` side, the class of graph records where the newest tree state wins). */'),
('export type EvidenceInputV1=Readonly<{sort:string;','export type EvidenceInputV1=Readonly<{outcome:"refused"|"superseded";sort:string;'),
('const entry:EvidenceV1={id,atMs,sort:input.sort,','const entry:EvidenceV1={id,atMs,outcome:input.outcome,sort:input.sort,')])
edit(RC+'🧾️provenance/🟦️.ts',[
(' advised(sort:string):void;\n',' advised(sort:string):void;\n superseded(sort:string):void;\n'),
('advisory:Readonly<Record<string,number>>;evidence:','advisory:Readonly<Record<string,number>>;superseded:Readonly<Record<string,number>>;evidence:'),
('advice=new Map<string,number>();','advice=new Map<string,number>(),replaced=new Map<string,number>();'),
('  advised(sort){advice.set(sort,(advice.get(sort)??0)+1);},','  advised(sort){advice.set(sort,(advice.get(sort)??0)+1);},\n  superseded(sort){replaced.set(sort,(replaced.get(sort)??0)+1);},'),
('advisory:Object.fromEntries([...advice].sort(([a],[b])=>a<b?-1:1)),evidence:','advisory:Object.fromEntries([...advice].sort(([a],[b])=>a<b?-1:1)),superseded:Object.fromEntries([...replaced].sort(([a],[b])=>a<b?-1:1)),evidence:'),
('/** 🔍️ The paths whose digests differ','/** 🔁️ The classes of result where the newest write replaces the older one although the two differ: a graph project record is valid for the tree its writer saw, so with a tree that changes under the fleet the first record is the stale one. The server keeps the replaced result as evidence and counts it under `superseded`, never as a divergence. Units, closures and the waves of the owner pool stay first-writer-wins. */\nexport const PROVENANCE_SUPERSEDING_CLASSES_V1:readonly string[]=Object.freeze(["project-inference"]);\n\n/** 🔍️ The paths whose digests differ')])
edit(RC+'🖥️server/🟦️.ts',[
('import {openProvenanceV1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceClassV1,','import {openProvenanceV1,PROVENANCE_SUPERSEDING_CLASSES_V1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceClassV1,'),
('advisory?:Readonly<Record<string,number>>;evidence?:','advisory?:Readonly<Record<string,number>>;superseded?:Readonly<Record<string,number>>;evidence?:'),
('''       options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint,sort:sort.sort,differing:provenanceDifferingV1(old?.named??[],parsed.named)});
       const refusedRows=parsed.named,keptRows=old?.named??[],differing=provenanceDifferingV1(keptRows,refusedRows),reader=blobs(address.namespace);
       void options.provenance.evidence.keep({sort:sort.sort,''','''       const refusedRows=parsed.named,keptRows=old?.named??[],differing=provenanceDifferingV1(keptRows,refusedRows),reader=blobs(address.namespace);
       superseding=PROVENANCE_SUPERSEDING_CLASSES_V1.includes(sort.sort);
       if(!superseding)options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint,sort:sort.sort,differing});
       else options.provenance.superseded(sort.sort);
       void options.provenance.evidence.keep({outcome:superseding?"superseded":"refused",sort:sort.sort,'''),
('''       counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");
      }
     }
    }
    prior=sort.advisory?undefined:before;''','''       if(!superseding){counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");}
      }
     }
    }
    prior=sort.advisory||superseding?undefined:before;'''),
('    let old:ReturnType<typeof parseRemoteCacheStableResultV1>|undefined;','    let old:ReturnType<typeof parseRemoteCacheStableResultV1>|undefined,superseding=false;')])
p=B+'🏃️execution/🧬️schema/🌐️remote-cache.json'
edit(p,[
('            "advisory": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },\n','            "advisory": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },\n            "superseded": { "description": "Writes that replaced a different result of the same key because the newest result of that class wins (graph project records), by class; the replaced result is kept as evidence.", "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },\n'),
('      "required": ["id", "atMs", "sort", "namespace", "key", "keptOrigin", "refusedOrigin", "differing", "files"],\n      "properties": {\n        "id": { "type": "string", "pattern": "^[0-9A-Za-z._-]{1,160}$" },\n        "atMs": { "type": "integer", "minimum": 0 },','      "required": ["id", "atMs", "outcome", "sort", "namespace", "key", "keptOrigin", "refusedOrigin", "differing", "files"],\n      "properties": {\n        "id": { "type": "string", "pattern": "^[0-9A-Za-z._-]{1,160}$" },\n        "atMs": { "type": "integer", "minimum": 0 },\n        "outcome": { "description": "refused: the server serves the kept side and refused the other; superseded: the server serves the refused side, which replaced the kept one.", "enum": ["refused", "superseded"] },')])
# tests
edit(RC+'🧾️provenance/🧪️tests/🟦️.ts',[('const input=(key:string,sort:string)=>({sort,namespace','const input=(key:string,sort:string)=>({outcome:"refused" as const,sort,namespace')])
edit(RC+'🧾️provenance/🧪️tests/🧮️by-class/🟦️.ts',[
('expect(status.divergences.count).toBe(5);expect(status.divergences.byClass).toEqual({closure:1,other:1,"project-inference":2,unit:1});','expect(status.divergences.count).toBe(3);expect(status.divergences.byClass).toEqual({closure:1,other:1,unit:1});expect(status.divergences.superseded).toEqual({"project-inference":2});'),
('const status=await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;since:number;byClass:Record<string,number>}};','const status=await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;since:number;byClass:Record<string,number>;superseded:Record<string,number>}};'),
('  expect((await put(b,key,named(paths.filter(path=>path!=="unit.json"),"second"))).status,label).toBe(409);','  expect((await put(b,key,named(paths.filter(path=>path!=="unit.json"),"second"))).status,label).toBe(label.startsWith("record")?200:409);')])
with open(RC+'🧾️provenance/🧪️tests/🧮️by-class/🟦️.ts','a') as f:f.write(open(__import__('os').path.dirname(__import__('os').path.abspath(__file__))+'/superseded-test.ts').read())
print("ok")
