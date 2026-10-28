import type {VerifiedStampV1} from "../🔭️coverage/🟨️.mjs";
import type {GenerationManifestV1,GenerationOwnerRowV1,GenerationRowV1,GenerationSealedV1} from "../🟨️.mjs";
/** 🔀️ The declared constants of the generation merge. */
export interface MergePolicyV1{readonly version:1;readonly rounds:number;readonly envOwner:string;readonly stubFields:readonly string[];readonly conflicts:readonly MergeConflictV1[]}
/** 📐️ The declared constants of the generation merge. */
export const MERGE_POLICY_V1:MergePolicyV1;
/** 🚧️ Why a delta cannot be merged onto the head. */
export type MergeConflictV1="identity"|"head"|"index"|"structure"|"environment"|"owner"|"global"|"membership"|"derived"|"rounds";
/** 👻️ Whether an owner is the environment or a derived fact. */
export function isPseudoOwnerV1(name:string):boolean;
/** 🗂️ The fact index shape the merge reads. */
export interface MergeIndexV1{readonly owners:readonly string[];readonly rows:readonly GenerationRowV1[];readonly classes:readonly {readonly r:readonly number[];readonly o:readonly number[]}[]}
/** 🧾️ The rows of the named owners of one fact index. */
export function ownerRowsOfV1(index:MergeIndexV1,names:ReadonlySet<string>):Map<string,GenerationRowV1[]>;
/** 🩹️ A fact index patch. */
export interface MergePatchV1{readonly drop:readonly string[];readonly upserts:ReadonlyMap<string,readonly GenerationRowV1[]>;readonly append:ReadonlyMap<string,readonly GenerationRowV1[]>}
/** 🧮️ The pure merge decision. */
export function mergeGenerationV1(options:{base:GenerationManifestV1;head:GenerationManifestV1;mine:GenerationManifestV1;baseIndex:MergeIndexV1;headIndex:MergeIndexV1;patch:MergePatchV1;verified?:VerifiedStampV1;headVerified?:VerifiedStampV1;key:(row:GenerationRowV1)=>string}):{conflict:MergeConflictV1;owner?:string}|{conflict?:undefined;owners:Record<string,GenerationOwnerRowV1>;global:string;membership:string;frontier:string|null;drop:readonly string[];upserts:Map<string,GenerationRowV1[]>;append:ReadonlyMap<string,readonly GenerationRowV1[]>;theirs:string[];ours:string[];verified?:VerifiedStampV1};
/** 🔌️ The store operations a merge needs. */
export interface MergePortV1{read(signal?:AbortSignal):Promise<GenerationSealedV1|undefined>;readIndex(digest:string):Promise<MergeIndexV1|undefined>;writeManifest(manifest:GenerationManifestV1):Promise<string>;publish(options:{manifestDigest:string;expectedGeneration:number;verified?:VerifiedStampV1;signal?:AbortSignal}):Promise<{published:boolean;pointer:unknown}>;patch(index:MergeIndexV1,patch:MergePatchV1|{drop:readonly string[]}):MergeIndexV1;pack(index:MergeIndexV1):{bytes:Uint8Array;expandedBytes:number};keep(bytes:Uint8Array):Promise<{digest:string;bytes:number}>;key(row:GenerationRowV1):string;stats?:{merged:number;mergeConflicts:number}}
/** 🔁️ Merges a delta onto the newest head after a lost swap. */
export function mergeIntoHeadV1(port:MergePortV1,input:{base:GenerationSealedV1;mine:GenerationManifestV1;patch:MergePatchV1;verified?:VerifiedStampV1;signal?:AbortSignal}):Promise<{conflict:MergeConflictV1;owner?:string;outcome?:undefined}|{conflict?:undefined;outcome:{published:true;pointer:unknown};next:GenerationManifestV1;index:MergeIndexV1;packed:{bytes:Uint8Array;expandedBytes:number};kept:{digest:string;bytes:number};theirs:string[];rounds:number}>;
