"""🪪 [td-graph] batch `envelope` (after td-leases `envelope`): the envelope-only files of `build-workspace-graph-laws` declare the birth their suite row grants (owner, budget 300000 ms, no authored capabilities: the row declares none), so a bare `bun test ./file` runs under the same capabilities and the suite runner holds them to it. usage: python3 -I envelope.py <root>"""
import os, sys
root = sys.argv[1]
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/"
DECLARE = 'declareBuildTestEnvelopeV1({owner:"test:build-workspace-graph-laws",maximumElapsedMilliseconds:300000,capabilities:{}});\n'
EDITS = {
 "🔄️candidate/🟦️.ts": [('import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";\n', 'import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";\n')],
 "🔄️settlement/🟦️.ts": [('import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";\n', 'import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";\n')],
 "🧭️portable-cli/🟦️.ts": [('import {registerBuildTestLawV1} from "@semio-tech/process/testing/invocation";\n', 'import {declareBuildTestEnvelopeV1,registerBuildTestLawV1} from "@semio-tech/process/testing/invocation";\n')],
}
for rel, pairs in EDITS.items():
    path = os.path.join(root, G + rel); text = open(path, encoding="utf8").read()
    if "declareBuildTestEnvelopeV1(" in text: sys.exit("already declared: " + rel)
    for old, new in pairs:
        if text.count(old) != 1: sys.exit(f"anchor x{text.count(old)} in {rel}")
        text = text.replace(old, new)
    lines = text.split("\n"); last = max(i for i, l in enumerate(lines) if l.startswith("import"))
    lines.insert(last + 1, DECLARE.rstrip("\n"))
    open(path, "w", encoding="utf8").write("\n".join(lines)); print("edited", rel)
