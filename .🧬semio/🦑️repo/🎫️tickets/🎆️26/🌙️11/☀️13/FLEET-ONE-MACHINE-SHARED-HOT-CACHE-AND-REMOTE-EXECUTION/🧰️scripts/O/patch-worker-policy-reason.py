#!/usr/bin/env python3
"""Package O: a unit that a worker's action policy refuses is declined as `worker-policy`, not as `no-idle-machine`. Usage: patch-worker-policy-reason.py <🌐️remote-execution dir> [--dry]"""
import sys, pathlib
RE = pathlib.Path(sys.argv[1]); DRY = "--dry" in sys.argv
O = RE / "🚀️offload"
edits = {}
def sub(p, old, new):
    t = edits.get(p, p.read_text())
    assert t.count(old) == 1, (p.name, old[:80], t.count(old))
    edits[p] = t.replace(old, new)
sub(O / "📐️policy/🟦️.ts", '|"incomplete-inputs"', '|"worker-policy"|"incomplete-inputs"')
sub(O / "🧬️schema/🔣️.json", '        "incomplete-inputs",\n        "cancelled"', '        "worker-policy",\n        "incomplete-inputs",\n        "cancelled"')
sub(O / "🧬️schema/🔣️.json", '      "reason.incomplete-inputs": "the other machine lacked an input",', '      "reason.worker-policy": "the action policy of a worker refuses its arguments",\n      "reason.incomplete-inputs": "the other machine lacked an input",')
sub(O / "🧬️schema/🔣️.json", '      "reason.incomplete-inputs": "der anderen Maschine fehlte eine Eingabe",', '      "reason.worker-policy": "die Aktionsrichtlinie eines Workers lehnt ihre Argumente ab",\n      "reason.incomplete-inputs": "der anderen Maschine fehlte eine Eingabe",')
C = O / "🔌️client/🟦️.ts"
sub(C, '  const code=response.status?.code??0;\n', '  const code=response.status?.code??0;\n  if(code!==0&&String(response.status?.message??"").includes("Action refused by the worker policy"))return declined("worker-policy",String(response.status?.message).slice(0,200));\n')
M = O / "🟦️.ts"
sub(M, 'outcome.reason==="infrastructure"||outcome.reason==="incomplete-inputs"||outcome.reason==="path-dependent-output"?{fellBack:1}', 'outcome.reason==="infrastructure"||outcome.reason==="incomplete-inputs"||outcome.reason==="path-dependent-output"||outcome.reason==="worker-policy"?{fellBack:1}')
sub(M, 'if(execution.reason==="no-idle-machine"||execution.reason==="queue-timeout")writeOffloadMemoV1', 'if(execution.reason==="no-idle-machine"||execution.reason==="queue-timeout"||execution.reason==="worker-policy")writeOffloadMemoV1')
for p in edits: print("edit", p.relative_to(RE))
if not DRY:
    for p, t in edits.items(): p.write_text(t)
