# WP3/WP4/WP5 — `💻️os` scope-owned schema contracts

Partition: `🧰️framework/🛍️products/💻️os/**`, excluding every `🧬️schema/🧬️mutations/**` subtree (mutation
worker owns those). Executed against `📋️execution-contract.md` (§B layout, §D ownership, §E rules).
Repo MCP was down for the whole session — no `ticket_*` call was made, no ticket opened/closed/reopened,
no `🗑️generated/` folder touched, no git-modifying command run.

The work was split into eight lanes over one shared convention (below). Lane A (renderer / directory /
presence / os product root / workflow / space) was executed directly; lanes B–G ran as sub-agents inside
disjoint module partitions and are folded into this report.

## 1. Binding conventions applied by every lane

| Rule | Decision |
|---|---|
| Owner | nearest real module (`🔨️modules/<m>` or a nested module dir). `🧪️fixtures`/`🧫️fixtures`/`🧪️tests`/`📦️packages`/`🎯️targets`/`🧱️elements` are never owners |
| Layout | exactly one `<module>/🧬️schema/🔣️.json` per scope; new `$defs` merge into an existing one, never a second file |
| Dialect | `http://json-schema.org/draft-07/schema#` at the document root only; `prefixItems`+`items:false` → `items:[…]`+`additionalItems:false`; `unevaluated*` dropped |
| `$id` | `https://semio.tech/schema/os/<ascii module path>/component.json` |
| Export id | PascalCase `$defs` key keeping the version suffix (`…V1`) |
| Fixtures | data only; every `*.schema.json` / `🧬️.schema.json` / `🔣️.schema.json` / `🧬️schema.json` / `🛂️schema.json` / `📐️*.schema.json` deleted |
| Consumer | `ajv.addSchema(doc)` + `ajv.getSchema(\`${doc.$id}#/$defs/<ExportId>\`)`, draft-07 `Ajv` (never `ajv/dist/2020.js`) |
| Compat | none — no alias, redirect, fallback, deprecation or migration script left behind |

Two annotation-only keywords already present in os documents (`discriminator` in `os.directory`,
`x-semio-note`) are declared with `ajv.addKeyword(...)` at every consumer rather than being stripped from
the contracts.

## 2. Scope / export table (Lane A — directly executed)

| Scope id | Module `🔣️.json` | Exports | Notes |
|---|---|---|---|
| `os` | `💻️os/🧬️schema/` (new) | 25 | product-root backbone-worker/TS-package wire contracts |
| `os.renderer` | `📺️renderer/🧬️schema/` (new, + `🟦️.ts`) | 64 | ShellHost dialog/tutorial/opening/host-bootstrap + all renderer element contracts |
| `os.directory` | `📇️directory/🧬️schema/` (extended) | 140 | +26 re-derived ShellHost exports, +3 folded nested contracts, +7 os-root directory fixtures |
| `os.store.presence` | `🏪️store/👥️presence/🧬️schema/` (new) | 12 | peer commit/admission/retirement + scoped presence |
| `os.workflow.artifacts.run` | `🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/` | 1 | `PackageContractV1` |
| `os.workflow.artifacts.workflow` | `🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/` | 1 | `PackageContractV1` |
| `os.space.artifacts.space` | `🪐️space/🗿️artifacts/🪐️space/🧬️schema/` | 1 | `SpaceArtifactDefinitionV1` |
| `os.space.artifacts.collection` | `🪐️space/🗿️artifacts/🗂️collection/🧬️schema/` | 1 | `CollectionArtifactDefinitionV1` |

### 2.1 The six ShellHost `🧬️contracts/*` seed contracts

