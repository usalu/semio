import{test}from"bun:test";
import{mkdirSync,writeFileSync,existsSync,symlinkSync}from"node:fs";
import{join}from"node:path";
const F=process.env.H87_F??"/Users/ueli/Documents/semio/🧰️framework/";
const storage=await import(F+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");
const readContext=await import(F+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/📥️read-context/🟦️.ts");
const issued=await import(F+"🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const leases=await import(F+"🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts");
const {withBuildTestInvocationV1}=await import(F+"🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts");
const N=Number(process.env.H87_ENTRIES??"20000");
const root=process.env.H87_ROOT??"/private/tmp/h87root/a";const prefix="a".repeat(20);
const mark=(m:string)=>require("node:fs").appendFileSync("/private/tmp/h87root/log/marks.txt",new Date().toISOString()+" "+m+"\n");mark("module loaded");
test("bench",()=>withBuildTestInvocationV1(async original=>{
 mark("test body");const dir=join(root,"j",prefix);
 if(!existsSync(join(dir,"package.json"))){
  mkdirSync(join(dir,"node_modules"),{recursive:true});writeFileSync(join(dir,"package.json"),"{}");
  let n=0;for(let d=0;n<N;d++){const p=join(dir,"node_modules","pkg"+d,"lib");mkdirSync(p,{recursive:true});n+=2;for(let f=0;f<40&&n<N;f++,n++)writeFileSync(join(p,"f"+f+".js"),"x".repeat(300+f*50));}
  console.log("fixture built",n);
 }
 const stop=new AbortController(),control=issued.createIssuedResourceControlV1(original,"dependencies:payload:bench",0,stop.signal);
 let checks=0,checkMs=0;const check=()=>{const c0=performance.now();try{checkBody();}finally{checkMs+=performance.now()-c0;checks++;}};const checkBody=()=>{issued.requireIssuedResourceControlV1(control);leases.checkResourceLeaseControl(control);};
 const context=Object.freeze({root,control,invocation:original,check,recheck:async()=>check()});
 mark("before capture");const write=process.stderr.write;
 const t=performance.now();let r:any;try{r=await storage.readJavascriptDependencySelectedPayloadV1(context,control,prefix,1);}finally{}
 const ms=performance.now()-t;require("node:fs").appendFileSync("/private/tmp/h87root/log/result.txt",[new Date().toISOString(),(process.env.H87_F?"overlay":"live"),"entries",r.entries,"files",r.files,"work",r.work,"ms",Math.round(ms),"checks",checks,"checkMs",Math.round(checkMs),"us-per-entry",Math.round(ms*1000/r.entries),"sha",r.sha256.slice(0,16)].join(" ")+"\n");console.log("RESULT entries",r.entries,"files",r.files,"work",r.work,"ms",Math.round(ms),"per-entry-us",Math.round(ms*1000/r.entries),"sha",r.sha256.slice(0,16));
}),3600000);
