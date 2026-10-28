/** 🔍️ Read-only probe: for a sealed memo record of the live tree, lists the slice rows (with nested leaves) whose memo mentions name each given workspace-relative path or lie in its directory, the owners holding them, and the cost of opening the generation's fact index view. Usage: bun paths.ts <root> <record.json> <relative path>... */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,recordFile,...targets]=process.argv.slice(2) as [string,string,...string[]];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const {scopeWithDerivedV1,isDerivedOwnerV1}=await import(L+"/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs");
const {decodeVerifiedScopeV1}=await import(L+"/🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs");
const memo=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts");
const {readChangeStateV1}=await import(root+"/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts");
const rss=()=>Math.round(process.memoryUsage().rss/1048576),cpu=()=>{const usage=process.cpuUsage();return +((usage.user+usage.system)/1e6).toFixed(2);};
const record=JSON.parse(readFileSync(recordFile,"utf8")),pin=JSON.parse(record.pin);
const engine=generation.createGenerationStoreV1({root:join(root,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
let started=performance.now();
const manifest=await engine.readManifestAt(record.manifest);if(!manifest)throw Error("manifest retired");
const index=await engine.readIndexView(manifest.factIndex);
console.log("[DEBUG] open",JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss(),rows:index.rows.length}));
started=performance.now();
const owners=Object.keys(manifest.owners).sort(generation.compareCodePointsV1),covered=decodeVerifiedScopeV1(owners,pin.covered);
const selection=generation.selectFactClassesV1(index,scopeWithDerivedV1(index,new Set([...covered,generation.GENERATION_POLICY_V1.envOwner])));
const state=readChangeStateV1(join(recordFile,"../.."));if(!state)throw Error("no journal state");
const excluded=memo.memoExcluderV1(state.exclude,true);
const wanted=new Set(targets),parents=new Set(targets.map(path=>path.slice(0,path.lastIndexOf("/"))));
const ownersOf=(id:number):string[]=>{const out:string[]=[];for(const cls of index.classes)if(cls.r.includes(id))for(const owner of cls.o)out.push(index.owners[owner]);return out;};
const hits:Array<Record<string,unknown>>=[];let leaves=0,bytes=0;
for(const id of selection.ids){
 const row=index.rows[id];
 bytes+=JSON.stringify(row).length;
 let depth=0;
 for(const leaf of memo.memoLeavesV1(row)){
  leaves++;
  const reach=memo.memoReachV1(leaf,root,excluded);if(!reach){depth++;continue;}
  const classes=memo.memoClassesV1(leaf,reach);
  const named=classes.names.filter((path:string)=>wanted.has(path)),inDir=[...classes.strict,...classes.listing].filter((path:string)=>parents.has(path));
  if(named.length||inDir.length)hits.push({id,top:row.kind,topPath:String(row.path).slice(-90),leaf:leaf.kind,leafPath:String(leaf.path).slice(-90),nested:leaf!==row,named,inDir,recheck:reach.recheck});
  depth++;
 }
}
console.log("[DEBUG] slice",JSON.stringify({ms:Math.round(performance.now()-started),cpu:cpu(),rss:rss(),selected:selection.ids.length,leaves,jsonBytes:bytes}));
const summary=new Map<string,number>();
for(const hit of hits){const key=[hit.top,hit.leaf,hit.nested?"nested":"top",(hit.named as string[]).length?"named":"dir"].join(" ");summary.set(key,(summary.get(key)??0)+1);}
console.log("[DEBUG] hits",JSON.stringify(Object.fromEntries(summary)));
const tops=[...new Set(hits.map(hit=>hit.id as number))].sort((a,b)=>Number(hits.find(hit=>hit.id===b)!.leaf==="typescriptText")-Number(hits.find(hit=>hit.id===a)!.leaf==="typescriptText"));
for(const id of tops.slice(0,40)){const hit=hits.find(item=>item.id===id)!;const holders=ownersOf(id);console.log("[DEBUG] row",JSON.stringify({...hit,owners:holders.length,derivedOwners:holders.filter(isDerivedOwnerV1).length,projects:holders.filter((name:string)=>!isDerivedOwnerV1(name)).slice(0,4)}));}
console.log("[DEBUG] distinct top rows",tops.length,"cpu",cpu(),"rss",rss());
