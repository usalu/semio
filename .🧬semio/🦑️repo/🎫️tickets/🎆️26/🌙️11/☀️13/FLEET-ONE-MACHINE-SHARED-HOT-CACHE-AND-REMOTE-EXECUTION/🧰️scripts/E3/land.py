import sys,json
R=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(R+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:70])
        s=s.replace(a,b)
    open(R+path,'w').write(s)
edit('🔬️action-result/🟦️.ts',[
('manifest:boolean;paths:readonly string[]}>;','manifest:boolean;paths:readonly string[];named:readonly Readonly<{path:string;hash:string}>[]}>;'),
('paths:string[]=[];let manifest=false;','paths:string[]=[],rows:{path:string;hash:string}[]=[];let manifest=false;'),
('   paths.push(path);\n','   paths.push(path);\n   if(entry!==undefined&&!volatile.includes(path))rows.push({path,hash:entry.hash});\n'),
('manifest,paths:Object.freeze(paths)});','manifest,paths:Object.freeze(paths),named:Object.freeze(rows)});'),
('`manifest` says whether such a file was present. */','`manifest` says whether such a file was present, `named` the path and digest of every other output file, so two results of one key can name the outputs that differ. */')])
edit('🧾️provenance/🟦️.ts',[
('/** ⚠️ One unit key that two origins produced differently: the first entry stays, the later write was refused. */\nexport type DivergenceV1=Readonly<{namespace:string;kind:"ac"|"cas"|"nx";key:string;keptOrigin:string;keptSha256:string;refusedOrigin:string;refusedSha256:string;atMs:number}>;',
'/** ⚠️ One key that two origins produced differently: the first entry stays, the later write was refused. `sort` is the class of result (`unit`, `project-inference`, `closure`, `other`) and `differing` the paths of the output files whose digests differ between the two results (at most 16, sorted; a path only one side names counts), so the nondeterministic output of a unit is named without fetching either result. */\nexport type DivergenceV1=Readonly<{namespace:string;kind:"ac"|"cas"|"nx";key:string;keptOrigin:string;keptSha256:string;refusedOrigin:string;refusedSha256:string;sort:string;differing:readonly string[];atMs:number}>;'),
(' diverged(row:Omit<DivergenceV1,"atMs">,sort?:string):void;',' diverged(row:Omit<DivergenceV1,"atMs">):void;\n advised(sort:string):void;'),
(' summary():Readonly<{count:number;recent:readonly DivergenceV1[];since:number;byClass:Readonly<Record<string,number>>}>;',' summary():Readonly<{count:number;recent:readonly DivergenceV1[];since:number;byClass:Readonly<Record<string,number>>;advisory:Readonly<Record<string,number>>}>;'),
('/** 🕰️ The output names','''/** 📝️ The output names whose results are notes and ledgers of one machine, not pure functions of their key: a failure note carries the writer and its time, a cost sheet its measurements, a wave of the owner pool its publisher and creation time. Two machines write them differently by design, so the server never refuses the later write (the newest note replaces the older, which also lets a fresh failure note supersede a stale one) and counts it under `advisory` instead of `byClass`. Each name maps to its class. */
export const PROVENANCE_ADVISORY_OUTPUTS_V1:Readonly<Record<string,string>>=Object.freeze({"failure.br":"failure-note","costs.br":"costs","pointer.br":"owner-pool","head.br":"owner-pool","units.br":"owner-pool","session.br":"owner-pool"});

/** 🏷️ The class of a result from the output names of both results of one key (the new write and the one held): `advisory` when a name is one of `PROVENANCE_ADVISORY_OUTPUTS_V1`, else `unit` (a unit manifest present), `project-inference` (a graph owner record), `closure` (a closure bundle) or `other`. */
export function provenanceClassV1(paths:readonly string[],manifest:boolean):Readonly<{sort:string;advisory:boolean}>{
 for(const path of paths){const sort=PROVENANCE_ADVISORY_OUTPUTS_V1[path];if(sort!==undefined)return{sort,advisory:true};}
 return{sort:manifest?"unit":paths.includes("record.br")?"project-inference":paths.some(path=>path.startsWith("closure-"))?"closure":"other",advisory:false};
}

/** 🔍️ The paths whose digests differ between two results (a path only one names counts), sorted and cut to 16. */
export function provenanceDifferingV1(kept:readonly Readonly<{path:string;hash:string}>[],refused:readonly Readonly<{path:string;hash:string}>[]):readonly string[]{
 const left=new Map(kept.map(row=>[row.path,row.hash])),right=new Map(refused.map(row=>[row.path,row.hash])),names=new Set([...left.keys(),...right.keys()]);
 return[...names].filter(name=>left.get(name)!==right.get(name)).sort().slice(0,16);
}

/** 🕰️ The output names'''),
('`<directory>/fleet/divergences.json` and','`<directory>/fleet/divergences.json` (the newest 1000 rows with the lifetime total and the lifetime count per class) and'),
(' const classes=new Map<string,number>(),opened=clock();\n let divergences:DivergenceV1[]=[],pending:string[]=[],lines=0,writing:Promise<void>=Promise.resolve(),total=0,since=0,dirty=false;',
' const classes=new Map<string,number>(),advice=new Map<string,number>();\n let divergences:DivergenceV1[]=[],pending:string[]=[],lines=0,writing:Promise<void>=Promise.resolve(),total=0,since=0,dirty=false,opened=clock(),count=0;'),
('divergences=(value as {rows:DivergenceV1[]}).rows;}catch{}','const sheetValue=value as {since:number;total:number;classes:Record<string,number>;rows:DivergenceV1[]};divergences=sheetValue.rows;count=sheetValue.total;opened=sheetValue.since;for(const [name,number] of Object.entries(sheetValue.classes))classes.set(name,number);}catch{}'),
('''  diverged(row,sort="other"){
   classes.set(sort,(classes.get(sort)??0)+1);
   const entry''','''  advised(sort){advice.set(sort,(advice.get(sort)??0)+1);},
  diverged(row){
   classes.set(row.sort,(classes.get(row.sort)??0)+1);count++;
   const entry'''),
('JSON.stringify({version:1,rows:divergences})+"\\n"','JSON.stringify({version:1,since:opened,total:count,classes:Object.fromEntries([...classes].sort(([a],[b])=>a<b?-1:1)),rows:divergences})+"\\n"'),
('summary:()=>({count:divergences.length,recent:divergences.slice(-RECENT),since:opened,byClass:Object.fromEntries([...classes].sort(([a],[b])=>a<b?-1:1))}),','summary:()=>({count,recent:divergences.slice(-RECENT),since:opened,byClass:Object.fromEntries([...classes].sort(([a],[b])=>a<b?-1:1)),advisory:Object.fromEntries([...advice].sort(([a],[b])=>a<b?-1:1))}),')])
s=open(R+'🖥️server/🟦️.ts').read()
a=s.index('    const held=await options.store.readBytes(address.namespace,"ac",address.key,limit)');b=s.index('    prior=before;\n')
assert s.count('    prior=before;\n')==1
new='''    const held=await options.store.readBytes(address.namespace,"ac",address.key,limit),before=options.provenance.of(address.namespace,"ac",address.key),expect=request.headers.get("x-semio-expect");
    if(expect!==null&&held!==undefined&&expect!==createHash("sha256").update(held).digest("hex")){counter.conflicts++;return text(409,"Conflict: the head moved since it was read");}
    let old:ReturnType<typeof parseRemoteCacheStableResultV1>|undefined;
    if(held!==undefined)try{old=parseRemoteCacheStableResultV1(held,PROVENANCE_VOLATILE_OUTPUTS_V1);}catch{}
    const sort=provenanceClassV1([...parsed.paths,...(old?.paths??[])],parsed.manifest||old?.manifest===true);
    if(held!==undefined&&expect===null&&before?.origin!==access!.origin){
     if(sort.advisory)options.provenance.advised(sort.sort);
     else{
      const kept=old===undefined?before?.fingerprint:provenanceFingerprintV1([...old.stable,...old.trees].map(row=>row.hash));
      if(kept!==undefined&&kept!==fingerprint){
       options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint,sort:sort.sort,differing:provenanceDifferingV1(old?.named??[],parsed.named)});
       counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");
      }
     }
    }
    prior=sort.advisory?undefined:before;
'''
s=s[:a]+new+s[b+len('    prior=before;\n'):]
open(R+'🖥️server/🟦️.ts','w').write(s)
edit('🖥️server/🟦️.ts',[
('import {openProvenanceV1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceFingerprintV1,','import {openProvenanceV1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceClassV1,provenanceDifferingV1,provenanceFingerprintV1,'),
('byClass?:Readonly<Record<string,number>>}>;consumption','byClass?:Readonly<Record<string,number>>;advisory?:Readonly<Record<string,number>>}>;consumption')])
p=R+'🛰️fleet/🪪️identity/🧬️schema/🔣️.json'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,a[:50]
    s=s.replace(a,b)
