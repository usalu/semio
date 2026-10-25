import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";const name=process.argv[3]??"@semio-tech/print";
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
const live=JSON.parse(brotliDecompressSync(readFileSync(join(g,"o",m.owners[name].fragment+".json.br"))).toString()).node;
const fresh=JSON.parse(readFileSync(process.argv[2],"utf8"))[name].project;
const WS="/Users/ueli/Documents/semio";
const eff=(project)=>{const named=project.namedInputs??{};const out=new Map();
 const expand=(item,seen)=>{if(item&&typeof item==="object"&&typeof item.env==="string"&&item.env.startsWith("SEMIO_SOURCE_CLOSURE_")){const key=item.env.slice(21);try{return JSON.parse(readFileSync("/private/tmp/gr1022/t6/scratch/build-input-closures/"+key+".json","utf8")).files.map(f=>"{workspaceRoot}/"+f);}catch{return[item];}}if(typeof item==="string"&&Object.hasOwn(named,item)&&!seen.has(item)){seen.add(item);return named[item].flatMap(x=>expand(x,seen));}return[item];};
 for(const[t,target]of Object.entries(project.targets)){const seen=new Set();out.set(t,new Set((target.inputs??[]).flatMap(x=>expand(x,seen)).map(x=>JSON.stringify(x).split(WS).join("<WS>").replaceAll("<WS>","{abs}"))));}
 return out;};
const a=eff(live),b=eff(fresh);let diff=0,same=0;
for(const t of new Set([...a.keys(),...b.keys()])){const x=a.get(t)??new Set(),y=b.get(t)??new Set();const only1=[...x].filter(i=>!y.has(i)),only2=[...y].filter(i=>!x.has(i));if(only1.length||only2.length){diff++;if(diff<6)console.log(t,"live-only",only1.length,only1.slice(0,2),"fresh-only",only2.length,only2.slice(0,2));}else same++;}
console.log("targets",a.size,b.size,"same",same,"different",diff);
const size=p=>JSON.stringify(p).length;console.log("bytes live",size(live),"fresh",size(fresh));
