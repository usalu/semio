# Fresh Build Pipeline

Play uses its Nx `prepare-release` graph to publish runtime components before its site builder creates `dist/pages/{play,map,media,modules}`. The release builder stages a new Vite output, then moves its files into independently hosted pages. The existing deployment output occupies about 610 MiB; this is an observation of an earlier build, not proof of current readiness.

`--skip-nx-cache` reexecutes Nx tasks but does not empty Cargo's shared compiler intermediates. This run isolates both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`, plus Nx cache and project graph data, beneath `.🧬semio/🦑️repo/⚡️cache/play-fleet/2026-10-07-release`. No shared cache or other process is removed. Generated command output is recorded under this ticket's `🗑️generated` directory.

The existing launch entry supports an ordinary build only. Add an uncached `build-fresh` target to make complete compiler and task freshness reproducible with a newly created cache generation on every invocation. Its contract must discard inherited Nx task identity, force local and remote cache bypass, and preserve cancellation through the existing repository Nx launcher.

The release build was launched on 2026-10-07 with four concurrent Nx tasks. Runtime production-page verification belongs to the parent agent; application unit and artifact editor verification belong to the editor agent. No Play deployment workflow exists in `.github/workflows`; publishing itself is outside this preparation run.

## Catalog Ordering

The current source catalog correctly discovers moved component owners under `🌎️hub/🧩️compositions`. Cad's descriptor declares app channel 20, while the current host requires 23; the canonical registry therefore intentionally excludes Cad and other stale owners. Draw and Puzzle have current descriptors and remain admitted. Changing path discovery or synthesizing registry rows would conceal the real descriptor freshness failure.

Per-variant session tasks depend on both the canonical registry generator and fresh owner descriptor producers. These siblings can run in parallel, leaving canonical generated TypeScript stale even though individual sessions read current descriptors afterward. The new `catalog-release` target waits for all 28 framework release preparations, then invokes the existing registry generator. Play's own release validation waits for this publication before it reads canonical module layout. The final Vite bundle then consumes the freshly published catalog.

## Verification

The language-neutral fresh-build fixture covers isolated compiler and graph directories, discarded inherited Nx task identity, caller environment preservation, complete Nx task graph selection and local/remote cache bypass. The test independently parses argument semantics with the existing third-party `yargs-parser`. A real red run failed the unimplemented environment law; the following run passed both environment and command tests. The catalog-order regression subsequently failed on the missing graph edge before its implementation.

Native artifact publication always runs Cargo in a newly created capture target and the explicitly selected intermediate build directory. Browser materialization always runs transpilation and a descriptor probe. Neither publisher has an internal result-cache shortcut that defeats Nx cache bypass. Byte-identical final file publication may remain content-identical after recompilation, which does not imply a compiler cache hit.

Initial release progress waited for the repository-wide Cargo preparation lease held by an unrelated bulk artifact preparation. Only that owner's active process was inspected; no unrelated process was stopped and no lease was removed. Once it completed, this graph advanced into generator and descriptor-tool production.

## Current Owner APIs

The first executable fresh-target run exposed a real import failure before compiler preparation: GIS tile utilities had moved out of the styling Vite module. Play now imports tile planning and prefetch directly from the canonical GIS artifact owner. The builder forwards its cancellation signal and displays tile progress. The Play Vite configuration also replaces the removed playground plugin helper with the current domain-neutral asset dispatcher and canonical playground asset providers, matching the framework's current development and WGPU compositions.

After these repairs, the fresh target allocated a new private generation named `release-BK9gnO` and reached the full prerequisite source-generator graph. No earlier compiler target or intermediate directory belongs to this generation. This is progress evidence, not a completed release claim.

The fresh rebuild, final catalog publication, production release gate, routing gate, browser capability probe and final-page publication audit are registered in both the launch seed and generated launch configuration. The diagnostics remain executable from their preserved ticket scripts after generated logs are removed.

The exact production worker-routing probe is also registered in both launch configurations. The final fresh contract recheck passed all three tests through Nx. An independent Nx/Bun runtime invocation of Play's canonical map prefetch with an already aborted signal returned the original cancellation reason before any network operation, with its successful temporary debug output captured in `map-prefetch-cancellation.log`.

## Full Freshness Audit

