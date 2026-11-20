const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const LB:any=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await LB.libraryBootstrap;
const O:any=await import(B+"👁️observation/🟨️.mjs");
const SA:any=await import(B+"🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs");
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const P:any=await import(B+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
import{readFileSync,readdirSync,statSync,existsSync}from"node:fs";
import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const provs=[...Object.values(SA.prepareSourceAnalysesV1()).map((p:any)=>p.id),...SI.scriptProviderIdsV1()];
console.log("providers",provs.map((x:string)=>x.slice(0,40)));
const root="/Users/ueli/Documents/semio",ac=root+"/.cache/f/cache-server/v1/main/ac",cas=process.env.HOME+"/.cache/semio/cas/v1/o";
const host=P.hostOfRootV1(root),gen=P.dependencyGenerationV1(root);const WANT=process.argv.slice(2);let shown=0;
for(const d of readdirSync(ac))for(const f of readdirSync(join(ac,d))){const p=join(ac,d,f);if(Date.now()-statSync(p).mtimeMs>100*60e3)continue;
 const text=readFileSync(p).toString("latin1"),at=text.indexOf('{"version"');if(at<0)continue;let meta:any;try{meta=JSON.parse(text.slice(at));}catch{continue;}
 if(meta.kind!=="project-inference")continue;const doc=meta.documents?.find((x:any)=>x.name==="record.br");if(!doc)continue;
 const blob=join(cas,doc.hash.slice(0,2),doc.hash.slice(2));if(!existsSync(blob))continue;
 const rec=P.unpackRecordV1(JSON.parse(brotliDecompressSync(readFileSync(blob)).toString("utf8")));
 const nm=/"name":"([^"]+)"/.exec(String(rec.image).slice(0,600))?.[1]??"?";if(!WANT.includes(nm))continue;
 console.log("MATCH",nm);const local=P.localRecordV1(rec,host,{providers:provs,generation:gen}).record;
 const R2:any=await import(B+"👁️observation/📥️reuse/🟨️.mjs");
 const names:string[]=R2.captureInferenceInventoryReferencesV1(JSON.parse(String(local.image)));
 const dirs={SEMIO_SOURCE_CLOSURE_:root+"/.cache/b/g/m/build-input-closures/",SEMIO_CARGO_LOCK_:root+"/.cache/b/g/m/build-lock-closures/"};
 const miss:string[]=[];for(const n of names){const pre=n.startsWith("SEMIO_SOURCE_CLOSURE_")?"SEMIO_SOURCE_CLOSURE_":"SEMIO_CARGO_LOCK_";if(!existsSync(dirs[pre as keyof typeof dirs]+n.slice(pre.length)+".json"))miss.push(pre.slice(6,12)+n.slice(pre.length,pre.length+8));}
 console.log(nm,"rec@"+new Date(statSync(p).mtimeMs).toISOString().slice(11,19),String(meta.producer).split(":")[0].slice(-10),"names",names.length,"missing",miss.length,JSON.stringify(miss.slice(0,6)));
}
