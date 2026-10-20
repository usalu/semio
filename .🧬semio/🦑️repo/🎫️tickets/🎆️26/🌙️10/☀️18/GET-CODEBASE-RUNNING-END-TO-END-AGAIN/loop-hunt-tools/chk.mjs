import fs from "node:fs";import {brotliDecompressSync} from "node:zlib";import {createHash} from "node:crypto";
const np=process.argv[2];const files=process.argv.slice(3);
const out={};
for(const f of files){
 const text=brotliDecompressSync(fs.readFileSync(np+"/"+f)).toString("utf8"),nl=text.indexOf("\n"),h=JSON.parse(text.slice(0,nl));
 const kinds={},stale=[];
 for(const {kind,path,fact} of h.observations){
  kinds[kind]=(kinds[kind]??0)+1;
  if(kind==="file"){
   if(fact.error!==undefined)continue;
   let st;try{st=fs.statSync(path);}catch{stale.push([kind,path,"missing"]);continue;}
   if(st.size!==fact.size){stale.push([kind,path,"size",fact.size,st.size,new Date(st.mtimeMs).toISOString()]);continue;}
   if(st.mtimeMs===fact.mtimeMs&&st.ctimeMs===fact.ctimeMs&&st.ino===fact.ino)continue;
   try{if(createHash("sha256").update(fs.readFileSync(path)).digest("hex")!==fact.sha256)stale.push([kind,path,"sha",new Date(fact.mtimeMs).toISOString(),new Date(st.mtimeMs).toISOString()]);}catch{stale.push([kind,path,"unreadable"]);}
  }
 }
 out[f]={n:h.observations.length,kinds,stale:stale.slice(0,12),nstale:stale.length};
}
console.log(JSON.stringify(out,null,1));