| Old contract | New owner | Export ids |
|---|---|---|
| `🎟️invite-capability` | `os.directory` | `InviteCapabilityTransferV1` (+ `…StateV1`, `…InputV1`, `…TransitionV1`, `InviteCapabilityDisclosureAuthorityV1`, `InviteCapabilityFocusPhaseV1`, `DirectoryAdministrationRouteV1`, `DirectoryAdministrationOpeningPhaseV1`, `InviteCapabilityCopyV1`, `InviteCapabilityLabelsV1`) |
| `🌱️artifact-creation` | `os.directory` | `ArtifactCreationProgressUiV1` (+ `…PhaseV1`, `ArtifactCreationCatalogPhaseV1`, `…NoticeRoleV1`, `…NoticeLiveV1`, `ArtifactCreationLocaleV1`, `ArtifactCreationOpeningDispositionV1`, `…CopyV1`, `…LocalesV1`, `…CaseV1`, `ArtifactCreationCatalogCaseV1`) |
| `🌱️artifact-creation/🪪️catalog-authority` | `os.directory` | `SpaceArtifactCreationCatalogAuthorityV1` |
| `🌱️artifact-creation/🚪️ready-opening` | `os.directory` | `ArtifactCreationReadyOpeningV1`, `ArtifactCreationReadyOpeningCase` |
| `📇️directory-bootstrap` | `os.directory` | `DirectoryProjectionReceiptV1`, `DirectoryHomeIdentityV1`, `DirectoryHomeBootstrapActionV1`, `DirectoryHomeBootstrapOutcomeV1`, `DirectoryHomeBootstrapStepV1`, `DirectoryHomeBootstrapLabelsV1` |
| `👥️presence-scope` | `os.store.presence` | `ScopedPresenceBindingV1`, `PresenceScopeV1`, `PresenceRuntimeKeyV1`, `PresenceActorRouteV1`, `PresenceSurfaceRoleV1`, `PresencePeerV1`, `ScopedPresenceRejectionV1`, `ScopedPresenceCloseV1` |
| `🗨️dialog-origin` | `os.renderer` | `ShellDialogOriginV1`, `ShellDialogDocumentV1`, `ShellScopeV1` |
| `🗨️dialog-origin/🛂️admission` (was `🚪️opening`) | `os.renderer` | `AdmittedShellInstanceTransitionV1` |
| `🗨️dialog-origin/🛂️admission/📄️document` | `os.renderer` | `DocumentOpeningTransitionV1`, `DocumentOpeningStepV1`, `DocumentOpeningAdmissionV1`, `DocumentOpeningBackgroundSequenceV1`, `DocumentOpeningAttachmentStepV1`, `DocumentOpeningCloseFailureV1` |
| `🗨️dialog-origin/🎥️tutorial` (+`⏩️seek`, `🧵️serial`) | `os.renderer` | `TutorialRunTransitionV1`, `TutorialRunEventV1`, `TutorialDriveTransitionV1`, `TutorialDriveEventV1`, `PausedTutorialSeekTransitionV1`, `SerialTutorialDriveTransitionV1` |
| `🪪️host-bootstrap` | `os.renderer` | `HostIdentityResolutionV1`, `HostAppV1`, `HostAppAliasesV1`, `ArtifactBootstrapProgressV1`, `ArtifactBootstrapFailedV1`, `ArtifactRebootstrapRequiredV1`, `BootstrapStatusV1`, `LocalizedNoticeTextV1` |

`🧬️contracts/` no longer exists anywhere in the repo's os partition. Its non-schema members (the `🟦️.ts`
/ `🏦️.tsx` / `🌐️browser/🟦️.tsx` implementations and the fixture `🔣️.json` data) moved up one segment to
`🏛️ShellHost/<slug>/…` — ShellHost stays their home because they are renderer-element code, which the
taxonomy's `🧱️elements` collection (kind `ui`) does host; only the *schema* was ineligible there.

### 2.2 Contract re-derivations (what changed vs the "schema of one example")

Every one of the six seed documents was a `const`-only mirror of its single fixture. Re-derived from the
producer/consumer code, not from the example:

