# P9d Owned Folder Event Log

## Outcome

The OS `folder://` authoritative document/blob store no longer links or executes SQLite. `FolderEventLogStorage` is an owned append-only event store used by the host, MCP workspace, sync endpoint, and blob boundary.

## Record format and projection

- Versioned `SEMIOEL1` record magic.
- Length-delimited payload plus owned FNV-1a checksum.
- Document snapshot event: timestamp, document id, schema id, pack bytes, spr bytes.
- Blob put event: timestamp, content hash, media type, bytes.
- Blob delete tombstone: timestamp and content hash.
- Reads deterministically fold newest matching events; document indexes fold the latest event per id and sort newest-first with a stable id tie-break.
- Multiple handles for one folder share a process-local writer lock. Each append uses one opened append stream and calls `sync_data` before completion.
- A partial final record is treated as an interrupted append and ignored; a complete corrupt record is rejected.

## Dependency change

- Removed the host crate's unconditional `rusqlite` dependency.
- Removed the OS kernel's optional `rusqlite` edge from its `sync` feature.
- The separate generic DB facade and Hub directory backend still contain their own driver features and remain separate Phase 9 packets.

## Verification

Existing folder pack/spr and blob tests were moved to the event-log type. Added a two-handle/partial-tail recovery test. The host-level focused gate is pending the shared stdio compiler wall and will be recorded separately; no pass is claimed here yet.
