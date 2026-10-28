/** 🧩️ The declared constants of the claim registry. */
export interface ClaimsPolicyV1{readonly version:1;readonly directory:string;readonly chunkOwners:number;readonly tailOwners:number;readonly pollMilliseconds:number;readonly noticeMilliseconds:number;readonly maximumClaims:number;readonly maximumOwners:number;readonly slotMegabytes:number;readonly slotMemoryBasisPoints:number;readonly maximumSlots:number;readonly environmentSlots:string;readonly failedRetryMilliseconds:number;readonly failureReasonCharacters:number;readonly lockNameCharacters:number}
/** 📐️ The declared constants of the claim registry. */
export const CLAIMS_POLICY_V1:ClaimsPolicyV1;
/** 🔑️ The keys of the bilingual prefetch texts. */
export type ClaimMessageKeyV1="joined"|"computing"|"slotsBusy"|"served"|"failed"|"prepared";
/** 🗣️ The bilingual texts of the prefetch. */
export const CLAIM_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<ClaimMessageKeyV1,string>>>>;
/** 🖨️ Renders one prefetch message in the language the caller resolved. */
export function renderClaimMessageV1(key:ClaimMessageKeyV1,fields:Record<string,string|number>,language:"en"|"de"):string;
/** 🔍️ A closed claim document: the plan it belongs to, the owners in strict code point order and its state. */
export interface ClaimV1{readonly version:1;readonly token:string;readonly base:number;readonly implementation:string;readonly owners:readonly string[];readonly state:"running"|"done"|"failed";readonly startedMs:number;readonly endedMs?:number;readonly stamps?:Readonly<Record<string,string>>}
/** 👀️ A claim as one command sees it: with whether the lease of a running claim is held. */
export type SeenClaimV1=ClaimV1&{readonly live:boolean};
/** 🛂️ Admits a closed claim document or refuses it. */
export function readClaimV1(value:unknown):ClaimV1;
/** 📦️ What one command computes next: the open owners, the ones it takes, the ones a peer is running and how many are prepared or covered. */
export interface ChunkChoiceV1{readonly open:string[];readonly take:string[];readonly running:string[];readonly prepared:number;readonly covered:number}
/** 🧮️ Chooses what one command computes next from the claims it sees. */
export function chunkOfV1(options:{wanted:readonly string[];stamps?:Readonly<Record<string,string>>;claims:readonly Pick<SeenClaimV1,"token"|"base"|"implementation"|"owners"|"state"|"live"|"endedMs"|"stamps">[];base:number;implementation:string;chunk?:number;tail?:number;nowMs?:number}):ChunkChoiceV1;
/** 🎟️ The number of compute slots: the environment value in range, else the memory share divided by the slot size. */
export function slotCountV1(options:{memoryMegabytes:number;environment?:Record<string,string|undefined>}):number;
/** ✋️ A held lease that is given back by `release`. */
export interface ClaimLeaseHoldV1{release():void}
/** 🔐️ The lease operations the registry needs: try, hold and wait for a resource. */
export interface ClaimLeasePortV1{tryHold(resource:string):Promise<ClaimLeaseHoldV1|undefined>;hold(resource:string,options?:{signal?:AbortSignal}):Promise<ClaimLeaseHoldV1>;wait(resource:string,options?:{signal?:AbortSignal;timeoutMs?:number}):Promise<boolean>}
/** ⏳️ Answers true as soon as one of the leases has no exclusive holder, false after `timeoutMs`; every wait carries the caller's own signal. */
export function waitAnyLeaseV1(lease:ClaimLeasePortV1,resources:readonly string[],options:{signal?:AbortSignal;timeoutMs:number}):Promise<boolean>;
/** 🗄️ The file operations the registry needs: read, exclusive write, list and remove. */
export interface ClaimFilePortV1{read(path:string):Promise<Uint8Array|undefined>;write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}):Promise<boolean>;list(directory:string):Promise<{name:string;modifiedMs:number}[]>;remove(path:string):Promise<void>}
/** 🛠️ What one prefetch needs: the plan, the wanted owners, the computation and the optional progress, notice and cancellation ports. */
export interface ClaimPrefetchOptionsV1{base:number;implementation:string;wanted:readonly string[];stamps?:Readonly<Record<string,string>>;run(owners:string[],progress:{done:number;total:number}):Promise<void>;served?():Promise<number|undefined>;say?(key:ClaimMessageKeyV1,fields:Record<string,string|number>):void;progress?(row:{done:number;total:number}):void|Promise<void>;signal?:AbortSignal;slots?:number}
/** 📒️ The registry of claims: prefetch, sweep, list and the names of its lease resources. */
export interface ClaimRegistryV1{prefetch(options:ClaimPrefetchOptionsV1):Promise<{computed:number;served:boolean}>;idle(options:{base:number;implementation:string;wanted:readonly string[];builderFree:()=>Promise<boolean>;signal?:AbortSignal}):Promise<boolean>;sweep(signal?:AbortSignal):Promise<void>;list(signal?:AbortSignal):Promise<SeenClaimV1[]>;readonly resource:{claim(token:string):string;admission:string;slot(at:number):string}}
/** 🏗️ Creates the claim registry of the generation documents under `root` over a file port and a lease port. */
export function createClaimRegistryV1(options:{root:string;fs:ClaimFilePortV1;lease:ClaimLeasePortV1;now?:()=>number;token?:()=>string}):ClaimRegistryV1;
/** 🛡️ Runs a prefetch that can never fail its command: a failure is told through `say` and answered as `failed`, a cancellation is rethrown. */
export function prepareStaleProjectsV1(options:ClaimPrefetchOptionsV1&{registry:ClaimRegistryV1}):Promise<{computed:number;served:boolean;failed?:true}>;
/** 🔗️ The lease port of the registry over the queued lease acquisition of the process module, which the caller injects. */
export function createQueuedClaimLeasePortV1(options:{acquire:(request:{directory:string;resource:string;mode:"shared"|"exclusive";sharedCohort?:boolean;owner:string;control?:unknown;signal:AbortSignal;timeoutMs?:number})=>Promise<ClaimLeaseHoldV1>;directory:string;control?:{readonly signal:AbortSignal}}):ClaimLeasePortV1;
/** 🔒️ A reference lease port over lock files for tests and non repository runtimes. */
export function createFileClaimLeasePortV1(directory:string,options?:{sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}):ClaimLeasePortV1;