The private generation `release-BK9gnO` reached real Rust compilation. Live compiler arguments named its private `cargo/build` output directories for host dependencies and browser WASM dependencies. However, the generic Flow prerequisite selected `wasm-pack build --dev`: the fresh launcher had not yet forced the repository's shipping preference. This partial generation was explicitly cancelled by SIGINT to owned bootstrap PID 49292; its child compiler processes stopped and the launcher returned 130. No unrelated process was interrupted. Its outputs are not accepted as the final release.

A real red regression then failed two laws: the environment did not force `SEMIO_BUILD_MODE=ship`, and a Vite isolation selector did not exist. The permanent fresh launcher now forces shipping mode for every prerequisite, and Play's Vite configuration puts its state under the caller-selected private Nx cache directory. The new final attempt uses `fresh-release-ship.log`; its generation and completed producer evidence will be recorded below after execution.

The subsequent Nx test run passed all four fresh-release laws, including shipping mode and Vite isolation. The corrected fresh build was launched only after this green result.

The corrected launcher allocated `release-VEXZqG`. Its outer target is uncached; the inner full graph uses `--skip-nx-cache --skip-remote-cache --parallel=4`, private Cargo target/intermediates, private Nx graph/cache data and private Play Vite storage. All seven requested release build, gate and diagnostic launch entries were verified present in the parsed generated launch configuration.

The editor agent's retained feature audit and real-browser acceptance oracle were subsequently registered at the next two diagnostic positions in both launch files. Their generated launch entries parsed and validated successfully, bringing this work's total to nine commands.

The corrected generation subsequently reached actual Flow compilation with `wasm-pack build --profile wasm-release`. Live Rust compiler arguments named `release-VEXZqG/cargo/build/wasm32-unknown-unknown/wasm-release` and its host profile directories. This confirms both the corrected shipping selection and genuinely new intermediate production before final component/catalog/page completion.

The publication agent's spaced-path argument serialization probe was registered at the next available diagnostic position, 11.08. Parsing the generated launch JSON confirmed preservation of the Windows path backslashes. This brings the task's authored launch additions to ten commands. Native descriptor-tool and Flow release-WASM dependencies then compiled concurrently in the corrected private generation.

The Flow producer completed in 372.2 seconds and published a 9.49 MiB browser WASM binding. The native descriptor-tool producer completed its optimized release profile in 6 minutes 15 seconds and staged one deliverable into its owned `dist/build`. Private Cargo intermediates occupied about 2 GiB at this stage. These successful producer outputs still precede the complete actor descriptor/materializer closure and final site/page output.

GIS tiles under `.🧬semio/🗺️map/{osm-tiles,openfreemap-vt}` are downloaded external raster/vector source inputs, analogous to installed toolchain or package sources. They are copied into a newly staged build; prefetch preserves existing source bytes and downloads absent tiles, with cancellation and progress. They are not previous generated Play modules, compiler intermediates, bridges or descriptor outputs. Clearing these shared provider downloads would alter other developers' map input availability and require live provider access unnecessarily.

Browser materialization creates a new staging directory on every execution, writes current host/shard bridge sources, invokes JCO transpilation and release optimization, runs the real descriptor probe, finalizes hashes and publishes the newly produced files. Native component and descriptor targets recompile against the private Cargo intermediate directory. Browser support reconstructs vendor shims and the shard worker; its existing Typst font check preserves only immutable font seed inputs. Content-identical artifact publication receipts may preserve identical bytes after production, but do not skip any compiler, transpiler or descriptor execution.

The final browser asset copier receives only `playRuntimeComponentIds()` resolved through the newly published canonical catalog, plus the exact support and font owners. It does not blindly copy old removed component directories from the shared module-output root. Build staging and final page publication replace each output inventory completely.

## Shared Preparation Queue

The corrected generation's initial producers reached the repository-wide exclusive Cargo preparation queue. The descriptor-tool wrapper prepares its source owner, then its build script performs a second current-source preparation before invoking Cargo. Flow and graph generator slots likewise wait for current authored source preparation. The observed live tree contained their preparation processes and no Rust compiler descendant at that point; unrelated developers' Rust processes cannot establish progress for this build.

Read-only scope inspection found 120 root workspace packages, 138 artifact packages and 45 hub packages. Root owner discovery is authored with `[!.]*/Cargo.toml` and `[!.]*/**/📦️packages/🦀️rust/Cargo.toml`; these broad current-source scans run under the shared preparation lease. Native process sampling showed active CPU work with unsymbolized JavaScript frames, not a sleeping or abandoned lease. No source-owner preparation is bypassed, no lease is removed and no unrelated owner is stopped.

