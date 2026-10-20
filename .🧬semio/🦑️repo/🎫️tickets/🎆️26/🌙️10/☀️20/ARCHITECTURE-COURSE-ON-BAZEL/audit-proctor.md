# Audit: proctor and course Rust under the Bazel backend

Read-only audit by aud-proctor, 2026-10-20 ~15:15 CEST. Nothing was built, run or edited; no cargo, bazel or git command was run.
Paths are relative to the repo root `/Users/ueli/Documents/semio`. Line numbers drift: the peer fleet edited `📋️project.json` (proctor 15:13, course) and the Bazel adapter `📜️script.ts` while this audit ran. Re-check line numbers before editing.

Short names: `PROC` = `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/` (proctor package), `BOOT` = `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`, `COURSE` = `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/`, `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/`, `NATIVE` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`, `REPOCARGO` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/`.

## Summary

1. The proctor's own cargo path does not go through `bun repo cargo --store=shared`. Production builds call raw `cargo` in-process (`NATIVE` 854, 860). The launcher argv exists only in `BOOT` 259-267, which is used by a test alone.
2. Proctor cache inputs under-cover the Rust closure. `cargoSources` and `cargoTests` rely on `^cargo`, and the nx `cargo` named input (`nx.json` 39) is `{projectRoot}/**/*.rs`. Framework crates keep their real modules in owner trees outside their `📦️packages/🦀️rust` root. So an edit to, say, `semio-framework-async` or `-replication` can produce a cache HIT for `@teaching/proctor:test`. The course `check` target, now `cache: true`, has the same gap, and the course `check` validates the course in Rust, so a stale hit there is a correctness bug.
3. The peer's per-crate Cargo action is only partly live. The classification, the `cpu:1024` exclusive resource and the toolchain key are wired into the live adapter. The `repo_cargo` manifest/argv/BUILD renderer and the `🦀️cargo/🏃️action` runner are not referenced by any live code. Proctor targets are `repo:script`, not `native`, so none of the Cargo wiring applies to them.
4. Every course path that needs the proctor (dev, check, test-e2e, docker-image-build, deploy-check) rebuilds it in-process. That costs about 27 s of ingress per call (peer measurement) plus host-wide whole-host compile leases. `deploy-check` builds it three times. Nothing consumes a declared Bazel output, so the paths are valid inside a Bazel action (all repo tasks are `host_state`, hence `no-sandbox`).

## 1. Proctor targets

File: `PROC📋️project.json` (read 15:13).

| Target | Executor and command | Cache, inputs, outputs | Lines |
|---|---|---|---|
| build | `repo:script`, `bun ./📜️script.ts build` | cache true, inputs `cargoSources`, outputs `{projectRoot}/.🧾️store/build.json` | 41-53 |
| test | `repo:script`, `bun ./📜️script.ts test` | cache true, inputs `cargoTests`, outputs none | 54-66 |
| dev | `repo:script`, `dev`, continuous, dashboard port 8791 | cache false | 67-85 |
| check | `repo:script`, `check` | cache true (changed from false), inputs `cargoTests` plus `{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/**/*` and the env vars | 86 onward |
| rebuild, health, backup, restore, erase, prune, capacity | `repo:script` | cache false | 95-end |
| docker | none in this project; the docker verbs live in the course project | | |

Named inputs: `cargoSources` (22-32) = `cargo`, `^cargo`, `sharedGlobals`, `Cargo.lock`, `🎓️teaching/🛂️proctor/**/*.rs`, `**/Cargo.toml`, `BOOT`, `📜️script.ts`. `cargoTests` (33-40) = `cargoSources`, `default`, course fixtures and schema.

`PROC📜️script.ts`: every verb except `build`, `test` and `capacity` builds the binary through `BOOT` (`buildProctor`, line 270 of `BOOT`) and runs it from a store copy. `build` calls `buildAdmittedCargoArtifactsV1` (line 41). `test` calls `runRepositoryCargoTests` (line 49), which runs `cargo test` for the three crates.

Note: `cargo` in `cargoSources` resolves from the root `nx.json` line 39 (`{projectRoot}/**/*.rs`, `Cargo.toml`, `.cargo/config.toml`, `rust-toolchain.toml`).

