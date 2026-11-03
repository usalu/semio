import sys
F=sys.argv[1]
W="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
L=open(F).read().split("\n")
def line(n,prefix):
    assert L[n-1].startswith(prefix),(n,L[n-1][:80])
def sub(n,prefix,old,new):
    line(n,prefix);assert L[n-1].count(old)==1,(n,old,L[n-1].count(old));L[n-1]=L[n-1].replace(old,new)
sub(5,"import {PathFlattenJob}","import {PathFlattenJob}","import {PathFlattenJob,type FlatContours}")
line(6,"import {UnitRetirement")
L[5]='import {advanceCursor} from "../🎨️paint/🟦️.ts";\nimport {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";'
sub(14," private vector:DocumentVectorPreparationJob","vectorClose:WorkRetirement|null","vectorClose:RetainedWorkRetirementCursor|null")
sub(14," private vector:DocumentVectorPreparationJob","private retiredFlats:PathFlattenJob[]=[];","private retiredFlats:RetainedWorkRetirementCursor[]=[];private flatOut:FlatContours|null=null;")
sub(16," private area=new Sum()","private transferred=false;","private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
line(20," private step(source:DrawingSceneSource):void{")
v=L[20]
assert v.startswith('  if(this.phase==="vector"){if(this.vector.advance(source,1).done){')
tail=v[len('  if(this.phase==="vector"){if(this.vector.advance(source,1).done){'):]
NEW=r''' private step(c:RetainedWorkProducerStep,source:DrawingSceneSource):void{
  if(this.phase==="vector"){let progress:{done:boolean};try{progress=this.vector.advance(source,1,c.work);}finally{c.retainedTurn.commitActualWork(this.vector.lastTurnWork());}if(c.hasActualWork())c.postSemanticWork();if(progress.done){'''+tail+r'''
  if(this.phase==="paint"){if(c.advanceChild(this.paint!).done){this.scene=this.paint!.result();this.phase="contours";}return;}
  if(this.phase==="union"){if(c.advanceChild(this.union!).done){this.phase="moments";this.segment=0;}return;}
  if(this.phase==="length"&&this.flatten){if(c.advanceChild(this.flatten).done){const moved=this.flatten.intoRetirement();this.retiredFlats.push(moved.job);this.flatOut=moved.output!;this.flatten=null;}return;}
  this.unit();c.commitSemanticWork();
 }
 private unit():void{'''
# lines: 21 vector->NEW (already includes phase vector,paint,union,length child); old 22 paint is L[21]
assert L[21].startswith('  if(this.phase==="paint")'),L[21][:60]
assert L[22].startswith('  if(this.phase==="contours"){'),L[22][:60]
assert L[29].startswith('  if(this.phase==="union")'),L[29][:60]
L[19]=NEW;L[20]=None;L[21]=None;L[29]=None
line(33,"   if(this.flatten){if(!this.flatten.advance(1).done)return;")
L[32]=r'''   if(this.flatOut){const contours=this.flatOut,contour=contours.at(this.flattenContour);if(contour){const count=contour.closed?contour.points.length:Math.max(0,contour.points.length-1);if(this.flattenPoint<count){const a=contour.points.at(this.flattenPoint)!,b=contour.points.at((this.flattenPoint+1)%contour.points.length)!,length=Math.hypot(b[0]-a[0],b[1]-a[1]);this.length.add(length);this.lx.add((a[0]/2+b[0]/2)*length);this.ly.add((a[1]/2+b[1]/2)*length);this.flattenPoint++;}else{this.flattenContour++;this.flattenPoint=0;}return;}this.retiredFlats.push(contours.intoRetirement());this.flatOut=null;this.node++;return;}'''
line(37," advance(source:DrawingSceneSource,grant:number)")
L[36]=r''' lastTurnWork():number{return this.#runtime.lastTurnWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.paint?.failurePending()===true||this.union?.failurePending()===true||this.flatten?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||this.paint?.settlementPending()===true||this.union?.settlementPending()===true||this.flatten?.settlementPending()===true;}
 advance(source:DrawingSceneSource,grant:number,work:RetainedWorkLedger):DrawingScenePlacementProgress{return this.#runtime.advance(grant,work,{maximumWork:this.algorithms.maxWork,workLimitMessage:"Placement work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Scene placement cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c,source);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}'''
line(40," intoRetirement():{job:WorkRetirement;output:DrawingScenePlacement|null}")
s=L[39].replace("{job:WorkRetirement;output:DrawingScenePlacement|null}","{job:RetainedWorkRetirementCursor;output:DrawingScenePlacement|null}",1).replace("children:WorkRetirement[]=[]","children:RetainedWorkRetirementCursor[]=[]",1)
old="if(this.flatten){this.flatten.cancel();children.push(this.flatten.intoRetirement().job);}for(const flat of this.retiredFlats)children.push(flat.intoRetirement().job);"
assert s.count(old)==1
s=s.replace(old,"if(this.flatten){this.flatten.cancel();children.push(this.flatten.intoRetirement().job);}if(this.flatOut){children.push(this.flatOut.intoRetirement());this.flatOut=null;}for(const flat of this.retiredFlats)children.push(flat);")
i=s.index("return{output,job:new UnitRetirement(")
s=s[:i]+r'''return{output,job:new RetainedWorkRetirement((work,turn)=>{const child=children.at(-1);if(advanceCursor(child??null,work,turn))return false;if(!work.canStep())return false;turn.commitActualWork();if(child){children.pop();return false;}this.vectorClose=null;this.paint=null;this.scene=null;this.union=null;this.flatten=null;this.retiredFlats=[];this.operands=[];this.operand={contours:[],fillRule:"nonzero"};this.output=null;this.failure=null;return true;})};}'''
L[39]=s
open(F,"w").write("\n".join(l for l in L if l is not None))
