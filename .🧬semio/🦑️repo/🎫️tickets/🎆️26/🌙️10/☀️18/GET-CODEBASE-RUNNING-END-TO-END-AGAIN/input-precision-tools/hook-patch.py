#!/usr/bin/env python3
# 🪝 Anchored edits that wire the input precision derivation into the inference project-assembly component; idempotent, applied to the CURRENT live text.
import os
HERE = os.path.dirname(os.path.abspath(__file__))
MARK = "preciseInputsV1"

def edits():
    block = open(HERE + "/staged/hook.mjs.txt", encoding="utf8").read()
    return [
        ('import { withHostPlatformOutputsV1 } from "../../../🎯️host-platform/🟦️.ts";\n',
         'import { withHostPlatformOutputsV1 } from "../../../🎯️host-platform/🟦️.ts";\nimport { createRequire } from "node:module";\nimport { observedTypeScriptHost } from "../../../../🏗️build/👁️observation/🟨️.mjs";\nimport { deriveInputsV1, inputPrecisionDecisionV1, readInputPrecisionV1, selectRuleV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";\n'),
        ('const { existsSync, readFileSync } = observedFs;\n', 'const { existsSync, readFileSync, readdirSync } = observedFs;\n'),
        ('export const component = "project-assembly";\n', 'export const component = "project-assembly";\n' + block),
        ('    normalized[name] = loaded.targetBuildModes(name, cachedOutputsAdmittedV1(policy));\n',
         '    if (policy.cache && policy.inputs) policy = await preciseInputsV1(json.name, name, inherited.options?.command ?? inherited.command, policy, workspaceRoot, facts, scripts, internCommandSources);\n    normalized[name] = loaded.targetBuildModes(name, cachedOutputsAdmittedV1(policy));\n'),
    ]

def apply(text):
    if MARK in text: return text, False
    for anchor, replacement in edits():
        if text.count(anchor) != 1: raise SystemExit("anchor count %d: %s" % (text.count(anchor), anchor[:70]))
        text = text.replace(anchor, replacement)
    return text, True
