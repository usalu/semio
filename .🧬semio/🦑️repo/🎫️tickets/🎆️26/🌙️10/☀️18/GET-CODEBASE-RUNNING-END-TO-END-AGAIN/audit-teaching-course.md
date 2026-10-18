# 📓 Audit: "Architektur und Technologie" course (teaching / architecture quiz)

Date: 2026-10-18. Read-only audit. Paths are relative to the repo root `/Users/ueli/Documents/semio`.
Nothing was built, served, tested or committed; no modifying git command was run (`git --no-optional-locks status` and `git log` only).

Short names used below:
- `QUIZ` = `🎓️teaching/🏛️architecture/❓️quiz/` (site, catalog, deploy, e2e, stack)
- `QUIZ-PKG` = `QUIZ📦️packages/🟦️typescript/` (`@teaching/architecture-quiz`, `📋️project.json`, `📜️script.ts`)
- `PROCTOR` = `🎓️teaching/🛂️proctor/` (Rust API server, `@teaching/proctor`)
- `PROCTOR-PKG` = `PROCTOR📦️packages/🦀️rust/` (`📋️project.json`, `📜️script.ts`, `Cargo.toml`)

## 1. Deliverables and stacks

| Deliverable | What | Stack | Where it is built |
|---|---|---|---|
| Site `quizze.architektur-und-technologie.de` | static build of the quiz SPA, proctor origin baked in, CSP sealed | TypeScript, React (`@semio-tech/quiz-react`), Vite 7, Tailwind 4 (`@tailwindcss/vite`), CSS from `QUIZ🎨️.css` | `QUIZ-PKG` target `publish`, output `dist/release/pages/quizzes` |
| Proctor API `semio.iek.uni-hannover.de` | one Rust binary `proctor` (axum, tokio), one SQLite file, CQRS/event sourcing; API only | Rust (nightly-2026-07-20 per `rust-toolchain.toml`), workspace `🎓️teaching/Cargo.toml` | `PROCTOR-PKG` targets `build`/`dev`; release via Docker |
| Proctor image | `ghcr.io/usalu/architecture-quiz-proctor` (distroless runtime, port 8791) | Dockerfile, builder `rust:1.98.1-bookworm` | `QUIZ🚀️deploy/Dockerfile` |
| Proctor host bundle | `compose.yaml`, `Caddyfile` (TLS), `.env`, `certificates/`, image tar | Docker Compose, Caddy | `docker-stack-bundle` -> `QUIZ-PKG/dist/proctor` |
| Catalog and quizzes | `QUIZ🔣️.json` (catalog `architecture`, 4 quiz paths) and 4 `⚡️energy/*/❓️quiz/🔣️.json` (`semio.quiz/v1`) | JSON, schemas in `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json` | content only |
| Pets (lazy chunk) | 20 species, `QUIZ` loads `../🐾️pets/🟦️.ts` via `import()` | TypeScript, `@semio-tech/pets-react`, storybook dev on 6074 | `🎓️teaching/🏛️architecture/🐾️pets/` |
| Test and gate tooling | Vitest 4 (quiz TS), cargo tests (proctor), Playwright 1.62.1 (e2e, Chromium), ajv 8 (schema) | | |

Workspace root for the course: `🎓️teaching/package.json` (bun workspaces, 121 members, all with a `package.json`; `nx@23.2.0` patch present) and `🎓️teaching/Cargo.toml` (cargo workspace: proctor, `teaching-proctor-domain`, `teaching-proctor-base`).

## 2. Launch rows and their targets

All rows are in `.vscode/launch.json` (working copy) and in `.vscode/🧩️launch.seed.jsonc` (28 teaching rows in both, names identical). Every row is `bun repo run <project>:<target>` with `cwd: ${workspaceFolder}`. Line numbers are the `"name"` line in `.vscode/launch.json`.

