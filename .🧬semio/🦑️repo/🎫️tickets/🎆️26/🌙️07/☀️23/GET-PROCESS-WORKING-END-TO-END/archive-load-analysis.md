# Archive Load Analysis (read-only agent, 2026-11-22)

Scope: `every_example_loads_through_the_member_less_archive_door` stays at "Running, 127 of 127 records", and the browser's maintenance Stage(24) makes zero progress. I only read code and the existing logs (`🗑️generated/ed40.txt`, `ed41.txt`). I built nothing and edited no source.

## Verdict

**None of the three `receive_step` `None` conditions fires.** The `[DEBUG] recvnone` counter is 0 in both ed40 and ed41, and `req=None` appears in every `recstep` sample, so the pinned `receive_request` is never set. Relaxing the pin does not change anything.

The stall happens one level lower, in the first Receive action: **seeding the top-level native receiver.** That seed is one indivisible copy of the whole receiver value (~271 KiB). Each record step admits it, but the unquoted receiving scope offers it only half of the turn's copy grant, so the seed can never run. Every futile attempt still pays the scope's admission and control fees out of the pinned Source stock. The stock drains until even the outer gate fails (`g.copy = 277151 < 277538`), and from then on nothing moves.

## Evidence (ed41, after the covering-loan fix)

- Every Receive sample (phase 6, from `recstep#24000` to `#678000`) shows `demand = RetirementDemand { copy_bytes: 277538, capacity_bytes: 0, depth: 1 }`. The value is constant across about 650k calls.
  - That is `size_of::<Phase>()` (1) plus `NativeOwnedStorage::<S::Receiver>::seed_demand()`, which is `size_of::<Receiver>() + size_of::<bool>()`. So `size_of::<S::Receiver>()` is about 277,536 B.
  - `capacity 0` rules out the reserve path, which quotes the Layout as capacity. `depth 1` rules out a nested field, because `nested()` adds depth. A template part would be at most 1024 B plus metadata. The only matching path is the one-shot seed:
    `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs:28` `NativeReceivingOriginal::demands`, `if !self.receiver.initialized(){ match T::receiver_template(false){ … None => seed_demand() } }`.
  - Derived struct records use the default `receiver_template` = `None` (`🌱️value/🛬️decode/🧳️owned/🦀️.rs:33`). Only the enum derive provides a per-field seed template (`✨️derive/🛬️receive/🔀️enum/🦀️.rs:64`).
- Fuel per sample is about 1.998M of 2M. A 277 KiB seed would charge about 277k fuel when published, so the seed never ran.
- Samples #24000 to #654000 have `g.copy` of about 519k to 524k. That is the 524,288 covering loan minus a few KiB, so the outer gate in `advance_original` passes.
- From #657000, `g = {items 64, copy 277151}` on fresh turns. That is the pinned Source's remaining lifetime stock (`fixture_renew_turn` → `renew_pinned_turn` → `retained_min(grant, cx.original_source_total_grant())`, `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:629-638`). The caller stock is `caller.json` `maximumCopyBytes` 528,482,304. About 504 MiB spread over about 650k futile attempts is roughly 800 B per attempt, which matches the per-scope admission and control fees.
- ed40 (fixed 64 KiB loan): the gate at `record/🦀️.rs:180` already fails, because 65536 < 277538.

## Exact mechanism