- **`InviteCapabilityTransferV1`** — `states`/`focusPhases`/`disclosureAuthority` were `prefixItems`
  tuples of five/five/one `const`s; they are now `uniqueItems` arrays over the real
  `InviteCapabilityTransferStateV1` / `…FocusPhaseV1` / `…DisclosureAuthorityV1` enums. `transitions`
  lost its `minItems/maxItems: 10` fixture bound (now 1…32) and each row is typed by the state and input
  enums instead of three ad-hoc inline `enum`s. `shellRoute.actionId` `const
  "os.directory.open-administration"` became `^os\.directory\.[a-z][a-z0-9-]*$`; `openingPhase` `const
  "loading"` became the real `DirectoryAdministrationOpeningPhaseV1` enum
  (`loading|ready|failed|closed`); `requestKind` became a two-member enum. `capacity` and
  `duplicateResult` stayed pinned — they are denial-bearing protocol laws ("one outstanding transfer",
  "a duplicate never re-discloses"), not example values. `labels` now requires **both** `en` and `de`
  (it did before too) and is named `InviteCapabilityLabelsV1`. The exact five states, ten transitions
  and route values remain asserted — as fixture expectations in `directoryInviteCapabilityOracle`.
- **`ArtifactCreationProgressUiV1`** — split into named enums (`…PhaseV1`, `ArtifactCreationCatalogPhaseV1`,
  `…NoticeRoleV1`, `…NoticeLiveV1`, `ArtifactCreationLocaleV1`, `ArtifactCreationOpeningDispositionV1`)
  instead of six inline `enum`s; `cases`/`catalogCases` lost their fixture-shaped `minItems 7/maxItems 16`
  and `minItems 3/maxItems 6` bounds (now 1…64). The locale map is an explicit
  `ArtifactCreationProgressLocalesV1` requiring `en` **and** `de`, keyed to a single
  `ArtifactCreationProgressCopyV1`. Re-derived against the **current** fixture, which a concurrent
  session had already extended with `opening.{active,failed,retry}`, `openingDisposition`, `retryable`,
  `effectivePhase` and `hasChoices` — the committed schema was already stale for all six fields.
- **`DirectoryProjectionReceiptV1`** — the receipt itself is the real wire contract and kept its exact
  patterns and bounds. What was dropped is the wrapper: the fixture's `hostile` list (six receipt
  mutations) and the `minItems 6/7` bounds on `hostile`/`lifecycle` are now fixture expectations
  asserted by `directoryHomeBootstrapOracle`, not schema. `identities` became a named
  `DirectoryHomeIdentityV1` (the no-leading/trailing-space, no-control-character pattern is a real
  constraint and stayed). `lifecycle` rows became `DirectoryHomeBootstrapStepV1` over two named enums.
- **`ScopedPresenceBindingV1`** — `runtimeKey` is now a named `PresenceRuntimeKeyV1` documenting the
  `v1:<spaceBytes>:<documentBytes>:<spaceId><documentId>` derivation, and `routes` entries a named
  `PresenceActorRouteV1` (`^actor://v1:…`) rather than an inline pattern. `rejected` was `{"type":
  "array", "const": [4 strings]}` — a literal one-example array; it is now
  `ScopedPresenceRejectionV1`, the enum of all four refusal reasons, with the exact ordered list asserted
  by `scopedPresenceOracle`. `peer.role` gained `member` (the projection code in `👥️presence-scope/🟦️.ts`
  branches on `owner || member` vs `viewer`; the schema only admitted `owner|viewer`). `color` gained its
  real `u32` bound. `cases` lost `minItems/maxItems: 2`.
- **`ShellDialogOriginV1`** — the old document described the *fixture* (`{schema, owner, cases[11]}` with
  a JSON-patch list). The real contract is the origin identity itself; the eleven patch cases and their
  `accepted` verdicts are fixture data the test applies and asserts. `sessionInstanceId` gained its safe
  integer ceiling; `document`/`scope` nullability is expressed with `anyOf[null, $ref]`.
