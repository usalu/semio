import{unpackRecordV1,localRecordV1,hostOfRootV1,dependencyGenerationV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import*as O from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
import{readdirSync,readFileSync,existsSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";import{homedir}from"node:os";
const root="/Users/ueli/Documents/semio",ac=root+"/.cache/f/cache-server/v1/main/ac",cas=homedir()+"/.cache/semio/cas/v1/o";
const sinceMs=Date.now()-Number(process.env.HOURS??4)*3600e3,N=Number(process.env.N??50);
const rows:any[]=[];
for(const d of readdirSync(ac))for(const f of readdirSync(join(ac,d))){let b:Buffer;try{b=readFileSync(join(ac,d,f));}catch{continue;}const i=b.indexOf('{"version"');if(i<0)continue;let j:any;try{j=JSON.parse(b.subarray(i).toString("utf8").split("\n")[0]);}catch{continue;}if(j.kind!=="project-inference"||Number(j.createdMs)<sinceMs)continue;rows.push(j);}
console.log("records since",rows.length);
const by=new Map<string,any>();for(const r of rows){by.set(r.digest,r);}
const producers=new Map<string,number>();for(const r of by.values()){const p=String(r.producer).split(":")[0];producers.set(p,(producers.get(p)??0)+1);}console.log([...producers]);
const host=hostOfRootV1(root),gen=dependencyGenerationV1(root);
let n=0,ok=0,bad=0,decodeFail=0;const perRecord:string[]=[];const kinds=new Map<string,number>(),sample:string[]=[];
const list=[...by.values()].filter(r=>!String(r.producer).startsWith("ueli-mac.")).sort(()=>Math.random()-.5);
for(const r of list){if(n>=N)break;const doc=r.documents?.find((x:any)=>x.name==="record.br");if(!doc)continue;const h=String(doc.hash).replace(/^[a-z0-9]+:/,"");const p=join(cas,h.slice(0,2),h.slice(2));if(!existsSync(p)){decodeFail++;continue;}
 let rec:any;try{rec=unpackRecordV1(JSON.parse(brotliDecompressSync(readFileSync(p)).toString("utf8")));}catch(e){decodeFail++;continue;}
 n++;let local:any;try{local=localRecordV1(rec,host,{providers:[],generation:gen}).record;}catch(e:any){bad++;kinds.set("localize:"+String(e.message).slice(0,60),(kinds.get("localize:"+String(e.message).slice(0,60))??0)+1);continue;}
 let failed=false;const here=new Map<string,number>();for(const row of local.observations){if(row.kind==="stamp"||row.kind==="analysis")continue;let f;try{f=O.observationRowFreshV1(row);}catch(e){f=false;}if(f!==true&&row.kind!=="closure"){failed=true;if(sample.length<40)sample.push(row.kind+" "+String(row.path).replace("/Users/ueli/Documents/semio/","").slice(0,150));const k=row.kind+(row.path?":"+String(row.path).replace(/^.*\/(?=[^/]+\/[^/]+$)/,"").slice(0,40):"");{const k=row.kind+(row.kind==="field"?":"+row.fact?.format:"");here.set(k,(here.get(k)??0)+1);kinds.set(k,(kinds.get(k)??0)+1);}if(sample.length<12)sample.push(JSON.stringify(row).slice(0,200));}}
 if(failed){bad++;perRecord.push(String(rec.image?JSON.parse(rec.image).name:"?")+" "+String(r.producer).split(":")[0].slice(0,34)+" "+JSON.stringify([...here]));}else ok++;}
console.log(JSON.stringify({sampled:n,fresh:ok,stale:bad,decodeFail,failedRowKinds:[...kinds]}));
console.log(perRecord.length+" records with non-closure failures");console.log([...new Set(sample)].join("\n"));
