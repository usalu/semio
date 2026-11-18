import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",ce=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce";
const{createClosureStoreV1}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs");
const names=readdirSync(ce+"/e").filter(n=>n.endsWith(".json.br"));const N=Number(process.argv[2]||300),ids:string[]=[];
for(let i=0;i<names.length&&ids.length<N;i+=Math.floor(names.length/N))ids.push(JSON.parse(brotliDecompressSync(readFileSync(ce+"/e/"+names[i])).toString("utf8")).id);
const store=createClosureStoreV1({directory:ce,readOnly:true});const mb=(x:number)=>Math.round(x/1048576*10)/10;
Bun.gc(true);const h0=process.memoryUsage().heapUsed;const held:any[]=[];let t=performance.now();
for(const id of ids){held.push(await store.load(id));}
const ms=performance.now()-t;const cpu=process.cpuUsage();Bun.gc(true);const h1=process.memoryUsage().heapUsed;
let layers=0,mem=0;for(const e of held)if(e){layers+=e.digests.length;mem+=e.sidecar?.members?.reduce((s:number,l:any)=>s+(l?.length??0),0)??0;}
const uniq=new Set<any>(),uf=new Set<any>(),ud=new Set<string>();let uref=0,rowsN=0,walkStr=new Set<string>(),valItems=0;for(const e of held)if(e){for(const l of e.sidecar?.members??[])if(l&&!uniq.has(l)){uniq.add(l);uref+=l.length;for(const w of l)walkStr.add(w);}for(const f of e.sidecar?.facts??[])if(f)uf.add(f);for(const d of e.digests)ud.add(d);rowsN+=e.rows.length;valItems+=Array.isArray(e.value)?e.value.length:0;}
console.log(JSON.stringify({uniqueMemberArrays:uniq.size,uniqueMemberRefs:uref,distinctWalkStrings:walkStr.size,uniqueFacts:uf.size,distinctDigests:ud.size,rows:rowsN,valueItems:valItems,entries:held.filter(Boolean).length,loadMs:Math.round(ms),cpuMs:Math.round((cpu.user+cpu.system)/1000),heapGrowthMB:mb(h1-h0),perEntryKB:Math.round((h1-h0)/1024/held.length),layers,memberRefs:mem}));process.exit(0);
