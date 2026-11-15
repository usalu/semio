import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {INTERFACE_POLICY_V1,interfaceDigestV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔭️interface/🟦️.ts";
import vectors from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔭️interface/🧪️tests/🔣️.json" with {type:"json"};
import {INTERFACE_CLOSURE_POLICY_V1,interfaceClosureDigestV1,interfaceClosureFilesV1,interfaceDependencyInputV1,interfaceFileFactsV1,interfaceTargetV1,isInterfaceClosureV1,type InterfaceRowV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {sourceClosureDigest} from "../../🧮️digest/🟨️.mjs";
import {wire,withNativePreparation} from "../../../🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-inputs-revision"});

/** 🧿️ Interface closure laws: a dependency crate of a check task is keyed by what a type check observes of its sources; the rule equals the unit-cache rule on every vector of the compiler-checked corpus, the closure reader agrees with the pure rule, and a closure that does not list the marker keeps its bytes. */
const policy=INTERFACE_CLOSURE_POLICY_V1,hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const rowsOf=(files:Record<string,string|null>):InterfaceRowV1[]=>Object.entries(files).map(([name,text])=>text===null?{name,sha256:null,facts:null}:{name,sha256:hash(text),facts:name.endsWith(".rs")?interfaceFileFactsV1(Buffer.from(text)):null});
const digestOf=(files:Record<string,string|null>)=>interfaceClosureDigestV1(rowsOf(files));
const reference=(files:Record<string,string|null>)=>interfaceDigestV1(Object.entries(files).map(([name,text])=>({name,bytes:text===null?null:Buffer.from(text)}))).digest;
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the policy is a closed constant of its schema and every member exists",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema.definitions.Policy,policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema.definitions.Policy,{...policy,extra:1})).toBe(false);
 expect(policy.members).toContain(policy.marker);
 let root=import.meta.dir;while(!existsSync(join(root,policy.marker))&&dirname(root)!==root)root=dirname(root);
 for(const member of policy.members)expect(existsSync(join(root,member)),member).toBe(true);
 expect(policy.members).toContain(policy.marker);
 expect(policy.members.some(path=>path.endsWith("/🔭️interface/🟦️.ts")&&path.includes("unit-cache"))).toBe(true);
});

test("the closure rule gives the same equality as the unit-cache rule on all 37 compiler-checked vectors, with one file and with a crate around it",()=>{
 expect(vectors.cases.length).toBe(37);
 for(const row of vectors.cases){
  const a={"lib.rs":row.old},b={"lib.rs":row.new};
  expect(digestOf(a)===digestOf(b),row.label).toBe(row.equal);
  expect(reference(a)===reference(b),row.label+" (unit rule)").toBe(row.equal);
  const around=(text:string)=>({"Cargo.toml":"[package]\nname=\"a\"\n","lib.rs":text,"util.rs":"pub fn u()->u32{1}\n","data.json":"{}"});
  expect(digestOf(around(row.old))===digestOf(around(row.new)),row.label+" (crate)").toBe(row.equal);
 }
});

test("the set rule: one include or one unmaskable file puts every Rust file back to bytes, other files and absence count",()=>{
 const base={"a.rs":"pub fn f(x: u32) -> u32 { x + 1 }\n","b.rs":"pub fn g() -> u32 { 2 }\n"};
 const edited={...base,"a.rs":"pub fn f(x: u32) -> u32 { let y = x * 3; y + 7 }\n"};
 expect(digestOf(base)).toBe(digestOf(edited));
 const spliced={...base,"c.rs":"include!(\"b.rs\");\n"};
 expect(digestOf(spliced)).not.toBe(digestOf({...spliced,"a.rs":edited["a.rs"]}));
 const broken={...base,"c.rs":"pub fn h() { \"unfinished\n"};
 expect(digestOf(broken)).not.toBe(digestOf({...broken,"a.rs":edited["a.rs"]}));
 expect(digestOf({...base,"Cargo.toml":"x"})).not.toBe(digestOf({...base,"Cargo.toml":"y"}));
 expect(digestOf({...base,"d.rs":null})).not.toBe(digestOf(base));
 expect(digestOf({...base,"d.rs":null})).not.toBe(digestOf({...base,"d.rs":""}));
 expect(digestOf({"b.rs":base["b.rs"],"a.rs":base["a.rs"]})).toBe(digestOf(base));
 for(const files of[base,spliced,broken,edited])expect(digestOf(files)===digestOf(base),"matches the unit rule").toBe(reference(files)===reference(base));
});

