# Current Interface Read-Only Audit

Read-only source review on 2026-10-02. No production source, registrations, tests, Git state, or native/compiler jobs were changed or executed. The execution report was read first. Its recorded GREEN receipts are provenance supplied by that report, not independently rerun audit receipts.

All paths below are relative to `/Users/ueli/Documents/semio`. `N` denotes `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization`; `L` is its parent library directory.

## Actionable Callback Boundary Gap

`N/🚪️source-admission/📁️io/🟦️.ts:59-84` receives caller-owned options. It lexically validates `options.scope` at line 61, invokes progress at line 74, then reads `options.scope` for repository fences at line 77, progress at line 81 and final prepared scope at line 84. A legal caller can retain a mutable object while passing it to this readonly interface; its progress callback can mutate that object after validation. Repository-fence validation is separate from lexical validation. The returned scope is therefore not guaranteed to be the value lexically validated before the callback. This is a source-demonstrated temporal validation gap; no runtime exploit was executed.

`N/🚪️source-admission/📁️io/🟦️.ts:265-267` also reads `options.structuralDirectoryNames` only after prior callbacks. Caller mutation can alter membership after the operation begins. `N/🏃️operation/🟦️.ts:12-13` gives callbacks a fresh but unfrozen progress record. Fresh records prevent cross-event aliasing, but compile-time readonly does not establish runtime immutability.

Recommended closure: snapshot scalar options, detach the structural directory roster and capture the callback before the first callback; use the same validated snapshot throughout preparation and collection. Freeze emitted progress records if immutable callback payloads are contractual. Add a language-neutral callback mutation case before implementation. This gap does not contradict existing cancellation/progress receipts, which may cover different behaviors.

## Canonical Owners and Dependency Evidence

The six physical leaves exist: `N/🛣️path/🟦️.ts`, `N/📁️input/🟦️.ts`, `N/🏃️operation/🟦️.ts`, `N/🔣️taxonomy/🟦️.ts`, `N/🚪️source-admission/🟦️.ts`, and `N/🚪️source-admission/📁️io/🟦️.ts`.

Actual imports establish an acyclic six-owner local graph: path imports only `node:path`; input imports native path/fs/crypto; operation imports nothing; pure admission imports path at line 2; taxonomy imports path/input at lines 6-7; IO imports taxonomy/admission/operation/path/input at lines 7-11. Taxonomy and IO also import actual discovery and canonical JSON implementations. This review establishes the inspected local graph, not the full transitive discovery closure.

Mutation captured-source physically imports `../../🚪️source-admission/📁️io/🟦️.ts` at `N/🧬️mutation/📸️captured-source/🟦️.ts:5`; its binding is not inferred from a package name. Scoped searches of `L/🟦️.ts` and `N/🟦️.ts` found no matching export of loadTaxonomy/canonicalJson/inventoryTaxonomySources and no `loadNormalizationTaxonomy` spelling. This is a scoped negative result, not a repository-wide alias census.

## Cache and Session Boundaries

`N/🔣️taxonomy/🟦️.ts:416-422` defines exactly five content-fact fields. The private capacity-eight map at lines 1216-1218 holds this type. At lines 1220-1237 every load obtains a current physical snapshot and verifies current ancestor identity before cache lookup; only freshly captured contentHash indexes cached facts. The returned facts are detached with structuredClone; path/input/new matcher are added only at the load boundary. No captured path, physical input receipt, or matcher is retained in the inspected content cache.

The IO index cache at `N/🚪️source-admission/📁️io/🟦️.ts:107-145` intentionally contains physical Git receipt state; it is not a pure parser cache. Its lookup obtains a fresh observation first, declines unsupported Git configurations, and verifies an unchanged observation after Git enumeration before storing frozen copies. Cold return rows remain mutable runtime objects while warm rows are frozen; cache contents are detached copies, so mutation of the cold return does not mutate the cached records. This distinction should remain explicit if callback/result immutability becomes a whole public API requirement.

## Cancellation and Deletion Limits

IO checks cancellation between phases and per source observation (`N/🚪️source-admission/📁️io/🟦️.ts:253-286`). The synchronous `execFileSync` at line 135 has neither timeout nor asynchronous cancellation; large native enumeration cannot be interrupted by these checkpoints. JSON serialization and pure projection at lines 290-291 likewise have no internal checkpoint. These are practical boundedness limits, not observed test failures.

Removing the normalization orchestrator while retaining its six nested leaf directories can preserve these direct consumers. Removing the entire normalization subtree cannot: mutation captured-source explicitly binds a nested leaf, and taxonomy/IO still depend on discovery. No whole library/framework deletion closure, export census, runtime portability or physical race proof follows from this review.

## Deferred Wake and Adjacent Registration Inspection

`L/🔣️taxonomy.json:15600-15621` physically registers cooperative-host-pump and pool-use-ownership for fixture/schema/test owner kinds. The deferred-wake registration at lines 15649-15661 includes both `🔔️deferred-wake-ownership` and `🔔️deferred-wake` with those same three owner kinds.

`L/🧪️tests/🧱️rust-source-direction/🔔️deferred-wake-ownership/🟦️.ts:10-11` physically loads the intended sibling fixture/schema paths. Its second law directly reads neutral and writer assets, validates both with AJV and owned schema validation, refuses writer-specific fields in neutral data, and AST-checks exact native package/target/law rosters. Lines 84-85 bind `semio-framework-async`/`semio_framework_async` and `semio-framework-os-kernel-db`/`db` with all-features on the specific writer. These static assertions are authored; this auditor did not execute them or any native law.

The reviewed registrations and gate evidence do not prove full current taxonomy generation, all cooperative/pool-use asset consumers, or native execution. Those remain separate owner receipts and explicit deletion-closure work.
