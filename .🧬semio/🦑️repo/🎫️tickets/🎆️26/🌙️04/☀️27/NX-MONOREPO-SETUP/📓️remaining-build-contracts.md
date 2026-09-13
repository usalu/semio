# Remaining Build Contracts

The historical artifact-check run after native target inference reported eleven missing output contracts: four report builds, Demonstrator build, Stdio build-wasm-release, Hub build, WGPU wasm, OS Dev build, Print build, and Print build-viz.

Initial inspection of the report/print commands confirms dynamic document and output path selection, font provisioning inside compilation, and custom source watchers. Report outputs default to each document directory/dist; arbitrary second arguments and SEMIO_PRINT_OUTPUT_DIR can redirect writes. Per-document Nx targets need fixed output ownership and separate font preparation before caching these operations. The generic default report build currently duplicates the Zwischenbericht producer and needs an explicit aggregate/default selection boundary.

Demonstrator build/dev calls buildPlugins, buildEngineWasm and registry preparation internally, then starts Vite. E2E starts its own raw script server and can reuse an existing port. It needs the same explicit session/support/component/engine preparation and single server ownership model as the React playground work, including a concrete deterministic build deliverable. No output declaration has been added over this hidden pipeline.

These were initial inspection findings; the checkpoints below and linked compiler/report notes record later changes. Demonstrator remains pending.

## Print Font Prerequisite — 2026-09-08

The three canonical print fonts are tracked TTF assets. The previous helper downloaded from floating main URLs when files were absent and copied to an unowned global cache using only file size. It now validates authored TTF sources and atomically stages `@semio-tech/print:fonts` deliverables into the Print package's `dist/fonts`. No source assets are downloaded or rewritten. A minimal script owns this Nx target, and Print/Report build and long-test targets declare font preparation explicitly. Hidden font preparation was removed from Report compilation and Print long-test implementations.

The language-neutral catalog is schema validated, and native `@napi-rs/canvas` successfully loads each staged font. The test verifies byte identity, stale-file removal and preservation after invalid source bytes. The red test failed on the absent staging API; the green probe passed. Permanent Print quick tests now include the same coverage. The actual Nx producer and restore checks are pending.

Current Bericht implementation builds all three reports when no selection is given, superseding the earlier duplicate-default observation. Its per-document ownership and the Print compiler/tool provisioning refactor remain unfinished.

The real font target completed in 229 ms of Nx task-run duration (100 ms producer critical path); an identical invocation reported 1/1 cache hits. Deleting the owned font deliverable tree and rerunning Nx restored every byte from cache, and the native font loader opened all three restored TTFs. `@semio-tech/print:test-quick` passed in 4.7 seconds, including permanent font tests. The registry generator regenerated `.vscode/launch.json` from the updated seed in 33.4 seconds; the Print font command is present. These are task-run durations and exclude earlier shared-graph waiting.

## PDF ownership checkpoint — 2026-09-08

Print now exposes 87 catalog-owned PDF leaves and aggregate-only template/gallery targets. Pinned compiler, pinned minimal TeX support bundle and tracked font producers are separate Nx prerequisites. Actual Print PDF byte repeatability, Nx clean-output restoration and independent PDF.js text consumption passed for viz-api before the latest generic publisher extraction. The full 87-document collection remains running (34 leaves had published at this checkpoint). See `📓️print-compiler.md`; final-source warm restoration remains pending.

Report now exposes three catalog-owned PDF leaves, an aggregate verifier, Nx-coordinated watches and a separate four-fragment generator contract. Its configuration and root watch vectors pass, and existing fragment bytes passed the renderer check. Full Nx runtime/cache/editor qualifications are running. See `📓️report-documents.md`. The remaining non-PDF build owners listed above still need their hidden pipelines refactored.

## Qualified PDF and Dependency Checkpoint

The first 87-document Print build passed (174 PDFs); stronger all-page PDF.js consumers are running after the final compiler API extraction. Report native repeat, clean cache restoration and PDF.js consumption passed for all three documents (403 pages total). Read-only actor-network preview matches all four tracked outputs. These checkpoints predate the newest scoped external-dependency inputs and workspace-root precedence fix; the Report cache probe is running again and Print's final warm qualification remains outstanding.

The repository now supplies Nx with a location-sensitive Bun dependency graph, including nested packages, peer contexts and authored patch bytes. Eleven isolated daemon-backed cache scenarios and native Nx hash-plan/Bun resolver vectors passed. Cargo's generic command hashes its TOML parser dependency instead of all JavaScript lock entries. Eight native preparation/cache scenarios and twelve native source/restoration/linked-consumer scenarios passed with those inputs. See `📓️external-dependencies.md` for shared graph checks, precise timings and the final helper test still running. Other build owners, lifecycle leases and general storage retention remain pending.

## Current Non-PDF Checkpoint — 2026-09-08

Demonstrator now has explicit seven-session React preparation, production Vite artifact staging, development activation, and invocation-specific E2E preparation/serve/consumer targets. Generic native Nx + Vite cancellation, HTTP readiness and real HMR tests pass; full product qualification remains pending. The release attempt failed after 42m44s of task execution: styling rejected a then-current font catalog identity, and Flow compilation could not resolve newly moved Flow/Playbook artifact crates. Current files now contain the expected font catalog identity and those dependencies; these concurrent repairs need a fresh runtime retry. No actual site bundle or product cache restoration has been claimed.

The duplicate Stdio `build-wasm-release` target and its raw `cargo rustc` command were removed. The existing inferred `@semio-tech/stdio-plugin:component-release` target is the canonical WASI component producer, with exclusive profile output `dist/component-release`; `materialize-release` owns the browser projection. Both canonical commands were already present in generated editor launchers. Repository source/editor searches found no callers of the removed entry point beyond its own declaration and router. `COMPONENT_PROFILE` remains used by catalog-root validation. This removes one stale output-contract violation; catalog-root/describe orchestration remains part of the broader follow-up.

The repo/registry rerun encountered a discovered source disappearing before graph dependency reading. The shared plugin was concurrently updated with an ENOENT guard after that failure; this task did not overwrite that fix. A new native graph/test run is in progress.

## Restored Test Discovery — 2026-09-08 19:40

The post-restart registry run discovered all taxonomy-named files and exposed stale relative fixture paths in three suites, a stale WASI schema filename, and accidental collection of the Storybook Playwright suite by Vitest. The Vitest owner now selects its four suites, while the existing Storybook Playwright owner retains its suite. Three malformed product-path segments in the Storybook config were repaired to existing source paths.

Registry fixture reads now resolve from their actual owner; schema checks resolve named definitions in the current activation, WASI-profile, extension-directory and plugin-identity schema modules. The deployment catalog reference now points at the current extension definition, avoiding an unresolved removed schema ID. The 19:38 native Nx run passed all 25 tests in four registry suites (Vitest duration 29.64 seconds under concurrent Print and Flow compilation). This run explicitly used the long test level; the default level's wall-time gate is not claimed qualified by that result. The encompassing `repo:test` run remains active.

A read of pinned Nx 23.2.0 shows that native task-result retrieval can restore remote outputs before `finalizeCacheHits`, and local restoration occurs before lifecycle `preRunSteps` in bulk cache resolution. An executor-only lease is therefore insufficient. The full mutation/reader lease must span retrieval, restoration, native execution and cache capture; installation must additionally protect dependency reads during graph construction. Current SQLite service-session leases do not cover those boundaries. No extra Nx patch has yet been applied for that integration.

## Nx Bootstrap Isolation — 2026-09-08

The full 19:38 repository/registry run passed (3m14s, both test targets executed). A subsequent launch then failed before Nx started because eager root-script taxonomy validation observed a concurrently inconsistent unrelated application contract. A language-neutral native fixture reproduced the architectural coupling: the public `nx --version` alias executed a deliberately throwing application root script, while the pinned native Nx CLI succeeded.

The existing coordinator and selection resolver have now moved into a dedicated `⚡️caching/🚀️bootstrap/📜️script.ts`. The public Bun `nx` alias selects that bootstrap; the obsolete root `nx` command registration was removed. The shared environment function now lives under process infrastructure and is reused by the library barrel. Ordinary Nx startup imports only system APIs and lightweight process/workspace infrastructure; application selection helpers are loaded only for commands requesting those selections. The poisoned-source native fixture passed against the pinned Nx CLI after this refactor. Coordinator cancellation, all selection fixtures and the full current-source repository suite still require repetition after the move. Installation/resource leasing is not yet added to this bootstrap.

The native live `bun nx --version` check passed with local Nx 23.2.0 after bootstrap extraction. Five remaining direct calls to the removed root `nx` command were changed to the public Bun alias (three Hub commands and two repository test modules). Existing Hub nested orchestration remains tracked; this change repairs the entry path and does not claim to flatten those pipelines.

Flow release compilation completed with errors after 12m54s: 41 compiler diagnostics were missing imported Flow/DAG symbols following source relocation (30 uses of `widget_id_for`, plus store/mutation/schema/port types). Current bridge/host source already contains explicit imports added by a concurrent worker; no duplicate repair was applied. A focused retry is running against those current sources.

The new bootstrap's four cancellation fallback vectors passed after updating the old ticket probe's process mock to include its environment. The independent esbuild module graph contains exactly five eagerly loaded repository infrastructure files and only system imports; native version parity passed again. The full repository run then reached its editor contract and caught a newly added schema-registry project without launcher entries. Six build/check/test-level commands were added to the seed in the existing native group/order. This was unrelated to coordinator behavior.

