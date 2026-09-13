# Architect Exact Window Ownership Implementation

## Outcome

Architect Program now stores the mounted Register selector, Adjacency filter, Graph viewport and Report selection in the exact invoking window's bounded `WindowConfig` owner. The editor app config no longer declares, serializes or mutates those values. Each owner has an independent state and mutation contract, first-party JSON DSL/Pack and text/binary operation codecs, and the framework's bounded window-config lifecycle, preparation factory and disposer.

`RunReport::handle_with_view` remains the semantic command handler that authors a `ReportRecord`. It selects that new record only when the invocation identifies a currently mounted Report window. An invocation without a concrete window, or from a known window of another kind, authors the record without changing any Report window. A stale concrete window id rejects the command. The renderer reads the selected identity from the exact Report config and resolves the full record from `ProgramSnapshot.reports`; the former duplicated `activeReportJson` and report copy in `lastResultJson` are gone.

The interactive-job audit did not classify report generation as migrated. `RunReport::handle_with_view` synchronously calls `build_report(program, kind)` and `report_record_from`; no retained job owner, bounded extent, progress or cancellation route exists for that work. `runReport` is therefore `BatchOnlyPendingRewrite`. The exact targeting semantics are preserved and covered by a direct-handler native law, while the cooperative UI route remains open work.

## Ownership Contract

| Concern | Exact owner | Mutation | Render source |
| --- | --- | --- | --- |
| Register selection | `ArchitectRegisterWindowConfig.active_register` | `SetActiveRegister` | exact Register `ConfigView.window` |
| Adjacency filter | `ArchitectAdjacencyWindowConfig.adjacency_kind_filter` | `SetAdjacencyKindFilter` | exact Adjacency `ConfigView.window` |
| Graph camera | `ArchitectGraphWindowConfig.viewport: Viewport2d` | `SetViewport` | exact Graph `ConfigView.window`, passed directly to `NodeGraphScene` |
| Report selection | `ArchitectReportWindowConfig.selected_report_id: Option<EntityId>` | `SelectReport` | exact Report config plus `ProgramSnapshot.reports` |

The Graph boundary preserves the shared nested `{ viewport: Viewport2d }` renderer action. It does not recreate flattened camera coordinates. `RunReport` never searches for a Report instance by kind or borrows focus from another kind.

A Report window with no selection renders a localized instruction to generate a report in that window. A selection whose authored record was deleted renders a localized missing-record state. Type, creation time and version labels use the existing `LocalizedLabel` resolution path in English and German; report titles and section names remain authored record data.

Ordinary window/config stores retain `HistoryLane::Document`. This work adds no Config history variant. Search and analysis output caches remain app-local write-only fields because their ownership requires a separate design. Dormant presence fields with similar names remain presence state and were not promoted into new owners.

## Interactive Execution Audit

The manifest has twenty declared actions. Only three are `Migrated`: `selectRegister`, `setAdjacencyFilter` and `nodeGraphViewport`. They have an `ArchitectWindowCommandJobFactory`, matching bounded-first-step proof rows, exact `WindowConfig` publication contracts, a 4 KiB raw/output envelope and an extent of one work item. The retained reducer takes `view_state` from the invocation-owned job context and routes to the same exact-address helper used by direct editor handling.

