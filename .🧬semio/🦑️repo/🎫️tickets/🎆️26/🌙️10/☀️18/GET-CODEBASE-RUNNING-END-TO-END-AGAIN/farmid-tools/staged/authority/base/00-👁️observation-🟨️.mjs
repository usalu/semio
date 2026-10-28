import{refuseBuildV1}from"../📖️texts/🟦️.ts";
import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import{declareFormatStampV1}from"../🧷️format-stamps/🟨️.mjs";
import{renameOverBusyFileSyncV1,staleImplementationV1}from"../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {withBuildGraphOptionalPublicationV1,registerBuildGraphOptionalHandleV1,requireRepositoryBuildGraphObservationPathV1} from "../📥️inputs/💾️storage/🟨️.mjs";
import * as fs from "node:fs";
import { readFile as readFileAsync, stat as statAsync } from "node:fs/promises";
import { AsyncLocalStorage } from "node:async_hooks";
import { createHash } from "node:crypto";
import { sha256HexV1, utf8TextV1 } from "../🧂️text-bytes/🟨️.mjs";

/** 🔢️ A hex SHA-256 digest, compiled once. */
const SHA256_HEX_V1 = /^[a-f0-9]{64}$/u;
import { GRAPH_MEMORY_POLICY_V1, graphStringInternerV1, internJsonStringsV1 } from "../🧠️memory/🟨️.mjs";
import { createRowTableV1, factKeyV1 } from "./🔢️row-table/🟨️.mjs";
import { brotliCompress, brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";

/** 🗜️ Persisted inference records are brotli streams: their observation rows repeat across projects and shrink about 15 times. */
const packInferenceRecordV1=bytes=>new Promise((accept,reject)=>brotliCompress(bytes,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:bytes.length}},(error,packed)=>error?reject(error):accept(packed)));
import{createRequire}from"node:module";
import { basename, dirname, join, resolve, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { openRecordStore, recordKey, RECORD_STORE_PACK_LIMIT } from "../../⚡️caching/🗃️records/🟦️.ts";
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const observationSchemaURLV1=new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),observationSchemaCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(observationSchemaURLV1),65536,true),sourceAnalysisSchemaV1=JSON.parse(observationSchemaCaptureV1.text??""),observationSchemaRevisionV1=observationSchemaCaptureV1.sha256;
import{requireExactSourceReadBatchV1,readExactSourceFileV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";

/** 🗃️ This module's own bytes: persisted observations validated by another revision of these rules are a miss. */
const OBSERVATION_REVISION = captureBoundedSourceRevisionV1(fileURLToPath(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),8388608).sha256;

/** 🔖️ The declared stamps of the inference protocol, registered by the stamps component of each revision: a project record names the stamp of every inferrer that took part in its output instead of the bytes of their code, and a stamp row holds while that inferrer still carries the stamp it was recorded with. */
const inferenceStamps=new Map();
let stampedFragments=[];
export function registerInferenceStampsV1(entries,fragments=[]){for(const[path,stamp]of entries){if(typeof path!=="string"||!path||typeof stamp!=="string"||!stamp)throw Error("Inference stamp path and value required");inferenceStamps.set(path,stamp);}if(!Array.isArray(fragments)||fragments.some(fragment=>typeof fragment!=="string"||fragment.length<2))throw Error("Inference source fragments required");stampedFragments=[...fragments];}

/** 🏷️ The declared stamp of the persisted project record format: a record written under another stamp is a miss. */
const recordStampV1=()=>sourceAnalysisSchemaV1.$defs.record.const.stamp;
declareFormatStampV1("graph.record",sourceAnalysisSchemaV1);

/** 📌️ Records the stamp of each named inferrer in the active inference scope, so the record that is being computed names exactly the protocol versions it took part in. */
export function observedInferenceStampsV1(paths){for(const path of paths)record("stamp",path);}

/** 🧷️ Refuses a statically cached observation instance whose executing Source or privately captured schema changed. */
export function requireCurrentObservationV1(){if(implementationPinnedV1())return;try{if(captureBoundedSourceRevisionV1(fileURLToPath(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),8388608).sha256===OBSERVATION_REVISION&&captureBoundedSourceRevisionV1(fileURLToPath(observationSchemaURLV1),65536).sha256===observationSchemaRevisionV1)return;}catch(cause){throw Object.assign(staleImplementationV1("Stale loaded observation implementation; restart required"),{code:"BUILD_AUTHORITY_CHANGED",cause});}throw Object.assign(staleImplementationV1("Stale loaded observation implementation; restart required"),{code:"BUILD_AUTHORITY_CHANGED"});}

/** ⏱️ Files stamped this close to the start of a computation may still change within one timestamp tick, so their result is never persisted (racy-git rule). */
const RACY_MS = 2000;

const scopes = new AsyncLocalStorage();
const inferenceValidationScopes = new AsyncLocalStorage();
const sourceCaptures = new AsyncLocalStorage();
const recordScopes = new AsyncLocalStorage();
const recordStores = new Map();
const recordStoreHandles = new Map();
const usedInference = new Map();
const originalInferencePublications = new Map();
const loadObservations = new Map();
const projectComputations = new Map();
const projectImages = new Map();
const projectChecks = new WeakMap();
const sourceAnalyses = new Map();
const activeSourceAnalyses = new Set();
const sourceAnalysisImages = new Map();
const SOURCE_ANALYSIS_BYTES = 16 * 1024 * 1024;
let sourceAnalysisBytes = 0;
let loadSealed = false;
let authorityDigest;

/** 🧊️ Keeps the serialized image of recently inferred projects up to the declared byte bound, oldest first out: a repeated round in this process reuses a retained image, every other repeat reads the persisted record, so the images of thousands of projects never accumulate. */
const retainedProjectImages=new Set();let retainedProjectImageBytes=0;
function retainProjectImageV1(cachePath,entry){
 const previous=projectImages.get(cachePath);if(previous?.image)retainedProjectImageBytes-=previous.image.length;
 retainedProjectImages.delete(cachePath);projectImages.set(cachePath,entry);retainedProjectImages.add(cachePath);retainedProjectImageBytes+=entry.image.length;
 while(retainedProjectImageBytes>GRAPH_MEMORY_POLICY_V1.retainedImageBytes&&retainedProjectImages.size){const oldest=retainedProjectImages.values().next().value,held=projectImages.get(oldest);retainedProjectImages.delete(oldest);if(held?.image){retainedProjectImageBytes-=held.image.length;held.image=null;}}
}

/** 📊️ Exposes owned project computation counts without influencing source admission. */
export function projectInferenceCountsV1(){return Object.fromEntries(projectComputations);}

/** 📦️ The facts one project record consumed, whether it was computed or reused in this process. */
export function projectInferenceObservationsV1(cachePath){return originalInferencePublications.get(cachePath)?.observations;}

/** 🪨️ Runs a partial inference in project scope without an all-or-nothing round: every project is consistent with the facts of its own record, contradictory facts between projects are legitimate and a project that raced an edit is stale in the next verification instead of restarting the pass. */
export async function observeInferenceSnapshotV1(compute){
 requireCurrentObservationV1();loadSealed=true;
 const nested=new Map();
 return inferenceValidationScopes.run(nested,()=>scopes.run(nested,compute));
}

/** 📊️ Reports owned observation counts without exposing or discarding original source facts. */
export function sourceObservationStatsV1(){let projectObservationRows=0,projectImageBytes=0;for(const project of projectImages.values()){projectObservationRows+=project.observations.length;projectImageBytes+=project.image?.length??0;}return {loadObservations:loadObservations.size,projectImages:projectImages.size,projectImageBytes,originalInferencePublications:originalInferencePublications.size,projectObservationRows,captureObservations:sourceCaptures.getStore()?.size??0,recordStores:recordStores.size,analysisEntries:sourceAnalysisImages.size,analysisBytes:sourceAnalysisBytes};}

/** 🔏️ Registers one explicit parser revision; receipts can never provide executable analysis code. */
export function registerSourceAnalysisV1({id,revision,analyze}){
 if(typeof id!=="string"||!id.length||id.length>256||id.trim()!==id||id.includes("\0")||typeof revision!=="string"||!SHA256_HEX_V1.test(revision)||typeof analyze!=="function")throw Error("Explicit source analysis identity, revision and parser required");
 const prior=sourceAnalyses.get(id);if(prior?.provider.revision===revision){if(prior.analyze!==analyze)throw Error("Ambiguous source analysis revision registration");return prior.provider;}
 const provider=Object.freeze({id,revision});sourceAnalyses.set(id,{provider,analyze});return provider;
}

/** 🪪️ Admits only the immutable currently registered parser handle. */
function sourceAnalysisProviderV1(provider){const registered=sourceAnalyses.get(provider?.id);if(!registered||registered.provider!==provider)throw Error("Current registered source analysis provider required");return registered;}

/** 🧷️ Shares a physical validation only within the same original parent inference whose final receipt still rechecks every fact. */
/** 📁️ Directories one inference round already proved real; the round's final receipt still rechecks every recorded fact. */
const roundDirectories=new WeakMap();
const rowChecks=new WeakMap();
function projectObservationHoldsV1(row,parent){
 parent=inferenceValidationScopes.getStore()??parent;
 let checks=parent&&projectChecks.get(parent),rows=parent&&rowChecks.get(parent);if(parent&&!checks){checks=new Map();projectChecks.set(parent,checks);}if(parent&&!rows){rows=new Map();rowChecks.set(parent,rows);}
 const shared=rows?.get(row);if(shared!==undefined)return shared;
 const key=observationIdentityV1(row),fact=JSON.stringify(row.fact),previous=checks?.get(key);if(previous?.fact===fact){rows?.set(row,previous.holds);return previous.holds;}
 const holds=row.kind==="file"?fileFactHolds(row.path,row.fact)&&JSON.stringify(facts.file(row.path)?.sha256??null)===JSON.stringify(row.fact.sha256??null):current([row]);
 checks?.set(key,{fact,holds});rows?.set(row,holds);return holds;
}

/** 📛️ The identity every persisted project record of this process is written under: its record stamp and the authority digest of its module loads, equal in every process whose records another may reuse. */
export function inferenceRecordIdentityV1(){return sha256HexV1(JSON.stringify([recordStampV1(),inferenceAuthorityDigestV1()]));}

/** 🪪️ Seals module-load authority and names it by the meaning of its data facts, so project records inherit the declared policies and schemas by digest instead of by rows; every file below a fragment of the inference protocol is named by its stamp, never by its bytes. */
export function inferenceAuthorityDigestV1(){
 loadSealed=true;
 return authorityDigest??=sha256HexV1(JSON.stringify([...loadObservations.values()].filter(row=>!(row.kind==="file"&&stampedFragments.some(fragment=>row.path.includes(fragment)))).map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])).sort()));
}

