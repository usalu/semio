import type{LayoutRect,TextStyleRun}from'../../../📸️snapshot/🟦️.ts';
import type{PageGuidesDelta,TextStyleRunsDelta}from'../../../🔺️diff/🟦️.ts';
import type{Grant,Receipt}from'../../../🔺️diff/🧳️retained/📋️keyed-plan/🟦️.ts';
type Row=LayoutRect|TextStyleRun;type Kind='guides'|'runs';type Out={index:number;guide:LayoutRect}|{index:number;run:TextStyleRun};
const emptyGuide=():LayoutRect=>({x:{bits:0n},y:{bits:0n},w:{bits:0n},h:{bits:0n}}),emptyRun=():TextStyleRun=>({start:0n,end:0n,paragraphStyleId:null,characterStyleId:null});
const sameGuide=(q:LayoutRect,h:LayoutRect|undefined)=>h!==undefined&&q.x.bits===h.x.bits&&q.y.bits===h.y.bits&&q.w.bits===h.w.bits&&q.h.bits===h.h.bits;
/** 📍️ Original positional rows move individually; skipped strings remain under the admitted source owner. */
export class RetainedPositionalOutput{private source:Row[]|undefined;private base:readonly Row[]|undefined;private rows:Out[]|undefined;private extent:number;private index=0;private offset=0;private equal=true;private phase=0;private closing=false;
 constructor(private kind:Kind,source:Row[],base:readonly Row[]){this.source=source;this.base=base;this.extent=source.length}
 demand(){return{copyBytes:512,capacityBytes:this.phase===0?this.extent*(this.kind==='guides'?40:64):0,releaseBytes:0,depth:1}}
 advance(g:Grant):Receipt{if(this.closing||this.phase===5)throw Error('positional output unavailable');const d=this.demand(),r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<1)return r;r.copyBytes=d.copyBytes;r.capacityBytes=d.capacityBytes;const source=this.source!,base=this.base!;switch(this.phase){case 0:this.rows=[];this.phase=1;break;case 1:if(this.index===source.length){this.phase=5;r.done=true;break}if(this.kind==='guides'){this.equal=sameGuide(source[this.index]as LayoutRect,base[this.index]as LayoutRect|undefined);this.phase=4}else{const q=source[this.index]as TextStyleRun,h=base[this.index]as TextStyleRun|undefined;this.equal=h!==undefined&&q.start===h.start&&q.end===h.end;this.phase=2}break;case 2:case 3:{const q=source[this.index]as TextStyleRun,h=base[this.index]as TextStyleRun|undefined,value=this.phase===2?q.paragraphStyleId:q.characterStyleId,held=(this.phase===2?h?.paragraphStyleId:h?.characterStyleId)??null;let done=true;if(value!==null&&held!==null&&value.length===held.length){const end=Math.min(value.length,this.offset+64);this.equal&&=value.slice(this.offset,end)===held.slice(this.offset,end);this.offset=end;done=end===value.length}else if(value!==held)this.equal=false;if(done){this.offset=0;this.phase++}break}case 4:if(!this.equal){const row=source[this.index]!;if(this.kind==='guides'){this.rows!.push({index:this.index,guide:row as LayoutRect});source[this.index]=emptyGuide()}else{this.rows!.push({index:this.index,run:row as TextStyleRun});source[this.index]=emptyRun()}}this.index++;this.phase=1;break}return r}
 take():PageGuidesDelta|TextStyleRunsDelta{if(this.phase!==5||this.closing||!this.rows)throw Error('positional output not ready');const rows=this.rows;this.rows=undefined;return{len:this.extent===this.base!.length?null:this.extent,rows}as PageGuidesDelta|TextStyleRunsDelta}
 beginClose(){this.closing=true}close(g:Grant):Receipt{const r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return r;r.copyBytes=512;if(this.rows?.length){this.rows.pop();return r}if(this.source?.length){this.source.pop();return r}this.rows=undefined;this.source=undefined;this.base=undefined;r.done=true;return r}terminalIsEmpty(){return this.closing&&this.rows===undefined&&this.source===undefined&&this.base===undefined}}

