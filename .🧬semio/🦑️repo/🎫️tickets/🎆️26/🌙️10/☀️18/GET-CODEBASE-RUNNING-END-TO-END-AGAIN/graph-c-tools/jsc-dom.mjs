import{readFileSync}from"node:fs";
const s=JSON.parse(readFileSync(process.argv[2],"utf8"));
const N=s.nodes.length/7,E=s.edges.length/4,idx=new Map();
const size=new Float64Array(N),cls=new Int32Array(N);
for(let i=0;i<N;i++){idx.set(s.nodes[i*7],i);size[i]=s.nodes[i*7+1];cls[i]=s.nodes[i*7+2];}
// CSR adjacency (all edges strong); node index 0 = synthetic root (id 0) gets edges to every root
const cnt=new Uint32Array(N+1);const ef=new Int32Array(E),et=new Int32Array(E),ed=new Int32Array(E);
for(let e=0;e<E;e++){const f=idx.get(s.edges[e*4]),t=idx.get(s.edges[e*4+1]);ef[e]=f??-1;et[e]=t??-1;ed[e]=s.edges[e*4+3];if(f!==undefined&&t!==undefined)cnt[f+1]++;}
const rootList=[];for(let r=0;r<s.roots.length;r+=3){const i=idx.get(s.roots[r]);if(i!==undefined)rootList.push(i);}
for(let i=0;i<N;i++)cnt[i+1]+=cnt[i];
const first=cnt.slice(),adjn=new Uint32Array(cnt[N]),adje=new Uint32Array(cnt[N]),fill=cnt.slice(0,N);
for(let e=0;e<E;e++)if(ef[e]>=0&&et[e]>=0){adjn[fill[ef[e]]]=et[e];adje[fill[ef[e]]++]=e;}
const SR=N; // super root
const order=[],state=new Uint8Array(N+1),it=new Uint32Array(N+1);for(let i=0;i<N;i++)it[i]=first[i];
const stack=[SR],rootIt=[0];state[SR]=1;let ri=0;
while(stack.length){const v=stack[stack.length-1];
 if(v===SR){if(ri<rootList.length){const w=rootList[ri++];if(!state[w]){state[w]=1;stack.push(w);}}else{stack.pop();order.push(v);}continue;}
 if(it[v]<first[v+1]){const w=adjn[it[v]++];if(!state[w]){state[w]=1;stack.push(w);}}else{stack.pop();order.push(v);}}
const po=new Int32Array(N+1).fill(-1);order.forEach((v,i)=>po[v]=i);
console.log("reachable",order.length-1,"of",N);
const pc=new Uint32Array(N+2);
for(const v of order){if(v===SR){for(const w of rootList)pc[w+1]++;}else for(let k=first[v];k<first[v+1];k++)pc[adjn[k]+1]++;}
for(let i=0;i<=N;i++)pc[i+1]+=pc[i];const pf=pc.slice(),pred=new Uint32Array(pc[N+1]);
for(const v of order){if(v===SR){for(const w of rootList)pred[pf[w]++]=SR;}else for(let k=first[v];k<first[v+1];k++)pred[pf[adjn[k]]++]=v;}
const idom=new Int32Array(N+1).fill(-1);idom[SR]=SR;
const inter=(a,b)=>{while(a!==b){while(po[a]<po[b])a=idom[a];while(po[b]<po[a])b=idom[b];}return a;};
let ch=true;while(ch){ch=false;for(let i=order.length-2;i>=0;i--){const v=order[i];let nd=-1;for(let k=pc[v];k<pc[v+1];k++){const p=pred[k];if(idom[p]===-1)continue;nd=nd===-1?p:inter(p,nd);}if(nd!==-1&&idom[v]!==nd){idom[v]=nd;ch=true;}}}
const ret=new Float64Array(N+1);for(let v=0;v<N;v++)ret[v]=po[v]>=0?size[v]:0;
for(let i=0;i<order.length-1;i++){const v=order[i];if(idom[v]>=0&&v!==SR)ret[idom[v]]+=ret[v];}
const nm=v=>v===SR?"<ROOT>":s.nodeClassNames[cls[v]];
// edge name from idom to v
const edgeTo=new Map();for(let v=0;v<N;v++)if(po[v]>=0&&idom[v]>=0&&idom[v]!==SR){for(let k=first[idom[v]];k<first[idom[v]+1];k++)if(adjn[k]===v){const e=adje[k],t=s.edgeTypes[s.edges[e*4+2]],d=s.edges[e*4+3];edgeTo.set(v,t==="Property"||t==="Variable"?s.edgeNames[d]:t==="Index"?"[]":t);break;}}
const lab=v=>nm(v)+(edgeTo.has(v)?"."+edgeTo.get(v):"");
console.log("total MB",(ret[SR]/1048576).toFixed(0));
const top=[...order].filter(v=>v!==SR).sort((a,b)=>ret[b]-ret[a]).slice(0,+process.argv[3]||40);
for(const v of top){const ch=[];let p=v;for(let k=0;k<7&&p!==SR&&p>=0;k++){ch.push(lab(p));p=idom[p];}console.log((ret[v]/1048576).toFixed(1).padStart(7),ch.join(" <- "));}