## 2. Cargo integration: how `build`, `test` and `check` reach cargo

What the proctor actually runs:

- `PROC📜️script.ts` `build` → `buildAdmittedCargoArtifactsV1` → provider `REPOCARGO🔌️provider/🟦️.ts:18` → `buildRepositoryCargoArtifacts` (`⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:41-50`) → `buildCargoArtifacts` (`NATIVE` 265) → `captureCargoArtifacts` (`NATIVE` 833). That last function does `const commandPort = options.commandPort ?? { command: "cargo", args: [] }` (854) and `spawn(commandPort.command, ...)` (860). No caller in production sets `commandPort` (grep: only `NATIVE` 854 and a live-replay law use the name). So proctor compiles spawn raw `cargo`, with `CARGO_TARGET_DIR` and build dir from the issued store (857-860).
- `test` → `REPOCARGO🎟️admission/🟦️.ts:27` calls `runCargoTestsV1` with no port argument, so the default `{command:"cargo"}` applies (`🧪️testing/🦀️cargo/🟦️.ts:202`).
- The only launcher path is `REPOCARGO🎟️admission/🟦️.ts:38-41` (`runRepositoryCargoCommandV1`, argv `--store=shared --configuration=…`), used by `bun repo cargo` itself and not by the proctor.
- `BOOT` 259-267 (`proctorBuildLaunch`) builds the exact `bun repo cargo --store=shared --configuration=… -- build --locked --package teaching-proctor --bin proctor --message-format=json-render-diagnostics` argv, run in the `🎓️teaching` workspace. Its only callers are `COURSE🧪️tests/🧱️local-stack/🟦️.ts` 358 and 369. It is dead code for production, so the brief's rule is not implemented.

Custody still holds: the root `.cargo/config.toml` sets `[build] rustc` to the ingress wrapper, and the issued store environment is what lets it compile. The deviation is about the entry point, not about bypassing custody. Decide whether to route the proctor through the launcher (wire `proctorBuildLaunch` as `commandPort`, see fix 5) or change the brief.

### Is the peer's per-crate Bazel Cargo action live?

Verdict: partly, from `result-bzl-rust.md` (in the ticket `☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`).

Live (grep of the source):
- `BZ📜️script.ts` 21 imports `bazelCargoClassV1`, `bazelCargoToolchainDigestV1`, `BAZEL_CARGO_RULE_V1`.
- Classification: `BZ📜️script.ts` ~271 marks a task `exclusive` only when `bazelCargoClassV1(task)` matches. That helper (`BZ🦀️cargo/🟦️.ts` 20-25) requires `metadata.semio.execution.kind === "native"` and the command `bun … native cargo (check|build|test) --manifest …` (the regex from `BZ🦀️cargo/🧬️schema/🔣️.json`).
- `BZ📜️script.ts` ~411 emits `exclusive = True`; `repo_task` (~540) takes `resource_set = cargo_resources if ctx.attr.exclusive` (~539). `cargo_resources` returns `cpu: 1024`, so Bazel serialises native cargo actions. The peer measured this.
- Toolchain identity: `BZ📜️script.ts` ~486 adds `bazelCargoToolchainDigestV1` to the task fingerprint for native cargo tasks.
- `BAZEL_CARGO_RULE_V1` is written into `tasks.bzl` (`BZ📜️script.ts` ~611), but no target instantiates `repo_cargo`.

Not live (no call site outside the module and its tests, verified by grep): `bazelCargoManifestV1`, `bazelCargoArgumentsV1`, `bazelCargoArtifactsV1`, `renderBazelCargoBuildV1` (`BZ🦀️cargo/🟦️.ts` 28-77) and the runner `BZ🦀️cargo/🏃️action/🟦️.ts`. The peer's open item 2 (artifact copy under store custody) and item 3 (typed `repo_cargo` for the wasm chain) are therefore not done.

Native targets live: `🧰️framework/.../🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs:62` emits `bun <script> native cargo build|check|test --manifest …` with `outputs: dist/build/<mode>` for build and `parallelism: false` for check and test, and `🕸️graph/🦀️rust/🟦️.ts:29` reads it back.

