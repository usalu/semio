import{readFileSync}from"node:fs";
const snap=JSON.parse(readFileSync(process.argv[2],"utf8")),top=Number(process.argv[3]??40);
const{nodes,edges,nodeClassNames,edgeNames,edgeTypes}=snap,N=nodes.length/4,id=[],size=[],cls=[],index=new Map();
for(let i=0;i<N;i++){id[i]=nodes[i*4];size[i]=nodes[i*4+1];cls[i]=nodeClassNames[nodes[i*4+2]];index.set(id[i],i);}
const out=Array.from({length:N},()=>[]);
for(let e=0;e<edges.length;e+=4){const from=index.get(edges[e]),to=index.get(edges[e+1]);if(from!==undefined&&to!==undefined)out[from].push([to,edgeTypes[edges[e+2]]==="Property"||edgeTypes[edges[e+2]]==="Variable"?edgeNames[edges[e+3]]:edgeTypes[edges[e+2]]==="Index"?"["+edges[e+3]+"]":"",edgeTypes[edges[e+2]]]);}
const parent=new Int32Array(N).fill(-2),label=new Array(N),order=[];parent[0]=-1;const queue=[0];
for(let q=0;q<queue.length;q++){const at=queue[q];order.push(at);for(const[to,name]of out[at])if(parent[to]===-2){parent[to]=at;label[to]=name;queue.push(to);}}
const sub=Float64Array.from(size);for(let i=order.length-1;i>0;i--){const at=order[i];if(parent[at]>=0)sub[parent[at]]+=sub[at];}
const byClass={};let total=0;for(let i=0;i<N;i++){total+=size[i];byClass[cls[i]]=(byClass[cls[i]]??[0,0]);byClass[cls[i]][0]+=size[i];byClass[cls[i]][1]++;}
const mb=n=>(n/1048576).toFixed(1);
console.log("TOTAL",mb(total),"MB nodes",N);
console.log("BY CLASS (self)");for(const[k,v]of Object.entries(byClass).sort((a,b)=>b[1][0]-a[1][0]).slice(0,12))console.log(" ",k.padEnd(28),mb(v[0]).padStart(8),"MB",v[1]);
const kids=Array.from({length:N},()=>[]);for(let i=1;i<order.length;i++){const at=order[i];if(parent[at]>=0)kids[parent[at]].push(at);}
const threshold=Number(process.argv[4]??0.025)*total;
console.log("TREE (spanning-tree subtree size, nodes >= "+(threshold/1048576).toFixed(0)+" MB)");
const walk=(at,depth)=>{if(depth>16)return;for(const child of kids[at].sort((a,b)=>sub[b]-sub[a])){if(sub[child]<threshold)break;console.log(" ".repeat(depth)+mb(sub[child]).padStart(7)+" MB  "+(label[child]||"·")+" <"+cls[child]+"> self "+mb(size[child]));walk(child,depth+1);}};
walk(0,0);