## Locked Dependency Closure

The Cad descriptor prerequisite failed with Cargo's real `cannot update the lock file .../🌎️hub/Cargo.lock because --locked was passed` error. The current first-party artifact-reference crate had been introduced throughout the authored dependency graph but was absent from that workspace lock. A scoped `bun nx exec --projects=workspace --excludeTaskDependencies -- cargo update --workspace --offline --manifest-path 🌎️hub/Cargo.toml` completed successfully, adding this first-party dependency and leaving all 190 registry dependencies unchanged. This follows the existing dependency infrastructure's workspace lock procedure without updating unrelated locks or disabling locked builds.

Both the scoped no-dependency check and full `cargo metadata --locked --offline --format-version 1 --manifest-path 🌎️hub/Cargo.toml` completed with exit 0 after the correction. The failed shipping graph was interrupted at only its owned bootstrap PID 64963; the launcher exited 130 and its named descendant wrappers stopped. Its partial `release-VEXZqG` output is not presented as a complete release. Another entirely cold generation was launched with the corrected lock and identical tested shipping/cache isolation contract, logging to `fresh-release-ship-locked.log`.

The new launcher allocated `release-6Z7Lxu`. Additional full locked offline metadata checks for the root framework workspace and `✏️s` artifact workspace both completed with exit 0, confirming their authored lock closures without modifying either lock.

Within this generation, actual Flow compilation completed its optimized `wasm-release` profile in 4 minutes 12 seconds. The native descriptor-tool compiler progressed concurrently from fresh Cranelift dependencies into current framework/schema crates. All observations refer to descendants of the active inner Nx PID 86657, not unrelated repository compilers.

The publication auditor's retained `verify-hashes` command was registered at diagnostic order 11.09 as `🔎️diagnostic🏢️semio-tech🎡️play🔏️digests` in both launch configurations. Parsed generated JSON and the exact seed row validated the workspace-scoped Nx command, quoted absolute ticket script path and unchanged command argument. The total authored release command registrations is now eleven.

The native descriptor-tool producer completed its optimized `release` compilation in 7 minutes 37 seconds. The early registry projection again withheld stale descriptors as expected. Its inferred dependency is `repo:generator-inputs`, so its subsequent scheduling and successful catalog/launch publication confirm that the preceding read-only Git fsmonitor `daemon terminated` stderr did not fail the dependency target. No Git configuration was changed. The final ordered catalog stage remains required to replace this deliberately incomplete early snapshot.

Surface subsequently completed optimized `wasm-release` compilation in 3 minutes 52 seconds. Source inspection of `SupportScript` confirmed that the shared browser shard worker is package-agnostic source emitted directly by `shardWorkerSource()`, alongside freshly staged vendor shims. It does not embed the early incomplete plugin catalog. Actual runtime component selection is resolved later through the final canonical registry before site publication.

Cad's actual metadata prerequisite subsequently passed locked dependency admission, then failed authored Cad artifact Rust compilation with 44 errors. The baseline and source repair handoff are retained in `📓️cad-compilation-blocker.md`. The parent directed preservation of the current initially empty cache generation while independent producers continue. Recovery may reuse only compiler work produced during this cold task and must retain cache bypass, locked dependency admission and eventual successful whole-graph consistency.

The direct AEC package unit gate is registered in both launch configurations at `4_gate` order 11.10 as `⚖️gate🏢️semio-tech🎡️play🏢️aec-building`. It uses the supported Nx test target with ship mode and both task caches bypassed. Its portable launcher includes task prerequisites for zero-touch use; the current cold recovery unit invocation may exclude already-completed prerequisites. No ephemeral generation path is embedded. Generated JSON and the identical authored seed row parsed and exact row equality was verified. Total authored registrations: twelve.

The parent subsequently assigned a bounded native shutdown trace to the editor agent. Any producer output while its temporary DEBUG source instrumentation or environment switch exists is provisional. The final consistent full graph rerun and optimized publication acceptance require explicit editor clearance that the temporary source and switch are removed. Current clean Cargo source fingerprints in the same initially empty generation must produce the accepted release; no final pages handoff precedes that clearance and complete graph success.

