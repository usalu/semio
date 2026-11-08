STATUS: GREEN for what ran. Native: actor-receipt full crate (lib 2 + actor-bootstrap 4 + actor-receipt 6 tests, reviewfix-622 rc 0); fem sparse-ldlt/assembly/subspace/pcg check (612 rc 0); ui-draw-contract check (614 rc 0); result-page check (615 rc 0, finding 9 now compile-verified). Wasm32-wasip2: fem ldlt+assembly (613 rc 0). Not run: wasm for receipt/draw/result-page, fem tests, any runtime test of findings 2/3/4/11/12 (compile only, no failing-before law).

# reviewfix6: triage of 1107-audit-review-8

Logs in `C/🗑️generated/1105b/`: reviewfix-603 (actor-receipt `--test actor-receipt`, rc 0), reviewfix-605 (check fem sparse-ldlt + assembly native, rc 0), reviewfix-606 (result-page, rc 1 upstream). reviewfix-602 shows the sibling test target `actor-bootstrap` fails to compile (`clock`, `epoch` field), peer breakage not in my files. Native only: no wasm check was run.

| # | Verdict | Evidence / action |
|---|---|---|
| 1 | REAL, VERIFIED | Reader strides were 136 (`4400`), publisher and `OriginalEscrowSnapshot::read` use 184 (23 u64 words = `size_of::<OriginalEscrowRow>()`). One-row length 232 was accepted by `valid_escrow`/`write_to`, rejected by `read` and `borrowed_scalar_escrow`. Now `ORIGINAL_ESCROW_{HEADER,ROW}_BYTES/MAX_ROWS/MAX_BYTES` + `original_escrow_length_valid` live in `job/📬️outcome/📤️publication/🪙️escrow/🦀️.rs`, re-exported by `actor/🎟️retained-turn/📤️receipt/🦀️.rs`; writer, both readers and the snapshot parser derive from them. Law: `escrow_extent_laws` in `receipt/📦️packages/🦀️rust/tests/actor-receipt.rs` round-trips 0..=32 rows through write, `read`, `validate_original_source_header_body`, `borrowed_scalar_escrow` (fails on the old 136 readers for every rows>0) and pins stride to `size_of::<OriginalEscrowRow>()`. |
| 10 | REAL, VERIFIED | `Close{complete,blocked}` both true wrote flags 3, reader rejects; production never produces it (`borrow_original`: complete needs Ok, blocked needs Err). Writer now refuses it. The three `assert!`s on caller data (`write` borrowed/TurnOutput, ready-without-returned) are typed refusals. Law: `writer_refuses_instead_of_panicking_...`. |
| 2 | REAL, PARTIAL, compiles (reviewfix-605) | `numerical_owner_turn` mapped `Err(())` to zero-progress `Pending`. Now `Blocked`, and `numerical_close_original` turns `Blocked` into a typed `InvariantViolated` refusal (all tests assert "close cannot block", so this is the intended vocabulary). NOT fixed: the tuple-shaped `close_retained_step` ladders (`(false,0,0)` on `Err(())`, ldlt/csr/assembly/subspace/pcg via `numerical_ladder_turn`) still erase the error; needs the tuple to become a `Result`, a cross-crate signature change left to `statik`. |
| 4 | REAL, compiles (reviewfix-605) | `reserve_extent` debited `performed` only on `Ready`. Now debited whenever `performed != 0` or ready, on Denied and Err too; the Err still maps to `DimensionMismatch` (type erasure kept). No failing-before test (needs an allocator-failure double). |
| 9 | REAL (rule violation), FIXED-unverified | `write_raw_step` `expect` + unchecked slice, and two `plan.unwrap()` in `📃️pending` replaced by refusals. |
| 11 | PARTLY REAL | Fixed: two `expect`s in ldlt `run-and-close` (now `Fault`), `expect` at `factor-and-job:569` (now `ValueError`). Open: `expect`s in `LdltFactor::solve*` (infallible public API, ripples) and the discarded `SparseError` at `run-and-close:101`. |
| 12 | REAL (rule), compiles (reviewfix-605) | `assemble_cell`: `expect` and `% partitions.len()` now `FemError` (`checked_rem`). The only caller already treats `Err` as fault. |
| 3 | REAL, OPEN | `numerical_close_original` charges constant `work: 1`; fix needs the grant-shaped owner closes to report performed work (effect-shaped signature), a cross-crate change. Mesh `close_step` same. |
| 5 | CONDITIONAL, NOT CHANGED | `fits`-false returns are the demand-quote handshake (callers read `write_demands`/`close_demands` first and size the grant); no in-crate caller loops on it. Reachability depends on unreviewed `mounted/`. |
| 6 | FALSE POSITIVE (by design) | A retained chunk is a physical loan: the consumer holds a pointer into it, so close must wait for the hand-back (`acknowledge_chunk_on_source`); freeing it earlier would be unsound. A host that never hands back is a host fault, not a state-machine one. |
| 7 | FALSE POSITIVE (by design) | Zero copy/capacity grant is the same quote handshake (`output_demands` quotes `copy_bytes: 1` for pending output); the caller must size the grant. |
| 8 | LOW, NOT CHANGED | Item-count fuel is a flat unit per step; no double charge (auditor agrees). |
| 13, 14, 15 | NOT TRIAGED IN DEPTH | 13 (partition buffer over release grant) and 15 (publication drop) conditional and unproven even by the auditor; 14 is a performance rewrite (HashSet validation) outside this lane's compile budget. `case_rhs_old` legacy name left for `statik`. |
| 16 | FALSE POSITIVE (by design) | A name is one `UiText` (`<= UI_TEXT_MAX_BYTES`, a setting): an atomic fixed-capacity value, validated and built in one step; the chunked path is the byte copy above it (`body.len().min(grant)`), which is already chunked. |
| draw-contract | NOT REVIEWED | |

