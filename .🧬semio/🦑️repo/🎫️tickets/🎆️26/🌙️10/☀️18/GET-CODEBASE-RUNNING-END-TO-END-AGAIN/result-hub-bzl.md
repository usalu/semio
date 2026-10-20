# result-hub-bzl: the 🌎️hub area on Bazel (map, non-cached producers, staged fixes, proof plan)

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `BZ` = `L/🏗️build/🔧️tools/🏗️bazel`. Tools and staged set: `C/hub-bzl-tools/`. Wave 1020c STRICT phase: nothing was written outside `C/`, no `bun repo`, no Bazel, no cargo. All analysis ran the LIVE adapter functions (`planBazelTasksV1`, `bazelActionsV1`, `bazelTaskCoverageV1`, `bazelTaskInputsV1`, `bazelSourceIndexV1`) as pure functions over a static graph; the proof matrix after GO is scripted but NOT RUN.

## Verdict

1. The hub is Rust and TypeScript only. It has no Go: no `go.mod`, no `🐹️.go` and no `go.work` entry under `🌎️hub` (checked on `git ls-files` and on disk). The Go services live in the repo product (27 Go tasks, all under `🧰️framework/🛍️products/🦑️repo`). The Go service test of the proof is therefore `@semio-tech/repo-coordinator-go:test` (the coordinator server), not a hub project.
2. Hub model: 162 projects, 754 tasks (301 authored in 115 `📋️project.json` manifests, 453 inferred: 152 `build`, 150 `check`, 107 `test` native Cargo leaves and 44 `component` targets). 55 manifests carry authored targets, 60 carry none and get only inferred leaves, 47 projects are Cargo-only (44 component crates plus `semio-hub-stdio-catalog`, `semio-hub-stdio-mutation-bridge`, `semio-hub-stdio-test-oracle`). 154 tracked `Cargo.toml` (153 packages), 44 of them plugin components carrying 156 playground rows.
3. 652 of 754 hub tasks are Bazel-cacheable today (action cache, disk cache, remote cache ready); 102 are not: 13 continuous (`bazel run`), 44 `component` and 45 intrinsically live or side-effecting. The gap is not the cache machinery, it is the seven concrete items under "Non-cached producing steps". One is fixed by the staged set (Go toolchain identity); the others have a named owner or need the real graph (post-GO).
4. Pre-GO verification of the staged set: 4 new tests plus the existing `📥️fingerprint` (2), `📥️coverage` (6) and Cargo (7 + 1 skip) suites pass on the patched copy; a mutation (unpatched adapter) makes the new fingerprint test fail; the Go classifier agrees with the live coverage classifier on all 8,031 static-graph tasks.

## Hub map (live adapter on the static graph + inferred hub Cargo targets; the full per-target table is `C/hub-bzl-tools/hub-dispositions.tsv`, 754 rows: project, root, target, family, disposition, Bazel kind, cache, reason, exclusive, declared files, closure, uncached ancestors)

| area (root under `🌎️hub`) | projects | tasks | cached actions | exclusive native leaves | `bazel test` | uncached | interactive |
|---|---:|---:|---:|---:|---:|---:|---:|
| `📦️packages` (os-hub Rust, os-hub-ts) | 2 | 137 | 93 | 1 | 5 | 26 | 12 |
| `🧩️compositions/*` except stdio (35 composition directories: plugin crates, component crates, space, flow, gis, norm, ...) | 79 | 315 | 50 | 189 | 38 | 38 | 0 |
| `🧩️compositions/🗄️stdio` (plugin, 9 extensions, publication, oracle) | 25 | 130 | 7 | 59 | 44 | 20 | 0 |
| services: `🗿️artifact-authority`, `📇️directory`, `💡️inference`, `🏗️bootstrap`, `🔐️auth`, `🗄️stores`, `🚀️local-bootstrap`, `🚧️refusal`, `🛰️lag-rebootstrap`, `📊️observability`, `🔨️modules` | 44 | 133 | 1 | 130 | 2 | 0 | 0 |
| frontend `🔨️modules/🛡️admin` and `🧪️tests/*` (11 projects, renderer wgpu) | 12 | 39 | 1 | 30 | 2 | 5 | 1 |
| total | 162 | 754 | 152 | 409 | 91 | 89 | 13 |

