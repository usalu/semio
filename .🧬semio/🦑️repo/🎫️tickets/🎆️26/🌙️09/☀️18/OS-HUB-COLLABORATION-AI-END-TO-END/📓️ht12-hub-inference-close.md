# HT12 — the hub suite's last two OWN reds: the committer close on the value store

Spec: `📓️ht11-hub-suite-final-two.md` §6–§11 + `🗑️generated/coordinator-hub-nextest-full-0207.txt`.

Slice = L1 `inference::runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply`
(`🔬️unit:819`) and L3 `…quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer`
(`🔬️unit:1515`). The other seven reds at 02:07 are sibling JC1's trusted-catalog/browser-actor work
(`trusted browser actor identity differs from its package or renderer`) — not taken.

## 1. What the 02:07 capture says — HT11's witness named a DIFFERENT phase

Both laws, byte-identical:

```
committer close: Storage, last close-store step Some("value Blocked outstanding_reads=0
  returned_reads=2 terminal_is_empty=false phase=semio/DisplacedOwners/started=true/active=false")
```

HT11 §9 assumed the refusing arm was `SemioStoreClosePhase::ReturnedLeases` and moved the sweep into
`ArtifactStore::take_returned_snapshot_read_retirement` to cure it. The witness it added in the same
round proves that assumption wrong: the phase that answers `Blocked` is **`DisplacedOwners`**, with
`started=true` and `active=false` — a different arm of the same disposer, which HT11's sweep never
touches.

Two corrections to HT11's reading of the record, both from the hub's own
`close_store` (`🏃️runtime:2443–2449`):

- `terminal_is_empty` there is `close_owned_terminal_is_empty()` =
  `owned_disposer_terminal && snapshot_read_leases_terminal_is_empty()`. A blocked disposer is never
  terminal, so `false` is unconditional and says nothing about the lease registry.
- `outstanding_reads` / `returned_reads` are the lease census, printed next to a refusal that is
  **not** the lease phase's. They are a symptom, not the blocker — but they are the *cause*, as §2
  shows.

## 2. Root cause — a phase-ordering deadlock in `SemioStoreOwnedDisposer`

`DisplacedOwners` forwards `ArtifactStoreCloseView::maintenance_retirements_step`, which drains the
store's displaced-owner queue. The snapshot owners in that queue are pushed as
`SemioSnapshotRetirementFactory::retire(Arc<P>)` = `shared_retirement`, and
`SharedRetirement::close_step` (`🧰️framework/…/🏪️store/♻️retirement/🦀️.rs:311–318`) answers
**`Blocked` whenever `Arc::try_unwrap` fails**, i.e. whenever the displaced snapshot is still
aliased.

Who aliases it: `SnapshotReadLeaseRegistry`. `ArtifactStore::snapshot_read` issues the lease with
`try_issue(owner.clone())`, and returning a lease
(`return_snapshot_read`, `🏪️store/🦀️.rs:292–300`) drops only the READER's alias — the registry keeps
its own `Arc` until the lease is *retired*. So a returned-but-unretired lease pins the snapshot it
was taken on.

Put the two together:

| turn | state |
|---|---|
| lease taken on snapshot S | registry holds an `Arc` of S |
| lease returned | registry still holds that `Arc` (`returned_reads=2`, `outstanding_reads=0`) |
| commit 1 | S moves to the tail-undo cache (`replace_current_retained`, `:15604–15616`) |
| commit 2 | the tail cache is evicted → S is pushed into the displaced-owner queue (`:15918`) |
| close, `DisplacedOwners` | `Arc::try_unwrap(S)` fails — the registry alias is still there → **`Blocked`** |
| close, `ReturnedLeases` | would drop that alias — but the cursor never gets there |

The closer is stuck behind an alias only its own next phase can release.

