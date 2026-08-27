# Shared Reserved Routes And Store Publication Audit

Date: 2026-08-27  
Scope: read-only source audit of the shared `copy`, `cut`, `paste`, and `import-media` routes and the official static interactivity verifier. No product source, fixture, build, formatter, Nx, or Cargo command was run.

## Verdict

The smallest honest architecture is a two-owner route, not a generic framework implementation:

1. the framework owns exact wire admission, bounded mounted-job scheduling, cancellation, fresh commit gating, fixed host result/ACK pages, and fail-closed dispatch; and
2. the concrete `ArtifactEditor` owns each route's semantic producer plus every non-host publication lane. It must provide a route-specific `ArtifactOwnedToolJobFactory`, an `ArtifactReservedToolJob`, a precise publication contract, and a real Store preparation factory for each declared durable lane.

`HostOnly` is the sole route category a framework-owned factory may certify. A route that can return an `Emit`, apply Presence/Transient, or change Artifact/Config/Draft must be `AppOwned`; a framework fallback may not convert it into a host-only success. This permits one shared controller and one shared Store protocol without inventing 35 generic import implementations.

## Current Evidence

### Shared Controller

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
  - `ArtifactApp::register_tool_job_factories`, `build_reserved_tool_job`, and the five one-item preparation hooks provide the right app seam (around lines 11780–11840).
  - `run_framework_reserved_job` owns exact qualified-proof admission, mounted worker stepping, checkpoint validation, output equality, cancellation, terminal close, and commit permit (around 18206).
  - `build_artifact_reserved_action_job` and `build_artifact_reserved_media_job` construct the app-owned request; the latter already fails closed if no app builder exists (around 20620–20675).
  - `dispatch_framework_reserved_action` still falls back to `FrameworkCopyJob`, `FrameworkCutJob`, and `FrameworkPasteJob` if the app returns `None` (around 20953–20977). Its post-job branches then call `commit_framework_clipboard_completion` or direct Store routes. This is the non-resumable semantic escape.
  - `dispatch_import_media` runs the app producer but directly applies Presence/Transient and calls `dispatch_emit` after the reserved job (around 21826); these publication/commit turns are outside `MountedTypedCommandFullOperation`.
  - `framework_reserved_job!` defines `FrameworkImportMediaJobFactory`, but registration deliberately excludes it. The macro's generic cursor only pages `raw` and returns it as output; it cannot prove route semantics or Store publication (around 14690–14878 and 15055–15057).

