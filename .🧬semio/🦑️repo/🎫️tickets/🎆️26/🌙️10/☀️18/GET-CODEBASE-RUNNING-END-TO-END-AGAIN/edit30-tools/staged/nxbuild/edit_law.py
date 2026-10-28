"""📋 Anchored, idempotent extension of the `♻️cargo-metadata` law for batch `nxbuild`: edit_law.py <root> check|apply. Adds the build-step plan law and the real Nextest law (the build starts `cargo test --no-run` only, the binaries list equals Nextest's own)."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
P = os.path.join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/♻️cargo-metadata/🟦️.ts")
ANCHOR = 'test("repository test execution writes the planned metadata and hands it to the plan, and planning returns the metadata it planned from",()=>{'
ADD = r'''test("the Nextest build step reads the planned metadata only when it describes the request's own workspace manifest, keeping every other argument",()=>{
 for(const raw of fixture.policies){
  const policy={...raw,coverageEnabled:raw.coveragePath!==null} as CargoTestPolicyV1,request={manifestPath:policy.manifestPath,packages:["owner-package","second"],cwd:"/owner"},reused=cargoTestPlanV1(request,policy,"/owner/m.json","/owner/c.json"),own=cargoTestPlanV1(request,policy,"/owner/m.json","/owner/c.json",policy.manifestPath),other=cargoTestPlanV1(request,policy,"/owner/m.json","/owner/c.json","/elsewhere/Cargo.toml");
  expect(other).toEqual(reused);
  if(!policy.nextest||policy.coverageEnabled){expect(own).toEqual(reused);continue;}
  expect(own.slice(1)).toEqual(reused.slice(1));
  const before=reused[0]!.args,after=own[0]!.args,at=before.indexOf("--manifest-path");
  expect(after).toEqual([...before.slice(0,at),"--cargo-metadata","/owner/c.json",...before.slice(at+2)]);
 }
 expect(()=>cargoTestPlanV1({manifestPath:fixture.policies[0]!.manifestPath,packages:["p"],cwd:"/owner"},{...fixture.policies[0],coverageEnabled:false} as CargoTestPolicyV1,"/owner/m.json","/owner/c.json","relative/Cargo.toml")).toThrow();
});

test("real Nextest builds from the planned metadata with only `cargo test --no-run` and lists the same binaries as from its own metadata",()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"nxb-"))),shim=join(base,"bin"),log=join(base,"cargo.log"),real=spawnSync("rustup",["which","cargo"],{encoding:"utf8"}).stdout.trim();
 try{
  const put=(path:string,body:string)=>{mkdirSync(join(base,path,".."),{recursive:true});writeFileSync(join(base,path),body);};
  put("Cargo.toml",'[workspace]\nresolver="2"\nmembers=["app","core"]\n');
  put("app/Cargo.toml",'[package]\nname="nxb-app"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nnxb-core={path="../core"}\n[target.x86_64-pc-windows-msvc.dependencies]\nnxb-win={path="../win"}\n');put("app/src/lib.rs","#[test] fn uses_core() { assert_eq!(nxb_core::one(), 1); }\n");
  put("core/Cargo.toml",'[package]\nname="nxb-core"\nversion="0.1.0"\nedition="2021"\n');put("core/src/lib.rs","pub fn one() -> u8 { 1 }\n#[test] fn core_one() {}\n");put("core/tests/smoke.rs","#[test] fn smoke() {}\n");
  put("win/Cargo.toml",'[package]\nname="nxb-win"\nversion="0.1.0"\nedition="2021"\n');put("win/src/lib.rs","\n");
  put("bin/cargo",`#!/bin/sh\necho "$*" >> '${log}'\nexec '${real}' "$@"\n`);chmodSync(join(shim,"cargo"),0o755);
  const manifest=join(base,"Cargo.toml"),environment={...process.env,CARGO_TARGET_DIR:join(base,"target")},run=(command:string,args:string[],shimmed=false)=>{const result=spawnSync(command,args,{cwd:base,encoding:"utf8",env:shimmed?{...environment,PATH:shim+":"+process.env.PATH,CARGO:join(shim,"cargo")}:environment,maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return result.stdout;};
  run("cargo",["generate-lockfile","--offline","--manifest-path",manifest]);
  writeFileSync(join(base,"planned.json"),run("cargo",["metadata","--format-version=1","--locked","--offline","--manifest-path",manifest]));
  const own=JSON.parse(run("cargo-nextest",["nextest","list","--list-type","binaries-only","--message-format","json","--manifest-path",manifest,"-p","nxb-app","-p","nxb-core"],true));
  const before=readFileSync(log,"utf8").split("\n").filter(Boolean);expect(before.some(line=>/(^| )metadata /u.test(line))).toBe(true);
  rmSync(log,{force:true});
  const planned=JSON.parse(run("cargo-nextest",["nextest","list","--list-type","binaries-only","--message-format","json","--cargo-metadata",join(base,"planned.json"),"-p","nxb-app","-p","nxb-core"],true));
  const calls=readFileSync(log,"utf8").split("\n").filter(Boolean);
  expect(calls.length).toBe(1);expect(calls[0]).toContain("test --manifest-path "+manifest+" --no-run");expect(calls[0]).toContain("--package nxb-app");expect(calls[0]).toContain("--package nxb-core");
  expect(planned).toEqual(own);expect(Object.keys(planned["rust-binaries"]).sort()).toEqual(["nxb-app","nxb-core","nxb-core::smoke"]);
 }finally{rmSync(base,{recursive:true,force:true});}
},300_000);

'''
text = open(P, encoding="utf8").read()
if ADD.split("\n")[0] in text: print("same"); sys.exit(0)
if text.count(ANCHOR) != 1: print("ANCHOR"); sys.exit(1)
new = text.replace(ANCHOR, ADD + ANCHOR)
new = new.replace('expect(testing).toContain("cargoTestPlanV1(request,policy,metadata,cargoMetadata)");', 'expect(testing).toContain("cargoTestPlanV1(request,policy,metadata,cargoMetadata,cargoMetadata&&typeof root===\\"string\\"?join(root,\\"Cargo.toml\\"):undefined)");', 1)
new = new.replace("/** ♻ Contract: a Nextest assertion step reads the resolved `cargo metadata` Cargo planning already captured", "/** ♻ Contract: a Nextest assertion step, and the Nextest build when the metadata describes the request's own workspace manifest, read the resolved `cargo metadata` Cargo planning already captured", 1)
print("edit")
if verb == "apply":
    mode = os.stat(P).st_mode & 0o777; tmp = P + ".e30"; open(tmp, "w", encoding="utf8").write(new); os.chmod(tmp, mode); os.replace(tmp, P)
