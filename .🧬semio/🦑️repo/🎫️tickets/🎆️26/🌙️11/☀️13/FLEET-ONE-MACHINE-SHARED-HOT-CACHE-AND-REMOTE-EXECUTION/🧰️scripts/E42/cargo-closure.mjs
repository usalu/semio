import{existsSync}from"node:fs";import{join}from"node:path";
const W="/Users/ueli/Documents/semio",B=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const inf=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await inf.libraryBootstrap;
const nm=await import(B+"/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs");
const registry=JSON.parse(inf.readBuildOwnerRegistrySourceV1(W));
const owners=registry.owners.filter(o=>existsSync(join(W,o.root,"Cargo.toml")));
const N=Number(process.env.N??40),pick=owners.filter((_,i)=>i%Math.max(1,Math.floor(owners.length/N))===0).slice(0,N);
const time=f=>{const t=performance.now(),c=process.cpuUsage(),r=f(),u=process.cpuUsage(c);return{r,wall:performance.now()-t,cpu:(u.user+u.system)/1000};};
let shared=[],cold=[];
const cache=new Map(),closures=new Map();
for(const o of pick){const x=time(()=>nm.nativeDependencyRoots(o.root,W,true,cache,closures));shared.push({root:o.root.slice(-40),n:x.r.length,cpu:x.cpu,wall:x.wall});}
const digest=rows=>Bun.hash(JSON.stringify(rows)).toString(16);
const sharedDigest=digest(pick.map(o=>nm.nativeDependencyRoots(o.root,W,true,cache,closures)));
for(const o of pick){const x=time(()=>nm.nativeDependencyRoots(o.root,W,true,new Map(),new Map()));cold.push({n:x.r.length,cpu:x.cpu,wall:x.wall});}
const sum=a=>a.reduce((s,x)=>s+x.cpu,0);
console.log(JSON.stringify({owners:owners.length,sampled:pick.length,sharedCpuMs:Math.round(sum(shared)),coldEachCpuMs:Math.round(sum(cold)),firstShared:shared.slice(0,3),medianColdMs:cold.map(x=>x.cpu).sort((a,b)=>a-b)[cold.length>>1],medianSharedMs:shared.map(x=>x.cpu).sort((a,b)=>a-b)[shared.length>>1],maxMembers:Math.max(...shared.map(x=>x.n)),sharedDigest}));
