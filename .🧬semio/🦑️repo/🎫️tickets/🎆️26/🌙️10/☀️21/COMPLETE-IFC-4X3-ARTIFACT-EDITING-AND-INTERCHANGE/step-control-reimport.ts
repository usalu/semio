/** 🔁️ Compare independently emitted STEP with the admitted owned graph. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";
const ticket=import.meta.dir,root=resolve(ticket,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"),output=resolve(ticket,"🗑️generated/stepctl");
const {parseIfc4x3Step}=await import(pathToFileURL(resolve(base,"🚪️io/step.ts")).href),cases=await Bun.file(resolve(output,"cases.json")).json();
for(const row of cases){const own=await parseIfc4x3Step(await Bun.file(resolve(output,row.name+".owned.ifc")).text(),{batchSize:4096}),external=await parseIfc4x3Step(await Bun.file(resolve(output,row.name+".independent.ifc")).text(),{batchSize:4096});assert.deepEqual(external,own);}
console.log(JSON.stringify({models:cases.length,reimportMismatches:0}));
