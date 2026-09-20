# HT8 — hub suite, the final four + the new timeout (outcome 2's gate to green)

Slice HT8. Spec: `📓️ht7-hub-suite-final-four.md` (batch 1 + batch 2) and the 20:26 rerun
`🗑️generated/coordinator-hub-nextest-full.txt` — **321 tests: 316 passed / 4 failed / 1 TIMED OUT**,
suite wall time **309 s** (was ~30 s).
Rule 26 binds me: no `cargo test` / `nextest` / `build` on `-p semio-hub`;
`cargo check -p semio-hub --all-targets` and kernel/gis crate checks under rule 25's private
`CARGO_TARGET_DIR=…/target-ht8` only.

## 0. What the 20:26 rerun printed — every HT7 instrument spoke

| # | law | 20:26 evidence |
|---|---|---|
| **L5** | `tests::only_an_author_of_the_space_can_delegate_to_an_agent` | **TIMEOUT [300.005 s]** — new; owns the 309 s wall time |
| L1 | `…gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint…` | `refused as Some(PublisherRegionDiffers); "attempt 0 wanted inference-de4939… present=true, pack carried []"` |
| L2 | `…prepared_approval_survives_restart_and_reconciles_exactly_once` | `refused as Some(WitnessDiffers) / Some(ProposalHash)` — **not** HT7's ranked `FenceInactive` |
| L3 | `…quick::gis_map_abandoned_pre_witness_request…` | `state Assembly, maintenance_owns=true, gate_free=false, ingress_released=false` |
| L4 | `tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `retained pool-use owners 3, retained slots { capability_open: 0, catalog_read: 0, catalog_bootstrap: 0, create_catalog: 0, sync_hello: 0, artifact_retirement: 0 }` — **all six 0** |

Every one of HT7's four instruments spoke, and three of the four answers falsify the ranking HT7
wrote down. Sections 1-5 take each law in the order the suite's wall time demands.

## 1. L5 — the new 300 s TIMEOUT, named and bounded *(root of the 309 s suite)*

`tests::only_an_author_of_the_space_can_delegate_to_an_agent` **passed in 0.119 s at 12:25** and now
burns 300 s of nextest kill time on its own — the whole difference between a 31 s suite and a 309 s
one. It is the same shape HT4 §1 bounded for
`space_administration_page_v1_route_denies_a_spectator_the_author_windows`: that law went green and
the hang moved to the next law over the same fixture path (`test_state` → three sessions → space →
spectator membership → `spawn_server` → routes), which is evidence the hang is **in the shared
fixture or the teardown, not in the law's own claim**.

`LawHangWatchdogV1` (HT4) was armed on exactly ONE law, so it could not name this one. **Landed:**

| hunk | what it does |
|---|---|
| `🔬️bin-unit/🦀️.rs` `LawHangWatchdogV1` now carries a `phase` | `arm()` seeds `"armed, before the first step"`, `at(phase)` records the step the law entered, and the abort line prints `last phase entered: <phase>`. The watchdog is an OS thread, so it still fires when the current-thread runtime itself is wedged and `tokio::time::timeout` cannot |
| L5 armed + every await bounded | `bounded_law_step` names all six fixture awaits and `bounded_http_request` (20 s) the two refused delegations; the listing read is bounded too |
| the last phase is **teardown** | `watchdog` is declared FIRST, so it drops LAST — after `state`. Its final phase is `"body complete, tearing down the hub state and its worker pool"`, so a hang in `Database`'s drop (L4's family) is named as such instead of looking like the law |

Next rerun therefore answers L5 in **≤25 s** with one of: a named `bounded_law_step` panic (that
await never resolves), or the watchdog's abort naming the phase — and if that phase is the teardown
one, L5 and L4 are the same defect. Suite wall time returns to ~35 s either way. No bound was
removed; the 15/20/60 s guards HT4 installed are untouched.

## 2. L1 — neither hypothesis: the `pack` half of an `ArtifactPair` is the GENESIS, always *(oracle fixed)*

HT7 left two hypotheses, region **absent** (store/stamping staleness) or region **renamed** (gis
producer). The 20:26 evidence — `folded pair carried []`, i.e. the decoded pack has **no regions at
all**, not a renamed one — sent me to the pair format itself, and the answer is a third thing,
proved at three independent sites:

| site | what it proves |
|---|---|
| `🏪️store/🦀️.rs:12117` `print_document_pack` | `let pack = envelope.vcs.initial_snapshot.encode_pack()` — the pack half is the **genesis snapshot**; the edits are the `spr` half |
| `🏪️store/🦀️.rs:10380` `apply_ops_binary_impl` (the native codec every checkpoint applies operations through, via `🔏️trusted-catalog/🦀️.rs:277`) | after `dispatch_apply_exact`, it returns `print_document_pack(...)` — so applying an operation to a pair **grows the spr and reprints the same genesis pack** |
| `🏪️store/🦀️.rs:3377` `open_member_store` → `parse_decoded_document_spr(&pack, history)` | every load path folds the `spr` back **onto** the pack, so a pack carrying the current state would double-apply |

`initial_snapshot` is never advanced anywhere in the store. So no committed mutation can ever appear
in `pair.pack`, for any artifact, by construction — the law's publisher was asking the genesis
whether an approval had happened. `snapshot_pack()` is also what the law's closing assertion reads
(`reverted == genesis_snapshot`), which under genesis semantics was **trivially true** and proved
nothing about the undo.

**Landed** (`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`) — a stronger oracle, no assertion dropped:
`published_pair_snapshot(pack, spr)` folds the published pair through the product's own
`parse_document_pack` and hands back the head state (`ParsedDocumentText.snapshot`, with the
envelope retired via `into_owners()` so no fail-closed projection root aborts). The publisher's
region guard and the closing `reverted` read both now fold. The law's own arithmetic already fits:
attempts 0/1 (the approval) must carry the region, attempt 2 (the undo) must not, and the undo's
folded head must equal the genesis — three claims that are only meaningful once the pair is folded.

**No gis source was touched, and none is needed**: the WAL corpus's frozen command carries
`{"CreateRegion":{…"id":"inference-<jobId>","kind":"inference-bounds"…}}` verbatim, so
`create_region_group_work` produces exactly the three fields the publisher demands (§3 measured it).

## 3. L2 — `WitnessDiffers / ProposalHash`: two corpora, one job, drifted apart *(root found, repaired)*

HT7 ranked `FenceInactive`; the rerun printed **`ProposalHash`** instead, so the fence and the
frontier are fine and HT3a's WAL `scan`→`Done` change is exonerated. The conjunct compares the
committed witness's `proposal_hash` against the reopened ledger's own prepared outbox row.

`git diff` on the two frozen corpora names the root exactly:

| key | `HEAD` | working tree (HT4 §6's regeneration) | ledger corpus `outbox` |
|---|---|---|---|
| `proposalHash` | `c6d60dfe…a2bcc` | `418545e5…31dc29` | `c6d60dfe…a2bcc` |
| `commandHash` | `ff026b41…c3dda` | `9727e358…deed07` | `ff026b41…c3dda` |
| command `mutationId` | `e9dc772d…9346b` | `f9c05f70…5f3b7` | `e9dc772d…9346b` |

At `HEAD` the two corpora described **one** approval of job `4da0cbcd…` — same scope, same actor,
same hashes. HT4 rightly regenerated `🧾️inference-wal-proof-v1` from the real producer (its diff
payload was the literal placeholder `010203`), and the WAL laws went green; the ledger corpus
`🗺️gis-inference-job-v1` kept the placeholder-derived digests, so from that moment a committed
witness could not match its own prepared row. `prepare_approval` (`🪶️sqlite/🦀️.rs:839`) enforces
`sha256(stored proposal) == proposal_hash`, so the ledger's `proposal` must be the real bytes —
there is no way to fix the hash alone.

**Landed** — the ledger corpus re-derived from the SAME producer read-out
(`🐍️ht8-align-ledger-outbox.py`, in the ticket folder; only the five coupled keys move, `jobId`
unchanged): `proposal` = the real `CreateRegion` JSON (verified: its sha256 **is** the WAL corpus's
`proposalHash`), `commandHex` = the WAL corpus's `encodedHex`, plus the two hashes and the mutation
id. The frozen oracle that pinned the placeholder moved in lockstep and stayed exact, never loosened:
`🧬️schema/🔣️.json`'s `InferenceApprovalOutboxV1.proposal` const, and the TypeScript twin's literal
(now `INFERENCE_APPROVAL_OUTBOX_PROPOSAL_V1`, used by the type and both decoder branches). Verified
by script that schema const, TS literal and fixture are byte-identical.

The other reader of this block,
`runtime::tests::gis_map_approval_fails_closed_without_a_composition_transaction…` (green today),
asserts `prepared.mutation_id == approval_mutation_id(job_id, sha256(proposal))` **and**
`== outbox["mutationId"]` — all three keys moved together, so its identity still closes.

## 4. L3 — the deadlock stayed fixed; the stall is inside the Assembly turn *(instrumented)*

HT7 §1's map-lock fix held and §8's printer spoke: `state Assembly, maintenance_owns=true,
gate_free=false, ingress_released=false`. So the document writer and the ingress are still held by a
turn that never hands off, and the stall is `drive_abandoned_turn`'s `Assembly` arm
(`🏃️runtime:1641`), which `cancel()`s and then `advance(one item)`s. From source there are exactly
two ways that never leaves `Assembly`, and they are different defects:

- **`Ok(_) → Continue`** — `fail(Cancelled)` sets the phase to `ClosingValue`, and
  `close_assembly_publication` (`🗄️durable-group/🦀️.rs:1169`) returns `Ok(false)` for every
  `Pending`/`Blocked` `close_step`, so the phase never reaches `ClosingParent → Terminal`: a
  retirement cursor that will not drain under a one-item grant.
- **`Err(_) → Retry`** — an admission/preparation error re-entered every turn.

The previous instrument collapsed both into "Assembly". **Landed:** `record_abandoned_assembly_turn`
(`🏃️runtime/🦀️.rs`, `cfg(test)` recorder in the same shape as HT7's conflict arms) stores
`"Continue on Progress(<phase>)"` or `"Retry on <error>"`, the helper now prints
`Assembly phase={:?}` from `owner.phase()` beside `last abandoned turn {:?}`. One rerun names the
sub-phase and separates the two defects.

## 5. L4 — all six families are 0, and the seventh has no table *(instrumented)*

The census answered every family HT6 and HT7 proposed with **0**, including HT7 §7's
`artifact_retirement`: `{ capability_open: 0, catalog_read: 0, catalog_bootstrap: 0,
create_catalog: 0, sync_hello: 0, artifact_retirement: 0 }` with `retained pool-use owners 3`. So
neither HT6's `create_catalog` release nor HT7's non-terminal `ArtifactAuthority::drop` parking a
cursor is the leak, and the surviving clones are held by something with **no slot table at all**.

Source names it: `spawn_with_pool_use` (`🗿️artifact/🦀️.rs:5360`) puts a **second** clone into
`ArtifactRunnerHandoff.pool_use` beside the authority's own `_pool_use`, and that one is surrendered
only at the runner's terminal transition (`:5131`). The handoff lives inside the runner, not in a
registry — so an authority already absent from `open_artifacts` (hence `open artifacts 0`) and
holding no retirement cursor can still pin the database's use through a runner that never went
terminal. Two such runners plus the `Database`'s own use is exactly the observed **3**.

**Landed:** `artifact_runner_handoff_pool_use_live_slots()` — a process-global count incremented
where the handoff is built and decremented both at the terminal take and in a new
`Drop for ArtifactRunnerHandoff` (so it counts LIVE handoff-held clones and never over-counts a
freed one) — plus a `runner_handoff` field in `DatabaseRetainedPoolUseCensus`. Non-zero on the next
rerun means the shutdown must drive those runners terminal (or close the authorities terminally)
before the pool deadline; zero means the clones are on an `ArtifactHandle` the hub itself still owns
at shutdown, which is the last place left.

## 6. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht8 cargo check -p semio-hub --all-targets` (§1, §2, §4, §5) | **EXIT 0**, 0 errors, 316 warnings | `🗑️generated/ht8-check-hub-1.txt` |
| same, after §3's corpus + schema work | **EXIT 0**, 0 errors, 318 warnings | `🗑️generated/ht8-check-hub-2.txt` |
| `cargo check -p semio-framework-os-kernel-db --lib` | **EXIT 0**, 26 warnings | `🗑️generated/ht8-check-db-lib.txt` |
| `cargo check -p semio-framework-os-kernel-db --all-targets` | **EXIT 101** — the SAME pre-existing peer breakage HT7 §4 reported (`E0432: unresolved import semio_framework_pack` at `🛢️db/📦️packages/🦀️rust/🦀️.rs:12`), in that crate's `lib test` target only and unrelated to my hunks | `🗑️generated/ht8-check-db-1.txt` |
| `tsc --noEmit --strict` on the TypeScript twin, standalone (it has no imports) | **no diagnostics** | — |
| schema const ↔ TS literal ↔ fixture `outbox.proposal` byte-identical; `sha256(proposal)` = the WAL corpus's `proposalHash`; `jobId` unchanged | **all true** | `🐍️ht8-align-ledger-outbox.py` output |

