import sys
F=sys.argv[1]
W="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/"
L=open(F).read().split("\n")
def line(n,prefix):
    assert L[n-1].startswith(prefix),(n,L[n-1][:80])
def sub(n,prefix,old,new):
    line(n,prefix);assert L[n-1].count(old)==1,(n,old,L[n-1].count(old));L[n-1]=L[n-1].replace(old,new)
line(3,"import {UnitRetirement")
L[2]='import type {WorkRetirement} from "@semio-tech/framework/2d/retire";\nimport {advanceCursor} from "../🎨️paint/🟦️.ts";\nimport {OwnedPixelImage} from "'+"../"*12+'🧰️framework/🔨️modules/🔲️pixels/✍️editing/📦️owned/🟦️.ts";\nimport {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor} from "'+W+'🟦️.ts";\nimport {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "'+W+'📈️producer/🟦️.ts";\nimport {DrawingSceneWorkController,driveDrawingSceneWork} from "../🎟️work/🟦️.ts";'
sub(27," private retiredChildren:WorkRetirement[]=[];","private retiredChildren:WorkRetirement[]=[];","private retiredChildren:RetainedWorkRetirementCursor[]=[];private retiredOld:WorkRetirement[]=[];private ownedImages:OwnedPixelImage[]=[];readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};")
sub(36," private painterRetirement","painterRetirement:WorkRetirement|null","painterRetirement:RetainedWorkRetirementCursor|null")
sub(37," private samplerRetirement","samplerRetirement:WorkRetirement|null","samplerRetirement:RetainedWorkRetirementCursor|null")
sub(37," private samplerRetirement","sampled:PixelImage|null","sampled:OwnedPixelImage|null")
sub(111," private step():void","private step():void","private unit():void")
for n,p in [(134,'  else if(this.phase==="path")'),(136,'  else if(this.phase==="image")'),(138,'  else if(this.phase==="compositing")')]:
    line(n,p);L[n-1]=None
sub(137,'  else if(this.phase==="imageCleanup")',"{const image=this.sampled!;this.sampled=null;this.add(image,this.crop);}","{const owned=this.sampled!;this.sampled=null;this.ownedImages.push(owned);this.add(owned.borrow(),this.crop);}")
sub(139,'  else if(this.phase==="compositingCleanup")',"this.retiredChildren.push(this.compositorRetirement!)","this.retiredOld.push(this.compositorRetirement!)")
line(141," advance(budget:number):RasterSceneProgress{")
STEP=r'''  private step(c:RetainedWorkProducerStep):void{
   if(this.phase==="path"){if(c.advanceChild(this.painter!).done){const retired=this.painter!.intoRetirement();this.painterRetirement=retired.job;this.painted=retired.output!;this.painter=null;this.phase="pathCleanup";}return;}
   if(this.phase==="image"){if(c.advanceChild(this.sampler!).done){const retired=this.sampler!.intoRetirement();this.samplerRetirement=retired.job;this.sampled=retired.output!;this.sampler=null;this.phase="imageCleanup";}return;}
   if(this.phase==="compositing"){if(this.compositor!.advance(1).done){const retired=this.compositor!.intoRetirement();this.compositorRetirement=retired.job;this.output=retired.output!;this.compositor=null;this.phase="compositingCleanup";}c.commitSemanticWork();return;}
   this.unit();c.commitSemanticWork();
  }
  lastTurnWork():number{return this.#runtime.lastTurnWork();}
  lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
  nextWorkDemand():RetainedWorkDemand{return this.#demand;}
  failurePending():boolean{return this.failure!==null||this.painter?.failurePending()===true||this.sampler?.failurePending()===true;}
  settlementPending():boolean{return this.phase==="complete"||(this.phase==="path"&&this.painter?.settlementPending()===true)||(this.phase==="image"&&this.sampler?.settlementPending()===true);}
  progress():RasterSceneProgress{return {phase:this.phase,nodes:this.nodes,totalNodes:this.totalNodes,pixels:this.pixels,assets:this.assets,totalAssets:this.totalAssets,sourceBytes:this.sourceBytes,admittedImages:this.admittedImages,work:this.work,done:this.phase==="complete"};}
  advance(items:number,work:RetainedWorkLedger):RasterSceneProgress{
   try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Scene raster work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancelled();if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
   catch(error){if(this.failure===null&&!this.aborted)this.failure=error;throw error;}
  }
  /** 🧹️ Plain composed pixels carry no funded owner; the published image closes in one actual step. */
  retireOutput(_output:PixelImage):RetainedWorkRetirementCursor{let done=false;return new RetainedWorkRetirement((work,turn)=>{if(done)return true;if(!work.canStep())return false;done=true;turn.commitActualWork();return false;});}'''
