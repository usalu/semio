#!/usr/bin/env bun
// Fills the durable closure-verification memo (generations/a) in parallel to the planners.
// Works from the end of the question list, the planner works from the start, so they meet in the middle.
// usage: closure-verify-helper.mjs <shard> <shards> [maxRssMegabytes=1400]
// Run through: repo --service=off tools exec --light bun -- <this file> 0 3
import fs from"node:fs";import{join}from"node:path";
const root="/home/deck/semio",G=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph";
const[shard=0,shards=1,maxRss=1400]=process.argv.slice(2).map(Number);
const generations=await import(G+"/🕰️generations/🟨️.mjs"),durable=await import(G+"/🕰️generations/✂️cutoff/💾️durable/🟨️.mjs"),producer=await import(G+"/📥️inference/🟨️.mjs");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const base=root+"/.cache/b/g/m/generations",pointer=JSON.parse(fs.readFileSync(base+"/current.json","utf8")),manifest=JSON.parse(fs.readFileSync(base+"/m/"+pointer.manifest+".json","utf8"));
const index=generations.openPackedFactIndexV1(fs.readFileSync(base+"/x/"+manifest.factIndex+".facts.br"),generations.GENERATION_POLICY_V1.maximumIndexBytes);
const questions=[];for(let id=0;id<index.rows.length;id++)if(index.rowKindOf(id)==="derived"){const row=index.rows[id];questions.push({kind:row.kind,path:row.path,key:generations.canonicalJsonV1([row.kind,row.path])});}
const mine=questions.filter((_,i)=>i%shards===shard).reverse();
const files=generations.createNodeFilePortV1(),lease=generations.createFileLeasePortV1(root+"/.cache/b/g/m/generations/l-helper"),memo=durable.createDurableAnswersV1({root:base,fs:files,lease,resource:"helper"});
const registry=JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)),derive=await producer.openGenerationDeriverV1({workspaceRoot:root,registry});
let done=0,skipped=0;const started=Date.now();
for(const q of mine){
 if(await memo.get(q.key,true)){skipped++;continue;}
 try{const answer=await derive({kind:q.kind,path:q.path,fact:null});if(answer.rows.length)await memo.set(q.key,{fact:answer.fact,rows:[...answer.rows]});done++;}catch(error){console.error("[helper] failed",q.key.slice(0,80),String(error).slice(0,120));}
 if((done+skipped)%25===0)console.log(`[helper] shard ${shard}/${shards}: ${done} derived, ${skipped} present of ${mine.length}, ${Math.round(done*60000/(Date.now()-started))}/min, rss ${process.memoryUsage().rss>>20} MB`);
 if(process.memoryUsage().rss>>20>maxRss){console.log("[helper] rss limit, exiting; rerun to continue");process.exit(0);}
}
console.log(`[helper] shard ${shard} finished: ${done} derived, ${skipped} present`);
