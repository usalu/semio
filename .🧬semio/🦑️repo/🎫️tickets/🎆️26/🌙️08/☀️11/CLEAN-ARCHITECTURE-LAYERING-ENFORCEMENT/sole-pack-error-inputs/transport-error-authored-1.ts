/** ⚠️ Only transport-capable boundaries own external failure causes. */
import {ValueError,type ValueRefusalKind} from "../../🌱️value/⚠️refusal/🟦️.ts";
import {PackRefusal} from "./🪶️refusal/🟦️.ts";
import type {CapturedTransport,PackTransportCategory,PackRetryDisposition} from "./🔌️capture/🟦️.ts";
export {PackRefusal,packRefusalMessage} from "./🪶️refusal/🟦️.ts";
export type {PackRefusalData,PagedRefusalMetadata,PagedAllocationMetadata} from "./🪶️refusal/🟦️.ts";
export type {PackTransportCategory,PackRetryDisposition} from "./🔌️capture/🟦️.ts";

export type PackCauseKind=Readonly<{kind:"refusal";refusalKind:ValueRefusalKind}>|Readonly<{kind:"transport";category:PackTransportCategory}>;
export type PackErrorData=Readonly<{kind:"Refusal";error:PackRefusal}>|Readonly<{kind:"TransportFailure";error:CapturedTransport}>;
export type PackValueProjection=Readonly<{kind:"value";error:ValueError}>|Readonly<{kind:"pack";error:PackError}>;

/** 📤️ Semantic conversion is total; transport conversion returns its original owner. */
export class PackError extends Error{
 constructor(public readonly data:PackErrorData){
  super(data.kind==="Refusal"?data.error.message:`io error: ${data.error.cause.message}`,{cause:data.error.cause});
  this.name="PackError";
 }
 get causeKind():PackCauseKind{return this.data.kind==="TransportFailure"?this.data.error.causeKind:{kind:"refusal",refusalKind:this.data.error.kind};}
 get refusalKind():ValueRefusalKind|null{return this.data.kind==="Refusal"?this.data.error.kind:null;}
 get retry():PackRetryDisposition|null{return this.data.kind==="TransportFailure"?this.data.error.retry:this.data.error.data.kind==="Io"?this.data.error.data.retry:null;}
 intoValueError():PackValueProjection{return this.data.kind==="Refusal"?{kind:"value",error:this.data.error.intoValueError()}:{kind:"pack",error:this};}
}