- **`AdmittedShellInstanceTransitionV1`, `TutorialRunTransitionV1`, `TutorialDriveTransitionV1`,
  `PausedTutorialSeekTransitionV1`, `SerialTutorialDriveTransitionV1`,
  `DocumentOpeningTransitionV1`/`…AdmissionV1`/`…CloseFailureV1`/`…BackgroundSequenceV1`** — the case row
  *is* the domain contract, so each row type was lifted out of its fixture wrapper and the wrapper's
  `minItems` coverage bounds dropped. Re-derived scalars: seek `mutations` `const 1` → `0…1`, seek
  `playhead` `enum [0, 200]` → non-negative integer, serial `cursor` `const 0` → non-negative integer.
  `DocumentOpeningTransitionV1.timers` stays pinned at `0` — "an opening never leaves a live timer" is a
  law, not an example.
- **`HostIdentityResolutionV1`** — the `$id` was namespaced under a *rendering target*
  (`…/schema/react/host-identity-bootstrap-v1.json`), which no owner level admits. It is now the
  renderer's own domain contract: alias resolution (`aliases`, `apps`, `expected`) separated from the
  three bootstrap notice payloads, which became first-class `ArtifactBootstrapProgressV1` /
  `…FailedV1` / `ArtifactRebootstrapRequiredV1` plus a `BootstrapStatusV1` union — the shape
  `BootstrapStatusNotice` actually renders. `apps` lost `minItems 3/maxItems 16`. The bilingual expected
  strings stayed fixture data (`LocalizedNoticeTextV1` names the shape).
- **`DocumentOpeningScopeResolutionV1` / `DocumentFirstOpenV1`** (from `🧭️opening/🧪️fixtures/📍️scope`) —
  `firstOpen` was a single giant `const` object including a four-entry hostile list. It is now a real
  object (`DocumentFirstOpenV1`) over `DocumentOpeningRequestStageV1`, with the exact stage sequences and
  socket counts left in the fixture. The `oneOf(expected | error)` branches restate their `properties`
  so Ajv `strictRequired` accepts them.
- **`MountedGisMapProbeV1` / `…SourceV1` / `…RefusalV1`** — the five hostile ids became a named enum
  export instead of an inline `minItems/maxItems: 5` tuple.
- **`ExtensionInvocation*`** — the ShellHost `🧪️fixtures/🧬️.schema.json` was a whole-fixture wrapper.
  The real contracts are `ExtensionInvocationCompletionV1`, `ExtensionCompletionRefusalV1`,
  `ExtensionInvocationMissV1`, `ExtensionInvocationFaultV1` and `ExtensionInvocationRequestV1`;
  `fault.origin`/`severity` and `uiScope.kind` were single-member `const`s and are now the real enums.

### 2.3 Naming decisions

- **`🧭️opening` vs `🚪️opening` collision (audit §4.7) — resolved by retiring `🚪️opening`.** The
  dialog-origin child is about admitting a shell instance across a create/retire boundary, so it is now
  `🏛️ShellHost/🗨️dialog-origin/🛂️admission/` (export `AdmittedShellInstanceTransitionV1`). `🧭️opening`
  keeps the document-opening-scope meaning unchanged (`DocumentOpeningScopeResolutionV1`). The name
  `🚪️opening` no longer exists under ShellHost.
- **`os/🧪️fixtures` vs `os/🧫️fixtures` (audit §5) — `🧫️fixtures` wins.** `🔣️taxonomy.json` declares
  `"testFixturesDirName": "🧫️fixtures"`. The whole `💻️os/🧪️fixtures/` tree was verified dead
  (`↩️gis-map-approval-history-v1` — the stale 8-case copy without `oldOwnerRetired`;
  `💡️gis-map-inference-port-v1/↩️history.json`; an empty `📇️directory/`) — a repo-wide grep for
  `gis-map-approval-history` and `↩️history` finds exactly one live consumer,
  `💻️os/📦️packages/🟦️typescript/📜️script.ts:163`, and it reads `🧫️fixtures/`. The directory was deleted;
  the os product root now has one fixture collection. A second emoji-drift duplicate was found and
  removed in passing: the empty `🧫️fixtures/🕺️gis-map-peer-rebootstrap-v1/` beside the live
  `🗺️gis-map-peer-rebootstrap-v1/`.