The editor subsequently confirmed all temporary StdIO shutdown instrumentation removed and no subsequent command includes `SEMIO_CAD_CLOSE_DIAGNOSTIC`. The bounded native diagnostic exposed a concrete TailSnapshot blockage; canonical source repair and validation remain parent/editor-owned. Final acceptance awaits that source readiness as well as the complete clean optimized graph. The current graph continues independent producers.

The direct canonical AEC describe command is registered in seed and generated launch configuration at `4_build` order 11.03 as `📇️catalog🏢️semio-tech🎡️play🏢️aec-building`. It includes prerequisites for portable zero-touch use, forces ship mode and bypasses both Nx task caches. The current generation scoped invocation excludes its already-successful component prerequisite. Exact authored row and generated JSON validation passed. Total authored launch registrations: thirteen.


Read-only current generated registry observation during active prerequisite graph: PLUGIN_BUILD_TARGETS still has only draw/puzzle and no extension/host rows, although source module directory list includes current live owners. This intermediate catalog is not runtime readiness. Official final catalog target after complete release producers must regenerate and validate the deployed rows before Vite/pages; source Play activation/pane gates remain final-stage checks. No hand edits of generated source or partial readiness claim.


Final-catalog admission audit: canonical GenerateScript intentionally calls renderCatalogFiles(..., "exclude") for development catalogs. Play final CatalogScript currently invokes that generator directly; PreparationScript verifies the deployed module list but does not itself reject withheld stale descriptor rows. Therefore a final Play catalog target should first use the existing canonical renderCatalogFiles(..., "refuse") admission before any generator writes, preserving normal development behavior. This is a precise release-only gate, not a catalog source/coverage workaround. Proposed proof is a fifth existing fresh-build source contract case with language-neutral catalogAdmission="refuse" and TypeScript AST oracle for strict-before-write ordering, followed by actual stale-catalog refusal and existing pipeline tests.


Strict final admission actual red: registered scoped Play source gate19461/Nx exit1, one new catalog-admission assertion failed and four existing fresh contracts passed; TypeScript AST oracle observed no strict call before publication. Production fix now calls canonical renderCatalogFiles(repoRoot, undefined, "refuse") before the normal generator. It rejects stale source/descriptor dependencies before writing any final catalog output. No dev generator source behavior changed. Current-source green proof follows via the same uncached scoped target/neutral fixture.


Strict final catalog source contract green42637 actual Nx exit0: five passed, zero failed, 9.3s target; neutral policy, TypeScript AST strict-before-write ordering, compiler/Nx/Vite isolation and independent yargs argument oracle all passed. Physical strict admission/catalog publication remains part of the final full release graph after producer prerequisites; no early catalog output mutation was performed.

## Current Release Boundary

The new bounded common-root/public prepared projection law actually passed in the existing private generation (session57842, one native law covering four histories/three lanes/commit and abort,1226 outside selection). Its compiler/source proof is retained in `📓️bounded-batch-group-root-preparation.md`. The original strict Process maintenance case also passed (session28071,158.719s,379 outside selection), directly confirmed from `🗑️generated/process3d-maintenance-valid-fixture-current.log`; schema and actor/archive/closure assertions were preserved.

The publication owner reports all four focused Process physical-demand laws current green (38171,4passed376outside,.133s); whole unfiltered380 Process gate93073 is still running. The editor owns newly discovered input-vector census/retirement closure and private child planning; no full producer/release readiness is claimed.

The original main graph is still in actual private Puzzle compilation with known historical graph failures and no terminal footer. The final whole321 current-source cache-bypass run must follow the pending owner proofs, then strict final catalog, runtime assembly and Vite/publication. Existing previous pages cannot be accepted as a fresh release. Source/cargo work already produced within this initially empty private generation may be reused through current-source fingerprints; no pre-task compiler target was reused.

Portable launch commands through11.88 are registered in both seed and generated launch, retaining exact existing selectors and budgets. `📋️pipeline-authored-paths.md` is the physical-only hunk-scoped file attribution for later ticket closure.

Current focused presence native log also emits Bun NO_COLOR/FORCE_COLOR warning with internal tty/node assert warning stack, then proceeds through owner preparation into Cargo artifact lock. The main log contains no draw-plugin explicit error/failure lines; color-warning stack alone is not a compiler failure proof. Healthy owned Puzzle wasm-opt remains active.