Open: result-page compile once the replication peer breakage is fixed; 2 (tuple ladders), 3, 11 remainder, 13-15 as above.

## Round 2 (coordinator follow-up)

Chain script `C/1105b-tools/reviewfix6/chain.sh` runs 611 (actor-receipt all tests), 612 (fem sparse-ldlt, assembly, subspace, pcg native), 613 (fem ldlt+assembly wasm32-wasip2), 614 (ui-draw-contract), 615 (result-page); logs `C/🗑️generated/1105b/reviewfix-61{1..5}.{txt,rc}`. Verdict lines for these runs are appended below when they finish.

| # | Verdict | Change |
|---|---|---|
| 2 (tuple ladders) | REAL, FIXED-unverified | Single point: `numerical_ladder_turn` maps a turn that neither completes nor progresses (`(false,0,0)`, which every owner `Err(())` and every too-small-grant return produced, and which no productive turn can produce) to `Blocked`; `numerical_close_original` turns `Blocked` into a typed `InvariantViolated`. Covers ldlt, csr/assembly, subspace, pcg. The owner's own error kind is still not carried (`()` at source). |
| 3 | REAL, FIXED-unverified | `numerical_close_original` now charges work 1 only when the turn completed or made progress, 0 for a gate refusal. Honest because each ladder turn performs exactly one owner action. Mesh `close_step` (`fuel(1)` around owner closes) left: it is a different crate family and its owners expose no performed work. |
| 11 | REAL, FIXED-unverified | `LdltFactor::solve*`: `expect`s replaced by `map_while(column_entry)` (same iteration, no panic). Discarded `SparseError` at `run-and-close:101` is reported as `NumericalRun::Fault` (not silent); `Fault` carries no payload, left. |
| 13 | CONDITIONAL, now surfaced | partitions `Vec` release `> maximum_bytes` returned `(false,0,0)` forever; under fix 2 it is a typed refusal. Whether `close_frontier` omits it is unproven; the quote is the fixed `NUMERICAL_CLOSE_RELEASE_BYTES` page, far above `workers * size_of::<AssemblyPartitionBuffer>()`. |
| 14 | PERF REAL, DEFERRED; wallet FALSE POSITIVE | `step_one_funded` (wallet) is called only from tests: a no-turn entry point, allowed by W1. The O(N^2) node-pair, O(E*R*N) reference and O(S*D*ndof) constraint scans are real but replacing them needs a funded sorted index owner and new construction stages, a redesign not safe to compile-prove in this wave. `case_rhs_old` legacy name: not touched. |
| 15 | FALSE POSITIVE | The branch is reached only when `publication.close_step` returned `None`, which `numerical_writer_close` returns only for an empty writer slot (`slot.as_mut()?`), so the dropped publication owns nothing. |
| draw-contract | REVIEWED | Work accounting is outside this crate (`retire_step_checked` is a one-action step, the caller charges). Fixed: `.expect` in `retire_step_checked` (now `Err(PagedListError InvariantViolated)`), guarded `unwrap` in `retained-append:122`. Still present, not changed: `layers.last_mut().expect("layer")` after push (queries-and-push:41), `shared-border-finalize` `expect/unwrap` at 126, 130, 138, 139, 208, 285-305 (their `retire_step()->bool` has no error channel), `shared-border-clip:137`. These are guarded invariants; removing them needs signature changes in the border cursor. |
| actor-bootstrap target | FIXED-unverified | Test only: `clock` helper added to the second test's module; `RetainedTurnInput` epoch/grant split adopted (`budget.input.grant = ...remaining; epoch += 1`). |


### Round 2 results
- 611 failed on the bootstrap lib-test source (`bootstrap/🦀️.rs:78`, same epoch/grant split); fixed. 621 then failed at runtime because `return_original` now returns the grant and `remaining_fuel` separately; all four bootstrap sites now set `input.grant` and `input.work.fuel` (= `remaining_fuel`) and bump the epoch. 622 passes. The `actor-bootstrap` target (`reviewfix-602`) is fixed and green.
- Finding 9 (result-page `unwrap`/`expect` to refusals): compile-verified (615).
- Replication `history-index` was no longer blocking (615 got through).
- Fixes 2, 3, 11, 12, 4 have no new failing-before test; they are compile-verified only. A law for 2 would need a close-owner double that refuses; not written.
- Counts: real+fixed 1, 2, 3, 4, 9, 10, 11, 12; real+deferred 14 (perf), part of 2 (owner error kind not carried), 11 (SparseError payload), draw-contract finalize/clip expects; false positive 6, 7, 15, 16, 14-wallet; conditional 5, 8, 13.
