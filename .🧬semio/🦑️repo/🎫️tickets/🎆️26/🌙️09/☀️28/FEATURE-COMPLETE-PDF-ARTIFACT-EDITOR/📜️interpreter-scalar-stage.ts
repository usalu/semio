/** 🧮 Move interpreter scalar semantics to their own bounded, dependency-free Rust owner. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,readFileSync} from "node:fs";
import {join,relative,resolve} from "node:path";
const interpreter="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter";
const scalar=interpreter+"/🧮️numeric-execution";
const packageRoot=scalar+"/📦️packages/🦀️rust";
const packageName="semio-plugin-interpreter-scalar";
const global="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧯️test-support/📜️repo-text/🦀️.rs";
const owner=interpreter+"/📦️packages/🦀️rust/Cargo.toml";
const identity=interpreter+"/🏃️runtime/📥️snapshot-operation-and-input-records/🦀️.rs";
const main=interpreter+"/🦀️.rs";
const model=interpreter+"/🧱️core-types-and-module-model/🦀️.rs";
const dispatch=scalar+"/🦀️.rs";
const math=interpreter+"/🔢️numeric-semantics/🦀️.rs";
const vars=scalar+"/📐️value.rs";
const algo=scalar+"/🔢️dispatch.rs";
const module=scalar+"/🌱️scalar.rs";
type Row = [string,string|null,string];
const records=new Map<string,Row>();
const sha=(s:string)=>createHash("sha256").update(s).digest("hex");
const read=(p:string)=>readFileSync(p,"utf8");
function record(p:string,body:string){
    const has=existsSync(p);
    assert.ok(resolve(p).normalize("NFD").length<=256,p);
    const pre=has?read(p):null;
    if(pre!==body)records.set(p,[p,pre===null?null:sha(pre),body]);
}
function exact(s:string,old:string,value:string){
    assert.equal(s.split(old).length,2,old.slice(0,170));
    return s.replace(old,value);
}
function section(s:string,from:string,to:string){
    const a=s.indexOf(from),b=s.indexOf(to,a+from.length);
    assert.ok(a>=0&&b>a,from);return s.slice(a,b);
}
const modelSource=read(model);
const valueTypes=section(modelSource,"#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum ValueType","#[derive(Clone, Debug, PartialEq, Eq)]\npub struct HostCall");
record(model,exact(modelSource,valueTypes,"pub use semio_plugin_interpreter_scalar::{CoreError, Value, ValueType};\n\n"));
record(vars,"//! 📐 Scalar values, types and traps of the core interpreter.\n\nuse super::{canonical_f32, canonical_f64};\nuse std::fmt::{Display, Formatter};\n\n"+valueTypes.replaceAll("pub(super) fn","pub fn"));
const dispatchSource=read(dispatch);
const machineAdapter=section(dispatchSource,"pub(super) fn execute_numeric(opcode:","//#region 🧮️NumericExecution");
record(dispatch,"//! 🧮 Numeric instructions of the interpreter machine.\n\nuse super::*;\n\n"+machineAdapter.trimEnd()+"\n");
record(algo,exact(dispatchSource,machineAdapter,"\n").replaceAll("pub(super) ","pub ").replace("impl NumericError{fn message","impl NumericError{pub fn message"));
record(math,read(math).replaceAll("pub(super) fn","pub fn"));
record(module,`//! 🔢 Scalar execution semantics of the repository's WASM core interpreter.
#[path = "📐️value.rs"]
mod value;
pub use value::{CoreError, Value, ValueType};
#[path = "🔢️dispatch.rs"]
mod dispatch;
pub use dispatch::{NumericError, execute_numeric_fixed, numeric_arity};
#[path = "../🔢️numeric-semantics/🦀️.rs"]
mod numeric_semantics;
pub use numeric_semantics::*;

pub const SOURCE: &str = concat!(
    include_str!("🌱️scalar.rs"),
    include_str!("📐️value.rs"),
    include_str!("🔢️dispatch.rs"),
    include_str!("../🔢️numeric-semantics/🦀️.rs"),
);
`);
let coreSource=read(main);
coreSource=exact(coreSource,"pub const INTERPRETER_SOURCE: &str = concat!(", "pub const INTERPRETER_SOURCE: &[&str] = &[concat!(");
coreSource=exact(coreSource,'    include_str!("🔢️numeric-semantics/🦀️.rs"),\n',"");
coreSource=exact(coreSource,'    include_str!("🏗️instance/🧊️prepared/🦀️.rs"),\n','    include_str!("🏗️instance/🧊️prepared/🦀️.rs"),\n    include_str!("🏗️instance/🧊️prepared/🧵️image-wire/🦀️.rs"),\n');
coreSource=exact(coreSource,');\n\nuse std::fmt::{Display, Formatter};\nuse std::sync::Arc;', '), semio_plugin_interpreter_scalar::SOURCE];\n\nuse std::sync::Arc;');
coreSource=exact(coreSource,"use numeric_execution::{execute_numeric,execute_numeric_fixed,numeric_arity,NumericError,map_top};","use numeric_execution::{execute_numeric, map_top};");
coreSource=exact(coreSource,'#[path = "🔢️numeric-semantics/🦀️.rs"]\nmod numeric_semantics;\n','use semio_plugin_interpreter_scalar as numeric_semantics;\n');
const scalarUse=section(coreSource,"use numeric_semantics::{",";\n\n");
coreSource=exact(coreSource,scalarUse,"use numeric_semantics::{execute_numeric_fixed, numeric_arity, NumericError, saturating_i32_from_f32, saturating_i32_from_f64, saturating_i64_from_f32, saturating_i64_from_f64}");
record(main,coreSource);
let ident=read(identity);
ident=exact(ident,"        let interpreter = semio_plugin_interpreter::INTERPRETER_SOURCE.as_bytes();","        let interpreter = semio_plugin_interpreter::INTERPRETER_SOURCE;\n        let interpreter_len = interpreter.iter().map(|part| part.len()).sum::<usize>();");
ident=exact(ident,"72 + interpreter.len() + host.len()+imports.len()", "72 + interpreter_len + host.len() + imports.len()");
ident=exact(ident,"        engine.extend_from_slice(&(interpreter.len() as u64).to_le_bytes());\n        engine.extend_from_slice(interpreter);","        engine.extend_from_slice(&(interpreter_len as u64).to_le_bytes());\n        for part in interpreter { engine.extend_from_slice(part.as_bytes()); }");
record(identity,ident);
let testSupport=read(global);
for(const [anchor,added]of [
    ["/🧮️numeric-execution/🦀️.rs",["/🧮️numeric-execution/🌱️scalar.rs","/🧮️numeric-execution/📐️value.rs","/🧮️numeric-execution/🔢️dispatch.rs"]],
    ["/🏗️instance/🧊️prepared/🦀️.rs",["/🏗️instance/🧊️prepared/🧵️image-wire/🦀️.rs"]],
] as const){
    const line=testSupport.split("\n").find(x=>x.includes("/🧠️interpreter")&&x.includes(anchor));
    if(line===undefined)continue;
    testSupport=exact(testSupport,line,[line,...added.map(next=>line.replace(anchor,next))].join("\n"));
}
record(global,testSupport);
record(owner,exact(read(owner),"[dependencies]\n","[dependencies]\n"+packageName+" = { workspace = true }\n"));
record(packageRoot+"/Cargo.toml",`[package]
workspace = "${relative(resolve(packageRoot),process.cwd()).replaceAll("\\","/")}"
name = "${packageName}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Scalar execution, values and numeric laws of the plugin WASM interpreter"

[package.metadata.semio]
role = "framework"
id = "plugin-interpreter-scalar"

[lints]
workspace = true

[lib]
path = "../../🌱️scalar.rs"
`);
record(packageRoot+"/📋️project.json",JSON.stringify({
    name:"@semio-tech/framework-plugin-interpreter-scalar",
    root:packageRoot,sourceRoot:scalar,projectType:"library",tags:["lang:rust","role:framework","family:os-plugin"],
},null,2)+"\n");
const root=read("Cargo.toml");
const member=`    "${interpreter}/📦️packages/🦀️rust",`;
const binding=`semio-plugin-interpreter = { path = "${interpreter}/📦️packages/🦀️rust" }`;
record("Cargo.toml",exact(exact(root,member,member+`\n    "${packageRoot}",`),binding,binding+`\n${packageName} = { path = "${packageRoot}" }`));
const git=Bun.spawn(["repo","tools","exec","--light","git","--","-c","core.quotePath=false","ls-files","-z","--","Cargo.lock","**/Cargo.lock"],{stdout:"pipe",stderr:"inherit"});
const files=(await new Response(git.stdout).text()).split("\0").filter(Boolean);
assert.equal(await git.exited,0);
let locks=0;
for(const file of files){
    if(!existsSync(file))continue;
    const lock=read(file);
    if(!lock.includes('name = "semio-plugin-interpreter"'))continue;
    const anchor='[[package]]\nname = "semio-plugin-interpreter"\n';
    const range=section(lock,anchor,"\n[[package]]");
    let entry=exact(range,"dependencies = [\n","dependencies = [\n");
    const lane=entry.indexOf(' "semio-framework-value",');
    assert.ok(lane>0,file);
    const after=entry.indexOf("\n",lane)+1;
    entry=entry.slice(0,after)+' "'+packageName+'",\n'+entry.slice(after);
    const following='[[package]]\nname = "semio-plugin-interpreter-runtime"';
    const insertion='[[package]]\nname = "'+packageName+'"\nversion = "0.1.0"\n\n';
    const updated=exact(lock,range,entry);
    const pattern=/\[\[package\]\]\nname = "semio-plugin-interpreter-runtime"\n[\s\S]*?\n\n/;
    const runtime=updated.match(pattern)?.[0];
    assert.ok(runtime&&updated.includes(following),file);
    record(file,exact(updated,runtime,runtime+insertion));
    locks++;
}
assert.ok(locks>=20);
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const original=new Map<string,string|null>();
const projected=new Map<string,string|null>();
for(const [file,,body]of records.values()){original.set(resolve(file),existsSync(file)?read(file):null);projected.set(resolve(file),body);projected.set(file,body);}
const law=[];
for(const os of ["linux","macos","windows","wasi"]){
    const cfg=[`target_os="${os}"`,os==="wasi"?'target_arch="wasm32"':os==="macos"?'target_arch="aarch64"':'target_arch="x86_64"',os==="wasi"?"wasm":os==="windows"?"windows":"unix"];
    const baseline=await measureRustCrate(process.cwd(),resolve(main),false,new Map(),resolve(owner,".."),cfg);
    for(const [name,root,manifest]of [["core",main,owner],["scalar",module,packageRoot+"/Cargo.toml"],["runtime",interpreter+"/🏃️runtime/🦀️.rs",interpreter+"/🏃️runtime/📦️packages/🦀️rust/Cargo.toml"]]){
        const actual=await measureRustCrate(process.cwd(),resolve(root),false,projected,resolve(manifest,".."),cfg);
        const {files,lines,bytes}=actual;
        law.push({os,name,files,lines,bytes,baseline:name==="core"?{files:baseline.files,lines:baseline.lines,bytes:baseline.bytes}:undefined});
        assert.ok(files<=32&&lines<=4000&&bytes<=262144,JSON.stringify(law.at(-1)));
    }
}
for(const [file,pre]of original)assert.equal(existsSync(file)?read(file):null,pre,file);
const generated=join(import.meta.dir,"🗑️generated");
await Bun.write(join(generated,"interpreter-scalar-overlay.json"),JSON.stringify([...records.values()],null,2)+"\n");
await Bun.write(join(generated,"interpreter-scalar-bounds.json"),JSON.stringify(law,null,2)+"\n");
console.log("[DEBUG] Staged interpreter Scalar cut",records.size,locks,law[0],law[1]);