test("the graph names the interface group exactly as the policy does",()=>{
 expect(policy.group).toBe("nativeInterface");
 const inputs=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
 expect(inputs).toContain('nativeInterface: workspaceNegationClosed(');
 expect(inputs).toContain('name === "nativeInterface" ? "interface" : "bytes"');
 const natives=readFileSync(new URL("../../../🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs",import.meta.url),"utf8");
 expect(natives).toContain("interfaceDependencyInputV1(name, target, input, dependencies)");
 expect(natives).not.toContain('{ input: "nativeSources", projects: dependencies }');
});

test("the roster of an interface closure carries the marker and the implementation, and only that roster is one",()=>{
 const files=interfaceClosureFilesV1(["z/lib.rs","a/Cargo.toml"]);
 expect(files).toEqual([...new Set([...files])].sort());
 expect(isInterfaceClosureV1(files)).toBe(true);
 expect(isInterfaceClosureV1(["z/lib.rs","a/Cargo.toml"])).toBe(false);
 for(const member of policy.members)expect(files).toContain(member);
});

test("only the ^nativeSources of a production check names the interface group",()=>{
 const production={metadata:{semio:{execution:{scope:"production"}}}},testing={metadata:{semio:{execution:{scope:"test"}}}};
 expect(interfaceTargetV1("check",production)).toBe(true);
 for(const [name,target] of[["check",testing],["build",production],["test",production],["lint",production],["check-extra",production],["check",{}]] as const)expect(interfaceTargetV1(name,target),name).toBe(false);
 expect(interfaceDependencyInputV1("check",production,"^nativeSources",["p","q"])).toEqual([{input:"nativeInterface",projects:["p","q"]}]);
 expect(interfaceDependencyInputV1("check",production,"^nativeTestSources",["p"])).toEqual([{input:"nativeSources",projects:["p"]}]);
 expect(interfaceDependencyInputV1("build",production,"^nativeSources",["p"])).toEqual([{input:"nativeSources",projects:["p"]}]);
 expect(interfaceDependencyInputV1("test",testing,"^nativeSources",["p"])).toEqual([{input:"nativeSources",projects:["p"]}]);
 expect(interfaceDependencyInputV1("check",production,"^nativeSources",[])).toEqual([]);
});

