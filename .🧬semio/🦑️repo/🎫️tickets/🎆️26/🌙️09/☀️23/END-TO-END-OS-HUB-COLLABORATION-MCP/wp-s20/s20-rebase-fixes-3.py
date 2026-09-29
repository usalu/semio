"""🧯️ S20 faults overlay, session 15: TS fix-ups found by `tsc` of the os package on the rebased overlay — the duplicated
agent-lane oracle import the rebase union kept (live imports both oracles now), and the PluginRuntime law's record literal
that lacked the record's `class`. Idempotent. Usage: python3 s20-rebase-fixes-3.py"""
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/🧰️framework/🛍️products/💻️os/🔨️modules")
EDITS = [
    ("🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
     'import { agentLaneCarriageOracle, agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";\nimport { agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";\n',
     'import { agentLaneCarriageOracle, agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";\n'),
    ("📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx",
     'code: "test.parked-fault", origin: "app" as const, message, parameters: [] });',
     'code: "test.parked-fault", origin: "app" as const, class: "precondition-failed" as const, message, parameters: [] });'),
]
for rel, old, new in EDITS:
    path = OVERLAY / rel
    text = path.read_text()
    if new in text and old not in text:
        continue
    assert text.count(old) == 1, rel
    path.write_text(text.replace(old, new))
    print("fixed", rel.split("/")[-1], new[:60])
