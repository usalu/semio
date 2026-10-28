/** ♻ Contract: a Nextest assertion step reads the resolved `cargo metadata` Cargo planning already captured (`--cargo-metadata`, without `--manifest-path`, which Nextest refuses beside it), every other step and plan stays as it was, and real Nextest then lists exactly the tests it lists from its own metadata while starting no Cargo process; repository test execution writes the planned metadata and hands it over.
 * @see ../../🟦️.ts
 * @see ../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts
 * @see https://nexte.st/docs/ci-features/reusing-builds/ */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cargoTestPlanV1,type CargoTestPolicyV1} from "../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

test("only the Nextest assertion step changes, to read the planned metadata instead of the manifest",()=>{
 for(const raw of fixture.policies){
  const policy={...raw,coverageEnabled:raw.coveragePath!==null} as CargoTestPolicyV1,request={manifestPath:policy.manifestPath,packages:["owner-package"],cwd:"/owner"},plain=cargoTestPlanV1(request,policy,"/owner/results/binaries-metadata.json"),reused=cargoTestPlanV1(request,policy,"/owner/results/binaries-metadata.json","/owner/results/cargo-metadata.json");
  if(!policy.nextest||policy.coverageEnabled){expect(reused).toEqual(plain);continue;}
  expect(reused.slice(0,-1)).toEqual(plain.slice(0,-1));
  const before=plain.at(-1)!.args,after=reused.at(-1)!.args,at=before.indexOf("--manifest-path");
  expect(after).toEqual([...before.slice(0,at),"--cargo-metadata","/owner/results/cargo-metadata.json",...before.slice(at+2)]);
 }
 expect(()=>cargoTestPlanV1({manifestPath:fixture.policies[0]!.manifestPath,packages:[],cwd:"/owner"},{...fixture.policies[0],coverageEnabled:false} as CargoTestPolicyV1,"/owner/m.json","relative.json")).toThrow();
});

test("real Nextest lists the same tests from the planned metadata as from its own and starts no Cargo process for them",()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"nxm-"))),shim=join(base,"bin"),log=join(base,"cargo.log"),real=spawnSync("rustup",["which","cargo"],{encoding:"utf8"}).stdout.trim();
 try{
  const put=(path:string,body:string)=>{mkdirSync(join(base,path,".."),{recursive:true});writeFileSync(join(base,path),body);};
  put("Cargo.toml",'[workspace]\nresolver="2"\nmembers=["app","core"]\n');
  put("app/Cargo.toml",'[package]\nname="nxm-app"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nnxm-core={path="../core"}\n[target.x86_64-pc-windows-msvc.dependencies]\nnxm-win={path="../win"}\n');put("app/src/lib.rs","#[test] fn uses_core() { assert_eq!(nxm_core::one(), 1); }\n");
  put("core/Cargo.toml",'[package]\nname="nxm-core"\nversion="0.1.0"\nedition="2021"\n');put("core/src/lib.rs","pub fn one() -> u8 { 1 }\n#[test] fn core_one() {}\n");put("core/tests/smoke.rs","#[test] fn smoke() {}\n");
  put("win/Cargo.toml",'[package]\nname="nxm-win"\nversion="0.1.0"\nedition="2021"\n');put("win/src/lib.rs","\n");
  put("bin/cargo",`#!/bin/sh\necho "$*" >> '${log}'\nexec '${real}' "$@"\n`);chmodSync(join(shim,"cargo"),0o755);
  const manifest=join(base,"Cargo.toml"),environment={...process.env,CARGO_TARGET_DIR:join(base,"target")},run=(command:string,args:string[],shimmed=false)=>{const result=spawnSync(command,args,{cwd:base,encoding:"utf8",env:shimmed?{...environment,PATH:shim+":"+process.env.PATH}:environment,maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return result.stdout;};
  run("cargo",["generate-lockfile","--offline","--manifest-path",manifest]);
  writeFileSync(join(base,"planned.json"),run("cargo",["metadata","--format-version=1","--locked","--offline","--manifest-path",manifest]));
  writeFileSync(join(base,"binaries.json"),run("cargo",["nextest","list","--list-type","binaries-only","--message-format","json","--manifest-path",manifest,"-p","nxm-app","-p","nxm-core"]));
  const tests=(text:string)=>Object.values((JSON.parse(text) as {"rust-suites":Record<string,{testcases:Record<string,unknown>}>})["rust-suites"]).flatMap(suite=>Object.keys(suite.testcases)).sort();
  const own=tests(run("cargo-nextest",["nextest","list","--binaries-metadata",join(base,"binaries.json"),"--manifest-path",manifest,"--message-format","json"],true));
  rmSync(log,{force:true});
  const reused=tests(run("cargo-nextest",["nextest","list","--binaries-metadata",join(base,"binaries.json"),"--cargo-metadata",join(base,"planned.json"),"--message-format","json"],true));
  run("cargo-nextest",["nextest","run","--binaries-metadata",join(base,"binaries.json"),"--cargo-metadata",join(base,"planned.json"),"--no-tests","fail"],true);
  expect(reused).toEqual(own);expect(own).toEqual(["core_one","smoke","uses_core"]);
  expect(spawnSync("cat",[log],{encoding:"utf8"}).stdout).toBe("");
 }finally{rmSync(base,{recursive:true,force:true});}
},300_000);

test("repository test execution writes the planned metadata and hands it to the plan, and planning returns the metadata it planned from",()=>{
 const testing=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),capture=readFileSync(new URL("../../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts",import.meta.url),"utf8");
 expect(testing).toContain('writeFileSync(cargoMetadata,JSON.stringify(planned))');expect(testing).toContain("cargoTestPlanV1(request,policy,metadata,cargoMetadata)");expect(testing).toContain("captureCargoCompilationPlanV1(");
 expect(capture).toContain("return Object.freeze({graph,metadata:packageFacts});");
});
