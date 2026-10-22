L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
O = L + "/🏗️build/👁️observation/🟨️.mjs"
D = L + "/🏗️build/👁️observation/🟨️.d.mts"
memo = '''const verifiedBlocks=new Map(),VERIFIED_BLOCK_ROWS=4194304;let verifiedBlockRows=0;
/** 🧊️ Remembers a block whose bytes, hash and rows this process verified, by the full identity of its leaf. */
function rememberVerifiedBlockV1(path,epoch,rows){if(verifiedBlockRows+rows.length>VERIFIED_BLOCK_ROWS){verifiedBlocks.clear();verifiedBlockRows=0;}const prior=verifiedBlocks.get(path);if(prior)verifiedBlockRows-=prior.rows.length;verifiedBlocks.set(path,{epoch,rows,recheck:undefined});verifiedBlockRows+=rows.length;}
/** 🧊️ A verified block while its leaf still has the identity it was verified with (device, inode, mode, links, size and both nanosecond stamps); any other leaf is read and validated again. */
function verifiedBlockV1(path){
 const entry=verifiedBlocks.get(path);if(entry===undefined)return undefined;
 try{const row=fs.lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&row.nlink===1n&&observationEpochV1(row)===entry.epoch){entry.recheck??=()=>{requireRepositoryBuildGraphObservationPathV1(path,"read");const actual=fs.lstatSync(path,{bigint:true});if(!actual.isFile()||actual.isSymbolicLink()||observationEpochV1(actual)!==entry.epoch)throw Error("Observation leaf changed before delivery");};return entry;}}catch{}
 verifiedBlockRows-=entry.rows.length;verifiedBlocks.delete(path);return undefined;
}
function readObservationBlockV1(directory,digest,optional=false,budget=observationReadBudgetV1()){const path=inferenceObservationBlockPathV1(directory,digest),verified=verifiedBlockV1(path);if(verified){requireRepositoryBuildGraphObservationPathV1(path,"read");observationChargeV1(budget,"rows",verified.rows.length);return{rows:verified.rows,recheck:verified.recheck,epoch:verified.epoch};}const physical='''
lookup_old = '''  let candidate;
  const record=readInferencePhysicalRecordV1(cachePath,true);if(record){const{header,line}=record;if(header.digest===digest&&(nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations)))candidate={value:record.value,header,headerDigest:createHash("sha256").update(line).digest("hex")};for(const{kind,path,fact}of header.observations)if(kind==="file"&&typeof fact?.sha256==="string")hints.set(path,fact);}
  if(candidate&&await inferenceReusableV1(reusable,candidate.value)&&(nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations))){const{header,value,headerDigest}=candidate;retainSourceFactsV1(header.observations);retainObservationsV1(parent,header.observations);usedInference.set(cachePath,headerDigest);originalInferencePublications.set(cachePath,{path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});return value;}
'''
lookup_new = '''  const lookup=async()=>{
  let candidate;
  const record=readInferencePhysicalRecordV1(cachePath,true);if(record){const{header,line}=record;if(header.digest===digest&&(nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations)))candidate={value:record.value,header,headerDigest:createHash("sha256").update(line).digest("hex")};for(const{kind,path,fact}of header.observations)if(kind==="file"&&typeof fact?.sha256==="string")hints.set(path,fact);}
  if(candidate&&await inferenceReusableV1(reusable,candidate.value)&&(nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations))){const{header,value,headerDigest}=candidate;retainSourceFactsV1(header.observations);retainObservationsV1(parent,header.observations);usedInference.set(cachePath,headerDigest);originalInferencePublications.set(cachePath,{path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});return{value};}
  return undefined;
  };
  const reused=await lookup()??(prepare!==undefined&&await prepare()?await lookup():undefined);
  if(reused)return reused.value;
'''
EDITS = [
 (O, 'function readObservationBlockV1(directory,digest,optional=false,budget=observationReadBudgetV1()){const path=inferenceObservationBlockPathV1(directory,digest);const physical=', memo),
 (O, 'observationChargeV1(budget,"rows",rows.length);physical.recheck();return{rows:internObservationsV1(rows),recheck:physical.recheck};}', 'observationChargeV1(budget,"rows",rows.length);physical.recheck();const shared=internObservationsV1(rows);rememberVerifiedBlockV1(path,physical.epoch,shared);return{rows:shared,recheck:physical.recheck,epoch:physical.epoch};}'),
 (O, 'recheck();return{bytes,recheck};', 'recheck();return{bytes,recheck,epoch};'),
 (O, lookup_old, lookup_new),
 (O, 'async function observeInferenceComputationV1({cachePath,key,reusable},compute,nestedProject) {', 'async function observeInferenceComputationV1({cachePath,key,reusable,prepare},compute,nestedProject) {'),
 (O, 'export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[]},compute){', 'export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare},compute){'),
 (O, 'const value=await observeInferenceComputationV1({cachePath,key,reusable},async()=>{projectComputations.set(owner,', 'const value=await observeInferenceComputationV1({cachePath,key,reusable,prepare},async()=>{projectComputations.set(owner,'),
 (O, '/** 🪪️ Seals module-load authority', '/** 🔑️ The identity every persisted project record of this process is written under: its record stamp and the authority digest of its module loads, equal in every process whose records another may reuse. */\nexport function inferenceRecordIdentityV1(){return createHash("sha256").update(JSON.stringify([recordStampV1(),inferenceAuthorityDigestV1()])).digest("hex");}\n\n/** 🪪️ Seals module-load authority'),
 (D, 'export function inferenceAuthorityDigestV1():string;', 'export function inferenceAuthorityDigestV1():string;\n/** 🪪️ The identity every persisted project record of this process is written under. */\nexport function inferenceRecordIdentityV1():string;'),
 (D, 'stamps?:readonly string[]}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;', 'stamps?:readonly string[];prepare?:()=>boolean|PromiseLike<boolean>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;'),
]
