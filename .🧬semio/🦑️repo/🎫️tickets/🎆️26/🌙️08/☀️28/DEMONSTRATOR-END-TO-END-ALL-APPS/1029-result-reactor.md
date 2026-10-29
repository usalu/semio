# 1029 result: reactor

STATUS: GREEN for both REVIEW items at crate level (wasm32-wasip2: runtime-ops, reactor-turn, reactor-wit-bridge, reactor-context compile; check `C/🗑️generated/1029/reactor-chk1.{txt,rc}`). Energy hub component and the full closure were NOT reached: the check stops at two foreign crates (see Frontier). One OUT-OF-SCOPE BLOCKER found for the browser panes (see Finding A3).

## B. Energy intent dispatch (resumability) - RESOLVED by deletion of dead code
- The resumable intent contract is `OriginalIntentActionSource` (`plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/intent.rs`): it retains the converted args (`arguments`), the fault (`failure`) and the selected-surface metadata across polls; `plugin_dispatch_intent` (singular, `🦀️.rs` ~l.464) feeds exactly those retained slots into `PluginApp::handle_intent_frame(intent, args, original_fault, meta, identity, cx)` and returns `Ok(None)` to be re-polled (`Ok(None)` = not finished).
- The ONLY production caller is `plugin/⚛️reactor/🔄️turn/📥️receiving/🦀️.rs:119` (`RetainedIntents::step` -> `plugin_dispatch_intent`, with `intent_source` kept in the `INTENTS` thread-local across polls). `plugin_dispatch_intents` (plural, the batch fn energy adapted with LOCAL `args/original_fault/meta`) had NO caller (rg over the whole repo: only doc comments and one fixture list entry). Its local-state adaptation was therefore dead code, but it was a trap: calling it would have lost the retained slots on every `Ok(None)`.
- `handle_intent_frame` implementer (`plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs:502`) documents the same contract ("retained intent source donates its original converted arguments and selected metadata").
- FIX (clean, no shim): deleted `plugin_dispatch_intents` from `typed-operations/🦀️.rs`, dropped its unused `instance_actor` import, rewrote the doc of `plugin_dispatch_intent` (what the batch doc claimed now describes the retained-source design). Doc/name references retargeted to the singular: `reactor/🔄️turn/📨️event-dispatch/🦀️.rs` (2 doc lines), `reactor/🏃️runtime/🦀️.rs:24`, `reactor/🔌️app/🦀️.rs:269`, `plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/📡️host-events-intents-and-streamed-tick/🦀️.rs:71`, fixture list `🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧪️tests/🧫️fixtures/📥️receiving/🔣️.json` (`plugin_dispatch_intents` -> `plugin_dispatch_intent`; the file has no loader, it is a descriptive list).
- Evidence: chk1 compiles `semio-framework-plugin-runtime-ops` (53 warnings; the 4 unused-import warnings in typed-operations `🦀️.rs` l.9/12/12/18 are identical in HEAD, not caused by this edit).

## A. Single event per poll
Kernel contract: `poll_kernel(events: &mut Option<Event>)` (`reactor/🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs:72`) and `select_turn_event` (`reactor/🔄️turn/📨️event-dispatch/🦀️.rs:9`) serve ONE slot per turn; one pending output (`outputs_held`/`busy`) and non-mergeable `TurnResult` receipts rule out a guest-side drain loop. Decision: keep the guest refusal (reactor-context `single_kernel_event`, `🎟️context/🌉️call/🎭️exports/🦀️.rs:24`; wit-bridge `poll`, `🧬️wit-bridge/🪢️bridge/🦀️.rs:85-89`) and make every host deliver one event per poll.

Why not change the WIT type: `option<event>` would inline the whole `event` variant (> 16 flat values) and bring back the indirect-parameter-area leak documented at `stage-command-page` in `reactor.wit`; `list<event>` stays, the law is now in the schema.

### A1. Native hosts - verified, no change needed
- `execute_actor_turn` (`plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs:118`) forwards what it is given; callers: shard loop dispatches exactly `std::slice::from_ref(event)` or `&[]` (`plugin/🖥️host/🧵️shard/🎬️shard-loop-frame-turns-and-job-control/🦀️.rs:~92`); MCP headless channel `instances/🦀️.rs:85` and `documents-and-backbone/🦀️.rs:223` use an `owed` queue that holds at most one event (`mem::take` then at most one ack or one wake pushed, `retryable_lifecycle_turn` already requires `events.len() <= 1`, `host/🎭️guest-runtime/🧱️guest-types-and-runtime-trait/🦀️.rs:150`); other callers pass `from_ref`/single-element slices.
- Wasmtime host passes the list through; no multi-event producer exists.

