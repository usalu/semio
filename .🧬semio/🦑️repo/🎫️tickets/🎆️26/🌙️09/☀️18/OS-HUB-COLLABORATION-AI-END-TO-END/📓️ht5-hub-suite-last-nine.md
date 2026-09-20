# HT5 — hub suite, the last nine (outcome 2's gate to green)

Slice HT5. Spec: `📓️ht4-hub-suite-final-reds.md` batch 3 hand-over table +
`🗑️generated/coordinator-hub-nextest-full.txt` (17:52 run: **321 — 312 passed / 9 red**).
Rule 26 binds me: no `cargo test`/`nextest`/`build` on `-p semio-hub`; `cargo check -p semio-hub
--all-targets` under rule 25's private `CARGO_TARGET_DIR=…/target-ht5` only.

## Work list (HT4 batch 3)

| # | law | HT4's reading | HT5 status |
|---|---|---|---|
| A | `…approval_committed_event_reaches_actor_frontier…` | stamped `mutation_id` never reaches the published edit (SIGABRT) | **root fixed** §1 |
| B | drop-witness trap `GisMapSnapshotRootRetirement::drop` | masks every post-mount failure | **root fixed** §2 |
| C | wal law 1 trace 3 `same-id-altered-envelope` | fixture defect — tamper hits the footer checksum | **fixture fixed** §4 |
| C2 | wal law 2 `rejects_hash_matched…` | stacks on the same tamper machinery | **fixture fixed** §5 |
| D1 | `terminal_close_waits_for_unpolled_cleanup…` | `Busy{retained_uses:1}` | narrowed, §6 |
| D2 | `quick::abandoned_pre_witness…` | times out in phase `Preflight` | not triaged, §6 |
| E1 | `retained_short_admin_request…` | `admin-authority-unavailable` on the competing writer | **root named**, §7 |
| E2 | `admin_removal…` | `database shutdown deadline elapsed` in `stop_recovery_server` | same family as D1, §6 |
| E3 | `checkpoint…rejects_stale…` | its own fixture replaces an immutable descriptor | **law-fixture defect proven**, §8 |


## 1. Root A — the Store re-minted the identity the durable decision was hashed over *(fixed)*

**Measured, from source.** `ArtifactStore::fold_batch_item` (`🏪️store/🦀️.rs:16897`) discarded the
prepared candidate's own identity and overwrote it twice:

```rust
let identity = authority.edit_id();
drop((applied_edit_id, tail_edit_id));      // ← the stamped id, thrown away
…
meta.mutation_id = Some(MutationId(format!("{}#{position}", stage.edit.id)));
```

`edit_id()` is the Store's content-addressed mint over `(actor, sequence, next_clock)` — the
`edit-70c74d54b7150193` the law read. `gis2d_one_item_edit`'s stamped arm *does* set
`id = stamp.mutation_id.0` and `mutation_meta[0].mutation_id = stamp.mutation_id`, and
`prepare_one_item` carries both into `applied_edit_id`/`tail_edit_id` — the fold is where they died.
This is one root under **three** observations: the law's `head_edit_id` mismatch at
`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:502`, and both WAL `— mutation-id` warns, because
`durable_decision_event_match` (`🧾️wal/🦀️.rs:268`) requires
`meta.mutation_id == target.mutation_id` **exactly**, with no `#position` suffix.

It is a live-hub defect, not a test artefact: every hub-committed approval published an edit that no
verifier could bind to the approval it executed.

**Landed** — HT4's defaulted-trait-method pattern, extended to the identity, inert for every
non-stamping caller:

