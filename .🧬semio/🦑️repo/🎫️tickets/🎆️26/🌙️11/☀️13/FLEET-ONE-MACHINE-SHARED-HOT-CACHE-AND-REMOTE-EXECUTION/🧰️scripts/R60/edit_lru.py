D="work/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/♻️lru/"
t=open(D+"🟦️.ts",encoding="utf8").read()
# 1. first own-group test: expectations of the two-tier pass
old1=t[t.index('test("the own group retires its least recently written units, header before packs'):t.index('test("the retired units and the remaining bytes of one group equal')]
new1='''test("the own group sheds the accelerator packs of its least recently written units first, keeps every record while that is enough, keeps shared observation blocks and protected units, and compacts its index",async()=>{
 const now=Date.now(),group="selections/"+"9".repeat(64),rows=[
  {name:record(1),bytes:100,ageMs:30*HOUR},{name:pack(1,"pack"),bytes:4000,ageMs:30*HOUR},{name:pack(1,"idx"),bytes:200,ageMs:30*HOUR},
  {name:record(2),bytes:100,ageMs:20*HOUR},{name:pack(2,"pack"),bytes:3000,ageMs:25*HOUR},
  {name:record(3),bytes:100,ageMs:60000},{name:pack(3,"pack"),bytes:5000,ageMs:60000},
  {name:leaf(4,1),bytes:700,ageMs:40*HOUR},{name:leaf(5,3),bytes:300,ageMs:10*HOUR}],root=agedImage(group,rows,now);
 const total=rows.reduce((sum,row)=>sum+row.bytes,0),before=files(root);
 const plan=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:6600,otherBytes:500,dryRun:true,now});
 expect(plan).toMatchObject({usedBytes:total+500,shedUnits:3,shedBytes:7500,retiredEntries:0,retiredFiles:0,protected:0,remainingBytes:6500});expect(files(root)).toEqual(before);
 const report=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:6600,otherBytes:500,now});
 expect(report).toMatchObject({shedUnits:3,shedBytes:7500,retiredEntries:0,retiredFiles:4,retiredBytes:0,protected:0,cancelled:false});
 const left=files(root).map(name=>name.slice(("inference/"+group+"/").length));
 for(const gone of[pack(1,"pack"),pack(1,"idx"),pack(2,"pack"),leaf(5,3)])expect(left.includes(gone)).toBe(false);
 for(const kept of[record(1),record(2),record(3),pack(3,"pack"),leaf(4,1)])expect(left.includes(kept)).toBe(true);
 const index=parseMemberIndexV1(readFileSync(join(root,"inference",...group.split("/"),"members.jsonl"),"utf8"),group);expect(index.valid).toBe(true);expect(index.rows.size).toBe(5);
 const within=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:1000000,now});expect(within).toMatchObject({shedUnits:0,retiredEntries:0,retiredFiles:0});
 let calls=0;const cut=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(()=>calls++>0),{targetBytes:0,now});expect(cut.cancelled).toBe(true);
 await expect(scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:-1})).rejects.toThrow("exact-optional-graph-target-bytes-required");
});

test("a finished owner record outlives the packs of every owner: a cache that cannot hold the packs keeps all headers, and only a cache that cannot hold the headers retires the oldest of them",async()=>{
 const now=Date.now(),group="selections/"+"8".repeat(64),count=40,rows=Array.from({length:count},(_,index)=>[{name:record(index+1),bytes:7000,ageMs:(100-index)*HOUR},{name:pack(index+1,"pack"),bytes:300000,ageMs:(100-index)*HOUR},{name:pack(index+1,"idx"),bytes:20000,ageMs:(100-index)*HOUR}]).flat(),root=agedImage(group,rows,now);
 const headers=count*7000,packs=count*320000,target=headers+1000000;
 const report=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:target,now});
 expect(report.retiredEntries).toBe(0);expect(report.remainingBytes).toBeLessThanOrEqual(target);expect(report.shedBytes).toBe(headers+packs-report.remainingBytes);
 const left=new Set(files(root).map(name=>name.slice(("inference/"+group+"/").length)));
 for(let index=1;index<=count;index++)expect(left.has(record(index)),"record "+index).toBe(true);
 expect(left.has(pack(1,"pack"))).toBe(false);expect(left.has(pack(count,"pack"))).toBe(true);
 const tight=await scan.retireBuildGraphOptionalEntriesLruV1(root,group,control(),{targetBytes:headers-3*7000,now});
 expect(tight.retiredEntries).toBe(4);expect(tight.remainingBytes).toBeLessThanOrEqual(headers-3*7000);
 const last=new Set(files(root).map(name=>name.slice(("inference/"+group+"/").length)));
 for(let index=1;index<=4;index++)expect(last.has(record(index)),"oldest record "+index).toBe(false);for(let index=5;index<=count;index++)expect(last.has(record(index)),"record "+index).toBe(true);
});

'''
t=t.replace(old1,new1)
# 2. seeded entries test: classification
old2=t[t.index('  const left=new Set(files(root).map(name=>name.slice(("inference/"+group+"/").length))),unit=(name:string)=>scan.optionalEntryOfV1(name)'):t.index(' const file=join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"lru-entries-"')]
new2='''  const left=new Set(files(root).map(name=>name.slice(("inference/"+group+"/").length))),unit=(name:string)=>scan.optionalEntryOfV1(name),lost=rows.filter(row=>unit(row.name)!==undefined&&!left.has(row.name)),keys=(list:readonly{name:string}[])=>[...new Set(list.map(row=>unit(row.name)!))].sort(),shed=keys(lost.filter(row=>scan.optionalAcceleratorV1(row.name))),retired=keys(lost.filter(row=>!scan.optionalAcceleratorV1(row.name)));
  cases.push({targetBytes,otherBytes,now,protectedMs:600000,leaves:rows.map(row=>({name:row.name,bytes:row.bytes,newest:now-row.ageMs}))});
  actual.push({shed,retired,remainingBytes:report.remainingBytes});rmSync(root,{recursive:true,force:true});
 }
'''
t=t.replace(old2,new2)
t=t.replace('test("the retired units and the remaining bytes of one group equal the independent Python model on 120 seeded groups"','test("the shed units, the retired units and the remaining bytes of one group equal the independent Python model on 120 seeded groups"')
open(D+"🟦️.ts","w",encoding="utf8").write(t)
# oracle
py=open(D+"🐍️.py",encoding="utf8").read()
a=py.index("def retire_entries(case):");b=py.index("handler = retire_entries")
newpy='''def accelerator(name):
    return re.search(r"\\.records/|^mr/", name) is not None


def retire_entries(case):
    units = {}
    for leaf in case["leaves"]:
        key = unit(leaf["name"])
        if key is None:
            continue
        row = units.setdefault(key, {"key": key, "newest": 0, "fast": 0, "rest": 0, "has_rest": False, "has_fast": False})
        row["newest"] = max(row["newest"], leaf["newest"])
        if accelerator(leaf["name"]):
            row["fast"] += leaf["bytes"]
            row["has_fast"] = True
        else:
            row["rest"] += leaf["bytes"]
            row["has_rest"] = True
    remaining = sum(leaf["bytes"] for leaf in case["leaves"]) + case["otherBytes"]
    order = sorted(units.values(), key=lambda r: (r["newest"], r["key"]))
    shed, retired = [], []
    for row in order:
        if remaining <= case["targetBytes"]:
            break
        if not row["has_fast"] or case["now"] - row["newest"] < case["protectedMs"]:
            continue
        shed.append(row["key"])
        remaining -= row["fast"]
    for row in order:
        if remaining <= case["targetBytes"]:
            break
        if not row["has_rest"] or case["now"] - row["newest"] < case["protectedMs"]:
            continue
        retired.append(row["key"])
        remaining -= row["rest"]
    return {"shed": sorted(set(shed) | set(retired) & {r["key"] for r in order if r["has_fast"]}), "retired": sorted(retired), "remainingBytes": max(0, remaining)}


'''
py=py[:a]+newpy+py[b:]
py=py.replace('the units inside one group: a record header with its packs, a memory pack with its index, any other leaf alone; observation blocks belong to no unit)','the units inside one group: a record header with its packs, a memory pack with its index, any other leaf alone; observation blocks belong to no unit; two tiers, each oldest first: the accelerator packs of every unit first, whole units only while that is not enough)')
open(D+"🐍️.py","w",encoding="utf8").write(py)
