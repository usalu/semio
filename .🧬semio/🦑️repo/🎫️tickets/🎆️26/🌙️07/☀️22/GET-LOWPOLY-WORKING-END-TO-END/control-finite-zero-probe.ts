
/** 🔎️ Observes public envelope admission without changing its protocol or forging issued authority. */
import{resolve}from"node:path";
import{pathToFileURL}from"node:url";
const api=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts")).href);
const data=api.createScriptProcessEnvelope({version:1,owner:"fixture:finite-zero-policy",maximumElapsedMilliseconds:0},{fixture:"finite-zero"},Date.now());
let admitted=false,bodyEntered=false,controlIssued=false,observed;
try{
 const incoming=api.readScriptProcessEnvelope({...data,deadlineEpochMilliseconds:Date.now()+1000},Date.now());
 admitted=true;
 await api.withScriptProcessEnvelope(incoming,async original=>{
  bodyEntered=true;
  const control=api.createIssuedResourceControlV1(original,"finite-zero-policy",0);
  controlIssued=true;observed=control.remainingMilliseconds();
 });
}catch(error){observed=error instanceof Error?error.message:String(error);}
console.log("[DEBUG] "+JSON.stringify({inputPolicy:0,inputFiniteDeadline:true,admitted,bodyEntered,controlIssued,observed}));
