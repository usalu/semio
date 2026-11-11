# semio-plugin-plugin-program migration

Crate `🔌️plugin/🏗️builder/🔌️program/📦️packages/🦀️rust` (lib root `🔌️plugin/🦀️.rs`). All 122 baseline errors were in `⚛️reactor/🪟️surfaces/{native,read}.rs`; no other file of the crate failed.

## Changes
- `read.rs`: helpers moved to `semio_plugin_job::original_steps` (`built_with`, `funded`, `replied`, `retired`, `retired_poll`, `begun`, `unbuilt`, `unit`). New `Unit` (resident `OriginalConstructionAdmission` + marker) funds every one-unit state change; `advance_cursor`/`take_cursor`/`close_cursor` drive `RetainedCloneOwner` through `advance_into`/`take`/`close_into`. `RetainedFieldCursor` replaced by `RetainedCloneOwner` (begin-close is implicit in `close_into`).
- `native.rs`: `OriginalSurfaceRefresh` on the same helpers; decoder identity is `(NativeAllocationIdentity,u64)`; per-string allocation goes through settled `built`; the removed `Arc<ViewModel>` `RetireOwned` impl is replaced by `ViewLease` (`ArcLeaseRetirement` + `OriginalSourceProjection<ViewModel>`); registry close uses `retired_poll`/`begun` with `step_granted(body,effect)`/`begin_granted(slot,effect)`. Fixed the phase-0 view-absent test, which fired after the view moved into the admission.
- New `projection.rs`: `SourceAdmission` keeps the admission, projection frame and action resident across turns (drive and cancellation close, same stage protocol as store snapshot-clone).
- Source handback uses `take_authority_into` and `close_step`.

## Contract changes for callers (outside this crate, not edited)
`OriginalSurfaceRefresh::admit`, `OriginalSurfaceViewRead::admit`, `OriginalSurfaceViewCapture::admit` now take `(cx, admission:&mut OriginalConstructionAdmission, slot)` and adopt the admission. Callers: runtime `🛂️manifest/🦀️.rs` (lines 219, 225; pass `&mut receiving.construction`) and `metadata-tests.rs`, `🧰️ops/🧾️typed-operations/response.rs` (+ tests `original-view-read.rs`, `response-tests.rs`).

## Registry retirement (second round)
`SurfaceContexts` mutations no longer drop `ViewModel`s (a production bug: non-empty ordered-map roots panic on drop).
- `insert` (including every refusal path), `update_view`, `remove` and window pruning push replaced, removed and refused bindings/views into a resident `displaced: Vec<Displaced>`; `Displaced` is a `RetireOwned` enum (`Binding`/`View`).
- `SurfaceContexts: RetireOwned` through the derive-able `SurfaceContextsOwned{slots,view_state,displaced,retiring}`; the seed asserts the source/capture lease and `RetainedCloneClose` are released.
- `retire_displaced_step(cx)` retires one displaced projection per funded step (`true` once none pending); `close_original_step` drains it first and `original_terminal_is_empty` includes it.
- `SurfaceContexts::get` and `SurfaceContext` removed (they cloned a `ViewModel` the caller could only drop); tests use `borrow`.
- Callers outside the crate (not edited; callers agent): runtime `🧰️ops/🖥️surface-render/🦀️.rs` calls `insert` (l.44), `remove` (l.52), `update_view` (l.120, l.275) and must pump `retire_displaced_step` in a funded turn after each mutation and before dropping an instance's `surface_contexts` (also covered by `close_original_step`). Plus the earlier `admit` signature changes.
- Tests: `🧪️tests/🪟️surface-context-lifecycle` drains every `ViewModel`/registry through `drain`, which runs `ControlledRetirement::step_into` inside the framework's `driven` helper (a genuine original producer invocation) with each step granted exactly the cursor's own `next_demand` quote. The earlier stall was not a grant problem: the plain `EffectBank::run` lacks an original producer invocation, so ordered-map field retirement failed with "retirement field requires its original producer invocation" and my loop counted `Err` as settled. New dev-dependency `semio-framework-value-testing-admission`.

## Status
`check --lib` 0 errors; `check --tests` 0 errors; `test -p semio-plugin-plugin-program --features original-surface-testing`: 8 passed, 0 failed (`plugin-program-14.log`). Temporary prints removed.
