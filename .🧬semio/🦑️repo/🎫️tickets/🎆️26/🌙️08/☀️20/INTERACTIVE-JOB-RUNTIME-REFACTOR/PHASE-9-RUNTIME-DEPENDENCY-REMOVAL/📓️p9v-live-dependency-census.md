# P9v Live Dependency Census

## Scope

This checkpoint records the dependency-freeze inventory on 2026-08-21 after the owned schema, async, SHA-256, WIT-inspection, MVT-protobuf, and stdio async-trait packets. `compose/` remains excluded by the master plan.

## Reproducible Commands

```text
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list rust
bun ./📜️script.ts verify dependencies list js
```

## Result

- Baseline: 238 unique third-party ecosystem/name entries at `95b8688ee2f62f4056b6403c282bf0c76172c37c`.
- Current: 215 entries; dependency freeze passes with no additions.
- Removed from the baseline: 23 entries.
- Current Rust names: 81.
- Current JavaScript names: 134.
- Newly confirmed removals in this checkpoint include `async-trait`, `prost`, and `wit-parser`.

The current count is a progress ratchet, not the Phase 9/10 exit gate. Phase 9 still owns runtime replacements including compression/ZIP, entropy, media, geometry, network/server, storage, WASM host, and the remaining schema/serialization rows. Phase 10 still owns the native UI/render stack and the JavaScript build/test/product stack.

## Active Packets

- Owned deterministic constrained triangulation replacing `spade`.
- Owned resumable fixed-Huffman DEFLATE plus retained-format parity work.
- Renderer-owned async entrypoint and native I/O boundary work.
- Database and infinite-crate de-async compilation repair, which protects subsequent dependency gates from stale call shapes.

## 2026-08-22 Checkpoint

After the owned-WASM bounded packet, owned compression follow-up, manifest cleanup, and intervening
runtime packets, the live freeze gate reports **207** third-party ecosystem/name identities, down 31
from the 238 baseline. The current split is **73 Rust** and **134 JavaScript** identities; Rust is 70
runtime and 3 test-only. The root-only `flate2` and `libz-sys` workspace declarations were removed,
leaving zero external row whose sole user is the root `Cargo.toml`.

`bun ./📜️script.ts verify dependencies` exits `0` with no new dependency. The baseline removals now
include `rayon`, `spade`, `thiserror`, `ts-rs`, `notify`, `reqwest`, `uuid`, `jsonschema`, `prost`,
and the two stale root compression declarations. This remains a progress ratchet rather than the
Phase 9/10 exit gate: 207 external identities are still present.

## 2026-08-22 Animate and Compose Follow-up

The freeze gate now reports **205** third-party ecosystem/name identities: **71 Rust** and **134
JavaScript**. The two identity-level removals since the preceding checkpoint are `rust:comemo` and
`rust:ecow`; both were stale direct Animate dependencies, with Animate's public text values moved to
owned `String`. Additional source-proven stale rows (`fontdb`, `base64`, and five Compose JavaScript
rows) were removed from individual manifests but remain identity-level dependencies of other
workspace packages, so the unique-name total correctly falls by two rather than by the raw row count.

`cargo metadata --no-deps --format-version 1`, `bun ./📜️script.ts verify dependencies`, and both
ecosystem list commands exit `0`. This is still not the Phase 9/10 gate: **205 external identities
remain**, and the Animate crate's separate 1,296-diagnostic de-async/generated-macro backlog prevents
claiming a clean crate check for that packet.

## 2026-08-22 Owned Planar Boolean Follow-up

After replacing `geo` with the owned deterministic planar-arrangement kernel, the coordinator reran
`bun ./📜️script.ts verify dependencies`. The command exits `0` at **204** third-party identities:
**70 Rust** and **134 JavaScript**, down 34 from the 238-identity baseline. `rust:geo` is now among
the baseline removals, and no new identity was introduced. This remains a progress checkpoint rather
than the Phase 9/10 exit gate; all 204 remaining identities still require owned replacements or
source-proven deletion.

## 2026-08-22 Native Futures Surface Consolidation

The OS synchronization actor's only `futures-util` use was native WebSocket extension traits and
split-stream types. Those imports now use the already-retained `futures` facade, and the facade row
is declared only in the non-WASM target table. `cargo check -p semio-framework-os-kernel --features
sync --lib` passes. The WASM check was started but cancelled while it was waiting for the shared
Cargo build lock, so no WASM result is claimed at this checkpoint; the manifest target boundary is
nevertheless explicit and the prior unconditional `futures-util` edge is gone.

The dependency freeze now exits `0` at **203** third-party identities: **69 Rust** and **134
JavaScript**, down 35 from baseline. `rust:futures-util` is the new identity-level removal and no new
identity was introduced. This is consolidation toward the owned async runtime, not a Phase 9
closure claim: `futures` remains external and must still be removed with the remaining runtime.

