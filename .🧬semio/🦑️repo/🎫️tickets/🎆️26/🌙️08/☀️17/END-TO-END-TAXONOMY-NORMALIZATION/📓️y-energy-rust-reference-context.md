# Energy Rust Reference Context

## Scope and Authority

This lane changes shared Rust syntax discovery and taxonomy reference resolution. It does not move production files, modify shared Git state, or inspect the opaque Compose trees. Exact Energy Cargo and glue inputs are read only. Authored vectors, test sources, Cargo oracle source, and this audit are retained; completed generated compiler output will be disposed of only after it is no longer needed.

The Energy aggregate contains five literal-array surfaces passed through `owner.join(surface)`. Its `owner` is the immutable `replace-model` child of a manifest-relative mutation root, not the aggregate source directory. The previous generic `.join("...")` matcher could therefore select sibling lookalikes. The replacement resolves the actual Cargo manifest through the declared Rust module graph, then follows exact lexical local bindings. Normalization must rewrite each segment relative to its projected physical base.

The required external context consists of the Energy Rust package manifest and its declared glue entry point. Nested inline modules explicitly use `#[path = "."]`; the glue mounts the aggregate and scenario files with exact path attributes. Multiple lexical contexts in one source do not imply multiple Cargo owners. Distinct manifest owners are ambiguous and must reject physical-path authority.

## Released Implementation

- Added language-neutral path-binding and module-ownership vectors before implementation; the first run failed on the missing exported implementation.
- Shared `inspectRustModuleGraph` now carries manifest identity and source-chain provenance. The root mutation graph delegates to it while retaining conventional-root and dependency behavior; conventional roots do not confer Cargo environment authority.
- Shared `inspectRustManifestPathReferences` recognizes immutable local bindings rooted at the fully qualified standard Path constructor and exact Cargo environment macro. Mutable/shadowed bindings, arbitrary constructors/environments, computed arrays, macro payload lookalikes, escaped literals, and loop variables used elsewhere are not accepted.
- The normalizer no longer treats an arbitrary `.join("leaf")` as source-file-relative. Proven references carry exact physical targets, source base, and digest-bound module/manifest provenance. The lexical unsupported-token pass cannot reinterpret an already proven join span as a sibling reference.
- Standard assertion-message ownership is separately tokenized and tested; the Energy diagnostic path is updated as an actual message reference.
- Cargo facts use the existing repository-owned TOML parser interface in strict ownership mode. The root mutation graph retains its former conventional-root and dependency behavior. Physical reference authority currently requires a declared/default library entry; this release does not invent ownership for build scripts or binary entries.
- Potential basename/suffix matches are admission or unsupported-reference evidence only. They never authorize an edit. A physical edit requires the exact manifest-relative target and frozen source-chain hashes.

## Executed Checks

The full registered command was executed with `NX_DAEMON=false`: `bun nx run @semio-tech/repo-lib:test-rust-physical-reference-context --skip-nx-cache`. Before the repository-wide scope correction below, it passed **15 tests, 103 assertions, 11.03 seconds**, with Nx exit zero. The final expanded command passed **19 tests, 127 assertions, 38.25 seconds**, with Nx exit zero. The existing `long` test level accommodates a cold Cargo oracle; fundamental and transaction time gates were not relaxed.

The packet includes exact Nx/launch registration, immutable and shadowed bindings, source module ownership, a deliberately wrong sibling, source and committed Cargo runtime reads, rollback/retry/commit and empty replan, missing/ambiguous ownership, unsafe inputs, consumer/manifest drift, new incoming references, and cancellation. The independent test-only `syn` AST implementation reproduces the language-neutral assertion and binding outputs. The actual Cargo runtime prints the expected target contents before rollback, after rollback, and after commit. `pathe` independently reproduces the nested-repository physical coordinates used by the scope regressions.

These are bounded integration results, not full-monorepo acceptance. The coordinator separately owns the complete Energy fixture and real production plan/apply. No production files were moved by this lane.

## Repository-Wide Ownership Scope Correction

The first fresh Draw scan rejected a retained negative fixture's symlinked Cargo manifest. Its caller was only joining a local fixture path, but the incoming index combined all lookup candidates with affected targets and eagerly built the fixture's whole Cargo graph.

