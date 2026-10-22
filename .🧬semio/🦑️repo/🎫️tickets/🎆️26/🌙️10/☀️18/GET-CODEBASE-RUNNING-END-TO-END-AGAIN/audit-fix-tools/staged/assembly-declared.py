#!/usr/bin/env python3
"""Anchored patch of project-assembly: native classification comes only from the declared `metadata.semio.execution` field. Dry run by default (reports anchors); --overlay writes the patched copy into the overlay; --write patches the live file (apply only on GO)."""
import os, sys
REPO = os.environ.get("AUDIT_FIX_REPO", "/Users/ueli/Documents/semio")
OVERLAY = os.environ.get("AUDIT_FIX_OVERLAY", "/private/tmp/audit-fix/overlay")
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
live = os.path.join(REPO, REL)
text = open(live, encoding="utf8").read()
def sub(t, old, new):
    if t.count(old) != 1: sys.exit("refused: anchor drift (%d matches): %s" % (t.count(old), old[:90]))
    return t.replace(old, new)
def cut(t, start, end, replacement=""):
    i = t.find(start)
    if i < 0 or t.count(start) != 1: sys.exit("refused: start anchor drift: " + start[:80])
    j = t.find(end, i)
    if j < 0: sys.exit("refused: end anchor drift: " + end[:80])
    return t[:i] + replacement + t[j + len(end):]
# 1. delete the closure driver table and its probe
text = cut(text, 'const CARGO_TEST_DRIVER = "/🏃️process/🧪️testing/🦀️cargo/";\n', "NATIVE_COMMAND_DRIVERS.some((driver) => path.includes(driver)));\n}\n")
# 2. the test scope is declared, never inferred
text = cut(text, "/** 🧪️ A native target runs the crate's tests when its contract says so", "return name.startsWith(\"test\") || commandSources.some((path) => typeof path === \"string\" && path.includes(CARGO_TEST_DRIVER));\n}\n",
 '/** 🧪️ A native target runs the crate\'s tests exactly when its declared execution contract says so.\n * @param {{ kind: "native", scope: "production" | "test" }} execution */\nexport function nativeTargetRunsTestsV1(execution) {\n  return execution.scope === "test";\n}\n')
# 3. no memo: the closure is needed once per native target, for its inputs only
text = cut(text, "  const commandSourceMemo = new Map();\n", "    return sources;\n  };\n")
# 4. declared classification
text = sub(text, "targetExecutionKindV1(target,`${json.name}:${name}`), orchestrator =", "targetExecutionKindV1(target,`${json.name}:${name}`), orchestrator =")
i = text.index("    const nativePolicyTarget = execution")
j = text.index("\n", text.index("    const nativeTarget = execution", i)) + 1
text = text[:i] + '    if (execution === undefined && (typeof policy.options?.command === "string" || Array.isArray(policy.options?.commands))) throw Error(`Repository target execution contract required: ${json.name}:${name}`);\n    const nativeTarget = execution === "native" && !orchestrator, nativePolicyTarget = nativeTarget;\n' + text[j:]
text = sub(text, "      const commandSources = await commandSourcesOf(policy);\n", "      const commandSources = await nativeTargetCommandInputs(policy, workspaceRoot, commandInputs, scripts, root);\n")
text = sub(text, "nativeTargetRunsTestsV1(name, target.metadata?.semio?.execution, commandSources)", "nativeTargetRunsTestsV1(target.metadata.semio.execution)")
for gone in ["CARGO_TEST_DRIVER", "NATIVE_COMMAND_DRIVERS", "reachesNativeCommandDriverV1", "commandSourcesOf", "commandSourceMemo"]:
    if gone in text: sys.exit("refused: leftover " + gone)
mode = "overlay" if "--overlay" in sys.argv else "write" if "--write" in sys.argv else "dry-run"
if mode == "overlay": open(os.path.join(OVERLAY, REL), "w", encoding="utf8").write(text)
elif mode == "write": open(live, "w", encoding="utf8").write(text)
print(mode, "ok", len(text))
