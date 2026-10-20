# Closure Audit: @teaching/architecture-course on Bazel (read-only)

Auditor: aud-closure. Static reading only. No builds, no Bazel runs, no `--dry-run`, no source edits.
Paths are relative to `/Users/ueli/Documents/semio` unless absolute. Unprefixed paths inside a row are relative to `SITE` (or to the named owner). Scripts and raw outputs are in `🗑️generated/aud-closure/` (`closure.py`, `gap.py`, `gap2.py`, `gap2.json`, `pkgnames.json`, `summ.py`, `deps-summary.txt`).
The live Bazel adapter (`…/🔧️tools/🏗️bazel/📜️script.ts`, 819 lines) and `🔣️taxonomy.json` were modified by peers during the audit (mtime 14:57-14:58). Line numbers below are from the reads at about 15:10 and may drift.

Short names: `PKG` = `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript`, `SITE` = `🎓️teaching/🏛️architecture/📚️course`, `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `INF` = `…/🏗️build/🕸️graph/📥️inference`.

## 1. Summary

- The course manifest declares 16 targets. All 16 are registered in the router (`PKG/📜️script.ts:131`).
- Structurally the closure is wired through the Bazel backend. Two targets are cached (`test`, `typecheck`). Six explicit dependency projects feed them.
- Blocking for the brief's goals:
  - P1-1: the release and deploy verbs cannot be selected through Bazel without an ambient `SEMIO_BUILD_MODE=release`.
  - P1-2: the cached `test` and `typecheck` under-declare inputs. 27 TS and JSON files that the tests or tsconfig reach are outside the default inputs, so they can replay stale green results.
  - P1-3: the cached `test` depends on host state (Node binary, ticket env, loopback sockets) that is not in its key.
  - P1-4: the Rust proctor is never a cached Bazel product. Every path that uses it rebuilds it inside uncached actions.

## 2. Course Targets (`PKG/📋️project.json`)

| Target (line) | Kind | Explicit dependsOn | Inputs | Outputs | Cache / continuous | Risk |
|---|---|---|---|---|---|---|
| preflight (81) | host preflight | none | adapter default `{projectRoot}/**/*` | none | false | low |
| ci-browser-prepare (95) | system packages (`playwright install-deps chromium`, `📜️script.ts:40`) | none | none | none | false | Linux and root only |
| dev (109) | continuous: site dev server plus proctor build and launch (`🧱️stack/🟦️.ts:383-415`) | assets:build, ui-styling-tokens:generate | adapter default | none | false, continuous | cargo; ports 6061, 6063, 8793; ready marker fake (P2-1) |
| dev-site (162) | continuous: vite dev server | same | adapter default | none | false, continuous | fixed port |
| build (192) | vite site build | same | `default` + env `PROCTOR_URL` (207) | none (vite stages to products store) | false | deterministic but uncached (P2-6) |
| publish (217) | release site build and CDN staging (`🚀️deploy/🌐️site/🟦️.ts:109-124`) | same | adapter default | `{projectRoot}/dist/release/pages/course` (230) | false | needs caller release mode (P1-1) |
| test (233) | vitest, node env, `quick` / `long` / `exhaustive` | none explicit; generators inferred from router closure (see P3-1) | `default` (240) | none | **true** | P1-2, P1-3 |
| test-e2e (246) | Playwright, dev and rehearsal stacks, Chromium, ports, cargo | same as dev | adapter default | none | false | `node` on PATH; Chromium download; release for rehearsal |
| typecheck (272) | `tsc -p tsconfig.json` | framework-rs:generate, ui-contract-rs:generate, ui-rs:generate, framework-actor-rs:typegen, ui-styling-tokens:generate, assets:build | `default` + `{dependentTasksOutputFiles:"**/*",transitive:false}` (291) | none | **true** | P1-2; tsbuildinfo in node_modules (P2-4) |
| check (297) | `checkCourse`: builds and launches proctor | none | adapter default | none | false | cargo |
| docker-image-build (318) | proctor release build, docker build (`🚀️deploy/🐳️docker-image/🟦️.ts:41`) | none | adapter default | none | false | needs release mode (P1-1); no proctor dep; base image pulls |
| docker-image-check | docker run of image | none | adapter default | none | false | docker daemon, loopback port |
| docker-image-publish (342) | docker push (`🐳️docker-image/🟦️.ts:80`) | none | adapter default | none | false | registry write |
| docker-stack-check (352) | compose stack, Caddy | none | adapter default | none | false | docker, ports, pulls |
| docker-stack-bundle (368) | stages bundle (`🎁️stack-bundle/🟦️.ts:93-126`) | none | adapter default | `{projectRoot}/dist/proctor` (382) | false | `--out` can escape output (P2-9); pulls Caddy image (`:113`) |
| deploy-check (385) | readiness gate: build, typecheck and e2e run inline (`⚖️readiness/🟦️.ts:42`, `📜️script.ts:121-128`) | assets:build, ui-styling-tokens:generate, @teaching/architecture-course:typecheck | adapter default | none | false | typecheck runs twice (P2-2) |

Adapter default inputs when `inputs` is absent: `{projectRoot}/**/*` (`BZ/📜️script.ts:214`). Only cached targets matter for correctness.

## 3. Dependency Targets

| Target | Manifest | cache | Declared outputs | Contract-injected outputs (`🔣️taxonomy.json` `generatorContracts`) | Process | Notes |
|---|---|---|---|---|---|---|
| @semio-tech/assets:build | `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json` | true | `{projectRoot}/../../README.md` (line 32, tracked), `🌱️metabolism/🔣️icons/🤖️generated`, `🔣️icons/🤖️generated/{🖼️icons,🖼️icon_svgs,🔤️shortcodes,🪪️icon-name/🦀️.rs}` | `assets-build`: README and catalog tracked, bindings ignored | bun script | writes tracked README |
| @semio-tech/ui-styling-tokens:generate | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json` | true | none (`outputs` absent) | `styling-tokens`: `🎨️palette/🎨️.css` (tracked), `🎨️palette/🔷️.cs`, `🔤️tokens/🐍️.py`, `🔤️tokens/🦀️.rs`, `🤖️generated` | cargo | tracked CSS rewritten |
| @semio-tech/framework-rs:generate | `🧰️framework/📦️packages/🦀️rust/📋️project.json` | true | none | `framework-manifest`: `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` | cargo (admitted helpers) | output only via contract |
| @semio-tech/ui-rs:generate | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json` | true | none | `ui-axes`: `🎚️axes/🤖️generated/🦀️.rs`, `🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` | cargo | output only via contract |
| @semio-tech/ui-contract-rs:generate | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json` | true | none | `ui-contract`: `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` | cargo | output only via contract |
| @semio-tech/framework-actor-rs:typegen | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json` (339-342) | true | `{projectRoot}/../../🤖️generated/🎭️actor` | `actor-typegen` (ignored) | cargo | declared |
| @teaching/proctor:build (not in course closure) | `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` (379-382) | true | `{projectRoot}/.🧾️store/build.json` only | none | cargo | binary lives in compiler store (P1-4) |

Generator outputs are gitignored (`.gitignore:97`, `**/🤖️generated/`). Course TS imports them through the framework closure (examples: `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`, `…/🎭️actor/📮️shard-client/📜️wire/🟦️.ts`, `…/🖼️assets/🔣️icons/🟦️.ts`, `…/🖱️ui/🎨️styling/🌓️theme/🟦️.ts`).

## 4. Dependency Tree (text)

```
@teaching/architecture-course
├─ preflight                               (no deps)
├─ ci-browser-prepare                      (no deps; Linux apt)
├─ dev  [continuous]                       → assets:build, ui-styling-tokens:generate
│   ├─ inline: buildProctor() (cargo, shared store)     [not a graph dep]
│   └─ child: dev-site [continuous]        → same two deps
├─ dev-site [continuous]                   → assets:build, ui-styling-tokens:generate
├─ build                                   → assets:build, ui-styling-tokens:generate
├─ publish                                 → assets:build, ui-styling-tokens:generate
├─ test                                    (no explicit deps; see P3-1)
├─ test-e2e                                → assets:build, ui-styling-tokens:generate
│   └─ inline: buildProctor(), Playwright, Chromium
├─ typecheck                               → framework-rs:generate
│                                            ui-contract-rs:generate
│                                            ui-rs:generate
│                                            framework-actor-rs:typegen
│                                            ui-styling-tokens:generate
│                                            assets:build
├─ check                                   (no deps; inline buildProctor)
├─ docker-image-build                      (no deps; inline buildProctor(release))
├─ docker-image-check / -publish            (no deps)
├─ docker-stack-check / docker-stack-bundle (no deps)
└─ deploy-check                            → assets:build, ui-styling-tokens:generate,
                                             @teaching/architecture-course:typecheck
                                             inline: build, typecheck (again), test-e2e

