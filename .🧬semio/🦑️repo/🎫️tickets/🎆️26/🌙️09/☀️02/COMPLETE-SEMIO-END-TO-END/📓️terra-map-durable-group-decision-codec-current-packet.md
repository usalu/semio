# Map Durable-Group Decision Codec — Current Implementation Packet

Audit date: 2026-09-05. Read-only source audit; no build or runtime was run.

## Decision

Make the first durable Map transaction one kernel-owned, fixed-shape **parent + drawing + value decision**. The journal event must contain the three already-prepared, recoverable Store outcomes, rather than the GIS inference request or three independent mutation inputs. Persisting only `GisMapCreateRegionGroupWorkV1` would require executing application-owned preparation again after restart and would mint a different Store clock/sequence/edit authority.

The P0 decision is deliberately not a generic `Vec` transaction and it has no image participant. Its reusable kernel shape is `owned-three-member`, while GIS supplies the exact typed work and fixed membership.

## Current concrete inputs and boundaries

| Input / boundary | Current source | Consequence for the codec |
| --- | --- | --- |
| Typed Map work | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45-56,92-164` | `GisMapCreateRegionGroupWorkV1` is exactly parent mutation/inverse, drawing child/mutation/inverse, and value child/mutation/inverse. It rejects an image member and caps those six canonical JSON values at 65,536 bytes. It is not a Store outcome or a durable record. |
| Parent and child identities | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:78-95`; `…/🧬️schema/📸️snapshot/🦀️.rs:25-53` | The only admitted children are `gismap-drawing!s.stdio.semio@v1/drawing` and `gismap-value!s.stdio.semio@v1/value`; the parent dialect is `s.gis.gismap@1/*`. The local child materialization is intentionally not serialized (`🏪️store/🦀️.rs:2665-66`), so journal identity must use `childId` + `ArtifactRef`, never a local owner. |
| Actual child operations | `…/💡️inferences/🦀️.rs:131-153`; `🗄️stdio/…/🖊️drawing/🧬️schema/🧬️mutations/➕create-node/🦀️.rs:12-18`; `…/🔢️value/🧬️schema/🧬️mutations/➕insert-list-item/🦀️.rs:9-15` | Drawing is a public `CreateNode { parent, index, node }`; value uses `SemioValueMutation::from_value` because `InsertListItem` fields are `pub(crate)`. The journal must consume the already-typed/coded result, not try to construct a value child mutation in the kernel. |
| Member wire seam | `🏪️store/🦀️.rs:13250-13271,17510-25,17695-17715` | `MemberStoreOneItemWireRequest` supplies original op bytes and exact base generation/revision. It gives a retained typed preparation, but exposes no durable prepared-outcome codec. |
| Actual prepared outcome | `🏪️store/🦀️.rs:13067-13208,15317-55,15407-86` | `ArtifactStoreOneItemLiveAuthority` owns operation, base, next sequence/HLC, actor and group id; the private prepared candidate owns the edit, post snapshot, edit digest and pointer seal. A recovery record must preserve the semantic authority and full outcome, while the Store alone recreates its pointer seal. |
| In-memory group substrate | `🌿️vcs/🦀️.rs:188-245,452-535`; `🏪️store/🦀️.rs:17728-1787` | One visibility owner can hide staged history/cursor entries. It is not restart data and ordinary publication explicitly rejects a group-reserved candidate. |
| Existing coordinator is unsuitable | `🏪️store/🦀️.rs:19389-19565` | `TransactionCoordinator` applies child stores and then parent, stamps each tail afterwards, and compensates with sequential undo. It cannot emit this decision or establish atomic durable visibility. |
| Current journal envelope | `🛢️db/📝️wal/🦀️.rs:423-45,447-85,673-727` | `WalRecord::Event` is an opaque raw payload in one WAL transaction. A new decision codec can be carried there without making DB understand GIS. `WalRecordBatch` has 64 entries, but this P0 submits one Event body. |
| Maximum event bytes | `🛢️db/📝️wal/🦀️.rs:75-99,2175-2185`; `🗄️storage/🦀️.rs:69-72`; `🏪️store/🦀️.rs:13014-17` | `MAX_FIELD_BYTES` is 1,048,576, but an accepted WAL segment is only `DB_IO_MAX_READ_BYTES == 496 KiB == 507,904` **including** the SPR header, segment-header transaction, logical Begin/Event/Commit frames, and physical commit frame. A one-MiB Event cannot reach replay. Three maximum one-item candidates therefore need a much smaller group cap. |
| Dependency direction | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:42-61`; `…/🛢️db/📦️packages/🦀️rust/Cargo.toml:32-51` | DB already depends on the kernel. The schema, canonical validation and journal trait belong in `os_store`; DB implements the trait with WAL. The kernel must not name `ArtifactWal`, `WalRecord`, or a storage backend. |

## Canonical V1 record

Place the new schema in a small `🏪️store/🧩composition/🗄️durable-group` module. The public value should derive `ToValue`, `FromValue`, and `dsl::DslArtifact`, with the same camel-case and canonical JSON discipline used by `DirectoryCommandRequestV1` (`📇️directory/🧬️schema/🦀️.rs:481-519`). The WAL body is `ArtifactPack::encode_pack()` of the sealed decision; the neutral JSON projection is the independent oracle. Do not store raw `serde_json` bytes.

```text
DurableOwnedThreeMemberDecisionV1 {
  schema: "semio.store.durable-owned-three-member-decision.v1",
  anchor: DurableOwnedGroupAnchorV1,
  anchor_sha256: lower-hex SHA-256,
  decision_sha256: lower-hex SHA-256,
  parent: DurableOwnedGroupMemberV1,
  drawing: DurableOwnedGroupMemberV1,
  value: DurableOwnedGroupMemberV1
}

