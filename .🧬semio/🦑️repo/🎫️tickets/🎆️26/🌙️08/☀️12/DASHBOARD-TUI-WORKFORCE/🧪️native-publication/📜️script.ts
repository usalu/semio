import assert from "node:assert/strict";
import { readFileSync, writeFileSync, renameSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),generated=join(ticket,"🗑️generated"),library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",owner=`${library}/⚡️caching`,fixture=`${owner}/🧫️fixtures/🧊️native-renderer-outputs`;
function edit(path:string,transform:(text:string)=>string):void{const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(after,before);const staged=join(generated,`native-publication-${Date.now()}`);writeFileSync(staged,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(staged,file);}
if(process.argv[2]==="test"){
 edit(`${fixture}/🔣️.json`,text=>{const data=JSON.parse(text);data.heldRun={profile:"dev",readyTimeoutMs:5000};return JSON.stringify(data,null,2)+"\n";});
 edit(`${fixture}/📐️schema/🔣️.json`,text=>{const data=JSON.parse(text);data.required.push("heldRun");data.properties.heldRun={const:{profile:"dev",readyTimeoutMs:5000}};return JSON.stringify(data,null,2)+"\n";});
 edit(`${owner}/🧪️tests/🧊️native-renderer-outputs/🟦️.ts`,text=>{
  const source='const source = (stdout: string) => `fn main() { print!(${JSON.stringify(stdout)}); }\\n`;',end='  put(`${rust}/🦀️.rs`, source(fixture.changedStdout));\n  await run(); assert.deepEqual(runs(), ["dev", "dev", "release", "release"]); await check(fixture.changedStdout);';assert.ok(text.includes(source));assert.ok(text.includes(end));
  return text.replace('import { mkdirSync,','import { existsSync, mkdirSync,').replace(source,'const source = (stdout: string) => `fn main() { let output=${JSON.stringify(stdout)}; print!("{}",output); let args:Vec<String>=std::env::args().collect(); if args.get(1).map(String::as_str)==Some("--hold") { std::fs::write(&args[2],output).unwrap(); while !std::path::Path::new(&args[3]).exists() { std::thread::sleep(std::time::Duration::from_millis(10)); } } }\\n`;').replace(end,readFileSync(join(import.meta.dir,"📓️held-test.ts.md"),"utf8"));
 });
}
if(process.argv[2]==="run"){
 mkdirSync(generated,{recursive:true});
 await (await import(join(root,owner,"🧪️tests/🧊️native-renderer-outputs/🟦️.ts"))).testNativeRendererOutputs(root,generated);
 console.log("[DEBUG] actual native publication with a running child passed");
}
if(process.argv[2]==="fix"){
 edit("🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts",text=>text.replace('import { dirname, resolve, join, isAbsolute, relative }','import { basename, dirname, resolve, join, isAbsolute, relative }').replace('import { copyFile, open }','import { createHash } from "node:crypto";\nimport { copyFile, open }')+readFileSync(join(import.meta.dir,"📓️execution.ts.md"),"utf8"));
 edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts",text=>{
  const call='  await runTool(executable, [...args], cwd, signal, false, nativeRunnerEnvironment(environment));';assert.ok(text.includes(call));
  return text.replace('import { dirname, join, resolve }','import { basename, dirname, join, resolve }').replace('import { once }','import { pinExecutableArtifact } from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";\nimport { once }').replace(call,'  const workspace=resolve(cwd),cache=join(workspace,".🧬semio/🦑️repo/⚡️cache");\n  const binary=await pinExecutableArtifact(resolve(executable),join(cache,"tools/native-executables",basename(executable)),{signal,leaseDirectory:join(cache,"agents/resource-leases"),onWait:({elapsedMs})=>console.log(`[native] Waiting for executable publication (${elapsedMs}ms); Ctrl+C cancels`)});\n  await runTool(binary, [...args], cwd, signal, false, nativeRunnerEnvironment(environment));');
 });
 edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts",text=>{
  const start=text.indexOf('export async function dashboardExecutable('),end=text.indexOf('\n\n\n/**',start);assert.ok(start>0&&end>start);
  const body='export async function dashboardExecutable(packageRoot: string, workspace: string): Promise<string> {\n  const controller=new AbortController(),cancel=()=>controller.abort();\n  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);\n  try {\n    const name=process.platform==="win32"?"semio.exe":"semio",cache=join(workspace,".🧬semio/🦑️repo/⚡️cache");\n    return await pinExecutableArtifact(resolve(packageRoot,"dist/build",name),join(cache,"tools/dashboard-cli"),{signal:controller.signal,leaseDirectory:join(cache,"agents/resource-leases"),onWait:({elapsedMs})=>console.log(`[dashboard] Waiting for executable publication (${elapsedMs}ms); Ctrl+C cancels`)});\n  } finally {process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}\n}';
  return (text.slice(0,start)+body+text.slice(end)).replace('import { createHash } from "node:crypto";\n','').replace('import { readFile, mkdir }','import { mkdir }').replace('import { stageArtifacts }','import { pinExecutableArtifact }').replace(/^import \{ acquireResourceLease \}.*\r?\n/m,'');
 });
}

