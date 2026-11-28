import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import ts from "typescript";
import texts from "../📖️texts/🔣️.json";
import {formatPreflightV1,preflightTargetsV1,renderPreflightTextV1,rustReferencesV1,tsSpecifiersV1,unitPeaksFromWardensV1} from "../🟦️.ts";

/** 🛫️ The target preflight finds, without compiling, what would only fail after the queue: every refusal is typed, bilingual and names its owner; the import extraction agrees with the TypeScript compiler's own `preProcessFile` on a corpus; the lock check agrees with a Python reading of the same files. */
const owners=(rows:readonly [string,string,string][])=>({version:1,owners:rows.map(([name,root,manifest])=>({name,root,manifest,packages:[]}))});
function write(root:string,path:string,text:string){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
function world(build:(root:string)=>void,rows:readonly [string,string,string][],contracts:Record<string,unknown>={}){
 const root=mkdtempSync(join(tmpdir(),"h98-"));write(root,"📋️build-owners.json",JSON.stringify(owners(rows)));
 write(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",JSON.stringify({generatorContracts:contracts}));build(root);return root;
}
const gen=(path:string,target:string|null,ownerPath:string|null)=>({ownership:target?"owned":"external",ownerPath,target,outputRoots:[{path,inclusion:"ignored"}]});

test("a missing generated import schedules its producer first instead of refusing late",async()=>{
 const root=world(r=>{write(r,"app/📋️project.json",'{"name":"app","targets":{}}');write(r,"app/🟦️.ts",'import {x} from "../gen/🤖️generated/🟦️.ts";export const y=x;');write(r,"gen/📋️project.json",'{"name":"gen","targets":{"emit":{}}}');},[["app","app","app/📋️project.json"],["gen","gen","gen/📋️project.json"]],{emit:gen("gen/🤖️generated","gen:emit","gen")});
 const report=await preflightTargetsV1({root,targets:["app:build"]});
 expect(report.ok).toBe(true);expect(report.schedule.map(row=>row.target)).toEqual(["gen:emit"]);rmSync(root,{recursive:true});
});
test("a generated file with no producer, an unregistered one and a producer without the target are typed refusals",async()=>{
 const rows:[string,string,string][]=[["app","app","app/📋️project.json"],["gen","gen","gen/📋️project.json"]];
 for(const [contract,code] of [[gen("gen/🤖️generated",null,null),"generated-no-producer"],[gen("gen/🤖️generated","ghost:emit","ghost"),"generated-producer-unregistered"],[gen("gen/🤖️generated","gen:missing","gen"),"generated-producer-target-missing"]] as const){
  const root=world(r=>{write(r,"app/📋️project.json",'{"name":"app","targets":{}}');write(r,"app/🟦️.ts",'import "../gen/🤖️generated/🟦️.ts";');write(r,"gen/📋️project.json",'{"name":"gen","targets":{"emit":{}}}');},rows,{c:contract});
  const report=await preflightTargetsV1({root,targets:["app"]});expect(report.ok).toBe(false);expect(report.findings.map(f=>f.code)).toContain(`preflight.${code}`);rmSync(root,{recursive:true});
 }
});
test("an unresolved relative import is refused with its owner and a ?raw suffix still resolves",async()=>{
 const root=world(r=>{write(r,"app/📋️project.json",'{"name":"app","targets":{}}');write(r,"app/🟦️.ts",'import a from "./a.txt?raw";import "./missing.ts";import "react";');write(r,"app/a.txt","x");},[["app","app","app/📋️project.json"]]);
 const report=await preflightTargetsV1({root,targets:["app"]});
 expect(report.findings.map(f=>[f.code,f.owner])).toEqual([["preflight.import-unresolved","app"]]);expect(report.findings[0]!.detail.en).toContain("./missing.ts");rmSync(root,{recursive:true});
});
const workspace=(extra:string,lock:string,member:string='[package]\nname = "a"\nversion = "0.1.0"\nworkspace = "../.."\n[lib]\npath = "🦀️.rs"\n[dependencies]\nserde = "1"\n')=>(r:string)=>{
 write(r,"ws/Cargo.toml",`[workspace]\nmembers = ["crates/a"${extra}]\n`);write(r,"ws/crates/a/Cargo.toml",member);write(r,"ws/crates/a/🦀️.rs","mod inner;\n");write(r,"ws/crates/a/inner.rs","");write(r,"ws/Cargo.lock",lock);
};
const goodLock='version = 4\n[[package]]\nname = "a"\nversion = "0.1.0"\ndependencies = [\n "serde",\n]\n[[package]]\nname = "serde"\nversion = "1.0.5"\nsource = "registry+x"\n';
const rustRows:[string,string,string][]=[["a","ws/crates/a","ws/crates/a/Cargo.toml"]];
test("Cargo lock drift is refused with the manifest, the lock and the fix command",async()=>{
 const cases:[string,string,string,string?][]=[["consistent",goodLock,"ok"],["version drift",goodLock.replace('"0.1.0"','"0.0.9"'),"lock-stale"],["member missing",goodLock.replace('name = "a"','name = "b"'),"lock-stale"],["dependency not locked",goodLock.replace(' "serde",\n',""),"lock-stale"],["locked not declared",goodLock.replace(' "serde",\n',' "serde",\n "tokio",\n'),"lock-stale"],["requirement unmet",goodLock.replace('1.0.5','2.0.0'),"lock-stale"]];
 for(const [name,lock,expected] of cases){
  const root=world(workspace("",lock),rustRows);const report=await preflightTargetsV1({root,targets:["a"]});
  if(expected==="ok")expect([name,report.findings.map(f=>f.code)]).toEqual([name,[]]);else{expect([name,report.findings[0]?.code]).toEqual([name,"preflight."+expected]);expect(report.findings[0]!.command).toContain("cargo generate-lockfile --manifest-path ws/Cargo.toml");expect(report.findings[0]!.subject).toBe("ws/Cargo.lock");}
  rmSync(root,{recursive:true});
 }
 const missing=world(r=>{workspace("","")(r);rmSync(join(r,"ws/Cargo.lock"));},rustRows);expect((await preflightTargetsV1({root:missing,targets:["a"]})).findings[0]!.code).toBe("preflight.lock-missing");
 const absent=world(workspace(',"crates/zz"',goodLock),rustRows);expect((await preflightTargetsV1({root:absent,targets:["a"]})).findings.map(f=>f.code)).toContain("preflight.manifest-missing");
 const notMember=world(r=>{workspace("",goodLock,'[package]\nname = "a"\nversion = "0.1.0"\nworkspace = "../.."\n[lib]\npath = "🦀️.rs"\n[dependencies]\nserde = "1"\nb = {path="../b"}\n')(r);write(r,"ws/crates/b/Cargo.toml",'[package]\nname = "b"\nversion = "0.1.0"\n');},rustRows);
 expect((await preflightTargetsV1({root:notMember,targets:["a"]})).findings.map(f=>f.code)).toContain("preflight.manifest-not-member");
});
test("Rust modules and includes resolve, #[path] after other attributes works, generated includes schedule their producer",async()=>{
 const root=world(r=>{workspace("",goodLock)(r);write(r,"ws/crates/a/🦀️.rs",'mod inner;\n#[cfg(test)]\n#[path = "t/x.rs"]\nmod tests;\nmod gone;\nconst G:&str=include_str!("../../../gen/🤖️generated/y.txt");\n');write(r,"ws/crates/a/t/x.rs","");write(r,"gen/📋️project.json",'{"name":"gen","targets":{"emit":{}}}');},[...rustRows,["gen","gen","gen/📋️project.json"]],{emit:gen("gen/🤖️generated","gen:emit","gen")});
 const report=await preflightTargetsV1({root,targets:["a"]});
 expect(report.findings.map(f=>f.code)).toEqual(["preflight.rust-mod-unresolved"]);expect(report.schedule.map(s=>s.target)).toEqual(["gen:emit"]);
});
test("memory and size plan refuses units over the ceiling and warns for large unmeasured sources",async()=>{
 const root=world(r=>{write(r,"app/📋️project.json",'{"name":"app","targets":{}}');write(r,"app/🟦️.ts","export {};");write(r,"big/📋️project.json",'{"name":"big","targets":{}}');write(r,"big/🟦️.ts","x".repeat(5000));},[["app","app","app/📋️project.json"],["big","big","big/📋️project.json"]]);
 const report=await preflightTargetsV1({root,targets:["app","big"],peaks:{app:2000},ceilingMegabytes:1536,largeSourceBytes:4096});
 expect(report.findings.map(f=>[f.code,f.severity])).toEqual([["preflight.memory-over-ceiling","refusal"],["preflight.source-large","warning"]]);
 expect(report.units.map(u=>u.verdict)).toEqual(["over-ceiling","large-source"]);
});
test("an unknown target is refused and every finding text is complete in English and German",async()=>{
 const root=world(()=>{},[]);const report=await preflightTargetsV1({root,targets:["nope:build"]});expect(report.findings[0]!.code).toBe("preflight.target-unknown");
 for(const [code,entry] of Object.entries(texts.findings))for(const text of [entry.detail,entry.fix]){expect(text.en.length).toBeGreaterThan(10);expect(text.de.length).toBeGreaterThan(10);expect([...text.en.matchAll(/\{(\w+)\}/g)].map(m=>m[1]).sort(),code).toEqual([...text.de.matchAll(/\{(\w+)\}/g)].map(m=>m[1]).sort());}
 expect(renderPreflightTextV1({en:"{a}",de:"{a}!"},{a:"x"})).toEqual({en:"x",de:"x!"});
});
test("import extraction agrees with the TypeScript compiler on a corpus",()=>{
 const base=process.env.H98_CORPUS??"";if(!base)return;
 const files:string[]=[];const visit=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(["node_modules","target",".git"].includes(entry.name))continue;const path=join(directory,entry.name);if(entry.isDirectory())visit(path);else if(/\.(ts|tsx)$/.test(entry.name)&&files.length<400)files.push(path);}};visit(base);
 let compared=0;for(const file of files){const source=readFileSync(file,"utf8"),oracle=new Set(ts.preProcessFile(source,true,true).importedFiles.map(row=>row.fileName)),ours=new Set(tsSpecifiersV1(source));for(const spec of oracle){if(!source.includes(JSON.stringify(spec))&&!source.includes(`'${spec}'`))continue;expect([file,spec,ours.has(spec)]).toEqual([file,spec,true]);}compared++;}
 expect(compared).toBeGreaterThan(50);
});
test("Rust reference extraction",()=>{
 const refs=rustReferencesV1('// mod commented;\n#[path="a/b.rs"]mod p;\npub(crate) mod q;\nuse foo::bar;\nuse baz as qux;\nconst X:&str=include_str!("d/e.txt");\n/* mod hidden; */');
 expect(refs.mods).toEqual([{name:"p",path:"a/b.rs"},{name:"q"}]);expect(refs.includes).toEqual(["d/e.txt"]);expect(refs.uses).toContain("foo");expect(rustReferencesV1('macro_rules! m {() => { use hidden::x; mod nope; }}\nuse real::y;').uses).toEqual(["real"]);expect(refs.declared).toContain("qux");
});
test("wardens peaks feed the plan and the report prints in one language",async()=>{
 const store=mkdtempSync(join(tmpdir(),"h98s-"));write(store,"origins/o1/memory.json",JSON.stringify({lastAlarms:[{owner:"semio-build-scheduler rustc --crate-name big_unit --edition=2021",megabytes:1750},{owner:"x rustc --crate-name big_unit",megabytes:900}]}));
 const peaks=unitPeaksFromWardensV1(store);expect(peaks).toEqual({big_unit:1750});
 const root=world(r=>{write(r,"b/Cargo.toml",'[workspace]\n[package]\nname = "big-unit"\nversion = "0.1.0"\n');write(r,"b/Cargo.lock",'[[package]]\nname = "big-unit"\nversion = "0.1.0"\n');},[["big-unit","b","b/Cargo.toml"]]);
 const report=await preflightTargetsV1({root,targets:["big-unit"],peaks});expect(report.findings[0]!.code).toBe("preflight.memory-over-ceiling");
 expect(formatPreflightV1(report,"de")).toContain("Abgelehnt");expect(formatPreflightV1(report,"en")).toContain("Refused");expect(formatPreflightV1(report,"de")).not.toContain("Refused");
});
