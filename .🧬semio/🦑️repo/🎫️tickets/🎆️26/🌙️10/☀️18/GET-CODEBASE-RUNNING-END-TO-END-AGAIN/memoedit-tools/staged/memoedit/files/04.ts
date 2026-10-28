/** ✏️️ Contract: a sealed memo whose journal names a relevant change re-checks exactly the facts that read the changed paths with the planner's own checks and serves when they all hold (an edit that keeps a file's imports, an atomic save, a mode change, a touched directory), and plans for every change the planner would see (an added import, a rename, a raw content change, a derived or TypeScript text fact, a new member or walked directory) and for changes it cannot prove (a path that became a symbolic link, a directory renamed over another). Oracles: the planner's own verdict on the same tree, i.e. the engine's verification of every fact (`observationsHold`) plus the membership advance of the frontier (`advanceFrontierV1`), and, on a real generation store, the planner itself (`refreshClosure` builds or not); the same lookup without the edit ports (the base behaviour) refuses every touched case; a settlement that trusts the rows or skips the frontier is wrong somewhere.
 * @see ../../🟦️.ts `settleMemoEditsV1`
 * @see ../../📤️relevance/🟦️.ts `buildGraphMemoRelevanceV1` */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {appendFileSync,chmodSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {advanceFrontierV1,walkFrontierV1} from "../../../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {encodeGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {createHolds,createWorld,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1,ChangeRowV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {readChangeStateV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1,type ChangeAnswerV1,type ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {startChangeJournalServiceV1} from "../../../../../🏡️resident/📜️journal/🟦️.ts";
import {GRAPH_MEMO_POLICY_V1,collectGraphMemoV1,collectMemoChangesV1,openGraphMemoV1,packMemoRelevanceV1,settleMemoEditsV1,unpackMemoEditIndexV1,unpackMemoRelevanceV1,type GraphMemoEditPortsV1,type MemoFrontierV1,type MemoRowV1} from "../../🟦️.ts";
import {buildGraphMemoRelevanceV1,writeGraphMemoSidecarFilesV1,writeGraphMemoSliceSidecarV1,type MemoBuiltV1} from "../../📤️relevance/🟦️.ts";
import {memoFactsHoldV1} from "../../🚪️command/🟦️.ts";

const EPOCH="5e5e5e5e5e5e5e5e",NAMES=new Set(["node_modules",".git"]),insensitive=process.platform==="darwin"||process.platform==="win32",LATER=60000;
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-edit-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const excluded=(relative:string):boolean=>relative.split("/").some(part=>NAMES.has(part));
const imports=observation.registerSourceAnalysisV1({id:"memo-edit-law:v1",revision:"2".repeat(64),analyze:(_path:string,bytes:Buffer)=>({imports:[...bytes.toString("utf8").matchAll(/^import ["']([^"']+)["'];?$/gmu)].map(match=>match[1]!).sort()})});
const sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
const descend=(name:string)=>!NAMES.has(name),member=(path:string)=>path.slice(path.lastIndexOf("/")+1)==="project.json";

/** 🧪️ The workspace of the engine law: an analysed file and its import, a probed file, an unread sibling, a file read whole, a file a derived fact names, a file read as TypeScript text, two members, a walked directory no fact lists, an excluded region. Directories are settled one minute in the past, so the frontier can stamp them. */
function fixture():{root:string;outside:string}{
 const root=join(sandbox,"w"+(counter++)),outside=root+".ext";
 for(const directory of ["src/sub","lib","docs","node_modules/pkg"])mkdirSync(join(root,directory),{recursive:true});
 mkdirSync(outside,{recursive:true});
 const files:Record<string,string>={"project.json":"{}\n","src/a.ts":"import './b.ts';\nexport const a=1;\n","src/b.ts":"export const b=2;\n","src/notes.md":"notes\n","src/sub/c.ts":"c\n","src/sub/notes.txt":"n\n","src/sub/project.json":"{\"name\":\"sub\"}\n","lib/d.ts":"d\n","lib/e.ts":"e\n","docs/readme.md":"docs\n","node_modules/pkg/index.js":"module.exports=1;\n"};
 for(const [path,text] of Object.entries(files))writeFileSync(join(root,path),text);
 writeFileSync(join(outside,"a.ts"),files["src/a.ts"]!);
 const settled=new Date(Date.now()-LATER);for(const directory of [".","src","src/sub","lib","docs"])utimesSync(join(root,directory),settled,settled);
 return{root,outside};
}
/** 📸️ The facts of one reading session, taken with the real observation module, plus one derived fact naming `lib/d.ts` (the engine never checks a derived fact; the planner asks its question again). */
async function harvest(root:string):Promise<{rows:readonly MemoRowV1[];raw:readonly MemoRowV1[]}>{
 const at=(path:string)=>join(root,path),captured=await observation.captureObservedSourceInputsV1(()=>{
  const fs=observation.observedFs;
  observation.observedSourceAnalysisV1(imports,at("src/a.ts"));fs.existsSync(at("src/a.ts"));fs.existsSync(at("src/b.ts"));fs.lstatSync(at("src/b.ts"));fs.existsSync(at("src/missing.ts"));
  fs.readdirSync(at("src"),{withFileTypes:true});fs.readdirSync(at("src/sub"));fs.readFileSync(at("src/sub/c.ts"),"utf8");fs.readFileSync(at("project.json"),"utf8");
  observation.observedRequireResolve(at("src/a.ts"),"./b.ts");
  (observation.observedTypeScriptHost({readFile:(path:string)=>readFileSync(path,"utf8")} as never) as {readFile(path:string):string|undefined}).readFile(at("lib/e.ts"));
 },undefined,false);
 const raw=captured.observations as unknown as MemoRowV1[];
 return{raw,rows:[...raw,{kind:"derived",path:JSON.stringify(["memo-edit-law:question",at("lib/d.ts")]),fact:"answer"}]};
}
const ports=(root:string)=>({
 list:async(directory:string)=>{try{return readdirSync(join(root,directory),{withFileTypes:true}).map(entry=>({name:String(entry.name),kind:(entry.isSymbolicLink()?"other":entry.isDirectory()?"directory":entry.isFile()?"file":"other") as "directory"|"file"|"other"}));}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=lstatSync(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
 descend:(name:string)=>descend(name),member,
});
const walk=async(root:string)=>({...await walkFrontierV1(ports(root)),capturedAt:Date.now()});
/** 🧭️ The planner's membership verdict: the advance of the frontier answers the same directories and members (with their modes). */
async function membershipKept(root:string,frontier:Awaited<ReturnType<typeof walk>>):Promise<boolean>{
 const advanced=await advanceFrontierV1({...ports(root),frontier});
 if(!advanced)return false;
 if("unchanged" in advanced)return true;
 const key=(rows:readonly (readonly unknown[])[])=>JSON.stringify(rows.map(row=>row.length===2?row:[row[0]]).sort());
 return key(advanced.members)===key(frontier.members)&&key(advanced.directories)===key(frontier.directories);
}

/** 🪞️ The model journal: every entry whose identity changed between two snapshots (ctime included), minus the excluded regions; `advisory` reports every row as 'modified' and drops directory rows, as a coalescing watcher may. */
type Snapshot=Map<string,string>;
function snapshot(root:string):Snapshot{
 const out:Snapshot=new Map(),visit=(directory:string)=>{
  for(const name of readdirSync(join(root,directory))){
   const path=directory?directory+"/"+name:name,status=lstatSync(join(root,path));
   out.set(path,[status.isDirectory()?"d":status.isSymbolicLink()?"l"+readlinkSync(join(root,path)):"f",status.size,status.mtimeMs,status.ctimeMs,status.ino,status.mode].join("|"));
   if(status.isDirectory())visit(path);
  }
 };
 visit("");return out;
}
function journalRows(before:Snapshot,after:Snapshot,advisory:boolean):ChangeRowV1[]{
 const rows:ChangeRowV1[]=[];
 for(const path of new Set([...before.keys(),...after.keys()])){
  if(excluded(path))continue;
  const was=before.get(path),now=after.get(path);if(was===now)continue;
  const directory=(now??was)!.startsWith("d");if(advisory&&directory)continue;
  rows.push([advisory?"m":was===undefined?"a":now===undefined?"r":"m",directory?1:0,path]);
 }
 return rows;
}
class ScriptedJournal implements ChangeJournalClientV1{
 rows:ChangeRowV1[]=[];
 excluder(){return excluded;}
 async cursor(){return{epoch:EPOCH,seq:this.rows.length};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:this.rows.length},rows:this.rows.slice(from.seq)}:{exact:false,reason:"epoch"};}
}

type Mutation={name:string;serve:boolean;apply:(root:string,outside:string)=>void;advisory?:boolean;transient?:readonly string[]};
const atomic=(root:string,path:string,text:string)=>{writeFileSync(join(root,path+".tmp"),text);renameSync(join(root,path+".tmp"),join(root,path));};
const mutations:Mutation[]=[
 {name:"content-only edit of an analysed file (its imports are kept)",serve:true,apply:root=>appendFileSync(join(root,"src/a.ts"),"// a note\n")},
 {name:"edit that adds an import",serve:false,apply:root=>appendFileSync(join(root,"src/a.ts"),"import './sub/c.ts';\n")},
 {name:"atomic save keeping the imports",serve:true,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nexport const a=2;\n")},
 {name:"atomic save adding an import",serve:false,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nimport './sub/c.ts';\nexport const a=1;\n")},
 {name:"atomic save keeping the imports, reported only as 'modified'",serve:true,advisory:true,transient:["src/a.ts.tmp"],apply:root=>atomic(root,"src/a.ts","import './b.ts';\nexport const a=3;\n")},
 {name:"rename the analysed file",serve:false,apply:root=>renameSync(join(root,"src/a.ts"),join(root,"src/a2.ts"))},
 {name:"delete the analysed file",serve:false,apply:root=>unlinkSync(join(root,"src/a.ts"))},
 {name:"rename an unread sibling in a listed directory",serve:false,apply:root=>renameSync(join(root,"src/notes.md"),join(root,"src/notes2.md"))},
 {name:"content edit of an unread sibling",serve:true,apply:root=>appendFileSync(join(root,"src/notes.md"),"more\n")},
 {name:"content-only edit and an unread sibling edit together",serve:true,apply:root=>{appendFileSync(join(root,"src/a.ts"),"// b note\n");appendFileSync(join(root,"src/notes.md"),"more\n");}},
 {name:"content edit of a file read whole",serve:false,apply:root=>appendFileSync(join(root,"src/sub/c.ts"),"more\n")},
 {name:"edit of a file a derived fact names",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"more\n")},
 {name:"edit of a file read as TypeScript text",serve:false,apply:root=>appendFileSync(join(root,"lib/e.ts"),"more\n")},
 {name:"a new member in a walked directory no fact lists",serve:false,apply:root=>writeFileSync(join(root,"docs/project.json"),"{}\n")},
 {name:"a new walked directory",serve:false,apply:root=>mkdirSync(join(root,"docs/more"))},
 {name:"a creation reported only as 'modified'",serve:false,advisory:true,apply:root=>writeFileSync(join(root,"src/ghost.ts"),"g\n")},
 {name:"create the file a probe found missing",serve:false,apply:root=>writeFileSync(join(root,"src/missing.ts"),"m\n")},
 {name:"replace the analysed file by a symbolic link to an identical copy",serve:false,apply:(root,outside)=>{unlinkSync(join(root,"src/a.ts"));symlinkSync(join(outside,"a.ts"),join(root,"src/a.ts"));}},
 {name:"replace a probed file by a symbolic link",serve:false,apply:(root,outside)=>{unlinkSync(join(root,"src/b.ts"));symlinkSync(join(outside,"a.ts"),join(root,"src/b.ts"));}},
 {name:"touch a walked directory",serve:true,apply:root=>{const now=new Date();utimesSync(join(root,"src/sub"),now,new Date(now.getTime()+7000));}},
 {name:"change the mode of the analysed file",serve:true,apply:root=>chmodSync(join(root,"src/a.ts"),0o755)},
 {name:"change the mode of a member",serve:false,apply:root=>chmodSync(join(root,"src/sub/project.json"),0o755)},
 {name:"rename a walked directory",serve:false,apply:root=>renameSync(join(root,"src/sub"),join(root,"src/sub2"))},
 {name:"recreate a walked directory with the same contents (another inode)",serve:false,apply:root=>{rmSync(join(root,"docs"),{recursive:true});mkdirSync(join(root,"docs"));writeFileSync(join(root,"docs/readme.md"),"docs\n");}},
 {name:"nothing",serve:true,apply:()=>{}},
];

const POINTER_MANIFEST="a".repeat(64),INDEX="b".repeat(64),FRONTIER="c".repeat(64);
/** 🏗️ One sealed memo over a fixture: relevance and edit index from the real builder, a record sealed by the real memo, ports that answer rows by position and the walked frontier with the walk predicates. */
async function sealed(root:string){
 const {rows,raw}=await harvest(root),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async relative=>{try{return lstatSync(join(root,relative)).isSymbolicLink();}catch{return false;}}});
 const journal=new ScriptedJournal(),directory=join(sandbox,"memo"+counter++),stem=join(sandbox,"side"+counter++),pointerFile=join(sandbox,"cur"+counter+++".json");
 writeFileSync(pointerFile,JSON.stringify({generation:1,manifest:POINTER_MANIFEST}));
 writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER});
 const edit:GraphMemoEditPortsV1={rows:async(index,ids)=>index===INDEX?ids.map(id=>rows[id]!):undefined,frontier:async digest=>digest===FRONTIER?{frontier,descend:(name:string)=>descend(name),member}:undefined};
 const pin=JSON.stringify({version:1,owner:1,generation:1,manifest:POINTER_MANIFEST,covered:{atMs:1,count:0,owners:""}}),plan="{\"projects\":{}}";
 const open=(extra:Record<string,unknown>={})=>openGraphMemoV1({root,directory,client:journal,key:"e".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>observation.observationsHold(list as never,64,new Set(),true),edit,...extra});
 expect(await (await open()).seal({plan,pin,projects:0})).toBe("light");
 expect(await (await open()).seal({plan,pin,projects:0,sidecar:stem+".json"})).toBe("sealed");
 return{rows,raw,frontier,built,journal,directory,open,edit};
}
/** ⚖️ The planner's verdict on the tree: every raw fact holds by the engine, the membership advance keeps the frontier, and the derived fact's input is the one it read. */
async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:string):Promise<boolean>{
 let same=false;try{same=sha(readFileSync(join(root,"lib/d.ts")))===d;}catch{}
 return await observation.observationsHold(raw as never,64,new Set(),true)&&await membershipKept(root,frontier)&&same;
}