Rule 26 respected: no `cargo test`, `nextest` or binary build on `-p semio-hub` from this slice, so
**no hub law was observed passing**. The live hub on 7611 was not touched. Nothing in
`🗑️generated` that I did not create was removed, and no `🗑️generated` folder was swept.

## 7. Files changed

- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — §1's phase-carrying `LawHangWatchdogV1` and L5's armed,
  bounded body.
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §2's `published_pair_snapshot` and the two
  folded reads; §4's `Assembly phase=…` + `last abandoned turn` printer.
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — §4's `record_abandoned_assembly_turn` /
  `last_abandoned_assembly_turn` and the two instrumented outcomes of the abandoned `Assembly` arm.
- `🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json` — §3's five re-derived `outbox` keys.
- `🌎️hub/💡️inference/🧬️schema/🔣️.json`, `🌎️hub/💡️inference/🧬️schema/🟦️.ts` — §3's frozen oracle,
  moved in lockstep and still exact.
- `🧰️framework/…/🛢️db/🗿️artifact/🦀️.rs` — §5's `ARTIFACT_RUNNER_HANDOFF_POOL_USES`, its reader,
  the decrement at the terminal take and `Drop for ArtifactRunnerHandoff`.
- `🧰️framework/…/🛢️db/⚙️engine/🦀️.rs` — §5's `runner_handoff` census field.
- `🐍️ht8-align-ledger-outbox.py` — the corpus re-derivation script (ticket folder).