| Actions | Handler work | Retained owner | Disposition |
| --- | --- | --- | --- |
| `selectRegister`, `setAdjacencyFilter`, `nodeGraphViewport` | One bounded scalar/typed-config transition and one exact WindowConfig publication | `ArchitectWindowCommandJobFactory` | `Migrated` |
| `addElement` | Creates one record, but accepts an unbounded name and had no retained proof/factory | None | `BatchOnlyPendingRewrite` |
| `addRegisterItem`, `applyTemplate` | Can clone the document and apply a template-sized mutation set | None | `BatchOnlyPendingRewrite` |
| `removeElement`, `removeRegisterItem`, `setAdjacencyKind` | Scan document registers/adjacencies and may emit document-sized mutation sets | None | `BatchOnlyPendingRewrite` |
| `patchRegisterItem`, `setAdjacencyField`, `nodeGraphEdit` | Parse unbounded JSON and search/expand document edits; graph edit may emit input- and document-sized mutations | None | `BatchOnlyPendingRewrite` |
| `importProgram`, `importRegistersCsv` | Parse unbounded program/CSV payloads and build whole-document load effects | None | `BatchOnlyPendingRewrite` |
| `exportProgram`, `exportRegistersCsv` | Serialize whole-document output | None | `BatchOnlyPendingRewrite` |
| `search`, `runValidation`, `runAnalysis` | Traverse the program; analysis also authors a record and serializes result caches | None | `BatchOnlyPendingRewrite` |
| `runReport` | Builds the full report and authors a `ReportRecord` before exact Report-window selection | None | `BatchOnlyPendingRewrite` |

The command enum also contains `importProgramRequest`, a bounded host file-picker effect. It is not one of the twenty manifest-declared actions and was not assigned a fabricated manifest classification or retained owner in this slice.

## Schema And Fixture Ledger

Each of the following owner roots contains `🦀️.rs` plus strict JSON Schema, TypeScript, GraphQL and Protobuf facets under `🧬️schema`:

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config`

The obsolete `📌️.empty.md` marker was removed from all four populated config roots.

The neutral fixture and independent oracle live at:

- `.../📋️register/🎚️config/🧫️fixtures/🔬️window-ownership/🔣️.json`
- `.../📋️register/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`
- `.../📋️register/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs`

The fixture contains two mounted windows of each of the four kinds. Its left-window mutations and right-window controls prove exact-instance isolation, unchanged document data, serialized reload, selected authored identity, and an explicit deleted Report selection. The TypeScript oracle validates the schemas with Ajv 2020 and independently applies equivalent RFC 6902 patches with `fast-json-patch`.

## Changed Source Ledger

App-config removal is applied across:

- `✏️editor/🎚️config/🦀️.rs`
- `✏️editor/🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`
- `✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema/🔣️.json`
- `✏️editor/🎚️config/🧫️fixtures/🔁️mutation-contracts.json`
- `✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`

Runtime routing and command behavior are updated in:

- `✏️editor/🦀️.rs`
- `✏️editor/🎮️commands/📋️register/🦀️.rs`
- `✏️editor/🎮️commands/↔️adjacency/🦀️.rs`
- `✏️editor/🎮️commands/🕸️graph/🦀️.rs`
- `✏️editor/🎮️commands/🔬️analysis/🦀️.rs`
- `✏️editor/🎮️commands/🏗️element/🦀️.rs`
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`

The Architect Rust package now declares the existing workspace `semio-framework-job` crate directly because implementing the public `ToolJobFactory` trait requires naming its `Operation` parameter. This follows the retained-owner dependency used by sibling plugins and adds no external package.

`ArchitectPlayApp` now explicitly registers its actual bounded document/config store owners and paired disposers, the no-draft/no-transient owners and disposers, and bounded presence-root retirement plus `PresenceStoreOwnedDisposer`. `EditorApp` intentionally forwards those authorities from the domain; it does not install permissive defaults. The initial native close failure therefore identified an Architect registration omission rather than a Store defect. The correction preserves the ordinary document history lane and uses the framework's retained close state machine for every owned lane.

Render consumers and their typed/test facets are updated in the four window roots `📋️register`, `↔️adjacency`, `🕸️graph` and `📓️report`. The inspection and artifact panel sources/tests were updated to remove the old app-wide active-register read. The viewer Register documentation now names the editor's exact Register owner.

## Registered Verification

The root script exposes `verify architect-window-ownership oracle|native`. Root `📋️project.json` registers:

