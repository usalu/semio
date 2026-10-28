import {deepStrictEqual,strictEqual,throws} from 'node:assert/strict';
import corpus from './cases.json' with {type:'json'};
import {TextFlowCursor,flowFrameFromModel,type FlowFrame,type FlowFrameSource,type FlowRegion} from '../🟦️.ts';
import {binary64} from '@semio-tech/framework/value/ieee754';
import type {FrameText} from '../../../../../🧬️schema/🟦️.ts';
const row=(region:FlowRegion)=>[region.frame,region.column,region.start,region.end,region.rect.x,region.rect.y,region.rect.width,region.rect.height];
/** 🧵️ Portable feedback laws preserve UTF-8 offsets, column order, cancellation and terminal overset. */
export function testTextFlow(geometry:(frame:FlowFrame,column:number)=>number[],validate:(frame:unknown)=>boolean):{cases:number;grants:number;refusals:number;cancelled:number;sources:number}{
  const frames=corpus.frames as FlowFrame[];
  for(const frame of frames)strictEqual(validate(frame),true);
  const original=frames[0]!,b=original.bounds,i=original.inset;
  const model:FrameText={kind:'text',id:'text',layerId:'content',locked:null,visible:null,bounds:{x:binary64(b.x),y:binary64(b.y),w:binary64(b.width),h:binary64(b.height),rotation:binary64(b.rotation)},storyId:'story',threadNext:'next',columns:original.columns,inset:{x:binary64(i.x),y:binary64(i.y),w:binary64(i.width),h:binary64(i.height)},wrapMode:'box'};
  deepStrictEqual(flowFrameFromModel(model,{pageIndex:original.pageIndex,frameIndex:original.frameIndex,parent:original.parent,storyIndex:original.storyIndex,next:original.next,gutter:original.gutter}),original);
  for(const vector of corpus.cases)for(const grant of [1,2,64]){
    const text=new TextEncoder().encode(vector.text),cursor=new TextFlowCursor(vector.head),regions:number[][]=[];let status='progress',at=0;
    for(let turn=0;turn<64;turn++){const step=cursor.advance(frames,text,grant);status=step.kind;if(step.kind==='region'){const region=step.region;regions.push(row(region));deepStrictEqual(row(region).slice(4),Array.from(geometry(frames[region.frame]!,region.column)));cursor.accept(text,vector.consumed[at++]!);}else if(step.kind!=='progress')break;}
    deepStrictEqual(regions,vector.regions,vector.name);strictEqual(status,vector.status,vector.name);
  }
  const text=new TextEncoder().encode('Aé🌍Z'),cursor=new TextFlowCursor(0);
  strictEqual(cursor.advance(frames,text,0).kind,'progress');let step=cursor.advance(frames,text,1);strictEqual(step.kind,'region');if(step.kind!=='region')throw Error('region');
  const before=row(step.region);for(const invalid of [-1,2,4,9]){throws(()=>cursor.accept(text,invalid));step=cursor.advance(frames,text,1);if(step.kind!=='region')throw Error('retained region');deepStrictEqual(row(step.region),before);}
  cursor.cancel();strictEqual(cursor.advance(frames,text,0).kind,'cancelled');strictEqual(cursor.terminalIsEmpty(),true);
  const cycle=frames.map(frame=>({...frame,next:0})),cyclic=new TextFlowCursor(1);let refused=false;try{for(let turn=0;turn<32;turn++){const step=cyclic.advance(cycle,text,1);if(step.kind==='region')cyclic.accept(text,0);}}catch{refused=true;}strictEqual(refused,true);
  const wrongStory=frames.map(frame=>({...frame,storyIndex:frame.pageIndex})),wrong=new TextFlowCursor(0);wrong.advance(wrongStory,text,1);wrong.accept(text,0);wrong.advance(wrongStory,text,1);wrong.accept(text,0);throws(()=>wrong.advance(wrongStory,text,1));
  const stale=new TextEncoder().encode('Aé🌍Z'),bound=new TextFlowCursor(1);bound.advance(frames,text,1);throws(()=>bound.accept(stale,1));
  const sourceCase=corpus.sourceCases,pages:readonly (readonly FlowFrame[])[]=Array.from({length:Math.ceil(frames.length/sourceCase.pageSize)},(_,page)=>frames.slice(page*sourceCase.pageSize,(page+1)*sourceCase.pageSize));
  let reads=0;const source:FlowFrameSource={length:frames.length,get(index){reads++;return pages[Math.floor(index/sourceCase.pageSize)]?.[index%sourceCase.pageSize];}};
  for(const vector of corpus.cases){const bytes=new TextEncoder().encode(vector.text),paged=new TextFlowCursor(vector.head),rows:number[][]=[];let status='progress',at=0;for(let turn=0;turn<64;turn++){const step=paged.advanceSource(source,bytes,1);status=step.kind;if(step.kind==='region'){rows.push(row(step.region));const before=reads;paged.advanceSource(source,bytes,1);strictEqual(reads-before,sourceCase.pendingReads);paged.accept(bytes,vector.consumed[at++]!);}else if(step.kind!=='progress')break;}deepStrictEqual(rows,vector.regions);strictEqual(status,vector.status);}
  const sparse:FlowFrameSource={length:sourceCase.maximumLength,get(index){reads++;return index===sourceCase.maximumOrdinal?{...original,next:null}:undefined;}};
  const last=new TextFlowCursor(sourceCase.maximumOrdinal),lastText=new TextEncoder().encode('x');let beforeReads=reads;strictEqual(last.advanceSource(sparse,lastText,0).kind,'progress');strictEqual(reads-beforeReads,sourceCase.zeroGrantReads);const lastStep=last.advanceSource(sparse,lastText,1);strictEqual(lastStep.kind,'region');strictEqual(reads-beforeReads,sourceCase.readsPerRegion);if(lastStep.kind==='region')strictEqual(lastStep.region.frame,sourceCase.maximumOrdinal);
  for(const length of sourceCase.invalidLengths) throws(()=>new TextFlowCursor(0).advanceSource({length,get(){throw Error('must not read invalid source');}},lastText,1),/layout.flow.source-length/);
  return{sources:corpus.cases.length+1+sourceCase.invalidLengths.length,cases:corpus.cases.length,grants:3,refusals:7,cancelled:1};
}
