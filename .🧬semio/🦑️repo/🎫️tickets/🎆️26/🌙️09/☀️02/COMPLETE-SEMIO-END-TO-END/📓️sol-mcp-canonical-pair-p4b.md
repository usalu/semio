# MCP Canonical Active-Checkpoint Pair P4-B

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Scope: canonical active-checkpoint pair selection, exact authenticated HTTP delivery, neutral framing, independent oracle, owned gate, and launch freshness.

## Outcome

P4-B now exposes one exact route:

```text
GET /spaces/{space}/documents/{document}/active-checkpoint/pair
Accept: application/vnd.semio.canonical-checkpoint-pair.v1
Authorization: Bearer <member-session-or-exact-document-share>
```

The route admits only a path-only request with exactly one matching `Accept`, no query, no `Range`, and exactly one bearer credential. Before any owned token allocation, the borrowed authorization text is bounded by `AUTH_TEXT_MAX_BYTES`, must use the exact `Bearer ` grammar, and must parse as the typed current Session or Share capability. Empty, oversized, malformed, duplicate, and wrong-kind credentials fail closed. Authorization then accepts only a current space member session or the exact document share token. Public-space fallback, anonymous callers, nonmembers, cross-space/cross-document substitution, and admin identity without membership remain denied.

The response is assembled only from `VerifiedActiveCheckpointPairReader`. The reader resolves the exact active public checkpoint, retrieves the matching verified public checkpoint, revalidates scope/public identity and current descriptor digest, preflights both nonzero byte lengths with checked arithmetic against the 64 MiB and 16,384 x 4 KiB record ceilings, reads the two private CAS blobs without publishing storage keys, and verifies each length/SHA-256 plus the aggregate pack-then-SPR SHA-256. Replacement of the directory active pointer cannot substitute another pair after selection because the selected verified checkpoint and identities are owned values for the read.

The wire is strict big-endian length-prefixed `header -> pack records -> SPR records -> Complete terminal`. Records are nonempty, at most 4 KiB, and carry exact part, ordinal, and byte offset. The receiver rejects missing terminal, trailing bytes, reordered/wrong ordinals, length/digest/aggregate mismatch, malformed metadata, and resource-limit overflow. The ETag is SHA-256 over a domain separator and the canonical public header only.

Authorization is revalidated after metadata selection, before each record, and immediately before the terminal. Because the handler retains the framed body until all checks and the terminal succeed, revocation at any tested boundary returns an empty `401` rather than publishing a partial stream. Successful responses carry exactly the owned policy headers: canonical content type, `Cache-Control: private, no-store`, `Vary: Authorization`, and the quoted canonical ETag.

Every admitted request owns a bounded lifecycle control: atomic cancellation and active-ownership state plus one fixed progress slot for each of the eight transfer stages. Progress rejects zero totals, over-completion, total changes, and regressions. The handler has a monotonic Tokio deadline around the entire admitted operation; expiry cancels the control and returns an empty `504` before body publication. Dropping the live HTTP request future, proven through an actual TCP disconnect, cancels the reader work, stops progress, and releases active ownership. Normal return disarms the lifecycle guard only after the complete response is owned; `Ready` is reported only after the full framed body and headers exist.

## Neutral schema and independent oracle

The strict Draft 2020-12 schema and fixture live under `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🧬️canonical-pair/`. The independent TypeScript oracle validates the fixture with AJV, independently reconstructs the framing with Node `Buffer`, and verifies the SHA-256 identities and domain-separated ETag with WebCrypto and Node crypto.

Final-source oracle evidence from session `58549`: **green**, 1 passed and 10 skipped; focused test 168 ms, Vitest 4.15 s.

## Focused Rust and route evidence

The final owned command ran with both compiler wrappers disabled and an isolated ticket-local Cargo target:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/canonical-pair-target-final \
  RUSTC_WRAPPER='' CARGO_BUILD_RUSTC_WRAPPER='' \
  bun nx run os-hub:canonical-pair-check --skip-nx-cache
