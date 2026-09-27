# World3d Per-Window Viewport Retention

## Contract and ownership

The neutral instance-title contract now carries one retained viewport with a shell-session owner, window id, boot projection, selected projection/icon and complete selected camera. The strict schema validates that lane before the mounted React law consumes it.

`World3dWindowViewRegistryV1` is owned by `ShellHost`, not a process-global per-id map. It supplies separate stores for the primary program and every spawned program, so changing the primary session does not erase a still-live spawned viewport. A primary owner change clears that owner's old projection seeds and retires exactly that owner. A real window close retires only that window; a spawned owner retires when its final pane closes. Existing one-shot projection seeds remain sticky for the initial StrictMode mount sequence and are ignored once a live retained snapshot exists.

`World3dHost` retains the live parsed camera, viewport ownership, fit revision, detach epoch, content-frame seed, last dispatched camera, observed and attached scene-camera text, and a pending projection selection. This prevents a maximize/tab remount from replaying the boot Top seed and lets the existing echo predicate distinguish the guest's own `setCamera` echo from a genuine external camera change. Projection chrome remains on the existing title/icon path; this packet does not change authored-title or explicit-rename policy.

## Fail-first and repair

The mounted law seeds Orthographic/Top, applies Axonometric/Isometric through the actual World3dHost gizmo callback, unmounts the host, and mounts the same live window again. Before production wiring, the remounted rig received Orthographic/Top instead of the selected Axonometric/Isometric state.

After wiring the store, the same law preserves the selected projection and full camera across the live remount. Explicit retirement then restores the boot projection on the next mount.

Two follow-up mounted laws close lifecycle races found in read-only review. A projection intent now remains retained when the snap driver clears its local trigger at animation start and only retires when the final gizmo camera is accepted, so a remount during the 280 ms snap resumes the selection. A primary-owner retirement preserves a spawned owner's retained camera while clearing the retired primary owner's unconsumed boot seed.

## Focused receipt

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-world3d-retention/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx' --silent=false --reporter=verbose --testNamePattern='retains the selected projection and camera across a live window remount, then retires them with the window'
```

Result: one test passed and 18 were skipped; Vitest took 13.60 seconds and Nx took 14.8 seconds. The jsdom canvas/Three DOM diagnostics are pre-existing test-environment noise; the mounted host reached every projection/camera assertion.

The first complete World3d interaction file passed 19 of 19 tests. After the lifecycle follow-up, the three retention laws passed together in 14.27 seconds of Vitest and 16.6 seconds of Nx time. The final complete file passed 21 of 21 tests; Vitest took 14.81 seconds and Nx took 17.2 seconds.

The shared instance-title/schema file passed 5 of 5 tests after its retained-camera tuples were tightened to exact three-number arrays. Vitest took 11.82 seconds and Nx took 13.3 seconds.

## Limits

The focused law mounts the actual production World3dHost with renderer seams for WebGL-owned controls. Root owns fresh physical Shell verification and the full React/native/WASM gates. This packet changes no WGPU Shell code.

Root's fresh physical React check selected Isometric, maximized and unfocused the window, and observed Isometric still selected without a runtime error. Terra's final read-only lifecycle review found the current primary/spawned scopes, close retirement, boot-seed cleanup and snap-start retention coherent.

The React package typecheck ran for 55.2 seconds and reported five title-contract errors outside this viewport implementation: one existing `string` to branded `UiLabel` assignment in `Shell/🟦️.tsx:939`, plus four existing typing errors in the root-owned instance-title test at lines 60, 77/84 and 117. It reported no error in the World3dHost or ShellHost retention changes. Root owns that title-contract repair.

## Title-contract type repair

The title and icon fixture now crosses the UI contracts through their real typed boundaries. `withWindowLayoutTitle` turns the explicit runtime title into a `UiLabel` when it enters `WindowLayoutNode`; the test derives the projection icon from the fixture's actual projection spec and validates that it belongs to the icon catalogue before dispatch. Its authored seed titles use `uiDataLabel`, the nullable layout is narrowed once before geometry checks, and mounted `Mode` windows include the required icon.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-title-typecheck/tmp' bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧱️elements/🛠️ShellHelpers/🧪️tests/🌐️instance-title/🟦️.tsx' --silent=false --reporter=verbose
```

Passed 1 file and 5 tests. Vitest took 18.44 seconds; Nx took 20.1 seconds.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-title-typecheck/tmp' bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache --excludeTaskDependencies
```

Passed with no diagnostics. Nx took 39.4 seconds.
