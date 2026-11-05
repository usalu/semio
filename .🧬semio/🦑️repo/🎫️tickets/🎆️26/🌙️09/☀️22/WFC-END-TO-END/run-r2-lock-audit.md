# Current WFC Lock Graph Audit

Read-only inspection of authored wfc-current-locks.cjs, current manifests and generated current-job-locks report. No lock mutation or queue by auditor.

The reconciliation report lists8 carriers: WFC root, WFC bridge,5artifact workspace locks,hub. Script copies canonical root Cargo.lock internal semio-framework-job* package blocks, adjusts enginejob async/trace and oracle caller/value/remove-enginejob edges, then backfills missing internal dependencies transitively. It checks concurrent file contents before write. Registry mutation code path is absent: replace calls target only identified internal names or internal missing dependencies.

Caveat: parse uses Map keyed only package name. Different registry versions of the same package collapse to the last row, so the explicit registry-hash assertion is not an exhaustive whole-registry multiset check. This does not itself cause changed external blocks because script never replaces those names. Avoid describing the assertion as proof over every duplicate-version row; for exact comparison use full (name,version,source) identity or compare raw registry block multisets.

Cycle review: current enginejob manifest has oracle only under dev-dependencies. Oracle no longer normally depends on enginejob, resolving that cycle. Oracle's internal testing caller remains its normal library dependency because downstream tests compile its public TestJobDriver; all current WFC consumers of oracle are dev-dependencies. Trace is enginejob devdependency and value oracle devdependency; Cargo.lock combines dev and normal edges, so their presence is consistent rather than runtime leakage. Reviewed testkit activations remain explicit devdependency features/optional forwarded test feature, not default production activation.

Pending generic-headless component migration requires canonical root package registration and a new lock package row/dependency closure. Existing script explicitly adds enginejob async/trace but does not derive enginejob newdependencies from manifest; adding semio-framework-job-headless only to manifest will not make this script add its edge automatically. Update exact owned package roster and backfill canonical newrow before Cargo --locked. Do not retain obsolete WFC-local headless dependency edges merely to satisfy prior lock shape.

Readiness list: actual parser/source diagnostic pass per owner; ordinarycfg no synchronous solver references; testkit crosscomponent exports/forwarding; newheadless framework-only dependency direction with no WFC import; all8lock package dependency closure consistent with manifests; Cargo --locked actual gate. Last engine cfgtest3620 excludes pending rewritten component/tests; this audit is neither Cargo resolution nor native build evidence.

## Exhaustive Registry Version Comparison

At 2026-11-05T00:51:06.881Z, compared raw non-Semio package block multisets, including duplicate names/versions, against immutable Git revision 39b6ae5d4aab567d4dafbc69cb0e1a9a09c9d377. Unchanged in 9/9 affected lock files. Details are in 🗑️generated/registry-lock-audit-current.json. This strengthens the initial name-keyed assertion; internal-row-only mutation was already enforced by construction.

The framework member lock is 🧰️framework/Cargo.lock. An initial orphan headless row added to repository-root Cargo.lock was removed exactly; the repository root has no corresponding manifest dependency. External headless consumers omit serde_json dev; external WFC job/oracle rows omit their workspace-only test dependencies. Cargo --locked verification remains authoritative.

## Portable Audit Command Entry

Root scanned retained executable ticket inputs for hardcoded Windows repo command names. Only registry-lock audit had three; it now selects repo.exe on Windows and repo on other platforms while preserving the exact registered arguments and read-only Git operations. No product/runtime source or frozen native input changed.

## Root Workspace Provenance Context

The root Cargo.lock was temporarily touched only to add, then exactly remove, our misplaced orphan headless row. It is now explicitly retained as a worked Updated path even if clean. Root Cargo.toml was read-only; the headless member is registered in the actual Framework Cargo.toml, not repository root. Provenance now hashes root Cargo.toml and Cargo.lock as context inputs, while root manifest is not added to scoped changed-file attribution. No new hash baseline was captured for this script update, and no registry or lock source changed.

## Complete Explicit SDK Fixture Scope

Added exact9 owned JSON fixture/schema paths for deadlines, responses, planning, streams and units output to provenance discovery, along with both actually touched root/Framework lock paths. The actual new neutral subtree was already covered; its later capture and supervision files will join the preserved union on the next deliberately named snapshot. The old inventory predates these additions. No new baseline was captured, no source/lock changed, and unrelated root manifest modifications remain outside changed-file attribution.