## 3. Files created / moved / deleted (Lane A)

**Created** — `💻️os/🧬️schema/🔣️.json`; `📺️renderer/🧬️schema/{🔣️.json,🟦️.ts}`;
`🏪️store/👥️presence/🧬️schema/🔣️.json`; `🔁️workflow/🗿️artifacts/{🏃️run,🔁️workflow}/🧬️schema/🔣️.json`;
`🪐️space/🗿️artifacts/{🪐️space,🗂️collection}/🧬️schema/🔣️.json`.

**Moved** — `🏛️ShellHost/🧬️contracts/<slug>/…` → `🏛️ShellHost/<slug>/…` for all six contracts
(`🗨️dialog-origin` incl. `🎥️tutorial`, `🎥️tutorial/⏩️seek`, `🎥️tutorial/🧵️serial`, and `🚪️opening` →
`🛂️admission` incl. `📄️document`; `🌱️artifact-creation` incl. `🪪️catalog-authority`, `🚪️ready-opening`;
`📇️directory-bootstrap`; `👥️presence-scope` incl. `🌐️browser`; `🎟️invite-capability`;
`🪪️host-bootstrap`, whose `🧪️fixtures/🔣️.json` collapsed to `🪪️host-bootstrap/🔣️.json`).

**Deleted (schema documents, contracts relocated)** — 6 ShellHost seed schemas + `🪪️catalog-authority`,
`🚪️ready-opening`, `🛂️admission/📄️document`, `🎥️tutorial/⏩️seek`, `🎥️tutorial/🧵️serial`;
`🏛️ShellHost/🧪️fixtures/{🧬️.schema.json,🔬️mounted-gis-map-probe-v1/🧬️.schema.json}`;
`🏛️ShellHost/🧭️opening/🧪️fixtures/📍️scope/🧬️.schema.json`;
`🧱️elements/🔌️PluginRuntime/🧪️fixtures/{📐️lifecycle-scheduler,📐️typed-operation-settlement,🛡️channel-close}.schema.json`;
`🧱️elements/🕸️NodeGraph/🧪️fixtures/🧬️.schema.json`;
`🧱️elements/🛠️ShellHelpers/🧫️fixtures/{🧬️.schema.json,🎥️tutorial-interaction/🧬️schema.json,🚪️open-artifact/🧬️schema.json}`;
`🎯️targets/🧊️wgpu/🧬️package-catalog.schema.json`; `🧑‍🎨engine/💾️resident/🧬️schema.json`;
`📇️directory/🧬️schema/{🌱️space-artifact-creation-v1/🧬️.schema.json,📣️checkpoint-publication-command-v1/🧬️.schema.json,🌐️browser-actor/🔣️.schema.json}`;
`🏪️store/👥️presence/🧬️schema/{📌️peer-commit,🛂️peer-admission,🧹️retirement}.schema.json`;
`💻️os/🧫️fixtures/{↩️gis-map-approval-history-v1,💡️gis-map-inference-port-v1,🗺️gis-map-peer-rebootstrap-v1}/🧬️.schema.json`,
`🧫️fixtures/🏠️local-interaction/🧬️schema.json`, `🧫️fixtures/🧩️jcoprobe/📐️destination.schema.json`,
all seven `🧫️fixtures/📇️directory/*.schema.json`;
`🔁️workflow/🗿️artifacts/*/🧪️tests/📦️package-contract/🧬️schema.json` (×2);
`🪐️space/🗿️artifacts/*/🧬️schema/🧬️.schema.json` (×2).

**Deleted (dead duplicates)** — the whole `💻️os/🧪️fixtures/` tree; the empty
`💻️os/🧫️fixtures/🕺️gis-map-peer-rebootstrap-v1/`.