The native watcher probe initially lacked the newly required taxonomy input for the current Nx metadata plugin. After including that fixture input, the real Nx watcher/server proof passed: transitive source rebuild, changed HTTP output, ignored generated report, parent exit 143, stopped descendants, released port and preserved shared daemon. The next full repo/registry run is active.

Flow's current-source retry exited 1 after 2m41s with five compilation errors in the OS kernel's Store implementation. These are separate from the repaired Flow imports. Product build/cache qualification remains incomplete.

The native coordinator rerun is complete and green. A fresh Bun installation/frozen-lockfile proof of the pinned Nx patch is now running in a private ticket fixture, including the newly added fourth real-Vite service scenario. Shared workspace dependencies are not being reinstalled. Current Store source has also changed since the five-error compiler run; its SpaceHost backbone operations now delegate to `self.meta`, so Flow qualification has been retried against current source rather than applying an overlapping edit.

The private fresh-install proof passed with both initial and frozen Bun installation, daemon environment/diagnostics/retention checks, all ten graph-coalescing cases and all four native continuous-service cases (including real Vite). Its fixture is `daemon-patch-qxQZJd`; the shared workspace installation was untouched. The full monorepo rerun remains active and has reached current editor/project contracts.

The fresh fixture’s patch SHA-256 is `f8818dda6dcb53f37a84edcfaeaae9647e27ebfb9cd62410e4d63cb0fb522832`; all 5 patched module files match the current shared installation byte-for-byte.

The full bootstrap-era repository/registry run passed (9m22s, 447 inventoried projects; both uncached test targets executed). The isolated bootstrap fixture now runs a copy of only its five declared infrastructure source files, with poisoned application sources present; it also passed. Watch-triggered commands are being routed through the same public `bun nx` bootstrap so environment/selection/lifecycle behavior has one entry point. A focused real watcher/server regression is running for that last change.

The final public `bun nx` watch callback passed the native watcher/server regression (`nx-bootstrap-public-watch.log`, exit 0). Flow retry `flow-release-current-store.log` terminated with exit 143 after 14m12s; no compiler failure is established by that result and this task did not request its termination. Another release retry is deferred while local free space has fallen to 12 GiB and the Print collection is still active.

## Hub Build Ownership

The current audit reports three remaining missing output declarations: Hub build, WGPU wasm and OS Dev build. Hub build invoked `os-hub-admin:build` inside its script despite already declaring that Nx prerequisite, then compiled an unstaged release binary. The target now directly selects the existing generic Cargo artifact producer with a fixed release/binary selection and exclusive `dist/build` output. The duplicate Hub build class/router entry was removed; the outer admin prerequisite remains. This also avoids eagerly importing the large Hub application/test script merely to build its binary. Other Hub dev/test nested pipelines remain follow-up work.

The language-neutral contract failed first on the absent output declaration. It now passes JSON Schema validation, native Nx task-graph construction (one admin dependency and one native producer), and independent esbuild import-graph verification. `os-hub:build` is running with two Cargo workers; product binary execution, warm caching and deleted-output restoration remain unqualified.

The first real Hub graph failed in its admin prerequisite after 5m01s (native binary producer did not run). Vite traversed test-only dynamic imports guarded by `import.meta.vitest`, eventually rejecting Node `path.resolve` in a browser bundle. The admin Vite config now defines that guard as `undefined`, matching the existing OS frontend config. A language-neutral browser probe failed on `node:fs` with the old config and passes with the corrected config through esbuild, including execution of the resulting module. The full Hub graph is running again in `hub-build-browser-filter.log`.

## 2026-09-08 interrupted validation state

The ticket generated directory was removed by another operation while Print, repo tests and editor generation were running. This task did not perform that deletion. The Print run subsequently failed several leaves with missing `semio-viz-chart-biology-ecology.sty` and `semio-viz-chart-engineering.sty` inputs. Its earlier log was gone; new output recreated the log. At 21:27 local time the positively identified own Print collection parent PID 4442 was sent SIGTERM to stop this already invalid collection run. It had not reached output-backup/restoration. No new collection pass, warm-hit result or restoration result is claimed. Previously recorded individual Print/report proofs remain historical evidence only. The native Hub retry exited 1 after its admin prerequisite succeeded, but its failure log was removed before error inspection.

## OS Dev production build inspection (2026-09-08, after bootstrap qualification)

Native `bun nx show project @semio-tech/framework-os-dev --json` exited 0. Its effective build target remains cache:false, has no outputs and depends only on assets:build. BuildScript at the package script line 1482 calls PluginBuildScript, optionally a nested WGPU wasm script, buildEngineWasm and Vite. This is still hidden cross-project scheduling. The generated prepare-<variant>-react-<profile> graph already names session, browser support, fonts, engine wasm and the full component materialization closure. No OS Dev build source has been changed in this inspection.

The Vite config unconditionally reads a development activation receipt, uses its plugin identities to select copied modules and copies the mutable installed-extension directory. Merely enabling caching or adding an output declaration would therefore preserve undeclared local runtime state. A production build needs a deterministic descriptor assembled from prepared source-owned components and extensions, then an explicitly owned per-variant distribution output. Development activation should continue to own its mutable runtime state. The existing release build also sends all trailing positional/CLI arguments to Vite; fixture/brand/base selection must be made explicit before caching. WGPU selection must remain an outer Nx target, not a nested wasm script.

The raw/effective cwd display initially appeared escaped, but native filesystem samefile and realpath checks confirmed that configured and canonical paths identify the same directory. No cwd repair was made. Existing distribution generation has its own layout, input witnesses and output publication helpers; inspect and reuse its applicable ownership rules rather than introducing another untracked distribution.

## Production browser ownership implementation

Added a schema-owned production component plan and Vite copier using the existing strict browser artifact ownership markers. A language-neutral fixture initially failed for the missing implementation, then for an incorrect fixture extension route; the fixture was corrected to the hand-authored deployment authority. Native Vite emitted five owned source families, omitted ambient installation metadata, and real JavaScript consumers executed both relocated plugin and extension imports. write:false left existing bytes unchanged and created no new output tree. Actual production Vite configuration resolved the prepared note session without reading a development activation receipt.

The production config is now an asynchronous factory, with mutable activation/store plugins and extension installation roots restricted to serve mode. The existing in-memory distribution compiler was updated to invoke that factory with build mode. Generated release build targets have unique per-variant outputs and explicit session/support/font/engine/component/assets prerequisites; the graphlib/Cargo oracle passed all 120 preparation targets and checked every release build closure. Default public build/WGPU routing, full real product compilation, cache restoration, input qualification and editor regeneration remain in progress.

### Native restoration and environment input proof

The production copier now reuses the existing browserArtifactVitePlugin writeBundle hook. The native Nx fixture first exposed an invalid test assumption: ordinary filesets did not track modified materialized files under dist. The fixture was corrected to model a real upstream producer, its owned outputs and dependentTasksOutputFiles, matching the production graph. Warm replay and deleted-output restoration then preserved the complete file inventory byte-for-byte with one Vite invocation. Editing a producer source byte reran both preparation and Vite. Restored plugin and extension imports executed successfully. Earlier fixture failures for the Nx CLI location, unnamed root package and Bun-only import.meta.dir in a bundled Vite config were corrected; pinned Nx package metadata and standard import.meta.url are used.

The browser input contract now hashes every exposed VITE_ environment value, the configured Hub/data/GIS variables, and all four production dotenv files including ignored local files. The Node hash matched an independent Bun CryptoHasher result; a VITE value or dotenv byte change changed the hash, while an unrelated test selection did not. The public executor input command printed only its hash. The production executor esbuild graph contains eleven inputs and no application/test command imports; explicit build variant routing sets release/react/variant before Nx runs. Native Nx reports sixty unique release build targets and the generated editor has sixty build commands; the later build-inputs editor command still needs regeneration.

The real Note build is active through its outer seventeen prerequisites. Assets emitted 286 deterministic files, browser support and registry/session generation succeeded, and Flow/native compilation is running. This is not yet a completed product build or product cache restoration result. Default generic build and WGPU paths still need replacement; new explicit release targets do not yet forward SEMIO_LOCKED_* preferences, while VITE_* values are captured by the new input contract.

### Preference forwarding preserved

The former library preference function and constants were moved to a pure playground/preferences module with one mapping authority. Existing library exports remain normal barrel exports of that implementation. The new browser executor forwards those preferences and the input hasher hashes the resulting effective VITE values. The schema fixture and independent lodash transform passed; changing a locked locale changes the input hash, and a trimmed SEMIO_LOCKED_LOCALE equals its effective VITE value. The executor boundary now has thirteen inputs and still imports no application/test orchestration. The actual note production config still resolves successfully without an activation receipt. The earlier limitation about missing SEMIO_LOCKED_* forwarding is resolved.

### Production selection, editor and native optimizer qualification — 2026-09-08 21:35 UTC

Production selection now rejects missing/malformed sessions, mismatched variants or registry identities, duplicate/unknown plugins and incorrect role-specific module URLs. Its native production configuration, Vite copy, dependency-change, warm replay and deleted-output restoration probes passed. Editor regeneration completed with all sixty release build commands and the build-inputs command.