| Line | Launch row (name, abbreviated) | Command | project.json target | `📜️script.ts` command (router) |
|---|---|---|---|---|
| 81104 | 📦️build quiz | `@teaching/architecture-quiz:build` | `QUIZ-PKG` `build` (L135) | `build` -> `runViteBuild` |
| 81119 | ✅️check quiz catalog | `@teaching/architecture-quiz:check` | `check` (L214) | `check` -> `checkQuizCatalog` |
| 81214 | 🚚️publish quiz | `@teaching/architecture-quiz:publish` | `publish` (L157) | `publish` -> `publishQuizSite` |
| 81291 | ⚖️gate quiz e2e | `@teaching/architecture-quiz:test-e2e` | `test-e2e` (L182) | `test-e2e` -> `runQuizEndToEnd` |
| 81392 | ⚖️gate quiz docker-image | `@teaching/architecture-quiz:docker-image-check` | `docker-image-check` (L231) | `docker-image-check` |
| 81487 | ⚖️gate quiz docker-stack | `@teaching/architecture-quiz:docker-stack-check` | `docker-stack-check` (L252) | `docker-stack-check` |
| 81582 | 📦️build quiz docker-image | `@teaching/architecture-quiz:docker-image-build` | `docker-image-build` (L222) | `buildQuizImage` (`docker build … --file Dockerfile .`, `QUIZ🚀️deploy/🟦️.ts` L421) |
| 81597 | ⚖️gate quiz deploy | `@teaching/architecture-quiz:deploy-check` | `deploy-check` (L273) | `deploy-check` -> `checkQuizDeployment` (8 steps, L943) |
| 81692 | 🚚️publish quiz docker-image | `@teaching/architecture-quiz:docker-image-publish` | `docker-image-publish` (L240, dependsOn `docker-image-build`) | `publishQuizImage` |
| 81787 | 📦️bundle quiz docker-stack | `@teaching/architecture-quiz:docker-stack-bundle` | `docker-stack-bundle` (L261) | `stageQuizStackBundle` |
| 88109 | 🛠️dev quiz | `@teaching/architecture-quiz:dev` | `dev` (L52) | `dev` -> `runDevStack` (proctor + site) |
| 88131 | 🛠️dev quiz site | `@teaching/architecture-quiz:dev-site` | `dev-site` (L105) | `dev-site` -> Vite dev server |
| 88380 | 🛠️dev quiz typecheck | `@teaching/architecture-quiz:typecheck` | `typecheck` (L196) | `typecheck` -> `typecheckRepositoryProjectV1` |
| 88350 | 🧪️test quiz | `@teaching/architecture-quiz:test` | `test` (L172) | `test` -> `runVitest` (`QUIZ🧪️tests/🎚️config/🟦️.ts`) |
| 88153 | 🛠️dev proctor | `@teaching/proctor:dev` | `PROCTOR-PKG` `dev` (L41) | `dev` (build + run private copy on 8791) |
| 88169 / 88184 / 88200 | 🔁️rebuild / 🩺️health / 💾️backup proctor | `@teaching/proctor:rebuild` / `health` / `backup` | L69 / L77 / L85 | same names |
| 88215 / 88230 / 88245 | ♻️restore / 🧨️erase (dry-run, real) | `@teaching/proctor:restore`, `erase -- --handle …` | L94 / L117 | same names |
| 88260 / 88275 | 🧹️prune (dry-run, real) | `@teaching/proctor:prune -- --older-than …` | L156 | same name |
| 81229 | ✅️check proctor catalog | `@teaching/proctor:check` | `check` (L60) | `check [catalog]` |
| 81377 | 📦️build proctor | `@teaching/proctor:build` | `build` (L21) | `build` |
| 88335 | 🧪️test proctor | `@teaching/proctor:test` | `test` (L33) | `test [fundamental\|quick\|long\|exhaustive]` |
| 81260 | ⚖️gate proctor capacity | `@teaching/proctor:capacity` | `capacity` (L187) | `capacity` |
| 106998 | 🛠️dev pets stories | `@semio-tech/pets-react:dev` (`PETS_STORIES_PORT=6074`) | `framework/…/🐾️pets/🎯️targets/⚛️react` `dev` | n/a |
| 206470 (compound) | 🧭️compound quiz + proctor | runs `🛠️dev proctor` and `🛠️dev quiz` together, `stopAll: true` | | |

