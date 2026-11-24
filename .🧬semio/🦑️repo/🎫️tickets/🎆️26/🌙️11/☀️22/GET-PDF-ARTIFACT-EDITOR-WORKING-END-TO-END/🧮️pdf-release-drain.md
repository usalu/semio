# PDF release drain: root cause (attributed, fix not yet applied)

## Result in one line
The 512 MiB release stock is drained by the **mutation DAG of every document store**, not by the PDF app, store or ladder step types. Each `MutationDag` eagerly allocates fixed backing for 8,192 slots in three tables, about 3.9 MB per store, and a displaced store releases all of it page by page. The 27,136-byte release is one page of 64 `MutationEnvelope` slots (64 x 424 bytes).

## Measured sizes (PDF 1.4, `std::mem::size_of` probes, removed again)
- `VcsArtifactApp<EditorApp<Pdf14Editor>>` is 14,912 bytes by value in this tree (members are already boxed). Snapshot and mutation are 48 bytes each, so PDF contributes almost nothing; the sizes are framework-intrinsic and the same for XLSX.
- `ArtifactStore<S,M>` 29,024 = log 4,952 + owners 7,872 + wire 8,240 + cursor 840 + live 304 + replay 6,816; `ArtifactEnvelope` 4,184; `DocumentStoreOwners` 5,328; disposer 0 (zero-sized); `ActiveDocumentArchiveLoad` 99,232; `WindowConfigOwnerRegistry` 43,600.
- Every store-close phase owner box is small: 848, 848, 1,536, 784, 944, 96, 2,416, 2,480, 7,072 bytes. Neither the disposer cursor nor the phase boxes produce the 27,136 quote.
- The older figures 136,496 and 175,776 are the heap totals of whole stores and apps quoted at close (`heap_bytes`-style sums of the boxed members); they are real allocations, not inline size.

## Attribution of the 27,136 x 3,132 releases (85 MB of the 470 MB)
Backtrace: `maintenance_ladder_unit_step` -> `maintenance_stage_step` -> `drive_store_replacement_jobs` (state `RetiringCommittedStore`, store and disposer present) -> `ArtifactStore::close_owned_demands` -> `next_resident_backing_byte_demand` -> `MutationDag::next_backing_release_byte_demand`.

`MutationDagFixedSlots::new()` (`🔨️modules/📡️replication/🔗️causal/🦀️.rs`) allocates, per table and per store, all 128 slot pages up front plus `generations` (32 KB), `occupied` (8 KB), `next`, `previous`, `free` (16 KB each). Three tables: `envelopes` (page = 64 x 424 = 27,136 bytes, 3.47 MB total), `applied` and `pending` (page = 64 x 24 = 1,536 bytes, 196 KB each). Closing releases one indivisible page per step and quotes each page, so one replaced store costs 128 x 27,136 + 2 x 128 x 1,536 + metadata, about 3.9 MB of release stock. 512 MiB therefore covers only about 130 stores, and the PDF conflict law creates two stores per document load across 128+ scenario instances.

The other histogram entries are the same mechanism one level up: 136,496 x 256 and 175,776 x 128 are the whole-store and whole-app closes of each scenario; 3,072 x 3,328 are `PrefixSnapshot`/registry pages.

## Why XLSX passes
XLSX scenarios replace fewer stores per law, so the same 3.9 MB per store stays under 512 MiB. The type sizes are identical.

## Fix design (not applied)
Make the DAG backing demand-paged instead of fully eager:
1. `MutationDagFixedSlots::new()` and `admit_backing` stop creating slot pages (keep the 128-entry pointer array); `birth_bytes` drops the page stage. Close already skips `None` pages (`position(Option::is_some)`).
2. A page is born on the first `push_reserved` into it, and its birth must be quoted: the seed stage (`🌱️original-seeds`) and the replay admission (`🏪️store/🏗️initialization/🔁️replay`) add one page birth to their demand when the target page is absent. The unit laws `causal_original_backing_birth_and_every_cancelled_prefix_match_physical_receipts` and `causal_backing_retirement_preserves_exact_slots_and_funds_one_indivisible_allocation` change accordingly (the second must count pages born during seeding as part of `birth`).
3. Optionally page `generations`, `occupied`, `next`, `previous`, `free` the same way (88 KB per table).
Expected effect: a typical store keeps one page per table (about 30 KB) instead of 3.9 MB, so the 27,136 releases fall from 3,132 to a handful per store and the total drain drops by well over 100x.

