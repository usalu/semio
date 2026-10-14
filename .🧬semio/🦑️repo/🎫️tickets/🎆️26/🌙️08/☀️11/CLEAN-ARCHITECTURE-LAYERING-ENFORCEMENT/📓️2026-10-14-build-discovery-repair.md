# Build Discovery Repair and Verification

## Authored Repair

- Declared TypeScript source contracts now consume a registered, compiler/schema/implementation revision bound language-neutral import image. Current source bytes are still read on every analysis request; generated boundaries, workspace/link containment, external declarations and observed module resolution remain enforced.
- Source contract traversal has an asynchronous entry point with original caller checkpoints every sixteen members and before/after traversal. Project admission awaits computation before serializing the JSON image.
- Native manifest, subtree/owner closures, command-import closures and command dependency images share invocation-scoped caches. Every hit validates consumed facts and replays all original observations into each receiving project; cached facts are not silently omitted from later project receipts.
- Identical nested validations share the original outer inference context. The outer inference publication and complete build graph retain fresh source and before/after membership validation; the explicit unchecked graph reuse branches were not enabled.
- Source-only `@semio-tech/repo-lib:test-build-discovery` is declared in the actual Repo project, dispatched by its permanent package `📜️script.ts`, and registered in both launch source and launch output. Source schema/corpus and bounded async/source/current memo tests are authored; independent oracles include Ajv, TypeScript, esbuild and Bun runtime output.

## Original Caller and Runtime Evidence

The exact command is `bun repo run @semio-tech/repo-lib:test-build-discovery`. The existing configured `createNativeBuildCallerCapabilitiesV1` creates its closed grant before the original process envelope. The original policy is finite 600000 ms. Caller inputs are retained in `build-discovery-caller.json` and `build-discovery-caller2.json`; generated logs are in `🗑️generated/build-discovery`.

The first original attempt was deliberately interrupted after an observation source change and is not acceptance. The second original caller terminated with exit 1 at the unchanged 600000 ms deadline during a failure inventory/membership recheck after completing project 831/932. It never collected the source tests, so **no tests are claimed passing**. A third independent original caller also terminated with exit 1 at its unchanged 600000 ms deadline while waiting in `cache:graph-store`; it never began discovery or collected tests.

The second caller rejected a concurrent CAD `Cargo.toml` edit: the consumed file changed from size 7400/SHA `a1ecac87e13a2e9e877def6eddf1ecfc1c5030099b92a18c9ea2c77cb26b01ef` to size 7422/SHA `c38d65509d970bb541a934c4de7c0032b8a0f4f1e631afb7d61692448ee60cf3`. The log reports `Conflicting inference observations`, unchanged membership, and `build:refresh`. This is observed refusal evidence, not a passing acceptance run.

Current measured completed project records: 856. Latest completed index: 774.

| Project | Authority ms | Total ms |
| --- | ---: | ---: |
| `@semio-tech/energy-model-rs` | 2 | 15071 |
| `@semio-tech/architect-program-rs` | 1 | 13983 |
| `@semio-tech/architect-program-rs` | 1 | 13734 |
| `workspace` | 0 | 12808 |
| `@semio-tech/fem-3d-rs` | 3 | 12401 |
| `@semio-tech/framework-os-dev` | 1 | 8705 |
| `@semio-tech/norm-en1993-rs` | 2 | 7149 |
| `@semio-tech/fem-2d-rs` | 1 | 6258 |

## Remaining Qualification

Complete the registered source suite and the exact Actor and DSL registered original callers under their unchanged resource grants. Complete current member/source validation remains required. No native window, native compiler, or full consumer acceptance is claimed by this repair checkpoint.

## Rust Cache Receipt Coverage Repair

Both Rust parser snapshots and entry-closure hits now use the same validated observed memo wrapper. Actual graph project facts remain separately allocated per project, but this also closes the exposed helper receiving-scope gap if callers supply a shared map. The portable receiving-scope corpus tests the same closure hit and a different-entry closure miss with a shared Rust parser snapshot, native manifest dependency edits, and independent fast-glob/`smol-toml` outputs. These tests are authored and remain unrun because the real source front door has not reached them.

Original inference failures now trace immediately before failure inventory recheck, preserving the concrete cause even if the existing caller deadline expires during revalidation.

## Third Caller Resource Observation

The third actual Repo caller uses `build-discovery-caller3.json` and `frontdoor3.log`. It terminated with `Original Script process deadline exhausted` while waiting in `cache:graph-store`; no discovery or tests started. Read-only queue inspection shows live shared-cohort claims for PIDs 14260, 23688, 25013, 31565 and 35014 on the graph fixed-image lease, with matching SQLite resource identity. This establishes active shared custody and waiting, but does not identify a proven lock bug. No foreign process was stopped, no queue/lease was reset, no shared cache was manually pruned, and no resource deadline/grant was increased.

Read-only inspection additionally identified the concrete writer queue: resource hash `175e70c7547c79a40b239c7d8876d1018469e9a511a8c18e3a5b97f41562fb0b` is held exclusively by PID 23688 (`workspace:release`), with queued exclusive claimants 25013 and the third caller 35014. SQLite read-only identity reads were refused with `database is locked`; `lsof` showed PID 23688's open exclusive lease database. The current graph source declares an exclusive `build-graph-state:<store.root>` writer lease around graph loading. The graph image reports only 1% filesystem use, so no disk-full claim is supported. Waiting preserves the original live writer's custody; it is not a test success or permission to remove its lease.

## Qualification Status

Three registered original attempts are retained. Attempt one was interrupted because source implementation changed; attempt two reached project 831/932 and then exhausted the caller deadline during failure revalidation; attempt three exhausted the caller deadline behind the existing live graph writer. **Source tests, strict TypeScript checks, full graph completion and downstream Actor/DSL/native acceptance remain unqualified.** No actual test run occurred, and no claim otherwise is made.
