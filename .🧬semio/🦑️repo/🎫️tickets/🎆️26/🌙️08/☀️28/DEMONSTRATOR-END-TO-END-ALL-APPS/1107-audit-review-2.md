# 🔎️ 1107 Audit Review 2 (aud-review-2)

Read-only review of `git -c core.fsmonitor=false diff HEAD` for crates ported since the value audit: pack-json (+ canonical, native-input, output-receiving, source), io-control, io-sqlite-snapshot and -model, replication-*, value-edit, value-snapshot-read, tool-run-* (lib and os plugin), deflate, machine, pixels, geometry, ui-*, window-config-*, os-semio.

Scope diff: `C/🗑️generated/1107-review-2-scope.diff` (130 files, 8741 lines, pathspec over those crate directories). Changed-file list: `C/🗑️generated/1107-review-2-names.txt`. No runs, no edits outside C, no sub-agents. Paths are repo-relative. Owners come from the 07:34 and 08:25 addenda in `C/1107-brief.md`; agent ids from `C/1107-lanes.md`.

Scope notes: `📐️geometry` (framework) has zero changes in the diff. The geometry crates named in the brief are plugin-side only and are not reviewed here. ui-* changes are almost all tests. The lanes file has no agent ids for port-e and port-f, so those owners are named by role only.

## Findings, ranked by runtime impact

### 1. `reborrow(grant, effect.maximum_work_units())` in two live paths (high impact, confidence high on the code, medium on reachability)

