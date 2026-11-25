/** 🔬️ [DEBUG] probe: crate closure digests of the stored graph against the cold identities, before and after one adopt cargo pass. */
import{readFileSync}from"node:fs";import{deserialize}from"node:v8";import{dirname}from"node:path";
const R="/Users/ueli/Documents/semio",CG=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🌳️closure-graph/";
const inference=await import(R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const cg=await import(CG+"🟨️.mjs"),cargo=await import(CG+"🦀️cargo/🟨️.mjs");
const {SP,COLD}=process.env,cold=JSON.parse(readFileSync(COLD,"utf8")).identities,reg=deserialize(readFileSync(SP+"/wave/session.bin")).registry.owners.filter(o=>String(o.manifest).endsWith("Cargo.toml"));
const graph=await cg.loadClosureGraphV1(SP+"/store"),off=1607;
const ids=reg.map(o=>graph.closureDigest([cargo.cargoOwnerRootV1(R,dirname(o.manifest))]));
console.log("[DEBUG] stored vs cold",ids.filter((d,i)=>d===cold[off+i]).length,"of",ids.length);
const bad=ids.findIndex((d,i)=>d!==cold[off+i]);console.log("[DEBUG] first bad",bad,reg[bad]?.manifest);
const memo=new Map();for(const r of graph.records())if(r.kind!=="module"&&r.kind!=="resource"&&r.kind!=="missing"&&r.kind!=="skip"&&!cargo.cargoNodeStampHoldsV1(R,r,memo))console.log("[DEBUG] untrusted",r.kind,r.name.slice(-90));
const untrusted=new Set([...graph.records()].filter(r=>r.kind!=="module"&&r.kind!=="resource"&&r.kind!=="missing"&&r.kind!=="skip"&&!cargo.cargoNodeStampHoldsV1(R,r,new Map())).map(r=>r.key));
const reaches=reg.map(o=>graph.reach([cargo.cargoOwnerRootV1(R,dirname(o.manifest))]).keys.some(k=>untrusted.has(k)));
const nodes=new Map(),facts=new Map();for(const o of reg){try{cargo.cargoCrateNodesV1(R,dirname(o.manifest),{nodes,facts,held:k=>untrusted.has(k)?undefined:graph.get(k)});}catch{}}graph.update(nodes.values());
const after=reg.map(o=>graph.closureDigest([cargo.cargoOwnerRootV1(R,dirname(o.manifest))])),moved=after.map((d,i)=>d!==cold[off+i]);
console.log("[DEBUG] moved",moved.filter(Boolean).length,"reaching",reaches.filter(Boolean).length,"moved without reaching",moved.filter((m,i)=>m&&!reaches[i]).length,"reaching but unmoved",moved.filter((m,i)=>!m&&reaches[i]).length);
process.exit(0);
