
/** 🧫️ Is the answer of an in-process replay: the walk's own recorded observations compared with the live tree, a deferral that proves nothing, or a typed mismatch. */
export type GenerationReplayResultV1=Readonly<{status:"match";sha256:string}>|Readonly<{status:"deferred";reason:string;path?:string}>|Readonly<{status:"mismatch";class:GenerationMismatchClassV1|"seal";path?:string}>;
/** 🔂️ Is how a completed walk was proven against the live tree before the command trusted it. */
export type GenerationConfirmationV1<W>=Readonly<{via:"replay";sha256:string}>|Readonly<{via:"walk";sha256:string;walk:W}>;
/** 🔁️ Confirms a completed walk by replaying its recorded observations and falls back to one more complete walk only when the replay cannot decide. */
export async function confirmGenerationWalkV1<W extends Readonly<{sha256:string}>>(ports:Readonly<{replay:()=>Promise<GenerationReplayResultV1>;walk:()=>Promise<W>}>):Promise<GenerationConfirmationV1<W>>{
 const replayed=await ports.replay();
 if(replayed.status==="match")return Object.freeze({via:"replay",sha256:replayed.sha256});
 const walk=await ports.walk();
 return Object.freeze({via:"walk",sha256:walk.sha256,walk});
}
/** 🕊️ Closes every way one receipt publication ends; only a cancelled run throws. */
export type GenerationPublishOutcomeV1="published"|"present"|"deferred"|"stale"|"failed"|"busy";
/** 🤲️ Runs one publication attempt that must never decide readiness: every failure except cancellation is reported and swallowed. */
export async function publishGenerationBestEffortV1(ports:Readonly<{attempt:()=>Promise<GenerationPublishOutcomeV1>;aborted:()=>boolean;report:(outcome:GenerationPublishOutcomeV1|"error",detail?:string)=>void}>):Promise<GenerationPublishOutcomeV1|"error">{
 try{const outcome=await ports.attempt();ports.report(outcome);return outcome;}
 catch(error){if(ports.aborted())throw error;ports.report("error",String((error as Error)?.message??error).slice(0,300));return "error";}
}
/** 🧷️ Serialises receipt writers among themselves without queueing: a held writer lease is reported as busy, a cancelled run throws. */
export async function runGenerationWriterV1(ports:Readonly<{acquire:()=>Promise<Readonly<{release:()=>void}>>;aborted:()=>boolean;body:()=>Promise<GenerationPublishOutcomeV1>}>):Promise<GenerationPublishOutcomeV1>{
 let lease:Readonly<{release:()=>void}>;
 try{lease=await ports.acquire();}
 catch(error){if(ports.aborted())throw error;if(/^Resource lease timed out/u.test(String((error as Error)?.message)))return "busy";throw error;}
 try{return await ports.body();}
 finally{lease.release();}
}
