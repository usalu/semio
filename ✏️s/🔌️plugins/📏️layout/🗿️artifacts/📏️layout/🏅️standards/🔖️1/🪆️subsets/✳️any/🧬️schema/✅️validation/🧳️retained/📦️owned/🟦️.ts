import {LayoutValidationCursor,type ValidationGrant,type ValidationDemand} from '../🟦️.ts';
import type {LayoutGraphRead} from '../../👁️graph/🟦️.ts';
import {childAdmissionAdmits,emptyChildAdmissionProgress,type ChildAdmissionProgress} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts';
export interface OwnedLayoutGraphRead extends LayoutGraphRead{beginClose():void;closeDemand():ValidationDemand;closeStep(g:ValidationGrant):ChildAdmissionProgress;terminalIsEmpty():boolean}
const META=256;
const nested=(d:ValidationDemand):ValidationDemand=>({...d,copyBytes:d.copyBytes+META,depth:d.depth+1});
const child=(g:ValidationGrant):ValidationGrant=>({...g,maximumCopyBytes:g.maximumCopyBytes-META,maximumDepth:g.maximumDepth-1});
const paid=(p:ChildAdmissionProgress):ChildAdmissionProgress=>({...p,copiedItems:Math.max(1,p.copiedItems),copiedBytes:p.copiedBytes+META});
/** 📦️ Logical reader envelope custody mirrors native stable Box-before-future and future-before-reader closure. */
export class OwnedLayoutGraphValidation{
 private reader:OwnedLayoutGraphRead|undefined;private bytes=0;private cursor=new LayoutValidationCursor();private closing=false;private readerClosing=false;
 static readerBirthDemand(bytes:number):ValidationDemand{if(!Number.isSafeInteger(bytes)||bytes<1)throw Error('Invalid fixed graph reader header');return{copyBytes:bytes*2+META,capacityBytes:bytes,releaseBytes:0,depth:1}}
 start(reader:OwnedLayoutGraphRead,bytes:number,g:ValidationGrant):ChildAdmissionProgress{const d=OwnedLayoutGraphValidation.readerBirthDemand(bytes);if(this.reader||this.closing||!childAdmissionAdmits(g,d))throw Error('Owned graph reader birth is unfunded');this.reader=reader;this.bytes=bytes;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:d.capacityBytes,releasedBytes:0}}
 demand():ValidationDemand{if(this.closing||!this.reader)throw Error('Owned graph frontier is invalid');return nested(this.cursor.demand(this.reader))}
 advance(g:ValidationGrant){const d=this.demand();if(!childAdmissionAdmits(g,d))return{done:false,error:undefined,progress:emptyChildAdmissionProgress()};const p=this.cursor.advance(this.reader!,child(g));return{done:p.done,error:p.error,progress:paid({copiedItems:p.items,copiedBytes:p.copyBytes,retainedCapacityBytes:p.capacityBytes,releasedBytes:p.releaseBytes})}}
 beginClose(){this.closing=true;this.cursor.beginClose()}
 closeDemand():ValidationDemand{if(!this.cursor.terminalIsEmpty())return nested(this.cursor.closeDemand());if(this.reader){if(!this.readerClosing)return{copyBytes:META,capacityBytes:0,releaseBytes:0,depth:1};if(!this.reader.terminalIsEmpty())return nested(this.reader.closeDemand());return{copyBytes:this.bytes*2+META,capacityBytes:0,releaseBytes:this.bytes,depth:1}}return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0}}
 closeStep(g:ValidationGrant):ChildAdmissionProgress{if(!this.closing)throw Error('Owned graph close has not begun');const d=this.closeDemand();if(this.terminalIsEmpty()||!childAdmissionAdmits(g,d))return emptyChildAdmissionProgress();if(!this.cursor.terminalIsEmpty()){const p=this.cursor.closeStep(child(g));return paid({copiedItems:p.items,copiedBytes:p.copyBytes,retainedCapacityBytes:p.capacityBytes,releasedBytes:p.releaseBytes})}if(!this.readerClosing){this.reader!.beginClose();this.readerClosing=true;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:0}}if(!this.reader!.terminalIsEmpty())return paid(this.reader!.closeStep(child(g)));this.reader=undefined;this.bytes=0;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:d.releaseBytes}}
 terminalIsEmpty(){return this.reader===undefined&&this.cursor.terminalIsEmpty()}
}
