# Native WGPU Engagement Pointer

The native World3d authority reads `engagementSessionActive` from the published scene selection. While the flag is active, primary presses and pointer motion publish the declared engagement actions through the bounded input outbox. The authority uses the existing live camera and clipped viewport to intersect the ray with world Z=0. It rejects parallel rays, intersections behind the camera, nonfinite coordinates, and points outside the live pick bounds.

The action carries the window instance ID, the scene surface ID, and the last path segment of the scene surface ID as the pane. The WGPU pointer entry verifies the retained target's window and scene surface before enqueuing. The bounded publication rechecks the window, surface, controller, and action declaration before emitting a staged pointer action. Existing handle and touch-navigation handlers keep their gesture routes.

The move queue retains the newest pending hover sample while preserving press and release order. A primary press claimed by engagement also owns its release, even if a guest update clears the scene flag in the meantime. Camera settle continues to report the camera without replaying a synthetic engagement hover.

The renderer's frame transaction steps the World3d authority with its input outbox. Frame actions transfer descriptors from that outbox to deferred gesture dispatch, and the shell resolves the explicit `windowId` against the live view before calling the guest with the flat arguments.

The shared JSON fixture defines a valid ground point, a clipped point, parallel and rear-facing rays, and inactive or undeclared engagement. The native journal additionally checks motion coalescing, the flat action payload, and release ownership. A Three.js perspective ray independently checks the fixture coordinates.

The Three.js check and `repo tools exec git -- diff --check` passed. The non-test Rust source closures total 3,957 lines and 243,470 bytes for interaction, and 3,053 lines and 161,057 bytes for cursors.

The focused native journal passed with one test, zero failures, and 17 unrelated cases filtered out. Cargo completed the cold prototype build in 76 minutes 55 seconds and ran the test in 0.05 seconds.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-os-infinite-world-host --features testkit --test world_host_journal_sequences engagement_pointer_uses_the_live_selection_camera_clip_and_window_address
```

The WGPU Chromium probe now wheels over Workpiece and requires the renderer's live camera and the guest's published camera to agree on a position different from the starting pose. It projects two fixture ground positions through that camera. Each move must reach the addressed Workpiece with all three coordinates inside tolerance and leave the processed mesh unchanged. A physical click must issue the addressed placement and change the mesh. The Bun transpile check, scoped whitespace check, Three.js placement oracle and Ajv camera payload oracle passed. The live browser assertions are pending. [Chromium probe](verify-wgpu-placement.ts)

The browser probe witnesses both delivered hover positions, an unchanged published mesh during hover and a changed mesh after placement. The WGPU compositor appends engagement segment vertices to the same line buffer as selection and other overlays. Its `sceneDraws` diagnostic counts the batched draw, so a stable count cannot establish whether the wireframe appeared or cleared. The renderer's existing native reference-and-pick law exercises segment decode and emitted endpoints. The React browser probe contains direct wireframe movement and removal assertions.

The WGPU probe now saves `wgpu-workpiece-armed.png`, `wgpu-workpiece-hover-1.png`, `wgpu-workpiece-hover-2.png` and `wgpu-workpiece-committed.png` under the ticket's `🗑️generated` directory. The armed capture follows the utility click and precedes the first placement move. The hover captures follow the addressed actions and unchanged mesh checks. The last capture follows the addressed press and observed processed mesh change.

The same crop covers the addressed Workpiece World3d rectangle throughout the sequence. It trims two CSS pixels from the pane edges, intersects the visible canvas and the 1,600 × 1,000 viewport, and uses one image pixel per CSS pixel. Two animation frames precede each sample. The probe takes up to five samples, saves the latest, and logs whether two consecutive PNGs matched. The image evidence is for inspecting the wireframe's presence, movement, and removal. A stable pair can precede a later worker repaint, and live visibility is unverified until the canonical WGPU run produces the captures. The probe transpiled with Bun.

A separate managed Chromium loopback probe checked the screenshot path at CSS scale. It cleared a WebGPU canvas to opaque magenta, waited two animation frames per sample, captured the same 12 × 8 clip twice, and decoded the PNG with `createImageBitmap`. The samples matched byte-for-byte and the decoded pixel was `[255, 0, 255, 255]`. This verifies the screenshot mechanism in the managed browser. The four Workpiece captures still depend on the live WGPU app.

The managed Chromium probe now compares the real Workpiece PNG before placement hover with the frame after each of two moves. It projects the authored cut box into the clip, requires a visible change in that region, and checks that the same surface keeps its mesh until the pointer press. The recorded screenshots remain available for diagnosis. A separate check used Sharp to encode synthetic images, compared them with the production probe's PNG and crop logic, and observed 0 changes for the identical and out-of-region inputs and 20 for the in-region input. The live Process3d Chromium run still awaits activation.

The WGPU probe now sends a real Escape after the first placement hover and requires an `engagementAbort` action in the accepted chrome ledger. It waits for the existing Cut control's mirrored `aria-pressed` state to clear before re-arming. The second hover must draw the moved preview, and committing must update the mesh and return Cut to its idle state. The production accessibility mirror passed a jsdom check for all five projected state changes. The live native keyboard and pointer sequence awaits activation.
