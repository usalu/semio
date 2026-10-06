import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import assert from "node:assert/strict";
const pairs: {path:string;before:string|null;after:string}[]=JSON.parse(readFileSync(join(import.meta.dir,"../held-provider-pairs.json"),"utf8"));
assert.equal(pairs.length,4);
for(const pair of pairs)assert.equal(existsSync(pair.path)?readFileSync(pair.path,"utf8"):null,pair.before,"Concurrent Curation provider change: "+pair.path);
for(const pair of pairs){mkdirSync(dirname(pair.path),{recursive:true});writeFileSync(pair.path,pair.after);}
for(const pair of pairs)assert.equal(readFileSync(pair.path,"utf8"),pair.after);
console.log("[DEBUG] Curation coherent semantic cells and actual backing provider mounted paths="+pairs.length);
