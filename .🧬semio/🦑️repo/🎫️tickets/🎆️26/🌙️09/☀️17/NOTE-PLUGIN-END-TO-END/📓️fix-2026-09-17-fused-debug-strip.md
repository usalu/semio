# 🩹️ Fused-statement damage from the `[DEBUG]` console strip (2026-09-17)

## Symptom
Note react serve (6080) answered 500 for the shell bundle: esbuild `Cannot use a declaration in a single-statement context`
at three `🏛️ShellHost/🟦️.tsx` sites (`if (!extensionEntry)   const completion = …`). Every React playground was affected.

## Cause
`26/09/17/DEMONSTRATOR-REMOVE-DEBUG-CONSOLE/🐍️strip-debug-console.mjs` (run 10:30–10:57 local over 15 renderer/kernel/actor files).
Its first branch removes an `ExpressionStatement` holding a `[DEBUG]` console call even when that statement is the bare body of an
`if`/`for` — the `if`-aware branch below it is unreachable. The removal also keeps the statement's own indentation, so the head fuses
with the next line: `if (cond) console.warn("[DEBUG] …");\n  next;` → `if (cond)   next;`. Only three sites were syntax errors;
the rest silently changed behaviour, e.g. `if (!("ok" in outcome))   return response;`,
`if (action.action === "undo" || action.action === "redo")   return refuse("owner-mismatch", …)` (every other action fell through),
`if (reason === "dispatch-failed")   outcome = refuse(reason, detail)`, `for (const error of this.errors)   this.phase = "done"` (kernel),
`if (leftoverFriendly.length)   drive.report("command")` (plugin-bridge), `if (outcome.stopped === "budget")   if (outcome.stopped === "idle" …)` (PluginRuntime).

## Repair
- `🐍️strip-debug-console-fixed.mjs`: removes the OUTERMOST debug-only statement (call / `if`-`else` / loop / block) with its line
  indentation; `{}` in a single-statement slot; `undefined` in expression positions.
- `🐍️repair-fused-debug-strip.mjs <files…>`: per file `git merge-file --ours current buggy(HEAD) fixed(HEAD)` — applies exactly the
  buggy→fixed delta onto the working tree, so later hand-edits (plugin-bridge/PluginRuntime/ShellHost cleanups, the kernel
  `createBundledPluginSource` + ShellHost PROD plugin source) survive. `SEMIO_REPAIR_DRY=1` merges into copies.
- `🐍️parse-check.mjs <files…>`: TypeScript parse diagnostics + fused-head signature scan.

Result: all 15 files `parse=0 fused=0`; the remaining ShellHost `git diff -w` vs HEAD holds only intended edits
(bundled plugin source, `[DEBUG]` removals, `.catch(() => undefined)` rewrites).