Consequence for the proctor: its targets are explicit `repo:script` targets and override any inferred native target. They are not `native`, so they get no `exclusive` resource, no toolchain key, no exact `nativeSources` inputs and no `dist/build/<mode>` output.

Process view (confirmed in source): every non-hermetic repo task carries `host_state` (`BZ📜️script.ts` ~411), which sets `no-sandbox` (~523). Repo tasks therefore run in the real checkout, which is why the cargo store and absolute paths work.

## 3. Teaching Cargo workspace and closure

`🎓️teaching/Cargo.toml` (`[workspace]`, lines 5-9) has three members: `PROC`, `🛂️proctor/🔨️modules/🧠️domain/📦️packages/🦀️rust` and `🛂️proctor/🔨️modules/🧱️base/📦️packages/🦀️rust`. The root `Cargo.toml` excludes `🎓️teaching` (its `exclude` list, line 803). The proctor binary is `[[bin]] proctor` at `BOOT🦀️.rs` (`PROC/Cargo.toml` 21-23).

Dependencies on framework crates (`[workspace.dependencies]`, `🎓️teaching/Cargo.toml` 26-35): `semio-framework-course`, `-course-presence`, `-server`, `-async`, `-replication`, `-hash`, `-dispatch-macros`, `-test-fixtures`, all as `path = "../🧰️framework/…"`.

Closure measured from `🎓️teaching/Cargo.lock` (218 `[[package]]` entries in total, traversal by a small Python script at `aud-proctor-tools/closure.py`):
- Closure from `teaching-proctor`: 218 packages = 83 path crates (3 teaching + 80 `semio-framework-*`) + 135 registry crates. The lock does not record dev-only edges, so `ureq`, `tungstenite` and `sha2` are included even though they are dev dependencies.
- Closure from `teaching-proctor-base`: 188 packages, 81 path crates (80 framework).
- Pins diverging from the root `Cargo.lock`: 71 of 135 registry pins in the teaching lock have a different name and version in the root lock, and 64 match. Only the matching 64 can reuse artifacts from the root workspace in the shared target dir. Expect a cold compile of about 71 extra registry crates plus the 80 framework crates.
- The peer's result-bzl-man counts 73 path crates; the difference is dev dependencies.

## 4. Which course targets need the proctor, and how they find it

Course target map (`COURSE📋️project.json`, `COURSE📜️script.ts`):

| Course target | Line in project.json | Proctor use | Where |
|---|---|---|---|
| dev | 109 (`📜️script.ts` 45-49) | `runDevStack` builds in-process (`🧱️stack/🟦️.ts` 266 first build, 356 rebuild on source change), launches `serve` from the pinned store copy (295), supervises | `COURSE🧱️stack/🟦️.ts` |
| check | 297 (`📜️script.ts` 83-87) | `checkCourse` (`🚀️deploy/🟦️.ts` 227-236): `buildProctor` in dev mode, then `launch(["check", course])` | `COURSE🚀️deploy/🟦️.ts:231-232` |
| test-e2e | 246 (`📜️script.ts` 71-75) | Builds once per run (`🎭️e2e/🟦️.ts` 354) unless `--proctor <executable>`; with that flag, `launchProctor` copies the file into `.🧬semio/🎓️teaching/proctor-bin` (`BOOT` 473-476) | `COURSE🎭️e2e/🟦️.ts` 85-103, 248, 353-355 |
| docker-image-build | 318 (`📜️script.ts` 93-97) | `buildCourseImage` (`🚀️deploy/🐳️docker-image/🟦️.ts`): Linux x64 gate (32), `buildProctor` release (41), check launch (42), `withExecutable` streams the binary into the docker context (44-48); the image packaging requires native linux release (`📦️image/🟦️.ts` 110-112) | `COURSE🚀️deploy/🐳️docker-image/🟦️.ts` |
| deploy-check | 385 (`📜️script.ts` 121-129) | Readiness steps: `checkCourse`, `buildCourseImage`, `checkCourseImage`, `checkCourseStack` (`⚖️readiness/🟦️.ts` 43-47) and the e2e gate | `COURSE🚀️deploy/⚖️readiness/🟦️.ts` |