import type{LayoutMutation}from'../../🟦️.ts';
import type{Frame,LayoutSnapshot}from'../../../📸️snapshot/🟦️.ts';
import type{LayoutDiff,PagePatch}from'../../../🔺️diff/🟦️.ts';
const emptyDiff=():LayoutDiff=>({schema:null,name:null,grid:null,paragraphStyles:null,characterStyles:null,stories:null,links:null,parentPages:null,spreads:null,pages:null,printTarget:null,dataFields:null});
const emptyPagePatch=():PagePatch=>({name:null,width:null,height:null,margin_top:null,margin_right:null,margin_bottom:null,margin_left:null,columns_count:null,columns_gutter:null,overrides:{removed:[],inserted:[],moved:[]},frames:{inserted:[],removed:[],moved:[],modified:[]},layers:{inserted:[],removed:[],moved:[],modified:[]}});
type FrameInput=Extract<LayoutMutation,{CreateFrame:unknown}|{DeleteFrame:unknown}>;
const vacantFrame=():Frame=>({kind:'rect',id:'',layerId:'',bounds:{x:{bits:0n},y:{bits:0n},w:{bits:0n},h:{bits:0n},rotation:{bits:0n}},locked:null,visible:null,fill:null,stroke:null});
/** 🖼️ The prepared Frame moves once while an overridden layer string stays in the original payload owner. */
export class RetainedFrameOutput{private diff:LayoutDiff|undefined;private phase=0;private closing=false;
 constructor(private source:FrameInput|undefined,private base:LayoutSnapshot|undefined,private at:readonly number[]){}
 demand(){return{copyBytes:1024,capacityBytes:this.phase===0?256:0,releaseBytes:0,depth:1}}
 advance(g:Grant):Receipt{if(this.closing||this.phase>1)throw Error('frame output unavailable');const d=this.demand(),r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<1)return r;r.copyBytes=d.copyBytes;r.capacityBytes=d.capacityBytes;if(this.phase===0){this.diff=emptyDiff();this.diff.pages={inserted:[],removed:[],moved:[],modified:[]};this.phase=1;return r}const m=this.source!,page=this.base!.pages[this.at[0]!]!;if('CreateFrame'in m){const p=m.CreateFrame,row=p.frame;if(p.layerId!==null){const old=row.layerId;row.layerId=p.layerId;p.layerId=old}p.frame=vacantFrame();const patch=emptyPagePatch();patch.frames.inserted.push({index:p.index??page.frames.length,row});this.diff!.pages!.modified.push({id:p.pageId,patch});p.pageId=''}else{const p=m.DeleteFrame;const patch=emptyPagePatch();patch.frames.removed.push({id:p.frameId,index:this.at[1]!});this.diff!.pages!.modified.push({id:p.pageId,patch});p.pageId='';p.frameId=''}this.phase=2;r.done=true;return r}
 take(g:Grant){if(this.phase!==2||this.closing)throw Error('frame output not ready');if(g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return undefined;const d=this.diff;this.diff=undefined;this.phase=3;return d}
 beginClose(){this.closing=true}
 close(g:Grant):Receipt{const r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<1024||g.maximumDepth<1)return r;r.copyBytes=1024;if(this.diff!==undefined){this.diff=undefined;return r}this.source=undefined;this.base=undefined;r.done=true;return r}
 terminalIsEmpty(){return this.closing&&this.diff===undefined&&this.source===undefined&&this.base===undefined}
}

