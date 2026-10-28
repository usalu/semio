"""🧾 Anchored, idempotent edits of batch `witpub` on a root: edit.py <root> check|apply. The input witness becomes a publisher-written source file (script.ts: plan written by the planner, files written by the refresh, BUILD/Starlark without the witness action; plan-reuse policy: `.witness` files are publisher outputs)."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
B = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel")
EDITS = {
 "📜️script.ts": [
  ('import {BAZEL_WITNESS_RUNNER,BAZEL_WITNESS_STARLARK} from "./🚦️test-classes/🧾️witness/🟦️.ts";',
   'import {bazelWitnessFilesOfPlanV1,bazelWitnessFilesV1,bazelWitnessPlanTextV1,readBazelWitnessPlanV1} from "./🚦️test-classes/🧾️witness/🟦️.ts";'),
  ('captureBazelSourceSnapshotV1,readBazelSourceSnapshotPlanV1,renderBazelSourceSnapshotV1} from "./📸️snapshot/🟦️.ts";',
   'captureBazelSourceSnapshotV1,readBazelSourceSnapshotPlanV1,readBazelSourceSnapshotV1,renderBazelSourceSnapshotV1} from "./📸️snapshot/🟦️.ts";'),
  ('...(input.actions.some(action=>action.test?.witness)?["repo_inputs_witness"]:[]),', ''),
  (' for(const source of [...new Set(input.actions.filter(action=>action.test?.witness&&action.sources).map(action=>action.sources!))].sort(order))lines.push("repo_inputs_witness(",`    name = ${quote("w_"+source)},`,`    srcs = ${list([":"+source])},`,`    runner = ${quote("witness.mjs")},`,`    runtime = ${quote(input.bun)},`,")","");\n', ''),
  ('["witnesses",list([":w_"+action.sources])]', '["witnesses",list([":w_"+action.sources+bazelWitnessFilesV1.suffix])]'),
  ('"witnesses": attr.label_list(),', '"witnesses": attr.label_list(allow_files = True),'),
  (' await writeChanged(join(workspace,"witness.mjs"),BAZEL_WITNESS_RUNNER);\n', ''),
  (' const aliases=bazelSourceSnapshotDirectoriesV1(classified.files,directory=>bazelSourceAliasV1(directory));',
   ' await writeChanged(join(workspace,bazelWitnessFilesV1.plan),bazelWitnessPlanTextV1(new Map([...sources].filter(([id])=>witnessed.has(id)).map(([id,source])=>{const{marks,extra}=index.selectAll(inputs.get(id)!);return["w_"+source,[...index.files.filter((path,position)=>marks[position]&&declarable.has(path)),...extra.filter(path=>declarable.has(path))]] as [string,string[]];}))));\n const aliases=bazelSourceSnapshotDirectoriesV1(classified.files,directory=>bazelSourceAliasV1(directory));'),
  ('STARLARK_RULES+BAZEL_WITNESS_STARLARK+"\\n"+BAZEL_CARGO_RULE_V1', 'STARLARK_RULES+"\\n"+BAZEL_CARGO_RULE_V1'),
  (' if(snapshot.vanished.length){await removeBazelPlanRecordV1(workspace);throw concurrentEditV1("vanished",snapshot.vanished[0]!,undefined,"snapshot");}\n',
   ' if(snapshot.vanished.length){await removeBazelPlanRecordV1(workspace);throw concurrentEditV1("vanished",snapshot.vanished[0]!,undefined,"snapshot");}\n const witnessPlan=await readBazelWitnessPlanV1(workspace);\n if(witnessPlan)await profilePhaseV1("bazel.witness-refresh",async()=>{for(const[name,text]of bazelWitnessFilesOfPlanV1(witnessPlan,await readBazelSourceSnapshotV1(workspace)))await writeChanged(join(workspace,name),text);});\n'),
  ('/** 🔄️ Publishes what a plan leaves to its publisher, after a fresh plan and after a reused one: captures the planned sources into the workspace (an unchanged file keeps its copy) and writes',
   '/** 🔄️ Publishes what a plan leaves to its publisher, after a fresh plan and after a reused one: captures the planned sources into the workspace (an unchanged file keeps its copy), writes every input witness of the witness plan from the capture manifest (rewritten only when its digest moved) and writes'),
 ],
 "♻️plan-reuse/🧬️schema/🔣️.json": [('"excludedSuffixes": [".tmp"]', '"excludedSuffixes": [".tmp", ".witness"]')],
}
bad = 0
for rel, edits in EDITS.items():
    path = os.path.join(B, rel); text = open(path, encoding="utf8").read(); new = text
    for old, rep in edits:
        if rep and new.count(rep) == 1: pass
        elif new.count(old) == 1: new = new.replace(old, rep)
        elif not rep and old not in new: pass
        else: print("ANCHOR", rel, repr(old[:80]), new.count(old)); bad += 1
    print(("applied" if new == text and verb == "check" else "edit") if new != text else "same", rel)
    if verb == "apply" and new != text and not bad:
        mode = os.stat(path).st_mode & 0o777; tmp = path + ".e30"; open(tmp, "w", encoding="utf8").write(new); os.chmod(tmp, mode); os.replace(tmp, path)
sys.exit(1 if bad else 0)
