# Architect Exact Window Ownership Implementation

## Outcome

Architect Program now stores the mounted Register selector, Adjacency filter, Graph viewport and Report selection in the exact invoking window's bounded `WindowConfig` owner. The editor app config no longer declares, serializes or mutates those values. Each owner has an independent state and mutation contract, first-party JSON DSL/Pack and text/binary operation codecs, and the framework's bounded window-config lifecycle, preparation factory and disposer.

`RunReport` remains a user-triggered document mutation that authors a `ReportRecord`. It selects that new record only when the invocation identifies a currently mounted Report window. An invocation without a concrete window, or from a known window of another kind, authors the record without changing any Report window. A stale concrete window id rejects the command. The renderer reads the selected identity from the exact Report config and resolves the full record from `ProgramSnapshot.reports`; the former duplicated `activeReportJson` and report copy in `lastResultJson` are gone.

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

Render consumers and their typed/test facets are updated in the four window roots `📋️register`, `↔️adjacency`, `🕸️graph` and `📓️report`. The inspection and artifact panel sources/tests were updated to remove the old app-wide active-register read. The viewer Register documentation now names the editor's exact Register owner.

## Registered Verification

The root script exposes `verify architect-window-ownership oracle|native`. Root `📋️project.json` registers:

- `workspace:architect-window-ownership-oracle`
- `workspace:architect-window-ownership-native`

Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` register the matching launch entries at orders `311.224` and `311.225`.

The oracle launcher passed with explicit `NX_WORKSPACE_ROOT`, `REPO_ROOT`, `NX_DAEMON=false`, and fresh ticket-local Nx cache/workspace-data directories. Observed output:

```text
architect-window-ownership ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid
NX Successfully ran target architect-window-ownership-oracle for project workspace
```

The same route's strict TypeScript compilation passed for all four config facets and the oracle.

The registered native launcher is currently compiling in the coordinated Cargo lane. Its selected filter includes six tests: four localized Report render laws, the schema/codec/fixture law, and the 2 MiB-stack eight-window app law. Final native evidence will replace this paragraph when execution completes.

## Native Law Boundaries

The native app law uses two Register, two Adjacency, two Graph and two Report windows. It verifies exact WindowConfig result lanes, same-kind isolation, shared `Viewport2d`, unchanged document bytes for view-only commands, unchanged app-cache bytes, authored report counts, no-window and different-kind Report behavior, stale-id rejection, exact left/right Report selection, selected-record rendering, all eight config Pack reloads into a fresh registered app, and terminal close of both apps.

The happy-path reopen proves lifecycle serialization and restoration. It does not claim rejection of a foreign inner partition identity; that shared loader defect is separately reproduced and tracked in `window-config-pack-inner-identity-law.md`.

## Remaining Ownership Work

- `search_query`, `search_history_json`, `last_result_json` and `last_analysis_json` remain Architect app-local search/analysis caches. Their write-only result ownership is unresolved and is intentionally not copied into any window.
- Architect presence still declares dormant register/filter/camera fields. They have no mounted producer/render consumer in this slice and require a separate presence-contract decision.
- The shared loader's foreign inner-id admission remains a framework-owned repair and is outside this implementation.
- Broader editor and NodeGraph consumer gates remain root-coordinated; this report claims only the named registered routes after their recorded results.