test("the edit index names every row by the entries it reads and survives packing",async()=>{
 const {root}=fixture(),{rows}=await harvest(root),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async()=>false});
 expect(built.unsealable).toBeNull();
 const stem=join(sandbox,"idx"+counter++);writeGraphMemoSidecarFilesV1(stem,{file:"/x",generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER,ids:rows.map((_,at)=>at*10+3)});
 const edits=unpackMemoEditIndexV1(readFileSync(stem+GRAPH_MEMO_POLICY_V1.editSuffix))!;
 expect(edits.insensitive).toBe(insensitive);
 const position=(kind:string,path:string)=>rows.findIndex(row=>row.kind===kind&&row.path===join(root,path));
 expect(edits.named("src/a.ts")).toEqual(expect.arrayContaining([position("analysis","src/a.ts")*10+3,position("exists","src/a.ts")*10+3]));
 expect(edits.inside("src"),"the listing of src reads its children").toContain(position("readdir","src")*10+3);
 expect(edits.above("src")&&edits.above("")&&!edits.above("src/a.ts")).toBe(true);
 expect(edits.frontier("docs")&&edits.frontier("src/sub")&&!edits.frontier("node_modules")).toBe(true);
 expect(edits.member("src/sub/project.json")&&!edits.member("src/a.ts")).toBe(true);
 expect(edits.named("lib/d.ts").length,"the derived fact names its input").toBe(1);
 expect(JSON.parse(readFileSync(stem+".json","utf8")).edit).toEqual({index:INDEX,frontier:FRONTIER});
 expect(unpackMemoEditIndexV1(Buffer.concat([readFileSync(stem+GRAPH_MEMO_POLICY_V1.editSuffix),Buffer.from([0])]))).toBeUndefined();
});

