# HT3a — hub `inference::` reds (8 of the suite's last 18)

Baseline (coordinator rerun 15:20, `🗑️generated/coordinator-hub-nextest-full.txt`): **321 — 303 passed / 18 failed**.
Mine: 8, all under `inference::`. Rule 26 binds me: I may not run `cargo test`/`nextest` on `-p semio-hub`,
so **nothing below is verified at runtime** — only `cargo check -p semio-hub --all-targets` (EXIT 0) is measured.

| # | law | panic (file:line) | message |
|---|---|---|---|
| 1 | `wal::tests::inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces` | `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:303` | `"exact-committed-command"` left `"absent"` right `"verified"` |
| 2 | `wal::tests::inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands` | `…/🔬️unit/🦀️.rs:374` | `a matching durable hash cannot bypass "trailing-byte-after-hlc"` |
| 3 | `wal::tests::chain::inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch` | `…/⛓️chain/🦀️.rs:232` | `actual retained WAL "one-segment-three-commits"` left `false` right `true` |
| 4 | `wal::tests::chain::quick::inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof` | `…/⛓️chain/🦀️.rs:321` | `assertion failed: verifier.close_steps() > 0` |
| 5 | `sqlite::tests::gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once` | `…/🔬️unit/🦀️.rs:338:163` | `Option::unwrap()` on `None` |
| 6 | `runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:494` | `public checkpoint refusal remains a retained nonterminal publication, got Some(Denied)` |
| 7 | `runtime::tests::gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission` | `…/🔬️unit/🦀️.rs:804` | `worker pool shutdown: Busy { retained_uses: 1 }` |
| 8 | `runtime::tests::quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `…/🔬️unit/🦀️.rs:1323` | `approval reaches its retained cancellation phase: Elapsed(())` |

## 1. Triage

### 1.1 What the two named recent changes do **not** explain

The slice brief asked me to check M6 (`AuthSessionKind::Agent` / principal kinds) and KD2
(`CheckpointPublicationBlobV1` + `blake3` CAS address) against the WAL's canonical bytes first.
**Neither touches this lane.** Measured:

* `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` mtime `09-19 03:02`, `🧾️wal/🦀️.rs` unchanged before today, the whole
  `💡️inference/` production tree is untouched by today's M6/KD2 work (`find -newermt` over `🌎️hub`: the only
  files newer than 09-20 00:08 are `🔐️auth`, `📇️directory`, `🗿️artifact-authority`, `🏗️bootstrap`, `🔬️bin-unit`).
* The WAL actor grammar is `user:<32hex>#session:<32hex>` (`🧾️wal/🦀️.rs:63-66`) and the fixture's actor is
  `user:a×32#session:b×32` — no principal kind reaches it.
* `CheckpointPublicationBlobV1` is not referenced anywhere under `💡️inference/🧾️wal` or `💡️inference/🏃️runtime`.
* The fixture is internally consistent with the production mutation-id derivation: recomputing
  `sha256("semio.hub.inference-approval-mutation/v1\0" + jobId + "\0" + proposalHash)[..32]` over
  `🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` gives exactly its `command.mutationId`
  (`e9dc772df5a1648204db93ff78e9346b`). So H1b's R1 fixture repair is in the tree **and compiled** (test
  file mtime `09-20 00:06`, nextest run `15:20`) — and the law is still red. **R1 as stated is closed and
  was not the whole root.**

### 1.2 Laws 1 + 5 are one root: the clean committed event does not bind

Trace `exact-committed-command` writes the **untampered** `durable.record.canonical_pack()` and expects
`verified`; it gets `absent`, i.e. `Ok(None)`, i.e. `matched_transaction` was never set in
`🧾️wal/🦀️.rs::scan`. Law 5 is the same fact one call deeper: `🔬️unit/🦀️.rs:338` is
`committed_fixture_witness()`'s `.unwrap().unwrap()`, and column `163` is the **second** unwrap — the
`Option`, not the `Result`. So `verify` returned `Ok(None)` there too.

`Ok(None)` (rather than `Err(Invalid)`) narrows it to exactly three silent gates, all of which threw the
reason away before this slice:

