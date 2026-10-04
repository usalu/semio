/** ⚠️ Unmounted source-authored Pack causes; production provider rebasing remains separate. */
import {ValueError,type ValueRefusalKind} from "../../../../../../../../🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts";
import {TextError} from "../../../../../../../../🧰️framework/🔨️modules/⚠️diagnostic/🚧️text-error/🟦️.ts";
import type {CapturedTransport} from "./🔌️capture/🟦️.ts";
export {ValueError,TextError};
export type {ValueRefusalKind};

export type PagedRefusalMetadata=Readonly<{kind:"ownershipLimit"|"allocationFailed"|"invariantViolated";reason:string}>;
export type PagedAllocationMetadata=PagedRefusalMetadata&Readonly<{allocatedBytes:number}>;
export type PackTransportCategory="nativeIo"|"httpRequest"|"httpBody";
export type PackCauseKind=Readonly<{kind:"refusal";refusalKind:ValueRefusalKind}>|Readonly<{kind:"transport";category:PackTransportCategory}>;

export type PackErrorData =
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
 | Readonly<{kind:"Io";error:ValueError;retry:"never"|"transient"}>
 | Readonly<{kind:"TransportFailure";error:CapturedTransport}>;

/** 🗨️ Preserves presentation without making it machine-cause authority. */
export function packErrorMessage(data:PackErrorData):string {
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
  case "TransportFailure":return `io error: ${data.error.cause.message}`;
 }
}

/** 🪪️ Keeps an authored kind and original owned source independent of display. */
export class PackError extends Error {
 static fromPagedRefusal(error:PagedRefusalMetadata,what:string,offset:bigint):PackError{return new PackError({kind:"RetainedMalformed",refusalKind:error.kind,what,offset,detail:error.reason});}
 static fromPagedAllocation(error:PagedAllocationMetadata,what:string,offset:bigint):PackError{return new PackError({kind:"RetainedAllocation",refusalKind:error.kind,what,offset,detail:error.reason,allocatedBytes:error.allocatedBytes});}
 constructor(public readonly data:PackErrorData){
  super(packErrorMessage(data),{cause:data.kind==="TransportFailure"?data.error.cause:data.kind==="ValueRefusal"||data.kind==="TextRefusal"||data.kind==="Io"?data.error:undefined});
  this.name="PackError";
 }
 get causeKind():PackCauseKind{return this.data.kind==="TransportFailure"?this.data.error.causeKind:{kind:"refusal",refusalKind:this.refusalKind!};}
 get refusalKind():ValueRefusalKind|null {
  switch(this.data.kind){
   case "BadMagic":case "Truncated":case "ChecksumMismatch":case "ContentHashMismatch":case "NonCanonical":return "invalidValue";
   case "UnsupportedVersion":case "UnknownRequiredFlags":case "UnsupportedCodec":return "unsupportedOwner";
   case "LimitExceeded":case "RetainedMalformed":case "RetainedAllocation":case "Malformed":return this.data.refusalKind;
   case "ValueRefusal":case "TextRefusal":case "Io":return this.data.error.kind;
   case "TransportFailure":return null;
  }
 }
 get retry():"never"|"transient"|null{return this.data.kind==="TransportFailure"?this.data.error.retry:this.data.kind==="Io"?this.data.retry:null;}
}
