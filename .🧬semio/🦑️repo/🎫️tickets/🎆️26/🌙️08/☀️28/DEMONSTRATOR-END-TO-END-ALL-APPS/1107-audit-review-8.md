# 🔎 1107 Audit, Lane review-8 (read-only)

Scope: the seven ported crates, read by hand against `1107-brief.md` rulings W1 to W4, R1 to R7 and the single-charge rule. No cargo, no edits, no git writes, no sub-agents. Nothing was compiled or run, so every finding below is a reading. Confidence is stated per finding.

## Coverage

| Crate (`Cargo.toml` name) | Read | Not reviewed |
|---|---|---|
| `semio-framework-actor-receipt` | root codec, `⚠️failure`, `🌱️bootstrap`, `💼️job`, both `tests/*.rs` | `🌱️bootstrap` ledger only checked for write/read symmetry and the work sums, not run |
| `semio-framework-actor-job-state` | whole crate | nothing material |
| `semio-plugin-result-page` | lib root, `📃️pending`, `📃️wire` | `mounted`, `producer`, `emission` (other crates) |
| `semio-ui-draw-contract` | located at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📦️packages/🦀️rust/Cargo.toml`; not reviewed (ran out of audit budget) | all |
| `semio-ui-runtime-presentation` | lib root, `📦️resources/📤️transport/📨️message` (mesh receiving) | `🎛️context`, `🎯️dispatch`, `🪪️entity`, `🕸️tracking`, `🎭️present` only grep-scanned for panic sites, nothing found |
| `semio-s-fem-analyses-assembly` | `🔶️csr-build-and-run` (close and funded step), `🪡️element-build` (close ladder, pending element), `🧷️job-state-and-construction` (step and validation stages) | `🧪️tests`, most of `🪡️element-build` body |
| `semio-s-fem-sparse-ldlt` | `🚀️run-and-close`, `🔄️restore-cursor`, `🔩️factor-and-job` (constructor, reservation, column microcursor, close), and the shared `🤝️numerical-protocol` helpers they call | `🧪️tests`, `solve*`, `paged-ldlt` |

Skipped: `semio-ui-draw-contract` was located late and was not read. The presentation entity, dispatch, context and tracking modules were grep-scanned only. `HeapSortCursor::next/advance`, `close_paged_owner_step` and `PagedList` reservation internals live outside the crates and were taken from their call sites.

## Findings, ranked

### 1. Receipt readers reject every non-empty escrow extent the publisher writes (high, definite by reading)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/🦀️.rs:43`: `valid_escrow` accepts `length` in `48..=5936` with `(length-48)%184==0`. That matches the publisher's row size.
- Same file, `:66` (`OriginalActorFixedReceipt::read`), `:71` (`borrowed_scalar_escrow`), and `:97` through `:99` (prefix and body validators via `read`): the escrow check is `(48..=4400)` with `(length-48)%136==0`.
- The publisher is `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🦀️.rs:5`, `ROW=184`, `MAX=48+32*184=5936`.

Failure: a writer emits a receipt with one row, so `length=232`. `write_to` accepts it (`(232-48)%184==0`). `read` rejects it (`184%136=48`). `validate_original_source_header_body` therefore rejects a receipt the same crate just wrote. The only length that passes both checks is 48 (zero rows) or multiples of 3176. The tests in `📦️packages/🦀️rust/tests/actor-receipt.rs` only use `length:48`, so they never see this.

Fix direction: one shared constant for the escrow row stride and maximum, used by writer, reader and the publisher.

### 2. LDLT and numerical close turns convert owner errors into `Pending` forever (high for liveness, definite by reading)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🤝️numerical-protocol/🦀️.rs:79` to `:83`: `numerical_owner_turn`. `Err(()) => Some(Pending{progress: default})`. The error is erased and the job reports `Pending` with zero progress.
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🔩️factor-and-job/🦀️.rs:180` to `:186` (`close_ldlt_checkpoint_step`) propagates `Err(())` from every owner. `:581` to `:585` (`close_retained_step`) and `:608` to `:610` (`LdltFactor::close_step`) map `Err(())` to `(false,0,0)`, which `numerical_ladder_turn` turns into `Pending{0}`.
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🔄️restore-cursor/🦀️.rs:154`: restore close calls `numerical_owner_turn(close_ldlt_checkpoint_step(...))`.
- Same pattern in the assembly ladder: `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🪡️element-build/🦀️.rs` (`close_retained_step`, `Err(()) => return (false,0,0)` at every owner, for example lanes 0, 1, 2, 3).

