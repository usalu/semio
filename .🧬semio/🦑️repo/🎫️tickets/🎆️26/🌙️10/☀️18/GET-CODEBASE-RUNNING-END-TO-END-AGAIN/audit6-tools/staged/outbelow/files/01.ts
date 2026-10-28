import {afterAll,beforeAll,expect,test} from "bun:test";
import {appendFileSync,chmodSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import observationSchema from "../../../../👁️observation/🧬️schema/🔣️.json";
import * as observation from "../../../../👁️observation/🟨️.mjs";
import {formatChangeCursorV1,type ChangeCursorV1,type ChangeRowV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1,type ChangeAnswerV1,type ChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {GRAPH_MEMO_POLICY_V1,classifyMemoChangesV1,collectGraphMemoV1,firstRelevantChangeV1,graphMemoEnabledV1,graphMemoKeyV1,memoClassesV1,memoDirectoriesV1,memoExcluderV1,memoIdentityHoldsV1,memoReachV1,memoStampsHoldV1,openGraphMemoV1,packMemoRelevanceV1,packMemoStampsV1,renderGraphMemoTextV1,unpackMemoRelevanceV1,type MemoRecheckV1,type MemoRowV1} from "../🟦️.ts";
import {buildGraphMemoRelevanceV1,writeGraphMemoSidecarFilesV1,type MemoBuiltV1} from "../📤️relevance/🟦️.ts";
import {openGraphMemoForCommandV1} from "../🚪️command/🟦️.ts";
import {startChangeJournalServiceV1} from "../../../../🏡️resident/📜️journal/🟦️.ts";
import {readChangeStateV1,writeChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),EPOCH="0123456789abcdef",NAMES=new Set(["node_modules","dist",".git"]);
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
let counter=0;
const excluded=(relative:string):boolean=>relative.split("/").some(part=>NAMES.has(part));
const provider=observation.registerSourceAnalysisV1({id:"memo-law:v1",revision:"1".repeat(64),analyze:(_path:string,bytes:Buffer)=>({length:bytes.length})});

/** 🧪️ A workspace of the shape the laws need: mentioned directories, an unmentioned one, excluded regions, a link inside and a link to a directory outside the root. */
function fixture():{root:string;outside:string}{
 const root=join(sandbox,"w"+(counter++)),outside=root+".ext";
 for(const directory of ["src/sub/deep","lib/.🧾️store","docs","other","node_modules/pkg","dist"])mkdirSync(join(root,directory),{recursive:true});
 mkdirSync(outside,{recursive:true});
 const files:Record<string,string>={"src/a.ts":"export const a=1;\n","src/b.ts":"export const b=2;\n","src/sub/c.ts":"c\n","src/sub/deep/d.ts":"d\n","lib/e.ts":"export const e=3;\n","docs/readme.md":"# docs\n","other/f.ts":"f\n","node_modules/pkg/package.json":"{\"main\":\"index.js\"}\n","node_modules/pkg/index.js":"module.exports=1;\n","dist/out.js":"out\n","dist/gen.ts":"gen\n","src/extra.ts":"x\n","src/sub/extra.ts":"x\n","src/sub/deep/extra.ts":"x\n","lib/extra.ts":"x\n","lib/.🧾️store/build.json":"{}\n"};
 for(const [path,text] of Object.entries(files))writeFileSync(join(root,path),text);
 writeFileSync(join(outside,"z1.ts"),"z1\n");
 symlinkSync("src",join(root,"link"));symlinkSync(outside,join(root,"link2"));
 return{root,outside};
}
/** 📸️ Harvests the facts of one fixed reading session with the real observation module: every kind a graph generation holds from the file system. */
async function harvest(root:string):Promise<readonly MemoRowV1[]>{
 const at=(path:string)=>join(root,path),captured=await observation.captureObservedSourceInputsV1(()=>{
  const fs=observation.observedFs;
  fs.readFileSync(at("src/a.ts"),"utf8");fs.readFileSync(at("src/sub/c.ts"),"utf8");fs.existsSync(at("src/b.ts"));fs.existsSync(at("src/missing.ts"));fs.statSync(at("src/sub"));fs.lstatSync(at("src/sub/deep/d.ts"));
  fs.readdirSync(at("src"));fs.readdirSync(at("src/sub"));fs.readdirSync(at("src/sub/deep"),{withFileTypes:true});fs.readdirSync(at("lib/.🧾️store"));fs.realpathSync(at("src"));
  observation.observedRequireResolve(at("src/a.ts"),"./b.ts");try{observation.observedRequireResolve(at("src/a.ts"),"pkg");}catch{}
  fs.existsSync(at("node_modules/pkg/index.js"));fs.existsSync(at("dist/out.js"));fs.existsSync(at("link/a.ts"));fs.readdirSync(at("link"));fs.existsSync(at("link2/z1.ts"));fs.existsSync(at("link2/z2.ts"));
  observation.observedSourceAnalysisV1(provider,at("lib/e.ts"));observation.observedSourceAnalysisV1(provider,at("dist/gen.ts"));
 },undefined,false);
 return captured.observations as unknown as MemoRowV1[];
}
const frontierOf=(root:string)=>{
 const directories=[".","src","src/sub","src/sub/deep","lib","lib/.🧾️store"].map(path=>{const status=lstatSync(join(root,path));return[path,status.mtimeMs,status.ino] as [string,number,number];}),members:[string,number][]=[];
 for(const directory of directories)for(const entry of readdirSync(join(root,directory[0]),{withFileTypes:true}))if(entry.isFile()&&/^(a|e)\.ts$/u.test(entry.name))members.push([directory[0]==="."?entry.name:directory[0]+"/"+entry.name,0]);
 return{directories,members,capturedAt:Date.now()+60000};
};
const isLink=async(root:string,relative:string):Promise<boolean>=>{try{return lstatSync(join(root,relative)).isSymbolicLink();}catch{return false;}};
const build=(root:string,rows:readonly MemoRowV1[]):Promise<MemoBuiltV1>=>buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier:frontierOf(root),excluded,isLink:relative=>isLink(root,relative)});
const hold=(rows:readonly MemoRowV1[])=>observation.observationsHold(rows as never,64,new Set(),true);

