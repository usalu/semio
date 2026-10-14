# Web Build Implementation

The web workstream introduces explicit prototype/release compiler policy, isolated mutable state and staged mode-qualified deliverables. Prototype disables final JavaScript/CSS minimization and retains source maps. Release selects the installed compilers' optimization defaults and removes delivery source maps. Intermediate JCO source snapshots remain unminified because the final closed actor worker applies the selected delivery policy.

## Ownership and Publication

Vite compilation uses a process-owned CLI with original cancellation, finite budget and visible progress. Vite state keys include owner, compiler version, mode, configuration, platform and architecture. Immutable output publication retains repository artifact manifests and publication leases. Dev listeners each receive their own mutable optimizer cache; warm optimizer snapshots are seeded/published under short exclusive state claims. Shutdown drains pending transforms and dependency scans before listener closure, which fixes the observed Bun/Vite close loop.

Next compiles into an owned warm state directory with a private tsconfig and SDK node_modules scope. Its delivered runtime is assembled from the installed SDK's actual NFT trace closure under workspace `dist/<mode>/repo-coordinator/next`; the shorter root avoids SDK filenames exceeding the 256-character bound. Directory symlink trace records do not become artifact files; traced physical files are normalized into standalone dependency paths. Conflicting dependency bytes and escaped workspace traces are refused. Start serves that assembled runtime rather than resolving a different workspace Next installation.

TypeScript incremental checks and declaration generation claim one CPU slot through the fair shared host gate. Bundles acquire their own compiler claim and release it before the declaration/typecheck phases. Every web compiler now shares the neutral repository `.cache/framework/leases` namespace for host and state admission. The initial `agents/resource-leases` location was replaced by the concurrently strengthened neutral control contract. Native integration owns fair per-core cohorts, whole-host writers and inherited driver reservations.

Simple JavaScript bundles use an owned esbuild CLI child. Artifact packages requiring Bun's `with { type: "text" }` semantics explicitly select a canonical Bun worker. That worker refuses inherited partial-host reservations, requires the genuine script invocation envelope, reads bounded stdin, checks mode consistency, supports original process-tree cancellation, and creates missing response parent directories for zero-touch debugger launches. Closed actor and Flow browser bundling also run in that cancellable child. The production stdin publisher drains backpressure and terminates on failed publication/EPIPE.

Declaration emission uses the installed TypeScript SDK to emit in memory, rewrites relative/import-type/export references through AST transforms, and flattens generated modules into bounded hash filenames. JSON and ambient declarations are retained. Canonical artifact snapshots are `📤️dist/<mode>`; existing package `dist` exports are leased active projections and remain uncached. Browser build consumers select canonical `dist/wasm/<mode>` using native portable `semioWasmOwner` metadata, and canonical component/plugin-module roots are selected from the same effective mode.

## Recorded Actual Validation

These results were observed in console logs, not inferred from source assertions. Logs live below ticket `🗑️generated` until final cleanup.

| Validation | Observed result |
|---|---|
| Vite fixture prototype/release | 1403/783 JS bytes in the initial fixture; source maps only in prototype; later canonical-WASM fixture 1478/832 bytes and explicit active-view rejection |
| Same-owner concurrent Vite builds | Both finished, state exclusion and staged artifact publication retained |
| Concurrent Vite listeners | Both served HTML and module transforms, different ports, cancel shutdown completed; a third listener reused a warm optimizer snapshot |
| TypeScript incremental state | Cold check 294ms, warm 259ms; warm output omitted Check time; actual TS2322 source edit invalidated and failed |
| Flattened declarations | Independent installed tsc consumed local interface, ambient and JSON imports in both modes, three emitted modules |
| Bun worker runtime oracle | Prototype/release closed actor outputs imported and matched independent esbuild output; actual compile-start-before-abort child was killed and host admission subsequently reacquired |
| Genuine worker invocation | Actual Nx web fixture passed after original envelope propagation (17.2s oracle run); direct invocation checks also passed |
| VSCode build | Both prototype and release actual builds passed; actual Node/esbuild host behavior matched |
| VSCode VSIX | Both mode packages passed installed vsce CLI; prototype archive 36.82KB |
| Architecture quiz | Actual prototype build passed 47.7s, 72 files; actual release build passed 28.5s before final shared-state import relocation |
| Artifact CSV release | Actual Nx test including independent tsc consumer and SQLite suite passed 12.3s |
| Artifact PDF prototype | Actual essential-Bun child build passed 7.4s |
| Next compiler | Installed Next 15.5.27 compiled all 14 pages after SDK scope correction; immutable projection and closed runtime acceptance completed below |

