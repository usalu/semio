import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const dir=process.argv[2],root="/Users/ueli/Documents/semio",limit=Number(process.argv[3]??300);
const names=readdirSync(dir).filter(n=>n.endsWith(".json")).map(n=>[n,statSync(join(dir,n)).mtimeMs]).sort((a,b)=>b[1]-a[1]).slice(0,limit);
const byKind={},outside={},examples={};let fleet=0,total=0;
for(const[n]of names){let raw=readFileSync(join(dir,n));let text;try{text=brotliDecompressSync(raw).toString("utf8");}catch{text=raw.toString("utf8");}
 const head=JSON.parse(text.slice(0,text.indexOf("\n")));total++;
 let isFleet=false;try{statSync(join(dir,n+".fleet"));isFleet=true;fleet++;}catch{}
 for(const r of head.observations??[]){byKind[r.kind]=(byKind[r.kind]??0)+1;
  const ps=[r.path];if(r.kind==="closure"){try{ps.splice(0,1,JSON.parse(r.path)[0]);}catch{}}
  for(const p of ps){if(typeof p==="string"&&p.startsWith("/")&&!p.startsWith(root+"/")&&p!==root){const k=r.kind+(isFleet?" fleet":" own");outside[k]=(outside[k]??0)+1;(examples[k]??=new Set()).add(p.slice(0,140));}}
 }}
console.log(JSON.stringify({total,fleet,byKind,outside},null,1));for(const[k,v]of Object.entries(examples))console.log(k,[...v].slice(0,6));