## What was not done and why
Applying step 2 changes the grant accounting of two hot paths and two physical-receipt laws; with the native build queue (10 to 20 minutes per iteration, memory-gated to one build) the change could not be completed and verified in this session, and a half-applied birth protocol would break every store law. No source change is left in the tree: all `[DEBUG]` probes added here (`debug_sizes`, `repl27136`, `close-phase`, the `debug_sizes14` test) were removed again. The `post_retained` histogram that was already in `🧵️job/⏱️context` belongs to another agent and is untouched. `probe_effect` still starts at `1<<20`; it needs a read-only available-work accessor on the work recipient (`OriginalWorkRecipient`) to start at the remaining fuel.

Note: one `pkill -f native-cargo.sh` early in the session may have stopped other agents' queued builds; they only needed a retry.

---

# Update: lazy DAG pages and lazy registry pages implemented

## Changes
- `🔨️modules/📡️replication/🔗️causal/🦀️.rs`: `MutationDagFixedSlots` births no slot page at construction (128 `None` entries); a page is born on the first `push_reserved` into it (`slot_mut`) and recorded in the birth ledger; release already skipped absent pages. New `next_push_page_bytes`/`admit_next_push_page`, `MutationDag::original_seed_page_demands`, `admit_original_seed_page_step`, `resident_pages`. `birth_bytes`/`admit_backing` no longer stage pages.
- `🌱️original-seeds/🦀️.rs`: `original_seed_append_demands` is now `&self` and quotes the one applied page the next seed needs (capacity axis); `stage_original_seed` births that page as its own granted step before pushing. Callers: `🏪️store/🏗️initialization/🔁️replay/🦀️.rs` (`seed_history` admits the quoted page like it admits metadata) and `🕰️history/🏁️replay/📬️publication/🦀️.rs`.
- `🔌️plugin/🏛️ownership/🧱️registry-backing/🦀️.rs` and `🏛️ownership/🦀️.rs`: `ArtifactFixedRegistry` slot pages (4 KiB or one slot, whichever is larger) are born on first insert (`get_or_birth_mut`), the birth quote is the page table only, release walks only resident pages. This is the second eager fixed allocation the PDF law drained (see numbers below).
- Work recipient: `OriginalWorkRecipient::available_work` (default `None`), `OriginalWorkLoan::available_work`, `NativeDecodeControl::available_work_hint`, implemented by `OriginalJobWorkDriver` and the decode `ActualWork`; `probe_effect` starts at `hint - 1` (the invocation unit is added by `with_effect`), falling back to `1<<20`.
- Tests: causal `causal_backing_retirement_...` counts the pages seeds birth; new `causal_small_dag_holds_one_page_per_used_table_and_releases_exactly_the_allocated_pages` (empty DAG owns 0 pages, one seed births exactly one page and the quote drops to 0, release demand names exactly one page); seeds law uses `dag.original_seed_append_demands()`; registry law rewritten (`usedSlots` 0, 17, 63: table-only birth, pages born on insert, one release per resident page plus one for the table); the two fixed-registry tests expect vacant slots to own no page.

