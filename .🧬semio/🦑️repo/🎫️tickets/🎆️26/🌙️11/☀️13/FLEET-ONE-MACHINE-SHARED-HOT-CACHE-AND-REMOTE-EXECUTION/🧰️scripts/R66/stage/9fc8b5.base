import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{recordOwnerCostFileV1}from"../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{fleetReadStaleGenerationV1}from"../../🤝️prefetch/🟦️.ts";

/** 🙋️ Contract: a graph read decides fleet-first by its admitted memory (`SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` reaches the decision): a serving-sized machine and a disabled fleet read stay local and silent, a 16 GB shape with thousands of stale projects and no fleet to ask refuses in plain text with the designed refusal instead of waiting for ever. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const request=(count:number,extra:readonly string[]=[],roots:Record<string,string>={})=>({generation:3,dirty:[...Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`),...extra],roots,structural:false,implementation:"a".repeat(64)});
const invocation={control:{signal:new AbortController().signal}} as never;
async function run(count:number,environment:Record<string,string>,extra:readonly string[]=[],roots:Record<string,string>={},prepare?:(root:string)=>void){
 const root=mkdtempSync(join(tmpdir(),"fleet-read-"));scratch.push(root);prepare?.(root);
 const lines:string[]=[],write=process.stderr.write.bind(process.stderr);
 process.stderr.write=((chunk:string|Uint8Array)=>{lines.push(String(chunk));return true;}) as never;
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;
 try{const result=await fleetReadStaleGenerationV1({root,request:request(count,extra,roots),invocation,language:"en",environment:{PATH:process.env.PATH,HOME:root,...environment},hostMegabytes:()=>Number(environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES)});return{lines,error:undefined,result:result as {solo?:{owners:{name:string}[];grant:{managedMegabytes:number}}}|undefined};}
 catch(error){return{lines,error:error as {name:string;key:string;code:string}};}
 finally{process.stderr.write=write;if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
}

test("a disabled fleet read and a serving-sized machine say nothing and read locally",async()=>{
 expect(await run(5000,{SEMIO_GRAPH_FLEET_READ:"off",SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"})).toMatchObject({lines:[],error:undefined});
 expect(await run(5000,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"32768"})).toMatchObject({lines:[],error:undefined});
});

test("a 16 GB shape with 5 000 stale projects and no fleet cache says so in plain text and infers them itself, wave after wave, instead of refusing; only a machine that cannot run the smallest wave is refused with the designed class",async()=>{
 const result=await run(5000,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"});
 expect(result.error).toBeUndefined();expect(result.lines.join("")).toContain("cannot be reached");expect(result.lines.join("")).toContain("5000 stale projects");
 const tiny=await run(5000,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"4096"});
 expect(tiny.error?.name).toBe("RepoError");expect(tiny.error?.key).toBe("build.graph.wanted-expired");expect(tiny.error?.code).toBe("unserved");
});

test("a 16 GB shape with few stale projects stays local and silent",async()=>{
 expect(await run(40,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"})).toMatchObject({lines:[],error:undefined});
});

test("the workspace root is too heavy to share a 16 GB machine but a machine without a fleet to ask reads alone: it is named as the solo owner with the solo grant and nothing is refused; a squeezed machine still attempts it",async()=>{
 const alone=await run(1,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},["workspace"],{workspace:"."});
 expect(alone.error).toBeUndefined();expect(alone.result?.solo?.owners.map(row=>row.name)).toEqual(["workspace"]);expect(alone.result?.solo?.grant.managedMegabytes).toBeGreaterThanOrEqual(3100);
 expect(alone.lines.join("")).toContain("too heavy");expect(alone.lines.join("")).toContain("one at a time");
});

test("a project this machine recorded as heavy runs alone when the solo grant holds it, is attempted alone even when its recorded cost is above the grant (a learned number never refuses), and a light one stays local",async()=>{
 const record=(megabytes:number)=>(root:string)=>{recordOwnerCostFileV1(join(root,".cache/f/graph-costs/owner-costs.json"),{name:"@semio-tech/stopped",kind:"project",megabytes,seconds:30,rows:100,stopped:true,atMs:Date.now()});};
 const fits=await run(0,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},["@semio-tech/stopped"],{},record(3500));
 expect(fits.error).toBeUndefined();expect(fits.result?.solo?.owners.map(row=>row.name)).toEqual(["@semio-tech/stopped"]);expect(fits.lines.join("")).toContain("@semio-tech/stopped");
 const over=await run(0,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},["@semio-tech/stopped"],{},record(5000));
 expect(over.error).toBeUndefined();expect(over.result?.solo?.owners.map(row=>row.name)).toEqual(["@semio-tech/stopped"]);
 expect(await run(0,{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"},["@semio-tech/light"],{},record(3500))).toMatchObject({lines:[],error:undefined});
});
