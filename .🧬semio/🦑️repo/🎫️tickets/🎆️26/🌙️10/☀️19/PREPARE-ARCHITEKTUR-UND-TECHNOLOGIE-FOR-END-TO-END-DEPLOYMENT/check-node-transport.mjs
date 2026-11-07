import {strict as assert} from "node:assert";
import {readFile,mkdir,mkdtemp,writeFile,rm} from "node:fs/promises";
import {join,dirname} from "node:path";
import {fileURLToPath} from "node:url";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {buildSourceClosureNodeArgumentsV1,buildSourceClosureNodeLoaderV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs";
import schema from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🧫️policy.json" with {type:"json"};

const policy=schema.definitions.Policy.const,base=Buffer.byteLength(JSON.stringify({payload:""}));
for(const delta of[-1,0,1]){const payload="x".repeat(policy.maximumSnapshotBytes-base+delta),make=()=>buildSourceClosureNodeArgumentsV1({payload},policy,128,"node");if(delta<=0)assert.equal(Buffer.byteLength(make().bootstrap),policy.maximumSnapshotBytes+delta+1);else assert.throws(make,/Bounded captured Source channel/);}
assert.throws(()=>buildSourceClosureNodeArgumentsV1({payload:""},{...policy,maximumSnapshotBytes:policy.maximumSnapshotBytes+1},128,"node"),/Bounded captured Source channel/);
const cases=[];for(const field of fixture.fields)for(const mutation of fixture.mutations){const value={...policy};if(mutation==="missing")delete value[field];else if(mutation==="mistyped")value[field]=String(value[field]);else value[field]++;cases.push(value);}cases.push({...policy,[fixture.extraField]:1});
const installed=JSON.parse(await readFile(join(process.env.LOCALAPPDATA,"Semio","repo","installed.json"),"utf8")),digest=createHash("sha256").update("{}").digest("hex");for(const value of cases){assert.throws(()=>buildSourceClosureNodeArgumentsV1({},value,128,"node"));const result=spawnSync(installed.executable,["tools","exec","node","--","--input-type=module","--eval",buildSourceClosureNodeLoaderV1,"--",digest,JSON.stringify(value)],{input:"{}\n",encoding:"utf8",timeout:30000,maxBuffer:65536});assert.notEqual(result.status,0);assert.match(result.stderr,/Bounded captured Source policy required/);}
const generated=join(dirname(fileURLToPath(import.meta.url)),"🗑️generated");await mkdir(generated,{recursive:true});const area=await mkdtemp(join(generated,"node-policy-"));try{const source='export async function runCapturedBuildSourceV1(){console.log("node-policy-ok");}',sha256=createHash("sha256").update(source).digest("hex");await writeFile(join(area,"entry.mjs"),source);const prepared=buildSourceClosureNodeArgumentsV1({version:1,root:area,entry:"entry.mjs",files:[{path:"entry.mjs",sha256,base64:Buffer.from(source).toString("base64"),imports:{}}]},policy,128,"node"),result=spawnSync(installed.executable,["tools","exec","node","--",...prepared.argv],{input:prepared.bootstrap,encoding:"utf8",timeout:30000,maxBuffer:65536});assert.equal(result.status,0,result.stderr);assert.match(result.stdout,/node-policy-ok/);}finally{await rm(area,{recursive:true,force:true});}
console.log(JSON.stringify({snapshotBoundary:policy.maximumSnapshotBytes,forgedPolicies:cases.length,actualLoaderRefusals:cases.length}));