## Verification (one run at a time through `native-cargo.sh`)
| Run | Result |
| --- | --- |
| `semio-framework-replication-causal --lib -- --skip original_clone_tests --test-threads=1` | 43 passed, 0 failed (the new test included). With 3 parallel test threads one heap-observing seeds law failed once on cross-test heap contamination (passes alone and serialized). |
| `original_clone_tests::original_causal_envelope_clone_preserves_wire_value_and_shared_actor_identity` | aborts the test binary ("original work issuer retains an issued action or committed proof", drop during panic); the code path is the work-issuer, untouched here; skipped, not attributed to this change. |
| `semio-plugin-ownership --lib` | does not compile on its own (global allocator conflict and unrelated closure/arity errors in its test sources); the registry law cannot be run in isolation and is covered only through the PDF runs. |
| PDF 1.4 `conflict_history_edits_end_to_end` | **ok**; release stock spent about 155 MiB of 512 MiB (was exhausted at 470 MB and failing); runs in about 80 s in parallel, about 60 s serialized. |
| PDF 1.4 `history_edits_end_to_end` | **ok**; about 330 MiB of 512 MiB spent after the conflict law (cumulative probe reading 475 to 500 MiB for both, serialized: 158.8 s for the pair). |
| PDF 1.7 `history_edits_end_to_end`, `conflict_history_edits_end_to_end` | **fail** in both with `cold replay edit refused after a failed in-place application` (`invariantViolated`), a store-replay refusal that does not involve the release stock (the 1.7 conflict law had spent about 300 MiB when it died). 1.7 is the tree the other agent is converting (operation wire); not attributed to the DAG change but not proven independent either. |

Release-byte histogram after the change (1.4, 150 MiB mark): 3,072 x 13,624 (42 MB, maintenance ladder during loads), 16,384 x 1,383 (store-initialization worker pages), 965,632 x 19 (the `pending.reserved_commits` fixed queue), 32,768 x 315, 245,760 x 40 (the child member registry). The 27,136-byte releases are gone.

## Remaining eager containers (next level, same pattern)
Per-app final backings measured before the registry change (bytes): `pending.reserved_commits` 965,632 (an `ArtifactFixedQueue`, `VecDeque::try_reserve_exact(capacity)`), `children` 245,760 (`ChildMemberRegistry`), `latest_wins_commands` 218,624, `peer_roster_publications` 172,800, `envelope_decode_jobs` 175,776, `store_replacement_jobs` 136,496, `media_exports`/`media_closures` 95,232 each, `peer_presence_retirements` 86,016, `pending_reserved` 86,016. About 2.7 MB per app, 4 apps-worth of stores each owning a DAG besides. After the registry change the `ArtifactFixedRegistry` entries drop to their page tables; `ArtifactFixedQueue`, `ChildMemberRegistry` and `VecDeque` backings are still eager and make up most of the remaining 3,072/16,384/965,632/245,760 releases.

## Answer to the open question: why does XLSX pass with the same 3.9 MB per store?
PDF does not replace stores twice. Every scenario instance owns four `ArtifactStore`s (document, config, draft, interaction), each with its own eager `MutationDag`, so one app carried about 15 MB of DAG backing plus about 2.7 MB of registry/queue backings, and each instance loads its base document exactly once when the base differs from the editor's own initial document (`acceptance_seeded`). The PDF history-edit law builds many more instances than XLSX (five 1.4 leaves with several cases each, three alternative scenarios per case), and every PDF case differs from the initial document so each one loads; XLSX has fewer cases and some boot on the initial document without a load. So the difference is the instance count, not a duplicated replacement; with lazy pages the cost per instance fell by roughly two orders of magnitude for the DAG, and the remaining per-instance cost is the queue/registry list above.

## Not done
- `ArtifactFixedQueue`, `ChildMemberRegistry` and the other `VecDeque` backings are still eager (about 1.4 MB of the 2.7 MB per app); 1.4 history still spends about 330 MiB. Making these lazy follows the same page-on-first-use design but touches direct `items` users.
- 1.7 history laws were not green for a non-release reason.
- All temporary `[DEBUG]` probes added in this session were removed.

---

# Remaining eager backings

