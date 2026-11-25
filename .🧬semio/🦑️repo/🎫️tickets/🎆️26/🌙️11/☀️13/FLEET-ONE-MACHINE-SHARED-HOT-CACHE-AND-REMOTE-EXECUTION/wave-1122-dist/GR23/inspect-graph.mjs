import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const dir=process.argv[2],head=JSON.parse(readFileSync(join(dir,"head.json"),"utf8"));
let raw=0,nodes=0,witnessBytes=0,edges=0,keyBytes=0,extraNoWitness=0;const kinds={};
for(const d of head.chunks){if(!d)continue;const t=brotliDecompressSync(readFileSync(join(dir,"c",d+".json.br"))).toString();raw+=t.length;const c=JSON.parse(t);for(const [key,name,kind,digest,e,extra] of c.records){nodes++;edges+=e.length;keyBytes+=key.length;kinds[kind]=(kinds[kind]??0)+1;if(extra){const w=extra.witness;witnessBytes+=JSON.stringify(w??null).length;const {witness,...rest}=extra;extraNoWitness+=JSON.stringify(rest).length;}}}
let poolBytes=0,bases=0;if(head.pool){const t=brotliDecompressSync(readFileSync(join(dir,"c",head.pool+".json.br"))).toString();poolBytes=t.length;bases=JSON.parse(t).bases.length;}
console.log(JSON.stringify({chunks:head.chunks.filter(Boolean).length,raw,nodes,edges,keyBytes,witnessBytes,extraNoWitness,poolBytes,bases,kinds,rss:process.memoryUsage().rss>>20}));