The latest broad transitive tsc check reported repository errors outside the touched compiler owners, including actor/UI/schema and existing native orchestration declarations. It had no error at the changed artifact/composition/Next/state owner paths. Full stdio dependency build surfaced two Rust dependency failures and formerly eleven unsupported text-import bundles; the latter now explicitly select the essential Bun worker. This is not a claim that the complete dependency graph passes.

At the earlier checkpoint, pending validations were: Next short-root standalone publication and HTTP runtime, new installed-NFT/text-import fixture, final quiz release after neutral owner relocation, VSCode release rerun, and essential-Bun release artifact consumer. Shared Nx retries encountered ENOSPC writing the project graph; parent removed oversized disposable ticket exports before retries. No Windows/Linux runtime run has been performed on this macOS host.

## Touched Files

The following inventory records this workstream's files; concurrent root/Nx/native edits to shared files are attributed to their respective owners. Every path is relative to the workspace root.

- `.github/workflows/architecture-quiz.yml`
- `.storybook/main.ts`
- `tsconfig.json`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📜️script.ts`
- `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/📜️script.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏗️build/🟦️.ts`
- `🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts`
- `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📜️script.ts`
- `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts`
- `📜️script.ts`
- `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🗃️state/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🗃️state/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🎚️mode/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📍️output/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🏗️builder/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🕸️wasm/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧫️cases.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🧫️cases.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧬️declarations/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️bun/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🏗️web-build/🧫️cases.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🎚️mode/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📦️publication/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/next.config.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts`

Removed: coordinator package `next.config.ts`; configuration is now the canonical SDK-loadable `next.config.mjs`. Original repo-local state/typecheck owners were moved concurrently into their neutral canonical locations by another contributor; this workstream adapted imports and modified the new canonical owners. Metadata/launch/native mode changes remain in the parent/Nx/native reports.

Additional shared runtime/pipeline edits:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`

## Final Run Updates

The final release batch `web-actual-release-batch.log` reran current source and succeeded for architecture quiz and VSCode (two successful tasks), while Next refused an observed mixed-SDK trace. The installed Next compiler's own next-server trace points at 15.5.27; its page traces point at the workspace 15.5.22 installation despite actual warm state resolution and page generation using 15.5.27. The assembler binds traced SDK externals to the installed compiler and its declared dependency/peer roots, matching the runtime module scope. Other dependency-byte conflicts remain rejected.

The corrected release Next build passed in 11.2s and published 992 files. A physical snapshot census found maximum absolute path length 236 characters; delivered Next is 15.5.27 and React is 19.3.0, matching the admitted compiler scope. Prototype build and HTTP runtime completed in the later closed-runtime acceptance below.

The final current-source Nx web fixture passed in 59.7s (`web-final-oracles-capacity.log`). It exercised actual installed NFT traces, both-mode essential Bun text imports versus the independent esbuild loader oracle, actual mid-compilation child cancellation and subsequent host reacquisition, prototype/release Vite bytes 1478/832, same-checkout concurrent staged builds, concurrent listeners plus warm optimizer publication, TypeScript cold/warm 534/388ms with warm Check time omitted, independent flattened declarations in both modes, and expected source-error invalidation.

The current PDF release test built 44 files and emitted 41 declaration modules. Its independent strict tsc consumer and all registered TypeScript suite checks succeeded. The first runtime suite then reported 5 pass / 1 fail: its pre-existing Rust syntax oracle could not find the configured replay-law function at `stream-roles/🧪️tests/🟦️.ts:92`. This failure is outside the build policy and publication change, and the complete PDF test is therefore not reported as passing.

Host/state ordering was corrected after an actual two-graph deadlock reproduction: the old independent path acquired state then queued host capacity, while an inherited graph held host capacity then queued the same state. The red run timed out with AbortError; after changing the canonical owner to acquire host first, both graphs completed. The permanent web fixture reproduces this opposite arrival and passed in 22.4s after the fix. Unused outer state locks were removed from artifact/composition builds, and artifact test imports and consumer probes now live in their retained private compilation snapshot. Concurrent CSV prototype/release full tests passed 18 tests each (191 expectations), 11.9s/9.8s; canonical outputs received no temporary consumer files.

