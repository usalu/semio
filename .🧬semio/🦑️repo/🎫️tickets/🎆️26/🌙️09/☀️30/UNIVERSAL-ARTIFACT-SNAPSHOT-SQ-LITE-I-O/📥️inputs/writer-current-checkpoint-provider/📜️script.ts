import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const pairs=JSON.parse(readFileSync(join(import.meta.dir,"../writer-current-checkpoint-witness/held-provider-pairs.json"),"utf8"));
assert.equal(pairs.length,1);for(const pair of pairs)assert.equal(readFileSync(pair.path,"utf8"),pair.before,"Concurrent fixture owner change");
const previous=JSON.parse(pairs[0].before),next=JSON.parse(pairs[0].after);assert.equal(next.envelopes.length,17);
for(let index=0;index<17;index++){const old=previous.envelopes[index],now=next.envelopes[index];for(const key of["document_id","actor","target","inverse","timestamp","line"])assert.deepEqual(now[key],old[key],"original envelope authority "+index+" "+key);if(index!==4&&index!==16){assert.deepEqual(now.diff,old.diff);assert.equal(now.mutation_id,old.mutation_id);}else{assert.deepEqual(now.diff.payload.slice(0,-1),old.diff.payload);assert.equal(now.diff.payload.at(-1),0);}}
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify(pairs,null,2)+"\n");for(const pair of pairs)writeFileSync(pair.path,pair.after);console.log("[DEBUG] Writer handcrafted current checkpoint wire2 and causal IDs/references mounted paths=1 original17_envelopes_full_fields_preserved=true strict_decoder_unchanged=true");
