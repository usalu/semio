/** 🧪️ Reproduces the native watcher readiness regression without starting a daemon. */
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import assert from "node:assert/strict";
import Ajv from "ajv";
const workspace=process.cwd(),base=path.join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment"),read=file=>JSON.parse(fs.readFileSync(file,"utf8")),fixture=read(path.join(base,"🧫️fixtures/🔣️.json")),schema=read(path.join(base,"🧬️schema/🔣️.json"));
assert.equal(new Ajv({strict:true}).validate(schema,fixture),true);
const sdk=createRequire(path.join(workspace,"package.json")),helper=await import(pathToFileURL(path.join(base,"🧰️watcher-startup/🟦️.ts"))),source=fs.readFileSync(sdk.resolve("nx/src/daemon/server/server"),"utf8");
let admittedSource=source;
if(process.argv.includes("--authored-patch")){
 const owner=await import(pathToFileURL(path.join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts"))),name="dist/src/daemon/server/server.js",patch=fs.readFileSync(path.join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch"),"utf8"),section=patch.split("diff --git ").find(row=>row.startsWith("a/"+name+" b/")),current="diff --git "+section,baseline=current.slice(0,current.lastIndexOf("@@ -455,6 +480,7 @@"));
 const reverse=baseline.split("\n").map(line=>line.startsWith("@@ ")?line.replace(/^@@ -(\d+)(,\d+)? \+(\d+)(,\d+)? @@/u,(_,a,b,c,d)=>"@@ -"+c+(d??"")+" +"+a+(b??"")+" @@"):line.startsWith("--- ")?line.replace("--- a/","+++ b/"):line.startsWith("+++ ")?line.replace("+++ b/","--- a/"):line.startsWith("+")?"-"+line.slice(1):line.startsWith("-")?"+"+line.slice(1):line).join("\n");
 const original=owner.applyToolingPatchImagesV1(new Map([[name,Buffer.from(source)]]),reverse),updated=owner.applyToolingPatchImagesV1(original,current);
 admittedSource=Buffer.from(updated.get(name)).toString("utf8");
 const {spawnSync}=await import("node:child_process"),oracle=spawnSync("repo",["tools","exec","--light","python","--",path.join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🧪️tests/🐍️oracle/🐍️.py")],{input:JSON.stringify({op:"apply",rows:[{before:Buffer.from(original.get(name)).toString("utf8"),patch:current}]}),encoding:"utf8",env:process.env});
 assert.equal(oracle.status,0,oracle.stderr);assert.equal(JSON.parse(oracle.stdout)[0],admittedSource);console.log(JSON.stringify({independentPythonPatchOracle:true}));

}
const actual=await helper.probeDaemonWatcherStartupV1(admittedSource,fixture.daemonWatcherFailure);
console.log(JSON.stringify(actual));assert.deepEqual(actual,fixture.daemonWatcherFailure.expected);
