import { strict as assert } from "node:assert";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const base=resolve(process.argv[2],"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight");
const {buildPreflightProblemsV1,readBuildPreflightPolicyV1}=await import(pathToFileURL(resolve(base,"🟦️.ts")).href);
const fixture=await Bun.file(resolve(base,"🧫️fixtures/🔣️.json")).json();
for(const row of fixture.windows){const actual=buildPreflightProblemsV1(readBuildPreflightPolicyV1().platforms.win32,row.facts);console.log("[DEBUG] "+row.id+" "+JSON.stringify(actual));assert.deepEqual(actual,row.problems,row.id);}
const {probeBuildPreflightCaseV1}=await import(pathToFileURL(resolve(base,"🟦️.ts")).href);
const {nativeDirectoryProtocolV1}=await import(pathToFileURL(resolve(process.argv[2],"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts")).href);
const protocol=nativeDirectoryProtocolV1();
for(const row of fixture.probes){const message=row.probe==="near-match"?protocol.refusals.caseAccess+" ":protocol.refusals[row.probe]??row.probe,error=row.probe==="foreign"?{message:protocol.refusals.caseAccess}:row.probe==="cancelled"?new DOMException("cancelled","AbortError"):new Error(message),probe=()=>{if(row.probe==="sensitive")return true;if(row.probe==="insensitive")return false;throw error;};let result,caught;try{result=probeBuildPreflightCaseV1(row.platform,row.kind,probe);}catch(failure){caught=failure;}if(row.expected==="refused")assert.equal(caught,error,row.id);else{assert.equal(caught,undefined,row.id);assert.equal(result,row.expected,row.id);}console.log("[DEBUG] probe "+row.id+" "+row.expected);}
for(const platform of["linux","darwin"])assert.throws(()=>buildPreflightProblemsV1(readBuildPreflightPolicyV1().platforms[platform],{...fixture.windows[0].facts,platform,caseSensitive:"fold-safe"}),/Closed build preflight facts required/);
