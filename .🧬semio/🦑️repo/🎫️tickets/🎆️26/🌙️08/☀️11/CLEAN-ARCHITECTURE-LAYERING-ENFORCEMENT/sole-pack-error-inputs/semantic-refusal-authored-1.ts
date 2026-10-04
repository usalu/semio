/** 🪶️ A semantic container refusal has no external transport cause. */
import {ValueError,type ValueRefusalKind} from "../../../🌱️value/⚠️refusal/🟦️.ts";
import {TextError} from "../../../⚠️diagnostic/🚧️text-error/🟦️.ts";
import type {PackTransportCategory,PackRetryDisposition,TransportCaptureRefusal} from "../🔌️capture/🟦️.ts";

export type PagedRefusalMetadata=Readonly<{kind:"ownershipLimit"|"allocationFailed"|"invariantViolated";reason:string}>;
export type PagedAllocationMetadata=PagedRefusalMetadata&Readonly<{allocatedBytes:number}>;
export type PackRefusalData=
 | Readonly<{kind:"BadMagic"|"ContentHashMismatch"}>
 | Readonly<{kind:"UnsupportedVersion";major:number;minor:number}>
 | Readonly<{kind:"UnknownRequiredFlags";flags:number}>
 | Readonly<{kind:"Truncated";offset:bigint}>
 | Readonly<{kind:"ChecksumMismatch";segment:string;offset:bigint}>
 | Readonly<{kind:"NonCanonical";detail:string}>
 | Readonly<{kind:"UnsupportedCodec";codec:number}>
 | Readonly<{kind:"LimitExceeded";refusalKind:ValueRefusalKind;limit:string}>
 | Readonly<{kind:"RetainedMalformed"|"Malformed";refusalKind:ValueRefusalKind;what:string;offset:bigint;detail:string}>
 | Readonly<{kind:"RetainedAllocation";refusalKind:ValueRefusalKind;allocatedBytes:number;what:string;offset:bigint;detail:string}>
 | Readonly<{kind:"ValueRefusal";error:ValueError}>
 | Readonly<{kind:"TextRefusal";error:TextError}>
 | Readonly<{kind:"Io";error:ValueError;retry:PackRetryDisposition}>
 | Readonly<{kind:"TransportAdmission";category:PackTransportCategory;refusal:TransportCaptureRefusal}>;

/** 🗨️ Presentation never determines semantic authority. */
export function packRefusalMessage(data:PackRefusalData):string{
 switch(data.kind){
  case "BadMagic":return "bad magic";
  case "ContentHashMismatch":return "content hash mismatch";
  case "UnsupportedVersion":return `unsupported version ${data.major}.${data.minor}`;
  case "UnknownRequiredFlags":return `unknown required feature bits 0x${data.flags.toString(16)}`;
  case "Truncated":return `truncated at offset ${data.offset}`;
  case "ChecksumMismatch":return `checksum mismatch in ${data.segment} at offset ${data.offset}`;
  case "NonCanonical":return `non-canonical encoding: ${data.detail}`;
  case "UnsupportedCodec":return `unsupported codec ${data.codec}`;
  case "LimitExceeded":return `limit exceeded: ${data.limit}`;
  case "RetainedMalformed":case "RetainedAllocation":case "Malformed":return `malformed ${data.what} at offset ${data.offset}: ${data.detail}`;
  case "ValueRefusal":return `schema error: ${data.error.message}`;
  case "TextRefusal":return `schema error: ${data.error.toString()}`;
  case "Io":return `io error: ${data.error.message}`;
  case "TransportAdmission":return `transport admission: ${data.refusal.message}`;
 }
}

/** 🪪️ Owned typed causes retain their exact kind, message and source. */
export class PackRefusal extends Error{
 static fromPagedRefusal(error:PagedRefusalMetadata,what:string,offset:bigint):PackRefusal{return new PackRefusal({kind:"RetainedMalformed",refusalKind:error.kind,what,offset,detail:error.reason});}
 static fromPagedAllocation(error:PagedAllocationMetadata,what:string,offset:bigint):PackRefusal{return new PackRefusal({kind:"RetainedAllocation",refusalKind:error.kind,what,offset,detail:error.reason,allocatedBytes:error.allocatedBytes});}
 constructor(public readonly data:PackRefusalData){
  super(packRefusalMessage(data),{cause:data.kind==="ValueRefusal"||data.kind==="TextRefusal"||data.kind==="Io"?data.error:data.kind==="TransportAdmission"?data.refusal:undefined});
  this.name="PackRefusal";
 }
 get kind():ValueRefusalKind{
  switch(this.data.kind){
   case "BadMagic":case "Truncated":case "ChecksumMismatch":case "ContentHashMismatch":case "NonCanonical":return "invalidValue";
   case "UnsupportedVersion":case "UnknownRequiredFlags":case "UnsupportedCodec":return "unsupportedOwner";
   case "LimitExceeded":case "RetainedMalformed":case "RetainedAllocation":case "Malformed":return this.data.refusalKind;
   case "ValueRefusal":case "TextRefusal":case "Io":return this.data.error.kind;
   case "TransportAdmission":return this.data.refusal.kind;
  }
 }
 intoValueError():ValueError{
  switch(this.data.kind){
   case "ValueRefusal":case "Io":return this.data.error;
   case "TextRefusal":return new ValueError(this.data.error.kind,this.data.error.message);
   default:return new ValueError(this.kind,this.message);
  }
 }
}