test("soundness and precision against the planner's verdict: each adversarial change is served exactly when the facts that read it hold and the membership is kept, the base behaviour refuses every touched case, and a trusting settlement or one that reads the frontier of the present tree is caught",async()=>{
 let served=0,refused=0,trustingWrong=0,blindWrong=0;
 for(const mutation of mutations){
  const {root,outside}=fixture(),memo=await sealed(root),d=sha(readFileSync(join(root,"lib/d.ts")));
  expect(await planner(root,memo.raw,memo.frontier,d),mutation.name+" (fresh)").toBe(true);
  const before=snapshot(root);mutation.apply(root,outside);
  const rows=[...(mutation.transient??[]).flatMap(path=>mutation.advisory?[["m",0,path] as ChangeRowV1]:[["a",0,path] as ChangeRowV1,["r",0,path] as ChangeRowV1]),...journalRows(before,snapshot(root),mutation.advisory===true)];
  memo.journal.rows.push(...rows);
  const verdict=await planner(root,memo.raw,memo.frontier,d),base=await (await memo.open({edit:undefined})).lookup(),answer=await (await memo.open()).lookup();
  if(process.env.MEMO_LAW_TABLE)console.log("[DEBUG] edit law",mutation.name.padEnd(70),JSON.stringify(rows).slice(0,90).padEnd(92),"memo",answer.hit?"serves":"refuses "+(answer.hit?"":answer.detail),"planner",verdict?"holds":"stale");
  if(answer.hit)expect(verdict,"the memo served what the planner would plan: "+mutation.name).toBe(true);
  expect(answer.hit,mutation.name).toBe(mutation.serve);
  if(rows.some(row=>unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!.holds(row[2])))expect(base.hit,"the base memo refuses a touched entry: "+mutation.name).toBe(false);
  if(answer.hit&&rows.length&&mutation.name.startsWith("content-only"))expect(answer.edited?.rows,mutation.name).toBeGreaterThan(0);
  answer.hit?served++:refused++;
  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!,collected=collectMemoChangesV1(relevance,rows),edits=unpackMemoEditIndexV1(readFileSync(join(memo.directory,(JSON.parse(readFileSync(join(memo.directory,"e".repeat(40)+".json"),"utf8")) as {relevance:{edit:{id:string}}}).relevance.edit.id+GRAPH_MEMO_POLICY_V1.editSuffix)))!;
  const listed=new Map([...collected.pending].map(([directory,row])=>[directory,row[2]]));
  const settle=(over:Partial<Parameters<typeof settleMemoEditsV1>[0]>)=>settleMemoEditsV1({root,relevance,edits,changed:collected.changed,listed,index:INDEX,frontier:FRONTIER,ports:memo.edit,holds:list=>observation.observationsHold(list as never,64,new Set(),true),now:()=>Date.now()+LATER,...over});
  if((await settle({holds:async()=>true})).held&&!verdict)trustingWrong++;
  const blind:GraphMemoEditPortsV1={...memo.edit,frontier:async()=>({frontier:await walk(root),descend:(name:string)=>descend(name),member})};
  if((await settle({ports:blind})).held&&!verdict)blindWrong++;
 }
 expect(served).toBeGreaterThanOrEqual(7);expect(refused).toBeGreaterThanOrEqual(15);
 expect(trustingWrong,"a settlement that does not re-check the rows must be wrong somewhere").toBeGreaterThan(0);
 expect(blindWrong,"a settlement that proves membership against the present tree instead of the sealed frontier must be wrong somewhere").toBeGreaterThan(0);
},180000);