| file:line | change |
|---|---|
| `🏪️store/🦀️.rs` trait `ArtifactStoreOneItemPreparationFactory` | new `stamped_mutation_id() -> Option<MutationId>`, default `None`, beside `stamped_clock()` |
| `🏪️store/🦀️.rs` trait `ArtifactStoreBatchItemAuthority` | the same defaulted method + the forwarding impl for `Arc<dyn …PreparationFactory>` |
| `🏪️store/🦀️.rs` `ArtifactStoreOneItemLiveAuthority` | new private `stamped_edit_id: Option<String>` + `stamped_edit_id()` accessor |
| `🏪️store/🦀️.rs` batch admission (beside the `stamped_clock` gate) | reads the stamped identity and admits it under the same `ARTIFACT_STORE_ONE_ITEM_ID_BYTES` fixed capacity every other Store identity answers to; over-long or empty is a typed refusal, `"stamped publication identity exceeds its fixed identity capacity"` |
| `🏪️store/🦀️.rs` `edit_id()` | returns the stamped identity when the authority carries one. The uniqueness argument in its docstring survives: the approval's mutation id is itself content-addressed over the job and the proposal |
| `🏪️store/🦀️.rs` fold contract | a stamped authority additionally requires `candidate.edit.id == stamped id` — an app factory cannot publish under a stamp it did not prepare |
| `🏪️store/🦀️.rs` fold `mutation_id` rewrite | position 0 of a stamped publication keeps the stamped mutation id; every other position, and every unstamped gesture, keeps `<edit-id>#<position>` |
| `✏️editor/🦀️.rs:609` (gismap) | `Gis2dOneItemPreparationFactory::stamped_mutation_id` returns the stamp's mutation id |

