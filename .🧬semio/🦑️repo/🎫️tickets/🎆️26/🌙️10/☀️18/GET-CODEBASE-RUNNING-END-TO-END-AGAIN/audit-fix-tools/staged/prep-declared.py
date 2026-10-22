#!/usr/bin/env python3
"""Anchored patch of withNativePreparation (native-manifests): the preparation gate reads the declared execution only. Native targets prepare by their declared `preparation` (default: their scope), orchestration targets prepare as non-test, source and undeclared targets never. Dry run by default; --overlay <dir> / --write."""
import os, sys
REPO = os.environ.get("AUDIT_FIX_REPO", "/Users/ueli/Documents/semio")
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs"
t = open(os.path.join(REPO, REL), encoding="utf8").read()
old1 = 'if (execution === "source" || execution === undefined && !/^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;'
new1 = 'const declared = target.metadata?.semio?.execution;\n    if (execution !== "native" && execution !== "orchestration" || declared.preparation === "none" || generatorTargets.has(`${project.name}:${name}`)) continue;'
old2 = 'const tests = execution === "native" ? target.metadata.semio.execution.scope === "test" : execution === "orchestration" ? false : /^(?:test|bench)(?:-|$)/.test(name);'
new2 = 'const tests = execution === "native" ? (declared.preparation ?? declared.scope) === "test" : false;'
for o in (old1, old2):
    if t.count(o) != 1: sys.exit("refused: anchor drift: " + o[:80])
t = t.replace(old1, new1).replace(old2, new2)
if "--write" in sys.argv: open(os.path.join(REPO, REL), "w", encoding="utf8").write(t); print("write ok")
elif "--overlay" in sys.argv:
    d = os.path.join(sys.argv[sys.argv.index("--overlay") + 1], REL); os.makedirs(os.path.dirname(d), exist_ok=True); open(d, "w", encoding="utf8").write(t); print("overlay ok")
else: print("dry-run ok")
