/** 🏷️ Owns a stable collection identity without product semantics. */
export interface Identified<TId> { id():TId; }
/** 🩹️ Owns a caller-defined patch and comparison. */
export interface Patchable<TPatch> { applyPatch(patch:TPatch):void; diffPatch(other:this):TPatch|null; }
/** 🧪️ Shared additive witness reproduces the original collection-law item. */
export class Item implements Identified<string>,Patchable<number> {
 constructor(private identity:string,public value:number){}
 id():string{return this.identity;}
 applyPatch(patch:number):void{this.value+=patch;}
 diffPatch(other:this):number|null{return other.value===this.value?null:other.value-this.value;}
}
