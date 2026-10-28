/** 🧳️ Contract: a sealed graph-memo slice survives the refresh of owners OUTSIDE it. Another request re-infers an unrelated owner and publishes a newer generation (the generation pointer moves); the memo of the slice is carried to that generation and served, because every owner of the slice and every generation-wide digest is identical there, while a change of an owner of the slice refuses (journal first, carry rule second). Oracles: the generation store's own answer for the slice (the planner re-verifies it without building and assembles the same projects from the newer generation), and an independent Python model of the carry rule over an authored corpus.
 * @see ../../🧳️carry/🟦️.ts */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {decodeVerifiedScopeV1} from "../../../../../🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs";
import {encodeGenerationPinV1,readGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {createWorld,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1,ChangeRowV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {openGraphMemoV1} from "../../🟦️.ts";
import {writeGraphMemoSliceSidecarV1} from "../../📤️relevance/🟦️.ts";
import {carryGraphMemoPinV1,graphMemoSliceSameV1,type MemoCarryManifestV1} from "../../🧳️carry/🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};

const here=dirname(fileURLToPath(import.meta.url)),EPOCH="0a1b2c3d4e5f6071",exclude={rooted:[".gen",".leases"],names:["node_modules"]},insensitive=process.platform==="darwin"||process.platform==="win32";
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-carry-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

/** 📓️ A journal that reports exactly the rows the law pushes, in order. */
class ScriptedJournal implements ChangeJournalClientV1{
 rows:ChangeRowV1[]=[];
 excluder(){return(relative:string)=>relative.split("/").some(part=>exclude.names.includes(part))||exclude.rooted.some(path=>relative===path||relative.startsWith(path+"/"));}
 async cursor(){return{epoch:EPOCH,seq:this.rows.length};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:this.rows.length},rows:this.rows.slice(from.seq)}:{exact:false,reason:"epoch"};}
}

const pinOf=(current:GenerationSealedV1,names:ReadonlySet<string>)=>encodeGenerationPinV1({owner:1,generation:current.pointer.generation,manifest:current.pointer.manifest!,owners:Object.keys(current.manifest!.owners).sort(compareCodePointsV1),covered:names,atMs:Date.now()});

test("the carry rule answers as the independent Python model on every authored case",()=>{
 const answers=cases.cases.map(row=>graphMemoSliceSameV1(row.from as MemoCarryManifestV1,row.to as MemoCarryManifestV1,row.covered));
 expect(answers).toEqual(cases.cases.map(row=>row.expected));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),join(here,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(answers);
});

