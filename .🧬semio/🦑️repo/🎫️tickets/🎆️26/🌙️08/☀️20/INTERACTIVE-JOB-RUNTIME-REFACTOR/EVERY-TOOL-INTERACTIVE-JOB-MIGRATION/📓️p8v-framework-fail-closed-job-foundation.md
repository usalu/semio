# P8v Framework Fail-Closed Job Foundation

## Disposition

**FOUNDATION IMPLEMENTED; REPOSITORY MIGRATION INCOMPLETE AND FAIL-CLOSED.**

The framework no longer grants UI execution authority from `bounded_catalog`, no longer mounts one
generic factory over an application's generated command inventory, and no longer substitutes
`typed-command` when an exact key is missing. The permanent static gate intentionally rejects the
current repository. The exact machine-readable rejection ledger is
[`📊️p8v-remaining-command-ledger.json`](./📊️p8v-remaining-command-ledger.json).

No repository-completion or runtime-pass claim is made.

## Changed API

### Action Bus

Every `ToolJobFactory` now implements `execution_contract() -> ToolExecutionContract`. The contract
contains all admission facts instead of one declarative `Migrated` flag:

- exact maximum raw-wire bytes, enforced by `ActionBus::dispatch_wire` before the factory decoder;
- exact maximum decoded items, per-step work units, and output bytes;
- a hard `max_step_micros` strictly below 8,000;
- non-zero checkpoint and progress cadence;
- per-operation cancellation; and
- commit freshness validation immediately before exposure.

`ToolExecutionContract::resumable` and `ToolExecutionContract::bounded_first_step` are the two static
shapes. Registration rejects zero or non-interactive bounds. `register_alias` only accepts an alias
after its exact target factory exists. Unknown keys and unknown aliases do not fall back.

### Manifest

`ActionDefinition::bounded_catalog` and `CommandDefinition::bounded_catalog` now construct an
`Unclassified` catalog row. They do not set `InteractiveJobClassification::Migrated`. Execution
authority must come from an explicit classification together with an exact registered factory or an
exact bounded-first-step proof.

### Plugin Dispatch

The former `AppCommandJob` / `AppCommandJobFactory<A>` path was replaced with the deliberately narrow
`BoundedFirstStepCommandJob` route. Its static proof table contains only the nine independently
audited command keys:

| Cohort | Exact keys | Static maximums |
| --- | --- | --- |
| Draw | `canvasPointerDown` | 16,384 raw bytes; 32 decoded/work units; 16,384 output bytes; 7,500 µs |
| Flow | `duplicateWidget`, `duplicateWidgetStep` | 4,096 raw/output bytes; 64 decoded/work units; 7,500 µs |
| Forms | `setTryValue`, `setTryValueStep` | 4,096 raw/output bytes; 64 decoded/work units; 7,500 µs |
| Remodel | `runReconstruction`, `retryStage`, `runStage`, `advanceReconstruction` | 1,114,112 raw bytes; 1,024 decoded/work units; 4,096 output bytes; 7,500 µs |

The application registry intersects generated schema IDs, explicit `Migrated` declarations, and the
static proof table. It creates one factory registration per exact key. Same-named commands in a
different owner file do not inherit a proof.

Typed dispatch now rejects a missing declaration or missing exact factory before cache/snapshot/job
construction. A bare binary command frame is rejected before deserialization because it carries no
owner-qualified exact key and therefore cannot select a command-specific envelope. One fresh
`JobScope` supplies cancellation per operation. One `WorkerJobSession::step` is admitted per turn;
the old `run_on_worker_async` whole-handler wait is absent. The actual operation, base revision, and
generation are preserved through the factory and `validate_commit` runs immediately before any
ephemeral/result exposure.

Media import and configuration binary dispatch no longer call `A::import_media` or
`ConfigStore::dispatch_binary` directly. They return `interactive-job.missing-factory` until an exact
resumable route exists. Framework history and clipboard definitions are now `Unclassified`, so their
old direct implementations are unreachable at the classification gate. They remain listed as
release-blocking framework migrations below rather than being disguised by a terminal adapter.

### WFC Factory

All six live `ToolJobFactory` implementations in the source tree now have explicit contracts:

1. `AssemblyInferenceJobFactory` — resumable, bounded by its existing assembly cardinality and
   output constants.
2. `MountedCompetingFactory` — explicit bounded test factory.
3. `CompetingFactory` — explicit bounded test factory.
4. `EchoFactory` — explicit bounded action-bus test factory.
5. `NumberFactory` — exact eight-byte wire test factory.
6. `BoundedFirstStepCommandJobFactory<A>` — one exact audited command key and proof per instance.

The machine ledger records their file, line, type, and `explicit` status.

## Static Verifier

`verify interactivity tool-jobs` now:

- enumerates all 775 production `app_commands!` rows and ordinary literal builder registrations;
- masks in-file test modules from the production command census;
- resolves literal and constant-backed explicit classifications;
- reads measurable proof constructors rather than accepting the text `Migrated`;
- enumerates every `ToolJobFactory` implementation and requires `execution_contract`;
- rejects a `bounded_catalog` default that assigns `Migrated`;
- rejects `typed-command` fallback and `run_on_worker_async` in typed dispatch;
- requires `WorkerJobSession` and commit validation before exposure;
- inventories process-global payload-store candidates; and
- emits every unproved row in a machine-readable remaining-command ledger.

Five synthetic verifier self-tests prove rejection of the previous 775-row false positive, a missing
alias/fallback, a non-macro Puzzle-style registration, a data-derived unbounded handler, and a default
`Migrated` builder.

## Exact Remaining Migration Ledger

The current generated ledger contains:

- 50 macro hosts / 50 invocations / 775 rows / 773 unique macro rows;
- 656 literal registration observations after test-module masking;
- 9 exact independently audited admissions;
- 875 unique owner-file command registrations still fail-closed;
- 12 framework-reserved routes still fail-closed pending real resumable factories: `undo`, `redo`,
  `commitCheckpoint`, `createAlternative`, `switchAlternative`, `checkoutCheckpoint`,
  `revertToCommand`, `copy`, `cut`, `paste`, `import-media`, and `configuration-binary`;
- 35 process-global payload-store candidates requiring operation-owned state or a reviewed static
  exemption; and
- 6/6 `ToolJobFactory` implementations carrying explicit execution contracts.

Each of the 875 command rows includes owner file, exact ID, discovery source (`macro` or `literal`),
and rejection reason. This is the compile/migration source of truth for the separate plugin cohorts.
The framework-reserved rows cannot be re-enabled by changing their manifest classification: exact
factories and contracts are required.

## Gates Run

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021` on changed Rust sources | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; 5/5 clean |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny mode clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1; JSON written; 35 global-store candidates, 12 framework routes, and 875 command rows rejected |

## Gates Not Run

Per the disk constraint, no Cargo command, build, Rust test, Wasm compile/execution, cache deletion, git
mutation, or ticket metadata operation was run. Therefore native type/borrow/Send validation and
runtime behavior remain unverified. The mandatory later gates are:

- native framework/plugin compile after each remaining cohort adds factories/contracts;
- Wasm component compile and activation;
- worker turn handoff, cancellation latency, progress/checkpoint resume, and stale-result rejection;
- scale tests at every declared raw/cardinality/work/output maximum; and
- end-to-end framework history, clipboard, media import, and configuration jobs after those explicit
  resumable factories replace the presently fail-closed surfaces.
