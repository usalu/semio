/** 🏁️ Graph-free concurrency harness of lane builderlock: two agents (two operating system processes) refresh the graph of a synthetic workspace through the REAL generation store, load and (with `--mode slice`) slice custody of `--root`, after one edit each; measures the wall of the concurrent pair against the same two refreshes one after the other.
 * usage: bun race.ts --root <checkout> --mode single|slice --scenario disjoint|overlap [--setup-ms 3000] [--owner-ms 5500] [--edits 1] [--rounds 3] [--work <dir>] [--json <file>]
 * `single` = today's one builder custody (one exclusive cross-process lease around every build); `slice` = the slice custody of batch multiwriter. Inference cost is simulated: `setup-ms` once per build (the whole-workspace setup), `owner-ms` per inferred owner. */
import {cpSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";

const argument=(name:string,fallback?:string)=>{const at=process.argv.indexOf("--"+name);return at>0?process.argv[at+1]!:fallback;};
const root=resolve(argument("root")!),G=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations");
const world=await import(join(G,"🧪️tests/🏃️world/🟦️.ts")),load=await import(join(G,"🔌️load/🟨️.mjs")),generations=await import(join(G,"🟨️.mjs"));
const sleep=(ms:number)=>new Promise(done=>setTimeout(done,ms));

/** 🧑‍💻️ One agent process: refreshes the closure `roots` and prints when it started, held its custody, computed and finished. */
async function child(dir:string,mode:string,roots:string[],setupMs:number,ownerMs:number){
 const started=Date.now(),mock=world.createMockHost(dir,{ownerDelayMs:ownerMs}),host=mock.host as any,discover=host.discoverPart,row:Record<string,unknown>={started};
 host.discoverPart=async(spec:any,signal:AbortSignal,control:unknown)=>{row.computeStart=Date.now();await sleep(setupMs);const part=await discover(spec,signal,control);row.computeEnd=Date.now();row.owners=spec.owners;return part;};
 if(mode==="single"){
  const lease=generations.createFileLeasePortV1(join(dir,".builder"));
  host.withBuilder=async(body:any)=>lease.exclusive("graph-builder",async()=>{row.held=Date.now();return body({root:join(dir,".gen")},{published:mock.published});});
 }else{
  const custody=await import(join(G,"🔐️custody/🟨️.mjs")),claims=await import(join(G,"🧩️claims/🟨️.mjs"));
  const slice=custody.createSliceCustodyV1({root:join(dir,".gen"),fs:generations.createNodeFilePortV1(),lease:claims.createFileClaimLeasePortV1(join(dir,".custody"))});
  host.withBuilder=async(body:any,request?:{dirty:readonly string[];structural:boolean})=>{const claim=await slice.hold({owners:request?.dirty??[],structural:request===undefined||request.structural});row.held??=Date.now();try{return await body({root:join(dir,".gen")},{published:mock.published},claim);}finally{await claim.release();}};
 }
 await load.loadGenerationGraphV1(mock.host,{closure:roots});
 row.ended=Date.now();row.said=mock.said.map((entry:{key:string})=>entry.key);row.computed=mock.log.length;
 process.stdout.write(JSON.stringify(row)+"\n");
}

/** 🧪️ Seeds two independent rings, publishes the first generation and applies the scenario's edits; the result is the template every round copies. */
async function template(dir:string,scenario:string,edits:number){
 await world.seed(dir,{...world.ringFiles(4,"a"),...world.ringFiles(4,"b")});
 await load.loadGenerationGraphV1(world.createMockHost(dir).host);
 const names=scenario==="disjoint"?[...Array.from({length:edits},(_,i)=>"a"+(1+i%3)),...Array.from({length:edits},(_,i)=>"b"+(1+i%3))]:Array.from({length:2*edits},(_,i)=>"a"+(1+i%3));
 await world.applyOps(dir,[...new Set(names)].map(name=>({op:"write",path:`pkg/${name}/src/index.ts`,content:`export const ${name}edit=1;\n`})));
 return scenario==="disjoint"?[["a0"],["b0"]]:[["a0"],["a3"]];
}

const run=(dir:string,mode:string,roots:string[],setupMs:number,ownerMs:number)=>{
 const process=Bun.spawn([globalThis.process.execPath,import.meta.path,"child","--root",root,"--dir",dir,"--mode",mode,"--roots",roots.join(","),"--setup-ms",String(setupMs),"--owner-ms",String(ownerMs)],{stdout:"pipe",stderr:"pipe"});
 return Promise.all([new Response(process.stdout).text(),new Response(process.stderr).text(),process.exited]).then(([out,err,code])=>{if(code!==0)throw Error("[DEBUG] child failed "+code+": "+err.slice(-2000));return JSON.parse(out.trim().split("\n").at(-1)!);});
};

if(process.argv[2]==="child"){await child(argument("dir")!,argument("mode")!,argument("roots")!.split(","),Number(argument("setup-ms")),Number(argument("owner-ms")));process.exit(0);}

const mode=argument("mode","slice")!,scenario=argument("scenario","disjoint")!,setupMs=Number(argument("setup-ms","3000")),ownerMs=Number(argument("owner-ms","5500")),edits=Number(argument("edits","1")),rounds=Number(argument("rounds","3"));
const work=argument("work")??mkdtempSync(join(process.env.TMPDIR??"/tmp","bk-race-"));mkdirSync(work,{recursive:true});
const base=join(work,"template");rmSync(base,{recursive:true,force:true});mkdirSync(base,{recursive:true});
const roots=await template(base,scenario,edits),rows:any[]=[];
for(let round=0;round<rounds;round++){
 const serial=join(work,"serial-"+round),pair=join(work,"pair-"+round);cpSync(base,serial,{recursive:true});cpSync(base,pair,{recursive:true});
 const s0=Date.now(),first=await run(serial,mode,roots[0]!,setupMs,ownerMs),second=await run(serial,mode,roots[1]!,setupMs,ownerMs),serialWall=Date.now()-s0;
 const p0=Date.now(),both=await Promise.all(roots.map(list=>run(pair,mode,list,setupMs,ownerMs))),pairWall=Date.now()-p0;
 const spans=both.filter(row=>row.computeStart).map(row=>[row.computeStart,row.computeEnd]),overlapMs=spans.length===2?Math.max(0,Math.min(spans[0][1],spans[1][1])-Math.max(spans[0][0],spans[1][0])):0;
 rows.push({round,serialWall,pairWall,speedup:+(serialWall/pairWall).toFixed(2),overlapMs,waits:both.map(row=>row.held?row.held-row.started:null),computed:both.map(row=>row.computed),said:both.map(row=>row.said.filter((key:string)=>["published","merged","blocked","blockedAll","waiting"].includes(key))),serialComputed:[first.computed,second.computed]});
 rmSync(serial,{recursive:true,force:true});rmSync(pair,{recursive:true,force:true});
}
const median=(values:number[])=>[...values].sort((a,b)=>a-b)[Math.floor(values.length/2)]!;
const summary={root,mode,scenario,setupMs,ownerMs,edits,rounds,serialMedian:median(rows.map(row=>row.serialWall)),pairMedian:median(rows.map(row=>row.pairWall)),rows};
const json=argument("json");if(json)writeFileSync(json,JSON.stringify(summary,null,1));
console.log(JSON.stringify({mode,scenario,serialMedian:summary.serialMedian,pairMedian:summary.pairMedian,speedup:+(summary.serialMedian/summary.pairMedian).toFixed(2),rows:rows.map(row=>({serial:row.serialWall,pair:row.pairWall,overlapMs:row.overlapMs,waits:row.waits,computed:row.computed,said:row.said}))}));
if(!argument("work"))rmSync(work,{recursive:true,force:true});
