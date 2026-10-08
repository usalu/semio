# AEC Building Value Owner Repair

## Actual Compiler Baseline

The cold `release-6Z7Lxu` graph's AEC-building component-dev failure is retained in `🗑️generated/fresh-release-ship-locked.log`: 15 errors include E0603 private kernel imports of `DslValue`, `FromValue` and `ToValue`, downstream missing codec traits and E0433 unresolved `json` calls. The root and `create-building-storey` mutation imported value types through the kernel rather than their canonical `semio_framework_value` owner.

## Assigned Source Repair

The applicable root, `✏️s/AGENTS.md` and `✏️s/🔌️plugins/📐️cad/AGENTS.md` instructions were read. A recursive extension-file inventory found no more-specific AEC AGENTS file. Only the two assigned AEC source files were changed. The root keeps `pack_rt` in the kernel import and imports all value types/derive names directly from `semio_framework_value`; the mutation imports its derive names from that same owner. `Cargo.toml` already declares the direct first-party value dependency, so no duplicate dependency or kernel compatibility reexport was added.

The root's first-party JSON module import now binds `self as json`, the actual name used by its manifest builders. This addresses the same real compiler closure's unresolved-module errors without adding an external JSON runtime library. Scanning every Cad extension Rust source found no further live kernel imports of these private value names; spatial-shape contains only a historical documentation mention, which was not treated as a live defect.

## Schema and Independent Oracle

Existing co-located `create-building-storey` tests consume the retained neutral `🧫️fixtures/🏢️storeys.json` payload vectors and the committed neutral wire witness. The first-party pack JSON decode is compared against independent `serde_json`, and canonical wire dispatch is compared against `serde_json` decode of the same witness. Import ownership changes do not change those schemas or payload semantics. The real failing compile is the red baseline; no speculative replacement fixture or duplicate implementation was introduced.

## Verification Boundary

The build agent's actual AEC component-dev retry finished exit 0 after 28m45s including shared preparation/build queue time, using the current cold shipping generation, private Cargo/Nx directories, cache bypass and already-passed prerequisites excluded. `🗑️generated/aec-repair-component-dev.log` retains the successful Cargo component and metadata pipeline. This verifies the canonical imports and JSON alias through the real component compiler.

After the registered Cad snapshot native target passed its 15 scoped tests, the existing registered `@semio-tech/cad-extension-aec-building-rust:test` target was launched with that same `release-6Z7Lxu` private native target/build store, without pre-task compiler cache reuse or a new generation. Its initial output is retained at `🗑️generated/aec-value-owner-unit.log`. No release provider deployment has occurred.

That actual native unit run failed exit 1 after 17m24s with one test-only E0061: its manifest parser assertion called the current first-party `json::parse` without the required `JsonMemberPolicy`. The AEC root unit test now explicitly uses `json::JsonMemberPolicy::Reject`, matching the existing neutral storey decoder test's rejection policy. The test retains every manifest assertion; no runtime overload, default-policy fallback or compatibility API was introduced. The same registered target was rerun in the same task-created compiler stores, with output retained separately in `🗑️generated/aec-value-owner-unit-green.log`.

The corrected run completed actual exit 1 after 6m11s: Nextest run `6c452109-c6a2-4d51-a819-01571968e867` executed all nine cases, with eight passed, one failed and none skipped. The eight semantic cases, including the neutral storey payload and committed wire witness comparisons against `serde_json`, passed. The sole failure is the existing `component::descriptor_is_fresh` assertion: retained AEC descriptor bytes use app-channel version 20 and prior member ordering, while the current declaration uses version 23 and current ordering. This is a real freshness gate failure, not a runtime semantic failure. The build agent owns canonical AEC `:describe` materialization after the verified shipping component and will notify readiness for the complete nine-case rerun. No descriptor bytes are hand-edited and no assertion is skipped or relaxed.

Canonical `:describe` then completed actual exit 0 in 1m3s, recorded in `🗑️generated/aec-current-describe.log`, producing current component metadata and hashes through the existing materializer. The complete nine-case unit rerun completed actual exit 1 in `🗑️generated/aec-current-descriptor-unit-green.log`, again with eight semantic passes and one descriptor freshness failure. Independent comparison of the two retained byte arrays finds both are 2524 bytes and both declare app-channel version 23. Their first difference is byte 1492: native declaration topic fields preserve `appId,moduleId,label,iconId,computersJson`, while the freshly described component bytes preserve `appId,computersJson,iconId,label,moduleId`. This identifies a remaining materializer/declaration member-order contract defect; repeating descriptor generation alone did not repair it. The parent and build agent were notified to diagnose the canonical producer. The freshness assertion remains intact. The final optimized graph and publication still wait for the separate canonical Store repair and these complete gates.

## Current Nine-Case Retry Compiler Receipt

Session 69865 completed actual Nx EXIT 1 after 14m25s before executing AEC tests. Its only Rust errors were dependency Cad editor media retirement lines 1537/1539, using unqualified DslValue::String. The current canonical authority is semio_framework_value::DslValue, already a direct Cad dependency. In coordination with the editor agent, only these two constructor paths are now explicitly qualified; no kernel alias/import or fallback is added. The unchanged complete nine-case package retry 52806 is active in 🗑️generated/aec-building-cad-value-authority-unit.log with the same task-cold outputs. Exact descriptor freshness is still pending this native result.

## Complete Current Unit Gate

Session 52806 returned actual Nx exit 0. The complete nine-case package unit gate ran nine tests, all nine passed, zero skipped, including exact descriptor freshness after the canonical serialization repair. Duration 26m 27s; retained actual log `🗑️generated/aec-building-cad-value-authority-unit.log`. The two explicit Cad media `DslValue` references compiled with their canonical Value owner.
