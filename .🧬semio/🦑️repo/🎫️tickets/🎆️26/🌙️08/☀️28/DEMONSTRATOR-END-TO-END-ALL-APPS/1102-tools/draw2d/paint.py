import sys
F=sys.argv[1]
W="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
L=open(F).read().split("\n")
def line(n,prefix):
    assert L[n-1].startswith(prefix),(n,L[n-1][:80])
def sub(n,prefix,old,new,count=1):
    line(n,prefix)
    assert L[n-1].count(old)==count,(n,old,L[n-1].count(old))
    L[n-1]=L[n-1].replace(old,new)
def setl(n,prefix,new):
    line(n,prefix);L[n-1]=new
# imports
sub(2,"import {PathFlattenJob","type FlatContour}","type FlatContour,type FlatContours}")
sub(3,"import {StrokeOutlineJob","type StrokeGeometryStyle}","type StrokeGeometryStyle,type StrokeContours}")
setl(6,"import {UnitRetirement",'import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerChild,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";')
# StrokeCanonicalJob
sub(29," private source:Point[][]","private childRetirement:WorkRetirement|null=null;private retiredChildren:WorkRetirement[]=[];","private childRetirement:RetainedWorkRetirementCursor|null=null;private retiredChildren:RetainedWorkRetirementCursor[]=[];")
sub(29," private source:Point[][]","private transferred=false;","private transferred=false;private failure:unknown=null;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
sub(32," private step():void","private step():void","private unit():void")
line(37,'  case"canonicalizing":if(this.child!.advanceWork(1))');L[36]=None
line(42," advance(grant:number):boolean")
SC_ADV=r''' private step(c:RetainedWorkProducerStep):void{
  if(this.phase==="canonicalizing"){if(c.advanceChild(this.child!).done){const moved=this.child!.intoRetirement();this.child=null;if(this.childRetirement)this.retiredChildren.push(this.childRetirement);this.childRetirement=moved.job;this.segments=moved.output!;this.phase="canonicalCleanup";}return;}
  this.unit();c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.child?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||(this.phase==="canonicalizing"&&this.child?.settlementPending()===true);}
 advance(items:number,work:RetainedWorkLedger):boolean{return this.#runtime.advance(items,work,{maximumWork:this.maxWork,workLimitMessage:"Painted stroke union exceeds work limit",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted stroke union cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.phase==="complete"});}'''
L[41]=SC_ADV
line(44," intoRetirement():{job:WorkRetirement;output:Point[][]|null}")
front=L[43][:L[43].index("let slot=0;")].replace("job:WorkRetirement;","job:RetainedWorkRetirementCursor;")
SC_RET=r'''let slot=0;return{output,job:new RetainedWorkRetirement((work,turn)=>{
  const again=():boolean=>{turn.commitActualWork();return false;};
  if(slot===0&&advanceCursor(this.retiredChildren.at(-1)??this.childRetirement,work,turn))return false;
  if(!work.canStep())return false;
  switch(slot){
   case 0:if(this.retiredChildren.length){this.retiredChildren.pop();return again();}if(this.childRetirement){this.childRetirement=null;return again();}break;
   case 1:if(this.segments.length){this.segments.pop();return again();}break;
   case 2:if(!retireContours(this.source))return again();break;
   case 3:if(!retireStrokeRegions(this.active))return again();break;
   case 4:if(!retireStrokeRegions(this.current))return again();break;
   case 5:if(!retireStrokeRegions(this.next))return again();break;
   case 6:if(!retireContours(this.region.contours))return again();break;
   case 7:if(this.combined.length){this.combined.pop();return again();}break;
   case 8:{const source=this.retiredSources.at(-1);if(source){if(retireContours(source))this.retiredSources.pop();return again();}break;}
   case 9:{const segments=this.retiredSegments.at(-1);if(segments){if(segments.length)segments.pop();else this.retiredSegments.pop();return again();}break;}
   case 10:{const groups=this.retiredGroups.at(-1);if(groups){if(retireStrokeRegions(groups))this.retiredGroups.pop();return again();}break;}
   default:turn.commitActualWork();return true;
  }
  slot++;return again();
 })};}'''
L[43]=front+SC_RET
# close job
CLOSE=r'''export class PaintedRegionsCloseJob implements RetainedWorkRetirementCursor{
 private slot=0;private regions:PaintedPathRegions|null;private cursor:RetainedWorkRetirement;
 constructor(regions:PaintedPathRegions){this.regions=regions;this.cursor=new RetainedWorkRetirement((work,turn)=>{if(!work.canStep())return false;turn.commitActualWork();if(this.slot===2){this.regions=null;return true;}if(retireContours(this.slot===0?this.regions!.fill:this.regions!.stroke))this.slot++;return false;});}
 terminalIsEmpty():boolean{return this.cursor.terminalIsEmpty();}
 advance(grant:number,work:RetainedWorkLedger){return this.cursor.advance(grant,work);}
 nextDemand():RetainedWorkDemand{return this.cursor.nextDemand();}
 lastTurnWork():number{return this.cursor.lastTurnWork();}'''
line(47,"export class PaintedRegionsCloseJob");line(48," private slot=0");line(49," constructor(regions");line(50," terminalIsEmpty");line(51," advance(grant")
L[46]=CLOSE;L[47]=L[48]=L[49]=L[50]=None
# Prepare job fields
sub(56," private input:PaintedPathPreparation","private flat:FlatContour[]=[];","private flat:FlatContours|null=null;private flatCursor:RetainedWorkRetirementCursor|null=null;")
sub(56," private input:PaintedPathPreparation","private polygons:Point[][]=[];","private polygons:StrokeContours|null=null;private polygonCursor:RetainedWorkRetirementCursor|null=null;")
sub(57," private flatten:PathFlattenJob","flattenRetirement:WorkRetirement|null","flattenRetirement:RetainedWorkRetirementCursor|null")
sub(57," private flatten:PathFlattenJob","outlineRetirement:WorkRetirement|null","outlineRetirement:RetainedWorkRetirementCursor|null")
sub(58," private canonical:BooleanJob","canonicalRetirement:WorkRetirement|null","canonicalRetirement:RetainedWorkRetirementCursor|null")
sub(58," private canonical:BooleanJob","strokeCanonicalRetirement:WorkRetirement|null","strokeCanonicalRetirement:RetainedWorkRetirementCursor|null")
sub(59," private phase:PaintedPreparationProgress","private transferred=false;","private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
setl(71," private retireOne():boolean",r'''private retireOne():boolean{if(this.cleanup===12)return true;let done=true;switch(this.cleanup){case 0:this.flattenRetirement=null;break;case 1:this.outlineRetirement=null;break;case 2:this.canonicalRetirement=null;break;case 3:done=retireContours(this.canonicalSource);break;case 4:if(this.canonicalSegments.length){this.canonicalSegments.pop();done=false;}break;case 5:this.flatCursor=null;break;case 6:done=retireContours(this.contours);break;case 7:this.polygonCursor=null;break;case 8:if(this.segments.length){this.segments.pop();done=false;}else this.segments=[];break;case 9:this.style=null;this.input={...this.input,stroke:null};break;case 10:this.strokeCanonicalRetirement=null;break;case 11:this.failure=null;break;}if(done)this.cleanup++;return this.cleanup===12;}''')
L[70]=" "+L[70]
sub(72," private step(source:","private step(source:","private unit(source:")
line(75,'   case"flattening":');L[74]=None
line(79,'   case"stroke":');L[78]=None
line(82,'   case"strokeCanonical":');L[81]=None
line(84,'   case"canonicalizing":');L[83]=None
sub(77,'   case"fill"',"this.flat[this.at]","this.flat!.at(this.at)")
sub(77,'   case"fill"',"c.points[this.point-1]!","c.points.at(this.point-1)!")
sub(78,'   case"contours"',"this.flat[this.at]","this.flat!.at(this.at)")
sub(78,'   case"contours"',"[...c.points[this.point-1]!]","[...c.points.at(this.point-1)!]")
sub(81,'   case"polygons"',"this.polygons[this.at]","this.polygons!.at(this.at)")
sub(81,'   case"polygons"',"p[this.point-1]!","p.at(this.point-1)!")
line(91," advanceWork(grant");line(92," advance(grant")
PREP=r''' private step(c:RetainedWorkProducerStep,source:(index:number)=>PaintedSourceSegment|undefined):void{
  switch(this.phase){
   case"flattening":{if(c.advanceChild(this.flatten!).done){const moved=this.flatten!.intoRetirement();this.flattenRetirement=moved.job;this.flat=moved.output!;this.flatten=null;this.phase="flattenCleanup";}return;}
   case"stroke":{if(c.advanceChild(this.outline!).done){const moved=this.outline!.intoRetirement();this.outlineRetirement=moved.job;this.polygons=moved.output!;this.outline=null;this.phase="strokeCleanup";}return;}
   case"strokeCanonical":{if(c.advanceChild(this.strokeCanonical!)){const moved=this.strokeCanonical!.intoRetirement();this.strokeCanonical=null;this.strokeCanonicalRetirement=moved.job;this.regions!.stroke=moved.output!;this.at=0;this.point=0;this.phase="strokeAccount";}return;}
   case"canonicalizing":{if(c.advanceChild(this.canonical!).done){const moved=this.canonical!.intoRetirement();this.canonical=null;this.canonicalRetirement=moved.job;this.canonicalSegments=moved.output!;this.phase="canonicalCleanup";}return;}
  }
  this.unit(source);c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.flatten?.failurePending()===true||this.outline?.failurePending()===true||this.strokeCanonical?.failurePending()===true||this.canonical?.failurePending()===true;}
 settlementPending():boolean{switch(this.phase){case"complete":return true;case"flattening":return this.flatten?.settlementPending()===true;case"stroke":return this.outline?.settlementPending()===true;case"strokeCanonical":return this.strokeCanonical?.settlementPending()===true;case"canonicalizing":return this.canonical?.settlementPending()===true;default:return false;}}
 advance(items:number,work:RetainedWorkLedger,source:(index:number)=>PaintedSourceSegment|undefined):PaintedPreparationProgress{return this.#runtime.advance(items,work,{maximumWork:this.input.limits.maxWork,workLimitMessage:"Painted preparation exceeds work limit",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted preparation cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c,source);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}
 /** 🪢️ A borrowed segment reader binds one scene node to this producer as a caller-funded child. */
 asChild(source:(index:number)=>PaintedSourceSegment|undefined):RetainedWorkProducerChild<PaintedPreparationProgress>{return{advance:(items,work)=>this.advance(items,work,source),lastTurnWork:()=>this.lastTurnWork(),lastTurnSemanticWork:()=>this.lastTurnSemanticWork(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending()};}'''
L[90]=PREP;L[91]=None
line(95," intoRetirement():{job:WorkRetirement;output:PaintedPathRegions|null}")
s=L[94]
s=s.replace("{job:WorkRetirement;output:PaintedPathRegions|null}","{job:RetainedWorkRetirementCursor;output:PaintedPathRegions|null}",1)
i=s.index("return{output,job:new UnitRetirement(")
RET=r'''return{output,job:new RetainedWorkRetirement((work,turn)=>{if(this.cleanup===5&&this.flat!==null){this.flatCursor=this.flat.intoRetirement();this.flat=null;}if(this.cleanup===7&&this.polygons!==null){this.polygonCursor=this.polygons.intoRetirement();this.polygons=null;}const cursor=[this.flattenRetirement,this.outlineRetirement,this.canonicalRetirement,null,null,this.flatCursor,null,this.polygonCursor,null,null,this.strokeCanonicalRetirement,null,discard][this.cleanup]??null;if(advanceCursor(cursor,work,turn))return false;if(!work.canStep())return false;turn.commitActualWork();if(this.cleanup<12){this.retireOne();return false;}return true;})};}'''
L[94]=s[:i]+RET
# helper before docstring of StrokeCanonicalJob
line(27,"/** 🪵️ Bounded union waves")
HELP='/** ⛓️ A child cursor advances inside the parent turn and posts its own actual work once. */\nexport function advanceCursor(child:RetainedWorkRetirementCursor|null,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{if(child===null||child.terminalIsEmpty())return false;try{child.advance(1,work);}finally{turn.commitActualWork(child.lastTurnWork());}return true;}\n'
L[26]=HELP+L[26]
open(F,"w").write("\n".join(l for l in L if l is not None))
