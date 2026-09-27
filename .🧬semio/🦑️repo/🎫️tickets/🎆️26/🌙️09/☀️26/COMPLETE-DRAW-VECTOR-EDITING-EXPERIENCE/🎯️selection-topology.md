# Selection Topology

## Reproduction

The Draw preview on port 6064 rendered the Demo with one layer and zero selected. Clicking Select All selected its action tree row but left drawing selection empty; repeating with a double click did not change selection. No new console error accompanied the action. The old missing NamedLayoutStore import error predates the successful shell reload and is unrelated.

## Cause

Draw declares HierarchyProvider::Flat. The framework deliberately resolves that declaration to an empty DomainTopology. Its reserved SelectAll verb enumerates this topology, so the action cannot discover any layer even though its click handler is wired. The declaration also gives document-change pruning no authoritative layer membership.

## Contract

Use the existing framework interaction topology schema, with stroke-granularity nodes in document pre-order and structural group parent IDs. Boolean children are references, not nested nodes. Hidden or locked layers remain members so the Layers panel can select and inspect them; canvas gesture eligibility stays in hit testing. The language-neutral fixture covers empty, nested and every layer kind, including references that must not create duplicate or phantom nodes. Rust and TypeScript share the fixture; Three.js Object3D traversal is the independent test oracle.

## Verification in Progress

The first attempted target was incorrectly named @semio-tech/draw and failed before running tests. The registered @semio-tech/draw-js:test target is now running the manifest regression before the implementation. Existing native test handle 30810 and component build handle 43904 remain live and are not restarted.

## Implementation and Current Results

- Added iterative Rust and TypeScript interaction-topology twins under the editor interaction taxonomy. Traversal uses one stack frame per nesting level, preserving document pre-order without recursive call-stack growth.
- Changed the editor declaration to HierarchyProvider::Topology and connected its ArtifactEditor hook to the document-derived topology.
- Added an integration test for Select All on an untouched two-layer document, preservation of document content, deletion pruning, and the empty-document case. The existing manifest test now requires the authoritative topology declaration.
- The registered TypeScript target reproduced the old missing declaration (one failing regression); after implementation it passed **49 tests / 1,152 assertions**, including empty and nested fixture agreement with Three.js. Logs: `🗑️generated/tests-selection-topology-contract-red.txt` and `🗑️generated/tests-selection-topology-fixed.txt`. The first attempt's wrong-target failure is recorded separately and is not a test failure.
- Scoped git diff whitespace validation passed. No native runtime or browser success is claimed yet. Native handle 30810 and plugin component handle 43904 started before this edit; check their test roster and compiled artifact before assuming they include it.
- Preview tab 4 continues to receive shared-checkout hot reloads and temporarily displays Loading plugins. A subsequent action-control click found no matching element during that state; it was not retried blindly. No new console error appeared.

## Preview Dependency Observation

At 19:57:37 local time, the existing preview log reported `http proxy error: /trusted-catalog/plugin-modules` with `ECONNREFUSED`. The browser remained on Loading plugins after shared shell source reloads. This is separate from the verified pre-fix Select All behavior. The preview handle remains 33503; neither it nor another contributor’s backend process was restarted. TypeScript run 34811 is terminal success. Native 30810 and component 43904 were polled again and remain live.
