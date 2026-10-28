// Rewrites the named input `schedulingSourceExclusions` of the build policy and nx.json from the scheduling table (idempotent).
import {readFileSync,writeFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio/";
const {buildSchedulingExclusionPatternsV1,buildSchedulingExclusionsInputV1}=await import(root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts");
const expected=[...buildSchedulingExclusionPatternsV1()];
for(const file of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","nx.json"]){
 const raw=readFileSync(root+file,"utf8"),json=JSON.parse(raw);
 const indent=raw.startsWith('{\n  "')?2:raw.startsWith('{\n "')?1:raw.startsWith('{\n\t')?"\t":2;
 const current=json.namedInputs[buildSchedulingExclusionsInputV1] as string[];
 console.log(file,"indent",JSON.stringify(indent),"current",current.length,"expected",expected.length,"same",JSON.stringify(current)===JSON.stringify(expected));
 if(JSON.stringify(current)===JSON.stringify(expected))continue;
 json.namedInputs[buildSchedulingExclusionsInputV1]=expected;
 const out=JSON.stringify(json,null,indent)+"\n";
 if(JSON.stringify(JSON.parse(raw),null,indent)+"\n"!==raw)throw Error("round trip differs for "+file);
 writeFileSync(root+file,out);
}
