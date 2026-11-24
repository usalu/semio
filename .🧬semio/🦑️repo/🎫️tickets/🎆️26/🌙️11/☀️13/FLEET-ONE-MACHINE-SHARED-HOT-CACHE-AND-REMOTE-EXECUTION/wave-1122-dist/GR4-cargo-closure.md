# GR4: what `repo cargo` does before cargo starts, and what was changed (2026-11-23)

LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, AUTH = `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/`.

## 1. Trace (proof workspace, `repo cargo --store=shared --configuration=prototype -- build -p p-leaf --offline`, SEMIO_REPO_PROFILE=1)

`repo cargo` never reads the project graph. `⌨️command/⚙️runtime/🟦️.ts:45` returns into `launchRepositoryCargoV1` before graph selection (`ensureTaskDependenciesV1`, `loadBuildGraphV1`). Its own phases:

| phase | what it does | graph? | measured (taskpolicy -b, load 45-65) |
|---|---|---|---|
| door.load / door.capacity | host-capacity admission (`withBuildCapacityV1`); fleet fan-out `offloadFanOutJobsV1` admits a light local grant only if some other machine has headroom | no | ~200 s queued; then "admitted cargo with 1 of 10 jobs" |
| door.capabilities / door.script | script envelope, router | no | short |
| source.capture | walk of the repo tool's own TS import closure (1,798 files, entry-specific memo under `~/Library/Caches/semio-repo/source-memo`) | no | 20-30 min when the memo is stale |
| build:implementation-source | second read of the same closure into the immutable image (`📥️source/🔏️closure`, sync `readFileSync` per file) | no | 20+ min |
| command stage | rustup generation custody ("generation scope" = `cargo.rustup.generation-scope`, a WRAPPER around the whole body, not a fixed cost: verify receipt + seal manifest), compiler store, leases | no | wrapper |
| `cargo fetch`, `cargo config get`, plan memo (`cargo metadata`, `--unit-graph`, keyed by manifest digests), source admission | cargo subprocesses; plan memo already memoized by digests (`🕸️units/📥️capture/🧠️memo`) | no | cargo failed at planning after ~155 s: the proof workspace lacks `cargo-features=["trim-paths"]` that the repo `prototype` profile needs (P1 used `-Z trim-paths`). Use a repo crate instead. |

The project graph is read only by `repo run <project>:<target>` and by `withRepositoryDeclaredCargoArtifactV1` (`🏃️process/🦀️cargo/📦️acquisition/🎟️issuer/🟦️.ts:34`). That function has NO caller in the tree (grep over `🧰️framework`), so wiring the manifest closure into it would change nothing. The semio-tech native build goes through `repo run`; its selected graph read is not touched by this lane.

## 2. The big finding: `taskpolicy -b` is the slowness
Same 300-file read, normal vs `taskpolicy -b` on the loaded host: 1.8 s vs more than 120 s (killed). Every long time above is measured under the `-b` the preamble requires. Unthrottled, a capture script (`wave-1122-dist/GR4-capture-bench.ts`, real tree, entry `⌨️command/🦀️cargo`, 1,798 files, load ~40):

| step | seconds |
|---|---|
| cold walk, no memo | 5.5 |
| memo holds (full replay) | 0.2 |
| memo with 20 files differing (partial repair, new) | 0.3, "memo held except 20 changed file(s) with unchanged imports" |
| replay after repair | 0.2 |

So an unthrottled `source.capture` is seconds, not minutes. The 20 s warm target cannot be demonstrated for the whole `repo cargo` while the binding `-b` addendum holds; I did not violate it. Please allow one unthrottled `repo cargo ... check -p semio-framework-async-timer` run after `[ok] site-build`, or lift the addendum for me.

