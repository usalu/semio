# DSL Fixture Sweep Test Leaf

## Boundary

Extracted only the former `#[cfg(all(test, feature = "dsl-fixture-sweep-full"))] mod tests` into `semio-framework-os-dsl-fixture-sweep`, a normal workspace test leaf under the fixture-sweep taxonomy. The kernel still mounts the original source file, and its later M5 grammar/protocol modules are retained byte-for-byte. Kernel dev dependencies are now only the async test macro. The new leaf owns all 28 former fleet edges and the same macro as normal dependencies, plus the public kernel API. No optional runtime edge, reverse dependency, repository-test-host dependency or duplicate test was added.

The neutral fixture was pinned before extraction: 54 registry rows (including Workflow, Space and Collection host snapshots), two asynchronous laws, 29 exact dependency identities/default-feature policies, the whole original module body and the retained M5 suffix. The module comparison reverses only the necessary external-crate qualification and restores its original two-newline EOF boundary. Registry, discovery, imports, test bodies and existing assertions otherwise match the original hash. AJV exact structural equality and independent Node/WebCrypto hashes reject 16 hostile changes. A second filesystem traversal independently checks Bun glob discovery.

## Registered Commands

`@semio-tech/dsl-fixture-sweep-rs:source-check` runs only the neutral/source oracle. `test-quick` stays source-only. `test`, `test-long` and `test-exhaustive` use the shared exact Cargo runner against one integration binary, selecting both preserved FQNs exactly once. The explicit root `test dsl` phase now routes to that leaf. No focused kernel target routes to the sweep.

The native gate also requires the actual sweep summary: nonzero example directories, fixture files and law checks, exactly 54 registered rows, and nonzero migrated asset coverage. It records unmapped and soft-skipped counts without changing the original per-fixture behavior. An empty/all-unmapped or all-soft-skipped run cannot become green. `RUST_TEST_NOCAPTURE=1` makes the existing runtime counts observable in retained exact-law logs; this is not a runtime result yet.

Two launch seed entries expose source and native checks. The native entry explicitly supplies the active ticket's generated evidence root, the existing warm `native-openable-provider-sol-target`, one Cargo job and the bounded one-hour build budget. Generated launch was changed only through its owner.

## Evidence

- Baseline read-only census `e33563`: 54 registry rows, two laws and 29 dev edges. All 29 physical dependency manifests verified present by `e2a78a`.
- Registered source gate 92419: intentional RED on the still-live kernel fleet feature, before extraction.
- Source gates 96683 and 97203: RED on the extracted module's final-newline hash difference only. Corrected comparison preserves the original EOF convention; no assertion or registry expectation changed.
- Registered source gate 8455: GREEN, exit 0; 54 rows, 28 moved edges, one retained/shared async macro, two preserved laws, 16 hostile cases. Read-only discovery found 186 example directories and 365 asset-first `.semio` files; inventory SHA-256 `52e4fc3934e8bd0f503d83678f25dc17a9d4c2198b585e21145677b8cb231ac0`.
- Final registered source gate 84971: GREEN, exit 0, the same 54/28/2/16 contract and 186/365 discovery counts. Bun glob and the independent recursive traversal agree on every path; Node/WebCrypto reproduce both pinned source hashes. The root DSL routing and source-only quick target are checked as well.
- Owner launch generation 16107 and freshness 46634: GREEN, exit 0; 59 plugins, 60 playgrounds and 45 framework packages. Both dedicated launch entries are present in generated output.
- Nonmutating Rust parser check d91c31 and owned diff hygiene e04eed: GREEN. These are syntax/source evidence, not compilation or assertions.

## Explicit Remaining Qualification

No Cargo check, native build or exhaustive sweep was run for this packet: the backend owns the fleet's serialized heavy slot. Native law-check/unmapped counts and actual exhaustive conformance remain pending, not inferred from filesystem counts. Historical root full-fleet metadata 32064 RED and stdio/provider/all-features REDs remain separate and unchanged. The split removes a misplaced dependency boundary; it does not claim those implementations or member runtime assertions are accepted.
