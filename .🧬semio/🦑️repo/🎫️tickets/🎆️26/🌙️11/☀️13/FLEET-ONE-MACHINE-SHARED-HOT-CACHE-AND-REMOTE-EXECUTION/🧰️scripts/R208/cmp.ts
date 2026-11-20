import{unpackRecordV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import{readFileSync,readdirSync,statSync,existsSync}from"node:fs";
import{createHash}from"node:crypto";
import{brotliDecompressSync}from"node:zlib";
import{join}from"node:path";
const root="/Users/ueli/Documents/semio",ac=root+"/.cache/f/cache-server/v1/main/ac",cas=process.env.HOME+"/.cache/semio/cas/v1/o";
const want=new Map<string,{kind:string;path:string}[]>();
for(const l of readFileSync("stale.tsv","utf8").trim().split("\n")){const [o,k,p]=l.split("\t");(want.get(o)??want.set(o,[]).get(o)!).push({kind:k,path:p});}
const now=Date.now();const out:string[]=[];
for(const d of readdirSync(ac))for(const f of readdirSync(join(ac,d))){const p=join(ac,d,f),m=statSync(p).mtimeMs;if(now-m>6*3600e3)continue;
 const text=readFileSync(p).toString("latin1"),at=text.indexOf('{"version"');if(at<0)continue;let meta:any;try{meta=JSON.parse(text.slice(at));}catch{continue;}
 if(meta.kind!=="project-inference")continue;const doc=meta.documents?.find((x:any)=>x.name==="record.br");if(!doc)continue;
 const blob=join(cas,doc.hash.slice(0,2),doc.hash.slice(2));if(!existsSync(blob))continue;
 let rec:any;try{rec=unpackRecordV1(JSON.parse(brotliDecompressSync(readFileSync(blob)).toString("utf8")));}catch{continue;}
 const nm=/"name":"([^"]+)"/.exec(String(rec.image).slice(0,600))?.[1]??"?";const w=want.get(nm);if(!w)continue;
 for(const x of w){const rel=x.path.slice(root.length);const row=rec.observations.find((r:any)=>(r.kind===x.kind)&&String(r.path).replace("⟦semio-root⟧","")===rel);
  if(!row){out.push(`${nm} rec@${new Date(m).toISOString().slice(11,19)} ${String(meta.producer).split(":")[0].slice(-10)} NO-ROW ${x.kind} ${rel.slice(-60)}`);continue;}
  const src=row.kind==="analysis"?row.fact.source:row.fact;let cur="absent";try{const b=readFileSync(x.path),s=statSync(x.path);cur=`size ${b.length} sha ${createHash("sha256").update(b).digest("hex").slice(0,10)} mtime ${new Date(s.mtimeMs).toISOString().slice(11,19)}`;}catch{}
  out.push(`${nm} rec@${new Date(m).toISOString().slice(11,19)} ${String(meta.producer).split(":")[0].slice(-10)} ${x.kind} rec: size ${src.size} sha ${String(src.sha256).slice(0,10)} | now: ${cur} ${rel.slice(-50)}`);}
}
console.log(out.sort().join("\n"));
