import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {hostOfRootV1,neutralRowsV1,rowMemoV1}=await import(B+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const host=hostOfRootV1("/Users/ueli/Documents/semio"),ce=process.argv[2],count=Number(process.argv[3]??40);
const dec=(p:string)=>JSON.parse(brotliDecompressSync(readFileSync(p)).toString("utf8"));
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs] as [string,number]).sort((a,b)=>b[1]-a[1]).slice(0,count);
const memo=rowMemoV1(),done=new Set<string>(),hits:Record<string,{n:number;ex:Set<string>}>={};let rows=0;
const names2=["⟦semio-root⟧/node_modules/"];
for(const[n]of names){const e=dec(join(ce,"e",n));const all=[...e.rows];for(const d of new Set<string>(e.layers)){if(done.has(d))continue;done.add(d);try{all.push(...dec(join(ce,"l",d+".json.br")));}catch{}}
 for(const r of neutralRowsV1(all,host,new Map(),undefined,memo)){rows++;let p=r.path;if(r.kind==="closure")try{p=JSON.parse(p)[0];}catch{}
  for(const m of names2)if(String(p).includes(m)){const g=hits[r.kind+" "+m]??={n:0,ex:new Set()};g.n++;if(g.ex.size<3)g.ex.add(JSON.stringify(r).slice(0,300));}}}
console.log({entries:names.length,rows});for(const[k,g]of Object.entries(hits).sort((a,b)=>b[1].n-a[1].n))console.log(g.n,k,[...g.ex]);