if(process.argv[2]==="owner"){
 for(const path of ["🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-publication/📓️execution.ts.md"])edit(path,text=>text.replace('cacheDirectory: string, options: ArtifactPublicationOptions','cacheDirectory: string, owner: string, options: ArtifactPublicationOptions').replace('stageArtifacts(directory,"immutable-executable",','stageArtifacts(directory,owner,'));
 edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts",text=>text.replace('basename(executable)),{signal,','basename(executable)),"native-execution",{signal,'));
 edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts",text=>text.replace('join(cache,"tools/dashboard-cli"),{signal:','join(cache,"tools/dashboard-cli"),"dashboard-execution",{signal:'));
}

if(process.argv[2]==="hosts")edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts",text=>{
 const acquire='  const workspace=resolve(cwd),cache=join(workspace,".🧬semio/🦑️repo/⚡️cache");\n  const binary=await pinExecutableArtifact(resolve(executable),join(cache,"tools/native-executables",basename(executable)),"native-execution",{signal,leaseDirectory:join(cache,"agents/resource-leases"),onWait:({elapsedMs})=>console.log(`[native] Waiting for executable publication (${elapsedMs}ms); Ctrl+C cancels`)});',anchor='/** 🏃️ Keeps the owning JavaScript event loop responsive while the native child runs. */',mcp='try {await runTool(binary,[transport,...args],this.repoRoot,controller.signal,false,process.env);}';assert.ok(text.includes(acquire)&&text.includes(anchor)&&text.includes(mcp));
 const helper='/** 🧊️ Acquires the selected native host without locking its mutable producer output. */\nasync function nativeExecutable(executable: string, cwd: string, signal: AbortSignal): Promise<string> {\n  const workspace=resolve(cwd),cache=join(workspace,".🧬semio/🦑️repo/⚡️cache");\n  return pinExecutableArtifact(resolve(executable),join(cache,"tools/native-executables",basename(executable)),"native-execution",{signal,leaseDirectory:join(cache,"agents/resource-leases"),onWait:({elapsedMs})=>console.log(`[native] Waiting for executable publication (${elapsedMs}ms); Ctrl+C cancels`)});\n}\n\n';
 return text.replace(acquire,'  const binary=await nativeExecutable(executable,cwd,signal);').replace(anchor,helper+anchor).replace(mcp,'try {await runTool(await nativeExecutable(binary,this.repoRoot,controller.signal),[transport,...args],this.repoRoot,controller.signal,false,process.env);}');
});

if(process.argv[2]==="runtime")await (await import(join(root,owner,"🧪️tests/🧊️native-runtime/🟦️.ts"))).testNativeRuntime(root,generated);
if(process.argv[2]==="runtime-contract")edit(`${owner}/🧪️tests/🧊️native-runtime/🟦️.ts`,text=>{const before='assert.equal(target.continuous, operation === "run");';assert.ok(text.includes(before));return text.replace(before,'assert.equal(target.continuous, false);');});

if(process.argv[2]==="runtime-inputs")edit(`${owner}/🧪️tests/🧊️native-runtime/🟦️.ts`,text=>text.replace('role = "plugin"\\n[[package.metadata.semio.playground]]','component-kind = "plugin"\\n[[package.metadata.semio.playground]]').replace('const { cacheInternals } = await import("../../../🟨️.mjs");','const { cacheInternals, libraryBootstrap } = await import("../../../🟨️.mjs");\n  await libraryBootstrap;'));
if(process.argv[2]==="lookup")edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts",text=>text.replace('import { basename, dirname, join, resolve }','import { basename, dirname, isAbsolute, join, resolve }').replace('async function nativeExecutable(executable: string, cwd: string, signal: AbortSignal): Promise<string> {','async function nativeExecutable(executable: string, cwd: string, signal: AbortSignal): Promise<string> {\n  if(!isAbsolute(executable))return executable;'));

if(process.argv[2]==="consumer-proof")edit(`${owner}/🧪️tests/🧊️native-runtime/🟦️.ts`,text=>{
 const anchor='    assert.equal(createHash("sha256").update(bytes).digest("hex"), descriptor.hashes.wasmSha256);\n  }';assert.ok(text.includes(anchor));return text.replace(anchor,anchor.replace('\n  }','\n    console.log("[DEBUG] Native consumer fetched its live asset");\n  }'));
});

if(process.argv[2]==="channel-test"){
 const test="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs";
 for(const path of [test,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-entrypoint/📓️smoke-test.rs.md"])edit(path,text=>{const anchor='    let fixture=&contract["emptySmoke"];';assert.ok(text.includes(anchor));return text.replace(anchor,anchor+'\n    let registry:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/📇️registry/🧬️schema/🔣️.json")).expect("current descriptor contract");\n    assert_eq!(fixture["descriptor"]["executionProtocol"]["appChannelVersion"],registry["$defs"]["CatalogDescriptorV1"]["properties"]["executionProtocol"]["properties"]["appChannelVersion"]["const"],"the smoke input uses the current host channel");');});
}
if(process.argv[2]==="channel-fix")edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧫️fixtures/🚪️headless/🔣️.json",text=>{const data=JSON.parse(text),schema=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json"),"utf8"));data.emptySmoke.descriptor.executionProtocol.appChannelVersion=schema.$defs.CatalogDescriptorV1.properties.executionProtocol.properties.appChannelVersion.const;return JSON.stringify(data,null,2)+"\n";});
