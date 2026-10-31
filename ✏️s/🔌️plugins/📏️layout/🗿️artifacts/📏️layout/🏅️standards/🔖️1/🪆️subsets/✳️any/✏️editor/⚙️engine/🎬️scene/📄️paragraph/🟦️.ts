/** 🎚️ Admitted authored style indices lend canonical font metadata without copying story or family content. */
import type {ParagraphStyle as LayoutParagraphStyle,CharacterStyle} from "../../../../🧬️schema/🟦️.ts";
export interface LayoutGlyphStyle {family:string;weight:number;italic:boolean}
export interface LayoutStyleSource {paragraphStyles:readonly Pick<LayoutParagraphStyle,"fontFamily"|"fontWeight">[];characterStyles:readonly Pick<CharacterStyle,"fontFamily"|"fontWeight"|"italic">[]}
export function layoutGlyphStyle(source:LayoutStyleSource,paragraph:number|null,character:number|null):LayoutGlyphStyle {const p=paragraph===null?null:source.paragraphStyles[paragraph],c=character===null?null:source.characterStyles[character];if(paragraph!==null&&!p||character!==null&&!c)throw Error("Layout authored style index is unavailable");return{family:c?.fontFamily??p?.fontFamily??"Layout Sans",weight:c?.fontWeight??p?.fontWeight??400,italic:c?.italic??false};}