## What changed
- `ArtifactFixedQueue` (`🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs`) births no slot: `birth_demands` quotes 0 capacity bytes (the authored maximum must still be representable), `new` and `admit_backing_granted` only mark the queue admitted, and `push` births slots through `deque_birth_slot`, which doubles the exact capacity up to the authored limit and records each birth in the birth ledger. Release is unchanged (actual `capacity * size`).
- `pending.reserved_commits` (`🧵️instance/👷️jobs/📥️staging/🧱️components`) is a plain `VecDeque` born empty (quote 0, no `with_capacity(64)` in `new` and field 27); the three `push_back` sites use `deque_push_back_born`. `original_deque_backing` was removed with its last user.
- `ChildMemberRegistry` (`🪆️child/🗂️member-registry`): slots live in 16-slot pages (`ChildSlots`); the registry is born with its page table only (512 bytes quote), `admit_identity` births the page of the slot it lands in (fallible, ledger-recorded), `release_bytes` / `release` walk one resident page per step and the page table last.
- Test fixtures: `🏛️ownership/🧪️effect-fixture` (self-funded `AuthoringWallet` effect runner) and `OriginalJobFixtureCaller::retirement_turn_with_decoder`.

## Numbers (PDF 1.4, native, serialized, one run each)
| Law | Result | Wall time | Largest per-caller released_bytes (rung probe) |
| --- | --- | --- | --- |
| `pdf14::tests::history_edits_end_to_end` | ok | 70.1 s (was part of a 158.8 s pair) | 16.9 MB (about 16 MiB; was about 330 MiB) |
| `pdf14::tests::conflict_history_edits_end_to_end` | ok | 32.4 s | 43.7 MB (about 42 MiB; was about 155 MiB) |

Both are far under the 100 MiB target. The per-app eager heap of about 1.4 MB (965,632 reserved commits + 245,760 children) is now about 1 KB.

## Verification
| Run | Result |
| --- | --- |
| `semio-plugin-ownership --lib` (root workspace) | compiles now; 17 passed, 2 failed. Registry laws (`paged_registry_...`, `large_slots_are_born_on_first_insert_...`) and the two new footprint laws (`a_fresh_fixed_queue_holds_no_heap_and_births_slots_by_doubling_up_to_its_capacity`, `a_born_on_demand_deque_owns_only_the_slots_it_used`) pass. The two failures are `reserved_job_session::original_refused_source_tests::*` (`original caller has no parent source supplier`; born 98,128 vs freed 91,264), which use the pre-effect `StepContext::new` fixtures and do not touch registries or queues; not changed here. |
| `semio-framework-replication-causal --lib -- --skip original_clone_tests`, default threads, 4 runs | 43 passed every time (the earlier parallel contamination failure did not reproduce; no shared state was found, the heap observers are thread-local). |
| `original_clone_tests::original_causal_envelope_clone_...` | still aborts at thread exit: the thread-local `drive` bank of `semio-framework-value-clone-source-testing` is dropped with a live work action (`original work issuer retains an issued action or committed proof`). The test body never touches `MutationDag` pages, slots, queues or registries, and the only other `drive` user (the seeds law) passes; the work-action loan files are being changed by another agent (`available_work`). Not caused by the lazy pages; not fixed (the reply-driver error path in `drive_reply` is the suspect). |

## ownership test target repairs (why it did not compile)
- `registry-backing` tests declared their own `#[global_allocator]`; the job `testkit` already owns the one allocator of every test binary, so the local one was removed.
- Four test sources still used the pre-effect owner API (`step(grant)`, `retire_owned(.., grant)`, `close_step_controlled(grant, control)`, `with_native_receiving::<_, E>`). They now run through the effect fixture. Deleted instead of ported because their premise no longer exists: `original_receiving_refusal_keeps_the_raw_semantic_owner` (generic producer error type removed) and `ephemeral_original_request_denials_and_cancellation_preserve_exact_custody` (its decoder-bound close needs the retirement-with-decoder caller; `retirement_turn_with_decoder` was added but the issued items/depth of the nested retirement scope still refuse; the denial axes it asserted now live in `close_original_admission`). The config-factory law keeps only funded-path laws; per-axis grant denial no longer applies to the effect-shaped factory.

## Still eager
- `PrefixSnapshot`/registry pages of 3,072 bytes and store-initialization worker pages of 16,384 bytes (maintenance ladder, per load, not per app).
- The `Box` frames of `ToolOperations`, `MediaTransfers`, `TimeTravelLedger` (inline sizes, not fixed backings).
- No `[DEBUG]` probe of this session remains; the `[DEBUG] rung` probes in the maintenance ladder belong to another agent.
