# Native World3d terrain

The native host now applies the `terrainJson` scene lane before resolving its visible tiles. It forwards the configured zoom range to the shared terrain session and uses the existing terrain asset lane. The URL and tile coordinates must match a current request before a response can update the elevation cache.

The native scene uses the shared terrain projection for position pins, routes, and region boundaries. It builds one new overlay tile per frame, depth-tests markers and paths, and invalidates projected geometry when the overlay data changes. An overlay-only edit keeps already loaded terrain meshes available.

The native viewport shows the elevation source label in its lower-left corner. Source URLs are rendered as links by React.

The native renderer advances a fixed, fuel- and deadline-limited batch of tile writes per frame. An automatically framed camera can adjust altitude once from the DEM tile containing the project origin. A saved camera pose or user gesture skips the adjustment.

The focused World3d host test exercises the actual scene lane, an independently encoded Terrarium tile, stale response rejection, the overlay cache, and all three rendered feature classes.

`repo cargo --store=shared --configuration=prototype -- test -p semio-os-infinite-world-host --features=testkit --test world_host_unit_2 terrain` passed both tests.

Native asset transport failures enter the terrain tile miss cache while the failed tile stays visible. Panning out of the area and returning allows another fetch. Changing the DEM source, origin, or exaggeration also clears the cache.

## Renderer build prerequisites

The renderer check uncovered an execution-fuel method outside its owning `impl GuestRelayWakeDriver`. The existing method moved inside the implementation in the wake policy module.

The kernel protocol check also found that its public replay registry accessors returned private types. The registry types and the refusal step result now have the visibility their existing runtime callers require. The registry fields and refusal publication helper are scoped to the replay module.

An intermediate renderer check stopped in `semio-framework-job-return` with E0599 because the compiled payload dependency did not yet expose `OriginalReturnEscrowTable::supports_core_header_extent`. A retry against current source passed that dependency and the kernel protocol package.

A subsequent check found that public `reserve_seq()` returned a private `RendererSequenceReservation`. The reservation type is now public with its fields encapsulated. The next check passed the kernel-retained package.

The first check of the renderer frame transaction and runtime mailbox stopped in `semio-renderer-wgpu-kernel-pool` with four errors. The kernel pool now imports the existing app replay decoder and the guest runtime trait that supplies its compile method. The shard identity policy now derives the framework value decoder using the same field naming and unknown-field rejection as its Serde contract.

The next renderer check stopped at the plugin SDK's re-export of `WindowResourceRequests`. That public type is present in the current window package source. The check's admitted source predates that edit.

The live-source retry of the kernel pool, frame transaction, and runtime mailbox advanced through cached and rebuilt dependencies. It was canceled before a package verdict to leave resources available for the normal native launcher.

## Native s plugin document

The GIS editor and viewer terrain windows publish their descriptors on the ordinary `World3dScene.terrain_json` lane. The native renderer invokes the World3d host through its scene surface step. The host decodes that lane before scheduling terrain and overlays. HTTP tile requests use the renderer's existing bounded native asset stream and OS HTTP service.

`repo run @semio-tech/framework-os-dev:run-s-native-prototype --lang=en` opens Space's landing app. `repo run @semio-tech/framework-os-dev:run-gis3d-native-prototype --lang=en -- --example demo` selects the registered GIS Terrain editor and built-in demo in the same native renderer. The launcher opens the admitted playground catalog and native plugin modules for the selected variant. A visible native GIS document is still pending.

Both launchers waited for the first sealed project graph. The owner-only launch with `--exclude-dependencies` still joined the graph build because the native s target comes from inferred playground targets. Invoking either script path directly failed because native runtime publication and execution require the parent script invocation issued by the repository build.

`repo show projects --graph-policy=current --lang=en` exited 1 with “this machine holds no sealed generation of the whole workspace.” Applying `SEMIO_BUILD_GRAPH_POLICY=current` to the normal GIS3D run still joined the same active fact-store load, so that duplicate was canceled. The original GIS3D run remains queued behind the same fact-store process, which advanced to 1,390 completed owner checks and 60,096 verified memo rows across 296,011 source observations. At that point the loader had run for 4,046 seconds against its expected 600 seconds, and the launcher had not spawned the native renderer.

The inline elevation test submits a 33 × 33 grid through the World3d scene lane and checks that the native host publishes one drawable mesh, samples an overlay at its expected 115 m elevation, enqueues no tile requests, and retires the mesh when the source is removed.

`repo cargo --store=shared --configuration=prototype -- test -p semio-os-infinite-world-host --features=testkit --test world_host_unit_2 terrain` passed all three terrain tests against the current first-party JSON decoder. The earlier run had captured the surface manifest before the Pack dependency was added and failed to resolve that import.

The original native launcher subsequently advanced to Nx tooling authorizer publication and exited 1 with “Authenticated derivative material reference required” (reference `6313f7d6`). The native renderer did not start. The same GIS3D target was started with `repo --tool=semio` to use the repository's in-house backend. That request joined the active graph reader. A localhost probe found ports 6070, 6083, and 6066 closed. The current `repo` binary declines the root `dev playground` command as unrecognized.

## Elevation brush pointer integration

