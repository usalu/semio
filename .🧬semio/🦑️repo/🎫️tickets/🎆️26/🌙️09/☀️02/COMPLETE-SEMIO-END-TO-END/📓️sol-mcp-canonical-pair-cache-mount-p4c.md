# MCP Canonical Pair Cache/Mount P4-C Implementation

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Stable source boundary

P4-C now adds one private canonical-pair actor to each `HubRemoteBinding`. The actor is authority-generation fenced and owns the complete descriptor-ready, per-full-scope loading, verified-cache, opaque-mount, refreshing, revoked, and closed lifecycle. Its exact identity contains normalized Hub origin, opaque authority generation, full document scope, descriptor digest, active checkpoint, canonical P4-B ETag, and optional catalog generation. Equal full identities may join one in-flight receipt; other identities cannot. Join completion is a typed `Pending | Published | Failed` signal, so invalidation cannot be confused with publication or return a stale cache entry.

The cache is process-local, bounded to four entries and 8 MiB, and admits at most 4 MiB per verified pair. Receipt and mount identifiers use checked increments and fail closed on exhaustion. Refresh, stream loss/reconnect, rebootstrap, membership/session revocation, non-success HTTP authority results, integrity failure, deadline, and restart discard the mount, cancel/fail receipts, and wipe verified bytes. Raw pair bytes and bearer text do not enter public resources or diagnostics; the MCP resource boundary remains descriptor-only.

The receiver independently implements the P4-B wire contract: exact media type and path-only request, unique required response headers, Content-Length preflight before allocation, bounded incremental reads, big-endian header and record framing, pack-before-SPR contiguous offsets, exact terminal and no trailing data, part and aggregate SHA-256, domain-separated ETag, full authority identity, elapsed cancellation/deadline checkpoints, fresh pre-publication session/descriptor authorization, and zero-on-drop wire/scratch/part/cache owners. The worst-case wire bound counts split pack and SPR record ceilings separately. Empty `head_edit_id` is accepted only as the optional initial frontier field; required identifiers remain non-empty.

Native integration keeps the protected credential inside `NativeDirectoryTransport`, sends the exact authenticated GET through the existing bounded HTTP pool, streams the response into the receiver, and maps transport cancellation/deadline/availability without returning locator or credential details. `NativeHubBindingDriver` owns this transport beside the binding and exposes only an opaque verified mount.

## Neutral contract and independent oracle

The neutral JSON fixture and strict JSON Schema contain one literal 342-byte P4-B response plus fifteen negative vectors: truncation, reordering, duplication, oversized record, wrong scope, wrong digest, wrong checkpoint, malformed UTF-8, control-character text, bad part hash, bad aggregate, bad ETag, missing terminal, trailing data, and equal document id in another space.

The Bun/Node oracle is independent of Rust. AJV validates the schema; fatal `TextDecoder` plus Unicode control classification validates text; WebCrypto and Node crypto independently recompute hashes and the domain-separated ETag; and a structural key/resource check proves binding/space isolation and descriptor-only resource exposure.

Fresh final-source oracle terminal:

```text
canonical-pair-cache-mount oracle: 3/3; negatives 15/15
```

## Rust laws and permanent gate

Three focused Rust laws are present for:

1. literal neutral decode, every malformed vector, optional initial frontier, and candidate wiping;
2. exact transport request, cache hit without HTTP, fixed-credit eviction, cross-binding isolation, and revoke clearing; and
3. Content-Length preflight, streaming cancellation/deadline, expiry during receive, equal-identity join failure typing, no resurrection after invalidation, and rebootstrap invalidation.

The existing MCP `📜️script.ts` owns `canonical-pair-check`. Its uncached Nx target runs the three exact laws, the independent oracle, the MCP all-feature check, and the Hub all-feature binary check. The authored launch seed and generated launch output register `⚖️gate🧭️canonical-pair🌉️cache-mount` immediately after the server-side canonical-pair gate.

## Truthful runtime boundary

No Rust assertion is credited yet. Two isolated no-run attempts were invalidated by the concurrent taxonomy move before the MCP crate could compile:

- session `20933d` stopped during workspace loading because the root/FEM dependency still named the temporarily missing single plugin package path;
- session `37653` completed the cold dependency build, then stopped with Cargo status 101 because the MCP package moved from single to doubled `📦️packages` while rustc was live, so Cargo's already-loaded manifest referenced a source path that ceased to exist.

Neither terminal contained a P4-C source diagnostic. The exact retry is:

```text
env CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/mcp-canonical-pair-p4c-sol-target' RUSTC_WRAPPER= cargo test --manifest-path Cargo.toml --lib --no-run
```

It must run from the currently resolved MCP Rust package directory after a stable path interval, followed by the uncached Nx `canonical-pair-check`. Launch generation is likewise pending a stable registry path interval: the prior generator attempt stopped before writes because its import moved concurrently. The manually projected launch bytes match the authored seed, but generator freshness is not claimed.

## Verification hygiene

`rustfmt` parses the final pair source, the TypeScript router/project JSON parse, and `git diff --check` is clean for the owned boundary. The isolated target is retained only to avoid another cold dependency build when the taxonomy paths stabilize. No sibling target or report was removed.