(The 44 `component` targets split as 34 in the first composition row and 10 in the stdio row; `bazel test` counts the 76 authored script-routed Cargo tests and 15 bun tests only: the 107 native Cargo tests stay exclusive actions.)

By family and Bazel execution (`bazelActionsV1` kind): cargo 661 (443 cached actions, of which 409 are exclusive native leaves; 183 `test`-disposition, of which 76 authored script-routed tests run as `repo_test` and 107 native tests run as exclusive actions; 25 uncached; 10 interactive), bun-test 17 (15 `repo_test`, 2 uncached), bun-script 21 (4 actions, 14 uncached, 3 interactive), wasm 50 (6 actions, 44 uncached `component`), tsc 1 (`os-hub-ts:typecheck`), docker 2, playwright 2 (uncached). Uncached reasons: driver-owned 44, live 21, side-effecting 19, aggregate 4, diagnostic 1, continuous 13 (interactive `bazel run`).
Lazy minimal build (static closure): median closure 1 task (hub targets are leaves in the static graph); 51 targets carry uncached ancestors: the 44 `component` targets, 6 cached tests that consume component outputs, and `os-hub-ts:docker-image-build` (through `workspace:prepare` and `plugin-registry:generate`).

Model limits (honest): the static graph has authored targets only; inferred targets are replicated from `cargoTargets`/`componentBuildTargets` without their dependency edges and without the real `nativeSources` closure, so declared-file counts are lower bounds for native targets and upper bounds for authored `default` inputs. The real numbers come from `bun repo show project <name>` after GO (script below).

## Non-cached producing steps and what to do (ranked)