@semio-tech/assets:build           [cache]  (leaf; cargo-free)
@semio-tech/ui-styling-tokens:generate [cache] (leaf; cargo)
@semio-tech/framework-rs:generate  [cache]  (leaf; cargo)
@semio-tech/ui-rs:generate         [cache]  (leaf; cargo)
@semio-tech/ui-contract-rs:generate[cache]  (leaf; cargo)
@semio-tech/framework-actor-rs:typegen [cache] (leaf; cargo)
@teaching/proctor:build            (not reached from course; its binary is not cached)
```

Implicit edges: every script-backed course target imports the same router closure. That closure reaches generated modules, so inference adds the generator targets as implicit dependencies (`INF/🧭️script-inputs/🟨️.mjs:161`, applied at `INF/🧱️project-assembly/🟨️.mjs:229`). This is not verified against a plan.

## 5. Bazel Suitability Per Task

- Cacheable deterministic action: `assets:build`, `ui-styling-tokens:generate`, `framework-rs:generate`, `ui-rs:generate`, `ui-contract-rs:generate`, `framework-actor-rs:typegen`, `typecheck` (after P1-2), `test` (after P1-2 and P1-3).
- Test: `test` (cached, `outputs: []`). The staged `repo_test` change in `result-bzl-coverage.md` would turn it into `bazel test`. The same key defects apply.
- Continuous or interactive: `dev`, `dev-site`.
- Uncached host state: `test-e2e` (ports, Chromium, node, processes), `docker-*`, `deploy-check`, `check` (cargo, proctor launch), `preflight`, `ci-browser-prepare` (apt, sudo), `publish` (CDN staging, dist write).
- Uncached but deterministic: `build` (P2-6).
- Rust and cargo: the five generators, `check`, `dev`, `test-e2e`, `docker-image-build`, `deploy-check` (in-process `buildProctor`), and `@teaching/proctor:build`.
- Network: `test-e2e` first run (Chromium download, `🎭️e2e/🟦️.ts` header), `docker-stack-bundle` (`pull` at 113), `docker-image-publish` (`push` at 80), `ci-browser-prepare` (apt), `docker-image-build` (base image digest).
- Absolute or OS paths: `docker-stack-bundle --out` is user-supplied (P2-9). `docker-image-check` uses OS `tmpdir()` (`🩺️docker-image-check/🟦️.ts:204`). Nothing is hard-coded.
- Non-ASCII in outputs: declared outputs contain emoji path segments (`{projectRoot}` and `🔣️icons/🤖️generated`). Snapshots flatten files to `files/<hash>` (`BZ/📜️script.ts:629`), and every non-hermetic task runs `no-sandbox` (`BZ/📜️script.ts:411`, `host_state`). So the darwin sandbox encoding issue is not triggered, and Bazel-visible labels are `t_<hash>`.
- Whole-repo inputs: none. The default covers 3,283 tracked files (`gap2.py`), with about 2,345 under `🖱️ui/**`. See P2-10.

## 6. Prioritized Defects

### P1 (blocks brief goals or can replay wrong results)

**P1-1. Release and deploy verbs are not selectable through Bazel.**
- The dashboard `execution-mode` parameter sets `SEMIO_BUILD_MODE=release`, but only the dashboard reads it. The adapter reads neither `metadata` nor the dashboard parameters (`dashboard` does not occur in `BZ/📜️script.ts`). The `execution-mode` parameter is declared on `publish` (`PKG/📋️project.json:218`), `test-e2e` (248), `docker-image-build` (319), `docker-image-publish` (343) and `deploy-check` (386); the release-mode guard matters for the first, second, third and fifth.
- The course has no `configurations` on any target (checked). A request for `:release` or `--configuration=release` fails with `Unknown build configuration` (`BZ/📜️script.ts:83`, `taskIdentity` at 79). The selector accepts a configuration only when the target declares it (`BZ/🧭️selection/🟦️.ts:91-92`).
- The course refuses release unless the caller's build mode is release: `SITE/🏗️builder/🎚️mode/🟦️.ts:9` ("require the original release build mode; select --configuration=release"). It is applied at `SITE/🚀️deploy/🌐️site/🟦️.ts:110`, the rehearsal path, and the proctor cross-mode guard (`🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:226-229`).
- Net effect, from reading the code: `bun repo --tool=bazel run @teaching/architecture-course:publish` works only when the invoking shell exports `SEMIO_BUILD_MODE=release`, and the documented `:release` route fails. Not verified by a run.
- Fix: declare `configurations.release` with `defaultConfiguration` where release is required. The mode check reads the caller's authority (`readNativeCallerAuthorityV1`), not the task env, so the Bazel route must also set that authority from the selected configuration. This is a small generic change, justified because it blocks the course. Verify with `--dry-run` and a real release run.

**P1-2. Cached `test` and `typecheck` under-declare inputs (stale green).**
- Method: import closure computed from every course `.ts` file (tests, deploy, e2e, stack, builder, the tsconfig include set). It includes the router closure (939 files) and the test and tsc closure (1,322). The gap is 27 files reached only by tests or tsc and not matched by the `default` globs (`🗑️generated/aud-closure/gap2.json`).
- Energy quiz content, 5 files: `SITE/📚️course/🟦️.ts:12-16` imports `../../⚡️energy/*/❓️quiz/🔣️.json` (physics, ventilation, heating, cooling, demand). Default has no glob for these topic folders: its only match is `{workspaceRoot}/🎓️teaching/🏛️architecture/**/📚️course/🔣️.json` (`PKG/📋️project.json:58`) plus the course content folder `📚️course/📚️course/**/*`.
- `@semio-tech/assets` source, about 20 files: `SITE/🟦️.ts:16` imports `@semio-tech/assets/logos/emblem/dark-round`. `SITE/🧪️tests/🧪️deploy/🟦️.ts:19-20` imports `🖼️assets/🏗️builder/📦️publication`. No `🖼️assets` glob exists in `default` (`PKG/📋️project.json:25-80`).
- Vitest config helper: `SITE/🧪️tests/🎚️config/🟦️.ts:3` imports `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts` (with fixture and schema). Not in default and not in the router closure.
- Proctor fixtures read at runtime (not imports): `SITE/🧪️tests/🧪️course/🟦️.ts:41` reads `🎓️teaching/🛂️proctor/🔨️modules/📚️course/🧰️fixtures/🦀️.rs`, which names `🎓️teaching/🛂️proctor/🧪️tests/🧫️fixtures/📚️catalog/🔣️.json`. Line 56 reads `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🧰️harness/🦀️.rs`. Default covers only `🛂️proctor/🏗️bootstrap/**`, `🧪️tests/🏋️capacity/**`, the README and Cargo.toml (`PKG/📋️project.json:53-54`). Changing the shared catalog that the Rust suites also use leaves the course test cached.
- Fix: add to `default` `{workspaceRoot}/🎓️teaching/🏛️architecture/⚡️energy/**/*`, `{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/**/*`, `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/**/*`, `{workspaceRoot}/🎓️teaching/🛂️proctor/🧪️tests/🧫️fixtures/**/*`, `{workspaceRoot}/🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🧰️harness/**/*`, `{workspaceRoot}/🎓️teaching/🛂️proctor/🔨️modules/📚️course/🧰️fixtures/**/*`. Re-run `gap2.py`; it should report 0.

**P1-3. Cached `test` depends on host state that is not in its key.**
- Spawns the Vite CLI under `bun` and `node` with `spawnSync`: `SITE/🧪️tests/🧱️local-stack/🟦️.ts:105-115`. Binds loopback sockets and fetches them: `:83-93`.
- The fingerprint hashes the bun version only (`BZ/📜️script.ts:483-484`). The Node version is not keyed. Declared env is hashed only for inputs of the form `{env}`, and `test` declares none (`:487`).
- Requires `SEMIO_TICKET_DIR` and throws without it: `SITE/🧪️tests/🏗️proctor-build/🟦️.ts:49-50, 74-75`. Writes under `SEMIO_TICKET_DIR/🗑️generated`: `SITE/🧪️tests/🐳️image/🟦️.ts:34`, `🧪️tests/🧪️deploy/🧰️deploy-fixtures/🟦️.ts:36`.
- Evidence: a real `test` run failed on the ticket env and on `tar-stream` (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-bzl-real.md:67`). That failure is environmental, but the ticket dependency is real.
- Fix: add `{"runtime":"node --version"}` (the adapter supports runtime inputs, `BZ/📜️script.ts:487`) and `{"env":"SEMIO_TICKET_DIR"}`. Move the local-stack, proctor-build and image tests to a `cache:false` target, or make them hermetic.