**The kernel's own cursor does not have this bug.** `ArtifactStoreCursorDisposer::new()`
(`🏪️store/🦀️.rs:1939`) starts at `ReturnedReads` and advances to `Displaced` (`:1976–1991`) —
returned leases first, deliberately. `SemioStoreOwnedDisposer::new()` started at `DisplacedOwners`
and advanced to `ReturnedLeases`: **the two phases were in the opposite order.** Every
`s.stdio.semio` member store — the GisMap `value` and `drawing` children among them — carried it.

## 3. Native repro — exact, in a 1-second loop

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️unit/🦀️.rs`, new law
`returned_read_leases_retire_before_the_displaced_owners_that_alias_them`: build a real
`SemioValueSnapshot` member store the way the hub's `value_store` does, take two snapshot read
leases, return both, commit twice, then drive `close_owned_step(1, 4096)` to terminal.

Pre-fix (`🗑️generated/ht12-repro-2.txt`), byte-for-byte the hub's record:

```
value store close blocked with outstanding_reads=0 returned_reads=2
  phase=semio/DisplacedOwners/started=true/active=false
```

The first attempt (`ht12-repro-1.txt`) used ONE commit and passed — the aliased snapshot sits in the
tail-undo cache after one commit and is only evicted into the displaced queue by the second. That
negative result is what pins the shape: it takes a *second* decision on the document, which is
exactly what L1 (approval, then durable undo) and L3 (approval, then abandon) each do, and exactly
why no other hub law reaches it.

## 4. Fix

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs` — the two phases are swapped into the kernel
cursor's order, and nothing else changes:

| line | change |
|---|---|
| `:832` | `SemioStoreClosePhase` declares `ReturnedLeases` before `DisplacedOwners`, with a docstring naming the registry-alias law |
| `:867` | `SemioStoreOwnedDisposer::new()` starts at `ReturnedLeases` |
| `:926` | the `ReturnedLeases` `None` arm advances to `DisplacedOwners` (was `HistoryMutations`) |
| `:936` | the `DisplacedOwners` `Complete` arm advances to `HistoryMutations` (was `ReturnedLeases`) |

Both arms keep their bodies verbatim — the same grants, the same terminal-empty witnesses, the same
`Blocked` refusals. Nothing is loosened: a `Blocked` at `ReturnedLeases` still means a lease is
genuinely OUT, and a `Blocked` at `DisplacedOwners` now means a displaced owner is shared with
someone who is *not* this store's own lease registry — a real external holder, which is what that
refusal was written for.

HT11's sweep in `ArtifactStore::take_returned_snapshot_read_retirement` is untouched and still
needed: it is what makes the `ReturnedLeases` phase drain in bounded turns instead of stalling on a
round-robin cursor miss. The two fixes compose — HT11's makes the lease phase *finish*, this one
makes the closer *reach* it.

No other disposer carries the inversion. The only other `maintenance_retirements_step` callers are
the kernel cursor (already correct) and the plugin SDK's maintenance pump stages
(`🔌️plugin/🦀️.rs:29109`, `:29125`), which yield on `Blocked` and drain returned reads on their own
stages rather than fail closed.

## 5. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht12 cargo test -p semio-s-artifact-stdio-semio --lib returned_read_leases_retire_before` (one commit — shape probe) | EXIT 0, passes: does NOT reproduce | `🗑️generated/ht12-repro-1.txt` |
| same, two commits, BEFORE the fix | **EXIT 101** — `Blocked … phase=semio/DisplacedOwners/started=true/active=false` | `🗑️generated/ht12-repro-2.txt` |
| same, AFTER the fix | **EXIT 0**, 1 passed | `🗑️generated/ht12-repro-3.txt` |
| `… cargo test -p semio-s-artifact-stdio-semio --lib` (whole crate) | **EXIT 0**, **2557 passed, 0 failed**, 1 ignored | `🗑️generated/ht12-stdio-semio-lib.txt` |
| `… cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht12-check-hub-1.txt` |

