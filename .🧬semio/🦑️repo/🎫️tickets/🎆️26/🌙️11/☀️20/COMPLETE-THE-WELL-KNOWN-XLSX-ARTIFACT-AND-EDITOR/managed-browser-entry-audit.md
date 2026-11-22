# Managed Browser Entry Audit

At the 2026-11-21 19:39 local fleet snapshot, full S host PID 26630, standalone host PID 26629 and Office browser publisher PID 27395 were active. The full S log was in the selected graph's file inventory, reporting 303,375 source observations and 1,246 completed owner checks around 2,007 seconds. The standalone and Office publisher logs explicitly waited behind the same checkout fact-store leader. Fleet reported no published graph generation. English, German and I/O watchers each remained waiting for host 26630; this is not a browser verdict. These are sampled observations, not a diagnosis of the leader's internal wait.

No supported narrower managed route that bypasses the shared graph and still supplies the current real Office product, activation and UI was established. The successfully published `playground-session:stdio-xlsx` is the standalone session and its catalog, not the shared Office browser module or React activation. The independently transpiled captured Office component passed Chromium execution, but has no current managed product receipt or activation and is not a UI journey. A warm serve with excluded dependencies or manually substituting that capture would not demonstrate the required managed provenance.

The supported existing commands remain the cold managed `@semio-tech/framework-os-dev:dev-s-react-prototype` and `dev-stdio-xlsx-react-prototype` targets. The ticket's `run-browser-io.ts` provides their host and readiness-gated acceptance invocations, detailed in [full-s-browser-audit.md](full-s-browser-audit.md). Following successful managed HTTP readiness, the English and German natural-file gates and the `stdio-office/xlsx` Hub I/O matrix must each produce their own terminal result; the standalone pane is a separate gate. No new host, build or browser test was launched in this audit.

## Narrow target re-evaluation

The earlier conclusion above did not account for the documented exact-owner `--exclude-dependencies` entry. It preserves the managed invocation envelope and can avoid dependency graph inference. It also skips all declared prerequisite tasks; it does not establish that their products are current. The following is a candidate execution sequence, not an executed or proven browser result:

```sh
repo --tool=semio run @semio-tech/os-plugin-native-transpile-rs:build --configuration=release --exclude-dependencies
repo --tool=semio run workspace:components --exclude-dependencies -- describe stdio-office --profile prototype --facet browser
```

The native tool target builds from an admitted successful Cargo capture and publishes the executable in its selected build mode. The workspace publisher declares the descriptor emitter, native transpiler, and browser support as prerequisites. The existing descriptor and shim receipts were observed previously; their current admission still needs verification. The publisher invokes the component driver itself and may rebuild Office; the independently captured Cargo bytes cannot be substituted as its managed receipt.

The narrower component targets inferred from the crate manifests are `semio-hub-stdio-component:component` and `semio-hub-stdio-office-component:component`. Their owner driver publishes the real component product, each declaring descriptor emitter, native transpiler, browser support, and `workspace:deps-wasm-opt` as prerequisites. Office declares `depends-on = ["stdio"]`; the standalone session names both components. These targets may be invoked individually with `--exclude-dependencies` once those four prerequisite products are admitted; the Office publisher above and Office component target are alternative publication routes, not two independent proofs.

The inferred standalone React DAG is session, browser support, `semio-os-infinite:fonts`, renderer and declared engines, and boot components; then `prepare-stdio-xlsx-react-prototype`, `activate-stdio-xlsx-react-prototype`, and `dev-stdio-xlsx-react-prototype`, all under `@semio-tech/framework-os-dev`. Preparation validates the session, staged component facts, shim and shard support, and font asset; activation independently reads the session and publishes the activation receipt. For individually admitted prerequisites the last three may be attempted in that order with `repo --tool=semio run <target> --exclude-dependencies`. The exact renderer/engine closure and current prerequisite receipts have not been independently established, so this is not yet a complete proven graph-free host launch. Excluding dependencies cannot substitute for obtaining those products. None of these candidate commands was run in this audit.

## Prototype tool check and exact managed owners

The live browser driver calls `ensureNativeTranspileV1` with the issued mode and reads the native tool using the component's profile. The native-transpile build writes its executable after checking completed Cargo provenance. It does not emit a separate tool product manifest. The release executable is relevant to release work. The current prototype directory already contains an executable. The prototype descriptor emitter completed `capabilities` with `{"version":1,"prepareActor":true}`, and its executable digest matched one non-cancelled, successful Cargo compiler artifact in the retained receipt.

Stopped the redundant release-tool preparation and started the exact prototype component owners together through the normal managed driver. The whole-S and standalone watchers remain tied to their existing managed hosts.

```sh
repo --tool=semio run-many --targets=component --projects=semio-hub-stdio-component,semio-hub-stdio-office-component --configuration=prototype --exclude-dependencies --parallel=1
```

## Prototype product snapshot

At 20:01 on 21 November, the prototype `stdio-xlsx` session manifest and all three files existed; the `s` session directory was absent. The Preview 2 shim and shard manifests and all listed files existed. The actual `infinite:fonts` managed product directory contained its manifest and the 8,757,072-byte font binary. The missing workspace font store receipt alone did not establish a missing font product. These existence checks do not prove current-source admission.

The repository WASM request contract hashes the source owner to the first 24 SHA-256 hex digits. Prototype product directories were absent for surface (`c31d3002bfa916304f57c297`), editor (`901f7777ac37d0b41cc143fe`), NodeGraph (`0eac234ec2c308c96333b196`), and Puzzle (`d30e56a055ea2d5bde08382c`) below `.cache/b/p/m/browser/prototype/wasm-packages`. No ready manifest could be validated there. React preparation declares the first three as renderer prerequisites. The S browser contribution expressly requests Puzzle too. Their producers are `@semio-tech/framework-surface-rs:wasm`, `@semio-tech/framework-editor-rs:wasm`, `@semio-tech/framework-os-node-graph-rs:wasm`, and `@semio-tech/puzzle-plugin:wasm`; each requires its own prerequisites to be admitted when excluding dependencies.

The S variant is declared by the Space component manifest. Its inferred session owner is `@semio-tech/plugin-registry:session-s`, and its boot selection starts from `semio-hub-space-component:component`. The full runtime closure was not independently resolved. At the snapshot only the Layout browser module manifest existed; managed Stdio and Office component run PID 13568 was active. Fleet also showed a renderer-labelled peer PID 94097, but its truncated phase did not identify any of the four exact WASM producers. All measurements are a point-in-time snapshot of the shared worktree.

## Full S boot and port selection

The generated S preparation target passes `space` to the runtime component closure with `appScoped: true`. The Space manifest has no component `depends-on`, `extends`, or `consumes` declarations. The runtime closure suppresses host-wide catalog fanout for that boot selection. Its exact boot closure is therefore `semio-hub-space-component:component` alone; that module was absent at the preceding snapshot. The full S session still covers the host catalog. Preparation and activation exclude unprepared non-host components but refuse a missing host; the XLSX journey also needs the independently managed Stdio and Office products.

The React `ServeScript` uses `S_OS_PORT`, falling back to the variant port, with a fixed listener. A distinct port avoids an HTTP listener collision. Preparation and activation nevertheless publish to the same variant, profile, and renderer runtime path regardless of port. A second standalone launch could race the already-owned standalone host when it leaves graph planning and writes that same activation; coordinate the host before another same-lane activation. The full S and standalone runtime paths are distinct.
