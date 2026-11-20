/** 🤖️ Contract: what the filesystem says about a path below a machine-local directory (`🤖️generated`: git ignores it, a generator fills it on the machines that ran it) is no input of a project, for every kind of fact that observes the filesystem and not for directory listings alone: a read of such a path records no row, and a recorded row of any of these kinds holds on a machine whose directory is absent, empty or filled otherwise. One predicate serves recording, judging and the listings. A path of the same name outside the directory, and a file named like it, stay inputs.
 * @see ../../🟨️.mjs `isMachineLocalPathV1` `machineLocalRowV1` */
import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import schema from"../../🧬️schema/🔣️.json";

const names=await import("../../🟨️.mjs") as any,observation=await import("../../../🟨️.mjs") as any;
const kinds=schema.$defs.rules.const.machineLocalPathKinds as string[],hex=(text:string)=>createHash("sha256").update(text).digest("hex");
const provider=observation.registerSourceAnalysisV1({id:"build:machine-local-paths:v1:"+hex("law"),revision:hex("machine-local-paths"),analyze:(_path:string,bytes:Buffer)=>({length:bytes.length})});
const root=mkdtempSync(join(realpathSync(tmpdir()),"machine-local-paths-"));
afterAll(()=>{rmSync(root,{recursive:true,force:true});});
const put=(path:string,text="{\"a\":1}\n")=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);return join(root,path);};
const facts:Record<string,unknown>={
 file:{size:1,mtimeMs:1,ctimeMs:1,ino:1,sha256:hex("x")},field:{format:"json",pointer:"/a",paths:[],digest:hex("y")},analysis:{provider:provider.id,revision:provider.revision,source:{size:1,mtimeMs:1,ctimeMs:1,ino:1,sha256:hex("z")},resultSha256:hex("r"),nestedObservations:[]},
 exists:true,fileExists:true,directoryExists:true,stat:"file",lstat:"file",realpath:"/nowhere/real",realpathNative:"/nowhere/real",typescriptText:hex("t"),typescriptRealpath:"/nowhere/real",readdir:hex("d"),readdirNames:hex("n"),directories:hex("s"),
};
const STORE=schema.$defs.rules.const.machineLocalDirectoryNames[1] as string;
const holds=(row:object)=>{try{return observation.observationRowFreshV1(row)===true;}catch{return false;}};

test("the store directory of a package is machine-local too: a listing of a directory holding it is the same with and without it, and git ignores it",()=>{
 expect(schema.$defs.rules.const.machineLocalDirectoryNames).toContain(STORE);
 expect(names.isMachineLocalPathV1(join(root,"a",STORE,"b.json"))).toBe(true);
 const withStore=mkdtempSync(join(root,"list-a-")),without=mkdtempSync(join(root,"list-b-"));
 for(const base of[withStore,without])writeFileSync(join(base,"Cargo.toml"),"x\n");
 mkdirSync(join(withStore,STORE));writeFileSync(join(withStore,STORE,"state.json"),"{}\n");
 for(const kind of["readdir","readdirNames"] as const)expect(observation.directoryFactOfV1(kind,withStore),kind).toBe(observation.directoryFactOfV1(kind,without));
});

test("the predicate names a path below the directory, in either separator, and nothing else",()=>{
 for(const path of[join(root,"🤖️generated","b.ts"),join(root,"a","🤖️generated","b","c.ts"),"C:\\semio\\a\\🤖️generated\\b.ts"])expect(names.isMachineLocalPathV1(path),path).toBe(true);
 for(const path of[join(root,"🤖️generated"),join(root,"a","🤖️generated"),join(root,"generated","b.ts"),join(root,"🤖️generated-not","b.ts"),join(root,"a","b.ts"),""])expect(names.isMachineLocalPathV1(path),path).toBe(false);
 expect(names.isMachineLocalPathV1(join(root,"🤖️generated"),true)).toBe(true);
 expect(names.machineLocalRowV1("readdirNames",join(root,"🤖️generated"))).toBe(true);
 expect(names.machineLocalRowV1("exists",join(root,"🤖️generated"))).toBe(false);
 expect(names.machineLocalRowV1("closure",join(root,"🤖️generated","x"))).toBe(false);
 expect(names.machineLocalRowV1("resolve",join(root,"🤖️generated","x"))).toBe(false);
});

test("every kind the schema declares is a fact that observes the filesystem, and every such kind of the verifier is declared",async()=>{
 const text=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text(),block=text.slice(text.indexOf("const facts = {"),text.indexOf("\n};",text.indexOf("const facts = {"))),found=[...block.matchAll(/^\s{2}(\w+):/gmu)].map(match=>match[1]!);
 const other=new Set(["stamp","closure","registry","resolve"]);
 expect([...new Set([...found.filter(kind=>!other.has(kind)),"file"])].sort()).toEqual([...kinds].sort());
});

for(const kind of kinds)test("a recorded "+kind+" row over a machine-local path holds where the directory is absent; over any other path it does not ("+kind+")",async()=>{
 const local=join(root,"gone","🤖️generated","x.json"),plain=join(root,"gone","plain","x.json"),row=(path:string)=>({kind,path,fact:facts[kind]});
 expect(holds(row(local)),"machine-local").toBe(true);
 expect(await observation.firstFailingRowV1([row(local)],undefined),"judged by the verifier of a stored record").toBeUndefined();
 expect(await observation.firstFailingRowV1([row(join(root,"gone",STORE,"x.json"))],undefined),"below the store directory of a package").toBeUndefined();
 expect(holds(row(plain)),"control: the same fact over an ordinary path is refused").toBe(false);
});

test("a read of a machine-local path records no row of any kind, a read beside it records its rows, and the record holds when the directory is gone",async()=>{
 const generated=put("p/🤖️generated/in.json"),plain=put("p/in.json"),cachePath=join(root,"np","record.json");
 await new Promise(done=>setTimeout(done,2200));
 await observation.observeInference({cachePath,key:["record"]},async()=>{
  const fs=observation.observedFs;
  for(const path of[generated,plain]){fs.existsSync(path);fs.statSync(path);fs.lstatSync(path);fs.readFileSync(path,"utf8");observation.observedFieldV1(path,"json","/a");observation.observedSourceAnalysisV1(provider,path);}
  fs.readdirSync(dirname(generated));fs.readdirSync(dirname(plain));
  return{ok:true};
 });
 const rows=observation.projectInferenceObservationsV1(cachePath) as{kind:string;path:string}[];
 expect(rows.filter(row=>row.path.includes("🤖️generated")).map(row=>row.kind+" "+row.path)).toEqual([]);
 expect(new Set(rows.filter(row=>row.path===plain).map(row=>row.kind))).toEqual(new Set(["exists","stat","lstat","file","field","analysis"]));
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["record"]})).verdict).toBe("holds");
 rmSync(dirname(generated),{recursive:true});
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["record"]})).verdict).toBe("holds");
 writeFileSync(plain,"{\"a\":2}\n");
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["record"]})).verdict).toBe("stale");
});

test("a file named like the directory is source: its rows are recorded",async()=>{
 const filed=put("q/🤖️generated","not a directory\n"),cachePath=join(root,"np","filed.json");
 await new Promise(done=>setTimeout(done,2200));
 await observation.observeInference({cachePath,key:["filed"]},async()=>({text:observation.observedFs.readFileSync(filed,"utf8")}));
 const rows=observation.projectInferenceObservationsV1(cachePath) as{kind:string;path:string}[];
 expect(rows.length).toBeGreaterThan(0);
});
