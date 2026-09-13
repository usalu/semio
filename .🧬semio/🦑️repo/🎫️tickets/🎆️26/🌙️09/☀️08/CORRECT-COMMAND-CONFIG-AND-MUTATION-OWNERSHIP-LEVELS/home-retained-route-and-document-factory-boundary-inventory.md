# Home Retained Route and Document Factory Boundary Inventory

## Decision

The Home panel host/session owner is accepted independently. The current Home action catalogue is not fully migrated: `BoundedArtifactCommandWork::step` invokes `home_retained_reduce` once, and that reducer synchronously dispatches the complete command handler. The retained envelope owns wire paging, checkpointing, publication, cancellation and close, but those outer phases do not divide handler work. A handler is therefore `Migrated` only when every admitted branch is a fixed, bounded scalar/config transition. Seven handlers still perform whole projection, authored document, catalogue or filesystem work in that one reducer step and must remain `BatchOnlyPendingRewrite` until each has a real staged owner.

The truthful retained factory surface is nine routes, in the declaration order of `HomeCommand`: `openSpace`, `navigateVirtualFileSystemNode`, `goHome`, `createSpace`, `deleteSpace`, `shareSpace`, `manageSpace`, `copyInviteLink`, and `presenceHeartbeat`. The former `setClient` command/config mutation was removed after its value moved to the host-owned `ViewModel.sessionIdentity` projection.

## Retained execution boundary

The current owner is `HomeRetainedCommandJobFactory` in `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`. Its wire owner rejects more than 128 KiB and hands the decoded command to `ArtifactRetainedCommandJob`. The Home `build_tool_job` then installs `BoundedArtifactCommandWork` with `HOME_RETAINED_WORK_ITEMS = 1`. `home_retained_extent` only counts admitted payload bytes and always reports one work item. `home_retained_reduce` calls `HomeCommand::dispatch` for the complete handler. There is no handler-specific cursor, intermediate state, progress unit or cancellation check inside that call.

`HOME_RETAINED_TOOL_IDS`, `HOME_RETAINED_PUBLICATION_CONTRACTS`, the generated bounded-first-step proof, `build_tool_job`, the authored manifest `action_interactive_job` rows and the language-neutral retained fixture describe the same nine-route surface. The seven payload-bearing migrated routes admit at most `HOME_RETAINED_SCALAR_BYTES`, which is the shared 4,096-byte public invocation string limit; the two remaining migrated routes have no scalar payload. `build_tool_job` retains the request's current view context so the reducer can require the live host identity. An action outside that set remains dispatchable through the batch path and is declared `BatchOnlyPendingRewrite`; it cannot be registered under the migrated factory.

## Route classification

| Command | Actual handler work in its worst admitted branch | Existing owner | Truthful classification |
| --- | --- | --- | --- |
| `openSpace` | Formats one bounded route string and emits `Navigate`. | Home retained job, `HostOnly` publication. | `Migrated` |
| `navigateVirtualFileSystemNode` | Removes a fixed prefix, formats one bounded route string and emits `Navigate`. | Home retained job, `HostOnly` publication. | `Migrated` |
| `goHome` | Emits one constant `Navigate`. | Home retained job, `HostOnly` publication. | `Migrated` |
| `createSpace` | Trims bounded scalar arguments and emits one dialog or one shell relay. | Home retained job, `HostOnly` publication. | `Migrated` |
| `deleteSpace` | Reads one Boolean and emits one dialog or one shell relay. | Home retained job, `HostOnly` publication. | `Migrated` |
| `shareSpace` | Trims bounded scalar arguments and emits one dialog or one shell relay. | Home retained job, `HostOnly` publication. | `Migrated` |
| `manageSpace` | Validates one bounded ID and emits one shell relay. | Home retained job, `HostOnly` publication. | `Migrated` |
| `copyInviteLink` | Normalizes bounded scalar arguments and emits one shell relay. | Home retained job, `HostOnly` publication. | `Migrated` |
| `presenceHeartbeat` | Emits the declared empty result. | Home retained job, `HostOnly` publication. | `Migrated` |
| `applyDirectoryEventPage` | Parses an authenticated page, folds up to the page limit into the current directory read model, hashes and serializes the resulting projection, which can be up to the Home config envelope, before returning one replacement mutation and receipt event. | No handler-level cursor. Outer retained work falsely reports one item. | `BatchOnlyPendingRewrite` |
| `createStudio` | Creates a catalogue or folder-backed studio, performs filesystem/catalogue operations, bridges async registration with `resolve_ready`, and authors a Home document mutation. | No handler-level retained owner. | `BatchOnlyPendingRewrite` |
| `bindSpaceFile` | Opens a file backbone, resolves and encodes the whole studio document, writes it, synchronizes the catalogue and removes the draft through synchronous/`resolve_ready` calls. | No handler-level retained owner. | `BatchOnlyPendingRewrite` |
| `importSpace` | Parses imported DSL and writes catalogue state before authoring a Home document mutation. | No handler-level retained owner. | `BatchOnlyPendingRewrite` |
| `deleteVirtualFileSystemNode` | Resolves draft/catalogue ports, removes draft/catalogue data and authors a Home document mutation. | No handler-level retained owner. | `BatchOnlyPendingRewrite` |
| `renameSpace` | The submit branch is a bounded shell relay, but the empty-name branch decodes the entire stored directory projection to find the current label before opening its dialog. Classification covers both branches. | No handler-level retained projection decode cursor. | `BatchOnlyPendingRewrite` |
| `foldDirectoryEvents` | Parses the admitted JSON array, iterates every event, serializes every event and allocates one config mutation per event in a single work step. | No handler-level cursor; reported work extent is always one. | `BatchOnlyPendingRewrite` |

