import json
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124/f/"
def rd(p):return open(p,encoding="utf8").read()
def wr(p,s):open(p,"w",encoding="utf8").write(s)
def sub(s,a,b,count=1):
    assert s.count(a)==count,(a[:70],s.count(a))
    return s.replace(a,b)
f=rd(S+"base/fresh.ts")
f=sub(f,"the answer a tool closure gave when the record was made) is put to `closureHolds`, the same question the taker asks;","the answer a tool closure gave when the record was made) is put to `closureHolds`, the same question the taker asks, and every other row (a directory listing, an existence, a real path: what the taker's `projectRowsHoldV1` probes besides the files) is put to `rowHolds` when it is given (a record that a taker rejects on a listing is stale for the pool as well, or no pool ever refreshes it and every reader infers the owner itself);")
f=sub(f,"closureHolds?:((row:Row)=>boolean)|undefined;createdMs","closureHolds?:((row:Row)=>boolean)|undefined;rowHolds?:((row:Row)=>boolean)|undefined;createdMs")
f=sub(f,"""return{fresh:stale===0,stale,checked};""","""if(input.rowHolds!==undefined)for(const row of input.rows)if(row.kind!=="file"&&row.kind!=="analysis"&&row.kind!=="closure"){checked++;if(!input.rowHolds(row))stale++;}
 return{fresh:stale===0,stale,checked};""")
wr(S+"stage/fresh.ts",f)
p=rd(S+"base/fresh.py")
p=sub(p,'''    return {"fresh": stale == 0, "stale": stale, "checked": checked}''','''    if "holds" in case:
        for row in rows:
            if row["kind"] not in ("file", "analysis", "closure"):
                checked += 1
                stale += 0 if case["holds"].get(row["kind"] + ":" + row["path"]) is True else 1
    return {"fresh": stale == 0, "stale": stale, "checked": checked}''')
