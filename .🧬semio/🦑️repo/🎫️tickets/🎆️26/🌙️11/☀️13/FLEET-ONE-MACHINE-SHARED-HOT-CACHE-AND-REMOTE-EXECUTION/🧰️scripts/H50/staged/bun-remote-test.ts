import{test,expect}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{runOwnedBunFilesV1,readOwnedBunFilesPolicyV1,type OwnedBunFileResultV1}from"../../🟦️.ts";
import{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import corpus from"../🔣️.json";

const capture=(status:number,text:string)=>Object.freeze({version:1 as const,stdout:"",stderr:text,status,signal:null});

/** 🧪️ Runs four tiny files with a remote port that takes `a` (passed) and `c` (failed), keeps `b` and `d` here and, in the second run, throws for every file. */
async function runV1(take:(file:string)=>Promise<OwnedBunFileResultV1|undefined>,width:number):Promise<{dir:string;files:string[];rows:readonly OwnedBunFileResultV1[];measured:string[];seen:number[]}>{
 const dir=mkdtempSync(join(tmpdir(),"bun-remote-")),names=["a","b","c","d"],files=names.map(name=>join(dir,name+".test.ts")),measured:string[]=[],seen:number[]=[];
 for(const [position,file] of files.entries())writeFileSync(file,`import{test}from"bun:test";import{writeFileSync}from"node:fs";test("${names[position]}",()=>{writeFileSync(${JSON.stringify(join(dir,names[position]+".ran"))},"x");});`);
 const rows=await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:remote-lanes",maximumElapsedMilliseconds:120000},{},Date.now()),async invocation=>runOwnedBunFilesV1({invocation,control:createIssuedResourceControlV1(invocation,"test:remote-lanes",0),cwd:dir,files,arguments:[],environment:{...process.env},policy:{...readOwnedBunFilesPolicyV1(corpus.policy),maximumParallelFiles:1},remote:{width,take:async file=>take(file),measure:file=>{measured.push(file);}},onFileResult:async(_row,index)=>{seen.push(index);}}));
 return{dir,files,rows,measured,seen};
}

test("a remote lane takes some files, the local lanes run the rest, the results stay in file order and a remote failure is a verdict",async()=>{
 const offered:string[]=[],run=await runV1(async file=>{offered.push(file);if(file.endsWith("a.test.ts"))return{file,capture:capture(0,"ran on g1-x\n")};if(file.endsWith("c.test.ts"))return{file,capture:capture(1,"failed on g1-y\n")};return undefined;},2);
 try{
  expect(run.rows.map(row=>row.file)).toEqual(run.files);
  expect(run.rows.map(row=>row.capture.status)).toEqual([0,0,1,0]);
  expect(run.rows[0]!.capture.stderr).toBe("ran on g1-x\n");expect(run.rows[2]!.capture.stderr).toBe("failed on g1-y\n");
  expect(["a","b","c","d"].map(name=>existsSync(join(run.dir,name+".ran")))).toEqual([false,true,false,true]);
  expect([...offered].sort()).toEqual([...run.files].sort());
  expect(run.measured.map(file=>file.slice(-10)).sort()).toEqual(["/b.test.ts","/d.test.ts"]);
  expect([...run.seen].sort()).toEqual([0,1,2,3]);
 }finally{rmSync(run.dir,{recursive:true,force:true});}
},60000);

test("a remote port that throws sends every file to the local lanes",async()=>{
 const run=await runV1(async()=>{throw Error("remote down");},3);
 try{
  expect(run.rows.map(row=>row.capture.status)).toEqual([0,0,0,0]);
  expect(["a","b","c","d"].every(name=>existsSync(join(run.dir,name+".ran")))).toBe(true);
  expect(run.measured).toHaveLength(4);
 }finally{rmSync(run.dir,{recursive:true,force:true});}
},60000);
