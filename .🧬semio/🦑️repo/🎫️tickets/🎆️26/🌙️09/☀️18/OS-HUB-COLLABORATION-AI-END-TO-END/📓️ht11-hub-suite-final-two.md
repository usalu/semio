# HT11 — the hub suite's final two

Spec: `📓️ht10-hub-suite-final-three.md` §13–§19 + the two laws' stderr in
`🗑️generated/coordinator-hub-nextest-full-0047.txt`.

Inherited state (capture `…-0047.txt`, `Summary [39.056s] 321 tests run: 319 passed, 2 failed`):

| law | named state at 00:47 |
|---|---|
| L1 `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `:773` — `retained durable undo: Conflict, prepare conflict Some("Published/stores_match base_frontier=false base_digest=true"), conflict arm None, identity mismatch None` |
| L3 `quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `:1515` — `committer close: Storage` |

Both are plain FAILs with named states — no hang, no SIGABRT. HT9/HT10's instrumentation paid off.

## 1. L1 — the false conjunct is `base_frontier`, and `Published` was never a post-commit join

`prepare conflict Some("Published/stores_match base_frontier=false base_digest=true")` says three
things at once, and `conflict arm None` says the refusal never reached `advance_turn` at all — it is
`prepare_retained_document`, one layer BEFORE the join HT10 named:

- `base_digest=true` — the undo's `after_base.digest()` **equals** the publication identity's
  `base_digest`. That is the pack law, measured rather than argued: the test builds `after_base.pack`
  from `owners.parent.snapshot_pack().pack`, `snapshot_pack`'s own docstring says
  *`pack` (genesis) + `spr` (event log)*, and production's `map_base` (`🏃️runtime:3446`) keeps
  `pair.pair().pack` and drops the `spr`. So every `InferenceMapBaseV1.pack` in this system is the
  GENESIS pack, at every frontier, and `after_base_digest` is `checkpoint.pack.sha256` — the same
  genesis digest the approval froze.
- `base_frontier=false` — the undo names `terminal.frontier`, i.e. the frontier the approval's
  publication PRODUCED, while `identity.base_frontier` is the frontier it was made FROM.
- Therefore the arm fell into its `stores_match(owners, decode_pack(base.pack))` fallback and
  compared the **post-approval Stores against the genesis pack**. That comparison is false by
  construction past the first commit — exactly the category error HT10 §1 removed from the *retry*
  branch and left standing on the *different base* branch.

This is not undo-specific. Any second decision on a committed Map document hits it; the undo law is
simply the first test that reaches a second decision.

**Product fix.** A `Published` state may now be joined by a base that names the frontier that
publication produced, which is the projection `publish` already verifies against the actor
checkpoint (`🏃️runtime:2116–2119`): same document, `head_edit_id == identity.mutation_id`,
`head_edit_ordinal`/`last_commit_seq` each exactly +1, same descriptor and same (genesis) pack
digest. Landed as `publication_result_base()` plus `stores_consistent()` — "the three Stores still
fold to one parent-derived triple", which is the part of `stores_match` that survives at any
frontier — and wired into the four post-commit gates the undo crosses:

| gate | change |
|---|---|
| `prepare_retained_document` `Published` arm (`:1450`) | accepts the publication's own result before falling back to `stores_match`; the record now prints `published_result` and `stores_consistent` too |
| `drive_turn` `Published` else-arm (`:1803`) | same admission for the candidate identity, so the undo routes to `Preflight` instead of `DrivePublishedIdentityAndStoresDiffer` |
| `finish_preflight` (`:2087`) | `stores_match` **or** `stores_consistent`; the base frontier is already proven by `checkpoint_matches_frontier` against the live actor checkpoint one line above, and the generation against `observed` |
| `mount_document` (`:1233`) | the `Ready`/`Published` arm split; only the `Published` half is relaxed |

Every change is strictly *additive* — it admits a base it can name exactly and rejects everything it
rejected before. `Ready` (the pre-commit family) keeps `stores_match` untouched: there the base pack
really is the live state and that comparison is the only content check there is. `build_assembly` and
`preflight_undo` are untouched: both derive the undo work from `decode_pack(base.pack)` and check it
against `matches_fixed_three_parent`, and they agree with each other and with the law's own fixture.

## 2. L3 — `Blocked` is not always terminal, and the law never printed the census

`committer close: Storage` is HT10 §15's named refusal firing. It carries no numbers because the
panic printed only the error: `record_close_store_step`'s
`value Blocked outstanding_reads=N returned_reads=M` is written but nothing reads it unless the
30 s watchdog fires, and it no longer does.

