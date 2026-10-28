import{test}from"bun:test";
import{relative}from"node:path";
import{fileURLToPath}from"node:url";
import{captureCompilerBuildSourceClosureV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🏗️compiler/🟦️.ts";
import{buildSourceClosureStreamLoaderV1 as baseLoader,buildSourceClosureIdentityV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import{runOwnedCommand}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const variant=process.env.VARIANT??"base";
const deepFreeze='const deepFreezeV=value=>{if(value&&typeof value==="object"&&!Object.isFrozen(value)){Object.freeze(value);for(const item of Object.values(value))deepFreezeV(item);}return value;};';
const patch=(text:string)=>{
 if(variant==="base")return text;
 let out=text;
 if(variant==="globals"||variant==="shared"){
  out=out.replace("Bun.plugin({name:'closed-immutable-implementation'",deepFreeze+"globalThis.__semioCaptured=deepFreezeV({physical:JSON.parse(physicalSelectionText),resolutions:JSON.parse(resolutionSelectionText)});Bun.plugin({name:'closed-immutable-implementation'");
  out=out.replace("'import.meta.semioCapturedPhysicalSources':physicalSelectionText","'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCaptured.physical'").replace("'import.meta.semioCapturedSourceResolutions':resolutionSelectionText","'import.meta.semioCapturedSourceResolutions':'globalThis.__semioCaptured.resolutions'");
 }
 if(variant==="shared"){
  out=out.replace("compiler=new Bun.Transpiler({loader,target:'bun',define:{","compiler=(globalThis.__semioTranspilers??={})[loader]??=new Bun.Transpiler({loader,target:'bun',define:{");
  out=out.replace("'import.meta.url':JSON.stringify(url.pathToFileURL(name).href),'import.meta.path':JSON.stringify(name),'import.meta.filename':JSON.stringify(name),'import.meta.dir':JSON.stringify(path.dirname(name)),'import.meta.dirname':JSON.stringify(path.dirname(name)),","'import.meta.url':'__semioMeta$.url','import.meta.path':'__semioMeta$.path','import.meta.filename':'__semioMeta$.path','import.meta.dir':'__semioMeta$.dir','import.meta.dirname':'__semioMeta$.dir',");
  out=out.replace("return{contents:redirect(transformed),loader:'js'};","const prologue='var __semioMeta$=Object.freeze({url:'+JSON.stringify(url.pathToFileURL(name).href)+',path:'+JSON.stringify(name)+',dir:'+JSON.stringify(path.dirname(name))+'});';return{contents:prologue+redirect(transformed),loader:'js'};");
 }
 if(out===text)throw Error("variant did not apply");
 return out;
};
const buildSourceClosureStreamLoaderV1=patch(baseLoader);
test("[DEBUG] loader footprint",()=>withBuildTestInvocationV1(async original=>{
 const workspace=process.cwd(),entry=relative(workspace,fileURLToPath(new URL("./footprint-probe"+(process.env.PROBE?"-"+process.env.PROBE:"")+".ts",import.meta.url))).split("\\").join("/"),control=createIssuedResourceControlV1(original,"test:footprint",120000);
 const started=performance.now(),snapshot=await captureCompilerBuildSourceClosureV1(workspace,entry,original,control),captured=performance.now()-started,text=JSON.stringify(snapshot),sha256=buildSourceClosureIdentityV1(snapshot);
 const out={stdout:"",stderr:""};
 await runOwnedCommand(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",sha256],workspace,"footprint",120000,{control,signal:control.signal,env:{...scriptResourceProcessEnvironmentV1(control,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},auxiliaryInput:{version:1,text},stdout:"ignore",output:{maximumBytes:65536,maximumLines:256},outputConsumer:(bytes,stream)=>{out[stream]+=Buffer.from(bytes).toString("utf8");}});
 console.log("VARIANT "+variant);console.log("CAPTURE "+JSON.stringify({files:snapshot.files.length,snapshotMB:Math.round(Buffer.byteLength(text)/1048576*10)/10,captureMs:Math.round(captured),doorRssMB:Math.round(process.memoryUsage().rss/1048576)}));
 console.log(out.stdout);if(out.stderr)console.log(out.stderr.slice(0,2000));
}),240000);
