/** 🛡️ Replays a completed walk and contains a replay that cannot capture, so the complete walk decides instead of the command failing. */
export function replayJavascriptDependencyGenerationWalkV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<GenerationReplayResultV1>{
 return guardGenerationReplayV1({run:()=>replayJavascriptDependencyGenerationWalkUnguardedV1(workspace,context,control,checkpoint,walk,startedAtMs,observed,subject),aborted:()=>control.signal.aborted});
}