The 58 warnings are the proof the hub expansion really ran. The whole-crate run is the regression
gate for the phase swap: every other `s.stdio.semio` member-store close law (including
`a_semio_member_mints_and_reopens_a_real_child_envelope`, which closes two real member stores) is
green in the new order.

Per rule 26 no `cargo test`/`nextest`/`build` was run on `-p semio-hub`.

## 6. Files changed

| file | change |
|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs` | `SemioStoreClosePhase` / `SemioStoreOwnedDisposer`: `ReturnedLeases` is drained before `DisplacedOwners`, matching the kernel's `ArtifactStoreCursorDisposer` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/🔬️unit/🦀️.rs` | new law `returned_read_leases_retire_before_the_displaced_owners_that_alias_them` — the native repro, now a permanent regression gate |

No law was deleted, `#[ignore]`d or loosened. No hub file was touched this round.

## 7. Per-law expectation

| law | expectation after this fix |
|---|---|
| **L3** `…quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` (`:1515`) | **PASS.** Its only remaining failure was `committer close: Storage` at this exact arm; everything before the close already passed at 01:28 and 02:07. |
| **L1** `…gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` (`:819`) | **PASS.** HT11 §6 proved the durable undo applies and every assertion after it passes; it died 46 lines later in the same `committer.close()` with the same record. |

If either is still red, the record now distinguishes the cases by itself:
`phase=semio/ReturnedLeases` = a lease genuinely OUT (holder hunt resumes at
`retire_unstaged_store_member` / `abort_staged_store_member`, HT11 §2);
`phase=semio/DisplacedOwners` with `returned_reads=0` = a displaced owner shared with a holder
outside this store, which is a different defect from the one measured here.

## 8. Honest gaps

- Both laws are verified by `cargo check -p semio-hub` plus the native repro of their exact refusal;
  neither hub law was run (rule 26). The coordinator's rerun decides them.
- The repro drives `apply_one` on a standalone member store, not the hub's durable three-store group
  commit. It produces the identical record (`outstanding_reads=0 returned_reads=2
  phase=semio/DisplacedOwners/started=true/active=false`), so the arm and the alias are the same —
  but the durable-group path reaching that queue is inferred from the record, not separately
  measured.
- Still open, out of slice (HT11 §5): `InferenceMapBaseV1.pack` is the genesis pack at every
  frontier, yet `preflight_undo` / `build_assembly` read `decode_pack(base.pack)` as the current
  snapshot. Nothing is red today because they agree with each other and with the fixture.
- Not investigated: the seven JC1 trusted-catalog/browser-actor reds at 02:07.

**needs hub rerun.**

## 9. Rerun 02:45 — the phase fix LANDED and moved the census; a second blocker remains

`🗑️generated/coordinator-hub-nextest-full-0245.txt`, 321 run / 318 passed / 3 failed (the third is
JC1's browser-actor law). Both of mine, still byte-identical to each other:

```
committer close: Storage, last close-store step Some("value Blocked outstanding_reads=0
  returned_reads=0 terminal_is_empty=false phase=semio/DisplacedOwners/started=true/active=false")
```

**`returned_reads` went 2 → 0.** That is the proof the hub links the crate I fixed and that §4 did
exactly what it claimed: the lease registry is now fully drained *before* `DisplacedOwners`, so the
alias that blocked the displaced snapshot at 02:07 is gone. §2's defect is cured and will not come
back — the native law (`ht12-repro-4.txt`, still green after the `🗑️generated` wipe) pins it.

**No other disposer has the wrong order.** The repo has exactly two `ArtifactStoreOwnedDisposer`
impls: the kernel's `ArtifactStoreCursorDisposer` (`🏪️store/🦀️.rs:1953`, always `ReturnedReads`
first) and the stdio-semio one I fixed. The GisMap value store uses the latter — confirmed by the
`phase=semio/…` prefix, which only that impl emits.

