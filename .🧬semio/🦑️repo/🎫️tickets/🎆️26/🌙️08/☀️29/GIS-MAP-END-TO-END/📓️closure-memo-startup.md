# Closure memo startup verification

The GIS dev command stopped before Vite when the generation exchange imported closure memo functions that the shared shard module did not yet export. A test on the pinned Bun 1.4.2 runtime reproduced the missing `encodeClosureMemoV1` export. The shared shard implementation now exports the memo encoder and reader and accepts the memo when collecting fragment references.

The memo schema describes the versioned, checksummed `entries` format used by the concurrent shard implementation. The local file is bounded to 24 MiB. The independent fixture tests compare fragment-to-closure mappings against SQLite and validate the wire shape with Ajv. They cover exact fragment reuse, replacement, equal digests in different namespaces, empty entries, invalid entries, checksum mismatches, cancellation and size limits.

The exact `DevScript` module imported by the root router loaded successfully. The two memo tests and nine existing closure leaf integration tests passed on the pinned Bun runtime. The initial `repo tools exec bun` was waiting for machine capacity, so verification used the same provisioned Bun binary directly. Concurrent edits also aligned generation publishing and restoration with the REAPI closure shard call signatures. The parent exchange file was left to its owners.
