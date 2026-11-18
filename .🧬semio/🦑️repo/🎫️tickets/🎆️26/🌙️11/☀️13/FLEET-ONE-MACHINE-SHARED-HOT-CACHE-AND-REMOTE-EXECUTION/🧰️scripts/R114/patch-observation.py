import sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs"
out=sys.argv[1]
s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('requireRepositoryBuildGraphObservationPathV1} from "../📥️inputs/💾️storage/🟨️.mjs";','requireRepositoryBuildGraphObservationPathV1 as requireManagedGraphObservationPathV1} from "../📥️inputs/💾️storage/🟨️.mjs";')
rep('const recordScopes = new AsyncLocalStorage();\n','const recordScopes = new AsyncLocalStorage();\nconst plainRecordReads = new AsyncLocalStorage();\nconst requireRepositoryBuildGraphObservationPathV1 = (path, mode) => mode === "read" && plainRecordReads.getStore() === true ? Object.freeze({ kind: "ordinary" }) : requireManagedGraphObservationPathV1(path, mode);\n')
rep('const lookupClockV1={count:0,read:0,settle:0,hold:0,reusable:0,offer:0,gate:0};','const lookupClockV1={count:0,read:0,verdict:0,settle:0,hold:0,reusable:0,offer:0,gate:0};')
rep('async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false},compute,nestedProject) {','async function observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer=false,verdict},compute,nestedProject) {')
rep('if(header.digest===digest){await lookupLapAsyncV1("settle",()=>settleClosureRowsV1(header.observations,record.value));if(await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(header.observations,record.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest))))candidate={value:record.value,header,headerDigest:sha256HexV1(line)};}',
 'if(header.digest===digest){const headerDigest=sha256HexV1(line),said=verdict===undefined?undefined:await lookupLapAsyncV1("verdict",()=>verdict({headerDigest})).catch(()=>undefined);if(said?.verdict==="holds"&&said.headerDigest===headerDigest)candidate={value:record.value,header,headerDigest,adopted:true};else{await lookupLapAsyncV1("settle",()=>settleClosureRowsV1(header.observations,record.value));if(await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(header.observations,record.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest))))candidate={value:record.value,header,headerDigest};}}')
rep('if(candidate&&await lookupLapAsyncV1("reusable",()=>inferenceReusableV1(reusable,candidate.value))&&await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(candidate.header.observations,candidate.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest)))){','if(candidate&&await lookupLapAsyncV1("reusable",()=>inferenceReusableV1(reusable,candidate.value))&&(candidate.adopted===true||await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(candidate.header.observations,candidate.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest))))){')
rep('export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare,fleet,deferOffer=false},compute){','export async function observeProjectInferenceV1({cachePath,key,owner,reusable,stamps=[],prepare,fleet,deferOffer=false,verdict},compute){')
rep('observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer},async()=>{projectComputations','observeInferenceComputationV1({cachePath,key,reusable,prepare,fleet,deferOffer,verdict},async()=>{projectComputations')
rep('export async function observeInference({ cachePath, key, reusable }, compute) {\n  requireCurrentObservationV1();requireInferenceReuseV1(reusable);\n  return observeInferenceComputationV1({cachePath,key,reusable},compute,false);','export async function observeInference({ cachePath, key, reusable, verdict }, compute) {\n  requireCurrentObservationV1();requireInferenceReuseV1(reusable);\n  return observeInferenceComputationV1({cachePath,key,reusable,verdict},compute,false);')
rep('/** 🪡️ Records the answer one closure entry gave to the running inference. */','''/** 🤝️ The verdict of one process on one stored project record, the only verification the record gets: the record is read plainly (a farm child holds no custody of the graph store and only reads), its closure entries are settled and every observed row is verified, the closure rows through the provider and the others as they stand now. `headerDigest` names the record by the digest of its header line, which covers every observed row and every block digest the record names, so a reader that holds the same header and adopts a `holds` verdict verifies nothing itself; `other` is a record of another key, `missing` none, `stale` a row that no longer holds. A verdict is single use and never kept: a later verdict judges the files as they are then.
 * @see ./🧪️tests/🤝️adopted-verdict */
export async function verifyStoredProjectRecordV1({cachePath,key}){
 requireCurrentObservationV1();
 const record=plainRecordReads.run(true,()=>readInferenceRecordOrMissV1(cachePath,{say:()=>{}}));
 if(!record)return{verdict:"missing",verifier:process.pid};
 const{header,line,value}=record,headerDigest=sha256HexV1(line);
 if(header.digest!==sha256HexV1(JSON.stringify([recordStampV1(),null,key])))return{verdict:"other",headerDigest,verifier:process.pid};
 await settleClosureRowsV1(header.observations,value);
 const holds=await projectRowsHoldV1(header.observations,value,rest=>current(rest));
 return{verdict:holds?"holds":"stale",headerDigest,rows:header.observations.length,verifier:process.pid};
}
/** 🪡️ Records the answer one closure entry gave to the running inference. */''')
open(out,"w").write(s)