**What is left is a different sub-cause in the same arm.** With BOTH lease counts at zero, nothing
the lease registry owns can be blocking. `ArtifactStoreDisplacedRetirements::close_step`
(`🏪️store/🦀️.rs:1744–1764`) has only two ways to answer `Blocked`, and they need opposite cures:

1. **a queued owner is blocked** — the front owner is a `SharedRetirement` whose `Arc` is aliased by
   a holder that is *not* a read lease. The live candidate is the durable-group root: staging clones
   the store's current snapshot into `root.tail_undo_cache`
   (`🗄️durable-group/🦀️.rs:583`), and `adopt_staged_store_member` pushes `previous_current` into the
   queue while the root may still alias it.
2. **`owners` is EMPTY and a reservation is live** (`:1749`) — a leaked displaced-owner slot. Staging
   reserves **12** slots into `root.displaced_reservation` (`🗄️durable-group/🦀️.rs:557`), and the
   ONLY two releases are `adopt_staged_store_member` (`:698`) and `abort_staged_store_member`
   (`:643`). A group root that is staged and then **abandoned without a decision** never reaches
   either — which is literally L3's name (`…abandoned_pre_witness_request…`), and L1 reaches the same
   state through its second decision. `ArtifactStore`'s own terminal predicates (`:16204`, `:16264`)
   already require `durable_group_root.is_none()`, so a staged root can never close.

Case 2 is the strong hypothesis, but it is a hypothesis: the record cannot currently tell the two
apart, and I will not guess-fix a reservation-release path on the abandon route.

**Landed this round — the record now names the sub-cause itself:**

| file | change |
|---|---|
| `🧰️framework/…/🏪️store/🦀️.rs` | `ArtifactStoreDisplacedRetirements::witness()` (`owners=N/reserved=M`) + `ArtifactStore::close_displaced_witness()`, which appends the durable-group root state (`none`/`staged`/`adopted`) |
| `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` | `close_store`'s `Blocked` record carries `displaced=owners=N/reserved=M/group=…` |

The next capture decides it in one line:

- `displaced=owners=0/reserved=12/group=staged` → **case 2**: the abandoned/undecided group root
  never released its reservation. Cure: the abandon route must `abort_staged_store_member` (or the
  closer must relinquish an undecided root) before the stores close.
- `displaced=owners=N>0/reserved=…/group=…` → **case 1**: a queued owner is externally aliased; the
  holder is named by which owner sits at the queue front, and `group=staged|adopted` says whether the
  durable-group root is the aliaser.
- `group=none` with `reserved=0` and `owners=0` would contradict the refusal and mean the census and
  the arm disagree — a third defect.

### Verification (round 2)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht12 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht12-check-hub-2.txt` |
| `… cargo test -p semio-s-artifact-stdio-semio --lib returned_read_leases_retire_before` | **EXIT 0**, 1 passed | `🗑️generated/ht12-repro-4.txt` |

Captures from before the ~02:30 machine clean were wiped; `ht12-repro-4.txt` and
`ht12-check-hub-2.txt` are the re-taken evidence. §5's earlier rows are no longer on disk.

**needs hub rerun.**

## 10. Rerun 02:52 — the census picked case 1, and named the alias holder

`🗑️generated/coordinator-hub-nextest-full-0252.txt`, 321 run / 319 passed / 2 failed (JC1's law is
green again; only L1 + L3 remain). The witness from §9 decided the fork in one line:

| law | `displaced=` |
|---|---|
| one | `owners=11/reserved=0/group=none` |
| other | `owners=2/reserved=0/group=none` |

`reserved=0` kills §9's leaked-reservation hypothesis outright, and `group=none` clears the
durable-group root. It is **case 1**: QUEUED displaced owners that `Arc::try_unwrap` cannot claim
while both lease counts are zero — a non-lease alias holder.

