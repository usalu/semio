STATUS: PARTIAL. GREEN (testdebt-g-26, rc 1 overall because of the red crates below): value-retained-map lib 17/17, shared-issuance lib 16/16, retirement-owner 1/1, testing-presence-retirement (compiles, 0 tests). RED/UNVERIFIED: persistent (16/24 lib, 0/2 integration), edit (compiles, 3+ red), native-default (4/8 lib, 1/2 integration), ordered (several red; the set test blocks on the bun oracle queue and was killed). snapshot-read tests refused at source admission.

Logs: C/🗑️generated/1105b/testdebt-g-<n>.txt (11 retained-map green, 15/24 sealed, 16/17/18/19/22/23/24/25 compile triage, 26 last). Runners and edit scripts: C/1105b-tools/testdebt-g/ (t.sh, c.sh, m.sh, tx.py, txedit.py, bal.py).

## Production faults found by the laws and fixed (all claimed in C/1105b-claims.md)
- ordered-map: no close-work quotes on edit/clone cursors (ControlledRetirement refused every frame: 'retirement cursor lacks a work quote'); added close_work_demand across bounded-ord, lookup, insert, remove, StagedLease, clone cursor, EditRetirement.
- ordered-map clone cursor phases 2/4: child take handoff reported work 0 with a 41-byte progress.
- value-ordered UpdateCursor::next_copy_byte_demand: quoted copy 0 for the Entry/Node allocation phases that admit_shared_allocation (copy axis now checked) refuses below shared_allocation_value_demand. UNVERIFIED (compiled in g-26, ordered tests partly still red).
- persistent page_birth_demand quoted no copy for SealedShared page admission (admit_leaf/branch refused). Verified: the persistent lib went 9 to 16 passing.
- native-default array cursor phase 0: N==0 faulted the cursor, the real empty array could never be published. UNVERIFIED (edit after g-26 compile).

## Stale expectations changed (reasons)
- bounded-ord maximumTurnWork 1 to 3 (two held-alias binding re-checks plus the byte compare).
- staged birth copy total: constructor_copy_bytes is only the alias handoff (0); staged turns bill 248 placement bytes (STAGED_BIRTH_PLACEMENT_COPY_BYTES).
- sealed: borrowedReturn receipt copy now the real duplicate/return header copy; birth work refused 2, success 4 (= birth_work_demand, issuance + prepared birth); alias work 3; admission grants fund the complete admission_demand (admission_grant); native-child admission accepts only Ok(Some).
- per-axis next_*_byte_demand grants replaced by the selected event's complete quote (selected_grant).
- native-default laws 6 and 9 (interrupted/refused receipt) removed: the cursor no longer accepts receipts (the effect bridge owns that recovery); the missing-recipient law now also proves retry. Array null-allocation law split into refused_array_allocation_leaves_the_default_failed_until_its_paid_close (the cursor is failed, not retryable).

## Open (root cause, owner)
- persistent splice/set/clone quote omits SealedShared::duplicate_copy_bytes ('sealed alias handoff is not funded'); update/read debit law 'work <= 1' predates alias work 3; builder normal births now copy 952 bytes (law says 0); observation quote 128 vs payload page 4096. Owner: persistent / runtime2.
- edit: remaining red in commit and key-preserving transfer/receive tests (close quote above the 4096 body).
- ordered: owned-entry effect.rs:8 take_admitted==Some(false) and the controlled-insert 'exceeded its original work preflight' still red; ordered set test waits on the bun oracle queue.
- snapshot-read (port-a): test build refused, reason files, remainingFiles 0 at field-clone; needs a split, not raised.
- testdebt-h tickets quote (copy 0) vs terminal step (still requires and reports copy) briefly made every terminal close inert (retained-map 3/17 at g-24); green again at g-26.
- value-edit receive test and native-default edits after g-26 (marker assertion, close body 1<<16) are UNVERIFIED.

## Rulings received from runtime2 (not yet applied, next owner of persistent)
- persistent: every demand/next_demand/selected quote of a step that can reach SealedShared::try_duplicate (splice:26-28, root 🦀️.rs:125 projection, 🦀️.rs:198-199 leaf/branch handoff; set/clone alike) adds `duplicate_copy_bytes::<T>()` copy and `duplicate_work_demand()` work; update rows with that derivation, do not weaken try_duplicate. Builder 952 copy bytes: assert quote == size_of or fix the over-quote.
- snapshot-read: port-a splits the crate/test files (never raise the limit); refusal was reason files, remainingFiles 0 at field-clone.
- tickets: testdebt-h makes quote and terminal step one edit (0 on every axis, zero grant observes terminal).
- snapshot-read admission passes after port-a's merge (port-a-131); its test target is blocked by a peer private-field error in persistent clone/🦀️.rs:12 (not an edit of mine).
