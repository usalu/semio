# result-play-bzl: semio-tech-play on Bazel, cached chain and warm re-activation (2026-10-20, wave 1020c, stage only)

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Static work only: no `bun repo`, no Bazel, no cargo, no repo write outside `C/`. Tools: `C/play-bzl-tools/`. **Status: staged, ready for cold run (waiting for GO and the coordinator's heavy-run slot).**

## 0. Verdict

1. Under `--tool=bazel` the play chain is already cached everywhere except **five** producing steps. Of those, three belong to the generic plugin pipeline and are now owned by `pg-chain` (agreed split, section 5); two are play-owned and staged here.
2. The reason a warm re-activation is not near-instant today is one thing: the 35 `prepare-<lane>-react-prototype` tasks are `driver-owned` (uncached). Each runs `buildComponentClosureV1` (cargo, describe, jco, wasm-opt) inside the task. In the last measured closure (`result-bzl-lazy.md`: 107 tasks) exactly those 35 were the `no-cache` actions: after `bazel clean` 72 of 107 were disk hits and 35 re-ran the driver. After `pg-chain` lands (one cached `component` action per component, lane prepare = verdict) plus my staged set, the only uncached actions left in the activation closure are two cheap probes (`workspace:deps-wasm-opt`, `repo:generator-inputs`).
3. Found a soundness defect on the way: `repo:generator-inputs` is cached on library-only inputs but its output is a digest over the discovered plugin crates; a cache hit returns a stale digest, so `s-dev-composition:generate` (hence every `session-<lane>`) would not re-run after a plugin `Cargo.toml` or descriptor change. Staged fix: uncached probe with early cutoff.
4. Serve stays interactive (`bazel run`, continuous); `test-e2e` stays live (Playwright against a running server). Neither is a producing step.

## 1. Chain, step by step (dependencies read from `🏢️semio-tech/🎡️play/📋️project.json` and the generator `L/🏗️build/…/🎮️playground-targets/🟨️.mjs`)

Numbers: 35 play lanes, 68 distinct components in the union of lane boot closures (42 plugins, 26 extensions, 7 owner trees; sum of lane memberships 77: `demonstrator` 28, `imperative` 6, nine `stdio-*` extension lanes 2 each incl. `stdio`, the other 24 lanes 1 each). Computed with the real `runtimeComponentClosure` grammar over tracked Cargo manifests (`C/play-bzl-tools/closure.ts`).

| # | Step (target) | Tool work | Bazel disposition today | Gap | Fix / owner |
|---|---|---|---|---|---|
| 1 | `workspace:deps-wasm-opt` | provisions pinned Binaryen into the tools store | uncached (`side-effecting/mutates-host-store`), probes in about 1 s when present | action key of dependents does not carry the tool identity | `pg-chain`: `component` gets a `{runtime: wasm fingerprint}` input (the `fingerprint` subcommand exists and prints `binaryen:<ver>:<platform>:<sha256>`). The probe itself is accepted host state, one shared task per build |
| 2 | `workspace:deps-browsers` | Playwright chromium into repo cache | uncached host state; e2e only | none for activation | accepted (already installed, probe) |
| 3 | `@semio-tech/os-plugin-describe-rs:build` | cargo build of the descriptor emitter | cached cargo action (`hostPlatformOutputsV1`) | none | none |
| 4 | `semio-framework-canvas-font-assets:build`, `semio-framework-os-infinite:fonts` | font packing | cached, receipt/`.🧾️store` outputs | none | none |
| 5 | engine `<p>:wasm` x4 (`surface`, `editor`, `graph`, `puzzle`) | wasm-bindgen packages published to the products store | cached (`receiptedStoreTargetV1`, receipt is the restorable output; store content verified after the run, repaired alone when gone) | none | none |
| 6 | `@semio-tech/framework-plugin-web:support` | publishes preview2 shim vendor and shard worker | uncached (`managed-output`), outputs declared | could be a receipted cached target | `pg-chain` (receipt output) |
| 7 | `repo:generator-inputs` | hashes every discovered plugin manifest, descriptor, example and the catalog implementation graph into one digest file | **cached on library-only inputs** | stale digest on plugin edits (defect) | **staged here** (E3): uncached diagnostic probe, early cutoff on the digest |
| 8 | `@semio-tech/s-dev-composition:generate` (catalog; dependency of every `session-<lane>`) | renders the catalog into `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` | cached, but declares **91,035 files** (`default` = `✏️s/**/*` plus framework plugin); 12+ s hashing per run even on a hit | over-declared inputs | **staged here** (E2): `inputs: ["sharedGlobals"]`; the generator contract (`taxonomy.json` `plugin-registry`: 39 `inputPatterns` plus the `repo:generator-inputs` digest) supplies the exact set |
| 9 | `@semio-tech/plugin-registry:session-<lane>` x35 | pure function of runtime-closure descriptors | cached | none | none |
| 10 | per component: cargo wasm32-wasip2 cdylib, describe, jco transpile, wasm-opt, browser publish (68 components) | in-process driver inside `prepare-<lane>` | **uncached** (inside driver-owned prepare); the standalone `component` target is also `driver-owned` | the main gap | `pg-chain`: `<project>:component` cached via store receipt, deterministic, exact inputs; prepare-<lane> depends on them |
| 11 | `framework-os-dev:prepare-<lane>-react-prototype` x35 | calls the driver, validates session, fonts | uncached `driver-owned` | verdict only once 10 exists | `pg-chain`: verdict aggregate over component tasks |
| 12 | `framework-os-dev:activate-<lane>-react-prototype` x35 | digests staged modules, writes lane receipt and extension installs into `.cache/b/p/m/browser/prototype/runtime/react/<lane>` | cached (`cache:true`, deterministic) | input is only `dependentTasksOutputFiles`; the script also hashes component owner trees (`writeStagedSourceFreshness`) | `pg-chain`: add owner-root source inputs (requirement 4 sent) |
| 13 | play `prepare-prototype` | verifies the staged component directories | uncached (`driver-owned` by policy.json exact row) | pure verdict of its transitive outputs | **staged here** (E1) cached `deterministic/check-without-outputs`, outputs `[]` |
| 14 | play `activate-prototype` | merges the 35 lane receipts into the union receipt `dist/♻️activation/prototype` | uncached (same row) | pure function of transitive outputs | **staged here** (E1) cached `deterministic/pure-function-of-inputs`, output `dist/♻️activation/prototype` |
| 15 | play `serve` / `dev` (Vite dev server :6033) | long-running; reads staged modules, engines, fonts | interactive (`bazel run`) | by nature | stays; Vite dependency pre-bundling is the tool's own warm cache under `.cache/b/p/m/vite`, not an action |
| 16 | `prepare-e2e` -> `serve-e2e` -> `test-e2e` | opens an invocation-scoped service session (port 0), starts a frozen Vite, runs Playwright (11 tests, 1 worker) | live / continuous / side-effecting by nature | none | stays; readiness budget already fixed by `zt-e2e-fix` (`🧩️runtime/🧪️e2e/🔣️.json`) |

Not on this chain: `build-<lane>-react[-release]` (Vite production builds per lane) and play `build` (release pages); they are release-only and stay orchestrator targets. The e2e chain never runs a Vite production build.

## 2. Staged set (`C/play-bzl-tools/`, apply after GO)

`apply.py` (dry-run default, `--write`, all-or-nothing, anchored per target block, refuses when a base hash drifts unless `--accept-drift`; base hashes in `bases.json`; edited copies in `staged/<sha8>/`, source recorded in `staged/*/SOURCE`). `stage.py` re-records bases and copies. All three files are currently uncommitted-modified by peers, hence anchors per block.

| id | File | Change |
|---|---|---|
| E1 | `🏢️semio-tech/🎡️play/📋️project.json` | `prepare-prototype`: `cache:true`, decision `deterministic/check-without-outputs`; `activate-prototype`: `cache:true`, decision `deterministic/pure-function-of-inputs`. The manifest decision wins over the exact `driver-owned` rows in `L/⚡️caching/🔣️policy.json` (`declaredCacheDecisionV1` is read before `targetClassV1`). Inputs are unchanged (`activationSources` plus transitive dependent outputs) |
| E2 | `✏️s/🧑‍💻dev/📋️project.json` | `generate` gets `inputs: ["sharedGlobals"]` (drops 91,035 declared files) |
| E3 | `L/⚡️caching/📋️project.json` | `generator-inputs`: `cache:false`, decision `diagnostic/measures-environment` (re-scans, writes the digest, cutoff on unchanged bytes) |

Verification done (static): `bun C/play-bzl-tools/law-check.ts` runs the live `cacheCorrectnessFindingsV1` over the three edited targets as authored: 0 findings each. `apply.py` dry-run ok on the live tree (22257->22251, 5793->5844, 53201->53353 bytes). Not run: inference, `show projects`, any test of the generated graph.

Verification harness (run after GO, one heavy process at a time):
- `cold-run.sh preflight|cold|warm|probe comment|probe code|e2e` (the cold phase refuses without `PLAY_BZL_GO=1`). Preflight refuses when load >= 8, free disk < 60 GB or any foreign cargo/rustc/wasm-opt/jco/vite/bazel process runs.
- `trace-generate.sh` (+ `fs-trace.ts`, `envelope-run.ts`, `compare-reads.ts`): runs the catalog generator under a node:fs read tracer and prints every read that neither the contract patterns nor `registryCatalogInputPaths` declare. This is the proof that E2 dropped nothing the generator reads. Tracer proven on a toy script (sync and promises, not `Bun.file`); the envelope launcher and comparer type-check (`bun build`), not run.

## 3. Warm model and what to expect

After `pg-chain` plus E1-E3, for `@semio-tech/semio-tech-play:activate-prototype` the closure is about 35 sessions + 35 lane prepares + 35 lane activates + 68 components + roughly 12 engine/font/support/describe tasks + 2 play verifiers. Executed on a warm run: `deps-wasm-opt` and `generator-inputs` only (two small uncached probes); everything else is an action cache hit. Expected wall: Bazel client + closure workspace generation + fingerprinting, tens of seconds (previous 107-task closure: generation 6 s, load/analysis 3 s, warm answer 0.3 s; the former 12+ s tax of the 91k-file `generate` is gone with E2). One edited component source: that component's `component` action, its lane prepare verdict (unchanged receipts cut off if the module bytes are equal), its lane activate only if bytes or owner source changed, then the two play verifiers; sibling lanes stay hits.

Cold (estimate, unchanged order of magnitude): about 3 h for 68 components, now serialized by Bazel (`exclusive` cargo resource) instead of the driver's own jobserver; ingress fixed cost 27 s per call (`ingress-perf` reduces) adds about 30 min worst case over 68 component actions. Disk: keep 60 GB free (preflight). RAM: one cargo at a time; `CARGO_PROFILE_WASM_DEV_DEBUG=false` per memory.

## 4. Risks and open items

1. **Soundness of E2 depends on the contract.** The 39 `inputPatterns` plus the digest must cover everything `generate` reads. `trace-generate.sh` proves it; run it before trusting a warm hit. If it reports reads, add them to the contract's `inputPatterns` (taxonomy, library-frozen) rather than restoring `default`.
2. **Store content is not in Bazel's cache.** Receipt-only outputs (components, wasm, fonts, support) mean a fresh checkout with a warm Bazel remote cache restores receipts, fails store verification and re-executes each producer alone (uncached repair). Local warm re-activation is fully cached. Closing this needs a store-aware restore (publish-on-restore) in the adapter; not staged (core snapshot code, adapter owner).
3. **Transitive restore.** After a build the client restores only the selected task's snapshot (`dist/♻️activation/prototype`), not the transitive module directories; a launcher (`serve`) in a checkout whose store lacks them would not find them. Irrelevant locally, relevant for remote-cache-only machines. Same owner as 2.
4. **`stdio` is in the play union** (nine `stdio-*` lanes depend on it). Memory `project-stdio-wasm-component-unlinkable` (2026-09-07) says its component cannot link (1,000,000 function ceiling); the 2026-09-19 play recipe lists it as activated, so treat as unverified. A failing component fails its lanes exactly as the driver does today (`Native build failed` after `--keep-going`); run Bazel with `--keep_going`.
5. **Demonstrator** has the same exact `driver-owned` rows (`prepare-prototype`, `activate-prototype`, `activate-lanes-prototype`, `prepare-release`) in `policy.json`; `demo-bzl` needs the same manifest override (message sent to `pg-chain`, repeat to `demo-bzl`).
6. **Fleet wasm mutex.** Peers serialize raw wasm32 cargo through `/tmp/semio-wasm-build.lock`; the ingress has its own lease but a raw cargo outside it deadlocks the shared build dir (memory `feedback-fine-grain-locking-wasm-deadlock`). Preflight warns when the lock exists; the coordinator must hold the fleet quiet for the cold run.
7. **Bazel teardown of `serve`** (audit R3) and `--release` handling in `run-e2e.sh` were not re-checked; `cold-run.sh e2e` reuses `run-e2e.sh`, verify `lsof -nP -iTCP:6033 -sTCP:LISTEN` after.
8. `os-plugin-describe-rs:build` outputs the emitter into the store as a raw `hostPlatformOutputsV1` directory (restored directly, not receipted). Fine locally; flag for the store-aware restore work in 2.

## 5. Coordination

- `pg-chain` (a80e2ba1e2dfdac31) owns: `component-targets`, `playground-targets`, `project-assembly` wrapper, `target-policy`, `💾️cache.json` reason, os-dev prepare/build scripts, `support` target, `bazelCargoClassV1` second command shape (`exclusive` + toolchain key). I do not touch them. Requirements I sent: component per boot-closure id as lane dependency, descriptor pair + wasm fingerprint as `component` inputs, owner-root source inputs for activate, exclusive component actions and `--keep_going`, remote-cache limit note, policy.json row conflict.
- `input-precision` covers `s-dev-composition:typecheck` but not `:generate`; E2 is not in their rules (grep of `staged/rules.json`).
- Library paths are frozen until the coordinator lifts it; E3 touches `⚡️caching/📋️project.json` (library).

## 6. Files

Created: `C/result-play-bzl.md`, `C/play-bzl-tools/{apply.py,stage.py,bases.json,law-check.ts,closure.ts,cold-run.sh,trace-generate.sh,fs-trace.ts,envelope-run.ts,compare-reads.ts}`, `C/play-bzl-tools/staged/*`. Repo files outside `C/`: none written.

## 7. Commands run

`bun C/play-bzl-tools/closure.ts` (68-component union, 120 s cap, done); `python3 C/play-bzl-tools/apply.py` (dry-run ok); `bun C/play-bzl-tools/law-check.ts` (0 findings); `bun --preload fs-trace.ts` on a toy script in `/private/tmp/play-bzl` (works, removed); `bun build --target=bun` of the three harness scripts (ok); `bash -n cold-run.sh` (ok). Not run: anything under `bun repo`, Bazel, cargo, Playwright.
