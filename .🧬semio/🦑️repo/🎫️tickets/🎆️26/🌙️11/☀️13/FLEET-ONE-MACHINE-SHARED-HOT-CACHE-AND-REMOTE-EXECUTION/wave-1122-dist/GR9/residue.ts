import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {hostOfRootV1,neutralRowsV1,rowMemoV1}=await import(B+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const root="/Users/ueli/Documents/semio",host=hostOfRootV1(root);
const ce=process.argv[2],count=Number(process.argv[3]??40);
const dec=(p:string)=>JSON.parse(brotliDecompressSync(readFileSync(p)).toString("utf8"));
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs] as [string,number]).sort((a,b)=>b[1]-a[1]).slice(0,count);
const memo=rowMemoV1(),res:Record<string,{n:number;ex:Set<string>}>={};let rows=0;const done=new Set<string>();
const flag=(kind:string,text:string)=>{const m=text.match(/\/(?:Users|home|private|opt|var|tmp|usr|Library|nix|root|mnt)\/[^"\\\],]{0,100}|[A-Za-z]:[\\/][^"\\\],]{0,60}/u);if(!m)return;const g=res[kind+" "+m[0].split("/").slice(0,3).join("/")]??={n:0,ex:new Set()};g.n++;if(g.ex.size<3)g.ex.add(m[0].slice(0,110));};
for(const[n]of names){const e=dec(join(ce,"e",n));const all=[...e.rows];for(const d of new Set<string>(e.layers)){if(done.has(d))continue;done.add(d);try{all.push(...dec(join(ce,"l",d+".json.br")));}catch{}}
 const out=neutralRowsV1(all,host,new Map(),undefined,memo);rows+=out.length;
 for(const r of out){flag(r.kind,r.path);if(typeof r.fact==="string")flag(r.kind+".fact",r.fact);else if(r.fact&&typeof r.fact==="object"){const s=JSON.stringify(r.fact);flag(r.kind+".fact",s.replace(/"(mtimeMs|ctimeMs|ino)":[0-9.e+-]+/g,""));}}}
console.log({entries:names.length,rows});for(const[k,g]of Object.entries(res).sort((a,b)=>b[1].n-a[1].n).slice(0,20))console.log(g.n,k,[...g.ex]);
