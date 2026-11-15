import {readdirSync,readFileSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {join} from "node:path";
const root=process.argv[2],N=Number(process.argv[3]??400);
const names=readdirSync(root+"/e").filter(n=>n.endsWith(".br"));
const step=Math.max(1,Math.floor(names.length/N));const freq=new Map();let ent=0,ownRows=0,layerCount=0;const fns={};
const kinds={};
for(let i=0;i<names.length;i+=step){const r=JSON.parse(brotliDecompressSync(readFileSync(join(root,"e",names[i])),{maxOutputLength:1<<29}).toString());
 ent++;ownRows+=r.rows.length;layerCount+=r.layers.length;fns[r.recipe.fn]=(fns[r.recipe.fn]??0)+1;for(const d of r.layers)freq.set(d,(freq.get(d)??0)+1);for(const row of r.rows)kinds[row.kind]=(kinds[row.kind]??0)+1;}
console.log({entries:ent,avgOwnRows:ownRows/ent,avgLayers:layerCount/ent,distinctLayers:freq.size,fns,kinds});
const top=[...freq].sort((a,b)=>b[1]-a[1]).slice(0,8);
for(const [d,c] of top){let sz=0,rows=0,ak={};try{const b=brotliDecompressSync(readFileSync(join(root,"l",d+".json.br")),{maxOutputLength:1<<29});sz=b.length;const rs=JSON.parse(b.toString());rows=rs.length;for(const x of rs)ak[x.kind]=(ak[x.kind]??0)+1;}catch{}
 console.log(d.slice(0,8),"used by",c,"of",ent,"bytes",sz,"rows",rows,ak);}
