# Framework Bootstrap Repair — 2026-10-28

## Observed Failure

The retained frontend-typecheck.log establishes that framework-rs:generate entered framework/📦️packages/🦀️rust/📜️script.ts:46. Its direct compilation graph capture refreshed the framework lockfile (fetching the missing encoding_rs registry entry), then cargo metadata --format-version=1 --locked --offline failed because adler2 v2.0.1 had not been downloaded. Refreshing a lockfile does not provision every archive named by its complete metadata resolve.

No manual Cargo fetch was used for this repair. The scheduler helper's earlier fix did not cover this separate generic compiler-store consumer.

## Change

The neutral CompilerStoreViewV1 now exposes the existing CargoCompilerInputsPortV1. The repository's actual provider binds that port to repositoryCargoCompilerInputsV1 using the privately issued original invocation, its repository root and network policy, the actual retained compiler storage, and each final command's explicit consumption intent. The framework owner therefore does not import repository-specific provisioning code.

captureCargo now enters withCargoCompilerInputsV1 before metadata/source planning. The same admitted arguments, environment and control flow through planning and source admission. Registry/configuration custody spans the final compiler and generated output consumption. The compiler enters the writer custody offered by that exact input view. Only after acquisition are --locked and --offline applied to the final compiler arguments, preserving any libtest argument boundary; an online original can fetch its missing archive, while an offline original remains offline and refuses cold acquisition.

The type-generation output is still copied only after successful compilation. Failed compilation retires its input scope and temporary capture output.

## Test-First Evidence

A schema-validated portable fixture models a current lockfile and missing archive, warm offline execution, cold offline refusal and compilation failure. Its actual framework function extraction failed before the implementation edit at “missing archive before offline metadata.” It now passes. A second execution test extracts the actual repository provider binding and verifies the original network/root, final request, storage and control identity.

The combined focused suites pass **10 tests, 634 assertions**:

- existing repository dependency consumption and scheduler bootstrap laws;
- new framework cold bootstrap and actual provider binding;
- existing neutral compiler consumer continuation/custody laws.

Ajv independently validates the fixture schema; SQLite independently checks acquisition precedes metadata and compilation precedes retirement. The neutral compiler consumer harness was aligned with the already-current captureCargoCompilationPlanV1 API; it had still mocked the previous graph-only API and initially failed with ReferenceError. This was a harness mismatch, not a regression introduced in the compiler implementation.

The three edited implementation files have no TypeScript syntax diagnostics. Full semantic compilation and real cold-cache bootstrap remain subject to the live owned check below.

## Live Owned Check

Tool session **48809** runs:

```text
SEMIO_LANGUAGE=en repo run @semio-tech/framework-rs:generate --configuration=release
```

It was admitted after 415 seconds and completed cached input-closure verification. It currently waits for the occupied graph computation slot in generation 0, with 29 stale projects open; no success or renewed native failure is claimed. No standalone proctor or full end-to-end invocation was duplicated. The coordinator's existing full course gate remains independent. This agent must poll 48809 while it owns the handle, because tool process handles are namespace-local.

## File Ledger

Updated:

- 🧰️framework/📦️packages/🦀️rust/📜️script.ts
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🔌️provider/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔌️provider/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts

Created alongside the existing dependency-consumption corpus:

- framework.json
- framework.schema.json
- framework.ts

These three files are under 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption.

Created this ticket report. No AGENTS file, native C compiler implementation, git state or remote deployment was changed.

The owned check was admitted after 415 seconds, completed cached closure verification, and now waits for the existing graph computation slot with 29 stale projects open. It has not reached framework compiler execution yet.

## Sibling Prerequisite Audit

Every declared course typecheck prerequisite was traced independently; prior green output was not treated as proof of input acquisition.

| Prerequisite | Actual route before offline metadata |
|---|---|
| framework-rs:generate | The unique direct generic compiler-store capture, repaired above. |
| framework-actor-rs:typegen | exportActorTypegen in actor/🧬️typegen/🏃️execution/🟦️.ts calls runAdmittedCargoCommandV1 with its exact manifest, typegen target and output environment. |
| ui-contract-rs:generate | runTypegenExportTest in ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts calls runAdmittedCargoCommandV1 with its exact manifest, typegen package and test target. |
| ui-rs:generate | GenerateAxesScript in ui/🎚️axes/📜️script.ts reads authored axes and publishes projections in TypeScript; it invokes no Cargo metadata/compiler. |
| assets:build | BuildScript calls publishAssetArtifacts to write deterministic catalog/metabolism/icon outputs; it invokes no Cargo metadata/compiler. |
| ui-styling-tokens:generate | GenerateScript calls generateStylingArtifacts with writeGeneratedFileIfChanged; it invokes no Cargo metadata/compiler. |

The actor and UI-contract native routes both pass through the neutral admission provider's runCommand mapping to runRepositoryCargoCommandV1 (library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts), which invokes launchRepositoryCargoV1. In the canonical launcher (library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts), withRepositoryCargoInputsV1 encloses the source graph capture (currently lines 203 and 206 respectively). The scheduler preparation before this scope uses the separately repaired prepareRepositoryCargoSchedulerV1. Thus both sibling exporters acquire their own complete dependency resolve before offline metadata and retain registry custody; they do not rely on the framework exporter warming shared archives. No sibling code edit was needed.

## Actual Graph Activity Observation

The sole visible graph-farm worker is PID 212277, whose command role is --semio-role=graph-farm-worker; parent PID 194452 belongs to the concurrent framework-os-mcp-rs build. Its running claim 8c1fd562f0381a077e523bb0 covers 128 owners in generation 0. The previous claim 26a56deafc5eac5549ea170b completed 128 owners.

Two process samples 37.4 seconds apart showed worker CPU ticks advancing from 92,475 to 96,198 (approximately 37.23 seconds of CPU), state R and about 1.4 GiB RSS. This proves active computation rather than a dead owner. No new per-owner result artifact appeared during that sample, so no claim of a specific inferred owner's completion is made. The focused generate command still waits on that one graph compute slot with 29 stale projects open. No process was restarted or cancelled.

The focused run’s new polling output is retained temporarily at `🗑️generated/framework-generate.log`; the original missing-archive repair has not reached real compiler execution while the peer graph slot remains occupied. At the latest poll that graph wait exceeded 1,270 seconds.

A later graph-worker sample still found PID 212277 in state R with the same graph-farm role and about 1,459 MiB RSS. CPU increased from 96,198 to 182,520 ticks over 867.6 seconds (approximately 863.2 CPU seconds). The same 128-owner claim remained running without endedMs; this confirms sustained computation but still no completed claim. The owned generate check remained a waiter.
