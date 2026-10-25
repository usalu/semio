import{brotliDecompressSync,brotliCompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const v=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2]),{maxOutputLength:1<<30}).toString("utf8"));
const paths=new Set(),segs=new Set(),facts=new Map(),dirs=new Set();let pathBytes=0,factBytes=0;
const pk=new Map(),trie=new Set();
for(const r of v.rows){paths.add(r.path);const f=JSON.stringify(r.fact);facts.set(f,(facts.get(f)||0)+1);pathBytes+=Buffer.byteLength(r.path);factBytes+=Buffer.byteLength(f);
 const parts=r.path.split("/");let acc="";for(const p of parts){segs.add(p);acc=acc?acc+"/"+p:p;trie.add(acc);}}
const uniqPathBytes=[...paths].reduce((a,p)=>a+Buffer.byteLength(p),0),segBytes=[...segs].reduce((a,p)=>a+Buffer.byteLength(p),0);
console.log("rows",v.rows.length,"unique paths",paths.size,"unique path bytes",uniqPathBytes,"total path bytes",pathBytes);
console.log("segments",segs.size,"seg bytes",segBytes,"trie nodes",trie.size);
const fb=[...facts.keys()].reduce((a,f)=>a+Buffer.byteLength(f),0);
console.log("unique facts",facts.size,"unique fact bytes",fb,"total fact bytes",factBytes);
// by kind unique fact
const byKind=new Map();for(const r of v.rows){const f=JSON.stringify(r.fact);const e=byKind.get(r.kind)||{n:0,set:new Set(),b:0};e.n++;if(!e.set.has(f)){e.set.add(f);e.b+=Buffer.byteLength(f);}byKind.set(r.kind,e);}
for(const[k,e]of byKind)console.log(k,"rows",e.n,"unique facts",e.set.size,"uniq bytes",e.b);
// brotli sizes
const rowsJson=JSON.stringify(v.rows);console.log("rows brotli q5",brotliCompressSync(Buffer.from(rowsJson),{params:{1:5,2:22}}).length);