**P1-4. The Rust proctor is never a cached Bazel product.**
- `@teaching/proctor:build` declares only `.🧾️store/build.json` (`🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json:379-382`). The executable is captured in the shared compiler store (`🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:270-305`, store path at 280).
- No course target depends on it. Each path calls `buildProctor()` in-process inside an uncached action: `dev` (`SITE/🧱️stack/🟦️.ts:266`, `:356`), `check` (`SITE/🚀️deploy/🟦️.ts:231`), `test-e2e` (`SITE/🎭️e2e/🟦️.ts:42`), `docker-image-build` (`SITE/🚀️deploy/🐳️docker-image/🟦️.ts:41`).
- Effect: warm `publish`, `test-e2e`, `deploy-check`, `docker-*`, `check` and `dev` re-run `bun repo cargo`. Only cargo's incremental build helps. This is the boundary `result-bzl-man.md` item 3 accepted, but it misses the brief's "warm rebuilds are cache hits".
- Fix options: (a) a cached proctor product in the products store, with consumers depending on it and passing the path, as `test-e2e --proctor <executable>` already allows (`SITE/🎭️e2e/🟦️.ts:85-109`). (b) Accept and document these verbs as uncached.

### P2 (correctness risk or significant friction)

**P2-1. Continuous readiness and dashboard parameters have no Bazel meaning.** `dev` and `dev-site` carry `metadata.semio.dashboard.ready` (`PKG/📋️project.json:113-118`, `166`), the `steady` flag (119: `TEACHING_ARCHITECTURE_COURSE_WATCH=off`), and the `stack` choice `beside` (126: `PROCTOR_PORT`, `PROCTOR_DATA` with a `{workspace}` placeholder). The adapter reads only `execution.readyWhen`. With none, "ready" is the `[bazel] task-started` line printed before the child starts (`BZ/📜️script.ts:669`, consumed at `BZ/📜️script.ts:738`). So `bun repo --tool=bazel run @teaching/architecture-course:dev` reports ready before the server listens. `dev:beside` is rejected at `BZ/📜️script.ts:83`. Fix: have the stack print a ready marker after `awaitReady` and set `readyWhen`; expose `steady` and `beside` as configurations.

