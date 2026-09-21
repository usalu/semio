# Component Engine Close Lane

Status: fail-first law and accepted bridge design after the host identity source migration on 2026-09-21. Native107 compiled the host-key packet and ran 32 selected laws: 25 passed and seven failed in the concurrent UI lifetime/presentation packet. The component Engine close law is queued for Native108 and production remains unchanged until that focused RED is recorded.

## Proven boundary

The CPU registry already separates its two identities correctly. `ensure_engine_surface(host_id, surface_id, ...)` reserves the registry slot with `host_id`; `EngineSurfaceIdentity.id` therefore contains the checked host key used by staged packets and the GPU raster key. The separately checked `EngineSurface.surface_id` carries the public document address used only by action builders. Sibling mounts with one public `surfaceId` do not share a CPU or GPU key after the current packet.

The remaining leak is retirement. `SceneSurfaceRetirement` drains the host-keyed scene state and pending raster producer, while `PairedEngineSurfaceClose` only runs as part of whole `OsHost` retirement and scans every engine slot. Component removal and same-window replacement do not close the exact EngineCanvas CPU token, its GPU candidate, or the raster table entry that the candidate published.

## Required lane

1. Extend `EngineSurfaceRegistration` with the exact `EngineSurfaceToken` returned by `ensure_engine_surface`. `World3d` and `IconRender` registrations carry no engine token. The Shell mirror retains this token with the host key; retirement never looks a token up through the public document address.
2. Factor the per-token portion of `PairedEngineSurfaceClose` into one reusable owner with phases `BeginCpu`, `BeginGpu`, `Cpu`, `Gpu`, `Raster`, `Witness`, and `Terminal`. The whole-host scanner composes the owner after its existing scan. The component lane admits one exact token and never scans.
3. Before CPU close, remove only a staged packet whose `EngineSurfaceIdentity` exactly matches the token and host ID. The current `StagedEngineScenes::remove_surface` is test-only and compares only the text ID; production needs a generation-bearing exact removal.
4. After the EngineCanvas GPU candidate is terminal, retire the exact `engine:<host_id>` entry from `GpuContext`'s `RasterTextureTable`. This requires an exact-key bounded retirement that also removes the key from committed, previous, and candidate residency sets. It must wait for any presentation owning those sets; it must not close the whole raster table.
5. A fixed component-close request owner is shared by the runtime worker and `OsHost`. `build_and_publish_snapshot` advances one phase before `admit_next_frame`, skips admission while the lane is pending, and invalidates `RESOURCE_READY`. The browser worker includes the same pending witness in `continue_frame`; its live host is the same `OsHost`, so native and browser use one protocol.
6. The Shell bridge separately takes an exact host-keyed `World3dState` or `IconRender` state and drives its dynamic retirement. It hands an engine token to the external lane only for engine-backed kinds. `SceneSurfaceRetirement` returns the UI component grant only after local raster retirement, Shell world retirement when applicable, and the external engine acknowledgement are all terminal.

## Accepted bridge API

The UI retirement callback only talks to a fixed independent request bridge. It must not borrow or call back into `RuntimeMailbox`, Shell, `AppPresenter`, UI tree state, or GPU state while the UI `RefCell` is live.

- `request_component_surface_close(owner, engine_token)` admits one exact `ScenePointerTarget` and optional `EngineSurfaceToken`, returning a generation-keyed bridge token or the untouched request on backpressure.
- `step_component_surface_close(runtime, presenter)` is OsHost-only and advances one bounded phase: exact staged removal, Shell World/Icon close, paired CPU/GPU close, exact raster-residency close, then witness.
- `component_surface_close_terminal(token)` observes only the exact bridge generation.
- `acknowledge_component_surface_close(token)` releases the terminal slot and its admission credit.

`SceneSurfaceRetirement` keeps the `SCENE_STATE` external reservation while a request is refused or pending. After its local pending-raster, gesture, hover, list-transfer, and state owners are empty, it admits once, waits for the exact terminal witness, acknowledges it, and only then lets the existing `SCENE_STATE` acknowledgement run. Generic scene kinds with neither an Engine token nor a Shell dynamic host complete locally; Engine and World/Icon tests must pump the real OsHost bridge.

The exact Engine token is bound to `SceneSurfaceState` during paint. NodeGraph, TiledMap, and Board2d emit `EngineSurfaceRegistration`; World3d emits a registration with no Engine token. Paint2d and TextEditor allocate Engine hosts but do not emit those Shell mirror registrations, so the common successful `sync_engine_scene` path must bind their tokens too. A retirement-time lookup by public `surfaceId` is forbidden.

## Fail-first laws

- Two engine-backed siblings share one canonical document `surfaceId` and have distinct host identities, staged packet identities, and `engine:<host>` raster keys.
- Closing sibling A removes only A's staged packet, CPU token, GPU token, and raster table entry. Sibling B remains renderable and its action still publishes the common canonical `surfaceId`.
- A staged packet from the retired token cannot be admitted after same-key replacement; the successor's generation survives the old acknowledgement.
- One close opportunity advances one phase, frame admission remains shut while pending, and browser continuation remains armed until the exact terminal witness is acknowledged.
- World3d and IconRender component replacement inside a still-live window retire the exact host state without retiring a sibling or waiting for whole-window closure.

The existing Native102 257th Canvas mount capacity failure is fail-first evidence for missing component retirement at the generic scene registry. The new laws must cover the distinct CPU, GPU candidate, and raster table owners; that receipt alone does not prove those owners close.

The first native law is now registered as `a_component_engine_close_retires_only_its_exact_staged_host_and_preserves_the_wire_surface`. It creates two CPU siblings with host ids `scene.41.3.7.1` and `scene.41.4.2.1` over the same public `shared-document-surface`, stages both generations, retires A through the current CPU ladder without the test helper's broad staged drain, and expects only B to remain staged. Production was intentionally left unchanged until the parent records the focused RED receipt.
