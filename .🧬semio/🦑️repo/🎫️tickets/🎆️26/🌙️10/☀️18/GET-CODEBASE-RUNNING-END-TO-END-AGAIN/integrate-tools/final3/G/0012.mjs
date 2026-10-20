import{captureBoundedSourceRevisionV1,physicalSourceOriginV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import {withBuildGraphOptionalPublicationV1,registerBuildGraphOptionalHandleV1} from "../📥️inputs/💾️storage/🟨️.mjs";
import * as fs from "node:fs";
import { readFile as readFileAsync, stat as statAsync } from "node:fs/promises";
import { AsyncLocalStorage } from "node:async_hooks";
import { createHash } from "node:crypto";
import { brotliCompress, brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";

/** 🗜️ Persisted inference records are brotli streams: their observation rows repeat across projects and shrink about 15 times. */
const packInferenceRecordV1=bytes=>new Promise((accept,reject)=>brotliCompress(bytes,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:bytes.length}},(error,packed)=>error?reject(error):accept(packed)));
import { createRequire } from "node:module";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { openRecordStore, recordKey } from "../../⚡️caching/🗃️records/🟦️.ts";
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const observationSchemaURLV1=new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),observationSchemaCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(observationSchemaURLV1),65536,true),sourceAnalysisSchemaV1=JSON.parse(observationSchemaCaptureV1.text??""),observationSchemaRevisionV1=observationSchemaCaptureV1.sha256;
import{requireExactSourceReadBatchV1,readExactSourceFileV1}from"../📥️inputs/🧮️digest/📥️read/🟨️.mjs";

/** 🗃️ This module's own bytes: persisted observations validated by another revision of these rules are a miss. */
const OBSERVATION_REVISION = captureBoundedSourceRevisionV1(fileURLToPath(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),8388608).sha256;

/** 🧷️ Refuses a statically cached observation instance whose executing Source or privately captured schema changed. */
export function requireCurrentObservationV1(){try{if(captureBoundedSourceRevisionV1(fileURLToPath(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),8388608).sha256===OBSERVATION_REVISION&&captureBoundedSourceRevisionV1(fileURLToPath(observationSchemaURLV1),65536).sha256===observationSchemaRevisionV1)return;}catch(cause){throw Object.assign(Error("Stale loaded observation implementation; restart required"),{code:"BUILD_AUTHORITY_CHANGED",cause});}throw Object.assign(Error("Stale loaded observation implementation; restart required"),{code:"BUILD_AUTHORITY_CHANGED"});}

/** ⏱️ Files stamped this close to the start of a computation may still change within one timestamp tick, so their result is never persisted (racy-git rule). */
const RACY_MS = 2000;

const scopes = new AsyncLocalStorage();
const inferenceValidationScopes = new AsyncLocalStorage();
const sourceCaptures = new AsyncLocalStorage();
const recordScopes = new AsyncLocalStorage();
const recordStores = new Map();
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

/** 📊️ Exposes owned project computation counts without influencing source admission. */
export function projectInferenceCountsV1(){return Object.fromEntries(projectComputations);}

/** 📊️ Reports owned observation counts without exposing or discarding original source facts. */
export function sourceObservationStatsV1(){let projectObservationRows=0;for(const project of projectImages.values())projectObservationRows+=project.observations.length;return {loadObservations:loadObservations.size,projectImages:projectImages.size,originalInferencePublications:originalInferencePublications.size,projectObservationRows,captureObservations:sourceCaptures.getStore()?.size??0,recordStores:recordStores.size,analysisEntries:sourceAnalysisImages.size,analysisBytes:sourceAnalysisBytes};}

/** 🔏️ Registers one explicit parser revision; receipts can never provide executable analysis code. */
export function registerSourceAnalysisV1({id,revision,analyze}){
 if(typeof id!=="string"||!id.length||id.length>256||id.trim()!==id||id.includes("\0")||typeof revision!=="string"||!/^[a-f0-9]{64}$/u.test(revision)||typeof analyze!=="function")throw Error("Explicit source analysis identity, revision and parser required");
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

/** 🪪️ Seals module-load authority and names it by the meaning of its facts, so project records inherit implementation identity by digest instead of by rows. */
export function inferenceAuthorityDigestV1(){
 loadSealed=true;
 return authorityDigest??=createHash("sha256").update(JSON.stringify([...loadObservations.values()].map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])).sort())).digest("hex");
}