/** 📦️ Admits one physical project's own input observations through its own computation, bound to the sealed authority digest. */
export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare},compute){
 requireCurrentObservationV1();requireInferenceReuseV1(reusable);originalInferencePublications.delete(cachePath);usedInference.delete(cachePath);
 const parent=scopes.getStore(),identity=sha256HexV1(JSON.stringify([recordStampV1(),inferenceAuthorityDigestV1(),key])),previous=projectImages.get(cachePath);
 const inherit=rows=>{retainSourceFactsV1(rows);retainObservationsV1(parent,rows);};
 if(previous?.identity===identity&&previous.image!==null&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){
  const value=internJsonStringsV1(JSON.parse(previous.image.toString("utf8")),graphStringInternerV1);
  if(await inferenceReusableV1(reusable,value)&&(parent?consistentObservationsV1(previous.observations):current(previous.observations))){inherit(previous.observations);originalInferencePublications.set(cachePath,previous.publication);usedInference.set(cachePath,previous.headerDigest);return value;}
 }
 const value=await observeInferenceComputationV1({cachePath,key,reusable,prepare},async()=>{projectComputations.set(owner,(projectComputations.get(owner)??0)+1);observedInferenceStampsV1(stamps);const image=jsonImage(await compute());if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");return JSON.parse(image);},Boolean(parent&&inferenceValidationScopes.getStore()));
 const publication=originalInferencePublications.get(cachePath);
 if(!publication)throw Error("Original project inference observations required");
 inherit(publication.observations);
 retainProjectImageV1(cachePath,{identity,image:Buffer.from(JSON.stringify(value)),observations:publication.observations,publication,headerDigest:usedInference.get(cachePath)??null});
 return internJsonStringsV1(value,graphStringInternerV1);
}

/** 📥️ Retains every actual source fact, including facts inherited from valid inference records. */
function retainSourceFactsV1(rows){retainObservationsV1(sourceCaptures.getStore(),rows);}

/** 🧾️ Preserves distinct consumed results for a path, including contradictory reads. */
function retainObservationsV1(target,rows){
 if(!target)return;
 if(target instanceof DetachedScopeV1&&Array.isArray(rows)&&rows.length>=DETACHED_LAYER_ROWS){target.layers.add(rows);return;}
 for(const row of rows)target.set(rowTableV1.id(row),row);
}

/** 🧅️ The scope of a detached computation: rows it observed itself, and the stored row arrays of the shared computations it reused, kept by reference so that a thousand detached computations over one closure hold that closure once. */
class DetachedScopeV1 extends Map{constructor(){super();this.layers=new Set();}}
const DETACHED_LAYER_ROWS=64;

/** 🔑️ The exact content key of one observation row, computed once per row object; flat facts avoid JSON serialisation. */
function observationKeyV1(row){return row.kind+"\0"+row.path+"\0"+factKeyV1(row.fact);}

/** 🪢️ Shares one row object between every project that observed identical bytes, so persisted headers cost memory by distinct fact, not by project. */
const rowTableV1=createRowTableV1({factKey:factKeyV1});

/** 🧾️ The JSON text of one observation row, serialised once per row object: a header costs concatenation, not the repeated serialisation of rows that thousands of projects share. */
const observationJsons=new WeakMap();
function observationRowsJsonV1(rows){return "["+rows.map(row=>{let text=observationJsons.get(row);if(text===undefined){text=JSON.stringify(row);observationJsons.set(row,text);}return text;}).join(",")+"]";}
function internObservationsV1(rows){return rows.map(row=>rowTableV1.canonical(row.kind,row.path,row.fact,()=>{if(typeof row.path==="string")row.path=graphStringInternerV1.share(row.path);return row;}));}
function canonicalObservationV1(kind,path,fact){return rowTableV1.canonical(kind,path,fact,()=>({kind,path:graphStringInternerV1.share(path),fact}));}
function recordRowV1(row){
 const capture=sourceCaptures.getStore(),target = scopes.getStore() ?? (loadSealed||capture ? undefined : loadObservations);
 if(!target&&!capture)return;
 retainObservationsV1(target,[row]);retainSourceFactsV1([row]);
}

/** 🤝️ Validates a persisted project's facts through the original inference's shared physical checks, so a fact that many projects consumed is probed once per inference while the final receipt still rechecks all of them. */
const sharedChecks=new WeakMap();
function sharedObservationsHoldV1(rows,parent){
 parent=inferenceValidationScopes.getStore()??parent;if(!parent)return current(rows);
 let checks=sharedChecks.get(parent);if(!checks){checks=new Map();sharedChecks.set(parent,checks);}
 return consistentObservationsV1(rows)&&rows.every(row=>{const key=rowTableV1.id(row),previous=checks.get(key);if(previous!==undefined)return previous;const holds=current([row]);checks.set(key,holds);return holds;});
}

/** 🪸️ The verdict on the stored rows of one shared computation, remembered per validation round by array identity, so thousands of detached or nested users of one closure validate its rows once. */
const memoRowChecksV1=new WeakMap();
function memoRowsKnownV1(rows){if(rows.length<DETACHED_LAYER_ROWS)return undefined;const round=inferenceValidationScopes.getStore();return round?memoRowChecksV1.get(round)?.get(rows):undefined;}
function memoRowsRememberV1(rows,holds){if(rows.length<DETACHED_LAYER_ROWS)return;const round=inferenceValidationScopes.getStore();if(!round)return;let checks=memoRowChecksV1.get(round);if(!checks)memoRowChecksV1.set(round,checks=new WeakMap());checks.set(rows,holds);}
function memoRowsHoldV1(rows,parent){const known=memoRowsKnownV1(rows);if(known!==undefined)return known;const holds=rows.every(row=>projectObservationHoldsV1(row,parent));memoRowsRememberV1(rows,holds);return holds;}

/** 🧵️ Shares validated invocation work while replaying every consumed fact into each receiving project. */
export function memoizedObservedComputationV1(cache,key,compute) {
 const parent=scopes.getStore(),previous=cache.get(key);
 if(previous&&memoRowsHoldV1(previous.observations,parent)){retainObservationsV1(parent,previous.observations);retainSourceFactsV1(previous.observations);return previous.value;}
 const nested=new Map();let stored;
 try{const value=scopes.run(nested,compute);if(value&&typeof value.then==="function")throw Error("Synchronous observed computation required");stored=[...nested.values()];if(value!==null)cache.set(key,{value,observations:stored});return value;}
 finally{const rows=stored??[...nested.values()];retainObservationsV1(parent,rows);retainSourceFactsV1(rows);}
}

const validatedAsyncDeliveriesV1=new WeakMap();

/** 🪬️ Reuses only a fully delivered validated hit in the exact live receiving frames. */
function validatedAsyncDeliveryV1(entry,parent,capture,validation,remember=false){
 if(!parent||!capture||!validation)return false;
 let parents=validatedAsyncDeliveriesV1.get(entry);if(!parents){if(!remember)return false;validatedAsyncDeliveriesV1.set(entry,parents=new WeakMap());}
 let captures=parents.get(parent);if(!captures){if(!remember)return false;parents.set(parent,captures=new WeakMap());}
 let validations=captures.get(capture);if(!validations){if(!remember)return false;captures.set(capture,validations=new WeakSet());}
 const delivered=validations.has(validation);if(remember)validations.add(validation);return delivered;
}

/** 🌊️ Retains complete observations while an original caller yields asynchronous inference. */
export async function memoizedObservedAsyncComputationV1(cache,key,compute,checkpoint) {
 if(typeof checkpoint!=="function")throw Error("Original asynchronous inference checkpoint required");
 const parent=scopes.getStore(),capture=sourceCaptures.getStore(),validation=inferenceValidationScopes.getStore()??parent,previous=cache.get(key);
 if(previous){if(validatedAsyncDeliveryV1(previous,parent,capture,validation)){await checkpoint(true);return previous.value;}let holds=true;const known=memoRowsKnownV1(previous.observations);if(known!==undefined)holds=known;else{for(const row of previous.observations){await checkpoint();if(!projectObservationHoldsV1(row,parent)){holds=false;break;}}memoRowsRememberV1(previous.observations,holds);}if(holds){retainObservationsV1(parent,previous.observations);retainSourceFactsV1(previous.observations);validatedAsyncDeliveryV1(previous,parent,capture,validation,true);return previous.value;}}
 const nested=new Map();let stored;
 try{const value=await scopes.run(nested,compute);stored=[...nested.values()];if(value!==null)cache.set(key,{value,observations:stored});return value;}
 finally{const rows=stored??[...nested.values()];retainObservationsV1(parent,rows);retainSourceFactsV1(rows);}
}

/** ⚖️ Refuses mixed source images while allowing identical bytes with different file stamps. */
const observationIdentities=new WeakMap(),observationMeanings=new WeakMap();
function observationIdentityV1(row){let value=observationIdentities.get(row);if(value===undefined){value=row.kind+"\0"+row.path+(row.kind==="analysis"?"\0"+row.fact?.provider:"");observationIdentities.set(row,value);}return value;}

/** 🧠️ Distinguishes semantic results independently of their original source byte hints. */
function observationMeaningV1(row){let value=observationMeanings.get(row);if(value===undefined){value=JSON.stringify(row.kind==="file"&&row.fact?.sha256?{sha256:row.fact.sha256}:row.kind==="analysis"?{provider:row.fact?.provider,revision:row.fact?.revision,resultSha256:row.fact?.resultSha256}:row.kind==="resolve"?canonicalResolutionV1(row.fact):row.fact);observationMeanings.set(row,value);}return value;}

/** 🧨️ Names the first contradictory source read and whether it contradicts sealed module-load authority, which no re-inference in this process can repair. */
function conflictingInferenceErrorV1(rows){
 const seen=new Map(),authority=new Set([...loadObservations.values()].map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])));
 for(const row of rows){const key=observationIdentityV1(row),meaning=observationMeaningV1(row);if(seen.has(key)&&seen.get(key)!==meaning){const loaded=authority.has(JSON.stringify([key,seen.get(key)]))||authority.has(JSON.stringify([key,meaning]));return Object.assign(Error(loaded?"Build graph implementation authority changed during inference; restart the owned build invocation: "+row.kind+":"+row.path:"Conflicting inference observations: "+row.kind+":"+row.path),{code:loaded?"BUILD_AUTHORITY_CHANGED":"BUILD_SOURCE_CONFLICT",path:row.path});}seen.set(key,meaning);}
 return Object.assign(Error("Conflicting inference observations"),{code:"BUILD_SOURCE_CONFLICT"});
}

/** ⚖️ Refuses conflicting results from the same source consumer. */
function consistentObservationsV1(rows){const seen=new Map();for(const row of rows){const key=observationIdentityV1(row),fact=observationMeaningV1(row);if(seen.has(key)&&seen.get(key)!==fact)return false;seen.set(key,fact);}return true;}

/** 🕸️ Captures exact inference dependencies independently of nested observation scopes. */
export async function captureObservedSourceInputsV1(compute,onFailure,includeAuthority=true){
 if(typeof includeAuthority!=="boolean")throw Error("Explicit Source authority capture required");const parent=sourceCaptures.getStore(),captured=new Map(),observations=()=>{if(includeAuthority)retainObservationsV1(captured,loadObservations.values());return [...captured.values()];};
 try{return {value:await sourceCaptures.run(captured,compute),observations:observations(),authorityObservations:[...loadObservations.values()]};}
 catch(error){onFailure?.(observations(),[...loadObservations.values()]);throw error;}
 finally{retainObservationsV1(parent,captured.values());captured.clear();}
}

/** 🔏️ Records only the exact bytes read by the original live bounded Source batch. */
export async function observedExactSourceFileV1(batch,root,name){requireExactSourceReadBatchV1(batch,root);const value=await readExactSourceFileV1(batch,name);requireExactSourceReadBatchV1(batch,root);if(value)record("file",resolve(root,name),{size:value.byteLength,mtimeMs:0,ctimeMs:0,ino:0,sha256:value.sha256});return value;}

/** 🧭️ Normalizes every accepted fs path argument to one absolute key. */
function absolute(path) {
  return resolve(path instanceof URL ? fileURLToPath(path) : typeof path === "string" ? path : String(path));
}

/** 🏷️ The node type a stat result denotes, the only stat fact graph inference reads. */
function nodeType(stat) {
  return stat.isSymbolicLink() ? "symlink" : stat.isFile() ? "file" : stat.isDirectory() ? "directory" : "other";
}

/** 🧾️ The identity of one file's bytes: stamp plus content digest, or the error code that replaced them. */
function fileFact(path) {
  try {
    const stat = fs.statSync(path);
    return { size: stat.size, mtimeMs: stat.mtimeMs, ctimeMs: stat.ctimeMs, ino: stat.ino, sha256: sha256HexV1(fs.readFileSync(path)) };
  } catch (error) { return { error: error.code ?? "EUNKNOWN" }; }
}

/** 🧬️ Captures the bytes actually passed to a parser, with stamps retained solely as validation hints. */
function sourceFactV1(stamp,bytes){return {size:bytes.length,mtimeMs:stamp instanceof Error?0:stamp.mtimeMs,ctimeMs:stamp instanceof Error?0:stamp.ctimeMs,ino:stamp instanceof Error?0:stamp.ino,sha256:sha256HexV1(bytes)};}

