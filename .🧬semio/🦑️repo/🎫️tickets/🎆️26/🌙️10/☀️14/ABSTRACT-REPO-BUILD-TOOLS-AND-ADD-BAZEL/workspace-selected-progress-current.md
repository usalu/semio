# Current Workspace Selected-Admission Progress

Read-only interpretation of actual public workspace32266. No producer, SDK reader, profile attachment, test process or permanent Source change was started by this audit. Root owns the active ordinary-policy command; no terminal result is inferred here.

## Measured Log Prefix

The bounded snapshot covers the first21,340,825 bytes /219,196 lines of `public-workspace-build-canonical-ui.log`, starting at byte0. Its SHA256 is `a2d2aa8c297228b97cc58598acac563f3011b84e770b0a32f6bded44a91babf8`. Python command counters and independent system SQLite grouping agree exactly on197,071 relevant progress events. Generated receipt: `workspace32266-selected-progress-counts.json`. The live log may have advanced after this captured prefix.

| Command | Events |
| --- | ---: |
| selected-declaration | 4464 |
| selected-catalog-targets | 4464 |
| selected-command-sources | 4464 |
| selected-native-closure | 4464 |
| selected-compile-closure | 4463 |
| selected-normalization | 90 |
| selected-native-preparation | 90 |
| selected-source-closure | 90 |
| selected-source-inputs | 140688 |
| selected-compile-imports | 33793 |
| build:refresh | 1 |

These are events, not unique owners, files, targets or configurations. No explicit `[DEBUG] selected-owner-stage` owner identities are present in this prefix because the verbose authority trace is off. No `build:settle` or Bun-child start progress was present in the relevant counter query. One `build:refresh` proves an observable loader attempt retry; it does not identify its precise invalidating fact or prove that a completed graph was published before that retry.

## Why Stages Repeat

`closeBuildOwnerAdmissionV1` (admission43–72) starts with `workspace:test-workspace-build`. It maintains admitted owners, active targets, requested named-input groups, base facts and an exact sorted target/input revision per owner. A newly encountered owner receives a base `inspect(owner,[],[])`. If it has active targets or input groups, it receives a separate inspection for that revision. New source owners, package/native dependencies, target prerequisites and catalog providers expand the frontier. Within one closure computation, unchanged revisions skip another inspection. Newly admitted owners and changed target/input revisions legitimately produce more stage groups; they are not automatically retries or a repeated configuration walk.

Every inspection invokes `discoverBuildOwnerAdmissionFactsV1`, which emits declaration, catalog-targets, command-sources, native-closure and compile-closure stages. Inspections with selected targets or named inputs additionally emit normalization, native-preparation and source-closure stages. Configuration is resolved from the original selection/defaults; a stage event does not carry a configuration identifier. There is no evidence here that the loop is enumerating every configuration.

`yieldInferenceV1` (inference1601) checks Original on every invocation and publishes every16 loop operations (or on a forced checkpoint). Source-input and compile-import events arise from directory/file/pattern/import work inside an inspection, not one event per owner. Multiple publishes may also occur while checking candidate imports already visited by the closure algorithm. Therefore the140,688 and33,793 counters cannot be converted into exact unique-file or owner counts.

Each selected `inventory()` (graph273) creates a new closure inside `settleObservedComputationV1`, with fresh per-computation frontier/fact maps and native caches. A normal fresh graph attempt has an initial inventory and a final inventory comparison (graph274/296), so a complete stage sequence may repeat even without a failed candidate. A source-observation settlement conflict can also replay that inventory. The selected inventory does not supply an `onRound` callback, so its inner settlement rounds cannot be counted from `build:settle` absence. The current log lacks enough owner/round identity to assign all repeated groups to these causes.

## Existing Finite Completeness Guards

- Admission closure permits `pass=0` through `registry.owners.length+65536`, then rejects `Finite canonical owner closure exceeded`. A separate read of the current registry finds2484 owners; this is not evidence that the active command began with precisely that version or admitted all2484.
- Observation settlement defaults to at most64 rounds, retains cancellation, validates consistency/current facts and rejects a persistent real failure or conflict.
- The production graph loader permits attempts0 through8, then rejects changing Source authority. Each unsuccessful attempt emits `build:refresh`.
- The active public CLI uses the existing ordinary Original0. It has no ticket20-minute aggregate timer. The actual workspace Bun300000ms child timer begins when that child is launched, after admission/bootstrap.
- The prepared ticket ordinary ALL mode retains its additional caller guard at two matching refresh events. That separate qualifier is not running inside public workspace32266.

There is no observed basis here for declaring an infinite loop, predicting completion time or removing a completeness/currentness check. The progress represents actual checkpoints, while exact unique-owner progress and the reason for the single retry remain unmeasured. No performance savings are extrapolated from counts.

## Inspected Current Source Hashes

| Source under library | SHA256 |
| --- | --- |
| build/graph/admission TypeScript | `7f6f9cfb535f5aa30599a69bb84d01f4150aec409042141562816e42576275cc` |
| build/graph/inference MJS | `d3f0b1eae0e5d3fba90fe974f99c2ca3c2dc8a6352deb2e0299e76a8e84736cb` |
| build/graph TypeScript | `41ddc48e0bc2b369d64eb370a3e8ad0db45a690e5d0291bd2576f180ba79c12b` |
| build/observation MJS | `425fcec4e001631bf977ca6e30c992c2c5ee02441b645fb6282f7211e9db884a` |

These record inspected current bytes, not independently authenticated loaded bytes of the running process. Root retains actual execution/Source authority.