Reading the registry rather than guessing the holder produced a second, concrete defect. HT10 made
`SnapshotRetirementStep::Blocked` a hard `Storage` refusal on the strength of "`Blocked` is
permanent for the closer". It is not always: `ArtifactStoreCursorDisposer::close_step`'s
`ReturnedReads` arm answers `Blocked` whenever `take_returned_snapshot_read_retirement()` gives
`None` and the registry is non-empty, and `SnapshotReadLeaseRegistry::try_take_one_returned` answers
`None` **whenever its round-robin cleanup cursor lands on a lease that is still out** — it advances
the cursor past it and returns nothing, even when another slot is returned and retirable. That miss
is a *documented law*, not a bug: fixture case `live-read-does-not-starve-returned`
(`🧫️fixtures/♻️snapshot-read-retirement/🔣️.json`) pins the visit sequence `[null, 2, 0]` for
exactly that shape. So the registry stays as it is, and the close cursor is what must tell the two
states apart.

**Product fix** (`🏪️store/🦀️.rs`): the `ReturnedReads` arm answers `Pending` while
`snapshot_read_leases_have_returned()` — progress is available on a later turn, bounded by the
registry capacity — and `Blocked` only when the registry holds nothing but leases that are still
OUT, which is the state no closer can leave. A new `ArtifactStoreCloseView::snapshot_read_leases_have_returned()`
exposes the distinction. HT10's refusal law is preserved for the case it was written for.

**Instrumentation so the next capture is decisive if that was not the whole root:** `close_store`'s
`Blocked` record now also carries `terminal_is_empty`, and `🔬️unit:1515` prints
`last_close_store_step()` in its panic, so the numbers `outstanding_reads=N returned_reads=M` reach
the capture whether or not the watchdog fires.

**Not established:** WHO holds an outstanding value-store lease, if one is genuinely out. Traced and
excluded this round: the GisMap one-item preparation returns its `base` lease in `close_step`
(`✏️editor/🦀️.rs:703`) and `SnapshotRead`'s `Drop` returns it in any case; both
`DurableOwnedThreeStoreMapAssemblyV1::drop` and `ArtifactStoreBatchPublication::drop` assert
terminal emptiness, so neither can have leaked a live publication without panicking first; and the
`ErasedSnapshotRead` path (`snapshot_read_erased_now`) has no hub caller — its only callers are in
`🔌️plugin`. That leaves the staged member-publication abort path
(`retire_unstaged_store_member` / `abort_staged_store_member`) as the untested branch. The census
now reaches the capture.

## 3. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht11 cargo check -p semio-hub --all-targets` (L1 edits) | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht11-check-hub-1.txt` |
| same, after the kernel close-cursor edits | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht11-check-hub-2.txt` |
| `CARGO_TARGET_DIR=…/target-ht11 cargo test -p semio-framework-os-kernel snapshot_read_retirement` | **EXIT 0**, 1 passed | `🗑️generated/ht11-kernel-test-1.txt` |
| `… cargo test -p semio-framework-os-kernel` (whole `--lib`) | 1056 passed, **48 failed** — see below | `🗑️generated/ht11-kernel-test-2.txt` |

The 58 warnings are the proof the expansion really ran (zero errors alone would be meaningless).

**The 48 kernel failures are not this slice's.** The working tree carries 111 modified files from
live peers, and the failures span areas this slice never touched: `os_dsl::grammar` fails with
`found zero component.grammar.semio under ✏️s/🔌️plugins` (a discovery/environment failure from the
peers' in-flight plugin edits), plus `os_directory::schema`, `os_pack::value`, `os_spr::channel` and
`os_spr::protocol_laws`. The string `Blocked` and the phase name `ReturnedReads` appear **zero**
times in the whole 48-failure output, and the one law that covers the line this slice changed —
`snapshot_read_retirement_skips_empty_slots_and_wraps_without_starvation` — **passes**. No baseline
run was taken (rule 26's budget and one-builder etiquette), so this is an argument from the failure
texts, not a bisect.

Per rule 26 no `cargo test`/`nextest`/`build` was run on `-p semio-hub`. Both laws are therefore
**verified by `cargo check` only**; the coordinator's rerun is what decides them.

## 4. Files changed

| file | change |
|---|---|
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | `stores_consistent()` + `publication_result_base()`; the `Published` join admits the frontier its own publication produced at `prepare_retained_document`, `drive_turn`, `finish_preflight` and `mount_document`; a new `preflight/…` conflict record; `close_store`'s `Blocked` record carries `terminal_is_empty` |
| `🧰️framework/…/🏪️store/🦀️.rs` | `ArtifactStoreCloseView::snapshot_read_leases_have_returned()`; the cursor disposer's `ReturnedReads` phase answers `Pending` while a returned lease is still retirable and `Blocked` only for leases that are still OUT |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | `:1515` prints `last_close_store_step()` (instrumentation only; no law changed) |

No law was deleted, `#[ignore]`d or loosened. The `Ready` (pre-commit) gates keep `stores_match`
exactly as it was.

