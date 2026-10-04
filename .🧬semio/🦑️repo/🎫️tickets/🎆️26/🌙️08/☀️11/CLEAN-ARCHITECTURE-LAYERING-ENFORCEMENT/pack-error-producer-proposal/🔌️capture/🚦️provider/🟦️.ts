/** 🚦️ Unmounted provider reservation: admission precedes the operation that can create an external cause. */
import {CapturedTransport,type CaptureAdmission,type CaptureCell,type CaptureStorage} from "../🟦️.ts";
import type {PackTransportCategory,ValueRefusalKind} from "../../🟦️.ts";

export type ReservationResult=Readonly<{kind:null;value:TransportReservation}>|Readonly<{kind:ValueRefusalKind}>;

/** 📦️ Success releases unused capacity; failure transfers the already admitted cell without reserving again. */
export class TransportReservation{
 #active=true;
 constructor(readonly category:PackTransportCategory,readonly retry:"never"|"transient",private readonly cell:CaptureCell,private readonly storage:CaptureStorage,private readonly admission:CaptureAdmission,private readonly before:bigint){}
 publish(source:Error):CapturedTransport{
  if(!this.#active)throw new Error("transport reservation already consumed");
  this.#active=false;this.cell.source=source;this.cell.references=1;
  return new CapturedTransport(this.category,this.retry,this.cell,this.storage);
 }
 dispose():void{if(!this.#active)return;this.#active=false;this.admission.ownedBytes=this.before;this.storage.release(this.cell);}
}

/** 🧱️ Both refusal boundaries complete before IO can produce a concrete error. */
export function reserveTransport(category:PackTransportCategory,retry:"never"|"transient",requestedBytes:bigint,admission:CaptureAdmission,storage:CaptureStorage):ReservationResult{
 const before=admission.ownedBytes,refuse=(kind:ValueRefusalKind):ReservationResult=>({kind});
 if(requestedBytes<0n||requestedBytes>(1n<<63n)-1n||requestedBytes>admission.maximumBytes-before)return refuse("ownershipLimit");
 if(!admission.observe("before",requestedBytes))return refuse("canceled");
 const cell=storage.reserve(requestedBytes);
 if(cell===null)return refuse("allocationFailed");
 if(cell.source!==null||cell.references!==0||cell.allocatedBytes<requestedBytes){storage.release(cell);return refuse("invariantViolated");}
 if(cell.allocatedBytes>admission.maximumBytes-before){storage.release(cell);return refuse("ownershipLimit");}
 if(!admission.observe("after",cell.allocatedBytes)){storage.release(cell);return refuse("canceled");}
 admission.ownedBytes=before+cell.allocatedBytes;
 return{kind:null,value:new TransportReservation(category,retry,cell,storage,admission,before)};
}
