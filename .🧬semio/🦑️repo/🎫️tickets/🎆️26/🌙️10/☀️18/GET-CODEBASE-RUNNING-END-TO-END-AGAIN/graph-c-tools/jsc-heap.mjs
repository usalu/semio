import{readFileSync}from"node:fs";
const s=JSON.parse(readFileSync(process.argv[2],"utf8"));
console.log(Object.keys(s),s.nodes.length/7,s.edges.length/4,JSON.stringify(s.edgeTypes),s.nodeClassNames?.length,JSON.stringify(s.edgeNames?.slice(0,5)));
const N=s.nodes.length/7;const size=new Float64Array(N),cls=new Int32Array(N),id=new Array(N);
for(let i=0;i<N;i++){id[i]=s.nodes[i*7];size[i]=s.nodes[i*7+1];cls[i]=s.nodes[i*7+2];}
const byCls={};for(let i=0;i<N;i++){const n=s.nodeClassNames[cls[i]];const e=byCls[n]??=[0,0];e[0]++;e[1]+=size[i];}
console.log(Object.entries(byCls).sort((a,b)=>b[1][1]-a[1][1]).slice(0,12).map(([k,v])=>k+" n="+v[0]+" MB="+(v[1]/1048576).toFixed(1)).join("\n"));
const E=s.edges.length/4,idx=new Map();for(let i=0;i<N;i++)idx.set(s.nodes[i*7],i);
const strCls=s.nodeClassNames.indexOf("string");
const refBy=new Map(),stringRefs=new Int32Array(N),inDeg=new Int32Array(N);
for(let e=0;e<E;e++){const from=idx.get(s.edges[e*4]),to=idx.get(s.edges[e*4+1]);if(to===undefined||from===undefined)continue;inDeg[to]++;if(cls[to]===strCls){stringRefs[from]++;const k=s.nodeClassNames[cls[from]]+"/"+s.edgeTypes[s.edges[e*4+2]];refBy.set(k,(refBy.get(k)??0)+1);}}
console.log("string referrers by class/edge:\n"+[...refBy].sort((a,b)=>b[1]-a[1]).slice(0,10).map(([k,v])=>k+" "+v).join("\n"));
let strNo=0;for(let i=0;i<N;i++)if(cls[i]===strCls&&inDeg[i]===0)strNo++;console.log("strings with no referrer",strNo);
const top=[...Array(N).keys()].sort((a,b)=>stringRefs[b]-stringRefs[a]).slice(0,12);
for(const i of top)console.log(stringRefs[i],s.nodeClassNames[cls[i]],"size",size[i],"label",s.labels?.[s.nodes[i*7+4]]??"");
console.log("roots",s.roots.length,JSON.stringify(s.roots.slice(0,6)));
const rt=new Map();for(let r=0;r<s.roots.length;r+=3){const k=s.roots[r+1];rt.set(k,(rt.get(k)??0)+1);}
console.log("root groups [typeIndexOrLabel,count]",JSON.stringify([...rt].sort((a,b)=>b[1]-a[1]).slice(0,8)),"labels sample",JSON.stringify([...rt.keys()].slice(0,4).map(k=>s.labels?.[k])));
const rootNodes=[];for(let r=0;r<s.roots.length;r+=3)rootNodes.push(idx.get(s.roots[r]));
const rc={};for(const i of rootNodes){if(i===undefined)continue;const k=s.nodeClassNames[cls[i]];rc[k]=(rc[k]??0)+1;}console.log("root node classes",JSON.stringify(rc));
// reverse adjacency (first referrer only, non-string) with edge name
const parent=new Int32Array(N).fill(-1),pedge=new Array(N);
for(let e=0;e<E;e++){const from=idx.get(s.edges[e*4]),to=idx.get(s.edges[e*4+1]);if(to===undefined||from===undefined||parent[to]!==-1)continue;if(cls[from]===strCls)continue;parent[to]=from;const t=s.edgeTypes[s.edges[e*4+2]],d=s.edges[e*4+3];pedge[to]=t==="Property"||t==="Variable"?s.edgeNames[d]:(t==="Index"?"["+d+"]":t);}
const chain=i=>{const out=[];for(let k=0;k<9&&i!==-1&&i!==undefined;k++){out.push(s.nodeClassNames[cls[i]]+(pedge[i]!==undefined?"."+pedge[i]:""));i=parent[i];}return out.join(" <- ");};
const seenChains=new Map();
for(const i of [...Array(N).keys()].sort((a,b)=>stringRefs[b]-stringRefs[a]).slice(0,400)){if(cls[i]===strCls)continue;const c=chain(i);seenChains.set(c,(seenChains.get(c)??0)+stringRefs[i]);}
console.log([...seenChains].sort((a,b)=>b[1]-a[1]).slice(0,10).map(([c,n])=>n+"  "+c).join("\n"));
