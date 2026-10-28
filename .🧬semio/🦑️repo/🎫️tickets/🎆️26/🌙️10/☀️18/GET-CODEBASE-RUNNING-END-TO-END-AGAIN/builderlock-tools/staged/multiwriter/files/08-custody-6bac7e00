import type {ClaimFilePortV1,ClaimLeasePortV1} from "../🧩️claims/🟨️.mjs";
/** 🔐️ The declared constants of the slice custody. */
export interface CustodyPolicyV1{readonly version:1;readonly directory:string;readonly pollMilliseconds:number;readonly noticeMilliseconds:number;readonly maximumClaims:number;readonly maximumOwners:number;readonly sampleOwners:number;readonly widenRounds:number}
/** 📐️ The declared constants of the slice custody. */
export const CUSTODY_POLICY_V1:CustodyPolicyV1;
/** 🔑️ The keys of the bilingual custody texts. */
export type CustodyMessageKeyV1="blocked"|"blockedAll";
/** 🗨️ The bilingual texts of the slice custody. */
export const CUSTODY_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<CustodyMessageKeyV1,string>>>>;
/** 🖨️ Renders one custody message in the language the caller resolved. */
export function renderCustodyMessageV1(key:CustodyMessageKeyV1,fields:Record<string,string|number>,language:"en"|"de"):string;
/** 🪪️ A closed custody claim document. */
export interface CustodyClaimV1{readonly version:1;readonly token:string;readonly ticket:number;readonly owners:readonly string[];readonly structural:boolean;readonly state:"waiting"|"held";readonly queuedMs:number}
/** 🛂️ Admits a closed custody claim or refuses it. */
export function readCustodyClaimV1(value:unknown):CustodyClaimV1;
/** ⚔️ Whether two claims may not build at the same time. */
export function claimsConflictV1(a:Pick<CustodyClaimV1,"owners"|"structural">,b:Pick<CustodyClaimV1,"owners"|"structural">):boolean;
/** 🚦️ Decides one admission against the other live claims. */
export function admissionOfV1<C extends Pick<CustodyClaimV1,"token"|"ticket"|"owners"|"structural"|"state">>(mine:C,others:readonly C[]):{admit:boolean;blockers:C[]};
/** 🧭️ What a builder asks the custody for: the owners it writes, structural when they cannot be named. */
export interface CustodyRequestV1{readonly dirty:readonly string[];readonly structural:boolean}
/** 🎫️ A held claim. */
export interface SliceClaimV1{readonly token:string;readonly owners:readonly string[];readonly structural:boolean;extend(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<{held:boolean;request:CustodyRequestV1}>;alone(options?:{signal?:AbortSignal}):Promise<boolean>;release():Promise<void>}
/** 🏛️ The slice custody of one generation store. */
export interface SliceCustodyV1{hold(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal;say?:(key:CustodyMessageKeyV1,fields:Record<string,string|number>)=>void}):Promise<SliceClaimV1>;idle(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<boolean>;list(signal?:AbortSignal):Promise<CustodyClaimV1[]>;readonly resource:{claim(token:string):string;admission:string}}
/** 🏗️ Creates the slice custody of the generation documents under `root`. */
export function createSliceCustodyV1(options:{root:string;fs:Omit<ClaimFilePortV1,never>;lease:ClaimLeasePortV1;now?:()=>number;token?:()=>string}):SliceCustodyV1;
