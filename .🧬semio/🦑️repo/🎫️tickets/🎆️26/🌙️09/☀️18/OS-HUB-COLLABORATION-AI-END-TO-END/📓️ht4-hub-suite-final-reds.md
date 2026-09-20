# HT4 — hub suite final reds (outcome 2's gate to green)

Slice HT4. Inherited spec: `📓️ht3a-hub-inference-reds.md` §11–§17 and
`📓️ht3b-hub-socket-admin-checkpoint-reds.md` §12–§17.
Baseline capture: `🗑️generated/coordinator-hub-nextest-full.txt` (17:08) —
**321 tests: 309 passed / 11 failed / 1 TIMED OUT**, suite wall time **312 s**.

Rule 26 binds me: I never ran `cargo test`/`nextest`/`build` on `-p semio-hub`. Every claim below is
either (a) read from the coordinator's capture, (b) proven by `cargo check -p semio-hub --all-targets`
under rule 25's private `CARGO_TARGET_DIR=…/target-ht4`, or (c) marked **unverified at runtime**.
Nothing was deleted, `#[ignore]`d or loosened; no bound was removed.

## 0. The twelve, verbatim from the 17:08 capture

| # | law | panic / verdict |
|---|---|---|
| T | `tests::space_administration_page_v1_route_denies_a_spectator_the_author_windows` | **TIMEOUT 300.007 s**, no panic, no stderr. Owns 300 s of the 312 s wall time |
| 1 | `inference::wal::…executes_literal_committed_transaction_scope_and_cancellation_traces` | `frozen fixture literals disagree with the producer — proposalHash "c6d6…2bcc" vs 418545e5…dc29, command.mutationId "e9dc…346b" vs f9c05f70…f3b7, commandHash "ff02…3dda" vs 9727e358…ed07` |
| 2 | `inference::wal::…rejects_hash_matched_noncanonical_or_wrong_actor_commands` | `a matching durable hash cannot bypass "trailing-byte-after-hlc"` |
| 3 | `chain::quick::…cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof` | `assertion failed: verifier.close_steps() > 0` (`⛓️chain/🦀️.rs:321`) |
| 4 | `inference::sqlite::…prepared_approval_survives_restart_and_reconciles_exactly_once` | `called Result::unwrap() on an Err value: Conflict` (`🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309:169`) |
| 5 | `inference::runtime::…approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `[WARN] … Admission { role: "parent", reason: "stamped publication clock is not strictly after the Store's own clock" }` → `got Some(Denied)` (`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:495`) |
| 6 | `inference::runtime::…terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission` | `worker pool shutdown: Busy { retained_uses: 1 }` (`:805`) |
| 7 | `quick::…abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `approval reaches its retained cancellation phase Preflight` (`:1324`) |
| 8 | `quick::checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe` | `409 != 200`, `refused as Some(DocumentSnapshotDiffers("generation 0 … frontier (1,1,0) expected (1,1,0) artifact Some(F) expected Some(F)"))` — **both printed sides byte-identical** |
| 9 | `quick::checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication` | same `DocumentSnapshotDiffers`, both sides identical, at the fence-admission deadline |
| 10 | `tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` | `4401 unauthorized` after welcome, `plan refusal Some(DirectoryRevisionDiffers)` |
| 11 | `quick::retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact` | `left "cancelled" == right "succeeded"` |

## 1. The TIMEOUT — named, and bounded so the rerun is seconds