- `workspace:architect-window-ownership-oracle`
- `workspace:architect-window-ownership-native`

Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` register the matching launch entries at orders `311.224` and `311.225`.

The final source-coherence oracle launcher passed with explicit `NX_WORKSPACE_ROOT`, `REPO_ROOT`, `NX_DAEMON=false`, and fresh ticket-local `architect-window-oracle-nx-{cache,data}-20260913-final` directories. Nx reported a 7.9 second run with zero cache hits. Observed output:

```text
architect-window-ownership ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid
NX Successfully ran target architect-window-ownership-oracle for project workspace
```

The same route's strict TypeScript compilation passed for all four config facets and the oracle.

The final registered native launcher ran with explicit `NX_WORKSPACE_ROOT`, `REPO_ROOT`, `NX_DAEMON=false`, fresh ticket-local `architect-native-nx-{cache,data}-20260913-r9` directories, and zero Nx cache hits. It passed all eight selected tests with 2,061 filtered out. Rust reported 1.58 seconds of test execution; Nx reported 13 minutes 59 seconds including shared-dependency lock/compilation time:

```text
[DEBUG] Architect window ownership runtime entered the 2 MiB thread
[DEBUG] Architect window ownership runtime began its heap-pinned future
[DEBUG] Architect window ownership runtime constructed the first registered app
[DEBUG] Architect runtime dispatched three bounded retained window commands, isolated eight exact windows, rendered all four owner kinds, restored every owner pack, preserved document and app-cache bytes, and closed terminal-empty
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 2061 filtered out; finished in 1.58s
NX Successfully ran target architect-window-ownership-native for project workspace
Run duration: 13m 59s
Cache: 0/1 hit (0%)
```

The preceding registered attempt was externally terminated by `SIGTERM` during shared-framework compilation before this selected binary ran. It supplied no Architect compiler or runtime diagnostic and is recorded only as an interrupted gate. The successful fresh rerun above is the acceptance result.

## Native Law Boundaries

The actual registered native app law uses two Register, two Adjacency, two Graph and two Report windows. It dispatches only the three retained commands and verifies exact WindowConfig result lanes, same-kind isolation, shared `Viewport2d`, unchanged document bytes, unchanged app-cache bytes, all four owner-kind renders, all eight config Pack reloads into a fresh registered app, and terminal close of both apps. Its large async state is pinned directly on the heap before the local executor accepts it, so the dedicated thread keeps the normal 2 MiB stack rather than hiding stack pressure with a larger allowance. A separate direct-handler law verifies Report authoring with no window and a different-kind window, stale concrete-id rejection, distinct authored identities, and exact left/right Report selection without presenting synchronous report generation as a migrated interactive job.

The happy-path reopen proves lifecycle serialization and restoration. It does not claim rejection of a foreign inner partition identity; that shared loader defect is separately reproduced and tracked in `window-config-pack-inner-identity-law.md`.

## Remaining Ownership Work

- `search_query`, `search_history_json`, `last_result_json` and `last_analysis_json` remain Architect app-local search/analysis caches. Their write-only result ownership is unresolved and is intentionally not copied into any window.
- Architect presence still declares dormant register/filter/camera fields. They have no mounted producer/render consumer in this slice and require a separate presence-contract decision.
- Seventeen declared Architect actions remain `BatchOnlyPendingRewrite`: `addElement`, `addRegisterItem`, `applyTemplate`, `exportProgram`, `exportRegistersCsv`, `importProgram`, `importRegistersCsv`, `nodeGraphEdit`, `patchRegisterItem`, `removeElement`, `removeRegisterItem`, `runAnalysis`, `runReport`, `runValidation`, `search`, `setAdjacencyField` and `setAdjacencyKind`. Each requires an honest retained owner before it can be classified as migrated. The future `runReport` retained route must preserve the exact invocation/selection law established here.
- The shared loader's foreign inner-id admission remains a framework-owned repair and is outside this implementation.
- Broader editor and NodeGraph consumer gates remain root-coordinated; this report claims only the named registered routes after their recorded results.
