#!/usr/bin/env python3
"""Switches both runtime-input call sites of the Bazel script to the keyed command runner: `edit_script.py <work-root>`. Idempotent."""
import os, sys

F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
root = os.path.abspath(sys.argv[1])
path = os.path.join(root, F)
text = open(path, encoding="utf8").read()
pairs = [
    ('import {readWorldWitnessSwitchV1,renderWorldWitnessTextV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1,worldWitnessV1,type WorldWitnessRecorderV1} from "../../🔭️witness/🟨️.mjs";\n',
     'import {readWorldWitnessSwitchV1,renderWorldWitnessTextV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1,worldWitnessV1,type WorldWitnessRecorderV1} from "../../🔭️witness/🟨️.mjs";\nimport {runBazelRuntimeCommandV1} from "./🌿️environment/🟦️.ts";\n'),
    ('const run=()=>captureOwnedCommandV1(process.execPath,["exec",runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original}),witness=worldWitnessV1();',
     'const run=()=>runBazelRuntimeCommandV1(root,runtime,original,phaseControl),witness=worldWitnessV1();'),
    ('return hash(JSON.stringify(await captureOwnedCommandV1(process.execPath,["exec",runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original})))===expected;',
     'return hash(JSON.stringify(await runBazelRuntimeCommandV1(root,runtime,original,phaseControl)))===expected;'),
]
for old, new in pairs:
    if new in text:
        continue
    if text.count(old) != 1:
        sys.exit(f"anchor not unique/missing: {old[:90]!r} ({text.count(old)})")
    text = text.replace(old, new)
open(path, "w", encoding="utf8").write(text)