/** 📦️ Admits one physical project's own input observations through its own computation, bound to the sealed authority digest. */
export async function observeProjectInferenceV1({cachePath,key,owner,reusable},compute){
 requireCurrentObservationV1();requireInferenceReuseV1(reusable);originalInferencePublications.delete(cachePath);usedInference.delete(cachePath);
 const parent=scopes.getStore(),identity=createHash("sha256").update(JSON.stringify([OBSERVATION_REVISION,inferenceAuthorityDigestV1(),key])).digest("hex"),previous=projectImages.get(cachePath);
 const inherit=rows=>{retainSourceFactsV1(rows);retainObservationsV1(parent,rows);};
 if(previous?.identity===identity&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){
  const value=JSON.parse(previous.image.toString("utf8"));
  if(await inferenceReusableV1(reusable,value)&&(parent?consistentObservationsV1(previous.observations):current(previous.observations))){inherit(previous.observations);originalInferencePublications.set(cachePath,previous.publication);usedInference.set(cachePath,previous.headerDigest);return value;}
 }
 const value=await observeInferenceComputationV1({cachePath,key,reusable},async()=>{projectComputations.set(owner,(projectComputations.get(owner)??0)+1);const image=jsonImage(await compute());if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");return JSON.parse(image);},Boolean(parent&&inferenceValidationScopes.getStore()));
 const publication=originalInferencePublications.get(cachePath);
 if(!publication)throw Error("Original project inference observations required");
 inherit(publication.observations);
 projectImages.set(cachePath,{identity,image:Buffer.from(JSON.stringify(value)),observations:publication.observations,publication,headerDigest:usedInference.get(cachePath)??null});
 return value;
}

/** 📥️ Retains every actual source fact, including facts inherited from valid inference records. */
function retainSourceFactsV1(rows){retainObservationsV1(sourceCaptures.getStore(),rows);}

/** 🧾️ Preserves distinct consumed results for a path, including contradictory reads. */
function retainObservationsV1(target,rows){if(target)for(const row of rows)target.set(observationKeyV1(row),row);}

/** 🔑️ The exact content key of one observation row, computed once per row object; flat facts avoid JSON serialisation. */
const observationKeys=new WeakMap();
function factKeyV1(fact){
 if(fact===null||typeof fact!=="object")return typeof fact+String(fact);
 if(typeof fact.sha256==="string"&&fact.provider===undefined&&typeof fact.size==="number")return "f"+fact.size+","+fact.mtimeMs+","+fact.ctimeMs+","+fact.ino+","+fact.sha256;
 return JSON.stringify(fact);
}
function observationKeyV1(row){let key=observationKeys.get(row);if(key===undefined){key=row.kind+"\0"+row.path+"\0"+factKeyV1(row.fact);observationKeys.set(row,key);}return key;}

/** 🪢️ Shares one row object between every project that observed identical bytes, so persisted headers cost memory by distinct fact, not by project. */
const internedObservations=new Map(),INTERNED_OBSERVATIONS=4194304;

/** 🧾️ The JSON text of one observation row, serialised once per row object: a header costs concatenation, not the repeated serialisation of rows that thousands of projects share. */
const observationJsons=new WeakMap();
function observationRowsJsonV1(rows){return "["+rows.map(row=>{let text=observationJsons.get(row);if(text===undefined){text=JSON.stringify(row);observationJsons.set(row,text);}return text;}).join(",")+"]";}
function internObservationsV1(rows){return rows.map(row=>{if(internedObservations.size>INTERNED_OBSERVATIONS)internedObservations.clear();const key=observationKeyV1(row),known=internedObservations.get(key);if(known)return known;internedObservations.set(key,row);return row;});}
function canonicalObservationV1(kind,path,fact){
 if(internedObservations.size>INTERNED_OBSERVATIONS)internedObservations.clear();
 const key=kind+"\0"+path+"\0"+factKeyV1(fact),known=internedObservations.get(key);if(known)return known;
 const row={kind,path,fact};observationKeys.set(row,key);internedObservations.set(key,row);return row;
}
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
 return consistentObservationsV1(rows)&&rows.every(row=>{const key=observationKeyV1(row),previous=checks.get(key);if(previous!==undefined)return previous;const holds=current([row]);checks.set(key,holds);return holds;});
}

/** 🧵️ Shares validated invocation work while replaying every consumed fact into each receiving project. */
export function memoizedObservedComputationV1(cache,key,compute) {
 const parent=scopes.getStore(),previous=cache.get(key);
 if(previous&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){retainObservationsV1(parent,previous.observations);retainSourceFactsV1(previous.observations);return previous.value;}
 const nested=new Map();
 try{const value=scopes.run(nested,compute);if(value&&typeof value.then==="function")throw Error("Synchronous observed computation required");if(value!==null)cache.set(key,{value,observations:[...nested.values()]});return value;}
 finally{retainObservationsV1(parent,nested.values());retainSourceFactsV1(nested.values());}
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
 if(previous){if(validatedAsyncDeliveryV1(previous,parent,capture,validation)){await checkpoint(true);return previous.value;}let holds=true;for(const row of previous.observations){await checkpoint();if(!projectObservationHoldsV1(row,parent)){holds=false;break;}}if(holds){retainObservationsV1(parent,previous.observations);retainSourceFactsV1(previous.observations);validatedAsyncDeliveryV1(previous,parent,capture,validation,true);return previous.value;}}
 const nested=new Map();
 try{const value=await scopes.run(nested,compute);if(value!==null)cache.set(key,{value,observations:[...nested.values()]});return value;}
 finally{retainObservationsV1(parent,nested.values());retainSourceFactsV1(nested.values());}
}

/** ⚖️ Refuses mixed source images while allowing identical bytes with different file stamps. */
const observationIdentities=new WeakMap(),observationMeanings=new WeakMap();
function observationIdentityV1(row){let value=observationIdentities.get(row);if(value===undefined){value=row.kind+"\0"+row.path+(row.kind==="analysis"?"\0"+row.fact?.provider:"");observationIdentities.set(row,value);}return value;}

/** 🧠️ Distinguishes semantic results independently of their original source byte hints. */
function observationMeaningV1(row){let value=observationMeanings.get(row);if(value===undefined){value=JSON.stringify(row.kind==="file"&&row.fact?.sha256?{sha256:row.fact.sha256}:row.kind==="analysis"?{provider:row.fact?.provider,revision:row.fact?.revision,resultSha256:row.fact?.resultSha256}:row.fact);observationMeanings.set(row,value);}return value;}

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
    return { size: stat.size, mtimeMs: stat.mtimeMs, ctimeMs: stat.ctimeMs, ino: stat.ino, sha256: createHash("sha256").update(fs.readFileSync(path)).digest("hex") };
  } catch (error) { return { error: error.code ?? "EUNKNOWN" }; }
}