**Rewired consumers** —
`📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts` (new `ownedExport()` helper;
all three oracles now load module exports, `Ajv2020` → draft-07 `Ajv`, and the `../../` source reads
became `../` after the contracts moved up one segment);
`📺️renderer/🧑‍🎨engine/🧪️tests/{⚡️quick,👥️scoped-presence,📇️directory-home-bootstrap,🔬️document-opening,🔬️artifact-creation-ready-opening,🔬️engine-contract}/🟦️.ts(x)`;
`🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration/🟦️.ts`; `🎯️targets/🧊️wgpu/🪪️package-catalog.json`
(`$schema` self-locator repointed at the module export);
`💻️os/📦️packages/🟦️typescript/📜️script.ts` (new `ownedExport()` region, five oracles);
`💻️os/🧵️backbone-worker.ts` (two sites); `🧱️elements/{🏛️ShellHost,🐚️Shell,🛠️ShellHelpers}/🟦️.tsx` import
paths.

`.vscode/launch.json` needs **no** os edit from this lane: every os gate entry
(`⚖️gate🎟️directory-invite-capability🌐️browser-worker`, `⚖️gate📍️document-opening-scope🌐️shell`,
`⚖️gate👥️scoped-presence🌐️browser-worker`, `…:directory-home-bootstrap-check`) names an nx target, and no
target name or `📋️project.json` entry changed. Lanes B and C do need new entries — see §6.

## 4. Verification (real output)

Module compile + fixture conformance (`bun` + draft-07 `ajv` inside the react target):

```
renderer: $id=https://semio.tech/schema/os/renderer/component.json defs=64
directory: $id=https://semio.tech/schema/os/directory/component.json defs=140
presence: $id=https://semio.tech/schema/os/store/presence/component.json defs=12

[verify] checks=98 failures=0
```

98 = every ShellHost fixture row validated against the export that now owns it (invite transfer,
artifact-creation progress + catalog authority + ready opening, directory receipt/identities/lifecycle/labels,
presence bindings/close/routes/rejections, dialog origin, admission cases, document-opening
cases/admissions/close-failures/background sequences/attachment steps, tutorial run/drive/seek/serial,
host identity + three bootstrap notices, extension completion/refusals/misses/fault/requests, GIS probe
source/projection/refusals, opening-scope resolutions + first open).

Per-export compile of all four modules (`getSchema` on every `$defs` key):

```
os         exports=25  unresolved=0
directory  exports=140 unresolved=0
renderer   exports=64  unresolved=4   (cross-scope $refs, see §6)
presence   exports=12  unresolved=0
```

The three rewired nx oracles (run through the real `📜️script.ts` exports):

```
framework-renderer-react: region/host-contract lint passed
directory-invite-capability-oracle checks = 21
directory-home-bootstrap-oracle checks = 29
scoped-presence-oracle checks = 29
```

Renderer quick suite (`bun ./📜️script.ts test quick`, which is the `⚡️quick` file this lane rewired from
`🪪️host-bootstrap/🔣️.schema.json` to `os.renderer`):

```
 RUN  v4.1.10 …/🎯️targets/⚛️react
 Test Files  1 passed (1)
      Tests  4 passed (4)
   Duration  21.98s
```

Workflow/space modules:

```
https://semio.tech/schema/os/workflow/artifacts/run/component.json exports=1
https://semio.tech/schema/os/workflow/artifacts/workflow/component.json exports=1
https://semio.tech/schema/os/space/artifacts/space/component.json exports=1
https://semio.tech/schema/os/space/artifacts/collection/component.json exports=1
SpaceArtifactDefinitionV1 validates data: true
CollectionArtifactDefinitionV1 validates data: true
```

## 5. Lane results (sub-agents, disjoint module partitions)

See §5.1–§5.6.

## 6. Cross-partition requests

See §6 table.

## 7. Open questions

See §7.