1. `OriginalRecordMemberSnapshotOpen::advance_original` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:180`) quotes `opening_demand` (277538). It widens `g` to cover that within `cx.retained_grant()`, which is about 520k, so it passes.
2. `receive_step` (`record/🦀️.rs:121-142`) opens an **unquoted** receiving scope with `received(...)`, which calls `cx.with_native_receiving_work` (`record/🦀️.rs:51-58`, `⏱️context/🦀️.rs:200-203`).
3. `original_receiving_outcome` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs:125-127`) prices an unquoted scope as `unquoted_receiving_grant(whole)`, which sets `maximum_copy_bytes: whole.maximum_copy_bytes.div_ceil(2)` (`🛬️owned/🦀️.rs:22-31`). With `whole` at about 520k the scope offers about 260k. The comment on `UNQUOTED_RECEIVING_COPY_BYTES` states the contract this violates: *"Larger atomic native copies must quote their work."*
4. Inside the scope, `available = child.restricted_by(offered)` is about 260k. All three `None` checks pass (`units == work`, `minimum_advance_items == 1` for an uninitialized receiver), so `receiver.advance(...)` runs.
5. `advance_once` (`🧳️original/🦀️.rs:84`) checks `let d=self.demands(..)?; if !retained_clone_grant_admits(rem,d){ … done(Ok(Progress(default))) }`. Here 277537 > 260k, so the step is idle with work 0. `advance` returns `Complete(Progress(0))`, `receive_step` returns `Default`, and `receive_request` stays `None`.
6. The scope was still admitted (`admit_scope` → `consume_retained(progress)`, plus the control charge in `finish_scope`), so the pinned Source stock shrinks by its fees every turn. When it falls below 277538, step 1 also fails. That is the permanent zero-progress state seen in the test ("127 of 127 records" is the archive's frame count; the member is stuck in `OpeningMembers`) and in the browser (`RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` on Stage(24)).

The same pattern applies to every caller of `with_native_receiving_work` whose next indivisible action is larger than half the turn. The record member open is the one this test hits.

## Minimal principled fix (no shim)

Follow the contract the framework already states: an atomic native copy that the automatic half-share cannot fund must **quote** its scope. The pattern already exists in `🔌️plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs:39-48`: `original_receiving_scope_quote(demand)` plus `invocation_extent`, then `with_native_receiving_quoted`. The record receive step skips it.

### 1. Framework context: a covering receiving scope with work

In `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (next to `with_native_receiving_work`, line 200) and `🛬️owned/🦀️.rs:125-127`, give the unquoted `ReceiveWithWork` scope an indivisible floor. Use the automatic share, raised on each axis to cover the floor, never above `whole`:

```rust
// ⏱️context/🦀️.rs (replaces with_native_receiving_work; every caller passes its next indivisible demand)
pub fn with_native_receiving_work(&mut self,out:&mut Option<OriginalNativeReceivingOutcome>,floor:RetirementDemand,operation:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant,u64)->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
    self.receiving.open(self.retained_grant()).map_err(OriginalTurnFailure::from)?;
    let mut operation=Some(operation);self.original_receiving_outcome(out,None,Some(floor),OriginalReceivingOperation::ReceiveWithWork(&mut |n,g,u|operation.take().unwrap()(n,g,u)))
}
// 🛬️owned/🦀️.rs:125-127
None => (covering_receiving_grant(unquoted_receiving_grant(whole),floor.map(original_receiving_scope_quote),whole), unquoted_receiving_work(self.fuel_remaining)),
/// 📐️ Raises the automatic share to an indivisible next action, never past the whole turn quantum.
fn covering_receiving_grant(share:G,floor:Option<G>,whole:G)->G{let Some(f)=floor else{return share};let up=|s:usize,f:usize,w:usize|s.max(f).min(w);G{maximum_items:up(share.maximum_items,f.maximum_items,whole.maximum_items),maximum_copy_bytes:up(share.maximum_copy_bytes,f.maximum_copy_bytes,whole.maximum_copy_bytes),maximum_capacity_bytes:up(share.maximum_capacity_bytes,f.maximum_capacity_bytes,whole.maximum_capacity_bytes),maximum_release_bytes:up(share.maximum_release_bytes,f.maximum_release_bytes,whole.maximum_release_bytes),maximum_depth:share.maximum_depth}}
```

`OriginalReturnScopePlan::quote` (`📦️stock/🎟️scope/🦀️.rs:66`) already returns `None` and charges nothing when the turn cannot afford `quantum + 2*control`. A floor that does not fit therefore costs no stock, which also stops the drain. Per CLAUDE.md (no legacy, fix all callers at once), every `with_native_receiving_work` caller passes its next demand: tool-run tick ops, the time-travel store-state, `document-archive-load/paged-parent.rs:96`, the typed-pack-load native step, `operation-wire/replacement:131`, and the record step. A caller with no indivisible action passes `RetirementDemand::default()`, which keeps the old behaviour.

### 2. Record step passes its exact next receiver demand

`record/🦀️.rs:51` `received` takes the floor. `record/🦀️.rs:121-124` `receive_step` computes it once, from the same quote `opening_demand` uses:

```rust
let (body,units)=self.receive_request.map_or((child.maximum_copy_bytes,cx.fuel_remaining()),|(g,u)|(g.maximum_copy_bytes,u));
let floor=receiver.demands(body,units)?;
let Some(issued_turn)=received(cx,floor,|native,offered,work|{ … unchanged … })? else{return Ok(Default::default())};
```

### 3. Structural follow-up (enum parity, removes the 271 KiB atomic action)

The struct derive (`🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs`, `NativeReceive` impl near line 110) should emit `receiver_template` as a per-field `NativeSeedTemplate::fields(..)` plan, exactly as `🔀️enum/🦀️.rs:64-72` does with `seed_field_size`/`write_seed_field`. Seeding a big snapshot receiver then becomes field-sized parts instead of one copy of the whole receiver. This keeps fix 1 general (any indivisible action), and fix 3 removes the largest such action. Fix 1 alone unblocks the test and the browser.

Not required: the pin relaxation in `receive_step` (`pinned.restricted_by(available)`). It is harmless but untested by this failure, because `receive_request` is never `Some` here. Remove the `[DEBUG] recvnone`/`recstep` prints in `record/🦀️.rs:129,219` once this lands.

## Test idea (TDD)

- **Language-agnostic fixture** `🛬️owned/🧪️tests/🧫️fixtures/covering-receiving-scope.json` with rows `{whole, floor, expectOffered, expectCharged}`:
  - floor below the half-share: offered = half.
  - half < floor ≤ whole − 2·control: offered = floor.
  - floor > whole − 2·control: scope denied, charged = 0.
- **Context law** (`🧵️job/⏱️context/🧪️tests`): drive `with_native_receiving_work(out, floor, …)` for each row. Assert the offered grant passed to the closure, and that `return_budget.remaining()` is unchanged on a denied row (no fee for a turn that cannot afford the floor).
- **Record law** (`record/🧪️tests/🦀️.rs`, which already builds `OriginalRecordMemberSnapshotOpen` with `opening_demand`): use a record type whose `size_of::<S::Receiver>()` R satisfies `R < turn_copy < 2R`. Drive `step` on fresh turns with a pinned Source. Assert Ready in bounded turns, and assert the stock fell by no more than the sum of the published receipts plus per-scope control (no futile-attempt drain).
- **Third-party cross-check**: the value test suite already compares against `serde_json` for the same records. Decode the same `.semio` member via `serde_json` into the same `S` and `assert_eq!` against the opened snapshot.
- **Regression**: `every_example_loads_through_the_member_less_archive_door` should settle all three examples. The browser `verify-process-firefox-bidi.ts` should no longer log `FEM cleanup semantic failure … stage=24`.

## Files read

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`, `…/🚪️open/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs`, `…/📦️storage/🦀️.rs`, `…/📦️storage/🌱️seed/🦀️.rs`, `…/🧩️record/📦️storage/🦀️.rs`, `…/🌳️value/🦀️.rs`, `…/🌳️value/🎟️reservation/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs`, `…/🔀️enum/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`, `…/🛬️owned/🦀️.rs`, `🧵️job/🛬️receiving/🎟️state/🦀️.rs`, `🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs`, `🧵️job/🏭️construction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`, `🔌️plugin/🪜️maintenance-ladder/🦀️.rs`, `🔌️plugin/🗃️document/🔁️store-replacement-admission-and-member-drive/🦀️.rs`
- `🔌️plugin/🧪️testing/🎟️caller/` (policy JSON), `…/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs`, `🔌️plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs`
- process3d editor test `🧪️tests/🔬️unit/🧩️contributions-and-examples/🦀️.rs`
