# Direct `repo ./📜️script.ts <cmd>` and the Original envelope (2026-11-23)

Finding: not a regression, no code changed. By design the launcher never mints the Original envelope.

- `repo ./x.ts` is the Go launcher route (`🚀️launcher/📦️packages/🐹️go/🏗️entry/🐹️.go`, `Execute`): it resolves the runtime and hands over to `bun <script>`; it sets no `SEMIO_SCRIPT_PROCESS_INVOCATION`.
- 656 package `📜️script.ts` files end in the thin entry `await receiveScriptProcessInvocation(process.env, ...)` (process module), which refuses a missing envelope on purpose (README "Script entries", line 680: envelope-only files, Original comes from the owning Repo operation with its capabilities and budget).
- Only scripts that open their own birth work bare: the root `📜️script.ts` and those using `runRepoScriptMain` (`🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts`, receive-or-open with owner `repo-script`, 0 ms budget).
- The same failure occurs for every package router (checked `✒️writer`), not only Draw. `bun`'s bunfig preload (runtime pin) does not apply from a package cwd and must not mint envelopes either.
- Minting one in the launcher or a preload would bypass the capability/budget grants of the registered target, which the design forbids; a per-script switch to `runRepoScriptMain` would open a 0 ms root birth and weaken the guard in 656 files.

Working routes: registered target `repo run @semio-tech/draw-js:publication-authority-audit` (executor `repo:script` issues the Original) or `repo tools exec --light bun -- <launcher>` with `createScriptProcessEnvelope` as in `draw-native-dev.ts`.

Verification attempt: `repo run @semio-tech/draw-js:publication-authority-audit` stalled in "Reading the project graph slice" (door.script, 114 s, 0 of 600 owner checks) and was cancelled by the 115 s cap; with `draw-js` as name the resident service failed "Unknown canonical build owner" (project name is `@semio-tech/draw-js`). Both belong to the graph-service work of others.
