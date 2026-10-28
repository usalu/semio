import{expect,test}from"bun:test";
import{mkdtemp,readFile,rm,writeFile}from"node:fs/promises";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{BUILD_GRAPH_PLAN_PIN_SUFFIX_V1,writeBuildGraphPlanV1}from"../../🕸️graph/🟦️.ts";
import{recordSealedGenerationPinV1}from"../../../🕸️graph/🟦️.ts";

const project=(index:number)=>({name:"p"+index,root:"packages/p"+index,targets:{build:{execution:{kind:"noop"},inputs:["{projectRoot}/**/*",{input:"default",dependencies:true}],outputs:[],note:index%7===0?undefined:"é \"quoted\"\\"+index}},tags:["a","b",index%3===0?"c":"d"]});
const graph=(projects:number,edges:number)=>({version:1,projects:Object.fromEntries(Array.from({length:projects},(_,index)=>["p"+index,project(index)])),dependencies:Array.from({length:edges},(_,index)=>({source:"p"+(index%projects),target:"p"+((index*7+1)%projects),type:"static"})),namedInputs:{default:["{projectRoot}/**/*"]},externalNodes:{"npm:x":{name:"npm:x",data:{version:"1"}},"npm:y":undefined},unavailable:[],admission:{selection:{kind:"all"},registry:"r",closure:"c"}});

test("the plan file is byte-identical to JSON.stringify of the graph, for graphs that cross every chunk boundary",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"semio-plan-file-"));
 try{
  recordSealedGenerationPinV1(undefined);
  for(const[projects,edges]of[[0,0],[1,0],[3,1],[40,4095],[40,4096],[40,4097],[300,10000]]as const){
   const value=graph(projects,edges),file=join(directory,`g-${projects}-${edges}.json`);
   await writeBuildGraphPlanV1(file,value as never);
   const text=await readFile(file,"utf8");
   expect(text,`${projects}/${edges}`).toBe(JSON.stringify(value));
   expect(JSON.parse(text)).toEqual(JSON.parse(JSON.stringify(value)));
  }
  const file=join(directory,"big.json"),value=graph(3000,40000);
  await writeBuildGraphPlanV1(file,value as never);
  expect(await readFile(file,"utf8")).toBe(JSON.stringify(value));
 }finally{await rm(directory,{recursive:true,force:true});}
},60000);

test("the plan file refuses to overwrite, stops when cancelled and still names the generation it served",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"semio-plan-file-"));
 try{
  recordSealedGenerationPinV1(undefined);
  const file=join(directory,"once.json");
  await writeBuildGraphPlanV1(file,graph(2,1) as never);
  await expect(writeBuildGraphPlanV1(file,graph(2,1) as never)).rejects.toThrow();
  const aborted=new AbortController();aborted.abort(Error("cancelled by the caller"));
  await expect(writeBuildGraphPlanV1(join(directory,"stopped.json"),graph(3000,20000) as never,aborted.signal)).rejects.toThrow("cancelled by the caller");
  recordSealedGenerationPinV1('{"pin":3}');
  await writeBuildGraphPlanV1(join(directory,"pinned.json"),graph(2,1) as never);
  expect(await readFile(join(directory,"pinned.json"+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1),"utf8")).toBe('{"pin":3}');
  await writeFile(join(directory,"marker"),"");
 }finally{recordSealedGenerationPinV1(undefined);await rm(directory,{recursive:true,force:true});}
});
