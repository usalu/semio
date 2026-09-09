# Flow Fixture Schema Topology Recovery Audit

Captured: 2026-09-09T15:54:47+02:00

This is a read-only source and staged-diff audit. No source was edited and no Bun, Nx, Cargo, formatter, or native law command was run.

## First cause

Both Recovery 9 routes fail while AJV compiles their neutral fixture schema references:

- `🗑️generated/flow-recovery-9-child-edit-check.txt` reports missing `FlowChildAddWidget`.
- `🗑️generated/flow-recovery-9-add-widget-retained-check.txt` reports missing `FlowAddWidgetRetained`.

The production artifact schema at `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` is correctly registered under `https://semio.tech/schema/s/flow/flow/artifact.json`. Its staged change deliberately reduces the artifact document to persisted `schema` and `content` fields (24 lines, from 3,095). It contains no `$defs`. This is a definition-ownership loss, not an AJV registration failure.

The prior `HEAD` artifact schema contained all fixture contracts as artifact-root `$defs`. They are absent from the current tree: the fixture files and source-contract test still refer to them, but no relocated schema module currently defines them.

## Correct owner and registration

Do not put the neutral fixture contracts back into the public persisted-artifact schema. The coherent scoped owner is a new editor fixture-contract schema module under:

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧬️schema/🔣️.json`

It should have its own fixture-contract `$id`, distinct from `…/artifact.json`. The source-contract test and package router should register that module and compile its exports through that module's `$id`.

The existing strict AJV setup must remain on both paths:

- `x-semio-formats`
- `x-semio-state`, `x-semio-child-kind`, `x-semio-child-standard`, `x-semio-child-subset`
- formats `double`, `float`, `int32`, `int64`, `uint32`, `uint64`

The selected fixture-definition closure has no non-local `$ref`; the currently registered IO and child schemas may remain registered but are not a semantic dependency of this fixture-only contract module.

## Exact recovery set

The source-contract file calls 13 exports. Their combined historical dependency closure is 35 definitions:

`FlowChildAddWidget`, `FlowChildAddWidgetChild`, `FlowChildAddWidgetDialect`, `FlowChildAddWidgetEdge`, `FlowChildAddWidgetNode`, `FlowChildAddWidgetParam`, `FlowChildAddWidgetPoint`, `FlowChildAddWidgetSnapshot`, `FlowTreeProjection`, `FlowTreeProjectionNode`, `FlowHostWire`, `FlowArtifactRecipes`, `FlowArtifactCanonical`, `FlowArtifactCanonicalChrome`, `FlowArtifactCanonicalDictionary`, `FlowArtifactCanonicalEntry`, `FlowArtifactCanonicalGui`, `FlowArtifactCanonicalLayout`, `FlowArtifactCanonicalMutation`, `FlowArtifactCanonicalNeuron`, `FlowArtifactCanonicalOptionalLayout`, `FlowArtifactCanonicalPreview`, `FlowArtifactCanonicalSynapse`, `FlowArtifactCanonicalTree`, `FlowArtifactCanonicalValue`, `FlowArtifactCanonicalWidget`, `FlowGrantFrontier`, `FlowSliderLabels`, `FlowDeleteCascade`, `FlowContentIdentity`, `FlowStoreOwners`, `FlowPresenceOwners`, `FlowPresenceOwnersText`, `FlowTransientOwners`, `FlowViewerOwners`.

The retained add-widget oracle requires six additional definitions:

`FlowAddWidgetRetained`, `FlowAddWidgetRetainedCommand`, `FlowAddWidgetRetainedCoordinate`, `FlowAddWidgetRetainedDialect`, `FlowAddWidgetRetainedOutcome`, and `FlowAddWidgetRetainedRequest`.

The combined scoped recovery set is 41 definitions. It is smaller than the removed 3,071-definition block and preserves the new public artifact boundary.

## Affected source-only gates

`📦️packages/🦀️rust/📜️script.ts` routes `test-source`, `child-identity-check`, and `child-edit-check` through the same `source-contract/🟦️.ts` import. Therefore adding only the eight `FlowChildAddWidget` definitions would merely reveal the next missing source-contract export. `add-widget-retained-check` has its own AJV setup and needs the six retained definitions independently before its native laws can start.

The two Recovery 9 logs establish only the pre-native AJV failure. A rerun is needed after the new fixture schema is registered to accept either source or native behavior.

## Candidate comparison update

Root's `🗑️generated/flow-editor-fixture-schema-candidate.json` was compared read-only against both `HEAD` and `599a5d8450^` at the former artifact-schema path. It contains 41 definitions, all byte-identical to both preserved sources. It has its proposed distinct fixture `$id`:

`https://semio.tech/schema/s/flow/flow/editor/fixtures.json`

No selected definition is missing or stale. The candidate intentionally excludes only six former definitions outside the current fixture-root closure: `CameraJson`, `FlowArtifactRecipesLayout`, `FlowSceneOwnerLaw`, `SynapseSpec`, `Widget`, and `WidgetLayout`.

All 27 candidate references are local `#/$defs/…` references. The candidate itself uses only `x-semio-formats` and declares no JSON Schema `format` values. The router's broader current strict registration set is safe to retain, but only `x-semio-formats` is required by this fixture module.

## GrantFrontier Current-Source Supplement

Updated: 2026-09-09T16:06:47+02:00. Current read-only source inspection found the scoped GrantFrontier declaration aligned with the narrowed fixture. `canonicalVariants` is absent from the fixture and `FlowGrantFrontier` definition. The definition keeps `additionalProperties: false`, requires only `schema`, `maximumTextBytes`, `productionGrantBytes`, `preparationGrantBytes`, and `cases`, and now requires at least six cases. The fixture carries exactly the six current boundary cases. The source-contract hostile corpus deliberately injects `canonicalVariants` as an obsolete additional field and removes one case as a six-case-corpus rejection vector. This preserves rejection coverage for the removed declaration rather than retaining it as live fixture data. No source task was run in this update.
