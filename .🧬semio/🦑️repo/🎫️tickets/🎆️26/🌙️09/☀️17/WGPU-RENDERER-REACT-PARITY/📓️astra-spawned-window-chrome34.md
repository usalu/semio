# Spawned Window Chrome Parity

The sealed paired runtime21 journey reproduced different names for the same Display drag: React created `3-Point` and `Orthographic`; WGPU created two `Puzzle3D` tabs. The native shell now seeds the concrete projection, title and icon together when a new window is created through either direct Display activation or a template drop. Moving an existing tab preserves that window's selected projection and custom chrome.

The existing language-neutral instance-title corpus supplies encoded projection specifications, titles and icons. The actual React codec, projection label/icon functions and rendered Mode produce the same expectations. The focused React file passed 6/6, with Nx exit 0 in 30.2 seconds (`spawned-title-react16.log`). The native law exercises both creation routes and a subsequent physical tab move; native execution remains pending. Terra's independent source audit found no concrete regression.

The change is not yet verified in a rebuilt browser artifact. Paired runtime22 must repeat template creation and relocation with the new build before runtime parity can be claimed.

## Runtime 23: Orthographic palette drag

A physical drag from the expanded Display palette Orthographic handle `(291,585)` to the right side of the Perspective body `(1190,360)` created a third window. Its visible tab and AX name are both **Orthographic**, with the matching grid icon and orthographic world view. Existing Curvilinear and Perspective sibling windows remain visible. New AX surface identity: `puzzle3d-main-2-6-0`. This confirms the spawned-title repair in the sealed WASM23 artifact through actual input and screenshot; worker console receipts remain unavailable pending the diagnostics bridge fix.

During setup the exposed AX secondary Expand action for Parallel did not change the row even after a later capture; the actual visible chevron click expanded it immediately. Orthographic itself incorrectly exposed collapsed/Expand despite having no children. These separate accessibility issues were forwarded to SolCanvas.
