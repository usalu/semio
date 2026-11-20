import{unpackRecordV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import{readFileSync,readdirSync,statSync,existsSync}from"node:fs";
import{brotliDecompressSync}from"node:zlib";
import{join}from"node:path";
const root="/Users/ueli/Documents/semio",ac=root+"/.cache/f/cache-server/v1/main/ac",cas=process.env.HOME+"/.cache/semio/cas/v1/o";
const minutes=Number(process.argv[2]??240),now=Date.now();
const entries:{m:number;p:string}[]=[];
for(const d of readdirSync(ac))for(const f of readdirSync(join(ac,d))){const p=join(ac,d,f),m=statSync(p).mtimeMs;if(now-m<minutes*60000)entries.push({m,p});}
const byOwner=new Map<string,{prod:string;key:string;rows:any[];at:number}[]>();
let n=0;
for(const {p,m} of entries){
 const text=readFileSync(p).toString("latin1"),at=text.indexOf('{"version"');if(at<0)continue;
 let meta:any;try{meta=JSON.parse(text.slice(at));}catch{continue;}
 if(meta.kind!=="project-inference")continue;
 const doc=meta.documents?.find((d:any)=>d.name==="record.br");if(!doc)continue;
 const blob=join(cas,doc.hash.slice(0,2),doc.hash.slice(2));if(!existsSync(blob))continue;
 try{
  const rec:any=unpackRecordV1(JSON.parse(brotliDecompressSync(readFileSync(blob)).toString("utf8")));
  const nm=/"name":"([^"]+)"/.exec(String(rec.image).slice(0,600))?.[1]??"?";
  const prod=String(meta.producer).split(":")[0].replace(/\.fritz\.box|-lan/,"");
  const rows=rec.observations.filter((r:any)=>r.kind!=="stamp").map((r:any)=>r.kind+"\u0001"+r.path+"\u0001"+JSON.stringify(r.fact));
  const l=byOwner.get(nm)??[];l.push({prod,key:meta.key??"",rows,at:m});byOwner.set(nm,l);n++;
 }catch(e){}
}

console.log("records",n,"owners",byOwner.size);
const WIN=Number(process.argv[3]??20)*60000;
const val:Record<string,number>={},only:Record<string,number>={},sv:Record<string,string[]>={},so:Record<string,string[]>={};let pairs=0,same=0;
const pk=new Map<string,number>();
for(const [nm,l] of byOwner){
 for(let i=0;i<l.length;i++)for(let j=i+1;j<l.length;j++){const a=l[i],b=l[j];if(a.prod===b.prod||Math.abs(a.at-b.at)>WIN)continue;
  const A=new Map(a.rows.map(r=>{const [k,p,f]=r.split("\u0001");return [k+"\u0001"+p,f] as const}));const B=new Map(b.rows.map(r=>{const [k,p,f]=r.split("\u0001");return [k+"\u0001"+p,f] as const}));
  pairs++;let d=0;
  for(const [kp,f] of A){const k=kp.split("\u0001")[0];const g=B.get(kp);if(g===undefined){only[k]=(only[k]??0)+1;d++;const x=so[k]??=[];if(x.length<5)x.push(nm+" only-in "+a.prod.slice(-8)+" vs "+b.prod.slice(-8)+" | "+kp.replace("\u0001"," ").slice(0,230)+" => "+f.slice(0,100));}
   else if(g!==f){val[k]=(val[k]??0)+1;d++;const x=sv[k]??=[];if(x.length<5)x.push(nm+" "+a.prod.slice(-8)+" vs "+b.prod.slice(-8)+" | "+kp.replace("\u0001"," ").slice(0,230)+" => "+f.slice(0,90)+" vs "+g.slice(0,90));}}
  for(const [kp,f] of B){if(!A.has(kp)){const k=kp.split("\u0001")[0];only[k]=(only[k]??0)+1;d++;const x=so[k]??=[];if(x.length<5)x.push(nm+" only-in "+b.prod.slice(-8)+" vs "+a.prod.slice(-8)+" | "+kp.replace("\u0001"," ").slice(0,230)+" => "+f.slice(0,100));}}
  if(d===0)same++;
 }
}
console.log("pairs",pairs,"identical",same);console.log("VALUE-DIFF",JSON.stringify(val));console.log("ONLY-ONE-SIDE",JSON.stringify(only));
for(const k of Object.keys(sv)){console.log("== valdiff",k);console.log(sv[k].join("\n"));}
for(const k of Object.keys(so)){console.log("== only",k);console.log(so[k].join("\n"));}