1. the target transaction's event does not start with `\x89SEMIO\r\n\x1a\n` → the event is skipped entirely
   (`🧾️wal/🦀️.rs:430`);
2. `durable_decision_event_match`'s binding gate (`:255-261`: forward count, mutation-meta count,
   dependencies, `mutation_id`, edit `actor`, `author_id`) returns `(_, _, false)`;
3. the proposal-hash or command-hash comparison returns `false`.

All three returned a bare `false`/skip. That is why three sessions of reading could not name the stage.
**Fixed here by surfacing, see §2.1.**

### 1.3 Law 3 is a fixture-**producer** defect, independent of §1.2

`⛓️chain/🦀️.rs::chain_segments` corrupts the wrong transaction:

```rust
let transactions = if count == 1 { 1..=2 } else { index + 1..=index + 1 };
for tx in transactions {
    …
    let mut event = durable.record.canonical_pack().to_vec();
    if tx == 1 { let last = event.len() - 1; event[last] ^= 1; }   // ← always the TARGET
```

The target transaction is `trace["receiptTransactionId"].unwrap_or(1)` = **1**, and the target segment is
`0`. So in the one-segment layout (`1..=2`) and in the two-segment layout (segment 0 carries tx 1) the
helper corrupts *the very transaction the law then asks the verifier to prove*. Cases
`one-segment-three-commits` and `two-segments-exact-prior-tip` both declare `accepted: true`, which is
unsatisfiable: whichever way the pack decoder treats a flipped trailing byte — hard decode error
(`Err(Invalid)`) or a decodable-but-no-longer-binding decision (`Ok(None)`) — `matches!(result, Ok(Some(_)))`
is `false`. The two accepted cases can never pass as written, which matches the observed
`left: false, right: true`.