type TextFrameInput=Extract<LayoutMutation,{UpdateTextFrame:unknown}>;
/** 📑️ Text threading metadata and exact inset words transfer from the original prepared command. */
export class RetainedTextFrameOutput{private diff:LayoutDiff|undefined;private phase=0;private closing=false;
 constructor(private source:TextFrameInput|undefined){}
 demand(){return{copyBytes:1024,capacityBytes:this.phase===0?256:0,releaseBytes:0,depth:1}}
 advance(g:Grant):Receipt{if(this.closing||this.phase>1)throw Error('text-frame output unavailable');const d=this.demand(),r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<1)return r;r.copyBytes=d.copyBytes;r.capacityBytes=d.capacityBytes;if(this.phase===0){this.diff=emptyDiff();this.diff.pages={inserted:[],removed:[],moved:[],modified:[]};this.phase=1;return r}const p=this.source!.UpdateTextFrame,patch=emptyPagePatch();patch.frames.modified.push({id:p.frameId,patch:{x:null,y:null,width:null,height:null,wrap_mode:null,columns:null,story_id:p.storyId,thread_next:p.threadNext,inset_x:p.insetX,inset_y:p.insetY,inset_width:p.insetWidth,inset_height:p.insetHeight}});this.diff!.pages!.modified.push({id:p.pageId,patch});p.pageId='';p.frameId='';p.storyId='';p.threadNext=null;this.phase=2;r.done=true;return r}
 take(g:Grant){if(this.phase!==2||this.closing)throw Error('text-frame output not ready');if(g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return undefined;const d=this.diff;this.diff=undefined;this.phase=3;return d}
 beginClose(){this.closing=true}
 close(g:Grant):Receipt{const r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<1024||g.maximumDepth<1)return r;r.copyBytes=1024;if(this.diff!==undefined){this.diff=undefined;return r}this.source=undefined;r.done=true;return r}
 terminalIsEmpty(){return this.closing&&this.diff===undefined&&this.source===undefined}
}

type ParentRemovalInput=Extract<LayoutMutation,{DeleteParentPage:unknown}>;
/** 🗄️ A sparse parent removal moves the prepared original identifier with its original ordinal. */
export class RetainedParentRemoval{private diff:LayoutDiff|undefined;private phase=0;private closing=false;
 constructor(private source:ParentRemovalInput|undefined,private at:number){}
 demand(){return{copyBytes:512,capacityBytes:this.phase===0?128:0,releaseBytes:0,depth:1}}
 advance(g:Grant):Receipt{if(this.closing||this.phase>1)throw Error('parent removal unavailable');const d=this.demand(),r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<1)return r;r.copyBytes=d.copyBytes;r.capacityBytes=d.capacityBytes;if(this.phase===0){this.diff=emptyDiff();this.diff.parentPages={inserted:[],removed:[],moved:[],modified:[]};this.phase=1;return r}const p=this.source!.DeleteParentPage;this.diff!.parentPages!.removed.push({id:p.id,index:this.at});p.id='';this.phase=2;r.done=true;return r}
 take(g:Grant){if(this.phase!==2||this.closing)throw Error('parent removal not ready');if(g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return undefined;const d=this.diff;this.diff=undefined;this.phase=3;return d}
 beginClose(){this.closing=true}
 close(g:Grant):Receipt{const r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return r;r.copyBytes=512;if(this.diff!==undefined){this.diff=undefined;return r}this.source=undefined;r.done=true;return r}
 terminalIsEmpty(){return this.closing&&this.diff===undefined&&this.source===undefined}
}

type BackgroundInput=Extract<LayoutMutation,{SetPageBackgroundDrawing:unknown}>;
/** 🖊️ Original prepared nullable member custody moves into one sparse page setter. */
export class RetainedBackgroundOutput{private diff:LayoutDiff|undefined;private phase=0;private closing=false;
 constructor(private source:BackgroundInput|undefined){}
 demand(){return{copyBytes:512,capacityBytes:this.phase===0?128:0,releaseBytes:0,depth:1}}
 advance(g:Grant):Receipt{if(this.closing||this.phase>1)throw Error('background output unavailable');const d=this.demand(),r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<1)return r;r.copyBytes=d.copyBytes;r.capacityBytes=d.capacityBytes;if(this.phase===0){this.diff=emptyDiff();this.diff.pages={inserted:[],removed:[],moved:[],modified:[]};this.phase=1;return r}const p=this.source!.SetPageBackgroundDrawing,patch=emptyPagePatch();patch.background_drawing=p.drawing;this.diff!.pages!.modified.push({id:p.pageId,patch});p.pageId='';p.drawing=null;this.phase=2;r.done=true;return r}
 take(g:Grant){if(this.phase!==2||this.closing)throw Error('background output not ready');if(g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return undefined;const d=this.diff;this.diff=undefined;this.phase=3;return d}
 beginClose(){this.closing=true}
 close(g:Grant):Receipt{const r:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return r;r.copyBytes=512;if(this.diff!==undefined){this.diff=undefined;return r}this.source=undefined;r.done=true;return r}
 terminalIsEmpty(){return this.closing&&this.diff===undefined&&this.source===undefined}
}