/** 🔬️ Runs a synchronous JSON parser under its own complete nested observation scope. */
function analyzeSourceV1(registered,path,bytes){
 const key=JSON.stringify([registered.provider.id,registered.provider.revision,path]);if(activeSourceAnalyses.has(key))throw Error("Recursive source analysis refused");
 const nested=new Map(),parent=scopes.getStore();activeSourceAnalyses.add(key);
 try{const image=jsonImage(scopes.run(nested,()=>registered.analyze(path,bytes)));if(image===undefined)throw Error("Source analysis requires a synchronous JSON image");return {image,nestedObservations:[...nested.values()]};}finally{activeSourceAnalyses.delete(key);retainObservationsV1(parent,nested.values());retainSourceFactsV1(nested.values());}
}

/** 🪶️ Bounds reusable JSON parser results without retaining source buffers or parser trees. */
function sourceAnalysisImageV1(registered,path,bytes,sha256){
 const records=recordScopes.getStore()?.records,key=recordKey("source-analysis-v1",registered.provider.id,registered.provider.revision,path,sha256),previous=sourceAnalysisImages.get(key);let result;
 if(previous){sourceAnalysisImages.delete(key);sourceAnalysisBytes-=previous.bytes;if(current(previous.result.nestedObservations))result=previous.result;}
 const hit=result?undefined:records?.get(key);
 if(hit!==undefined)try{const candidate=JSON.parse(utf8TextV1(hit));if(typeof candidate.image==="string"&&jsonImage(JSON.parse(candidate.image))===candidate.image&&Array.isArray(candidate.nestedObservations)&&current(candidate.nestedObservations))result=candidate;}catch{}
 if(!result)result=analyzeSourceV1(registered,path,bytes);
 const size=Buffer.byteLength(result.image)+Buffer.byteLength(JSON.stringify(result.nestedObservations))+192;
 if(size<=SOURCE_ANALYSIS_BYTES){while(sourceAnalysisBytes+size>SOURCE_ANALYSIS_BYTES){const oldest=sourceAnalysisImages.keys().next().value;sourceAnalysisBytes-=sourceAnalysisImages.get(oldest).bytes;sourceAnalysisImages.delete(oldest);}sourceAnalysisImages.set(key,{result,bytes:size});sourceAnalysisBytes+=size;records?.put(key,Buffer.from(JSON.stringify(result)));}
 return result;
}

/** 📦️ Reuses semantic images only while every nested dependency remains current. */
function observeSourceBytesV1(registered,path,bytes,source){
 const result=sourceAnalysisImageV1(registered,path,bytes,source.sha256);
 retainObservationsV1(scopes.getStore(),result.nestedObservations);retainSourceFactsV1(result.nestedObservations);
 const fact={provider:registered.provider.id,revision:registered.provider.revision,source,resultSha256:sha256HexV1(result.image),nestedObservations:result.nestedObservations};record("analysis",path,fact);return JSON.parse(result.image);
}

/** 📖️ Observes one parser's semantics without asserting that unrelated source body bytes remain unchanged. */
export function observedSourceAnalysisV1(provider,path){
 const registered=sourceAnalysisProviderV1(provider),key=absolute(path);let stamp,bytes;
 try{stamp=fs.statSync(path);}catch(error){stamp=error;}
 try{bytes=fs.readFileSync(path);}catch(error){record("file",key,{error:error.code??"EUNKNOWN"});throw error;}
 const source=sourceFactV1(stamp,bytes);try{return observeSourceBytesV1(registered,key,bytes,source);}catch(error){record("file",key,source);throw error;}
}

/** 🌊️ Reads parser input asynchronously while preserving synchronous parser and nested probe semantics. */
export async function observedSourceAnalysisAsyncV1(provider,path){
 const registered=sourceAnalysisProviderV1(provider),key=absolute(path);let stamp,bytes;
 try{stamp=await statAsync(path);}catch(error){stamp=error;}
 try{bytes=await readFileAsync(path);}catch(error){record("file",key,{error:error.code??"EUNKNOWN"});throw error;}
 const source=sourceFactV1(stamp,bytes);try{return observeSourceBytesV1(registered,key,bytes,source);}catch(error){record("file",key,source);throw error;}
}

/** 🤝️ Validates the facts one analysis consumed through the round's shared checks, so a fact that thousands of analyses consumed is probed once per inference round. */
const nestedChecks=new WeakMap();
function nestedObservationsHoldV1(rows){
 const round=inferenceValidationScopes.getStore();if(!round)return current(rows);
 let checks=nestedChecks.get(round);if(!checks){checks=new Map();nestedChecks.set(round,checks);}
 return consistentObservationsV1(rows)&&rows.every(row=>{const key=rowTableV1.id(row),known=checks.get(key);if(known!==undefined)return known;const holds=current([row]);checks.set(key,holds);return holds;});
}

/** 🔁️ Replays current registered parsers after byte changes and retains every nested dependency; a file whose size, inode and both stamps are those of the recorded source is the recorded source, exactly as for a `file` fact, and is not read again. */
function sourceAnalysisFactHoldsV1(path,fact){
 if(sourceAnalysisContractV1.validate(fact).length)return false;
 const registered=sourceAnalyses.get(fact?.provider);
 if(!registered||registered.provider.revision!==fact.revision||!Array.isArray(fact.nestedObservations)||!fact.source||!SHA256_HEX_V1.test(fact.source.sha256??"")||!SHA256_HEX_V1.test(fact.resultSha256??""))return false;
 if(!nestedObservationsHoldV1(fact.nestedObservations))return false;
 try{
  const stamp=fs.statSync(path),recorded=fact.source;if(stamp.size===recorded.size&&stamp.mtimeMs===recorded.mtimeMs&&stamp.ctimeMs===recorded.ctimeMs&&stamp.ino===recorded.ino)return true;
  const bytes=fs.readFileSync(path),sha256=sha256HexV1(bytes);if(sha256===fact.source.sha256)return true;
  const replay=sourceCaptures.run(undefined,()=>recordScopes.run(undefined,()=>scopes.run(undefined,()=>sourceAnalysisImageV1(registered,path,bytes,sha256))));
  if(sha256HexV1(replay.image)!==fact.resultSha256||!current(replay.nestedObservations))return false;
  const before=new Set(fact.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)]))),after=new Set(replay.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])));
  return before.size===after.size&&[...before].every(row=>after.has(row));
 }catch{return false;}
}

/** 📁️ Fingerprints exactly the names or typed entries a directory read returned. */
function directoryFactV1(entries,typed=false){return sha256HexV1(JSON.stringify(entries.map(entry=>typed?[String(entry.name),nodeType(entry)]:String(entry)).sort()));}

/** 🔤️ Decodes the BOM forms accepted by the TypeScript host without another runtime dependency. */
function typescriptTextV1(bytes){if(bytes.length>=2&&bytes[0]===254&&bytes[1]===255){const swapped=Buffer.from(bytes);for(let index=0;index+1<swapped.length;index+=2){const byte=swapped[index];swapped[index]=swapped[index+1];swapped[index+1]=byte;}return swapped.toString("utf16le",2);}if(bytes.length>=2&&bytes[0]===255&&bytes[1]===254)return bytes.toString("utf16le",2);return bytes.toString("utf8",bytes.length>=3&&bytes[0]===239&&bytes[1]===187&&bytes[2]===191?3:0);}

/** 📂️ Replays the host's sorted immediate directories, following directory symlinks. */
function typescriptDirectoriesV1(path){try{return fs.readdirSync(path||".",{withFileTypes:true}).filter(entry=>{try{return(entry.isSymbolicLink()?fs.statSync(join(path,entry.name)):entry).isDirectory();}catch{return false;}}).map(entry=>entry.name).sort();}catch{return[];}}

/** 🔗️ The seam of the shared command closure entries: the inferrer of command sources registers how the answer of a `closure` fact is read now (`answer`, synchronous: an answer is only given while the rows the entry was computed from still hold) and how the entries named by facts are brought up to date (`settle`, asynchronous, called before facts are checked, with the value of the record the facts belong to, which carries the recipes they name). A `closure` fact is a question put to an entry of the closure of a command, never a copy of its rows. */
let closureProviderV1;
export function registerClosureProviderV1(provider){
 if(!provider||typeof provider.answer!=="function"||typeof provider.settle!=="function")throw refuseBuildV1("invalid","observation.closure-provider-incomplete");
 closureProviderV1=provider;
}
/** 🛎️ Brings the closure entries named by the `closure` facts among `rows` up to date, so the synchronous checks that follow can answer. */
export async function settleClosureRowsV1(rows,value){
 if(!closureProviderV1)return;
 const paths=new Set();for(const row of rows)if(row.kind==="closure")paths.add(row.path);
 if(paths.size)await closureProviderV1.settle([...paths],value);
}
/** 🪡️ Records the answer one closure entry gave to the running inference. */
export function recordClosureFactV1(path,answer){record("closure",path,answer);}
/** 📏️ The entry counts of the in-process caches a graph construction fills (what {@link releaseInferenceMemoryV1} drops). */
export function inferenceCacheSizesV1(){return {interned:rowTableV1.size(),verifiedBlockRows,settledReads:settledReads.size,projectImages:projectImages.size,publications:originalInferencePublications.size,registryFiles:registryOwnersV1.size};}
/** 🕊️ Drops every in-process cache a graph construction filled (interned rows, verified blocks, project images and publications, settled reads, analysis images, registry rows): they only save recomputation, a process that keeps them while it idles (a loader through a long Bazel phase) pins gigabytes. Safe only when no inference runs; the next inference refills what it needs. */
export function releaseInferenceMemoryV1(){
 rowTableV1.clear();verifiedBlocks.clear();verifiedBlockRows=0;blocksByIdentity.clear();blocksRefused.clear();
 settledReads.clear();settledReadBytes=0;fileRecordImages.clear();fileRecordImageBytes=0;sourceAnalysisImages.clear();sourceAnalysisBytes=0;
 projectImages.clear();retainedProjectImages.clear();retainedProjectImageBytes=0;usedInference.clear();originalInferencePublications.clear();registryOwnersV1.clear();
}
/** 📒️ The owner rows of the committed owner registry by name, parsed once per file stamp: the registry is a question the inference asks one owner at a time, never a file it depends on as a whole. */
const registryOwnersV1=new Map();
function registryOwnerRowV1(path){
 let workspaceRoot,name;try{[workspaceRoot,name]=JSON.parse(path);}catch{return "!unreadable";}
 const file=join(workspaceRoot,"📋️build-owners.json");let status;
 try{status=fs.statSync(file,{bigint:true});}catch(error){return `!${error.code??"EUNKNOWN"}`;}
 const stamp=status.size+":"+status.mtimeNs+":"+status.ino;let known=registryOwnersV1.get(file);
 if(known?.stamp!==stamp){
  let owners;try{owners=new Map(JSON.parse(fs.readFileSync(file,"utf8")).owners.map(owner=>[owner.name,JSON.stringify({root:owner.root,manifest:owner.manifest})]));}catch{return "!unreadable";}
  known={stamp,owners};registryOwnersV1.set(file,known);
 }
 return known.owners.get(name)??"null";
}
/** 🪪️ Records the registry row (root and manifest) one inference read for one owner name, or its absence. */
export function recordRegistryOwnerV1(workspaceRoot,name,row){record("registry",JSON.stringify([workspaceRoot,name]),row===undefined||row===null?"null":JSON.stringify({root:row.root,manifest:row.manifest}));}
/** 🪢️ Whether recorded rows still hold, sharing the checks of the running inference round. */
export function observationRowsHoldV1(rows){return sharedObservationsHoldV1(rows,scopes.getStore());}
/** 🔭️ The running validation round, if any; an entry validated once in a round stays valid for it. */
export function observationRoundV1(){return inferenceValidationScopes.getStore();}
/** 🔂️ Runs `compute` inside one validation round (the round already open when there is one): the verdict on a stored fact or a memoized computation is reached once per round instead of once per consumer, as a snapshot pass does; a fact that changes during the round is stale in the next verification. */
export function withObservationRoundV1(compute){return inferenceValidationScopes.run(inferenceValidationScopes.getStore()??new Map(),compute);}
/** 🫧️ Runs `compute` with its observations detached from the running inference: they are returned, not recorded into the project that asked, so a shared computation (the closure of a command) is stored once and every project that uses it records one question instead of its rows. */
export async function observeDetachedV1(compute){
 const rows=new DetachedScopeV1(),captured=new DetachedScopeV1(),taken=()=>({rows:[...rows.values()],layers:[...new Set([...rows.layers,...captured.layers])]});
 try{const value=await sourceCaptures.run(captured,()=>scopes.run(rows,compute));return{value,...taken()};}
 catch(error){if(error!==null&&typeof error==="object"&&Object.isExtensible(error))Object.defineProperty(error,DETACHED_FAILURE_V1,{value:taken(),configurable:true});throw error;}
}

