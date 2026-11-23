import type {ResourceLeaseControl} from "../../../../🔒️leases/🎛️control/🟦️.ts";
import schema from "../🧬️schema/🧹️cleanup.json" with {type:"json"};
import{progressDeadlineRemainingV1}from"../../../../⏱️budget/🟦️.ts";

declare const cleanupBrand:unique symbol;
/** 🛟️ Identifies one privately accepted physical operation in the canonical process owner; a finite body keeps one nonrenewable whole deadline, an ordinary body a progress deadline. */
export type AcceptedPhysicalCleanupV1=Readonly<{[cleanupBrand]:true}>;
/** 📥️ Requests a finite reserve without conferring image or guard custody. */
export type PhysicalCleanupRequestV1=Readonly<{version:1;kind:"native-image"|"artifact-scratch";resource:string;reserveMilliseconds:number}>;
/** ⏱️ Captures immutable body funding; null deadlines retain an ordinary body with its preaccepted terminal reserve. */
export type AcceptedPhysicalCleanupFactsV1=Readonly<{version:1;kind:"native-image"|"artifact-scratch";resource:string;acceptedAtEpochMilliseconds:number;deadlineEpochMilliseconds:number|null;bodyDeadlineEpochMilliseconds:number|null;reserveMilliseconds:number;cleanupSignal:AbortSignal}>;
/** 🧷️ Keeps the Native operation and its physical settlement inside the accepted process scope. */
export type AcceptedPhysicalCleanupBodyV1<T>=(enrollment:AcceptedPhysicalCleanupV1,bodyControl:ResourceLeaseControl)=>Promise<T>;
/** 🔭️ The closed progress-deadline policy of an ordinary body: every observable progress event renews the idle reserve until the policy maximum since cleanup began. */
export const acceptedPhysicalCleanupProgressV1=Object.freeze({...schema.definitions.ProgressProtocol.const,events:Object.freeze([...schema.definitions.ProgressProtocol.const.events])});
/** 🧹️ Computes the idle window one progress event grants: the reserve, cut to what remains of the policy maximum; zero means no further renewal. */
export function acceptedPhysicalCleanupWindowV1(reserveMilliseconds:number,elapsedMilliseconds:number):number{return Math.max(0,progressDeadlineRemainingV1({idleMilliseconds:reserveMilliseconds,maximumMilliseconds:acceptedPhysicalCleanupProgressV1.maximumMilliseconds},Math.max(0,elapsedMilliseconds),0));}
