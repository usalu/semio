# Retained GIS Map Committer

## Boundary

The Hub runtime now has a production-shaped, deliberately unregistered `RetainedGisMapApprovalCommitterV1`. One mounted document state owns the exact DB `ArtifactHandle`, parent Map Store, drawing Store, and value Store. Approval can move those owners only through `Ready -> Assembly -> Journal -> Verification -> Committed`; shutdown moves any phase through `Closing` and returns every Store to its exact owned disposer.

The committer rederives the fixed-three GIS work from the canonical Map base, checks the exact ordered `gismap-drawing` and `gismap-value` members, checks the canonical command and deterministic mutation identity, and uses `DurableOwnedThreeStoreMapAssemblyV1`. Its journal sink comes only from the mounted document `ArtifactHandle::durable_group_journal_sink`. It never calls generic submit and returns no receipt until `InferenceWalVerifierV1` finds the sole one-Event committed WAL transaction.

The GIS preparation factories gained a server-derived stamp containing the approved mutation ID and HLT. The parent WAL verifier now compares the physical decision edit's mutation ID as well as actor, author, payload, inverse, and timestamp before reconstructing the command hash.

## Schema-first evidence

- Initial source oracle: RED, exactly `retained GIS Map committer source is missing RetainedGisMapApprovalCommitterV1`.
- Current source oracle: GREEN, `ajv=1 hostile=7 node-sha256=2 independent-bounds=2 preview=1 lifecycle=9 committer=6 visibility=7 errors=11 approval-rejections=11 cross-fixture=1`.
- The existing GIS fixed-three native run `exact-cargo-laws-nYkzCz` has group 00 GREEN4 and group 01 still building the full Stdio dependency at the time of this checkpoint. It predates the Hub committer source and is not evidence for this runtime.

## Nonclaims and remaining integration

Production Hub construction still injects `UnavailableGisMapApprovalCommitterV1`. This is intentional. Before registration, the Hub document-write gate must be shared with this owner; startup must mount committed-decision recovery before new approvals; checkpoint publication must consume the same updated parent Store authority; and process shutdown must demonstrate retained close/retry under injected journal and Store retirement faults. No current receipt therefore claims a live Hub `applied=true`, process restart recovery, user-visible Map update, or undo/redo path.