| # | Step | Evidence | Why not cached | Fix / owner |
|---|---|---|---|---|
| P1 | `component` x44 (all `semio-hub-*-component`, driver-owned) | cargo cdylib, describe, jco, `wasm-opt`, probe, module in one orchestrator task; outputs in the leased products store; each pulls 2 uncached ancestors: `workspace:deps-wasm-opt` (side-effecting `prepare`) and `@semio-tech/framework-plugin-web:support` (managed-output) | store-custody publication, Cargo freshness and stage cache own freshness | `pg-chain` (typed `repo_cargo` chain with early cutoff, `result-bzl-rust.md` S-series) must generate targets for the 36 `hubRoots` of `C/cargo-plan-tools/lane-roots.json` and make the two shared prerequisites cached or declared toolchains (one fix serves all 44 components and the 156 playground rows). Downstream cached targets that wait on it: `os-hub:test-all-features`, `os-hub:component-codec-check`, `os-hub:component-codec-budget-check`, `@semio-tech/gis-plugin:inference-discovery-{check,oracle}`, `@semio-tech/draw-plugin:guest-instance-check` (6 targets, `dependentTasksOutputFiles`): they can never be a warm 0-spawn run while `component` is uncached. |
| P2 | Go (repo coordinator) | the action key had no Go toolchain identity (only `go.work`/`go.work.sum`); `go` is host-provided (`~/.local/go`, 1.25.0), nothing pins it | upgrading or reinstalling Go would replay stale test and build results | STAGED (below): spawn-free toolchain digest in the key of every Go-family task. A digest-pinned hermetic Go toolchain (`tools: ["go"]` in the rexec contract) still needs published archive pins that cannot be fetched in this phase; `🔒️hermetic.json` accepts only `bun` today. |
| P3 | script-routed Cargo tasks (217 authored cached tasks in cargo roots; 45 of the 56 hub `📜️script.ts` call the ingress directly, e.g. `@semio-tech/draw-plugin:test` runs `runRepositoryCargoTests`) | `bazelCargoClassV1` recognises only `native cargo (check|build|test) --manifest`; these tasks are neither `exclusive` nor keyed by a Cargo toolchain digest; 76 run as `repo_test` | many ingress processes can start together (27 s fixed cost each, measured in `result-bzl-rust.md`); the ingress lease serializes the compile but each waiting process holds a bun | not staged: needs the real graph to see whether the inference marks them `native` (`show project @semio-tech/draw-plugin`). Candidate: a derived `parallelism: false` on targets whose script closure reaches the Cargo driver plus a Bazel mapping of `parallelism: false` on Rust-root tasks to `exclusive` (do not map it globally: 624 playground `activate-*` targets also declare it). Owner: `mem-gov` / project-assembly. |
| P4 | coarse inputs | 54 cached hub tasks declare more than 5,000 files in the static model: `@semio-tech/stdio-plugin` (34 targets: `stdioTree` = 28,114 files, `home-io-surface-check` 77,173), `@semio-tech/norm-plugin` (9, 11,166), `@semio-tech/hub-stdio-test-oracle` (7, 89,058 to 99,361), `@semio-tech/hub-compositions:native` (39,376), `os-hub-ts:typecheck` (63,891), `os-hub-ts:test` and `os-hub-admin:test` (about 7,400) | hand-authored globs (`stdioSurfaceReads` = whole framework `**/*.{rs,ts,js,json}`) | `input-precision` (derives from import and Cargo closures); the stdio `*-check` and test-oracle groups are the largest hub wins. |
| P5 | workspace-writing producers in `@semio-tech/stdio-plugin` (`native-codec-projection` outputs 3 tracked catalog files plus `✏️s/.../*/📜️artifact-definition.json` glob, `artifact-directory-wiring-generate`, `subset-directory-wiring-generate`, `catalog-root`, `composition-prepare`, `home-io-surface-check`, `editor-component-check`, `parent-removal-*`) | all `side-effecting: mutates-workspace` by the cache-fix classification | outputs are tracked files or an unbounded glob, so no exact `outputs` list exists | owner (`uncached-owners` pattern): split generate from check; a generator with a closed output list becomes a cached action whose restore writes the tracked file. |
| P6 | publish chain: `os-hub:publish`, `@semio-tech/hub-stdio-publication:publish`, `os-hub:trusted-catalog-bootstrap` (+`trusted-catalog-preflight`, aggregate) | intrinsically side-effecting; the bootstrap builds `wasm-release` stdio+gis cold on purpose (provenance rule in the hub publish-chain notes: never reuse on-disk wasm) | policy, not tooling | keep as uncached `repo_task`s (run only when requested). Open decision for the coordinator: a Bazel-cached wasm-release build keyed by exact sources plus toolchain digests is a stronger provenance than on-disk freshness, but contradicts the cold-build rule; do not enable without that decision. Note that stdio's own component linked once only as a 377 MB core.wasm (1,000,000-function link ceiling): a cached stdio component action must be measured before it is promised. |
| P7 | store-receipt targets (`os-hub-admin:build`, output `.🧾️store/build.json`) | the Bazel output is the receipt, the vite content lives in the store | a hit restores a receipt, not content | already covered by the receipt patch of `result-bzl-real.md` (restore verified, repair path); re-check on `os-hub-admin:build` in the proof. |

Intrinsic, stays uncached by design (45 tasks): 21 `live` (two-client, document growth, drills, browser-actor containment, wgpu journeys), 19 side-effecting (`backend-up|down`, `docker-image-build`, `setup`, `hostile-input-check`, `io-matrix`, stdio generators), 4 aggregate, 1 diagnostic, 13 continuous (`dev*`, `*-watch`, `scoped-presence-browser-serve`, `local-hub-owner`), plus the 44 `component` under P1.

## Staged set (nothing applied; `python3 C/hub-bzl-tools/apply.py` is a dry run, `--write` applies after GO)

`apply.py` checks every anchor (exactly one occurrence), refuses when the module directory exists with different content, checks the 256-character path budget (longest 95) and writes nothing unless all checks pass.