test("a served edit re-seals: the stamp of the directory it listed again is renewed, so the next sibling edit there is settled by the stamp alone, and the journal cursor moves past the edit",async()=>{
 const {root}=fixture(),memo=await sealed(root),recordFile=join(memo.directory,"e".repeat(40)+".json"),record=()=>JSON.parse(readFileSync(recordFile,"utf8")) as {cursor:string;relevance:{stamps:string}};
 const before=snapshot(root),stamps=record().relevance.stamps;atomic(root,"src/sub/c.ts","c\n");
 memo.journal.rows.push(["a",0,"src/sub/c.ts.tmp"],["r",0,"src/sub/c.ts.tmp"],...journalRows(before,snapshot(root),false));
 const first=await (await memo.open()).lookup();
 expect(first.hit,"an atomic save with the same bytes in a stamped directory").toBe(true);
 if(first.hit)expect(first.edited?.rows).toBeGreaterThan(0);
 expect(record().relevance.stamps,"the stamp table was renewed").not.toBe(stamps);
 expect(record().cursor.endsWith(":"+memo.journal.rows.length)||record().cursor.endsWith(String(memo.journal.rows.length))).toBe(true);
 appendFileSync(join(root,"src/sub/notes.txt"),"x\n");memo.journal.rows.push(["m",0,"src/sub/notes.txt"]);
 const second=await (await memo.open()).lookup();
 expect(second.hit).toBe(true);
 if(second.hit)expect(second.edited,"settled by the renewed stamp, no listing again").toBeUndefined();
 collectGraphMemoV1(memo.directory);
 expect(readdirSync(memo.directory).some(name=>name.endsWith(GRAPH_MEMO_POLICY_V1.editSuffix)),"retention keeps the edit index a record names").toBe(true);
},60000);

