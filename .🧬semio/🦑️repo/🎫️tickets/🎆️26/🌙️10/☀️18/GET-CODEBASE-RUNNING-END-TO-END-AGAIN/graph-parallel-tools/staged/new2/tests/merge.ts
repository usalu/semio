import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{createFarmV1,createInternDecoderV1}from"../../../../🏊️farm/🟨️.mjs";
import{createBuildGraphOptionalQuotaV1,withBuildGraphOptionalPublicationV1}from"../../../../📥️inputs/💾️storage/🟨️.mjs";
import{createOptionalAccountingV1}from"../../../../📥️inputs/💾️storage/🏊️leaves/🟨️.mjs";
import*as generations from"../../🟨️.mjs";
import{shapeComputeResultV1}from"../../🔌️load/🟨️.mjs";
import{ownerLeaves,ownerResult,writeLeaf}from"./🧰️service/🟨️.mjs";

const workerEntry=fileURLToPath(new URL("../../../../🏊️farm/👷️worker/🟨️.mjs",import.meta.url)),service=new URL("./🧰️service/🟨️.mjs",import.meta.url).href,OWNERS=36;

/** 🗂️ Hashes every file below a directory by its relative name. */
function tree(root:string){
 const out:Record<string,string>={},walk=(directory:string)=>{for(const name of readdirSync(directory).sort()){const path=join(directory,name);if(statSync(path).isDirectory())walk(path);else out[relative(root,path)]=createHash("sha256").update(readFileSync(path)).digest("hex");}};
 walk(root);return out;
}

/** 🏗️ Publishes one generation from per-owner results, in the engine's own store, and answers the files it wrote and the assembled graph. */
async function publish(root:string,results:{node:any;rows:any[]}[]){
 const part={owners:results.map(result=>({name:result.node.name,root:result.node.root,node:result.node,nodeRows:result.rows})),edgeOnly:[],edges:[],externalNodes:{},parseUnavailable:[],rows:results.flatMap(result=>result.rows),env:{digest:"e".repeat(64),rows:[]},replaceTags:[]};
 const shaped=shapeComputeResultV1({workspaceRoot:"/world",part,stubs:{}} as never);
 const frontier=generations.createFrontierV1({directories:[],members:[],capturedAt:0}),engine=generations.createGenerationStoreV1({root,holds:async()=>true,resources:{facts:"facts",builder:"builder"},now:()=>0});
 const result=await engine.refresh({identity:"a".repeat(64),implementation:"c".repeat(64),policy:"fresh",membership:async()=>({digest:"b".repeat(64),frontier,changes:[]}),compute:async()=>shaped,signal:undefined,progress:undefined} as never);
 const assembled=await engine.assemble(result.current,{compare:(a:string,b:string)=>a<b?-1:a>b?1:0});
 return{files:tree(root),graph:JSON.stringify(assembled)};
}

test("a generation merged from workers in any completion order is byte for byte the generation the serial results give, and so is every graph leaf the workers wrote themselves",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"merge-law-"));
 try{
  const serialInference=join(directory,"serial-graph","inference");mkdirSync(serialInference,{recursive:true});
  for(let n=0;n<OWNERS;n++)for(const leaf of ownerLeaves(n))writeLeaf(serialInference,leaf,withBuildGraphOptionalPublicationV1);
  const serial=await publish(join(directory,"serial"),Array.from({length:OWNERS},(_,n)=>ownerResult(n)));
  const serialLeaves=tree(join(directory,"serial-graph"));
  expect(Object.keys(serial.files).length).toBeGreaterThan(3);expect(Object.keys(serialLeaves).length).toBe(OWNERS+1);
  for(const [index,workers] of [2,5].entries()){
   const graph=join(directory,"graph-"+workers),inference=join(graph,"inference");mkdirSync(inference,{recursive:true});
   const farm=createFarmV1({entry:workerEntry,services:{merge:service},workers});
   farm.open("merge",{roots:{inference}},{accept:(_kind:string,identity:unknown)=>identity==="merge-law"});
   const tables=new Map<string,ReturnType<typeof createInternDecoderV1>>(),canonical=new Map<string,unknown>(),leaves:{name:string;bytes:number}[]=[];
   try{
    const replies=await Promise.all(Array.from({length:OWNERS},(_,n)=>farm.infer("merge",{n,delay:((n*7919+index*104729)%31)})));
    const results=replies.map(outcome=>{
     const reply=(outcome as {reply:any}).reply;
     let table=tables.get(reply.worker);if(!table)tables.set(reply.worker,table=createInternDecoderV1({canonical:(row:any)=>{const key=JSON.stringify(row);return (canonical.get(key)??(canonical.set(key,row),row)) as any;}}));
     table.learn(reply.freshRows);leaves.push(...reply.leaves);
     return{node:reply.node,rows:table.decode(reply.rowIds)};
    });
    const merged=await publish(join(directory,"farm-"+workers),results);
    expect(merged.files).toEqual(serial.files);
    expect(merged.graph).toBe(serial.graph);
    expect(tree(graph)).toEqual(serialLeaves);
    expect(farm.stats().fallbacks).toBe(0);
    const quota=createBuildGraphOptionalQuotaV1({maximumBytes:2**30,usedBytes:0,availableBytes:2**40,minimumAvailableBytes:1}),calls:number[]=[],accounting=createOptionalAccountingV1({ledger:{publish:(plan:any,write:()=>void)=>{calls.push(plan.retainedDeltaBytes);write();return true;}},quota});
    const outcome=accounting.adopt(leaves.filter((leaf,at)=>leaves.findIndex(other=>other.name===leaf.name)===at),()=>{});accounting.flush(true);
    expect(outcome.refused).toEqual([]);
    const physical=Object.keys(serialLeaves).reduce((sum,name)=>sum+statSync(join(graph,name)).size,0);
    expect(quota.usedBytes).toBe(physical);expect(calls.reduce((sum,bytes)=>sum+bytes,0)).toBe(physical);expect(calls.length).toBeLessThanOrEqual(2);
   }finally{await farm.close();}
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);