| Target | Change |
|---|---|
| new `BZ/🐹️go/🟦️.ts` | `bazelGoClassV1` (a task is Go exactly when the neutral family table says so, single source of truth with the coverage classifier), `bazelGoBinaryV1` (GOROOT then PATH, links resolved, no spawn), `bazelGoIdentityV1` (hash of `GOROOT/VERSION`, hash of the binary, host triple, values of `CGO_ENABLED GOARCH GOEXPERIMENT GOFLAGS GOOS GOTOOLCHAIN`), `bazelGoToolchainDigestV1` |
| new `BZ/🐹️go/🧬️schema/🔣️.json` | closed policy (binary names, version file, variable list, host map for 5 platforms) and the closed identity |
| new `BZ/🐹️go/🧪️tests/{🟦️.ts,🔣️.json}` | language-agnostic corpus: class rows, 10 toolchain rows |
| edit `BZ/📜️script.ts` (2 anchors) | import after the Cargo import; `taskFingerprint` takes the Go digest (memoized once per run, environment = ambient plus issued) after the Cargo block |
| edit `BZ/🧪️tests/📥️fingerprint/🟦️.ts` (2 bindings) | the extracted-function harness binds the two new names (without it the existing suite fails with `bazelGoClassV1 is not defined`) |

Verification (copies under `/private/tmp/hubbzl`, one runner at a time, explicit `./` paths; the overlay carries copies of ajv, typescript, no symlinks):
- `bun test ./🏗️bazel/🐹️go/🧪️tests/🟦️.ts` (patched overlay): 4 pass, 54 expectations: class rows equal the rule table; identity validates against Ajv and contains no host path; digest equals a Python `hashlib`/`json` oracle on all 10 rows, is identical for the same toolchain under two install paths and through GOROOT, PATH or a symlinked directory, ignores `HOME`/`GOCACHE`, differs for version, binary, `GOFLAGS`/`CGO_ENABLED`, host and absence; the adapter's own `taskFingerprint` (extracted from the patched source) changes with the Go digest for a Go task and not for a Rust one.
- Mutation: dropping the binary hash from the module fails the digest test; running the same test against the unpatched adapter fails the fingerprint test (3 pass, 1 fail).
- Patched overlay regression: `📥️fingerprint` 2 pass, `📥️coverage` 6 pass, Cargo suite 7 pass 1 skip (unpatched copy: identical). `📥️actions` could not load in the overlay (workspace package specifiers), the patch does not touch planning. `tsc --noEmit --strict --exactOptionalPropertyTypes` on the module: clean. `bun build --no-bundle` of the patched adapter: ok.
- Cross-check over the static graph (`go-agree.ts`): staged classifier vs `bazelTaskCoverageV1(...).family === "go"` on 8,031 planned tasks: 8,031 agree, 27 Go tasks.
- Not run: Windows and Linux (the digest uses `path.delimiter` and `go.exe`), the taxonomy gate for the new directory names `🐹️go`, `🧪️tests` below it (`🦀️cargo` set the precedent, but the Go ecosystem has its own `packageBoundaryRules`; if the gate refuses the name use another emoji prefix, nothing else depends on it).

## Proof plan after GO (5 hub-relevant targets; script `C/hub-bzl-tools/prove.py`, NOT RUN)

Run alone on a quiet host, after `show projects`, `tools setup nx|bazel` and the adapter set are live and after `apply.py --write`:

| id | target | what it shows | edit used for the one-edit step (one trailing newline, hash-verified revert) |
|---|---|---|---|
| go-service-test | `@semio-tech/repo-coordinator-go:test` | Go family, `repo_test`, toolchain identity in the key | `.../🖥️server/🎛️coordinator/📦️packages/🐹️go/📦️main/🐹️.go` |
| hub-rust-lib-check | `@semio-tech/hub-auth-client-rs:check` | hub Rust lib through the ingress, exclusive action, crate source lives outside the project root (`🔐️auth/🔌️client/🦀️.rs`) so it tests the real native closure | `🌎️hub/🔐️auth/🔌️client/🦀️.rs` |
| hub-frontend-build | `os-hub-admin:build` | vite frontend, store receipt | `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🟦️.tsx` |
| composition-build | `semio-hub-note-component:build` | composition crate (cdylib) native build, cached `dist/build/<mode>` | `🌎️hub/🧩️compositions/🗒️note/🧩️component/🦀️.rs` |
| hub-publication-test | `@semio-tech/hub-stdio-publication:test` | bun test as `bazel test`, stdio publication | `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/🟦️.ts` |

Per target: `--dry-run` under `--tool=bazel` and `--tool=nx` (planned tasks), cold (`--cold` = `cache clear`, global, ask the coordinator first), warm (expect 0 executed), after-clean (`--after-clean`: the harness copies the disk cache directory reported by `cache status`, runs `cache clear`, restores the directory; skipped above 3 GiB; expect 0 executed and all cache hits), one edit (expect executed >= 1 for the edited chain only), revert (expect 0 executed, cache hit). Spawns are the delta of the Bazel activity ledger (`builds, actions, executed, cacheHits`) that `bun repo --tool=bazel cache status` prints, an oracle independent of the run output. Additional informational run: `semio-hub-note-component:component` twice to put a number on P1 (expected: executes every time). Command: `python3 C/hub-bzl-tools/prove.py --cold --after-clean` (all five) or `--only N`. Real-graph checks to run first: `bun repo show project @semio-tech/draw-plugin` and `@semio-tech/hub-auth-client-rs` (are `test|check` marked native, what are their expanded inputs) to settle P3.

## Findings for other owners

1. `pg-chain`: the 44 hub `component` targets and the two shared uncached prerequisites are the whole hub warm-run cost for component-dependent tests (6 targets).
2. `input-precision`: the hub list under P4 is the concrete worklist (54 targets, stdio first).
3. `rexec`/`bzl-rbe`: `🔒️hermetic.json` allows `bun` only. A Go (and Rust) hermetic tool entry needs per-platform archive pins; this phase could not fetch them, so no hash was invented.
4. `bzl-lazy`/CLI: `after-clean` with a retained disk cache has no CLI verb (`cache clear` removes output bases and disk cache together); a `cache clear --keep-actions` scope would make that proof a one-liner.
5. `mem-gov`: P3, the number of concurrent bun ingress processes under `bazel test` for the 76 authored hub Cargo tests.
6. Static graph hazard seen, not hub-specific: `@semio-tech/gis-gismap-rs:verify-inference-mcp`, `@semio-tech/gis-plugin:inference-discovery-{check,oracle}` depend on `semio-hub-gis-component:component`, which exists only as an inferred project (absent in a manifest-only reconstruction); the real graph has it.

## Files

`C/hub-bzl-tools/`: `graph.ts` (static graph, scoped copy of the bzl-breadth reconstruction, writes `C/🗑️generated/hub-bzl`), `census.ts`, `census-live.ts` (live-adapter census + closures), `go-agree.ts`, `patches.py`, `apply.py`, `overlay-bazel.py` (base and patched copies of the Bazel adapter directory with absolute imports), `prove.py`, `hub-dispositions.tsv`, `staged/…/🐹️go/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/{🟦️.ts,🔣️.json}}`. This report.
Reproduce: `bun C/hub-bzl-tools/graph.ts && bun C/hub-bzl-tools/census.ts && bun C/hub-bzl-tools/census-live.ts`; overlay tests: `python3 C/hub-bzl-tools/overlay-bazel.py`, copy `node_modules/.bun/ajv@8.20.0/node_modules/.` and `node_modules/typescript` into `/private/tmp/hubbzl/patched/node_modules`, then `bun test ./🏗️bazel/🐹️go/🧪️tests/🟦️.ts` there.

## Cleanup

`C/🗑️generated/hub-bzl` and `/private/tmp/hubbzl` removed; no process left running; nothing outside `C/` written.