/** 🏷️ Names the property under which a failed detached computation hands the facts it read to its caller. */
const DETACHED_FAILURE_V1=Symbol.for("semio.observation.detached-failure");

/** 🧾️ The facts a detached computation read before it failed (rows and shared layers), or undefined for an error that carries none: they are the reason of the failure, so whoever withholds work because of it must hold them. */
export function detachedFailureV1(error){return error!==null&&typeof error==="object"?error[DETACHED_FAILURE_V1]:undefined;}

/** 🧲️ Records the facts of a failed detached computation into the observation the caller runs in, so a result built on that failure goes stale exactly when one of the facts it rests on changes. */
export function retainDetachedFailureV1(error){
 const failure=detachedFailureV1(error);if(!failure)return;
 const rows=[...failure.rows,...failure.layers.flat()];
 retainObservationsV1(scopes.getStore(),rows);retainSourceFactsV1(rows);
}
/** 🧲️ Whether the shared rows of one reused computation still hold, checked once per validation round however many detached computations share them. */
export function observationLayerHoldsV1(layer){return memoRowsHoldV1(layer,scopes.getStore());}

/** 🧭️ The spelling in which a module resolution is recorded: the id of the dependency generation inside `.cache/b/d/m/j/<id>/` is a placeholder, so a new generation of the same packages does not stale every resolution of the repository's own command sources. */
const DEPENDENCY_GENERATION_PATH_V1=/([\\/]\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/])[0-9a-f]{16,64}(?=[\\/])/u;
export function canonicalResolutionV1(path){return typeof path==="string"?path.replace(DEPENDENCY_GENERATION_PATH_V1,"$1*"):path;}

/** ⚖️ Whether a recorded fact equals the freshly recomputed one; recorded module resolutions are compared in their canonical spelling. */
function recordedFactHoldsV1(kind,path,fact){return JSON.stringify(facts[kind](path))===JSON.stringify(kind==="resolve"?canonicalResolutionV1(fact):fact);}

