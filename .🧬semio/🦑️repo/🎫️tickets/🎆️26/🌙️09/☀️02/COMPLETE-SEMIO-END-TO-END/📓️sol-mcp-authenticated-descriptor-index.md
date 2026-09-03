# MCP Authenticated Descriptor Index

## Outcome

P4-A now has an authenticated, actor-owned hub workspace binding. A hub origin cannot be constructed from the CLI without a distinct hub bearer. The binding performs the native directory client's bounded `GET /auth/sessions/me` followed by `GET /directory/spaces/{spaceId}`, requires the selected space and a current membership for the authenticated session subject, validates every returned document under its structural `DocumentScope`, and atomically publishes an immutable descriptor-only snapshot.

The synchronous MCP resource surface performs no network I/O. It reads only a `Ready` snapshot and returns retryable `PluginUnavailable` while the binding is unbound, refreshing, expired, disconnected, or revoked. It no longer turns an authority failure into an empty catalog.

## Contract and implementation

- `WorkspaceOrigin::Hub` and `HubOptions` require a non-optional token and redact it from `Debug`.
- The native binding state is `Unbound | Refreshing | Ready(Arc<AuthorizedDescriptorSnapshot>) | Revoked`.
- The immutable snapshot is keyed by full `DocumentScope { space_id, document_id }`; same document ids in another space cannot alias.
- The fixed retained cardinality is 4,096 documents. Token and retained diagnostic limits are both 4,096 bytes, identities are 512 bytes, and each native operation receives a 10-second deadline.
- Refresh checks cancellation before I/O and during descriptor validation. Progress is observable as phase plus completed/total counts.
- Every invalidation advances a generation fence, so an in-flight stale refresh cannot republish. Relevant directory events require refresh; target member removal and space deletion revoke and stop the actor. Rebootstrap, reconnect, lag, and stream loss clear readiness. A failed WebSocket dial may probe REST authorization but cannot expose that result as Ready while stream continuity is absent. REST 401/403 and missing membership revoke.
- `semio://workspace`, `semio://workspace/artifacts`, and `semio://workspace/scopes/{space}/{document}/descriptor` are snapshot-backed. The workspace response identifies the authenticated hub subject, not the caller-controlled local policy principal.
- Raw artifact bodies, `/schema`, and `/validation` remain explicitly retryable-unavailable for known hub documents. They are P4-B, not fabricated P4-A data.
- The existing internal kernel `ureq` feature and OS services runtime drive the existing `DirectoryClient` and finite `DirectoryStream`; no new external runtime dependency or permanent script was added.

## Neutral fixture and independent oracle

The language-neutral fixture and Draft 2020-12 schema live beside the Rust domain implementation:

- `🏠️workspace/🔗️remote/🧬️schema/🔣️.json`
- `🏠️workspace/🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json`

They cover no token, expired/unauthorized session, public space without membership, successful member snapshot, same-document cross-space rejection, member revocation, and reconnect invalidation. A separate TypeScript test validates the schema with AJV 2020, validates the emitted resource records with the third-party MCP SDK, and independently verifies scoped URI encoding and the no-raw/no-local-principal/no-bearer laws.

## Evidence

Initial focused Rust compilation was red with `E0277` because the authorized document wrapper derived `Eq` over a `DocumentView` that is intentionally only `PartialEq`. Removing the invalid stronger contract made the first five binding laws green.

Final gates on 2026-09-03:

- `CARGO_TARGET_DIR=<ticket>/🗑️generated/mcp-p4a-target SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/framework-os-mcp-rs:test --skip-nx-cache --verbose -- authenticated_hub_workspace`
  - Nextest profile `quick`: 7 selected across 2 binaries, 7 passed, 285 skipped, 0.050 seconds.
  - Includes actor authentication/cancellation/capacity/scope/revocation laws, the snapshot-only resource law, and the CLI missing/distinct-token law.
- `CARGO_TARGET_DIR=<ticket>/🗑️generated/mcp-p4a-target bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache --verbose`
  - Green native production binary build. Existing unrelated workspace warnings remain.
- `SEMIO_OS_MCP_BIN=<ticket>/🗑️generated/mcp-p4a-target/debug/semio-os-mcp bun nx run @semio-tech/framework-os-mcp:test --skip-nx-cache --verbose --testNamePattern='authenticated hub workspace fixture oracle'`
  - 1 file passed, 3 tests passed, 42 skipped. The configured native binary was found and used by the test surface.
- `git diff --check` on the MCP and umbrella-ticket paths: clean.

The first TypeScript oracle run was red because it used Bun-only `import.meta.dir` under Vitest. Resolving from `fileURLToPath(import.meta.url)` made the oracle cross-runtime and green.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️authenticated-hub-workspace.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`

## Explicit residuals

- The hub's underlying session-issuance endpoint remains insecure until the separate authentication packet replaces its development issuance path. P4-A consumes and verifies the resulting session but does not claim that issuance is secure.
- P4-B is still required for descriptor-bound canonical pack/SPR bytes, raw resource bodies, schema, and validation. The previously audited 64 MiB artifact-authority versus 496 KiB database blob limit is unchanged.
- No live external hub was available for this focused run. Native transport composition and the production binary compile are green; protocol behavior is exercised deterministically through the real `DirectoryClient` against a recording transport plus the independent TypeScript fixture oracle.
- The descriptor index is intentionally a bounded in-memory published snapshot, not a durable offline cache. Any continuity loss fails closed and requires fresh authenticated bootstrap.
