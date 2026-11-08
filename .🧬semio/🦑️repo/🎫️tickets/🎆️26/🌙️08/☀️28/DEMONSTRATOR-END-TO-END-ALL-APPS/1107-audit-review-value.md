# 🔎️ 1107 Value Lane Accounting Review (aud-review-value)

Read-only review of `git -c core.fsmonitor=false diff HEAD -- 🧰️framework/🔨️modules/🌱️value` (143 files, +1266/-1478). Full diff saved to `C/🗑️generated/1107-review-value.diff` (evidence, not deleted). No runs, no edits outside C, no sub-agents. Paths below are relative to `🧰️framework/🔨️modules/🌱️value/`.

## Findings, ranked by runtime impact

### 1. Child effects reborrow with the parent's authored maximum (critical, confidence high on arithmetic, medium-high on reachability)

`RetainedAllocationEffect::reborrow` (`🎟️control/🧬️contract/📦️effect/🦀️.rs:32`) rejects any child whose `maximum_work_units` exceeds `remaining_work_units()` (`min(original receipt remaining, maximum - spent)`). The diff's own test pins this (`📦️effect/🧪️tests/🦀️.rs` asserts `parent.reborrow(grant,2).is_err()` with remaining 1).

Yet about 50 added non-test call sites (about 32 files) call `effect.reborrow(grant, effect.maximum_work_units())`, e.g. `♻️retirement/📋️queue/🦀️.rs` (step/step_original, `expect("queue child exceeds original effect")` and `expect("retirement child exceeds original effect")`), `🧬️retained-clone/🗺️ordered-map/*`, `🧬️retained-clone/🧬️contract/*`, `🛬️decode/🧳️owned/🌳️value/👥️tuple/🦀️.rs` (`close_step`, `close_original_step`, and `?` in `advance_local` stage 3), `🛬️decode/🏭️construction/🧳️slots/🦀️.rs`, `🔗️read/*`, `📽️projection/*`.

The encode path makes this always fail. `🛫️encode/🎟️effect/🦀️.rs:28` builds the closure effect with `RetainedAllocationEffect::new(grant, u64::MAX, actual, sink)`. Then `maximum_work_units()` is `u64::MAX`, `remaining` is the finite receipt remainder (at most the reserved quote), so `reborrow` returns `Err`. The `?` sites return `OwnershipLimit`, and the `expect` sites panic. The `Some(bytes)` branch (around line 123) has no `catch_unwind`, so the panic propagates. The `None` branch wraps it in `catch_unwind`, which turns it into a refusal.

Even with a sane maximum, the same call fails once the parent has performed any work in that effect.

Fix: pass the child's own quote, or `effect.remaining_work_units()?`, and give the top-level effect the reserved quote instead of `u64::MAX`. Add a test that drives a nested cursor (list, map, tuple) through `run_closure_effect`. Existing tests use fixed small maxima and never reach this path.

### 2. Work reported without being recorded on the effect (high, confidence high on the code fact, medium on impact)

The brief's law is that the reported work equals `effect.performed` for that phase. These paths report work the original receipt never observed:

- `♻️retirement/📋️queue/🦀️.rs:101`: the owner-pop path sets `*work=1` and pops the frame, with no `effect.performed(1)`.
- `🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs` `close_step` (around lines 120 and 122): the frame-pop and `self.failure.take()` branches set `work=1` with no `performed(1)`.
- `⚠️refusal/🦀️.rs` `advance_nested` (around line 124): phases 2, 4 and 5 report `work = u128::from(result.is_ok())` with no effect at all.

Impact: the job layer debits `turn.work` (fuel is charged), but the original action records 0 for that unit. If `performed==0`, the settle path calls `cancel_for_action`, so the charge and the committed original work diverge.

Fix: record these units with `effect.performed(1)` inside the effect, or report 0.

### 3. Denied grants quote work they never record (medium, confidence high)

`🛬️decode/🧳️owned/🌳️value/👥️tuple/🦀️.rs` advance_local (line 19), `⚠️refusal/🦀️.rs` advance_local (line 133) and the bulk `advance_local` return `Progress(default)` with 0 recorded when `!retained_clone_grant_admits(g,d)`, while `select_advance` had quoted `Local{work: 1}` (or the demand). This breaks the brief's rule that the quote equals the recorded total. The charge stays accurate, because only performed work is charged, so the impact is mostly on the invariant and on the driver's stall accounting.

### 4. Zero-progress completions can spin the driver (medium, confidence medium)

- Tuple `advance_local` stage 2 (`if self.$field.is_none() { return Ok(Some(Progress(default))) }`) leaves `stage==2`, so the next call repeats the same take.
- Refusal `advance_local` phase 1 has the same shape.
- Refusal `advance_nested` phase 3 returns `Complete(Progress, work 0)` without advancing the phase when the birth grant is not admitted.
- Queue `close_original_birth_step` returns `done(Ok(Progress(default)))` with `work 0` when `maximum_depth` is exhausted by retained frames.