The second transaction exists to give the one-segment case its *third SPR commit* (the law's own name);
making the **non-target** transaction carry a different event is what the helper was reaching for. The
hostile corpus is unaffected: `record-crc-repaired` ("alter-nontarget-**record**" = a non-commit record)
still flips the first `WAL_EVENT` frame — the target's — repairs its CRC and leaves the SPR commit digest
stale, so it still must be rejected. **Fixed here, see §2.2.** Under both decoder readings the corrected
helper yields the declared verdict for every one of the 14 cases; that is the argument, not a run.

### 1.4 Laws 6 + 8 are one root (H1 §13's R6), and law 4 rides on §1.3

Law 6 wants the publisher's refusal to surface as `Err(Storage)`; it gets `Denied`, which is what
`GisMapApprovalCommitErrorV1::Rejected` maps to. `Rejected` comes from exactly one place on this path —
`🏃️runtime/🦀️.rs:1420-1426`, the `DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal` arm — so the
fixed-three assembly refuses **before** the journal ever begins, and the route can never reach the
publisher. The arm takes `take_terminal_owners()` and drops `terminal.failure`
(`Cancelled | Admission{role,reason} | Preparation{role,reason} | Binding(error)`), so the stage is lost.

Law 8 is the same defect seen from the other side: `poll_approval_to_phase(…, Journal)` waits for the
`Journal` state, which the Terminal arm never produces, and times out (`Elapsed(())`). Its `.expect(…)`
did not even say *which* of `Preflight | Assembly | Journal` timed out.

Law 4's `assert!(verifier.close_steps() > 0)` at `⛓️chain/🦀️.rs:321` sits after the retained-boundary
block, which reuses `chain_segments` — §1.3's corrected helper is a prerequisite for reading it honestly.

### 1.5 Law 7 — not diagnosable without a run, and the drop witness is **gone**

`pool.shutdown()` answers `Busy { retained_uses: 1 }`. Note that the `semio-pool-worker-1` /
`🏪️store:18399` "store reached Drop without its terminal-empty shallow-shell witness" panic quoted in
HT1 §26 **does not appear anywhere in the 15:20 capture** (`grep -c 'shallow-shell\|semio-pool-worker'` = 0).
The store-lifecycle half of R6 is therefore already closed; what remains is one un-retired worker-pool use
after `committer.close()`, `drop(committer)`, a successful `Arc::try_unwrap(database)` and
`database.shutdown()`. Since `try_unwrap` succeeds, the committer really did release the `Database`, so the
leaked use is held by something the database shutdown does not retire. Honest gap: I cannot bisect that
without running the binary.

## 2. Fixes landed

### 2.1 The WAL verifier now names the gate that closed (product, observability)

`🌎️hub/💡️inference/🧾️wal/🦀️.rs` — every silent refusal in §1.2 now emits one `[WARN]` line, following the
hub's existing stderr convention (`🏗️bootstrap/🦀️.rs:2408`, `🗄️stores/🦀️.rs:294`); no behaviour, wire code or
outcome changes:

| site | line |
|---|---|
| `admit_canonical` failure (`:245`) | `[WARN] inference wal: committed event is not an admissible durable decision — {error:?}` |
| `verify_fixed_three_edits` failure (`:252`) | `[WARN] inference wal: committed decision fails fixed-three verification — {error:?}` |
| binding gate (`:262-283`), now a discriminated `unbound` ladder | `… does not bind its approval target — forward-count \| mutation-meta-count \| mutation-dependencies \| mutation-id \| edit-actor \| author-id` |
| proposal hash (`:286`) | `… — proposal-hash` |
| command hash (`:305`) | `… — command-hash` |
| a receipt-transaction event that is not a pack at all (`scan`, `:455`) | `[WARN] inference wal: receipt transaction carries an event that is not a durable decision pack` |

### 2.2 The chain producer corrupts the non-target transaction

`🌎️hub/💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs:146` — `if tx == 1` → `if tx != 1`. Nothing is deleted,
ignored or loosened; every case keeps its declared `accepted` verdict and the CRC-repair corpus still
tampers the target's own frames. See §1.3 for why this is the producer's defect and not the law's.

### 2.3 The discarded assembly terminal failure is surfaced (H1 §13.3 / R6)

`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1423` — the `Terminal` arm now emits
`[WARN] inference runtime: fixed-three assembly refused before the journal for {key} — {terminal.failure:?}`
before the owners are moved into `Ready`. The route still answers `Rejected`/`Denied`, so no passing law
changes; the stage (`Admission{role,reason}` / `Preparation{role,reason}` / `Binding(..)` / `Cancelled`)
now reaches the operator and the next nextest capture.

### 2.4 Law 8's timeout names its phase

`🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:200,1323` — `AbandonedApprovalPhaseV1` gains
`#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the `.expect(…)` became
`panic!("approval reaches its retained cancellation phase {phase:?}")`. A message-only change: the law's
assertions are untouched.

## 3. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3a cargo check -p semio-hub --all-targets` (after §2.1–§2.3) | **EXIT 0, 0 errors**, `semio-hub` recompiled in 45.3 s | `🗑️generated/ht3a-check-1.txt` |
| same, after §2.4 | **EXIT 0, 0 errors, 312 warnings emitted** (warnings prove the expansion completed), 12.3 s | `🗑️generated/ht3a-check-2.txt` |

Not run, and not claimed: any hub test. Rule 26 reserves `cargo test`/`nextest -p semio-hub` to the
coordinator.

## 4. What the next rerun must answer

1. **Laws 1/5** — which `[WARN] inference wal: …` line precedes the `"exact-committed-command"` panic. That
   single token (`mutation-id` vs `edit-actor` vs `proposal-hash` vs `command-hash` vs a decode error vs
   "not a durable decision pack") names the defect outright and turns the remaining work into one edit
   through the fixture's producer (`durable_fixture_record` / `durable_owned_group_journal_test_record_from_edits`).
2. **Laws 6/8** — the `DurableOwnedThreeStoreMapAssemblyFailureV1` stage and reason from
   `[WARN] inference runtime: fixed-three assembly refused before the journal …`, plus which phase law 8
   now reports.
3. **Law 3** — whether §2.2 flips `one-segment-three-commits` and `two-segments-exact-prior-tip`, and
   whether `alter-nontarget-record-repair-crc-retain-old-commit` stays rejected.
4. **Law 4** — `close_steps()` after §2.2; and **law 7** — unchanged, still needs a run to bisect the one
   retained worker-pool use.

## 5. Files changed

| file | change |
|---|---|
| `🌎️hub/💡️inference/🧾️wal/🦀️.rs` | §2.1 — six discriminated `[WARN]` reasons; the binding gate became an `unbound` ladder; the command-hash comparison named |
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | §2.3 — the `Assembly → Terminal` arm no longer discards `terminal.failure` |
| `🌎️hub/💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs` | §2.2 — `chain_segments` corrupts the non-target transaction |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | §2.4 — `AbandonedApprovalPhaseV1` derives `Debug`; the phase reaches the timeout message |

Nothing outside `🌎️hub/💡️inference/` was touched; HT3b's `🏗️bootstrap` / `🔬️bin-unit` are untouched, and no
public enum gained a variant, so no exhaustive `match` outside this module is affected.

**needs hub rerun** — four files landed, `cargo check -p semio-hub --all-targets` EXIT 0 / 0 errors.
Expect law 3 (and possibly law 4) to flip on the producer fix, and expect the other six to fail with a
`[WARN]` line in stderr that names the stage.

---

# HT3a — batch 2 (coordinator rerun 16:16: **321 — 304 / 17**)

All 8 inference laws still red, but the batch-1 instrumentation did its job: **both silent roots are now
named, and one of them is a one-token product defect.** The whole capture contains exactly 5 `[WARN]` lines
(`grep -a WARN`): four wal, one runtime.

## 6. Root A — the WAL reader tests for a magic prefix the framework never writes **(fixed)**

```
[WARN] inference wal: receipt transaction carries an event that is not a durable decision pack   × 4
```

Not one `admit_canonical`, `verify_fixed_three_edits`, binding-gate, proposal-hash or command-hash warning
fired — the committed decision event was **never decoded at all**. The guard was

```rust
if exact.starts_with(b"\x89SEMIO\r\n\x1a\n") {        // 🧾️wal/🦀️.rs:451 — 10 bytes
```

and the framework's binary envelope magic is

```rust
pub const BINARY_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];   // 🧬️semio/🦀️.rs:115
```

`\x89SEM\r\n\x1a\n`, **eight** bytes — the hub spelled `SEMIO` where `wrap_binary` writes `SEM`. Every
durable decision event ever committed to the WAL therefore fails the prefix test and is skipped in silence,
so `scan` returns `Ok(None)` for a perfectly valid proof. This is a real production defect on the
approval→commit→proof path, not a test defect: `verify` can never witness any committed approval.

It is the single root of laws 1, 3 and 5, and it also explains 2 and 4:

* law 1 `"exact-committed-command"` → `absent`;
* law 5 = the same `Ok(None)` through `committed_fixture_witness()`'s second `unwrap` (`:338:163`);
* law 3 — §1.3's producer fix was necessary but not sufficient: with no event ever decoded, the two
  `accepted: true` chain cases stay `false`;
* law 2 expects `Err(Invalid)` from hostile command bytes, which can only come from inside
  `durable_decision_event_match` — unreachable behind the wrong prefix;
* law 4's `close_steps() > 0` sits after the same never-decoded replay.

**Fixed:** `🌎️hub/💡️inference/🧾️wal/🦀️.rs:451` now tests
`exact.starts_with(&directory::os_store::semio_format::BINARY_MAGIC)` — the producer's own constant, so the
two can no longer drift.

## 7. Root B — the Hub's server-stamped one-item edit can never satisfy the Store authority **(named, owner needed)**

```
[WARN] inference runtime: fixed-three assembly refused before the journal for v1:32:32:cc…dd…
       — Preparation { role: "parent", reason: "validation failed: one-item semantic edit
         disagrees with its immutable Store authority" }
```

H1 §13.3's discarded terminal is now discriminated, and the stage is `Preparation{parent}`. The
disagreement is exactly one field:

| site | fact |
|---|---|
| `✏️s/…/🗺️gismap/…/✏️editor/🦀️.rs:564-571` | `gis2d_one_item_edit`'s **stamped** branch sets `meta.timestamp = stamp.timestamp` (the Hub's `identity.timestamp`); the unstamped branch uses `authority.next_clock()` |
| `🧰️framework/…/🏪️store/🦀️.rs:14158` | `validate_semantic_edit` refuses unless `meta.timestamp == self.next_clock` |
| `🧰️framework/…/🏪️store/🦀️.rs:16531-16532` | `next_clock = self.clock; next_clock.tick(now_ms())` — a wall-clock tick of the Store's own clock, which a server stamp can never equal |

