import{readFileSync}from"node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8")),lastMs=+process.argv[3],top=+process.argv[4]||30;
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
let t=0;const stamps=p.timeDeltas.map(d=>t+=d);const end=t;
const selfBy=new Map(),incl=new Map();
p.samples.forEach((id,i)=>{if(stamps[i]<end-lastMs*1000)return;const d=p.timeDeltas[i]??0;const n=nodes.get(id),k=n.callFrame.functionName+" "+n.callFrame.url.split("/").slice(-2).join("/")+":"+n.callFrame.lineNumber;selfBy.set(k,(selfBy.get(k)??0)+d);
 const seen=new Set();for(let c=id;c!==undefined;c=parent.get(c)){const m=nodes.get(c),kk=m.callFrame.functionName+" "+m.callFrame.url.split("/").slice(-2).join("/")+":"+m.callFrame.lineNumber;if(!seen.has(kk)){seen.add(kk);incl.set(kk,(incl.get(kk)??0)+d);}}});
const show=(m,label)=>{console.log("--",label);for(const[k,v]of[...m].sort((a,b)=>b[1]-a[1]).slice(0,top))console.log((v/1000).toFixed(0).padStart(7),k.slice(0,150));};
show(selfBy,"self");show(incl,"inclusive");
