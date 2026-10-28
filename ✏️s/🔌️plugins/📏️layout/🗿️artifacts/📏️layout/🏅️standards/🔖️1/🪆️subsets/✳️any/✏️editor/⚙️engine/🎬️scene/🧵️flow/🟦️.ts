import type {Frame} from '../../../../🧬️schema/🟦️.ts';
import {binary64Value} from '@semio-tech/framework/value/ieee754';
export interface FlowRect {x:number;y:number;width:number;height:number}
export interface FlowBounds extends FlowRect {rotation:number}
export interface FlowFrame {pageIndex:number;frameIndex:number;parent:boolean;storyIndex:number;next:number|null;columns:number;gutter:number;bounds:FlowBounds;inset:FlowRect}
/** 📚️ Immutable scalar rows stay with the original paged metadata owner. Length admits all u32 ordinals. */
export interface FlowFrameSource {readonly length:number;get(index:number):FlowFrame|undefined}
export interface FlowRegion {readonly frame:number;readonly pageIndex:number;readonly frameIndex:number;readonly parent:boolean;readonly storyIndex:number;readonly column:number;readonly start:number;readonly end:number;readonly rect:Readonly<FlowRect>;readonly rotation:number;readonly center:readonly[number,number]}
export type FlowStep={kind:'progress'|'done'|'overset'|'cancelled'}|{kind:'region';region:FlowRegion};
export type FlowFrameContext=Pick<FlowFrame,'pageIndex'|'frameIndex'|'parent'|'storyIndex'|'next'|'gutter'>;
const index=(value:number)=>Number.isInteger(value)&&value>=0&&value<=0xffffffff;
const boundary=(text:Uint8Array,at:number)=>index(at)&&at<=text.length&&(at===text.length||at===0||(text[at]!&0xc0)!==0x80);
function validate(frame:FlowFrame):void{
  if(![frame.pageIndex,frame.frameIndex,frame.storyIndex].every(index)||frame.next!==null&&!index(frame.next)||!index(frame.columns)||frame.columns===0||typeof frame.parent!=='boolean'||![frame.gutter,frame.bounds.x,frame.bounds.y,frame.bounds.width,frame.bounds.height,frame.bounds.rotation,frame.inset.x,frame.inset.y,frame.inset.width,frame.inset.height].every(Number.isFinite)||Math.min(frame.gutter,frame.bounds.width,frame.bounds.height,frame.inset.width,frame.inset.height)<0)throw Error('layout.flow.frame');
}
/** 🧭️ Projects one resolved canonical text frame after its stable story and thread addresses are admitted. */
export function flowFrameFromModel(frame:Frame,context:FlowFrameContext):FlowFrame{
  if(frame.kind!=='text')throw Error('layout.flow.text-frame');
  const b=frame.bounds,i=frame.inset,value:FlowFrame={...context,columns:frame.columns,bounds:{x:binary64Value(b.x),y:binary64Value(b.y),width:binary64Value(b.w),height:binary64Value(b.h),rotation:binary64Value(b.rotation)},inset:{x:binary64Value(i.x),y:binary64Value(i.y),width:binary64Value(i.w),height:binary64Value(i.h)}};
  validate(value);return value;
}
/** 🧵️ Constant-space feedback cursor over immutable resolved frames and an admitted immutable UTF-8 source. */
export class TextFlowCursor {
  private frame:number;private column=0;private columns=0;private nextFrame:number|null=null;private start:number;private entered=false;private visits=0;private story:number|null=null;
  private source:Uint8Array|null=null;private pending:FlowRegion|null=null;private terminal:'done'|'overset'|'cancelled'|null=null;
  constructor(head:number,start=0){if(!index(head)||!index(start))throw Error('layout.flow.address');this.frame=head;this.start=start;}
  private finish(kind:'done'|'overset'|'cancelled'):FlowStep{this.terminal=kind;this.source=null;this.pending=null;return{kind};}
  private next(next:number|null):FlowStep{
    this.column=0;this.entered=false;
    if(next===null)return this.finish('overset');
    this.frame=next;return{kind:'progress'};
  }
  advance(frames:readonly FlowFrame[],text:Uint8Array,grant:number):FlowStep{
    return this.advanceSource({length:frames.length,get:index=>frames[index]},text,grant);
  }
  advanceSource(frames:FlowFrameSource,text:Uint8Array,grant:number):FlowStep{
    if(this.terminal)return{kind:this.terminal};
    if(!index(grant))throw Error('layout.flow.grant');if(grant===0)return{kind:'progress'};
    if(this.source!==null&&this.source!==text)throw Error('layout.flow.stale-source');
    if(!boundary(text,this.start))throw Error('layout.flow.source-boundary');
    if(this.start===text.length)return this.finish('done');
    this.source=text;if(this.pending)return{kind:'region',region:this.pending};
    if(!Number.isInteger(frames.length)||frames.length<0||frames.length>0x100000000)throw Error('layout.flow.source-length');
    const frame=frames.get(this.frame);if(!frame)throw Error('layout.flow.frame-missing');validate(frame);this.columns=frame.columns;this.nextFrame=frame.next;
    if(!this.entered){if(this.visits>=frames.length)throw Error('layout.flow.cycle');if(this.story!==null&&this.story!==frame.storyIndex)throw Error('layout.flow.story');this.story=frame.storyIndex;this.visits++;this.entered=true;}
    const left=Math.max(0,frame.inset.x),top=Math.max(0,frame.inset.y),right=Math.min(frame.bounds.width,frame.inset.x+frame.inset.width),bottom=Math.min(frame.bounds.height,frame.inset.y+frame.inset.height);
    const width=(right-left-frame.gutter*(frame.columns-1))/frame.columns,height=bottom-top;
    if(!Number.isFinite(width)||!Number.isFinite(height))throw Error('layout.flow.geometry');
    if(width<=0||height<=0)return this.next(frame.next);
    const rect={x:frame.bounds.x+left+this.column*(width+frame.gutter),y:frame.bounds.y+top,width,height},center:[number,number]=[frame.bounds.x+frame.bounds.width*0.5,frame.bounds.y+frame.bounds.height*0.5];
    if(![rect.x,rect.y,...center].every(Number.isFinite))throw Error('layout.flow.geometry');
    this.pending=Object.freeze({frame:this.frame,pageIndex:frame.pageIndex,frameIndex:frame.frameIndex,parent:frame.parent,storyIndex:frame.storyIndex,column:this.column,start:this.start,end:text.length,rect:Object.freeze(rect),rotation:frame.bounds.rotation,center:Object.freeze(center)});
    return{kind:'region',region:this.pending};
  }
  accept(text:Uint8Array,end:number):void{
    if(!this.pending||this.source!==text||!boundary(text,end)||end<this.pending.start||end>this.pending.end)throw Error('layout.flow.feedback');
    const region=this.pending;this.pending=null;this.start=end;
    if(end===text.length){this.finish('done');return;}
    this.column=region.column+1;
    if(this.column>=this.columns)this.next(this.nextFrame);
  }
  cancel():void{if(!this.terminal)this.finish('cancelled');}
  terminalIsEmpty():boolean{return this.terminal!==null&&this.source===null&&this.pending===null;}
}
