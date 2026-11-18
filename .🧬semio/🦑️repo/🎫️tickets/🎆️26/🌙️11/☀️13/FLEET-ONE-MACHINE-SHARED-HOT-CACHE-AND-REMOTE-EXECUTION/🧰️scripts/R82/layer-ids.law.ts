/** 🪪️ Contract: a layer that an earlier process parsed is decided in the next process from its row ids and the live verdicts of its distinct rows, without parsing the layer again: the rows are still probed against the tree once each, so a fact that changed stales exactly the entries whose layers name it. The expected counts come from an independent Python model of the scenario. @see ../../🧭️script-inputs/🟨️.mjs @see ../../🧭️script-inputs/🧾️layer-index/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
process.env.SEMIO_GRAPH_WALK_FARM="off";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const pythonTool=requireTestToolV1("python");

const inference=await import("../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,entries=await import("../../🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const DIRECTORIES=40,ENTRIES=12,LAYERS_PER_ENTRY=6,TOUCHED=3;
const layerRows=(entry:number,layer:number)=>Array.from({length:10+layer},(_,at)=>(entry*3+layer*2+at)%DIRECTORIES);
const scenario=()=>({entries:Array.from({length:ENTRIES},(_,entry)=>Array.from({length:LAYERS_PER_ENTRY},(_,layer)=>layerRows(entry,layer))),touched:[TOUCHED]});

const model=()=>{
 const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{stdin:Buffer.from(JSON.stringify(scenario()))});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 return JSON.parse(result.stdout.toString()) as {distinctRows:number;rowReferences:number;distinctLayers:number;entries:number;staleEntries:number;staleLayers:number};
};

const prepare=async(prefix:string)=>{
 const root=await repository.scratch(prefix),store=await repository.scratch(prefix+"store-");roots.push(root,store);
 const rows:unknown[]=[];
 for(let at=0;at<DIRECTORIES;at++){
  const directory=join(root,"d"+at);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"f"),String(at));
  const detached=await observation.observeDetachedV1(()=>observation.observedFs.readdirSync(directory));
  rows.push(detached.rows[0]);
 }
 const writer=entries.createClosureStoreV1({directory:store}),recipes:unknown[]=[];
 for(const[at,layers]of scenario().entries.entries()){
  const recipe={fn:"closure-of",entries:["e"+at],extras:[],role:"test-discovery"};recipes.push(recipe);
  await writer.save(root+"\0"+scriptInputs.closureRecipeKeyV1(recipe),{recipe,value:[],rows:[],layers:layers.map(layer=>layer.map(row=>rows[row]))});
 }
 return{root,store,recipes};
};

const process_=async(root:string,store:string,recipes:unknown[],separately=false)=>{
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,store,false);
 const before={probes:observation.observationProbeCountsV1().readdir,layers:scriptInputs.layerVerdictCountsV1(),closure:scriptInputs.closureEntryCountsV1()};
 const owners=Array.from({length:ENTRIES/2},(_,at)=>recipes.slice(at*2,at*2+2));
 const pass=async()=>{
  for(const owner of owners)await observation.withObservationRoundV1(async()=>{
   if(!separately)return scriptInputs.primeSharedClosuresV1(root,{},{recipes:owner});
   for(const recipe of owner)try{await scriptInputs.primeSharedClosuresV1(root,{},{recipes:[recipe]});}catch{}
  });
 };
 await scriptInputs.withClosurePassV1(pass);
 const layers=scriptInputs.layerVerdictCountsV1(),closure=scriptInputs.closureEntryCountsV1();
 return{probes:observation.observationProbeCountsV1().readdir-before.probes,checked:layers.checked-before.layers.checked,indexed:layers.indexed-(before.layers.indexed??0),loaded:closure.loaded-before.closure.loaded,stale:closure.stale-before.closure.stale};
};

test("a layer parsed by an earlier process is decided from its ids, the rows are probed once each and nothing is parsed",async()=>repository.invoke(async()=>{
 const expected=model(),{root,store,recipes}=await prepare("layer-ids-warm-");
 const first=await process_(root,store,recipes),second=await process_(root,store,recipes);
 expect(first.stale).toBe(0);expect(first.loaded).toBe(expected.entries);
 expect(first.checked).toBe(expected.distinctLayers);expect(first.indexed).toBe(0);expect(first.probes).toBe(expected.distinctRows);
 expect(second.stale).toBe(0);expect(second.loaded).toBe(expected.entries);
 expect(second.checked).toBe(0);expect(second.indexed).toBe(expected.distinctLayers);expect(second.probes).toBe(expected.distinctRows);
}));

test("a fact that changed since the earlier process stales exactly the entries whose layers name it",async()=>repository.invoke(async()=>{
 const expected=model(),{root,store,recipes}=await prepare("layer-ids-stale-");
 await process_(root,store,recipes);
 writeFileSync(join(root,"d"+TOUCHED,"added"),"x");
 const second=await process_(root,store,recipes,true);
 expect(expected.staleEntries).toBeGreaterThan(0);expect(expected.staleEntries).toBeLessThan(expected.entries);
 expect(second.stale).toBe(expected.staleEntries);expect(second.loaded).toBe(expected.entries-expected.staleEntries);
 expect(second.indexed).toBeGreaterThan(0);
}));