## 5. Honest gaps

- **L1** has a named root and a product fix that follows the spec's own rule (compare the identity's
  base frontier/digest, never advanced Stores against the genesis pack). It is `cargo check`-green
  only. If the rerun moves the failure further in, the new records name the gate:
  `published_result` / `stores_consistent` at prepare and drive, `preflight/…` at preflight.
- **L3** is the weaker of the two. The `Blocked`-vs-`Pending` distinction is a real product defect
  with a law behind it, but it only cures L3 if the value store held a returned lease the cursor
  kept missing. If a lease is genuinely OUT, the rerun now prints
  `value Blocked outstanding_reads=N returned_reads=M terminal_is_empty=false` and the holder hunt
  resumes from the member-publication abort path named in §2.
- The `InferenceMapBaseV1.pack`-is-genesis finding is broader than these two laws: `preflight_undo`
  and `build_assembly` both treat `decode_pack(base.pack)` as the CURRENT snapshot. They agree with
  each other and with the law's fixture, so nothing is red today, but on a document with two or more
  applied edits that reading is wrong. Named here, not taken — it is outside this slice.
- The kernel crate's `--lib` suite is red at 48 for reasons argued, not bisected, in §3.

**needs hub rerun.**

## 6. Rerun 01:28 — L1's join is FIXED; both laws are now one defect

`🗑️generated/coordinator-hub-nextest-full-0128.txt`, 321 run / 319 passed / 2 failed in 45 s.

| law | before (00:47) | after (01:28) |
|---|---|---|
| L1 | `:773 Conflict, prepare conflict Published/stores_match base_frontier=false` | **`:819 committer close: Storage`** |
| L3 | `:1515 committer close: Storage` (no census) | `:1515 committer close: Storage, last close-store step Some("value Blocked outstanding_reads=0 returned_reads=2 terminal_is_empty=false")` |

**L1 §1's fix held completely.** The durable undo no longer refuses: it applied, and every assertion
after it passed — the receipt, the frontier deltas (`head_edit_ordinal`, `last_commit_seq`), the
non-genesis lineage contract, the retained ingress release, the reverted three-Store pair, and the
`Replayed` idempotency admission. L1 now dies 46 lines later, at `committer.close()` — the **same**
call L3 dies in. Two laws, one remaining defect.

## 7. L3 — `outstanding_reads=0`: nothing is out, the closer was giving up on a cursor miss

The census answers the coordinator's fork outright: **no lease is OUT**. `occupied=2`,
`returned=2`, `outstanding=0` — two leases already handed back, waiting to be retired. There is no
holder to hunt inside the durable-group composition; the value store's registry is simply not being
drained. `terminal_is_empty=false` is just `returned != 0`.

Why a returned-only registry still answered `Blocked`: `try_take_one_returned` retires the first
occupied slot at or after its round-robin cleanup cursor and answers `None` when that slot is not
returned, advancing the cursor by one — the behaviour fixture case `live-read-does-not-starve-returned`
pins. §2's per-turn `Pending` relied on the hub's close loop re-entering often enough to walk the
cursor round; it evidently does not, and one miss is still one `Blocked`.

**Product fix** (`🏪️store/🦀️.rs`, `ReturnedReads` arm): the close cursor now **sweeps**. It calls
`take_returned_snapshot_read_retirement()` repeatedly inside its own turn, up to one full registry
capacity, and stops on the first owner it retires or as soon as no returned lease remains. A sweep
that still sees `have_returned()` after a whole capacity of cursor advances is a real invariant
break and is now a named error, not a silent stall. `Blocked` therefore means exactly one thing
again: leases that are still OUT — which is the state HT10 wrote the refusal for, and which this
capture proves is not what is happening.

**Witness, in case a later phase is the one refusing:** `ArtifactStoreOwnedDisposer::close_phase_witness()`
(default `"unknown"`, implemented by the cursor disposer as `<phase>/started=…/active=…`) reaches the
hub's `Blocked` record as `phase=…`. So a `Blocked` that survives this fix names its cursor phase
instead of only its store. `🔬️unit:819` also prints `last_close_store_step()` now, so L1 reports the
census too rather than a bare `Storage`.

## 8. Verification (round 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht11 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht11-check-hub-3.txt` |
| `… cargo test -p semio-framework-os-kernel snapshot_read_retirement` | **EXIT 0**, 1 passed | `🗑️generated/ht11-kernel-test-3.txt` |

The registry law still passes: the sweep drives `try_take_one_returned` harder but does not change
what it does, so `live-read-does-not-starve-returned`'s visit sequence is untouched.

**needs hub rerun.**

## 9. Rerun 01:48 — `phase=unknown` located the real disposer

`🗑️generated/coordinator-hub-nextest-full-0148.txt`, 321 run / 311 passed / 10 failed.

