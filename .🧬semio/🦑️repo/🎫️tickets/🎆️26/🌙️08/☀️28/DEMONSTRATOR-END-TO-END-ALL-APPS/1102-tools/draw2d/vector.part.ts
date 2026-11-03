function drain(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
type VectorNext="tracing"|"text"|"algorithms"|"complete";
/** 🎬️ Resolves authored algorithms into a complete vector plan while preserving semantic text. */
export class DocumentVectorJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private preparation:DocumentSceneJob|null;private traces:DocumentTraceJob|null=null;private texts:DocumentTextJob|null=null;private algorithms:DocumentBooleanJob|null=null;private progress:DocumentVectorProgress=initialVectorProgress();
 private output:ScenePlanOwner|null=null;private handoff:ScenePlanOwner|null=null;private next:VectorNext="tracing";private closing:RetainedWorkRetirementCursor|null=null;private failure:unknown=null;private aborted=false;private transferred=false;private readonly limits:DocumentAlgorithmLimits;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){if(!Number.isSafeInteger(algorithms.maxWork)||algorithms.maxWork<1||algorithms.maxWork>1e9)fail("Invalid document vector work limit");this.limits={booleans:{...algorithms.booleans},trace:{...algorithms.trace},maxWork:algorithms.maxWork};DocumentBooleanJob.checkLimits(this.limits.booleans);DocumentTraceJob.checkLimits(this.limits.trace);this.preparation=new DocumentSceneJob(document,limits,algorithms.maxWork);}
 private close(job:RetainedWorkRetirementCursor,owner:ScenePlanOwner,next:VectorNext):void{this.closing=job;this.handoff=owner;this.next=next;}
 private step(c:RetainedWorkProducerStep):void{
  if(this.closing!==null){
   let done=false;try{done=this.closing.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing.lastTurnWork());}
   if(!done||c.hasActualWork())return;
   this.closing=null;const owner=this.handoff!;this.handoff=null;
   if(this.next==="tracing")this.traces=new DocumentTraceJob({plan:owner,limits:this.limits.trace});else if(this.next==="text")this.texts=new DocumentTextJob(owner);else if(this.next==="algorithms")this.algorithms=new DocumentBooleanJob({plan:owner,limits:this.limits.booleans});else{this.output=owner;this.progress.done=true;}
   this.progress.phase=this.next;c.commitControlWork();return;
  }
  if(this.progress.phase==="preparing"){this.progress.preparation=c.advanceChild(this.preparation!);if(this.progress.preparation.done){const moved=this.preparation!.intoRetirement();this.preparation=null;this.close(moved.job,moved.output!,"tracing");}return;}
  if(this.progress.phase==="tracing"){this.progress.tracing=c.advanceChild(this.traces!);if(this.progress.tracing.done){const moved=this.traces!.intoRetirement();this.traces=null;this.close(moved.job,moved.output!,"text");}return;}
  if(this.progress.phase==="text"){if(c.advanceChild(this.texts!).done){const moved=this.texts!.intoRetirement();this.texts=null;this.close(moved.job,moved.output!,"algorithms");}return;}
  this.progress.resolution=c.advanceChild(this.algorithms!);if(this.progress.resolution.done){const moved=this.algorithms!.intoRetirement();this.algorithms=null;this.close(moved.job,moved.output!,"complete");}
 }
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 failurePending():boolean{return this.failure!==null||this.preparation?.failurePending()===true||this.traces?.failurePending()===true||this.texts?.failurePending()===true||this.algorithms?.failurePending()===true;}
 settlementPending():boolean{return this.closing!==null||this.progress.done||this.preparation?.settlementPending()===true||this.traces?.settlementPending()===true||this.texts?.settlementPending()===true||this.algorithms?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):DocumentVectorProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.limits.maxWork,workLimitMessage:"Document vector work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancel();if(this.failure!==null)throw this.failure;},stopped:()=>this.progress.done||this.needsFontSources(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.progress.work,recordSemanticWork:n=>{this.progress.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({...this.progress})});}
  catch(error){if(this.failure===null&&!this.aborted)this.failure=error;throw error;}
 }
 needsFontSources():boolean{return this.texts?.needsFontSources()??false;}
 fontSourceFamily():string{return this.texts!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.texts)throw Error("Text source phase inactive");this.texts.admitFontSources(sources,unicodeBytes);}
 result():ScenePlanRead{if(this.aborted||this.transferred)cancel();if(this.failure)throw this.failure;if(!this.progress.done)fail("Document vector incomplete");return this.output!.borrow();}
 cancel():void{this.aborted=true;}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 /** 🧹️ Adopt every child and intermediate source plan before relinquishing vector ownership. */
 intoRetirement():{job:DocumentVectorRetirement;output:ScenePlanOwner|null}{
  if(this.transferred)throw Error("Vector ownership already transferred");this.transferred=true;
  const output=this.progress.done&&!this.aborted&&!this.failure?this.output:null;this.aborted=true;
  const cursors:RetainedWorkRetirementCursor[]=[];if(this.closing)cursors.push(this.closing);this.closing=null;
  const adopt=(moved:{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null})=>{cursors.push(moved.job);if(moved.output)cursors.push(new ScenePlanCloseJob(moved.output));};
  if(this.preparation){adopt(this.preparation.intoRetirement());this.preparation=null;}
  if(this.traces){adopt(this.traces.intoRetirement());this.traces=null;}
  if(this.texts){adopt(this.texts.intoRetirement());this.texts=null;}
  if(this.algorithms){adopt(this.algorithms.intoRetirement());this.algorithms=null;}
  if(this.handoff)cursors.push(new ScenePlanCloseJob(this.handoff));this.handoff=null;
  if(this.output&&this.output!==output)cursors.push(new ScenePlanCloseJob(this.output));this.output=null;
  const job=new RetainedWorkRetirement((work,turn)=>{const cursor=cursors.at(-1);if(cursor){if(!drain(cursor,work,turn))return false;cursors.pop();turn.commitActualWork();return false;}return true;});
  return{job,output};
 }
}
/** 👁️ Borrowed root and asset identities remain explicit for every producer turn. */
export class DocumentVectorPreparationJob{
 private job:DocumentVectorJob;private layers:DrawingSceneSource["layers"];private assets:DrawingSceneSource["assets"];
 constructor(source:DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits){this.layers=source.layers;this.assets=source.assets;this.job=new DocumentVectorJob(source,limits,algorithms);}
 advance(source:DrawingSceneSource,grant:number,work:RetainedWorkLedger):DocumentVectorProgress{if(source.layers!==this.layers||source.assets!==this.assets)fail("Captured scene source ownership changed");return this.job.advance(grant,work);}
 nextWorkDemand():RetainedWorkDemand{return this.job.nextWorkDemand();}
 lastTurnWork():number{return this.job.lastTurnWork();}
 lastTurnSemanticWork():number{return this.job.lastTurnSemanticWork();}
 failurePending():boolean{return this.job.failurePending();}
 settlementPending():boolean{return this.job.settlementPending();}
 needsFontSources():boolean{return this.job.needsFontSources();}
 fontSourceFamily():string{return this.job.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{this.job.admitFontSources(sources,unicodeBytes);}
 cancel():void{this.job.cancel();}
 result():ScenePlanRead{return this.job.result();}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{const moved=this.job.intoRetirement();this.layers=[];this.assets=DrawingAssetsOwner.fromEntriesCold([]);return moved;}
}
