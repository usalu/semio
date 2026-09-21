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

## MIME Ownership and Actual Bitmap Evidence

Layout5 reaches the real pointer transport but receives `dispatch-failed` for `canvasDragOver`: the producer lost the authored kind MIME. The neutral fixture now includes each row's full MIME roster, including empty-valued metadata. Four mounted laws fail first (`canvas-pointer-mime-red-1.log`, 18 pass/4 fail), then pass after the Tree pointer owner snapshots the source roster and Canvas2d forwards it unchanged for both normal and first-move terminal preview. The final focused suite passes 23/23, including owned-array cancellation and native transport isolation. The separate window-template pointer signature law passes 1/1 selected (564 outside filter).

Layout6 proves actual preview dispatches are applied. Its page preview pixel assertion fails because the screenshot includes Actions/Search/Utilities DOM overlays that disappear while the catalogue panel enters its drag state. Visual inspection confirms the document paint itself is unchanged. Layout7 records the React canvas's actual PNG bitmap for scene mutation comparisons, and also retains a separate full visible screenshot beside every bitmap. It does not remove the scene pixel assertion or change app state.

## Layout 7 Physical Reference

All four physical React catalogue cases pass. Page count changes from 2 to 3; rectangle, text and image each change the frame count from 3 to 4 after a fresh Demo reload. Page preview preserves the canvas bitmap, while the three frame previews change it. Every pointer leave restores the exact original bitmap and preserves the document count. Drop retires the preview before committing one authored item. The adapter waits for the actual preview and drop ledger outcomes to become applied.

The receipt in `🗑️generated/astra-runtime/layout-react-7/layout/react/receipt.json` retains early action snapshots, so some stored outcomes are null despite the later applied-outcome assertion. Layout8 refreshes both gesture receipts after terminal outcomes and rejects any refused preview, leave or drop. The original receipt is preserved. Browser logs contain a resource 404 and connection errors from the optional development bridge at port 54500; no page exception was recorded. This is physical React Layout evidence, not WGPU acceptance or a console-clean global gate.

Layout8 passes all four cases again, with every stored preview, leave and drop outcome applied in both gesture receipts. The revised receipt is `🗑️generated/astra-runtime/layout-react-8/layout/react/receipt.json`. Adapter typecheck12 also completes successfully after explicitly typing the semantic action-name roster; typecheck11's narrowing error remains recorded.
