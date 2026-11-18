import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import vectors from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {INTERFACE_POLICY_V1,currentHashV1,dependenciesFileV1,identityFileV1,interfaceDigestV1,interfaceEligibilityV1,interfaceIdentityV1,lexRustV1,maskRustSourceV1,parseListRootV1,rebaseMetadataV1,rebaseRestoredUnitV1,svhBytesV1,writeDependenciesV1,writeIdentitiesV1} from "../🟦️.ts";
import {parseRustcUnitV1} from "../../🔏️key/🟦️.ts";

/** 🧣️ Interface laws: a body-only edit leaves the interface digest of the source alone exactly where no dependant can observe the body, the Python model and the compiler itself agree, and a dependant compiled against the old library, rebased onto the new one, behaves like a dependant compiled against the new one. */
const here=fileURLToPath(new URL(".",import.meta.url)),policy=INTERFACE_POLICY_V1,sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"r39-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});

function findRustc():string[]|undefined{
 if(process.env.SEMIO_TEST_RUSTC)return[process.env.SEMIO_TEST_RUSTC];
 const roots=[process.env.SEMIO_TEST_CHECKOUT,"/Users/ueli/Documents/semio",resolve(here,"../../../../../../../../../..")].filter((root):root is string=>root!==undefined);
 for(const root of roots){
  try{
   const channel=/^channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(root,"rust-toolchain.toml"),"utf8"))?.[1];if(channel===undefined)continue;
   const directoryOf=join(process.env.RUSTUP_HOME??join(homedir(),".rustup"),"toolchains");
   const name=readdirSync(directoryOf).find(entry=>entry.startsWith(channel+"-"));
   if(name!==undefined&&existsSync(join(directoryOf,name,"bin","rustc")))return[join(directoryOf,name,"bin","rustc")];
  }catch{}
 }
 return undefined;
}
const rustc=findRustc(),real=rustc===undefined?test.skip:test;
const environment={...process.env,RUSTC_BOOTSTRAP:"1"} as Record<string,string|undefined>;

const analysis=(text:string)=>maskRustSourceV1(text);
const digestOf=(text:string)=>interfaceDigestV1([{name:"lib.rs",bytes:Buffer.from(text)}]).digest;

test("the policy is a closed constant of the schema",()=>{
 expect(schema.definitions.Policy.const).toEqual(policy);
 expect(Object.keys(policy).sort()).toEqual(["namespace","rebase","sidecars","source","unit","version"]);
});

test("every vector gives equal interface digests exactly where it says (TypeScript model)",()=>{
 for(const row of vectors.cases)expect(digestOf(row.old)===digestOf(row.new),row.label).toBe(row.equal);
});