The real Note prerequisite run compiled Flow successfully in the wasm-release profile (22m21s), then failed in wasm-pack 0.15.0's Binaryen optimization. Its default custom-profile arguments were only `-O`; the generated module requires bulk-memory and nontrapping float-to-int instructions. Flow's Cargo metadata now explicitly enables those two features for custom and release profiles while retaining optimization. The language-neutral WAT/JSON regression initially failed against missing metadata, then passed native Binaryen optimization and WebAssembly execution for overlapping memory copies and saturating conversions; Bun and smol-toml parsed the manifest identically. A real-binary optimizer probe is still running. The containing Note graph continues compiling its font tool. No product cache restoration has yet been established.

Pinned upstream evidence: [wasm-pack 0.15.0 manifest profiles](https://github.com/wasm-bindgen/wasm-pack/blob/v0.15.0/src/manifest/mod.rs) defines a separate `custom` profile, and [its optimizer runner](https://github.com/wasm-bindgen/wasm-pack/blob/v0.15.0/src/wasm_opt.rs) forwards the configured flags.

The repository suite exited 1 at the editor contract: inferred `@semio-tech/framework-os-config:build` lacked a launcher although check/test were present. The missing build entry was added to the native build group and editor regeneration passed. A fresh full repository suite is active. Earlier native daemon, service, lease, browser and preference assertions passed during that failed suite; it was not a complete suite pass.

The generator-input command's invalid-command startup probe exited as expected after 1.93s. This does not support attributing the previous two-minute generation stages primarily to module import cost. The later editor run measured generator-inputs 1m21s and generation 32.6s under concurrent compilation. No speculative import refactor was made.

### Completed contract suite and remaining runtime work — 2026-09-08 21:48 UTC

The full `repo:test` run exited 0: 449-project inventory, editor/playground checks, lifecycle/compiler assertions, native source-discovery oracles and materializer process-tree cancellation all passed (task time 8m13s). The small optimizer regression passed for bulk-memory and saturating conversions. The large Node/Binaryen probe and Flow Nx retry remain active. The duplicate Bun optimization probe was positively identified by PID/command and terminated; its one-second process sample contained JIT frames but does not explain its elapsed time. No optimizer performance conclusion is established.

The original Note graph completed font-tool compilation (22m26s) and staged all seventeen fonts (8,757,072 bytes). It is now continuing Surface prerequisites. Flow's retry is compiling currently changed shared crates. Neither complete Note distribution nor actual-product cache restoration has passed.

The OS Dev package's default build now forwards to `build-s-react-release`. Plain root build aliases for Aggregator, Forms, Raster, Draw, Puzzle 5D and Generation 2D forward to their explicit release targets; the corresponding plain editor build commands were updated in the seed. A new editor generation is active. Fixture-specific aliases and the generic legacy implementation remain to be migrated together with WGPU. These command-only changes do not establish that all products compile.

Dependency installation/read leases still require integration with Nx graph loading and its task/cache lifecycle. An installer-only or raw-command-name-only lease would miss dependent or batched installation tasks and cache restoration; neither incomplete guard was installed. The container lifecycle audit remains source-only: direct `bun install`, raw Go compilation and attach-time conditional rebuilds are still present. No container lifecycle script was changed or executed.

### Native optimizer qualification and explicit tool selection

The official Binaryen version_130 arm64 macOS archive was downloaded only into the ticket and verified against SHA-256 `79d3ab9f417d9e215f15f598f523d001a7d9ac1e59367e5c869fbdabd1cba72e`. Native wasm-opt reports version 130 and links `@rpath/libbinaryen.dylib`; a persisted native tool installation must retain that library beside its bin directory. It optimized the existing 9,212,081-byte Flow WASM into 8,550,906 bytes in 76.657 seconds, and WebAssembly.validate accepted the output. The output SHA-256 is `5dff117842231f330afaa235c6a7bc0286d08377dab203061cf3f3f98122e32c`. This validates the two optimizer feature flags on the real binary. It does not complete the newer Flow source build or the Note distribution. The JavaScript Node optimizer comparison was still running after fifteen minutes; no completed comparative speedup ratio is claimed.

Upstream asset metadata was read from the [Binaryen 130 release](https://github.com/WebAssembly/binaryen/releases/tag/version_130), published 2026-06-01. Six native archives are available: macOS arm64/x86_64, Linux aarch64/x86_64, and Windows arm64/x86_64. No permanent native acquisition target has been added yet.

The wasm-pack environment helper now honors the existing `SEMIO_WASM_OPT_BIN` override by placing its directory ahead of ambient tools while preserving the verified binding-generator directory. It validates the executable name and fails if a different executable shadows the selected optimizer. RED: the new language-neutral tool-directory fixture could not call the missing helper. GREEN: the fixture passed native Bun/system-which lookup parity, invalid-name and shadowing rejection, plus the existing optimizer execution tests. This was executed on macOS; Windows-specific lookup has not been run. The helper is used by runWasmPackWebBuild, but the currently running Flow retry loaded the previous environment helper and needs a subsequent native-override invocation after its compilation finishes.

The seven plain/default package build entrypoints were checked against the generated playground catalog and editor commands. Editor generation exited 0; the native os-config build launcher occurs exactly once.

A tail of the Flow compiler log initially appeared to be an obsolete namespace error. Full diagnostic inspection showed an unnecessary-qualification warning in Infinite DAG code. No source edit was applied; Flow compilation continues.

The JavaScript Node optimizer comparison exited 0. Its complete output is byte-identical to native Binaryen: 8,550,906 bytes with SHA-256 `5dff117842231f330afaa235c6a7bc0286d08377dab203061cf3f3f98122e32c`. Flow's new source compilation then completed successfully in 17m05s and entered JavaScript optimization. Its owned Nx bootstrap PID 55808 was verified and terminated (exit 143) after compilation, preserving the Cargo artifacts. A new Flow Nx invocation now selects the verified native optimizer through SEMIO_WASM_OPT_BIN. The original Note graph is separate and remains active.

### Explicit Native Optimizer Prerequisite

Actor and Puzzle choose the custom wasm-release profile but only declared dev/release optimizer policies. The regression failed on their missing custom policy, then passed after custom was explicitly set to false, matching their existing release intent. Surface and Editor already declare custom optimizer feature flags and were preserved. The fixture is parsed independently by Bun and smol-toml.

The next implementation isolates pinned native Binaryen 130 under the tools store, with an uncached Nx preparation target, checksum-verified downloads, retained runtime libraries, atomic publication and cancellation. Build tasks will consume the prepared tool through a graph dependency; hashing will use the pinned tool identity without downloading. Native qualification remains macOS arm64 until other platform executions occur.

Flow native-optimizer retry exited 0 through Nx: Rust compilation 13m07s, wasm-pack completion 14m01s, task 14m10s. It selected the exact ticket-native wasm-opt path and produced an 8.15 MiB binding binary. Its trailing stack was a FORCE_COLOR/NO_COLOR warning, not an assertion failure. Surface's original graph task also completed (29m23s wasm-pack, 6.69 MiB); Editor is now compiling. The original Note graph still carries the earlier Flow failure and has not completed.

The generalized tool process owner passed the existing native cancellation test, including killing its owned process tree. Native Binaryen manifest/selection and uncached target contract passed after observed missing-module and missing-target failures. Automatic preparation is now being exercised through workspace:deps-wasm-opt; build consumption has not yet been changed.

Native Binaryen preparation through Nx exited 0 in 3.7 seconds (3.4-second task). The isolated native Nx contract then passed: cold optimization, identical warm reuse, deleted output restoration, deletion/reacquisition of the private tool installation without rebuilding the cached output, and input-byte invalidation. Four preparation executions produced only one optimization until the source changed; the changed source produced a second optimization. The restored WAT fixture output is valid WebAssembly with SHA-256 `811e49dd3ec0fbc860b9727373b83ace1ea8fee40fa85917a48984bbc347959d`; system shasum independently matched Node's prepared-tool digest. The prepared distribution retains only wasm-opt and required dynamic libraries. Native Linux/Windows execution remains unqualified.

The first full-suite rerun failed immediately because the new test read the current package project instead of the explicit workspace parameter. That test root assumption was fixed and a full rerun is active. This does not invalidate the separately completed native Nx contract.

All six official Binaryen archives were downloaded, checked against the pinned byte counts/SHA-256 values and inspected by both system tar and Python tarfile. Selection parity passed for macOS arm64/x64, Linux arm64/x64 and Windows arm64/x64. The macOS packages require libbinaryen.dylib; the selected Linux and Windows packages contain standalone optimizer files with no archive-shipped dynamic library. The probe removed each downloaded archive after its verification. This establishes archive contents, not native Linux/Windows executability. Editor regeneration exited 0 and the seed/generated JSONC each register workspace:deps-wasm-opt exactly once.

Remaining fixture browser aliases still invoke the generic build with positional `fixture` arguments. The current generic BuildScript forwards these directly to Vite; no matching fixture-selection logic was found in that script, its Vite configuration, bootstrap invocation mapping or shared playground preference helper. No fixture aliases were changed in this pass; their intended runtime selection requires a verified contract before replacement.

Full repository suite after optimizer integration exited 0 in 3m12s: native command contracts, 449-project inventory, editor/playground, lifecycle/compiler, source-byte discovery and cancellation passed. Native Nx show-project independently confirms Flow wasm depends on workspace:deps-wasm-opt. Docker CLI is installed but its daemon socket is absent; no Linux container execution was claimed.

The original Note graph subsequently hit Puzzle 3D compile errors: retirement helper fixed capacity 32 disagreed with document weight capacity 256, and the browser bridge referenced undeclared js-sys types. Current source confirmed both failures. The retirement helper now accepts its map capacity as a const generic. The wasm-only bridge declares the existing js-sys version requirement through workspace metadata and explicitly reexports its Promise/Uint8Array surface; Cargo.lock records the added existing-package edge. A Puzzle wasm Nx retry is active. Puzzle is a declared linked browser session factory imported by the shared OS Dev entry, so its prerequisite was retained. No Puzzle runtime pass is claimed yet.

### Component Optimizer Ownership and Browser Build Outcome

The original Note graph ended after 129m04s with exit 130, reporting the earlier Flow and Puzzle wasm failures; its Vite target did not run. The Note component materializer itself finished and staged its bridge/descriptor. Native process inspection during its 18-minute optimization showed an explicitly selected Bun/JavaScript Binaryen process. That hardcoded selection has now been removed. Release materializers depend on workspace:deps-wasm-opt and execute the prepared native optimizer directly; both synchronous/asynchronous component optimizers honor explicit selection, and asynchronous child execution emits progress. The optimizer's JSON pin is now an import so bundled config consumers do not depend on its source-relative file location.

RED: the component optimization fixture could not call its missing exported test boundary. GREEN: native optimization changed only owned core files, preserved dev/foreign outputs and matched JavaScript Binaryen output bytes plus executed WASM results. The test is registered in repo:test, whose Nx target now declares the required optimizer preparation. The updated full suite is active. A new Note materialize-release Nx invocation will qualify the native path on the real component.

Puzzle's retry exited 1 after 15m17s. Its earlier weight-map capacity and js-sys errors were absent; it instead reported current 2D/5D compilation failures. The largest error groups are recorded below; no overall Puzzle pass is claimed.
- 88 × `error[E0433]: cannot find module or crate `infinite_canvas` in this scope`
- 3 × `error[E0425]: cannot find value `view_state` in this scope`
- 2 × `error[E0433]: cannot find `replace_fastener_semio_framework_geometry` in `mutations``
- 2 × `error[E0050]: method `context_menu` has 4 parameters but the declaration in trait `semio_framework_plugin::ArtifactEditor::context_menu` has 5`
- 2 × `error[E0308]: mismatched types`
- 2 × `error[E0433]: cannot find module or crate `js_sys` in this scope`
- 1 × `error[E0432]: unresolved import `super::replace_fastener_semio_framework_geometry``
- 1 × `error[E0432]: unresolved import `super::replace_part_2d_semio_framework_geometry``

## Materializer Prerequisite and Actor Export Retry

The release-materializer graph expectation was updated to include the explicit native optimizer dependency. The subsequent complete repo:test invocation passed (4m20s overall, 4m18s test task), including native component optimizer, graph, lifecycle and cancellation contracts.

The actual Note materialize-release run exited 130 after 12m08s because its component-release dependency failed: three actor-export macro expressions referenced ::dsl in the caller’s crate, where no such alias exists. The materializer did not execute. Current macro source confirmed all three references. The plugin package now explicitly reexports encode_fault_bytes from its existing framework kernel dependency, and the three macro expressions use the hygienic $crate export. A new actual Note materialize-release run is active; no real Note native-optimizer success is claimed yet. This task sent no cancellation signal to the failed run.

## Current Native Materializer and Inventory

After the macro hygiene correction, Note component-release compiled successfully in the wasm-release profile (11m24s) and staged its component. The materializer then completed native wasm-opt, emitting three ten-second progress messages, and reached descriptor execution. The descriptor failed on a current framework invariant: setActiveUtility was unclassified in note-composite, note-navigator and the Note editor app. This is an executed descriptor failure after optimization, not a materializer success or cache-restoration proof. The full invocation exited 1 in 12m17s.

The refreshed repo:audit exited 0 in 26.4 seconds and recorded 449 projects, 4303 commands, 890 artifacts and two unresolved static output-contract findings: framework-renderer-wgpu:wasm and framework-os-dev:build. That audit’s zero exit is report generation, not policy-check success. Hidden work, leases/retention, CI lanes, container lifecycle and platform/runtime qualification remain beyond these two static findings.

## Utility Classification Isolation

A private probe transpiled the exact staged Note component without optimization and ran the same Node/JSPI descriptor probe. It reproduced the same three setActiveUtility classification failures, so this failure is present before native Binaryen optimization. The input component SHA-256 was 9e8747e66296e445511cf95e9dddbe1de916ee23de2d11802fad1c22eba40765. The first probe had an incorrect ticket output parent and the second had an incomplete vendor path; both were corrected before the valid unoptimized result. An optimized comparison of the same bytes is still active.

Current Note source explicitly declared its own unclassified setActiveUtility action. That declaration prevented the framework from injecting its already-classified canonical utility action. The redundant declaration and its now-unused constant import were removed; Note still declares its utilities, so the framework owns this action and its typed arguments. A new Note materialize-release graph is active. The framework classifier was retained without exemptions.

## Paired Descriptor Result and Current Compiler Frontier

Both original and natively optimized versions of the same staged Note component exited 1 with the identical three utility-classification failures. The comparison is complete; neither result is a runtime success. Removing Note’s duplicate action preserves framework classification and remains to be qualified in a new component.

The utility-fix retry exited 130 after 1m21s because a concurrently introduced window-transient module had three incorrect crate-root references and three imports through private app aliases. The module is a sibling of app under component, so references now use super::window_transient and its fault types come directly from semio_framework. Another Note materializer retry is active. No signal was sent by this task.

## Isolated Tool Fingerprint

WASM tool hashing previously imported the entire caching command and test module tree. The fingerprint now lives beside native WASM tool acquisition, with the policy/runtime target pointing directly to that small script. The old caching-script class/router entry was removed. It reads immutable Binaryen identity without acquisition; other installed tools are queried asynchronously with a ten-second per-tool deadline and cancellation. Captured process progress goes to stderr so successful JSON stdout stays machine-readable.

RED: the new isolated fingerprint fixture rejected the missing command. GREEN: the command worked with an empty PATH, did not acquire a tool store, matched a native optimizer version override, rejected installer arguments, and passed its esbuild import-boundary assertion. The existing repo:toolchain editor command remains valid. Native Nx execution and the complete suite are active.

## Consolidated Validation at 23:53 UTC

The complete repo:test suite containing CI, container-context and isolated WASM-fingerprint contracts exited 0 in 2m30s (2m28s test task). Native repo:toolchain exited 0 in 891 ms and emitted valid installed-tool/pinned-Binaryen identity JSON. This remains local macOS arm64 validation.

The window-transient retry ended with exit 130 in 16.2 seconds because Cargo.lock required an update while --locked was enforced. It did not reach compilation, so the utility and transient fixes remain unqualified by that retry. No cancellation signal was sent.

## Successful Note Materializer and Subsequent Source Drift

The lock retry exited 0 through Nx in 13m50s. Note component-release compiled in 12m28s (12m29s target); native wasm-opt completed with progress and the real descriptor probe succeeded. The browser bridge and descriptor were staged. This qualifies the earlier actor macro, utility-action and window-transient fixes on macOS arm64.

The subsequent repeat could not establish warm-cache behavior: new UI/framework source changes triggered recompilation, which failed on duplicate ConfigView.window fields in the current plugin source. The run exited 130 after a 2m38s component task. No cancellation signal was sent. The completed preceding materialization remains a successful historical result; it is not a warm-cache or deleted-output restoration proof.

## Command Import Closure and Artifact Router Isolation

The container-bootstrap full suite failed on a type-only generated-playground import included by the command scanner but erased by esbuild. The scanner now uses TypeScript AST nodes, excludes erased import/type-query/declaration references, retains literal dynamic imports and createRequire factories, and refreshes cached parsing on source bytes. A language-neutral fixture agrees with esbuild and native Node execution before/after a runtime dependency change. One named type export still requires compiler module resolution even though esbuild removes its runtime side effect; the fixture records that observed distinction. Initial fixture path and export-side-effect assumptions failed and were corrected.

The next full suite passed that import comparison but failed the unchanged native-Cargo maximum-five-input boundary: newly added aggregate artifact test routing imported 31 files, including application/test modules. Rust and TypeScript artifact routers are now separate entry points under caching/📦️artifacts, and 141 package imports point directly to their respective owner. The native Cargo script again has exactly five inputs matching esbuild (861 ms combined scan/bundle probe). The native core reexports no compatibility artifact router. The complete suite and an actual PDF TypeScript package build are active.

The PDF TypeScript artifact package built through Nx with the separated router in 6.4s (5.7s target), then repeated with a 1/1 local cache hit in 360ms. No output deletion was performed. The full suite subsequently found one test-source witness still reading the old Rust artifact router location; that read was updated to the new owner and the retry is active.

## Complete Suite After Router Separation

The complete repo:test retry exited 0 in 3m49s (3m48s test task), including the corrected artifact-router source witness, native fake-Cargo level/filter/progress checks, runtime command import fixture and compiler boundaries, container runtime/creation hook, native tooling, restoration, 449-project graph, lifecycle and cancellation checks. The compiler entry remains isolated to five runtime inputs. Only unused system-library import names were trimmed after its earlier import-boundary probe; no command behavior changed in that trim.

## Trunk Lockfile Configuration

The native Trunk 0.21.14 config inspector reported build.locked=false. The WGPU Trunk configuration now sets locked=true. RED: the declarative fixture rejected the missing property. GREEN: Bun and smol-toml parsed the same configuration and the native Trunk inspector resolved locked=true. The native inspector required removing ambient NO_COLOR/FORCE_COLOR because this installed Trunk rejects NO_COLOR=1; the production Trunk process environment already removes both. No WGPU compilation was performed for this configuration check. The parser fixture is registered after the preceding full-suite pass and has passed separately.

The pinned [Trunk build configuration model](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/config/models/build.rs) defines locked and its former false default. The historical Trunk documentation domain did not return Trunk documentation and was not used.

The refreshed audit exited 0 in 33.6s and recorded 449 projects, 4307 commands, 890 artifacts and the same two unresolved static output findings. Its successful exit means the inventory was generated, not that those findings are closed.

## Native Trunk Lockfile Probe Reveals an Earlier Rewrite

The private stale-lock fixture ran native Trunk 0.21.14 with locked=true. Trunk nevertheless rewrote Cargo.lock during metadata discovery, compiled the tiny WASM crate in 1.29s, and exited 0. The probe correctly failed because it expected rejection with unchanged lock bytes. The configuration-only PASS above does not establish lockfile preservation. This ran entirely under the ticket generated directory and did not compile WGPU. The pinned [Rust pipeline source](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/pipelines/rust/mod.rs) requests Cargo metadata before applying the locked flag to its later build calls. The earlier metadata boundary needs correction.

## Locked Metadata Before Every Trunk Pipeline

Trunk now invokes the canonical Cargo script as a pre_build hook with locked metadata validation; the metadata JSON is discarded while errors and bounded progress remain visible. Trunk still receives locked=true for compilation. The same uncached operation is available through the WGPU check-lockfile target and its editor entry. There is no nested Nx invocation or dependency installation in the hook. The [pinned HTML pipeline](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/pipelines/html.rs) awaits pre-build hooks before creating Rust pipelines; the [metadata implementation](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/config/manifest.rs) explains the earlier unguarded call.

RED: the declarative contract rejected the missing hook. GREEN: Bun/smol-toml and native Trunk configuration agreed. The first native watch test failed because ambient RUST_LOG=warn suppressed the success marker; its fixture compiled in 0.93s and produced files, but that attempt did not establish the later rebuild check. The retry explicitly selected info logging and the actual pinned success message, then passed: an initially stale lock was rejected without rewriting bytes; a matching lock was accepted by native Cargo metadata; native Trunk watch compiled the fixture; corrupting the lock and editing its source rejected the next rebuild while preserving both stale lock bytes and the previous deliverable digest. The owned watcher process group was stopped and awaited by the test. This qualification is local macOS arm64 and uses a private tiny WASM crate; it is not a WGPU application build or a source-snapshot guarantee for simultaneous edits during compilation.

## Native WGPU Metadata and Completed Contract Run

The actual WGPU locked-metadata target exited 0 in 22.8s. It also ran four inferred source-generator prerequisites, revealing unwanted coupling from its initial check-lockfile name. The target is now named lockfile-check, consistent with other metadata validation commands. A graph-level regression fixture first failed on the unwanted fixture:generate edge; the earlier attempt failed to import the asynchronous plugin through require and was corrected to await import. The new graph test awaits qualification after editor regeneration.

The complete repo:test suite exited 0 in 6m37s (6m35s test task). The first invocation had omitted SEMIO_TICKET_DIR and stopped before the main contracts; the corrected invocation supplied the active ticket. This successful full run covers the first Trunk hook/editor contract and prior caching work. The new metadata-prerequisite assertion and persisted-container-state check were added afterward and need the next complete run.

## Final Metadata Target and Refreshed Inventory

The renamed lockfile-check ran against the real WGPU manifest with exactly one Nx task and no generators: exit 0, 5.7s overall / 3.6s task. Editor regeneration exited 0 in 1m34s. The refreshed audit exited 0 in 28.0s and recorded 449 projects, 4308 commands, 890 artifacts and the same two unresolved output findings. A complete suite containing the new metadata-prerequisite and persistent-container-state assertions is active.

## Complete Trunk and Persistent-State Suite

The complete repo:test run with the asynchronous Trunk metadata-prerequisite assertion and the container persisted-state contract exited 0 in 5m59s (5m58s test task). The new extension attach and host-bundle checks below were added after that run and have not yet been included in a complete suite.

## Extension Packaging Qualification

The actual repo-vscode:build-vsix invocation failed in its build prerequisite: Rollup rejected top-level await from framework manifest Vitest registration in a CommonJS output. Nx exited 130 in 15.4s and did not run packaging; no cancellation signal was sent. Its build configuration has been extracted into an owned pure module, preserving its existing settings, then corrected to define import.meta.vitest as undefined and retain every node: host module as external. This prevents bundling framework test registration into the extension host. RED: the isolated fixture rejected the missing define property. Native Vite/esbuild/Node fixture qualification and the real extension package retry are active.

## Extension Host Build Qualification

The ordinary Vite define was insufficient: inactive Vitest blocks still contained top-level await when Rollup processed the CommonJS entry. A pre-transform now uses Vite’s native transformWithEsbuild with the compile-time define, treeShaking and minifySyntax enabled. The pinned [Vite 7.3.6 early esbuild plugin](https://raw.githubusercontent.com/vitejs/vite/v7.3.6/packages/vite/src/node/plugins/esbuild.ts) overrides the ordinary configuration’s treeShaking/minifySyntax, which explains the earlier failed retries. The focused fixture passed using actual Vite and independent esbuild bundles executed under Node. Actual repo-vscode:build-vsix passed in 10.9s across three Nx tasks; extension.js was 85.21 kB and the host test bundle was 120.98 kB. No editor activation was performed.

The resulting VSIX contained nine files (38.12 kB), including newly nested build/test files missed by the old denylist. Runtime allowlisting and deterministic archive timestamps are now under test. Installed VSCE 3.9.2 already applies SOURCE_DATE_EPOCH to every archive entry; the production packager will use a fixed ZIP-safe epoch and UTC. The first fixture attempt failed manifest validation because its synthetic extension omitted activationEvents; the fixture was corrected before testing the allowlist.

Native VSCE packaging fixture now passes. Its corrected manifest exposed the old denylist shipping reports/build.log and nested build/test files. The first allowlist retry still included host tests because VSCE interprets !out/ as including that subtree; removing directory exceptions and retaining only the exact bundle path fixed selection. Both native Node/VSCE runs produced byte-identical archives after changing every source mtime; yauzl independently decoded the six expected archive entries with DOS time 0/date 33. The fixed SOURCE_DATE_EPOCH is 315532800 and TZ is UTC. The production build and full suite (including newly registered attach, host-build and package checks) are active.

The real extension build with the runtime allowlist passed in 9.0s (three tasks). Native Python zipfile inspection found exactly six entries, all dated 1980-01-01 00:00:00; archive size was 36,521 bytes and SHA-256 5eee2468ff3ef8b1a2123fe52480120ccd6266b750498c96ff1f2f256fd35f70. The immediate repeat passed in 1.1s with 3/3 local Nx cache hits. This proves current warm reuse; it does not yet prove deleted-output restoration or activation in an editor.

The complete repo:test suite passed in 3m31s (3m30s test task), now including extension attach, host build and deterministic package checks. Its graph inventory contained 449 projects. Existing isolated dependency/bootstrap, native cancellation, production browser restoration, cache/input and editor contracts also passed. A separate probe now snapshots the two actual extension Nx cache entries and exercises the installed native Nx restore operation against a private consumer directory; live development outputs remain in place.

Native Nx cache restoration of the actual successful extension tasks passed in an isolated consumer directory. The probe copied only task entries 11407283862796506386 (build: two files, 206,183 bytes) and 1482114527785381486 (build-vsix: one file, 36,521 bytes) into ticket-local snapshots. Installed Nx 23.2.0 restored each twice after deleting only its private consumer outputs, with byte-for-byte digest agreement. Native Node parsed the restored CommonJS bundle, and yauzl opened the restored VSIX and verified its embedded extension.js equals the separately restored build output. This directly tests the production cache payload and native restoration implementation; it does not run an editor activation or prove another operating system. No shared output was deleted.

## WGPU Boot Input Ownership

The browser boot entry imports PLAYGROUND_SESSION.variant from the mutable os-dev generated session file. ensurePluginRegistry writes that file for whichever playground was selected most recently; the current file contains generation3d. The WGPU dev URL already carries an explicit plugin query, while the leaf boot silently borrows this unrelated last activation when the query is absent. This couples otherwise identical renderer bundles to mutable ignored state. The next change replaces this fallback with DEFAULT_HOST_VARIANT from the deterministic generated playground catalog; explicit query selection stays authoritative. Native Bun and esbuild bundle fixtures will substitute conflicting session content without mutating the shared source, proving the dependency disappears. This alone will not qualify the complete WGPU output cache.

The initial boot fixture failed before its assertion because a nested Node-eval string contained an unescaped newline. Its program now serializes the complete extracted TypeScript-compiled descriptor body and expression together. A ticket-local interactive fixture harness runs registered probes under one uncached Nx exec invocation, allowing fresh Bun child processes for each TDD retry without repeatedly rebuilding the full graph. This harness performs no package installation and writes captured output only under the ticket generated directory. The actual fallback RED check remains pending.

The corrected descriptor fixture reproduced the ambient fallback (fixture-first instead of catalog default s). The boot source now imports DEFAULT_HOST_VARIANT from the deterministic catalog. Native Node executed the actual extracted descriptor functions across query/default/role/hub cases. Bun and esbuild built the real boot entry twice with conflicting session overlays; neither read the mutable session and each compiler produced identical parseable bytes across overlays. The esbuild oracle initially rejected a Unicode plugin-filter regular expression translated to unsupported Go escapes; its filter now uses an ASCII suffix and checks the exact normalized path in its callback.

A new generate-browser-boot target owns the exact generated JS file and declares registry/schema generator prerequisites. Trunk build/serve and dev declare that producer. Their leaf scripts check the prepared boot bytes; check-browser-worker also checks without writing. The fixture verifies missing/fresh/stale checks preserve output absence, bytes and mtime. Editor entries were added to the source launch catalog. Focused fixture passed; native generation is active. The target’s input groups are being narrowed from default to production to exclude tests. Further source/tool input refinement remains necessary because the WGPU project’s general inputs still include native toolchains.

The expanded boot fixture passed after two further RED checks: generator inputs now use production/^production, and check-browser-worker is explicitly uncached because current generated-file freshness cannot be reused from an old test result. Native generation passed in 1m28s across four tasks (including repo:generator-inputs), before the final configuration retry. The registry generator regenerated launch.json; both new editor commands occur once. Final configuration generation and full-suite qualification are active.

Final-input generation passed in 1m33s, and the complete suite passed in 4m21s with the new boot contracts. The subsequent production repeat passed in 58.9s but recorded 0/4 cache hits; the three boot task hashes observed so far differ, so warm reuse is not qualified. The generated file is 43,325 bytes, SHA-256 da3fc2eddd3a57090abb8ac6f5b9a4d0b0f2f72f3a53a442f3fd02ac7ea75180, with no PLAYGROUND_SESSION reference. Native Nx restored task 17656560694007429698 twice into a private consumer directory. Every restored byte matched the real cache payload, and native Node executed all selection fixtures extracted from the restored emitted JS and parsed the complete module. This does not start WebGPU.

A temporary, ticket-local Node preload now subscribes to pinned Nx task input collection and records hash details plus source-file digests for the boot, schema and registry tasks. It records no source text or raw environment values and does not modify Nx files. Two identical instrumented public bun nx invocations will identify the input changes responsible for cache misses; the preload itself changes NODE_OPTIONS, so its first run cannot be compared directly with the previous uninstrumented hash.

## Boot Cache Miss Evidence and Next Boundary

The temporary preload produced no receipts: the public launcher's devToolingEnv intentionally removes NODE_OPTIONS. No launcher or Nx module was changed to bypass that boundary. That run passed in 2m20s with only schema generation cached. A subsequent ordinary run passed in 1m47s with 0/4 hits. Comparing the native Nx file-map snapshots before/after found 141 changed file records, with unchanged external-dependency graph identity. These are observed concurrent source changes, not inferred timing noise. One matched the boot target's broad library source inputs: caching/📦️artifacts/🦀️rust/📜️script.ts. The editor seed and launch output also changed.

Comparing actual cached registry outputs for task hashes 11327136198814017886 and 2014575713800177248 found exactly one changed output: .vscode/launch.json. The boot target's transitive **/* dependent-output input currently includes this unrelated editor file. Its compiled bundle contains runtime code from only five files: browser-boot, browser-frame-transport, browser-interactive-job-port, UI Ports/interactive-jobs, and the deterministic generated playground catalog. These observations justify narrowing this target's command/source closure and generated-output dependency, rather than repeatedly retrying an input set that includes unrelated native and editor work.

The native check-browser-worker run reached and passed the boot comparison, then failed on stale existing frame-worker.js (34.7s). It did not regenerate that file. The separate generate-frame-worker target is now running before repeating the read-only check.

### Planned Narrow Input Contract

Extract the shared Bun browser bundler and boot render/check functions into a pure WGPU browser-build module. A dedicated 📜️script.ts will produce the boot artifact using the small existing process router, avoiding the mixed native renderer command module and broad repository library imports. Keep public package exports as ordinary owned module exports if needed by the existing tests; do not add legacy argument routes.

The Nx plugin should support declarative relative source input groups: explicit entry points, explicit generated leaf modules, and recursive static relative imports. Built-in modules are allowed; bare external imports and computed module loads must fail this restricted contract instead of silently omitting dependencies. This feature is intentionally for owned relative module graphs; other target contracts remain separate. Validate its declarative schema with a third-party schema oracle and compare the collected files with native compiler inputs. Known generated leaves must be permitted to be missing during graph creation, since their Nx producer restores them later. Enforce that generated leaf modules have no runtime imports when compiling, so a newly generated hidden source dependency cannot enter a cached result.

The boot target can then hash the exact source/command closure, Bun configuration and runtime identity, and only the generated playground catalog output. The thin generator needs the registry producer, but no native compiler fingerprint or framework schema prerequisite. Test missing-generated bootstrap, input membership, rejection of unsupported imports, test/editor/native-source independence, actual generation, cache reuse and private restoration. Do not label the current broad target fully optimized or the complete WGPU renderer cache qualified.

### 2026-09-09 Browser Input Contract Implementation

The frame-worker regeneration completed successfully in 6.0 seconds. A fresh browser-worker check is running. The source input contract is being implemented as a restricted relative TypeScript module graph: explicit entrypoints, declared external packages, and generated data-only module boundaries. Missing generated boundaries are permitted during graph construction and must exist and contain no runtime imports before generation. A language-neutral fixture and JSON Schema precede implementation; native esbuild input metadata and Bun/Node runtime results are the independent oracles. The first focused probe is pending. This contract does not attempt arbitrary package export resolution.

The native browser-worker freshness target passed in 23.0 seconds (five tasks; one prerequisite cache hit). The new source closure test first failed on the missing implementation. Its independent esbuild oracle then exposed that inline type re-exports can require compiler source resolution; the scanner now conservatively preserves those edges and excludes explicit type-only declarations. The corrected source fixture passed through the Nx fixture lab (6.9 seconds), including native Bun/esbuild compilation and Node execution, missing generated inputs, changed imports, schema rejection and generated runtime-import rejection. The boot target now uses a dedicated thin script and a derived `browserBootSources` group, depends only on the registry generator, and hashes only its generated playground catalog rather than the entire registry output tree. Native task hashing and warm-cache qualification remain pending.

Native generation passed (1m48s, three tasks). The boot output remains 43,325 bytes with SHA-256 `da3fc2eddd3a57090abb8ac6f5b9a4d0b0f2f72f3a53a442f3fd02ac7ea75180`. The immediate repeat passed (1m23s): both registry prerequisites executed, but **generate-browser-boot used the local cache**, task hash `11346122360804166874`. This qualifies the removal of unrelated prerequisite-output invalidation. The uncached registry guard and registry generation each still consumed about 30 seconds in that repeat; they remain separate optimization work. The complete repository contract suite and restoration of the actual narrow-input cache entry are running.

### Registry Fingerprint Failure Guard — Current Nx Qualification

A private native Nx 23.2.0 fixture confirmed the historical runtime-hasher limitation still exists. A successful task was cached with a fingerprint command that prints a constant string. Changing only that command's exit status to 1 while preserving stdout resulted in Nx exit 0 and a cache hit; the task execution counter remained 1. Adding an uncached prerequisite running that same failing fingerprint prevented the downstream task from running (Nx exit 130; execution counter still 1). Logs and receipt are under `🗑️generated/runtime-hash-exit-*`; the retained input is `🔬️runtime-hash-exit/📜️script.ts`. **Do not remove the uncached guard.**

A possible next optimization is for the uncached guard to publish an atomically written, owned digest file and for the registry generator to hash that prerequisite output. That would retain failure propagation while avoiding the second full catalog scan in Nx's runtime input command. It requires explicit output ownership, metadata tests, a native fixture proving content changes invalidate the downstream cache, and warm/failure/restore qualification. This design is not implemented yet. All fingerprint source membership and exact-byte hashing must remain intact.

The actual narrow-input boot cache entry `11346122360804166874` restored 43,325 bytes twice through native Nx into a private consumer. Restored code passed all selection fixtures in native Node and complete ES-module syntax validation. The first full-suite run failed in the new numeric output oracle because Nx forces terminal colors; the fixture now emits its number as JSON text. The corrected full-suite run is pending. No production source changed after the qualified boot cache hit.

The JSON-output full-suite run reached the empty-checkout bootstrap fixture and failed after 7m20s: its explicit graph-source copy list omitted the newly extracted TypeScript source-input module, producing a native Nx plugin ENOENT. The fixture list now includes that module and its schema. This was an incomplete test checkout, not a missing file in the actual workspace. The earlier numeric-oracle issue is fixed and all new source-input tests passed in this run. A fresh full-suite validation follows.

The native digest-prerequisite experiment passed: cold execution count 2, warm cache hit count 2, changed digest count 3, deleted-output cache restoration count 3, and failed guard exit 130/count 3. Exact restored output matched the changed source state, and failure preserved the last successful digest. This qualifies the native Nx mechanism for the proposed registry optimization; no production registry receipt implementation has been added.

### Registry Fingerprint Cost Sample

The read-only stage probe completed against the actual workspace: 58,403 input witnesses, 32,998,508 content bytes, 59.18 seconds total. Discovery consumed 47.26 seconds, final exact-byte hashing 8.45 seconds, taxonomy loading 2.45 seconds, view setup 0.65 seconds, and imports 0.37 seconds. Concurrent machine load makes this a cost sample rather than a controlled benchmark. It nevertheless shows that import extraction alone cannot address the main delay. Preserve the uncached guard and remove the second full discovery/hash pass through a tested digest prerequisite. Retained probe: `🔬️registry-fingerprint-cost/📜️script.ts`; raw timing receipt: `🗑️generated/registry-fingerprint-cost.json`.

### Browser Source Isolation — Full Validation Complete

The corrected native `bun nx run repo:test` completed successfully: **8m31s, two tasks**, including the source-input fixtures, actual native boot-related contracts, minimal empty-checkout tooling and repository-plugin loading, 449-project inventory, editor/playground targets, lifecycle handling, compiler contracts, source discovery and cancellation. Log: `🗑️generated/browser-source-inputs-complete-suite-bootstrap-manifest.log`. The two earlier failed runs remain recorded as numeric console formatting and an incomplete fixture source manifest. This completes validation of the browser source-input isolation change. The monorepo goal remains open; registry scan duplication and the broader build/CI/platform/lease work listed above remain unfinished.

### Registry Digest Prerequisite — Implementation In Progress

The schema and language-neutral fixture preceded implementation; the first focused run failed on the missing publisher. The publisher now writes canonical `{digest,kind,version}` JSON through an exclusive temporary file and atomic rename, preserves an identical receipt's bytes/mtime, validates digest/kind/path, and rejects foreign symlink directories/files. The focused fixture passed against Ajv and `fast-json-stable-stringify`, filesystem mtime/byte checks, and the resolved Nx generator metadata (1.45 seconds).

The cache policy now maps `registry-catalog` to `repo:generator-inputs` and its one output `.🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/📇️registry/🔣️.json`. That store is excluded from catalog discovery, avoiding a self-referential fingerprint. The public target remains uncached, uses the thin `⚡️caching/🔏️inputs/📜️script.ts` router, and retains the exact membership/kind/content-byte hashing algorithm. The registry generator hashes the declared prerequisite output and no longer runs discovery in a runtime input command. The old fingerprint command implementation was removed from the broad caching script.

Native registry generation passed (1m34s; two tasks). The prerequisite consumed 1m1s and registry generation 30.1s on this loaded workspace. It produced the receipt and refreshed the registry/editor outputs. Warm reuse, exact ignored-output invalidation, and the new complete-suite run are still pending; do not attribute the prior browser-isolation full-suite pass to these newer changes.

### Registry Receipt Results and Fixture Storage Isolation

The exact production receipt path, explicitly ignored by both Git and Nx in a private native fixture, passed cold/warm/changed-content/deleted-output/failing-prerequisite checks. The real registry repeat passed in 1m4s with no hits; an observed repeat passed in 46.5s, also rebuilding. Its fingerprint changed from `5e5cd332407f4c746f2ba45a742a36440c0c8dc42888646ed483103f6d7e7413` to `f0c60ecc2a22610e65a232879769d10af4a5367b0d1ec03a3473b37f1194b96d`, so that repeat did not have identical discovered inputs. The receipt mechanism's controlled native reuse/invalidation is qualified; a real-registry warm cache hit is not claimed.

The complete receipt contract suite passed in 4m28s. Subsequent instrumentation exposed a pre-existing isolation bug in its empty-checkout fixture: it inherited the live `NX_WORKSPACE_DATA_DIRECTORY`, replacing the live file map with 25 fixture files and the cached graph with `workspace` plus `fixture-local-dependency`. Therefore the attempted before/after file-map comparison is **invalid as a source-change inventory**; its snapshots describe different workspaces and must not be used to attribute invalidation.

A regression fixture now injects deliberately foreign data/cache paths under the ticket and checks that their sentinel state survives native Nx execution. It first failed by overwriting that foreign file map, then passed after the fixture explicitly selected its own data/cache directories. Other private bootstrap/dependency/service environments now pin workspace root, data directory and cache directory as well. Normal Nx graph construction restored the live graph; checks found `repo`, the registry and WGPU projects present and 83,541 real workspace files. No shared daemon was stopped and no cache reset was used. The complete suite with this isolation fix is running.

The full receipt suite with private fixture storage passed in **1m52s**, two tasks. The actual graph subsequently retained 698 projects and 83,547 source file records. No private fixture project replaced the live graph. Log: `🗑️generated/registry-input-receipt-complete-suite-private-storage.log`.

### Full Native Audit Scope

The audit now includes all native providers: 698 projects, 6,575 commands and 7,588 artifact/store entries. Twelve findings remain, including ten inferred publishing executors newly exposed by full coverage. Indexed ownership checks passed Python parity and a 10,000-owner scaling regression. The actual audit passed in 15.2s. The complete suite failed at its existing public-script assertion for those ten publishing targets after 4m09s; it is not green. See `📓️native-graph-coverage.md` for precise coverage and the corrected Nx exec scope diagnosis.

The inferred publisher classification was corrected: native Nx executors are legitimate under the supplied plan and remain uncached in native task objects. Corrected full suite PASS **2m27s**; full native audit PASS **10.3s**, 698 projects/6,576 commands/7,588 entries/**two** missing-output findings. The prior ten publisher findings were false positives and are superseded. Broader runtime/input/CI/platform qualification remains unfinished.

### Confirmed Inferred Rust Test Input Gap

A native Nx hash probe found 35 missing compilation manifests (including the subject itself) in the inferred DOCX transitional subject test, which hashed 556 files. This adds a confirmed cache-input defect beyond the two missing-output findings in the automated audit. See `📓️inferred-test-inputs.md` for evidence and the next implementation boundary. No real application test/cache-hit claim is made by the hash-only probe.


## 2026-09-12 Native Inventory Refresh

Current resolved graph: 704 projects, 7250 executable commands, 7902 declared artifact/storage entries, four unresolved contract findings. The previous two missing-output findings are superseded by this snapshot.

- `@semio-tech/framework-async:twin`: CACHE-06 — Target still needs an explicit output contract. Source: `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📋️project.json`.
- `@semio-tech/ui-styling:twin`: CACHE-06 — Target still needs an explicit output contract. Source: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📋️project.json`.
- `@semio-tech/framework-renderer-wgpu:generate-frame-worker`: CACHE-03 — Output overlaps @semio-tech/framework-os:generate-wgpu at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js. Source: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`.
- `@semio-tech/framework-renderer-wgpu:generate-browser-boot`: CACHE-03 — Output overlaps @semio-tech/framework-os:generate-wgpu at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🤖️generated/🟨️.js. Source: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`.

Inferred Rust test inputs now cover transitive Cargo projects and native generators, with native fixture reuse/restoration/invalidation proof and zero missing manifests in a real case. See `📓️inferred-test-inputs.md`. The broader suite remains failing on three print taxonomy validator errors after the browser import relocation repair passed.


The two twin targets now declare `outputs: []`, matching their console-only checks. The CSS animation analysis is explicitly cacheable. The continuation twin includes current-host wall-clock deadlines and a timed scheduling chain, so it is explicitly uncached. Both actual Nx targets passed with cache bypass in 784 ms of target execution (the continuation suite ran eight real-event-loop cases). These commands ran through their authored Bun routers; the source's “Node twin” label is not a claim that this invocation used Node for the test body. The refreshed audit is running; WGPU generator output overlaps remain unresolved.


The post-twin audit completed successfully and now reports **2 unresolved findings**, both WGPU duplicate output owners. The twin missing-output findings are resolved. The current package generator `runWgpuPackageGenerator` really renders/writes six leaves, including browser boot and frame worker; those two are also owned by dedicated renderer targets. Removing declarations alone would hide real competing writers. Their generator implementation, schema/catalog ownership, consumers and tests need one coordinated refactor. No WGPU generator ownership source was modified in this turn.


## WGPU Ownership Checkpoint — 2026-09-12

The preceding WGPU duplicate-writer findings are resolved by physical producer metadata and coordinated renderer/publication changes. The semantic package preview still contains all six artifacts; Nx assigns four declaration outputs to OS generation and one browser output to each dedicated renderer target. Actual Nx package generation and both browser cache hits have been observed. Native inventory: 704 projects, 7,260 commands, 7,900 artifact/storage entries, zero unresolved automated contract findings. See `📓️generator-ownership.md` for native graph, payload/source parity, cache results and remaining qualification. This automated result does not close the broad platform, lifecycle, retention, CI and release-promotion requirements of the goal.


### Next WGPU Build Boundary Confirmed

Current renderer package script still performs environment preparation inside both Trunk build and serve: `ensureTrunk` probes PATH and falls back to unversioned `cargo install trunk --locked`; `ensureWasmTarget` invokes `rustup target add` when needed. These remain real hidden prerequisite/toolchain-selection issues even though the output-contract inventory is clean. The serve path also starts an asset server internally, and build/serve share the renderer output root. The next refactor must give pinned tool acquisition and Rust target preparation explicit Nx prerequisites, then verify serving/publication ownership and cancellation against the current lifecycle code. No Trunk build, install or shared tool mutation was run during this inspection.

The historical storage note describes an older artifact publication lock. Current publication now imports the shared resource-lease module and workspace-root resolver; independent esbuild inspection confirmed the three-file closure. Its new lease behavior must be considered when resuming retention work instead of reimplementing the superseded lock.


### Confirmed WGPU Compiler Cache Gap

A direct read of the native cached graph after the producer partition shows `@semio-tech/framework-renderer-wgpu:wasm` has `cache: true` and `outputs: []`. Its actual `TrunkBuildScript` runs `trunk build` into the shared renderer cache directory and copies the stable JavaScript/WebAssembly pair there. Therefore its cache can replay success without restoring the compiler deliverables. This is a confirmed CACHE-06 defect that the current explicit-output-declaration audit misses because an empty output list satisfies its structural check. The zero automated findings must not be interpreted as a complete compiler-output audit.

This compiler producer is the next work item: move finite Trunk compilation/publication into a dedicated, exclusively owned deliverable contract, make serving consume/activate that result under its live-store ownership, and supply pinned tools/targets through Nx prerequisites. The current cache policy was inspected but not changed in this checkpoint. Actual native-build is separately cacheable with `dist/native-dev`; serve/dev remain uncached continuous targets.

### Native Output and Trunk Prerequisite Follow-Up

The native renderer's declarations also pointed at the wrong package: Nx named the TypeScript `dist/native-*`, while the publisher and runner use the Rust package. Both profiles now declare their actual Rust artifact roots and share a narrow native producer/path helper. The permanent test compiles, compares with Cargo, restores deleted executable bytes/permissions through native Nx, and checks source invalidation. See `📓️native-renderer-outputs.md`; this supersedes the earlier assumption that the native output contract was already correct.

The observed Trunk/Rust-target acquisition is now an explicit uncached `workspace:deps-trunk` prerequisite of WGPU wasm/dev/serve and aggregate wasm setup. Ready/stale/missing tooling controls and the authored native Nx graph pass; the editor configuration was regenerated. See `📓️trunk-prerequisites.md`. Complete Trunk output ownership, optimizer/bindgen selection, live plugin/extension inputs and serving remain unfinished. The native runner also still invokes activation through a nested Nx process, and its generator/input closure needs separate qualification.

Default playground discovery's repeated repository input views were another expensive path exposed by full validation. It now reuses one view, preserves explicit views, and matches the generated 61-playground projection. See `📓️playground-discovery.md`. The broader suite is running again; no complete pass is claimed at this checkpoint.

The refreshed structural audit is 704 projects / 7,270 commands / 7,900 records / zero automated findings. Direct inspection found four cached compiler/packaging targets with empty outputs: WGPU wasm, OS Dev generic build, Python styling build and .NET styling build. Their implementations write real artifacts. See `📓️compiler-output-gaps.md`; these manual findings prevent treating the automated audit as complete. In particular, OS Dev's old generic build still calls the WGPU script directly and must be replaced with the explicit production graph.


### 2026-09-13 Compiler Follow-Up

The prior 3m33s suite reached the .NET bootstrap fixture and failed because its expected Compile Include was still the old package-local source. The fixture now names ../../🖥️host/🔷️.cs and additionally requires that file to exist. The next full run (compiler-output-suite-dotnet-path.log) stopped earlier after 46.3s on schema-entity-catalog taxonomy input/output ordering; it did not reach that repaired assertion. The live taxonomy was already reordered when inspected, so this task did not edit it.

Styling Python/.NET build metadata had been changed concurrently to cache:false while retaining empty outputs. This prevents stale success replay but does not satisfy the intended cache restoration contract. A new .NET native fixture failed as expected on the uncached/empty contract (589ms), then executed the actual assembly publisher and direct MSBuild compiler. Their runtime values matched, but binary comparison exposed native intermediate path differences (19.5s); canonical state path mapping is under test. No shared styling compiler state was deleted.


The .NET styling producer now has a dedicated script, native-state lease, canonical path mapping and owned final output. Native MSBuild/Nx restoration tests pass; its real second repository build reused the compiler cache. OS Dev generic build now depends on the explicit default production graph through a three-file completion command, with updated editor entries and native Vite/Nx restoration coverage. See `📓️styling-outputs.md` and `📓️production-completion.md`. Python wheel and finite WGPU Trunk ownership remain open.

The current shared CachePruneScript takes an exclusive `cache-prune` lease, then deletes eligible units based on age/budget. Its writers use separate resource keys; the styling .NET state lease added here currently coordinates restore/build processes only. Producer/read leases still need integration with reclamation. The artifact registry marks native cleanup/coverage pending, but that does not disable this separate cache-prune implementation. No destructive prune was run during this inspection.

The Python styling wheel now has an owned dist/build output and pinned backend tools. Native Nx/uv deletion, restoration, byte equality, changed-source and no-editable-preparation proofs pass; the actual repository test and next-run compiler/test cache hits also pass. See 📓️styling-outputs.md. Finite WGPU Trunk output ownership remains open: its HTML still copies active plugin/extension stores and the entire source asset tree, so simply declaring its shared output directory would be unsound.

The fresh structural inventory is 704 projects / 7,279 commands / 7,902 artifact/storage entries / zero automated findings. It includes the new Python contract; the manual Trunk finding remains. The 3m47s full suite passed the repaired transitive generator assertion and both compiler-output proofs, then failed a stale extracted NxScript test harness that lacked nxChildEnvironment. The focused cancellation check now imports the actual pure environment helper and passes all four malformed/unavailable process-snapshot vectors (721 ms); it is a standalone test module called by the full suite. No coordinator production behavior changed. A fresh full run is in progress.

WGPU live publication is now an explicit uncached activation operation; preparation is pure and cacheable, and activation no longer calls that prerequisite internally. The native Nx fixture confirms no duplicate publication and recreation of deleted live state. See 📓️wgpu-live-activation.md. This does not resolve the finite Trunk compiler output boundary, live-store concurrency/retention or full native application qualification.

## Full Repository Validation — 2026-09-13

The actual-root `repo:test` invocation completed successfully: 5m12s, both tasks executed (0/2 cache hits). Native inventory covered 704 projects and 6896 targets. Compiler output, Python/.NET wheel/assembly execution and native oracles, WGPU live publication, graph ownership, source-byte discovery, coordinator and materializer cancellation contracts all passed. The separate structural audit covered 7281 commands and 7902 artifact-storage entries with zero automated findings. This does not qualify the remaining Trunk output/serving, active retention, CI trust, or unexecuted platforms.

## Finite WGPU Outputs — 2026-09-13

Dev/release compiler publication, independent Trunk byte comparison, native JavaScript/WASM execution and Nx cold/warm/restoration/source-change contracts now pass. Native Vite HTTP routing to the restored profile outputs passes. Matching-profile playground activation dependencies and the separate browser generator boundaries pass their focused regressions. The extended repository suite and actual full renderer compilation remain in progress; concurrent Cargo builds are using the same native intermediate store. No shared process was stopped and no shared compiler cache was deleted. Live serving/composition, custom build-script input narrowing, active retention, CI isolation and unexecuted platforms remain outstanding.

The refreshed structural audit reports 704 projects, 7290 commands, 7904 artifact-storage entries and zero automated findings. Manual follow-up confirms `.github/workflows` is currently an empty real directory (not a symlink); tested CI baseline selection helpers alone do not constitute an installed CI workflow. CI workflow wiring remains a concrete outstanding implementation item.

The extended actual-root repo:test suite passed on 2026-09-13: 4m36s, 0/2 cache hits. It includes both native Trunk profiles, poisoned Trunk environment isolation, native Vite HTTP bytes, all compiler restoration proofs, graph/editor/lifecycle and cancellation contracts (704 projects, 6903 targets). The actual full renderer build remains running; it is not yet qualified.

## WGPU Browser Host — 2026-09-13

The live WGPU host now consumes completed outputs through an uncached, continuous Nx target. The public Nx launcher owns profile-specific activation watching; Trunk serving, occupied-port takeover, the old Cargo-package HTML/Trunk config and copy-route parser have been removed. Native Vite byte delivery, reload, cancellation and four route tests pass, including an actual HTTP run against the compiled repository renderer. Full suite revalidation is ongoing: the first rerun encountered a concurrently relocated Vite fixture path (fixed); the next failed the newly added duplicate-output regression before its compiler fix. Canonical-only compiler publication is under native fixture and actual-root verification.

Outstanding work still includes isolated WGPU variant/profile extension installations (the current activation still publishes to the shared live store), native runner nested activation, custom build-script input narrowing, reclamation coordinated with producers/readers, installed CI workflow wiring and native Windows/Linux/devcontainer qualification. HTTP byte equality does not qualify WebGPU painting or complete playground interaction. The goal remains active.

The latest native full suite passed canonical-only renderer restoration, both browser serving profiles, boot selection, native Vite lifecycle and production distribution/completion checks, then stopped at native preparation because current taxonomy references removed configuration contract IDs (2m46s). This is the same condition that stopped actual renderer republishing and frame-worker prerequisites. The refreshed structural inventory is 704 projects / 7292 commands / 7904 artifact/storage entries / zero automated findings; these manual issues remain. A direct cause is that Cargo/Trunk acquisition still loads the broad root application router and its taxonomy during tool synchronization, so unrelated source-policy edits can prevent tool preparation. This is an outstanding command-boundary issue as well as a transient shared-workspace failure.

## Native Setup Import Boundary — 2026-09-13

The native dependency import issue is resolved: nine uncached setup leaves now use the narrow implementation. Schema/native Cargo/esbuild proof and the actual workspace:deps-trunk target pass. See 📓️native-dependency-boundary.md. This removes application taxonomy from native provisioning; graph generation and the complete application/test pipeline still require separate qualification.

## Canonical WGPU And Explicit Cleanup — 2026-09-13

Actual WGPU wasm publication passed and retired the duplicate aliases; see 📓️trunk-output-boundary.md. The full contract run progressed past the repaired taxonomy and failed at an outdated Hub configuration reference; the focused corrected Hub contract passes. Cargo no longer launches detached cleanup after native tasks, with red/green native proof in 📓️cargo-cleanup-boundary.md. Active-use-safe retention, WGPU runtime extension isolation and other listed goal gaps remain open.