/** 🪞️ The model journal: the exact set of changed entries between two snapshots of the tree (identity includes ctime, so a rewrite that restores the modification time is still a change), minus the regions a journal never reports. */
type Snapshot=Map<string,string>;
function snapshot(root:string):Snapshot{
 const out:Snapshot=new Map(),walk=(directory:string)=>{
  for(const name of readdirSync(join(root,directory))){
   const path=directory?directory+"/"+name:name,status=lstatSync(join(root,path)),link=status.isSymbolicLink()?"->"+readFileSyncLink(join(root,path)):"";
   out.set(path,[status.isDirectory()?"d":status.isSymbolicLink()?"l":"f",status.size,status.mtimeMs,status.ctimeMs,status.ino,status.mode,link].join("|"));
   if(status.isDirectory())walk(path);
  }
 };
 walk("");return out;
}
const readFileSyncLink=(path:string):string=>{try{return String(readlinkSync(path));}catch{return "?";}};
function journalRows(before:Snapshot,after:Snapshot,advisory?:"m"):ChangeRowV1[]{
 const rows:ChangeRowV1[]=[];
 for(const path of new Set([...before.keys(),...after.keys()])){
  if(excluded(path))continue;
  const was=before.get(path),now=after.get(path);
  if(was===now)continue;
  if(advisory&&(now??was)!.startsWith("d"))continue;
  rows.push([advisory??(was===undefined?"a":now===undefined?"r":"m"),(now??was)!.startsWith("d")?1:0,path]);
 }
 return rows;
}

type Mutation={name:string;apply:(root:string,outside:string)=>void;visible:boolean;advisory?:"m"};
const rewrite=(path:string,text:string):void=>{const before=lstatSync(path);writeFileSync(path,text);utimesSync(path,before.atime,before.mtime);};
const mutations:Mutation[]=[
 {name:"rewrite with the same size and the old mtime",visible:true,apply:root=>rewrite(join(root,"src/a.ts"),"export const a=9;\n")},
 {name:"append to a nested file",visible:true,apply:root=>appendFileSync(join(root,"src/sub/c.ts"),"more\n")},
 {name:"create a sibling in a mentioned directory",visible:true,apply:root=>writeFileSync(join(root,"src/new.ts"),"n\n")},
 {name:"create a file a probe found missing",visible:true,apply:root=>writeFileSync(join(root,"src/missing.ts"),"m\n")},
 {name:"delete a probed file",visible:true,apply:root=>unlinkSync(join(root,"src/b.ts"))},
 {name:"rename a file",visible:true,apply:root=>renameSync(join(root,"src/b.ts"),join(root,"src/b2.ts"))},
 {name:"rename a directory",visible:true,apply:root=>renameSync(join(root,"src/sub"),join(root,"src/sub2"))},
 {name:"atomic save over a file",visible:true,apply:root=>{writeFileSync(join(root,"src/a.ts.tmp"),"export const a=1;\n");renameSync(join(root,"src/a.ts.tmp"),join(root,"src/a.ts"));}},
 {name:"retarget a link",visible:true,apply:root=>{unlinkSync(join(root,"link"));symlinkSync("lib",join(root,"link"));}},
 {name:"case-only rename",visible:true,apply:root=>renameSync(join(root,"src/a.ts"),join(root,"src/A.ts"))},
 {name:"change the mode of a file",visible:true,apply:root=>chmodSync(join(root,"src/a.ts"),0o755)},
 {name:"touch a directory",visible:true,apply:root=>{const now=new Date();utimesSync(join(root,"src/sub"),now,new Date(now.getTime()+7000));}},
 {name:"create a file in a link target outside the root",visible:false,apply:(_root,outside)=>writeFileSync(join(outside,"z2.ts"),"z2\n")},
 {name:"delete a file in a link target outside the root",visible:false,apply:(_root,outside)=>unlinkSync(join(outside,"z1.ts"))},
 {name:"delete a probed file in an excluded region",visible:false,apply:root=>unlinkSync(join(root,"node_modules/pkg/index.js"))},
 {name:"delete a probed file in dist",visible:false,apply:root=>unlinkSync(join(root,"dist/out.js"))},
 {name:"rewrite an analysed file in dist",visible:false,apply:root=>appendFileSync(join(root,"dist/gen.ts"),"more\n")},
 {name:"create a file in an unmentioned directory",visible:true,apply:root=>writeFileSync(join(root,"docs/new.md"),"x\n")},
 {name:"rewrite a file in an unmentioned directory",visible:true,apply:root=>appendFileSync(join(root,"docs/readme.md"),"x\n")},
 {name:"create a nested directory in an unmentioned one",visible:true,apply:root=>{mkdirSync(join(root,"other/deep/er"),{recursive:true});writeFileSync(join(root,"other/deep/er/g.ts"),"g\n");}},
 {name:"create a file in an excluded region",visible:false,apply:root=>writeFileSync(join(root,"dist/new.js"),"x\n")},
 {name:"nothing",visible:true,apply:()=>{}},
 {name:"rewrite an unread sibling in a strict directory (a resolution names it)",visible:true,apply:root=>appendFileSync(join(root,"src/extra.ts"),"more\n")},
 {name:"rewrite an unread sibling in a stamped directory",visible:true,apply:root=>appendFileSync(join(root,"src/sub/extra.ts"),"more\n")},
 {name:"touch an unread sibling in a stamped directory",visible:true,apply:root=>{const now=new Date();utimesSync(join(root,"src/sub/extra.ts"),now,new Date(now.getTime()+9000));}},
 {name:"rewrite an unread sibling in a stamped directory no fact lists",visible:true,apply:root=>appendFileSync(join(root,"lib/extra.ts"),"more\n")},
 {name:"remove and recreate an unread sibling in a stamped directory",visible:true,apply:root=>{unlinkSync(join(root,"src/sub/extra.ts"));writeFileSync(join(root,"src/sub/extra.ts"),"x\n");}},
 {name:"replace an unread sibling by a directory (the typed listing changes)",visible:true,apply:root=>{unlinkSync(join(root,"src/sub/deep/extra.ts"));mkdirSync(join(root,"src/sub/deep/extra.ts"));}},
 {name:"a creation the journal reports only as modified (a coalesced create and remove)",visible:true,advisory:"m",apply:root=>writeFileSync(join(root,"src/sub/ghost.ts"),"g\n")},
 {name:"a removal the journal reports only as modified",visible:true,advisory:"m",apply:root=>unlinkSync(join(root,"src/sub/extra.ts"))},
 {name:"a build rewrites its output the way the store does: lock directory, temp file, rename over the old one",visible:true,apply:root=>{const store=join(root,"lib/.🧾️store");mkdirSync(join(store,"build.json.lock"));writeFileSync(join(store,"build.json.tmp"),"{\"n\":2}\n");renameSync(join(store,"build.json.tmp"),join(store,"build.json"));rmSync(join(store,"build.json.lock"),{recursive:true});}},
 {name:"a build leaves a second output file in its output directory",visible:true,apply:root=>writeFileSync(join(root,"lib/.🧾️store/other.json"),"{}\n")},
 {name:"a build removes its output file",visible:true,apply:root=>unlinkSync(join(root,"lib/.🧾️store/build.json"))},
];