wr(S+"stage/fresh.py",p)
d=json.load(open(S+"base/fresh.json",encoding="utf8"))
rows=[{"kind":"file","path":"/r/a.ts","fact":{"sha256":"a1","size":1}},{"kind":"exists","path":"/r/d","fact":True},{"kind":"readdirNames","path":"/r/dir","fact":"digest-1"},{"kind":"directories","path":"/r/dir","fact":"digest-2"}]
d["verdicts"].append({"name":"listing rows that hold keep the record fresh","rows":rows,"tree":{"/r/a.ts":"a1"},"holds":{"exists:/r/d":True,"readdirNames:/r/dir":True,"directories:/r/dir":True},"createdMs":0,"now":1000000})
d["verdicts"].append({"name":"a listing the taker rejects makes the record stale although every digest holds","rows":rows,"tree":{"/r/a.ts":"a1"},"holds":{"exists:/r/d":True,"readdirNames:/r/dir":False,"directories:/r/dir":True},"createdMs":0,"now":1000000})
d["verdicts"].append({"name":"without a decider the listing rows are not judged","rows":rows,"tree":{"/r/a.ts":"a1"},"createdMs":0,"now":1000000})
json.dump(d,open(S+"stage/fresh.json","w",encoding="utf8"),indent=1,ensure_ascii=False)
t=rd(S+"base/fresh.test.ts")
t=sub(t,"closureHolds:(row as {closures?:Record<string,boolean>}).closures===undefined?undefined:each=>(row as {closures:Record<string,boolean>}).closures[each.path]===true}))).toEqual(oracle.verdicts);","closureHolds:(row as {closures?:Record<string,boolean>}).closures===undefined?undefined:each=>(row as {closures:Record<string,boolean>}).closures[each.path]===true,rowHolds:(row as {holds?:Record<string,boolean>}).holds===undefined?undefined:each=>(row as {holds:Record<string,boolean>}).holds[each.kind+\":\"+each.path]===true}))).toEqual(oracle.verdicts);")
t=sub(t,"toEqual([true,false,false,true,true,true,false,true]);","toEqual([true,false,false,true,true,true,false,true,true,false,true]);")
t=sub(t,'import{FRESH_POLICY_V1,fileDigestV1,freshVerdictV1,observedDigestsV1,refreshStateV1}from"../🟦️.ts";','import{FRESH_POLICY_V1,fileDigestV1,freshVerdictV1,observedDigestsV1,refreshStateV1}from"../🟦️.ts";\nimport{directoryFactOfV1,observationRowFreshV1}from"../../../../👁️observation/🟨️.mjs";')
t+='''
test("a listing row is judged by the same question the taker asks: a file added to the directory makes the record stale, an unrelated edit does not",()=>{
 const root=mkdtempSync(join(tmpdir(),"fresh-listing-"));scratch.push(root);
 writeFileSync(join(root,"a.ts"),"a");writeFileSync(join(root,"b.ts"),"b");
 const rows=[{kind:"file",path:join(root,"a.ts"),fact:{sha256:fileDigestV1(join(root,"a.ts"))}},{kind:"readdirNames",path:root,fact:directoryFactOfV1("readdirNames",root)}];
 const judge=()=>freshVerdictV1({rows,hashOf:fileDigestV1,rowHolds:row=>observationRowFreshV1(row),createdMs:0,now:10*FRESH_POLICY_V1.minimumAgeMilliseconds});
 expect(judge()).toEqual({fresh:true,stale:0,checked:2});
 writeFileSync(join(root,"b.ts"),"b edited");expect(judge().fresh).toBe(true);
 writeFileSync(join(root,"c.ts"),"c");
 expect(judge()).toEqual({fresh:false,stale:1,checked:2});
 expect(freshVerdictV1({rows,hashOf:fileDigestV1,createdMs:0,now:10*FRESH_POLICY_V1.minimumAgeMilliseconds}).fresh,"no decider: the old verdict").toBe(true);
});
'''
wr(S+"stage/fresh.test.ts",t)
r=rd(S+"base/records.ts")
r=sub(r,'''/** 🚪️ The answer of the gate:''','''/** 📂️ The question a taker puts to every row that is neither a file digest nor a closure (a directory listing, an existence, a real path): the verdict of `observationRowFreshV1` on this tree; a row it cannot judge counts as fresh, never as stale. */
async function rowHoldsOfV1():Promise<((row:ObservationRowV1)=>boolean)|undefined>{
 try{const observation=await import(OBSERVATION) as {observationRowFreshV1(row:unknown):boolean};return row=>{try{return observation.observationRowFreshV1(row);}catch{return true;}};}catch{return undefined;}
}

/** 🚪️ The answer of the gate:''')
r=sub(r,"closureHolds:await closureHoldsOfV1(taken.record.observations,taken.record.image),createdMs:found.note.createdMs","closureHolds:await closureHoldsOfV1(taken.record.observations,taken.record.image),rowHolds:await rowHoldsOfV1(),createdMs:found.note.createdMs")
wr(S+"stage/records.ts",r)
pr=rd(S+"base/pool-reader.test.ts")
pr=sub(pr,'import{observationLayerHoldsV1,observationRowsHoldV1}from"../../../../👁️observation/🟨️.mjs";','import{directoryFactOfV1,observationLayerHoldsV1,observationRowFreshV1,observationRowsHoldV1}from"../../../../👁️observation/🟨️.mjs";')
pr+='''
test("a record whose directory listing changed is not stored for the pool either: the taker rejects it on the listing, so the pool refreshes it",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("listing");tree(root,"export const e=1;\\n");await walk(root);
  const dir=join(root,"libs","alpha"),listing={kind:"readdirNames",path:dir,fact:directoryFactOfV1("readdirNames",dir)},info={...infoOf(root),exportRecord:()=>({digest:sha("d"),authority:[],observations:[rowOf(root,"libs/alpha/a.ts"),listing],image})};
  const first=createProjectFleetPortV1({root,environment,owner:"list:1"})!,claim=await first.claim(info);
  expect(claim.state).toBe("granted");expect(await (claim as {publish():Promise<boolean>}).publish()).toBe(true);await first.flush();
  expect(observationRowFreshV1(listing),"the listing holds").toBe(true);
  expect((await createProjectFleetPortV1({root,environment,owner:"list:2",now:later})!.claim(info)).state,"nothing changed: stored").toBe("stored");
  writeFileSync(join(dir,"machine-local.rs"),"fn main(){}\\n");rmSync(join(root,".cache"),{recursive:true,force:true});
  expect(observationRowFreshV1(listing),"a file appeared: the taker rejects the row").toBe(false);
  const again=await createProjectFleetPortV1({root,environment,owner:"list:3",now:later})!.claim(info);
  expect(again.state,"the pool must not call stored what a taker rejects on a listing").toBe("granted");
  await (again as {abandon():Promise<void>}).abandon();
 }finally{await server.stop();}
},120000);
'''
wr(S+"stage/pool-reader.test.ts",pr)
print("ok")