L[140]=STEP
for n in (142,143,144,145): L[n-1]=None
line(147," intoRetirement():{job:WorkRetirement;output:PixelImage|null}{")
RET=r'''  intoRetirement():{job:RetainedWorkRetirementCursor;output:PixelImage|null}{
   if(this.transferred)throw Error("Raster scene ownership already transferred");this.transferred=true;const children=this.retiredChildren;this.retiredChildren=[];const olds=this.retiredOld;this.retiredOld=[];const images:PixelImage[]=[];const owned:RetainedWorkRetirementCursor[]=[];
   if(this.painter){const moved=this.painter.intoRetirement();children.push(moved.job);if(moved.output)images.push(moved.output);this.painter=null;}
   if(this.sampler){const moved=this.sampler.intoRetirement();children.push(moved.job);if(moved.output)owned.push(moved.output.intoRetirement());this.sampler=null;}
   if(this.compositor){const moved=this.compositor.intoRetirement();olds.push(moved.job);if(moved.output)images.push(moved.output);this.compositor=null;}
   for(const child of [this.painterRetirement,this.samplerRetirement])if(child)children.push(child);this.painterRetirement=null;this.samplerRetirement=null;if(this.compositorRetirement)olds.push(this.compositorRetirement);this.compositorRetirement=null;
   if(this.painted)images.push(this.painted);this.painted=null;if(this.sampled)owned.push(this.sampled.intoRetirement());this.sampled=null;for(const image of this.ownedImages)owned.push(image.intoRetirement());this.ownedImages=[];
   const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.output:null;if(this.output&&!output)images.push(this.output);this.output=null;this.aborted=true;
   let source=this.source,assetSource=this.assetSource,current=this.current,assetCurrent=this.assetCurrent;this.source=[];this.assetSource=[];this.current=null;this.assetCurrent=null;let sourceAt=0,assetAt=0;
   const catalog=this.catalog,admitted=this.admitted,ids=this.ids,closed=this.closed;this.catalog=new Map();this.admitted=new Map();this.ids=new Set();this.closed=new Set();
   const layers=this.root;this.root=[];while(this.stack.length){const scope=this.stack.pop()!;layers.push({kind:"group",children:scope.layers,opacity:scope.group.opacity,blend:scope.group.blendMode,visible:true,transform:identity,mask:null});}
   const imageRecords=this.images;this.images=Object.create(null);const keys=Object.keys(imageRecords);let slot=0;
   const entry=(map:Map<unknown,unknown>|Set<unknown>)=>{const next=map.keys().next();if(!next.done){map.delete(next.value);return false;}return true;};
   return{output,job:new RetainedWorkRetirement((work,turn)=>{
    if(slot===0&&advanceCursor(children.at(-1)??owned.at(-1)??null,work,turn))return false;
    if(!work.canStep())return false;
    turn.commitActualWork();
    switch(slot){
     case 0:if(children.length){children.pop();return false;}if(owned.length){owned.pop();return false;}break;
     case 1:{const child=olds.at(-1);if(child){if(child.advance(1).done)olds.pop();return false;}break;}
     case 2:if(sourceAt<source.length){sourceAt++;return false;}source=[];break;
     case 3:if(assetAt<assetSource.length){assetAt++;return false;}assetSource=[];break;
     case 4:current=null;assetCurrent=null;break;
     case 5:if(!entry(catalog))return false;break;
     case 6:if(!entry(admitted))return false;break;
     case 7:if(!entry(ids))return false;break;
     case 8:if(!entry(closed))return false;break;
     case 9:{const layer=layers.pop();if(layer){if(layer.kind==="group")layers.push(...layer.children);return false;}break;}
     case 10:{const key=keys.pop();if(key!==undefined){delete imageRecords[key];return false;}break;}
     case 11:if(images.pop())return false;break;
     default:return current===null&&assetCurrent===null;
    }
    slot++;return false;
   })};
  }'''
L[146]=RET
for n in range(148,161): L[n-1]=None
# async wrapper
i=[k for k,l in enumerate(L) if l and l.startswith("/** ⏳️ Yields between scene grants")][0]
L[i]='/** ⏳️ Caller-funded turns publish only a fully composed candidate after its owned close. */'
L[i+1]='export async function rasterizeScene(input:RasterSceneInput,control:{work:DrawingSceneWorkController;operation:object;source:object;onProgress?:(progress:RasterSceneProgress)=>void}):Promise<PixelImage>{return driveDrawingSceneWork(control.work,control.operation,control.source,new RasterSceneJob(input),control.onProgress);}'
L=L[:i+2]
open(F,"w").write("\n".join(l for l in L if l is not None))