## 2026-08-22 Layout Font Surface Consolidation

Layout's only direct `fontique` use was `Blob`, already part of its public Parley text boundary. The
scene engine now imports `parley::fontique::Blob`, so the redundant direct manifest row is removed
without changing the exposed Layout API. The exact Layout crate check advanced through that import
and then failed in the separate de-async repair backlog with 696 errors (first error: the new
`ComposeSource` lifetime contract); no clean Layout build is claimed here.

The dependency freeze exits `0` at **202** identities: **68 Rust** and **134 JavaScript**, down 36
from baseline. `rust:fontique` is the new direct identity-level removal and no identity was added.
Parley still brings its text stack transitively, so this remains surface consolidation rather than
the Phase 10 owned-text replacement or a zero-dependency claim.

## 2026-08-22 Infinite Font Database Surface Consolidation

Infinite's two bundled-font setup sites now construct the database through the existing SVG
boundary (`usvg::fontdb::Database`), removing its redundant direct `fontdb` declaration. Cargo
metadata and the dependency freeze pass. The exact Infinite library check was attempted repeatedly
but cancelled while waiting for the shared build-directory lock owned by active long-running gates,
so no new Infinite build result is claimed at this checkpoint.

The freeze gate exits `0` at **201** identities: **67 Rust** and **134 JavaScript**, down 37 from
baseline. `rust:fontdb` is the new direct identity-level removal. `usvg` still brings its font
database transitively; this is a narrower dependency surface, not the owned SVG/font stack or the
Phase 10 exit gate.

## 2026-08-22 Infinite Vello Encoding Surface Consolidation

Infinite exposed an unused public escape hatch returning `vello_encoding::Encoding`, even though
all internal consumers use the owned `Scene::is_empty` and `Scene::path_count` wrappers. Removing
that escape hatch also removes the only direct `vello_encoding` manifest row and prevents an
external renderer type from leaking through the public API. `cargo metadata --no-deps
--format-version 1` and the dependency freeze pass, and the source/manifest census contains no
remaining direct `vello_encoding` reference outside ticket history.

The freeze gate now exits `0` at **200** identities: **66 Rust** and **134 JavaScript**, down 38 from
baseline. The retained `vello` renderer still depends transitively on its encoding package, so this
is API/surface consolidation rather than the owned-renderer or zero-dependency exit gate.

## 2026-08-22 Owned Shader Contract Validation

The UI render contract no longer declares `naga` as a test dependency. Its shader registry now uses
an owned lexical and structural WGSL gate that skips nested comments, balances delimiters, rejects
illegal `async fn`, discovers vertex/fragment/compute entry points, and proves that every configured
pipeline entry point is present. Runtime backend/device creation remains the semantic shader
validator; this packet does not claim that the structural gate is a complete WGSL type checker.

`cargo test -p semio-framework-ui-render --lib` passes all **125** tests after the replacement, and
`cargo fmt` plus the dependency freeze pass. A separate `RUSTFLAGS='-D warnings'` rerun was cancelled
while waiting behind the production owned-WASM link to avoid exhausting the shared volume, so no
new strict-warning result is claimed here yet.

The freeze gate exits `0` at **199** identities: **65 Rust** and **134 JavaScript**, down 39 from the
238 baseline. `rust:naga` is the new identity-level removal. This is an owned test-contract
replacement; the native render backend cohort remains in Phase 10.

## 2026-08-22 Owned Test Drivers

Two test-only Rust identities were removed without weakening the exercised production boundaries:

- MCP HTTP tests now drive the real `axum::Router` through a local `poll_ready` + `call` one-shot
  adapter, eliminating the direct `tower` utility dependency and all `.oneshot` call sites. The
  focused MCP no-run build type-checks that transport module and currently stops later in the
  concurrently edited owned-plugin workspace boundary; no transport error remains.
- Hub Postgres tests now use a private disposable-container fixture over the Docker CLI, with an
  ephemeral loopback port, deterministic Postgres 16 image, readiness deadline, and `Drop` cleanup.
  `cargo test -p semio-hub --no-default-features --features postgres --lib --no-run` exits `0` with
  no Hub warning. Runtime execution was not claimed because the local Docker daemon is unavailable.

This removes `rust:tower` and `rust:testcontainers-modules` from the direct dependency census.
Alongside three source-proven unused root JavaScript rows (`@modelcontextprotocol/ext-apps`,
`lint-staged`, and `vite-plugin-singlefile`), the freeze gate exits `0` at **194** identities:
**63 Rust** and **131 JavaScript**, down 44 from baseline. Transitive `tower` remains behind Axum;
this is a direct-surface reduction, not the owned HTTP stack or zero-dependency exit gate.