test("fail closed: a tampered edit index, a record without one, a retired generation, a missing frontier and too many paths or rows refuse",async()=>{
 const {root}=fixture(),memo=await sealed(root),recordFile=join(memo.directory,"e".repeat(40)+".json"),record=JSON.parse(readFileSync(recordFile,"utf8")) as {relevance:{edit:{id:string}}},indexFile=join(memo.directory,record.relevance.edit.id+GRAPH_MEMO_POLICY_V1.editSuffix),saved=readFileSync(indexFile);
 appendFileSync(join(root,"src/a.ts"),"// edit\n");memo.journal.rows.push(["m",0,"src/a.ts"]);
 writeFileSync(indexFile,Buffer.concat([saved,Buffer.from([1])]));expect(await (await memo.open()).lookup()).toEqual({hit:false,reason:"generation"});writeFileSync(indexFile,saved);
 expect(await (await memo.open({edit:{rows:async()=>undefined,frontier:memo.edit.frontier}})).lookup(),"a retired fact index").toEqual({hit:false,reason:"changed",detail:"src/a.ts"});
 const {edit:_drop,...without}=record.relevance as Record<string,unknown>;writeFileSync(recordFile,JSON.stringify({...record,relevance:without}));
 expect(await (await memo.open()).lookup(),"a record sealed without an edit index plans as before").toEqual({hit:false,reason:"changed",detail:"src/a.ts"});
 writeFileSync(recordFile,JSON.stringify(record));
 expect((await (await memo.open()).lookup()).hit).toBe(true);
 const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(memo.built.directories,insensitive,memo.built.strict,memo.built.stamped))!,edits=unpackMemoEditIndexV1(saved)!,base={root,relevance,edits,listed:new Map<string,string>(),index:INDEX,frontier:FRONTIER as string|null,ports:memo.edit,holds:async()=>true,now:()=>Date.now()};
 expect(await settleMemoEditsV1({...base,changed:Array.from({length:GRAPH_MEMO_POLICY_V1.maximumEditPaths+1},(_,at)=>["m",0,"src/a.ts"+at] as ChangeRowV1)})).toEqual({held:false,path:"src/a.ts0"});
 expect(await settleMemoEditsV1({...base,edits:{...edits,named:()=>Array.from({length:GRAPH_MEMO_POLICY_V1.maximumEditRows+1},(_,at)=>at)},changed:[["m",0,"src/a.ts"]]})).toEqual({held:false,path:"src/a.ts"});
 expect(await settleMemoEditsV1({...base,frontier:null,changed:[["m",1,"src/sub"]]}),"a generation without a frontier proves no directory").toEqual({held:false,path:"src/sub"});
 expect(await settleMemoEditsV1({...base,changed:[["m",0,"src/a.ts"]]})).toMatchObject({held:true});
},60000);

