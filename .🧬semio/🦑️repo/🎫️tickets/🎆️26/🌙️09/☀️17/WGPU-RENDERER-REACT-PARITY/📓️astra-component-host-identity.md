# Component Host Identity And Retirement

The wire document surface ID cannot key native component-owned state: every legal sibling Component::Surface node receives the same document surface_id, while React mounts separate hosts. The new sibling Canvas oracle passes35/35 React tests and proves independent cameras plus survival after one sibling unmounts. Current native fail-first has not executed because presented-revision integration has compile errors.

## Permanent Model

Keep surface_id as the plugin command address. Add a distinct runtime component host identity, minted from the stable mounted document generation, generational arena node identity, and component generation. Key retained scene state, EngineSurface CPU/GPU owners, pending raster uploads, editor overlays, cameras, and focus/clipboard addresses by that identity. Ordinary payload refresh keeps it; key/kind replacement, arena reuse, and window remount change it. Every physical input resolves its accepted retained component before reaching those registries.

A runtime-only UiComponentSceneNode host_id can carry this key without changing serialized scene data: serde(skip) and value(skip) both support that boundary. UiDocumentReconcile Mount already receives window_generation and establishes component_generation. NodeId needs an explicit owned scalar-parts accessor, not a debug-formatted key. Tests that instantiate scene nodes directly must stamp deliberate fixture identities; no empty-ID or document-ID compatibility fallback is permitted.

## Retirement Order

The exact window token first revokes input. Queued scene jobs, capture owners, Canvas gestures, focus, and clipboard streams retire silently. Each removed component hands over its exact host identity. The scene-state cursor drains nested strings, JSON, maps, sets, and vectors under per-item/per-byte grants and acknowledges the held map capacity only at terminal. CPU and GPU EngineSurface retirement reuse their existing per-token phases, with a renderer-owned external acknowledgement before the Ui window slot can be released. Pending rasters and editor UI state require the same selective host close. Closing a sibling must preserve every other host in the document.

## Current Implementation Status

Root has implemented bounded scene-state retirement and its close/replacement barriers, queued scene/capture retirement, and silent Canvas close guards. These are source-parsed, not yet Rust-verified. The first single-owner-per-document mount stamp is insufficient for sibling hosts and must be replaced before acceptance. Native103/104 failed before test execution due invalid Clone derivations in the new presented revision baseline; Sol is retaining credited copy semantics.

## Required Verification

Use the shared camera fixture for257 sequential working cameras, close silence, same-ID successor protection, sibling cameras, and sibling removal. Extend native coverage to all EngineSurface host variants and their actual CPU/GPU close acknowledgements, plus late Ink/Text clipboard completion. Then full renderer, UI, Flow, Draw, browser worker checks; rebuild matching selected plugin versions; execute short paired checkpoint, full57steps, Dock8, and all15 surface families. No browser or feature-complete acceptance is currently claimed.


## Native105 And Integration In Progress

Native105 was compile-only: the credited staged UI candidate baseline compiled, then the renderer stopped on 18 transitional host-ID and ordered-container call sites. No behavioral tests ran. The full diagnostic receipt is in `🗑️generated/astra-runtime/renderer-native105-retirement-and-presented/diagnostics.txt`.

Internal Canvas gestures, hover, camera deadlines, list transfers, scene controls, TextEditor popups, Ink state, raster upload keys, and World/Icon scene maps now use component host IDs. Document action payloads retain their original surface IDs. Canvas gesture callers now pass the stable window lifetime rather than the current tree revision. These edits are source changes awaiting native verification.

TextEditor popup state has moved into the admitted scene owner. Its rename text, spans, and queued menu label drain with that owner. Component retirement drains Canvas gesture/hover metadata and the exact pending raster surface; a checked-out raster token stays in its map until returned, and its item grant returns only after the retained raster owner is empty. Engine CPU/GPU and retained atlas-resource closure still require the audited external host lane.

Flow20 is running against the credited baseline while the renderer integration is completed. No new renderer bundle has been activated and no browser parity acceptance is claimed.


## Presented Scene Retirement Timing Audit

A newly identified integration risk needs an explicit law: candidate reconciliation can report an old component as removed while the previously presented tree still displays and routes input to that same host. The root scene retirement callback must receive authority only when the last presented/candidate owner of that exact host has released it. Otherwise the new close fence would disable the visible predecessor before its replacement pixels are acknowledged. The UI owner is auditing whether old presented document closure supplies the final callback and whether candidate-only callbacks must be deferred. This is an open risk, not a verified failure or a completed fix.

Native106 is running the integrated host-ID source. Focus and clipboard production retirement is held until its failure receipt; the late native callback test is present but requires a separate exact executable invocation because its name was not in the submitted filter.


## After Native107

The actual renderer receipt is 25/32 passing, including the 257-mount capacity test. Three clipboard failures are valid lifetime regressions; one keyboard test and two Canvas replacement tests remain red. The sibling test did not reach host admission: its root record was 0 while Shell publication declares root 1, producing `MultipleParents`. The fixture now uses root 1 and child records 2/3, and exercises both normalized wheel routes plus removing one sibling while preserving the other's zoom using the same neutral camera fixture as the 35 passing React tests. This expanded native law awaits execution.

`UiRetiredComponentScene` now carries the actual runtime host ID. Interpreter retirement moves that field instead of deriving it from a possibly different retained arena. Resource-owner comparison now uses mounted host identity independently of arena node location; pointer routing still requires an exact live tree node. These source changes follow the Native107 snapshot and are not covered by its green results.

The UI implementation work now includes a window-level component mount epoch, exact surviving-host projection across retained arenas, deferred removal until presentation acknowledgement, and capture mapping that allows frames to continue during held drags. The Engine close packet remains separate and requires native/GPU failure-first receipts.