### The holder is the tail-undo cache, installed by the durable-group ADOPT

Staging a group member caches the pre-commit snapshot as that group's tail-undo entry —
`tail_undo_cache: Some((tail_edit_id, Arc::clone(&store.current)))`
(`🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:583`). Adoption then does two things that contradict
each other:

```rust
let previous_current = std::mem::replace(&mut *store.current, root.current.take()…);   // :674
retain_displaced_owner(…, snapshot_factory.retire(previous_current));                  // :675  ← queued
…
*store.tail_undo_cache = root.tail_undo_cache.take();                                   // :685  ← SAME Arc
```

`previous_current` at adopt time IS the `store.current` that staging cloned, so the snapshot pushed
into the displaced queue at `:675` is still aliased by the tail cache installed at `:685`.
`SharedRetirement::close_step` cannot unwrap it and answers `Blocked` — and the close cursor drains
`DisplacedOwners` **nine phases before** `TailSnapshot`, so nothing ever releases the alias. This is
§2's defect one layer down: the same "the closer blocks on an alias only a later phase can free",
with the tail cache in place of the lease registry.

`owners=11` vs `owners=2` is just how many owners each law's store had queued behind the blocked one.

The guard was already written three times elsewhere for the symmetric case — `:681` (outgoing tail
vs new current), `commit_document_roots_retained` (`🏪️store/🦀️.rs:15678`) and
`replace_tail_undo_cache_retained` (`:15915`). Only the INCOMING tail at adopt had none.
`abort_staged_store_member` is already correct: there `store.current` is unchanged, so its `:681`-
style guard fires.

**Product fix** (`🗄️durable-group/🦀️.rs:674–685`): take `next_tail` first and hand the outgoing
current to the tail cache instead of the displaced queue when the two are the same owner —
`drop(previous_current)`, exactly as the three existing guards do. The leftover reserved slot is
released by the `release_owner_slots` call that already follows.

### Verification (round 3)

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht12 cargo check -p semio-hub --all-targets` | **EXIT 0**, 0 errors, 58 warnings | `🗑️generated/ht12-check-hub-3.txt` |
| `… cargo test -p semio-framework-os-kernel --lib durable` | **EXIT 0**, **20 passed, 0 failed** | `🗑️generated/ht12-kernel-durable.txt` |
| `… cargo test -p semio-framework-os-kernel --lib store::` | 327 passed, **39 failed** | `🗑️generated/ht12-kernel-store.txt` |
| same, with MY guard neutralised (`if false && …`) | **327 passed, 39 failed — identical** | `🗑️generated/ht12-kernel-store-baseline.txt` |
| `… cargo test -p semio-s-artifact-stdio-semio --lib returned_read_leases_retire_before` | **EXIT 0**, 1 passed | `🗑️generated/ht12-repro-6.txt` |

The 39 kernel `store::` failures are **not mine, and this time it is measured rather than argued**:
the baseline row above is the same filter with my guard switched off, and the counts are identical.
`🏪️store/🦀️.rs` is `MM` in `git status` — peers hold ~43 in-flight lines there besides my 13.
The 20 durable-group laws, which are the ones that exercise `adopt_staged_store_member`, are green.

### Honest gap on this round

The adopt-path alias is **proven by construction** (`:583` clones `store.current`; `:674` takes that
same value as `previous_current`; `:685` installs the clone) and matches the capture exactly, but it
is NOT reproduced in my 1-second native law: that law drives a standalone member store, and reaching
`adopt_staged_store_member` needs a full three-store durable assembly. Extending the law to the
laws' shape ruled the cheaper holders OUT rather than in — two returned leases, two commits and a
derived `SpaceMember::undo` all close cleanly (`ht12-repro-5.txt`/`-6.txt`), which is what pushed the
search onto the group path. The kernel's own 20 durable-group laws cover adopt and stay green.

**needs hub rerun.**
