// Samples the closure store: layers per entry, sharing between entries, rows per layer.
import {readdirSync,readFileSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";import {join} from "node:path";
const dir=process.argv[2],n=Number(process.argv[3]??300),names=readdirSync(join(dir,"e")).sort(()=>Math.random()-.5).slice(0,n);
const refs=new Map();let totalRefs=0,rowsTotal=0,byFn={};
for(const f of names){const r=JSON.parse(brotliDecompressSync(readFileSync(join(dir,"e",f))).toString());const fn=r.recipe?.fn??"?";byFn[fn]=(byFn[fn]??0)+1;for(const d of r.layers){refs.set(d,(refs.get(d)??0)+1);totalRefs++;}rowsTotal+=r.rows.length;}
const sizes=[];for(const d of [...refs.keys()].slice(0,100)){try{sizes.push(JSON.parse(brotliDecompressSync(readFileSync(join(dir,"l",d+".json.br"))).toString()).length);}catch{}}
console.log(JSON.stringify({sampled:names.length,byFn,layerRefs:totalRefs,distinctLayers:refs.size,sharedLayers:[...refs.values()].filter(v=>v>1).length,avgLayersPerEntry:totalRefs/names.length,entryRowsAvg:rowsTotal/names.length,layerRowsSample:sizes.slice(0,20),layerRowsAvg:sizes.reduce((a,b)=>a+b,0)/Math.max(1,sizes.length)}));
