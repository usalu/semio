#!/usr/bin/env python3
"""Merged copy of the cli-latency set for the round-6 tree (after revision-fix and the live implementation pin):
 - apply-profile-hooks: `door` and `invocation-capacity` anchors carry the live `withImplementationPinV1` wrapper;
 - staged/apply.py: revision-stamp is cut in two edits on the post-revision-fix text (`current()` first asks the pin, then the settled stamp, then the unchanged() check);
   door-validate keeps the child-door edit and its test, but drops the 8 script edits and the light test (live `runBuildCliV1` already classifies the command after the global options)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/cli-latency-tools", "cli-latency")
H = T + "/apply-profile-hooks.py"
CAP = '"./🌡️host-capacity/🟦️.ts"'
old_door = '("door","  const{withBuildCapacityV1}=await import(\\"./🌡️host-capacity/🟦️.ts\\");\\n  await withBuildCapacityV1(args.slice(prefix),environment,granted=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment));",'
new_door = '("door","  const{withBuildCapacityV1}=await import(\\"./🌡️host-capacity/🟦️.ts\\");\\n  const{withImplementationPinV1}=await import(\\"./📥️inputs/🔏️revision/🟨️.mjs\\");\\n  await withBuildCapacityV1(args.slice(prefix),environment,granted=>withImplementationPinV1(()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment)));",'
patch(H, old_door, new_door)
patch(H, '"  const{withBuildCapacityV1}=await profilePhaseV1(\\"door.load\\",()=>import(\\"./🌡️host-capacity/🟦️.ts\\"));\\n  await profilePhaseV1(\\"door.capacity\\",()=>withBuildCapacityV1(args.slice(prefix),environment,granted=>profilePhaseV1(\\"door.admitted\\",()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment))));"',
      '"  const{withBuildCapacityV1}=await profilePhaseV1(\\"door.load\\",()=>import(\\"./🌡️host-capacity/🟦️.ts\\"));\\n  const{withImplementationPinV1}=await import(\\"./📥️inputs/🔏️revision/🟨️.mjs\\");\\n  await profilePhaseV1(\\"door.capacity\\",()=>withBuildCapacityV1(args.slice(prefix),environment,granted=>profilePhaseV1(\\"door.admitted\\",()=>withImplementationPinV1(()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment)))));"')
patch(H, '("invocation-capacity"," const{withBuildCapacityV1}=await import(\\"./🌡️host-capacity/🟦️.ts\\");\\n await withBuildCapacityV1(args,process.env,granted=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl),{control:phaseControl});",',
      '("invocation-capacity",",{withBuildCapacityV1}=await import(\\"./🌡️host-capacity/🟦️.ts\\");\\n const{withImplementationPinV1}=await import(\\"./📥️inputs/🔏️revision/🟨️.mjs\\");\\n await withBuildCapacityV1(args,process.env,granted=>withImplementationPinV1(()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl)),{control:phaseControl});",')
patch(H, '" const{withBuildCapacityV1}=await profilePhaseV1(\\"invocation.gate-load\\",()=>import(\\"./🌡️host-capacity/🟦️.ts\\"));\\n await profilePhaseV1(\\"invocation.gate\\",()=>withBuildCapacityV1(args,process.env,granted=>profilePhaseV1(\\"invocation.admitted\\",()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl)),{control:phaseControl}));"',
      '",{withBuildCapacityV1}=await profilePhaseV1(\\"invocation.gate-load\\",()=>import(\\"./🌡️host-capacity/🟦️.ts\\"));\\n const{withImplementationPinV1}=await import(\\"./📥️inputs/🔏️revision/🟨️.mjs\\");\\n await profilePhaseV1(\\"invocation.gate\\",()=>withBuildCapacityV1(args,process.env,granted=>profilePhaseV1(\\"invocation.admitted\\",()=>withImplementationPinV1(()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl))),{control:phaseControl}));"')

_main = open(H, encoding="utf8").read()
assert _main.count("runBuildCliV1(findWorkspaceRoot") == 2
open(H, "w", encoding="utf8").write(_main.replace("runBuildCliV1(findWorkspaceRoot", "runBuildMainV1(findWorkspaceRoot"))
text = open(H, encoding="utf8").read()
bare = 'SEMIO_BUILD_REUSE_GRAPH===\\"true\\")'
if text.count(bare) == 2 and "selected.graphPolicy" not in text:
    open(H, "w", encoding="utf8").write(text.replace(bare, 'SEMIO_BUILD_REUSE_GRAPH===\\"true\\",undefined,selected.graphPolicy)'))
_h = open(H, encoding="utf8").read()
_h = _h.replace('import { join, resolve, dirname } from \\"node:path\\";', 'import { join, resolve, dirname,relative } from \\"node:path\\";')
open(H, "w", encoding="utf8").write(_h)
n = drop_named_tuples(H, {"invocation-capacity", "body"})
print("cli-latency hooks: dropped", n, "edits of the pre-kernel invocation path (re-profile the runtime kernel: owner)")
A = T + "/staged/apply.py"
patch(A, ":undefined;\\nfunction current(){if(',\n 'const ownSourceURL=", ":undefined;',\n 'const ownSourceURL=")
patch(A, " 'function current(){if(ownUnchanged())return;if('),\n", " ''),\n(\"revision-stamp\",REVISION+\"/🟨️.mjs\",'function current(){if(implementationPinnedV1())return;','function current(){if(implementationPinnedV1())return;if(ownUnchanged())return;'),\n")
n = drop_rows(A, "door-validate", ["SCRIPT", 'B+"/🧪️tests/🚪️light/*'])
assert n == 10, n
text = open(A, encoding="utf8").read()
k = text.index("# ---- door-validate (light commands)")
text = text[:k] + "# (door-validate light commands: live runBuildCliV1 already classifies the command after the global options; the script edits and the light test are not part of the merged copy)\n" + text[k:].split("\n", 1)[1]
open(A, "w", encoding="utf8").write(text)
print("cli-latency merged copy:", T)
