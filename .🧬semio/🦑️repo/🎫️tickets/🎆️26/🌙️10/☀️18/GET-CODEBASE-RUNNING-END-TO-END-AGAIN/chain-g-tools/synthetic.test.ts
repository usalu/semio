import{test}from"bun:test";
import{createHash}from"node:crypto";
import{buildSourceClosureStreamLoaderV1 as liveLoader,buildSourceClosureIdentityV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import{sourceClosureLoaderV1}from"./loader-before.ts";
import{runOwnedCommand}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const digest=(value:string)=>createHash("sha256").update(value).digest("hex");
const loader=process.env.LOADER==="before"?sourceClosureLoaderV1(true):liveLoader;
const modules=Number(process.env.MODULES??600),declared=Number(process.env.DECLARED??60);
test("[DEBUG] synthetic loader footprint",()=>withBuildTestInvocationV1(async original=>{
 const root="/tmp/chain-g-law-root",files:{path:string;sha256:string;base64:string;imports:Record<string,string>}[]=[];
 const entryText='import"./m0.ts";export async function runCapturedBuildSourceV1(args,proof){proof.recheck();const rss=process.memoryUsage().rss;console.log("SYN "+JSON.stringify({rssMB:Math.round(rss/1048576),textOk:proof.text===undefined?null:require("node:crypto").createHash("sha256").update(proof.text()).digest("hex")===proof.sha256}));}';
 const add=(path:string,text:string,imports:Record<string,string>)=>files.push({path,sha256:digest(text),base64:Buffer.from(text).toString("base64"),imports});
 add("entry.ts",entryText,{"./m0.ts":"m0.ts"});
 for(let index=0;index<modules;index++){const imports:Record<string,string>={};if(index+1<modules)imports["./m"+(index+1)+".ts"]="m"+(index+1)+".ts";for(let k=0;k<declared;k++)imports["./declared-import-specifier-number-"+k+".ts"]="m0.ts";add("m"+index+".ts",(index+1<modules?'import"./m'+(index+1)+'.ts";':'')+"export const v"+index+"="+index+";",imports);}
 files.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
 const snapshot={version:1 as const,root,entry:"entry.ts",files},text=JSON.stringify(snapshot),sha256=buildSourceClosureIdentityV1(snapshot),control=createIssuedResourceControlV1(original,"test:syn",90000),observed={stdout:"",stderr:""};
 await runOwnedCommand(process.execPath,["--eval",loader,"--",sha256],process.cwd(),"syn",90000,{control,signal:control.signal,env:{...scriptInvocationEnvironment(original,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},auxiliaryInput:{version:1,text},stdout:"ignore",output:{maximumBytes:65536,maximumLines:256},outputConsumer:(bytes,stream)=>{observed[stream]+=Buffer.from(bytes).toString("utf8");}});
 console.log("LOADER",process.env.LOADER??"live","modules",modules,"snapshotKB",Math.round(text.length/1024));console.log(observed.stdout);if(observed.stderr)console.log(observed.stderr.slice(0,1500));
}),120000);