/** 🔎️ Recomputes one observation's fact exactly as it was first taken. */
const facts = {
  stamp: (path) => inferenceStamps.get(path) ?? "!unregistered",
  analysis: (path,fact) => sourceAnalysisFactHoldsV1(path,fact),
  file: (path) => fileFact(path),
  exists: (path) => fs.existsSync(path),
  stat: (path) => { try { return nodeType(fs.statSync(path)); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  lstat: (path) => { try { return nodeType(fs.lstatSync(path)); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  readdir: (path) => { try { return directoryFactV1(fs.readdirSync(path, { withFileTypes: true }),true); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  readdirNames: (path) => {try{return directoryFactV1(fs.readdirSync(path));}catch(error){return `!${error.code??"EUNKNOWN"}`;}},
  fileExists: (path) => {try{return fs.statSync(path).isFile();}catch{return false;}},
  directoryExists: (path) => {try{return fs.statSync(path).isDirectory();}catch{return false;}},
  directories: (path) => directoryFactV1(typescriptDirectoriesV1(path)),
  typescriptText: (path) => {try{return sha256HexV1(typescriptTextV1(fs.readFileSync(path)));}catch{return "!missing";}},
  typescriptRealpath: (path) => {try{return path.length<260?fs.realpathSync.native(path):fs.realpathSync(path);}catch{return path;}},
  realpath: (path) => { try { return fs.realpathSync(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  realpathNative: (path) => { try { return fs.realpathSync.native(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  closure: (path) => closureProviderV1 ? closureProviderV1.answer(path) : "!unregistered",
  registry: (path) => registryOwnerRowV1(path),
  resolve: (key) => { const [from, request] = JSON.parse(key); try { return canonicalResolutionV1(createRequire(from).resolve(request)); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
};

/** ✍️ Records one fact in the active inference scope, or — while modules evaluate before the first inference and outside every capture — in the module-load authority; captured reads outside a scope belong to their capture only. */
function record(kind, path, fact) {
  const capture=sourceCaptures.getStore(),target = scopes.getStore() ?? (loadSealed||capture ? undefined : loadObservations);
  if(!target&&!capture)return;
  const row=canonicalObservationV1(kind,path,fact===undefined?facts[kind](path):fact);
  retainObservationsV1(target,[row]);retainSourceFactsV1([row]);
}

/** 🔬️ Runs one probe and records the fact its own outcome proves, so observing costs no second syscall. */
function probe(kind, path, run, fact) {
  let value;
  try { value = run(); } catch (error) { record(kind, absolute(path), `!${error.code ?? "EUNKNOWN"}`); throw error; }
  record(kind, absolute(path), fact(value));
  return value;
}

/** 🔤️ The encoding a `readFileSync`/`readFile` options argument selects, if any. */
function encodingOf(options) {
  return typeof options === "string" ? options : options?.encoding ?? undefined;
}

/** 📖️ Reads bytes once: the stamp is taken first and the digest from the very bytes returned, so a change racing the read can only make the fact stale, never wrong. */
function readObserved(path, stamp, bytes, options) {
  const fact = sourceFactV1(stamp,bytes);
  record("file", absolute(path), fact);
  const encoding = encodingOf(options);
  settleRead(absolute(path), stamp, bytes, fact, encoding !== undefined);
  return encoding ? bytes.toString(encoding) : bytes;
}

/** 🧊️ Source bytes whose stamp was already settled when read, so an unchanged file is read and hashed once while its stamp still matches. */
const settledReads = new Map(), SETTLED_READ_BYTES = 128 * 1024 * 1024;

/** 🧠️ The JSON image of the analysis of a settled file, per namespace and path, while the file keeps its stamp: the analysis is a pure function of the bytes, so a file that many projects ask about is read, hashed, parsed and stored once and every later asker parses the image. Bounded like the settled reads. */
const fileRecordImages = new Map(), FILE_RECORD_IMAGE_BYTES = 64 * 1024 * 1024, FILE_RECORD_IMAGE_LIMIT = 1024 * 1024;
let fileRecordImageBytes = 0;
function rememberFileRecordImageV1(memoKey, key, image) {
  const settled = settledReads.get(key), previous = fileRecordImages.get(memoKey);
  if (previous) { fileRecordImageBytes -= previous.image.length; fileRecordImages.delete(memoKey); }
  if (settled === undefined || image.length > FILE_RECORD_IMAGE_LIMIT) return;
  while (fileRecordImageBytes + image.length > FILE_RECORD_IMAGE_BYTES && fileRecordImages.size > 0) {
    const oldest = fileRecordImages.keys().next().value;
    fileRecordImageBytes -= fileRecordImages.get(oldest).image.length;
    fileRecordImages.delete(oldest);
  }
  fileRecordImages.set(memoKey, { size: settled.size, mtimeMs: settled.mtimeMs, ctimeMs: settled.ctimeMs, ino: settled.ino, row: settled.row, image });
  fileRecordImageBytes += image.length;
}
function rememberedFileRecordImageV1(memoKey, path) {
  const known = fileRecordImages.get(memoKey);
  if (known === undefined) return undefined;
  let stamp;
  try { stamp = fs.statSync(path); } catch {}
  if (stamp && stamp.size === known.size && stamp.mtimeMs === known.mtimeMs && stamp.ctimeMs === known.ctimeMs && stamp.ino === known.ino) return known;
  fileRecordImageBytes -= known.image.length;
  fileRecordImages.delete(memoKey);
  return undefined;
}
let settledReadBytes = 0;
function settleRead(key, stamp, bytes, fact, owned = false) {
  if (stamp instanceof Error || !Buffer.isBuffer(bytes) || bytes.length > 4 * 1024 * 1024) return;
  const taken = Date.now();
  if (!(stamp.mtimeMs < taken - RACY_MS && stamp.ctimeMs < taken - RACY_MS)) { const previous = settledReads.get(key); if (previous) { settledReadBytes -= previous.bytes.length; settledReads.delete(key); } return; }
  const previous = settledReads.get(key); if (previous) settledReadBytes -= previous.bytes.length;
  if (settledReadBytes + bytes.length > SETTLED_READ_BYTES) { settledReads.clear(); settledReadBytes = 0; }
  settledReads.set(key, { size: stamp.size, mtimeMs: stamp.mtimeMs, ctimeMs: stamp.ctimeMs, ino: stamp.ino, bytes: owned ? bytes : Buffer.from(bytes), row: canonicalObservationV1("file", key, fact) }); settledReadBytes += bytes.length;
}
function settledRead(path, stamp, options) {
  if (stamp instanceof Error) return undefined;
  const key = absolute(path), hit = settledReads.get(key);
  if (!hit || hit.size !== stamp.size || hit.mtimeMs !== stamp.mtimeMs || hit.ctimeMs !== stamp.ctimeMs || hit.ino !== stamp.ino) return undefined;
  recordRowV1(hit.row);
  const encoding = encodingOf(options);
  return { value: encoding ? hit.bytes.toString(encoding) : Buffer.from(hit.bytes) };
}

/** 👁️ The `node:fs` reads graph inference may perform, each recording what it observed so a persisted result can later be proven current. Writes stay on `node:fs`. */
export const observedFs = Object.freeze({
  readFileSync(path, options) {
    let stamp;
    try { stamp = fs.statSync(path); } catch (error) { stamp = error; }
    const settled = settledRead(path, stamp, options); if (settled) return settled.value;
    let bytes;
    try { bytes = fs.readFileSync(path); } catch (error) { record("file", absolute(path), { error: error.code ?? "EUNKNOWN" }); throw error; }
    return readObserved(path, stamp, bytes, options);
  },
  existsSync(path) {
    const key = absolute(path);
    const value = fs.existsSync(path);
    record("exists", key, value);
    return value;
  },
  statSync(path, options) { return probe("stat", path, () => fs.statSync(path, options), (value) => value ? nodeType(value) : "!ENOENT"); },
  lstatSync(path, options) {
    const round = options === undefined ? inferenceValidationScopes.getStore() : undefined;
    if (!round) return probe("lstat", path, () => fs.lstatSync(path, options), (value) => value ? nodeType(value) : "!ENOENT");
    let directories = roundDirectories.get(round); if (!directories) { directories = new Map(); roundDirectories.set(round, directories); }
    const key = absolute(path), known = directories.get(key);
    if (known) { recordRowV1(known.row); return known.value; }
    const value = probe("lstat", path, () => fs.lstatSync(path), (value) => value ? nodeType(value) : "!ENOENT");
    if (value.isDirectory()) directories.set(key, { value, row: canonicalObservationV1("lstat", key, "directory") });
    return value;
  },
  readdirSync(path, options) {const typed=options?.withFileTypes===true;return probe(typed?"readdir":"readdirNames",path,()=>fs.readdirSync(path,options),value=>directoryFactV1(value,typed));},
  realpathSync: Object.assign((path, options) => probe("realpath", path, () => fs.realpathSync(path, options), (value) => String(value)), { native: (path, options) => probe("realpathNative", path, () => fs.realpathSync.native(path, options), (value) => String(value)) }),
  async readFile(path, options) {
    let stamp;
    try { stamp = await statAsync(path); } catch (error) { stamp = error; }
    const settled = settledRead(path, stamp, options); if (settled) return settled.value;
    let bytes;
    try { bytes = await readFileAsync(path); } catch (error) { record("file", absolute(path), { error: error.code ?? "EUNKNOWN" }); throw error; }
    return readObserved(path, stamp, bytes, options);
  },
});

if(!import.meta.semioFrozenAuthorizers){
 observedFs.readFileSync(new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)));
 observedFs.readFileSync(new URL("../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)));
}
const sourceAnalysisContractV1=compileJsonSchemaSubset(sourceAnalysisSchemaV1.$defs.sourceAnalysis,sourceAnalysisSchemaV1);

/** 🧷️ `createRequire(from).resolve(request)` with its outcome recorded, so a persisted result is invalidated when module resolution would now answer differently. */
export function observedRequireResolve(from, request) {
  const key = JSON.stringify([absolute(from), request]);
  try {
    const resolved = createRequire(from).resolve(request);
    record("resolve", key, canonicalResolutionV1(resolved));
    return resolved;
  } catch (error) { record("resolve", key, `!${error.code ?? "EUNKNOWN"}`); throw error; }
}

/** 🧩️ A TypeScript module resolution host that delegates to TypeScript's own `sys` and records every probe it answers. */
export function observedTypeScriptHost(sys) {
  return {
    ...sys,
    fileExists: (path) => probe("fileExists",path,()=>sys.fileExists(path),value=>value),
    directoryExists: (path) => probe("directoryExists",path,()=>sys.directoryExists(path),value=>value),
    readFile: (path, encoding) => probe("typescriptText",path,()=>sys.readFile(path,encoding),value=>value===undefined?"!missing":sha256HexV1(value)),
    realpath: sys.realpath && ((path) => probe("typescriptRealpath",path,()=>sys.realpath(path),value=>value)),
    getDirectories: (path) => probe("directories",path,()=>sys.getDirectories(path),value=>directoryFactV1(value)),
  };
}

/** ⚖️ Structural equality that distinguishes absent from `undefined`, so only results that survive JSON exactly are persisted. */
function jsonExact(value) {
  if (value === null || typeof value === "string" || typeof value === "boolean") return true;
  if (typeof value === "number") return Number.isFinite(value);
  if (Array.isArray(value)) return value.every((item, index) => index in value && jsonExact(item));
  if (typeof value !== "object" || Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null) return false;
  return Object.values(value).every(jsonExact);
}

/** 🪞️ The JSON image of a value when it is built only from plain objects, arrays, strings, booleans, finite numbers and `null` (object
 * properties holding `undefined` are dropped, exactly as a consumer reading them sees `undefined`), or `undefined` when it is not. */
function jsonImage(value) {
  const representable = (item) => item === null || typeof item === "string" || typeof item === "boolean" || typeof item === "number" && Number.isFinite(item) || Array.isArray(item) && item.every((element, index) => index in item && element !== undefined && representable(element)) || typeof item === "object" && !Array.isArray(item) && (Object.getPrototypeOf(item) === Object.prototype || Object.getPrototypeOf(item) === null) && Object.values(item).every((entry) => entry === undefined || representable(entry));
  return representable(value) ? JSON.stringify(value) : undefined;
}

/**
 * 🧠️ Memoizes a pure per-content analysis across inference runs: inside {@link observeInference} the result for `namespace` (which must
 * name the analysis code revision) and `inputs` (every byte it reads) is kept in a content-addressed record store, and both a hit and a miss
 * return the value's JSON image so they are indistinguishable. Outside an inference, or for a value without a JSON image, it computes.
 * @template T
 * @param {string} namespace
 * @param {readonly (string | Uint8Array)[]} inputs
 * @param {() => T} compute
 * @returns {T}
 */
export function memoizedRecord(namespace, inputs, compute) {
  const store = recordScopes.getStore()?.records;
  if (!store) return compute();
  const key = recordKey(namespace, ...inputs), hit = store.get(key);
  if (hit !== undefined) { try { return JSON.parse(utf8TextV1(hit)); } catch {} }
  const value = compute(), image = jsonImage(value);
  if (image === undefined) return value;
  store.put(key, Buffer.from(image, "utf8"));
  return JSON.parse(image);
}

/**
 * 🗂️ {@link memoizedRecord} for an analysis of one file's bytes, keyed by their digest, that reads the file only when it must: a file whose
 * stamp equals the one the previous inference recorded (results are persisted only when no input was racy) is known by that digest, its
 * fact is recorded without a read, and a stored analysis is returned. Otherwise the bytes are read (and observed) and analysed.
 * @template T
 * @param {string} namespace
 * @param {string} path
 * @param {(bytes: Buffer) => T} compute
 * @returns {T}
 */
export function memoizedFileRecord(namespace, path, compute) {
  const scope = recordScopes.getStore();
  if (!scope) return compute(observedFs.readFileSync(path));
  const key = absolute(path), memoKey = namespace + "\0" + key, remembered = rememberedFileRecordImageV1(memoKey, path);
  if (remembered !== undefined) { recordRowV1(remembered.row); return JSON.parse(remembered.image); }
  const hint = scope.hints.get(key);
  if (hint) {
    let stamp;
    try { stamp = fs.statSync(path); } catch {}
    if (stamp && stamp.size === hint.size && stamp.mtimeMs === hint.mtimeMs && stamp.ctimeMs === hint.ctimeMs && stamp.ino === hint.ino) {
      const hit = scope.records.get(recordKey(namespace, hint.sha256));
      if (hit !== undefined) {
        try { const value = JSON.parse(utf8TextV1(hit)); record("file", key, hint); return value; } catch {}
      }
    }
  }
  const bytes = observedFs.readFileSync(path), value = compute(bytes), image = jsonImage(value);
  if (image === undefined) return value;
  rememberFileRecordImageV1(memoKey, key, image);
  scope.records.put(recordKey(namespace, sha256HexV1(bytes)), Buffer.from(image, "utf8"));
  return JSON.parse(image);
}

/** 🌊️ Reads a module asynchronously only when its original settled byte fingerprint has no verified semantic record. */
export async function memoizedFileRecordAsyncV1(namespace,path,compute){
 const scope=recordScopes.getStore(),key=absolute(path),hint=scope?.hints.get(key);
 if(hint){
  let stamp;try{stamp=await statAsync(path);}catch{}
  if(stamp&&stamp.size===hint.size&&stamp.mtimeMs===hint.mtimeMs&&stamp.ctimeMs===hint.ctimeMs&&stamp.ino===hint.ino){
   const hit=scope.records.get(recordKey(namespace,hint.sha256));if(hit!==undefined)try{const value=JSON.parse(utf8TextV1(hit));record("file",key,hint);return value;}catch{}
  }
 }
 const bytes=await observedFs.readFile(path),value=compute(bytes),image=jsonImage(value);
 if(scope&&image!==undefined&&Buffer.byteLength(image)<=1024*1024){scope.records.put(recordKey(namespace,sha256HexV1(bytes)),Buffer.from(image,"utf8"));return JSON.parse(image);}
 return value;
}

/** 🪪️ True when a recorded file fact still names the same bytes: an unchanged stamp proves it cheaply, and a restamped file of the same size is proven by its digest, so rewrites of identical content stay hits. */
function fileFactHolds(path, fact) {
  if (fact.error !== undefined) { try { fs.readFileSync(path); return false; } catch (error) { return fact.error === (error.code ?? "EUNKNOWN"); } }
  let current;
  try { current = fs.statSync(path); } catch { return false; }
  if (current.size !== fact.size) return false;
  if (current.mtimeMs === fact.mtimeMs && current.ctimeMs === fact.ctimeMs && current.ino === fact.ino) return true;
  try { return sha256HexV1(fs.readFileSync(path)) === fact.sha256; } catch { return false; }
}

/** ✅️ True when every recorded fact still holds; stops at the first stale fact. Synchronous probes beat the four-thread pool here: each is one cached-inode syscall. */
function current(observations) {
  return consistentObservationsV1(observations)&&observations.every(({ kind, path, fact }) => kind === "file" ? fileFactHolds(path, fact) : kind==="analysis"?sourceAnalysisFactHoldsV1(path,fact):typeof facts[kind] === "function" && recordedFactHoldsV1(kind, path, fact));
}

/**
 * 📸️ Returns `compute()`'s result, reusing the persisted result for `key` while every filesystem fact the computation observed — reads,
 * existence and type probes, directory listings, real paths and module resolutions, including those made while its modules loaded —
 * still holds. A result is persisted only when it is exactly JSON-representable and no file it read was stamped within {@link RACY_MS}
 * of the start, so a hit is indistinguishable from a recomputation.
 * @template T
 * @param {{ readonly cachePath: string; readonly key: unknown }} options
 * @param {() => Promise<T> | T} compute
 * @returns {Promise<T>}
 */
/** 🧳️ Keeps packed descriptors inside the original graph writer lifetime. */
function observedRecordStoreV1(path){
 let records=recordStores.get(path);if(!records){records=openRecordStore(path,{publication:(plan,write)=>withBuildGraphOptionalPublicationV1(join(path,"0-0-0000000000000000.pack"),plan,write)});recordStores.set(path,records);const original=records;recordStoreHandles.set(path,registerBuildGraphOptionalHandleV1(()=>{try{original.close();}finally{if(recordStores.get(path)===original){recordStores.delete(path);recordStoreHandles.delete(path);}}}));}return records;
}

/** 🔌️ Closes one project's pack store once its inference is persisted: its descriptors and location index are not kept for the remaining projects of the graph. */
function releaseObservedRecordStoreV1(path){
 const records=recordStores.get(path);if(!records)return;
 recordStores.delete(path);const unregister=recordStoreHandles.get(path);recordStoreHandles.delete(path);
 try{records.close();}finally{unregister?.();}
}

/**
 * 🪨️ Runs one inference pass in rounds until a single round observed one mutually consistent source snapshot; only that round's facts reach
 * the caller's scope and capture. Projects whose images still hold are reused, so a round re-infers only the projects a concurrent edit
 * touched. A failing round is repeated while any fact it consumed has changed since, or was stamped within {@link RACY_MS} (a half-written
 * file); otherwise its failure is genuine and rethrown with its facts retained. A failure that demands a restart (`BUILD_AUTHORITY_CHANGED`) or
 * that returns unchanged (same code and message) in the next round is genuine as well: a peer editing the tree must not keep a deterministic
 * failure looking transient for `limit` whole-inference rounds.
 * @template T
 * @param {() => Promise<T> | T} compute
 * @param {{ readonly signal?: AbortSignal; readonly limit?: number; readonly onRound?: (round: { round: number; conflict?: string; failure?: unknown }) => void | Promise<void> }} [options]
 * @returns {Promise<T>}
 */
export async function settleObservedComputationV1(compute,{signal,limit=64,onRound}={}){
 const parent=scopes.getStore(),capture=sourceCaptures.getStore(),accept=(rows,captured)=>{retainObservationsV1(parent,rows);retainObservationsV1(capture,captured);};
 let previousFailure;
 for(let round=0;;round++){
  signal?.throwIfAborted();
  const nested=new Map(),captured=new Map(),started=Date.now();let value,failure,failed=false;
  try{value=await sourceCaptures.run(captured,()=>inferenceValidationScopes.run(nested,()=>scopes.run(nested,compute)));}catch(error){failed=true;failure=error;}
  signal?.throwIfAborted();
  const rows=[...nested.values()],consistent=consistentObservationsV1(rows);
  if(consistent&&!failed){accept(rows,captured.values());return value;}
  const signature=failed?JSON.stringify([failure?.code??null,(()=>{try{return String(failure?.message??failure);}catch{return"";}})()]):undefined;
  if(failed&&(failure?.code==="BUILD_AUTHORITY_CHANGED"||failure?.code==="BUILD_COLLECTED_ERRORS"||signature===previousFailure)){accept(rows,captured.values());throw failure;}
  previousFailure=signature;
  if(consistent&&await observationsHold(rows,64,new Set(),new Set(),signal)){
   signal?.throwIfAborted();
   const youngest=Math.max(0,...rows.map(({kind,fact})=>{const source=kind==="file"?fact:kind==="analysis"?fact.source:undefined;return source?Math.max(source.mtimeMs??0,source.ctimeMs??0):0;}));
   if(youngest<started-RACY_MS){accept(rows,captured.values());throw failure;}
   await new Promise((resolve,reject)=>{const timer=setTimeout(resolve,Math.max(0,youngest+RACY_MS-Date.now()));signal?.addEventListener("abort",()=>{clearTimeout(timer);reject(signal.reason);},{once:true});});
  }
  signal?.throwIfAborted();
  await onRound?.({round,...(consistent?{}:{conflict:conflictingInferenceErrorV1(rows).path}),...(failed?{failure}:{})});
  signal?.throwIfAborted();
  if(round+1>=limit){accept(rows,captured.values());throw failed?failure:conflictingInferenceErrorV1(rows);}
 }
}

function requireInferenceReuseV1(reusable){if(reusable!==undefined&&typeof reusable!=="function")throw Error("Explicit inference reuse predicate required");}

/** 🧷️ Keeps receiving failures outside corrupt-cache miss handling. */
async function inferenceReusableV1(reusable,value){requireCurrentObservationV1();if(reusable===undefined)return true;const holds=await reusable(value);requireCurrentObservationV1();if(typeof holds!=="boolean")throw Error("Boolean inference reuse result required");return holds;}

export async function observeInference({ cachePath, key, reusable }, compute) {
  requireCurrentObservationV1();requireInferenceReuseV1(reusable);
  return observeInferenceComputationV1({cachePath,key,reusable},compute,false);
}

/** 🧷️ Validates nested project records within one original inference; its outer publication still rechecks every source fact freshly. */
async function observeInferenceComputationV1({cachePath,key,reusable,prepare},compute,nestedProject) {
  requireCurrentObservationV1();loadSealed = true;
  originalInferencePublications.delete(cachePath);usedInference.delete(cachePath);
  const parent=scopes.getStore();
  const digest = sha256HexV1(JSON.stringify([nestedProject?recordStampV1():OBSERVATION_REVISION, nestedProject?inferenceAuthorityDigestV1():null, key])), hints = new Map(), authority=nestedProject?[]:[...loadObservations.values()];
  const lookup=async()=>{
  let candidate;
  let record;try{record=readInferencePhysicalRecordV1(cachePath,true);}catch(error){if(!observationBlockDamagedV1(error))throw error;record=null;}if(record){await settleClosureRowsV1(record.header.observations,record.value);const{header,line}=record;if(header.digest===digest&&(nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations)))candidate={value:record.value,header,headerDigest:sha256HexV1(line)};for(const{kind,path,fact}of header.observations)if(kind==="file"&&typeof fact?.sha256==="string")hints.set(path,fact);}
  if(candidate&&await inferenceReusableV1(reusable,candidate.value)&&(nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations))){const{header,value,headerDigest}=candidate;retainSourceFactsV1(header.observations);retainObservationsV1(parent,header.observations);usedInference.set(cachePath,headerDigest);originalInferencePublications.set(cachePath,{path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});return{value};}
  return undefined;
  };
  const reused=await lookup()??(prepare!==undefined&&await prepare()?await lookup():undefined);
  if(reused)return reused.value;
  const observations = new Map(), started = Date.now();
  const records=observedRecordStoreV1(`${cachePath}.records`);
  const result = await inferenceValidationScopes.run(inferenceValidationScopes.getStore()??observations,()=>recordScopes.run({ records, hints }, () => scopes.run(observations, compute)));
  requireCurrentObservationV1();records.flush();
  if(records.stats.records>2*records.stats.accessed)records.retainAccessed();else if(records.stats.packs>RECORD_STORE_PACK_LIMIT)records.compact();
  releaseObservedRecordStoreV1(`${cachePath}.records`);
  const rows = [...authority, ...observations.values()];retainSourceFactsV1(rows);
  retainObservationsV1(parent,rows);
  const consistent=consistentObservationsV1(rows);
  if(!consistent&&!nestedProject)throw conflictingInferenceErrorV1(rows);
  originalInferencePublications.delete(cachePath);
  if (jsonImage(result) !== undefined) originalInferencePublications.set(cachePath, {path:cachePath,digest,authorityObservations:authority,observations:rows});
  const racy = rows.some(({ kind, fact }) => {const source=kind==="file"?fact:kind==="analysis"?fact.source:undefined;return source&&(source.mtimeMs>=started-RACY_MS||source.ctimeMs>=started-RACY_MS);});
  usedInference.set(cachePath, null);
  if (consistent && !racy && jsonExact(result) && (nestedProject?rows.every(row=>projectObservationHoldsV1(row,parent)):current(rows))) {
    fs.mkdirSync(dirname(cachePath), { recursive: true });
    const image=JSON.stringify(result),temporary = `${cachePath}.${process.pid}.${started}.tmp`, header = inferenceHeaderTextV1(digest,sha256HexV1(image),authority,rows,dirname(cachePath));
    const packed=await packInferenceRecordV1(Buffer.from(`${header}\n${image}`)),bytes=packed.byteLength;let previous=0;try{previous=fs.lstatSync(cachePath).size;}catch(error){if(error.code!=="ENOENT")throw error;}
    if(withBuildGraphOptionalPublicationV1(cachePath,{peakBytes:bytes,retainedDeltaBytes:bytes-previous},()=>{fs.writeFileSync(temporary, packed,{flag:"wx"});renameOverBusyFileSyncV1(temporary,cachePath);}))usedInference.set(cachePath,sha256HexV1(header));
  }
  return result;
}

/**
 * 🧾️ Runs a graph dependency computation while recording every filesystem fact it observes, then publishes a receipt for the graph seal:
 * this process, when it completed, the exact inference records (by header digest) this process's node inference used in the receipt's
 * directory, and the dependency
 * facts. A receipt is published only when every input is non-racy and still holds and every inference record used was persisted; otherwise
 * any previous receipt is removed, so a seal can never pair a graph with inputs it was not computed from.
 * @template T
 * @param {{ readonly receiptPath: string; readonly recordDirectory?: string }} options
 * @param {() => Promise<T> | T} compute
 * @returns {Promise<T>}
 */
export async function observeDependencies({ receiptPath, recordDirectory }, compute) {
  requireCurrentObservationV1();
  const observations = new Map(), started = Date.now(),publicationPath=receiptPath.replace(/\.receipt\.json$/u,".publication.json"),hints=new Map();
  if(recordDirectory)try{const previous=JSON.parse(fs.readFileSync(publicationPath,"utf8"));if(previous.version===1&&previous.revision===OBSERVATION_REVISION&&Array.isArray(previous.observations)&&Number.isSafeInteger(previous.startedAt))for(const row of previous.observations)if(row.kind==="file"&&SHA256_HEX_V1.test(row.fact?.sha256??"")&&row.fact.mtimeMs<previous.startedAt-RACY_MS&&row.fact.ctimeMs<previous.startedAt-RACY_MS)hints.set(row.path,row.fact);}catch{}
  const records=recordDirectory?observedRecordStoreV1(recordDirectory):undefined;
  const run=()=>scopes.run(observations,compute),result=await(records?recordScopes.run({records,hints},run):run());
  requireCurrentObservationV1();records?.flush();if(records){if(records.stats.records>2*records.stats.accessed)records.retainAccessed();else if(records.stats.packs>RECORD_STORE_PACK_LIMIT)records.compact();}
  const rows = [...observations.values()], inference = [...usedInference].filter(([path]) => dirname(path) === dirname(receiptPath)).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
  if(!consistentObservationsV1(rows))throw conflictingInferenceErrorV1(rows);
  const racy = rows.some(({ kind, fact }) => {const source=kind==="file"?fact:kind==="analysis"?fact.source:undefined;return source&&(source.mtimeMs>=started-RACY_MS||source.ctimeMs>=started-RACY_MS);});
  fs.mkdirSync(dirname(receiptPath), { recursive: true });
  const originalInference=[...originalInferencePublications.values()].filter(value=>dirname(value.path)===dirname(receiptPath)).sort((left,right)=>Buffer.compare(Buffer.from(left.path),Buffer.from(right.path)));
  if(publicationPath===receiptPath)throw Error("Original inference receipt must have an owned publication path");
  const producerPath=resolve(dirname(publicationPath),"p-"+process.pid,basename(publicationPath));
  if(originalInference.length&&jsonImage(result)!==undefined){const image=JSON.stringify({version:1,revision:OBSERVATION_REVISION,pid:process.pid,startedAt:started,completedAt:Date.now(),inference:originalInference,observations:rows});fs.mkdirSync(dirname(producerPath),{recursive:true});for(const path of [producerPath,publicationPath]){const temporary=path+"."+process.pid+"."+started+".tmp",parent=resolve(dirname(path)),status=fs.lstatSync(parent);if(temporary.length>256||[...temporary].length>256||!status.isDirectory()||status.isSymbolicLink()||fs.realpathSync(parent)!==parent)throw Error("Original inference producer namespace refused");fs.writeFileSync(temporary,image,{flag:"wx"});renameOverBusyFileSyncV1(temporary,path);}}else for(const path of [producerPath,publicationPath])fs.rmSync(path,{force:true});
  if (racy || inference.length === 0 || inference.some(([, digest]) => digest === null) || !current(rows)) { fs.rmSync(receiptPath, { force: true }); return result; }
  const temporary = `${receiptPath}.${process.pid}.${started}.tmp`;
  fs.writeFileSync(temporary, JSON.stringify({ format: 1, revision: OBSERVATION_REVISION, pid: process.pid, completedAt: Date.now(), inference, observations: rows }));
  renameOverBusyFileSyncV1(temporary, receiptPath);
  return result;
}

/** 📥️ Captures freshly computed source facts and refuses a preparation whose inputs changed before receipt. */
export async function captureSourcePreparationV1(compute, control) {
  const observations = new Map(), value = await scopes.run(observations, compute), original = [...observations.values()];
  return {value,original,received:await receiveSourcePreparationV1(original,control)};
}

/** 🔐️ Rechecks and records every original source fingerprint under the same preparation control. */
export async function receiveSourcePreparationV1(original,control){
  const received=[];
  for(const row of original){
    control.check();if(received.length%128===0)await control.advance("source-fingerprints");
    const next=row.kind==="analysis"&&sourceAnalysisFactHoldsV1(row.path,row.fact)?row:{kind:row.kind,path:row.path,fact:facts[row.kind](row.path,row.fact)};
    if(JSON.stringify(row.fact)!==JSON.stringify(next.fact))throw Error(`Original source preparation inputs changed: ${row.kind}:${row.path}`);
    received.push(next);
  }
  return received;
}

/** 📜️ The header line of a persisted inference record (its digest and observations), read without the result that follows it. */
export function inferenceHeaderLine(cachePath){return readInferencePhysicalRecordV1(cachePath).line;}


const observationUTF8V1=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});
const observationBlockPolicyV1=sourceAnalysisSchemaV1.$defs.observationBlock.const,blockRowsValidatorV1=compileJsonSchemaSubset({$defs:sourceAnalysisSchemaV1.$defs,$ref:"#/$defs/observationRows"}),headerValidatorV1=compileJsonSchemaSubset({$defs:sourceAnalysisSchemaV1.$defs,$ref:"#/$defs/inferenceHeader"});
/** 📏️ Projects finite aggregate admission DATA without granting physical read custody. */
export function observationBlockBudgetAdmitsV1(used,next,maximum){return Number.isSafeInteger(used)&&Number.isSafeInteger(next)&&Number.isSafeInteger(maximum)&&used>=0&&next>=0&&maximum>=1&&used<=maximum-next;}
function observationReadBudgetV1(){return{packed:0,output:0,rows:0,nodes:0,parentSets:new Map()};}
function observationChargeV1(budget,kind,next){const maximum=observationBlockPolicyV1[{packed:"maximumAggregatePackedBytes",output:"maximumAggregateOutputBytes",rows:"maximumAggregateRows",nodes:"maximumAggregateNodes"}[kind]];if(!observationBlockBudgetAdmitsV1(budget[kind],next,maximum))throw Error("Finite aggregate observation "+kind+" budget exceeded");budget[kind]+=next;}
function observationParentsFenceV1(path,parents){return()=>{requireRepositoryBuildGraphObservationPathV1(path,"read");for(const original of parents){const row=fs.lstatSync(original.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||[row.dev,row.ino,row.mode].join(":")!==original.identity)throw Error("Observation ancestry changed");}};}
function observationAbsenceFenceV1(path,parents,missing){const unchanged=observationParentsFenceV1(path,Object.freeze(parents.map(row=>Object.freeze({...row}))));return Object.freeze({absent:true,missing,parentFence:unchanged,recheck:()=>{unchanged();try{fs.lstatSync(missing);throw Error("Absent observation coordinate reappeared");}catch(error){if(error?.code!=="ENOENT")throw error;}}});}
function observationLeafFenceV1(path,parents,epoch){const unchanged=observationParentsFenceV1(path,parents);return()=>{unchanged();const actual=fs.lstatSync(path,{bigint:true});if(!actual.isFile()||actual.isSymbolicLink()||actual.nlink!==1n||observationEpochV1(actual)!==epoch)throw Error("Observation leaf changed before delivery");};}
function observationEpochV1(row){return[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");}
function observationPhysicalReadV1(path,maximumBytes,optional,namedBlock=false,budget=observationReadBudgetV1(),absenceProof=false){
 path=resolve(path);const custody=requireRepositoryBuildGraphObservationPathV1(path,"read"),check=()=>requireRepositoryBuildGraphObservationPathV1(path,"read"),parents=[],ancestry=[];for(let cursor=dirname(path);;cursor=dirname(cursor)){if(ancestry.length>=observationBlockPolicyV1.maximumAncestors)throw Error("Finite observation ancestry required");ancestry.push(cursor);if(dirname(cursor)===cursor)break;}ancestry.reverse();let device,inside=false;
 const unchanged=observationParentsFenceV1(path,parents);
 for(const directory of ancestry){if(custody.root===directory)inside=true;let row;try{row=fs.lstatSync(directory,{bigint:true});}catch(error){if(error?.code!=="ENOENT"||!optional||custody.kind==="managed"&&(directory===custody.root||!inside||!relative(custody.root,directory).split(sep).some(name=>["np","ip","rr","inference"].includes(name))))throw error;unchanged();try{fs.lstatSync(directory);throw Error("Absent observation namespace reappeared");}catch(probe){if(probe?.code!=="ENOENT")throw probe;}return absenceProof?observationAbsenceFenceV1(path,parents,directory):null;}if(!row.isDirectory()||row.isSymbolicLink()||inside&&device!==undefined&&row.dev!==device)throw Error("Original observation physical ancestry required");if(inside)device=row.dev;parents.push({path:directory,identity:[row.dev,row.ino,row.mode].join(":")});}
 const parentKey=dirname(path),retained=budget.parentSets.get(parentKey);let retainedParents;if(retained){if(retained.length!==parents.length||parents.some((row,index)=>row.path!==retained[index].path||row.identity!==retained[index].identity))throw Error("Observation retained ancestry changed");retainedParents=retained;}else{if(!observationBlockBudgetAdmitsV1(budget.parentSets.size,1,observationBlockPolicyV1.maximumParentSets))throw Error("Finite observation parent sets exceeded");retainedParents=Object.freeze(parents.map(row=>Object.freeze(row)));budget.parentSets.set(parentKey,retainedParents);}
 let initial;try{initial=fs.lstatSync(path,{bigint:true});}catch(error){if(error?.code!=="ENOENT"||!optional){if(error?.code==="ENOENT"&&namedBlock){unchanged();try{fs.lstatSync(path);throw Error("Absent named inference block reappeared");}catch(probe){if(probe?.code!=="ENOENT")throw probe;}throw Object.assign(Error("Named inference observation block is absent"),{code:"BUILD_INFERENCE_BLOCK_ABSENT",cause:error});}throw error;}unchanged();try{fs.lstatSync(path);throw Error("Absent observation leaf reappeared");}catch(probe){if(probe?.code!=="ENOENT")throw probe;}return absenceProof?observationAbsenceFenceV1(path,parents,path):null;}if(!initial.isFile()||initial.isSymbolicLink()||initial.nlink!==1n||inside&&initial.dev!==device||initial.size<1n||initial.size>BigInt(maximumBytes))throw Error("Bounded original observation leaf required");const epoch=observationEpochV1(initial),descriptor=fs.openSync(path,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW??0)|(fs.constants.O_NONBLOCK??0));let bytes,readFailed=false;try{check();const opened=fs.fstatSync(descriptor,{bigint:true});if(!opened.isFile()||observationEpochV1(opened)!==epoch)throw Error("Observation descriptor identity changed");observationChargeV1(budget,"packed",Number(initial.size));bytes=Buffer.alloc(Number(initial.size));let offset=0;while(offset<bytes.length){check();const count=fs.readSync(descriptor,bytes,offset,Math.min(65536,bytes.length-offset),offset);if(count===0)throw Error("Partial observation leaf read");offset+=count;}if(observationEpochV1(fs.fstatSync(descriptor,{bigint:true}))!==epoch)throw Error("Observation descriptor changed during read");}catch(error){readFailed=true;throw error;}finally{try{fs.closeSync(descriptor);}catch(error){if(!readFailed)throw error;}}const recheck=observationLeafFenceV1(path,retainedParents,epoch);recheck();return{bytes,recheck,epoch};
}
function observationJsonV1(text,budget=observationReadBudgetV1()){let depth=0,nodes=0,string=false,escape=false;for(let index=0;index<text.length;index++){const char=text[index];if(string){if(escape)escape=false;else if(char==="\\")escape=true;else if(char==='"')string=false;continue;}if(char==='"'){string=true;if(++nodes>observationBlockPolicyV1.maximumNodes)throw Error("Finite observation JSON nodes required");}else if(char==="["||char==="{"){if(++depth>observationBlockPolicyV1.maximumDepth||++nodes>observationBlockPolicyV1.maximumNodes)throw Error("Finite observation JSON depth required");}else if(char==="]"||char==="}")depth--;else if(char===","&&++nodes>observationBlockPolicyV1.maximumNodes)throw Error("Finite observation JSON nodes required");}observationChargeV1(budget,"nodes",nodes);return JSON.parse(text);}
function observationInflateV1(packed,budget,maximum){const remaining=observationBlockPolicyV1.maximumAggregateOutputBytes-budget.output;if(remaining<1)throw Error("Finite aggregate observation output budget exhausted");const bytes=brotliDecompressSync(packed,{maxOutputLength:Math.min(maximum,remaining)});observationChargeV1(budget,"output",bytes.length);return bytes;}
const verifiedBlocks=new Map(),VERIFIED_BLOCK_ROWS=4194304;let verifiedBlockRows=0;
/** 🧊️ Remembers a block whose bytes, hash and rows this process verified, by the full identity of its leaf. */
function rememberVerifiedBlockV1(path,epoch,rows){if(verifiedBlockRows+rows.length>VERIFIED_BLOCK_ROWS){verifiedBlocks.clear();verifiedBlockRows=0;}const prior=verifiedBlocks.get(path);if(prior)verifiedBlockRows-=prior.rows.length;verifiedBlocks.set(path,{epoch,rows,recheck:undefined});verifiedBlockRows+=rows.length;}
/** 🧊️ A verified block while its leaf still has the identity it was verified with (device, inode, mode, links, size and both nanosecond stamps); any other leaf is read and validated again. */
function verifiedBlockV1(path){
 const entry=verifiedBlocks.get(path);if(entry===undefined)return undefined;
 try{const row=fs.lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&row.nlink===1n&&observationEpochV1(row)===entry.epoch){entry.recheck??=()=>{requireRepositoryBuildGraphObservationPathV1(path,"read");const actual=fs.lstatSync(path,{bigint:true});if(!actual.isFile()||actual.isSymbolicLink()||observationEpochV1(actual)!==entry.epoch)throw Error("Observation leaf changed before delivery");};return entry;}}catch{}
 verifiedBlockRows-=entry.rows.length;verifiedBlocks.delete(path);return undefined;
}
/** 🧊️ Registers a block that a farm worker of this implementation validated and that the caller verified to hold, by its hash, the text it is named by: its first reader takes the rows of the registration instead of validating the leaf again. A leaf that is not the one regular file the adoption wrote is not registered. */
export function adoptVerifiedBlockV1(path,text){
 try{const row=fs.lstatSync(path,{bigint:true});if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n)return;rememberVerifiedBlockV1(path,observationEpochV1(row),internObservationsV1(JSON.parse(text)));}catch{}
}
function readObservationBlockV1(directory,digest,optional=false,budget=observationReadBudgetV1()){const path=inferenceObservationBlockPathV1(directory,digest),verified=verifiedBlockV1(path);if(verified){requireRepositoryBuildGraphObservationPathV1(path,"read");observationChargeV1(budget,"rows",verified.rows.length);return{rows:verified.rows,recheck:verified.recheck,epoch:verified.epoch};}const physical=observationPhysicalReadV1(path,observationBlockPolicyV1.maximumPackedBytes,optional,true,budget,optional);if(!physical||physical.absent)return physical;const bytes=observationInflateV1(physical.bytes,budget,observationBlockPolicyV1.maximumOutputBytes);if(sha256HexV1(bytes)!==digest)throw Error("Inference block content differs from its name");const rows=observationJsonV1(observationUTF8V1.decode(bytes),budget);if(blockRowsValidatorV1.validate(rows).length)throw Error("Closed inference observation block rows required");observationChargeV1(budget,"rows",rows.length);physical.recheck();const shared=internObservationsV1(rows);rememberVerifiedBlockV1(path,physical.epoch,shared);return{rows:shared,recheck:physical.recheck,epoch:physical.epoch};}
function readInferencePhysicalRecordV1(cachePath,optional=false){const budget=observationReadBudgetV1(),physical=observationPhysicalReadV1(cachePath,observationBlockPolicyV1.maximumHeaderPackedBytes,optional,false,budget);if(!physical)return null;const text=observationUTF8V1.decode(observationInflateV1(physical.bytes,budget,observationBlockPolicyV1.maximumHeaderOutputBytes)),newline=text.indexOf("\n");if(newline<0)throw Error("Complete inference header and result required");const line=text.slice(0,newline),header=observationJsonV1(line,budget);if(headerValidatorV1.validate(header).length)throw Error("Closed inference record header required");const result=text.slice(newline+1);if(header.resultSha256!==sha256HexV1(result))throw Error("Inference result content differs from its header");const value=observationJsonV1(result,budget);observationChargeV1(budget,"rows",header.authorityObservations.length+header.observations.length);const expanded=expandInferenceBlocksV1(dirname(cachePath),header.blocks,budget);header.observations=[...expanded,...internObservationsV1(header.observations)];if(header.observations.length>observationBlockPolicyV1.maximumHeaderRows)throw Error("Finite expanded inference rows required");physical.recheck();return{header,line,value};}

/** 🧱️ The shared fact table of persisted records: the rows of one record are cut into content-defined blocks (a cut follows every row whose key hash hits the mask once a block has its minimum rows), so the long runs of rows that thousands of projects share, such as one command closure, become the same block files and a record header lists block digests instead of repeating the rows. */
const BLOCK_MINIMUM_ROWS=32,BLOCK_MAXIMUM_ROWS=4096,BLOCK_CUT_MASK=0x7f;
const blockRowIdentities=new WeakMap(),blockRowCuts=new WeakMap(),blocksByIdentity=new Map();
let blockRowCounter=0;
const blocksRefused=new Map(),BLOCK_REFUSAL_MILLISECONDS=30000;
const blockRowIdV1=row=>{let id=blockRowIdentities.get(row);if(id===undefined){id=++blockRowCounter;blockRowIdentities.set(row,id);}return id;};
const blockRowCutV1=row=>{let cut=blockRowCuts.get(row);if(cut===undefined){const key=observationKeyV1(row);let hash=0x811c9dc5;for(let at=0;at<key.length;at++){hash^=key.charCodeAt(at);hash=Math.imul(hash,16777619);}cut=(hash>>>0)&BLOCK_CUT_MASK;blockRowCuts.set(row,cut);}return cut;};
function partitionObservationsV1(rows){
 const blocks=[];let current=[],ids=[];
 for(const row of rows){
  current.push(row);ids.push(blockRowIdV1(row));
  if(current.length>=BLOCK_MAXIMUM_ROWS||current.length>=BLOCK_MINIMUM_ROWS&&blockRowCutV1(row)===0){blocks.push({rows:current,key:ids.join(",")});current=[];ids=[];}
 }
 if(current.length)blocks.push({rows:current,key:ids.join(",")});
 return blocks;
}

/** 🧊️ One full-digest observation block path is shared by readers and publishers. */
export function inferenceObservationBlockPathV1(directory,digest){const policy=sourceAnalysisSchemaV1.$defs.observationBlock.const;if(typeof directory!=="string"||resolve(directory)!==directory||typeof digest!=="string"||!new RegExp(policy.digestPattern,"u").test(digest))throw Error("Canonical observation block path required");const path=join(directory,policy.directory,digest+policy.extension);if(path.length>256)throw Error("Observation block path exceeds256 characters");return path;}

/** 🩹️ Whether a failed read says that a block leaf is absent or damaged (not a block, not its name, not closed rows): the record that names it is a miss and the next publication replaces the leaf; anything else is a fault of the store and propagates. */
function observationBlockDamagedV1(error){return error?.code==="BUILD_INFERENCE_BLOCK_ABSENT"||/^(?:Z_|ERR_BROTLI|ERR_ZLIB)/u.test(String(error?.code))||error?.name==="ZlibError"||/Decompression failed|Inference block content differs from its name|Inference result content differs from its header|Closed inference observation block rows required/u.test(String(error?.message));}

/** 💾️ Publishes shared block bytes only after current physical presence, hash and closed rows are validated. */
function persistInferenceBlockV1(directory,block){
 requireRepositoryBuildGraphObservationPathV1(directory,"write");let known=blocksByIdentity.get(block.key);if(known===undefined){known={digest:sha256HexV1(observationRowsJsonV1(block.rows))};blocksByIdentity.set(block.key,known);}block.digest=known.digest;const path=inferenceObservationBlockPathV1(directory,known.digest);let present;try{present=readObservationBlockV1(directory,known.digest,true);}catch(error){if(!observationBlockDamagedV1(error))throw error;fs.unlinkSync(path);present=readObservationBlockV1(directory,known.digest,true);}if(present&&!present.absent){present.recheck();return true;}if((blocksRefused.get(directory)??0)>Date.now())return false;const packed=brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows)),{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22}}),temporary=path+"."+process.pid+"."+Date.now()+".tmp";if(temporary.length>observationBlockPolicyV1.maximumPathUnits)throw Error("Observation block staging path exceeds256 characters");if(!withBuildGraphOptionalPublicationV1(path,{peakBytes:packed.byteLength,retainedDeltaBytes:packed.byteLength},()=>{present.recheck();let created;if(present.missing===dirname(path)){fs.mkdirSync(dirname(path));const row=fs.lstatSync(dirname(path),{bigint:true}),parent=fs.lstatSync(dirname(dirname(path)),{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==parent.dev)throw Error("Original created block directory required");created=[row.dev,row.ino,row.mode].join(":");}else if(present.missing!==path)throw Error("Only exact block directory creation admitted");const fence=()=>{present.parentFence();if(created!==undefined){const row=fs.lstatSync(dirname(path),{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||[row.dev,row.ino,row.mode].join(":")!==created)throw Error("Created block directory identity changed");}};fence();fs.writeFileSync(temporary,packed,{flag:"wx"});fence();renameOverBusyFileSyncV1(temporary,path);fence();})){blocksRefused.set(directory,Date.now()+BLOCK_REFUSAL_MILLISECONDS);return false;}const published=readObservationBlockV1(directory,known.digest);published.recheck();return true;
}

/** 🧾️ The header text of a record: blocks first (written before the record that names them), rows no block could hold inline. */
function inferenceHeaderTextV1(digest,resultSha256,authority,rows,directory){
 const blocks=[],inline=[];
 for(const block of partitionObservationsV1(rows)){if(persistInferenceBlockV1(directory,block))blocks.push(block.digest);else for(const row of block.rows)inline.push(row);}
 return `{"digest":${JSON.stringify(digest)},"resultSha256":"${resultSha256}","authorityObservations":${observationRowsJsonV1(authority)},"blocks":${JSON.stringify(blocks)},"observations":${observationRowsJsonV1(inline)}}`;
}

/** 📖️ Revalidates every named block physically and retains all identities through the final delivery fence. */
function expandInferenceBlocksV1(directory,digests,budget=observationReadBudgetV1()){const rows=[],proofs=[];if(!Array.isArray(digests)||digests.length>observationBlockPolicyV1.maximumBlocks)throw Error("Finite inference block roster required");for(const digest of digests){const block=readObservationBlockV1(directory,digest,false,budget);proofs.push(block.recheck);for(const row of block.rows)rows.push(row);if(rows.length>observationBlockPolicyV1.maximumHeaderRows)throw Error("Finite expanded inference rows required");}for(const recheck of proofs)recheck();return rows;}

/** 🧾️ Reads a physically authenticated closed inference header and all named blocks under actual reader custody. */
export function readInferenceHeaderV1(cachePath){return readInferencePhysicalRecordV1(cachePath).header;}

/** 🧹️ The block digests one record header names, read without expanding any block; a header that is not a closed inference header refuses, because liveness cannot then be proven. */
function inferenceHeaderBlocksV1(file){
 const stat=fs.lstatSync(file,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1n||stat.size<1n||stat.size>BigInt(observationBlockPolicyV1.maximumHeaderPackedBytes))throw refuseBuildV1("invalid","observation.inference-header-unbounded",{file});
 const text=brotliDecompressSync(fs.readFileSync(file),{maxOutputLength:observationBlockPolicyV1.maximumHeaderOutputBytes}).toString("utf8"),newline=text.indexOf("\n");if(newline<0)throw refuseBuildV1("invalid","observation.inference-header-incomplete",{file});
 const header=JSON.parse(text.slice(0,newline));if(headerValidatorV1.validate(header).length)throw refuseBuildV1("invalid","observation.inference-record-header-open",{file});
 return header.blocks;
}

/** 🧹️ Removes the observation blocks of one record directory that no record header names any more: every re-inference writes a new header and new blocks, the superseded blocks are never read again, and without this they accumulate until the finite inventory of the graph store is exceeded and no graph command can run. A block is live when a header of `directory` names it; a block younger than `graceMilliseconds` is kept (its header may still be written); a header that cannot be read as a closed header aborts before anything is removed. The caller holds the writer custody of the record namespace, so no header appears meanwhile. `dryRun` only counts; below `max(minimumBlocks, blocksPerHeader × headers)` blocks nothing is read and the sweep reports `skipped`. Reports progress and stops at the next checkpoint when `signal` aborts. */
export async function sweepSupersededObservationBlocksV1(directory,{headerPattern=/^[a-f0-9]{24}\.json$/u,graceMilliseconds=600000,now=Date.now(),dryRun=false,minimumBlocks=0,blocksPerHeader=0,signal,progress}={}){
 const blocks=join(directory,observationBlockPolicyV1.directory),digest=new RegExp(observationBlockPolicyV1.digestPattern,"u"),leaf=name=>name.endsWith(observationBlockPolicyV1.extension)&&digest.test(name.slice(0,-observationBlockPolicyV1.extension.length));
 let names;try{names=fs.readdirSync(blocks);}catch(error){if(error?.code==="ENOENT")return{headers:0,live:0,blocks:0,removed:0,bytes:0,kept:0};throw error;}
 const headers=fs.readdirSync(directory).filter(name=>headerPattern.test(name)).sort(),live=new Set();
 if(names.length<=Math.max(minimumBlocks,blocksPerHeader*headers.length))return{headers:headers.length,live:0,blocks:names.length,removed:0,bytes:0,kept:names.length,skipped:true};
 for(let at=0;at<headers.length;at++){signal?.throwIfAborted();for(const block of inferenceHeaderBlocksV1(join(directory,headers[at])))live.add(block);if(at%256===255){await progress?.({phase:"sweep-headers",done:at+1,total:headers.length});await new Promise(resolve=>setImmediate(resolve));}}
 let removed=0,bytes=0,kept=0;const total=names.length;
 for(let at=0;at<names.length;at++){
  signal?.throwIfAborted();const name=names[at];
  if(leaf(name)){
   if(live.has(name.slice(0,-observationBlockPolicyV1.extension.length)))kept++;
   else{
    const path=join(blocks,name);let stat;try{stat=fs.lstatSync(path,{bigint:true});}catch(error){if(error?.code==="ENOENT")continue;throw error;}
    if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1n||now-Number(stat.mtimeNs/1000000n)<graceMilliseconds)kept++;
    else{if(!dryRun)try{fs.unlinkSync(path);}catch(error){if(error?.code!=="ENOENT")throw error;continue;}removed++;bytes+=Number(stat.size);}
   }
  }
  if(at%1024===1023){await progress?.({phase:"sweep-blocks",done:at+1,total});await new Promise(resolve=>setImmediate(resolve));}
 }
 await progress?.({phase:"sweep-blocks",done:total,total});
 return{headers:headers.length,live:live.size,blocks:total,removed,bytes,kept};
}

/**
 * ✅️ {@link current} without blocking: every recorded fact re-checked through the libuv pool, `limit` at a time; false at the first stale
 * fact. Existence, type and listing facts about entries of a directory in `stableDirectories` (absolute paths whose entry sets are proven
 * unchanged, e.g. by unchanged directory stamps) hold without a probe: an entry can only appear, vanish or change type by changing its parent.
 * Existence follows symbolic links, so a present entry is probed for every entry in `symlinks`, and for every entry when `symlinks` is `true`; an absent entry of a stable directory cannot appear without changing that directory.
 */
export function observationsHold(observations, limit = 64, stableDirectories = new Set(), symlinks = new Set(),signal,onStale) {
  return inferenceValidationScopes.run(inferenceValidationScopes.getStore() ?? new Map(), () => observationsHoldInRoundV1(observations, limit, stableDirectories, symlinks, signal, onStale));
}

async function observationsHoldInRoundV1(observations, limit, stableDirectories, symlinks, signal, onStale) {
  signal?.throwIfAborted();await settleClosureRowsV1(observations);if(!consistentObservationsV1(observations)){const seen=new Map();for(const row of observations){const key=observationIdentityV1(row),fact=observationMeaningV1(row);if(seen.has(key)&&seen.get(key)!==fact){onStale?.(row);break;}seen.set(key,fact);}return false;}
  let next = 0, stale = false;
  const check = async ({ kind, path, fact }) => {
    const parent = path.slice(0, Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")));
    const absent = fact === false;
    if (kind === "lstat" && stableDirectories.has(parent) || kind === "exists" && stableDirectories.has(parent) && (absent || symlinks !== true && !symlinks.has(path)) || (kind === "fileExists" || kind === "directoryExists") && absent && stableDirectories.has(parent) || (kind === "readdir" || kind === "readdirNames") && stableDirectories.has(path)) return true;
    if (kind === "file") {
      if (fact.error !== undefined) return fileFactHolds(path, fact);
      let stamp;
      try { stamp = await statAsync(path); } catch { return false; }
      if (stamp.size !== fact.size) return false;
      if (stamp.mtimeMs === fact.mtimeMs && stamp.ctimeMs === fact.ctimeMs && stamp.ino === fact.ino) return true;
      try { return sha256HexV1(await readFileAsync(path)) === fact.sha256; } catch { return false; }
    }
    if (kind === "exists") { try { await statAsync(path); return fact === true; } catch { return fact === false; } }
    if(kind==="analysis")return sourceAnalysisFactHoldsV1(path,fact);
    return typeof facts[kind] === "function" && recordedFactHoldsV1(kind, path, fact);
  };
  const worker = async () => { while (!stale && next < observations.length){signal?.throwIfAborted();const row=observations[next++];if (!await check(row)){stale = true;onStale?.(row);}} };
  await Promise.all(Array.from({ length: Math.min(limit, observations.length) }, worker));
  return !stale;
}
