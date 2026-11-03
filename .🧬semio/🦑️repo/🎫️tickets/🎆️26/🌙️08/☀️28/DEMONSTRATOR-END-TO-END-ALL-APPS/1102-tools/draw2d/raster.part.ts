const list=<T,U>(source:{readonly length:number;at(index:number):T|undefined},map:(value:T)=>U):U[]=>Array.from({length:source.length},(_,at)=>{const value=source.at(at);if(value===undefined)fail("Missing scene row");return map(value);});
type PlanPaint=Extract<ScenePlanContent,{kind:"path"}>;
const plainFill=(fill:PlanPaint["fill"]):Fill|null=>fill===null?null:fill.kind==="solid"?{kind:"solid",color:[...fill.color]as Color}:{...fill,stops:list(fill.stops,stop=>({offset:stop.offset,color:[...stop.color]as Color}))}as Fill;
const plainStroke=(stroke:PlanPaint["stroke"]):RasterSceneNode["content"] extends infer C?(C extends {kind:"path"}?C["stroke"]:never):never=>stroke===null?null:{...stroke,color:[...stroke.color]as Color,dash:stroke.dash?list(stroke.dash,value=>value):stroke.dash}as never;
function rasterNode(node:ScenePlanNode):RasterSceneNode{const content=node.content;if(content.kind!=="path"&&content.kind!=="glyphs"&&content.kind!=="image")fail("Unresolved "+content.kind+" layer: "+node.id);return{id:node.id,groups:list(node.groups,group=>group),transform:[...node.transform]as RasterSceneNode["transform"],opacity:node.opacity,blendMode:node.blendMode,visible:node.visible,content:content.kind==="image"?{kind:"image",asset:content.asset,width:content.width,height:content.height}:{kind:"path",segments:list(content.segments,segment=>segment),fillRule:content.fillRule,fill:plainFill(content.fill),stroke:plainStroke(content.stroke)}};}
/** 📷️ Uses the same complete vector producer before per-record raster admission. */
export class DocumentRasterJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private vector:DocumentVectorJob|null;private prepared:DocumentVectorProgress=initialVectorProgress();
 private plan:ScenePlanOwner|null=null;private view:ScenePlanRead|null=null;private assets:RasterSceneAsset[]=[];private nodes:RasterSceneNode[]=[];private at=0;private assetAt=0;private work=0;private phase:DocumentRasterProgress["phase"]="preparing";
 private raster:RasterSceneJob|null=null;private rendered:RasterSceneProgress|null=null;private output:PixelImage|null=null;private failure:unknown=null;private aborted=false;private readonly viewport:DocumentSceneViewport;private readonly maxWork:number;private closing:RetainedWorkRetirementCursor|null=null;private afterClose:"resolving"|"complete"="resolving";private transferred=false;private handoff:ScenePlanOwner|null=null;private rasterClose:RetainedWorkRetirementCursor|null=null;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits){this.viewport={...viewport,origin:[...viewport.origin]};const check=new RasterSceneJob({...this.viewport,assets:[],nodes:[]});check.cancel();this.vector=new DocumentVectorJob(document,limits,resolutionLimits);this.maxWork=resolutionLimits.maxWork;}
 private pixelScale:readonly[number,number]=[1,1];
 /** 🔍️ Maps the immutable artboard world extent to requested output pixel dimensions. */
 setPixelScale(x:number,y:number):void{if(this.work!==0||![x,y].every(n=>Number.isFinite(n)&&n>0))fail("Invalid document pixel scale");this.pixelScale=[x,y];}
 needsFontSources():boolean{return this.vector?.needsFontSources()??false;}
 fontSourceFamily():string{return this.vector!.fontSourceFamily();}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.vector)throw Error("Text source phase inactive");this.vector.admitFontSources(sources,unicodeBytes);}
 private step(c:RetainedWorkProducerStep):void{
  if(this.closing!==null){let done=false;try{done=this.closing.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing.lastTurnWork());}if(!done||c.hasActualWork())return;this.closing=null;if(this.afterClose==="resolving"){this.plan=this.handoff;this.handoff=null;this.view=this.plan!.borrow();this.phase="resolving";}else this.phase="complete";c.commitControlWork();return;}
  if(this.vector){this.prepared=c.advanceChild(this.vector);if(this.prepared.done){const moved=this.vector.intoRetirement();this.vector=null;this.handoff=moved.output!;this.closing=moved.job;this.afterClose="resolving";}else this.phase=this.prepared.phase;return;}
  if(this.phase==="resolving"){const view=this.view!;if(this.assetAt<view.assets.length){const asset=view.assets.at(this.assetAt++)!;this.assets.push(asset);c.commitSemanticWork();return;}const n=view.nodes.at(this.at++);if(!n){this.raster=new RasterSceneJob({...this.viewport,assets:this.assets,nodes:this.nodes});this.phase="raster";c.commitSemanticWork();return;}if(n.content.kind!=="group"){const node=rasterNode(n),m=node.transform,[x,y]=this.pixelScale;this.nodes.push({...node,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y]});}c.commitSemanticWork();return;}
  if(this.phase==="raster"){this.rendered=c.advanceChild(this.raster!);if(this.rendered.done){const moved=this.raster!.intoRetirement();this.output=moved.output;this.raster=null;this.rasterClose=moved.job;this.phase="rasterClose";}return;}
  if(this.phase==="rasterClose"){let done=false;try{done=this.rasterClose!.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.rasterClose!.lastTurnWork());}if(!done||c.hasActualWork())return;this.rasterClose=null;this.view=null;this.closing=new ScenePlanCloseJob(this.plan!);this.plan=null;this.afterClose="complete";c.commitControlWork();return;}
 }
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 failurePending():boolean{return this.failure!==null||this.vector?.failurePending()===true||this.raster?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||this.closing!==null||this.phase==="rasterClose"||this.vector?.settlementPending()===true||this.raster?.settlementPending()===true;}
 advance(items:number,work:RetainedWorkLedger):DocumentRasterProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.maxWork,workLimitMessage:"Document raster work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancel();if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete"||this.needsFontSources(),settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase==="rasterClose"?"raster":this.phase,preparation:this.prepared.preparation,tracing:this.prepared.tracing,resolution:this.prepared.resolution,raster:this.rendered,nodes:Math.min(this.at,this.prepared.preparation.layers),work:this.work,done:this.phase==="complete"})});}
  catch(error){if(this.failure===null&&!this.aborted)this.failure=error;throw error;}
 }
 /** 🧹️ Composes the actual preparation, raster, remaining plan and unpublished pixel owners. */
 intoRetirement():{job:RetainedWorkRetirementCursor;output:PixelImage|null}{
  if(this.transferred)throw Error("Document raster ownership already transferred");this.transferred=true;const cursors:RetainedWorkRetirementCursor[]=[];const images:PixelImage[]=[];
  if(this.closing)cursors.push(this.closing);this.closing=null;if(this.rasterClose)cursors.push(this.rasterClose);this.rasterClose=null;
  if(this.vector){const moved=this.vector.intoRetirement();cursors.push(moved.job);if(moved.output)cursors.push(new ScenePlanCloseJob(moved.output));this.vector=null;}
  if(this.raster){const moved=this.raster.intoRetirement();cursors.push(moved.job);if(moved.output)images.push(moved.output);this.raster=null;}
  if(this.handoff)cursors.push(new ScenePlanCloseJob(this.handoff));this.handoff=null;if(this.plan)cursors.push(new ScenePlanCloseJob(this.plan));this.plan=null;this.view=null;
  let assets=this.assets,nodes=this.nodes;this.assets=[];this.nodes=[];
  const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.output:null;if(this.output&&!output)images.push(this.output);this.output=null;this.aborted=true;let slot=0;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{
   if(slot===0){const cursor=cursors.at(-1);if(cursor){if(!drain(cursor,work,turn))return false;cursors.pop();turn.commitActualWork();return false;}}
   if(!work.canStep())return false;turn.commitActualWork();
   switch(slot){case 0:break;case 1:if(assets.length){assets.pop();return false;}assets=[];break;case 2:if(nodes.pop())return false;nodes=[];break;case 3:if(images.pop())return false;break;default:return true;}
   slot++;return false;
  })};
 }
 cancel():void{this.aborted=true;}
 retireOutput(_output:PixelImage):RetainedWorkRetirementCursor{let done=false;return new RetainedWorkRetirement((work,turn)=>{if(done)return true;if(!work.canStep())return false;done=true;turn.commitActualWork();return false;});}
 result():PixelImage{if(this.aborted||this.transferred)cancel();if(this.failure)throw this.failure;if(this.phase!=="complete")fail("Document raster incomplete");return this.output!;}
}
/** ⏳️ Publishes document pixels only after responsive preparation, raster completion and owned close. */
export async function rasterizeDocument(document:DrawingArtifact,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolutionLimits:DocumentAlgorithmLimits,options:DocumentRasterOptions):Promise<PixelImage>{
 options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();return completeSceneProducer(new DocumentRasterJob(document,limits,viewport,resolutionLimits),document,options);
}
export {DocumentSceneJob,prepareDocumentScene} from "./🎟️owned/🟦️.ts";
export type {DocumentSceneLimits,DocumentSceneProgress,DocumentSceneOptions} from "./🎟️owned/🟦️.ts";
/** 🚪️ Hands prepared paths/images to raster; unresolved work is an explicit refusal. */
export function resolvedSceneInput(plan:ScenePlanRead,input:Omit<RasterSceneInput,"assets"|"nodes">):RasterSceneInput{
 const nodes:RasterSceneNode[]=[];for(let at=0;at<plan.nodes.length;at++){const n=plan.nodes.at(at)!;if(n.content.kind==="group")continue;nodes.push(rasterNode(n));}return{...input,assets:list(plan.assets,asset=>asset),nodes};
}
export {sceneSelectionRelation,type SceneSelectionRelation} from "./🧭️selection/🟦️.ts";
function fail(message:string):never{throw new drawingDrawingArtifactGuardRefusal("$scene",message);}
const cancel=():never=>{throw new DOMException("Document scene preparation cancelled","AbortError");};
