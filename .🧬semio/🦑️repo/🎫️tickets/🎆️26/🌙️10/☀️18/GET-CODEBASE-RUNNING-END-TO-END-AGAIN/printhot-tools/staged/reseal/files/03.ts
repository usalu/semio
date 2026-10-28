/** 🔂️ Contract: the run after a light record seals although another request published a newer generation in between, when the slice is the same there (the carry rule); a changed slice still leaves only a light record. This is the gate's print sequence of G36: print1 kept a light record of generation 633, a course build published 634 (7 owners outside print's slice), print2 was refused `other-generation` and kept only another light record, so print's memo served at the earliest on the fourth run. Oracles: the generation store itself (the planner verifies the slice in the newer generation without building, and assembles the identical slice from both generations; after an edit of the slice it builds and the slices differ).
 * @see ../../📤️relevance/🟦️.ts `writeGraphMemoSliceSidecarV1`
 * @see ../../🧳️carry/🟦️.ts `graphMemoSliceSameV1` */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {encodeGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {createWorld,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1,ChangeRowV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {openGraphMemoV1} from "../../🟦️.ts";
import {writeGraphMemoSliceSidecarV1} from "../../📤️relevance/🟦️.ts";

const EPOCH="1b2c3d4e5f607182",exclude={rooted:[".gen",".leases"],names:["node_modules"]},insensitive=process.platform==="darwin"||process.platform==="win32";
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-reseal-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

/** 📓️ A journal that reports exactly the rows the law pushes, in order. */
class ScriptedJournal implements ChangeJournalClientV1{
 rows:ChangeRowV1[]=[];
 excluder(){return(relative:string)=>relative.split("/").some(part=>exclude.names.includes(part))||exclude.rooted.some(path=>relative===path||relative.startsWith(path+"/"));}
 async cursor(){return{epoch:EPOCH,seq:this.rows.length};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:this.rows.length},rows:this.rows.slice(from.seq)}:{exact:false,reason:"epoch"};}
}
const pinOf=(current:GenerationSealedV1,names:ReadonlySet<string>)=>encodeGenerationPinV1({owner:1,generation:current.pointer.generation,manifest:current.pointer.manifest!,owners:Object.keys(current.manifest!.owners).sort(compareCodePointsV1),covered:names,atMs:Date.now()});

test("a light record of an older generation whose slice is unchanged lets the next run seal and the third serve; a changed slice keeps it light",async()=>{
 const dir=join(sandbox,"w"+counter++);
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a"}),"pkg/a/src/index.ts":"export const a=1;\n","pkg/b/project.json":JSON.stringify({name:"b",uses:["pkg/b/data.txt"]}),"pkg/b/data.txt":"one\n","pkg/b/src/index.ts":"export const b=2;\n"});
 const settled=new Date(Date.now()-60000);for(const directory of ["pkg","pkg/a","pkg/a/src","pkg/b","pkg/b/src"])utimesSync(join(dir,directory),settled,settled);
 const world=createWorld(dir),generations=join(dir,".gen"),journal=new ScriptedJournal(),slice=new Set(["a"]),directory=join(sandbox,"memo"+counter++);
 const memo=()=>openGraphMemoV1({root:dir,directory,client:journal,key:"r".repeat(64),insensitive,holds:rows=>observation.observationsHold(rows as never,64,new Set(),true)});
 const sidecar=async(pin:string,expected:string|undefined)=>{const stem=join(sandbox,"s"+counter++);await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin,stem,expected,client:journal,insensitive});return{stem,unsealable:(JSON.parse(readFileSync(stem+".json","utf8")) as {unsealable:string|null}).unsealable};};

 const g1=(await world.refreshClosure(["a"])).current,plan1=canonicalJsonV1(await world.graph(g1,["a"]));
 expect(await (await memo()).seal({plan:plan1,pin:pinOf(g1,slice),projects:1}),"run 1 keeps a light record").toBe("light");

 writeFileSync(join(dir,"pkg/b/data.txt"),"two\n");journal.rows.push(["m",0,"pkg/b/data.txt"]);
 const peer=await world.refreshClosure(["b"]);expect(peer.computed,"another request re-infers only b").toBe(1);
 const g2=peer.current;expect(g2.pointer.generation).toBeGreaterThan(g1.pointer.generation);
 const planner=await world.refreshClosure(["a"]);
 expect(planner.built,"oracle: the planner serves the slice of a in the newer generation without building").toBe(false);
 expect(canonicalJsonV1(await world.graph(g2,["a"])),"oracle: the slice of a assembled from both generations is identical").toBe(plan1);

 const run2=await memo();expect(await run2.lookup()).toEqual({hit:false,reason:"light"});
 expect(run2.expected(),"the light record still names the older generation").toBe(g1.pointer.manifest!);
 const side=await sidecar(pinOf(g2,slice),run2.expected());
 expect(side.unsealable,"the child seals the unchanged slice of the newer generation (base: other-generation)").toBeNull();
 expect(await run2.seal({plan:plan1,pin:pinOf(g2,slice),projects:1,sidecar:side.stem+".json"}),"run 2 seals").toBe("sealed");
 const run3=await (await memo()).lookup();
 expect(run3).toMatchObject({hit:true,projects:1});

 writeFileSync(join(dir,"pkg/a/project.json"),JSON.stringify({name:"a",package:"@syn/a"}));journal.rows.push(["m",0,"pkg/a/project.json"]);
 const edited=await world.refreshClosure(["a"]);expect(edited.built,"oracle: an edit of the slice makes the planner build").toBe(true);
 const g3=edited.current;expect(canonicalJsonV1(await world.graph(g3,["a"])),"oracle: and the slices differ").not.toBe(plan1);
 expect((await sidecar(pinOf(g3,slice),g2.pointer.manifest!)).unsealable,"a changed slice in the newer generation keeps the record light").toBe("other-generation");
 expect((await sidecar(pinOf(g3,slice),"e".repeat(64))).unsealable,"an expected generation that is not retained keeps it light").toBe("other-generation");
 expect((await sidecar(pinOf(g3,slice),g3.pointer.manifest!)).unsealable,"the same generation seals as before").toBeNull();
 expect((await sidecar(pinOf(g3,slice),undefined)).unsealable,"no expectation (no record) seals as before").toBeNull();
 rmSync(join(generations,GENERATION_POLICY_V1.manifests,g1.pointer.manifest!+".json"));
 expect((await sidecar(pinOf(g3,new Set(["b"])),g1.pointer.manifest!)).unsealable,"a retired expected generation keeps it light").toBe("other-generation");
},60000);
