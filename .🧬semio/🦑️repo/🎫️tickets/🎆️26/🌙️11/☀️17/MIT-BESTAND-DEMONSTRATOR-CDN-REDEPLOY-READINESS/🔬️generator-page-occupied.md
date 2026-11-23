# 🔬️ Generator `original page allocation attempt is occupied` (root cause NOT yet isolated)

## Status

Static analysis did not isolate the second allocation; no law test could be written without guessing. A permanent-ish
diagnostic was added so the next procedural build names the exact caller.

## Mechanism (verified by reading)

- The refusal is `reserve_page_using_for_limit` (`🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`): the effect's `actual()` marker is
  already `Some` when a page reservation starts, i.e. one `RetainedAllocationEffect` frame (one `OriginalEffectRun`) already
  recorded a physical allocation before the list reserved a page.
- Every host that opens effects creates a FRESH marker per effect (`OriginalEffectRun::begin`, `with_effect_reported`,
  `with_allocated_effect_reported`, `StepContext::with_original_effect_at`), so repeated `advance_with_turn_ceiling` calls in the
  bundle's 4096-iteration loop (`store/⚙️engine/🧳️bundle/🦀️.rs:216`) do NOT share a marker. That loop is not the cause.
- Nested/reborrowed effects (`nested`, `reborrow`) share the parent marker. So the culprit is a single step that allocates
  something (Vec/String/Box, or a pre-set `*effect.actual()=Some(..)`) and then, in the same effect, calls a list
  `reserve_*`. Other call sites guard this with their own message (`vector allocation ...`, `JSON collection ...`, `UTF-8 ...`);
  only unguarded list callers surface the list message.

## Eliminated

member batch receiving (`📨️emission/🧳️receiving`), binary operation receiver, canonical-edit sealer, snapshot-clone phases,
paged UTF-8 append (single call per effect), intrinsic body `advance` (already breaks on `actual().is_some()`), plugin command pack
receiving (one step per effect), tool-run member fold/emit (dead callers), dsl vector receiving (one allocation per phase).
Known unguarded-class hint: `verify-wave-4.md` records `RetainedIntrinsicBody::new` doing a second page allocation in one effect (test only).

## Change made (diagnostic, `[DEBUG]` prefixed, to be removed after the cause is fixed)

`🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`: `#[track_caller]` on `reserve_one`, `reserve_one_using`, `reserve_capacity_one`,
`reserve_exact_capacity_one`, `reserve_one_funded`, `reserve_capacity_one_funded`, `reserve_effect`, `reserve_page_using_for_limit`;
at the occupied refusal, `eprintln!("[DEBUG] page allocation occupied caller=<file:line> actual=<Some(bytes)>")`. `check -p semio-framework-value-persistent` passes.
The log names the outermost non-`track_caller` call site that reserved the second page and the marker value left by the first allocation.

## Needs rebuild

Any module that links the list crate: procedural (to reproduce in the Generator pane), and demonstrator if it must stay in sync. Read
`[DEBUG] page allocation occupied caller=...` in the pane console, then fix that caller (settle/break after the first allocation, as
`Materializer::advance` does) and add the law test for it.

## Blocker observed

`semio-os-store-engine` currently fails to compile (duplicate `close_owned_step_resident`, E0592) from a concurrent edit, so the
store-level tests (`semio-os-store-member-open-tests`) cannot run right now.

## 03:55 Re-Probe

Procedural rebuilt 02:30 to 03:50 with the caller diagnostic. Generator no longer shows `original page allocation attempt is occupied`, so a concurrent change removed the double allocation. Generator now fails like Koordinator: `[DEBUG] one-item publication blocked`, then `setActiveExample` stalled (`pending_artifact=true publication=true`). The `batch blocked at` diagnostic lands with the demonstrator build started at 03:30.
