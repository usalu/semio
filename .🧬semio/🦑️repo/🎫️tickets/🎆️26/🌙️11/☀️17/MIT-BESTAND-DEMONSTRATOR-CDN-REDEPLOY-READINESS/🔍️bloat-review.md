# Bloat cuts review: applied rounds 1 to 5 and Round 6 patch

Scope: read-only review (no cargo, no repo edits). Baseline for the diffs was `2f4d8f89820..HEAD` on the seven files named in Round 4 of `🎈️component-bloat.md`; the cuts are already committed (`3cde99d423d`). Paths below are relative to `🧰️framework/🔨️modules/🌱️value/`; `controlled` = `♻️retirement/🎮️controlled/🦀️.rs`.

## 1. Applied cuts

### Verified equivalent (no behaviour change found)
- `controlled`: `ControlledCore` / `OriginalSlot`. Each former `self.value` test maps one to one: `ready` is `is_none_or(retirement_ready)`, `present`, `supported`, `birth_demand` (same `Err` text when absent), `admit` forwards to `admit_retirement_cursor`. Check order in `next_demand`, `next_computational_work`, `step_with`, `step_original`, `requires_original_*` is identical. `release`/`abandon_release` drop the same fields as the old `Drop`/`abandon` (`abandon` still skips `child_reply`, as before).
- `controlled:76-80` `admit_frame_slot`/`admit_frame`: refusal order, `actual` marker handling, work accounting and the `Ok(None)` at work 0 are preserved; the raw pointer is cast to `ControlledRetirement<T>` only after the allocation succeeded, with the same layout (non-zero size, so no ZST path is needed).
- `step_with`/`step_original` take `&mut dyn FnMut` (`Invoke`). The closure is invoked at most once per call, captures only reborrows (`control`, `original`, `reply`, `protocol_failure`) and never aliases `self` (the core is a separate field from `value`, and `value` is lent as `&mut dyn OriginalSlot` next to `&mut self.core`; disjoint). No lifetime or aliasing problem found.
- Frame crate `OriginalProducerDrive`/`BoundProducerDrive`: `invoke` runs where the closure ran, `finish` runs where `authority.finish` ran, the typed authority is reborrowed once, error paths and the `producer_refusal` bookkeeping are unchanged. The only new error is `"original producer callback already consumed"`, unreachable because each drive is built per call.
- Driver macro: `impl<'c> NativeDecodeControl<'c>` fixes `H = NativeDecodeControl<'c>`; every wrapper builds one `BoundProducerDrive` per call; retirement control keeps its `RetirementFundingScope` inside the closure, so drop order is unchanged.
- `native_close_settle` over `native_close_settle_dyn`: the wrapper `take().expect(..)` panics only if the dyn callee called `close` twice; it calls it once through `OriginalEffectReply::borrow(..).and_then(..)`.
- `plan_field_stage` (tree) versus the old per-field arms: all stage arms compared (0 to 8, present and missing). Stage 0 present stays in the macro, stage 5 missing is the bare skip, stage 8 missing and unknown stages stay `invalid`, lazy `source.borrow()` only when the slot is not ready, `option_bytes` and `field_demand(0)` unchanged.

### Findings
1. **Medium, unrelated behaviour change inside the diff range**: `controlled:376-384` `borrowed_child_step` now records `performed_since(start)?` before `slot.take()` and returns `RetirementStep::Failure(error,work)` instead of `Failure(error,0)`. This is a real semantic change (Work now charged on a child failure) that is not part of the bloat cut and not in the bloat note. The new `?` also introduces an error path that returns with the child reply still stranded in `child_reply`/`slot`; the caller (`step_reply`) turns it into `protocol_failure`, so the stranded reply is never taken. Move `let work=..?` after `slot.take()` or document and test it.
2. **Low**: `value/♻️retirement/🦀️.rs:80` still carries the `eprintln!("[DEBUG] birth mismatch ..")` in the error branch of `retirement_cursor_birth_demand` (runs inside every wasm component on a layout mismatch). Remove it when the layout check is proven (Round 6 keeps it verbatim in `retirement_cursor_birth_core` to stay behaviour neutral).
3. **Low, note accuracy**: `🎈️component-bloat.md` says `reserve_cursor_slot` "lost its unused generic reserve parameter"; it kept a `Reserve<'_>` (`&mut dyn FnMut`) parameter (`controlled:322`) and the test `🧪️tests/🦀️.rs:18` was adapted to `&mut |list,effect|`. Public signature did change (callers must pass `&mut` closures).
4. **Low, CLAUDE.md**: `NativeFieldPlanSlot` (`owned/🌳️value/🦀️.rs` trait) and `plan_field_stage` (`:285`) share the docstring emoji `🧭️`; the file ends with two blank lines before the trait. Docstring emojis must be unique.
5. **Info**: the diff range also contains unrelated edits in the frame crate (`acknowledgement_reporting`, `cancel_phase` phases 7 and 8, `child_failure`, `Self::invalid(reason)`, `source_turn` source) and `with_original_effect`; they were not reviewed here.
6. **Info**: Round 7 describe failures for fem and procedural ("original interpreter deadline requires its caller clock", "no declared composer capability owns the runtime claims") cannot be attributed to the cuts from the diff; none of the cut files touches clock or composer capability code. Re-run describe on HEAD without the cuts only if the cause is not found elsewhere.

