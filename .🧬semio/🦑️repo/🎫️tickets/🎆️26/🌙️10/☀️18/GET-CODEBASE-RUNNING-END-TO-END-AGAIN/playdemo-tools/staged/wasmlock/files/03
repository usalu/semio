import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {WASM_BINDGEN_LOCKS_V1,requireWasmBindgenLocksV1,requireWasmBindgenSourceLockV1,wasmBindgenLockMismatches,wasmBindgenLockOfV1,wasmBindgenVersion} from "../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";
const lock=(version:string)=>`version = 4\n\n[[package]]\nname = "wasm-bindgen"\nversion = "${version}"\n`;
const root=resolve(import.meta.dir,"../../../../../../../..");
test("a lockfile whose wasm-bindgen differs from the pinned generator is named, an equal one is silent",()=>{
 expect(wasmBindgenLockMismatches("0.2.129",{"a/Cargo.lock":lock("0.2.129")})).toEqual([]);
 expect(wasmBindgenLockMismatches("0.2.129",{"a/Cargo.lock":lock("0.2.126")})).toEqual(["a/Cargo.lock: wasm-bindgen 0.2.126, pinned binding generator 0.2.129"]);
 expect(wasmBindgenLockMismatches("0.2.129",{"b/Cargo.lock":"version = 4\n"})[0]).toContain("exactly one wasm-bindgen version");
});
test("every declared wasm lockfile of the repository carries the pinned generator version, checked against Bun's TOML parser",()=>{
 const pinned=wasmBindgenVersion(readFileSync(join(root,"Cargo.lock"),"utf8"));
 for(const path of WASM_BINDGEN_LOCKS_V1){
  const parsed=Bun.TOML.parse(readFileSync(join(root,path),"utf8")) as {package:{name:string;version:string}[]};
  expect({path,versions:parsed.package.filter(item=>item.name==="wasm-bindgen").map(item=>item.version)}).toEqual({path,versions:[pinned]});
 }
 expect(()=>requireWasmBindgenLocksV1(root)).not.toThrow();
});
test("a WASM source owner resolves the lockfile beside the nearest workspace manifest and is refused before compiling when that lock drifts",()=>{
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"wasm-lock-"));
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);};
 try{
  put("Cargo.toml","[workspace]\nmembers = [\"a\"]\n");put("Cargo.lock",lock("0.2.129"));put("a/Cargo.toml","[package]\nname = \"a\"\n");
  put("b/Cargo.toml","[workspace.dependencies]\nserde = \"1\"\n");put("b/Cargo.lock",lock("0.2.126"));put("b/c/Cargo.toml","[package]\nname = \"c\"\n");
  put("d/Cargo.toml","[package]\nname = \"d\"\n[workspace]\n");put("d/Cargo.lock",lock("0.2.129"));
  expect([wasmBindgenLockOfV1(base,"a"),wasmBindgenLockOfV1(base,"b/c"),wasmBindgenLockOfV1(base,"d")]).toEqual(["Cargo.lock","b/Cargo.lock","d/Cargo.lock"]);
  expect(()=>requireWasmBindgenSourceLockV1(base,"a")).not.toThrow();
  expect(()=>requireWasmBindgenSourceLockV1(base,"d")).not.toThrow();
  expect(()=>requireWasmBindgenSourceLockV1(base,"b/c")).toThrow("b/Cargo.lock: wasm-bindgen 0.2.126, pinned binding generator 0.2.129");
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("every WASM package source of the repository resolves a declared lockfile that carries the pinned generator",()=>{
 const listed=Bun.spawnSync(["git","ls-files","-z","--","*📋️project.json",":!.🧬semio"],{cwd:root});
 expect(listed.exitCode).toBe(0);
 const owners=new TextDecoder().decode(listed.stdout).split("\0").filter(path=>path&&existsSync(join(root,path))).flatMap(path=>Object.values((JSON.parse(readFileSync(join(root,path),"utf8")) as {targets?:Record<string,{metadata?:{semio?:{wasmPackageSourceV1?:{sourceOwner?:string}}}}>}).targets??{}).flatMap(target=>target.metadata?.semio?.wasmPackageSourceV1?.sourceOwner??[]));
 expect(owners.length).toBeGreaterThan(0);
 for(const owner of owners){
  expect({owner,lock:wasmBindgenLockOfV1(root,owner),declared:WASM_BINDGEN_LOCKS_V1.includes(wasmBindgenLockOfV1(root,owner))}).toEqual({owner,lock:wasmBindgenLockOfV1(root,owner),declared:true});
  expect(()=>requireWasmBindgenSourceLockV1(root,owner)).not.toThrow();
 }
});
