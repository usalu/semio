# Presented Input Candidate Retirement

## Actual runtime failure

The fresh WGPU19 activation faulted at `2026-09-21T15:59:48.279Z` with `worker-frame-failed: presented input candidate generation exhausted`. The captured console is `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-console.json`; the screenshot is `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-boot-fault.jpg`.

The text was not the actual refusal. `ShellState::seal_presented_input_candidate` can return either numeric exhaustion or `retained presented input candidate could not be sealed`, while `FrameBuildPhase::Chrome` mapped every error to the exhaustion string. It now forwards the exact static fault.

## Causal ownership gap

The retained candidate is an external Shell/UI owner, minted during the Chrome phase and carried by `FrameBuildCursor` → `AppFrameAfterChrome` → `AppFramePreparation` → `AppFramePresentation`. Only the presenter used to discard it. Three earlier exits did not:

- a transaction superseded after Chrome dropped its `FrameBuildCursor`;
- a cancelled or refused preparation closed its job and packets while ignoring `input_candidate`;
- a completed frame whose generation became stale was taken out of the retained job, dropped by `generation_is_fresh`, and then could no longer return its witness.

The next Chrome walk consequently met a sealed UI candidate and failed. `AppFramePreparation::terminal_is_empty`, `FrameBuildCursor::terminal_is_empty`, and `AppFramePresentation::terminal_is_empty` also omitted the witness, so the incorrect drop still appeared terminal-empty.

## Repair

`discard_frame_input_candidate` returns one exact witness through the runtime's current `ShellState`. A temporarily checked-out interaction or contended runtime answers `Pending`; the close owner retains the witness and retries rather than dropping it.

The same return step now covers:

- superseded `FrameTransaction` candidates before the transaction is dropped;
- cancelled Build and Prepare phases before their ordinary bounded close ladders;
- explicit `ActiveFrameBuild::close_step` retirement through that same candidate-first protocol;
- completed presentations before their packet/engine retirement;
- stale completion polling, which now leaves the frame inside `ActiveFrameBuild` so that close can return the witness.

`FrameBuildCursor` and `AppFrameAfterChrome` can move the witness into a nested `AppFramePreparation` only from their close ladders. Every production caller now enters those ladders through `retire_cancelled_phase`, which first returns the direct witness. The remaining direct `retire_active_phase` uses are candidate-free unit fixtures, so no recursive compatibility path is needed.

All three terminal predicates include `input_candidate.is_none()`. The presenter's accepted and aborted paths remain the sole owners after a fresh presentation is handed over.

## Laws and receipts

`🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs` adds:

- `superseded_frame_build_returns_its_exact_presented_input_candidate`
- `cancelled_frame_preparation_returns_its_exact_presented_input_candidate`
- `stale_completed_frame_returns_its_exact_presented_input_candidate`
- `cancelled_after_chrome_frame_returns_its_exact_presented_input_candidate`

Each carrier law seals witness N, drains the abandoned owner to terminal empty, then seals and acknowledges fresh witness N+1. This checks the observable Shell/UI slot can publish again; it does not rely only on a private empty-field assertion.

`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` adds `a_presented_input_seal_refusal_keeps_its_exact_fault_name`.

The affected Rust sources and tests parse under `rustfmt --edition 2021 --emit stdout`. Root owns the native Cargo gate.

The frame-action ledger's third-party source oracle was updated to retain its original action-ownership claim while requiring the external input witness return before the action-free candidate drop. Focused Bun/Nx receipt:

```text
SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true \
bun nx run '@semio-tech/framework-renderer-wgpu:test-browser-worker' --skip-nx-cache -- \
'🧪️tests/🧾️frame-action-ledger/🟦️.ts' \
-t 'returns a superseded input witness before dropping the action-free candidate'
```

Result: 1 PASS, 157 outside the filter, 12 files discovered, Vitest 6.77 s, Nx 13.7 s. Receipt: `🗑️generated/astra-presented-input-candidate-retirement/frame-action-ledger.log`.

Native130 executed the full renderer census and all five candidate-retirement/fault-name laws passed with zero skips; the overall run was 1,359 tests / 1,306 pass / 53 failures from other packets. The rebuilt WGPU19 activation then booted without the previous seal fault and rendered Puzzle geometry. A later physical Driver interaction exposed a distinct normal pre-submit supersession fault, recorded separately in `📓️astra-presenter-stale-input-reschedule.md`.