Five non-stamping `ArtifactStoreOneItemLiveAuthority` literals in `🗄️durable-group` (+3 in its and
canonical-edit's unit tests) pass `stamped_edit_id: None`; the group-id re-mint at
`🗄️durable-group/🦀️.rs:2510` inherits the base authority's stamp so a durable group cannot silently
re-id a stamped edit. `store_post_revision` never reads `edit_id()`, so no revision digest moved.

**Native kernel law** (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`):
`artifact_store_stamped_publication_carries_its_committed_identity_into_the_published_edit` —
a stamped publication's `Edit.id`, its folded `mutation_id` and the applied cursor's head all equal
the approval id, and an unstamped gesture on the same Store still gets `edit-…` + `#0`.
`DemoOneItemPreparationFactory::stamped_identity` is the fixture.

## 2. Root B — the drop witness aborted the process instead of reporting the law *(fixed)*

**Measured, from the 17:52 backtrace.** `GisMapSnapshotRootRetirement::drop`
(`🗺️gismap/…/🧬️mutations/💾️binary/🦀️.rs:313`) fires while the FIRST panic is still unwinding: the
tokio current-thread runtime shuts down, `close_and_shutdown_all` cancels the
`spawn_abandoned_request` task, which owns `RetainedGisMapApprovalCommitterV1` → the documents map →
`ArtifactStore` → `ArtifactStoreDisplacedRetirements` → the witness. A panic in a destructor during
cleanup is non-unwinding, so the process aborts and nextest reports `SIGABRT` with no failure site.
Every law that fails *after* mounting stores was hidden this way.

**Landed:** F1's rule, applied to all **five** drop witnesses in the gis tree (the assertion is the
law, so it is kept — it is only skipped when a panic is already in flight, where it can only destroy
the real diagnosis):

| line | witness |
|---|---|
| `:261` | `GisMapOwnedRetirement` |
| `:313` | `GisMapSnapshotRootRetirement` (the one that aborted) |
| `:478` | the `gis_map_owned_field_authority!` decode authority (macro — every generated field authority) |
| `:734` | `GisMapSnapshotCloneAuthority` |
| `:1244` | `GisMapStoreInitializationAuthority` |

`grep -A3 "impl Drop for"` over `✏️s/🔌️plugins/🌍️gis/` now reports **no** unguarded witness.
This is the same shape already law in `📡️replication`, `🌱️value`, `🖱️ui` and `🔌️plugin`.

## 3. Verification

| command | result | capture |
|---|---|---|
| `CARGO_TARGET_DIR=…/target-ht5 cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht5-check-kernel-1.txt` |
| `CARGO_TARGET_DIR=…/target-ht5 cargo test -p semio-framework-os-kernel --lib stamped_publication` | **3 passed, 0 failed** — HT4's two clock laws **and** HT5's new identity law, all at runtime | `🗑️generated/ht5-kernel-stamped-laws.txt` |
| `CARGO_TARGET_DIR=…/target-ht5 cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht5-check-hub-1.txt` |

Rule 26: no hub test/nextest/binary build ran from this slice. The three kernel store laws above are
the only runtime evidence; everything about the hub suite is source-level plus a compiling tree.

## 4. Root C — `same-id-altered-envelope` tampered the container, not the envelope *(fixture fixed)*

**Measured.** The trace's tamper was `let last = bytes.len() - 1; bytes[last] ^= 1;` on the canonical
pack (`🧾️wal/🧪️tests/🔬️unit/🦀️.rs:211-214`). The last byte of a `.spk` container is its footer
checksum, so `admit_canonical` refused the container (`Codec("checksum mismatch in footer at offset
80")`) and the product's `invalid` was **correct for that tamper**. The trace therefore proved
nothing about identity binding, which is what its name claims. `expected` is untouched.

**Landed, in the producer.** `DurableFixtureRecord` now also carries `altered_record`: a second,
**fully re-sealed** fixed-three decision built by the same
`durable_owned_group_journal_test_record_from_edits` path, holding the approval's `mutation_id`,
actor and timestamp, over a different proposal (the producer is now one `sealed(job, identity,
actor)` closure called three times, so canonical and altered records cannot drift apart).
`bindingMismatches[0].jobId` supplies the alternate 32-hex job id the inference proposal requires.

The one subtlety the re-seal forces: the receipt must name the decision the log actually committed,
or the scan fails `decision_hash != target.receipt.decision_sha256` → `Invalid` before any binding
is examined. `target()` now selects the altered record's `decision_sha256`/`anchor_sha256` when the
trace's own record list commits `altered-target` — derived from the existing fixture, **no fixture
key added or changed**. The replay then finds an admissible decision at its receipt that carries the
approval's mutation id but not the approval's proposal → `proposal-hash` unbound → `Ok(None)` →
**`absent`**, which is what the trace asserts and what the name means.

## 5. C2 — law 2's hostile bytes were never durable decisions at all *(fixture fixed)*

`inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands` wrote a bare
`protocol::encode_envelope(&MutationEnvelope)` into the receipt transaction. That is not a pack, so
`scan` never reaches `durable_decision_event_match` — it takes the
`"receipt transaction carries an event that is not a durable decision pack"` warn arm and the law's
`Err(Invalid)` can never happen. **It cannot be fixed on the product side**: trace 1
`missing-command` writes the same shape (`b"different non-Pack durable event"`) into the same
receipt transaction and its frozen answer is `absent`, so no rule about non-pack events can satisfy
both. The two differ only in `target.command_hash`, which no code path compares against raw event
bytes. Fixture defect, proven by the pair.

**Landed:** the three vectors now offer real durable packs, so the hash-matched claim is actually
tested — `trailing` appends a byte past the sealed payload, `overlong-varint` makes the first varint
after `BINARY_MAGIC` non-minimal (both refused by `decode_canonical_pack` → `Invalid`), and
`different-actor` uses the new `wrong_actor_record`: a canonical, admissible pack whose committed
author is somebody else, refused by the scan's own receipt-hash comparison. `target.command_hash` is
still forced to match the hostile bytes, which is the law's whole point: a matching durable hash
buys nothing. The assertion and the three selected vectors are unchanged.

## 6. D1 / E2 — one retained pool use, and the same family *(narrowed, not fixed)*

`pool.shutdown()` fails `Busy { retained_uses: 1 }` at `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:805`.
`retained_uses` counts live `Arc<WorkerPoolUse>` cells (`⏳️async/🦀️.rs:2005`), and in this law's
fixture exactly one is taken outside the `Database`: **`MemoryStorage::new` acquires one**
(`🛢️db/🗄️storage/🦀️.rs:6833`) and holds it for the life of the `Arc<DbBackend>`. The law's
`backend` Arc has two known owners — the committer and the `Database` — and both are released before
`pool.shutdown()` (`Arc::try_unwrap(database)` succeeds, so the committer's inner value is gone).
So a **third** `Arc<DbBackend>` clone, or a second `acquire_use`, survives; the `⚙️engine`,
`🗜️compact`, `🔄️sync` and `🗿️artifact` paths each take their own use and each parks it on a task
owner. The SIGABRT backtrace of §2 shows one such survivor by name: a live
`RetainedGisMapApprovalCommitterV1::spawn_abandoned_request` task, still owned by the tokio runtime
at shutdown. **E2** (`admin_removal…`, `database shutdown deadline elapsed` in
`stop_recovery_server`) is the same shape from the other side — a retained owner the shutdown
deadline waits for. Naming the exact survivor needs the suite, which rule 26 forbids me: the next
owner should assert `Arc::strong_count(&backend)` immediately before `pool.shutdown()` in D1 —
one line that turns "1 use" into the owner's identity.

**D2** (`quick::abandoned_pre_witness…`, times out in phase `Preflight`) is untouched by this slice.

## 7. E1 — the competing admin writer is refused by a fixed 2 s admission budget *(root named)*

`admin-authority-unavailable` with a non-`Denied` fence error comes from
`acquire_admin_intent_authority` (`🏗️bootstrap/🦀️.rs:8723`), which has exactly **two** `Unavailable`
sources: `tokio::time::timeout(2s, state.socket_binding_gates.acquire_bindings(bindings))` at
`:8724`, and the `socket_session_binding` timeout at `:8737`. The law's retained FIRST writer holds
its admitted binding while the HTTP waiter expires — that is the law's premise — and the competing
writer asks for the **same** bindings. The law took 10.2 s, an order of magnitude past the 2 s
budget. So the live root is `:8724`: a second admin writer's patience is a hard-coded 2 s that has
nothing to do with how long the retained effect legitimately runs, and the second writer is told
"authority unavailable" for what is ordinary queueing. Both HT3b's re-auth ranking and HT4's
`cancelled_before_effect` ranking are excluded — neither writes `Unavailable`.
I did not land the fix: bounding the wait by the operation's own deadline instead of the constant
touches the 503 mapping at `:6104` and every other admin route, and I cannot run the suite to see
what else pins 2 s.

## 8. E3 — the law replaces something the product declares immutable *(law-fixture defect, proven)*

`checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication` tries to make
the descriptor fence fire by re-announcing a changed descriptor
(`🔬️bin-unit/🦀️.rs:7051-7057`). `DirectoryCommand::AnnounceDocument` refuses that by design
(`📇️directory/🦀️.rs:2112`) and the green directory law
`document_descriptor_is_immutable_space_scoped_and_survives_restart`
(`📇️directory/🧪️tests/🔬️unit/🦀️.rs:1214`) pins it. There is no withdraw command, so the announced
descriptor of a live document can never change — the fence conjunct
`descriptor.as_ref() == Some(&self.descriptor)` (`🏗️bootstrap/🦀️.rs:3821`, `:3901`) is only
falsifiable by admitting a publication against a descriptor that is not the document's. The law
needs a second document, as HT4 read it; I did not rewrite it blind.

## 9. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — §1 (the stamped-identity trait methods,
  the authority field and accessor, the admission gate, `edit_id()`, the fold contract and the fold's
  `mutation_id` rewrite).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` — §1's native law and the
  `DemoOneItemPreparationFactory::stamped_identity` fixture.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` (+ its unit
  tests) and `🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs` — the non-stamping authority literals.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — §1's
  `stamped_mutation_id`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
  — §2's five drop witnesses.
- `🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs` — §4 and §5.

Nothing was deleted, `#[ignore]`d or loosened. No `expected` in
`🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json` changed; that file is **not** touched by this slice.

**needs hub rerun.** Expect the SIGABRT at `(147/321)` to become a reported failure site (§2), that
law to go green on §1, and WAL laws 1 and 2 to go green on §4/§5. Laws 4 (sqlite restart, which
re-prepares an approval whose stamp and identity are now both minted from the document) may move
with §1. Laws 6, 7, 10, 11 and the checkpoint stale law stay red with §6–§8's roots.

---

# HT5 — batch 2 (rerun 18:41: **321 — 314 / 7**, from 312 / 9)

Both WAL laws are **GREEN** on §4/§5 — the re-sealed `altered_record` and the pack-shaped hostile
bytes were the right roots, and no fixture literal was touched to get there.

## 10. The SIGABRT's second witness was in the kernel, not in gis *(fixed)*

All five gis witnesses guarded, so the 18:41 abort names a different one:
`ArtifactStoreOneItemAuthorityRetirement::drop` — *"Store live authority dropped before bounded
string retirement completed"* (`🏪️store/🧵️canonical-edit/🦀️.rs:598`), reached through the identical
chain (`ArtifactStore::drop` → `ArtifactStoreDisplacedRetirements` → boxed
`ErasedSnapshotRetirement`). **Guarded**, together with the four owner witnesses the same chain
drops (`🏪️store/♻️retirement/🦀️.rs:96` `Bytes`, `:204` `Sequence`, `:234` `ValueRetirement`,
`:269` `CursorStack`). `SnapshotReadLeaseRegistry` (`🏪️store/🦀️.rs:258`) was already guarded.
The `🧩️composition/🚪️open` witnesses are not on the store-drop chain and are left alone.

## 11. The law itself moved, and the product already states §1's invariant

`gis_map_approval_committed_event_reaches_actor_frontier…` no longer fails at `:502` — the stamped
identity assertion is **gone**. It now fails 7 lines earlier at `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:495`
with `public checkpoint refusal remains a retained nonterminal publication, got Some(Conflict)`
where `Storage` is required. Independent corroboration that §1's root was real:
`🏃️runtime/🦀️.rs:1832` refuses with **`Conflict`** unless
`projected.head_edit_id == identity.mutation_id` — the product had *already* declared that the
published head edit must be the approval's mutation id, which is exactly what the Store was
overwriting. Two `Conflict` producers are reachable at that point and the next rerun separates them,
because the abort no longer hides the site: `:1823` (the document state is not `Publishing` when the
publisher returns) and `:1832` (the projected frontier triple). I did not guess between them.

## 12. D1 — the ownership assert the coordinator asked for *(landed)*

`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`: the law now keeps a `backend_witness` clone of the
`Arc<DbBackend>` and asserts `Arc::strong_count(&backend_witness) == 1` immediately before
`pool.shutdown()`. `MemoryStorage::new` is the one `acquire_use` outside the `Database`
(`🛢️db/🗄️storage/🦀️.rs:6833`) and it lives exactly as long as that Arc, so the next rerun either
names the surviving clone's count or proves the leaked use is a `⚙️engine`/`🗜️compact`/`🔄️sync`
use parked on a task instead. Either way the failure stops being a bare `retained_uses: 1`.

## 13. E1 — the 2 s budget is a **product contract** *(decided from the callers; law-side fix stated)*

`timeout(2s, socket_binding_gates.acquire_bindings(…))` appears at **three** call sites and each maps
its expiry to the same typed `Unavailable`/503: socket-binding revalidation (`🏗️bootstrap/🦀️.rs:1780`),
the fenced directory command (`:5961`) and admin intent authority (`:8724`). One uniform rule — *no
fenced writer blocks on a peer's binding gate for more than 2 s; the caller retries* — is what bounds
every hub writer's latency. It is not a test-era constant, so it stays.

**The law is therefore unprovable as written.** `retained_short_admin_request…` issues its competing
writer while the first writer's effect is paused, then waits `ADMIN_OPERATION_DEADLINE + 2 s` for the
first HTTP waiter to 503 (measured: the law takes 10.18 s) before releasing the pause. The competing
writer is asked to contend for ≈ 5× the contract budget, so `admin-authority-unavailable` is the
**correct** product answer and `"succeeded"` cannot be reached. Bounding the retained operation
task's wait by `AdminOperationRuntime::deadline` instead does not rescue it either: both writers'
10 s operation deadlines start within milliseconds of each other, so the second expires before the
pause is released. The law's two claims have to be separated: (a) *a competing same-scope writer is
blocked, not refused, and succeeds when its predecessor finishes* — provable only while the
predecessor holds authority for **less than 2 s**; (b) *the HTTP waiter expires at its own deadline
without cancelling its admitted writer* — the existing 503 assertion, which needs no second writer.
I did not restructure it blind.

## 14. Verification (batch 2)

| command | result | capture |
|---|---|---|
| `cargo check -p semio-framework-os-kernel --all-targets` | **EXIT 0** | `🗑️generated/ht5-check-kernel-3.txt` |
| `cargo check -p semio-hub --all-targets` | **EXIT 0, 0 errors** | `🗑️generated/ht5-check-hub-4.txt` |
| `cargo test -p semio-framework-os-kernel --lib stamped_publication` | **3 passed** | `🗑️generated/ht5-kernel-stamped-laws.txt` |

Both crates were left green in the same tool round they were edited (GM1's `wasm-release` bootstrap
compiles them); no gis source was touched in batch 2.

Files changed in batch 2: `🏪️store/🧵️canonical-edit/🦀️.rs`, `🏪️store/♻️retirement/🦀️.rs`,
`🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`.

**needs hub rerun.** Expect the SIGABRT at `(146/321)` to become a reported FAIL at `:495` (§10), and
`terminal_close…` to name its surviving backend owner (§12). E1 and E3 are law-fixture defects whose
rewrites I did not guess at; D2, the sqlite restart law and `admin_removal…`'s teardown are untouched.