So `gis_map_*_stamped_one_item_preparation_factory` — used **only** by the Hub committer
(`🏃️runtime/🦀️.rs:1353-1355`) — has never been able to prepare a single edit. The stamp is not optional:
`durable_decision_event_match` reconstructs the canonical command with `timestamp: meta.timestamp` and
compares it to the prepared `command_hash` (`🧾️wal/🦀️.rs:298`), so the committed edit's HLC **must** be the
server-stamped one. The Store authority is the side that has to learn about the stamp, either by carrying
it on `ArtifactStoreOneItemLiveAuthority` or by admitting it in `validate_semantic_edit` (and its twins at
`:16704`, `:16706`, `:16837`).

I did **not** make that change: it lands in `🏪️store/🦀️.rs` (a shared 18k-line hot file under live peer
edits) and in the gis plugin editor — both outside my slice's `inference/` boundary, and it is a contract
decision between the Store authority and the Hub committer rather than a patch. It is the root of law 6
(`Denied` instead of `Storage`) and needs an owner.

## 8. Law 8 is **not** R6 — it never reaches Preflight

With §2.4's phase in the message the timeout now reads `approval reaches its retained cancellation phase
Preflight` (`🔬️unit/🦀️.rs:1324`) — the **first** phase, not `Journal`. The capture's single runtime WARN is
law 6's, so law 8's three commits never reach assembly at all. This is a third, still-unnamed root upstream
of the assembly (mount/preflight admission), not the discarded terminal. Honest gap.

## 9. Law 7 unchanged

`worker pool shutdown: Busy { retained_uses: 1 }` (`🔬️unit/🦀️.rs:805`), no new evidence; the
`🏪️store:18399` drop witness stays absent. Still needs a run to bisect which handle is not retired.

## 10. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht3a cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors, 306 warnings emitted**, 1 m 02 s | `🗑️generated/ht3a-check-3.txt` |

File changed in batch 2: `🌎️hub/💡️inference/🧾️wal/🦀️.rs` (one line, §6).

**needs hub rerun** — expect laws 1, 2, 3, 4 and 5 to flip on the magic fix (they are one root). Laws 6
and 8 will stay red: 6 needs the Root B owner decision (§7), 8 has a third root (§8), 7 is untouched (§9).

---

# HT3a — batch 3 (coordinator rerun 16:34: **321 — 306 / 15**)

The magic fix worked: the wal WARN count fell 4 → 1 (the one that remains is the `missing-command`
trace's deliberately non-pack event, which is the contract) and **every wal law moved to a later gate**.
None flipped, because each was hiding a second defect behind the first.

| law | gate at 16:16 | gate at 16:34 |
|---|---|---|
| 1 | `:303` `absent` vs `verified` | **`:276`** `assert!(witness.matches(…))` — the event now decodes, binds and is witnessed |
| 3 | `"one-segment-three-commits"` | **`"valid-segment-chain-wrong-stored-prior-tip"`** — batch 1's producer fix flipped both `accepted: true` cases |
| 5 | `🧾️wal/…:338` `Option::unwrap` on `None` | **`🪶️sqlite/…:309:169`** `Result::unwrap` on `Err(Conflict)` |
| 2 | `:374` | `:374` (unchanged) |
| 4 | `⛓️chain/…:321` | `⛓️chain/…:321` (unchanged) |
| 8 | `Journal` (assumed) | `Preflight` (§8) |

## 11. Root B landed — an explicit stamped-preparation path, no relaxed equality

Decision taken as directed: **the stamp wins**, because `durable_decision_event_match` rebuilds the
canonical command with `timestamp: meta.timestamp` and compares it to the prepared `command_hash`
(`🧾️wal/🦀️.rs:298`) — a Store-minted wall-clock tick could never reproduce it. So the committer's clock is
carried *into* the authority rather than checked loosely against it, and
`validate_semantic_edit`'s `meta.timestamp == self.next_clock` (`🏪️store/🦀️.rs:14158`) is **untouched**: it
still demands exact equality, and now that equality holds by construction.

| file | hunk |
|---|---|
| `🧰️framework/…/🏪️store/🦀️.rs:14277` | `ArtifactStoreOneItemPreparationFactory::stamped_clock() -> Option<HybridLogicalTimestamp>`, **defaulted to `None`** — additive, so every existing factory impl in the repo keeps compiling and every local gesture keeps the Store's own clock |
| `…/🏪️store/🦀️.rs:14293`, `:14309` | the same defaulted method on the private `ArtifactStoreBatchItemAuthority`, forwarded by the `Arc<dyn …PreparationFactory>` impl |
| `…/🏪️store/🦀️.rs:16537-16541` | at the authority mint: a declared stamp replaces `next_clock` **only if** `(physical_ms, logical)` is strictly after the Store's own clock; otherwise the admission is refused with the typed reason `"stamped publication clock is not strictly after the Store's own clock"` before any owner is reserved. The commit path already does `self.clock = next_clock`, so the Store clock advances monotonically past the stamp |
| `✏️s/…/🗺️gismap/…/✏️editor/🦀️.rs:600` | `Gis2dOneItemPreparationFactory::stamped_clock()` returns its `GisMapOneItemStampV1::timestamp`; `gis2d_one_item_edit`'s stamped branch is unchanged and now agrees with the authority it is validated against |

**Native store law added** (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`):
`artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_behind` — a stamp strictly ahead
admits, publishes one edit, and the committed `meta.timestamp` **and** `store.clock` are the stamp; the
same stamp replayed once the clock has reached it is refused with the exact typed reason and publishes
nothing. `DemoOneItemPreparationFactory` gained a `stamped(clock)` constructor; its preparation is
unchanged, which is the point — it builds the edit from `authority.next_clock`, so carrying the stamp into
the authority is all that was missing.

