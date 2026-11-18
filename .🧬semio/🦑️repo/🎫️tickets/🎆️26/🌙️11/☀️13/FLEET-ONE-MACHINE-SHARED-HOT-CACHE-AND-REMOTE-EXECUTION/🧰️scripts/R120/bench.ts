// R120: deterministic cold intake benchmark. Entries E_0..E_{N-1} each name the same K shared entries (travelling inside every entry with other sidecar bytes);
// a cold reader takes the entries one after the other. env: R120_K, R120_N, R120_ROWS, R120_SIDE (sidecar chars), R120_LAYERS
import{mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const{startRemoteCacheServerV1}=await import(L+"🌐️remote-cache/🖥️server/🟦️.ts");
const{createClosureEntryLaneV1}=await import(L+"🪪️graph-state/📦️project-records/🚚️entry-lane/🟦️.ts");
const SI:any=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),{libraryBootstrap}=await import(L+"🕸️graph/📥️inference/🟨️.mjs") as any;
const{createClosureStoreV1}=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs") as any;
await libraryBootstrap;
const K=Number(process.env.R120_K??8),N=Number(process.env.R120_N??6),ROWS=Number(process.env.R120_ROWS??3000),SIDE=Number(process.env.R120_SIDE??3000000),LAYERS=Number(process.env.R120_LAYERS??120);
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const base=mkdtempSync(join(tmpdir(),"r120-bench-"));
const tree=(root:string)=>{mkdirSync(join(root,"ce"),{recursive:true});mkdirSync(join(root,"src"),{recursive:true});for(let at=0;at<ROWS;at++){const file=join(root,"src","f"+at+".ts");writeFileSync(file,"export const v"+at+"="+at+";\n");utimesSync(file,1_700_000_000,1_700_000_000);}SI.bindClosureStoreV1(root,join(root,"ce"),true);SI.bindClosureScriptsV1(root,{});};
const rowOf=(root:string,at:number)=>{const text="export const v"+at+"="+at+";\n";return{kind:"file",path:`${root}/src/f${at}.ts`,fact:{sha256:sha(text),size:Buffer.byteLength(text),mtimeMs:1,ctimeMs:1,ino:1}};};
const recipeOf=(root:string,name:string)=>({fn:"native-target",target:{metadata:{semio:{}},options:{command:`repo "${root}/script.ts" run`,cwd:"libs/"+name,env:{CATALOG:`${root}/catalog`}}},projectRoot:"libs/"+name});
const derive=async(root:string,name:string,rowCount:number,layers:number)=>{
 const recipe=recipeOf(root,name),store=createClosureStoreV1({directory:join(root,"ce")}),rows=Array.from({length:rowCount},(_,at)=>rowOf(root,at)),per=Math.max(1,Math.floor(rowCount/layers));
 const shared=Array.from({length:layers},(_,l)=>rows.slice(per*l,l===layers-1?rowCount:per*(l+1)));
 await store.save(`${root}\0${SI.closureRecipeKeyV1(recipe)}`,{recipe,value:rows.map(row=>row.path.replace(root,"{workspaceRoot}")),rows:rows.slice(0,20),layers:shared,sidecar:{facts:[null],members:[null]}});
 return recipe;
};
const withShared=(shared:unknown[],variant:number)=>({...SI.closureTravelV1,async export(root:string,image:string){
 const own=await SI.closureTravelV1.export(root,image),extra:any[]=[];
 for(const recipe of shared)for(const row of await SI.closureTravelV1.export(root,JSON.stringify({closureRecipes:{s:recipe}})))extra.push({...row,bundle:{...row.bundle,layers:row.bundle.layers,sidecar:{facts:[{walked:variant+"x".repeat(SIDE)}],members:[null]}}});
 return[...own,...extra];
}});
const cache=mkdtempSync(join(base,"cache-")),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**32,maximumEntryBytes:2**30,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
const rootA=join(base,"a");tree(rootA);
const shared=[];for(let k=0;k<K;k++)shared.push(await derive(rootA,"shared"+k,ROWS,LAYERS));
const tops=[];for(let e=0;e<N;e++)tops.push(await derive(rootA,"top"+e,30,3));
const laneA=createClosureEntryLaneV1({root:rootA,environment,owner:"a:1"})!;
for(let e=0;e<N;e++)if(!await laneA.offer(tops[e],withShared(shared,e+1)))throw Error("offer refused "+e);
await laneA.flush();
const rootB=join(base,"b");tree(rootB);
const lane=createClosureEntryLaneV1({root:rootB,environment,owner:"b:1"})!;
let imports=0;const travel={...SI.closureTravelV1,async import(root:string,bundles:any[]){imports+=bundles.length;return SI.closureTravelV1.import(root,bundles);}};
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};
const c0=cpu(),w0=performance.now(),rows:string[]=[];
for(let e=0;e<N;e++){const a=cpu(),w=performance.now(),outcome=(await lane.take(recipeOf(rootB,"top"+e),travel)).outcome;rows.push(`take ${e}: ${outcome} cpu=${Math.round(cpu()-a)}ms wall=${Math.round(performance.now()-w)}ms imports=${imports}`);}
console.log(rows.join("\n"));
console.log("TOTAL",JSON.stringify({K,N,rows:ROWS,sidecarChars:SIDE,cpuSeconds:Math.round((cpu()-c0)/100)/10,wallSeconds:Math.round((performance.now()-w0)/100)/10,imports,maxRssMB:Math.round(process.resourceUsage().maxRSS/1048576)}));
await server.stop();rmSync(base,{recursive:true,force:true});process.exit(0);