Failure: an owner's `close_paged_owner_step` returns `Err(())` on a deterministic condition. Every close turn returns `Pending{0}` and the close never reaches `Complete`. Nothing reports the fault and nothing returns `Blocked`, which violates W4. The ladder never advances past a failing owner.

Fix direction: map `Err` to a typed refusal or `Blocked{reason}`, and keep the owner's error.

### 3. Close turns charge a constant `work: 1` and charge it even when nothing ran (medium, definite by reading)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🤝️numerical-protocol/🦀️.rs:73`: `numerical_close_original` returns `RetainedWorkOutcome { result, work: 1 }` regardless of what the turn performed. The turns (`close_turn`, `close_granted`, `close_granted` in restore-cursor) call grant-shaped owner closes that do not expose performed work. So the real CPU work of a close is not charged; a fixed one unit is charged instead. This is the pattern W2 forbids ("constant work:1 is honest only for a call that does exactly one action").
- Also at `:73`: when the gate refuses (`numerical_close_gate` false, LDLT run-and-close `:134`, restore-cursor `:148`, csr `:230`), the turn returns `Pending{0}`, which is mapped to `Progress(0)` with `work: 1`. A unit is debited for a turn that did nothing.
- Mesh receiving close uses the same shape: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🦀️.rs:96` (`close_step`) does `cx.consume_fuel(1)` around `destination.owner.close_step(grant)` and `receiver.close_step(grant)` without measuring what the owner did.

Fix direction: take the performed work from an effect (`with_original_effect`), and report zero when the gate refuses.

### 4. LDLT reservation drops the work of a denied or failed reservation (medium, definite on the `Denied`/`Err` branches; amount depends on whether `reserve_capacity_one` can perform work before denying)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🔩️factor-and-job/🦀️.rs:261` to `:267` (`reserve_extent`). `performed` is recorded only on the `Ready` branch via `receive_original_retained_work`. `Ok(Denied) => return Ok(false)` and `Err(_) => return Err(DimensionMismatch)` skip the receive. `with_original_effect` has already committed `performed` to the authority (`performed != 0 → commit`), so the work is spent on the authority side but never debited from fuel. The error is also flattened to `DimensionMismatch`.

Fix direction: receive `RetainedWorkOutcome{result, work: performed}` on every branch that ran the effect, and keep the error.

### 5. Result page: zero-work `Ok(false)` and `Progress(0)` returns when a grant cannot fit, with no refusal (medium, liveness; caller-dependent)

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:102` (`write_encoded`), `:134` (`write_raw_step`): `!fits(demand, grant)` returns `Ok(false)` with `work:0` and no progress.
- `:166` (`close_step`): `!fits(d, grant)` returns `Progress(default)` with `work:0`.

`fits` requires `maximum_items>0` and copy, capacity, release and depth bounds. A grant that cannot ever fit the demand returns the same answer every turn. The caller that loops on `Ok(false)` or `Progress` never gets a typed refusal. This violates W4 (close must not stay pending forever) and W2 (no idle retry). Whether it is reachable depends on the caller's grant, which is in `mounted/` and not reviewed.

### 6. Job state: a close can wait forever on an unacknowledged borrowed chunk (medium, conditional)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/🏠️state/🦀️.rs:19`: `close_output_slots` returns `Progress(default)` while `job_output_chunk_borrowed` is set.
- `:20`: `close_on_source` takes the `has_output()` branch (which includes the borrowed flag) and returns `Ok(false)` without ever reaching `provider.close`.
- Only `acknowledge_chunk_on_source` (`:16`) clears the flag. It returns `Ok(false)` when `retirement_should_yield` or the grant is zero. `read_on_source` (`:41`) refuses while borrowed.

Failure: a consumer reads a chunk with `retain=true` and then goes away (turn cancelled, provider dropped the job). Nothing clears the flag, so the close loops with zero progress forever, and the job can never reach `provider.close`. Cancellation does not bypass this, because the branch order in `close_on_source` checks output before cancellation.

