import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce/e";
const names=readdirSync(D).filter(n=>n.endsWith(".json.br"));const pick:string[]=[];for(let i=0;i<names.length&&pick.length<400;i+=Math.floor(names.length/400))pick.push(names[i]);
let rows=0,layers=0,bytes=0,maxRows=0,sidecarWalks=0,recipeBytes=0;const hist:number[]=[];
for(const n of pick){const text=brotliDecompressSync(readFileSync(D+"/"+n)).toString("utf8");bytes+=text.length;const r=JSON.parse(text);rows+=r.rows.length;maxRows=Math.max(maxRows,r.rows.length);layers+=r.layers.length;sidecarWalks+=(r.sidecar?.walks?.length??0);recipeBytes+=JSON.stringify(r.recipe).length;hist.push(r.rows.length);}
hist.sort((a,b)=>a-b);
console.log(JSON.stringify({entries:pick.length,meanRows:rows/pick.length,maxRows,medianRows:hist[hist.length>>1],meanLayers:layers/pick.length,meanTextKB:bytes/pick.length/1024,meanSidecarWalks:sidecarWalks/pick.length,meanRecipeBytes:recipeBytes/pick.length}));
