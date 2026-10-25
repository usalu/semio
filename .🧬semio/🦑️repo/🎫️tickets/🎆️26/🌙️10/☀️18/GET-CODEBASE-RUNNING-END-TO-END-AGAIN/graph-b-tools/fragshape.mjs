import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";const want=process.argv[2];
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
const f=JSON.parse(brotliDecompressSync(readFileSync(join(g,"o",m.owners[want].fragment+".json.br"))).toString());
const size=v=>JSON.stringify(v)?.length??0;
const walk=(v,path,depth)=>{if(depth>3||v===null||typeof v!=="object")return;const rows=Object.entries(v).map(([k,x])=>[size(x),k,x]).sort((a,b)=>b[0]-a[0]).slice(0,5);for(const[s,k,x]of rows){if(s<200000)continue;console.log(" ".repeat(depth*2)+path+"/"+k.slice(0,60),s,Array.isArray(x)?"array["+x.length+"]":"");walk(x,path+"/"+k.slice(0,30),depth+1);}};
console.log("total",size(f));walk(f,"",0);
