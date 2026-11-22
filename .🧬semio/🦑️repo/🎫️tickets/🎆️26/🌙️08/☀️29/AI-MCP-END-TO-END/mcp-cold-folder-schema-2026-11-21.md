# Cold Folder Artifact Schema

On a fresh folder workspace, `inference_submit` resolves the document kind through `semio://artifact/{id}/schema`. The previously seeded WFC artifact already had its genuine canonical Pack and SPR, but the schema resource required an open probe or a live plugin binding. The authorized request stopped at `PLUGIN_UNAVAILABLE`, before selecting the WFC inference service.

The folder schema resource now falls back to the existing durable snapshot when no live probe or binding supplies the schema. It reads the owner SPR's schema with the same parser used for explicit folder reopen. The schema read creates no editor or document binding. The probe lock is released before the storage read. Missing snapshots and SPR without a schema retain the prior retryable `PLUGIN_UNAVAILABLE`; the snapshot read still reports storage errors through its existing error path. Live probe, live plugin and hub lookups keep their prior responses.

The submission path still authorizes the caller before reading the schema and checks the inferred kind against the workspace's declared services. The stored folder alias is insufficient to select a service when the owner SPR cannot be decoded.

The four added neutral cases cover a live probe, a cold owner SPR whose folder alias differs, a malformed SPR with a plausible WFC alias, and a missing snapshot. The full Bun workspace suite passed three tests and 40 assertions, including the independent SQLite comparison and Ajv validation. The focused Rust regression writes through `FolderEventLogStorage`, uses a genuine owner-produced SPR for the cold case, and asserts absence of a document binding. The dedicated Rust test was authored but has not been run in a separate compiler session. The combined gateway build owns verification with the genuine seeded WFC SDK fixture.

## Cached link attempt

The ticket's read-only `mcp-schema-fast-link.ts` inspected five cached gateway dependency graphs to see whether a quick verification could reuse exact Cargo archives. The two newest graphs both referred to the missing consent archive fingerprint `9efe4e709973f87e`. The remaining graphs lacked their exact workspace archives, with fingerprints `575e7385ec525738`, `69822d7bce9a56e9` and `7e613c137388378b`. The shortcut stopped before calling the compiler, so it produced no gateway and wrote nothing into the active Cargo target. The seeded SDK observer remains attached to the combined gateway build.