test("an independent Python model gives the same equality, the same number of masked bodies and the same masked text",()=>{
 const run=spawnSync("python3",["-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"../🧬️schema/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const rows=run.stdout.trim().split("\n").map(line=>JSON.parse(line) as {label:string;equal:boolean;oldBodies:number;newBodies:number;oldMasked:string});
 expect(rows.length).toBe(vectors.cases.length);
 rows.forEach((row,index)=>{
  const vector=vectors.cases[index]!,old=analysis(vector.old),next=analysis(vector.new);
  expect(row.equal,vector.label).toBe(vector.equal);
  expect(row.oldBodies,vector.label).toBe(old.bodies);expect(row.newBodies,vector.label).toBe(next.bodies);
  expect(row.oldMasked,vector.label).toBe(old.masked);
 });
});

test("the lexer reads literals, lifetimes, nested comments and raw identifiers, and refuses unfinished ones",()=>{
 const kinds=(text:string)=>(lexRustV1(text)??[]).map(token=>token.kind+":"+token.text);
 expect(kinds("'a 'b' '\\n' r#\"x\"y\"# b'z' br\"q\" c\"s\" r#type /* a /* b */ c */ x // y")).toEqual(["lt:'a","chr:'b'","chr:'\\n'","str:r#\"x\"y\"#","chr:b'z'","str:br\"q\"","str:c\"s\"","id:r#type","id:x"]);
 expect(lexRustV1("fn f() { \"open")).toBeUndefined();
 expect(lexRustV1("/* open")).toBeUndefined();
 expect(kinds("1.5e3 1..2 0xFF_u8")).toEqual(["num:1.5e3","num:1","p:.","p:.","num:2","num:0xFF_u8"]);
});

test("a masked body is replaced by the placeholder and nothing else changes",()=>{
 const text="/// doc\npub fn f(x: u32) -> u32 { x + 1 }\npub const K: u32 = 2;\n",result=analysis(text);
 expect(result.masked).toBe("/// doc\npub fn f(x: u32) -> u32 "+policy.source.maskedBody+"\npub const K: u32 = 2;\n");
 expect(result.bodies).toBe(1);expect(result.void).toBe(false);
 expect(analysis("pub fn f() {").void,"unbalanced brackets cannot be read").toBe(true);
 expect(analysis("pub fn f() { x }\n").masked.length).toBeLessThan("pub fn f() { x }\n".length+policy.source.maskedBody.length);
});

test("a set of files: other files count by bytes, one unreadable file or one include! puts every Rust file back to bytes",()=>{
 const rust=(text:string)=>({name:"a.rs",bytes:Buffer.from(text)}),other=(text:string)=>({name:"Cargo.toml",bytes:Buffer.from(text)});
 const one="pub fn f() { 1 }\n",two="pub fn f() { 2 }\n";
 expect(interfaceDigestV1([rust(one),other("x")]).digest).toBe(interfaceDigestV1([rust(two),other("x")]).digest);
 expect(interfaceDigestV1([rust(one),other("x")]).digest).not.toBe(interfaceDigestV1([rust(one),other("y")]).digest);
 expect(interfaceDigestV1([rust(one),{name:"b.rs",bytes:Buffer.from("include!(\"c.rs\");")}]).digest).not.toBe(interfaceDigestV1([rust(two),{name:"b.rs",bytes:Buffer.from("include!(\"c.rs\");")}]).digest);
 expect(interfaceDigestV1([rust(one),{name:"b.rs",bytes:Buffer.from("fn g( {")}]).digest).not.toBe(interfaceDigestV1([rust(two),{name:"b.rs",bytes:Buffer.from("fn g( {")}]).digest);
 expect(interfaceDigestV1([{name:"a.rs",bytes:null}]).digest).not.toBe(interfaceDigestV1([{name:"a.rs",bytes:Buffer.from("")}]).digest);
 expect(interfaceDigestV1([rust(one),other("x")]).digest,"the order of the files does not matter").toBe(interfaceDigestV1([other("x"),rust(one)]).digest);
 expect(interfaceDigestV1([rust(one),rust(two)]).bodies).toBe(2);
});

test("a unit qualifies only as a library with a plain, unoptimised, metadata-only command line",()=>{
 const check=(extra:readonly string[],base:readonly string[]=["--crate-name","a","--edition=2021","src/lib.rs","--crate-type","lib","--emit=dep-info,metadata","-C","metadata=x","-Z","threads=0"])=>{const args=[...base,...extra];return interfaceEligibilityV1(parseRustcUnitV1(args),args);};
 expect(check([])).toBeUndefined();
 expect(check(["-C","opt-level=0"])).toBeUndefined();
 expect(check(["-C","opt-level=3"])).toBe("optimised");
 expect(check(["-Copt-level=s"])).toBe("optimised");
 expect(check(["-O"])).toBe("optimised");
 expect(check(["--test"])).toBe("test-harness");
 expect(check(["-Z","inline-mir"])).toBe("unstable:inline-mir");
 expect(check(["-Zcross-crate-inline-threshold=always"])).toBe("unstable:cross-crate-inline-threshold");
 expect(check([],["--crate-name","a","src/lib.rs","--crate-type","bin","--emit=dep-info,metadata"])).toBe("crate-type");
 expect(check([],["--crate-name","a","src/lib.rs","--crate-type","proc-macro","--emit=dep-info,metadata"])).toBe("crate-type");
 expect(check([],["--crate-name","a","src/lib.rs","--crate-type","lib","--emit=dep-info,link"])).toBe("no-metadata");
 expect(check([],["--crate-name","a","src/lib.rs","--crate-type","lib","--emit=dep-info,metadata,link"]),"a unit that emits code is linked to").toBe("emits-code");
});

test("metadata is rebased only where every old hash occurs exactly once, and exchanged hashes do not chain",()=>{
 const one="00112233445566778899aabbccddeeff",two="ffeeddccbbaa99887766554433221100",three="0f1e2d3c4b5a69788796a5b4c3d2e1f0";
 const bytes=Buffer.concat([Buffer.from("head"),svhBytesV1(one),Buffer.from("mid"),svhBytesV1(two),Buffer.from("tail")]);
 const swapped=rebaseMetadataV1(bytes,[[one,two],[two,one]])!;
 expect(swapped.equals(Buffer.concat([Buffer.from("head"),svhBytesV1(two),Buffer.from("mid"),svhBytesV1(one),Buffer.from("tail")]))).toBe(true);
 expect(rebaseMetadataV1(bytes,[[three,one]])).toBeUndefined();
 expect(rebaseMetadataV1(Buffer.concat([bytes,svhBytesV1(one)]),[[one,three]])).toBeUndefined();
 expect(rebaseMetadataV1(bytes,[])!.equals(bytes)).toBe(true);
 expect(bytes.indexOf(svhBytesV1(one))).toBe(4);
});

test("the listing of the compiler is read as the crate hash and the hashes of the crates it saw",()=>{
 const text="Crate info:\nname a-abc\nhash c70e4f6bfb6ab291ba5563e0628b7dc5 stable_crate_id StableCrateId(1)\n=External Dependencies=\n1 std-1f hash 96d575a8ae82913213e113d6a1496be4 host_hash None kind Unconditional public\n20 b-bbb hash 34126f20fcfb8b97fd59f2a944b38506 host_hash None kind Unconditional public\n";
 expect(parseListRootV1(text)).toEqual({svh:"c70e4f6bfb6ab291ba5563e0628b7dc5",dependencies:[["std-1f","96d575a8ae82913213e113d6a1496be4"],["b-bbb","34126f20fcfb8b97fd59f2a944b38506"]]});
 expect(parseListRootV1("nothing")).toBeUndefined();
});

type Library={rlib:string;rmeta:string};
type Pipeline={root:string;deps:string};
const run=(args:readonly string[],cwd:string)=>spawnSync(rustc![0]!,[...args],{cwd,encoding:"utf8",env:environment as NodeJS.ProcessEnv});
const depName=(name:string,kind:"rlib"|"rmeta")=>"lib"+name+"-"+name+"."+kind;
function library(pipeline:Pipeline,name:string,source:string,externs:readonly string[],mode:"metadata"|"code",extra:readonly string[]=[]):{status:number;stderr:string;library:Library}{
 const file=join(pipeline.root,name+"-"+sha(source).slice(0,8)+".rs");writeFileSync(file,source);
 const link=mode==="code",args=["--edition","2021","--crate-name",name,"--crate-type","lib","--emit",link?"dep-info,metadata,link":"dep-info,metadata","-Zembed-metadata=no","-C","metadata="+name,"-C","extra-filename=-"+name,"--out-dir",pipeline.deps,"-L","dependency="+pipeline.deps,...externs.flatMap(dependency=>link?["--extern",dependency+"="+join(pipeline.deps,depName(dependency,"rlib")),"--extern",dependency+"="+join(pipeline.deps,depName(dependency,"rmeta"))]:["--extern",dependency+"="+join(pipeline.deps,depName(dependency,"rmeta"))]),...extra,file];
 const result=run(args,pipeline.root);
 return{status:result.status??1,stderr:result.stderr.split(pipeline.root).join("<root>"),library:{rlib:join(pipeline.deps,depName(name,"rlib")),rmeta:join(pipeline.deps,depName(name,"rmeta"))}};
}
function binary(pipeline:Pipeline,externs:readonly string[],extra:readonly string[]=[]):{status:number;stderr:string;output:string}{
 const file=join(pipeline.root,"main.rs");writeFileSync(file,"fn main(){ println!(\"{}\", b::run()); }\n");
 const out=join(pipeline.root,"main-bin");
 const result=run(["--edition","2021","--crate-name","main","-Zembed-metadata=no","-L","dependency="+pipeline.deps,...externs.flatMap(dependency=>["--extern",dependency+"="+join(pipeline.deps,depName(dependency,"rlib")),"--extern",dependency+"="+join(pipeline.deps,depName(dependency,"rmeta"))]),...extra,"-o",out,file],pipeline.root);
 if(result.status!==0)return{status:result.status??1,stderr:result.stderr,output:""};
 const executed=spawnSync(out,[],{encoding:"utf8"});
 return{status:executed.status??1,stderr:executed.stderr,output:executed.stdout.trim()};
}
const pipeline=(label:string):Pipeline=>{const root=directory(label),deps=join(root,"deps");mkdirSync(deps,{recursive:true});return{root,deps};};
const fileDigest=(path:string)=>{try{return sha(readFileSync(path));}catch{return null;}};
const outcome=(result:{status:number;stderr:string})=>result.status===0?"ok":"error:"+(/error(\[E\d+\])?: [^\n]*/u.exec(result.stderr)?.[0]??result.stderr.slice(0,80));

for(const row of vectors.cases.filter(row=>row.rustc&&(row.equal||row.observable==="check"))){
 real("compiler law (check chain): "+row.label+(row.equal?": the dependant's check result against the old library is the one against the new, and it rebases onto the new":": the result changes, so a cutoff here would hide a diagnostic"),()=>{
  const oldWorld=pipeline("check-old"),newWorld=pipeline("check-new");
  expect(library(oldWorld,"a",row.old,[],"metadata").status).toBe(0);
  expect(library(newWorld,"a",row.new,[],"metadata").status).toBe(0);
  const against=library(oldWorld,"b",row.consumer,["a"],"metadata"),recompiled=library(newWorld,"b",row.consumer,["a"],"metadata");
  expect(against.status,against.stderr).toBe(0);
  if(row.equal){
   expect(outcome(recompiled)).toBe(outcome(against));
   expect(writeDependenciesV1(against.library.rmeta,rustc!,environment)).toBeDefined();
   const target=pipeline("check-rebased");
   expect(library(target,"a",row.new,[],"metadata").status).toBe(0);
   for(const suffix of["",policy.sidecars.dependencies])copyFileSync(against.library.rmeta+suffix,join(target.deps,depName("b","rmeta")+suffix));
   expect(rebaseRestoredUnitV1({outputs:[join(target.deps,depName("b","rmeta"))],directories:[target.deps],digest:fileDigest,inner:rustc!,environment})).toBe(true);
   const dependant=library(target,"c","pub fn h() -> u32 { b::run() }\n",["a","b"],"metadata");
   expect(dependant.status,dependant.stderr).toBe(0);
  }else{
   expect(digestOf(row.old)===digestOf(row.new),"the classifier refuses").toBe(false);
   expect(outcome(recompiled),"the dependant's check result differs").not.toBe(outcome(against));
  }
 },180000);
}

real("compiler law (why there is no code tier): a dependant's object may call an instance the old library defined and the new one does not, with or without shared generics",()=>{
 const row=vectors.cases.find(row=>row.label.startsWith("shared generic instance"))!;
 for(const flag of["-Zshare-generics=yes","-Zshare-generics=no"]){
  const full=pipeline("code-full"),old=pipeline("code-old"),target=pipeline("code-rebased"),extra=[flag];
  expect(library(full,"a",row.new,[],"code",extra).status).toBe(0);
  expect(library(full,"b",row.consumer,["a"],"code",extra).status).toBe(0);
  expect(binary(full,["b"],extra).status,"the full rebuild links").toBe(0);
  expect(library(old,"a",row.old,[],"code",extra).status).toBe(0);
  const consumer=library(old,"b",row.consumer,["a"],"code",extra);expect(consumer.status,consumer.stderr).toBe(0);
  expect(writeDependenciesV1(consumer.library.rmeta,rustc!,environment)).toBeDefined();
  expect(library(target,"a",row.new,[],"code",extra).status).toBe(0);
  for(const suffix of[".rlib",".rmeta",".rmeta"+policy.sidecars.dependencies])copyFileSync(consumer.library.rlib.replace(/\.rlib$/u,suffix),join(target.deps,"libb-b"+suffix));
  expect(rebaseRestoredUnitV1({outputs:[join(target.deps,"libb-b.rmeta")],directories:[target.deps],digest:fileDigest,inner:rustc!,environment})).toBe(true);
  const linked=binary(target,["b"],extra);
  expect(linked.status,flag+": the rebased object still refers to the instance of the old library, so it does not link").not.toBe(0);
  expect(linked.stderr).toContain("Undefined symbols");
 }
},180000);

real("compiler law: the crate hash listed by the compiler occurs exactly once in the metadata of the crate and of a dependant, and in no rlib",()=>{
 const p=pipeline("hash");
 const a=library(p,"a","pub fn f(x: u32) -> u32 { x + 1 }\n",[],"code");const b=library(p,"b","pub fn run() -> u32 { a::f(1) }\n",["a"],"code");
 expect(a.status).toBe(0);expect(b.status,b.stderr).toBe(0);
 const listedA=writeDependenciesV1(a.library.rmeta,rustc!,environment),listedB=writeDependenciesV1(b.library.rmeta,rustc!,environment);
 expect(listedA).toBeDefined();expect(listedB).toBeDefined();
 const hashA=JSON.parse(readFileSync(listedA!,"utf8")).svh as string,tableB=JSON.parse(readFileSync(listedB!,"utf8")).dependencies as [string,string][];
 expect(tableB.find(([id])=>id==="a-a")?.[1],"the dependant lists the library by name and filename suffix").toBe(hashA);
 const count=(bytes:Buffer,needle:Buffer)=>{let at=0,found=0;while((at=bytes.indexOf(needle,at))>=0){found++;at++;}return found;};
 expect(count(readFileSync(a.library.rmeta),svhBytesV1(hashA))).toBe(1);
 expect(count(readFileSync(b.library.rmeta),svhBytesV1(hashA))).toBe(1);
 expect(count(readFileSync(b.library.rlib),svhBytesV1(hashA))).toBe(0);
 expect(listedB).toBe(dependenciesFileV1(b.library.rmeta));
});

real("compiler law (metadata chain): without the rebase the dependant of a dependant fails with E0460, with it the chain checks and a changed signature is refused",()=>{
 const oldA="pub fn f(x: u32) -> u32 { x + 1 }\n",newA="pub fn f(x: u32) -> u32 { x + 2 }\n",b="pub fn g() -> u32 { a::f(1) }\n",c="pub fn h() -> u32 { a::f(1) + b::g() }\n";
 const first=pipeline("chain-old");
 expect(library(first,"a",oldA,[],"metadata").status).toBe(0);
 const oldB=library(first,"b",b,["a"],"metadata");expect(oldB.status,oldB.stderr).toBe(0);
 expect(writeDependenciesV1(oldB.library.rmeta,rustc!,environment)).toBeDefined();
 const second=pipeline("chain-new");
 expect(library(second,"a",newA,[],"metadata").status).toBe(0);
 copyFileSync(oldB.library.rmeta,join(second.deps,"libb-b.rmeta"));copyFileSync(oldB.library.rmeta+policy.sidecars.dependencies,join(second.deps,"libb-b.rmeta"+policy.sidecars.dependencies));
 const stale=library(second,"c",c,["a","b"],"metadata");
 expect(stale.status,"a library compiled against the old hash is refused").not.toBe(0);expect(stale.stderr).toContain("E0460");
 expect(rebaseRestoredUnitV1({outputs:[join(second.deps,"libb-b.rmeta")],directories:[second.deps],digest:fileDigest,inner:rustc!,environment})).toBe(true);
 const fresh=library(second,"c",c,["a","b"],"metadata");expect(fresh.status,fresh.stderr).toBe(0);
 const hashNow=currentHashV1(join(second.deps,"liba-a.rmeta"),fileDigest,rustc!,environment);
 expect(JSON.parse(readFileSync(join(second.deps,"libb-b.rmeta"+policy.sidecars.dependencies),"utf8")).dependencies.find(([id]:[string,string])=>id==="a-a")[1]).toBe(hashNow);
 expect(rebaseRestoredUnitV1({outputs:[join(second.deps,"libb-b.rmeta")],directories:[second.deps],digest:fileDigest,inner:rustc!,environment}),"a second rebase has nothing to do").toBe(true);
 const missing=pipeline("chain-missing");
 copyFileSync(oldB.library.rmeta,join(missing.deps,"libb-b.rmeta"));
 expect(rebaseRestoredUnitV1({outputs:[join(missing.deps,"libb-b.rmeta")],directories:[missing.deps],digest:fileDigest,inner:rustc!,environment}),"a unit without its sidecar cannot be proven consistent").toBe(false);
},180000);

test("identity sidecars name the current bytes of a metadata file and a changed file loses its identity",()=>{
 const root=directory("identity"),rmeta=join(root,"liba-a.rmeta"),rlib=join(root,"liba-a.rlib");
 writeFileSync(rmeta,"meta");writeFileSync(rlib,"lib");
 const identity="a".repeat(64);
 writeIdentitiesV1({outputs:[rmeta,rlib],digest:fileDigest,basis:"b".repeat(64),identity:()=>identity,inner:["false"],environment});
 expect(interfaceIdentityV1(rmeta,sha("meta"))).toBe("iface:"+identity);
 expect(interfaceIdentityV1(rmeta,sha("other")),"bytes changed behind the wrapper").toBeUndefined();
 expect(interfaceIdentityV1(join(root,"unknown.rmeta"),sha("meta"))).toBeUndefined();
 expect(interfaceIdentityV1(rlib,sha("lib")),"code files are never identified by interface").toBeUndefined();
 expect(existsSync(identityFileV1(rmeta))).toBe(true);expect(existsSync(identityFileV1(rlib))).toBe(false);
 writeIdentitiesV1({outputs:[rmeta],digest:fileDigest,basis:"b".repeat(64),identity:undefined,inner:["false"],environment});
 expect(interfaceIdentityV1(rmeta,sha("meta")),"a unit that does not qualify has no identity").toBeUndefined();
});
