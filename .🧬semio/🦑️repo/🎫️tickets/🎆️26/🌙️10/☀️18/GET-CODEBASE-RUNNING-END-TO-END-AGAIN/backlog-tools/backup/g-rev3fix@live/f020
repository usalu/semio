/** 🕰️ One observation row: what a computation read and the fact it saw. */
export interface GenerationRowV1{readonly kind:string;readonly path:string;readonly fact:unknown;readonly scope?:"project"|"native"|"environment"|"all"}
export interface GenerationPolicyV1{readonly version:1;readonly reasonRows:number;readonly pointer:string;readonly manifests:string;readonly fragments:string;readonly indexes:string;readonly frontiers:string;readonly envOwner:string;readonly policies:readonly ["fresh","current"];readonly retainGenerations:number;readonly graceMilliseconds:number;readonly verifyChunkRows:number;readonly verifyParallelChunks:number;readonly bisectBudgetCalls:number;readonly racyMilliseconds:number;readonly readConcurrency:number;readonly publishWaitMilliseconds:number;readonly pollMaximumMilliseconds:number;readonly maximumOwners:number;readonly maximumManifestBytes:number;readonly maximumFragmentBytes:number;readonly maximumIndexBytes:number;readonly compactionFactor:number;readonly compactionFloor:number;readonly compactionGrowth:number;readonly brotliQuality:number;readonly brotliWindow:number;readonly stampVerification:boolean;readonly waitNoticeMilliseconds:number;readonly hintMaximumAgeMilliseconds:number;readonly sweepWaitMilliseconds:number}
export const GENERATION_POLICY_V1:GenerationPolicyV1;
export type GenerationGraphPolicyV1="fresh"|"current";
export interface GenerationPointerV1{readonly version:1;readonly generation:number;readonly manifest:string|null;readonly parent:string|null;readonly reason?:string;readonly verified?:import("./🔭️coverage/🟨️.mjs").VerifiedStampV1}
export interface GenerationOwnerRowV1{readonly root:string;readonly kind:"project"|"unavailable";readonly fragment:string;readonly bytes:number;readonly edgesEnv:string;readonly targets:readonly string[];readonly config?:string;readonly nativeRoot?:string;readonly tag?:string}
export interface GenerationManifestV1{readonly version:1;readonly generation:number;readonly parent:string|null;readonly identity:string;readonly implementation:string;readonly env:string;readonly membership:string;readonly frontier:string|null;readonly factIndex:string;readonly global:string;readonly owners:Readonly<Record<string,GenerationOwnerRowV1>>;readonly built:{readonly kind:"cold"|"delta"|"refresh";readonly base:number|null;readonly dirty:readonly string[]}}
export interface GenerationFragmentV1{readonly version:1;readonly name:string;readonly root:string;readonly kind:"project"|"unavailable";readonly node:unknown;readonly edges:readonly {source:string;target:string;[key:string]:unknown}[];readonly edgesEnv:string;readonly targets:readonly string[];readonly config?:string;readonly nativeRoot?:string}
export interface GenerationSealedV1{readonly pointer:GenerationPointerV1;readonly manifest:GenerationManifestV1|undefined;readonly missing?:boolean}
export interface FactIndexV1{version:1;owners:string[];rows:GenerationRowV1[];classes:{r:number[];o:number[]}[]}
export interface FrontierV1{version:1;capturedAt:number;directories:[string,number,number][];members:[string,number][]}
export interface MembershipChangeV1{path:string;kind:"added"|"removed"|"changed";directory?:boolean;config?:boolean}
export interface GenerationMembershipV1{digest:string;frontier:FrontierV1;changes:MembershipChangeV1[]}
export interface GenerationStaleReasonV1{row?:GenerationRowV1;change?:{path:string;kind:"added"|"removed"|"changed"};owners:string[]}
export interface GenerationPlanV1{kind:"cold"|"clean"|"delta";reason?:string;dirty:Set<string>;added:string[];removed:string[];envStale:boolean;checked:number;stale?:number;reasons?:GenerationStaleReasonV1[];cutoff?:{questions:number;held:number;changed:number;failed:number};refresh?:ReadonlyMap<string,GenerationRowV1[]>}
export interface GenerationProgressV1{phase:"facts"|"verification"|"assembly";done:number;total:number}
export interface GenerationComputeSpecV1{cold:boolean;owners:string[];discover:string[];removed:string[];changed:string[];previousEnv:string|null;envStale:boolean;generation:number;coldReason?:string;reasons:GenerationStaleReasonV1[];staleRows:number;staleKinds?:Record<string,number>;stubs:Record<string,{root:string;kind:"project"|"unavailable";targets:readonly string[];nativeRoot?:string;config?:string;tag?:string}>}
export interface GenerationComputedOwnerV1{name:string;partial?:"edges";root?:string;kind?:"project"|"unavailable";node?:unknown;edges?:unknown[];targets?:string[];config?:string;nativeRoot?:string;tag?:string;rows?:GenerationRowV1[]}
export interface GenerationComputeResultV1{owners:GenerationComputedOwnerV1[];reEdge?:(name:string,fragment:{node?:unknown})=>Promise<unknown[]>;env:{digest:string;rows:GenerationRowV1[]};global?:unknown;replaceTags?:string[];derived?:import("./✂️cutoff/🟨️.mjs").DerivedFactV1[]}
export interface GenerationFilePortV1{read(path:string):Promise<Uint8Array|undefined>;write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}):Promise<boolean>;names(directory:string):Promise<string[]>;list(directory:string):Promise<{name:string;modifiedMs:number}[]>;remove(path:string):Promise<void>}
export interface GenerationLeasePortV1{exclusive<T>(resource:string,body:()=>Promise<T>,options?:{signal?:AbortSignal;timeoutMs?:number}):Promise<T>}
export interface GenerationStoreOptionsV1{root:string;fs?:GenerationFilePortV1;lease?:GenerationLeasePortV1;holds:(rows:readonly GenerationRowV1[],signal?:AbortSignal)=>Promise<boolean>;derive?:(row:GenerationRowV1,signal?:AbortSignal)=>Promise<import("./✂️cutoff/🟨️.mjs").DerivedAnswerV1>;resources:{facts:string;builder?:string};now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}
/** 🧩️ The stale owners a refresh found before it asked for the builder: the generation it planned against, their names in code point order and their roots. */
export interface GenerationPendingV1{readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>}
export interface GenerationRefreshOptionsV1{closure?:(current:GenerationSealedV1,control:{signal?:AbortSignal})=>Promise<ReadonlySet<string>>;identity:string;implementation:string;policy?:GenerationGraphPolicyV1;only?:ReadonlySet<string>;membership:(previous:{manifest:GenerationManifestV1;frontier:FrontierV1}|undefined,signal?:AbortSignal)=>Promise<GenerationMembershipV1>;compute:(spec:GenerationComputeSpecV1,signal?:AbortSignal)=>Promise<GenerationComputeResultV1>;signal?:AbortSignal;progress?:(event:GenerationProgressV1)=>void|Promise<void>;builder?:<T>(body:()=>Promise<T>,pending?:GenerationPendingV1)=>Promise<T>;requestedAt?:number}
export interface GenerationRefreshResultV1{current:GenerationSealedV1;served:"current"|"verified"|"covered"|"built";built:boolean;published:boolean;computed:number;plan?:GenerationPlanV1;index?:GenerationIndexSizeV1}
export interface GenerationAssembledV1{projects:Record<string,unknown>;dependencies:unknown[];externalNodes:Record<string,unknown>;unavailable:{root:string;[key:string]:unknown}[];generation:number;membership:string}
/** 📏️ The size of the fact index a build published. */
export interface GenerationIndexSizeV1{readonly rows:number;readonly owners:number;readonly expandedBytes:number;readonly packedBytes:number}
export interface GenerationStoreV1{
 /** 🕊️ Drops the parsed fact indexes the store keeps for patching (about a gigabyte on the real workspace); a reader that verified or built needs them no longer. */
 dropIndexes():void;
 read(signal?:AbortSignal):Promise<GenerationSealedV1|undefined>;readManifestAt(digest:string,signal?:AbortSignal):Promise<GenerationManifestV1|undefined>;readIndex(digest:string):Promise<FactIndexV1|undefined>;readIndexView(digest:string):Promise<FactIndexV1|undefined>;readFrontier(digest:string|null):Promise<FrontierV1|undefined>;readFragment(row:GenerationOwnerRowV1):Promise<GenerationFragmentV1|undefined>;
 readGlobal(manifest:GenerationManifestV1):Promise<{readonly externalNodes?:Record<string,unknown>;readonly unavailable?:readonly {readonly root:string}[]}>;
 catalog(current:GenerationSealedV1,options?:{signal?:AbortSignal}):Promise<{projects:{name:string;root:string;targets:readonly string[]}[];unavailable:{root:string}[]}>;
 plan(options:{current:GenerationSealedV1|undefined;identity:string;changes?:MembershipChangeV1[];only?:ReadonlySet<string>;skipEnvironment?:boolean;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<GenerationPlanV1>;
 build(options:Omit<GenerationRefreshOptionsV1,"policy"|"only"|"builder"|"requestedAt">&{only?:ReadonlySet<string>;observedAtMs?:number;coverage?:import("./🔭️coverage/🟨️.mjs").CoverageV1}):Promise<{current:GenerationSealedV1;built:boolean;published:boolean;computed:number;plan:GenerationPlanV1;index?:GenerationIndexSizeV1}>;
 refresh(options:GenerationRefreshOptionsV1):Promise<GenerationRefreshResultV1>;
 refreshClosure(options:Omit<GenerationRefreshOptionsV1,"only"|"policy">&{roots:readonly string[];neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];policy?:GenerationGraphPolicyV1}):Promise<GenerationRefreshResultV1&{names:Set<string>}>;
 closureOf(current:GenerationSealedV1|undefined,roots:readonly string[],options?:{neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];signal?:AbortSignal}):Promise<Set<string>>;
 assembleStream(current:GenerationSealedV1,options?:{names?:readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):{nodes():AsyncGenerator<[string,unknown],void,undefined>;finish():Promise<Omit<GenerationAssembledV1,"projects">>};
 assemble(current:GenerationSealedV1,options?:{names?:readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<GenerationAssembledV1>;
 snapshot(options?:{roots?:readonly string[];neighbors?:(fragment:GenerationFragmentV1)=>readonly string[];compare?:(a:string,b:string)=>number;signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"]}):Promise<{current:GenerationSealedV1;graph:GenerationAssembledV1}>;
 retire(options?:{signal?:AbortSignal}):Promise<void>;invalidate(options?:{reason?:string;signal?:AbortSignal}):Promise<boolean>;
 publish(options:{manifestDigest:string|null;expectedGeneration:number;reason?:string;verified?:import("./🔭️coverage/🟨️.mjs").VerifiedStampV1;signal?:AbortSignal}):Promise<{published:boolean;pointer?:GenerationPointerV1}>;
 readonly stats:{holdCalls:number;rowsChecked:number;computed:number;published:number;casConflicts:number;publishHeldMs:number[];indexLoads:number;indexViews:number;fragmentReads:number};
}
export function createGenerationStoreV1(options:GenerationStoreOptionsV1):GenerationStoreV1;
export function createNodeFilePortV1():GenerationFilePortV1;
export function createFileLeasePortV1(directory:string,options?:{staleMilliseconds?:number;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;onHold?:(row:{resource:string;heldMs:number;waitedMs:number})=>void}):GenerationLeasePortV1;
export function readGenerationPointerV1(bytes:Uint8Array):GenerationPointerV1;
export function readGenerationManifestV1(bytes:Uint8Array):GenerationManifestV1;
export function readGenerationFragmentV1(value:unknown):GenerationFragmentV1;
export function canonicalJsonV1(value:unknown):string;
/** 📦️ Encodes one canonical document within the expanded and packed reader byte ceiling. */
export function packGenerationDocumentV1(value:unknown,maximumBytes:number):Uint8Array;
export function packFactIndexV1(index:FactIndexV1,maximumBytes:number):{bytes:Uint8Array;expandedBytes:number};
export function unpackFactIndexV1(bytes:Uint8Array,maximumBytes:number):FactIndexV1;
/** 🪶️ Opens a packed fact index for verification only: rows become objects when they are read. */
export function openPackedFactIndexV1(bytes:Uint8Array,maximumBytes:number):FactIndexV1;
export function compareCodePointsV1(a:string,b:string):number;
export function rowKeyV1(row:GenerationRowV1):string;
export const ROW_KEY_TEXT_LIMIT_V1:128;
export function createFactIndexV1(ownerRows:ReadonlyMap<string,readonly GenerationRowV1[]>):FactIndexV1;
export function patchFactIndexV1(index:FactIndexV1,change:{drop?:readonly string[];upserts?:ReadonlyMap<string,readonly GenerationRowV1[]>;append?:ReadonlyMap<string,readonly GenerationRowV1[]>}):FactIndexV1;
export function refineFactIndexV1(index:FactIndexV1):FactIndexV1;
export function factConsumersV1(index:FactIndexV1):[string,string[]][];
export function staleFactRowsV1(index:FactIndexV1,ids:readonly number[],hold:(rows:GenerationRowV1[],signal?:AbortSignal)=>Promise<boolean>,options?:{signal?:AbortSignal;progress?:GenerationRefreshOptionsV1["progress"];chunk?:number;parallel?:number;budget?:number}):Promise<{stale:Set<number>;calls:number}>;
export function selectFactClassesV1(index:FactIndexV1,only?:ReadonlySet<string>):{classes:number[];ids:number[]};
export function staleConsumersV1(index:FactIndexV1,classes:readonly number[],stale:ReadonlySet<number>):Set<string>;
export function createFrontierV1(value:{directories:Iterable<[string,number,number]>;members:Iterable<[string,number]>;capturedAt:number}):FrontierV1;
/** 0️⃣ Whether a recorded frontier fact (an inode or a member mode) agrees with a present one; `0` is the native scan's word for a fact the platform does not give it. */
export function frontierFactHoldsV1(present:number,recorded:number):boolean;
export function frontierHoldsV1(frontier:FrontierV1,stat:(path:string)=>Promise<{directory:boolean;file:boolean;mtimeMs:number;ino:number;mode:number}|undefined>,options?:{signal?:AbortSignal;concurrency?:number;racyMilliseconds?:number}):Promise<boolean>;
export function diffFrontiersV1(before:FrontierV1,after:FrontierV1):MembershipChangeV1[];
/** 🧱️ The directories whose entry sets are proven unchanged between two frontiers. */
export function stableDirectoriesV1(before:FrontierV1,after:FrontierV1,options?:{racyMilliseconds?:number}):Set<string>;
/** 🚧️ The untagged withheld owners that hold no fact about the source that withholds them, so they must be inferred again. */
export function unfoundedWithheldOwnersV1(index:Pick<FactIndexV1,"owners"|"rows"|"classes">,owners:Readonly<Record<string,{kind:string;tag?:string;config?:string}>>,separator?:string):string[];