rep('"The cache server\'s record of a unit key that two origins produced differently: the first stays, the later one was refused. Both hashes are kept as evidence.",','"The cache server\'s record of a key that two origins produced differently: the first stays, the later one was refused. Both fingerprints are kept as evidence, with the class of the result and the output paths that differ. `total` and `classes` count every divergence since `since` (the newest 1000 rows are kept). Notes and ledgers that two machines write differently by design are no divergence.",')
rep('"required": ["version", "rows"],\n      "properties": {\n        "version": { "const": 1 },\n        "rows": {\n          "type": "array",\n          "maxItems": 1000,','"required": ["version", "since", "total", "classes", "rows"],\n      "properties": {\n        "version": { "const": 1 },\n        "since": { "$ref": "#/definitions/Milliseconds" },\n        "total": { "type": "integer", "minimum": 0 },\n        "classes": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },\n        "rows": {\n          "type": "array",\n          "maxItems": 1000,')
rep('"refusedOrigin", "refusedSha256", "atMs"],','"refusedOrigin", "refusedSha256", "sort", "differing", "atMs"],')
rep('              "atMs": { "$ref": "#/definitions/Milliseconds" }\n            }\n          }\n        }\n      }\n    },\n    "Consumption"','              "sort": { "type": "string", "description": "unit, project-inference, closure or other" },\n              "differing": { "type": "array", "maxItems": 16, "items": { "type": "string" }, "description": "The output paths whose digests differ between the kept and the refused result." },\n              "atMs": { "$ref": "#/definitions/Milliseconds" }\n            }\n          }\n        }\n      }\n    },\n    "Consumption"')
open(p,'w').write(s);json.loads(s)
