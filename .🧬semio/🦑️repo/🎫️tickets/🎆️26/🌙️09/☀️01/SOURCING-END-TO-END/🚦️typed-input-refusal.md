# A refused input is a typed per-input outcome, not a turn failure

Follow-up of `⏳️loading-input-deferral.md` (host gate done). This is the guest side.

## Root cause (confirmed in code)
- `EnvelopeJobs::document_loading_refusal` returned `Fault document.loading`; `dispatch_action_received` and
  `dispatch_typed_command_inner` returned it as `Err(OriginalTurnFailure)`, which propagated out of
  `advance_manifest_step` and failed the whole reactor turn (`actor-call-failure::semantic`, no payload, cause kept in guest custody).
- The bridge `poll` catch (`component-bridge-source`) then holds the native refusal. `OriginalOperationSource.holdNativeRefusal`
  is a one-way latch by design (`🧪️tests/🧬️component-codec-reply` "preserves a held native poll refusal across the next browser
  action"): the failed turn's cause is still in guest custody, so the source is unusable until it is returned. Every later
  `exclusive(...)` call rethrows it, including the load's own job steps.
- Audit result for the bridge: the split is already typed where the guest speaks typed. Job calls separate a typed refusal inside the
  reply (`original-job.call-refused`, per call, nothing held) from a thrown failure (held). `poll` separates a returned result
  (frames, including `AppFrame::Error`) from a thrown failure (held). So the latch is correct for genuine turn failures and the
  defect was only that a domain refusal of ONE input was classed as a thrown failure. The bridge needs no change.

## Fix (guest, schema-neutral: reuses `AppFrame::Error`, which the host already maps to `BrowserActorGuestRefusalV1`)
The refusal is answered at the input boundary, before any retained source is staged, so there is nothing to unwind and no
`OriginalTurnFailure` is ever created for it.
1. `PluginApp::input_refusal(&self, verb) -> Option<Fault>` (default `None`; `reactor/🔌️app/🦀️.rs`). `VcsArtifactApp` answers it from
   `ingest.document_loading_refusal` with the extension's `admitted_while_loading` (`instance/🔌️plugin-app-impl/🦀️.rs`).
2. `document_loading_refusal` now builds the fault `retryable` (a wait, not a rejection) (`instance/📄️docs/.../🧷️command-routes/🦀️.rs`).
3. Manifest receiving (`runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`): new typed outcome
   `ManifestOutcome = Result<InvocationResult, Fault>`; `begin_dispatch()` is true exactly once per input (first dispatch pass, the only
   one that may still refuse); `finish_refusal(fault)` settles the input. The three dispatch branches (artifact command, action
   invocation, app or mode command) ask `input_refusal` on that first pass. The source then retires through the existing paid
   close ladder (`close_step`), `take_result` hands back `Err(fault)` once the source is empty, `terminal_is_empty` includes the
   refusal, and a cancelled turn retires the held refusal through the existing `fault_close` `ControlledRetirement` custody
   (RetireOwned under the turn grant).
4. `OriginalCommandExchangeState.manifest` carries `ManifestOutcome` (`runtime/🧰️ops/📥️ingress/🦀️.rs`); `plugin_exchange` answers
   `Err(fault)` with `push_app_fault(frames, Some(seq), fault)` (`AppFrame::Error` in reply to the input's seq,
   `runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs`). No `command_terminal_fault` is set on purpose: the browser actor treats ingress status
   `fault` as `action-command-ingress-refused`, which triggers actor recovery; the ordinary `command-complete` plus an error frame
   becomes `BrowserActorGuestRefusalV1` and keeps the actor open (`🏪️store/👷️worker/.../🎯️action-dispatch`).
5. Turn-fatal stays for real invariants only: the in-instance checks (`dispatch_action_received`, `dispatch_typed_command_inner`) remain
   as the backstop for an input that was already in flight when a load began (a race the input boundary cannot see).

## Residuals (named, not hidden)
- UI intents (`plugin_dispatch_intent` -> `handle_intent_frame`) resolve their verb only after `command_from_intent`, with the typed
  command source already admitted, so a mutating intent racing a load still fails the turn. View and interaction intents are
  exempt from the refusal. Making that typed needs the typed command source to retire on refusal (own ticket).
- Mid-flight race (load starts after the first dispatch pass): backstop above, still turn-fatal.
- The host replay of a `document.loading` refusal is the host gate's job (`⏳️loading-input-deferral.md`); with the gate the guest refusal
  is only reachable for verbs the host passes.

## Tests
- Rust (language-agnostic law data in `🧪️tests/🧫️fixtures/🔣️.json` key `refusal`): `original_manifest_refusal_settles_as_a_typed_outcome_and_retires_the_source`
  in `runtime/🎞️command/📥️custody/🛂️manifest/🧪️tests/🦀️.rs` (admit, refuse, `begin_dispatch` once, source retires, `take_result`
  returns `Err` with code, message and `retryable`, source terminal). Chained into the feature build next to the other manifest laws.
- TS (same fixture): `🌐️browser-bundle/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts` "an input the guest refuses as a typed outcome holds
  nothing ...": through the generated component bridge a typed refusal leaves poll, startJob and stepJob working with zero held
  refusals, while a thrown turn failure holds one and the load's job step then rejects with `original-turn.native-return-refused: semantic`.
  `bun test ./🟦️.ts` in that folder: 6 pass.
- No third-party oracle exists for this policy (domain rule); the WIT oracle (JCO types) already checks the bridge calls.

## Rebuild
Guest Rust changed (runtime-ops, runtime-exchange, instance-app, reactor-app crates): the sourcing plugin wasm MUST be rebuilt and
re-staged. The bridge and shell TS are unchanged (no bridge regeneration beyond the normal stage).

## Verification status
- Native `cargo check --keep-going -p semio-plugin-runtime-exchange -p semio-plugin-instance-app` (own target dir `🗑️generated/native-tir`, script `typed-refusal-cargo.sh`): clean (`🗑️generated/tir-check4.txt`).
- Native `cargo test -p semio-plugin-runtime-ops --lib original_manifest_refusal`: 1 passed (`🗑️generated/tir-test3.txt`).
- `original_manifest_close_batches_paid_steps` (same module, untouched by this change) fails at its own `assert!(ready)` before reaching any new code; it was failing before (pre-existing).
- `bun test ./🟦️.ts` in `🧪️tests/🧬️wit-lowering`: 6 pass.
- wasm32-wasip2 `cargo check -p semio-hub-sourcing-component` (`typed-refusal-wasm-cargo.sh`, own target dir `🗑️generated/native-tir-wasm`, log `🗑️generated/tir-wasm-check.txt`): started, was at crate 384 of the dependency graph (`semio-plugin-registration`) when this note was written, no error so far; the plugin crates touched here were not yet reached. Read the log tail for `Finished`.
- Source-size gate: `instance-app` was at 260252 of 262144 bytes at the last census (`1029-result-resume.md`); this change adds about 190 bytes there.