Every target and router command in the table exists (checked: 14 quiz targets = 14 router registrations; 11 proctor targets = 11 router commands). The 88xxxx rows in the working copy were not compared line by line beyond their command; the seed and the working copy list the same 28 rows.

Other registrations: the quiz `package.json` scripts (`build`, `test`, `test:e2e`, `typecheck`) forward to the same `repo run` targets. Note that `test:e2e` maps to `:test-e2e`.

## 3. Build and dependency chain (source to running end-to-end instance)

Dev stack (launch row `🛠️dev quiz`, `🧭️compound`):
1. nx `dependsOn` for `QUIZ-PKG` `dev`: `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate` (both project names and targets exist).
2. `bun ./📜️script.ts dev` -> `runDevStack` (`QUIZ🧱️stack/🟦️.ts`): reuse a proctor on `PROCTOR_PORT` if one answers, else `buildProctor` (`PROCTOR🏗️bootstrap/🟦️.ts` L224-245, spawns `cargo build --package teaching-proctor --bin proctor` in `🎓️teaching`) and launch it with `PROCTOR_DATA=.🧬semio/🎓️teaching/proctor-dev` and `PROCTOR_CATALOG=🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` (defaults L36-55 of the bootstrap).
3. The launched proctor is supervised (rebuilt on Rust source change, relaunched on crash or catalog change).
4. The Vite dev server (`QUIZ🏗️builder/🌐️vite/🟦️.ts`) starts on `TEACHING_ARCHITECTURE_QUIZ_PORT` (6061) and proxies the gateway routes to `http://127.0.0.1:${PROCTOR_PORT ?? 8791}` (L28, L143).
5. Browser: `http://localhost:6061`.

Release site (`publish`, also CI `site` job):
1. `runViteBuild` in `release` mode with `VITE_PROCTOR_URL` = `PROCTOR_URL` or `https://semio.iek.uni-hannover.de` (L47, L140). CSP sealed, `CNAME`, `robots.txt`, `manifest.webmanifest` written.
2. `publishQuizSite` verifies the artifact (`siteArtifactProblems`, also imported by the e2e gate) against the production proctor and stages `dist/release/pages/quizzes` (`QUIZ🚀️deploy/🟦️.ts` L383-394).

Proctor image (CI `proctor` job, or locally):
1. `docker build -f QUIZ🚀️deploy/Dockerfile -t ghcr.io/usalu/architecture-quiz-proctor .` (context = repo root, `🚀️deploy/🟦️.ts` L421).
2. Builder stage: `COPY rust-toolchain.toml` (L30), `COPY . .` (L40), `cargo build --release --locked --package teaching-proctor --bin proctor` in `🎓️teaching` (L44), using `Dockerfile.dockerignore` (admits `!🎓️teaching/Cargo.lock`, `!.cargo`, `!**/*.rs`, `!**/Cargo.toml`, the quiz catalog and the stack files only).
3. Runtime stage: distroless, binary plus `/srv/quiz/content` (catalog and quizzes), `/srv/quiz/deploy` (compose.yaml, Caddyfile), `/srv/quiz/data`, `EXPOSE 8791` (L85). The binary validates the staged catalog during the build (L59-75).

Production: `docker-stack-bundle` (or the `compose.yaml` in the image) on `semio.iek.uni-hannover.de`, Caddy on 80/443 for TLS. Site on GitHub Pages / CDN.

Ports and env (the ones the course reads):