test("the planner's own check: environment rows by the environment rule, every other row by the engine, a fact the planner settles by another computation never holds",async()=>{
 const {root}=fixture(),{raw}=await harvest(root);
 expect(await memoFactsHoldV1(root,raw)).toBe(await observation.observationsHold(raw as never,64,new Set(),true));
 writeFileSync(join(root,"src/sub/project.json"),JSON.stringify({name:"sub"}));
 const field={kind:"field",path:"src/sub/project.json",fact:{extract:"project-json",value:{name:"sub",nativeRoot:null}}},text=readFileSync(join(root,"lib/e.ts")),status=lstatSync(join(root,"lib/e.ts")),digest={kind:"digest",path:"lib/e.ts",fact:{sha256:sha(text),size:status.size,mtimeMs:status.mtimeMs}};
 expect(await memoFactsHoldV1(root,[field,digest])).toBe(true);
 writeFileSync(join(root,"src/sub/project.json"),JSON.stringify({name:"other"}));
 expect(await memoFactsHoldV1(root,[field])).toBe(false);
 appendFileSync(join(root,"lib/e.ts"),"x\n");expect(await memoFactsHoldV1(root,[digest])).toBe(false);
 for(const kind of ["derived","closure","stamp","registry"])expect(await memoFactsHoldV1(root,[{kind,path:join(root,"lib/d.ts"),fact:"x"}]),kind).toBe(false);
 expect(await memoFactsHoldV1(root,[])).toBe(true);
},60000);

