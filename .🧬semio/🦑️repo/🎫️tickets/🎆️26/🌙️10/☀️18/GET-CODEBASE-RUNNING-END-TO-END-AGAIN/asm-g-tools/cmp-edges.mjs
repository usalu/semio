import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const[,,A,B,name]=process.argv;const load=d=>{const p=JSON.parse(readFileSync(join(d,"current.json"),"utf8")),m=JSON.parse(readFileSync(join(d,"m",p.manifest+".json"),"utf8"));return JSON.parse(brotliDecompressSync(readFileSync(join(d,"o",m.owners[name].fragment+".json.br"))).toString("utf8"));};
const a=load(A),b=load(B);const sa=new Set(a.edges.map(e=>JSON.stringify(e))),sb=new Set(b.edges.map(e=>JSON.stringify(e)));
console.log("edges",a.edges.length,b.edges.length);
for(const e of sa)if(!sb.has(e))console.log("only A",e.slice(0,260));for(const e of sb)if(!sa.has(e))console.log("only B",e.slice(0,260));
console.log("node equal",JSON.stringify(a.node)===JSON.stringify(b.node),"root",a.root);
