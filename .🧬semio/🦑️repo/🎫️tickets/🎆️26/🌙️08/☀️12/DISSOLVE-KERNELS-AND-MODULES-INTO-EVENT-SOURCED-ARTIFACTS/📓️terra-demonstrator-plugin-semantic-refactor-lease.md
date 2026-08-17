# Demonstrator Plugin Semantic Refactor Lease

## Baseline

- Owner: `✏️s/🔌️plugins/🎪️demonstrator`.
- Owner worktree state: clean; no owner path was dirty at lease start.
- Protected concurrent paths: all paths in [Semantic Module Refactor Lease Snapshot](./semantic-module-refactor-lease-snapshot.md), especially the root script, root project, registry/export, launch, and lock paths. They are excluded from this lease.
- No nested `AGENTS.md` exists below this plugin; the root and `✏️s/AGENTS.md` instructions apply.
- 130 authored files, including 118 `*component.*` leaves. The whole-owner SHA-256 manifest digest is `c5296c57d878e5dbdfccdf03384fe7c191dfe00db749156724ea6e4008c26cb9`.

| Facet | Leaves | SHA-256 manifest digest |
| --- | ---: | --- |
| Rust | 32 | `430a1ed01ba6d5ae1f22293784978da5e8ab90871f2d53c9ef4b5c4da99c74d8` |
| TypeScript | 29 | `9ccfc5c31f424a29bc4f88d3d952bd93b82b252a9cda544bf0adb7b6145cbabd` |
| GraphQL | 9 | `b1b571f4227b78625fcb8d6ceba12bb786c409882134763d44241b4566db05c1` |
| JSON Schema | 10 | `c3abde15a7f6ef8f0f95d323856eb092ab98bfb5f90c3a01cfc46b2f5b77e829` |
| Proto | 9 | `0a7fcd2afe38c131390e33ab454a125d53b96f678d111348b20b20b423c440b7` |
| Semio grammar | 5 | `40def7fe11661685949d41593b2715a2de95cf6ac5f568e20459193cea65d564` |
| Semio protocol | 4 | `1d50b9b976f4ee72bb01fc553f77f12aacf1bdcab518a533317391e9169920e3` |
| ABNF | 4 | `a3007eb8b3ae365b6e412ccb17d0cd7a2bcca3eef3fb16a7e8e0aa4b1574b5d3` |
| EBNF | 4 | `c331e14306259d4bc340ee1e800869e127ca3e85701d0c10b0ad37dec555cce2` |
| ANTLR | 4 | `139008fd78d06309128e07d2f8d533ca423fbe3f4f09fb070204827932d51c7e` |
| Kaitai | 4 | `1240e5e125f4aaef2b4069f726296d5bf09b421df2f048a69e24a9dc67a5ed56` |
| Spicy | 4 | `4f9bc8e93f770f68629149b83f950be1b192faa4d31e42bf9456132ac6f5ace8` |

The manifest digest is the SHA-256 of deterministic, path-sorted per-file SHA-256 lines. It is the retained baseline evidence for every source, schema, fixture, test, registrar, package file, and generated glue file under the owner.

## Consumer And Taxonomy Census

- Exactly two authored Rust leaves violate the required `<collection>/<specific>/component.<language>` suffix: the plugin root `🦀️component.rs` and generic `🎛️apps/🦀️component.rs`.
- The other 118 authored component leaves are already nested under a semantic collection and specific owner. Schema families are `snapshot`, `diff`, `mutations`, and `inferences`; transport leaves are each owned by their exact format and version.
- There is no implementation-symbol consumer outside the owner. External references are only workspace membership, package/target metadata, catalog discovery, historical documentation, and protected root policy declarations.
- The generated Rust glue is the only production mount of the generic app facade, plugin-root facade, topology module, and old compatibility exports. It is generated and therefore is not writable in this lease.
- `🧭topology` has one terminal production consumer: the playground inference root. It is a candidate for dissolution into that semantic owner, subject to the generated-mount request.
- The generic glue compatibility surface has no consumers outside the owner and must be removed, not preserved: `schema`, `io`, `op`, `dsl`, `spr`, `diff`, `mutations`, and `snapshot` are all aliases to canonical versioned paths.

## Lease Rules

- Preserve exact schema/codec behavior and evaluate source, schema facets, fixtures, local tests, and local registrars together.
- Do not edit generated glue, root/taxonomy/script/index/lock files, or protected dirty paths.
- Use canonical paths directly; no forwarding module, alias, shim, compatibility layer, or migration code may remain.
- A specific extracted component needs semantic equivalence and two independent terminal production consumers. Otherwise fold it into its lowest semantic owner.

## Pending Work

1. Prove the topology fold and remove source-level aliases that would otherwise survive the generated surface.
2. Record a precise central-registrar regeneration request for glue path mounts and deletion of the compatibility modules.
3. Run the scoped Nx targets available after the source/referrer audit, retaining any registrar-dependent validation blocker.

## Applied Semantic Refactor

### Dissolved One-Consumer Owners

| Removed owner | Terminal production consumer evidence | Canonical outcome |
| --- | --- | --- |
| plugin-root `🦀️component.rs` | Only generated plugin entry mount | Folded into `🛂manifest/🎪️demonstrator/🦀️component.rs`.
| generic `🎛️apps/🦀️component.rs` | Only plugin-root `bundle` call | Folded into the same concrete demonstrator manifest; `assemble` is private.
| `💡️inferences/🧭topology/{🦀️,🟦️}component.*` | Rust `compute_playground_topology` had exactly one production consumer, the inference root; TypeScript had no independent consumer | Folded the topology value contract and inference into the Rust/TypeScript inference roots. GraphQL, JSON Schema, and Proto already own the same contract in their inference roots.
| `🚪️io/**/📄txt/🔖️utf-8/✳️any/{🦀️,🟦️}component.*` | No exporter registry row or fixture; both Rust implementations always returned an error | Removed the unsupported representation and its false `stdio.txt` read/capability path. It had no successful behavior to preserve.