test("on a real generation store the memo serves an edit exactly when the planner itself verifies the slice without building, and finds the re-checked rows by their position in the generation's fact index",async()=>{
 const dir=join(sandbox,"world"+counter++),journal=new ScriptedJournal(),generations=join(dir,".gen"),exclude={rooted:[".gen",".leases"],names:["node_modules"]};
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a"}),"pkg/a/src/index.ts":"export const a=1;\n","pkg/a/src/util.ts":"export const u=1;\n","pkg/b/project.json":JSON.stringify({name:"b",uses:["pkg/b/data.txt"]}),"pkg/b/data.txt":"one\n","pkg/b/src/index.ts":"export const b=2;\n"});
 const settled=new Date(Date.now()-LATER);for(const directory of ["pkg","pkg/a","pkg/a/src","pkg/b","pkg/b/src"])utimesSync(join(dir,directory),settled,settled);
 const world=createWorld(dir),slice=new Set(["a"]),directory=join(sandbox,"wm"+counter++);
 const store=createGenerationStoreV1({root:generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"law"}});
 const edit:GraphMemoEditPortsV1={rows:async(index,ids)=>{const view=await store.readIndexView(index);return view?ids.map(id=>view.rows[id] as MemoRowV1):undefined;},frontier:async digest=>{const frontier=await store.readFrontier(digest);return frontier?{frontier,descend:()=>true,member:(path:string)=>path.endsWith("/project.json")||path.endsWith(".ts")} as MemoFrontierV1:undefined;}};
 const holds=createHolds(dir),open=(withEdit=true)=>openGraphMemoV1({root:dir,directory,client:journal,key:"w".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>holds(list as never),...(withEdit?{edit}:{})});
 const pinOf=(current:GenerationSealedV1)=>encodeGenerationPinV1({owner:1,generation:current.pointer.generation,manifest:current.pointer.manifest!,owners:Object.keys(current.manifest!.owners).sort(compareCodePointsV1),covered:slice,atMs:Date.now()});
 const g1=(await world.refreshClosure(["a"])).current,plan=canonicalJsonV1(await world.graph(g1,["a"])),pin=pinOf(g1),stem=join(sandbox,"ws"+counter++);
 expect(await (await open()).seal({plan,pin,projects:1})).toBe("light");
 await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin,stem,expected:undefined,client:journal,insensitive});
 expect(JSON.parse(readFileSync(stem+".json","utf8")).edit).toEqual({index:g1.manifest!.factIndex,frontier:g1.manifest!.frontier});
 expect(await (await open()).seal({plan,pin,projects:1,sidecar:stem+".json"})).toBe("sealed");
 const touch=(path:string)=>{const later=new Date(Date.now()+3000);utimesSync(join(dir,path),later,later);journal.rows.push(["m",0,path]);};

 touch("pkg/a/src/index.ts");
 expect(await (await open(false)).lookup(),"base: a touched source refuses").toEqual({hit:false,reason:"changed",detail:"pkg/a/src/index.ts"});
 const touched=await (await open()).lookup();
 expect(touched,"a touched source whose facts hold is served").toMatchObject({hit:true,edited:{paths:1}});
 const oracle1=await world.refreshClosure(["a"]);expect(oracle1.built,"oracle: the planner verifies the slice without building").toBe(false);

 const before=snapshot(dir);writeFileSync(join(dir,"pkg/a/src/util.ts.tmp"),"export const u=1;\n");renameSync(join(dir,"pkg/a/src/util.ts.tmp"),join(dir,"pkg/a/src/util.ts"));
 journal.rows.push(["a",0,"pkg/a/src/util.ts.tmp"],["r",0,"pkg/a/src/util.ts.tmp"],...journalRows(before,snapshot(dir),false).filter(row=>!row[2].startsWith(".")));
 expect((await (await open()).lookup()).hit,"an atomic save with the same bytes is served").toBe(true);
 const oracle2=await world.refreshClosure(["a"]);expect(oracle2.built,"oracle: the planner keeps the generation").toBe(false);

 writeFileSync(join(dir,"pkg/a/src/index.ts"),"import \"@syn/b\";\nexport const a=1;\n");journal.rows.push(["m",0,"pkg/a/src/index.ts"]);
 expect(await (await open()).lookup(),"an edit that adds an import is planned").toEqual({hit:false,reason:"changed",detail:"pkg/a/src/index.ts"});
 const oracle3=await world.refreshClosure(["a"]);expect([oracle3.built,oracle3.computed],"oracle: the planner re-infers the owner").toEqual([true,1]);

 writeFileSync(join(dir,"pkg/b/data.txt"),"two\n");journal.rows.push(["m",0,"pkg/b/data.txt"]);
 const g3=oracle3.current,plan3=canonicalJsonV1(await world.graph(g3,["a"])),pin3=pinOf(g3),stem3=join(sandbox,"ws"+counter++),memo3=await open();
 await writeGraphMemoSliceSidecarV1({root:dir,generations,exclude,pin:pin3,stem:stem3,expected:undefined,client:journal,insensitive});
 expect(await memo3.seal({plan:plan3,pin:pin3,projects:1,sidecar:stem3+".json"})).toBe("sealed");
 writeFileSync(join(dir,"pkg/a/src/new.ts"),"export const n=1;\n");journal.rows.push(["a",0,"pkg/a/src/new.ts"]);
 const created=await (await open()).lookup();expect(created.hit,"a new source in a listed directory is planned").toBe(false);
 const oracle4=await world.refreshClosure(["a"]);expect(oracle4.built,"oracle: the planner builds").toBe(true);
},120000);