## 3. Changes
1. `source.capture` partial memo repair (AUTH). A stale memo no longer forces a full walk: `replayNxAuthorizerMemoPartialV1` (`🧊️memo/🟦️.ts`) proves directories, absences, listings and packages, and returns the paths whose bytes differ; `replayCapturedPartialV1` (`🟦️.ts`) re-reads only those (at most `maximumChangedFiles` 64, schema) and accepts them when each keeps its import shape (`fileShapeV1`: literal import sites plus named relative JSON paths, stored per file in the memo as `shapes`). It rewrites the memo with the new digests. A changed import shape, a vanished file, or a memo without `shapes` falls back to the full walk, so the result always equals a fresh walk. Measured on the real tree: 20 changed files, 0.3 s. After a peer's edit of 20 of 1,795 files within 50 min the old code did a full walk. The first run after any edit of the authorizer files themselves still walks once (the memo carries the implementation digest).
2. `🧮️manifest-closure` (LIB `⚡️caching/🦀️cargo/🧮️manifest-closure/`): `manifestClosureV1(root, roots, {scope, tests, memo})` derives local crates from Cargo.toml files only (path, build, target-table and `workspace = true` dependencies, member globs and `exclude`, auto-members below the workspace root, test dependencies, `[patch]`/`[replace]` reported as `unsupported`), memoized by manifest digest and listing digest (`manifestClosureMemoV1`). Not wired into any command (see section 1); it is the replacement for `nativeDependencyRoots`-style closures where a caller needs one.
3. Door admission (`door.capacity`): no code change. Fan-out already exists and defaults to `distribute`; it did not fire because `repo fleet status` shows 12 of 13 workers "refusing (no headroom)" (the Windows worker is another platform), so `fleetHeadroomV1` is 0. Not a defect; the wait is the host ledger.

## 4. Tests (repo test-file, taskpolicy -b, SEMIO_LANGUAGE=en)
- `⚡️caching/🦀️cargo/🧮️manifest-closure/🧪️tests/🟦️.ts`: 5 pass / 0 fail. Oracle: closure equals `cargo metadata` local packages and members (globbed/excluded members, renamed, optional, build, target-specific, dev, non-member path dependency with its own dev-dependency), the per-package reach equals the metadata resolve below the package (with and without test dependencies, `--all-features`), memo recompute on edit and on a new member, escape/missing/patch refusals. Cargo runs with a clean environment outside the repo (the repo's `.cargo/config.toml` wrapper otherwise applies).
- `AUTH/🧪️tests/🧊️memo/🟦️.ts`: 3 pass / 0 fail, 4 new mutation cases (edit with unchanged imports served "memo", gained import "capture", lost import "capture", two files edited "memo"), each equal to a plain capture digest.
- `AUTH/🧪️tests/🧭️ancestry/🟦️.ts`: 8 pass / 0 fail (unthrottled).

## 5. Not done / open
- Warm `repo cargo` first-rustc time: not measured (throttle). Two real-crate runs (`semio-framework-async-timer`) were killed in `build:implementation-source`, which does 1,800 synchronous reads and is likewise only slow under `-b`.
- Wiring the manifest closure into the semio-tech build: the only declared-artifact consumer has no callers; `repo run`'s selected graph read is unchanged (GR1 options i and ii remain open).
- Proof workspace needs `cargo-features = ["trim-paths"]` to run through `repo cargo`.
- Lock `lock.graph-cargo` released at the end of this report.

## 6. Unthrottled measurement (addendum withdrawn), `repo cargo --store=shared --configuration=prototype -- check -p semio-framework-async-timer`, load ~50
| phase (profile ms) | run 1 (memo repaired by the new code) | run 2 (warm) |
|---|---|---|
| door.capacity admission itself (to `door.admitted`) | 0.2 s | 0.5 s |
| source.capture | 14.1 s (walk after my edit of the authorizer files) | 0.25 s (memo) |
| rustup verify + generation scope setup + compiler store | ~1 s | ~1.6 s (to workspace-body at 6.3 s) |
| workspace-body (fetch, config, plan memo, source admission, then cargo) | 101.5 s | 21.2 s, of which cargo itself 1.7 s (`Finished ... in 1.68s`, nothing to rustc: the crate was already built) |
| total wall | 129 s | 29 s |

Warm run 2 reaches cargo's own invocation after about 6.3 s + 19.5 s of planning inside the body. This crate needed no rustc, so "time to first rustc" is not observable; the measured fixed cost before cargo runs is ~26 s warm, above the 20 s target. The 19.5 s is cargo subprocess work (`cargo fetch --locked`, `cargo config get`, plan-memo recall, source admission) inside `cargo.compiler.workspace-body`; not broken down further (no per-subphase marks there). Candidate next step: add profile marks around those four steps, then memoize `cargo fetch` and `config get` by digest.

door.capacity no longer dominates: admission took 0.2-0.5 s, and `repo fleet status` now shows 1 of 13 workers refusing for headroom (was 12). Fan-out did not fire because it fires only when the local unit would queue; the host admitted immediately ("1 of 10 jobs").