### 7. Job state: a zero copy or capacity grant makes the read return `Pending` with zero work (low to medium, conditional)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/🏠️state/🦀️.rs:42` checks only `maximum_items` and `maximum_depth`. `:48`: `count = min(remaining, grant.maximum_copy_bytes, grant.maximum_capacity_bytes)`; `if count==0 { return Ok(pending(...)) }` returns with no work and no progress. A turn whose remaining copy or capacity is zero therefore yields without making progress. If the next turn is handed the same zero grant, the job livelocks. This is a `Pending{work:0}` loop by W2's definition, and the brief's own failure class.

### 8. Job state: close charges `copied_items` as fuel on the cancel signal (low, not a double charge)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/🏠️state/🦀️.rs:20` and `:44`: `cx.consume_fuel(progress.copied_items as u64)`. The fuel debit is an item count, not measured work. `consume_retained` does not debit fuel (checked in `🧵️job/⏱️context/🦀️.rs:254`), so this is not a double charge, but the fuel is not an honest quote.

### 9. Result page: panics and unchecked slices on reachable inputs (low to medium)

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs:137` to `:138`: `write_raw_step` slices `bytes[self.written..self.written+count]` with no check that `self.written <= bytes.len()`, and calls `.expect("quoted initialized raw backing")`. `write_encoded` (`:112`) checks this with `checked_sub`; `write_raw_step` does not.
- `:104` and `:150`: `self.plan.unwrap()`. `ready_to_handoff` (`:76`) does not check `plan.is_some()`, so the invariant lives only in the callers.
- `:171`: `Drop` asserts unless the page is terminal. Dropping a page that holds a plan panics.
- `:120`: `u64::try_from(turn.work).unwrap_or(u64::MAX)` saturates to the full fuel width instead of failing. Low.

### 10. Receipts: asserts on writer inputs and a close flags asymmetry (low)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/🦀️.rs:53`, `:54` and `:60`: `assert!` on caller data (`InputAdmission` requires `ready ⇒ input_returned`; `TurnOutput` must not reach `write`). Reachable from callers in the same crate only through their own wrong inputs, but they panic rather than refuse.
- `:60` writes `Close{complete:true, blocked:true}` as flags `3`. The reader `:66` (`2 if … flags<=2`) and the prefix validator `:97` (`2=>flags<=2`) reject `3`. Writer accepts a value its own reader rejects. Either the writer must refuse it or the reader must accept it.

### 11. LDLT: `expect` on reachable paths and an error discarded at the step (low)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🚀️run-and-close/🦀️.rs:29` and `:62`: `expect("finished numerical writer retained its original payload")`.
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🔩️factor-and-job/🦀️.rs:569`: `expect("published factor column")`. `:621`, `:622`, `:631`, `:632`: `expect` in `LdltFactor::solve*`.
- `…/🚀️run-and-close/🦀️.rs:101`: `if self.advance_column_microcursor(context).is_err() { return Ok(NumericalRun::Fault) }` discards the `SparseError` (including `ZeroPivot{column}`) and reports a generic fault.
- The AGENTS rule bans `.expect()` in non-test code, so these are rule violations even where unreachable.

### 12. Assembly: an `expect` and a modulo by a possibly empty partition list (low)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🪡️element-build/🦀️.rs:543`: `expect("pending element exists")` in `assemble_cell`.
- `:556`: `pending.element_index % self.state.partitions.len()`. Division by zero if `partitions` was freed by the close ladder (lane 1 sets `partitions = Vec::new()` at `:241` and `:291`) and `assemble_cell` runs afterwards. The current state machine should prevent this; I did not prove it.

### 13. Assembly: close ladder can stall on a partition buffer larger than the release grant (low, conditional)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🪡️element-build/🦀️.rs:241`: if `partitions.capacity()*size_of::<AssemblyPartitionBuffer>()` exceeds `maximum_bytes`, the turn returns `(false,0,0)` every time. `close_frontier` does not include the partitions vector, so the gate admits the turn and the ladder never progresses. Small in practice (worker count times a small struct), but it is a close that can stay pending under a fixed release grant.