- `🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🦀️.rs`, `initialize_one`: `owner.reserve_one_funded(&mut effect.reborrow(RetainedCloneGrant{maximum_depth: grant.maximum_depth - 1, ..grant}, effect.maximum_work_units())?)`. Owner: bearbeiten.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📬️mutation/🦀️.rs`, `ErasedWindowMutationRetirement::close_step`: same call, returns `RetirementStep::Failure` on Err. Owner: port-b.

`RetainedAllocationEffect::reborrow` (`🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📦️effect/🦀️.rs:32-33`) returns Err when `maximum_work_units > remaining_work_units()`. Passing the parent's own maximum therefore fails as soon as any work has been performed on the parent, or whenever the receipt remainder is below the authored maximum. On the encode path (`u64::MAX` maximum, value review finding 1) it fails on every call. The correct call is `effect.nested(child)`, which the other sites in these same files already use. Reproduction was not run.

Impact: deflate page growth returns a refusal on the first page reservation after work has been performed. Window-mutation close fails instead of retiring the erased original.

Fix: replace both with `effect.nested(RetainedCloneGrant{maximum_depth: grant.maximum_depth-1, ..grant})`, or a checked child. Add a test that drives deflate reserve and window-mutation close through a parent effect that has already performed work.

### 2. Tool-run tick receive reports a constant `work:1` (medium-high impact, confidence medium)

`🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🦀️.rs`, `advance`: `control.receive_construction_step(demand, |control, grant| RetainedWorkOutcome{result: self.advance_admitted(...), work: 1})`. Inside `advance_admitted`, the nested `self.trace.advance(control, child)` and the parser calls do their own work, but the reported turn work is always 1. Owner: port-d.

The brief's law is that the reported work equals the recorded work for the phase. If `NativeDecodeControl` charges nested work on a separate ledger, the trace or parser work is never debited. If it charges through the same sink, it is debited twice. I did not read the `NativeDecodeControl` charge path, so the direction is unconfirmed.

Fix: report `work` from the actual effect or control observation, as `advance_admitted`'s callers do elsewhere in the crate.

### 3. Work reported with no `effect.performed` behind it (medium-high impact, confidence high on the code fact, medium on impact)

- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs`, `advance`: line `self.last_turn_work=1;` runs for every admitted turn. Flat-run, scalar, text and object-colon branches then produce no `effect.performed(..)` call. Owner: aggregator.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🛬️native/🦀️.rs`, phases 2 and 3 and `close_with`: `cx.consume_fuel(1)` and `consume_fuel(1u64.max(performed))` debit a unit even when the effect performed nothing. Owner: port-d.

The same class was flagged in the value audit for queue, bulk and refusal. Each case debits fuel for work the original action did not record, so performed and turn work diverge and `cancel_for_action` can run on a unit that was charged.

Fix: either record the unit with `effect.performed(1)` inside the effect, or report 0. For `max(1, performed)`, use `performed` and drop the floor.

### 4. Failure paths drop or skip charges for work already performed (medium impact, confidence high on the code)

In the tool-run os native tick (`🛬️native/🦀️.rs`):

- Phase 3: `Some((Some(Err((error,held))),_))` discards `performed` (`_`), so work the effect did before failing is never debited.
- Phase 2 and phase 3: `cx.consume_retained(p)?;cx.consume_fuel(..)` runs the retained debit first. If that `?` fails, the fuel debit after it is skipped for work already performed by the effect.
- `close_with`: the same ordering, `cx.consume_retained(progress)?` before `cx.consume_fuel(..)`.
- `None` path of `with_original_effect` returns before any debit; that one is correct (nothing ran).

Owner: port-d. Fix: take `performed` on every arm, and debit fuel before the retained `?`.

### 5. PNG decode drops error kinds and skips the effect on other phases (medium impact, confidence high on the code, medium on impact)

`🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs`, `ImageDecodeJob::step`:

- `"png"`: `advance(&mut effect.nested(grant)).map_err(|_| ImageDecodeError::Invalid("PNG reconstruction refused"))`. Every `ValueError` kind becomes `Invalid`, so WorkLimit and AllocationFailed refusals reach the caller as invalid input. A funded retry can never happen.
- `"jpeg"`, `"source-handoff"`, `"image-handoff"` and the source phase still call `advance(grant)` / `take_result(grant)` and never touch `effect`. Work done in those phases is not recorded on the original effect.

Owner: proc. Fix: pass `effect.nested(grant)` everywhere in `step`, and keep the refusal kind in the `ImageDecodeError` mapping.

### 6. Loosened test assertions (medium, confidence high)

`🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧪️tests/♻️localized-label-retirement/🦀️.rs`: the old `owner.step(zero_grant).unwrap()` and exact-equality checks became `assert!(result.is_err()||result.unwrap().progress()==RetainedCloneProgress::default())` in four denial cases. Any refusal now passes, including the wrong one, and the physical marker is checked only as `(0,0)`. Owner: port-c. Fix: assert the specific refusal variant for each denied field.

`🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🧪️testing/👥️presence-retirement/🦀️.rs`: 43 changed lines. Most removed asserts were rewritten with equivalent checks (`Some(1)` strong count, `original.as_deref()`). The `Busy` arm still checks heap `(0,0)`, but now matches a wrapped error. Not clearly weakened; not fully read. Owner: port-a.

### 7. Pending poll drops its progress (medium-low impact, confidence low-medium)

`🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs`, `retire_decode`: the `Ok(NativeReceivingEffectPoll::Pending{work})` arm returns `RetainedCloneProgress::default()` and drops `extra` from the operation. Any retained capacity or release performed by the operation before the pending return is then not reported to the physical marker. Owner: koordinator. Confidence is low because I did not verify whether `Pending` can carry retained progress in `receive_effect`.

### 8. Production panics added (medium-low impact, confidence high on the code)

- `🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🦀️.rs`, `account_close`: `.expect("flood close releases only retained capacity")`. Owner: proc.
- `⏯️tool-run/🎞️tick/🛬️native` and `📊️progress/🪟️presentation/🧬️update`: `ToolRunPresentationUpdate::admit(..).unwrap_or_else(|_| panic!("original presentation update birth was fully preflighted"))` in `begin_tick`, and `self.state.published.as_ref().unwrap()`. Owner: port-d.
- `🗂️registry/🦀️.rs` (window-config): `pub fn insert` now begins with `assert!(self.insertion.is_none(), ...)`. Any caller that inserts while an original insertion is pending panics. Owner: port-b.
- `🪶️sqlite-snapshot/🫴️receiving/🦀️.rs`: `checked_sub(before_work).expect("original SQL producer work is monotone")` and `slot.as_mut().unwrap()`. Owner: port-f.
- `🛫️snapshot/🦀️.rs` (io-control): `value.expect("completed snapshot retirement step")`. Safe only if `post` never turns an Err into Ok. Owner: koordinator.
- `🧭️tree/🦀️.rs`: `expect("checked immutable native canonical child")` inside the `project` closure. Guarded by the `canonical_tree_child(ordinal)?` precheck; low risk. Owner: aggregator.
- `📨️wire` (canonical JSON): `assert!(self.source.as_ref().is_some_and(RetainedCloneSource::terminal_is_empty))` in `return_original`. Owner: aggregator.
- About 90 added `expect`/`unwrap`/`panic` lines in scope. Most are state-machine `.as_mut().unwrap()` in tool-run entity and update cursors (`👥️entities`, `🧬️update`, `➕️insert`, `🧾️provenance`, `📸️version`). These sit on paths the phase machine guarantees, but any phase bug panics rather than refusing. Owner: port-d.

### 9. Underflow arithmetic on depth and copy budgets (low impact, confidence high on the code)

- deflate `initialize_one` and `retire` paths: `grant.maximum_depth - 1` with no guard. Debug panic at depth 0. In release the wrapped value is larger than the requested grant, and `restricted_by` then makes `reborrow` return Err, so it refuses instead of widening. Fix with `checked_sub`.
- tool-run tick receive, `advance_admitted`: `grant.maximum_copy_bytes - parent` and `grant.maximum_depth - 1`. Reached only after the caller's demand admission, which includes `parent` and depth 1, so it cannot underflow today. Still unchecked arithmetic on a production path; use `checked_sub`.

### 10. Zero-progress completions that could spin a driver (low, confidence low)

- canonical tree `advance`: `_ if output.is_empty()` returns `Progress(default)` with work 0 and does not advance phase. Repeats for as long as the caller passes empty output.
- tool-run os native `close_with`: `!funds(cx, demand)` returns `Pending{progress: default}`; `close_effect_step` returns `Progress(default)` when `remaining_work_units()==0`.
- canonical tree `return_original`: `!permits` returns `Progress(default)` with work 0.

I did not find the production driver's stall guard, so these are unconfirmed as spins.

## Checked and not confirmed

- Replication indexed (`📋️indexed/🦀️.rs`): every child now uses `effect.nested(child(g))` and charges through the shared sink. `maximum_work_units-1` is safe because `advance` returns early when the maximum is 0. No double charge found.
- Replication, `work_demand` path: `maximum_work_units==0` guard present.
- Value-snapshot-read `close_step_controlled` and `close_original_step` (`🌱️value/📸️snapshot/👁️read/🔗️source/🦀️.rs`): `grant.maximum_depth-1` sits after the `retained_clone_grant_admits(grant, d)` check, and the active-path demand carries depth at least 1. Uses `effect.nested`. Fine.
- Window-config `close_step`: `if grant.maximum_depth<2` guard before the subtraction. Fine apart from finding 1.
- JSON grammar (`🧮️parser-and-grammar-cursor`): the owed-work remainder is recorded on the effect exactly once (`effect.performed(owed - done)`), so there is no double charge. Fine.
- Tool-run tick `begin` / `trace.take`: `expect("quoted original trace output move")` was replaced with `ok_or_else(malformed)`. Fine.
- Tool-run tick `Pending` in `close_step`: converted to a malformed error, so no spin. Fine.
- Pack canonical tree pop path: `frames.pop()` releases the frame without a release report, but `release_empty_page(effect)` at the end of the list reports the bytes. No leak found.
- Pixels flood `account_close` now accounts only on `Complete`. If `Pending` never carries retained progress this is correct; not verified.
- No `todo!`, `unimplemented!` or `#[allow]` added in scope. No `//` comments added inside definitions.

## Owners (from the lanes and addenda)

| Finding | Owner |
| --- | --- |
| 1 deflate | bearbeiten |
| 1 window-config mutation | port-b |
| 2, 4, 8 tool-run and tool-run os native | port-d |
| 3 canonical tree | aggregator |
| 5, 8 pixels | proc |
| 6 ui-locale test | port-c |
| 6 presence-retirement test | port-a |
| 7, 8 io-control | koordinator |
| 8 sqlite receiving | port-f (no agent id in lanes file) |
| 8 window-config registry | port-b |
| pack-json base (`🧮️parser-and-grammar-cursor`) | unassigned in the lanes and addenda |
| geometry | no changes in framework; unassigned |

## Confidence and limits

- Finding 1 is verified on the code: `reborrow` (value `🎟️control/🧬️contract/📦️effect/🦀️.rs:32-33`) rejects a maximum above `remaining_work_units()`, and both call sites pass the parent's maximum. Reachability from a live encode or a window close was not traced to a caller.
- Findings 2 and 7 depend on `NativeDecodeControl` and `receive_effect` charge semantics that I did not read in full.
- Nothing was run. No test reproduces any finding.
- I read the scoped diff by grep and targeted hunks, not line by line.