The manifest is an irreducible runtime entry, not a reusable umbrella: it owns the demonstrator identity, artifact declaration, and one ordered registration transaction. All six retained foreign app registrations are semantically equivalent to the removed app facade and preserve their former order.

### Canonical Consumer Refactor

- Removed all artifact-root reexports of `PlaygroundSnapshot`, `PlaygroundDiff`, and `PlaygroundMutation`.
- Replaced every internal source reference to the former `playground::{schema,io,dsl,op,spr,diff,snapshot,mutations}` forwarding surface with its concrete `standards::v1::subsets::any` path.
- Removed unused public forwarding exports from schema construction/analysis, IO composition, and mutation text.
- Removed all ten no-op per-format `register()` functions. Artifact declaration owns the single real composer registration path.
- Kept snapshot, diff, mutations, inference, IO, and format leaves because they are distinct schema/transport contracts with multiple independent codec, composition, descriptor, or protocol consumers. The `change-schema` mutation triad remains because its payload, diff, and inverse each implement a different required semantic-mutation contract and are jointly consumed by mutation dispatch and text/binary codecs.

### Changed Paths

All paths below are relative to `✏️s/🔌️plugins/🎪️demonstrator`.

- Added: `🛂️manifest/🎪️demonstrator/🦀️component.rs`.
- Removed: `🦀️component.rs`, `🎛️apps/🦀️component.rs`, `🧬️schema/💡️inferences/🧭topology/{🦀️,🟦️}component.*`, and both `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/📄txt/🔖️utf-8/✳️any/{🦀️,🟦️}component.*` leaves.
- Canonicalized artifact/schema/direct-test consumers: `🗿️artifacts/🎪️playground/🦀️component.rs`, `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,💡️inferences/🦀️component.rs,📸️snapshot/📝️text/🦀️component.rs,🔺️diff/{🦀️component.rs,📝️text/🦀️component.rs},🧬️mutations/{🦀️component.rs,📝️text/🦀️component.rs,💾️binary/🦀️component.rs,✒️change-schema/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs}}`, and `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs`.
- Canonicalized active transport consumers and removed no-op local registrars: `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` and the ZIP/CSV/XLSX/JSON import/export Rust leaves beneath it.
- Central-only generated/package changes, verified but not authored by this lease: `📦️packages/🦀️rust/📦️glue.rs` and `📦️packages/🟦️typescript/📦️index.ts`.

### Final Census

- 111 authored component leaves, with zero paths shorter than `<collection>/<specific>/component.<language>`.
- Source-level public forwarding exports: zero.
- Source-level references through the old generic artifact aliases: zero.
- Topology leaf files: zero.
- Final whole-owner SHA-256 manifest digest: `135c3b782abfb0939eeb1de32cd5afe1b550465f8152f9b0d8653b660685f277`.
- Final Rust manifest digest: `bcf3fa8090a03723b439eb91d2c9f9ad1e91784687daab8375547371a073e92e` over 28 leaves; final TypeScript manifest digest: `83be398e74cc98683b5999910ae7c2053ade0d5da10890fd572978534f668818` over 26 leaves. Every other authored schema facet digest is unchanged from baseline.

## Central Registrar Handoff

The exact generated-file and TypeScript-index request is retained in [Terra Demonstrator Central Registrar Request](../🔧️patches/terra-demonstrator-central-registrar-request.md). The central registrar applied the topology/TXT/shim/apps removal, concrete manifest mount, restored canonical inference text/binary mounts, and minimal TypeScript package marker; this lease did not edit either generated file.

## Validation

| Check | Result |
| --- | --- |
| `git diff --check -- ✏️s/🔌️plugins/🎪️demonstrator` | Passed. |
| Taxonomy/alias static census | Passed: 111 leaves, zero short component paths, zero source forwarding exports, zero generic alias references, zero topology leaves, zero no-op local registrars, and zero unsupported `stdio.txt` source references. |
| `bun nx run @semio-tech/demonstrator-js:test` | Passed; emitted `[DEBUG] demonstrator ts ok`. This target only runs its current package script and does not type-check the registrar-owned index. |
| `bun nx run @semio-tech/demonstrator-plugin:test-quick` | Passed after the central registrar corrected the concrete `🛂️manifest` path and the five remaining direct owner/test imports were canonicalized. |
| Rust formatting | Blocked: `rustfmt` is not installed in this environment (`command not found`). |

The scoped Rust and JavaScript targets pass. Rust formatting remains unavailable in this environment; no protected dirty path was edited.

## Final Release Review

- Final `bun nx run @semio-tech/demonstrator-plugin:test-quick` exited `0` against the corrected central glue. The shared scoped run covers the demonstrator's 20 Rust tests.
- Final `bun nx run @semio-tech/demonstrator-js:test` exited `0` and emitted `[DEBUG] demonstrator ts ok`.
- Both staged and unstaged `git diff --check` reviews passed for the owner.
- The final direct-referrer sweep found no source reference through a retired playground facade, no forwarding `pub use`, no TXT capability/reference, no topology component, and no short component path. The restored inference text/binary glue mounts are present at `📦️glue.rs:50` and `:52`; removed topology/TXT mounts are absent.

### Remaining Non-Demonstrator Conditions

- `rustfmt` is not installed in the execution environment, so a formatter run cannot be recorded.
- The successful Rust target still emits pre-existing warnings from framework and other plugin owners. They are outside this lease and do not fail the demonstrator target.
