# Store Close: Cold Wallet Identity and Quote Body

## Goal 1: host `retained-laws` store close loops

Both failing tests (`actual_atomic_publication_is_fail_closed_and_retires_stale_candidate`, `every_store_replacement_phase_unit_fits_the_interactive_step_budget`) now pass; `retained_laws::` is 6 of 6 (`replay-s3.txt`).

### Measured causes (two, independent)

1. **Cold close lost its Work binding every turn.** `close_step_cold` built a fresh `AuthoringWallet` per call, and `run_with` created a fresh `OriginalWorkAuthority` (issuer identity) and a fresh `OriginalWork` scope per run. The shared-original close ladder (`FactorySharedRetirement::close`, `arc-custody/factory`) captures `OriginalWorkBindingIdentity` (scope, issuer, source) at its first action and `follow_between_actions` refuses any later action whose binding differs. With no Source turn the binding is never refreshed, so the second cold turn failed with "captured shared original changed its Work or destination". The failed turn left the ticket pinned (`FactoryTicketClose.body`), which then made the next quote answer "ticket close changed its selected body". The quote error was a symptom, not the design flaw, so `FactoryTicketClose` keeps its strict body law (`🔒️close/🧪️tests`, `changedBodyDenied`) unchanged.
2. **Quote and step used different bodies.** The ladder pages by `effect.grant().maximum_copy_bytes`. `run_invoked` widens the effect by the reply handoff (measured +80 copy bytes: stock 1024 gave an operation grant of 1104), while callers quote with the body they size the grant by. The documented design (`invocation_extent`: "the operation itself still sees only the caller's own extent") was not what `run_with` did.

### Fix

- `AuthoringWallet` (`🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️authoring/🦀️.rs`) is now one resident Work owner: it keeps its `OriginalWorkAuthority` and `OriginalWorkState` between runs, so every run of one wallet shares one binding and two wallets never do. New `AuthoringWallet::self_funded()` (unbounded stock for a caller that funds each run by its own quoted grant). `new` is no longer `const`.
- `ArtifactColdClose` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs`, exported from `semio_plugin`): one resident wallet for every cold close turn of one owner; each turn runs `close_step` on `effect.reborrow(grant, 64)`, exactly the quoted grant, so a ladder's body equals the body the caller quoted. `ArtifactOwnedDisposer::close_step_cold` is now `ArtifactColdClose::new().step(..)` (one-shot, fresh wallet). The wallet is not stored in `ArtifactDocumentStoreDisposer` on purpose: it stays a zero-size owner, so its terminal frame release and birth accounting are unchanged.
- Callers that loop a store close now hold one `ArtifactColdClose` and quote with `PLUGIN_SELF_FUNDED_BODY_BYTES` (the body the grant is sized by): host `retained-laws` (`close_store`, displaced-store loop) and the process3d editor wasm unit `close_store`.

### Laws

- `🎟️authoring/🧪️tests` (`case.json`, `schema.json`, TS twin `law.test.ts`: ajv corpus check plus SQLite cross-check), both green in `semio-framework-value-control-contract` (10 of 10, `stale-wallet2.txt`):
  - `one_wallet_is_one_resident_work_binding_across_runs` (one binding per wallet, none shared between wallets);
  - `an_exact_reborrow_of_an_invoked_effect_sees_exactly_its_callers_extent` (the extent `ArtifactColdClose` hands each turn; the outer effect is wider by the reply handoff).
- Integration law: host `retained-laws` close loops (6 of 6), which fail without either fix. `semio-plugin-ownership` lib tests do not compile today (28 stale errors, predating this work), so no law lives there.

Pending ticket invocations did not occur in these loops (a temporary print on every pending ticket child never fired), so nothing needed to resume across wallets. If a ladder ever leaves a reply pending in cold mode, its invocation identity (`authority`, `action`) is still per `invoke_original` call; that case is not covered and would need the cold driver to recover within the same invocation.

## Goal 2: `history_edits_end_to_end` final close (not fixed, measured lead)

Status: not fixed. A fresh measured run (`replay-g2c.txt`, `replay-g2d.txt`) no longer reaches the exhausted final close at all: the **first** terminal close of the first case fails with `captured shared original changed its Work or destination` (`acceptance-policy-pump-and-drive:114`). A temporary backtrace on that error (`replay-g2d.txt`, removed again) shows a pure Job-turn path, not a cold wallet: `VcsArtifactApp::close_step` -> `plugin_effect_close` -> `StepContext::with_original_effect_at` -> `OriginalJobWorkDriver` loan -> `ChildMemberRetirement::close_step` -> `SemioMembers::close_owned_step` -> store ladder -> `ArcRootRetirement` -> `FactoryTicketClose::drive` -> `FactorySharedRetirement::close` (`follow_between_actions`). The same capture-and-follow Work binding as Goal 1, but with a real Source turn: a close effect whose `work_source_turn()` is absent or whose action is a recovery cannot refresh the binding captured at admission. This did not occur in `replay-g4.txt` (older tree), and the files the job-context owner is editing right now (`🧵️job/⏱️context/🛬️owned`, `🛬️receiving/🎟️work`, `original-slot`, `🧬️retained-clone/🔗️source`, the `invalid(cause)` signature in `arc-custody/factory`) are the likely origin; it is not caused by the wallet change (this path never uses a wallet). It needs the owner of that change before the copy drain can be measured again.

Measured over-spend lead (the one number set that was captured before the failure, with a temporary print at the start of the close loop, removed again): at the start of the first close the Source total had been charged **16,786,730 B copy, 134,208,200 B capacity, 67,108,864 B release and 1,048,602 items** while `retained_progress()` (actual spend) showed only 83,948 B copy, 20,816 B capacity, 0 release and 79 items. The charged amounts are exactly `unquoted_receiving_grant` (`🧵️job/⏱️context/🛬️owned/🦀️.rs`: 16 MiB copy, 1 Mi items, 64 MiB release, half of the whole capacity): the first receiving scope reserves its whole offered share from the Source total and the share is not refunded to the total when the scope settles (or the scope stays resident). With a 504 MiB copy / 512 MiB release stock that is a fixed 3 percent copy and 12.5 percent release tax per receiving scope, which is consistent with the earlier exhaustion (`replay-g4.txt`: spent 527,797,634 B copy against a 528,482,304 B stock) but is a lead, not a proven cause (the scope may simply still be resident at that point). The close path itself (quote 64 B copy against 35 B remaining) is only where the exhausted stock shows up. If confirmed, the fix belongs in the receiving scope settle (return the unspent offered share to the Source total) and needs the job-context owner; the fixture stock was not touched.

## Files

- `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️authoring/🦀️.rs` (+ `🧪️tests/🦀️.rs`, `case.json`, `schema.json`, `law.test.ts`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs`, `🔌️plugin/🦀️.rs` (export)
- host `🧰️owned/🧪️tests/🔬️retained-laws/🦀️.rs`, process3d editor `🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`
- Touched and restored: `🌱️value/♻️retirement/🎟️tickets/🔒️close/🦀️.rs` (only temporary prints, now identical to before), `arc-custody/🏭️factory/🦀️.rs` (print only), `🧪️artifact-app-laws/🏗️registered-app-construction.../🦀️.rs` (prints only).
