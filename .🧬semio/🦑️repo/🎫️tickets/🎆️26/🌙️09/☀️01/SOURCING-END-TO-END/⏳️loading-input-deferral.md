# Inputs that arrive during a document load

## Cause (console `🗑️generated/probe/console-en-US.txt` lines 18-27)
1. The shell dispatches `noteShellCommand` (chrome journal note: dock, panel, window moves) through the one input funnel
   `ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts` -> `dispatchDirectBrowserActorAction`. Nothing in the funnel knew that a
   whole-document load was running.
2. Guest: `EnvelopeJobs::document_loading_refusal` (`🔌️plugin/🧵️instance/📄️docs/📥️staging/🧷️command-routes/🦀️.rs`) refuses every
   verb except view/interaction/extension-admitted ones with `Fault document.loading`, returned as `Err(OriginalTurnFailure)` from
   `dispatch_action_received` / `dispatch_typed_command_inner`. That is a failure of the whole reactor turn, not a typed outcome.
   The WIT `actor-call-failure` carries only `semantic` (no payload, no wait variant) and the cause stays in guest custody.
3. Bridge (`🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`): `poll` catch calls
   `originalSource.holdNativeRefusal(failure)`; every later `exclusive(...)` call (job `stepJob` of the load, typed-operation
   completion) rethrows that held refusal via `nativeRefusalError`. So the load's own job steps throw the noteShellCommand
   refusal and `applyLoadDocumentEffect` reports `shell.documentTransfer.load-failed`. The attribution is not a routing mistake:
   the source really is poisoned by the earlier turn failure, which is why the same text also appears under
   `typed-operation completion effects failed`.

## Fix (host, event-driven; no compatibility layer)
- New unit `ShellHost/⏳️document-load-gate/🟦️.ts`: `createDocumentLoadGateV1` counts running loads per `(pluginId, instanceId)`,
  parks deferred inputs behind them (`settled`, FIFO, bounded by `DOCUMENT_LOAD_DEFER_CAPACITY_V1 = 64`, cancellable through the
  staged `AbortSignal`), wakes them when the last overlapping load releases. `documentLoadWaitPolicyV1` decides per input:
  `noteShellCommand` -> `drop` (ephemeral chrome journal row, the replaced document owns nothing for it, ledger outcome `applied`);
  declared kind `mutation` / `clipboard` -> `defer` (replayed after the load; `queue-full` ledger refusal past capacity);
  everything else (view, history, interaction, shell, undeclared) -> `pass`, so `historyEditCancelReplay` still reaches the guest
  and can cancel the load.
- `loadDocumentArchive` (`🔒️identity-document-ports/📎️hub-port-and-document-backbone/🟦️.ts`), the single entry of every archive
  load, begins the gate before its first await and releases it in `finally`.
- Funnel `execute` consults the gate before owner checks, so a replayed input re-validates its effect owner and dialog origin.
- `shell-core-state` owns `documentLoadGateRef` next to `sessionWorkRef`; `NOTE_SHELL_COMMAND_ACTION_ID` is now exported.
- The UI never blocks: deferred inputs only park their own promise; the load, progress and cancel stay on their own path.

## Not changed on purpose (guest side)
A typed wait outcome needs a WIT change: `actor-call-failure` would gain a `waits` case or the refusal would become an
`AppFrame::Error` frame (the host already treats `publication.kind === "error"` as a non-fatal `BrowserActorGuestRefusalV1`).
Converting inside `plugin_dispatch_intent` / `advance_manifest_step` has to retire the retained `OriginalTurnFailure` under the
turn grant (RetireOwned custody) and cannot be proven without a native + wasm build, so no Rust is edited here and no Rust test
added. With the host gate the guest refusal is only reachable for verbs the host policy passes and the guest still waits
(history verbs outside the cancel path); the clean follow-up is: guest returns the `document.loading` Fault as an error frame
for command/intent dispatch, host maps `action-guest-refused: document.loading` to a retryable `not-applied` and replays.

## Tests
- `🧪️tests/⏳️document-load-gate/🟦️.ts` + language-neutral `🔣️.json` (policy rows, capacity): 15 passing with
  `bun test ./<path>`: policy rows, idle gate, FIFO replay after overlapping loads, instance isolation, capacity, cancellation,
  re-arming. No third-party oracle exists for this host policy.

## Rebuild
TS only: wasm rebuild NOT needed. The shell bundle must be rebuilt/served again (restart of the server on :6081 picks up the
changed ShellHost sources); no stager run is required for the plugin wasm.