test("the policy is closed under its schema, its table names every kind the observation schema declares, and both languages carry the same fields",()=>{
 const ajv=new Ajv({strict:false});
 expect(GRAPH_MEMO_POLICY_V1 as unknown).toEqual(schema.definitions.Policy.const);
 const kinds=new Set(Object.keys(GRAPH_MEMO_POLICY_V1.relevance)),declared=(observationSchema as unknown as {$defs:{observation:{properties:{kind:{enum:string[]}}}}}).$defs.observation.properties.kind.enum;
 for(const kind of declared)expect(kinds.has(kind),kind).toBe(true);
 for(const kind of ["field","digest","derived"])expect(kinds.has(kind),kind).toBe(true);
 for(const how of Object.values(GRAPH_MEMO_POLICY_V1.relevance))expect(["path","analysis","link","resolve","text","none"]).toContain(how);
 for(const kind of [...GRAPH_MEMO_POLICY_V1.stamped,...GRAPH_MEMO_POLICY_V1.following])expect(GRAPH_MEMO_POLICY_V1.relevance[kind],kind).toBe("path");
 expect(GRAPH_MEMO_POLICY_V1.stamped.filter(kind=>GRAPH_MEMO_POLICY_V1.following.includes(kind))).toEqual([]);
 const fieldsOf=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
 for(const key of ["served","refused"] as const)expect(fieldsOf(GRAPH_MEMO_POLICY_V1.texts.de![key]),key).toEqual(fieldsOf(GRAPH_MEMO_POLICY_V1.texts.en![key]));
 expect(renderGraphMemoTextV1("served",{projects:3,seconds:4,rows:5,rechecked:6},"de")).toContain("3 Projekte");
 expect(renderGraphMemoTextV1("served",{projects:3,seconds:4,rows:5,rechecked:6},"xx")).toContain("3 projects");
 const record={version:1,key:"a".repeat(64),cursor:EPOCH+":7",sealedAtMs:1,state:"sealed",manifest:"b".repeat(64),pointer:{file:"/x/current.json",generation:1,manifest:"b".repeat(64)},relevance:{id:"c".repeat(64),stamps:"e".repeat(64),rows:10,rechecked:2},plan:{sha256:"d".repeat(64),bytes:3},pin:"{}",projects:1};
 const check=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Record"});
 expect(check(record),JSON.stringify(check.errors)).toBe(true);
 expect(check({...record,extra:1})).toBe(false);expect(check({...record,cursor:"bad"})).toBe(false);expect(check({...record,state:"half"})).toBe(false);
 const {pointer:_pointer,relevance:_relevance,...light}=record;expect(check({...light,state:"light"}),JSON.stringify(check.errors)).toBe(true);
 const header=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Sidecar"});
 expect(header({version:1,pointer:record.pointer,rows:1,rechecked:0,directories:2,strict:1,stamped:1,linked:0,unsealable:null}),JSON.stringify(header.errors)).toBe(true);
 expect(header({version:1,pointer:record.pointer,rows:1,rechecked:0,directories:2,linked:0})).toBe(false);
});