None of them reads a binary from a Bazel output. The binary is always produced in-process and pinned in the compiler store (`BOOT` 301, `pinExecutableArtifact`). `--proctor` is the only path override, and it exists only on test-e2e.

Does this work inside a Bazel action?
- Output declaration: not needed. Repo tasks are `repo_task` actions with one snapshot directory (`BZ📜️script.ts` ~541, `.snapshot`). The proctor's real output lives in the store, not in the snapshot, so nothing is lost. `build`'s store receipt `.🧾️store/build.json` is the only declared output (see fix 6).
- Target dir outside execroot: fine, because `host_state` makes the action `no-sandbox` (`BZ📜️script.ts` ~523) and the cargo target and store live under the real checkout, reached through `.cargo/config.toml` (`target-dir`, `build-dir`) and `runtime.workspace` (`executeTask` `runTask` cwd at `BZ📜️script.ts` ~695).
- Relative `--proctor` paths: `runTask` runs with cwd = the task's `options.cwd` (the course package dir, `BZ📜️script.ts` ~695), not the user's shell or the repo root. A relative `--proctor` path therefore resolves against `COURSE`. Under Nx the same holds. Fix 7.
- Platform: `docker-image-build` throws immediately on macOS and Windows (`🐳️docker-image/🟦️.ts:32`). That is by design, but it conflicts with the brief's "every course target on macOS, Linux and Windows". The brief needs an explicit exemption.

## 5. Locks, ingress cost and serialization

