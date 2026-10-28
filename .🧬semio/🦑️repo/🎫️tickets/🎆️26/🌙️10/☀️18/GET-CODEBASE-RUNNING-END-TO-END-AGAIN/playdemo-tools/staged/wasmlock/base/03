import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {WASM_BINDGEN_LOCKS_V1,requireWasmBindgenLocksV1,wasmBindgenLockMismatches,wasmBindgenVersion} from "../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";
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