test("the switch and the key: only what can change the slice is part of the identity",()=>{
 expect(graphMemoEnabledV1({})).toBe(true);expect(graphMemoEnabledV1({SEMIO_BUILD_GRAPH_MEMO:"OFF"})).toBe(false);expect(()=>graphMemoEnabledV1({SEMIO_BUILD_GRAPH_MEMO:"maybe"})).toThrow(/Erwartet/u);
 const base={root:"/w",args:["run","p:build"],tool:"bazel",mode:"prototype",implementation:"i".repeat(64),environment:{SEMIO_BUILD_MODE:"prototype",HOME:"/h"}};
 const key=graphMemoKeyV1(base);
 expect(graphMemoKeyV1({...base,environment:{...base.environment,HOME:"/other",SEMIO_BUILD_GENERATION_PIN:"x",SEMIO_BUILD_INVOCATION_PID:"7",SEMIO_REPO_PROFILE:"1",SEMIO_BUILD_GRAPH_MEMO_OUT:"y",SEMIO_BUILD_PARALLEL:"3",SEMIO_BUILD_GRAPH_YIELD_TICKET:"/tmp/semio-graph-yield-16498.json",SEMIO_BUILD_GRAPH_REQUESTED_AT:"1793081919763",SEMIO_BUILD_GRAPH_YIELD_ROUND:"2",SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust"}})).toBe(key);
 expect(graphMemoKeyV1({...base,args:[...base.args,"--parallel=6"]})).toBe(key);
 expect(graphMemoKeyV1({...base,args:[...base.args,"--parallel","3"]})).toBe(key);
 for(const change of [{args:["run","q:build"]},{tool:"nx"},{mode:"production"},{implementation:"j".repeat(64)},{root:"/v"},{environment:{...base.environment,SEMIO_GRAPH_PHASES:"1"}},{environment:{...base.environment,SEMIO_BUILD_MODE:"production"}}])expect(graphMemoKeyV1({...base,...change}),JSON.stringify(change)).not.toBe(key);
});

test("what a fact reads: the model written in another language answers the same for every kind, root, exclusion and change",async()=>{
 const {root}=fixture(),rows=await harvest(root);
 const synthetic:MemoRowV1[]=[
  {kind:"resolve",path:JSON.stringify([root+"/src/a.ts","./b.ts"]),fact:root+"/src/b.ts"},{kind:"resolve",path:JSON.stringify([root+"/src/a.ts","pkg"]),fact:root+"/node_modules/pkg/index.js"},{kind:"resolve",path:JSON.stringify([root+"/src/a.ts",root+"/lib/e.ts"]),fact:"!MODULE_NOT_FOUND"},{kind:"resolve",path:"not json",fact:""},
  {kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:[root+"/lib/e.ts",root+"/dist/x.ts","/etc/hosts"]}),fact:"h"},{kind:"closure",path:JSON.stringify([root,"d",["generated"]]),fact:"true"},{kind:"stamp",path:"inferrer:x",fact:"s"},{kind:"registry",path:JSON.stringify([root,"@a/b"]),fact:{}},
  {kind:"field",path:"lib/📋️project.json",fact:{}},{kind:"digest",path:"Cargo.toml",fact:{}},{kind:"typescriptRealpath",path:root+"/lib/e.ts",fact:root+"/lib/e.ts"},{kind:"directoryExists",path:"/outside/dir",fact:true},{kind:"fileExists",path:root+"/a/node_modules/x",fact:false},{kind:"readdir",path:root,fact:"h"},{kind:"readdirNames",path:root+"/lib",fact:"h"},{kind:"directories",path:root+"/docs",fact:"h"},{kind:"readdir",path:root+"/other",fact:"h"},
  {kind:"analysis",path:root+"/lib/e.ts",fact:{source:{size:1,mtimeMs:1,ctimeMs:1,ino:1,sha256:"s"},nestedObservations:[{kind:"file",path:root+"/lib/g.ts",fact:{}},{kind:"exists",path:root+"/dist/h.ts",fact:true}]}},
 ];
 const paths=["src/a.ts","src","","src/sub/deep/x.ts","docs/new.md","docs","lib","lib/e.ts","lib/other.ts","other/f.ts","other/x.ts","src/sub/extra.ts","src/extra.ts","Cargo.toml","node_modules/x","src/SUB","LIB/e.ts"],stampable=["","lib","docs","other","src/sub","Cargo"],changes:ChangeRowV1[]=paths.flatMap(path=>[["m",0,path],["m",1,path],["a",1,path],["r",0,path]] as ChangeRowV1[]);
 const cases=[false,true].map(insensitive=>({root,excludedNames:[...NAMES],rows:[...rows,...synthetic],changes,insensitive,stampable}));
 const file=join(sandbox,"cases.json");writeFileSync(file,JSON.stringify(cases));
 const run=spawnSync(python,["-I",join(here,"🐍️.py"),file],{encoding:"utf8",maxBuffer:1<<28});expect(run.status,run.stderr).toBe(0);
 const model=JSON.parse(run.stdout) as {rows:({kind:string;mentions:string[];recheck:boolean}|null)[];directories:string[];strict:string[];stamped:string[];relevant:[boolean,string|null][]}[];
 for(const [index,{insensitive}] of cases.entries()){
  const vouched:string[]=[],hard=new Set<string>(),listing=new Set<string>(),mine:unknown[]=[];
  for(const row of cases[index]!.rows)for(const leaf of leavesOf(row)){const reach=memoReachV1(leaf,root,excluded);mine.push(reach===undefined?null:{kind:leaf.kind,mentions:[...reach.mentions],recheck:reach.recheck});if(reach&&!reach.recheck){const found=memoClassesV1(leaf,reach);vouched.push(...found.names);for(const path of found.strict)hard.add(path);for(const path of found.listing)listing.add(path);}}
  expect(mine).toEqual(model[index]!.rows);
  const directories=memoDirectoriesV1(vouched);
  expect([...directories].sort()).toEqual(model[index]!.directories);
  const stampSet=new Set(stampable),final=new Set([...hard,...[...listing].filter(path=>!stampSet.has(path))]),stamped=new Set(stampable.filter(path=>!final.has(path)));
  expect([...final].sort()).toEqual(model[index]!.strict);expect([...stamped].sort()).toEqual(model[index]!.stamped);
  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(directories,insensitive,final,stamped))!;
  expect(changes.map(change=>{const found=classifyMemoChangesV1(relevance,[change]);return[found.changed!==undefined,found.pending.size?[...found.pending.keys()][0]!:null];})).toEqual(model[index]!.relevant);
  expect(changes.map(change=>firstRelevantChangeV1(relevance,[change])!==undefined)).toEqual(model[index]!.relevant.map(([now,pending])=>now||pending!==null));
 }
 expect(memoReachV1({kind:"unknown-kind",path:"x",fact:1},root,excluded)).toBeUndefined();
 const unknown=await buildGraphMemoRelevanceV1({root,rows:()=>[{kind:"unknown-kind",path:"x",fact:1}],frontier:{directories:[],members:[]},excluded,isLink:async()=>false});
 expect(unknown.unsealable).toBe("kind:unknown-kind");
});
const leavesOf=(row:MemoRowV1):MemoRowV1[]=>[row,...(row.kind==="analysis"?((row.fact as {nestedObservations?:MemoRowV1[]}).nestedObservations??[]).flatMap(leavesOf):[])];

test("soundness against the engine: whatever the memo vouches for, the engine's own verification of the same facts agrees (22 adversarial mutations, a model journal and the real observation checks); a lying memo and a blind journal are caught",async()=>{
 let both=0,caught=0,lyingWrong=0,blindWrong=0,vouched=0,settled=0,skipWrong=0,outputServed=0;
 for(const mutation of mutations){
  const {root,outside}=fixture(),rows=await harvest(root),built=await build(root,rows);
  expect(built.unsealable,mutation.name).toBeNull();expect(built.linked,mutation.name).toBe(2);
  expect(await hold(rows),mutation.name+" (fresh)").toBe(true);
  const before=snapshot(root);mutation.apply(root,outside);const after=snapshot(root),changes=journalRows(before,after,mutation.advisory);
  const engine=await hold(rows),insensitive=process.platform==="darwin"||process.platform==="win32",relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped))!,stamps=packMemoStampsV1(built.stamps,insensitive);
  expect(built.stamped.size,mutation.name).toBeGreaterThanOrEqual(3);
  expect(built.recheck.rows.some(row=>row.kind==="readdirNames"&&row.path.endsWith("lib/.🧾️store")),mutation.name+": the listing of an output directory is re-checked by the engine").toBe(true);
  expect(built.stamped.has("lib/.🧾️store")||built.strict.has("lib/.🧾️store"),mutation.name+": an output directory is neither stamped nor strict").toBe(false);
  expect(built.edits.inside.paths.includes("lib/.🧾️store")||built.edits.named.paths.includes("lib/.🧾️store"),mutation.name+": the edit index holds nothing for an output directory (its listing is the engine's, every lookup)").toBe(false);
  const decide=async(rowsSeen:ChangeRowV1[],recheck:MemoRecheckV1,confirm=true):Promise<boolean>=>{
   const found=classifyMemoChangesV1(relevance,rowsSeen);if(found.changed)return false;
   if(found.pending.size&&confirm&&!memoStampsHoldV1(root,stamps,found.pending.keys(),insensitive))return false;
   return recheck.identities.every(identity=>memoIdentityHoldsV1(root,identity))&&await hold(recheck.rows);
  };
  const memo=await decide(changes,built.recheck);
  if(classifyMemoChangesV1(relevance,changes).pending.size&&memo)settled++;
  if(await decide(changes,built.recheck,false)&&!engine)skipWrong++;
  if(memo)expect(engine,"the memo vouched for a change the engine sees: "+mutation.name).toBe(true);
  if(memo&&engine)both++;
  if(!memo&&!engine)caught++;
  if(memo)vouched++;
  if(memo&&mutation.name.startsWith("a build rewrites its output"))outputServed++;
  if(process.env.MEMO_LAW_TABLE)console.log("law",mutation.name.padEnd(54),"journal rows",String(changes.length).padStart(2),"relevant",String(firstRelevantChangeV1(relevance,changes)?.[2]??"-").padEnd(16),"memo",memo?"serves":"refuses","engine",engine?"holds":"stale");
  if(await decide(changes,{rows:[],identities:[]})&&!engine)lyingWrong++;
  if(await decide([],built.recheck)&&!engine)blindWrong++;
 }
 expect(vouched).toBeGreaterThanOrEqual(9);expect(both).toBeGreaterThanOrEqual(9);expect(caught).toBeGreaterThanOrEqual(14);
 expect(outputServed,"a build rewriting its own output directory is served: the engine confirms the listing").toBe(1);
 expect(settled,"content edits of unread siblings in stamped directories are served after the stamp confirmed them").toBeGreaterThanOrEqual(3);
 expect(skipWrong,"a memo that trusts the advisory kind without the stamp must be wrong somewhere").toBeGreaterThan(0);
 expect(lyingWrong,"a memo without its recheck set must be wrong somewhere").toBeGreaterThan(0);expect(blindWrong,"a journal that sees nothing must be wrong somewhere").toBeGreaterThan(0);
},180000);