## 2. Round 6 patch: `apply-bloat-round6.py`

Located at `<ticket>/apply-bloat-round6.py`. It resolves files by name (ignoring U+FE0F), re-reads each file, asserts every anchor matches exactly once, is guarded by a marker per step (re-run prints "already applied") and accepts `--root DIR` and step names (`cursor relay tree derive backing`). It was dry-run twice against a scratch mirror of the six files (applied, then idempotent); brace balance of all five edited files is unchanged. It was NOT compiled.

| step | cut | what changes |
|---|---|---|
| cursor | draft item 5 | `admit_retirement_cursor` over `admit_cursor_slot(demand,Layout,effect)`; `retirement_cursor_birth_demand` over `retirement_cursor_birth_core` (plain facts and `type_name` fn pointers) |
| relay | draft item 5 | `native_relay` forwards to `#[inline(never)] native_relay_dyn` (explicit turbofish for inference) |
| tree | draft items 1 to 3 | `NativeFieldPlanSlot` gains `plan_close_step_into`, `plan_begin_close_into`, `plan_advance_work_demand`, `plan_advance`; helpers `apply_field_stage`, `apply_field_take` (+ `NativeFieldTaken`), `plan_close_field`, `close_field_slot`, `close_field_step`, `begin_field_close`, `advance_field`, `advance_field_work`; imports `native_close_settle_dyn` directly from the protocol crate (no re-export edit needed) |
| derive | draft items 1 to 3 | `field_apply` (stages 0,1,4,5,6,7 and the take post-processing hoisted; typed `take` and `place_field` stay), `close_plans`, `close_steps`, `nested_retire`, `nested_begin`, `work_steps`, `nested_steps` replaced by helper calls |
| backing | draft item 4 | `quote`/`drive` become one-line forwards to `quote_view`/`drive_owner` over private object-safe `BackingView`/`BackingOwner` adapters (`View`, `Owner`); per receiver type only `select` and `run` remain generic |

Design choices and deviations from the draft:
- The draft's `NativeFieldApplySlot` plus `unsafe fn place_field` shim needs a `&mut dyn NativePendingField` to `&mut Option<T>` downcast (needs `T: 'static` or a layout move). Not done: unsound-by-construction without a compiler. The typed `take`/`place_field` calls stay in the macro, so the `apply` reduction is smaller than the draft's 0.74 M estimate (only the six pure stages and the take post-processing go). The next step would be erasing the pair `(slot,pending)` behind one trait object per field type.
- Item 2 and 3 need no new `close_*` dyn on `NativeReceiveSlot` beyond the four trait methods; the retirement `next_owner_demands::<T>` edge is passed as one small closure per field (`&dyn Fn`) instead of a fn pointer because its generic signature was not verified.
- Evaluation order and short-circuiting of every arm was preserved (slot before default, default before pending owner, pending before placed value); `apply_field_take` reads `turn.work` before moving `turn.result` and returns the early `Settled` step exactly where the old code returned from the closure before the work accounting.
- Enum derive (`✨️derive/🛬️receive/🔀️enum/🦀️.rs`) is untouched: its per-variant `plan` arms use the same pattern and are the next candidate once this step compiles.
- `std::iter::once(..).chain(default)` was deliberately avoided for `&mut dyn` slots (invariance of the trait object lifetime); plain sequential helpers are used.

### Compile risks to check first (cannot be verified without cargo)
1. `View::<I,T,C>{..}` and `Owner::<I,T,C>{..}` struct expressions with lifetime elision; `fn view(&self)->&dyn BackingView{self}`.
2. Closure inference through `&mut |..|` against `&mut dyn for<'r> FnMut` in `close_field_step`, `begin_field_close`, `native_relay`.
3. `Some(&mut self.#default)` coercing to `Option<&mut dyn NativeFieldPlanSlot>` inside the derive output.
4. `RetainedWorkPoll::map` closure typing in `advance_field` (same expression as the old macro, now in a fn with an explicit return type).
5. The trait methods use `crate::native_decoding::OriginalEffectReply` in the impls and the protocol-crate alias in the trait; they are the same type in the old code (`native_close_settle` accepted both), so no change is expected.

Verification order after applying: `repo cargo` check for `semio-framework-value-retirement`, `semio-framework-value-native-receiver-protocol`, `semio-framework-value-native-owned` (tree), `semio-framework-value-derive`, then the retirement (30), receiving-frame (4), receiver-protocol (4) crate tests and the native-owned, native-decoding and derive suites that Round 7 did not run; then the energy component size.