```

Final-source session `58549`: **green**, exit `0`:

- 3 focused reader/framing laws passed;
- 3 route laws passed, 43 filtered, 0 failed;
- the independent neutral oracle passed 1/1; and
- `cargo check --all-features --bin os-hub` completed successfully in 2m51s.

The reader laws cover nonzero/checked 64 MiB and 16,384-record preflight, strict pack-before-SPR framing, missing-terminal/trailing/reordered rejection, fixture identities/ETag, and cancellation/deadline without terminal acceptance. The route laws cover pre-reader query/range/wrong-accept/duplicate-auth rejection; anonymous, malformed, public-fallback, nonmember/admin-bypass, cross-space equal-document, and cross-document-share denial; member and exact-share success; exact response headers; body decode and locator absence; revocation after metadata, before records, and before terminal; deleted/missing CAS; revoked share; and empty error bodies without locator or authority leakage.

The route laws additionally cover pre-allocation oversized and invalid-kind capability denial; bounded monotonic progress through `Ready`; empty-body `504` before work publication; and actual loopback TCP disconnect cancellation. The disconnect law proves that progress remains unchanged after connection loss and that active request ownership is released within the bounded deadline.

## Superseded fixture setup failure

The first isolated run, session `80945`, compiled cold in 4m44s and passed all 3 reader laws plus the early route-admission law, but the loopback route law stopped during setup before any HTTP assertion:

```text
📦️bin.rs:4180 create space: Backend("FOREIGN KEY constraint failed")
```

The fixture had invented owner ids (`another-owner` and `public-owner`) that were not durable users. The repair uses the already issued outsider's exact durable `user_id` to own both fixture spaces. It does not weaken the directory foreign key, membership checks, admin policy, or route authorization. Session `63246` then passed route 2/2 and oracle 1/1. Its trailing plain-feature check was reported as status 101 after warning-heavy tool output truncated the actual tail; a status-preserving retry in session `80798` reached Cargo's own `cargo_status=0`, exit `0`, with no diagnostic. The failure was not reproduced. Pre-audit all-feature session `76469` was green but was superseded after the lifecycle/admission review by the final all-feature session `58549`.

## Target ownership and launch freshness

`os-hub:canonical-pair-check` is owned by `🌎️hub/📦️packages/🦀️rust/📋️project.json` and delegates only to `bun ./📜️script.ts canonical-pair-check`. `CanonicalPairCheckScript` is registered in that directory's `📜️script.ts` and permanently ends with `cargo check --all-features --bin os-hub`.

Both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json` contain `⚖️gate🧭️canonical-pair🛡️server` at gate order `411.11`, invoking `bun nx run os-hub:canonical-pair-check --skip-nx-cache`.

Freshness command:

```text
bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache
```

The final-source rerun on 2026-09-03 was **green**, exit `0`; the registry reported that generated catalog and launch bytes are fresh.

## Cleanup and diff boundary

The exact isolated Cargo directory `<ticket>/🗑️generated/canonical-pair-target-final` occupied 4.8 GiB and was deleted after all final-source checks. Other ticket-generated files, including all Terra reports, and every unrelated concurrent workspace edit were left untouched.

The resume repaired the loopback fixture owner identity, made the permanent check all-feature, bounded and typed bearer admission before allocation, and added the request-owned cancellation/deadline/progress lifecycle plus its loopback laws on top of the already staged P4-B implementation. The live diff was reread after testing; concurrent S3 and browser-broker edits in the same files were preserved.

## Stable claim boundary

- This packet proves the canonical pair reader, exact HTTP route, fail-closed materialized response, neutral framing/receiver, independent TypeScript oracle, final all-feature compile, and launch freshness on macOS.
- PostgreSQL and Neo4j were not provisioned for this packet. Backend-specific runtime parity is not claimed beyond the shared reader/route laws and all-feature source compilation.
- This route currently materializes the bounded pair before returning it. It proves request-future cancellation on TCP disconnect while the handler owns the operation, but does not claim incremental HTTP body streaming after response publication.
- This packet does not claim the separate MCP cache, mount/catalog generation, SocketGrant, artifact retention/sweeper, or live tail-barrier lanes; their authority and lifecycle remain owned by their dedicated packets.
- No public checkpoint payload or successful/error HTTP body exposes a private CAS locator, storage key, bearer, or backend error string.

## Exact source surfaces

- `🌎️hub/🛰️lag-rebootstrap/🦀️.rs`
- `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🧬️canonical-pair/🧬️.schema.json`
- `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🧬️canonical-pair/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-mcp-canonical-pair-p4b.md`
