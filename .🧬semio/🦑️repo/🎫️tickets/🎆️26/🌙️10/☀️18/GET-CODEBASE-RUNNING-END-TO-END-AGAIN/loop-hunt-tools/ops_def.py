"""Defines the staged operations of set 2 (diagnostic collect-errors mode) and writes staged/ops.json; set 1 (settle repeat law) is archived in staged/ops-applied-1.json. Anchors are exact live text; apply.py refuses drift."""
import json,pathlib
HERE=pathlib.Path(__file__).resolve().parent
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G=f"{L}/🏗️build/🕸️graph"
OBS=f"{L}/🏗️build/👁️observation/🟨️.mjs"
NODES=f"{G}/📥️inference/📦️project-nodes/🟨️.mjs"
SETTLE_TEST=f"{L}/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat"
COLLECT=f"{G}/🧾️collect-errors"
edits=[]
def edit(file,anchor,replacement):edits.append({"file":file,"anchor":anchor,"replacement":replacement})

edit(OBS,'if(failed&&(failure?.code==="BUILD_AUTHORITY_CHANGED"||signature===previousFailure))','if(failed&&(failure?.code==="BUILD_AUTHORITY_CHANGED"||failure?.code==="BUILD_COLLECTED_ERRORS"||signature===previousFailure))')
edit(f"{SETTLE_TEST}/🔣️.json",'    { "name": "accepted-first-round",','    { "name": "collected-errors-are-final", "rounds": [{ "outcome": "failure", "message": "Error collection finished", "code": "BUILD_COLLECTED_ERRORS", "young": true }, { "outcome": "value" }], "expect": { "settled": "failure", "rounds": 1, "failure": "Error collection finished", "events": [] } },\n    { "name": "accepted-first-round",')
edit(f"{SETTLE_TEST}/🐍️oracle.py",'if row.get("code")=="BUILD_AUTHORITY_CHANGED" or signature==previous:','if row.get("code") in ("BUILD_AUTHORITY_CHANGED","BUILD_COLLECTED_ERRORS") or signature==previous:')

edit(NODES,'import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";','import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";\nimport { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";')
edit(NODES,'wanted = root => !wantedRoots || wantedRoots.has(root);','wanted = root => !wantedRoots || wantedRoots.has(root);\n  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });')
edit(NODES,'    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);\n    const project = {','    const build = async () => {\n    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);\n    const project = {')
edit(NODES,'    results.push([configFile, { projects: { [name]: project } }]);\n  }\n','    results.push([configFile, { projects: { [name]: project } }]);\n    };\n    if (collector) await collector.guard({ project: name, root, phase: "cargo-targets" }, build); else await build();\n  }\n')
edit(NODES,'await inferProject(jobs[cursor++]);await phases?.advance("inference",++finished);','{const job=jobs[cursor++];if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job));else await inferProject(job);}await phases?.advance("inference",++finished);')
edit(NODES,'  await phases?.end("inference",{projects:jobs.length});\n','  await phases?.end("inference",{projects:jobs.length});\n  if (collector) await collector.finish({ projects: jobs.length });\n')

newFiles=[
 {"file":f"{COLLECT}/🟨️.mjs","from":"staged/new/collect/module.mjs"},
 {"file":f"{COLLECT}/🟨️.d.mts","from":"staged/new/collect/module.d.mts"},
 {"file":f"{COLLECT}/🧬️schema/🔣️.json","from":"staged/new/collect/schema.json"},
 {"file":f"{COLLECT}/🧪️tests/🟦️.ts","from":"staged/new/collect/test.ts"},
 {"file":f"{COLLECT}/🧪️tests/🔣️.json","from":"staged/new/collect/fixture.json"},
 {"file":f"{COLLECT}/🧪️tests/🐍️oracle.py","from":"staged/new/collect/oracle.py"},
]
(HERE/"staged/ops.json").write_text(json.dumps({"version":2,"edits":edits,"newFiles":newFiles},ensure_ascii=False,indent=1),encoding="utf8")
print(len(edits),"edits",len(newFiles),"new files")
