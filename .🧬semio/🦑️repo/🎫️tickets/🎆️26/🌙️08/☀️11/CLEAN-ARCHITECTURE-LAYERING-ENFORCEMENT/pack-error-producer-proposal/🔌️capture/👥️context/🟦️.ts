/** 👥️ Unmounted ordered interleaving model; native mutex and physical context allocation are not proved here. */
import {CapturedTransport,type CaptureCell,type CaptureStorage} from "../🟦️.ts";
import type {PackTransportCategory,ValueRefusalKind} from "../../🟦️.ts";
import type {ValueError} from "../../🟦️.ts";

export interface ContextProgress{phase:"contextBefore"|"contextAfter"|"reserved"|"before"|"after"|"published"|"released";bytes:bigint;committedBytes:bigint;category:PackTransportCategory|null}
export interface ContextPolicy{checkpoint(progress:ContextProgress):ValueError|null;progress(progress:ContextProgress):void}
export interface ContextStorage{reserve(bytes:bigint):bigint|null;release(committedBytes:bigint):void}
interface ContextState{maximumBytes:bigint;committedBytes:bigint;references:number;canceled:boolean;policy:ContextPolicy;storage:ContextStorage;errors:CaptureStorage}
export type SharedContextResult=Readonly<{kind:null;value:SharedContext}>|Readonly<{kind:ValueRefusalKind;policy:ContextPolicy;error:ValueError|null}>;
export type SharedReservationResult=Readonly<{kind:null;value:SharedReservation}>|Readonly<{kind:ValueRefusalKind;error:ValueError|null}>;

/** 🪪️ Source/sink and published cause handles retain the same admitted ledger. */
export class SharedContext{
 #active=true;
 constructor(private readonly state:ContextState){}
 get committedBytes():bigint{this.requireActive();return this.state.committedBytes;}
 clone():SharedContext{this.requireActive();this.state.references++;return new SharedContext(this.state);}
 cancel():void{this.requireActive();this.state.canceled=true;}
 dispose(notify=true):void{if(!this.#active)return;this.#active=false;try{if(notify)this.notify("released",0n,null);}finally{if(--this.state.references===0)this.state.storage.release(this.state.committedBytes);}}
 notify(phase:ContextProgress["phase"],bytes:bigint,category:PackTransportCategory|null):void{this.state.policy.progress({phase,bytes,committedBytes:this.state.committedBytes,category});}
 rollback(bytes:bigint):void{this.state.committedBytes-=bytes;}
 reserve(requested:bigint,category:PackTransportCategory,retry:"never"|"transient"):SharedReservationResult{
  this.requireActive();const refuse=(kind:ValueRefusalKind,error:ValueError|null=null):SharedReservationResult=>({kind,error});
  const remaining=this.state.maximumBytes-this.state.committedBytes;
  if(requested<0n||requested>(1n<<63n)-1n||requested>remaining)return refuse("ownershipLimit");
  if(this.state.canceled)return refuse("canceled");
  const beforeError=this.state.policy.checkpoint({phase:"before",bytes:requested,committedBytes:this.state.committedBytes,category});
  if(beforeError!==null)return refuse(beforeError.kind,beforeError);
  if(requested>this.state.maximumBytes-this.state.committedBytes)return refuse("ownershipLimit");
  this.state.committedBytes+=requested;
  let charge=requested,cell:CaptureCell|null=null,owner:SharedReservation|null=null,transferred=false;
  try{
   cell=this.state.errors.reserve(requested);if(cell===null)return refuse("allocationFailed");
   if(cell.source!==null||cell.references!==0||cell.allocatedBytes<requested)return refuse("invariantViolated");
   const extra=cell.allocatedBytes-requested;
   if(extra>this.state.maximumBytes-this.state.committedBytes)return refuse("ownershipLimit");
   this.state.committedBytes+=extra;charge=cell.allocatedBytes;
   owner=new SharedReservation(this.clone(),cell,this.state.errors,category,retry);
   const afterError=this.state.canceled?null:this.state.policy.checkpoint({phase:"after",bytes:cell.allocatedBytes,committedBytes:this.state.committedBytes,category});
   if(afterError!==null)return refuse(afterError.kind,afterError);
   if(this.state.canceled)return refuse("canceled");
   this.notify("reserved",cell.allocatedBytes,category);transferred=true;
   return{kind:null,value:owner};
  }finally{
   if(!transferred){if(owner!==null)owner.dispose(false);else{try{if(cell!==null)this.state.errors.release(cell);}finally{this.rollback(charge);}}}
  }
 }
 private requireActive():void{if(!this.#active)throw new Error("shared context handle was released");}
}

/** 🧾️ Unused concurrent reserves refund their own charge rather than restoring an obsolete whole-ledger snapshot. */
export class SharedReservation{
 #active=true;
 constructor(private readonly context:SharedContext,private readonly cell:CaptureCell,private readonly storage:CaptureStorage,readonly category:PackTransportCategory,readonly retry:"never"|"transient"){}
 publish(source:Error):CapturedTransport{
  if(!this.#active)throw new Error("shared reservation was consumed");
  this.#active=false;this.cell.source=source;this.cell.references=1;
  let notifyRelease=true;
  const captured=new CapturedTransport(this.category,this.retry,this.cell,{reserve:()=>{throw new Error("published cause cannot reserve again");},release:cell=>{try{this.storage.release(cell);}finally{this.context.dispose(notifyRelease);}}});
  try{this.context.notify("published",this.cell.allocatedBytes,this.category);return captured;}catch(error){notifyRelease=false;captured.dispose();throw error;}
 }
 dispose(notify=true):void{if(!this.#active)return;this.#active=false;try{this.storage.release(this.cell);}finally{this.context.rollback(this.cell.allocatedBytes);try{if(notify)this.context.notify("released",this.cell.allocatedBytes,null);}finally{this.context.dispose(notify);}}}
}

/** 🧱️ The caller supplies finite credit, physical storage and an owned progress/cancellation policy explicitly. */
export function createSharedContext(maximumBytes:bigint,committedBytes:bigint,requestedBytes:bigint,policy:ContextPolicy,storage:ContextStorage,errors:CaptureStorage):SharedContextResult{
 const refuse=(kind:ValueRefusalKind,error:ValueError|null=null):SharedContextResult=>({kind,policy,error});
 if(requestedBytes<0n||requestedBytes>(1n<<63n)-1n||requestedBytes>maximumBytes-committedBytes)return refuse("ownershipLimit");
 const beforeError=policy.checkpoint({phase:"contextBefore",bytes:requestedBytes,committedBytes,category:null});
 if(beforeError!==null)return refuse(beforeError.kind,beforeError);
 const actual=storage.reserve(requestedBytes);if(actual===null)return refuse("allocationFailed");
 let value:SharedContext|null=null,transferred=false;
 try{
  if(actual<requestedBytes)return refuse("invariantViolated");
  if(actual>maximumBytes-committedBytes)return refuse("ownershipLimit");
  const afterError=policy.checkpoint({phase:"contextAfter",bytes:actual,committedBytes,category:null});
  if(afterError!==null)return refuse(afterError.kind,afterError);
  value=new SharedContext({maximumBytes,committedBytes:committedBytes+actual,references:1,canceled:false,policy,storage,errors});
  value.notify("reserved",actual,null);transferred=true;return{kind:null,value};
 }finally{if(!transferred){if(value!==null)value.dispose(false);else storage.release(committedBytes);}}
}