- Ingress (peer measurement, `result-bzl-rust.md`, load 75): about 27 s per `bun repo cargo` call: store 10.8 s, `fetch --locked` and config views 10.1 s, unit graph 0.7 s, source admission 1.1 s, writer lease 0.8 s, scheduler 2.2 s, cargo itself 0.8 s. The in-process proctor path runs the same preparation (`⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:45` and `BOOT` 283 `cargoWorkspacePreparationInvocationV1`). Expect the same order; the 27 s is the peer's figure for the launcher path, not measured here.
- Proctor builds per course run, by target: dev 1 (plus one per source change while supervised), check 1, test-e2e 1 (0 with `--proctor`), docker-image-build 1 (release), deploy-check 3 (checkCourse dev, e2e dev, image release). That is about 80 s of fixed ingress in deploy-check alone, before compile waits.
- Whole-host compile lease: `NATIVE` 857 wraps the whole cargo run in `withCompilerCapacityScopeV1({request:{kind:"whole-host"}})`. The lease mode is exclusive (`🎫️lease/🟦️.ts` 61), so proctor compiles serialise against every other compile on the host for the duration of cargo. In Bazel these proctor actions are not `exclusive` (their class is not native), so they start concurrently, hold a worker process (RepoBuild max 2, RepoCheck and RepoTest max 8, RepoScript max 4, `🧰️…/🏗️build/🔣️policy.json` kinds) and wait.
- Compiler charge `repo:compiler-charge:<image>:<mode>` (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` 31) is exclusive but held only while the reservation is acquired and released in `finally`. It does not serialise whole builds.
- Cache store `repository-cache-charge` (`📦️budget/📥️opening/🏪️store/🟦️.ts` 41) is also reservation-scoped and not a serialiser.
- Per-recipe lease `acquireCargoTargetRecipeV1` (`NATIVE` 289) serialises identical recipes only.
- In-process bodies: `BZ📜️script.ts` ~786 `inProcessLockV1` serialises in-process actions inside one worker process. Worker-kind `build` (max 2) therefore gives at most two concurrent proctor builds, and each one still waits on the whole-host lease.
- Binary copies: each build pins a copy (`pinExecutableArtifact`) and hashes it. The docker path streams the whole binary through a tar context into `docker build` (`🐳️docker-image/🟦️.ts` 44-48).

## 6. Prioritised fixes

P1 — correctness of the cache (do first):
1. Course `check` is now `cache: true` (`COURSE📋️project.json` ~303-316) with inputs that exclude every framework Rust source. Its validation logic is Rust, so a change in `semio-framework-course` or `-server` validation can produce a stale pass. Either revert to `cache: false` now, or add the framework Rust owner trees (fix 2) to its inputs.
2. Proctor `cargoSources` and `cargoTests` (`PROC📋️project.json` 22-40) must cover the owner trees of the 80 framework path crates. Quick and sound: generate the list from `Cargo.lock` path packages and their owner roots (`packageRoot`, `BOOT` 373-376), and add it as `{workspaceRoot}/…/**/*.rs` plus each `Cargo.toml`. Do not hand-list; the inference already computes the exact closure (`nativeSources`), and the peer's measurement shows exact inputs avoid over-invalidation.
3. Add a language-agnostic test (AGENTS rule): touch a file in `⏳️async/⏰️timer-wheel` and in `📡️replication`, then assert that `proctor:test` and course `check` are cache misses. Use the Bazel `cache status` or an input-expansion probe, not a cargo build.

P1 — Bazel shape:
4. Make the proctor's Cargo targets native (or otherwise classify them as `native`): inference-produced `build|check|test` with exact `nativeSources`, Bazel `exclusive`, toolchain key, and a declared output. Add a `--bin proctor` selection and copy the executable to `dist/build/<mode>/proctor`. Then dev, check, test-e2e `--proctor` and docker-image-build consume that declared file (absolute repo path) instead of rebuilding. That removes about 27 s per call and one whole-host compile per run.
5. Decide the cargo entry point. Either set `commandPort` in `buildRepositoryCargoArtifacts` (`⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:47`) and the test admission port (`REPOCARGO🎟️admission/🟦️.ts:27`) to the launcher argv already written in `BOOT` 263, or amend the brief. Either way delete `proctorBuildLaunch` or wire it; do not keep a dead twin of the real path.
6. `deploy-check` builds the dev proctor twice (checkCourse 231 and e2e 354) and the release once. Within one Bazel run, share one artifact (fix 4 solves this) or pass the `ProctorArtifact` into `checkCourseDeployment` (`COURSE🚀️deploy/⚖️readiness/🟦️.ts` 32-48).

P2 — behaviour and platform:
7. `--proctor` relative path resolves against the package dir under Bazel and Nx (`BZ📜️script.ts` ~695). Resolve it against the repo root in `COURSE🎭️e2e/🟦️.ts` (where `plan.proctor` is set, 102-103, and used at 248) or reject relative paths with a clear message.
8. Decide the platform rule for `docker-image-build`, `-check`, `-publish`, `docker-stack-*`: on macOS and Windows the Bazel target should report "not applicable on this platform" rather than fail inside a build. The brief currently demands macOS and Windows for every course target, which cannot hold for these.
9. `docker-image-publish` has no `dependsOn` on `docker-image-build` (`COURSE📋️project.json` 342). `publishCourseImage` checks the image exists (`🐳️docker-image/🟦️.ts` 67), so this is safe but ordering is by user. Add `dependsOn` for clarity.
10. The peer's `repo_cargo` rule, `renderBazelCargoBuildV1`, the manifest and argv helpers, and the runner `🦀️cargo/🏃️action` are unreferenced. Under the greenfield rules, either wire them (peer open items 2 and 3) or delete them. This belongs to the peer fleet, not to the course.
11. `build`'s declared output is a store receipt, while the binary lives in the store. A Bazel cache HIT restores the receipt only. Nothing consumes the binary, so this is accepted, but it should be recorded in the proctor's `📜️script.ts` header comment, matching the peer's memo on stale receipts.

## 7. Limits of this audit

- No graph was generated, so the claim that `^cargo` reaches all 80 framework path crates rests on the peer's dependency-edge measurement (`result-bzl-man.md` line 21) plus the source (`🕸️dependency-edges/🟨️.mjs` 187-193). I did not verify the live edge list.
- The 27 s ingress figure is the peer's measurement. I did not measure the proctor path.
- The closure numbers come from the lock file with a dev-edge caveat (section 3).
- Lines in `📋️project.json` and `📜️script.ts` moved during the audit; re-check before editing.