/** 🧬️ Captures the bytes actually passed to a parser, with stamps retained solely as validation hints. */
function sourceFactV1(stamp,bytes){return {size:bytes.length,mtimeMs:stamp instanceof Error?0:stamp.mtimeMs,ctimeMs:stamp instanceof Error?0:stamp.ctimeMs,ino:stamp instanceof Error?0:stamp.ino,sha256:createHash("sha256").update(bytes).digest("hex")};}

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
 if(hit!==undefined)try{const candidate=JSON.parse(Buffer.from(hit).toString("utf8"));if(typeof candidate.image==="string"&&jsonImage(JSON.parse(candidate.image))===candidate.image&&Array.isArray(candidate.nestedObservations)&&current(candidate.nestedObservations))result=candidate;}catch{}
 if(!result)result=analyzeSourceV1(registered,path,bytes);
 const size=Buffer.byteLength(result.image)+Buffer.byteLength(JSON.stringify(result.nestedObservations))+192;
 if(size<=SOURCE_ANALYSIS_BYTES){while(sourceAnalysisBytes+size>SOURCE_ANALYSIS_BYTES){const oldest=sourceAnalysisImages.keys().next().value;sourceAnalysisBytes-=sourceAnalysisImages.get(oldest).bytes;sourceAnalysisImages.delete(oldest);}sourceAnalysisImages.set(key,{result,bytes:size});sourceAnalysisBytes+=size;records?.put(key,Buffer.from(JSON.stringify(result)));}
 return result;
}

