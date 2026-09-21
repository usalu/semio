# Store Authority Retirement Audit

## Finding

Confirmed production close deadlock under a one-byte positive grant.

`ArtifactStoreBatchPublication::close_step` checks whether the complete `actor + group_id` byte count fits in the current grant, then returns `Blocked` until it does. See `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14898` through `:14904`. For an authority with any actor longer than one byte, repeating `{ maximum_items: 1, maximum_bytes: 1 }` cannot make progress. The publication remains non-terminal and its Drop witness then correctly refuses disposal.

The branch also drops the complete `Arc<ArtifactStoreOneItemLiveAuthority>` rather than transferring its strings to the domain’s byte retirement seam. That conflicts with the Store’s own `ArtifactStoreStringRetirement`, which already truncates one bounded byte page at a time (`.../🏪️store/🦀️.rs:444` through `:476`).

## Existing intended owner and its second defect

The first-party owner already exists: `ArtifactStoreOneItemLiveAuthority::retire` returns `ArtifactStoreOneItemAuthorityRetirement` at `.../🏪️store/🦀️.rs:14222` through `:14224`. Its close implementation moves actor and optional group into `ArtifactStoreStringRetirement` children and makes one-item, bounded-byte progress at `.../🏪️store/🧵️canonical-edit/🦀️.rs:552` through `:594`.

It presently has only two string slots:

```rust
strings: [Option<String>; 2]
```

After `Arc::into_inner`, it assigns only `actor` and `group_id` (`.../🧵️canonical-edit/🦀️.rs:581` through `:584`). The newer `stamped_edit_id` carried by `ArtifactStoreOneItemLiveAuthority` (`.../🏪️store/🦀️.rs:14142` through `:14152`) is omitted and would be dropped outside byte-accounted retirement. This is independently real for durable/stamped publication: Store admission writes `stamped_edit_id` into the live authority at `.../🏪️store/🦀️.rs:16731` through `:16749`.

## Minimal repair boundary

Keep authority disposal inside the publication, as a finite sub-owner:

1. Add `authority_retirement: Option<Box<dyn ErasedSnapshotRetirement>>` to `ArtifactStoreBatchPublication`, initialize it to `None` at its sole factory (`.../🏪️store/🦀️.rs:16753` through `:16775`), include it in `terminal_is_empty`, Drop proof, and `closing_owner_witness`.
2. In `close_step`, before the current direct authority branch, drive `authority_retirement.close_step(1, grant.maximum_bytes)`. Preserve `Blocked` only for zero item/byte credit; pass through bounded `Pending`; accept `Complete` only after `terminal_is_empty`, then clear this sub-owner.
3. When `authority` is present, move it once into `authority.retire()` and install that returned owner. Do not inspect total actor/group/stamped length and do not call `drop(self.authority.take())`.
4. Extend `ArtifactStoreOneItemAuthorityRetirement` to own three optional strings and transfer `actor`, `group_id`, and `stamped_edit_id`. Its existing active `ArtifactStoreStringRetirement` is the correct one-byte, Unicode-safe byte disposer.

This preserves the existing one owner per close turn: an initial transfer is one item, then exactly one string child receives at most the granted bytes. It retains no full string clone and does not raise a capacity. The current close order—preparation, source, stage, coalesce key, receipt, authority, fault—remains intact.

## Fail-first laws

### Actual Store law

Add a focused law in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` that drives a real `ArtifactStoreBatchPublication` through a stamped one-item preparation/acknowledgement, then repeatedly calls `close_step({1,1})`.

Use non-ASCII actor, group, and stamped ID values. Assert on every nonterminal turn:

- result is never `Blocked` after the positive grant;
- `released_items <= 1` and `released_bytes <= 1`;
- `closing_owner_witness` eventually moves from `authority` to `authority-retirement` and then `none`;
- the publication reaches `Complete` and `terminal_is_empty`.

The current direct authority branch makes the law fail at `authority` as soon as actor plus group exceeds one byte. That isolates this production owner rather than only testing the disposer.

### First-party disposer law

Extend `canonical_authority_final_unicode_strings_retire_under_single_byte_grants` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs:365` through `:387`. It already exercises `authority.retire()` with single-byte grants. Set a nonempty `stamped_edit_id` and require total released bytes to include all three UTF-8 strings. This fails until the retirement owner receives its third slot.

### Neutral/independent coverage status

`🧫️fixtures/artifact-store-one-item-publication-v1/🔣️.json` sets only the ordinary 4096-byte close grant (`:4` through `:10`) and its `interrupted-close` row asserts only `complete-empty` (`:24` through `:30`). The unit `SerdeOneItemPublicationOracle` consumes that fixture at `.../🏪️store/🧪️tests/🔬️unit/🦀️.rs:233` through `:253`; it does not execute real close ownership or model byte release. It can receive a neutral `singleByteAuthorityClose` record for schema acceptance and expected terminal outcome, but it is not a substitute for the actual Store law.

No existing third-party/differential close oracle was found that observes this `Arc` authority’s retirement bytes. The existing canonical first-party disposer test plus the real Store law are the necessary fail-first pair; a later differential oracle must model per-byte UTF-8 release, not merely JSON lifecycle fields.