### Store Authority Exists

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` contains a real split:

- `ArtifactStoreOneItemPreparationFactory::preflight/begin` transfers exact base/mutation owners only after a bounded footprint is accepted (12462–12478).
- `ArtifactStoreOneItemPublication` retains preparation, Store-minted live authority, receipt, retry/ACK, close, and terminal-empty witness (12521–12605).
- `ArtifactStore::begin_apply_one`, `advance_apply_one`, and `cancel_apply_one` advance preparation/cursor/preflight/publication in distinct one-item turns; generic replay/apply/outbound calls remain outside that protocol (14315–14545).
- The shared mounted publisher has the `PendingArtifactStorePublication` union and advances one named lane with a one-item grant (15999–16110 and 21290–21445).

Therefore no new generic Store reducer is needed. The missing link is to keep reserved-route completion under the same mounted publication owner and to require the exact domain preparation factory before a durable lane starts.

### Concrete Route Census

The canonical importer fixture is `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yj-importer-cohorts.json`:

| Route family | Logical route IDs | Owners | Current fixture status |
|---|---:|---:|---|
| Framework route contracts | `copy`, `cut`, `paste`, `import-media` | 4 | `copy/cut/paste` have framework fallbacks; import has no framework registration. None is a valid generic semantic producer. |
| App import-media | `import-media` | 36 | 35 monolithic/pending; 1 declared migrated (`Puzzle5dPlayApp`). Cohorts A/B/C are 12/12/12; C has the single migrated row. |
| Puzzle5d reserved cohort | all four IDs | 1 app | current source's `build_reserved_tool_job` accepts only `copy`; it cannot satisfy the verifier's four-route factory/state-machine proof. |

The latest retained official report, `📊️coordinator-official-tool-jobs-publication-lanes-r3-2026-08-27.json`, records 4 framework-reserved pending routes, 35 pending app importers, `durableOneItemPublicationBounded=true`, `ephemeralOneItemPublicationBounded=true`, and `fullToolOperationJobBounded=false`. This is historical static evidence, not a fresh verifier execution in this audit.

## Required Route Contract

Extend the existing app-owned registration record with explicit provenance and route output rather than inferring either from string patterns:

```text
RouteOwner = FrameworkHostOnly | AppOwned
RoutePublication = HostOnly | { lanes: non-empty subset of Artifact, Config, Draft, Presence, Transient, Child }
```

Admission must require:

- `FrameworkHostOnly`: registered shared factory, exact schema/key/contract, a retained route job, and a host result only. Its completion cannot carry `Emit` or ephemeral output.
- `AppOwned`: exact owner type + controller + tool id + schema registration, an app factory/builder pair, and a route job whose commit stays in the mounted operation.
- Durable `Artifact`/`Config`/`Draft`: the corresponding cached `build_*_store_one_item_preparation_factory()` is present; `begin_apply_one` owns the mutation/base and publishes only after its retained preparation reaches `Prepared`.
- Ephemeral `Presence`/`Transient`: the corresponding preparation and local-root retirement factories are present; ACK and close remain attached to the same mounted operation.
- No factory or missing lane authority: return the exact original request/owner to bounded close and fail before any semantic callback, `serde_json` whole-envelope build, `.apply`, `dispatch_emit`, or direct Store `dispatch`.

The framework should provide a shared *adapter* that moves `ArtifactReservedToolJob` completion into `MountedTypedCommandFullOperation` publication. It must not provide shared semantic `copy/cut/paste/import` jobs. An app can use a shared reusable implementation only by returning it through its own factory with an app-specific operation/preparation type and declared lanes.

## Verifier Changes

`📜️script.ts` currently has the right pieces but two separate recognizers:

- `toolJobFrameworkReservedRoutesExact` (1444) recognizes macro registrations, controller routing, and binary fail closure. It must reject framework ownership for the four routes unless the contract is explicitly `FrameworkHostOnly`; remove `FrameworkImportMediaJobFactory` from its accepted macro inventory and reject the copy/cut/paste fallback branch.
- `toolJobPuzzleReservedRoutesExact` (3377) is an app-owned special case. Generalize its owner/factory/route-job invariants into a `toolJobReservedAppRouteExact` helper and make Puzzle5d one fixture of that helper, not a second ownership model.
- `toolJobReservedImporterOwners` (3727) is the exact 36-owner import census but only checks token presence. It must join each owner to a registration/provenance record, a route-specific factory, declared lanes, and the appropriate preparation factories.
- `toolJobAppOwnedRows` / `toolJobPublicationAuthorityReady` (1204/1160) already parse `ArtifactOwnedToolJobFactory` publication contracts and the five preparation hooks. Reuse them for reserved routes and require a `RouteOwner::AppOwned` registration. A shared factory is accepted only from the framework registration table with `RouteOwner::FrameworkHostOnly`; app-owned routes are matched by `ownerFile + ownerType + controller + factory + toolId + schema`.
- `toolJobStoreOneItemPublicationBounded` (1644) already checks Store mechanics. Add an integration predicate for the reserved adapter: completion lane → `PendingArtifactStorePublication` → one-item begin/advance → result-page ACK → `close_step`/`terminal_is_empty`, with freshness before every pending/begin turn.

The official report should expose separate arrays: `acceptedFrameworkHostOnlyRoutes`, `acceptedAppOwnedReservedRoutes`, `pendingReservedRoutes`, and `pendingReservedImporterOwners`. It must never append a shared route to `acceptedReservedRoutes` merely because an app descriptor names the same tool id.

## Fixture And Self-Test Delta

Keep the existing language-neutral Store fixture/schema pair and add route-ownership cases to the same ticket fixture family:

1. framework host-only shared factory is admitted and cannot emit Store lanes;
2. app-owned copy/cut/paste/import with Artifact output reaches app preparation, one-item publication, result ACK, and terminal close;
3. absence of the app route factory, an undeclared lane, or a missing preparation factory rejects before semantic work and preserves the exact request owner;
4. a generic framework fallback for cut/paste/import is rejected;
5. stale revision/generation before preparation and between publication turns closes the retained owner without publishing;
6. cap and cap+1 raw wire / decoded item / work / result page cases preserve the original owner;
7. cancellation during producer, preparation, awaiting ACK, and close produces terminal-empty evidence;
8. provenance spoofing (shared factory presented as app-owned, or copied app type/controller/schema) is rejected.

The verifier's hostile mutation suite must mutate real production anchors, not only synthetic macro text: remove the app registration, change lane to `HostOnly`, restore the framework fallback, remove `begin_apply_one`, move `dispatch_emit` outside the mounted owner, omit freshness, and omit terminal-empty ACK/close. Each mutation must turn the exact route red.

## Staged Implementation

1. Define the explicit reserved-route provenance/output record in the framework plugin module and thread it into `QualifiedToolProof`/activation. Mark all four routes fail-closed until registered; delete the generic copy/cut/paste fallback and the unregistered import factory claim.
2. Extract the shared reserved-completion-to-mounted-publication adapter. Make it own publication, host ACK, cancellation, freshness, and close, then replace direct reserved-route `presence_store.apply`, `transient_store.apply`, `dispatch_emit`, and direct Store commit calls with that adapter.
3. Generalize the Puzzle5d route proof into the app-owned reserved-route contract and finish its four concrete route jobs as the reference implementation. Each job declares its actual lanes and uses app preparation for any durable mutation.
4. Migrate the remaining 35 importer owners cohort by cohort. Each owner adds only its own registration, job, publication contract, and preparation/retirement factories; unsupported import semantics remain fail-closed.
5. Update the JSON inventory statuses only when the source-level provenance/factory/lane/preparation join passes. Update the verifier predicates and hostile fixtures first, then run the authorized language-agnostic and third-party-oracle checks in a non-concurrent validation lane.

No compilation or runtime correctness claim is made here.
