import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const pairs: {path:string;before:string;after:string}[]=JSON.parse(readFileSync(join(import.meta.dir,"../held-provider-pairs.json"),"utf8"));assert.equal(pairs.length,1);
for(const pair of pairs)assert.equal(readFileSync(pair.path,"utf8"),pair.before,"Concurrent Curation Source owner change");
for(const pair of pairs)writeFileSync(pair.path,pair.after);
console.log("[DEBUG] Curation complete Source byte-backed indexes and complete consumed owner provider mounted paths=1");