**P2-2. `deploy-check` typechecks twice.** It declares `dependsOn: @teaching/architecture-course:typecheck` (`PKG/📋️project.json:385-…`), and it also calls the typecheck inline (`PKG/📜️script.ts:125`; `SITE/🚀️deploy/⚖️readiness/🟦️.ts:42`). The inline call is uncached. `build` and `test-e2e` also run inline (`PKG/📜️script.ts:122-128`), so they are not graph tasks and get no cache or per-step progress in Bazel. Fix: drop the explicit dependency, or make the inline step skip when the dependency ran.

**P2-3. Generator snapshots rewrite tracked files.** `assets:build` writes `{projectRoot}/../../README.md` (assets `📋️project.json:32`, tracked per contract). `ui-styling-tokens:generate` writes `🎨️palette/🎨️.css` (tracked). Restoring snapshots (`BZ/📜️script.ts:743`) overwrites those files in the checkout on every typecheck, dev or build hit. Risk: dirty-tree diffs and overwritten local edits.

**P2-4. `typecheck` writes into node_modules.** `PKG/tsconfig.json:5` sets `tsBuildInfoFile: node_modules/.cache/architecture-course-typecheck.tsbuildinfo`. `node_modules` is a symlink into the dependencies partition (`.cache/b/d/m/j/…/node_modules`). Writes go outside declared outputs into a leased store. Fix: put the tsbuildinfo under a declared path, or in the ticket's generated folder.