Current release compilation advanced through Puzzle materialize-release (actual owned wasm-opt CPU) into Sourcing component-release. Main remains live with no wholegraph footer. Current-source common-root/prepared-read public gate and strict Process maintenance now have real green receipts; broader Store543/546 leaves three assigned lifecycle roots and no readiness claim. Presence focused physical page caller repair is pending actual current compile. Final full321 must still follow owner convergence; no old dist output accepted.

Presence mounted local cancellation focused current-source gate11973 is actualGREEN1pass/1228outside/NxEXIT0,3m55s. Its parked worker fault page is correctly funded using exact phase demand; original Presence domain grant and root/value laws remain unchanged. Broader Store remains pending the other two assigned roots.

Read-only current main compiler audit found a concrete Process wood extension E0433: obsolete semio_framework_os_kernel::json reference. It is a distinct actual production compiler root beyond historical group MutationId.len errors; precise diagnostic sent to parent/publication owner for canonical pack-json import repair. No wholegraph footer; latest Sourcing component-release is productive.

Final current-source invocation plan: preserve the initially cold release-6Z7Lxu compiler target/build directories and both Nx cache-bypass flags, but select a new sibling nx-data-final workspace metadata directory. Root observed an unknown newly added target with existing graph data before a fresh scoped graph retry. Selecting fresh graph metadata for the final graph avoids projecting outdated target definitions without deleting shared or active metadata. This is a pending invocation choice, not a completed graph claim; main natural completion and current owner clearance still precede it.

The next main Sourcing extension stage exposed a real E0407 in the Semio owned disposer at1190: its next_close_byte_demand implementation does not match the dependency metadata ArtifactStoreOwnedDisposer trait seen by that in-flight compiler. Source scan additionally confirms Beams/Slabs/Windows catalogs still call removed kernel JSON APIs (two production calls and two test parser calls each). Exact compiler inventory/source paths sent to root/publication for canonical owner closure. This is actual additional release coverage, not a successful main graph.

Current original Store source inspection confirms member_group alias, Batch group field/getters and trait next_close_byte_demand exist. The seventeen Beams/Slabs compiler errors therefore do not prove missing current core definitions; they may reflect dependency metadata captured while the source interfaces were under concurrent development. No definitions restored or cache invalidation forced. Final stable-source graph must revalidate this closure; fresh Nx metadata alone is not a substitute for Cargo source fingerprint consistency.

Reinspection of permanent playFreshBuildEnvironment confirms ship mode, isolated Cargo target/build and Nx cache/metadata, both cache bypasses, plus deletion of inherited NX_TASK_* and NX_FORCE_REUSE_CACHED_GRAPH variables. Vite state is derived from selected NX_CACHE_DIRECTORY. Current seven-catalog canonical refresh explicitly uses the same initially-cold generation producer cargo/target+build, matching this shipping environment; native regressions use sibling cad-native outputs built within this task only. Final321current-source rerun remains pending owner closure and will use a new task-private graph metadata sibling without deleting the active generation.

## Final Current-Source Graph After Resume

The original build-fresh launcher finished incomplete on intentional interruption at762m12, so it cannot establish a whole-graph release. Its generation release-6Z7Lxu began cold and remains the selected producer store; repeated current-source proofs may reuse only compiler work born in this task. Once source owners freeze, the full canonical Play build must run with both Nx caches bypassed, ship mode and the same task-owned Cargo target/build directories; a fresh sibling Nx workspace-data directory forces a current graph projection without deleting healthy shared state. The helper playFreshBuildEnvironment supplies the production contract; final projection may use generation/nx-data-final. The new build-fresh public command still creates a new empty generation for independent zero-touch future rebuilds.

The read-only Nx history database has176 successes,257 failures and15 stopped rows from multiple native/producer runs, and no retained original root invocation rows. Those mixed totals do not give an exact original321 graph completion census, so no such count is inferred. Final acceptance requires the actual complete current-source graph footer, final catalog and emitted dist/pages, followed by the root-owned production/browser/publication gates.

Current graph focused4, native scheduling1, and complete Sourcing6 assertions have actual green receipts in their respective retained reports. Four Process catalog descriptors and the allocator-observed preborn adoption decision remain pending after a new codec import compiler-only failure; current source-ready retries are96978 and40399. No fresh-page or deployment readiness is claimed.
