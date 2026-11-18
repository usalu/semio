import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const dir="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np";
const names=readdirSync(dir).filter(n=>n.endsWith(".json"));console.log("records",names.length);
const step=Math.max(1,Math.floor(names.length/Number(process.argv[2]||400)));
const all=new Map<string,number>();let total=0,n=0,bytes=0;const perOwner:number[]=[];
for(let i=0;i<names.length;i+=step){try{const b=readFileSync(dir+"/"+names[i]);bytes+=b.length;const t=brotliDecompressSync(b).toString("utf8");const v=JSON.parse(t.slice(t.indexOf("\n")+1));const keys=Object.keys(v.closureRecipes??{});perOwner.push(keys.length);total+=keys.length;n++;for(const k of keys)all.set(k,(all.get(k)??0)+1);}catch(e){}}
const shared=[...all.values()].filter(c=>c>1).length;
const hot=[...all.values()].sort((a,b)=>b-a).slice(0,8);
console.log(JSON.stringify({sampled:n,perOwnerMean:total/n,totalRefs:total,distinct:all.size,distinctShare:all.size/total,sharedKeys:shared,hottest:hot,meanBytes:bytes/n}));
