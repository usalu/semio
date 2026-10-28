"""📝 Records the hub verdict (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
t = t.replace("every listed product has a verdict except the never-run hub (see table)", "every listed product has a verdict (hub: first official run ever, RED)")
t = t.replace("os-mcp consent test, custody queued-wait, tui keystroke law.", "os-mcp consent test, custody queued-wait, tui keystroke law, hub (3 causes).")
t = t.rstrip("\n") + "\n" + ("| rn-hub | os-hub, os-hub-ts :: build,test | 01:34 | 2,928 s | 9,405 MB | 1 | RED (first official run; 1,077 s queued, 903 projects re-inferred): (a) `os-hub-admin:build` — Vite fails to load `🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts`: `ReferenceError: require is not defined` "
       "evaluating `📚️library/🗂️workspaces/📦️payload/🟨️.cjs:3` inside Vite's ESM module runner (a CommonJS file in the config's import closure); (b) 2/2 test targets FAILED: one with `Original resource loan deadline exhausted` (`🧭️routing/📥️invocation/🏃️process/🟦️.ts:258`, capture under an exhausted loan), "
       "one with `Native Cargo owner preparation failed … Owned child input data required` (cargo workspace preparation child refused its input record). Not diagnosed further |\n")
open(p, "w", encoding="utf8").write(t)