import {FontParagraphJob,type FontParagraph,type ParagraphStyle,type ParagraphGlyph,type ParagraphLine} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/📄️paragraph/🟦️.ts";
import type {FontFace,FontOutlineLimits} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🟦️.ts";
import type {FontRunSpan} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/🟦️.ts";
import type {UnicodeData} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts";
import type {LineBreakData} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🟦️.ts";
import type {PathSegment} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📐️drawing-contract/🟦️.ts";
import {RetainedWorkBuffer,RetainedWorkLedger,RetainedWorkRetirement,retireRetainedWorkBuffer,retireRetainedWorkBuffers,popRetainedWorkBuffer,retainedWorkChildRetirement,type RetainedWorkRetirementCursor,type RetainedWorkDemand,type RetainedWorkInputSequence} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {TextFlowCursor,type FlowFrameSource,type FlowRegion} from "../🧵️flow/🟦️.ts";
function utf8End(source:Uint8Array,at:number,scalar:number):number{
 const byte=(offset:number,value:number)=>{if(source[at+offset]!==value)throw Error("Layout flow UTF-8 source differs");};
 if(scalar<128){byte(0,scalar);return at+1;}if(scalar<2048){byte(0,0xc0|scalar>>6);byte(1,0x80|scalar&63);return at+2;}if(scalar<65536){byte(0,0xe0|scalar>>12);byte(1,0x80|scalar>>6&63);byte(2,0x80|scalar&63);return at+3;}byte(0,0xf0|scalar>>18);byte(1,0x80|scalar>>12&63);byte(2,0x80|scalar>>6&63);byte(3,0x80|scalar&63);return at+4;
}
function emptyParagraph(c:RetainedWorkProducerStep):FontParagraph|null{if(!c.work.copyBytes(40))return null;const output={glyphs:new RetainedWorkBuffer<ParagraphGlyph>(88),segments:new RetainedWorkBuffer<PathSegment>(64),lines:new RetainedWorkBuffer<ParagraphLine>(80),width:0,height:0};c.commitControlWork();return output;}
type FlowProgress={done:boolean;cancelled:boolean;work:number};
/** 🧵️ One original story analysis feeds funded region cells and authentic child semantic outcomes. */
export class LayoutStoryFlowJob{
 #paragraph:FontParagraphJob|null;#cursor:TextFlowCursor;#regions=new RetainedWorkBuffer<FlowRegion>(128);#producer=new RetainedWorkProducerRuntime();#phase="validate";#at=0;#byte=0;#work=0;#last=0;#semantic=0;#pauseEnd=0;#region:FlowRegion|null=null;#overset=false;#cancelled=false;#transferred=false;#failed=false;#failure:unknown;#output:FontParagraph|null=null;#empty=false;#pending:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};#text:string|null;#utf8:Uint8Array|null;#frames:FlowFrameSource|null;readonly #maximum:number;
 constructor(text:string,utf8:Uint8Array,frames:FlowFrameSource|null,head:number,faces:RetainedWorkInputSequence<FontFace>,style:ParagraphStyle,spans:RetainedWorkInputSequence<FontRunSpan>,limits:FontOutlineLimits,unicode:UnicodeData,line:LineBreakData){if(!Number.isSafeInteger(limits.maxWork)||limits.maxWork<1||limits.maxWork>1e9)throw Error("Invalid Layout story work limit");this.#maximum=limits.maxWork;this.#text=text;this.#utf8=utf8;this.#frames=frames;this.#cursor=new TextFlowCursor(head);this.#paragraph=new FontParagraphJob(text,faces,{...style,width:1},spans,limits,unicode,line);}
 lastTurnWork(){return this.#last;}lastTurnSemanticWork(){return this.#semantic;}nextWorkDemand(){return this.#pending;}failurePending(){return this.#failed;}
 settlementPending(){return this.#phase==="supply"||this.#phase==="feedback"||this.#phase==="finish"||this.#phase==="empty"||this.#phase==="result"||this.#phase==="paragraph"&&this.#paragraph?.settlementPending()===true;}
 progress():FlowProgress{return{done:!this.#cancelled&&this.#phase==="complete",cancelled:this.#cancelled,work:this.#work};}
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#phase){
   case"validate":{const text=this.#text!,bytes=this.#utf8!;if(this.#at===text.length){c.commitSemanticWork();if(this.#byte!==bytes.length)throw Error("Layout flow UTF-8 source differs");this.#phase="region";return;}const scalar=text.codePointAt(this.#at)!;c.commitSemanticWork();if(scalar>=0xd800&&scalar<=0xdfff)throw Error("Layout flow source contains a lone surrogate");this.#byte=utf8End(bytes,this.#byte,scalar);this.#at+=scalar>65535?2:1;return;}
   case"region":{if(!c.slot(this.#regions))return;let next;try{next=this.#cursor.advanceSource(this.#frames!,this.#utf8!,1);}catch(error){c.commitSemanticWork();throw error;}if(next.kind==="region"){c.push(this.#regions,next.region);c.postSemanticWork();this.#region=next.region;this.#phase="supply";return;}c.commitSemanticWork();if(next.kind==="cancelled")throw Error("Layout flow cancelled");if(next.kind==="progress")return;this.#overset=next.kind==="overset";this.#phase=this.#regions.length===0?"empty":"finish";return;}
   case"supply":{const region=this.#region!;try{this.#paragraph!.supplyRegion({width:region.rect.width,height:region.rect.height,origin:[region.rect.x,region.rect.y]});}finally{c.commitControlWork();}this.#region=null;this.#phase="paragraph";return;}
   case"paragraph":{const result=c.advanceChild(this.#paragraph!);if(result.done){this.#phase="result";if(!c.hasActualWork())c.commitControlWork();return;}const pause=this.#paragraph!.flowPause();if(pause){this.#pauseEnd=pause.sourceEnd;this.#phase="feedback";if(!c.hasActualWork())c.commitControlWork();}return;}
   case"feedback":try{this.#cursor.accept(this.#utf8!,this.#pauseEnd);}finally{c.commitControlWork();}this.#phase="region";return;
   case"finish":try{this.#paragraph!.finishFlow();}finally{c.commitControlWork();}this.#phase="paragraph";return;
   case"empty":{const output=emptyParagraph(c);if(output===null)return;this.#output=output;this.#empty=true;this.#phase="complete";return;}
   case"result":this.#output=this.#paragraph!.result();if(!this.#cursor.terminalIsEmpty())this.#cursor.accept(this.#utf8!,this.#utf8!.length);this.#phase="complete";c.commitControlWork();return;
   default:throw Error("Unknown Layout story phase");
  }
 }
 advance(grant:number,work:RetainedWorkLedger):FlowProgress{
  this.#last=0;this.#semantic=0;if(this.#transferred)throw Error("Layout story flow was transferred");if(!Number.isSafeInteger(grant)||grant<0)throw Error("Invalid Layout flow grant");if(grant===0)return this.progress();
  try{return this.#producer.advance(grant,work,{maximumWork:this.#maximum,workLimitMessage:"Layout story flow work exceeded",validate:()=>{if(this.#failed)throw this.#failure;if(this.#phase!=="complete"&&!work.admitsAbsolute(this.#pending))work.requireAbsolute(this.#pending);},stopped:()=>this.#cancelled||this.#phase==="complete",semanticWork:()=>this.#work,recordSemanticWork:n=>{this.#work+=n;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>this.#step(c),progress:()=>this.progress()});}
  catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#last=this.#producer.lastTurnWork();this.#semantic=this.#producer.lastTurnSemanticWork();this.#pending=work.nextDemand;}
 }
 result():FontParagraph{if(this.#transferred||this.#cancelled||this.#failed||this.#phase!=="complete"||this.#output===null)throw Error("Layout story flow is incomplete");return this.#output;}
 glyphRegion(glyph:number):FlowRegion{const record=this.result().glyphs.get(glyph);if(!record)throw Error("Layout flow glyph is unavailable");const placement=this.#paragraph!.linePlacement(record.line),region=this.#regions.get(placement.region);if(!region)throw Error("Layout flow region is unavailable");return region;}
 isOverset(){this.result();return this.#overset;}
 cancel(){if(!this.#transferred){this.#cancelled=true;this.#paragraph?.cancel();this.#cursor.cancel();}}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:FontParagraph|null}{
  if(this.#transferred)throw Error("Layout flow was transferred");const output=!this.#cancelled&&!this.#failed&&this.#phase==="complete"?this.#output:null;this.#transferred=true;
  const child=retainedWorkChildRetirement(this.#paragraph,(value,work,turn)=>output!==null||retireRetainedWorkBuffers(work,turn,value.glyphs,value.segments,value.lines)),empty=this.#empty&&output===null?this.#output:null;this.#paragraph=null;let cleared=false;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{if(!child(work,turn)||!retireRetainedWorkBuffer(this.#regions,work,turn))return false;if(empty&&!retireRetainedWorkBuffers(work,turn,empty.glyphs,empty.segments,empty.lines))return false;if(!cleared){this.#text=null;this.#utf8=null;this.#frames=null;this.#output=null;this.#region=null;cleared=true;turn.commitActualWork();return false;}return true;})};
 }
}
export interface LayoutFlowAddresses extends FlowFrameSource{head(frame:number):number}
export interface LayoutFrameGlyphRange{readonly head:number;readonly source:number|null;readonly start:number;readonly end:number}
type RangeState={head:number;source:number|null;start:number;end:number};
export type LayoutAddressedFlowTurn={kind:"pending"}|{kind:"complete"}|{kind:"glyph";source:number;glyph:number;region:FlowRegion};
const pending=Object.freeze({kind:"pending"}as const),completed=Object.freeze({kind:"complete"}as const);
/** 📇️ Real frame-range cells cache one story without scanning unrelated glyphs during replay. */
export class LayoutAddressedFlowJob{
 #plans=new RetainedWorkBuffer<{head:number;job:LayoutStoryFlowJob}>(24);#rows=new RetainedWorkBuffer<{source:number|null;range:RangeState|null}>(56);#request:{frame:number;head:number;source:number;glyph:number;phase:"prepare"|"sourceLink"|"shape"|"index"|"replay"}|null=null;#moved=false;#producer=new RetainedWorkProducerRuntime();#work=0;#failed=false;#failure:unknown;#event:LayoutAddressedFlowTurn=pending;#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 constructor(private frames:LayoutFlowAddresses|null,private create:((head:number)=>LayoutStoryFlowJob)|null,private readonly maximumWork:number){if(!Number.isSafeInteger(maximumWork)||maximumWork<1||maximumWork>1e9)throw Error("Invalid Layout addressed work limit");}
 begin(frame:number):void{if(this.#moved||this.#request||this.#failed)throw Error("Layout addressed source is active");if(!Number.isSafeInteger(frame)||frame<0||!this.frames?.get(frame))throw Error("Layout addressed frame is unavailable");const head=this.frames.head(frame);if(!Number.isSafeInteger(head)||head<0||!this.frames.get(head))throw Error("Layout addressed thread head is unavailable");this.#request={frame,head,source:0,glyph:0,phase:"prepare"};}
 lastTurnWork(){return this.#producer.lastTurnWork();}lastTurnSemanticWork(){return this.#producer.lastTurnSemanticWork();}nextWorkDemand(){return this.#demand;}failurePending(){return this.#failed;}
 settlementPending(){const r=this.#request;return r!==null&&(r.phase==="sourceLink"||r.phase==="shape"&&this.#plans.get(r.source)?.job.settlementPending()===true);}
 #row(index:number){const value=this.#rows.get(index);if(value===undefined)throw Error("Layout addressed row unavailable");return value;}
 #plan(index:number){const value=this.#plans.get(index);if(value===undefined)throw Error("Layout addressed plan unavailable");return value;}
 advanceOne(work:RetainedWorkLedger):LayoutAddressedFlowTurn{
  this.#event=this.#request===null?completed:pending;
  try{return this.#producer.advance(1,work,{maximumWork:this.maximumWork,workLimitMessage:"Layout addressed work exceeded",validate:()=>{if(this.#moved)throw Error("Layout addressed flow was transferred");if(this.#failed)throw this.#failure;if(this.#request!==null&&!work.admitsAbsolute(this.#demand))work.requireAbsolute(this.#demand);},stopped:()=>this.#request===null||this.#event.kind!=="pending",semanticWork:()=>this.#work,recordSemanticWork:n=>{this.#work+=n;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>this.#step(c),progress:()=>this.#event});}
  catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#demand=work.nextDemand;}
 }
 #step(c:RetainedWorkProducerStep):void{
  const request=this.#request!;
  if(request.phase==="prepare"){
   if(!c.contiguous(this.#rows,this.frames!.length))return;
   if(this.#rows.length<this.frames!.length){if(!c.slot(this.#rows))return;c.push(this.#rows,{source:null,range:null});return;}
   const value=this.#row(request.head);if(value.source===null){if(!c.slot(this.#plans))return;c.requireSemanticWork();let job;try{job=this.create!(request.head);}catch(error){c.commitSemanticWork();throw error;}request.source=this.#plans.length;c.push(this.#plans,{head:request.head,job});request.phase="sourceLink";return;}
   request.source=value.source;request.glyph=this.#row(request.frame).range?.start??0;request.phase="replay";c.commitControlWork();return;
  }
  if(request.phase==="sourceLink"){if(!c.work.copyBytes(8))return;this.#row(request.head).source=request.source;c.work.notePhysicalEffect();request.phase="shape";c.commitControlWork();return;}
  const plan=this.#plan(request.source);
  if(request.phase==="shape"){if(c.advanceChild(plan.job).done){request.phase="index";if(!c.hasActualWork())c.commitControlWork();}return;}
  if(request.phase==="index"){
   const output=plan.job.result();if(request.glyph===output.glyphs.length){request.phase="replay";request.glyph=this.#row(request.frame).range?.start??0;c.commitControlWork();return;}
   const glyph=request.glyph,frame=plan.job.glyphRegion(glyph).frame,value=this.#row(frame);if(!c.work.copyBytes(value.range===null?40:8))return;
   if(value.range)value.range.end=glyph+1;else value.range={head:plan.head,source:request.source,start:glyph,end:glyph+1};c.work.notePhysicalEffect();request.glyph++;c.retainedTurn.commitActualWork();c.postSemanticWork();return;
  }
  const range=this.#row(request.frame).range;if(range===null||request.glyph===range.end){this.#request=null;this.#event=completed;c.commitControlWork();return;}
  const glyph=request.glyph++;this.#event={kind:"glyph",source:request.source,glyph,region:plan.job.glyphRegion(glyph)};c.commitSemanticWork();
 }
 glyph(source:number,glyph:number){const record=this.#plan(source).job.result().glyphs.get(glyph);if(record===undefined)throw Error("Layout addressed glyph is unavailable");return record;}
 range(frame:number):LayoutFrameGlyphRange{if(this.#moved||this.#request!==null)throw Error("Layout addressed frame is incomplete");const head=this.frames?.head(frame);if(head===undefined)throw Error("Layout addressed frame is unavailable");const value=this.#row(frame).range;return value===null?{head,source:this.#row(head).source,start:0,end:0}:Object.freeze(value);}
 intoRetirement():RetainedWorkRetirementCursor{
  if(this.#moved)throw Error("Layout addressed flow was transferred");this.#moved=true;let child:((work:RetainedWorkLedger,turn:import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts").RetainedWorkTurn)=>boolean)|null=null,cleared=false;
  return new RetainedWorkRetirement((work,turn)=>{if(this.#plans.length){if(child===null){child=retainedWorkChildRetirement(this.#plans.at(-1)!.job,(value,work,turn)=>retireRetainedWorkBuffers(work,turn,value.glyphs,value.segments,value.lines));turn.commitActualWork();return false;}if(!child(work,turn)||!popRetainedWorkBuffer(this.#plans,work,turn))return false;child=null;return false;}if(!retireRetainedWorkBuffers(work,turn,this.#plans,this.#rows))return false;if(!cleared){this.#request=null;this.frames=null;this.create=null;this.#event=completed;cleared=true;turn.commitActualWork();return false;}return true;});
 }
}
export interface LayoutLinkLabelSource{readonly links:readonly{readonly artifactKind:string}[]}
/** 🔗️ A fallback label retains genuine empty or shaped pages and its immutable source identity. */
export class LayoutLinkLabelJob{
 #paragraph:FontParagraphJob|null;#text:string|null;#output:FontParagraph|null=null;#phase="supply";#paused=false;#cancelled=false;#moved=false;#failed=false;#failure:unknown;#producer=new RetainedWorkProducerRuntime();#work=0;#last=0;#semantic=0;#empty=false;#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};readonly #maximum:number;
 constructor(private snapshot:LayoutLinkLabelSource|null,private index:number,private bounds:{x:number;y:number;width:number;height:number;rotation:number},faces:RetainedWorkInputSequence<FontFace>,limits:FontOutlineLimits,unicode:UnicodeData,line:LineBreakData){const link=snapshot?.links[index];if(!link)throw Error("Layout label link is missing");if(!Number.isSafeInteger(limits.maxWork)||limits.maxWork<1||limits.maxWork>1e9)throw Error("Invalid Layout label work limit");this.#maximum=limits.maxWork;this.#text=link.artifactKind;const width=Math.max(0,bounds.width-8),height=Math.max(0,bounds.height-8);this.#paragraph=new FontParagraphJob(this.#text,faces,{width:Math.max(1,width),origin:[bounds.x+4,bounds.y+4],size:11,lineHeight:13.2,tracking:0,alignment:"left"},[],limits,unicode,line);if(!width||!height||!this.#text.length)this.#phase="empty";}
 lastTurnWork(){return this.#last;}lastTurnSemanticWork(){return this.#semantic;}failurePending(){return this.#failed;}nextWorkDemand(){return this.#demand;}
 settlementPending(){return this.#phase==="empty"||this.#phase==="result"||this.#phase==="finish"||this.#phase==="paragraph"&&this.#paragraph?.settlementPending()===true;}
 #step(c:RetainedWorkProducerStep):void{switch(this.#phase){case"empty":{const value=emptyParagraph(c);if(value!==null){this.#output=value;this.#empty=true;this.#phase="complete";}return;}case"supply":try{this.#paragraph!.supplyRegion({width:this.bounds.width-8,height:this.bounds.height-8,origin:[this.bounds.x+4,this.bounds.y+4]});}finally{c.commitControlWork();}this.#phase="paragraph";return;case"paragraph":{const p=c.advanceChild(this.#paragraph!);if(p.done){this.#phase="result";if(!c.hasActualWork())c.commitControlWork();}else if(this.#paragraph!.flowPause()&&!this.#paused){this.#phase="finish";if(!c.hasActualWork())c.commitControlWork();}return;}case"finish":try{this.#paragraph!.finishFlow();}finally{c.commitControlWork();}this.#paused=true;this.#phase="paragraph";return;case"result":this.#output=this.#paragraph!.result();this.#phase="complete";c.commitControlWork();return;default:throw Error("Unknown Layout label phase");}}
 advance(snapshot:LayoutLinkLabelSource,grant:number,work:RetainedWorkLedger):FlowProgress{
  this.#last=0;this.#semantic=0;if(this.#moved)throw Error("Layout label was transferred");if(snapshot!==this.snapshot||snapshot.links[this.index]?.artifactKind!==this.#text)throw Error("Layout label source changed");if(!Number.isSafeInteger(grant)||grant<0)throw Error("Layout label grant");if(grant===0)return{done:!this.#cancelled&&this.#phase==="complete",cancelled:this.#cancelled,work:this.#work};
  try{return this.#producer.advance(grant,work,{maximumWork:this.#maximum,workLimitMessage:"Layout label work exceeded",validate:()=>{if(this.#failed)throw this.#failure;if(this.#phase!=="complete"&&!work.admitsAbsolute(this.#demand))work.requireAbsolute(this.#demand);},stopped:()=>this.#cancelled||this.#phase==="complete",semanticWork:()=>this.#work,recordSemanticWork:n=>{this.#work+=n;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>this.#step(c),progress:()=>({done:!this.#cancelled&&this.#phase==="complete",cancelled:this.#cancelled,work:this.#work})});}
  catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#last=this.#producer.lastTurnWork();this.#semantic=this.#producer.lastTurnSemanticWork();this.#demand=work.nextDemand;}
 }
 result(){if(this.#moved||this.#cancelled||this.#failed||this.#output===null)throw Error("Layout label is incomplete");return this.#output;}
 paint(){return{rotation:this.bounds.rotation,center:[this.bounds.x+this.bounds.width/2,this.bounds.y+this.bounds.height/2]as const};}
 cancel(){if(!this.#moved){this.#cancelled=true;this.#paragraph?.cancel();}}
 intoRetirement():{output:FontParagraph|null;job:RetainedWorkRetirementCursor}{
  if(this.#moved)throw Error("Layout label was transferred");this.#moved=true;const output=!this.#cancelled&&!this.#failed?this.#output:null,child=retainedWorkChildRetirement(this.#paragraph,(value,work,turn)=>output!==null||retireRetainedWorkBuffers(work,turn,value.glyphs,value.segments,value.lines)),empty=this.#empty&&output===null?this.#output:null;this.#paragraph=null;let cleared=false;
  return{output,job:new RetainedWorkRetirement((work,turn)=>{if(!child(work,turn))return false;if(empty&&!retireRetainedWorkBuffers(work,turn,empty.glyphs,empty.segments,empty.lines))return false;if(!cleared){this.snapshot=null;this.#text=null;this.#output=null;cleared=true;turn.commitActualWork();return false;}return true;})};
 }
}
