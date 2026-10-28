/** 🧾️ Contract: the observation rows of a fixed scenario (reads, listings, probes, repeats, an edit between two reads, a memoized replay, a persisted project record reused in a second round) are exactly the rows, in exactly the order, that the observation module recorded before the row table: the digests of the fixture were taken from that module. */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{captureObservedSourceInputsV1,memoizedObservedComputationV1,observeInferenceSnapshotV1,observeProjectInferenceV1,observedFs as fs,projectInferenceObservationsV1}from"../../🟨️.mjs";

type Row={kind:string;path:string;fact:any};
test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the scenario records the rows of the module before the row table, in order",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"obs-seq-"))),stamp=1700000000,{files,directories}=fixture;
 const put=(relative:string,text:string)=>{const path=join(root,relative);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,text);utimesSync(path,stamp,stamp);return path;};
 const digest=(rows:readonly Row[])=>createHash("sha256").update(JSON.stringify(rows.map(row=>[row.kind,row.path.replace(root,"@"),typeof row.fact==="object"&&row.fact?.sha256?{...row.fact,mtimeMs:0,ctimeMs:0,ino:0}:row.fact]))).digest("hex");
 try{
  for(let at=0;at<files;at++)put(`d${at%directories}/f${at}.txt`,"x".repeat(at)+"\n");
  const captured=await captureObservedSourceInputsV1(async()=>observeInferenceSnapshotV1(async()=>{
   for(let round=0;round<2;round++){
    for(let at=0;at<files;at++){const directory=join(root,`d${at%directories}`);fs.readFileSync(join(directory,`f${at}.txt`),"utf8");if(at%3===0)fs.existsSync(join(directory,`missing${at}`));if(at%5===0)fs.lstatSync(directory);fs.readdirSync(directory,{withFileTypes:true});fs.statSync(join(directory,`f${at}.txt`));}
    for(let at=0;at<files;at+=4)fs.readdirSync(join(root,`d${at%directories}`));
    if(round===0)put("d3/f3.txt","changed\n");
   }
   const cache=new Map(),first=memoizedObservedComputationV1(cache,"k",()=>{fs.readFileSync(join(root,"d1/f1.txt"));fs.existsSync(join(root,"nothing"));return 1;}),second=memoizedObservedComputationV1(cache,"k",()=>2);
   return[first,second];
  }),undefined,false);
  expect({rows:captured.observations.length,sha256:digest(captured.observations as never)}).toEqual(fixture.expected.capture);
  const record=join(root,"np.json"),seen:unknown[]=[];
  for(let round=0;round<2;round++){
   const value=await observeInferenceSnapshotV1(()=>observeProjectInferenceV1({cachePath:record,key:"k",owner:root,stamps:[]},async()=>fs.readdirSync(root).length));
   const rows=(projectInferenceObservationsV1(record)??[]) as Row[];seen.push({round,value,rows:rows.length,sha256:digest(rows)});
  }
  expect(seen).toEqual(fixture.expected.records);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