A stricter standalone runtime guard correctly refused the first startup closure because a required SDK `trace/index.js` was absent. The startup NFT resolver now selects SDK source roots before analysis, not merely during publication, and excludes development webpack/Electron and source-map/type inputs. At that checkpoint both mode rebuilds were in progress; ancestor fallback is forbidden by the test guard, so previous HTTP200 without that guard is not final acceptance.

Additional permanent test sources:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️next-runtime/🟦️.ts`
- Ticket input `inputs/next-runtime/📜️script.ts` (imports the permanent guard; retained after cleanup).

The actual NFT relative-resolution correction captured previously omitted SDK `trace/index.js` and `zod/index.cjs`. The guard then demonstrated that nested compiled-package manifests are part of runtime authority: omitting zod's `package.json` makes Node try `index.js` and fall back to an ancestor installation. Publication now also captures package scope manifests along each dependency's actual ancestor chain. Final both-mode rebuild/runtime checks completed with this complete file-and-manifest closure, as recorded below. One obsolete 89MiB Next state (`d606...`) was removed under its exclusive state lease after prior parent authorization; current warm states were retained.

## Completed Standalone Runtime Acceptance

Both final independent Node runtime guards passed: release 2.1s and prototype 2.2s. Each loaded SDK 15.5.27, recorded 219 local module resolutions without ancestor fallback, returned HTTP200 with 4876 HTML characters, and served all five referenced static assets with HTTP200. Actual asset bytes were 462822 in release and 1224638 in prototype. The guards had previously caught the missing startup helper, SDK trace directory index, zod entry and nested manifest, so these final results include a genuinely closed startup scope rather than accidental workspace resolution.

Final selected-SDK assembly includes default NFT source discovery, explicit SDK dependency/peer roots, Node relative-module resolution, and package scope manifests. Development webpack/Electron and map/type inputs are excluded from production startup tracing; compilation's own traces and emitted assets remain part of the mode snapshot. Both prototype/release builds ran concurrently in the same checkout and serialized their heavy compiler phases through the host gate.

A final prototype audit explicitly disabled tree shaking in Rollup, esbuild and Bun, in addition to final JS/CSS minification. Storybook retains its authored Rollup settings while applying this mode policy. VSCode now removes only compile-test `if (import.meta.vitest)` blocks through the installed TypeScript AST, preserving unrelated declarations and production else branches; the previous prototype syntax-minification exception is gone. A language-neutral fixture is compiled independently to CommonJS by esbuild after this semantic removal. Current prototype architecture quiz and VSCode actual builds both passed in a 32.2s batch, with quiz on the 30.3s critical path.

Additional shared public owned interface edited: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` adds an owned Rollup option record with boolean `treeshake`; no SDK type appears in its public interface.

## Original Admission Contract Final Integration

The concurrently strengthened neutral admission API now requires the actual caller's signal, remaining deadline, progress publisher and continuation ports. Web compiler owners preserve the incoming ScriptInvocation and combine optional local cancellation only through `scriptCompilerAdmissionControlV1`'s additional signal. Fixture-only invocation clones and identity assertions that would reject legitimate additional cancellation were removed. Every controlled child receives the remaining budget after its lease wait; no phase renews the requested child deadline after waiting. Stdio read-only tsc claims one CPU directly and no longer holds unused mutable state. Host-before-state ordering remains universal.

VSCode's actual SDK bundling runs through the canonical owned `compile-extension <output-directory>` child command. The parent reserves compiler capacity, passes the genuine invocation envelope and admitted width, and kills the process tree on original cancellation/deadline. Independent debug launches acquire the same host gate and create their output parents. Packaging receives a private mode snapshot. The target and both debug modes are registered by the Nx owner.

Current-source final checks after the admission contract change: quiz release succeeded with three actual Nx tasks (57.9s total, quiz44.8s); concurrent CSV prototype/release complete tests each passed18 tests and191 expectations (85s/72s including host waits). Latest actual VSCode release build-vsix passed10.1s including3tasks. Initial final Next retries exposed Node's required JSON import attributes in the newly reachable original-invocation schema owners; both explicit attributes were added, and Node24 directly imported the capacity owner successfully. The final web fixture and both permanent Next runtime checks completed after this correction, as recorded below.

