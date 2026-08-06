# M1 — VCS Identity (Wave 1a)

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Scope:** deterministic document VCS ids — no process-global counter.

## Changes

### `🌿️vcs/🦀️component.rs`

1. **Removed** `static ID_COUNTER: AtomicU64` (already gone prior to this pass; confirmed absent).
2. **Public helpers** in region `🆔️Ids`:
   - `content_addressed_entity_id(prefix, payload)` → `{prefix}-{hex16(blake3(prefix||0||payload))}`
   - `edit_scoped_id(edit_id, ordinal)` → `scoped-{hex16(blake3("{edit_id}:{ordinal}"))}`
   - `mint_edit_id(actor, sequence, forwards_fingerprint)`
   - `mint_change_id(edit_ids, description)`
   - `mint_alternative_id(name, checkpoint_ids)`
   - `mint_operation_id(operation_bytes)`
   - `create_document_vcs_id(prefix)` kept as `content_addressed_entity_id(prefix, prefix.as_bytes())` for callers without a distinguisher (same prefix collides by design).
3. **CollectionOperation / ItemPatch:** left as VCS-local twins. SPR frozen contract uses `Add { id, item, at }` / `Move { id, to }`; VCS keeps `index` / `to_index` for `apply_collection_operation`. Documented on both types — **not** `pub use` from spr.
4. Unit test `content_addressed_entity_and_mint_helpers_are_deterministic`.

### `🏪️store/🦀️component.rs` (mint call sites only)

Replaced time-/counter-based fingerprints with mint helpers:

| Site | Before (bad) | After |
|---|---|---|
| `Apply` / `AmendLast` edit ids | `edit_sequence` + `started_at` + envelope id | `mint_edit_id(actor, sequence, serde_json(forwards))` |
| `CommitCheckpoint` / reconcile change ids | `Debug` of pending / message string | `mint_change_id(edit_ids, description)` |
| `CreateAlternative` / reconcile alt ids | `name:checkpoint` string | `mint_alternative_id(name, checkpoint_ids)` |
| `replay_operations` / `parse_document_spr` op ids | `now_ms()` | `mint_operation_id(encode_op())` |
| text `replay_ops` op ids | `now_ms()` | `mint_operation_id(serde_json(op))` (no `OpBinary` bound) |
| space checkpoint | message + document id list | message + `serde_json(pins)` |
| space alternative | name only | name + checkpoint ids payload |

Store re-exports: `mint_*`, `content_addressed_entity_id`, `edit_scoped_id`.

## Non-goals / notes

- Space/`draft` callers of `create_document_vcs_id("draft")` remain prefix-only (content-stable for identical prefix).
- Checkpoint ids already use `content_addressed_checkpoint_id` (unchanged).
- Do **not** merge VCS collection ops into SPR until field names are unified.

## Verify

```bash
cargo check -p semio-framework-os-kernel --lib
```