/** 📦️ Reuses semantic images only while every nested dependency remains current. */
function observeSourceBytesV1(registered,path,bytes,source){
 const result=sourceAnalysisImageV1(registered,path,bytes,source.sha256);
 retainObservationsV1(scopes.getStore(),result.nestedObservations);retainSourceFactsV1(result.nestedObservations);
 const fact={provider:registered.provider.id,revision:registered.provider.revision,source,resultSha256:createHash("sha256").update(result.image).digest("hex"),nestedObservations:result.nestedObservations};record("analysis",path,fact);return JSON.parse(result.image);
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
 return consistentObservationsV1(rows)&&rows.every(row=>{const key=observationKeyV1(row),known=checks.get(key);if(known!==undefined)return known;const holds=current([row]);checks.set(key,holds);return holds;});
}

/** 🔁️ Replays current registered parsers after byte changes and retains every nested dependency. */
function sourceAnalysisFactHoldsV1(path,fact){
 if(sourceAnalysisContractV1.validate(fact).length)return false;
 const registered=sourceAnalyses.get(fact?.provider);
 if(!registered||registered.provider.revision!==fact.revision||!Array.isArray(fact.nestedObservations)||!fact.source||!/^[a-f0-9]{64}$/u.test(fact.source.sha256??"")||!/^[a-f0-9]{64}$/u.test(fact.resultSha256??""))return false;
 if(!nestedObservationsHoldV1(fact.nestedObservations))return false;
 try{
  const bytes=fs.readFileSync(path),sha256=createHash("sha256").update(bytes).digest("hex");if(sha256===fact.source.sha256)return true;
  const replay=sourceCaptures.run(undefined,()=>recordScopes.run(undefined,()=>scopes.run(undefined,()=>sourceAnalysisImageV1(registered,path,bytes,sha256))));
  if(createHash("sha256").update(replay.image).digest("hex")!==fact.resultSha256||!current(replay.nestedObservations))return false;
  const before=new Set(fact.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)]))),after=new Set(replay.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])));
  return before.size===after.size&&[...before].every(row=>after.has(row));
 }catch{return false;}
}

/** 📁️ Fingerprints exactly the names or typed entries a directory read returned. */
function directoryFactV1(entries,typed=false){return createHash("sha256").update(JSON.stringify(entries.map(entry=>typed?[String(entry.name),nodeType(entry)]:String(entry)).sort())).digest("hex");}

/** 🔤️ Decodes the BOM forms accepted by the TypeScript host without another runtime dependency. */
function typescriptTextV1(bytes){if(bytes.length>=2&&bytes[0]===254&&bytes[1]===255){const swapped=Buffer.from(bytes);for(let index=0;index+1<swapped.length;index+=2){const byte=swapped[index];swapped[index]=swapped[index+1];swapped[index+1]=byte;}return swapped.toString("utf16le",2);}if(bytes.length>=2&&bytes[0]===255&&bytes[1]===254)return bytes.toString("utf16le",2);return bytes.toString("utf8",bytes.length>=3&&bytes[0]===239&&bytes[1]===187&&bytes[2]===191?3:0);}

/** 📂️ Replays the host's sorted immediate directories, following directory symlinks. */
function typescriptDirectoriesV1(path){try{return fs.readdirSync(path||".",{withFileTypes:true}).filter(entry=>{try{return(entry.isSymbolicLink()?fs.statSync(join(path,entry.name)):entry).isDirectory();}catch{return false;}}).map(entry=>entry.name).sort();}catch{return[];}}

