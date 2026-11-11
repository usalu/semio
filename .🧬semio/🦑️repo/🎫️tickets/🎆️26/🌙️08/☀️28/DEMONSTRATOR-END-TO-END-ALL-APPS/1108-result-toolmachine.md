# Lane toolmachine: tool-machine, composition-emission, effect-bank dev-deps

## Done (wasm32-wasip2 via 🌎️hub/Cargo.toml, W-toolmachine-3, rc0)
semio-framework-job, semio-framework-action-bus, semio-plugin-host-configuration, semio-plugin-tool-machine, semio-plugin-composition-emission all green.
- `admit_original_construction_settled` (and settle-overhead quote use) now defined once in `semio-framework-job` (`🏭️construction`), re-exported by action-bus (`pub use`); tool-machine and host-configuration use it from job. Flowdoc3's `🏗️construction` copy and host-configuration's local `settled` loop are deleted. No Cargo change needed for tool-machine (action-bus dep withdrawn).
- tool-machine follow: `Projection<T>` (RetainedOwnedProjectionAdmission + admit/take ReplyStash) per input kind (`document_stage`, `config_stage`, `window_stage`); `project_leaf` pays admit and take turns via `paid_effect`; quotes stage-aware; close and terminal include the stages.
- gesture issuer: `gesture_issuance_grant`, `reserve_live(available,effect)`, `issue_live(birth,slot,operation,effect)` returning the receipt; render staging `admit_gesture_slot(..,birth,handoff,cx)` runs in `cx.with_original_effect` and `cx.consume_retained`; typed-command call passes `cx` (those two crates not compiled yet, behind decl-surface).
- composition metadata: issuers `advance(source, effect)`, `lease.admit_clone(effect)`.
- dev-dep `semio-framework-value-testing-effect-bank` re-added in dag-host and flow-host-laws Cargo.toml.

## Open
- Native (W-toolmachine-4, Cargo.toml): blocked by reds outside this lane: semio-plugin-tool-run (55, `tick/🛬️native` `RetirementCursor::close_step`), semio-plugin-document-media (29, `MediaRetireOwned::retirement`), semio-plugin-composition-publication (6). Flow-host read/core/session native check therefore not reached.
- Stale tests (grant-shaped): tool-machine gesture issuer tests, composition metadata unit tests, render staging gesture tests (`🧪️tests/🧪️gesture`); dag-host and laws ported tests unverified.
- LOCK NEEDED: dag-host and flow-host-laws Cargo.toml (effect-bank dev-dep).

## Rerun
`chk.sh W-toolmachine-N 🌎️hub/Cargo.toml wasm -p semio-framework-job -p semio-framework-action-bus -p semio-plugin-host-configuration -p semio-plugin-tool-machine -p semio-plugin-composition-emission`


## Round 2 and 3 (tool-run, Arc sites, tool-machine sub-crates)
FIXED-verified: native W-toolmachine-7 semio-plugin-tool-run (rc0); wasm W-toolmachine-9 tool-run + tool-machine (rc0); native W-toolmachine-11 semio-plugin-scrub-host, semio-plugin-typing-edit, semio-plugin-tool-machine (rc0).
- Arc sites: `ToolRunEntry.entities/payload` and `Metadata` are `SharedOriginalStorage`; the ledger apply path is the synchronous `ColdMutation` tier, so they are issued with `from_cold` and always replaced through `crate::retire_cold_shared` (elect_cold, wait unique, take value) in `🔁️refold-finalize-and-entry`, `📒️driver-and-ledger`, action-dispatch. No `admit_from` because there is no effect on that path.
- typing-edit: `OriginalTypingSettlement` owns a resident `birth` admission, `birth_failure` and `birth_close` (closed in close_step/close_demands/terminal_is_empty) for `OriginalTypingEdit::admit`; `SharedUtf8::admit(.., &mut None)`; owners close through `close_original_effect_step`.
- scrub-host: owner closes through `retired_effect` with `step(4096,effect)`.
- tool-run also: native cursor `native_retirement_reply!()` + inherent `close_step(body,effect)`.

## Open
- Flow-host native chain (W-toolmachine-12): blocked by `semio-framework-value-factory` (4 errors, `ReplyStash` not found in the crate root in `arc-custody/🏭️factory/🌳️root` and `🏭️factory`), another lane's in-progress edit. Also pubmedia: document-media 20, publication-mounted 6.
- Stale grant-shaped tests (tool-machine issuer tests, composition metadata tests, render staging gesture tests, tool-run tests); dag-host/laws ported tests unverified (needs lock-147).
- Rerun: `chk.sh W-toolmachine-N Cargo.toml native -p semio-plugin-composition-emission -p semio-os-flow-host-read -p semio-os-flow-host-core -p semio-os-flow-host-session -p semio-os-infinite-board-dag-host`

STATUS: FIXED-verified W-toolmachine-3 (wasm), W-toolmachine-7/11 (native tool-run, tool-machine, typing-edit, scrub-host); BLOCKED flow-host native chain behind value-factory (ReplyStash) and pubmedia crates
