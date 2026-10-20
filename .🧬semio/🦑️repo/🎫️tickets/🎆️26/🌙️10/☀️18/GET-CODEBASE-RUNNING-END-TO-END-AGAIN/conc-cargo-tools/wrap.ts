import {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment} from "./🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const files=process.argv.slice(2);
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"conc-cargo-wrap",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
 const env=scriptInvocationEnvironment(invocation,process.env as Record<string,string>);
 const child=Bun.spawn([process.execPath,"test","--timeout","240000",...files],{env,stdout:"inherit",stderr:"inherit",cwd:process.cwd()});
 await child.exited;process.exitCode=child.exitCode??1;
});