**Laws my hunk touches:** none existing. The trait methods are defaulted and the mint change is inert for
every `stamped_clock() == None` caller, i.e. everything except the Hub committer; the kernel's
`--all-targets` check is green with the new law compiled in.

## 12. Still open, with what is now known

1. **Law 1 (`:276`)** — the witness is produced and binds `durable.*`, but `witness.matches(…)` is called
   with the fixture's **frozen** `proposalHash` / `command.mutationId` / `commandHash`. Those literals are
   stale against today's `GisMapInference::create_region_group_work`; the fixture
   `🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` needs regenerating **through its producer**
   (`durable_fixture_record`), not editing by hand. I cannot regenerate it without running hub code
   (rule 26) — `encode_server_stamped_command_v1` lives in `💡️inference/✉️command`.
2. **Law 3** — `valid-segment-chain-wrong-stored-prior-tip` is `accepted: false` but now returns
   `Ok(Some(_))`. `verify_retained` opens at `target.receipt.segment_index` and `scan` breaks as soon as
   the receipt's transaction commits, so the **following segment's stored prior tip is never validated** —
   exactly the half of the law's own title (`…and_exact_cross_segment_tip_mismatch`) that has no
   implementation. Real product gap, not a fixture defect.
3. **Laws 2, 4** — unchanged, behind 1 and 2 above.
4. **Law 8** — times out at `Preflight`, upstream of assembly; unnamed third root.
5. **Law 7** — `Busy { retained_uses: 1 }`, unchanged.

