import os,sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B=R+"📚️library/🏗️build/"
edits=[
(B+"🌐️remote-cache/🛰️fleet/🟦️.ts",
 "selectedExecution:foundExecution?.endpoint??null,facts};",
 "selectedExecution:foundExecution?.endpoint??keptExecution,facts};",
 "  const copy:FleetCacheLocalCopyV1={",
 "  const keptExecution=foundExecution===undefined&&execution!==undefined&&local?.selectedExecution!=null&&local.facts.execution?.origin===execution.origin&&local.facts.execution.port===execution.port?local.selectedExecution:null;\n  const copy:FleetCacheLocalCopyV1={"),
(B+"🌐️remote-execution/🛰️sidecar/🟦️.ts",
 "  const entrySha256=protocol.sidecarEntryHashV1(entry)??\"0\".repeat(64),name=environment.SEMIO_FLEET_SIDECAR??role,beat=():void=>{try{protocol.writeSidecarRecordV1(",
 "  const entrySha256=protocol.sidecarEntryHashV1(entry)??\"0\".repeat(64),name=environment.SEMIO_FLEET_SIDECAR??role,beat=():void=>{try{const other=protocol.readSidecarRecordV1(record);if(other!==undefined&&other.pid!==process.pid&&other.startedMs>startedMs&&protocol.sidecarRecordLivesV1(other)){process.stderr.write(`${new Date().toISOString()} ${role} superseded by launcher ${other.pid} (started later), leaving / ersetzt durch Starter ${other.pid} (später gestartet), beendet sich\\n`);stop();return;}protocol.writeSidecarRecordV1(",
 None,None),
(B+"🌐️remote-execution/🚀️offload/🟦️.ts",
 " const endpoint=ports?.endpoint??resolveRemoteExecutionEndpointV1({environment,...workspace===undefined?{}:{root:workspace}})?.endpoint;\n if(!endpoint)return{decline:\"no-endpoint\",detail:\"\"};",
 " const resolveEndpoint=():string|undefined=>ports?.endpoint??resolveRemoteExecutionEndpointV1({environment,...workspace===undefined?{}:{root:workspace}})?.endpoint;\n let endpoint=resolveEndpoint();\n if(!endpoint&&workspace!==undefined){await refreshFleetCacheFactsV1({root:workspace,ttlMilliseconds:5000});endpoint=resolveEndpoint();}\n if(!endpoint)return{decline:\"no-endpoint\",detail:explainRemoteExecutionEndpointAbsenceV1({environment,...workspace===undefined?{}:{root:workspace}})};",
 None,None),
(B+"🌐️remote-execution/🚀️offload/🟦️.ts",
 "import {fleetCacheRootV1,remoteCacheFetchV1,resolveRemoteCacheV1} from",
 "import {fleetCacheRootV1,refreshFleetCacheFactsV1,remoteCacheFetchV1,resolveRemoteCacheV1} from",None,None),
(B+"🌐️remote-execution/🚀️offload/🟦️.ts",
 "import {resolveRemoteExecutionEndpointV1} from \"../🧭️endpoint/🟦️.ts\";",
 "import {explainRemoteExecutionEndpointAbsenceV1,resolveRemoteExecutionEndpointV1} from \"../🧭️endpoint/🟦️.ts\";",None,None),
]
for path,old,new,_a,_b in edits:
    s=open(path,encoding="utf8").read()
    if s.count(old)!=1: sys.exit(f"anchor count {s.count(old)} in {path}: {old[:60]}")
    if _a:
        if s.count(_a)!=1: sys.exit("pre-anchor")
        s=s.replace(_a,_b)
    s=s.replace(old,new)
    t=path+".f52tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,path)
# endpoint module append
p=B+"🌐️remote-execution/🧭️endpoint/🟦️.ts"
s=open(p,encoding="utf8").read()
if "explainRemoteExecutionEndpointAbsenceV1" not in s:
    s=s.rstrip("\n")+'''

/** 🔎️ Why no scheduler endpoint resolves, one line in English and German (the detail of a `no-endpoint` decline): no checkout known, no readable copy of the fleet facts, no scheduler published, or no address of the scheduler origin answered. Synchronous, reads the local copy only. */
export function explainRemoteExecutionEndpointAbsenceV1(input:Readonly<{environment:Environment;root?:string}>):string{
 const root=input.root??fleetCacheRootV1(input.environment);
 if(root===undefined)return "no fleet checkout is known (SEMIO_WORKSPACE_ROOT) / kein Flotten-Checkout bekannt (SEMIO_WORKSPACE_ROOT)";
 const copy=readLocalFleetCacheFactsV1(root);
 if(copy===undefined)return "no readable copy of the fleet facts (.cache/f/cache-facts.json) / keine lesbare Kopie der Flottenfakten (.cache/f/cache-facts.json)";
 const execution=copy.facts.execution;
 if(execution===undefined)return "the fleet facts publish no scheduler / die Flottenfakten veröffentlichen keinen Scheduler";
 return `no address of ${execution.origin} answers on port ${execution.port} / keine Adresse von ${execution.origin} antwortet am Port ${execution.port}`;
}
'''
    t=p+".f52tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p)
print("ok")