| Name | Default / value | Used by |
|---|---|---|
| `TEACHING_ARCHITECTURE_QUIZ_PORT` | 6061 (dev); beside stack 6063 | Vite dev server (`QUIZ-PKG` `package.json` `semio.app.port`) |
| `PROCTOR_PORT` | 8791 | dev proctor, Vite proxy target; beside stack 8793 |
| `PROCTOR_DATA`, `PROCTOR_CATALOG` | `.🧬semio/🎓️teaching/proctor-dev`, `QUIZ🔣️.json` | proctor dev launch |
| `PROCTOR_URL` | `https://semio.iek.uni-hannover.de` | release build bakes `VITE_PROCTOR_URL` |
| `PROCTOR_BIND`, `PROCTOR_MODE=production`, `PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de`, `PROCTOR_TRUSTED_FORWARDING=proxy` | set in the Dockerfile ENV (L77-83) | proctor image |
| `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` | disables Vite watch/HMR | dev stack |
| `SEMIO_BUILD_MODE` | `release` in CI (workflow `env` block, line 33); the local default was not verified | build output folder |
| `PLAYWRIGHT_BROWSERS_PATH` | repo tool cache if unset | e2e Chromium |
| e2e throw-away ports | dev site 6161 / proctor 8891; rehearsal site 6162 / proctor 8892 (`QUIZ🎭️e2e/🟦️.ts` L46) | `test-e2e` |

## 4. What the CI workflow runs (`.github/workflows/architecture-quiz.yml`)

Manual trigger only (`workflow_dispatch`, inputs `site` and `proctor`, both default true). Both jobs run only when `github.ref == 'refs/heads/main'`. `SEMIO_BUILD_MODE: release`. Concurrency group `architecture-quiz`.

- Job `site` (ubuntu-24.04, L43-70): checkout, Node 24.15.0, bun from `package.json`, cache `~/.bun/install/cache`, then
  - `bun repo dependencies sync --all` (L63),
  - `bun repo run @teaching/architecture-quiz:test` (L65),
  - `bun repo run @teaching/architecture-quiz:publish --configuration=release` (L67),
  - upload of `dist/release/pages/quizzes` (L70).
- Job `pages` (L72-84): `actions/deploy-pages` with OIDC.
- Job `proctor` (L86-118): same setup, then `docker-image-build` (L111), `docker login ghcr.io` with GITHUB_TOKEN, `docker-image-publish` (L118), which checks the image and stack and pushes `sha-<commit>`, the workspace version and `latest`.

Not in CI: `test-e2e`, `deploy-check`, `typecheck`, `check` and the capacity gate. The e2e gate and the readiness gate are run by hand only (both documented as such in the workflow header and the ticket reports).

## 5. Evidence of current breakage

### 5a. What the latest reports say

Ticket paths in this subsection are relative to `.🧬semio/🦑️repo/🎫️tickets/🎆️26/` (so `🌙️10/☀️02/…` means `🎆️26/🌙️10/☀️02/…`).

- `🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/📓️deploy-ready-report.md` (2026-10-02, checkpoint 667): `deploy-check` ended "ready to deploy"; e2e 36/36 in both topologies; capacity twice green; Linux site job green. This is a dated green on an older tree. Later checkpoints (668 and on) and the `.cargo` change of 10-14 are not covered by it. Open items it names: legal URLs `site.legal.imprint` and `privacy` empty (deploy-check warns), two bun versions (bun 1.4.2 drops fixture workspaces from `bun.lock`, pinned 1.3.14 in CI needs them: `frozen-lockfile` breaks), 17 taxonomy errors on the teaching workspace-root files (`Cargo.toml`, `Cargo.lock`, `bun.lock`, `package.json`, `.config/nextest.toml`), the emblem SVG stray text node.
- `🌙️10/☀️03/QUIZ-END-TO-END-FAULT-TOLERANCE/📓️report.md` (2026-10-03, closed): e2e green in both topologies except `🐕️pet-walk` (dev 2 failed, rehearsal 3 failed), which the report attributes to the collapsed-card geometry it fixed and handed to the QUIZ-PETS session. Open: restart the running dev stack once (its proctor predates the supervisor); Bun 1.3.13 does not proxy WebSockets through Vite (presence decorative); 18 taxonomy errors at the teaching root.
- `🌙️09/☀️28/UPDATING-ARCHITECTURE-QUIZ-WORDING` and `🌙️10/☀️02/QUIZ-POC-DEPLOY-BUNDLE`: closed; no open breakage named.

### 5b. Static check of paths (read-only, Node `fs` and text scans)

