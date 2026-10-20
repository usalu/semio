/** Replays the cache-verify scenario list against the Bazel probe in an ASCII scratch tree under /private/tmp/bzl-default (fixtures with and without non-ASCII names). One Bazel batch run at a time. */
import {chmodSync,existsSync,lstatSync,mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
import assert from "node:assert/strict";
const [repo,staged,variant,only]=process.argv.slice(2);
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const {bazelCacheProbeV1}=await import(join(staged!,B,"🔧️tools/🏗️bazel/🔁️probe/🟦️.ts"));
const ts=createRequire(join(repo!,"package.json"))("typescript");
const orchestration=readFileSync(join(repo!,L,"⚡️caching/🔁️verification/📋️orchestration/🟦️.ts"),"utf8"),tree=ts.createSourceFile("o.ts",orchestration,ts.ScriptTarget.Latest,true);
let script="";const visit=(node:any)=>{if(ts.isCallExpression(node)&&node.expression.getText()==="put"&&node.arguments[0]&&ts.isStringLiteral(node.arguments[0])&&node.arguments[0].text==="📜️script.ts")script=node.arguments[1].text;ts.forEachChild(node,visit);};visit(tree);
if(!script)throw Error("probe script not found");
const specification=JSON.parse(readFileSync(join(repo!,L,"⚡️caching/🔁️verification/🔣️probe.json"),"utf8"));
const modules=join(repo!,".cache/b/d/m/bazel/modules"),identity=readFileSync(join(modules,require("node:fs").readdirSync(modules)[0]!,"ready.json"),"utf8");
const directory=join(modules,require("node:fs").readdirSync(modules)[0]!);
const bazelBin=join(repo!,".cache/b/t/m/bazel/bin"),executable=join(bazelBin,require("node:fs").readdirSync(bazelBin)[0]!,"bazel");
const tool=bazelCacheProbeV1({version:"8.4.2",executable,installBase:join(repo!,".cache/b/t/m/bazel/install/8.4.2/darwin-arm64"),module:'module(name = "semio_tasks", version = "0.0.0")\n',lock:readFileSync(join(directory,"workspace/MODULE.bazel.lock"),"utf8"),repository:join(directory,"repositories")});
const root="/private/tmp/bzl-default/fx",fixture=join(root,variant==="unicode"?"fixture-🧪-ünï":"fixture");
rmSync(root,{recursive:true,force:true});mkdirSync(fixture,{recursive:true});
const put=(path:string,data:string|object)=>writeFileSync(join(fixture,path),typeof data==="string"?data:JSON.stringify(data,null,2));
for(const[path,data]of Object.entries(tool.files(specification)))put(path,data as any);
put("source.json",{value:42});put("toolchain.json",{version:1});put("spec.json",{case:1});put("unrelated.json",{ignored:1});put("validation.json",{valid:true});put("📜️script.ts",script);
const count=(task:string)=>existsSync(join(fixture,"state",task))?Number(readFileSync(join(fixture,"state",task),"utf8")):0;
const log:string[]=[];
const run=(scenario:string,task="build",mode="one",success=true)=>{
 const started=performance.now(),invocation=tool.invocation(specification,task,fixture,{...process.env,CACHE_PROBE_MODE:mode}),child=Bun.spawnSync([invocation.command,...invocation.arguments],{cwd:fixture,env:invocation.environment as any,stdout:"pipe",stderr:"pipe"});
 if((child.exitCode===0)!==success){console.error(child.stdout.toString()+child.stderr.toString());throw Error(scenario+": exit "+child.exitCode);}
 log.push(`${scenario}: exit=${child.exitCode} builds=${count("build")} tests=${count("test")} ${Math.round(performance.now()-started)}ms`);console.error(log.at(-1));
};
run("cold");assert.equal(count("build"),1);
run("warm");assert.equal(count("build"),1);
const expected=readFileSync(join(fixture,"dist/consumer.cjs"));rmSync(join(fixture,"dist"),{recursive:true});
run("restore");assert.equal(count("build"),1);assert.deepEqual(readFileSync(join(fixture,"dist/consumer.cjs")),expected);assert.equal(lstatSync(join(fixture,"dist/consumer.cjs")).mode&0o111,0o111);
const consumer=Bun.spawnSync([join(fixture,"dist/consumer.cjs")],{stdout:"pipe"});assert.equal(consumer.exitCode,0);assert.equal(consumer.stdout.toString().trim(),"42");
if(only==="short"){console.log(JSON.stringify({ok:true,scenarios:log.length,log}));process.exit(0);}
run("test-cold","test");assert.equal(count("test"),1);
put("spec.json",{case:2});run("test-only-build");assert.equal(count("build"),1);run("test-only-test","test");assert.equal(count("test"),2);
put("unrelated.json",{ignored:2});run("unrelated");assert.equal(count("build"),1);
put("source.json",{value:43});run("source-change");assert.equal(count("build"),2);
run("environment-change","build","two");assert.equal(count("build"),3);
put("toolchain.json",{version:2});run("toolchain-change","build","two");assert.equal(count("build"),4);
run("guarded-cold","guarded","two");assert.equal(count("guarded"),1);assert.equal(count("validate"),1);
run("guarded-warm","guarded","two");assert.equal(count("guarded"),1);assert.equal(count("validate"),2);
put("validation.json",{valid:false});run("guarded-rejected","guarded","two",false);assert.equal(count("guarded"),1);assert.equal(count("validate"),3);
console.log(JSON.stringify({ok:true,scenarios:log.length,log}));