### 14. Assembly construction: per-turn O(N²) and worse validation, and a wallet inside a funded path (medium, performance and rule)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🧷️job-state-and-construction/🦀️.rs:653` (`assembly_validate_node_pairs`): one node pair per turn across all N² pairs. For N = 10,000 nodes that is about 5×10⁷ turns. A `HashSet` would be O(N).
- `:672` (`assembly_validate_element_references`): for each element and each mounted reference, one scan over all nodes per turn. That is O(E·R·N) turns.
- `:741` (`assembly_mark_constraints`): for each support and fixed DOF, one linear scan of `dof_map.order` per turn, O(S·D·ndof).
- Each turn does one comparison and is charged one unit, so the charge is honest per turn but the total is quadratic or worse in model size.
- `:409` to `:415` (`step_one_funded`): `AuthoringWallet::new(grant, 64).run(grant, 64, …)`. W1 forbids a wallet on a path reachable from a turn, and W2 forbids a fixed 64-unit constant. Callers are outside this crate and were not checked; if any caller holds a `cx` or effect, this is an uncharged wallet.
- `:418` to `:421`: a `Denied` reservation returns `Ok(default)` and `:468` returns `(false, 0)`. With zero capacity granted, this repeats forever with no refusal (W4).
- `…/🔶️csr-build-and-run/🦀️.rs:321`: `case_rhs_old` is named as a legacy path. AGENTS forbids legacy support. Callers were not located in this pass (a repo-wide grep timed out).
- `…/🔶️csr-build-and-run/🦀️.rs:332`: `case.member_loads.iter().find(...)` per element, O(E·L). Low.

### 15. Assembly CSR close: publication dropped without its own close (low, conditional)

- `/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🔶️csr-build-and-run/🦀️.rs:240`: `else if self.publication.take().is_some()` drops the publication when `publication.close_step(grant)` returned `None`. Correct only if `None` means terminal. `close_demand()` is defined in another module and was not checked. If it can be `None` with owned data still present, the drop leaks it, and the `Drop` assert on the owner panics.

### 16. Mesh receiving: a whole name must fit in one turn's copy grant (medium, conditional, chunk-vs-cap violation)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🦀️.rs:80`: `let count=body.len().min(grant.maximum_copy_bytes); if count==0 {return Ok(false)}`, so a zero copy grant returns with no progress.
- `:85`: when the name is complete, `if grant.maximum_copy_bytes<extent {return Ok(false)}`. The whole name (up to `UI_TEXT_MAX_BYTES`, a setting in `🧬️contract/📏️budget/🦀️.rs:17`) must fit one turn's copy grant, or the packet never completes. This is a cap, not a chunk, which W2 forbids.
- `:71` and `:96`: `receive_step` and `close_step` return `Ok(false)` with zero work when a grant is too small (`:96` path for `destination.owner` and `receiver`). Same W4 concern as finding 2.

## Things checked and found consistent

- `actor-failure` (`⚠️failure/🦀️.rs`): write and read agree for all five statuses, flags and refusal tags. No fault found.
- `actor-job` (`💼️job/🦀️.rs`): write and read agree for every status. No fault found.
- Bootstrap ledger (`🌱️bootstrap/🦀️.rs`): word counts line up (`WRITE_OPERATIONS=38`, `EXTENT=304`, 37 read words), the prepaid and fresh arithmetic is consistent, and the handback is single-observed (`flags[1]` 1→2). The reserve quote `first_bootstrap_response_reserve` was not checked against the birth reserve beyond the test assertions.
- Job-state `close_owned_actor_slot` (`:14`): the single charge comes from `charge()` and the nested step's `performed_work()`. No double charge found.
- `with_original_effect` and `consume_work` in `🧵️job/⏱️context/🦀️.rs:263` and `:410`: the one debit is `consume_work` at the caller, which matches the single-charge ruling.
- LDLT column microcursor (`factor-and-job` `:331` to `:525`): every `admit_work` precedes its mutation and every reservation is chunked one owner page per turn. No work double charge found. Reservation via `push_reserved` is charged through `admit_work` (retained only), with the one fuel unit per turn coming from `run()`.
- LDLT restore cursor: sticky fault, identity check, and `expected_field` ordering are consistent.

## Top finding

Receipt readers reject every non-empty escrow extent the publisher writes (`.../📤️receipt/🦀️.rs:43` accepts a 184-byte row stride, `:66` and `:71` require 136), so a receipt with one row fails its own validator.
