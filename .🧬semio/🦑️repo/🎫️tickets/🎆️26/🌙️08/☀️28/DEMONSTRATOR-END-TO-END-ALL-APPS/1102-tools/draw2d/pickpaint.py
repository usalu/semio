import sys
F=sys.argv[1]
W="../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
L=open(F).read().split("\n")
def line(n,prefix):
    assert L[n-1].startswith(prefix),(n,L[n-1][:80])
def sub(n,prefix,old,new):
    line(n,prefix);assert L[n-1].count(old)==1,(n,old,L[n-1].count(old));L[n-1]=L[n-1].replace(old,new)
sub(2,"import {PathFlattenJob","type FlatContour}","type FlatContours}")
sub(3,"import {StrokeOutlineJob","type StrokeGeometryStyle}","type StrokeGeometryStyle,type StrokeContours}")
line(4,"import {UnitRetirement")
L[3]='import {advanceCursor} from "../../../🎬️scene/🎨️paint/🟦️.ts";\nimport {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";'
line(16,"function retireContours")
L[15]='type OwnedContour={points:Point[];closed:boolean};\n'+L[15].replace("(FlatContour|Point[])[]","(OwnedContour|Point[])[]")
sub(23," private query:PaintedPathQuery","private flat:FlatContour[]=[];private contours:FlatContour[]=[];private polygons:Point[][]=[];","private flat:FlatContours|null=null;private flatCursor:RetainedWorkRetirementCursor|null=null;private contours:OwnedContour[]=[];private polygons:StrokeContours|null=null;private polygonCursor:RetainedWorkRetirementCursor|null=null;")
sub(24," private flatten:PathFlattenJob","flattenRetirement:WorkRetirement|null","flattenRetirement:RetainedWorkRetirementCursor|null")
sub(24," private flatten:PathFlattenJob","outlineRetirement:WorkRetirement|null","outlineRetirement:RetainedWorkRetirementCursor|null")
sub(25," private phase:PaintedPathProgress","private transferred=false;","private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
line(38," private retireOne():boolean{");line(72," }");line(73,"}")
NEW=r'''  private retireStep(work:RetainedWorkLedger,turn:RetainedWorkTurn):0|1|2{
   if(this.cleanup===8)return 2;
   if(this.cleanup===2&&this.flat!==null){this.flatCursor=this.flat.intoRetirement();this.flat=null;}
   if(this.cleanup===4&&this.polygons!==null){this.polygonCursor=this.polygons.intoRetirement();this.polygons=null;}
   if(advanceCursor(this.cleanup===0?this.flattenRetirement:this.cleanup===1?this.outlineRetirement:this.cleanup===2?this.flatCursor:this.cleanup===4?this.polygonCursor:null,work,turn))return 0;
   if(!work.canStep())return 0;
   turn.commitActualWork();let complete=true;
   switch(this.cleanup){
    case 0:this.flattenRetirement=null;break;
    case 1:this.outlineRetirement=null;break;
    case 2:this.flatCursor=null;break;
    case 3:complete=retireContours(this.contours);break;
    case 4:this.polygonCursor=null;break;
    case 5:if(this.segments.length){this.segments.pop();complete=false;}else this.segments=[];break;
    case 6:this.style=null;this.query={...this.query,stroke:null};break;
    default:this.failure=null;
   }
   if(complete)this.cleanup++;return this.cleanup===8?2:1;
  }
  private unit(source:(index:number)=>PathSegment|undefined):void{
   switch(this.phase){
    case "admitting":{const segment=source(this.next);if(segment){if(this.next>=65536)invalid("Painted query exceeds segment limit");this.segments.push(own(segment));this.next++;}else{this.flatten=new PathFlattenJob({segments:this.segments,transform:this.query.transform,tolerance:this.query.flatness});this.phase="flattening";}break;}
    case "fill":{const c=this.flat!.at(this.at);if(c){if(!this.query.fill||c.points.length<2){this.at++;this.edge=0;}else if(this.edge<c.points.length){const a=map(c.points.at(this.edge)!,this.query.transform),b=map(c.points.at((this.edge+1)%c.points.length)!,this.query.transform);this.edge++;this.line(a,b);}else{this.at++;this.edge=0;}}else{this.at=0;this.edge=0;this.phase="contours";}break;}
    case "contours":{if(!this.style){this.publish();this.phase="cleanup";break;}const c=this.flat!.at(this.at);if(!c){this.outline=new StrokeOutlineJob({contours:this.contours,transform:this.query.transform,tolerance:this.query.flatness,style:this.style});this.phase="stroke";}else if(this.edge===0){this.contours.push({points:[],closed:c.closed});this.edge++;}else if(this.edge<=c.points.length){this.contours.at(-1)!.points.push([...c.points.at(this.edge-1)!]as[number,number]);this.edge++;}else{this.at++;this.edge=0;}break;}
    case "polygons":{const polygon=this.polygons!.at(this.at);if(polygon){if(this.edge<polygon.length){const a=map(polygon.at(this.edge)!,this.query.transform),b=map(polygon.at((this.edge+1)%polygon.length)!,this.query.transform);this.edge++;this.line(a,b);}else{this.strokeHit ||=this.boundary||this.winding!==0;this.winding=0;this.boundary=false;this.at++;this.edge=0;}}else{this.output={contains:this.output!.contains||this.strokeHit,bounds:Number.isFinite(this.bounds[0])?[...this.bounds]:null};this.phase="cleanup";}break;}
    default:throw Error("Unknown painted query phase");
   }
  }
  private step(c:RetainedWorkProducerStep,source:(index:number)=>PathSegment|undefined):void{
   switch(this.phase){
    case "flattening":{if(c.advanceChild(this.flatten!).done){const moved=this.flatten!.intoRetirement();this.flattenRetirement=moved.job;this.flat=moved.output!;this.flatten=null;this.phase="flattenCleanup";}return;}
    case "flattenCleanup":{if(advanceCursor(this.flattenRetirement,c.work,c.retainedTurn))return;this.flattenRetirement=null;this.phase="fill";c.commitControlWork();return;}
    case "stroke":{if(c.advanceChild(this.outline!).done){const moved=this.outline!.intoRetirement();this.outlineRetirement=moved.job;this.polygons=moved.output!;this.outline=null;this.phase="strokeCleanup";}return;}
    case "strokeCleanup":{if(advanceCursor(this.outlineRetirement,c.work,c.retainedTurn))return;this.outlineRetirement=null;this.publish();this.winding=0;this.boundary=false;this.at=0;this.edge=0;this.phase="polygons";c.commitControlWork();return;}
    case "cleanup":{const r=this.retireStep(c.work,c.retainedTurn);if(r===0)return;c.postSemanticWork();if(r===2)this.phase="complete";return;}
   }
   this.unit(source);c.commitSemanticWork();
  }
  lastTurnWork():number{return this.#runtime.lastTurnWork();}
  lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
  nextWorkDemand():RetainedWorkDemand{return this.#demand;}
  failurePending():boolean{return this.failure!==null||this.flatten?.failurePending()===true||this.outline?.failurePending()===true;}
  settlementPending():boolean{return this.phase==="complete"||(this.phase==="flattening"&&this.flatten?.settlementPending()===true)||(this.phase==="stroke"&&this.outline?.settlementPending()===true);}
  advance(items:number,work:RetainedWorkLedger,source:(index:number)=>PathSegment|undefined):PaintedPathProgress{try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Painted query work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted query cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c,source);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}catch(error){if(this.failure===null&&!this.cancelled)this.failure=error;throw error;}}
  result():PaintedPathHit{if(this.cancelled)throw new DOMException("Painted query cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Painted query incomplete");return this.output!;}
  cancel():void{this.cancelled=true;}
  /** 🧹️ Adopts active children and releases its actual private owners under caller grants. */
  intoRetirement():{job:RetainedWorkRetirementCursor;output:PaintedPathHit|null}{
   if(this.transferred)invalid("Painted query ownership already transferred");this.transferred=true;const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;this.cancelled=true;
   if(this.flatten){this.flatten.cancel();this.flattenRetirement=this.flatten.intoRetirement().job;this.flatten=null;}if(this.outline){this.outline.cancel();this.outlineRetirement=this.outline.intoRetirement().job;this.outline=null;}
   return{job:new RetainedWorkRetirement((work,turn)=>{const r=this.retireStep(work,turn);return r===2;}),output};
  }'''
L[37]=NEW
for i in range(38,72): L[i]=None
open(F,"w").write("\n".join(l for l in L if l is not None))