Nothing was deleted, `#[ignore]`d or loosened; no law lost an assertion; no bound was removed; no
gis or stdio plugin source was touched.

## 8. Honest gaps

- **Nothing was observed green.** Rule 26 forbids this slice from running the suite. Everything here
  is source-proved plus a compiling tree.
- §2 and §3 are landed roots; §1, §4 and §5 are guards and instruments, each replacing a guess with
  a string the next rerun prints.
- §3's blast radius is the widest thing I landed: it moves a frozen corpus AND the schema/TS oracle
  that pinned it. The Rust "field for field" law compares required-field names (not consts) and is
  unaffected; the structural draft-07 validator reads the same const I updated. The TypeScript twin
  type-checks standalone but I did **not** run the repo's vitest projects (outside this slice's
  budget and not hub laws) — if a TS law asserts the old literal, it will name itself.
- §2 changes the law's oracle, not the product. If the intended contract is instead that a published
  *checkpoint* compacts its pack to the state at the frontier, the product would need a compacted
  pair printer, which does not exist anywhere in the store today — three independent sites say the
  current contract is genesis-pack + folded-spr, and I did not invent a fourth.
- L4's shutdown defect is still unfixed: §5 decides between the last two candidate owners. A hub
  that reopens sqlite still cannot shut down cleanly.