## 13. Verification (batch 3)

| command | result | capture |
|---|---|---|
| `cargo check -p semio-framework-os-kernel --all-targets` (the crate that actually mounts `🏪️store`, not `semio-framework-plugin`) | **EXIT 0, 0 errors, 17 warnings** | `🗑️generated/ht3a-check-kernel.txt` |
| `cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors, 304 warnings** | `🗑️generated/ht3a-check-4.txt` |
| `cargo check -p semio-framework-plugin --lib` | **EXIT 0, 0 errors** | `🗑️generated/ht3a-check-plugin-lib.txt` |
| `cargo check -p semio-framework-plugin --all-targets` | **EXIT 101 — not mine and pre-existing**: `E0433 cannot find LocalizedLabel in semio_framework` at `🔌️plugin/…/🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs:19`, a peer's in-flight break (preamble rule 3) | `🗑️generated/ht3a-check-plugin.txt` |

All four with rule 25's `CARGO_TARGET_DIR=…/target-ht3a`.

Files changed in batch 3: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`,
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.

**needs hub rerun** — Root B is landed, so law 6 should now reach the publisher (`Storage`, not `Denied`)
and law 8 should move past `Preflight`; if the assembly still refuses, the `[WARN] inference runtime: …`
line will name the next stage. Please also run
`cargo nextest -p semio-framework-os-kernel -E 'test(stamped_publication_clock)'` — it is the new native
store law and I may not run it myself.

