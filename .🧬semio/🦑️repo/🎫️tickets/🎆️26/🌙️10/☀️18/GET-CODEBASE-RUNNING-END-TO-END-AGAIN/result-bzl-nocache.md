# result-bzl-nocache: no-cache census and conversions (executor bzl-nocache, wave 1020d, stage only)

READY: staged `C/bzl-nocache-tools/apply.ts` (dry run default, `--write` all-or-nothing, 21 ops in 15 manifests, 20 independent of peers, 1 waits for demo-bzl); verified in an overlay of copies; nothing outside `C/` written; waiting for the coordinator's GO. After GO: `bun apply.ts --write`, `bun validate.ts`, `bun law.ts <overlay>` equivalent on the live tree, `python3 -I verify-independent.py`, then `prove.sh light|bericht|course|wasm|storybook` (double run, 2nd must execute 0 local actions).

`C` = this ticket. `R` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`. Static work only: no `bun repo`, no Bazel, no cargo, no git write. Tools: `C/bzl-nocache-tools/` (`graph.ts`, `census.ts`, `slice.ts`, `closure.ts`, `analyze.ts`, `show-target.py`, `handler.py`, `staged/plan.ts` + `staged/ops.json`, `apply.ts`, `validate.ts`, `law.ts`, `verify-independent.py`, `prove.sh`). Scratch: `/private/tmp/bzl-nocache/` (overlay copies of the 15 manifests, the merged law root) and `C/🗑️generated/bzl-nocache*` (graph, census rows, records).

## 0. Verdict

1. The census of the live tree (policy of cache-fix and uncached-owners applied, 14:56) is **8,031 tasks: 3,771 cached, 1,320 interactive (`bazel run`), 2,940 uncached**. The coordinator's 2,870 is the earlier census; the difference is the 137 demotions of cache-fix minus 61 flips of uncached-owners plus targets added since. Every uncached task has a recorded class except three that were added after the 14:56 decisions (fixed here).
2. **85 % of the uncached tasks are not manifest targets.** 2,497 are synthesised per playground lane (156 lanes) by `playground-targets/🟨️.mjs`, the file of `pg-chain`. The authored manifests carry 443.
3. Of the 443 authored, **16 become cached** and 3 get their missing reason (this set); the rest stay uncached with a precise, evidenced reason (section 4). The remaining 427 are blessing tools that rewrite tracked files, live suites against running services, host-state provisioning and diagnostics, which are the tasks the brief keeps uncached, plus 103 fixture generators that need a per-generator refactor. The largest authored class that looked convertible (103 `fixture-generator` targets) is not, with evidence (section 4.2).
4. The synthesised class is the lever: 1,560 of the 2,497 (prepare, profile-less prepare aliases, wgpu activate) are converted by the staged `pg-chain` set (read from `pg-chain-tools/edits.py`); 624 native `run`/`smoke` are live; 313 Vite lane bundles stay with `pg-chain` (section 3).
5. Course closure: after this set only `test-e2e` (live) is uncached. Play and demonstrator: the remaining driver-owned tasks belong to `play-bzl`, `demo-bzl` and `pg-chain` (section 5).

## 1. Counts (static whole-graph reconstruction of `bzl-coverage-tools/graph.ts`, live policy module, live manifests)

| | tasks | cached | interactive | uncached |
|---|---:|---:|---:|---:|
| before (live tree) | 8,031 | 3,771 | 1,320 | **2,940** |
| after this set (overlay) | 8,031 | 3,787 | 1,320 | **2,924** |
| projected with the staged `pg-chain` set | 8,031 | 5,347 | 1,320 | **1,364** |
| projected with `pg-chain` and a sound Vite lane input contract (section 3) | 8,031 | 5,659 | 1,320 | **1,052** |

Uncached by recorded reason, before: driver-owned 1,571, live 744, side-effecting 546, diagnostic 46, aggregate 18, managed-output 12, undecided 3 (sum 2,940). After this set: driver-owned 1,571, live 745, side-effecting 541, diagnostic 47, aggregate 14, managed-output 6, undecided 0 (sum 2,924).

Uncached after this set by origin and shape:

| group | n | owner | verdict |
|---|---:|---|---|
| synth `prepare-<lane>-{react,wgpu,native}-{prototype,release}` + profile-less `prepare-<lane>-{react,wgpu}` + `activate-<lane>-wgpu-*` | 1,560 | pg-chain | converted by their staged set (cached verdict over `component` receipts) |
| synth `run-/smoke-<lane>-native-*` | 624 | pg-chain file | live (real native host window), stay |
| synth `build-<lane>-react`, `build-<lane>-react-release`, `build-all-playground-cdn-sites` | 313 | pg-chain file | section 3 |
| authored `fixture-generator-*` (103) | 103 | stdio/fixtures | stay, section 4.2 |
| authored live / drives-running-process | 75 | - | stay (e2e, acceptance against hub, MCP, browser, docker, dev server) |
| authored side-effecting / mutates-workspace (without fixture generators) | 61 | - | stay, section 4.1 |
| authored side-effecting / mutates-host-store | 58 | bzl-hermetic domain | stay (installs, provisions, prunes tools, dependencies, images) |
| authored live / exercises-host-resources | 46 | bzl-hermetic domain | stay (real locks, storage images, process tree) |
| authored diagnostic (measures-environment 30, reads-caller-input 17) | 47 | - | stay |
| authored driver-owned (demonstrator and play prepare/activate, `repo-client|repo-cli-go:build`, `repo-dashboard-rs:build|compile`, `framework-os-dev:plugin`) | 11 | play-bzl, demo-bzl, pg-chain | section 5 |
| authored aggregate / reenters-build-tool | 11 | - | stay until they declare `dependsOn` |
| authored managed-output (print fonts, cmake configure/build, `framework-plugin-web:support`, play and demonstrator `build`) | 6 | - | section 4.3 |
| authored publishes-remote | 6 | - | stay |
| authored aggregate / orders-prerequisites (`print:build`, `print:build-viz`, `bericht:build` pending) | 3 | - | section 4.3 |

## 2. What is converted (staged, `staged/plan.ts` is the single source, `staged/ops.json` the derived end states)

Method: each spec derives the `after` target from the live `before` target; `apply.ts` re-reads every manifest, refuses (exit 3) a target that is neither its `before` nor its `after`, edits only the changed top-level keys by span, proves `parse(after) == intended`, writes through a temporary file after a last re-read, and rolls everything back if one rename fails. `--status` prints pending, applied, drift or pending-requires per target. Idempotent (second `--write` in the overlay: 0 manifests).

| # | Target | Before | After | Evidence (all read in this session) |
|---|---|---|---|---|
| 1-3 | `framework-os-node-graph-rs:wasm`, `s-spatial-kernel-semio-session-rs:wasm`, `s-cad-composition-rs:wasm` | `cache: false`, managed-output | `cache: true` (receipt output already declared; `forwardAllArgs` dropped on the last two) | same receipted wasm producer (`buildAdmittedWasmWebV1` / `buildRepositoryWasmWebV1` + `verifyProducedWasmPackageV1`) as the eight cached siblings (surface, editor, puzzle, flow, actor, host, renderer-wgpu); `WasmScript.run()` ignores arguments |
| 4 | `@teaching/architecture-course:build` | `cache: false`, outputs `[]` | cached, output `{projectRoot}/.🧾️store/build.json`, no `forwardAllArgs`, inputs: default, `^default`, the whole course tree, `PROCTOR_URL`, dependency outputs | `runViteBuild` publishes through `withRepositoryArtifactDestinationV1`, which records the receipt (`⚡️caching/📦️artifacts/🟦️.ts`, `recordStoreContentV1`); identical declaration to the cached deck `build`; the course tree is declared because the Vite configuration path is a string the command closure does not follow |
| 5 | `semio-framework-canvas:stage-runtime-assets` | `cache: false`, two store directories | cached, receipt `.🧾️store/stage-runtime-assets.json` | `stageAdmittedArtifactTreesV1` -> provider `🔌️provider/🟦️.ts` uses `withRepositoryArtifactDestinationV1`; the owner already declares exact inputs |
| 6 | `workspace:build-storybook` | `cache: false` | cached, receipt `.🧾️store/build-storybook.json`, no `forwardAllArgs` | `buildRepositoryStorybookV1` uses `withRepositoryArtifactDestinationV1`; the script throws on extra arguments. Inputs stay the root `production` (whole repository): input-precision's class |
| 7-8 | `repo-test-runner-go:build`, `repo-metrics-go:build` | `cache: false`, outputs `[]`, mutates-workspace by name | cached verdict `deterministic/check-without-outputs` | `runCanonicalGoBuild(PACKAGE_DIR, ["./..."])` passes no `-o`; `buildGoArtifactsV1` stages only an `-o` output, so `go build ./...` publishes nothing: a compile check beside the cached `go test` |
| 9 | `print-viz-inference:build` | `cache: false`, mutates-workspace by name | cached verdict, inputs add the whole print tree, no `forwardAllArgs` | `runBuildScript` is two `tsc --noEmit` runs plus a barrel load; the program reads `📓️print/{🔨️modules,🚪️io,🧬️schema,🧪️tests}`, which `production` excludes |
| 10-11 | `ui-styling:generate`, `ui-styling-py:generate` | `cache: false`, mutates-workspace | `repo:noop` ordering node over `ui-styling-tokens:generate` | both handlers only print `[nx-generate] styling artifacts ready` |
| 12 | `repo-vscode:build` (+ `build-vsix` depends on `compile-extension`) | `cache: false`, second owner of `📤️dist/<mode>` | `repo:noop` alias of `compile-extension` | `BuildScript extends CompileExtensionScript {}`: same command, same output; the overlap cache-fix found is removed |
| 13-16 | `workspace:build`, `workspace:cpp`, `workspace:prepare`, `framework-os-dev:build` | `repo:script`, uncached `aggregate/orders-prerequisites` | `repo:noop` | handlers only log (`BuildScript` without slice, `CppScript all`, `setup prepare`, `CompleteScript`); finding 3 of uncached-owners |
| 17 | `mit-bestand-bericht:build` | uncached aggregate | cached verdict over the declared outputs of the three `build-*` dependencies | `CompleteScript` reads the `%PDF-` header of the published report PDFs only. **Waits for `demo-bzl`**: it moves those outputs from `dist/documents/<id>` (never produced) to the products-store slot; `apply.ts` reports `PENDING` until then |
| 18-20 | `repo:cache-retire-inactive-preview|-retire`, `repo:test-cargo-retirement` | uncached, no reason (added after 14:56) | uncached with `diagnostic/measures-environment`, `side-effecting/mutates-host-store`, `live/exercises-host-resources` | new targets, host store retirement and compiler-store dispatch |

Verification:
- `bun apply.ts --write` on the overlay (`SEMIO_ROOT=/private/tmp/bzl-nocache/ov`): 14 manifests, 20 targets, then 0; `--status` all applied.
- `bun validate.ts <overlay>`: 15 manifests against `🏗️build/🧬️schema/📋️project/🔣️.json` with Ajv: 0 invalid.
- `bun law.ts <overlay>`: the cache-correctness law (`cacheCorrectnessRepositoryFindingsV1`, production `targetPolicy`) over 3,814 records with the overlay manifests: **0 findings**. On the live tree the law reports 3 (`uncached-without-reason` for item 18-20), fixed by this set. Negative control (course `build` with `outputs: []`): `producer-without-outputs` reported.
- `bun census.ts` with the overlay: 16 tasks flip `uncached -> action`; every flipped task resolves through the live policy to a closed decision; none is refused (`unbounded`/`unclassified`: 0).
- `python3 -I verify-independent.py <overlay>`: standard-library re-derivation of the invariants (closed class/reason, receipt path form, no forwarded arguments on cached targets, no process fields on noops): 20 applied, 1 pending, 0 violations.
- Not run: any `bun repo` command, an Nx or Bazel graph load, a cache hit of a converted task. These are the post-GO proofs.

## 3. The synthesised class (2,497 tasks, `playground-targets/🟨️.mjs`, owned by `pg-chain`)

I did not edit this file or `component-targets`, `project-assembly`, `target-policy` or the os-dev scripts. Message sent to `pg-chain` (id acda06302c1e657ff) with the split; no reply yet. Reading their staged `edits.py` (18:07):
- covered: `component` per crate cached on receipts, `prepare-<lane>-react|wgpu|native-<profile>` cached verdicts over the closure's `component` targets, profile-less aliases (they spread the profile target), `activate-<lane>-wgpu-<profile>` cached with the owner-root input, `support` receipt.
- not covered by design: `build-<lane>-react`, `build-<lane>-react-release` (312 tasks) "stay with the Vite compiler's own state, because no input contract names the sources it bundles", and `build-all-playground-cdn-sites` (publishes to a remote).
- **Proposal for the 312 lane bundles** (for `pg-chain` to take, or for a follow-up after their apply): the missing contract is a path list. The Vite production build bundles the framework tree (`🧰️framework/**` outside tests and generated directories), the os-dev builder and distribution directories next to the project root (`🧑‍💻dev/{🏗️builder,🚚️distribution,♻️activation}`, which `production` of the project root does not cover) and the lane's plugin modules (already covered by the transitive receipts). Declaring `{workspaceRoot}/🧰️framework/**/*` (minus `🧪️tests`, `🤖️generated`, `📤️dist`) plus the lane's owner roots makes the cached bundle sound at the price of hashing the framework tree once per distinct lane key. With that, 312 more tasks become cacheable (table in section 1).
- stay uncached: `run-/smoke-<lane>-native-*` (624, a real native host window) and the continuous `serve`/`dev`/`mcp` targets.

## 4. Why the rest stays uncached (evidence)

### 4.1 `mutates-workspace` (61 + 103 fixture generators)
Read, not inferred by name: `ui-styling:generate`, `ui-styling-py:generate`, the two Go builds, `print-viz-inference:build`, `repo-vscode:build` were misjudged by name and are converted above. Stay: `channel-version generate` (rewrites every registered consumer, found by `git grep`), `regenerate-example(-assets)`, `generate-register-catalog`, `cad-js:generate|fixture` and `remodel-plugin:regenerate-example` (bless tracked golden assets with `--ignored` cargo tests), `plugin-registry:generate` (neutral publication root chosen at run time), `stdio-plugin:*` (36-artifact wiring over tracked definitions, spawns cargo), `workspace:{agents-write,format,generate-config,clean-*,schema-parsers-*,components}`, `repo-lib:workspaces-write`, `cargo-workspaces:members-write`, `repo-test-domain:test-{clean,gc,fixture-*}`, `assets:logo` (headless browser video capture through ffmpeg, not bit-reproducible), `print:generate-viz|generate-font-metrics`, `os-plugin-describe-rs:describe` (arguments are the input), `hub-stdio-publication:publish`.

### 4.2 The 103 `fixture-generator-*` targets: not convertible with the current producers
Read `note-1-any` and `stdio-gif-87a-any`: (a) `generate` **and** `manifests` both run the codec and write the committed fixtures into the sibling subset directories (the `manifests` command is not stdout-only); (b) `generate` merges its `testEvidence` index into the existing committed file (`previous` read back from `indexPath`), so the output is a function of its own previous output; (c) the codecs are raw `cargo run`/`cargo build` via `spawnSync` outside the compiler admission (a parallel Bazel action would race the shared target directory, see memory on fine-grain cargo locking); (d) 48 projects each have a bespoke program, the outputs are heterogeneous (`physical-directories`, `ENGINE_BIN`). These are blessing tools whose reproduction is verified by `test fixture reproduce`. Path to cached: split each into a pure `reproduce` (scratch `SEMIO_FIXTURE_OUT`, no index merge, admitted cargo) with the fixture directory as declared output; that is a per-generator refactor, not a manifest edit.

### 4.3 Managed-output and aggregates
- `print:fonts`: the print producer publishes through `withArtifactScratchV1`/`withArtifactDestinationTaskV1` of the print module, which never calls `recordStoreContentV1`; a receipt output would record nothing. Needs a 3-line receipt call after `preparePrintMaterialV1` (print owner), then the same cached declaration as the wasm producers.
- `print:build`, `print:build-viz`: `complete` reads the published PDFs from the store; no declared producer in `print` publishes them, so a verdict would not be a function of its inputs.
- `semio-tech-play:build`, `mit-bestand-demonstrator:build`: unpinned network (`prefetchMapTiles` fetches OSM and OpenFreeMap tiles into a cache directory no declared input pins) and, for play, the release driver with 13 jobs. Pinned snapshot needs one online run to generate a digest list; not stageable offline.
- `cpp-configure`, `cpp-build`: the cmake store has no receipt path (`🛠️cmake` never records).
- `framework-plugin-web:support`: pg-chain (receipt output, staged).
- `ui-styling-tokens:fonts`: downloads Google fonts (user agent string, no digest).
- `repo-client|repo-cli-go:build`, `repo-dashboard-rs:build|compile`: write the running CLI/dashboard binary in place (`🗃️bin/<mode>/repo`); restoring it from a cache over a running binary is the macOS in-place overwrite hazard. Stay driver-owned.

### 4.4 Live, host-state and diagnostic
`live` (121): every one runs a real hub, MCP gateway, browser, native window, docker stack, proctor or the build tool's own storage, lease and process-tree machinery (listed in the census rows; `framework-process:test-*` 15, `repo:test-*` 21, `repo-dashboard-rs:test-*`, `os-hub*:*-e2e|*-check`). `mutates-host-store` (58) installs or prunes tools, dependencies, images and caches. `diagnostic` (47) are benches, doctors, reports, git-state tests and argument-driven tools. None is a function of declared inputs; the host-resource suites are `bzl-hermetic`'s domain (a sandbox plus declared resources could make the pure parts cacheable).

## 5. Closures of the three products (before -> after this set -> expected after the peers)

| closure | tasks | uncached before | after this set | remaining after peers (play-bzl, demo-bzl, pg-chain) |
|---|---:|---:|---:|---|
| `@teaching/architecture-course` (`build test check typecheck dev test-e2e`) | 12 | 2 | **1** | `test-e2e` (live) only |
| `@semio-tech/semio-tech-play` (`build test serve test-e2e activate-prototype`) | 114 | 41 | 41 | 36 lane prepares + `activate-prototype` -> cached by pg-chain/play-bzl; left: `build` (tiles, release driver), `prepare-e2e`, `deps-browsers` (host), `test-e2e` (live), probes `deps-wasm-opt`, `generator-inputs` |
| `@semio-tech/mit-bestand-demonstrator` (`build test serve test-e2e activate-prototype canonical-architecture`) | 46 | 27 | 26 | 22 driver-owned (demo-bzl, pg-chain) -> cached; left: `build` (tiles), `prepare-e2e`, `deps-browsers`, `test-e2e` |
| `@semio-tech/mit-bestand-bericht` (`build test check-generated`) | 11 | 4 | 4 -> 3 after demo-bzl | `print:deps-tex`, `print:deps-tectonic` (pinned provisioning probes), `print:fonts` (4.3) |

## 6. Overlaps and apply order

- `pg-chain`: none of my targets; the synthesised shapes are theirs (section 3). Message sent.
- `demo-bzl`: `bericht:build` (item 17) requires its `build-*` outputs edit; apply `demo-bzl-tools/apply.py --write` first, then `bzl-nocache-tools/apply.ts --write` (it reports `PENDING` and applies the other 20 either way; rerun after `demo-bzl`).
- `play-bzl`, `hub-bzl`, `cache-fix`, `uncached-owners`: disjoint targets (they edit the play manifest, hub manifests and library; my 3 reason ops sit in `⚡️caching/📋️project.json` beside `cache-fix` ops on other targets of the same manifest, resolved by span edits).
- `input-precision`: items 4, 6, 9 and 17 declare explicit inputs because the targets become cached; they can narrow them later. `workspace:build-storybook` keeps the root `production` (whole-repository class). Not edited: `rules.json`.
- `bzl-hermetic`: none of my conversions needs a sandbox decision; items 7-11, 13-16 are hermetic-eligible (pure verdicts and noops).

## 7. Risks to prove after GO (each reverts alone by restoring the target's `before`, recorded in `ops.json`)

1. Receipts of items 4-6: the receipt is written only if the publish path records it (verified by reading), and `withStoreReceiptsV1` repairs a hit whose store content is gone. If a receipt stays `unwritten`, revert that target.
2. Items 1-3: the three wasm targets had an authored `cache: false` whose reason is not recorded in the manifests; the siblings prove the mechanism. Heavy (cargo wasm32), run `prove.sh wasm` alone and under the fleet wasm mutex.
3. Item 6 hashes the whole repository (root `production`): the 12 s hash cost applies on every hit until input-precision narrows it.
4. Item 12: `repo-vscode:build` as a noop removes the second owner of the `📤️dist/<mode>` output; `build-vsix` still writes inside the directory `compile-extension` owns (stays uncached, reason recorded).

## 8. Files

Created: `C/result-bzl-nocache.md`, `C/bzl-nocache-tools/{graph.ts,census.ts,slice.ts,closure.ts,analyze.ts,policy-at.ts,show-target.py,handler.py,apply.ts,validate.ts,law.ts,verify-independent.py,prove.sh}`, `C/bzl-nocache-tools/staged/{plan.ts,ops.json}`. Generated and removable: `C/🗑️generated/bzl-nocache`, `C/🗑️generated/bzl-nocache-after`, `/private/tmp/bzl-nocache`. Repo files outside `C/`: none written.