### A2. TS reservation hub - could deliver 4 multi-event batches; FIXED (one event per poll)
Sites (all in `🏪️store/👷️worker/🧷️actor-reservation/`):
1. `🖼️surface-rendering/🟦️.ts` `renderSurface`: `visibleEvents` (one `surface-visible` per window/panel/section) went out as ONE batch on turn 0. Now `owed` queue, one event per turn, then `wake`; render only completes when `!moreWork && owed.length === 0`.
2. same file `refreshDocumentSurfaces`: one batch for all surfaces. Now one poll + cold-status check + `driveTurnResult` per surface event.
3. `🩹️ui-patch-reconciliation/🟦️.ts` `reconcileUiPatches`: `[...acknowledgements, ...completions]` and `[...feedback, ...acknowledgements]`. Now one FIFO `owed` queue (feedback is queued ahead of older acks exactly as before), `owed.splice(0, 1)` per poll; the former 8-iteration cap is kept as an 8-patch-offer cap (`patchCount`) plus the existing `DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT` (256) bound on total turns (drain polls no longer eat the patch cap).
4. `🛤️poll-lanes/🟦️.ts` `invokeWithPages`: host-side enforcement at the single choke point, `events.length > 1` -> `Error("document browser actor: a reactor turn admits at most one event")` before the child call; doc on `invokePoll`.
Other `invokePoll` callers (document-backbone, action-dispatch, linked-read, retirement, activation, cold-pair) already send 0 or 1 event.
NOT verified by running: TS only reasoned/read; no tsc/test run (existing `🧪️tests/🎭️browser-actor-reservation/🟦️.ts` only checks lifecycle events, still consistent).

### A3. OUT-OF-SCOPE BLOCKER for browser panes (needs an owner)
The generated browser component bridge `plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts:~186` (`createActorApi().poll`) REFUSES every non-empty `events` / `commandPage` / `coldPairPage` with `original-turn.canonical-input-held` (held as producer input; asserted by `dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts:79,157`). Nothing in TS calls the guest exports `reserve-poll-input` / `write-poll-input` / `close-poll-input` (only the descriptor fixture names them), so as committed the browser host cannot deliver ANY event to the guest. The shard worker (`🧵️shard-worker-source/📨️message-handler/🟦️.ts:~227`) additionally batches every envelope of a `Grant` frame into one `poll` (Rust ShardLoop runs one turn per envelope). Both need the canonical-input delivery implemented one event per turn; I did not start it (large, unverifiable here). This is likely the next runtime wall for the 8 panes after compile is green.

### A4. Schema
`plugin/🧬️schema/📜️reactor.wit` `poll`: added the LAW doc (at most one event, why `list`, guest refusal text). Doc-only, ABI unchanged. No language-agnostic test added: the guest check lives in a wasm32-p2-only module (`exports`), and the TS enforcement is covered by construction only; follow-up: a fixture-driven test that `poll` with 2 events is refused by both guest and hub lane.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`
- `.../⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs` (docs), `.../⚛️reactor/🏃️runtime/🦀️.rs` (doc), `.../⚛️reactor/🔌️app/🦀️.rs` (doc)
- `.../🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/📡️host-events-intents-and-streamed-tick/🦀️.rs` (doc)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🧪️tests/🧫️fixtures/📥️receiving/🔣️.json`
- `.../🔌️plugin/🧬️schema/📜️reactor.wit`
- `.../🏪️store/👷️worker/🧷️actor-reservation/{🖼️surface-rendering,🩹️ui-patch-reconciliation,🛤️poll-lanes}/🟦️.ts`
SHARED CHANGE: none (no shared Rust API signature changed; `plugin_dispatch_intents` had no users).

## Frontier of chk1 (foreign, not touched)
Check (wasm32-wasip2, energy hub manifest, `-p runtime-ops -p reactor-turn -p reactor-context -p reactor-wit-bridge -p semio-hub-energy-component`) fails only in:
- `semio-framework-plugin-instance-typed-ops` (16): `semio_framework_plugin_reactor_app::original_plugin_turn_grant` does not exist (use `semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant`; sites `🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs:88`, `.../📬️publication/🏃️mounted-run-and-pump/🦀️.rs:80`), `M: ReferenceSpaceMember` / `OriginalStoreSlot: SpaceMember` bounds, `Fault: From<MemberValueError>`, `ValueRefusalKind::Malformed`, `str_as_str` unstable.
- `semio-framework-plugin-time-travel-turns` (15): `TimeTravelSession` missing, private `session_retirement`, non-exhaustive `TimeTravelQueryOutput::CommitCloseDemand`, `semio_framework_async` not a dependency.
Both are the instance/time-travel split family (puzzleplugin). reactor-turn, wit-bridge, reactor-context and runtime-ops compile (checking happened; only warnings).
NEXT: whoever owns instance-typed-ops/time-travel-turns fixes the above, then re-run `repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-energy-component`.