The seven pending routes need separate staged implementations that expose real extent and progress, observe cancellation between bounded units, retain intermediate state, and publish only after completion. The outer Home retained command wrapper is not evidence for those properties.

## Fixture and manifest correction boundary

The fixture, migrated constant, publication contracts and proof list now contain the nine migrated routes in declaration order. The seven pending fixture rows remain present with `BatchOnlyPendingRewrite`, empty publication lanes and a specific blocker naming the missing retained owner. Their manifest rows use `InteractiveJobClassification::BatchOnlyPendingRewrite`.

The corrected `creates_studio_via_home_action` test constructs the Home definition and exact action registry, installs it through the framework's normal app constructor seam, and proves the nine generated migrated actions against the nine factory routes. It does not accept an empty generated set.

## Home document Store lifecycle boundary

The failing `home_document_text_round_trips_through_the_store` creates a bare `ArtifactStore<SHomeSnapshot, SHomeMutation>` and immediately applies `change_catalog_generation`. The Store now requires the exact mutation retirement factory before it can insert an edit into `HistoryLane::Document`. Home currently supplies `build_artifact_store_one_item_preparation_factory`, but it does not implement `build_document_store_owners`. Those are different capabilities: preparation stages one publication; `DocumentStoreOwners` carries the snapshot, initial snapshot and mutation retirement factories plus the actual Store disposer.

The shared `bounded_document_store_owners::<SHomeSnapshot, SHomeMutation>()` is the existing real one-page lifecycle catalogue for an explicitly bounded artifact document, and `bounded_document_store_disposer` is its explicit close adapter. `SHomeSnapshot` contains only schema and catalogue generation, while `SHomeMutation` contains one bounded scalar generation mutation. Home should install that catalogue from `HomeApp::build_document_store_owners` and return the paired disposer from `build_document_store_disposer`. The standalone Store test must install the same exact catalogue before dispatch and drive the Store through terminal close using the paired lifecycle law or existing test support. It must retain ordinary `HistoryLane::Document`; no Config history variant or dummy retirement factory is needed.

## Implementation gates

1. Update the retained fixture first: nine migrated rows and seven explicit `BatchOnlyPendingRewrite` rows, with strict independent schema/oracle validation.
2. Update the source factory IDs, publication contracts, generated proof list and manifest classifications to the same catalogue.
3. Repair `creates_studio_via_home_action` to build the real action registry and validate the generated migrated set; retain the actual create-studio functional assertion through its batch route.
4. Install Home's exact bounded document Store lifecycle catalogue and paired disposer; update the direct Store law to install and close those owners.
5. Rerun the focused Home laws, then the registered `.237` full 90-test target. Home remains incomplete until all five previously observed failures are green in that full target.

Steps 1 through 4 are source-complete. Registered `.236` r8 is green; native and full-suite validation after the session-identity cutover remain pending. The seven `BatchOnlyPendingRewrite` handlers remain required implementation work after the independent milestone review.

## Current independent evidence

The directory/catalogue schema repair is separate and already source-complete: the shared directory schema now defines strict `DirectoryIndexedDocumentViewV1`, `DirectorySpace` and `DirectoryReadModel` records; Home Rust wire records use camel case; and the structural mutation law points to the existing `any/🔮️oracles/🔣️.json` asset. Registered Home oracle `.236` r6b passed through Nx in 3.7 seconds. Native confirmation remains part of the full `.237` acceptance.