Additional touched original-control boundary sources:

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`

## Lazy Dev Compilation Admission

Vite now wraps the installed public environment `transformRequest` and `warmupRequest` methods with original-control host admission. Public eager pre-transform requests are disabled at server/environment scope so nested fire-and-forget transformations cannot escape or recursively deadlock a claim. Already discovered optimizer promises are drained before the claim is released; dependency discovery and isolated warm snapshots remain enabled. An idle listener holds no host claim.

VSCode now runs its dev watcher in an owned canonical `watch-extension <entrypoint> <publication-directory>` child. Rollup `buildStart` admits the actual compilation, `writeBundle` captures and publishes a successful private snapshot then releases, and failed `buildEnd` / `closeBundle` release exceptional claims. Process-tree cancellation bounds SDK compilation and watcher closure. Each watcher uses a unique mutable output directory; active/debug publication is leased. The portable watch fixture changes handcrafted source while another graph holds host capacity, observes the actual queued rebuild, checks its emitted value against esbuild and checks post-cancellation admission.

Final admission retries also exposed another concurrently added neutral facade export (`requestedCompilerAdmissionControlV1`); native owner supplied the explicit reexport. Both Next builds completed under current controls (release121s and prototype291s including shared host/graph waits), before the runtime router's missing default probe directory was found and corrected. Next/web permanent checks now create their default cache probe root; ticket runs explicitly override it to ticket-generated output. Latest physical mode census: prototype2243 files with maximum path228, release2225 with maximum path226.

## Current-Control Final Runtime Results

After the facade and JSON boundary repairs, the expanded actual web fixture passed42.5s (`web-lazy-compiler-admission.log`). Vite prototype/release remained1478/832 bytes; both maps and immutable mode selection were verified. Actual Bun compilation-start-before-abort killed the child and released admission. Both concurrent listeners, optimizer snapshots, opposite-arrival host/state order and independent declaration consumers remained green. The new actual Rollup watcher queued behind a separately held host claim, published the warm handcrafted source update matching esbuild, held no idle capacity and cancelled its child tree. TypeScript cold658ms/warm1098ms retained semantic incremental reuse (warm Check time absent); elapsed timings include concurrent workload and are not presented as a speed guarantee.

Both current-control permanent Next standalone runtime targets passed2.3s each with existing build dependencies explicitly excluded for this final read-only runtime gate. The corresponding both-mode dependency builds had already completed under the same admission contract. Prototype/release each loaded the installed SDK15.5.27, returned HTTP200 and served five static assets; the guard again refused ancestor resolution. The stronger Vite warm-transform concurrency check passed35.6s, holding a real other-graph host claim while a new module request waited, then verifying its updated HTTP200 response after release.

Final current-source fixture after the declaration oracle's one-CPU/original-signal correction passed25.8s (`web-lazy-owned-oracles-final.log`). This run includes the stronger Vite warm request, actual watcher queued-update event, both-mode compiler/byte/map policy, actual Bun midcompile child cancellation, opposite-arrival host/state regression, real incremental source invalidation and admitted independent declaration consumers. Later full-workspace prototype growth validation exposed the response framing limitation corrected below. Next dev Webpack per-rebuild admission is delegated to the parent owner; this report's lazy dev acceptance covers the Vite and VSCode owners.

## Large Prototype Worker Files and Custody

The full-workspace prototype run exposed the16MiB bulk JSON response threshold. It was replaced by a bounded4096-byte language-neutral manifest binding request SHA256, kind and mode to unique private source/map/metadata files with byte lengths and SHA256. Output data never travels as a bulk JSON field. Entry bundles promote verified files directly; actor/module/snapshot consumers decode verified bytes only where their owned API needs source in memory. The worker refuses output path escape, symlink custody, changed sizes/identity and mismatched bytes/request/mode. Parent verification also checks actual SDK-observed source metadata and source file digests before promotion, and checks copied package output bytes. Package onLoad capture preserves Bun's actual loader, with metadata coverage checked against installed Bun's metafile. Separate output size thresholds were removed; the authored bounded input pipe remains.

A handcrafted prototype payload of17825792 characters produces an artifact above16MiB and matches the independently built esbuild exported length. The actual manifest validates with jsonschema, remains below4096 bytes, and an actual source-file tamper is refused by the owned digest reader. The complete expanded runtime fixture passed78s (`web-file-manifest-large-prototype.log`), including earlier lazy Vite/watch/cancellation/mode/incremental/declaration laws. The earlier watcher failure/recovery extension also passed143s under concurrent load: invalid syntax matched esbuild's refusal, released admission and preserved publication, followed by successful recovery. Cached Vite transforms now return the valid public module graph result without compiler admission; cold/invalidated transformations retain full admission.

Next dev actual fixture is now durable in the existing next-runtime owner, routed by `test next-runtime dev`; Nx metadata target repo:test-next-dev-runtime is cachefalse, outputs[], no production build dependency. It tests real source dev server startup/HTTP/assets, cached root serving while another graph holds capacity, cold health route admission and cancellation. Parent owns the grouped public Webpack hook integration and its launch/seed registration. Both actual development runs completed after the parent's grouped admission gate passed: prototype30.2s and release29.0s. Each loaded Next15.5.27, returned HTTP200, served four referenced assets, yielded idle host capacity, served cached root pages in73ms/100ms while another graph held the host, queued the cold `/api/v1/health` compilation, then returnedHTTP200 with `ok` after the claim was released. Both cancelled the owned development process tree and immediately reacquired host admission.

## Additive Cancellation Contract and Activation Modes

The canonical `scriptCompilerAdmissionControlV1` accepts an optional local cancellation signal alongside the genuine incoming invocation. It retains the invocation's original deadline, progress and continuation ports, and combines cancellation with `AbortSignal.any`; without an additional signal, the original signal identity is unchanged. The actual midcompile worker/service fixtures now use the genuine invocation directly, never a cloned control. A portable pre-aborted local-cancellation assertion verifies that the host refuses admission while the original caller remains alive. This restores the explicitly authorized public cancellation facility after a concurrent contract edit removed it.

Additional shared source touched: `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts`.

## Explicit Activation and Fixed Targets

Activation receipts, runtime roots and React-facing Play/Demonstrator consumers now use only prototype/release. The actual Nx regression failed against the old parser with `Invalid activation receipt`; the migrated receipt/path law then passed against installed Ajv and the expanded real compiler fixture (31.4s). Fixed authored Play/Demonstrator preparation/activation targets have visible defaultConfiguration and SEMIO_BUILD_MODE pins. The Nx owner's installed createTaskGraph oracle rejects an opposite-mode fixed dependency, and bootstrap rejects an explicit opposite configuration; generic builds retain mode-neutral preparation.

Exact authored target renames are prepare-prototype and activate-prototype in Play/Demonstrator, plus activate-lanes-prototype in Demonstrator. The corresponding union output is dist/♻️activation/prototype. Demonstrator preparation receives no profile argument and reads strict mode from the admitted Nx environment. Its lane aggregate now calls canonical runtime `📜️script.ts complete`, replacing Nx's noop executor. The OS distribution aggregate likewise calls canonical distribution `📜️script.ts complete`; both completion commands check the genuine invocation and accept no arguments. Parent owns their launch entries.

Activation and React serving read canonical mode-specific browser modules. Mutable source-content/stat indices are stored separately below each runtime; no activation mutates canonical build artifacts. The physical immutable-source fixture passed23.4s with the full worker/watch/Vite/type/declaration suite. A subsequent long-name fixture physically reproduced the256-character staging-path violation; source-state keys now use24 hexadecimal SHA256 characters below the runtime's 🔖️ directory and reserve64 characters for atomic staging filenames. Installed crypto-browserify independently produces the same key.

The focused strict TypeScript compiler closure finally passed after the parent repaired resource-lease facade exports and the Nx owner supplied the owned MJS declaration boundary (`web-owned-file-manifest-types6.log`, exit0, no diagnostics). This is a focused owner check, not a claim that all unrelated repository TypeScript passes.

Additional touched files for this final integration (relative to workspace):

- `🏢️semio-tech/🎡️play/📋️project.json`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playactivation/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema/🔣️.json`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️pipeline.json`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️invocations.json`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorunionreceipt/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧫️cases.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🔌️staging-root.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🚀️local-hub.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🚀️boot/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎬️host-activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧩️composition/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🧪️tests/🟦️.ts`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/🧊️live-activation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`

The final current activation/path/oracle suite passed23.6s (`web-activation-path-limit-green2.log`). It includes the longest variant/plugin source-state staging bound, the installed crypto-browserify key oracle, unchanged immutable source listing, original additional cancellation, large prototype manifest/custody, Vite cached/cold requests, watcher failed/recovery hooks, opposite-arrival state ordering and independent TypeScript declaration/incremental checks. The first short-path implementation correctly refused the overly deep ticket fixture root; the actual source-state fixture now uses its existing short private ticket root, while production runtime path selection is checked independently without copying repository sources.
