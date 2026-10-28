// [asm-g] cost of patchFactIndexV1 + packFactIndexV1 on the real index for a typical delta (one owner replaced, @env replaced). usage: bun patch-cost.mjs <gen-dir>
import{readFileSync}from"node:fs";import{join}from"node:path";import{heapStats}from"bun:jsc";
const D=process.argv[2],B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/",g=await import(B+"🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};const rss=()=>Math.round(process.memoryUsage().rss/1048576),live=()=>{Bun.gc(true);const h=heapStats();return Math.round((h.heapSize+h.extraMemorySize)/1048576);};
let c0=cpu(),t=performance.now();const index=g.unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
console.log("unpack ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0),"rss",rss(),"live",live());
const envId=index.owners.indexOf("@env"),ownerName=process.argv[3]??"@semio-tech/2d-booleans-rs",ownId=index.owners.indexOf(ownerName);
const rowsOf=id=>{const out=[];for(const c of index.classes)if(c.o.includes(id))for(const r of c.r)out.push(index.rows[r]);return out;};
const upserts=new Map([["@env",rowsOf(envId)],[ownerName,rowsOf(ownId)]]);console.log("upsert rows",[...upserts.values()].map(l=>l.length));
t=performance.now();c0=cpu();const patched=g.patchFactIndexV1(index,{upserts});console.log("patch ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0),"rss",rss(),"live",live(),"classes",patched.classes.length);
t=performance.now();c0=cpu();const packed=g.packFactIndexV1(patched,g.GENERATION_POLICY_V1.maximumIndexBytes);console.log("pack ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0),"packedMB",(packed.bytes.length/1048576).toFixed(1),"rss",rss());
const small=new Map([[ownerName,rowsOf(ownId).slice(0,50)]]);
t=performance.now();c0=cpu();const p2=g.patchFactIndexV1(index,{upserts:small});console.log("small patch ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0),"rss",rss());
t=performance.now();c0=cpu();const p3=g.patchFactIndexV1(index,{upserts:small});console.log("small patch again ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0));
t=performance.now();c0=cpu();for(const r of index.rows)g.rowKeyV1(r);console.log("rowKey all ms",Math.round(performance.now()-t)+" cpu "+Math.round(cpu()-c0));