New language-neutral cases reproduced this before the fix: the unrelated same-repository unsafe manifest and independent local fixture failed; the explicit parent escape already rejected correctly (**two failures, one pass**). The normalizer now carries affected paths separately from its full lookup/module universe. Ownership parsing is admitted only for an affected consumer or a possible affected physical target. The cache remains per index and independently proven repository coordinate root.

Unsafe nodes are not read and cannot confer module ownership. Their exact no-follow errors remain recorded; a governing unsafe manifest or source still rejects. Only regular, successfully parsed manifests and mounted source chains can provide an owner. An unrelated unsafe node does not manufacture ambiguity or block an independently proven owner. This changes reference authority, not inventory membership or normalization exclusions: neither ticket paths nor malformed nodes receive a permanent exemption.

The focused packet passed **six tests, 19 assertions**. The added independent-local case also passed rollback, retry, commit, source preservation and empty replan. An explicit parent escape remains rejected; a newly relevant unsafe consumer inserted after planning rejects fresh preapply without overwriting its source or symlink. The three extended scope/preapply cases passed **21 assertions** before the final 19-case registered run.

## Permanent Authored Inputs and Routing

The test source and Rust oracle were moved from temporary ticket locations into their semantic owners; their authored content was not discarded. The Cargo oracle manifest is a canonical TOML fixture input, not a new production Cargo workspace package. The registered route uses `📜️script.ts test rust-physical-reference-context long`; both launch catalogs place its target in `4_gate`, order `410.08`.

| Authored input | Bytes | SHA-256 |
| --- | ---: | --- |
| [Language-neutral vector](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️rust-physical-reference-context/🔣️.json) | 10255 | `19b0d562bdb5b6024f46f76d341431c20b7e731d31489b12fab8efa3048fb7e1` |
| [Permanent runtime test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-physical-reference-context/🟦️.test.ts) | 17846 | `39f5822a5255a66a4d69ed5d9d84790723d401e71ca0333606a5ab497d5b4a8e` |
| [Independent Rust oracle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-physical-reference-context/🦀️.rs) | 8799 | `e0a72c109aecab70f52c752ccde9cb1e5bd49d0c92357ee452f2177d5d61bdd0` |
| [Oracle Cargo input](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️rust-physical-reference-context/🧪️oracle/⚙️.toml) | 239 | `c0d028529d64853ec8ef892c7dce11a1be8dc25f243835e7fcd10294c5bedd08` |

The permanent tests materialize these authored inputs in unique ticket directories. Their completed Cargo target outputs and generated lockfiles are removed in `finally`; input copies remain. No hostile fixture was deleted to obtain a passing plan. Active transaction/recovery evidence is not treated as disposable while still needed for coordinator review.

The shared implementation locations are [discovery](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts), [normalization](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts), and the root mutation graph wrapper in [the root script](/Users/ueli/Documents/semio/📜️script.ts). All owned generic helper regions are released. The durable complete-Energy test design is recorded separately in [the acceptance-authority review](📓️z-energy-durable-test-authority.md).

## Earlier Compiler Output Awaiting Scoped Disposal

A bounded no-follow audit inspected only Rust fixture roots and their exact Cargo output names. Eight older output targets remain, totaling **167,253,994 bytes and 1,554 nodes**, with no symlinks found inside those targets. They are not authored inputs or new evidence authority. Disposal is deferred while the coordinator's live incoming capture is reading the ticket; no target, lockfile, hostile fixture, or transaction tree was deleted by this audit.

| Exact ticket-relative output | Bytes |
| --- | ---: |
| `🧪️rust-path-VMJhOv/🧪️build` | 5036628 |
| `🧪️rust-path-VMJhOv/pkg/Cargo.lock` | 151 |
| `🧪️rust-path-ypAkXJ/🧪️build` | 5036627 |
| `🧪️rust-path-ypAkXJ/pkg/Cargo.lock` | 151 |
| `🧪️rust-coordinate-oracle/🧪️target` | 133329064 |
| `🧪️rust-coordinate-oracle/Cargo.lock` | 2932 |
| `🧪️rust-syn-oracle-71MY99/🧪️target` | 23845509 |
| `🧪️rust-syn-oracle-71MY99/Cargo.lock` | 2932 |
