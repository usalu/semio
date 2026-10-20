import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8")),target=process.argv[3],depth=+(process.argv[4]??1);
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const self=new Map();for(let i=0;i<p.samples.length;i++)self.set(p.samples[i],(self.get(p.samples[i])??0)+p.timeDeltas[i]);
const nm=id=>{const cf=nodes.get(id).callFrame;return (cf.functionName||"anon")+"@"+decodeURIComponent(cf.url.split("/").slice(-2).join("/")).replace(/[^\x20-\x7e]/g,"").slice(-22)+":"+cf.lineNumber};
const agg=new Map();let total=0;
for(const [id,t] of self){const chain=[];let c=id;while(c!==undefined){chain.push(c);c=parent.get(c);}
 const i=chain.findIndex(x=>nm(x).startsWith(target));if(i<0)continue;total+=t;
 const below=chain.slice(0,i).reverse().slice(0,depth).map(nm).join(" > ")||"(self)";agg.set(below,(agg.get(below)??0)+t);}
console.log("total under",target,Math.round(total/1000),"ms");
console.log([...agg].sort((a,b)=>b[1]-a[1]).slice(0,10).map(([k,v])=>Math.round(v/1000)+"ms "+k).join("\n"));
