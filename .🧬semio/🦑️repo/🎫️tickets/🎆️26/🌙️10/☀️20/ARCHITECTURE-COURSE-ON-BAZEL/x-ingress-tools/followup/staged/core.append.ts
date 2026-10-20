
/** 🛡️ Contains a replay that cannot even capture: only cancellation throws, any other error is a deferral so the complete walk decides. */
export async function guardGenerationReplayV1(ports:Readonly<{run:()=>Promise<GenerationReplayResultV1>;aborted:()=>boolean}>):Promise<GenerationReplayResultV1>{
 try{return await ports.run();}
 catch(error){if(ports.aborted())throw error;return Object.freeze({status:"deferred",reason:"capture-failed",path:String((error as Error)?.message??error).slice(0,200)});}
}
/** ⏮️ Is the start time that bounds the settle rule of a walk: the earliest of the shared walk's own start and the current pass. */
export const generationWalkOriginV1=(sharedStartedAtMs:number|undefined,passStartedAtMs:number):number=>sharedStartedAtMs===undefined?passStartedAtMs:Math.min(sharedStartedAtMs,passStartedAtMs);
