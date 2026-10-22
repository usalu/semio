#!/usr/bin/env python3
"""Adds one recording line to the OVERLAY copy of project-assembly (never the live file): every classified target pushes [project, target, nativePolicyTarget, nativeTarget, declaredExecution, orchestrator, hasCommand, authored, nativeProject, executor, runsTests, nativePackage, generatorTarget, declaredExecution]; classify-only mode skips every later per-target step and the named-input assembly (the probe compares classification, not inputs)."""
import os, sys
OVERLAY = os.environ.get("AUDIT_FIX_OVERLAY", "/private/tmp/audit-fix/overlay")
P = OVERLAY + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
NEW = "--new" in sys.argv
RUNS = "nativeTarget ? nativeTargetRunsTestsV1(target.metadata?.semio?.execution) : null" if NEW else "nativeTarget ? nativeTargetRunsTestsV1(name, target.metadata?.semio?.execution, await commandSourcesOf(policy)) : null"
if not os.path.isfile(P) or os.path.islink(P): sys.exit("refused: overlay copy missing or a symlink")
t = open(P, encoding="utf8").read()
a = "    const artifactTarget = artifactTypeScript &&"
b = "    (globalThis.__execClass ??= []).push([json.name, name, Boolean(nativePolicyTarget), Boolean(nativeTarget), execution ?? null, Boolean(orchestrator), typeof policy.options?.command === \"string\" || Array.isArray(policy.options?.commands), Object.hasOwn(json.targets ?? {}, name), nativeProject, policy.executor ?? null, " + RUNS + ", nativeProject && Boolean(readToml(join(workspaceRoot, nativeRoot, \"Cargo.toml\")).package), Object.values(contracts).some((contract) => [contract.target, contract.previewTarget, contract.checkTarget].includes(`${json.name}:${name}`)), target.metadata?.semio?.execution ?? null]);\n    if (globalThis.__classifyOnly) continue;\n" + a
if "__execClass" in t: sys.exit("already instrumented")
if t.count(a) != 1: sys.exit("refused: anchor drift")
t = t.replace(a, b)
early_a = "  const namedInputs = await projectInputs({ ...json, targets: normalized }"
if t.count(early_a) != 1: sys.exit("refused: early-return anchor drift")
t = t.replace(early_a, "  if (globalThis.__classifyOnly) return { ...json, root, namedInputs: {}, targets: normalized };\n" + early_a)
open(P, "w", encoding="utf8").write(t)
print("instrumented")
