# result-uncached-owners: every `cache: false` records its reason, 61 hardly-hermetic verification tasks become cached (staged, nothing applied)

Executor `uncached-owners`. `C` = this ticket. STAGE-ONLY: nothing outside `C/` was written (no manifest, no library file, no `package.json`, no `bun.lock`, no root `📜️script.ts`, no taxonomy). No `bun repo` command, no Nx or Bazel graph load, no cargo, no sub-agent. All verification ran in a scratch overlay copy (`/private/tmp/ucov`, removed at the end). Tools: `C/uncached-owners-tools/`.

## Verdict

1. **The goal "no `cache: false` without a recorded reason" holds in the overlay** with the cache-fix set and this set applied: 0 findings of any cache-correctness rule over 3,811 authored targets (the new rule `uncached-without-reason` found 379 on today's manifests, 0 after), and the static whole-graph census has **0 tasks** whose uncached reason is `declared` (before: **373** of 8,028).
2. **378 targets decided** (all authored `cache: false` targets that recorded no reason; `energy-model-rs:test-bestest-regenerate` is the one overlap and stays with cache-fix): **61 become cached** (a verification that reads only repository files), **18 become explicit interactive** (`continuous: true` plus a decision), **299 keep `cache: false`** with a data-recorded decision.
3. Of the **93 owner candidates** of `bzl-coverage-owner-candidates.md`: **29 cached** (12 actions, 17 tests), **64 uncached** with a recorded reason (40 live, 8 mutates-workspace, 7 aggregate, 5 diagnostic, 4 managed-output). Of the **171 host-state-by-name** tasks (the "172" of bzl-coverage, graph drift of one): 5 cached, 5 interactive, **161 uncached** with a reason (73 side-effecting, 48 live, 36 diagnostic, 2 aggregate, 2 managed-output).
4. The neutral vocabulary needed four reasons that did not exist; they are added to the closed schema (see "Reasons"). Bazel maps the class of a decision to `no-cache` with that class as the recorded reason (unchanged adapter), so each uncached task now says why in Bazel too.
5. Not proven by an execution: the 61 cached verdicts are static judgments (handler call graph, file profile, declared inputs, the cache-correctness law). They must be run once live after "PAUSE LIFTED" (see Open items).

## Before and after

Static whole-graph reconstruction (`C/bzl-coverage-tools/graph.ts`, run with this checkout; 8,028 tasks, 4,217 synthesised). `census.ts` replays `bazelCacheDecisionV1` and the neutral `taskDispositionV1`/`taskFamilyV1`; "before" reproduces the bzl-coverage census (373 declared, 1,308 interactive, 1,560 driver-owned, 624 live, 3,847 cached), "after" resolves every authored target through the overlay manifests and the overlay policy.

| Measure (all 8,028 tasks) | Before | After (cache-fix + this set) |
|---|---:|---:|
| uncached because `declared` (`cache: false`, no reason) | **373** | **0** |
| cached (action + test) | 3,847 | 3,771 |
| interactive (`continuous`) | 1,308 | 1,320 |
| uncached with a recorded class | 2,500 | 2,937 |
| refused (`undeclared`/`unbounded`) | 0 | 0 |

The cached total falls by 76 because the cache-fix set (not this one) demotes 137 mis-cached targets; this set alone adds 61. Attribution by id is exact because the two sets are disjoint.

Scope of this set (378 targets, status before -> after):

| before -> after | targets |
|---|---:|
| declared -> cached action (`canonical-architecture`, `check`) | 37 |
| declared -> cached test (`test*`) | 24 |
| declared -> interactive | 12 |
| already interactive (no decision recorded) -> interactive with decision | 6 |
| declared -> uncached `live` | 118 |
| declared -> uncached `side-effecting` | 115 |
| declared -> uncached `diagnostic` | 43 |
| declared -> uncached `aggregate` | 13 |
| declared -> uncached `managed-output` | 9 |
| declared -> uncached `driver-owned` | 1 |

By evidence: 183 rows `read` (handler read), 64 `analysis` (handler call graph shows workspace-anchored write sites or host-store infrastructure), 131 `name` (command word is a provisioning, publishing, mutating or measuring verb). Every row carries its note in `classification.json`.

Non-Rust-family tasks (cargo, wasm and the aggregate excluded, 4,034 tasks): uncached `declared` 264 -> 0; cached action+test 1,847 -> 1,750 (this set +34 non-Rust flips; 27 of the 61 flips are cargo-family; the rest is cache-fix demotions); interactive 1,295 -> 1,300.

## Reasons (schema-first, added to `💾️cache.json`)

The decision record stays the cache-fix shape `{version: 1, class, reason}` in `metadata.semio.cache` of the manifest target. This set adds four reasons to existing classes (no new class, so the Bazel schema, the adapter and the coverage `Reason` enum need no change):

| class / reason | meaning | targets |
|---|---|---:|
| `side-effecting/mutates-host-store` (new) | installs, provisions, repairs or prunes tools, dependencies, caches or images of this machine | 57 |
| `live/exercises-host-resources` (new) | exercises the real locks, lease queues, storage volumes, process tree, descriptors, clock or installed toolchain; the verdict is a property of this host and moment | 45 |
| `aggregate/reenters-build-tool` (new) | re-enters `bun repo run` for other targets instead of declaring them as dependencies | 11 |
| `diagnostic/reads-caller-input` (new) | the output is a function of arguments or stdin the caller supplies | 17 |
| `live/drives-running-process` | real hub, MCP gateway, browser, dev server or docker stack | 73 |
| `side-effecting/mutates-workspace` | regenerates, formats or scaffolds tracked or generated files not declared as outputs | 52 |
| `diagnostic/measures-environment` | benchmark, report, doctor, probe, repository-state check | 26 |
| `managed-output/outputs-in-managed-store` | wasm, site, storybook, cmake build whose product lives in the leased store behind a receipt | 9 |
| `side-effecting/publishes-remote` | release, image, package publication | 6 |
| `aggregate/orders-prerequisites`, `driver-owned/freshness-owned-by-driver` | pure ordering; dashboard compile through the driver | 2, 1 |
| `continuous/serves-until-stopped` (+ `continuous: true`) | runs until stopped: server, watcher, shell, debugger, TUI | 18 |
| `deterministic/check-without-outputs` (cached, `outputs: []`) | verification over repository files | 61 |

Neutral dispositions after the change (`bzl-coverage` `taskDispositionV1`): the 18 interactive targets are `interactive` (`bazel run`), the 299 are `uncached` with the class as the recorded reason, the 61 are `action`/`test` (`bazel test` when the name matches `^test`).

## What became cached (61)

Rule: a target is cached when its handler is a verification (oracle, twin, contract, source or fixture test), writes nothing workspace-anchored (the test-artifact sink excepted), spawns no live process (server, browser, docker, hub) and touches no host store, lease, clock or git state. Each flip sets `cache: true`, `outputs: []` if it declared none, and `inputs`: the owner's explicit list, else the inputs of the project's own cached `test` target, else `default`+`^default`, plus the three test-environment inputs (`SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`) unless `testEnv` is already named. The decision (`deterministic/check-without-outputs`) is inherited from the policy rows (`canonical-architecture` is an exact row; `test*`, `check*` heads; `go-test` and `-check` tails); no flip needed an explicit declaration.

- `canonical-architecture` (33): artifact twins and contract oracles of the owning component (`stdio-*`, `cad-cad-rs`, `sequence-sequence-rs`, `energy-model-rs`, `procedural-*`, `raster-js`, `note-note-js`, `puzzle-plugin`, `os-hub`, `s-*`, `framework-*`, `ui-react`, `semio-framework-3d`, `repo-lib`, `mit-bestand-*`, ...). Artifact-rust ones run in-process twins (`CanonicalArchitectureScript` of the standard runner); the others run bun, vitest or cargo laws.
- TypeScript artifact `test` (4): `dag-dag`, `demonstrator-playground`, `trinity-jack`, `trinity-rewriting` (77 siblings are cached already).
- Source tests (15): `mit-bestand-demonstrator:test`, `:test-runtime-catalog`, `semio-tech-play:test`, `framework-source-workspace:test-source`, `ui-rs:test-text-editor-source`, `repo-cli-go:test-entrypoint-source`, `framework-os-mcp:test-native-staging`, `framework-os-kernel:test-directory-lease-fixture`, `repo-next-storage:test-nx-cache`, `repo:test-bootstrap-original-invocation-source` (fixture-driven, fixed clock), `framework-async:twin`, `repo-dashboard-rs:test-dispatch`, `repo-dashboard-rs:test-source-freshness`, `repo-lib:go-test`, `repo-lib:test-fixture-ownership`.
- Pure selections of the process module (5): `test-budget`, `test-cargo-source-admission`, `test-runtime-environment`, `test-native-cache-descriptors` (parses source text), `test-native-image-observation` (temp-directory simulation).
- Owner-declared inputs (2): `repo-lib:test-cargo-workspace-ownership`, `repo-lib:test-dependency-policy-bootstrap`.
- Course validation (2): `@teaching/proctor:check` and `@teaching/architecture-course:check` run `proctor check <course>` (a pure validation of the course directory by the binary built from checked-in sources). Explicit extra inputs: the course tree, `PROCTOR_COURSE`, and for the course project the proctor tree and the cargo toolchain files.

Cached candidates the census sees as `test`: 24; as `action`: 37 (the neutral `Policy.test.target` matches only `^test`, see Open items).

## What stays uncached, and why (299 + 18 interactive)

- **live / drives-running-process (73):** acceptance and e2e against a real hub, MCP gateway, browser or dev server (`s-gis-services-native:*-check`, `os-hub:*-live-*`, `hub-renderer-wgpu:*`, `framework-os-dev:two-human*`, `*-budget`, `interaction-latency`, `time-travel`, `parity-journey`, `framework-renderer-wgpu:browser-*-acceptance`, `repo-dashboard-rs:test-*` journeys, `*:test-e2e`, docker stack checks, `proctor:health|capacity`). Hermetic variants would need a fixture server owned by the test.
- **live / exercises-host-resources (45):** the build tool's own lease, storage, process-tree and runtime suites: 15 of `framework-process:test-*` (leases, fairness, capacity, process tree, descriptors, native storage, publication), 21 `repo:test-*` (they open the real storage images via `withRepositoryTestArtifactsV1`, bazel storage, javascript store, rustup, next runtime, daemon journal), `repo-lib:test-build-caller-birth|test-go-executable-acceptance|test-original-default-preparation-control`, `workspace:test-build-tool-bazel*` (run real Bazel), `repo-mcp:test-install`, `stdio-assembly-rs:deletion-proof`, `repo-dashboard-rs:test-process-deadline`, `workspace:cpp-test`. The sibling suites of the same projects that only use temp directories are already cached (24 of 44 in `framework-process`), which is the owners' own boundary.
- **side-effecting / mutates-host-store (57):** `deps*`, `setup*`, `install`, `cargo-prepare|synchronize`, `cache-prune*`, docker image build, browser preparation, `ci-browser-prepare`, `proctor:backup|restore|erase|prune|rebuild`, `gis-gismap-js:tiles-prefetch`, `print:deps-*`, `s-rebuild:guest-framework-check` (raw `cargo check` into the shared target directory).
- **side-effecting / mutates-workspace (52):** generators, formatters and scaffolders that write tracked files without declared outputs (`cad-js:generate|fixture`, `norm-en1996-rs:regenerate-example-assets`, `stdio-plugin:*` composition, `workspace:format|clean-*|schema-parsers-*|agents-write`, `os-hub:hostile-input-check|io-matrix` which also publish an acceptance record, `plugin-registry:*`, `repo-test-domain:test-gc|clean|fixture-*`). A generator with a stable output list is the next cache candidate once an owner declares the outputs (`stdio-plugin:native-codec-projection` declares two).
- **diagnostic (43):** `measures-environment` 26 (bench, doctor, audit, reports, repository-state tests `repo-lib:test-cross-platform-bootstrap|preflight-reference-basis|registry-catalog-gitlink-boundary` that read git state outside the declared inputs); `reads-caller-input` 17 (digest, emit, owner-command, `sqlite-oracle`, `git-*` tool commands, oracle runs).
- **aggregate (13):** `reenters-build-tool` 11 (`workspace:verify-gate|layering|canonical-architecture|build-owners-sync`, `hub-*:test-native`, `s-rebuild:rebuild-all`, `s-gis-services-native:fixture-ownership-check|canonical-pair-composition-check`, `os-hub:trusted-catalog-preflight|runtime-fixture-graph`) and `orders-prerequisites` 2 (`workspace:build`, `workspace:cpp`).
- **managed-output (9), publishes-remote (6), driver-owned (1):** store-receipted builds (course, play, demonstrator, spatial-kernel and cad wasm, node-graph wasm, storybook, cmake configure/build), `publish`/`release`/`docker-image-publish`, `repo-dashboard-rs:build`.
- **interactive (18):** `trinity-jack-shell:run`, `repo-mcp-rs:run`, `repo-coordinator-rs:run`, `repo-cli-rs:repo|mcp`, `repo-dashboard-rs:run|preferences`, `repo-developer-tools:terminal|debug|debuggers`, `repo-lib:ui-storybook-dev`, `assets:logo-dev`, `workspace:os|semio`, `os-hub:scoped-presence-browser-serve|local-hub-owner`, `os-hub-ts:residency-watch|boot-watch`. Each gains `continuous: true`; nothing depends on them (checked over the static graph).

## Design of the staged set

Everything is data plus two anchored appliers; nothing is a source copy (the deepest staged path would exceed the path limit, so `plan.json` stores hashes instead of copies).

- Reproduce: `prep.sh` (manifest list, static graph, target inventory, handler analysis), `decide.ts`, `plan.ts`; `decide.ts` and `plan.ts` were re-run on a fresh `prep.sh` and produced byte-identical `classification.json` and `manifest-ops.json`.
- `classification.json`: the recorded reason for every target (id, manifest, disposition, class/reason, basis, note). Produced by `decide.ts` from explicit, cited groups; it fails on an unassigned or doubly assigned target.
- `manifest-ops.json`: the end state of each target (`cache`, `continuous`, `outputs`, `inputs`, decision), keyed by manifest and target, from `plan.ts`. `apply-manifests.ts` re-reads each manifest right before its write, edits by span (cache-fix `jsonspan.ts`), proves `parse(after) == parse(before) + intended keys`, re-compares the file before an atomic rename, leaves a target a peer already decided (reports it) and is idempotent.
- `edits.ts` + `apply-library.ts` + `plan.json`: anchored library edits over the cache-fix result, hash-checked per target (`base`, `applied` or `drift`; `--rebase` accepts drift when every anchor still occurs once). Edits: four reasons in `💾️cache.json`; the law rule `uncached-without-reason` in `⚖️laws/🔒️cache-correctness/🟦️.ts` plus its corpus schema; four corpus cases and two corrected old cases (`b:gen`, `p:fix` now carry a decision); six decision rows (four valid, two cross-class invalid) in the inference cache corpus; five rows in the Bazel actions fixture (one per new reason and an interactive row).
- `apply.sh [--dry-run] [--rebase]`: library first (the schema must admit a reason before any manifest records it, because the Bazel adapter validates the decision against the closed schema), then manifests. Refuses with exit 3 when the cache-fix set is not applied. **Series: `C/cache-fix-tools/apply.sh`, then `C/uncached-owners-tools/apply.sh`.**
- Language-agnostic tests and cross-checks: the law corpus (`uncached-with-a-reason-is-fine`, `uncached-without-a-reason`, `interactive-with-a-reason-is-fine`, `live-resource-test-with-a-reason-is-fine`), the decision corpus, the actions fixture, the strict Ajv validation those suites already run, and `verify-independent.py` (Python standard library only) which re-derives the property from the manifests and the closed schema.

## Verification (exact commands and outcomes)

| Step | Command (cwd `C/uncached-owners-tools` unless noted) | Outcome |
|---|---|---|
| inventory | `bun scan.ts` | 3,811 authored targets; 374 `cache:false` + 5 default-cache-false without decision (+50 continuous names with a policy decision) |
| handler analysis | `bun analyze.ts` (reuses `cache-fix-tools/handler/classify.ts`) | 379 records, 232 flagged for hand reads |
| decide / plan | `bun decide.ts`; `bun plan.ts` | 378 rows; 378 operations in 91 manifests (61 cached, 18 interactive, 299 uncached) |
| scratch manifest apply | `SEMIO_ROOT=/private/tmp/ucov-m bun apply-manifests.ts` twice | 91 manifests edited, then 0 (idempotent); minimal diffs reviewed |
| manifest schema | ajv over the 91 edited manifests against `🏗️build/🧬️schema/📋️project/🔣️.json` | 91 valid, 0 invalid |
| overlay | `./overlay.sh` (copies; applies cache-fix, then this set) | ready; library 6 files edited, 91 manifests |
| law + corpus | `bun test ./…/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts` in the overlay | 5 pass, 0 fail (repository clean against all 11 rules) |
| Bazel decisions | `bun test ./…/🏗️bazel/🧪️tests/📥️actions/🟦️.ts` | 10 pass, 2 skip, 0 fail |
| cache decision corpus | `bun test ./…/📥️inference/🧪️tests/💾️cache/🟦️.ts` | 9 pass, 1 fail: `artifact packages replay ...` needs a live process issuance (`Live Original process issuance required`); the same test fails identically in the untouched live tree (`Build inference requires real authority ancestry`), so it is environmental and unrelated |
| negative control | law over the live manifests with the staged policy | `uncached-without-reason` 379 (plus cache-fix's own findings); over the overlay: no finding of any rule |
| independent check | `python3 -I verify-independent.py <root> <manifest list>` | live: 374 without decision (+50 policy-named continuous); overlay: 0 without, 0 invalid, rc 0 |
| census | `bun census.ts before|after`; `bun report-data.ts` | tables above; the "before" numbers reproduce the bzl-coverage census |
| apply preconditions | `./apply.sh --dry-run` (live); `SEMIO_ROOT=/private/tmp/ucov ./apply.sh --dry-run` | live: exit 3 until cache-fix is applied; overlay: nothing left to do |

Not run: any `bun repo` command, an Nx or Bazel graph load, cargo, a real cache hit of a flipped task, `apply.sh` against the live tree (stage-only until "PAUSE LIFTED"), the live inference suite (`show projects` is the coordinator's).

## Findings

1. Six targets were already `continuous: true` with `cache: false` and no recorded decision (`ui-storybook-dev` and peers); they now record one.
2. Eleven verbs re-enter the build tool (`bun repo run ... --skip-cache`) instead of declaring dependencies; they cannot be cached or distributed until their prerequisites become `dependsOn` edges.
3. `workspace:build` is an `aggregate` that is a script, not a `repo:noop`, so Bazel runs it as an uncached action.
4. A `cache: false` target with an authored decision of class `deterministic` would contradict itself; the law does not yet reject it (no such target exists).
5. The census shows the neutral `test` disposition matches only `^test`; the 37 cached verdicts that are not named `test*` (33 `canonical-architecture`, 2 `check`, `twin`, `go-test`) are `action`s, not `bazel test` results.

## Open items

1. Apply after "PAUSE LIFTED": `C/cache-fix-tools/apply.sh`, then `C/uncached-owners-tools/apply.sh` (re-run `--dry-run` first; `--rebase` if cache-fix re-staged the schema or law). Then run the law test and the three suites above in the real tree.
2. Run the 61 cached verdicts once live and a second time to see the hit, e.g. `bun repo run-many -t canonical-architecture` and the listed `test*` ids from `classification.json` (`disposition: cached`); a failure means the verdict was wrong or the input list too small: revert that target to `uncached` with the observed reason.
3. `bzl-coverage`: extend `Policy.test.target` to `canonical-architecture`, `check`, `twin` and `go-test` so those 37 verdicts run as cached `bazel test` results.
4. Owners of the 45 `exercises-host-resources` suites: split the pure unit part from the real-resource part; the pure part is cacheable (24 of the 44 suites of `framework-process` were cached before this set, 29 are after).
5. Owners of the 11 `reenters-build-tool` verbs: replace the nested `repo run` by `dependsOn`.
6. `stdio-plugin:native-codec-projection` declares outputs and is deterministic enough to be a cached generator; `energy-model-rs:test-bestest-regenerate` stays with cache-fix.
7. 131 rows are judged by command name and the handler head; `classification.json` marks each row's basis so a later hand read can overturn a row without touching code.
8. Residual: `C/🗑️generated/cachefix-ov` holds a few timestamped module copies written by the cache-fix overlay loader that `scan.ts` and `plan.ts` reuse; they belong to cache-fix's generated folder.

## Changes

Created under `C/uncached-owners-tools/`: `prep.sh`, `scan.ts`, `mk-targets.ts`, `analyze.ts`, `show.ts`, `profile.ts`, `profile-records.ts`, `decide.ts`, `classification.json`, `plan.ts`, `manifest-ops.json`, `apply-manifests.ts`, `edits.ts`, `apply-library.ts`, `build-staged.ts`, `plan.json`, `apply.sh`, `overlay.sh`, `policy-at.ts`, `census.ts`, `report-data.ts`, `verify-independent.py`. This report. Nothing else was written outside `C/🗑️generated/uncached-owners`.

## Cleanup

`/private/tmp/ucov`, `/private/tmp/ucov-m`, `/private/tmp/ucov-meta`, `/private/tmp/ucov-neg.ts` and `C/🗑️generated/uncached-owners` (rows, records, graph, census, overlay lists) removed; every tool regenerates them. No Bazel or Nx server was started. Nothing was committed.