None of these is an unbounded loop by itself. Each relies on the caller giving up after repeated zero-progress turns. The tests use `stalled<=143`, but I did not find the production driver's stall guard, so I could not confirm the caller behaviour.

### 5. Panics and unwraps added to non-test code, plus underflow (high for the rule, medium for runtime)

- About 66 added `.expect(...)`. The majority are the reborrow sites in finding 1. Others include the encode `expect("resident original effect")` and `expect("resident original failure")` (`🛫️encode/🎟️effect/🦀️.rs`), `expect("original text operation retained")` (`📝️text/📦️paged`), `expect("checked original inline owner")` and `expect("original inline output present")` (paged text), `expect("complete original UTF8 byte sequence")` and `expect("resident string byte owner")` (`🧬️contract/🔤️string-cursor`), `self.work.cancel().expect(...)` (`🛬️decode/🎟️work/🦀️.rs`), and `expect("retained matcher")` (`🔀️variant`).
- Added `.unwrap()` in production code: `input.project(rank, move|v| v.sequence_child(rank).unwrap())` in the tuple macro (both `demands` and `advance_nested`), `self.frames[self.depth].as_ref().unwrap()` in the bulk receiver (`advance_local`, phase-20 check), `self.inspection.header.as_ref().unwrap()` in `📋️list/🟰️equal`, and `state.child.as_mut().unwrap()` in `🧬️retained-clone/🌱️dynamic`. The tuple `project` unwrap is safe only if `demands` validated the index first, and the closure runs later.
- Underflow: `grant.maximum_depth-1` (queue `reserve_one_funded` child) and `effect.maximum_work_units()-1` (queue admit path). In release, the subtraction wraps to a huge limit.

Fix: replace with `ValueError` returns, and use `checked_sub`.

### 6. Test code no longer compiles against the constructor (medium, confidence high)

`RetainedAllocationEffect::new` now takes four arguments (`grant, maximum, actual, work`). Several test call sites still pass three, for example:

- `🧬️retained-clone/🔗️source/🎟️work/🧪️tests/🦀️.rs` lines 11 and 14
- `🧬️retained-clone/🔗️source/🧪️law-bank/🦀️.rs`
- `♻️retirement/🧬️contract/🧪️tests/parts.rs`
- `♻️retirement/🎮️controlled/🧪️tests/🦀️.rs`
- `♻️retirement/📋️collection/🧪️tests/native.rs`
- `🗂️ordered/✍️owned-entry/🧪️tests/effect.rs`
- `⚠️refusal/♻️retirement/🦀️.rs` (test, line 129)
- The new `🛬️decode/🏭️construction/🧳️slots/🦀️.rs` test and its `close` helper (`RetainedAllocationEffect::new(grant,u64::MAX,&mut capture_actual)`)

So those tests cannot prove the accounting until they are ported. The new shared-slots test is the most relevant, because it is the only test that exercises `NativeSharedSlots` closing.

### 7. Checked and not confirmed

- Loosened assertions: I compared removed and added asserts in the heaviest test files (`tuple-work.rs`, `alias/🧪️tests/work.rs`, `dynamic/🧪️tests/protocol.rs`, `original-scalar-public.rs`, `source/🧪️testing`). The removals I checked were rewrites to `effect::` wrappers with the same or stronger checks (`turn.result.unwrap()`, heap `(0,0)`, `complete==!cancel`, `output_ready`). I did not find a clearly weakened assertion. I did not read every test file.
- Double charge: the delegated-cursor path (queue `admit_owned_retirement` plus `adopt_reserved`, tuple and bulk `bind`) reports work that is recorded once on the shared sink. I found no clear double charge.
- Comments inside definitions: none added (`//` grep on added lines found only doc comments and strings).
- `todo!`, `unimplemented!`, `#[allow]`: none added.
- `panic!` added: only in test helpers (`"original owner requires paid close"`, `"original owner did not close"`, `_=>panic!` in law-bank). Not production.
- Retained bytes not released on close: the queue pop and bulk frame-pop report `released_bytes` only in the queue case (`size_of_val(owner)`). In the bulk frame-pop, `Frame` drops any owned `String`/`Vec` without a release report. I did not find an owned field that survives to that point, so this is unconfirmed.
- Refusal and failure variants: the refusal receiver's identity check and its `prepaid retirement node absent` and `retirement fields absent` checks are preserved in the new code.

## Confidence and limits

- Finding 1 rests on the `reborrow` guard, the `u64::MAX` construction at `🛫️encode/🎟️effect/🦀️.rs:28`, and the pinned test. I did not run anything, so this is a reading-based result, not a reproduction. A targeted test (nested tuple close under `run_closure_effect`) would confirm it.
- The review sampled the core accounting files and used grep sweeps over all 143 files. It did not read all 1.1 MB of diff line by line.
