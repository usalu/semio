# Shell Presented Fixture Coherence

## Native130 receipt

Native130 ran 1,359 renderer tests: 1,306 passed and 53 failed in 29.006 s. Eleven failures shared the exact panic:

```text
test input candidate witness: "retained presented input candidate could not be sealed"
```

They were the retained World pointer sequence fixture and its Shell, pointer-ingress, overlay-chrome and wheel wrappers. Receipt: `🗑️generated/astra-runtime/renderer-native130-full/failures.json`.

## Fixture defect

`retained_world_sequence_probe` mounted two visible UI documents, painted and acknowledged each document independently, then attempted another Shell-only seal for their combined body-hit registry. The presented input contract now requires every visible presented document to contribute a ready candidate to the same witness. A Shell-only reseal after the document candidates were consumed correctly refuses; teaching the production seal to accept it would hide stale pixels/input.

Refresh and replacement steps repeated the same invalid order: paint and acknowledge one document, then attempt a second acknowledgement for body-hit staging.

## Repair

`paint_tree_pointer_documents` now paints all simultaneously visible World documents into one candidate and registers all body hits before one `publish_retained_hit_registry` seal and acknowledgement. The sequence fixture keeps one active lease per visible row, retains superseded leases for bounded teardown, and repeats that full-frame publication on refresh or replacement.

No production presented-input rule changed. Existing pointer ownership, exact host, replacement and foreign-pointer assertions remain intact.

The changed fixture source parses under `rustfmt --edition 2021 --emit stdout`. Native verification remains owned by root.