**needs hub rerun.** Expect: **L5 answered in ≤25 s** with a named await or a named watchdog phase
(suite wall time back to ~35 s); **L1 green** on §2 (or, if still red, the arm plus a folded-pair
read-out that now lists real regions); **L2 green** on §3 (its `ProposalHash` conjunct is the only
one that was false); **L3 still red**, now printing `Assembly phase=<sub-phase>` and
`Continue on Progress(<phase>)` / `Retry on <error>`; **L4 still red**, now printing
`runner_handoff: N` — non-zero means the shutdown must drive those runners terminal, zero means the
clones sit on a live `ArtifactHandle`.

---

# HT8 — batch 2 (rerun 21:03: **321 — 315 / 6**, suite **32 s**, 0 timed out)

**L5 is GREEN** and the suite is back to 32 s from 309 s (§1). Two of the six are TC2's new document
index reds, not mine. My four moved as follows.

| law | 21:03 evidence | reading |
|---|---|---|
| L1 | `🌿️vcs/🦀️.rs:612` *"artifact history ledger reached Drop before every exact entry owner was retired"* | **the `Conflict` is gone** — the fold works; my disposal of the parsed envelope was wrong |
| L2 | `🪶️sqlite/🧪️tests:343` `unwrap()` on `Err(Expired)` | **`WitnessDiffers/ProposalHash` is gone** — §3's corpus alignment was the root; the law now dies 34 lines later on a line it had never reached |
| L3 | `Assembly phase=ClosingDrawing, last abandoned turn Some("Continue on Progress(ClosingDrawing)")` | the cancelled assembly closes its **value** publication and then never closes its **drawing** one |
| L4 | census now `{…, artifact_retirement: 0, runner_handoff: 0 }`, still `owners 3` | §5's family is **excluded**: no live runner handoff holds a clone, so no live `ArtifactAuthority` does either |

