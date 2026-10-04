/** 🔌️ Unmounted admission prototype; storage allocation witnesses belong to the injected owner. */
import type {PackTransportCategory} from "../🟦️.ts";
import type {ValueRefusalKind} from "../🟦️.ts";

export interface CaptureAdmission{maximumBytes:bigint;ownedBytes:bigint;observe(stage:"before"|"after",bytes:bigint):boolean}
export interface CaptureCell{allocatedBytes:bigint;source:Error|null;references:number}
export interface CaptureStorage{reserve(requestedBytes:bigint):CaptureCell|null;release(cell:CaptureCell):void}
export type CaptureResult=Readonly<{kind:null;value:CapturedTransport}>|Readonly<{kind:ValueRefusalKind;source:Error}>;

/** 🪪️ Shared clones preserve one original concrete cause and one owned storage cell. */
export class CapturedTransport{
 #active=true;
 constructor(readonly category:PackTransportCategory,readonly retry:"never"|"transient",private readonly cell:CaptureCell,private readonly storage:CaptureStorage){}
 get cause():Error{if(!this.#active||this.cell.source===null)throw new Error("released transport source");return this.cell.source;}
 get causeKind():Readonly<{kind:"transport";category:PackTransportCategory}>{return{kind:"transport",category:this.category};}
 get allocatedBytes():bigint{return this.cell.allocatedBytes;}
 clone():CapturedTransport{this.cause;this.cell.references++;return new CapturedTransport(this.category,this.retry,this.cell,this.storage);}
 dispose():void{if(!this.#active)return;this.#active=false;if(--this.cell.references===0)this.storage.release(this.cell);}
}

/** 🚦️ Moves the cause only after both observer boundaries and physical-capacity admission succeed. */
export function captureTransport(source:Error,category:PackTransportCategory,retry:"never"|"transient",requestedBytes:bigint,admission:CaptureAdmission,storage:CaptureStorage):CaptureResult{
 const before=admission.ownedBytes,refuse=(kind:ValueRefusalKind):CaptureResult=>({kind,source});
 if(requestedBytes<0n||requestedBytes>(1n<<63n)-1n||requestedBytes>admission.maximumBytes-before)return refuse("ownershipLimit");
 if(!admission.observe("before",requestedBytes))return refuse("canceled");
 const cell=storage.reserve(requestedBytes);
 if(cell===null)return refuse("allocationFailed");
 if(cell.source!==null||cell.references!==0||cell.allocatedBytes<requestedBytes){storage.release(cell);return refuse("invariantViolated");}
 if(cell.allocatedBytes>admission.maximumBytes-before){storage.release(cell);return refuse("ownershipLimit");}
 if(!admission.observe("after",cell.allocatedBytes)){storage.release(cell);return refuse("canceled");}
 cell.source=source;cell.references=1;admission.ownedBytes=before+cell.allocatedBytes;
 return{kind:null,value:new CapturedTransport(category,retry,cell,storage)};
}
