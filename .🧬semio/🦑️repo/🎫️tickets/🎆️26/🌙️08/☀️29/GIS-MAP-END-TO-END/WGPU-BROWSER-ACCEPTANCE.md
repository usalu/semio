# GIS2D Browser WGPU Acceptance

## Public launcher

Run `SEMIO_LANGUAGE=en repo ./📜️script.ts dev playground gis2d wgpu prototype`. The generated GIS2D row selects port 6140 and the GIS map editor. This starts the separate WGPU browser shell. The React GIS2D shell on port 6041 is independently owned and provides different renderer evidence.

The WGPU prototype renderer publication was missing. The graph-free launcher generated the browser boot, renderer boot, and frame worker, passed its pinned wasm-bindgen tool check, and admitted the 822-unit renderer source selection. Cargo successfully compiled the optimized `wasm32-unknown-unknown` renderer from the 1,104-unit artifact plan in 31m 19s. The launcher moved to the compiler provenance check before wasm-bindgen and browser publication.

The provenance check found a concurrent edit to the shared owned job context source and withheld the first linked renderer. The managed launcher re-admitted all 822 renderer units and produced a fresh plan of 1,104 artifact units under the same invocation. The warm retry received capacity and entered Cargo.

The warm pass reached provenance comparison and found a new edit in the shared framework Rust entry point. The launcher kept the renderer unpublished and entered its third snapshot attempt.

Cargo completed the third pass after roughly 23 minutes. Its recorded kernel input digest (`4503ed6e…`) differed from the admitted source digest (`70d3af7e…`). The kernel source changed again by 21:22:05 CET to `336a3fbc…`. The launcher exhausted its three allowed source snapshots and exited with `SEMIO_CONCURRENT_EDIT`. No renderer receipt or listener on port 6140 was published.

## Earlier detection of Cargo source changes

Ordinary artifact builds now use the existing bounded source drift watcher. After a proved byte mismatch, the build drains the stale compiler and releases its capacity before returning that edit to the build retry. The retained-source cases keep their existing witness deferral. Final source and artifact provenance checks still guard publication.

An ordinary build fixture created a delayed marker representing completion of the stale link. The original driver let it write the marker and spent 3.548 seconds in the compiler before a fresh build could publish. The updated driver stopped the stale child before the marker and ran both compiler attempts in 0.311 seconds. Exactly one fresh artifact and source receipt reached the consumer, and the receipt's source digest matched Noble SHA-256. All five focused tests passed with 79 assertions. These include two retained-witness cases covering drift both before and after the compiler announces consumed-source proof. The wider direct invocation also tried isolated cases that require an Original from the managed suite runner. Those cases stopped at test setup and do not provide results for the new watcher.

The fresh managed WGPU invocation exercised the watcher on its actual compiler. After roughly two minutes of Cargo work, it observed the framework Rust facade edit from `a53c3003…` to `54a732eb…`. An independent SHA-256 read matched the observed digest. The launcher drained that compiler and began source admission for attempt two under the original governed invocation. No renderer was published from the retired compiler.

The next plan found a separate kernel identity source edit (`2235b5ca…` to `1d89dead…`) during its precompiler check. The live SHA-256 matched the new digest. This time the planning loan threw the edit before retiring, closed the Original, and prevented the requested retry. A second neutral regression reproduced that closure with a source edit between planning and rechecking. Returning the typed edit through the planning loan and reporting it after retirement lets the build retry under its Original. The stale plan starts no compiler and exactly one subsequent compiler publishes. Both regressions, the completed-provenance tests, and the retained-witness tests passed together with 92 assertions across six cases. The WGPU launcher was reissued through a fresh Original with both changes and remains the sole WGPU renderer producer.

After a three-core grant, the reissued launcher caught another live edit in the retained-turn actor frame source (`5764635a…` to `1f20a134…`) and started the next planning attempt under the same Original. The live file digest matched the watcher's observation.

That second plan detected a live infinite-world interaction source edit (`2fe8e5c7…` to `7019dcc3…`) during its precompiler check. It started attempt three and resumed Cargo planning under the same Original. This is the planning stage that closed the earlier invocation before the fix.

## Standalone descriptor loading

The specific WGPU composition delivers the full installed catalog. The worker previously fetched every listed package descriptor before planning the variant. The graph-free launcher's generated GIS boot closure initially contained only GIS, so unrelated descriptors could block startup before the GIS actor loaded.

The worker now scopes implicit standalone descriptor loading to the variant owner and its contributors. It includes transitive dependencies only after reading their owner's trusted descriptor. Final planning checks presence, cycles, and exact version pins before actor startup. Host-wide and explicitly supplied mounts continue to read their supplied inventory. Lazy activation keeps access to the full admitted module address table.

The neutral regression reproduced the original attempt to read an unpublished unrelated descriptor. All four focused catalog worker cases passed after the change, with 11 assertions. The new case read exactly the four authorized fixture descriptors, selected the GIS2D editor, and agreed with an independent Graphlib reachability calculation. The frame worker regenerated and matched its deterministic projection using the authorized framework package entry. The wider source suite also reported a separate dev-serve argument mismatch, expecting direct `serve` while the current command returns a repo target. That broader run was cancelled while waiting on an independent process test.

