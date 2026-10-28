import {childAdmissionAdmits,emptyChildAdmissionProgress,type ChildAdmissionSource,type ChildAdmissionGrant,type ChildAdmissionDemand,type ChildAdmissionProgress} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🟦️.ts';
interface Held{value:unknown;identity:object;projection:object}
/** 🔗️ Portable pair leases use logical fixed backing; native uses actual original binding storage and release. */
export class FinalGraphSourceBindings{
 private live:Held|undefined;private authored:Held|undefined;private closing=false;
 ready(){return this.live!==undefined&&this.authored!==undefined&&!this.closing}
 private check(source:ChildAdmissionSource,held:Held|undefined){if(held!==undefined&&(source.value!==held.value||source.identity!==held.identity||source.projection!==held.projection))throw Error('Layout final graph original source identity changed')}
 verify(livePost:ChildAdmissionSource,authored:ChildAdmissionSource){this.check(livePost,this.live);this.check(authored,this.authored)}
 demand(livePost:ChildAdmissionSource,authored:ChildAdmissionSource):ChildAdmissionDemand{if(this.closing)throw Error('Layout final graph source bindings are closing');this.verify(livePost,authored);return this.ready()?{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0}:{copyBytes:384,capacityBytes:128,releaseBytes:0,depth:2}}
 advance(livePost:ChildAdmissionSource,authored:ChildAdmissionSource,g:ChildAdmissionGrant):{done:boolean;progress:ChildAdmissionProgress}{const d=this.demand(livePost,authored);if(this.ready())return{done:true,progress:emptyChildAdmissionProgress()};if(!childAdmissionAdmits(g,d))return{done:false,progress:emptyChildAdmissionProgress()};const source=this.live===undefined?livePost:authored,held={value:source.value,identity:source.identity,projection:source.projection};if(this.live===undefined)this.live=held;else this.authored=held;return{done:this.ready(),progress:{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:d.capacityBytes,releasedBytes:0}}}
 beginClose(){this.closing=true}
 closeDemand():ChildAdmissionDemand{return this.terminalIsEmpty()?{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0}:{copyBytes:384,capacityBytes:0,releaseBytes:128,depth:2}}
 closeStep(g:ChildAdmissionGrant):ChildAdmissionProgress{if(!this.closing)throw Error('Layout final graph source close has not begun');const d=this.closeDemand();if(this.terminalIsEmpty()||!childAdmissionAdmits(g,d))return emptyChildAdmissionProgress();if(this.authored!==undefined)this.authored=undefined;else this.live=undefined;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:d.releaseBytes}}
 terminalIsEmpty(){return this.live===undefined&&this.authored===undefined}
}
