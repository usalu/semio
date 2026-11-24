import{readdirSync,statSync,existsSync}from"node:fs";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const obs=await import(B+"👁️observation/🟨️.mjs");
const {hostOfRootV1,neutralRecordV1}=await import(B+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const root="/Users/ueli/Documents/semio",host=hostOfRootV1(root),dir=process.argv[2],count=Number(process.argv[3]??60);
const names=readdirSync(dir).filter(n=>n.endsWith(".json")).map(n=>[n,statSync(join(dir,n)).mtimeMs] as [string,number]).sort((a,b)=>b[1]-a[1]).slice(0,count);
const res:Record<string,{n:number;ex:Set<string>}>={};let ok=0,fail=0,fleet=0,rows=0;
const flag=(kind:string,text:string)=>{const m=text.match(/\/(?:Users|home|private|opt|var|tmp|usr|Library|nix|root|mnt)\/[^"\\\],]{0,100}|[A-Za-z]:[\\/][^"\\\],]{0,60}/u);if(!m)return;const g=res[kind+" "+m[0].split("/").slice(0,3).join("/")]??={n:0,ex:new Set()};g.n++;if(g.ex.size<3)g.ex.add(m[0].slice(0,110));};
for(const[n]of names){const path=join(dir,n);let portable:any;try{portable=await obs.exportProjectRecordV1(path);}catch(e:any){fail++;if(fail<3)console.log("export failed",String(e.message).slice(0,160));continue;}
 if(existsSync(path+".fleet"))fleet++;
 let neutral:any;try{neutral=neutralRecordV1(portable,host).record;}catch(e:any){fail++;console.log("neutral failed",n,String(e.message).slice(0,200));continue;}ok++;
 for(const r of [...neutral.observations,...neutral.authority]){rows++;flag(r.kind,r.path);if(typeof r.fact==="string")flag(r.kind+".fact",r.fact);else if(r.fact&&typeof r.fact==="object")flag(r.kind+".fact",JSON.stringify(r.fact).replace(/"(mtimeMs|ctimeMs|ino)":[0-9.e+-]+/g,""));}
 flag("image",neutral.image);}
console.log({tried:names.length,ok,fail,fleet,rows});for(const[k,g]of Object.entries(res).sort((a,b)=>b[1].n-a[1].n).slice(0,20))console.log(g.n,k,[...g.ex]);