## 9. L1 — a parsed envelope needs the bounded retirement protocol, not a drop *(fixed)*

`ParsedDocumentText`'s `into_envelope()` retires the replayed projection and hands back the envelope
— but every shell below it (`ArtifactEnvelope`, its `ArtifactVcs`, the history ledger, each `Edit`)
asserts in `Drop` that its nested owners were retired through the bounded protocol, and **only an
`ArtifactStore` ever runs that protocol**. My `drop(envelope.into_owners())` (copied from
`apply_ops_binary_impl`'s empty-ops branch, which only ever sees an envelope with no edits) therefore
aborted on the first folded pair that had history. The store already ships the right helper for a
document that belongs to no store — `store::test_support::retire_parsed_document`, which drives
`ArtifactStoreEnvelopeRetirement` to terminal emptiness. **Landed:** clone the folded snapshot, then
retire the parse through that helper.

## 10. L2 — one 1 000 ms `Instant` deadline shared across 60 lines *(fixed)*

`InferenceOperationControlV1::new(1000, 5)` is an **`Instant` deadline**, and `checkpoint()` answers
`Expired` on wall clock alone. The law minted ONE control at `:283` and reused it at `:284`, `:288`
and — newly reachable now that §3 unblocked the reconciliation — at `:343`, on the far side of
`committed_fixture_witness()`'s real WAL verify and the reconciliation itself. The run took 1.53 s,
so the shared control was long dead: it measured fleet scheduling, exactly the anti-pattern HT4
named when it replaced the per-call 2 s/5 s bounds. **Landed:** `control` is now a closure minting a
fresh control per query, so each read carries the same 1 000 ms / 5-unit bound it always did.
`progress` is `fetch_max`, never additive, so no cumulative claim existed to lose.

## 11. L3 — value retires, drawing does not *(instrumented to the owner)*

The recorder separated the two defects: it is **`Continue`**, not `Retry`, so no error is being
re-entered — `close_assembly_publication` keeps answering `Ok(false)` for the **drawing**
publication, turn after turn, while the structurally identical **value** publication completed one
phase earlier. Inside `ArtifactStoreBatchPublication::close_step` that means one of its sub-owners
refuses to retire under a one-item grant (preparation, source, stage, authority, receipt or fault),
and the bool return discards which. **Landed:**
`DurableOwnedThreeStoreMapAssemblyV1::closing_witness()` — a read-only accessor reporting the phase,
`failure`/`prepared`/`host`, and for each of the three publications its `phase()`,
`terminal_is_empty()`, `admitted_items()` and `staged_items()`. The helper prints it beside the
phase. One rerun names the owner that will not retire.

## 12. L4 — six families excluded, the seventh is the mount work *(instrumented)*

`runner_handoff: 0` excludes the whole authority family (an `ArtifactAuthority` cannot be alive
without its handoff), so the two surviving clones came from a `require_open_use()` call that is
**not** in any slot table. That call site list is finite — `mount_document` (`:8507`),
`create_document_catalog_retained` (`:8580`), `hello_retained` (`:8774`, `sync_hello: 0` and the hub
never calls it), `checkpoint_document` (`:8801`, a local dropped on return) — and only the first
hands its clone to a **boxed future that outlives the call**: `DatabaseDocumentMountOwner.work`
holds the `run_document_mount` future, and a retained-open retry holds a **second** clone in its
`resume` future (`:8495`). One mount owner that outlives its registry entry is therefore worth
exactly the observed **2**, and `open artifacts 0` cannot see it: the shutdown loop drives `Opening`
slots and then counts `registry.len()`, which says nothing about an owner no longer in the registry.

**Landed:** `DATABASE_DOCUMENT_MOUNT_WORK_LIVE` — incremented in `DatabaseDocumentMountOwner::new`,
decremented in a new `Drop for DatabaseDocumentMountOwner` — and a `mount_work` census field.
Non-zero on the next rerun means the shutdown must retire the mount owners' boxed work before the
pool deadline (the live-hub fix); zero means the clone sits in a caller-held
`DatabaseCreateCatalogFuture`, which is then the only site left.

## 13. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht8 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors | `🗑️generated/ht8-check-hub-3.txt` |

Rule 26 respected; the live hub on 7611 untouched; nothing deleted, `#[ignore]`d or loosened.

## 14. Files changed (batch 2)

- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` — §9's `retire_parsed_document` disposal;
  §11's printed closing witness.
- `🌎️hub/💡️inference/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs` — §10's per-query control.
- `🧰️framework/…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` — §11's `closing_witness()`.
- `🧰️framework/…/🛢️db/⚙️engine/🦀️.rs` — §12's mount-work counter, its `Drop`, and the `mount_work`
  census field.

## 15. Honest gaps (batch 2)

- Still nothing observed green from this slice (rule 26). §9 and §10 are corrections to my own
  batch-1 work plus a real bound defect; §11 and §12 are the next decisive instruments.
- §12 names the mechanism from source but does **not** yet fix it: the shutdown still has no step
  that retires in-flight mount work, so a hub that reopens sqlite still cannot shut down cleanly.
  I did not land the retirement step before the census proves the family, because the previous two
  guesses (HT6's `create_catalog`, HT7's `artifact_retirement`) were both wrong and both cost a rerun.

**needs hub rerun.** Expect: **L1 green** (§9 removes the only thing between the fold and the
assertions); **L2 green** (§10 removes the only thing between the reconciliation and the law's end);
**L3 red**, now printing which of parent/drawing/value refuses to retire and in what phase;
**L4 red**, now printing `mount_work: N`.
