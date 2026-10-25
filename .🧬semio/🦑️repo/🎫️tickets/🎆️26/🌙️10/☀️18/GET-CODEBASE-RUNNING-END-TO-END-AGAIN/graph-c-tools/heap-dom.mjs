import{readFileSync}from"node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8"));
const meta=snap.snapshot.meta,nf=meta.node_fields,ef=meta.edge_fields;
const NF=nf.length,EF=ef.length,nodes=snap.nodes,edges=snap.edges,strings=snap.strings;
const iType=nf.indexOf("type"),iName=nf.indexOf("name"),iSize=nf.indexOf("self_size"),iEC=nf.indexOf("edge_count"),iId=nf.indexOf("id");
const eType=ef.indexOf("type"),eName=ef.indexOf("name_or_index"),eTo=ef.indexOf("to_node");
const edgeTypes=meta.edge_types[0],nodeTypes=meta.node_types[0];
const N=nodes.length/NF;
const first=new Uint32Array(N+1);let e=0;for(let i=0;i<N;i++){first[i]=e;e+=nodes[i*NF+iEC];}first[N]=e;
const to=new Uint32Array(e/1|0);for(let k=0;k<e;k++)to[k]=edges[k*EF+eTo]/NF;
const weak=new Uint8Array(e);for(let k=0;k<e;k++){const t=edgeTypes[edges[k*EF+eType]];if(!process.env.WEAK&&(t==="weak"||t==="shortcut"))weak[k]=1;}
const tcount={};for(let i=0;i<N;i++){const t=nodeTypes[nodes[i*NF+iType]];tcount[t]=(tcount[t]||0)+1;}console.log("nodes",N,JSON.stringify(tcount));
// postorder DFS from node 0 over strong edges
const order=[],state=new Uint8Array(N),stack=[0],it=new Uint32Array(N);state[0]=1;it.set(first.subarray(0,N));
while(stack.length){const v=stack[stack.length-1];if(it[v]<first[v+1]){const k=it[v]++;if(weak[k])continue;const w=to[k];if(!state[w]){state[w]=1;stack.push(w);}}else{stack.pop();order.push(v);}}
const po=new Int32Array(N).fill(-1);order.forEach((v,i)=>po[v]=i);
// predecessors
const pc=new Uint32Array(N+1);for(let v=0;v<N;v++)for(let k=first[v];k<first[v+1];k++)if(!weak[k]&&po[v]>=0)pc[to[k]+1]++;
for(let i=0;i<N;i++)pc[i+1]+=pc[i];const pf=pc.slice(),pred=new Uint32Array(pc[N]);
for(let v=0;v<N;v++)if(po[v]>=0)for(let k=first[v];k<first[v+1];k++)if(!weak[k])pred[pf[to[k]]++]=v;
const idom=new Int32Array(N).fill(-1);idom[0]=0;
const intersect=(a,b)=>{while(a!==b){while(po[a]<po[b])a=idom[a];while(po[b]<po[a])b=idom[b];}return a;};
let changed=true;while(changed){changed=false;for(let i=order.length-2;i>=0;i--){const v=order[i];let nd=-1;for(let k=pc[v];k<pc[v+1];k++){const p=pred[k];if(idom[p]===-1)continue;nd=nd===-1?p:intersect(p,nd);}if(nd!==-1&&idom[v]!==nd){idom[v]=nd;changed=true;}}}
const ret=new Float64Array(N);for(let v=0;v<N;v++)ret[v]=po[v]>=0?(+process.env.COUNT?1:nodes[v*NF+iSize]):0;
for(let i=0;i<order.length-1;i++){const v=order[i];if(idom[v]>=0&&v!==0)ret[idom[v]]+=ret[v];}
const name=v=>nodeTypes[nodes[v*NF+iType]]+":"+String(strings[nodes[v*NF+iName]]).slice(0,60);
const ids=[...Array(N).keys()].filter(v=>po[v]>=0).sort((a,b)=>ret[b]-ret[a]).slice(0,+process.argv[3]||40);
console.log("total",(ret[0]/1048576).toFixed(0),"MB");
for(const v of ids){let path=[],p=v;for(let d=0;d<4&&p!==0;d++){p=idom[p];path.push(name(p));}console.log((ret[v]/1048576).toFixed(1).padStart(7),name(v),"<-",path.join(" <- "));}
