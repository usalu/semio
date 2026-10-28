/** 🧪️ Tests for the retention of the admission memos. */
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {pruneAdmissionsV1} from "../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.gc,layout=UNIT_CACHE_POLICY_V1.layout.admissions,DAY=86400000,base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(base,{recursive:true});
const seeded=(seed:number)=>{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};};
const survivors=(root:string):string[]=>readdirSync(join(root,layout)).flatMap(shard=>readdirSync(join(root,layout,shard)).map(name=>join(root,layout,shard,name))).sort();
const python=`
import json,os,sys
v=json.load(sys.stdin);base=os.path.join(v["root"],v["layout"]);lim=v["limits"];now=v["nowMs"]
rows=[];removed=set();freed=0
for shard in sorted(os.listdir(base)):
    for name in sorted(os.listdir(os.path.join(base,shard))):
        path=os.path.join(base,shard,name);st=os.stat(path);size=max(st.st_size,st.st_blocks*512);mtime=st.st_mtime*1000
        if now-mtime>(lim["retentionMs"] if name.endswith(".json") else lim["temporaryMs"]):
            removed.add(path);freed+=size
        else:
            rows.append((mtime,path,size))
kept=sum(row[2] for row in rows);rows.sort();index=0
while index<len(rows) and kept>lim["maximumBytes"]:
    removed.add(rows[index][1]);freed+=rows[index][2];kept-=rows[index][2];index+=1
every=set(os.path.join(base,s,n) for s in os.listdir(base) for n in os.listdir(os.path.join(base,s)))
print(json.dumps({"removed":len(removed),"freedBytes":freed,"keptBytes":kept,"keptFiles":len(rows)-index,"survivors":sorted(every-removed)}))
`;

test("the memo retention equals an independent Python sort-and-walk on seeded memo trees",async()=>{
 const now=Date.now();
 for(const seed of [1,7,20261026]){
  const random=seeded(seed),root=mkdtempSync(join(base,"adm-"));
  try{
   let total=0;
   for(let index=0;index<320;index++){
    const shard=(index%16).toString(16).padStart(2,"0"),key=(index*2654435761>>>0).toString(16).padStart(64,"0"),temporary=random()<0.1,name=key+(temporary?".json."+index.toString(16):".json"),size=1+Math.floor(random()*20000),age=Math.floor(random()*10*DAY/1000);
    mkdirSync(join(root,layout,shard),{recursive:true});writeFileSync(join(root,layout,shard,name),Buffer.alloc(size,index%251));
    const seconds=Math.floor(now/1000)-(temporary?Math.floor(random()*7200):age);utimesSync(join(root,layout,shard,name),seconds,seconds);total+=size;
   }
   const limits={retentionMs:rules.admissionRetentionMilliseconds,temporaryMs:rules.admissionTemporaryMilliseconds,maximumBytes:Math.floor(total*(0.05+random()*0.4))};
   const run=spawnSync(testPythonV1()??"python3",["-I","-c",python],{input:JSON.stringify({root,layout,limits,nowMs:now}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
   const expected=JSON.parse(run.stdout) as {removed:number;freedBytes:number;keptBytes:number;keptFiles:number;survivors:string[]};
   const result=await pruneAdmissionsV1(root,now,limits);
   expect({removed:result.removed,freedBytes:result.freedBytes,keptBytes:result.keptBytes,keptFiles:result.keptFiles},"seed "+seed).toEqual({removed:expected.removed,freedBytes:expected.freedBytes,keptBytes:expected.keptBytes,keptFiles:expected.keptFiles});
   expect(survivors(root),"survivors "+seed).toEqual(expected.survivors);
   expect(expected.removed).toBeGreaterThan(0);expect(expected.keptFiles).toBeGreaterThan(0);expect(expected.keptBytes).toBeLessThanOrEqual(limits.maximumBytes);
   expect((await pruneAdmissionsV1(root,now,limits)).removed,"a second pass finds nothing").toBe(0);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("a root without memos is empty, a cancellation stops the pass and the authored bounds are positive",async()=>{
 const root=mkdtempSync(join(base,"adm-"));
 try{
  const limits={retentionMs:DAY,temporaryMs:1000,maximumBytes:1};
  expect(await pruneAdmissionsV1(root,Date.now(),limits)).toEqual({removed:0,freedBytes:0,keptBytes:0,keptFiles:0});
  mkdirSync(join(root,layout,"00"),{recursive:true});writeFileSync(join(root,layout,"00","a.json"),"x");
  const stop=new AbortController();stop.abort(new Error("cancelled"));
  await expect(pruneAdmissionsV1(root,Date.now(),limits,stop.signal)).rejects.toThrow("cancelled");
  expect(readdirSync(join(root,layout,"00"))).toEqual(["a.json"]);
 }finally{rmSync(root,{recursive:true,force:true});}
 expect(rules.admissionShare).toBeGreaterThan(0);expect(rules.admissionRetentionMilliseconds).toBeGreaterThan(rules.admissionTemporaryMilliseconds);
});