A read-only probe of the actual WGPU catalog used the browser package already served on port 6041. The GIS descriptor and package receipt passed admission. The worker then followed the GIS descriptor's trusted `stdio =0.1.0` dependency and received HTTP 404 for the stdio descriptor. The committed GIS plugin assembly and descriptor declare that dependency. The component build metadata and its three dependency-bearing catalog projections now agree with that declaration. The focused suite passed five tests and 19 assertions, including a recursive SQLite comparison with the GIS2D and GIS3D boot closures. The direct `components iterate gis` command remains a single-component publication.

## Managed stdio publication

The selected prototype stdio component can be built on its own through the same repository driver that published the GIS actor and browser package. A read-only run of the live `componentPassPlanV1` returned one `prototype-00` pass. It selects `semio-hub-stdio-component` from `🌎️hub/Cargo.toml` for `wasm32-wasip2` in the prototype profile.

```sh
gis_ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END'
SEMIO_LANGUAGE=en SEMIO_BUILD_MODE=prototype \
  SEMIO_PLUGIN_CATALOG="$PWD/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated" \
  repo ./📜️script.ts components iterate stdio > "$gis_ticket/🗑️generated/wgpu-stdio-iterate.log" 2>&1
```

The local prototype store already has the Darwin descriptor emitter and native transpiler binaries. The prototype and release browser stdio package paths are both absent, and the inspected local compiler lanes have no linked `semio_hub_stdio_component.wasm`. Existing native stdio Rust libraries demonstrate source overlap with other builds. WASI unit-cache reuse is decided by the managed driver's platform, profile, and source keys during admission. The command uses the default warm generation and leaves the remote and local unit caches enabled. The stdio invocation started only after the active GIS edit build received its three-core grant and admitted all 928 sources.

The first stdio invocation reserved 13.0 GiB after 1,465 seconds in compiler-image admission. It passed host admission and reached Cargo configuration. Its original configuration snapshot then refused a concurrent edit to the Cargo profile constraints JSON and exited 1. The fresh managed invocation reserved 14.4 GiB after 24 seconds, received a one-core compiler grant, and admitted all 1,370 selected source units. It reached 461 compiled units while the WGPU renderer was retiring its second snapshot.

The managed stdio compiler remained active at 687 compiled units when the renderer launcher finished and reached 713 during the source-drift tests. Its Cargo source admission includes `semio-s-artifact-stdio-dwg-read`. The native baseline previously reached that reader using producer units compiled before the coordinated update, while the current sources contain the missing DWG types and method. The stdio run has yet to emit a relevant DWG diagnostic.

## Retained session freshness

The retained prototype session has a valid owner and file roster. Its copied plugin catalog still records an empty GIS dependency array, and its generated session lists GIS alone. The reader checks the file roster, compares the session TypeScript with a re-render of the copied JSON, and rechecks the owned artifact reader. The graph-free launcher previously treated the receipt's presence as sufficient to reuse the session. The normal producer is the registry's `SessionScript`, which reads the current selected catalog and publishes the three session files under the managed artifact owner. The graph target is `@semio-tech/plugin-registry:session-gis2d`.

The graph-free launcher now compares the selected catalog projection with the one returned by the existing session reader. A structurally equal match reuses the publication. When the retained projection is absent or differs, the launcher publishes the selected projection through the governed session producer. The browser paths reactivate when a session is refreshed. The cold, exact warm, and changed dependency cases failed before the new freshness operation existed and passed after it was wired into the launcher. The focused suite ran eight tests and 25 assertions.

The fresh governed WGPU launch reported `Playground session gis2d: 2 plugins staged`. The normal publication wrote the owner and file-roster receipt at 20:28:31.640 UTC, and the retained GIS catalog row now lists `stdio` in `dependsOn`. The launcher then entered its renderer build with the pinned wasm-bindgen 0.2.129.

The subsequent governed launch reused that exact receipt. Its modification time stayed at 20:28:31 UTC and there was no second staging event before renderer planning.

## Live probe

`probe-gis-wgpu.mjs` uses the WGPU readiness marker, OffscreenCanvas host, and retained GIS2D editor window identity. It requires completed tile responses with the expected media types, stable nonblank canvas pixels, visible changes after pan and zoom, and new tile requests after camera input. Desktop Chrome uses middle drag and wheel. Chrome mobile profiles use trusted Chromium touch pan and pinch. WebKit emulation can check a trusted tap. Pinned Node accepted the probe syntax.

Once the launcher reports the listener ready, run the pinned Node probe through the repo tool registry.

```sh
gis_ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END'
GIS_CHROMIUM_CHANNEL=chrome repo tools exec node -- "$gis_ticket/probe-gis-wgpu.mjs" 'http://127.0.0.1:6140/?plugin=gis2d' 'Desktop Chrome'
```

The browser probe awaits a published WGPU renderer and stdio component.