---

# HT3a — batch 4 (rerun 16:50: **321 — 306 / 15**; the new store law PASSES)

`artifact_store_stamped_publication_clock_is_admitted_ahead_and_refused_behind` **PASS** 1/1
(`🗑️generated/coordinator-kernel-stamped-law.txt`). Root B is wired end to end: law 6's stage moved from
`Preparation{parent, "semantic edit disagrees…"}` to **`Admission{parent, "stamped publication clock is not
strictly after the Store's own clock"}`** — my own new gate, reached and refusing. See §16 row 6.

## 14. Law 3's missing half implemented — the cross-segment tip chain is now walked to the end

The `db` replay cursor **already** verifies the link: `WalSegmentChain::check_segment_header`
(`🛢️db/📝️wal/🦀️.rs:1548-1572`) compares a segment header's `prev_chain_hash` against
`WalPriorChainTip::Verified(tip)`, the previous segment's last commit hash, and answers
`DbError::Corrupt` on a mismatch. The hub simply never reached it: `scan` set `target_finished` as soon as
the receipt's transaction committed and `break`-ed, so no later segment was ever opened. Tampering with a
later segment's stored prior tip was therefore invisible — the exact half of
`…_and_exact_cross_segment_tip_mismatch` that had no implementation.

`🌎️hub/💡️inference/🧾️wal/🦀️.rs::scan` now replays to `WalReplayStep::Done`:

* the `target_finished` early break and its assignment are **gone** — the witness is still recorded at the
  receipt's commit, but the replay keeps walking so every later segment header is opened and chained;
* a later `SegmentHeader` is no longer a silent stop: it must carry the same document, must have
  `prev_chain_hash.is_some()`, and its index must be exactly `last_segment + 1` (`:366-382`). The hash
  value itself is the cursor's verified `Verified(tip)` comparison, so the hub does not re-implement the
  chain — it stops discarding it.

Expected effect: `valid-segment-chain-wrong-stored-prior-tip`, `…wrong-document` and `…skipped-index` now
surface as `Err(Storage)` (`Ok(Some(_)) == false`, their declared `accepted: false`), while
`two-segments-exact-prior-tip` keeps its match — its tampered event sits in the non-target transaction,
which is never decoded. Cost: the verifier now reads the retained log to its end instead of stopping at
the receipt; that is bounded by `InferenceOperationControlV1`'s fuel and the 2 s replenish deadline
exactly as the pre-target region already was, and it is what WAL integrity requires.

## 15. Law 1's producer — the exact invocation, for the coordinator to run

The producer is `durable_fixture_record(&fixture)`
(`🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs:55`): it runs `GisMapInference::infer(&base)
.create_region_group_work(&base, jobId)` over the literal descriptor at `:64`, then derives
`proposal_hash`, `mutation_id` and `command_hash` through `encode_server_stamped_command_v1`. It is **not**
an `#[ignore]`d producer and there is no standalone binary — everything it needs lives inside
`semio-hub`'s `cfg(test)` tree, so under rule 26 only you can run it.

I made the law print it. `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:276` is now an `assert!` with a message
(assertions unchanged, nothing loosened), so **the ordinary rerun is the producer run**:

