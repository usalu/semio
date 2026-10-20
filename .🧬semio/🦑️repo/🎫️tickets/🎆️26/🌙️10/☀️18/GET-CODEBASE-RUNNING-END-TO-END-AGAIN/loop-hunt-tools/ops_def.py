"""Defines the staged operations and writes staged/ops.json (anchors are exact live text; apply.py refuses drift)."""
import json,pathlib
HERE=pathlib.Path(__file__).resolve().parent
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OBS=f"{L}/🏗️build/👁️observation/🟨️.mjs"
NODES=f"{L}/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
PHASES=f"{L}/🏗️build/🕸️graph/⏱️phases/🟨️.mjs"
PHASES_D=f"{L}/🏗️build/🕸️graph/⏱️phases/🟨️.d.mts"
TEST=f"{L}/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat"
edits=[]
def edit(file,anchor,replacement):edits.append({"file":file,"anchor":anchor,"replacement":replacement})

edit(OBS," * touched. A failing round is repeated while any fact it consumed has changed since, or was stamped within {@link RACY_MS} (a half-written\n * file); otherwise its failure is genuine and rethrown with its facts retained.",
" * touched. A failing round is repeated while any fact it consumed has changed since, or was stamped within {@link RACY_MS} (a half-written\n * file); otherwise its failure is genuine and rethrown with its facts retained. A failure that demands a restart (`BUILD_AUTHORITY_CHANGED`) or\n * that returns unchanged (same code and message) in the next round is genuine as well: a peer editing the tree must not keep a deterministic\n * failure looking transient for `limit` whole-inference rounds.")
edit(OBS,"accept=(rows,captured)=>{retainObservationsV1(parent,rows);retainObservationsV1(capture,captured);};\n for(let round=0;;round++){",
"accept=(rows,captured)=>{retainObservationsV1(parent,rows);retainObservationsV1(capture,captured);};\n let previousFailure;\n for(let round=0;;round++){")
edit(OBS,"  if(consistent&&!failed){accept(rows,captured.values());return value;}\n  if(consistent&&await observationsHold(",
"  if(consistent&&!failed){accept(rows,captured.values());return value;}\n  const signature=failed?JSON.stringify([failure?.code??null,(()=>{try{return String(failure?.message??failure);}catch{return\"\";}})()]):undefined;\n  if(failed&&(failure?.code===\"BUILD_AUTHORITY_CHANGED\"||signature===previousFailure)){accept(rows,captured.values());throw failure;}\n  previousFailure=signature;\n  if(consistent&&await observationsHold(")

edit(PHASES,"/** 🖨️ Writes the summary of a finished construction: always one line, so a slow cold build explains itself. */",
"/** 🔁️ Writes one machine readable row per repeated settlement round: the round, the first conflicting source or the failure (message, code, project) that made the whole inference start over; never silent, at most one row per round. */\nexport function writeGraphSettleRowV1(round){\n const text=value=>{try{return String(value?.message??value).slice(0,512);}catch{return\"\";}};\n process.stderr.write(\"[graph-settle] \"+JSON.stringify({round:round.round,...(round.conflict===undefined?{}:{conflict:round.conflict}),...(Object.hasOwn(round,\"failure\")?{failure:text(round.failure),...(round.failure?.code?{code:String(round.failure.code)}:{}),...(round.failure?.semioProject?{project:String(round.failure.semioProject)}:{})}:{})})+\"\\n\");\n}\n\n/** 🖨️ Writes the summary of a finished construction: always one line, so a slow cold build explains itself. */")
edit(PHASES_D,"export function writeGraphPhaseSummaryV1(rows:readonly GraphPhaseRowV1[]):void;",
"export function writeGraphPhaseSummaryV1(rows:readonly GraphPhaseRowV1[]):void;\n/** 🔁️ One repeated settlement round: its index, the first conflicting source or the failure that made the whole inference start over. */\nexport interface GraphSettleRoundV1{readonly round:number;readonly conflict?:string;readonly failure?:unknown}\nexport function writeGraphSettleRowV1(round:GraphSettleRoundV1):void;")

edit(NODES,'import { createGraphPhaseClockV1, writeGraphPhaseRowV1 } from "../../⏱️phases/🟨️.mjs";','import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";')
edit(NODES,'/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */',
'/** 🏷️ Names the project a failing inference belongs to without replacing the error, so a repeated settlement round says which project stops the build. */\nasync function tagProjectFailureV1(root, run) {\n  try { return await run(); }\n  catch (error) { if (error !== null && typeof error === "object" && !Object.hasOwn(error, "semioProject") && Object.isExtensible(error)) Object.defineProperty(error, "semioProject", { value: root, configurable: true }); throw error; }\n}\n\n/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */')
edit(NODES,'  return { signal: context.phaseControl?.signal, onRound: async () => { await yieldInferenceV1(context,"build:settle",0,true); } };',
'  return { signal: context.phaseControl?.signal, onRound: async round => { writeGraphSettleRowV1(round); await yieldInferenceV1(context,"build:settle",0,true); } };')
edit(NODES,'    result.projects[name]=await observeProjectInferenceV1({cachePath,','    result.projects[name]=await tagProjectFailureV1(root,()=>observeProjectInferenceV1({cachePath,')
edit(NODES,'return {unavailable:unavailableProjectV1(root,name,workspaceRoot,error)};}\n    });','return {unavailable:unavailableProjectV1(root,name,workspaceRoot,error)};}\n    }));')

newFiles=[
 {"file":f"{TEST}/🟦️.ts","from":"staged/new/settle-repeat/test.ts"},
 {"file":f"{TEST}/🔣️.json","from":"staged/new/settle-repeat/fixture.json"},
 {"file":f"{TEST}/🧬️schema/🔣️.json","from":"staged/new/settle-repeat/schema.json"},
 {"file":f"{TEST}/🐍️oracle.py","from":"staged/new/settle-repeat/oracle.py"},
]
(HERE/"staged").mkdir(exist_ok=True)
(HERE/"staged/ops.json").write_text(json.dumps({"version":1,"edits":edits,"newFiles":newFiles},ensure_ascii=False,indent=1),encoding="utf8")
print(len(edits),"edits",len(newFiles),"new files")