**P2-5. Missing declared outputs are accepted silently.** `BZ/📜️script.ts:628` swallows ENOENT unless the strict opt-in is set. The four generator dependencies of `typecheck` (framework-rs, ui-rs, ui-contract-rs, ui-styling-tokens) declare no `outputs`, so their snapshots depend entirely on contract injection (`INF/🧱️project-assembly/🟨️.mjs` `generatorContracts`). If a generator writes nothing, the snapshot is empty and typecheck replays. `result-cache-fix.md` stages `bz/optional-strict-outputs.json` but does not apply it. Fix: enable strict outputs for this closure.

**P2-6. `build` is uncached although it is deterministic.** `PKG/📋️project.json:192-…` sets `cache:false` with `outputs: []`, and the test in `result-bzl-man.md` pins it. The vite artifact is already content-addressed (`buildViteArtifact`, `🚀️deploy/🌐️site/🟦️.ts`; `result-bzl-man.md` item 2). Fix: `cache:true`, `outputs: []`, after confirming the artifact key covers the inputs (env `PROCTOR_URL` is already in inputs).

**P2-7. `typecheck` timeout.** `PKG/📜️script.ts:80` and `:125` pass `timeoutMs: 120_000` to `typecheckRepositoryProjectV1`. A cold tsc over about 1,322 files plus generated types may exceed this. Not measured.

