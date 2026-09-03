# P4-C Canonical Pair Cache/Mount Final Audit

## Scope and evidence state

Reconstructed on 2026-09-03 after concurrent ticket cleanup removed the earlier audit file. This is a read-only current-tree audit. It does **not** credit a reported command, compiler check, or oracle run as an independent runtime result.

Audited current sources:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🧪️oracle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`.

Initial verdict: **REJECT — source-level cache/revocation race and vacuous Rust-law gate; runtime remained unexecuted.** The successor re-read below records the current source disposition; this historical initial verdict is retained for traceability.

## Source-closed properties

The receiver uses an authenticated native HTTP-pool transport. It normalizes the credential origin, rejects a mismatched origin/media type/query/fragment, uses `fetch_protected_stream`, requires exactly one nonempty content type, ETag, and Content-Length, and bounds declared length before body receipt (`pair/🦀️.rs:98-133`). The body checks cancellation before reading, maps deadline/unavailable failures, rejects an overlarge chunk, and wipes the owned chunk after copying (`:69-94`).

The wire reader is bounded at 16 KiB header, 4 KiB record payload, 4 MiB verified bytes, 8 MiB cache, and four cache entries (`:13-18`, `:794-918`). It validates protocol/version, exact scope and descriptor digest, nonzero hashes, ETag domain separation, strict records/offsets/order/terminal, part and aggregate SHA-256. `WipeVec`, `WipeScratch`, and `PairBytes` wipe owned receipt material on all ordinary drops (`:228-305`); cached raw pair bytes are private, and `CanonicalPairMount` exposes only identity, baseline, and an opaque mount id (`:137-166`). P4-B contributes only a baseline checkpoint, not an authenticated required-tail/currentness claim.

The actor is binding-owned. Descriptor refresh, stream loss, rebootstrap, invalidation, revocation, and drop cancel outstanding loads and clear verified cache; `HubRemoteBinding` zeroes authority generation before invoking the actor (`remote/🦀️.rs:236-270`). Cache identity includes normalized hub origin, authority generation, scope, descriptor digest, checkpoint id, ETag, and optional catalog generation (`pair/🦀️.rs:137-146`, `:663-668`). The in-flight fetch path checks cancellation, expiry, exact expected identity, post-decode binding/authority generations, refreshed descriptor state, then publishes atomically under the actor lock (`:507-650`).

The Node oracle is materially independent for the canonical pair binary format: it uses AJV, Node `Buffer`, `TextDecoder({ fatal: true })`, Node SHA-256/WebCrypto, and its own frame/field/parser/negative mutations rather than importing Rust or the shipped MCP receiver. It validates the neutral fixture, valid frame, 15 malformed vectors, cache-key separation, no raw resource-shaped URI, and lifecycle-vector presence (`pair/🧪️oracle/🟦️.ts`). It is a corpus/parser parity oracle, not a real protected HTTP-pool, cancellation, or revocation race test.

The registered target is noncached and launch-seeded/generated at 411.12 (`project.json`; `.vscode/🧩️launch.seed.jsonc:3095-3103`; `.vscode/launch.json:4443-4451`).

## RED — cache hit can cross concurrent invalidation or revocation

`HubRemoteBinding::mount_canonical_pair` validates the snapshot and expected identity, takes the actor lock, obtains a cached mount, releases the lock, and returns immediately (`pair/🦀️.rs:527-531`). A concurrent `invalidate` or `revoke` can then increment the binding generation, zero authority generation, and clear that same actor cache (`remote/🦀️.rs:254-270`) before the caller receives the successful cached mount.

The fetch path does have a post-decode generation/ready-snapshot fence (`pair/🦀️.rs:627-650`); the cache-hit path does not. The mount is opaque and does not leak pack/SPR bytes, but a revoked or invalidated authority may still receive a semantically stale mount success. Add a post-cache-hit authority-generation, ready-state, and actor-generation fence that is linearized with invalidation, and a deterministic cache-hit-versus-revoke/invalidate law.

## RED — registered Rust laws are selector-vacuous

`CanonicalPairCheckScript` names three fully qualified tests and invokes Cargo with `--exact`, but does not first list/filter to prove one matching test, nor inspect the result for one executed test (`mcp rust 📜️script.ts:45-53`). Cargo can exit successfully after running zero tests if a law is renamed or missing. The neighboring P4-B/D0/admin gates use an explicit `--list` / one-match / FQN sequence; this one does not. Add the same exact-one selector proof before execution.

## Test coverage boundaries

The three source laws cover neutral malformed frame rejection/wipe, basic cache identity and fixed-credit eviction, and preflight/cancellation/expiry/in-flight invalidation/rebootstrap. They use an in-module `TestTransport`, not `NativeCanonicalPairTransport`, so they do not prove production header uniqueness, protected-pool credential injection, or real HTTP cancellation. They also do not exercise a concurrent cache-hit/revoke return boundary.

The Node parser does not test the native transport and is intentionally unable to prove raw-memory wiping. It also does not enforce the Rust header-frame 16 KiB maximum in its independent parser; current field maxima make the committed fixture small, but the oracle should encode the published header bound and a corresponding negative vector so future protocol-field expansion cannot silently drift.

## Acceptance requirements

1. Fix the cache-hit invalidation/revocation fence and add a deterministic race law.
2. Make the registered Rust selector exact-one and retain the noncached target/launch entry.
3. Extend the independent oracle to enforce all published framing bounds, including the header limit.
4. Run the registered uncached P4-C target after the source is stable and record its terminal output. A compiler check alone is not runtime acceptance.

## Successor Re-read — Cache-Return Fence and Gate Selector Closed in Source (2026-09-03)

The two RED observations above are source-superseded by the current bytes. `finish_mount_return` now takes `pair_actor` before its final actor-identity and binding/authority-generation check, and returns while that lock remains held (`pair/🦀️.rs:644-678`). The invalidating transitions take that identical lock *before* changing either generation/authority and before clearing the actor (`remote/🦀️.rs:244-253,260-280`). Thus a return which acquires the actor first linearizes before the invalidation; an invalidation which acquires it first makes the return's final generation/actor check fail. The deterministic cache-hit law pauses after cache lookup and before that shared-lock fence, runs a real `revoke`, and requires `StaleCompletion` plus the `Revoked` actor state (`pair/🦀️.rs:1321-1344`). This is the correct boundary for the earlier race; the test-only snapshot installer is not part of the production transition path.

The registered P4-C check is also source-non-vacuous now. It imports the bounded probe helpers, lists each of four suffixes, requires exactly one fully-qualified `: test` result, logs the names, and exact-runs them serially before the independent Node oracle and both all-feature checks (`mcp rust 📜️script.ts:5-15,49-76`). The fourth subject is the cache-hit/revocation law. This supersedes the former three-subject, selector-vacuity finding.

No independent target terminal has been run by this audit after this source change. The framing/oracle coverage qualification in the prior section remains to be re-read separately; source closure of the race and selection mechanism is not an execution or end-to-end transport verdict.
