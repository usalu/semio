import sys
F=sys.argv[1]
W="../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
L=open(F).read().split("\n")
def line(n,prefix):
    assert L[n-1].startswith(prefix),(n,L[n-1][:80])
def sub(n,prefix,old,new):
    line(n,prefix);assert L[n-1].count(old)==1,(n,old,L[n-1].count(old));L[n-1]=L[n-1].replace(old,new)
def setl(n,prefix,new):
    line(n,prefix);L[n-1]=new
sub(2,"import {PaintedPathPrepareJob","import {PaintedPathPrepareJob,","import {PaintedPathPrepareJob,advanceCursor,")
setl(6,"import {UnitRetirement",'import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";')
sub(19,"export class PreparedSceneCloseJob","implements WorkRetirement","implements RetainedWorkRetirementCursor")
sub(20," private geometry:PreparedGeometry[]","private plan:ScenePlanCloseJob|null;private regions:WorkRetirement|null=null;","private plan:ScenePlanCloseJob|null;private regions:RetainedWorkRetirementCursor|null=null;")
sub(20," private geometry:PreparedGeometry[]","private counter:UnitRetirement;","private cursor:RetainedWorkRetirement;")
sub(21," constructor(scene:PreparedScene","this.counter=new UnitRetirement(()=>this.step());","this.cursor=new RetainedWorkRetirement((work,turn)=>this.step(work,turn));")
setl(22," private step():boolean",r''' private step(work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{
  const again=():boolean=>{turn.commitActualWork();return false;};
  if(advanceCursor(this.regions??(this.phase===0?null:this.plan),work,turn))return false;
  if(!work.canStep())return false;
  if(this.regions){this.regions=null;return again();}
  if(this.fixed){if(this.fixed.length)this.fixed.pop();else this.fixed=null;return again();}
  if(this.phase===0){const g=this.extra??this.geometry.pop();this.extra=null;if(g){if(g.paint.kind==="path")this.regions=new PaintedRegionsCloseJob(g.paint.regions);else if(g.paint.kind!=="empty")this.fixed=g.paint.corners;return again();}this.geometry=[];this.phase=1;return again();}
  if(this.plan){this.plan=null;return again();}
  turn.commitActualWork();return true;
 }''')
setl(23," terminalIsEmpty():boolean"," terminalIsEmpty():boolean{return this.cursor.terminalIsEmpty();}")
setl(24," advance(grant:number)"," advance(grant:number,work:RetainedWorkLedger){return this.cursor.advance(grant,work);}\n nextDemand():RetainedWorkDemand{return this.cursor.nextDemand();}\n lastTurnWork():number{return this.cursor.lastTurnWork();}")
sub(29," private path:PaintedPathPrepareJob","pathRetirement:WorkRetirement|null","pathRetirement:RetainedWorkRetirementCursor|null")
sub(29," private path:PaintedPathPrepareJob","retiredPaths:WorkRetirement[]","retiredPaths:RetainedWorkRetirementCursor[]")
sub(29," private path:PaintedPathPrepareJob","private transferred=false;","private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
sub(33," private step():void{"," private step():void{"," private unit():void{")
line(39,'   case"path":{');L[38]=None
line(44," advance(grant:number):ScenePaintProgress")
L[43]=r''' private step(c:RetainedWorkProducerStep):void{
  if(this.validated&&this.phase==="path"){const node=this.plan!.nodes[this.index]!,content=node.content;if(content.kind!=="path"&&content.kind!=="glyphs")throw new RangeError("Painted scene path owner changed");if(c.advanceChild(this.path!.asChild(index=>content.segments[index])).done){const moved=this.path!.intoRetirement();this.path=null;if(this.pathRetirement)this.retiredPaths.push(this.pathRetirement);this.pathRetirement=moved.job;this.pending={geometryBounds:this.bounds,bounds:moved.output!.bounds,paint:{kind:"path",regions:moved.output!}};this.phase="pathCleanup";}return;}
  this.unit();c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.path?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||(this.phase==="path"&&this.path?.settlementPending()===true);}
 advance(items:number,work:RetainedWorkLedger):ScenePaintProgress{const job=this;return this.#runtime.advance(items,work,{get maximumWork(){return job.validated?job.limits.maxWork:Number.MAX_SAFE_INTEGER;},workLimitMessage:"Painted scene work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted scene cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,...this.counts,done:this.phase==="complete"})});}'''
line(47," intoRetirement():{job:WorkRetirement;output:PreparedScene|null}")
s=L[46].replace("{job:WorkRetirement;output:PreparedScene|null}","{job:RetainedWorkRetirementCursor;output:PreparedScene|null}",1)
i=s.index("return{output,job:new UnitRetirement(")
s=s[:i]+r'''return{output,job:new RetainedWorkRetirement((work,turn)=>{const retired=this.retiredPaths.at(-1);if(advanceCursor(retired??this.pathRetirement??discarded,work,turn))return false;if(!work.canStep())return false;turn.commitActualWork();if(retired){this.retiredPaths.pop();return false;}if(this.pathRetirement){this.pathRetirement=null;return false;}if(discarded){discarded=null;return false;}this.failure=null;return true;})};}'''
L[46]=s
open(F,"w").write("\n".join(l for l in L if l is not None))