const watcher=process.env.SEMIO_REPO_WATCH_BINARY??"";
test.skipIf(!watcher)("with a REAL file watcher: every served edit is one the planner keeps, and an edit that keeps the imports, an atomic save and a touched directory are served",async()=>{
 const wanted=new Set(["content-only edit of an analysed file (its imports are kept)","edit that adds an import","atomic save keeping the imports","rename the analysed file","content edit of a file read whole","a new member in a walked directory no fact lists","replace the analysed file by a symbolic link to an identical copy","touch a walked directory","change the mode of the analysed file","rename a walked directory"]),must=new Set(["content-only edit of an analysed file (its imports are kept)","atomic save keeping the imports","touch a walked directory"]);
 let served=0,refused=0;
 for(const mutation of mutations.filter(item=>wanted.has(item.name))){
  const {root,outside}=fixture(),journalDirectory=root+".journal",environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
  try{
   const state:{rows:readonly MemoRowV1[];frontier?:Awaited<ReturnType<typeof walk>>}={rows:[]},stem=join(sandbox,"real"+counter++),pointerFile=join(sandbox,"rc"+counter++);
   const edit:GraphMemoEditPortsV1={rows:async(_index,ids)=>ids.map(id=>state.rows[id]!),frontier:async()=>state.frontier&&{frontier:state.frontier,descend:(name:string)=>descend(name),member}};
   const client=createChangeJournalClientV1({directory:journalDirectory,root,environment,start:()=>startChangeJournalServiceV1({directory:journalDirectory,root,environment})}),open=()=>openGraphMemoV1({root,directory:join(journalDirectory,"memo"),client,key:"r".repeat(64),insensitive,now:()=>Date.now()+LATER,holds:list=>observation.observationsHold(list as never,64,new Set(),true),edit});
   const pin=JSON.stringify({version:1,owner:1,generation:1,manifest:POINTER_MANIFEST,covered:{atMs:1,count:0,owners:""}}),first=await open();expect(first.cursor,mutation.name).toBeDefined();
   expect(await first.seal({plan:"{\"projects\":{}}",pin,projects:0}),mutation.name).toBe("light");
   const sealing=await open(),{rows,raw}=await harvest(root),frontier=await walk(root),d=sha(readFileSync(join(root,"lib/d.ts")));state.rows=rows;state.frontier=frontier;
   const built:MemoBuiltV1=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier,excluded,isLink:async relative=>{try{return lstatSync(join(root,relative)).isSymbolicLink();}catch{return false;}}});
   writeFileSync(pointerFile,JSON.stringify({generation:1,manifest:POINTER_MANIFEST}));writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:1,manifest:POINTER_MANIFEST},built,insensitive,{index:INDEX,frontier:FRONTIER});
   expect(await sealing.seal({plan:"{\"projects\":{}}",pin,projects:0,sidecar:stem+".json"}),mutation.name).toBe("sealed");
   mutation.apply(root,outside);
   const verdict=await planner(root,raw,frontier,d),answer=await (await open()).lookup();
   if(process.env.MEMO_LAW_TABLE)console.log("[DEBUG] real",mutation.name.padEnd(70),"memo",answer.hit?"serves":"refuses "+answer.reason+" "+(answer.detail??""),"planner",verdict?"holds":"stale");
   if(answer.hit){expect(verdict,"the memo served what the planner would plan: "+mutation.name).toBe(true);served++;}else refused++;
   if(must.has(mutation.name))expect(answer.hit,"served with real journal rows: "+mutation.name).toBe(true);
  }finally{const pid=readChangeStateV1(journalDirectory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}rmSync(journalDirectory,{recursive:true,force:true});}
 }
 expect(served).toBeGreaterThanOrEqual(3);expect(refused).toBeGreaterThanOrEqual(5);
},300000);
