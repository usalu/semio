import type * as fs from "node:fs";
import type {readFile} from "node:fs/promises";
import type {ResourceLeaseControl} from "@semio-tech/process/leases";
import type{ExactSourceReadBatchV1,ExactSourceFileV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";

/** 🚧️ The declared limits of the observation module: its schema file is their single source. */
export const OBSERVATION_LIMITS_V1: Readonly<{settledReadMaximumFileBytes:number;settleRounds:number;holdNestingLimit:number;racyMilliseconds:number;sourceAnalysisBytes:number;sourceAnalysisOverheadBytes:number;settledReadBytes:number;fileRecordImageBytes:number;fileRecordImageMaximumBytes:number;verifiedBlockRows:number;blockMinimumRows:number;blockCutMask:number;blockRefusalMilliseconds:number;detachedLayerRows:number;leafReadChunkBytes:number;sweepGraceMilliseconds:number;fingerprintProgressRows:number;sweepHeaderProgressRows:number;sweepBlockProgressRows:number;maximumIdentifierCharacters:number;nativeRealpathMaximumCharacters:number}>;

/** 🧾️ Describes exactly one byte image admitted by the defining observation schema. */
export type SourceFileFactV1 = Readonly<{size:number;mtimeMs:number;ctimeMs:number;ino:number;sha256:string}>;
/** 🧯️ Retains a physical read failure without inventing a source image. */
export type SourceReadFailureV1 = Readonly<{error:string}>;
/** 🧠️ Records one registered parser's JSON result and complete nested consumed facts. */
export type SourceAnalysisFactV1 = Readonly<{provider:string;revision:string;source:SourceFileFactV1;resultSha256:string;nestedObservations:readonly SourceObservationV1[]}>;
/** 👁️ Uses the canonical schema's observation kinds and operation-specific fact shapes. */
export type SourceObservationV1 =
 | Readonly<{kind:"file";path:string;fact:SourceFileFactV1|SourceReadFailureV1}>
 | Readonly<{kind:"analysis";path:string;fact:SourceAnalysisFactV1}>
 | Readonly<{kind:"exists"|"fileExists"|"directoryExists";path:string;fact:boolean}>
 | Readonly<{kind:"stamp";path:string;fact:string}>
 | Readonly<{kind:"stat"|"lstat"|"readdir"|"realpath"|"realpathNative"|"resolve"|"readdirNames"|"directories"|"typescriptText"|"typescriptRealpath"|"registry";path:string;fact:string}>;
/** 🎟️ Names the immutable parser handle; structural copies remain refused by the runtime registry. */
export type SourceAnalysisProviderV1 = Readonly<{id:string;revision:string}>;
/** 📸️ Captures the actual result and every original Source and load-authority observation. */
export type CapturedSourceInputsV1<T> = Readonly<{value:T;observations:readonly SourceObservationV1[];authorityObservations:readonly SourceObservationV1[]}>;
/** 🧩️ Supplies only neutral compiler filesystem methods; external compiler types are not required. */
export interface SourceTypeScriptSystemV1 {
 fileExists(path:string):boolean;
 directoryExists(path:string):boolean;
 readFile(path:string,encoding?:string):string|undefined;
 getDirectories(path:string):string[];
 realpath?:((path:string)=>string)|undefined;
}
/** 📂️ Keeps system filesystem overloads while recording their original results. */
export const observedFs:Readonly<Pick<typeof fs,"readFileSync"|"existsSync"|"statSync"|"lstatSync"|"readdirSync"|"realpathSync"> & {readFile:typeof readFile}>;
/** 🔏️ Registers only the original parser function for one explicit identity and revision. */
export function registerSourceAnalysisV1(declaration:Readonly<{id:string;revision:string;analyze:(path:string,bytes:Buffer)=>unknown}>):SourceAnalysisProviderV1;
/** 🔬️ Returns an admitted JSON image; consumers must validate its defining parser result. */
export function observedSourceAnalysisV1(provider:SourceAnalysisProviderV1,path:fs.PathLike):unknown;
/** 🌊️ Retains original parser and nested Source observations through asynchronous input. */
export function observedSourceAnalysisAsyncV1(provider:SourceAnalysisProviderV1,path:fs.PathLike):Promise<unknown>;
/** 🧿️ Seals and fingerprints the original module-load authority without issuing a grant. */
export function inferenceAuthorityDigestV1():string;
/** 🪪️ The identity every persisted project record of this process is written under. */
export function inferenceRecordIdentityV1():string;
/** 🫱️ Seals the module-load authority of this process and answers its rows. */
export function inferenceAuthorityRowsV1():readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[];
/** 🎁️ Serves under a parent's sealed authority unless an own load row conflicts; answers the conflicts (`kind:path`), empty when adopted. */
export function adoptInferenceAuthorityV1(rows:readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[]):string[];
/** 🧊️ Registers a block a farm worker validated and the caller verified by its hash. */
export function adoptVerifiedBlockV1(path:string,text:string):void;
/** 📦️ Admits one physical project's awaited JSON computation and complete observations. */
export function observeProjectInferenceV1<T>(options:Readonly<{cachePath:string;key:unknown;owner:string;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;stamps?:readonly string[];prepare?:()=>boolean|PromiseLike<boolean>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🔖️ Registers the declared stamps of the inference protocol as `[path, stamp]` pairs and the path fragments they cover, which the module-load authority names by stamp instead of by bytes. */
export function registerInferenceStampsV1(entries:Iterable<readonly [string,string]>,fragments?:readonly string[]):void;
/** 📌️ Records the stamp of each named inferrer in the active inference scope. */
export function observedInferenceStampsV1(paths:readonly string[]):void;
/** 🧵️ Replays complete original observations into every receiving synchronous scope. */
export function memoizedObservedComputationV1<T>(cache:Map<unknown,unknown>,key:unknown,compute:()=>T):T;
/** 🧪️ Replays original facts while checking the receiving caller's continuation. */
export function memoizedObservedAsyncComputationV1<T>(cache:Map<unknown,unknown>,key:unknown,compute:()=>Promise<T>,checkpoint:()=>Promise<void>):Promise<T>;
/** 🕸️ Preserves every Source fact even when a computation fails. */
export function captureObservedSourceInputsV1<T>(compute:()=>T|PromiseLike<T>,onFailure?:((observations:SourceObservationV1[],authorityObservations:SourceObservationV1[])=>void)|undefined,includeAuthority?:boolean):Promise<CapturedSourceInputsV1<Awaited<T>>>;
/** ✍️ Records actual bounded bytes without accepting a caller-provided digest or file observation. */
export function observedExactSourceFileV1(batch:ExactSourceReadBatchV1,root:string,name:string):Promise<ExactSourceFileV1|null>;
/** 📍️ Records the actual system module resolver result or failure. */
export function observedRequireResolve(from:string|URL,request:string):string;
/** 🧭️ The spelling in which a module resolution is recorded: the dependency generation id is a placeholder. */
export function canonicalResolutionV1(path:string):string;
/** 🔗️ The seam of the shared command closure entries: how the answer of a `closure` fact is read now and how the entries named by facts are brought up to date. */
export function registerClosureProviderV1(provider:{answer(path:string):string;settle(paths:readonly string[],value?:unknown):Promise<void>}):void;
/** 🚚️ Brings the closure entries named by the `closure` facts among `rows` up to date. */
export function settleClosureRowsV1(rows:readonly {kind:string;path:string}[],value?:unknown):Promise<void>;
/** 📝️ Records the answer one closure entry gave to the running inference. */
export function recordClosureFactV1(path:string,answer:string):void;
/** 🖊️ Records the registry row (root and manifest) one inference read for one owner name, or its absence. */
/** 🕊️ Drops every in-process cache a graph construction filled; safe only when no inference runs. */
export function releaseInferenceMemoryV1():void;
/** 📏️ The entry counts of the in-process caches a graph construction fills. */
export function inferenceCacheSizesV1():{interned:number;verifiedBlockRows:number;settledReads:number;projectImages:number;publications:number;registryFiles:number};
export function recordRegistryOwnerV1(workspaceRoot:string,name:string,row:{root:string;manifest:string}|undefined|null):void;
/** ❓️ Whether recorded rows still hold, sharing the checks of the running inference round. */
export function observationRowsHoldV1(rows:readonly unknown[]):boolean;
/** 🤔️ Whether the shared rows of one reused computation still hold, checked once per validation round. */
export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;
/** 🏃️ The running validation round, if any. */
export function observationRoundV1():object|undefined;
/** 🔂️ Runs `compute` inside one validation round (the open one when there is one), so stored facts and memoized computations are validated once per round. */
export function withObservationRoundV1<T>(compute:()=>T):T;
/** ▶️ Runs a computation with its observations detached from the running inference; returns its rows and the shared row arrays it reused. */
export function observeDetachedV1<T>(compute:()=>Promise<T>):Promise<{value:T;rows:unknown[];layers:(readonly unknown[])[]}>;
/** 📚️ The facts a detached computation read before it failed, or undefined for an error that carries none. */
export function detachedFailureV1(error:unknown):{rows:unknown[];layers:(readonly unknown[])[]}|undefined;
/** 🧲️ Records the facts of a failed detached computation into the running observation. */
export function retainDetachedFailureV1(error:unknown):void;
/** 🧹️ Removes the observation blocks of one record directory that no record header names any more. */
export function sweepSupersededObservationBlocksV1(directory:string,options?:{headerPattern?:RegExp;graceMilliseconds?:number;now?:number;dryRun?:boolean;minimumBlocks?:number;blocksPerHeader?:number;signal?:AbortSignal;progress?:(row:{phase:string;done:number;total:number})=>void|Promise<void>}):Promise<{headers:number;live:number;blocks:number;removed:number;bytes:number;kept:number;skipped?:true}>;
/** ↩️ Returns the original neutral compiler system with observed filesystem ports. */
export function observedTypeScriptHost<T extends SourceTypeScriptSystemV1>(sys:T):T;
/** 🧳️ Memoizes only the current JSON image while retaining its original input identity. */
export function memoizedRecord<T>(namespace:string,inputs:readonly(string|Uint8Array)[],compute:()=>T):T;
/** 📖️ Parses one observed physical file through its original byte image. */
export function memoizedFileRecord<T>(namespace:string,path:fs.PathLike,compute:(bytes:Buffer)=>T):T;
/** 🪶️ Reads one observed file asynchronously without changing its parser or byte identity. */
export function memoizedFileRecordAsyncV1<T>(namespace:string,path:fs.PathLike,compute:(bytes:Buffer)=>T):Promise<Awaited<T>>;
/** 🪨️ Reports a current inference round through its original caller callback. */
export type SourceSettlementRoundV1 = Readonly<{round:number;conflict?:string;failure?:unknown}>;
/** ⏳️ Settles mutually consistent original facts under the existing signal and finite round limit. */
export function settleObservedComputationV1<T>(compute:()=>T|PromiseLike<T>,options?:Readonly<{signal?:AbortSignal;limit?:number;onRound?:(round:SourceSettlementRoundV1)=>void|Promise<void>}>):Promise<Awaited<T>>;
/** 🗃️ Reuses an inference result only while its complete current facts hold. */
export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🪢️ Publishes dependencies only with complete original inference and consumed facts. */
export function observeDependencies<T>(options:Readonly<{receiptPath:string;recordDirectory?:string}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 🎛️ Receives the original preparation control without constructing operation authority. */
export type SourcePreparationControlV1 = Pick<ResourceLeaseControl,"check"|"advance">;
/** 📥️ Captures and rechecks original preparation facts under the same control. */
export function captureSourcePreparationV1<T>(compute:()=>T|PromiseLike<T>,control:SourcePreparationControlV1):Promise<Readonly<{value:Awaited<T>;original:readonly SourceObservationV1[];received:readonly SourceObservationV1[]}>>;
/** 🔐️ Replays each original Source preparation fact through its unchanged receiving control. */
export function receiveSourcePreparationV1(original:readonly SourceObservationV1[],control:SourcePreparationControlV1):Promise<readonly SourceObservationV1[]>;
/** 📜️ Reads the exact persisted header without consuming its result image. */
export function inferenceHeaderLine(path:string):string|null;
/** 🖋️ A persisted record header with its fact blocks expanded. */
export function readInferenceHeaderV1(path:string):{digest:string;resultSha256:string;authorityObservations:readonly SourceObservationV1[];blocks:readonly string[];observations:readonly SourceObservationV1[]};
/** ✅️ Rechecks untrusted captured facts without admitting a provider or renewing Source authority. */
export function observationsHold(observations:readonly unknown[],limit?:number,stableDirectories?:Set<string>,symlinks?:Set<string>|true,signal?:AbortSignal,onStale?:((row:SourceObservationV1)=>void)|undefined):Promise<boolean>;
/** 📊️ Exposes owned computation counts without affecting admission. */
export function projectInferenceCountsV1():Record<string,number>;
/** 🌐️ The facts one project record consumed, whether it was computed or reused in this process. */
export function projectInferenceObservationsV1(cachePath:string):readonly SourceObservationV1[]|undefined;
/** 🚀️ Runs a partial inference in project scope without an all-or-nothing round. */
export function observeInferenceSnapshotV1<T>(compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;
/** 📈️ Reports original Source and memo bookkeeping without exposing or discarding facts. */
export function sourceObservationStatsV1():Readonly<{loadObservations:number;projectImages:number;projectImageBytes:number;originalInferencePublications:number;projectObservationRows:number;captureObservations:number;recordStores:number;analysisEntries:number;analysisBytes:number}>;

export function requireCurrentObservationV1():void;

/** 🪡️ Projects finite aggregate observation budget admission DATA. */
export function observationBlockBudgetAdmitsV1(used:number,next:number,maximum:number):boolean;
/** 🧱️ Names the canonical file of one persisted observation block below an inference directory. */
export function inferenceObservationBlockPathV1(directory:string,digest:string):string;
