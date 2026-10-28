import type{LayoutMutation}from'../../../🟦️.ts';
import type{LayoutSnapshot,Frame}from'../../../../📸️snapshot/🟦️.ts';
import{binary64Value,binary32Value}from'@semio-tech/framework/value/ieee754';
import{LayoutTargetLookup,type LayoutTargetSource}from'../../🔎️target/🟦️.ts';
import type{Grant,Receipt}from'../../../../🔺️diff/🧳️retained/📋️keyed-plan/🟦️.ts';
export interface FrameReceipt extends Receipt{unchanged:boolean}
type Input=Extract<LayoutMutation,{CreateFrame:unknown}|{DeleteFrame:unknown}>;
const payload=(m:Input)=>'CreateFrame'in m?m.CreateFrame:m.DeleteFrame;
const id=(m:Input)=>'CreateFrame'in m?m.CreateFrame.frame.id:m.DeleteFrame.frameId;
const finite=(v:Parameters<typeof binary64Value>[0])=>Number.isFinite(binary64Value(v));
function envelope(f:Frame){const b=f.bounds;if(![b.x,b.y,b.w,b.h,b.rotation].every(finite)||binary64Value(b.w)<=0||binary64Value(b.h)<=0)return false;if(f.kind==='rect')return[f.fill,f.stroke].every(c=>c===null||c.every(v=>Number.isFinite(binary32Value(v))));return f.kind!=='text'||f.columns>0&&[f.inset.x,f.inset.y,f.inset.w,f.inset.h].every(finite)&&binary64Value(f.inset.w)>=0&&binary64Value(f.inset.h)>=0}
/** 🖼️ Frame membership reads bounded original keys and retains numeric graph addresses. */
export class FrameDecision{private cursor=new LayoutTargetLookup();readonly at=[0,0];private next=[0,0];private parent=0;private page=0;private phase=2;private offset=0;private error:string|undefined;private closing=false;private m:Input|undefined;private b:LayoutSnapshot|undefined;
 demand(){return{copyBytes:1024,capacityBytes:0,releaseBytes:0,depth:1}}
 private go(phase:number){this.cursor=new LayoutTargetLookup();this.phase=phase;this.offset=0}
 private lookup(count:number,key:(i:number)=>string,target:string,g:Grant,out:FrameReceipt){const source:LayoutTargetSource={identity:this.phase,count,key},r=this.cursor.advance(source,this.phase,target,{...g,maximumCopyBytes:g.maximumCopyBytes-256});out.copyBytes=r.copyBytes+256;return this.cursor.result()}
 advance(m:Input,b:LayoutSnapshot,g:Grant):FrameReceipt{const out:FrameReceipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:this.phase===15,unchanged:false,...(this.error===undefined?{}:{error:this.error})};if(this.closing)throw Error('frame decision closing');if(this.m!==undefined&&(this.m!==m||this.b!==b))throw Error('frame source changed');if(out.done||g.maximumItems<1||g.maximumCopyBytes<1024||g.maximumDepth<1)return out;this.m=m;this.b=b;out.copyBytes=1024;const create='CreateFrame'in m,p=payload(m),q=create?m.CreateFrame:undefined;let result:ReturnType<LayoutTargetLookup['result']>;switch(this.phase){
 case 2:result=this.lookup(b.pages.length,i=>b.pages[i]!.id,p.pageId,g,out);if(result){if(result.index!==undefined){this.at[0]=result.index;this.go(3)}else out.error='mutation.target-missing'}break;
 case 3:{const rows=b.pages[this.at[0]!]!.frames;result=this.lookup(rows.length,i=>rows[i]!.id,id(m),g,out);if(result){if(create){if(result.index!==undefined)out.error='mutation.duplicate-id';else this.go(4)}else if(result.index!==undefined){this.at[1]=result.index;this.page=0;this.go(14)}else out.error='mutation.target-missing'}break}
 case 4:if(q!.index!==null&&q!.index>b.pages[this.at[0]!]!.frames.length)out.error='mutation.target-missing';else this.go(5);break;
 case 5:{const value=id(m),end=Math.min(value.length,this.offset+64),part=value.slice(this.offset,end);this.offset=end;if(/[^\p{White_Space}]/u.test(part)){this.page=0;this.go(6)}else if(end===value.length)out.error='mutation.invariant';break}
 case 6:if(this.page===b.pages.length)this.go(7);else if(this.page===this.at[0])this.page++;else{const rows=b.pages[this.page]!.frames;result=this.lookup(rows.length,i=>rows[i]!.id,id(m),g,out);if(result){if(result.index!==undefined)out.error='mutation.invariant';else{this.page++;this.go(6)}}}break;
 case 7:{const parent=b.pages[this.at[0]!]!.parentPageId;if(parent===null)this.go(9);else{result=this.lookup(b.parentPages.length,i=>b.parentPages[i]!.id,parent,g,out);if(result){if(result.index===undefined)out.error='mutation.invariant';else{this.parent=result.index;this.go(8)}}}break}
 case 8:{const rows=b.parentPages[this.parent]!.frames;result=this.lookup(rows.length,i=>rows[i]!.id,id(m),g,out);if(result){if(result.index!==undefined)out.error='mutation.invariant';else this.go(9)}break}
 case 9:if(envelope(q!.frame))this.go(10);else out.error='mutation.invariant';break;
 case 10:{const rows=b.pages[this.at[0]!]!.layers;result=this.lookup(rows.length,i=>rows[i]!.id,q!.layerId??q!.frame.layerId,g,out);if(result){if(result.index===undefined)out.error='mutation.invariant';else this.go(11)}break}
 case 11:{const f=q!.frame;if(f.kind==='rect')this.go(15);else{const rows=f.kind==='text'?b.stories:b.links,target=f.kind==='text'?f.storyId:f.linkId;result=this.lookup(rows.length,i=>rows[i]!.id,target,g,out);if(result){if(result.index===undefined)out.error='mutation.invariant';else if(f.kind==='image')this.go(15);else{this.page=0;this.go(12)}}}break}
 case 12:{const f=q!.frame;if(f.kind!=='text')throw Error('frame thread kind');if(f.threadNext===null)this.go(15);else if(this.page===b.pages.length)out.error='mutation.invariant';else{const rows=b.pages[this.page]!.frames;result=this.lookup(rows.length,i=>rows[i]!.id,f.threadNext,g,out);if(result){if(result.index!==undefined){this.next=[this.page,result.index];this.go(13)}else{this.page++;this.go(12)}}}break}
 case 13:{const f=q!.frame,h=b.pages[this.next[0]!]!.frames[this.next[1]!]!;if(f.kind!=='text'||h.kind!=='text'||f.storyId.length!==h.storyId.length)out.error='mutation.invariant';else{const end=Math.min(f.storyId.length,this.offset+64);if(f.storyId.slice(this.offset,end)!==h.storyId.slice(this.offset,end))out.error='mutation.invariant';else if(end===f.storyId.length){this.page=0;this.go(14)}else this.offset=end}break}
 case 14:if(this.page===b.pages.length)this.go(15);else{const f=q?.frame,target=create&&f?.kind==='text'?f.threadNext:id(m);if(target===null||target===undefined)throw Error('frame incoming target');const rows=b.pages[this.page]!.frames;result=this.lookup(rows.length,i=>{const frame=rows[i]!;return!create&&this.page===this.at[0]&&i===this.at[1]?'':frame.kind==='text'?frame.threadNext??'':''},target,g,out);if(result){if(result.index!==undefined)out.error='mutation.invariant';else{this.page++;this.go(14)}}}break;
 default:throw Error('frame decision phase')}
 if(out.error!==undefined){this.error=out.error;this.phase=15}out.done=this.phase===15;return out}
 beginClose(){this.closing=true;this.cursor.beginClose()}
 closeDemand(){return{copyBytes:512,capacityBytes:0,releaseBytes:0,depth:1}}
 close(g:Grant):Receipt{const out:Receipt={copyBytes:0,capacityBytes:0,releaseBytes:0,done:false};if(!this.closing||g.maximumItems<1||g.maximumCopyBytes<512||g.maximumDepth<1)return out;if(!this.cursor.terminalIsEmpty()){const r=this.cursor.close({...g,maximumCopyBytes:g.maximumCopyBytes-256});return{...r,copyBytes:r.copyBytes+256,done:false}}this.m=undefined;this.b=undefined;return{...out,copyBytes:512,done:true}}
 terminalIsEmpty(){return this.closing&&this.m===undefined&&this.b===undefined&&this.cursor.terminalIsEmpty()}
}
