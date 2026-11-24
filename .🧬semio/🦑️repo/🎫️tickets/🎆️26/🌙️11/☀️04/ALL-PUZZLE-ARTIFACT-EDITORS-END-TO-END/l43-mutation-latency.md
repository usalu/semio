# L43: 3D mutation latency (133 turns vs 8): instrumentation, scope amortisation, seal analysis

Status: written but not compiled or run (a build was running; the brief forbids cargo/repo builds). Change (b) is applied. Change (a) is analysed and documented below but not applied: it is not safe to change blind.

Paths are shortened. `FW` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `ED` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

Failure being chased (l41-3d.log): `translate=133turns/1610units [store=1485 page=95 retirement=10 worker=19]` on both 1 and 180 objects. The bound is 8 turns.

## 1. Temporary `[DEBUG]` histogram (to be removed after the next run)

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`: new inline module `semio_framework_value::debug_histogram`.
  - It keeps a thread-local fixed array of `Cell<u64>`, one per label in `DEBUG_UNIT_LABELS`. Counting does not allocate, so the heap-observation store laws are not disturbed.
  - Functions: `debug_unit`, `debug_unit_add`, `debug_units_reset`, `debug_units_report`. The report reads `[DEBUG] label=count …`.
- `FW/🏪️store/⚙️engine/🧳️bundle/🦀️.rs`:
  - The `advance_apply_batch` wrapper counts the branch it is about to take. The new `debug_apply_batch_branch` follows the inner dispatch order.
  - Branch labels:
    - Preparing: `apply.census`, `apply.item_close.{retiring,preparation}`, `apply.folding`, `apply.flip_cursor`, `apply.begin_next`, `apply.preparation_loop`, `apply.fold`.
    - Cursor: `apply.cursor`.
    - Preflight, digest: `preflight.digest.{take,release,admit_edit,admission,seal_birth}`, plus `preflight.digest.seal.<phase>` decoded from `witness()[0]` (countStart/count/header/hashStart/hash/takeOwner/sourceClose/finalize/ready/hashPending/closing).
    - Preflight, other: `preflight.begin_digest`, `preflight.diagnostics`, `preflight.flip`.
    - Publishing: `publishing.{message,metadata,commit}` on the original-commit route, otherwise `publishing.{diagnostics,transfer}`.
  - Outcome labels: `result.{blocked,progress,published,refused,terminal,error}`.
  - Digest receipts: `digest.items` and `digest.copy_bytes`, summed over digest steps.
- `FW/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs`: new `debug_granted_retirement_owner()`. It follows the order of `granted_retirement_step`. `advance_closing_publication` (`🔄️full-operation/🦀️.rs`) counts it as `closing.*`.
- The receiving-scope run (section 2) counts `run.scope`, `run.steps` and `run.stop.{more,work,items,blocked,other}`.
- `ED/🧪️tests/🔬️mutation-latency/🦀️.rs`:
  - `turn_census` resets the histogram before translate and before delete, and captures each report.
  - `Puzzle3dTurnCensus` stores them as `translate_debug` and `delete_debug`.
  - The `Display` output appends `translate-histogram=(…) delete-histogram=(…)`. Every assertion message of `one_mutation_publishes…` and the b54 print therefore names the dominant phases.
  - The 8-turn bound and all assertions are unchanged.
- To remove later: grep `debug_histogram` and `[DEBUG]`.

## 2. Applied: one receiving scope runs consecutive store units (l36 fix b)

Where: `FW/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`.
- New private `store_unit_run`.
- The `with_native_receiving_quoted` closure of `publish_mounted_typed_operation_unit` now uses it for the Artifact, Config and Draft lanes.
- Presence, Transient, WindowConfig and WindowTransient still run exactly one step.

What the run does:
- Inside the one scope it calls `advance_apply_batch_with_turn_ceiling` again and again.
- Each call is granted exactly what the run has not yet spent: the body grant (funded minus capture) minus the summed receipts, on the items, copy, capacity and release axes. Depth is not summed.
- It continues only while all of the following hold:
  - the unit returned `PreparationProgress` with a non-empty receipt that fits its own grant;
  - the lane still admits another unit (Artifact: `returned_snapshot_read_count() <= PUBLICATION_RETURNED_ROOT_ALLOWANCE`, the same gate the unit applies before a step);
  - the summed work is below the scope's work quote;
  - items remain.
- It stops on anything else: Published, Refused, an error, or a receipt that does not fit. That result is returned as the last result.
- A later `Blocked` unit is reported as `PreparationProgress(last checkpoint, 0)`, so earlier progress is never hidden as a stall.

Receipt accounting stays exact:
- `ownership = prior + <last result's receipt>` on every path.
  - Refused: prior + `preparation_refusal_progress`.
  - Domain error: prior + error progress + capture.
  - Published: prior + receipt.