test("a refresh outside the slice moves the pointer; the sealed slice is carried to the newer generation and served; a change of the slice refuses",async()=>{
 const dir=join(sandbox,"w"+counter++);
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a"}),"pkg/a/src/index.ts":"export const a=1;\n","pkg/b/project.json":JSON.stringify({name:"b",uses:["pkg/b/data.txt"]}),"pkg/b/data.txt":"one\n","pkg/b/src/index.ts":"export const b=2;\n"});
 const settled=new Date(Date.now()-60000);for(const directory of ["pkg","pkg/a","pkg/a/src","pkg/b","pkg/b/src"])utimesSync(join(dir,directory),settled,settled);
 const world=createWorld(dir),generations=join(dir,".gen"),journal=new ScriptedJournal();
 const cold=await world.refreshClosure(["a"]);expect(cold.published).toBe(true);
 const g1=cold.current,slice=new Set(["a"]),plan=canonicalJsonV1(await world.graph(g1,["a"])),stem=join(sandbox,"s"+counter++);
 await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin:pinOf(g1,slice),stem,client:journal,insensitive});
 expect((JSON.parse(readFileSync(stem+".json","utf8")) as {unsealable:string|null}).unsealable).toBeNull();
 let carries=0;
 const memo=(withCarry:boolean)=>openGraphMemoV1({root:dir,directory:join(sandbox,"memo"),client:journal,key:"c".repeat(64),insensitive,environment:{SEMIO_BUILD_INVOCATION_PID:"777"},holds:rows=>observation.observationsHold(rows as never,64,new Set(),true),
  ...(withCarry?{carry:async input=>{carries++;return carryGraphMemoPinV1({generations,...input});}}:{})});
 expect(await (await memo(true)).seal({plan,pin:pinOf(g1,slice),projects:1})).toBe("light");
 expect(await (await memo(true)).seal({plan,pin:pinOf(g1,slice),projects:1,sidecar:stem+".json"})).toBe("sealed");
 const first=await (await memo(true)).lookup();
 expect(first.hit&&first.carried).toBe(undefined);expect(carries).toBe(0);

 writeFileSync(join(dir,"pkg/b/data.txt"),"two\n");journal.rows.push(["m",0,"pkg/b/data.txt"]);
 const g2=(await world.refreshClosure(["b"]));
 expect(g2.built).toBe(true);expect(g2.computed,"only b is re-inferred").toBe(1);
 expect(g2.current.pointer.generation).toBeGreaterThan(g1.pointer.generation);
 expect(await (await memo(false)).lookup(),"negative control: without the carry a moved pointer refuses (the memo before this law)").toEqual({hit:false,reason:"pointer"});
 const carried=await (await memo(true)).lookup();
 expect(carried).toMatchObject({hit:true});expect(carries).toBe(1);
 if(!carried.hit)return;
 expect(carried.carried).toEqual({from:g1.pointer.generation,to:g2.current.pointer.generation});
 const pin=readGenerationPinV1(carried.pin!);
 expect(pin.owner).toBe(777);expect(pin.generation).toBe(g2.current.pointer.generation);expect(pin.manifest).toBe(g2.current.pointer.manifest!);
 expect([...decodeVerifiedScopeV1(Object.keys(g2.current.manifest!.owners).sort(compareCodePointsV1),pin.covered)]).toEqual(["a"]);
 const planner=await world.refreshClosure(["a"]);
 expect(planner.built,"oracle: the planner verifies the slice without building").toBe(false);
 expect(planner.current.pointer.manifest,"oracle: and serves the generation the memo was carried to").toBe(g2.current.pointer.manifest);
 expect(carried.plan,"oracle: the slice assembled from the newer generation is the plan the memo serves").toBe(canonicalJsonV1(await world.graph(g2.current,["a"])));
 expect((await (await memo(true)).lookup()).hit).toBe(true);expect(carries,"the record now names the newer generation").toBe(1);

 writeFileSync(join(dir,"pkg/a/src/index.ts"),"export const a=9;\n");journal.rows.push(["m",0,"pkg/a/src/index.ts"]);
 const g3=await world.refreshClosure(["a"]);expect(g3.built).toBe(true);
 expect(await (await memo(true)).lookup(),"a change of the slice is a journal change before any carry").toEqual({hit:false,reason:"changed",detail:"pkg/a/src/index.ts"});
 expect(carries).toBe(1);
 const g2pin=pinOf(g2.current,slice),to={generation:g3.current.pointer.generation,manifest:g3.current.pointer.manifest!},from={generation:g2.current.pointer.generation,manifest:g2.current.pointer.manifest!};
 expect(await carryGraphMemoPinV1({generations,from,to,pin:g2pin}),"the carry rule alone refuses a changed owner of the slice").toEqual({carried:false,reason:"owner:a"});
 expect(await carryGraphMemoPinV1({generations,from,to,pin:pinOf(g2.current,new Set(["b"]))}),"while a slice of b alone is carried").toMatchObject({carried:true});
 expect(await carryGraphMemoPinV1({generations,from:{generation:1,manifest:"e".repeat(64)},to,pin:g2pin}),"a pin of another manifest").toEqual({carried:false,reason:"pin"});
 expect(await carryGraphMemoPinV1({generations,from,to:{generation:to.generation+1,manifest:to.manifest},pin:g2pin}),"a pointer that moved again").toEqual({carried:false,reason:"moved"});
 rmSync(join(generations,GENERATION_POLICY_V1.manifests,from.manifest+".json"));
 expect(await carryGraphMemoPinV1({generations,from,to,pin:g2pin}),"a retired generation").toEqual({carried:false,reason:"retired "+from.generation});
},60000);
