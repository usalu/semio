/** 🔂️ Read-only: for a sealed memo record, answers whether the slice its pin covers was already the same slice in another retained generation (`bun reseal.ts <root> <record.json> <other-manifest>`): the carry rule over the two manifests, i.e. whether a light record naming that other generation should have let the child seal. Nothing is written. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,recordFile,other]=process.argv.slice(2) as [string,string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const {graphMemoSliceSameV1}=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🧳️carry/🟦️.ts")),coverage=await import(join(L,"🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs"));
const record=JSON.parse(readFileSync(recordFile,"utf8")),store=join(root,".cache/b/g/m/generations/m"),now=JSON.parse(readFileSync(join(store,record.manifest+".json"),"utf8")),then=JSON.parse(readFileSync(join(store,other+".json"),"utf8"));
const covered=coverage.decodeVerifiedScopeV1(Object.keys(now.owners).sort((a:string,b:string)=>a<b?-1:a>b?1:0),JSON.parse(record.pin).covered);
console.log(JSON.stringify({record:record.key.slice(0,12),projects:record.projects,sealed:now.generation,other:then.generation,covered:covered.size,rule:graphMemoSliceSameV1(then,now,covered),dirtyInNewer:now.built?.dirty?.length}));
