import glob,sys
p=sys.argv[1] if len(sys.argv)>1 else glob.glob("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts")[0]
s=open(p).read()
def once(a,b):
    global s
    assert s.count(a)==1,(a[:70],s.count(a))
    s=s.replace(a,b)
once('import {buildOperationArgumentsV1} from "../⚙️operations/🔤️options/🟦️.ts";','import {buildOperationArgumentsV1} from "../⚙️operations/🔤️options/🟦️.ts";\nimport {extractOffloadFlagV1} from "../🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";')
once('export function resolveBuildModeV1(selection: Selection, environment: NodeJS.ProcessEnv, declarations:readonly SelectedBuildTargetModeV1[]=[]): Selection {\n','export function resolveBuildModeV1(selection: Selection, environment: NodeJS.ProcessEnv, declarations:readonly SelectedBuildTargetModeV1[]=[]): Selection {\n  if (["run", "run-many", "affected", "cargo"].includes(selection.args[0] ?? "")) { const flagged = extractOffloadFlagV1(selection.args); if (flagged.mode !== undefined) selection = { ...selection, args: [...flagged.args], env: { ...selection.env, SEMIO_OFFLOAD: flagged.mode } }; }\n')
open(p,"w").write(s)
print("ok")
