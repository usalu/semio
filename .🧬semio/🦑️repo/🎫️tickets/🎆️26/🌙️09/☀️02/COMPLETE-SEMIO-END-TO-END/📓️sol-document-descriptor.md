# Durable Authorized Document Descriptor

## Outcome

Implemented the P0 durable `DocumentDescriptor` packet across the OS directory contract, TypeScript twin, hub directory service, every configured persistence backend, authenticated HTTP/WebSocket boundaries, client/admin read surfaces, and focused fixtures/tests.

The descriptor is immutable after its first `document.announced` event and contains:

- space-scoped `spaceId` + `documentId`;
- `artifactKind` + exact `artifactSchema`;
- owning `pluginId`, `packageId`, package `version`, and immutable `packageHash`;
- immutable `packSchemaHash`;
- positive `bootstrapVersion`;
- bootstrap `headSeq`, `commitSeq`, and `epoch` frontier;
- immutable `bootstrapSnapshotHash`.

All three hashes must be 64-character, lowercase, non-zero SHA-256 text. The bootstrap commit cannot exceed its head. Identical re-announcement is idempotent; a different descriptor for the same `(spaceId, documentId)` conflicts. Equal document ids in different spaces are isolated.

This is intentionally P0 only: it does not add bootstrap snapshot bytes, document CRUD, CRDTs, legacy compatibility, or a new runtime dependency.

## Design and behavior

- Added schema-first Rust, TypeScript, and JSON Schema forms for `DocumentOwner`, `DocumentFrontier`, `DocumentDescriptor`, and descriptor-backed `DocumentView`.
- Added `announce-document` and its append-only `document.announced` event. Rust and TypeScript folds project the first immutable descriptor and count it on the containing space.
- Added a language-neutral JSON fixture with canonical wire text plus conflicting-schema, cross-space-same-document, and revoked-reader scenarios.
- Added normalized SQLite/PostgreSQL projections and a Neo4j descriptor node projection. Projection rebuilds replay the event log; space deletion removes its descriptor projection.
- Added `get_document_descriptor` and `list_document_descriptors` to `HubDirectory` and all closed-set dispatch arms.
- Replaced the hub's volatile `schema_hashes` process map with the durable directory projection. No `schema_hashes` reference remains in the hub or directory sources.
- Space detail, `DirectoryClient`, and admin document responses now expose the descriptor and live frontier together.
- Ordinary announcement requires an authenticated author of the descriptor's space; the admin command boundary is separately authenticated. Spectators and revoked/non-members are rejected.
- WebSocket open authenticates and authorizes the caller before reading the durable descriptor, requires an existing descriptor, and compares Hello's artifact schema and pack-schema hash exactly before creating/opening the database handle. Hello can verify the durable identity but cannot define or mutate it.
- Status and share-token creation also require a durably announced document, preventing an unauthenticated/missing document name from minting server document state.
- The existing AJV test dependency is used only as a test oracle; no production dependency was added.

## Rust boundary note

`DirectoryCommandResponse` uses a small manual `ToValue` implementation. The value derive macro expands through the canonical path `::semio_framework_os_kernel`, while the hub deliberately imports that package only under the dependency alias `directory`. Deriving in `bin.rs` therefore produced three `E0433` errors unrelated to the packet shape. The manual implementation writes the same `{ events, result? }` first-party `DslValue` object and avoids adding a second dependency name or a serde compatibility layer. Directory request/response and directory-WebSocket JSON use the first-party value/pack bridge.

## Verification

### Passing

1. Rust descriptor contract plus SQLite durability/conflict/isolation/rebuild, using a ticket-local narrow harness that mounted the real schema and hub directory/backend sources while bypassing the concurrently broken OS store graph (harness deleted after the run):

   ```text
   CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/document-descriptor-rust-oracle-target" bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/📜️script.ts' test
   ```

   Result: **2 passed, 0 failed, 12 filtered**. The two tests were the language-neutral fixture canonical round-trip and `document_descriptor_is_immutable_space_scoped_and_survives_restart`.

2. All configured directory backends type-checked from their real sources in the same isolated harness:

   ```text
   CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/document-descriptor-rust-oracle-target" bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/📜️script.ts' check
   ```

   Result: **pass** for SQLite, PostgreSQL, and Neo4j; warnings only.

3. TypeScript schema/fold tests:

   ```text
   bun nx run @semio-tech/framework-os:test-quick --skip-nx-cache --testNamePattern='document descriptor|document.announced'
   ```

   Result: **2 passed, 216 skipped; 1 file passed, 2 skipped**.

4. AJV JSON Schema oracle:

   ```text
   bun nx run @semio-tech/framework-os-mcp:test-quick --skip-nx-cache --testNamePattern='document descriptor schema oracle'
   ```

   Result: **1 passed, 33 skipped; 1 file passed, 4 skipped**.

5. Hub TypeScript suite:

   ```text
   bun nx run os-hub-ts:test-quick --skip-nx-cache
   ```

   Result: **2 passed, 1 skipped; 1 file passed**. The real-server E2E remains intentionally skipped unless `HUB_E2E` points at a running hub.

6. Fixture/schema syntax and scoped whitespace/invariant checks:

   ```text
   jq empty '🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-descriptor.json' '🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json'
   rg -n 'schema_hashes' '🌎️hub' '🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory'
   git diff --check -- <descriptor-owned paths>
   ```

   Result: **pass**; JSON parses, no volatile `schema_hashes` remains, and the scoped diff has no whitespace errors.

### Shared-graph blockers and non-passing attempts

The production Rust package commands could not reach descriptor-owned tests because unrelated files were being edited concurrently:

- `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/document-descriptor-cargo-target" bun nx run @semio-tech/framework-os-kernel:test --skip-nx-cache -- descriptor` compiled the unrelated full plugin dev-dependency graph until the repository's 1,200,000 ms test budget expired; it emitted no descriptor diagnostic.
- `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/document-descriptor-cargo-target" bun nx run os-hub:test-quick --skip-nx-cache -- document_` first stopped in replication serde-removal fan-out at `📡️replication/🎮️mutation/🦀️.rs` (`MutationMeta.undo_policy`) and `📡️replication/⚔️conflict/🦀️.rs` (`ConflictKind`, `DispatchReport.policy`, `MergeReport.policy`), 16 errors. After that concurrent edit briefly cleared, the same command advanced to two pre-existing/current OS store errors at `🏪️store/🦀️.rs:19828` and `:19833`: `InteractionState` did not implement `serde::Serialize` / `serde::de::DeserializeOwned`.
- A later isolated dependency attempt observed the replication graph changing again (`📡️wire/🦀️.rs` missing `default_true` and serde implementations for selection/merge types). This confirms the full-package gate is external to the descriptor changes; the narrow real-source backend check above passed all descriptor-owned Rust code.

Accordingly, the focused Rust auth/WebSocket tests are present but the production hub binary test runner did not execute them in this worktree. No claim is made that the full hub package or live-server E2E passed.

## Files

Created:

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-descriptor.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🧪️document-descriptor-rust-oracle/`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-document-descriptor.md`

Updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️hygiene.test.ts`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📄️DocumentsPage/🟦️.tsx`

Several updated files also contain concurrent work outside this descriptor packet; only the descriptor-related sections above are attributed here.