React renders terrain as a mesh in `TerrainTileVisual` under `WorldTerrainLayer`. The triangle hit of that mesh is available to a pointer callback on the mesh. The parent World3d host's existing `worldPointerDown` and `worldPointerMove` use a Z=0 plane and therefore produce a displaced geographic coordinate on raised terrain.

Native interaction authority arms `WorldRayPickPurpose::Surface` for the existing object surface brush. That cursor traverses authored instance draws. Terrain has a separate draw pass through `sync_terrain`; its brush now raycasts the same inline grid triangles through the Surface crate.

The shared pick returns the nearest forward intersection, geographic longitude and latitude, and the nearest sample counted from the southwest. The neutral fixture includes a nonplanar cell, an oblique ray, and three misses. The browser harness compares those rays against Three.js mesh intersections.

Native World3d validates the plugin's `interactionJson.terrainBrush` descriptor. The bounded native action builder uses the descriptor's action ID, operation, radius, and value with the clicked grid sample's column and row. A drag, cancelled press, changed source, or changed brush descriptor suppresses the edit.

The exact Surface ray unit passed against the nested Framework workspace using the terrain package's manifest path with `--lib --locked embedded_elevation_pick`. An independent interpolation calculation confirmed 240 m at the edited sample and 204.5 m on its adjacent triangle. The Three.js CPU oracle passed all six rays from the neutral fixture.

The initial native journal compiled and then faulted on an unclaimed primary drag intent. The brush authority now consumes that gesture without dispatching an edit, preserving navigation from other buttons. The warm `world_host_journal_sequences native_terrain_brush_click` rerun passed and confirmed edit and drag behavior before the descriptor was extracted. The updated journal uses an action name and brush parameters distinct from the GIS Terrain presets to verify that World3d dispatches the plugin's exact authored values.

`repo cargo --store=shared --configuration=prototype -- test -p semio-os-infinite-world-host --features=testkit --test world_host_journal_sequences --test world_host_unit_2 terrain` passed all four native checks, including the plugin-authored action and arguments, drag suppression, scene wire decoding, embedded grid rendering, and Terrarium tile lifecycle.

The native Semio backend launcher remains active behind the same graph builder, which reached 1,993 completed owner checks and 521,728 verified memo rows over 307,465 source observations. It has yet to open the native GIS Terrain window. The repository root script has a graph-free `dev playground` route for React. Its authored protocol admits React only, so the native launcher remains on the normal repository target.

## Discovering and opening the built-in Terrain demo

The active s playground roster assigns `gis3d` to the GIS plugin and editor controller `s.gis.gisterrain@1/*#editor` on React port 6083 and native port 6183. The GIS plugin roster activates on the Terrain artifact kind. The Hub GIS composition registers that controller alongside the Terrain viewer. The scoped React playground passes `?example=demo` through the standard playground boot path.

In the full s Space host on port 6070, a Space panel exposes the command palette's registered programs. Selecting the editor under `semio / gis / 3d` creates the app from the resident GIS plugin. The focused editor's fixture picker filters the published examples by dialect and dispatches the ID `demo` to its controller. The editor's current command reads that ID from its own example catalogue and parses the bundled snapshot containing the Liège grid and overlays.

The committed Hub GIS component descriptor pair predates the current example source. Its semantic JSON declares the Terrain editor and `demo` but exposes a 159-byte Terrain DSL body containing only the original exaggeration and demo mesh. It also omits `editElevation`, `configureTerrainBrush`, and the terrain brush utilities. The ordinary picker can resolve the ID against that manifest, and a freshly built guest resolves it against the bundled catalogue. The repository's `components describe gis --profile prototype` generates both the binary and semantic JSON from the current component. Session 90446 selected exactly one component and entered Cargo planning and shared compiler admission.

A read-only acceptance probe followed the actual s playground's GIS package owner to its published descriptor. It passed Space ownership, Terrain app identity, GIS artifact activation, and Terrain Demo discovery. It failed the expected elevation and imported map DSL fields and the expected `editElevation`, `configureTerrainBrush`, and `generateTerrainContours` actions on the stale descriptor.

After 973 seconds of shared compiler admission, session 90446 was still active. The published descriptor pair retained both November 19 timestamps and the original file sizes. The native launch reached 3,259 completed owner checks and 723,904 verified memo rows. It remains queued behind the graph reader.

## Space startup Store contract

The normal s browser launcher failed compiling Space's one-item preparation. Space core and the Home configuration editor still implemented the former Store contract. Both now use the caller's original receiving control to admit and account for advance work. Both prepare already-owned values synchronously and hold no native decoder frame to recover. They pass the caller's retirement effect directly to Store's existing one-item owner ladder so close progress and refusals use the original grant.

The Home cancellation and cleanup test now drives the Store ladder with the original effect bank. It retains its zero-grant, close-before-begin, and terminal checks and covers the absence of a pending decoder frame. The targeted command uses the Home editor package manifest inside the s workspace. Session 39842 passed source admission and compiled Store, ABI, and manifest dependencies. It was canceled after 1,370 seconds to release compiler capacity for the normal scoped GIS3D browser launch. It had not reached either changed Space package or run the test. Completed Cargo artifacts remain cached.