**The eight new reds are not this slice's.** Sorted by panic message they are one family:
seven carry `Catalog("trusted browser actor identity differs from its package or renderer")`
(`trusted_catalog::…local_stdio_gis_profile…`, `…long::linked_stdio_gis_descriptor_failures…`,
`…long::gis_map_binding_constructs…`, and the three `🔬️bin-unit:396` ones —
`checkpoint_publication_route_*` ×2 and `native_openable_stdio_provider…` — which fail on
`stdio profile digests:` while BUILDING the trusted catalog, before any store exists), one is the
browser-actor corpus law (`assertion left == right failed: "closed-wasm"`), and
`space_administration_page_v1_route_denies_a_spectator_the_author_windows` times out on
`hub state open did not complete within 15s`, downstream of the same catalog. Two trusted-catalog
fixtures — `🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` and `🧫️fixtures/👥️two-package/🔣️.json` — have
mtimes after 01:20. Sibling JC1's in-flight jco/browser-actor vocabulary work. Reported, not taken.

**L1/L3: `phase=unknown`.** The witness added in §7 paid for itself on its first capture. `unknown`
is the trait's DEFAULT `close_phase_witness`, so the store that refuses does **not** use the
kernel's `ArtifactStoreCursorDisposer` — and §7's sweep, which lives in that cursor, never ran for
it. The GisMap **value** store is a `SemioValueSnapshot` store: its owned disposer is
`SemioStoreOwnedDisposer` in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:925`, which carries
its own copy of the defect, verbatim:

```rust
SemioStoreClosePhase::ReturnedLeases => match store.take_returned_snapshot_read_retirement()? {
    Some(retirement) => { *self.active = Some(retirement); Pending }
    None if !store.snapshot_read_leases_terminal_is_empty() => Blocked,
    None => { advance phase; Pending }
}
```

`outstanding_reads=0 returned_reads=2` is exactly that arm: two returned leases, a cursor miss,
`None`, a non-empty registry — `Blocked`, and HT10's hard `Storage`.

**Product fix, moved to the one place both disposers share.** `§7`'s sweep is reverted out of the
kernel cursor and lives in `ArtifactStore::take_returned_snapshot_read_retirement` instead: it now
drives `try_take_one_returned` until it retires an owner or `has_returned()` is false, bounded by
one full registry capacity, and a sweep that still sees returned leases after a whole capacity of
cursor advances is a named error. `None` therefore means "no returned lease" for **every** caller —
the kernel cursor, the stdio-semio disposer and the `SpaceMember` delegation alike — and `Blocked`
again means only "leases still OUT". No disposer had to learn the registry's cursor rule, and
`ArtifactStoreCloseView::snapshot_read_leases_have_returned()` was removed again with it.

`SemioStoreOwnedDisposer` also implements `close_phase_witness` now (`semio/<phase>/started=…/active=…`),
so the next `Blocked` on that store names its phase instead of answering `unknown`.

## 10. Verification (round 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht11 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht11-check-hub-4.txt` |
| `… cargo test -p semio-framework-os-kernel snapshot_read_retirement` | **EXIT 0**, 1 passed | `🗑️generated/ht11-kernel-test-4.txt` |

The hub check compiles `semio-s-artifact-stdio-semio` natively, so the plugin-side edits
(`#[derive(Debug)]` + `close_phase_witness`) are type-checked by it.

## 11. Hand-over

| item | state |
|---|---|
| L1 join (`publication_result_base` + `stores_consistent`) | **DONE** — proven by the 01:28 rerun: the durable undo applies and every assertion after it passes |
| L1 + L3 remaining cause | ONE defect, shared: `committer.close()` on the **value** store |
| Root, named and measured | `outstanding_reads=0 returned_reads=2` — nothing is leaked; the close gave up on a `try_take_one_returned` cursor miss |
| Fix landed this round | the sweep moved into `ArtifactStore::take_returned_snapshot_read_retirement` — covers the kernel cursor AND `SemioStoreOwnedDisposer`, which is the disposer that actually refuses |
| Verified | `cargo check -p semio-hub --all-targets` EXIT 0 (`ht11-check-hub-4.txt`); kernel law passes (`ht11-kernel-test-4.txt`). No hub test run (rule 26) |
| If still red | the record now prints `phase=semio/<phase>/…`; a `Blocked` there is a genuinely OUT lease and the holder hunt resumes at `retire_unstaged_store_member` / `abort_staged_store_member` (§2) |
| Eight new reds at 01:48 | JC1's — all `trusted browser actor identity differs from its package or renderer` / `"closed-wasm"`; two catalog fixtures touched after 01:20. Not taken |
| Open, out of slice | `InferenceMapBaseV1.pack` is the genesis pack at every frontier, yet `preflight_undo`/`build_assembly` read it as the current snapshot (§5) |

**needs hub rerun.**
