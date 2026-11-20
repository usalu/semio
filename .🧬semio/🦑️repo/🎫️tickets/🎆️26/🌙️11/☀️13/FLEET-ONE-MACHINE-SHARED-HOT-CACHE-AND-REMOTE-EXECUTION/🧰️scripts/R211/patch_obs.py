import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep('import { hollowDirectoryV1, sourceTreeFilterV1 } from "./📂️source-names/🟨️.mjs";','import { hollowDirectoryV1, machineLocalRowV1, sourceTreeFilterV1 } from "./📂️source-names/🟨️.mjs";')
rep(' const seen=rebaseRowV1(row);\n const factText=JSON.stringify(row.fact);',' const seen=rebaseRowV1(row);\n if(machineLocalRowV1(seen.kind,seen.path))return true;\n const factText=JSON.stringify(row.fact);')
rep('export function observationRowFreshV1(row){const seen=rebaseRowV1(row);return seen.kind==="file"?','export function observationRowFreshV1(row){const seen=rebaseRowV1(row);if(machineLocalRowV1(seen.kind,seen.path))return true;return seen.kind==="file"?')
rep('return consistentObservationsV1(observations)&&observations.every(({ kind, path, fact }) => kind === "file" ?','return consistentObservationsV1(observations)&&observations.every(({ kind, path, fact }) => machineLocalRowV1(kind, path) || kind === "file" ?')
rep('    const absent = fact === false;\n    const loadedAnswer','    if (machineLocalRowV1(kind, path)) return true;\n    const absent = fact === false;\n    const loadedAnswer')
rep('function record(kind, path, fact) {\n  footprintPulseV1();','function record(kind, path, fact) {\n  if(machineLocalRowV1(kind,path))return;\n  footprintPulseV1();')
rep(''' const registered=sourceAnalyses.get(fact?.provider);
 if(!registered){''',''' if(machineLocalRowV1("analysis",path))return undefined;
 const registered=registeredAnalysisV1(fact);
 if(!registered){''')
rep('''function sourceAnalysisFailureV1(path,fact){''','''function sourceAnalysisFailureV1(path,fact){''')
rep('''  const before=new Set(fact.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)]))),after=new Set(replay.nestedObservations.map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])));''','''  const before=new Set(fact.nestedObservations.map(replayRowKeyV1)),after=new Set(replay.nestedObservations.map(replayRowKeyV1));''')
rep('''/** 🧐️ Why a recorded analysis fact does not hold here''','''/** 🪪️ The registered parser a recorded analysis fact names: judged by what the parser is (its role, the id without the owner hash, and its revision) and not by the id this process happened to assign (the id hashes the module location and the loaded revision, so it differs between processes, roots and machines); the exact id wins when it carries the recorded revision, else any registered parser of the same role with that revision, else the exact id (so that the reason can name it).
 * @see ./🧪️tests/🪪️parser-identity */
function registeredAnalysisV1(fact){
 const exact=sourceAnalyses.get(fact?.provider);
 if(exact?.provider.revision===fact?.revision)return exact;
 const id=String(fact?.provider),role=id.slice(0,id.lastIndexOf(":")+1);
 if(role!=="")for(const[key,entry]of sourceAnalyses)if(key.startsWith(role)&&entry.provider.revision===fact?.revision)return entry;
 return exact;
}
/** 🔑️ A nested row as the comparison of a replay sees it: an analysis row by the role of its parser (no owner hash), every other row by identity and meaning. */
function replayRowKeyV1(row){
 if(row?.kind!=="analysis"||typeof row.fact?.provider!=="string")return JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)]);
 const id=row.fact.provider;return JSON.stringify(["analysis",row.path,id.slice(0,id.lastIndexOf(":")+1),row.fact.revision,row.fact.resultSha256]);
}
/** 🧐️ Why a recorded analysis fact does not hold here''')
open(p,"w",encoding="utf8").write(s)