test("the exact Source reader digests an interface closure like the pure rule and a plain closure by bytes",()=>withBuildTestInvocationV1(async original=>{
 const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Ticket Source fixture sink required");
 const root=mkdtempSync(join(sink,"if-")),control=createIssuedResourceControlV1(original,"source:interface-laws",120000);scratch.push(root);
 const crate={"c/Cargo.toml":"[package]\nname=\"c\"\n","c/lib.rs":"pub fn f(x: u32) -> u32 { x + 1 }\npub struct S;\nimpl S { pub fn m(&self) -> u32 { 1 } }\n","c/data.json":"{}"};
 const members=Object.fromEntries(policy.members.map(path=>[path,"member "+path]));
 const write=(files:Record<string,string>)=>{for(const [name,text] of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}};
 write({...crate,...members});
 const names=[...Object.keys(crate),...policy.members].sort();
 const interfaceRoster={version:1,files:names},plainRoster={version:1,files:Object.keys(crate).sort()};
 const first=await sourceClosureDigest(root,interfaceRoster,control),plain=await sourceClosureDigest(root,plainRoster,control);
 const pure=interfaceClosureDigestV1(names.map(name=>({name,sha256:hash(({...crate,...members} as Record<string,string>)[name]!),facts:name.endsWith(".rs")?interfaceFileFactsV1(Buffer.from(({...crate,...members} as Record<string,string>)[name]!)):null})));
 expect(first).toBe(pure);
 const legacy=(list:readonly string[])=>{const digest=createHash("sha256");for(const name of list){digest.update(name).update("\0");const bytes=readFileSync(join(root,name));digest.update("file\0").update(hash(bytes)).update("\0");}return digest.digest("hex");};
 expect(plain).toBe(legacy(plainRoster.files));
 write({"c/lib.rs":"pub fn f(x: u32) -> u32 { let y = x * 3; y + 7 }\npub struct S;\nimpl S { pub fn m(&self) -> u32 { 1 } }\n"});
 expect(await sourceClosureDigest(root,interfaceRoster,control)).toBe(first);
 expect(await sourceClosureDigest(root,plainRoster,control)).not.toBe(plain);
 write({"c/lib.rs":"pub fn f(x: u32) -> u64 { 1 }\npub struct S;\nimpl S { pub fn m(&self) -> u32 { 1 } }\n"});
 expect(await sourceClosureDigest(root,interfaceRoster,control)).not.toBe(first);
 write({"c/data.json":"{ }"});
 const afterData=await sourceClosureDigest(root,interfaceRoster,control);
 write({[policy.members[1]!]:"member changed"});
 expect(await sourceClosureDigest(root,interfaceRoster,control)).not.toBe(afterData);
}),120000);

test("project preparation names the interface group of the dependency crates for a production check and the source group for everything else",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r40-prep-")));scratch.push(root);
 const crate=(name:string,dependencies:string[])=>{mkdirSync(join(root,name),{recursive:true});writeFileSync(join(root,name,"Cargo.toml"),`[package]\nname="${name}"\nversion="0.1.0"\nedition="2021"\n[dependencies]\n`+dependencies.map(dependency=>`${dependency}={path="../${dependency}"}\n`).join(""));};
 crate("a",[]);crate("b",["a"]);crate("c",["b"]);
 wire({sourceAnalyses:{prepareSourceAnalysesV1:()=>undefined,RUST_SOURCE_CACHE:new Map(),RUST_INPUT_SNAPSHOTS:new WeakMap()},coordinates:{buildPath:(path:string)=>path.split("\\").join("/"),SCRIPT_BASENAME:"script.ts"},targetPolicy:{targetExecutionKindV1:(target:any)=>target.metadata?.semio?.execution?.kind,executionTargetV1:()=>({})}});
 const target=(scope:string)=>({inputs:["nativeSources","^nativeSources"],metadata:{semio:{execution:{kind:"native",scope}}}});
 const byRoot=new Map([["a","a-rs"],["b","b-rs"],["c","c-rs"]]);
 const prepare=(name:string)=>withNativePreparation({name:name+"-rs",root:name,metadata:{},targets:{check:target("production"),build:target("production"),test:{...target("test"),inputs:["nativeTestSources","^nativeSources"]},"check-more":target("production")}} as any,root,{},new Map(),byRoot,new Map()) as any;
 const c=prepare("c").targets;
 expect(c.check.inputs).toEqual(["nativeSources",{input:"nativeInterface",projects:["a-rs","b-rs"]}]);
 expect(c.build.inputs).toEqual(["nativeSources",{input:"nativeSources",projects:["a-rs","b-rs"]}]);
 expect(c.test.inputs).toEqual(["nativeTestSources",{input:"nativeSources",projects:["a-rs","b-rs"]}]);
 expect(c["check-more"].inputs).toEqual(["nativeSources",{input:"nativeSources",projects:["a-rs","b-rs"]}]);
 expect(prepare("a").targets.check.inputs).toEqual(["nativeSources"]);
 expect(prepare("b").targets.check.inputs).toEqual(["nativeSources",{input:"nativeInterface",projects:["a-rs"]}]);
});