- It is settled once through `consume_receiving_retained(funded, ownership)`.
- `OriginalReceivingScope::settle` still holds. Every unit's receipt already covers its own construction spend, so the sum covers the summed `receiving.spent`.
- The `PreparationProgress` arm now checks and records the summed `ownership` against the full body grant, not just the last unit's receipt. If an intermediate unit overspends its remaining grant on any axis, the sum exceeds the full grant and the existing fault fires.
- Fuel is the sum of `last_turn_work()` over the run. Before, it was the last unit's work, and each unit ran in its own scope.

Expected effect: the scope's control headroom (`original_receiving_scope_headroom`) and the per-unit scope admission are paid once per run instead of once per unit. Per l36 this goes from about 12 units per 64-item turn to about 50–60. Translate should drop from 133 to roughly 25–30 turns. That is NOT ≤ 8, because the store still spends one item per 256-byte seal window (section 3).

Store unit census: the pump still records one `Store` unit per `publish_mounted_typed_operation_unit` call. `translate_census.store` therefore drops to about one per scope run. It stays document-independent (`small == large`) and is still ≥ the number of turns.

Callers checked:
- `advance_apply_batch`'s signature and its single-unit semantics are unchanged. Store laws in `FW/🏪️store/🧪️tests/*`, durable-group (`🗄️durable-group/🧱️map-assembly…`), space-member (`👥️space-member/…`), the plugin `publication-retirement-authority` / `time-travel` laws, and the 2D/5D/GIS/WAV/DOCX/remodel editor unit tests call the store directly and see no change.
- The run is reached only through the plugin's typed-operation publication pump, which is shared by every app (2D, 3D, 5D, hub).
- The source anchor `source-rejection-anchors-and-host-lane` still finds `advance_apply_batch` and `pending_artifact_publication` inside `publish_mounted_typed_operation_unit`. Its separate `"\n        if let Some(pending) = …\n            let grant="` anchor was already stale before this change: the line after it is `let capture=`.

## 3. Not applied: the seal is the item amplifier (l36 fix a)

What the code shows (`🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`, `ArtifactCanonicalEditSealCursor`):
- The cost is per byte, not per node. Each `advance` call:
  - produces at most `SEAL_TURN_BYTES = 256` canonical bytes in a tree pass (`for _ in 0..SEAL_TURN_BYTES { if produced>=SEAL_TURN_BYTES … }`), or
  - hashes at most 256 bytes when `hash_pending` is set, or
  - performs one fixed transition.
- `next_work_demand` quotes 256 work for the tree and header phases. The bundle passes exactly this quote as the effect's maximum work (`receiving.with_effect(grant, owner.next_digest_work_demand(), …)`). So no grant can buy more than 256 bytes per unit.
- Every call reports `copied_items: 1`.
- For a canonical edit of length L, the seal costs about `L/256` (count pass) + `L/256` (hash-pass tree) + `L/256` (page hashes) + about 10 fixed units.
- 1485 store units for translate and 1137 for delete fit L in the tens of KiB. The new `preflight.digest.seal.count/hash/hashPending` and `digest.copy_bytes` counters will confirm L.
- `child()`'s `maximum_items=1` in `🔏️digest/🦀️.rs` is NOT the limiter. The seal reports one item per call regardless, and the window is fixed in bytes.
- The `RetainedCloneSource::admission_owned` ladder is a constant of about 12 phases (owner, alias, payload, payloadInitialize, lease, identity, projectionStart, leaseInitialize, leasePublish, leaseFinish, publication, take). It moves the Box; it does not clone the edit per node.