DurableOwnedGroupAnchorV1 {
  schema: "semio.store.owned-three-member-anchor.v1",
  parent: ArtifactRef,
  shape: "parent-drawing-value"
}

DurableOwnedGroupMemberV1 {
  role: "parent" | "drawing" | "value",
  reference: ArtifactRef,
  owner: null | OwnerRef,
  expected_generation: u64,
  expected_revision: [u8; 32],
  recovery_schema: String,
  recovery_pack: Vec<u8>,
  recovery_pack_sha256: lower-hex SHA-256,
  unbound_outcome_sha256: lower-hex SHA-256,
  post_generation: u64,
  post_revision: [u8; 32]
}
```

The fields are named individually, rather than in `Vec<member>`, to make duplicate/missing/reordered participants unrepresentable in the accepted V1 model. `drawing.owner` and `value.owner` are required and must be exact `OwnerRef { parent: anchor.parent, slot: "drawing"|"value", child_id: "gismap-drawing"|"gismap-value" }`; `parent.owner` is `null`. Admission also checks:

1. `parent.reference == anchor.parent`, its dialect is `s.gis.gismap@1/*`, and the parent snapshot's handles equal the drawing/value `reference` and stable child ids.
2. Drawing is exactly `s.stdio.semio@v1/drawing`; value is exactly `s.stdio.semio@v1/value`. No image field exists in this V1 record.
3. Each expected generation/revision is the preparation base, while post generation/revision is the Store-produced frontier. They must differ according to the one-item publication rule (generation increments once) and are rechecked immediately before staging/recovery.
4. `recovery_schema` is a registered member-specific outcome schema, at most `ARTIFACT_STORE_ONE_ITEM_ID_BYTES` (256) bytes. The kernel never decodes it itself; the exact participant-installed decoder does.

`recovery_pack` is not original mutation wire. It is a member-installed, binary `ArtifactPack` for one *bound* prepared outcome and must contain, at minimum: the Store-minted operation, base generation/revision and applied-count; next sequence and HLC; actor; the derived `decision_sha256` as group id; exact `Edit` (forward, inverse, metadata and edit id); post-snapshot pack; final edit digest; and final post generation/revision. Decoding must rebuild the semantic values, compare the full-pack hash and the Store-recomputed final derivations, then have `ArtifactStore` recreate its private `ArtifactStoreOneItemSeal`. It must never call the app preparation factory after recovery. The current pointer-bearing `ArtifactStoreOneItemPrepared` remains private.

### Identity and commitment

Use `semio_framework_hash::sha256_hex`, already the cross-host canonical request digest in the directory contract (`📇️directory/🧬️schema/🦀️.rs:481-84`):

```text
anchor_sha256             = SHA256(canonical_json(anchor))
unbound_outcome_sha256(m) = SHA256(canonical_pack(UnboundOneItemOutcomeV1(m)))
decision_sha256           = SHA256(canonical_json(unsigned decision))
recovery_pack_sha256(m)   = SHA256(exact bound recovery_pack bytes)
```

### Acyclicity correction — full outcomes cannot be merely “group-id omitted”

The earlier `recovery_semantic_sha256(decoded outcome with MutationMeta.group_id omitted)` was still cyclic. The concrete Store path is:

```text
decision_sha256
  -> MutationMeta.group_id                         (bound edit)
  -> Edit::to_value().mutationMeta                 (🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1551-73)
  -> CursorRevisionAccumulator::edit_digest        (🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12281-84)
  -> applied prefix digest                          (🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15463-67)
  -> CursorRevisionAccumulator::revision/post rev  (🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12319-24,15476-78)
```

Therefore a full `edit_digest`, prefix/record digest, or `post_revision` must **not** appear in the unsigned member projection. Clearing only `MutationMeta.group_id` while retaining any of those derived values retains the fixed point.

`UnboundOneItemOutcomeV1` is a new explicit Store-owned canonical projection for the three registered Map member codecs, not an ad-hoc filtered recovery pack. It contains only the exact pre-bind facts:

```text
UnboundOneItemOutcomeV1 {
  recovery_schema,
  operation, base_generation, base_revision, base_applied_edit_count,
  next_sequence_number, next_clock, actor,
  edit_without_group,       // complete Edit, exactly one meta, meta.group_id == None
  post_snapshot_pack        // exact member snapshot bytes computed while no group id exists
}
```

`edit_without_group` retains `Edit.id`, actor, forwards, inverse, description/coalesce/times/sequence, and every non-group `MutationMeta` field, including `mutation_id`. It is not safe to drop those values: they bind the exact Map result. The P0 member codecs must canonical-parse that shape and reject an absent or non-`None` sole metadata slot, unknown fields, an extra metadata slot, or a post snapshot that does not decode to that member's registered snapshot type.

The unsigned decision has only `schema`, `anchor`, and, for each fixed member, `role`, reference/owner, expected base frontier, `recovery_schema`, and `unbound_outcome_sha256`. It excludes `anchor_sha256`, `decision_sha256`, every bound `recovery_pack` and pack hash, `post_generation`, `post_revision`, final `edit_digest`, every cursor/prefix/id digest, and the private seal. The full record still carries the latter values; it just does not use them to derive the group id.

This distinction is source-required, not theoretical:

| Field | Current dependency on `group_id` | Unsigned treatment |
| --- | --- | --- |
| `Edit.id` / `MutationMeta.mutation_id` | The ordinary VCS mint hashes actor + sequence + forwards only (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:34-43`), then the normal single-op helper copies the id into `mutation_id` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10122-28`). The generic one-item contract, however, only checks that `id` is nonempty/≤256 (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13118-32`), so it cannot promise every factory chose an id independently of group. The current GIS factory does: `gis2d-retained-{next_sequence}` and `{id}#0` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398-420`). | Retain as opaque, already-created unbound fields. Do not claim a generic re-mint rule. The new group path must create them while authority has `group_id=None`; that removes the final id from app authority rather than trusting every factory. |
| `next_sequence_number` / `next_clock` | `begin_apply_one_owned` derives sequence by checked `edit_sequence + 1` and ticks the clock *before* assembling the authority, independently of its final `group_id` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15311-26`). The fields are then required in the edit/meta (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13118-29`). | Retain and bind them in `UnboundOneItemOutcomeV1`; do not synthesize them on recovery. |
| final edit digest | The canonical digest hashes `edit.id` plus canonical `Edit::to_value` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12281-84`), and `Edit::to_value` serializes `mutationMeta` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1551-73`). `MutationMeta::to_value` emits `group_id` when present (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1447-77`). | Exclude; recompute only after the final group id is inserted. |
| prefix and post revision | Publishing hashes the final edit digest into the applied prefix (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15463-67`) and obtains `content_revision` from that accumulator (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15476-78`). | Exclude both prefix/id digest and `post_revision`; derive and compare them only in Store staging/recovery after final edit digest. |
| private seal / local aliases | `ArtifactStoreOneItemSeal` holds an `Arc` authority, edit/post pointer addresses and the digest (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13164-78`); `ArtifactStoreOneItemPrepared` also holds `local_actor`, `applied_edit_id`, `tail_edit_id` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:13193-201`). | Never encode/hash them. Recreate them from the bound authority + exact edit/post owners; the aliases are respectively actor/id/id. |

The existing post-hoc `stamp_tail_group_id` is specifically unusable for this protocol. It mutates the persisted edit's metadata (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17960-66`) without recomputing the already-maintained revision accumulator. A durable group must bind before its final digest/prefix/revision are generated, never mutate a published tail.

### Exact bind and recovery order

The current member wire route forwards `request.group_id` into `begin_apply_one_owned` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17695-17714`), and that value is frozen in the live authority (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15317-31`). The current GIS preparation intentionally writes `group_id: None` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:405-16`), so passing a non-`None` group today fails the Store's exact metadata check. This proves the earlier proposed “prepare bound candidate, then calculate its id” order is not implementable by the present public API.

The smallest correct delta is a Store-private **late bind**, not a second app preparation:

1. Start all three retained preparations with a provisional authority whose `group_id` is `None`, and advance them only to their current pre-publication boundary (`prepare_one_item_publication`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17728-58`). This produces each post snapshot, edit id, inverse, metadata, sequence, and HLC once, with no final group value available to application code.
2. Store verifies/canonical-encodes the three `UnboundOneItemOutcomeV1` values, derives their hashes and then `decision_sha256`. The P0 Map codec's post snapshots are group-free products of the typed mutations: parent applies `CreateRegion`, drawing applies `CreateNode`, and value applies `InsertListItem` before any Store edit is built (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:100-153`); the parent snapshot has no `MutationMeta` field (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:25-57`).
3. A new non-public `ArtifactStore` group coordinator consumes each prepared owner, requires exactly one metadata record with `None`, creates a replacement `ArtifactStoreOneItemLiveAuthority` by copying operation/base/applied-count/sequence/HLC/actor and setting only `group_id=decision_sha256`, writes that one meta field, recomputes `prepared_edit_digest`, and calls the existing private `seal_prepared_owned` path. It must reject rather than invoke the app factory again. This yields one exact bound `Edit` and post root per member.
4. The same Store coordinator stages the bound history/cursor roots under `ArtifactGroupVisibilityOwner`, deriving `post_generation = base_generation + 1`, the final prefix and `post_revision` by the same accumulator operations used by one-item publishing. Only now encode the three bound `recovery_pack`s, require their full hashes, and persist the decision.
5. On replay, verify full-pack hashes; decode each registered bound pack; require exactly `decision_sha256` in each sole `MutationMeta.group_id`; regenerate `UnboundOneItemOutcomeV1` by replacing that one field with `None`; verify the three unbound hashes and then the decision digest; recompute final edit digest/prefix/post frontier in Store and compare the packed final fields; recreate seals; stage/adopt all three together. No member decoder gets to assert the final revision or fabricate a pointer seal.

This binds the same exact final outcome bytes: the unbound hash commits all non-derived outcome bytes, while the Store proves the only transition to the full pack is `None -> decision_sha256` plus the mechanically recomputed digest/revision/seal. No hash preimage includes a value downstream of that transition.

`decision_sha256` is the sole `MutationMeta.group_id` for all three members and the idempotency key. A repeated record is a no-op only if **all** three members already expose that exact group id and the recorded post generation/revision; all-three-base permits one recovery stage; every mixed or foreign state is corrupt and stays invisible. The Map inference job id is not a replacement idempotency key: it identifies the created region, whereas durable retry identity must cover the full Store outcome and base.

This uses the full 256-bit digest rather than the VCS helper's 64-bit display identifier (`🌿️vcs/🦀️.rs:16-25`), because the anchor routes a durable journal and must not be a shortened locator. The DB may derive its internal journal document id from `anchor_sha256`, but must retain and compare the full `anchor` inside every decision before applying it.

### Bounds

Define these *before* performing recovery-pack encoding or copying into a WAL owner:

| Rule | Ceiling | Reason |
| --- | ---: | --- |
| participant count | exactly 3 | fixed P0 shape |
| packed Event decision bytes | 491,520 (480 KiB) | safely below the 507,904-byte whole-segment read ceiling after all current SPR/WAL framing and the pre-existing segment-header transaction; `MAX_FIELD_BYTES` is not an admission guarantee |
| recovery pack per participant | 162,000 | `3 × 162,000 + 4,096 = 490,096`, leaving 1,424 bytes inside the Event cap for exact decision structure |
| fixed structural/identity bytes | 4,096 | includes the three refs, owners, schema ids, 32-byte digests and fixed numeric fields; final packed-size check remains authoritative |
| Map typed work inputs/inverses | 65,536 canonical JSON bytes | existing GIS admission, not a substitute for outcome-size admission |
| every identifier/schema/actor/group id | 256 bytes where it crosses the one-item seam | current Store identity ceiling |

The record must measure its *packed Event body* after encoding and reject it if above the first ceiling; per-member caps merely preflight it. A Map action whose complete recoverable outcome exceeds this P0 bound fails before any decision is made. Do not evade the cap with a CAS reference: persisting a CAS object plus a journal reference would introduce a second durability linearization that this decision does not yet own.

## Smallest kernel/DB seam

Add this narrow kernel interface beside the `SpaceMember` retained one-item methods; it holds no DB types:

```rust
pub trait DurableOwnedGroupJournalPort: Send {
    async fn commit_fsynced(
        &mut self,
        anchor: &DurableOwnedGroupAnchorV1,
        decision_pack: &[u8],
        decision_sha256: &str,
    ) -> Result<DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalError>;
}
```

The receipt contains only `anchor_sha256`, `decision_sha256`, the journal transaction id, verified commit sequence and chain hash. It is returned only after one committed fsync transaction. The retained Store coordinator owns the prepared participants and calls the port only after it has: validated fixed ownership; prepared all three; encoded and verified the decision; revalidated all bases; reserved/staged complete Store roots under one `ArtifactGroupVisibilityOwner`; and pre-reserved adoption/retirement capacity. On a port error it keeps the decision/candidates and aborts them under grants; after a receipt it flips the shared visibility decision and does only infallible adoption.

DB implements `commit_fsynced` with its own `ArtifactWal`: a single `WalRecord::Event(decision_pack)` as the body of one `TxBegin`/`TxCommit` transaction. Its committed-transaction replay gate must yield the Event only after the matching commit. DB decodes the pack through the kernel codec and invokes a kernel `recover_committed_owned_group` coordinator with all three exact members. `ArtifactEngine` is not this owner: it is per-document and its existing replay ignores Event records. This preserves DB → kernel dependency; the kernel never receives an `ArtifactWal` or `WalStorage` argument.

Recovery owns atomic visibility as follows: reconstruct all three retained candidates from the trusted event, verify the anchor/participants/frontiers and exact outcome digests, stage all three complete store roots under one fresh in-memory visibility owner, then commit/adopt once. There is no publication after recovering only a parent or one child. The WAL decision is durable; the visibility bit is intentionally process-local and is recreated from that decision.

## First executable neutral corpus

Add a language-agnostic fixture and JSON Schema at:

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️fixtures/🔣️.json`

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️fixtures/🧬️.schema.json`

Each row carries a canonical unsigned JSON oracle, three compact `recoveryPack` bytes, the calculated hashes, and an expected `admit | reject | recover | corrupt` result. Run it through `os_pack::json` plus `ArtifactPack` and an independent Bun JSON/sha256 oracle.

1. **`image-free-create-region-commits`** — parent `map-a!s.gis.gismap@1/*`; the two exact stdio refs; `CreateRegion`, `CreateNode`, and `InsertListItem` outcomes; three matching expected/post frontiers. Assert one decision id and all-new only after its committed receipt.
2. **`same-decision-replay-is-idempotent`** — replay row 1 twice. The second pass sees three matching post frontiers/group ids, writes no second history entry, and returns the same receipt identity.
3. **`forged-owned-child-rejected-before-journal`** — change drawing `childId`, dialect, owner slot, or owner parent in four hostile variants. No Event, no visibility flip, and every retained owner closes.
4. **`bound-edit-derivations-are-not-an-unsigned-preimage`** — take row 1's otherwise valid bound pack and alter only final `editDigest`, prefix/post revision, or post generation. The unbound semantic hash remains unchanged, but Store recomputation rejects all three before staging. This is the direct regression for the prior fixed-point mistake.
5. **`only-none-to-decision-group-binds`** — derive the decision from three unbound outcomes; bind it; then reject a second binding, a prebound non-matching group, an extra metadata slot, a changed edit id, and a changed post-snapshot byte. The accepted full pack's stripped projection must byte-equal the original unbound pack; its final digest must differ from the provisional group-free digest, and its post revision must be first derived by the bound Store stage.
6. **`tampered-outcome-or-commitment-rejected`** — mutate one recovery byte, one recovery digest, the anchor hash, the decision hash, add an unknown/duplicate JSON field, or add image membership. Canonical parsing/verification fails before a member decoder or Store stage runs.
7. **`mixed-recovery-is-corrupt-not-compensated`** — parent at recorded post frontier while drawing/value remain base (and the converse). Reject the replay without applying or undoing any member; this proves restart recovery cannot conceal a torn three-member state behind sequential compensation.
8. **`capacity-and-base-fence`** — one member recovery pack at 162,001 bytes, a packed Event decision over 491,520 bytes, and a changed expected generation/revision. All reject before `commit_fsynced`; no Event is accepted. Include an independent whole-segment oracle proving the accepted maximum includes framing rather than merely testing `MAX_FIELD_BYTES`.

The native law should reuse the existing one-item preparation/reservation path at `🏪️store/🦀️.rs:17695-17788` and the low-level shared-visibility corpus at `…:23335-23490`, then add the three-member captured-read and replay cases. DB adds the same rows to its fault/crash journal testkit only after the kernel codec law is green.

## Implementation order

1. Land the neutral JSON/schema fixture and a kernel codec law for canonical parse, pack round-trip, hashes, fixed roles, maps' exact refs/owners, and all bounds.
2. Add the Store-private unbound-to-bound owner transition and its neutral laws: exact `None -> decision_sha256`, full-digest/revision recomputation, and seal reconstitution without a second app preparation.
3. Add the three participant-installed unbound/full outcome codecs, then prove recovery has the same edit/post bytes and authority without invoking preparation.
4. Add complete-root group stage/adopt/recover to `ArtifactStore`/`SpaceMember`, retaining the existing group visibility primitives.
5. Implement the DB journal port over the committed WAL transaction cursor; then add crash/replay only-or-all acceptance.
6. Finally adapt GIS typed work to encode its three already-prepared outcomes. Do not route it through `TransactionCoordinator`.

## Nonclaims

This packet does not claim that a durable Map group is implemented, that the present typed GIS work is published, or that a normal child `document_pack_bytes` sequence is atomic. It intentionally excludes image mutation, child genesis, peer/N-way groups, CAS payload indirection, and sequential group undo.
