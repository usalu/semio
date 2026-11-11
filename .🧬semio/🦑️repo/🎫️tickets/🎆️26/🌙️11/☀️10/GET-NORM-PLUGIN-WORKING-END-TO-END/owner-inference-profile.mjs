import {createRequire} from "node:module";
import {execFileSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {heapStats} from "bun:jsc";
const root=process.cwd(),I=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const index=await import(I+"🟨️.profile-tmp.mjs"),observation=await import(B+"👁️observation/🟨️.mjs"),{withClosurePassV1}=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");

await index.libraryBootstrap;
const nodes=index.profileNodesV1;
const registry=JSON.parse(readFileSync(root+"/📋️build-owners.json","utf8"));
const files=execFileSync("git",["ls-files","-z"],{cwd:root,maxBuffer:1<<30}).toString().split("\0").filter(f=>f&&(f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||f.endsWith(".patch")));
const treeV1=()=>{{Bun.gc(true);const g=Bun.generateHeapSnapshot(),N=g.nodes.length/4,first=new Int32Array(N+1),E=g.edges.length/4;
const order=[];for(let e=0;e<E;e++)first[g.edges[e*4]+1]++;
// edges are sorted by from-node ordinal? build CSR by counting
const start=new Int32Array(N+1);for(let e=0;e<E;e++)start[g.edges[e*4]+1]++;for(let i=0;i<N;i++)start[i+1]+=start[i];const fill=start.slice(0,N),adj=new Int32Array(E);for(let e=0;e<E;e++)adj[fill[g.edges[e*4]]++]=e;
const parent=new Int32Array(N).fill(-1),pedge=new Int32Array(N).fill(-1),seen=new Uint8Array(N),queue=new Int32Array(N);let qh=0,qt=0;queue[qt++]=0;seen[0]=1;
while(qh<qt){const u=queue[qh++];for(let k=start[u];k<start[u+1];k++){const e=adj[k],v=g.edges[e*4+1];if(!seen[v]){seen[v]=1;parent[v]=u;pedge[v]=e;queue[qt++]=v;}}}
const sub=new Float64Array(N),cnt=new Int32Array(N);for(let i=qt-1;i>0;i--){const u=queue[i];sub[u]+=g.nodes[u*4+1];cnt[u]+=1;const p=parent[u];sub[p]+=sub[u];cnt[p]+=cnt[u];}
const label=u=>{const e=pedge[u];return g.nodeClassNames[g.nodes[u*4+2]]+"#"+u+"<-"+(e<0?"":(g.edges[e*4+2]===1?g.edgeNames[g.edges[e*4+3]]:g.edgeTypes[g.edges[e*4+2]]+g.edges[e*4+3]))};
const pathOf=u=>{const out=[];for(let d=0;d<14&&u>0;d++){out.push(label(u));u=parent[u];}return out.join(" < ")};
const top=[...Array(N).keys()].filter(u=>seen[u]).sort((a,b)=>sub[b]-sub[a]).slice(0,40);
console.log("TREE top subtrees");for(const u of top)console.log(Math.round(sub[u]/1e6)+"MB",cnt[u],pathOf(u).slice(0,700));}
};
const {boundedCacheStatsV1}=await import(B+"🧊️bounded/🟨️.mjs");const bounded=()=>console.log("BOUNDED",JSON.stringify(boundedCacheStatsV1().filter(c=>c.entries>0).map(c=>[c.name.slice(0,50),c.entries,c.bytes])));
const roots=readFileSync(process.env.ROOTS,"utf8").split("\n").filter(Boolean),sample=label=>{Bun.gc(true);const h=heapStats(),m=process.memoryUsage();console.log(label,JSON.stringify(observation.inferenceCacheSizesV1()),"rssMB",Math.round(m.rss/1e6),"peakRssMB",Math.round(process.resourceUsage().maxRSS/1e3),"heapMB",Math.round(h.heapSize/1e6),"objects",h.objectCount,"extMB",Math.round(h.extraMemorySize/1e6));};
const {withBuildGraphStagingV1}=await import(B+"📥️inputs/💾️storage/🟨️.mjs");const staging="/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad/staging";sample("start");
let done=0;const t0=performance.now();
await withBuildGraphStagingV1({root:staging},()=>withClosurePassV1(()=>observation.captureObservedSourceInputsV1(()=>observation.observeInferenceSnapshotV1(()=>nodes(files,{analyzeLockfile:false,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots:new Set(roots),onOwnerObservations:(n,r)=>{done++;if(process.env.LIMIT&&done===Number(process.env.LIMIT)){if(process.env.SNAP){Bun.gc(true);Bun.write(process.env.SNAP,Bun.generateHeapSnapshot("v8"));}else treeV1();setTimeout(()=>process.exit(0),20000);}if(done%Number(process.env.EVERY??5)===0){console.log("owner",done,"rows",r.length,"s",Math.round((performance.now()-t0)/1000));sample("  ");}}},{workspaceRoot:root,registry,preparationControl:{advance:async()=>{}}})),undefined,false)));
sample("end");bounded();const SI=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");SI.resetClosureEntriesV1();sample("after reset closure entries");observation.releaseInferenceMemoryV1();sample("after release inference memory");const {shrinkBoundedCachesV1}=await import(B+"🧊️bounded/🟨️.mjs");shrinkBoundedCachesV1(0);sample("after shrink bounded");if(process.env.HISTOGRAM){Bun.gc(true);const g=Bun.generateHeapSnapshot(),c=new Map(),z=new Map();for(let i=0;i<g.nodes.length;i+=4){const k=g.nodeClassNames[g.nodes[i+2]];c.set(k,(c.get(k)??0)+1);z.set(k,(z.get(k)??0)+g.nodes[i+1]);}console.log("HISTOGRAM by count");for(const[k,v]of[...c].sort((a,b)=>b[1]-a[1]).slice(0,20))console.log(v,Math.round(z.get(k)/1e6)+"MB",k);const r=new Map(),cls=i=>g.nodeClassNames[g.nodes[i*4+2]];for(let e=0;e<g.edges.length;e+=4){}
{const per=new Map();let from=-1;
 /* edges are grouped per node in node order; edge count not exposed, so walk via from in edges[e] */
 for(let e=0;e<g.edges.length;e+=4){const f=g.edges[e],t=g.edges[e+1];if(cls(t)!=="string")continue;const k=cls(f)+" ."+(g.edges[e+2]===1?g.edgeNames[g.edges[e+3]]:g.edgeTypes[g.edges[e+2]]);per.set(k,(per.get(k)??0)+1);}
 console.log("STRING RETAINERS");for(const[k,v]of[...per].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(v,k);}
console.log("HISTOGRAM by size");for(const[k,v]of[...z].sort((a,b)=>b[1]-a[1]).slice(0,12))console.log(Math.round(v/1e6)+"MB",c.get(k),k);}process.exit(0);