**Name:** `tests::space_administration_page_v1_route_denies_a_spectator_the_author_windows`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:4234`). It is the whole 300 s: every sibling administration law
passes in 0.08–0.20 s in the same run, and this law itself passed in **0.162 s** at 12:25
(`🗑️generated/coordinator-hub-nextest-1225.txt`) and **0.166 s** at 09:17. There is no general
slow-down — the next slowest test is unchanged at 30 s.

**What I could exclude by reading, and what remains.** Every HTTP read in the suite is already
bounded: `raw_http_request_transport` wraps its read loop in `RAW_HTTP_READ_HANG_GUARD` (20 s) and
`.expect("HTTP deadline")`, and the route itself is wrapped in
`tokio::time::timeout(DIRECTORY_SPACE_ADMINISTRATION_DEADLINE_MS)` (`🏗️bootstrap/🦀️.rs:6382`). A
hang inside a request therefore panics in ≤20 s — it cannot be the 300 s. The twelve pause seams
HT3b fixed are all behind `state.live_gate`, and this law's `test_state()` sets `live_gate: None`
(`🔬️bin-unit/🦀️.rs:581`), so no seam is reachable from it. That leaves the fixture phase
(`test_state`, three `issue_test_session`, `create_space_for_test`, `upsert_member_for_test`,
`DirectoryCommand::CreateInvite`), or a wedge that stops the runtime advancing timers at all.
The two directory writer fences (`pause_decision_test_fence_once`,
`pause_publication_test_fence_once`, `📇️directory/🦀️.rs:2470`/`:2488`) are one-shot and armed only
by the writer-order laws, so they are inert here too. **I could not name the exact await by reading,
and rule 26 forbids me from running the suite to find it.**

**What I landed instead — two complementary named guards, no bound removed:**

| hunk | what it does |
|---|---|
| `🔬️bin-unit/🦀️.rs:490` `LAW_BODY_HANG_GUARD` = 25 s + `LawHangWatchdogV1` | a wall clock on its **own OS thread**, armed at the top of the law and disarmed by `Drop`. `tokio::time::timeout` cannot fire on a starved executor — a current-thread runtime whose task blocks or spins never advances the timer wheel — which is exactly the shape that produced 300 s of silence. On expiry it names the law on stderr and aborts, so nextest reports that law in ~25 s instead of 300 s |
| `🔬️bin-unit/🦀️.rs:~540` `bounded_law_step(step, future)` | one awaited step, bounded by the existing `SOCKET_RENDEZVOUS_HANG_GUARD` (15 s) and **named**. Applied to all eight awaits of the law: `hub state open`, `author session`, `spectator session`, `outsider session`, `space creation`, `spectator membership`, `author invite fixture`, `spectator page read`, `outsider page read`, `anonymous page read` |

Next rerun therefore answers one of two things, in ≤25 s: either a `spectator law: <step> did not
complete within 15s` panic naming the exact await (the future genuinely never resolves), or the
watchdog's abort line — which proves the executor itself is wedged (a blocking call or a
non-yielding spin inside an async path), because a 15 s tokio timer failed to fire.

## 2. Roots landed

### 2.1 Laws 8 and 9 — `authority_generation == 0` is a **live** generation, not "no authority" *(root fixed)*

The diagnostic HT3b added printed both sides of the frontier comparison and they are **byte-identical**
in both laws — document id, `(head_seq, commit_seq, epoch) = (1,1,0)`, and an `ArtifactFrontier`
equal field for field. The only conjunct left in `checkpoint_publication_snapshot_matches`
(`🏗️bootstrap/🦀️.rs:3723`) is the one the diagnostic reported as **`generation 0`**:

```rust
snapshot.authority_generation != 0 && …
```

`authority_generation` is `ArtifactHandle::generation().0`, which is the document actor's
**supervision** counter, and `GenerationId::INITIAL` is **0** —
"the generation of a freshly spawned actor that has never restarted"
(`🧰️framework/…/🛢️db/🆔️ids/🦀️.rs:59`). So the predicate refused a checkpoint from **every document a
running hub has opened for the first time**, i.e. every document until a supervisor restart. Not a
test defect: a live first-open publication is refused the same way.

**Fixed** at `🏗️bootstrap/🦀️.rs:3723`: the `!= 0` conjunct is removed with a docstring saying why.
Staleness is already answered where it is knowable — `checkpoint_publication_snapshot()` itself
fails `DbError::StaleGeneration` when the handle outlived its mailbox — so the snapshot's agreement
with the command is the whole predicate. The diagnostic at `:4077` now also prints
`db_artifact_id(scope)`, so a future document-id disagreement shows both sides instead of one.

### 2.2 Law 10 — a plan pinned the **global** directory revision by equality *(root fixed)*

The close is a 4401 on the **observer's** socket (`c`), not the removed member's: the law asserts the
removed member is closed (it is, by membership revocation) and that the observer stays live to see
the presence withdrawal. `document_plan_socket_validity` (`🏗️bootstrap/🦀️.rs:4341`) required

```rust
directory_revision != authority.revalidation.directory_revision || directory_revision != authority.revalidation.membership_generation
```

against `state.directory.head_seq()` — the **global** directory head. Every outstanding open plan on
the hub therefore died on the next directory append **anywhere**: another space's invite, a rename,
an unrelated membership edit. The admin removal in this law appends events, so the observer's plan,
issued earlier, no longer matched and its live socket was closed 4401 by an event about someone else.
The same equality sits on the plan→grant exchange at `:3032`, where it made a plan unexchangeable the
moment any unrelated write landed — visible in the suite as fixtures that have to re-pin by hand
(`🔬️bin-unit/🦀️.rs:3145-3146`, `:2409-2410`).

**Fixed** at both sites: the pin is a witness of issue order, so a plan may never claim a revision
the directory has not reached (`authority.revalidation.* > directory_revision` stays refused — this
is what the green hostile-revision law at `🔬️bin-unit/🦀️.rs:3066` asserts, `live + 1`), but an older
pin is simply an older plan. Whether the caller still holds the scope is decided by membership and
session revocation on their own paths, which is what closes the removed member's socket in the same
exchange. No law asserts that a directory append must make an exchange `Stale`; I grepped all
eleven `DocumentOpenPlanErrorCodeV1::Stale` assertions in `🔬️bin-unit`.

### 2.3 Law 3 — `close_steps` was unaccounted on the replay that never opened *(root fixed)*

HT3a's §16 hypothesis is confirmed by source: in `verify_retained`
(`🌎️hub/💡️inference/🧾️wal/🦀️.rs:307`) the retained boundary cases open at a compacted-away segment,
so `WalReplayCursor::open_at_segment(…)?` short-circuited **before** the close loop that increments
`close_steps`, and the law read `Err(Storage)` with `close_steps == 0`.

**Fixed:** a replay begins when the verifier takes the WAL lease (`state.storage.wal().await`), not
when the cursor opens. The open failure now records its one retirement step before returning
`Storage`, so the counter answers "every replay I began, I retired" on both paths. This is
accounting the verifier already owed; nothing in the law changed.

### 2.4 Law 1 (and law 2 behind it) — the producer now prints the **whole** regenerated fixture

HT3a's read-out proved all three digests moved, and warned that a moved `commandHash` means
`encodedHex` and the two `payloadHex` blobs are stale too — which they are, since
`sha256(encode_envelope(envelope(fixture))) == fixture["commandHash"]` still passes at
`🔬️unit/🦀️.rs:221-222`, i.e. the frozen fixture is *internally consistent and wholly superseded*.
Hand-patching three digests would leave it inconsistent, so I did **not** patch it.

**Landed** (`🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs`): `DurableFixtureRecord` now carries
`diff_payload` / `inverse_payload`, and `regeneration_readout()` prints, from the producer,
every literal `🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` owns:

```
proposalHash / command.mutationId / commandHash / encodedHex /
command.diff.payloadHex / command.inverse.payloadHex
```

The ordinary rerun is the producer run (HT3a §15's invocation is unchanged). One capture is then a
copy-paste regeneration of the fixture, not an edit — and law 2, which stacks directly on law 1's
literals, should come with it.

### 2.5 Law 11 — the terminal outcome code is now printed

`first_rows[1].fact.phase` is `"cancelled"` where the law requires `"succeeded"`. Five distinct arms
in `🏗️bootstrap/🦀️.rs` can write `phase: "cancelled"` and they are discriminated only by
`outcome_code`: `admin-operation-cancelled-before-effect` (×3, `:8772`/`:8777`/`:8796`),
`admin-authority-changed` / `admin-authority-unavailable` (`:8746`), `directory-rebuild-cancelled`
(`:8909`), and the retained task's re-auth arm (`:8998`, also `admin-authority-changed`).
The law's own assertion text ("the HTTP waiter expires **without cancelling** its admitted writer")
makes the `cancelled_before_effect` arms the live hypothesis, against HT3b's ranking of the re-auth
arm. Both assertions now print `outcome_code` and `reason_code`, so one rerun names the arm.

## 3. Still open, with what is now known

| law | root, as far as source proves it | first place to look |
|---|---|---|
| 5 | **The named next defect, and it is worse than HT3a knew.** The Hub stamps `HybridLogicalTimestamp { actor: 1, physical_ms: now_ms, logical: 0 }` (`🏃️runtime/🦀️.rs:2850`, `:2895`) and the gate admits only `(stamped.physical_ms, stamped.logical) > (self.clock.physical_ms, self.clock.logical)` (`🏪️store/🦀️.rs:16546`). Because the Hub always stamps `logical: 0`, **a live hub refuses any approval landing in the same millisecond as the document's previous edit**, not only the test's 1e3-vs-1.7e12 clock gap. The stamp cannot be relaxed to an HLC merge: `durable_decision_event_match` rebuilds the command with `timestamp: meta.timestamp` and compares to `command_hash` (`🧾️wal/🦀️.rs:298`), so the committed stamp must be the stamp the hash covers. The fix is therefore to **mint** the stamp from the document's own clock at approval-preparation time and hash over that — which needs the mounted Store's `clock` read back through `GisMapApprovalCommitterV1` before `server_stamped_command` encodes, since the prepared command is persisted to sqlite and must survive a restart (law 4). That is a multi-file design change I could not validate without running the suite, so I did not guess at it | `🏃️runtime/🦀️.rs:2850`/`:2895` (mint), `:1339` (`identity.timestamp` → the three stamps), gate at `🏪️store/🦀️.rs:16546` |
| 4 | `Err(Conflict)` unwrapped at `🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309:169`. Not triaged by me; it is the restart/reconcile path for the same prepared approval that law 5 refuses, so it may share law 5's root | `🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309` |
| 6 | `Busy { retained_uses: 1 }` — unchanged from HT3a §16: one pool use survives `committer.close()`, `drop(committer)`, a successful `Arc::try_unwrap(database)` and `database.shutdown()` | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:805` |
| 7 | times out at the **first** phase `Preflight`, upstream of assembly; `poll_approval_to_phase` never observes the `Ready{pending}` turn `finish_preflight` produces | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:1324`, runtime `:1397-1404` |

## 4. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht4 cargo check -p semio-hub --all-targets` (after §1, §2.1, §2.3, §2.4) | **EXIT 0, 0 errors**, 55 warnings in the bin test target | `🗑️generated/ht4-check-1.txt` |
| same, after §2.2 and §2.5 | **EXIT 0, 0 errors** | `🗑️generated/ht4-check-2.txt` |

