# 🧮️ Digest stall of `setContributions` (2026-11-23)

Ticket `26/09/01/SOURCING-END-TO-END`. The browser build 17:35 still ends `setContributions` in
`interactive-job.publication-stalled`, and its `[DEBUG]` lines say: the commit-preflight digest step
(`PreflightingCommit` in `store/⚙️engine/🧳️bundle`) quotes `next_digest_work_demand() == 0`, the
original is not ready, not empty, and the step returns default progress on every unit.

## What the digest step is

`bundle/🦀️.rs` `PreflightingCommit`: `owner.ready()` (take the edit) → `owner.terminal_is_empty()` → else
`work = owner.next_digest_work_demand()`, `receiving.with_effect(grant, work, advance_digest)`; an all-zero
`RetainedCloneProgress` is answered `Blocked` (and recorded as "the store owner refused the granted page").

`original_ready` is `ArtifactCanonicalEditSealCursor::is_ready()` = phase `Ready` and the edit owner taken back. The seal
walks `CountStart → Count → Header → [HashStart → Hash] → TakeOwner → SourceClose → Finalize → Ready`.

## Measured natively (instrumented, `[DEBUG]` prints removed again)

Per-step trace of `the_real_demonstrator_pack_settles_through_the_typed_set_contributions_operation` (49 digest steps):

| Phase | Steps | Remark |
| --- | --- | --- |
| Count/Header | 1..11 | tree pass, 477 B edit, buffered in the 1 KiB page (single pass) |
| TakeOwner | 12..47 | 36 steps: the source's lease binding is closed alias by alias (demands 424/392, 48/128, ... depth 2..5), then the owner election and `take_value_step` (work 4) |
| TakeOwner, last | 47 | source empty, demand `(0,0,0)`, `Complete` reply with default progress: the seal quotes work 1, the source quotes 0 |
| SourceClose | 48 | demand `(2752,0,0)` natively |
| Finalize | 49 | |

The browser prints map onto this: `seal take` copy grant 501106 → 500490 (-616; native -952), 522606 → 522582 (-24; native -48): wasm32
sizes, one turn renewal between take 2 and 3 (capacity back to 16777216). The stuck demand `(1840,0,0,depth 2)` is the digest-level quote
(`nested` adds 1) of a seal-level `copy(1840)` at depth 1, which is the wasm32 size of the native `copy(size_of::<Option<RetainedCloneSource>>)`
= 2752 B: phase `SourceClose` with a terminal-empty source (or a closing seal at the same point). A quote of 0 cannot come from the seal's own
formula in that phase (`next_work_demand` returns 2), so either the quote printed was the performed work (also 0 when the seal
returns before doing anything), or the seal is in a different state than a native run reaches.

## Ruled out natively

- Native ran with receiving depth 64, copy ≈ 520 KB, capacity 16 MiB, release 1 MiB (the INTERACTIVE_COMPONENT turn grant is identical to the browser's): settles, one transient `Blocked`.
- Quotes do not depend on the body: `next_take_work_demand(body)` for the grant, 2048, 0 and the copy demand differ only at the final empty-source step (0 vs 1), never at the earlier steps.
- `requires_take_original_birth / control` and `requires_original_birth / control` are false at every one of the 37 take/close steps: no unserved original Birth.
- A roster past the 1 KiB seal page (two hashing passes, `HashStart/Hash`, a second alias) settles: new law `a_two_pass_roster_settles_through_the_typed_set_contributions_operation` (99 digest steps, same single transient Blocked).
- Per-turn alignment cannot be swept from a test: the app's receipts validate against its own policy (`registered fixture preparation receipt grant exceeds its original caller authority` for items below 64).

## Concurrent change by a peer (seen 19:29, not mine)

`🔏️seal/🦀️.rs` now makes the `TakeOwner` `Complete` reply carry `copied_items: 1` and `last_work >= 1`, so the last take step is no longer an
all-zero-progress step, and prints `[DEBUG] seal source close ...` from `SourceClose`. That removes the only zero-progress step my trace found
(step 47), which is exactly the `work=0`, default-progress, `original_ready=false` signature the browser printed. I did not touch those lines.

## What changed in this ticket step

Diagnostics that name the stalled digest step in the fault itself, so the next browser run is decisive without `[DEBUG]` prints:

- `🔏️seal/🦀️.rs`: `ArtifactCanonicalEditSealCursor::witness()` = `[phase | closing, hash_pending, tree, source, original, retirement, take_pending flags << 8.., work quote, next copy demand]`.
- `store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`: `ArtifactStoreBatchDigest::witness()` (seal witness, or admission/source/edit flags before the seal exists).
- `store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`: `ArtifactStoreBatchPublication::digest_witness()`.
- `plugin/🧩️composition/📬️publication/🦀️.rs`: `PendingArtifactStorePublication::digest_witness()`.
- `plugin/🧵️instance/.../📬️typed-operation-unit/🦀️.rs`: a `Blocked` unit with a digest in flight records "the commit-preflight digest made no progress (seal state bits, work quote, next copy demand)" with those three figures; the stall message then ends `[bits, quote, demand]`. Phase codes: 0 CountStart, 1 Count, 2 Header, 3 HashStart, 4 Hash, 5 TakeOwner, 6 SourceClose, 7 Finalize, 8 Ready; bit 8 closing, 9 hash_pending, 10 tree, 11 source, 12 original, 13 retirement, 14 take_pending.
- Removed: the `[DEBUG] seal take ...` print (the peer's `[DEBUG] seal source close ...` stays theirs); no `[DEBUG] digest ...` print exists in the tree any more.
- New law (editor unit tests): `a_two_pass_roster_settles_through_the_typed_set_contributions_operation`.

## Wasm

A wasm rebuild is required (framework production code changed). After it, a remaining stall reads
`... waiting on the commit-preflight digest made no progress (seal state bits, work quote, next copy demand) [bits, quote, demand]`.
`bits & 0xff` is the phase; `quote == 0` with phase 6 (`SourceClose`) or `bits` bit 8 would confirm the empty-source / closing quote hypothesis;
phase 5 with bit 14 would mean a pending take that never recovers.

## Not established

The browser-only trigger of the persistent zero quote is not reproduced natively (see "Ruled out"). The fix above is observability plus the peer's zero-progress fix; if the
stall persists, the witness names the exact state.

## Test counts (native cargo)

- curation lib, `--test-threads=1`: 98 passed, 0 failed (97 plus the new two-pass law).
- `semio-framework-replication-edit` `-- seal canonical`: 5 passed, 2 failed (`persistent_protocol_canonical_fields_preserve_original_projection_pointers_and_granted_heap`, `canonical_native_identity_assembly_preserves_original_unicode_fields_and_exact_granted_cancellation`: `demand.copy_bytes + demand.capacity_bytes <= 4096` on a constructor demand; no field was added to any cursor by this change, so this is pre-existing / peer-owned, not re-checked against a clean tree).
- `semio-os-store-engine-member-group -- digest`: no test matched the filter.
