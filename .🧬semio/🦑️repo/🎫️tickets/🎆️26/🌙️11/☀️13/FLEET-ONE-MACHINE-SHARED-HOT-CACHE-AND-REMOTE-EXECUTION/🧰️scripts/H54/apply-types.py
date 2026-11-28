import os,sys
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/"
P="🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/"
def edit(path,old,new):
    s=open(path,encoding="utf8").read()
    if new in s: return
    assert s.count(old)==1,(path,old,s.count(old))
    t=path+".h54tmp"
    open(t,"w",encoding="utf8").write(s.replace(old,new));os.replace(t,path)
edit(R+"🔐️consent/🟨️.d.mts","export const RESET_MARKER_V1:string;","export const RESET_MARKER_V1:string;\nexport const LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1:number;")
edit(R+"🟨️.d.mts","export {HostRunningErrorV1, createHostServiceV1} from","export {HostRunningErrorV1, createHostServiceV1, olderBusyServiceV1} from")
edit(R+"🟨️.d.mts","export {ACCESS_PROOF_VERSION_V1, PROTECTED_FOLDERS_V1,","export {ACCESS_PROOF_VERSION_V1, LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1, PROTECTED_FOLDERS_V1,")
edit(R+"🏗️composition/🌐️host/👷️slice-pool/🟨️.d.mts"," planGuard?:unknown;"," growth?:unknown;coalesce?:Readonly<{gatherMilliseconds:number;maximumTasks:number}>;planGuard?:unknown;")
edit(P+"🟦️.ts","export const processTestProtocolV1=protocol;",'export const processTestProtocolV1=Object.freeze({...protocol,output:Object.freeze({...protocol.output,overflow:protocol.output.overflow==="refuse"?"refuse" as const:"elide" as const})});')
