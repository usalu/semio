# Layout Physical Adapter

The fresh React activation completes in `🗑️generated/astra-runtime/activate-layout-react-1.log` and is served by the owned no-HMR listener at port 6379. Existing listeners are preserved.

Layout1 established that the current app intentionally has no example picker: `appSwitchesExamples()` requires the declared `setActiveExample` verb, which the actual editor and viewer manifests do not expose. `LayoutPlayApp::initial_snapshot()` creates `default_document()`, the bundled Demo document. The probe now reloads the browser before each case and asserts exact seed counts, rather than dispatching an undeclared command. The reset is pinned as `browser-reload` in the neutral probe schema and fixture.

Layout2 reached the Artifact panel and recorded the real DOM namespace `panel:layout-document/`. The adapter now uses exact section identities under the active, visible Artifact panel, and exact `panel:layout-catalogue/` source identities under the Catalogue panel.

Layout3 and Layout4 both reach two seed pages and the catalogue transfer handle. Layout4 additionally proves that physical pointer-down arms its ancestor as `draggable=true`. It nevertheless receives no native HTML drag events and no `canvasDragOver`, only applied `canvasPointerMove` actions. The passive browser event witness does not synthesize events or mutate application state.

Source trace points to a transport mismatch requiring audit: `Tree.buildPalettePointerProps()` prevents the native drag on pointer-down and starts `catalogueTreeDragController.pointerPaletteDrag`, while `Canvas2dHost` currently receives catalogue transfers only through HTML `onDragOver`/`onDragLeave`/`onDrop`. Other scene hosts have pointer-catalogue integration. Terra is independently checking this finding and its narrow repair. No Layout renderer acceptance is claimed.

React's public input ledger omits arguments by design. The adapter records its real `inputSeq`, controller, action, window and outcome. A successful HTML drop would be checked using the browser-trusted event, mounted host identity and canonical MIME payload, while WGPU can additionally expose its actual action arguments.

## Pointer Transfer Repair Boundary

Independent audit confirms the catalogue intentionally starts a pointer gesture. Sol's narrow repair gives Canvas2dHost a pointer transfer consumer while retaining the separate native HTML path. The focused mounted React oracle passes 21/21; an additional actual Tree listener-order regression is being added before source completion.

The physical probe's neutral contract now declares `pointer-catalogue`. Passive document capture records the trusted pointer-down source's exact rendered id, MIME and payload, then the receiving surface identity on pointer move/up. Assertions require the canonical source payload, addressed window, matching retained host id and an applied public ledger outcome. These witnesses observe the real physical gesture; no pointer or drag event is synthesized by page JavaScript. The browser-native HTML drag test remains a separate transport contract.