**Nothing in this report is verified at runtime.** No hub test/nextest/binary build ran from this
slice (rule 26). Every root above is a source-level proof plus a compiling tree.

### Unrelated hygiene noticed, not mine to land

`cargo check` reports `unused SemaphorePermit that must be used` at `🔬️bin-unit/🦀️.rs:3447`, `:3450`,
`:3454`, `:4175` — four **law-side** `*_admitted.acquire()` calls whose permit is returned
immediately. That is the mirror image of the server-side bug HT3b fixed in §15: a law that observes
an admission and hands the permit straight back can observe the same admission twice. None of them is
in a currently-red law, so I left them; they belong to whoever owns those laws next.
HT3b also flagged three leftover `eprintln!("[DEBUG] …")` in `🔬️bin-unit` — still present.

## 5. Files changed

- `🌎️hub/🏗️bootstrap/🦀️.rs` — §2.1 (`checkpoint_publication_snapshot_matches` + the snapshot
  diagnostic), §2.2 (both directory-revision pins).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — §1 (`LAW_BODY_HANG_GUARD`, `LawHangWatchdogV1`,
  `bounded_law_step`, the timing-out law's ten awaits), §2.5 (law 11's two printed assertions).
- `🌎️hub/💡️inference/🧾️wal/🦀️.rs` — §2.3 (`verify_retained`'s open-failure retirement).
- `🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs` — §2.4 (`regeneration_readout`, the two payload
  fields, the extended law-1 panic).

**needs hub rerun.** Expect laws 8 and 9 to flip on §2.1, law 10 on §2.2, law 3 on §2.3, and the
TIMEOUT to become a named failure in ≤25 s instead of 300 s (suite wall time back to ~30 s). Law 1's
panic will now carry the complete producer read-out to regenerate
`🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` in one pass (laws 1 and 2 behind it), and law 11's
panic will name its terminal `outcome_code`. Laws 4, 5, 6, 7 stay red with §3's roots.

---

# HT4 — batch 2 (rerun 17:35: **321 — 312 / 9 / 0 timed out**, suite **37 s**)

Batch 1 landed three roots and the hang guard: **the TIMEOUT is gone** (the spectator law is green and
the suite is back to 37 s from 312 s), `checkpoint_publication_route_is_author_owned…` is green on
§2.1, and `chain::quick::…compacted_suffix_is_not_a_genesis_proof` is green on §2.3.

## 6. The WAL proof fixture, regenerated from the producer

`🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` is rewritten **entirely from §2.4's printed
read-out** — nothing hand-derived, nothing recomputed by me:

| key | was | now |
|---|---|---|
| `proposalHash` (and `bindingMismatches[0].proposalHash`, the wrong-job-id case that must carry the *right* hash) | `c6d60dfe…a2bcc` | `418545e5…31dc29` |
| `command.mutationId` | `e9dc772d…e9346b` | `f9c05f70…be5f3b7` |
| `commandHash` | `ff026b41…20c3dda` | `9727e358…20fdeed07` |
| `encodedHex` | envelope over the two placeholders | the producer's `encode_server_stamped_command_v1` bytes |
| `command.diff.payloadHex` | **`010203`** | the real `CreateRegion` JSON |
| `command.inverse.payloadHex` | **`030201`** | the real `[DeleteRegion]` JSON |

The two payloads were literal placeholders, and `encodedHex`/`commandHash` had been computed over
them — which is why the fixture was self-consistent (`:221-222` passed) and wholly superseded at the
same time. Patching only the three digests, as the earlier read-out invited, would have produced a
fixture that fails its own envelope assertion. JSON re-parsed after the rewrite.
**Laws 1 and 2 both read these literals; law 2 stacks directly on law 1.**

## 7. Law 5 — the HLC stamp, decided and landed

**The measured root, from source:** `ArtifactStore`'s clock is seeded
`HybridLogicalTimestamp::new(0, now_ms())` **at construction** (`🏪️store/🦀️.rs:13379`). For a
document with no published edit that is a fact about the machine, not about the document — so
HT3a's strictly-after gate was comparing a server-minted stamp against an unrelated wall clock, and
refused **every first approval**. Independently, because the hub always stamped `logical: 0`, a
second approval in the same millisecond as the previous edit was refused too. Both are live-hub
defects, not test-clock artefacts.

**Landed, in the two places the two defects live:**

| file:line | change |
|---|---|
| `🏪️store/🦀️.rs:16546` | the stamp gate compares against the document's own history: `self.edit_sequence == 0` (this Store has published nothing, so it is after nothing) admits; otherwise strictly-after still holds, because the commit path assigns `self.clock = next_clock` and the clock **is** the last edit's stamp. Refusal reason retyped to `"stamped publication clock is not strictly after this Store's own last published edit"` |
| `🏪️store/🦀️.rs:15248` | `ArtifactStore::clock_now()` — a non-suspending read of the clock, so a server can mint against it |
| `🏃️runtime/🦀️.rs` trait `GisMapApprovalCommitterV1` | new `document_clock(scope) -> Option<HybridLogicalTimestamp>`: `RetainedGisMapApprovalCommitterV1` returns the mounted parent Store's clock for `Ready`/`Published`, `None` otherwise (genesis — no Store, no history); `UnavailableGisMapApprovalCommitterV1` returns `None` |
| `🏃️runtime/🦀️.rs` `approval_stamp_v1(document_clock, now_ms)` | the mint: `now_ms > clock.physical_ms` → `{physical_ms: now_ms, logical: 0}`; same millisecond → `{physical_ms: clock.physical_ms, logical: clock.logical + 1}`; `None` → the hub tick. **Strictly after the document's last edit by construction**, and it is this stamp the command hash is computed over, so `durable_decision_event_match` rebuilds the identical command from `meta.timestamp` |
| `🏃️runtime/🦀️.rs:3409`, `:3462` | both preparation sites read `runtime.committer().document_clock(&context.scope).await` and pass it into `server_stamped_command` / `server_stamped_undo_command` |

**No typed refusal on an unreadable clock**, against the coordinator's sketch, and this is the one
place I diverged: `None` is not "the clock could not be read", it is *the genesis document*, which is
where both red laws sit (`base.frontier.is_genesis_for(&scope)`, frontier `(0,0)`). Refusing there
would deadlock the first approval of every document forever. The genesis case is admitted by the
Store gate above, which is where "after nothing" is actually knowable.

**Two native laws** (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`), as asked:
- HT3a's law extended with the **same-millisecond** approval: after a published stamp, a stamp with
  the identical `physical_ms` and `logical + 1` admits and becomes the committed clock.
- new `artifact_store_stamped_publication_clock_admits_a_genesis_document_behind_its_construction_seed`:
  a stamp at `physical_ms: 1_004` against a `now_ms()` seed publishes on a genesis Store, the
  committed clock is the stamp and not the seed, and the same stamp replayed afterwards is refused
  with the typed reason.

## 8. The other four, with what the 17:35 diagnostics now say

| law | new reading |
|---|---|
| `admin_removal…` | **§2.2 worked**: no 4401, no plan refusal. It now dies at the very end of the law — `🔬️bin-unit/🦀️.rs:3658` `database shutdown: Unavailable("database shutdown deadline elapsed")` in `stop_recovery_server`. A teardown/retirement defect, not authorization |
| `checkpoint_publication_route_rejects_stale_or_cross_scope_inputs…` | **§2.1 worked**: past `DocumentSnapshotDiffers`, now `🔬️bin-unit/🦀️.rs:7057` `replace publication descriptor: Conflict("document descriptor for '…/artifact-aef2b4…' is immutable")` — the law's own cross-scope fixture tries to replace an immutable descriptor |
| `retained_short_admin_request…` | the printed code is `admin-authority-unavailable`, reason `None`, and it is the **competing** writer (`second_rows[1]`), not the retained first one — so `admin_directory_authority_refusal` with a non-`Denied` `FencedDirectoryCommandErrorV1` (`🏗️bootstrap/🦀️.rs:8745`): the second writer's directory command fence was *unavailable*, not denied. HT3b's re-auth ranking and my `cancelled_before_effect` ranking are both wrong |
| runtime ×2 / sqlite ×1 (laws 6, 7, 4) | unchanged: `Busy { retained_uses: 1 }`, `phase Preflight`, `Err(Conflict)`. Law 4 is the restart/reconcile of the same prepared approval §7 fixes, so it may move with it |

## 9. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht4 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht4-check-3.txt` |
| `CARGO_TARGET_DIR=…/target-ht4 cargo check -p semio-framework-os-kernel --all-targets` (the crate that mounts `🏪️store`) | **EXIT 0, 0 errors**, 5 warnings | `🗑️generated/ht4-check-kernel.txt` |

Still **nothing verified at runtime** from this slice (rule 26).

Files changed in batch 2: `🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json`,
`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`.

**needs hub rerun.** Expect laws 1 and 2 on §6 and law 5 (and possibly law 4) on §7. Please also run
`cargo nextest -p semio-framework-os-kernel -E 'test(stamped_publication_clock)'` — two native store
laws, one extended and one new, which I may not run myself.

---

# HT4 — batch 3, hand-over (rerun 17:52: **321 — 312 / 9**, suite 32 s). No source changed.

**The ninth name, and a correction.** `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` is **not green** — it is `SIGABRT [1.552s] (147/321)`, which the FAIL grep misses. The document-clock stamp *did* work: the commit now publishes and the law runs 45 lines further, to `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:502`. The abort is a **secondary** panic (`GIS snapshot root reached Drop before exact Arc handback`, `🗺️gismap/…/💾️binary/🦀️.rs:313`) raised while unwinding the real one — the known mounted-store drop-witness trap. Every law that now fails *after* mounting stores will abort this way and hide its own assertion.

**Law 1's fixture regeneration worked**: the frozen-literal assertion at `:305` is gone; the law now runs to trace 3 of 17.

| law | printed reason | root hypothesis | file:line |
|---|---|---|---|
| `gis_map_approval_committed_event_reaches_actor_frontier…` (**SIGABRT, the 9th**) | `left Some("edit-70c74d54b7150193") right Some("7d6bd4f3246ae47f074ed7a02011c0cf")` + `[WARN] … does not bind its approval target — mutation-id` | the published edit carries the Store authority's content-addressed `edit_id()` instead of the stamp's `mutation_id`, although `gis2d_one_item_edit`'s stamped arm sets `id = stamp.mutation_id.0`. So the parent edit is **not** going through the stamped factory, or the durable-group publish re-ids it. Same root as the WAL "mutation-id" warn | `🏪️store/🦀️.rs:14123` (`edit_id()`), stamped arm `✏️editor/🦀️.rs:566-571`, law `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:502` |
| (its abort) | `panic in a destructor during cleanup … non-unwinding panic. aborting` | `GisMapSnapshotRootRetirement::drop` runs on tokio runtime shutdown before the stores are closed; masks every post-mount assertion | `🗺️gismap/…/💾️binary/🦀️.rs:313`, law's teardown |
| `inference_wal_proof_executes_literal…` (law 1) | `"same-id-altered-envelope" left "invalid" right "absent"`, `[WARN] … Codec("checksum mismatch in footer at offset 80")` | **fixture defect, not product**: the tamper flips the *last byte of the canonical pack*, which is the footer checksum, so the container is corrupt and `invalid` is the correct product answer. To mean what its name says the trace must re-seal a pack whose envelope payload changed while `mutation_id` stayed — `durable_fixture_record` must produce a second, altered-and-resealed record. Do **not** change `expected` to `invalid` | tamper `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:211-214`, assertion `:342`, fixture trace 3 |
| `inference_wal_proof_rejects_hash_matched…` (law 2) | `a matching durable hash cannot bypass "trailing-byte-after-hlc"` | stacks on the same `durable_fixture_record` tamper machinery; its hostile bytes are swapped into the target event the same way | `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:413` |
| `gis_inference_sqlite_prepared_approval_survives_restart…` | `Result::unwrap() on Err(Conflict)` | the restart re-prepares an approval whose stamp must be re-minted from the document clock after reopen; the reconcile compares against the pre-restart command hash | `🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs:309:169` |
| `gis_map_terminal_close_waits_for_unpolled_cleanup…` | `worker pool shutdown: Busy { retained_uses: 1 }` | one pool use survives `close()` + `drop(committer)` + successful `Arc::try_unwrap(database)` + `database.shutdown()`; not a store, not the committer's `Database` | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:805` |
| `quick::gis_map_abandoned_pre_witness_request…` | `approval reaches its retained cancellation phase Preflight` | times out at the **first** phase; `poll_approval_to_phase` never observes the `Ready{pending}` turn `finish_preflight` produces | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:1324`, runtime `:1397-1404` |
| `quick::checkpoint_publication_route_rejects_stale…` | `replace publication descriptor: Conflict("document descriptor … is immutable")` | past my §2.1 fix; the law's own cross-scope fixture tries to replace an immutable descriptor — it needs a second document, not a descriptor replacement | `🔬️bin-unit/🦀️.rs:7057` |
| `admin_removal_revokes_visible_plan_presence…` | `database shutdown: Unavailable("database shutdown deadline elapsed")` | past my §2.2 fix; teardown only. The law's `stop_recovery_server` shuts a database whose retained owner is still held — same family as `Busy{retained_uses:1}` above | `🔬️bin-unit/🦀️.rs:3658` |
| `quick::retained_short_admin_request_drop…` | `competing writer terminal outcome admin-authority-unavailable reason None` | `admin_directory_authority_refusal` with a **non-`Denied`** `FencedDirectoryCommandErrorV1` on the *second* writer: its directory command fence was unavailable, not denied. Both HT3b's re-auth ranking and my `cancelled_before_effect` ranking are wrong | `🏗️bootstrap/🦀️.rs:8745`, fence `:5929`, law `🔬️bin-unit/🦀️.rs:6805` |

**Two things the next owner should take first**, because they unblock several rows: the stamped edit's
`mutation_id` not reaching the published edit (one root under the runtime law *and* both WAL
"mutation-id" warns), and the `GisMapSnapshotRootRetirement` drop trap, which turns every post-mount
failure into a site-less SIGABRT.

No source changed in batch 3, so no new check was run; `🗑️generated/ht4-check-3.txt` and
`ht4-check-kernel.txt` remain the last green `--all-targets` captures.
