const root="/Users/ueli/Documents/semio",api=await import(root+"/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts");
process.on("uncaughtException",(e:any,origin)=>{console.log("UNCAUGHT",origin,e?.message);});
const orig=AbortSignal.prototype.addEventListener;AbortSignal.prototype.addEventListener=function(this:AbortSignal,type:any,l:any,o:any){if(type==="abort")console.log("LISTEN",String(l).slice(0,120).replace(/\n/g," "),new Error().stack!.split("\n").slice(2,5).join(" | ").slice(0,500));return orig.call(this,type,l,o);} as any;
const thr=AbortSignal.prototype.throwIfAborted;AbortSignal.prototype.throwIfAborted=function(this:AbortSignal){if(this.aborted)console.log("THROW",new Error().stack!.split("\n").slice(2,6).join(" | ").slice(0,600));return thr.call(this);} as any;
const mode=process.argv[2]!;
await api.withScriptProcessEnvelope(api.createScriptProcessEnvelope({version:1,owner:"fixture:idle",maximumElapsedMilliseconds:mode==="finite"?20000:0},{fixture:"idle"},Date.now()),async original=>{
 const control=api.createIssuedResourceControlV1(original,"fixture:idle",mode==="finite"?15000:0);
 try{await api.withAcceptedPhysicalCleanupV1(control,{version:1,kind:"native-image",resource:"fixture:idle",reserveMilliseconds:200},async token=>{await api.beginAcceptedPhysicalCleanupV1(token);await new Promise(d=>setTimeout(d,600));});console.log("no error");}catch(e:any){console.log("caught",e.message);}
});
await new Promise(d=>setTimeout(d,300));console.log("end",mode);