**P2-8. Platform parity.** `ci-browser-prepare` is apt-only (`📜️script.ts:40`). `test-e2e` spawns `node` (`SITE/🎭️e2e/🟦️.ts:145`, `:307`), so Node must be on PATH even in Bun-only setups. Chromium downloads on first use. The earlier readiness audit recorded node v24.15 as the working version. Windows and macOS parity is not verified.

**P2-9. `docker-stack-bundle --out` escapes the declared output.** The default is `{projectRoot}/dist/proctor` (`PKG/📋️project.json:382`), but `--out` is user-supplied (`SITE/🚀️deploy/🎁️stack-bundle/🟦️.ts:103`). Low impact, since the target is uncached.

**P2-10. Over-broad default inputs.** `{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/**/*` (`PKG/📋️project.json:66`), `{workspaceRoot}/Cargo.toml` (73) and `{workspaceRoot}/package.json` invalidate course test and typecheck on unrelated edits. Low severity; safe direction.

### P3 (notes and verification items)

- **P3-1. Implicit generator edges.** The explicit `dependsOn` lists of `dev`, `build`, `publish`, `test`, `test-e2e` omit `framework-rs:generate`, `ui-rs:generate`, `ui-contract-rs:generate` and `framework-actor-rs:typegen`, which the router closure also imports. Inference should add them (`INF/🧭️script-inputs/🟨️.mjs:161`, `INF/🧱️project-assembly/🟨️.mjs:229`). Not verified. A `--dry-run` plan (`BZ/📜️script.ts`, the `--dry-run` path) must show them before this is trusted.
- **P3-2. Forwarded arguments are keyed.** `forwardAllArgs` arguments are recorded in the task (`BZ/📜️script.ts:126`) and embedded in the manifest (`BZ/📜️script.ts:419`). Because the manifest is an action input (`BZ/📜️script.ts:537`, `repo_task` inputs), `test quick` and `test exhaustive` get different action keys. No defect found.
- **P3-3. Destructive proctor verbs share the graph.** `@teaching/proctor:erase`, `prune`, `restore` (`🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json:407-416`) are uncached `repo:script` targets. They are not in the course closure. They run as `no-sandbox` `bazel build` actions when selected, so keep them out of any plan that includes the course.
- **P3-4. Per-launch output bases.** Continuous tasks run `bazel run` with their own output base under `outputs/launch/<ns>/<label>` (earlier `audit-bazel-targets.md` section 4). Path length against the 256-character rule on Windows is not checked.

## 7. Not Verified

- No `--dry-run`, no build, no Bazel run, no Windows or macOS execution.
- Implicit generator edges (P3-1) and the `:release` and caller-mode behavior (P1-1) need a dry-run and a real run.
- Cold tsc time (P2-7). Chromium availability. Path lengths of launch output bases (P3-4).

## 8. Earlier Audits, Course-Relevant

- `result-bzl-man.md` (course section): the manifest reflects its changes (`test` cache true with `default`, `typecheck` cache true with non-transitive `dependentTasksOutputFiles`). P1-2 is what that section did not cover.
- `audit-bazel-targets.md`, `audit-build-system-bazel.md`: "`inputs` object ignored" is superseded. Object inputs now pass through (`BZ/📜️script.ts:197-216`), and the fingerprint ignores `dependentTasksOutputFiles` (`BZ/📜️script.ts:483-489`). For `typecheck` this is harmless, because direct dependency snapshots are Bazel action inputs (`BZ/📜️script.ts:530-537`).
- `result-cache-fix.md`: fail-closed classification and the strict-outputs patch are staged, not applied. P2-5 depends on the patch.
- `result-bzl-real.md:67`: course `test` failed in a real run on the ticket env and on dependency sync. Confirms P1-3.
- `audit-e2e-readiness-1020.md` section 2c: the e2e path was checked statically. Node v24.15 is required on PATH. A cold proctor build is estimated at 10 to 25 minutes.
- `result-bzl-coverage.md`: the staged `repo_test` would make course `test` a `bazel test` result. P1-2 and P1-3 then matter more, because the test cache key becomes the Bazel test cache key.
