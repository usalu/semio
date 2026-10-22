/** 🧩 The segment of a rebuild that the service invocation changes: from the executor holding an issued invocation to the first tool process, using the real authority core of an applied overlay. */
import {spawn} from "node:child_process";
import {overlayPathsV1} from "./overlay-paths.mjs";

/** 📦 Imports the real core modules of an overlay. */
export async function loadCoreV1(overlay){
 const paths=overlayPathsV1(overlay),[service,session,caller,child,routing]=await Promise.all([paths.service,paths.session,paths.caller,paths.child,paths.routing].map(path=>import(path)));
 return{service,session,caller,child,routing};
}

/** 🎟 Issues the invocation of `repo service executor` through the real door and runs `body({held,admission})` beneath it. */
export async function withHeldInvocationV1(core,root,environment,body,closure){
 const capabilities=await core.child.createBuildBootstrapCapabilitiesV1(root,["repo","service","executor"],{...environment,SEMIO_BUILD_MODE:"prototype"});
 const policy=core.routing.readScriptPolicy({version:1,owner:"bench-executor",maximumElapsedMilliseconds:0});
 return core.routing.withScriptProcessEnvelope(core.routing.createScriptProcessEnvelope(policy,capabilities,Date.now()),async held=>{
  const runtime=core.session.endpointsV1(core.session.canonicalRootV1(root),environment);
  const admission=core.service.holdServiceInvocationV1(root,held,{key:core.session.readServiceKeyV1(runtime.key),revocations:core.session.createRevocationRegistryV1(runtime.revoked),audit:core.service.createServiceAuditV1(held,runtime.audit),closure,environment:()=>({...environment,SEMIO_BUILD_MODE:"prototype"})});
  await admission.warm();
  return body({held,admission,runtime});
 });
}

/** ▶ One admitted command: admission, the real launch checks of the repo and of its Nx child, then the stand-in tool. Resolves the timings in milliseconds. */
export async function runAdmittedCommandV1({core,root,held,admission,request,tool,stampDirectory,workMs}){
 const started=performance.now();let admitted=0,checked=0,exit=0;
 await admission.run(request,async ticket=>{
  admitted=performance.now();
  core.caller.readOwnedBuildInvocationV1(root,held,request.args);
  const nx=ticket.launches.find(launch=>launch[0]==="nx");
  core.caller.projectOwnedNxProcessEnvelopeV1(root,held,nx.slice(1));
  checked=performance.now();
  exit=await new Promise(resolve=>{
   const id=request.args[1],owner=id.split(":")[0],out=(id.endsWith(":test")?".out/"+owner+"-test":".out/"+owner);
   const child=spawn(tool,[id,root+"/"+out],{cwd:root,env:{...process.env,FAKE_STAMP_DIR:stampDirectory,FAKE_WORK_MS:String(workMs)},stdio:"ignore"});
   child.on("exit",code=>resolve(code??1));
  });
 });
 return{exit,admitMs:admitted-started,checkMs:checked-admitted};
}