```
CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub \
  cargo nextest run -p semio-hub -E 'test(inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces)' --no-capture
```

On failure it prints, in order, the six values:

```
frozen fixture literals disagree with the producer — proposalHash <frozen> vs <produced>,
command.mutationId <frozen> vs <produced>, commandHash <frozen> vs <produced>
```

Write the three `<produced>` values into `🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` as
`proposalHash`, `command.mutationId` and `commandHash`. **Caveat measured already:** the law's earlier
asserts at `:221-222` (`encodedHex`, and `sha256(encode_envelope(envelope(fixture))) == commandHash`)
**pass today**, so `encodedHex` and `commandHash` are self-consistent with `command.diff.payloadHex` /
`command.inverse.payloadHex`. If `commandHash` is one of the three that moved, then those two payload
hex blobs are stale too and must be re-emitted from the same `work.parent` / `work.parent_inverse` —
in that case do not hand-patch; say so and I will add the producer read-out for the payloads as well.

## 16. Hand-over — laws 2/4/6/7/8

| law | named gate / reason now printed | root hypothesis | file:line |
|---|---|---|---|
| 2 | `a matching durable hash cannot bypass "trailing-byte-after-hlc"` — expects `Err(Invalid)`, gets something else | reachable only once §15's literals are current; the hostile bytes are swapped into the *target* event, so it stacks directly on law 1 | `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:374` |
| 4 | `assertion failed: verifier.close_steps() > 0` in the `retainedBoundaries` block | the boundary cases open at a deleted segment 0, so `verify_retained` fails in `open_at_segment` **before** the close loop that increments `close_steps`; the close accounting must cover a replay that never opened | `🧾️wal/🦀️.rs:307-325`, law at `⛓️chain/🦀️.rs:321` |
| 6 | `Admission { role: "parent", reason: "stamped publication clock is not strictly after the Store's own clock" }` | **mine, and the next real defect**: the Hub stamps the prepared command's HLC (`physical_ms ≈ 1_000` in the fixture) while the mounted Store's clock is a wall-clock tick (`now_ms()`, ≈ 1.7e12). The stamp must be minted *from the document's own clock* at approval-preparation time — `identity.timestamp` is set at `🏃️runtime/🦀️.rs:1339` from the prepared command, and nothing merges it with `parent.generation_now()`'s clock | `🏃️runtime/🦀️.rs:1339`, gate at `🏪️store/🦀️.rs:16537` |
| 7 | `worker pool shutdown: Busy { retained_uses: 1 }`; the `🏪️store:18399` drop witness stays absent across three reruns | one pool use survives `committer.close()` + `drop(committer)` + a **successful** `Arc::try_unwrap(database)` + `database.shutdown()`, so it is held by something the database shutdown does not retire — not a store, not the committer's `Database` | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:805` |
| 8 | `approval reaches its retained cancellation phase Preflight` | times out at the **first** phase, upstream of assembly, so it is not R6; `poll_approval_to_phase` never observes the `Ready{pending}` turn `finish_preflight` produces | `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:1324`, runtime `:1397-1404` |

## 17. Verification (batch 4)

| command | result | capture |
|---|---|---|
| `cargo check -p semio-hub --all-targets` (after §14) | EXIT 101 → one `E0689` ambiguous integer on my own `last_segment`, fixed with `: u64` | `🗑️generated/ht3a-check-5.txt` |
| `cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors, 305 warnings** | `🗑️generated/ht3a-check-6.txt` |
| `cargo check -p semio-hub --all-targets` (after §15) | **EXIT 0, 0 errors, 307 warnings** | `🗑️generated/ht3a-check-7.txt` |

Files changed in batch 4: `🌎️hub/💡️inference/🧾️wal/🦀️.rs`,
`🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs`.

**needs hub rerun** — expect laws 3 and possibly 4 to flip on §14, and expect law 1's panic to carry the
three produced literals from §15. Laws 2, 6, 7, 8 stay red with the roots named in §16; law 6's next hop
is the Hub's own stamp derivation, which is inside `inference/` and is the first thing I would take next.
