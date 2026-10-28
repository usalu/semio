// [gram-g] old patch rule vs staged rule on the real index: same consumers per owner, memory and time. usage: bun patch-compare.mjs <gen-dir> <variant.mjs> [owner]
import{readFileSync}from"node:fs";import{join}from"node:path";import{heapStats}from"bun:jsc";
const D=process.argv[2],B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/",old=await import(B+"🟨️.mjs"),neu=await import(process.argv[3]);
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const bytes=readFileSync(join(D,"x",manifest.factIndex+".facts.br"));
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
const index=old.unpackFactIndexV1(bytes,old.GENERATION_POLICY_V1.maximumIndexBytes);
const ownerName=process.argv[4]??"@semio-tech/2d-booleans-rs",ownId=index.owners.indexOf(ownerName),envId=index.owners.indexOf("@env");
const rowsOf=id=>{const out=[];for(const c of index.classes)if(c.o.includes(id))for(const r of c.r)out.push(index.rows[r]);return out;};
const upserts=new Map([["@env",rowsOf(envId)],[ownerName,rowsOf(ownId).slice(0,20000)]]);
const consumers=ix=>{const per=new Map();for(const c of ix.classes)for(const o of c.o){const set=per.get(ix.owners[o])??new Set();for(const r of c.r)set.add(old.rowKeyV1(ix.rows[r]));per.set(ix.owners[o],set);}return per;};
const run=(mod,label)=>{Bun.gc(true);const r0=rss(),t=performance.now(),c=process.cpuUsage();const out=mod.patchFactIndexV1(index,{upserts});const cpu=process.cpuUsage(c);console.log(label,"ms",Math.round(performance.now()-t),"cpu",Math.round((cpu.user+cpu.system)/1000),"rss",r0,"->",rss(),"classes",out.classes.length,"rows",out.rows.length);return out;};
let a,b;if(process.env.ORDER==="staged-first"){b=run(neu,"staged");a=run(old,"old");}else{a=run(old,"old");b=run(neu,"staged");}
const ca=consumers(a),cb=consumers(b);let same=ca.size===cb.size;for(const [o,s] of ca){const t=cb.get(o);if(!t||t.size!==s.size){same=false;break;}for(const k of s)if(!t.has(k)){same=false;break;}if(!same)break;}
console.log("same consumers per owner:",same,"owners",ca.size);
const pa=old.packFactIndexV1(a,old.GENERATION_POLICY_V1.maximumIndexBytes),pb=old.packFactIndexV1(b,old.GENERATION_POLICY_V1.maximumIndexBytes);
console.log("packed MB old",(pa.bytes.length/1048576).toFixed(2),"staged",(pb.bytes.length/1048576).toFixed(2));
