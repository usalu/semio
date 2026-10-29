# Part21 Selection and Authority Boundary — 2026-10-29

## Actual selected Cargo graph

The exact command used for99185 was ordinary release/shared/parallelism1 Cargo from the stdio workspace:

`repo cargo --store=shared --configuration=release --parallelism=1 -- test --manifest-path 📇️registry/🧬️contract/📐️part21/📦️packages/🦀️rust/Cargo.toml --lib original_erased_part21_sources_keep_binary_admission_and_funded_closure -- --test-threads=1`

Recovered the actual retained planning memo read-only via canonical cache budget → rust partition → unit-cache layout, without starting Cargo or opening compiler authority. Memo:

`.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/p/6d6275b1d07b1f374bae425789b1248ef3a672bd3e365f352a2fe1e0f708db9f.plan.json.gz`

Compressed SHA256 `5077d2640e7b41f2807109bc872d645bd4d159c95e3f9fba4bd32f477f7315f7`, mtime2026-10-28T23:33:40.931Z. Exactly one raw root447: Part21 library, mode=test.509total units,382local workspace units matching the real admission log. Every509unit is reachable from that one root. Breadth-first traversal gives the shortest actual unit path:

| Unit | Package | Mode | Dependency edge |
|---:|---|---|---|
|447|semio-s-artifact-stdio-contract-part21|test|dev-dependency contract-testing|
|448|semio-s-artifact-stdio-contract-testing|build|unconditional normal OS-kernel|
|170|semio-framework-os-kernel|build|unconditional normal store-engine|
|195|semio-framework-os-store-engine|build|actual oversized source owner|

Evidence subset: `🗑️generated/part21-captured-selection.json`. This is authentic recorded graph evidence, not a fabricated/synthesized receipt or a fresh compilation.

The current canonical planning projection keeps test arguments unchanged. Capture removes only harness arguments after `--` when constructing planning selectors, keeps manifest/`--lib`/test filter and appends `--unit-graph`. Whole-workspace metadata is resolution input; `readCargoCompilationGraphV1` traverses actual roots and refuses unreachable units. The observed dependency is genuine, not accidental workspace-wide compiler selection. Lock refresh's whole-workspace parse was a separate earlier concern.

## Concrete current Source/caller map

Part21's only import of contract-testing is `decode_control` in `🧪️tests/🚦️sqlite-cohort/🦀️.rs`:13existing test functions,18calls. These preserve every owned variant, exact decimal fields, copy cancellation/ownership limits, malformed/duplicate fields, literal order/cumulative limits, list interior turns, depth refusal, headers/documents and original recipient behavior. Source inventory records42test functions in17Rust files across the Part21 tree; this is an inventory, not a claim of42executed/mounted runtime results. Hashes/names/caller counts are retained in `🗑️generated/part21-authority-callers.json`.

The newly requested erased binary law uses genuine `RetainedCloneSource`, the canonical `NativeDecodeControl`, and `OriginalConstructionFixture` directly. It does not call contract-testing. It still compiles alongside all13cohort laws because all library test Source must compile regardless of the execution filter. The present broad graph also unifies the Value `testkit` feature; a clean focused owner must declare that test capability directly rather than rely on incidental OS feature unification.

Contract-testing unconditionally mounts:
- native-owners: canonical decode/encode owner wrappers plus `decode_control`.
- routed-io: real routed IO and SQLite snapshot recipient behavior.
- native-output: full snapshot output/reconstruction and independent SQLite engine assertions.
- registered-job: real ActionBus/OriginalJobFixtureCaller jobs, retained wire and snapshot read custody.
- grants: general five-axis grants plus actual plugin mounted-owner policy.

Its kernel imports are real, not dummy aliases. Native owners use `NativeSnapshotDecodeOwner/NativeSnapshotEncodeOwner`; routed/output modules use full IO/SQLite/ArtifactSqliteSnapshotHost. These integrated laws must remain with their genuine owner and complete tests.

## Existing domain-neutral owner and recommended split

The snapshot direction types already have a canonical lower owner: `semio-framework-io-control` at `🧰️framework/🔨️modules/🚪️io/⏱️control`, with normal dependencies only Value and native-owned. OS store merely reexports them through `semio_framework_os_io::control`. No duplicate authority implementation or relocated production Store methods are necessary merely to issue a receiver fixture.

Recommended coherent boundary:
1. Give the real native receive test control/recipient lifecycle a focused test-support owner beside native-owned (or existing IO-control test support if it owns the required interface). Keep schema-first receipt/close laws there. It should borrow a real callback and install its actual `NativeDecodeRetirementRecipient`, execute the callback through the unchanged control, then close every retained recipient frontier with an explicit original five-axis grant. No fabricated Original, fake provenance, proxy input, copied control, or silent disposal.
2. Migrate all existing `decode_control` callers to the focused owner, preserving all13cohort functions, assertions/fixtures/cancellation/limits. The present helper leaks a boxed recipient to return a long-lived control; do not perpetuate that leak as the new authority design. Prefer a scoped owned driver/callback or the existing detached continuation protocol for tests needing multiple turns.
3. Keep native snapshot/routed/plugin/SQLite integration helpers and their actual kernel-dependent tests under the broad integration owner. Broad owner can depend on focused test-support; focused owner must not depend back on OS kernel. Manually update callers/direct manifest dependencies—no facade compatibility alias.
4. Explicitly declare required native-owned/Value test capabilities on actual test packages. Preserve whole Part21 library test compilation and the ordinary named law; no conditional omission/features to hide the other laws.
5. Freeze a neutral before/after coverage inventory with body/assertion/fixture fingerprints; validate real dependency graph and type ownership with independent TOML/graph oracle; execute the entire affected Part21 package plus focused driver denial/cancellation/recipient-close laws through canonical admission. Preserve separate kernel integration owning gates.

Alternative: decompose the607KB production store-engine into real bounded command/replay/branch owners, respecting inherent-impl/orphan/field custody and existing Store composition boundaries. This is still a real production defect and may be required independently by product scopes. It is substantially broader than Part21's domain-neutral receive fixture need. Source minification, cap increases, dropping devdependencies without migrating the actual laws, or a narrower test target/filter are not solutions.

No production source, manifests, fixtures, guard, or test selection was edited for this audit. No Cargo/service process is active here. Root/frontend website work continues independently.
