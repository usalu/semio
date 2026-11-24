# Prototype boot (2026-11-24)

## Timeline
- 10:38 to 12:30: waited for hub-15 (`semio-hub-flow-component:component`, 6597 s); `🌊️flow` staged 12:30.
- 12:31 WGPU 6118: `wgpu-native-activate.ts` published receipt [flow, procedural, raster]; `g3dwgpu` restarted; HTTP 200.
- Smoke (`🗑️generated/wgpu-native-smoke/{console.txt,shot.png}`): boot now passes plugin registry and reaches `procedural` editor, then fails:
  1. `[DEBUG] sealer blocked / one-item publication blocked ... stalled 'setActiveExample'` (stale staged procedural from Nov 23 14:39, 755+ sources newer, includes the "occupied" publication fix).
  2. `wgpu-worker panicked ... collection-arena/🦀️.rs:138: UI value arena requires its actual registered Source: UnsupportedOwner "UI arena has no accepting lifetime registration"`. Renderer wasm `.cache/g3d-native/wasm-dev` is from Nov 23 02:15 (older than source); likely fixed in source, needs rebuild (`wgpu-native-wasm-build.sh`, cold ~1 h, ~25 GiB).
  3. `404 /extension-modules/install` (extension inventory unavailable; non-fatal trace).
- 12:35: rebuild of procedural component started (`🗑️generated/native-component-procedural-16.txt`).

## Next
Activate (wgpu + react), rebuild renderer wasm if panic persists, restart serves, smoke both.

## Ownership (coordinator, 12:40)
Another agent owns the WGPU frame-worker panic, the renderer wasm rebuild and the 6118 smoke. This lane owns the procedural rebuild, React 6018 (activate with `S_G3D_RENDERER=react`, serve, smoke, screenshot, console errors) and the publication stall if it persists after the rebuild. The WGPU wasm is not rebuilt here.

## React 6018 (13:30 to 13:50)
- Procedural rebuild (`native-component-procedural-16.txt`, exit 0, staged 13:31). `S_G3D_RENDERER=react wgpu-native-activate.ts` published receipt [flow, procedural, raster].
- Serve root causes fixed in `serve-generation3d-native.ts`: (1) the old g3dreact serve (pid 97478) still held 6018, killed (session named in notes); (2) middleware mode with an external listener gives Vite `httpServer === null`, so the stream mux never attached its upgrade handler: `/semio-stream-mux` answered 200 instead of 101 (repeated console errors) and Vite HMR connected to port 24678 owned by the WGPU serve (400). Now Vite owns its HTTP server (`server.listen()`, `hmr: false`).
- Smoke `react-native-smoke.mjs` (copy of the wgpu smoke, 6018): 0 console errors, 0 page errors, 0 http >= 400. Publication stall and "occupied" error are gone. Screenshots `🗑️generated/react-native-smoke/shot.png`, `artifact.png`.
- Remaining: flow graph canvas and 3D preview show no geometry after 40 s; Artifact panel shows Brep.curve.polygon "Queued", Math.vector "Computing", Brep.solid.extrude "Queued": brep and math extension modules are not staged. Started `repo run semio-s-plugin-flow-{brep,math}-component:component -c prototype --exclude-dependencies` sequentially (`native-component-{brep,math}-16.txt`).

## brep extension build failure (14:20)
`repo run semio-s-plugin-flow-brep-component:component -c prototype --exclude-dependencies` failed after 31 m (`🗑️generated/native-component-brep-16.txt`, exit 1) with a single rustc error in `semio-s-plugin-flow-brep-original-jobs`:

`error[E0277]: (dyn ErasedSnapshotRetirement + 'static) cannot be shared between threads safely` at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves/🧾️topology-output/🦀️.rs:42` (`#[derive(semio_framework_value::RetireOwned)] struct TopologyOutput`), chain: `TopologyOutput.entry: Option<BudgetedOwnedEntry>` -> `neural_engine::EntryOwners` -> `OriginalOwnedEntry<Value>` (`🌱️value/🗂️ordered/✍️owned-entry`) -> `RetainedCloneClose.retirement: Option<Box<dyn ErasedSnapshotRetirement>>`. The derive (`✨️derive/🧬️retained-clone/🦀️.rs:102-105`) emits `where Self: Sync` overrides (`shared_original_demands`, `native_shared_original_admission`), so a non-Sync field makes the derive fail (trivial bound). `ErasedSnapshotRetirement: Send + Any` has no `Sync`, and nothing declares `RetainedCloneClose`/`OriginalOwnedEntry` Sync. The `entry` field is newer than the last green brep build (topology-output mtime Nov 20).

Not fixed here: the fix is a framework soundness decision (an `unsafe impl Sync for RetainedCloneClose`, as done for `RetainedCloneLeaseOwner`/`RetainedCloneSource`, or storing the entry out of the derived struct / making the derive's Sync overrides conditional), in an area others edit today (derive, value/retirement). Math extension build (`native-component-math-16.txt`) was still running at 14:24; brep is required for the 3D example (Brep.curve.polygon, Brep.solid.extrude), so the React preview stays empty until brep builds.

## React 6018 after brep/math (15:00 to 16:20)
- Math staged (`🧮️flow-extension-math`, exit 0). Brep rebuild 17 failed in crate `semio-s-plugin-flow-brep` (14x E0433 `super::original_curves` in `🔶️primitives-curves-surfaces-and-sweeps/🦀️.rs`: the module is the crate alias `original_jobs`); fixed by renaming the path; rebuild 18 exit 0, `🧊️flow-extension-brep` staged. `wgpu-native-activate.ts` (react) receipt now lists brep and math.
- Smoke: 0 console errors, but the flow canvas and preview stay empty: Artifact panel Brep.curve.polygon Queued, Math.vector Computing, Brep.solid.extrude Queued.
- Root cause chain (verified):
  1. The staged session copy `.cache/.../sessions/generation3d/🔌️plugins.json` was stale (Nov 21: no `on-node-kind:` events, procedural still depended on nine extensions). Regenerated graph-free with `native-session-refresh.ts` (same renderer as `playgroundSessionPublicationV1`; repo target `session-generation3d` not reachable: `Unknown admitted build target` on `semio-hub-procedural-component`, and the project owning it is not named in `repo show targets` output).
  2. Extensions are installed per variant into `.cache/.../runtime/react/generation3d/extensions/<dir>` (+ `📥️install.json`); the graph-free activation never did this. `native-extension-install.ts` (twin of `publishActivatedExtension`) installs brep and math.
  3. Still unmet: with lazy activation brep and math are deferred, and nothing requests them. `semioReactActivateOnDemand` answers `requested: []` for node-kind, event and tag requests and `not-installed` for `{kind:plugin}`, because the React shell's `plugins` prop is the boot-plan list (`bootFrameworkOsDev` returns only `order` of `expandPluginRegistry`), which excludes deferred rows, so the activator's `installed()`/candidates never contain them. And (per `📓️lazy-extension-activation-2026-11-23.md`, "Still open") the flow hub never emits the node-kind request. Boot rows come from the generated catalog TS (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts`), not from the session copy, so editing the session has no effect (tested and reverted).
- Needs a decision: (a) eager: `depends-on = ["flow","flow-extension-brep","flow-extension-math"]` in procedural's Cargo.toml and regenerate the catalog (`🔌️plugins.json`, `🧩️plugins/🟦️.ts`, `🦀️catalog.json`, `🗂️sources.json`, `🗿️artifacts/🦀️.rs`, then the session), or (b) finish lazy: pass deferred rows to the shell activator and emit the node-kind request from the flow hub (Rust, hub rebuild).