class FakeJournal implements ChangeJournalClientV1{
 epoch=EPOCH;seq=0;log:{seq:number;rows:ChangeRowV1[]}[]=[];refuse:undefined|"epoch"|"gap"|"no-service";calls=0;
 excluder(){return excluded;}
 push(...rows:ChangeRowV1[]):void{this.seq++;this.log.push({seq:this.seq,rows});}
 async cursor(){this.calls++;return this.refuse==="no-service"?{exact:false as const,reason:"no-service" as const}:{epoch:this.epoch,seq:this.seq};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{
  this.calls++;
  if(this.refuse==="no-service")return{exact:false,reason:"no-service"};
  if(this.refuse==="gap")return{exact:false,reason:"gap"};
  if(from.epoch!==this.epoch||this.refuse==="epoch")return{exact:false,reason:"epoch"};
  return{exact:true,cursor:{epoch:this.epoch,seq:this.seq},rows:this.log.filter(entry=>entry.seq>from.seq).flatMap(entry=>entry.rows)};
 }
}

test("lifecycle: seal, serve, narrow by the journal, refuse on every kind of drift, advance the cursor, keep a bounded directory",async()=>{
 const {root}=fixture(),rows=await harvest(root),built=await build(root,rows),directory=join(sandbox,"memo"+counter++),pointerFile=join(sandbox,"current"+counter+++".json"),stem=join(sandbox,"sidecar"+counter++),manifest="e".repeat(64);
 writeFileSync(pointerFile,JSON.stringify({generation:7,manifest,verified:{atMs:1}}));
 const header={file:pointerFile,generation:7,manifest};
 writeGraphMemoSidecarFilesV1(stem,header,built,true);
 const journal=new FakeJournal(),environment={SEMIO_BUILD_INVOCATION_PID:"4242"},key="k".repeat(64),plan=JSON.stringify({projects:{a:1,b:2},edges:[]}),pin=JSON.stringify({version:1,owner:1,generation:7,manifest,covered:{atMs:5,count:8,owners:"/w=="}});
 let now=1_000_000;
 const open=(extra:Record<string,unknown>={})=>openGraphMemoV1({root,directory,client:journal,key,insensitive:true,now:()=>now,environment,holds:rowsToCheck=>hold(rowsToCheck),...extra});
 const first=await open();expect(first.cursor).toBe(formatChangeCursorV1({epoch:EPOCH,seq:0}));
 expect(await first.lookup()).toEqual({hit:false,reason:"none"});expect(first.expected()).toBeUndefined();
 expect(await first.seal({plan,pin,projects:2,sidecar:stem+".nothing.json"})).toBe("light");
 const second=await open();expect(second.expected()).toBe(manifest);expect(await second.lookup()).toEqual({hit:false,reason:"light"});
 expect(await second.seal({plan,pin,projects:2,sidecar:stem+".json"})).toBe("sealed");
 journal.push(["m",0,"docs/elsewhere.md"]);
 now+=5000;
 const served=await (await open()).lookup();
 expect(served.hit).toBe(true);
 if(served.hit){
  expect(served.plan).toBe(plan);expect(served.projects).toBe(2);expect(served.ageMilliseconds).toBe(5000);expect(served.rechecked).toBeGreaterThan(0);
  const owned=JSON.parse(served.pin!);expect(owned).toEqual({version:1,owner:4242,generation:7,manifest,covered:{atMs:now,count:8,owners:"/w=="}});
 }
 const baseline=served.hit?served.rechecked:0;
 journal.push(["m",0,"src/sub/extra.ts"]);
 const settledHit=await (await open()).lookup();
 expect(settledHit.hit,"a content edit of an unread sibling in a stamped directory is settled by the stamp").toBe(true);
 if(settledHit.hit)expect(settledHit.rechecked).toBeGreaterThan(baseline);
 journal.push(["a",0,"src/sub/zzz.ts"]);
 expect(await (await open()).lookup(),"a new name in a stamped directory").toEqual({hit:false,reason:"changed",detail:"src/sub/zzz.ts"});
 journal.log.pop();journal.seq--;
 writeFileSync(join(root,"lib/zzz2.ts"),"z\n");journal.push(["m",0,"lib/zzz2.ts"]);
 expect(await (await open()).lookup(),"an advisory 'modified' that hides a new name moved the stamp").toEqual({hit:false,reason:"changed",detail:"lib/zzz2.ts"});
 rmSync(join(root,"lib/zzz2.ts"));journal.log.pop();journal.seq--;
 journal.push(["m",0,"src/a.ts"]);
 const changed=await (await open()).lookup();
 expect(changed).toEqual({hit:false,reason:"changed",detail:"src/a.ts"});
 journal.log.pop();journal.seq--;
 for(const refuse of ["epoch","gap","no-service"] as const){journal.refuse=refuse;const answer=await (await open()).lookup();expect(answer.hit,refuse).toBe(false);if(!answer.hit)expect(answer.reason,refuse).toBe("journal");journal.refuse=undefined;}
 expect((await (await open()).lookup()).hit).toBe(true);
 expect(await (await open({key:"z".repeat(64)})).lookup()).toEqual({hit:false,reason:"none"});
 const otherPointer=JSON.stringify({generation:8,manifest});writeFileSync(pointerFile,otherPointer);
 expect(await (await open()).lookup()).toEqual({hit:false,reason:"pointer"});
 writeFileSync(pointerFile,JSON.stringify({generation:7,manifest,verified:{atMs:99999}}));
 expect((await (await open()).lookup()).hit,"a republished verified stamp does not move the generation").toBe(true);
 const record=JSON.parse(readFileSync(join(directory,key.slice(0,40)+".json"),"utf8")) as {cursor:string;plan:{sha256:string};relevance:{id:string;stamps:string}};
 expect(record.cursor).toBe(formatChangeCursorV1({epoch:EPOCH,seq:journal.seq}));
 const planFile=join(directory,record.plan.sha256+".plan"),relFile=join(directory,record.relevance.id+".rel"),checkFile=join(directory,record.relevance.id+".chk"),stampFile=join(directory,record.relevance.stamps+".lst");
 const saved=[planFile,relFile,checkFile,stampFile].map(file=>readFileSync(file));
 writeFileSync(planFile,plan.replace("a","c"));expect(await (await open()).lookup()).toEqual({hit:false,reason:"plan"});writeFileSync(planFile,saved[0]!);
 writeFileSync(relFile,Buffer.concat([saved[1]!,Buffer.from([0])]));expect(await (await open()).lookup()).toEqual({hit:false,reason:"generation"});writeFileSync(relFile,saved[1]!);
 writeFileSync(checkFile,saved[2]!.toString("utf8").replace("dist/out.js","dist/oops.js"));expect(await (await open()).lookup()).toEqual({hit:false,reason:"generation"});writeFileSync(checkFile,saved[2]!);
 expect((await (await open()).lookup()).hit).toBe(true);
 journal.push(["m",0,"src/sub/extra.ts"]);
 writeFileSync(stampFile,Buffer.concat([saved[3]!,Buffer.from([0])]));expect(await (await open()).lookup(),"a tampered stamp table").toEqual({hit:false,reason:"generation"});writeFileSync(stampFile,saved[3]!);
 rmSync(stampFile);expect(await (await open()).lookup(),"a missing stamp table").toEqual({hit:false,reason:"generation"});writeFileSync(stampFile,saved[3]!);
 expect((await (await open()).lookup()).hit,"the stamp confirms again").toBe(true);
 rmSync(join(root,"dist/out.js"));expect(await (await open()).lookup()).toEqual({hit:false,reason:"stale"});writeFileSync(join(root,"dist/out.js"),"out\n");
 appendFileSync(join(root,"dist/gen.ts"),"more\n");expect(await (await open()).lookup()).toEqual({hit:false,reason:"stale"});
 const lonely=await openGraphMemoV1({root,directory,client:Object.assign(new FakeJournal(),{refuse:"no-service" as const}),key:"y".repeat(64)});
 expect(lonely.cursor).toBeUndefined();expect(await lonely.seal({plan,pin,projects:2,sidecar:stem+".json"})).toBe("journal");
 const moved=await open({key:"m".repeat(64)});writeFileSync(pointerFile,otherPointer);
 expect(await moved.seal({plan,pin,projects:2,sidecar:stem+".json"}),"a pointer that moved while the child ran keeps only the light record").toBe("light");
 writeFileSync(pointerFile,JSON.stringify({generation:7,manifest}));
 expect(await moved.seal({plan,pin:pin.replace(manifest,"f".repeat(64)),projects:2,sidecar:stem+".json"}),"a sidecar of another generation than the pin").toBe("light");
 expect(await moved.seal({plan,pin:undefined,projects:2,sidecar:stem+".json"})).toBe("pin");
 writeGraphMemoSidecarFilesV1(stem,header,{...built,unsealable:"kind:x"},true);
 expect(await moved.seal({plan,pin,projects:2,sidecar:stem+".json"})).toBe("light");
 for(let index=0;index<12;index++){
  writeGraphMemoSidecarFilesV1(stem,header,built,true);
  const entry=await open({key:String(index).padStart(64,"0")});await entry.seal({plan:plan+index,pin,projects:2,sidecar:stem+".json"});
  const time=new Date(Date.now()-(20-index)*1000);for(const name of readdirSync(directory))if(name.endsWith(".json"))utimesSync(join(directory,name),time,time);
 }
 collectGraphMemoV1(directory,8);
 expect(readdirSync(directory).filter(name=>name.endsWith(".json")).length).toBeLessThanOrEqual(8);
},120000);

test("the command opens a memo only inside a captured implementation",async()=>{
 const {root}=fixture();
 expect(await openGraphMemoForCommandV1(root,["run","p:build"],{tool:"bazel"},undefined,{SEMIO_BUILD_GRAPH_MEMO:"off"})).toBeUndefined();
 expect(await openGraphMemoForCommandV1(root,["run","p:build"],{tool:"bazel"},undefined,{SEMIO_REPO_CHANGES:"off"})).toBeUndefined();
});

const watcher=process.env.SEMIO_REPO_WATCH_BINARY??"";
test.skipIf(!watcher)("with a REAL file watcher: for each mutation the memo sealed before it answers a miss whenever the engine sees the change, and serves the unrelated ones",async()=>{
 let served=0,missed=0;
 for(const mutation of mutations.filter((_,index)=>[0,1,2,3,4,5,6,7,8,9,10,12,14,15,17,18,20,21,22,23,24,25,26,27,30,31,32].includes(index))){
  const {root,outside}=fixture(),journalDirectory=root+".journal",environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
  try{
   const rows=await harvest(root),built=await build(root,rows),stem=join(sandbox,"real"+counter++),pointerFile=join(sandbox,"cur"+counter++),manifest="a".repeat(64);
   writeFileSync(pointerFile,JSON.stringify({generation:1,manifest}));writeGraphMemoSidecarFilesV1(stem,{file:pointerFile,generation:1,manifest},built,true);
   const client=createChangeJournalClientV1({directory:journalDirectory,root,environment,start:()=>startChangeJournalServiceV1({directory:journalDirectory,root,environment})}),open=()=>openGraphMemoV1({root,directory:join(journalDirectory,"memo"),client,key:"r".repeat(64),holds:rowsToCheck=>hold(rowsToCheck)});
   const pin=JSON.stringify({version:1,owner:1,generation:1,manifest,covered:{atMs:1,count:0,owners:""}}),first=await open();expect(first.cursor,mutation.name).toBeDefined();
   expect(await first.seal({plan:"{\"projects\":{}}",pin,projects:0}),mutation.name).toBe("light");
   const sealing=await open();expect(await sealing.seal({plan:"{\"projects\":{}}",pin,projects:0,sidecar:stem+".json"}),mutation.name).toBe("sealed");
   mutation.apply(root,outside);
   const engine=await hold(rows),answer=await (await open()).lookup();
   if(answer.hit){expect(engine,"the memo served a graph the engine would refuse: "+mutation.name).toBe(true);served++;}else missed++;
   if(!engine)expect(answer.hit,mutation.name).toBe(false);
  }finally{const pid=readChangeStateV1(journalDirectory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}rmSync(journalDirectory,{recursive:true,force:true});}
 }
 expect(served).toBeGreaterThanOrEqual(1);expect(missed).toBeGreaterThanOrEqual(8);
},300000);

test("the cached exclusion rule is the journal's own: for rooted exclusions and excluded names, with and without folding, it answers as the client's predicate for every path of a random walk",()=>{
 const directory=join(sandbox,"state"+counter++);mkdirSync(directory,{recursive:true});
 const exclude={rooted:[".cache","a/b","Caf\u00e9/x"],names:["node_modules","dist",".git"]};
 writeChangeStateV1(directory,{version:1,epoch:EPOCH,instance:"i",pid:1,startedAt:1,root:"/w",fence:".fence",exclude,backend:"x",trust:"exact",ready:true} as never);
 let state=7;const next=(limit:number)=>{state=(state*1103515245+12345)&0x7fffffff;return state%limit;};
 const names=["a","b","c","node_modules","Dist","dist",".cache",".git","Caf\u00e9","cafe\u0301","x","\u{1F9EC}\uFE0Fy"];
 for(const insensitive of [false,true]){
  const client=createChangeJournalClientV1({directory,root:"/w",insensitive}),mine=memoExcluderV1(exclude,insensitive);
  for(let round=0;round<3000;round++){
   const path=Array.from({length:1+next(5)},()=>names[next(names.length)]).join("/");
   expect(mine(path),insensitive+" "+path).toBe(client.excluder()(path));
  }
 }
});

test("a memo that cannot serve says why: the reason and the first relevant change, in the command's language; a request without a record stays quiet",async()=>{
 const {planGraphWithMemoV1}=await import("../🚪️command/🟦️.ts");
 const run=async(answer:unknown,language:string)=>{const notices:string[]=[];const memo={lookup:async()=>answer,cursor:undefined,expected:()=>undefined,seal:async()=>"light"} as unknown as Parameters<typeof planGraphWithMemoV1>[0]["memo"];
  await planGraphWithMemoV1({memo,stem:"/x/memo",parse:text=>JSON.parse(text) as {projects:Record<string,unknown>},record:()=>{},notice:text=>{notices.push(text);},mark:()=>{},environment:{SEMIO_LANGUAGE:language},plan:async()=>({plan:JSON.stringify({projects:{a:{}}}),pin:undefined})});return notices;};
 const changed=await run({hit:false,reason:"changed",detail:"src/a.ts"},"en");
 expect(changed).toHaveLength(1);expect(changed[0]).toContain("changed: src/a.ts");expect(changed[0]).toContain("graph is planned");
 expect((await run({hit:false,reason:"journal"},"de"))[0]).toContain("(journal)");
 expect((await run({hit:false,reason:"journal"},"de"))[0]).toContain("Graph wird geplant");
 expect(await run({hit:false,reason:"none"},"en")).toEqual([]);
});

test("an output directory stays a watched name while a vouched fact reads an entry below it: replacing the whole directory by a rename (the journal names only the directory) is relevant, and the engine confirms the read entry changed while the listing holds",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"memo-out-"))),insensitive=process.platform==="darwin"||process.platform==="win32";
 try{
  const store=join(root,"out/.🧾️store");mkdirSync(store,{recursive:true});writeFileSync(join(store,"build.json"),"{\"v\":1}\n");
  const captured=await observation.captureObservedSourceInputsV1(()=>{observation.observedFs.readdirSync(store);observation.observedFs.readFileSync(join(store,"build.json"),"utf8");},undefined,false),rows=captured.observations as unknown as MemoRowV1[];
  const built=await buildGraphMemoRelevanceV1({root,rows:()=>rows,frontier:{directories:[],members:[]},excluded:()=>false,isLink:async()=>false});
  expect(built.unsealable).toBeNull();
  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped))!;
  mkdirSync(join(root,"out/next"));writeFileSync(join(root,"out/next/build.json"),"{\"v\":2}\n");rmSync(store,{recursive:true});renameSync(join(root,"out/next"),store);
  const journal:ChangeRowV1[]=[["a",1,"out/next"],["a",0,"out/next/build.json"],["r",1,"out/.🧾️store"],["n",1,"out/next"],["n",1,"out/.🧾️store"]];
  expect(await observation.observationsHold(rows.filter(row=>row.kind!=="file") as never,64,new Set(),true),"the listing alone still holds").toBe(true);
  expect(await observation.observationsHold(rows as never,64,new Set(),true),"the entry the fact read changed").toBe(false);
  expect(relevance.holds("out/.🧾️store"),"the output directory is a name a change must not touch while an entry below it is read").toBe(true);
  expect(classifyMemoChangesV1(relevance,journal).changed?.[2],"the rename over the output directory is relevant").toBe("out/.🧾️store");
 }finally{rmSync(root,{recursive:true,force:true});}
});