| Check | Scope | Result |
|---|---|---|
| Relative TS/TSX imports | 39 files of `QUIZ` (incl. `🧪️tests`, `🎭️e2e`, `🚀️deploy`, `🧱️stack`, `🏗️builder`, `📚️catalog`) and the proctor script and tests | 127 specifiers, 0 unresolved (the only miss was the `?raw` suffix on an SVG import at `QUIZ🟦️.ts` L16; the file `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg` exists) |
| Transitive import closure of 8 entry points (script, site entry, vite config, deploy, stack, e2e, catalog, pets entry) | 810 TS files, 800 of them in the framework | 1 unresolved specifier, a false positive: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` L263 and L564 import `../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/…` inside a template string that emits the generated shard worker. The `🪞️vendor` directory does not exist in the tree (it is produced by plugin materialization), so it can break only a plugin boot at runtime, not the course. The chain is `QUIZ-PKG/📜️script.ts` -> `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` (the repo library barrel) -> `🏪️store/📦️packaging` -> `🏪️store/📥️installation` -> the browser-bundle module. |
| tsconfig `paths` targets (`QUIZ-PKG/tsconfig.json`) | 9 aliases | 9/9 exist |
| nx `dependsOn` projects and targets | `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`, `@semio-tech/framework-rs:generate`, `@semio-tech/ui-contract-rs:generate`, `@semio-tech/ui-rs:generate`, `@semio-tech/framework-actor-rs:typegen` | all exist |
| Cargo `path =` deps | `🎓️teaching/Cargo.toml` (6), `PROCTOR-PKG/Cargo.toml` (9), base (3), domain (3) | all exist |
| Rust `#[path = …]` modules | all `.rs` under `🎓️teaching` (29) | all exist |
| Catalog quiz paths and `$schema`/`schema` refs | `QUIZ🔣️.json` (4 quizzes), 5 quiz catalogs, pets `🔣️.json`, deploy `🔣️.json` | 4/4 quiz paths, 7/7 schema refs resolve (one `http://json-schema.org/draft-07/schema#`) |
| Teaching bun workspace members | `🎓️teaching/package.json` | 121/121 have a `package.json`; `nx@23.2.0` patch file exists |
| Relative path string literals in teaching sources | 181 literals | 11 flagged, all false positives on reading: `QUIZ🧪️tests/🧪️deploy/🟦️.ts` (glob patterns for an include check), `QUIZ🧪️tests/🧱️local-stack/🟦️.ts` (`../energy/a/🔣️.json` is an intentional negative fixture), `QUIZ🧪️tests/🐾️pet-cast/🟦️.ts` and `🧪️tests/🧪️catalog/🟦️.ts` (resolved against `siteRoot` = `QUIZ`, correct), `PROCTOR🔨️modules/📚️catalog/🧰️fixtures/🦀️.rs` and `PROCTOR🧪️tests/🌐️end-to-end/🛡️edge/🦀️.rs` (`fixture_path!` resolves against the tested crate's `CARGO_MANIFEST_DIR`; computed correct for the base and the proctor package) |
| `node_modules` | `QUIZ-PKG/node_modules` (16 symlinks), `🎓️teaching/node_modules` (`.bun`, `@semio-tech`), root `node_modules` symlink | no dangling links; `playwright/cli.js`, vite, vitest, ajv, typescript, yaml, `@playwright/test` present |

Catalogs, pets, Rust module tree and workspace manifests are intact. The 10/02 report's "moved API" breakages (`defineTestAdapter`, the script API) are not visible as broken paths any more.

### 5c. Findings, most severe first

1. **Proctor image build very likely broken since 2026-10-14 (static reading, not run).** `.cargo/config.toml` L6-7 sets `[build] rustc = "…🚪️ingress/📜️script.ts"` and `rustc-wrapper = "bun"`. That script (`🚪️ingress/📜️script.ts`, line 3) throws `policy.refusal` unconditionally unless a launcher issued a reservation. The launcher's `repositoryCompilerLaunchEnvironmentV1` (`🚪️ingress/🟦️.ts`) sets `RUSTC` and `RUSTC_WRAPPER=""` in the child environment, which overrides the config; so `bun repo run …` works. The Docker build does not get that environment:
   - `QUIZ🚀️deploy/Dockerfile` L40 `COPY . .` and L44 `cargo build` run in `/src` with `/src/.cargo/config.toml` admitted (`Dockerfile.dockerignore` L11 `!.cargo`);
   - the rustc script (`🧰️framework/…/🚪️ingress/📜️script.ts`) and `bun` are not in the build context or the `rust:` image, so cargo cannot start `rustc` at all, or it refuses.
   - Affects `docker-image-build`, `docker-image-check`, `docker-stack-check`, `docker-image-publish` (CI `proctor` job, L111 and L118), `deploy-check` step "proctor image built", and `docker-stack-bundle` if it builds.
   - Evidence the regression is recent: `.cargo/config.toml` mtime 2026-10-14 20:40, last git commit touching it 2026-10-15 01:26 (`793f63533f2`); the 10/02 report says the same Dockerfile built then. Memory note `project-cargo-compiler-ingress-gate` describes the same refusal.
   - Fix direction (for the owner, not done): the Docker builder must either set the same authority or the Dockerfile must not depend on the host's `.cargo/config.toml` (for example `ENV RUSTC=rustc RUSTC_WRAPPER=` plus a `CARGO_HOME`-local config, or a `.cargo` exclusion in `Dockerfile.dockerignore`). Not decided here.
2. **Dev stack spawns raw `cargo` (medium, works only under the launcher).** `PROCTOR🏗️bootstrap/🟦️.ts` L226 `spawn("cargo", ["build", …])` with `env = process.env`. Under `bun repo run @teaching/architecture-quiz:dev` the launcher env (RUSTC) is inherited, so it should work. Run directly (`bun ./📜️script.ts dev`) or from a shell without the launcher, the cargo call hits the refusal. Same for `capacity` (`PROCTOR-PKG/📜️script.ts` `build`). Runtime not verified.
3. **`bun repo` entry point is currently broken** (user-stated). Every course entry point (launch rows, CI, `package.json` scripts, `deploy-check`) goes through `bun repo run`. Nothing here can be run until that is fixed.
4. **Known open items from reports (current state not re-verified):** `site.legal` empty (`deploy-check` warns); bun version split (`bun.lock` vs pinned 1.3.14, CI `frozen-lockfile`); 17-18 taxonomy errors on `🎓️teaching` root files; `🐕️pet-walk` e2e in rehearsal (3 failed on 10/03); presence WebSocket via Vite on Bun 1.3.13; running dev proctor predates the supervisor (restart once); emblem SVG stray text node (shared asset); the capacity gate script is type-checked by no target.
5. **Working tree state.** `.github/workflows/architecture-quiz.yml` (M), `.vscode/launch.json` (MM) and `.vscode/🧩️launch.seed.jsonc` (M) are modified and uncommitted; `QUIZ🧪️tests/🧪️deploy/🟦️.ts` is staged. CI runs from `main`, so these edits are not what CI tests.
6. **Coverage gap in CI (info).** The site job runs `test` and `publish`; the proctor job runs the image build and publish (which checks image and stack). `test-e2e`, `deploy-check` and `typecheck` are not in CI. `docker-image-publish` depends on `docker-image-build` (project.json L240-ish), so the image is built twice in the proctor job.
7. **Dist folders present but stale.** `QUIZ-PKG/dist/{prototype,release}` and `dist/proctor` exist from 10-13. They are not evidence of a current build.

## 6. What an end-to-end verification looks like

Automated (all via `bun repo run`, so blocked by item 3 above):
- Unit and contract tests: `@teaching/architecture-quiz:test` (Vitest, `QUIZ🧪️tests/🎚️config/🟦️.ts`; 133 tests at 10/02; 23 `🟦️.ts` spec files under `QUIZ🧪️tests/`, each a case such as `🥞️layered-home`, `🐾️pet-cast`, `🔌️connection-shortage`, `🛟️proctor-faults`, `📴️proctor-away`, `🗣️both-languages`, `📱️phone`, `📐️adaptive-layout`, `🏆️live-leaderboard`, `👥️shared-presence`).
- Proctor: `@teaching/proctor:test` (cargo: unit tests in each module, `PROCTOR🧪️tests/🔬️conformance`, `PROCTOR🧪️tests/🌐️end-to-end` with `ureq` and `tungstenite` clients; about 116 `#[test]` attributes in the proctor tree).
- Capacity gate: `@teaching/proctor:capacity` (`PROCTOR🧪️tests/🏋️capacity/`: hall-learner, transport, abusive-script, settings-and-measurements, proctor-process; `--learners n`, `--hall-only`).
- Type checks: `typecheck` (whole `tsconfig.json`, 120 s timeout) and `check` (catalog through `proctor check`).
- Contract parity (framework quiz, language-agnostic): cases under `🧰️framework/🛍️products/❓️quiz/` (108/108 at 10/02, 132/132 after the wire-version case at 10/03). Not re-run.
- E2E gate: `bun repo run @teaching/architecture-quiz:test-e2e` (`QUIZ🎭️e2e/🟦️.ts`): boots throw-away stacks `dev` (site 6161, proctor 8891) and `rehearsal` (site 6162, proctor 8892: release build plus production-mode proctor on another origin), runs Playwright (`QUIZ🎭️e2e/🎚️config/🟦️.ts`, projects `boot`, `desktop`, `phone`, `layout`, `presence`, `shortage`, `away`, `faults`, `pets`; `PLAYWRIGHT_BASE_URL` set by the gate). Chromium from `PLAYWRIGHT_BROWSERS_PATH` or repo cache; `~/Library/Caches/ms-playwright/chromium-1234` is present on this machine. Logs and traces in `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/` on failure.
- Readiness: `@teaching/architecture-quiz:deploy-check` runs 8 steps (no drift, sources type-checked, catalog valid in Rust core, site built as CDN artifact, proctor image built, image checked, stack checked, end-to-end gate). The step "proctor image built" is the one item 5c-1 predicts failing.
- Capacity and proctor live checks: `@teaching/proctor:health` (asks the dev proctor), `:rebuild`, `:backup`.

Manual / probe scripts that exist in ticket folders (not part of the repo gates):
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/`: `gates.sh`, `linux_site_job.sh` (fresh clone + working tree, Node 24.15.0, bun 1.3.14: the CI site job without GitHub), `stage_deliverables.sh`, `drive_staged_site.ts` (drives the staged site in Chromium with the proctor away), `websocket_failure_probe.ts`, `launch_quiz_test.py`, `📸️staged-site-sorting-guesses.png`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️03/QUIZ-END-TO-END-FAULT-TOLERANCE/upgrade_socket_probe.ts` (Bun upgrade socket `destroySoon` probe).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/UPDATING-ARCHITECTURE-QUIZ-WORDING/`: `deploy_readiness_*.sh|ts` probes (CSP probe, router bisect, WS hold/stop, rustc crash probe, runbook rehearsal).

Minimal runtime check once the CLI works (suggested order, not run): `@teaching/proctor:build` (tests cargo ingress), `@teaching/architecture-quiz:test`, `:dev` then open 6061 and confirm `GET /instance` on 8791 from the browser, `:docker-image-build` (confirms item 5c-1), `:test-e2e`, `:deploy-check`.

## Method and side effects

- Read-only. No `bun`, `nx`, `cargo`, `docker` or `bun repo` command was run. No source file was edited. No git modifying command was run.
- Checks used `ls`, `cat`, `sed`, `find`, `/usr/bin/grep` (the shell `grep` is aliased), and short Node `-e` scripts that only read files and print results. Repo-wide grep was not used (one over `🧰️framework/…/🦑️repo` timed out and was left to finish).
- One scratch file was written outside the ticket: `/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/projects.txt` (a list of `📋️project.json` paths, for the nx project-name check). It is not in the repo and was not deleted, per the no-deletion rule. Remove it when convenient.
- Only this report was created in the repo.