Clean fix, in order:
1. Make the seal window grant-driven.
   - Add `next_work_demand_within(grant)` (and the digest's `next_digest_work_demand_within(grant)`). It quotes `min(remaining canonical bytes, grant.maximum_copy_bytes)` instead of the fixed 256. The bundle passes that as the effect maximum.
   - In `advance`, loop over tree turn → page fill → hash flush → tree turn … until the quoted work or the copy grant runs out. The 1 KiB inline page stays the bound for buffered bytes; it is hashed and reused inside the same call instead of costing a separate unit per 256 bytes.
   - The count pass has no page (window 0 once unbuffered), so it can run to the grant directly.
   - Keep `copied_items: 1` per call and `copied_bytes` as the true sum. `canonical_fields_settle(effect, base, result, work)` must receive the summed work.
   - Update `🔏️seal/🧪️tests` and the digest tests in `🔏️digest/🧪️tests` for multi-window steps.
   - Expected result: about `3L/grant_copy` units, a handful for one turn's 32 KiB copy grant.
2. Seal the staged edit in place.
   - Replace `RetainedCloneSourceAdmission` + `admit_source` + `TakeOwner` + `SourceClose` (and their close ladder) with a cursor that aliases `stage.edit` (a pinned `Box<Edit<M>>` the publication owns exclusively during `PreflightingCommit`) for both passes and returns only the digest.
   - This removes about 15 constant units and the take/return of the edit.
   - It needs a `retained_alias`/`ArtifactCanonicalJsonTreeCursor` entry that accepts a borrowed root held across turns (today it requires a `RetainedCloneRef` from a source). That is an ownership and soundness change and needs review plus a compiled law run, so it is not done blind.

Expected total: (b) together with (a.1) puts translate at about 2–4 store turns, well under 8.

## Next run
`-p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`. Read `translate-histogram=(…)` in the failure message. If `preflight.digest.seal.*` dominates, implement 3.1.

## Files changed
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs` (temporary `[DEBUG]` `debug_histogram` module)
- `FW/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` (`[DEBUG]` branch labels and counting in the `advance_apply_batch` wrapper)
- `FW/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` (`store_unit_run`, summed receipt and work; run counters)
- `FW/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs` (`[DEBUG]` `debug_granted_retirement_owner`)
- `FW/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` (`[DEBUG]` count in `advance_closing_publication`)
- `ED/🧪️tests/🔬️mutation-latency/🦀️.rs` (histogram reset and capture; census fields; Display)

## 4. Applied (coordinator follow-up): grant-driven seal window (step 1 of section 3)

Status: written, not compiled or run. Step 2 (borrowed-root alias) is still not done.

### Seal cursor
File: `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`.

New items:
- `SEAL_MAXIMUM_WINDOW_BYTES = 16 * SEAL_PAGE_BYTES` (16 KiB).
- `ArtifactCanonicalEditSealCursor::work_window(grant)` returns `grant.maximum_copy_bytes` clamped to the range 256 to 16 KiB.
- `next_work_demand_within(window)` quotes `window` when the turn can chain. A turn can chain when the window is wider than 256 bytes, the cursor is not closing, and either a hash is pending or the tree is live. Otherwise it quotes exactly `next_work_demand()`.
- `advance_within(window, effect)`. `advance(effect)` is now `advance_within(256, effect)`, so the narrow path is byte-for-byte the old code.
- Private `advance_window`. Inside one effect it alternates:
  - (i) In-place page hash, limited by three bounds: the window left, the effect's remaining work, and the copy credit left.
  - (ii) One narrow canonical run (the old inner loop, unchanged, at most 256 produced bytes). It starts only while at least 256 work units remain in both the window and the effect.
- Phase transitions (tree terminal, header, take, close, finalize) stay on the narrow path as fixed one-step turns.

Exact receipts and work:
- Hashed bytes are recorded on the effect as they are consumed (`effect.performed(length)`). They are added to `last_work` and receipted as `copied_bytes`.
- Tree runs keep their own ownership receipts, under a grant reduced by what is already spent.
- Each call reports `copied_items: 1`.
- `canonical_fields_settle` therefore sees performed work ≤ reported work, and owes nothing extra.
- Every receipt fits its grant.

Digest unchanged:
- The same canonical byte stream is hashed in the same order. Only the chunking differs, and SHA-256 does not depend on chunking.
- The buffered single-page path and its length checks are kept.

### Digest trait and caller
- Digest trait (`FW/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`):
  - `ArtifactStoreBatchDigest` gains `digest_window(grant)`.
  - `next_digest_work_demand(window)` and `advance_digest(window, effect)` now take the window.
  - `NativeBatchDigest` forwards the window to the seal. The admission and close phases ignore it.
  - `witness()` quotes the narrow window.
- Bundle (`FW/🏪️store/⚙️engine/🧳️bundle/🦀️.rs`, PreflightingCommit):
  - It quotes the wide window from the turn grant and tries `with_effect` with that work.
  - If the receiving scope cannot fund it (`None`: nothing ran, nothing was charged), it retries with the narrow quote. A thin scope therefore never stalls; it only falls back to today's speed.

Expected effect: about 3·L/16 KiB seal units instead of 3·L/256 for a canonical length L, roughly 64 times fewer. Together with the receiving-scope run this should put translate at a few store turns.

### Callers checked
- `NativeBatchDigest` is the only implementor of `ArtifactStoreBatchDigest`. Every `begin_batch_digest` in 2D/3D/5D/hub/framework/plugins goes through `admit_artifact_batch_digest`, so none of them change.
- The bundle is the only production caller of `next_digest_work_demand` and `advance_digest`.
- `🏪️store/🧵️canonical-edit/🧳️source` (the `Traversal::Hash` canonical-edit source) and all existing seal tests keep calling `advance`/`next_work_demand`. They see the identical narrow behaviour.
- Stale test caller, not fixed: `🔏️digest/🧪️tests/🦀️.rs`, included by `🏪️store/📬️command/♻️retirement/🧪️tests/🚪️public/🦀️.rs`. It calls `owner.advance_digest(full)` with a grant. That already did not match the previous `advance_digest(effect)` signature, and it does not match the new `(window, effect)` one either. It needs an effect driver to be rewritten. I did not touch it.

### New law test
`a_wide_window_seal_chains_fill_and_hash_to_the_narrow_digest_in_fewer_turns` in `🔏️seal/🧪️tests/🦀️.rs`. For an edit of about 8192 rows it asserts:
- every receipt fits its grant;
- `last_turn_work ≤ next_work_demand_within(window)`;
- the same length and digest as the narrow seal;
- more than 8 times fewer turns.

### Files changed in this follow-up
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/🦀️.rs`
- `FW/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`
- `FW/🏪️store/⚙️engine/🧳️bundle/🦀️.rs`
