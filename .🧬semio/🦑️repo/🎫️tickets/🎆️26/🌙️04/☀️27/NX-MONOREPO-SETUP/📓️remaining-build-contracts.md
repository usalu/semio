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
