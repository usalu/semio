import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
const frag=new Map();const get=n=>{let f=frag.get(n);if(!f){const t=brotliDecompressSync(readFileSync(join(g,"o",m.owners[n].fragment+".json.br")));f={size:t.length,edges:JSON.parse(t.toString()).edges??[],node:JSON.parse(t.toString()).node};frag.set(n,f);}return f;};
const roots=process.argv.slice(2);const seen=new Set(),queue=[...roots];
while(queue.length){const n=queue.pop();if(seen.has(n)||!m.owners[n]||m.owners[n].kind!=="project")continue;seen.add(n);for(const e of get(n).edges)queue.push(e.target);}
const rows=[...seen].map(n=>[get(n).size,n]).sort((a,b)=>b[0]-a[0]);
console.log("generation",m.generation,"closure projects",seen.size,"MB",rows.reduce((a,r)=>a+r[0],0)/1e6);for(const r of rows.slice(0,8))console.log(r.join(" "));