/** 🔎️ Recomputes one observation's fact exactly as it was first taken. */
const facts = {
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
  typescriptText: (path) => {try{return createHash("sha256").update(typescriptTextV1(fs.readFileSync(path))).digest("hex");}catch{return "!missing";}},
  typescriptRealpath: (path) => {try{return path.length<260?fs.realpathSync.native(path):fs.realpathSync(path);}catch{return path;}},
  realpath: (path) => { try { return fs.realpathSync(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  realpathNative: (path) => { try { return fs.realpathSync.native(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  resolve: (key) => { const [from, request] = JSON.parse(key); try { return createRequire(from).resolve(request); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
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
  settleRead(absolute(path), stamp, bytes, fact);
  const encoding = encodingOf(options);
  return encoding ? bytes.toString(encoding) : bytes;
}

/** 🧊️ Source bytes whose stamp was already settled when read, so an unchanged file is read and hashed once while its stamp still matches. */
const settledReads = new Map(), SETTLED_READ_BYTES = 128 * 1024 * 1024;
let settledReadBytes = 0;
function settleRead(key, stamp, bytes, fact) {
  if (stamp instanceof Error || !Buffer.isBuffer(bytes) || bytes.length > 4 * 1024 * 1024) return;
  const taken = Date.now();
  if (!(stamp.mtimeMs < taken - RACY_MS && stamp.ctimeMs < taken - RACY_MS)) { const previous = settledReads.get(key); if (previous) { settledReadBytes -= previous.bytes.length; settledReads.delete(key); } return; }
  const previous = settledReads.get(key); if (previous) settledReadBytes -= previous.bytes.length;
  if (settledReadBytes + bytes.length > SETTLED_READ_BYTES) { settledReads.clear(); settledReadBytes = 0; }
  settledReads.set(key, { size: stamp.size, mtimeMs: stamp.mtimeMs, ctimeMs: stamp.ctimeMs, ino: stamp.ino, bytes: Buffer.from(bytes), row: canonicalObservationV1("file", key, fact) }); settledReadBytes += bytes.length;
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
    record("resolve", key, resolved);
    return resolved;
  } catch (error) { record("resolve", key, `!${error.code ?? "EUNKNOWN"}`); throw error; }
}

/** 🧩️ A TypeScript module resolution host that delegates to TypeScript's own `sys` and records every probe it answers. */
export function observedTypeScriptHost(sys) {
  return {
    ...sys,
    fileExists: (path) => probe("fileExists",path,()=>sys.fileExists(path),value=>value),
    directoryExists: (path) => probe("directoryExists",path,()=>sys.directoryExists(path),value=>value),
    readFile: (path, encoding) => probe("typescriptText",path,()=>sys.readFile(path,encoding),value=>value===undefined?"!missing":createHash("sha256").update(value).digest("hex")),
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
  if (hit !== undefined) { try { return JSON.parse(Buffer.from(hit).toString("utf8")); } catch {} }
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
  const key = absolute(path), hint = scope.hints.get(key);
  if (hint) {
    let stamp;
    try { stamp = fs.statSync(path); } catch {}
    if (stamp && stamp.size === hint.size && stamp.mtimeMs === hint.mtimeMs && stamp.ctimeMs === hint.ctimeMs && stamp.ino === hint.ino) {
      const hit = scope.records.get(recordKey(namespace, hint.sha256));
      if (hit !== undefined) {
        try { const value = JSON.parse(Buffer.from(hit).toString("utf8")); record("file", key, hint); return value; } catch {}
      }
    }
  }
  const bytes = observedFs.readFileSync(path), value = compute(bytes), image = jsonImage(value);
  if (image === undefined) return value;
  scope.records.put(recordKey(namespace, createHash("sha256").update(bytes).digest("hex")), Buffer.from(image, "utf8"));
  return JSON.parse(image);
}

/** 🌊️ Reads a module asynchronously only when its original settled byte fingerprint has no verified semantic record. */
export async function memoizedFileRecordAsyncV1(namespace,path,compute){
 const scope=recordScopes.getStore(),key=absolute(path),hint=scope?.hints.get(key);
 if(hint){
  let stamp;try{stamp=await statAsync(path);}catch{}
  if(stamp&&stamp.size===hint.size&&stamp.mtimeMs===hint.mtimeMs&&stamp.ctimeMs===hint.ctimeMs&&stamp.ino===hint.ino){
   const hit=scope.records.get(recordKey(namespace,hint.sha256));if(hit!==undefined)try{const value=JSON.parse(Buffer.from(hit).toString("utf8"));record("file",key,hint);return value;}catch{}
  }
 }
 const bytes=await observedFs.readFile(path),value=compute(bytes),image=jsonImage(value);
 if(scope&&image!==undefined&&Buffer.byteLength(image)<=1024*1024){scope.records.put(recordKey(namespace,createHash("sha256").update(bytes).digest("hex")),Buffer.from(image,"utf8"));return JSON.parse(image);}
 return value;
}

/** 🪪️ True when a recorded file fact still names the same bytes: an unchanged stamp proves it cheaply, and a restamped file of the same size is proven by its digest, so rewrites of identical content stay hits. */
function fileFactHolds(path, fact) {
  if (fact.error !== undefined) { try { fs.readFileSync(path); return false; } catch (error) { return fact.error === (error.code ?? "EUNKNOWN"); } }
  let current;
  try { current = fs.statSync(path); } catch { return false; }
  if (current.size !== fact.size) return false;
  if (current.mtimeMs === fact.mtimeMs && current.ctimeMs === fact.ctimeMs && current.ino === fact.ino) return true;
  try { return createHash("sha256").update(fs.readFileSync(path)).digest("hex") === fact.sha256; } catch { return false; }
}

/** ✅️ True when every recorded fact still holds; stops at the first stale fact. Synchronous probes beat the four-thread pool here: each is one cached-inode syscall. */
function current(observations) {
  return consistentObservationsV1(observations)&&observations.every(({ kind, path, fact }) => kind === "file" ? fileFactHolds(path, fact) : kind==="analysis"?sourceAnalysisFactHoldsV1(path,fact):typeof facts[kind] === "function" && JSON.stringify(facts[kind](path)) === JSON.stringify(fact));
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
 let records=recordStores.get(path);if(!records){records=openRecordStore(path,{publication:(plan,write)=>withBuildGraphOptionalPublicationV1(join(path,"0-0-0000000000000000.pack"),plan,write)});recordStores.set(path,records);const original=records;registerBuildGraphOptionalHandleV1(()=>{try{original.close();}finally{if(recordStores.get(path)===original)recordStores.delete(path);}});}return records;
}

/**
 * 🪨️ Runs one inference pass in rounds until a single round observed one mutually consistent source snapshot; only that round's facts reach
 * the caller's scope and capture. Projects whose images still hold are reused, so a round re-infers only the projects a concurrent edit
 * touched. A failing round is repeated while any fact it consumed has changed since, or was stamped within {@link RACY_MS} (a half-written
 * file); otherwise its failure is genuine and rethrown with its facts retained.
 * @template T
 * @param {() => Promise<T> | T} compute
 * @param {{ readonly signal?: AbortSignal; readonly limit?: number; readonly onRound?: (round: { round: number; conflict?: string; failure?: unknown }) => void | Promise<void> }} [options]
 * @returns {Promise<T>}
 */
export async function settleObservedComputationV1(compute,{signal,limit=64,onRound}={}){
 const parent=scopes.getStore(),capture=sourceCaptures.getStore(),accept=(rows,captured)=>{retainObservationsV1(parent,rows);retainObservationsV1(capture,captured);};
 for(let round=0;;round++){
  signal?.throwIfAborted();
  const nested=new Map(),captured=new Map(),started=Date.now();let value,failure,failed=false;
  try{value=await sourceCaptures.run(captured,()=>inferenceValidationScopes.run(nested,()=>scopes.run(nested,compute)));}catch(error){failed=true;failure=error;}
  signal?.throwIfAborted();
  const rows=[...nested.values()],consistent=consistentObservationsV1(rows);
  if(consistent&&!failed){accept(rows,captured.values());return value;}
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
async function observeInferenceComputationV1({cachePath,key,reusable},compute,nestedProject) {
  requireCurrentObservationV1();loadSealed = true;
  originalInferencePublications.delete(cachePath);usedInference.delete(cachePath);
  const parent=scopes.getStore();
  const digest = createHash("sha256").update(JSON.stringify([OBSERVATION_REVISION, nestedProject?inferenceAuthorityDigestV1():null, key])).digest("hex"), hints = new Map(), authority=nestedProject?[]:[...loadObservations.values()];
  let candidate;
  try {
    const text = brotliDecompressSync(fs.readFileSync(cachePath)).toString("utf8"), split = text.indexOf("\n"), header = JSON.parse(text.slice(0, split));if(Array.isArray(header.observations))header.observations=internObservationsV1(header.observations);
    if (header.digest === digest && header.resultSha256 === createHash("sha256").update(text.slice(split + 1)).digest("hex") && Array.isArray(header.observations) && (nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations))) {
      candidate={value:JSON.parse(text.slice(split+1)),header,headerDigest:createHash("sha256").update(text.slice(0,split)).digest("hex")};
    }
    for (const { kind, path, fact } of Array.isArray(header.observations) ? header.observations : []) if (kind === "file" && typeof fact?.sha256 === "string") hints.set(path, fact);
  } catch {}
  if(candidate&&await inferenceReusableV1(reusable,candidate.value)&&(nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations))){const{header,value,headerDigest}=candidate;retainSourceFactsV1(header.observations);retainObservationsV1(parent,header.observations);usedInference.set(cachePath,headerDigest);originalInferencePublications.set(cachePath,{path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});return value;}
  const observations = new Map(), started = Date.now();
  const records=observedRecordStoreV1(`${cachePath}.records`);
  const result = await inferenceValidationScopes.run(inferenceValidationScopes.getStore()??observations,()=>recordScopes.run({ records, hints }, () => scopes.run(observations, compute)));
  requireCurrentObservationV1();records.flush();
  if (records.stats.records > 2 * records.stats.accessed) records.retainAccessed();
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
    const image=JSON.stringify(result),temporary = `${cachePath}.${process.pid}.${started}.tmp`, header = `{"digest":${JSON.stringify(digest)},"resultSha256":"${createHash("sha256").update(image).digest("hex")}","authorityObservations":${observationRowsJsonV1(authority)},"observations":${observationRowsJsonV1(rows)}}`;
    const packed=await packInferenceRecordV1(Buffer.from(`${header}\n${image}`)),bytes=packed.byteLength;let previous=0;try{previous=fs.lstatSync(cachePath).size;}catch(error){if(error.code!=="ENOENT")throw error;}
    if(withBuildGraphOptionalPublicationV1(cachePath,{peakBytes:bytes,retainedDeltaBytes:bytes-previous},()=>{fs.writeFileSync(temporary, packed,{flag:"wx"});fs.renameSync(temporary,cachePath);}))usedInference.set(cachePath,createHash("sha256").update(header).digest("hex"));
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
  if(recordDirectory)try{const previous=JSON.parse(fs.readFileSync(publicationPath,"utf8"));if(previous.version===1&&previous.revision===OBSERVATION_REVISION&&Array.isArray(previous.observations)&&Number.isSafeInteger(previous.startedAt))for(const row of previous.observations)if(row.kind==="file"&&/^[a-f0-9]{64}$/u.test(row.fact?.sha256??"")&&row.fact.mtimeMs<previous.startedAt-RACY_MS&&row.fact.ctimeMs<previous.startedAt-RACY_MS)hints.set(row.path,row.fact);}catch{}
  const records=recordDirectory?observedRecordStoreV1(recordDirectory):undefined;
  const run=()=>scopes.run(observations,compute),result=await(records?recordScopes.run({records,hints},run):run());
  requireCurrentObservationV1();records?.flush();if(records&&records.stats.records>2*records.stats.accessed)records.retainAccessed();
  const rows = [...observations.values()], inference = [...usedInference].filter(([path]) => dirname(path) === dirname(receiptPath)).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
  if(!consistentObservationsV1(rows))throw conflictingInferenceErrorV1(rows);
  const racy = rows.some(({ kind, fact }) => {const source=kind==="file"?fact:kind==="analysis"?fact.source:undefined;return source&&(source.mtimeMs>=started-RACY_MS||source.ctimeMs>=started-RACY_MS);});
  fs.mkdirSync(dirname(receiptPath), { recursive: true });
  const originalInference=[...originalInferencePublications.values()].filter(value=>dirname(value.path)===dirname(receiptPath)).sort((left,right)=>Buffer.compare(Buffer.from(left.path),Buffer.from(right.path)));
  if(publicationPath===receiptPath)throw Error("Original inference receipt must have an owned publication path");
  const producerPath=resolve(dirname(publicationPath),"p-"+process.pid,basename(publicationPath));
  if(originalInference.length&&jsonImage(result)!==undefined){const image=JSON.stringify({version:1,revision:OBSERVATION_REVISION,pid:process.pid,startedAt:started,completedAt:Date.now(),inference:originalInference,observations:rows});fs.mkdirSync(dirname(producerPath),{recursive:true});for(const path of [producerPath,publicationPath]){const temporary=path+"."+process.pid+"."+started+".tmp",parent=resolve(dirname(path)),status=fs.lstatSync(parent);if(temporary.length>256||[...temporary].length>256||!status.isDirectory()||status.isSymbolicLink()||fs.realpathSync(parent)!==parent)throw Error("Original inference producer namespace refused");fs.writeFileSync(temporary,image,{flag:"wx"});fs.renameSync(temporary,path);}}else for(const path of [producerPath,publicationPath])fs.rmSync(path,{force:true});
  if (racy || inference.length === 0 || inference.some(([, digest]) => digest === null) || !current(rows)) { fs.rmSync(receiptPath, { force: true }); return result; }
  const temporary = `${receiptPath}.${process.pid}.${started}.tmp`;
  fs.writeFileSync(temporary, JSON.stringify({ format: 1, revision: OBSERVATION_REVISION, pid: process.pid, completedAt: Date.now(), inference, observations: rows }));
  fs.renameSync(temporary, receiptPath);
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
export function inferenceHeaderLine(cachePath) {
  const text = brotliDecompressSync(fs.readFileSync(cachePath)).toString("utf8"), newline = text.indexOf("\n");
  return newline < 0 ? null : text.slice(0, newline);
}

/**
 * ✅️ {@link current} without blocking: every recorded fact re-checked through the libuv pool, `limit` at a time; false at the first stale
 * fact. Existence, type and listing facts about entries of a directory in `stableDirectories` (absolute paths whose entry sets are proven
 * unchanged, e.g. by unchanged directory stamps) hold without a probe: an entry can only appear, vanish or change type by changing its parent.
 * Existence follows symbolic links, so it is probed for every entry in `symlinks`.
 */
export function observationsHold(observations, limit = 64, stableDirectories = new Set(), symlinks = new Set(),signal,onStale) {
  return inferenceValidationScopes.run(inferenceValidationScopes.getStore() ?? new Map(), () => observationsHoldInRoundV1(observations, limit, stableDirectories, symlinks, signal, onStale));
}

async function observationsHoldInRoundV1(observations, limit, stableDirectories, symlinks, signal, onStale) {
  signal?.throwIfAborted();if(!consistentObservationsV1(observations)){const seen=new Map();for(const row of observations){const key=observationIdentityV1(row),fact=observationMeaningV1(row);if(seen.has(key)&&seen.get(key)!==fact){onStale?.(row);break;}seen.set(key,fact);}return false;}
  let next = 0, stale = false;
  const check = async ({ kind, path, fact }) => {
    const parent = path.slice(0, Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")));
    if (kind === "lstat" && stableDirectories.has(parent) || kind === "exists" && stableDirectories.has(parent) && !symlinks.has(path) || kind === "readdir" && stableDirectories.has(path)) return true;
    if (kind === "file") {
      if (fact.error !== undefined) return fileFactHolds(path, fact);
      let stamp;
      try { stamp = await statAsync(path); } catch { return false; }
      if (stamp.size !== fact.size) return false;
      if (stamp.mtimeMs === fact.mtimeMs && stamp.ctimeMs === fact.ctimeMs && stamp.ino === fact.ino) return true;
      try { return createHash("sha256").update(await readFileAsync(path)).digest("hex") === fact.sha256; } catch { return false; }
    }
    if (kind === "exists") { try { await statAsync(path); return fact === true; } catch { return fact === false; } }
    if(kind==="analysis")return sourceAnalysisFactHoldsV1(path,fact);
    return typeof facts[kind] === "function" && JSON.stringify(facts[kind](path)) === JSON.stringify(fact);
  };
  const worker = async () => { while (!stale && next < observations.length){signal?.throwIfAborted();const row=observations[next++];if (!await check(row)){stale = true;onStale?.(row);}} };
  await Promise.all(Array.from({ length: Math.min(limit, observations.length) }, worker));
  return !stale;
}
