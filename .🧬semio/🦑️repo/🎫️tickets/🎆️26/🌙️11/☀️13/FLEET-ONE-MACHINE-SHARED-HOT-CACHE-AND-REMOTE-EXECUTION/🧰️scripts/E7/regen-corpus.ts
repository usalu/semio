import {readFileSync,writeFileSync} from "node:fs";
import {readHostCapacityPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {sqliteHostOracleV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔮️sqlite/🟦️.ts";
const path="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json";
const raw=readFileSync(path,"utf8"),f=JSON.parse(raw),policy=readHostCapacityPolicyV1(f.policy);
let changed=0;
for(const key of ["cases","corpus","rampCorpus"])for(const item of f[key]){const e=sqliteHostOracleV1(policy,item);if(JSON.stringify(e)!==JSON.stringify(item.expect)){item.expect=e;changed++;}}
console.log("changed",changed,"indent",raw.slice(0,30).replace(/\n/g,"\\n"));
const out:Record<string,unknown>={};for(const key of ["cases","corpus","rampCorpus"])for(const item of f[key])out[item.name]=item.expect;writeFileSync(process.argv[2]!,JSON.stringify(out));
