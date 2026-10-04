# 🚀 Deploy Readiness — Report (work package "deploy")

Scope: the static site at `quizzes.architektur-und-technologie.de` and the proctor stack at
`proctor.quizzes.architektur-und-technologie.de` — image, compose, Caddy, workflow, the site's release document, one
readiness gate, the runbook. Nothing was published outward: no `docker push`, no `docker login`, no workflow run, no `gh`
write call, no DNS. Everything else ran for real on this host (Windows 11, Docker Desktop, Linux engine, amd64).

Tree state verified: the uncommitted working tree on top of commit `48d881aa7ab` on 2026-10-02; the final gate run
started 02:47:50 and ended 03:18:01 local time (UTC+2; §3), the two test targets and the taxonomy ran right after (03:19–03:35).
Other agents (edge, domain, ui, dev-e2e) edited the proctor and the client during the whole work; the tree stopped
compiling several times, also after an earlier gate run (02:15). Whatever they change after 03:18 is not covered.

## 1. Outcome

| Deliverable | State |
|---|---|
| Existing pipeline proven today | `check`, `build`, `publish`, `docker-image-build`, `docker-image-check`, `docker-stack-check` all run; three real failures found and fixed (§2: F1, F2, F3) |
| Hardening for a public deployment | image, compose, Caddy, data, site/CDN, workflow, single source of truth — §2 |
| One readiness gate | `bun nx run @teaching/architecture-quiz:deploy-check` (launch row `⚖️gate🎓️teaching🏛️architecture❓️quiz🚀️deploy`, root script `check:teaching:architecture-quiz:deploy`) — §3 |
| Runbook | `🎓️teaching/🏛️architecture/❓️quiz/README.md`, section "Deploy"; every host command rehearsed verbatim (§3), the outward ones marked ⚠️ |

## 2. Findings and fixes

### Failures of the pipeline as it was

| # | Finding | Fix |
|---|---|---|
| F1 | **The site did not build from a clean checkout** (found by rehearsing the workflow's `site` job in a fresh Linux clone): the build imports two git-ignored generated sources, `🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🟦️.ts` and `🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts`. Every dev machine has them, so nobody saw it; the first workflow run would have failed. | `build`, `publish` and `deploy-check` now `dependsOn` `@semio-tech/assets:build` and `@semio-tech/ui-styling-tokens:generate`. Rehearsal after the fix: green (§3). |
| F2 | **The image build crashed** on the first run today: `rustc interrupted by SIGSEGV` inside LLVM (`IPSCCPPass`) while linking the `proctor` binary with thin LTO (pinned `nightly-2026-07-07`). Not deterministic: a probe (`deploy_readiness_rustc_crash.sh`) saw 0/5 with the workspace `-Z threads=8` and 0/5 without; it happened a second time during the final gate run, in `semio-framework-async`. | The Dockerfile attempts the cargo build twice; cargo resumes from every compiled unit, so only the crashed crate is repeated. In the final gate run exactly that happened: crash, second attempt, image built. A real compile error is reported twice. Open issue O1. |
| F3 | **Docker Desktop did not start**: the backend crashed on the stale AF_UNIX socket `%LOCALAPPDATA%\Docker\run\dockerInference` ("The file cannot be accessed by the system"), as an earlier agent had seen. | Renamed, nothing deleted: `Docker\run` → `Docker\run-stale-20261001`, `docker-secrets-engine` → `docker-secrets-engine-stale-20261001`. At 01:35 Docker Desktop updated itself (4.69.0 → 4.93.0, engine 29.4.0 → 29.8.1) in the middle of a build; every final run below is on 29.8.1. |

### Image (`🚀️deploy/Dockerfile`, new `🚀️deploy/Dockerfile.dockerignore`)

| Topic | Before | Now |
|---|---|---|
| Base images | `rust:1-bookworm`, `debian:bookworm-slim` (floating; the `rust` tag moved during this work) | `rust:1.98.1-bookworm@sha256:93ce27a8…971e`, `gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79…182f`; the `# syntax=` frontend image is gone (the engine's own frontend builds it) |
| Runtime content | Debian with apt packages `curl`, `ca-certificates`, `tini`; user `quiz` by name | distroless: glibc and nothing else — no shell, no package manager; `USER 65532:65532` by number; the binary (7.9 MB), the content (92 kB), the stack files (7 kB). Image 31.5 MB (was 97.6 MB) |
| Health check | `curl` in a shell | `HEALTHCHECK … CMD ["/usr/local/bin/proctor", "health"]`: the binary asks its own listener for `GET /instance` as the proxy would. `--start-period=5m --start-interval=2s`: healthy 3 s after start, never "unhealthy" during a long boot refold |
| PID 1 / stop | `tini` | the proctor is PID 1 and handles SIGTERM itself (`STOPSIGNAL SIGTERM`); `docker stop` drains with exit code 0 while a presence socket is open (measured 0.3–3 s) |
| Read-only root | not tried | the image runs with `--read-only --cap-drop ALL --security-opt no-new-privileges` (the image check starts it that way) |
| Catalog | copied | validated in the build (`proctor check` on the staged catalog): an image with content it would refuse to serve is never built |
| Labels | `source`, `title` | plus `description`, `licenses`, `version` (workspace version), `revision` (commit, `-dirty` when tracked files differ), `created` (commit time, so a rebuild of the same commit is the same image) |
| Build context | whole repository through the root `.dockerignore` (2.55 GB on the first build) | an allowlist: Rust sources, manifests, the quiz content, the two stack files — 180 MB from a clean tree. The compile layer is reused until one of those changes. Taxonomy contract `dockerfile-dockerignore` added |
| Stack files | on the host only via `git clone` | inside the image at `/srv/quiz/deploy`; a host copies them out (`docker create` + `docker cp`) and needs no clone |
| Architectures | — | the workflow publishes `linux/amd64`; the same Dockerfile builds natively on `linux/arm64` (not built here) |

### Compose (`🚀️deploy/compose.yaml`)

- `build:` removed: a host has no repository. `pull_policy: missing` stays, so a locally built image is used as it is.
- Both services: `read_only: true`, `cap_drop: [ALL]` (Caddy keeps `NET_BIND_SERVICE`), `no-new-privileges`, log rotation
  5 × 10 MB, limits (proctor 2 CPUs / 1 GiB / 512 pids, Caddy 1 CPU / 256 MiB / 256 pids), `restart: unless-stopped`.
- Caddy pinned (`caddy:2.11.4@sha256:0c994536…9b52`) and health-checked through its admin endpoint; it starts once the
  proctor is healthy.
- An optional `.env` beside the file is the environment of both services (`env_file`, `required: false`): every
  `PROCTOR_*` variable an operator sets reaches the proctor without compose listing it, and
  `PROCTOR_LIMIT_BODY_BYTES` reaches Caddy too — one knob, proven by the stack check with a non-default value.
- Volumes keep their project-scoped names (`architecture-quiz_proctor-data`, …); the proctor port is only `expose`d.
- The header no longer claims that `up -d` updates; it names backup, pull, up, and warns about `down --volumes`.

### Caddy (`🚀️deploy/Caddyfile`)

- `request_body max_size {$PROCTOR_LIMIT_BODY_BYTES:16384}` — the proctor's own limit (edge: 16384 bytes). Checked on the
  direct port (the proctor refuses 16385 bytes with 413 and reads 16384) and through Caddy.
- Timeouts `read_header 10s`, `read_body 30s`, `idle 2m`; upstream `dial_timeout 3s`, `response_header_timeout 30s`. A
  presence WebSocket held through Caddy for 180 s stayed open (`deploy_readiness_ws_hold.ts`).
- `lb_try_duration 20s`: while the proctor container is replaced, requests wait instead of failing — 60 of 60 requests
  answered `200` across a 6 s container replacement; the stack check asserts it on every run.
- Response headers: HSTS, `nosniff`, `X-Frame-Options: DENY`, `Content-Security-Policy: default-src 'none'; frame-ancestors
  'none'`, `Referrer-Policy`, no `Server`, no `Via`. HTTP → HTTPS `308`. A client's forged `X-Forwarded-Proto` changes
  nothing (Caddy replaces it). HTTP/3 advertised.
- `skip_install_trust` (the read-only container cannot install the internal CA used for local checks), admin-endpoint
  log lines excluded (the health check asks every 15 s), `caddy fmt` clean, `caddy validate` without a warning.
- Access log: off, documented as a decision — neither service records client addresses; the runbook says how to turn it
  on and that the retention must be decided first.
- Not possible with stock Caddy: rate limiting (needs a custom build). The proctor now has its own limits (edge).

### Data

- `proctor.sqlite` on the volume `architecture-quiz_proctor-data`; after a clean stop there is no write-ahead log left
  (asserted by the image check, read with `bun:sqlite`, an engine that is not the proctor's).
- Backup while serving: `docker compose exec -T proctor proctor backup - > file` (`VACUUM INTO` through a read-only
  handle; stdout because the container has no shell and a read-only root). Restore: `docker compose run --rm -T --no-deps
  proctor restore - < file` on a stopped proctor; it refuses a file that is not a whole proctor database and refuses
  while a proctor serves the volume (`… is in use (database is locked)`). Both verbs were first written here and are
  now owned and extended by the agent "domain" (`health`, `backup <directory/|file|->`, `restore`, `erase`).
- `rebuild` needs a stopped proctor and the catalog; the boot does it by itself when the catalog fingerprint or the
  projector revision changed. A database of another storage format is refused at boot with both formats named (the
  format went to v2 in this round: a v1 dev database is refused).
- The stack check proves on every run: backup while serving, an update with the data kept, restore refused while
  serving and accepted once stopped, erase on request.

### Site / CDN

- **Content-Security-Policy** as a `meta` tag (the only way on GitHub Pages), written by the site's Vite build and
  verified independently by `publish`: `default-src 'self'`; `script-src 'self'` plus the SHA-256 of each inline boot
  script; `style-src 'self'` plus the hash of the boot style; `style-src-attr 'unsafe-inline'`; `img-src 'self' data:`;
  `font-src 'self'`; `connect-src` exactly the baked proctor origin and its `wss:` twin; `object-src 'none'`;
  `base-uri 'none'`; `form-action 'self'`. Browser check: Chrome, both languages and appearances, no violation, no
  console error on the boot screens (`deploy_readiness_csp_probe.ts`); the full journeys run sealed in the end-to-end
  gate's rehearsal topology, whose devices fail a spec on any console error (final gate run: 19 of 19 specs).
- `_headers` (CDNs that honour it): every response `nosniff`, `X-Frame-Options: DENY`, `frame-ancestors 'none'`,
  `Referrer-Policy`; hashed assets immutable; documents `no-cache`. GitHub Pages ignores it: there, every file is cached
  ten minutes and deep links answer `404.html` with status 404 — documented.
- `robots.txt`, `manifest.webmanifest`, `theme-color` for both appearances, a description per language of the catalog,
  no default language in the document (with the agent "ui", who moved that into the shared host template).
- `publish` now also refuses: a `404.html` that is not the document, a script or stylesheet the document names but the
  artifact lacks, an unhashed file under `assets/` (they are served immutable), a source map, development leftovers
  (`sourceMappingURL=`, `/@vite/client`, `react-refresh`, `[DEBUG]`), and an artifact over budget (entry script
  260 kB gzip, stylesheet 60 kB gzip, 4 MB in total). Today: 59 files, 2.13 MB.

### Workflow (`.github/workflows/architecture-quiz.yml`)

- `actionlint` 1.7.12 (container `rhysd/actionlint`, test tool only): 0 errors.
- Both release jobs run on `main` only; every action pinned to a commit (`checkout` v7.0.1, `setup-node` v7.0.0,
  `setup-bun` v2.2.0, `cache` v6.1.0, `upload-pages-artifact` v5.0.0, `deploy-pages` v5.0.1, `login-action` v4.6.0; their
  inputs read from the pinned commits); `persist-credentials: false`; `bun install --frozen-lockfile --ignore-scripts`;
  the registry login happens after the image is built; `runs-on: ubuntu-24.04`; timeouts; bun's package cache.
- Least privilege unchanged and now tested: top level `contents: read`; `pages` job `pages: write` + `id-token: write`;
  `proctor` job `packages: write`.
- It uses the repository's verbs only: `test`, `publish`, `docker-image-build`, `docker-image-publish`. The publish verb
  itself runs the image check and the stack check and pushes only an image built from the checked-out commit without
  uncommitted changes, as `sha-<commit>` (immutable), `<version>` and `latest`, printing the digest.
- One dispatch deploys both parts or each one (inputs `site`, `proctor`).

### Single source of truth

- `🚀️deploy/🧬️schema/🔣️.json` (draft-07) is the contract of `🚀️deploy/🔣️.json`; validated with `ajv` in the tests.
  `site.legal { imprint?, privacy? }` is part of it (agent "ui" links them in the footer).
- `deploymentDrift()` (in `🚀️deploy/🟦️.ts`; step 1 of the gate and a test with seeded drifts): any host or image
  repository other than the declared ones in the Dockerfile, compose, Caddyfile, workflow, both READMEs or the package;
  the baked port, origin, account, health probe and stack files of the image; base images and actions without a pin;
  a target named in docs, workflow or root scripts that the package does not have; the Pages artifact path.

## 3. Commands run and results

| Command | Result |
|---|---|
| `bun nx run @teaching/architecture-quiz:check` (baseline) | exit 0: catalog `architecture`, 4 quizzes, 7 badges |
| `bun nx run @teaching/architecture-quiz:build --skip-nx-cache`, `:publish` (baseline) | exit 0; 57 files, 2.06 MB staged |
| `bun nx run @teaching/architecture-quiz:docker-image-build` (baseline) | **failed** (F2), second run exit 0. The image check and the stack check of the files as they were did not run: I replaced those files before a baseline image existed; a probe of that image confirmed that `docker stop` drains with a presence socket open (297 ms, exit 0) |
| `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` (03:19) | exit 0: 4 files, 40 tests (9 of them the deployment tests) |
| `bun nx run @teaching/architecture-quiz:docker-image-build` (final files, engine 29.8.1) | exit 0; 31,538,984 bytes in the final gate run |
| `bun nx run @teaching/architecture-quiz:docker-image-check` | exit 0: labels, user 65532, no shell, stack files of this checkout inside; ready after 1.3 s read-only without capabilities; cleartext 403; preflights 204 with the grant, foreign origin none; body 16385 → 413, 16384 read; presence between two learners, foreign origin refused; HEALTHCHECK healthy after 3.3 s; stop drained in 0.5 s with a socket open, exit 0; volume holds `proctor.sqlite` only, 1 event |
| `bun nx run @teaching/architecture-quiz:docker-stack-check` | exit 0: footprint `Caddyfile` + `compose.yaml` from the image; `.env` overrides; `caddy fmt`/`validate` clean; healthy after 5.9 s; hardening as inspected; headers; 308; forged proto ignored; cross-origin; body 32769 → 413, 32768 read (limit from `.env` in both services); presence over `wss`; backup 90 kB read with `bun:sqlite`; update with no failed request, learners kept; restore refused while serving, accepted once stopped; erase by handle; `down --volumes` left nothing |
| `bun 📜️script.ts docker-image-publish` with an empty `DOCKER_CONFIG` (no credentials) | refused before any push: `was built from 48d881aa…-dirty: only an image of a commit without uncommitted changes is published` |
| `deploy_readiness_runbook_rehearsal.sh` (the runbook's host commands verbatim, `.env` with `PROCTOR_HOST=localhost`) | exit 0: bootstrap from the image, verify, backup, cron line, update, rollback to `sha-48d881aa7ab5`, restore, rebuild, logs, `down` keeps the volumes |
| `deploy_readiness_linux_site_job.sh` (workflow `site` job in `node:24.15.0-bookworm`, fresh clone + working tree, bun 1.3.14) | first run **failed** (F1); after the fix: install exit 0, `test` 33/33, `publish` exit 0 (59 files), 0 tracked files changed afterwards |
| `deploy_readiness_clean_tree_build.ts` (every committable file, no ignored file) | exit 0: context 180 MB, image builds, catalog validated |
| `actionlint` on the workflow | 0 errors |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` (03:34) | `clean=false errors=1`: `directory-kind-unresolved 🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity` — a new directory of the agent "edge", not registered yet. `--scope "🎓️teaching/🏛️architecture"` (everything of this package): `clean=true errors=0 warnings=0` |
| `bun nx run @teaching/architecture-quiz:deploy-check` (the gate), 02:47:50–03:18:01 | **exit 0**, `ready to deploy`: no drift (0 s); catalog valid in the Rust core (211 s); site built and verified, 59 files, 2,142,996 bytes (45 s); image built (573 s, including one compiler crash and the second attempt); image checked (13 s); stack checked (52 s); end-to-end gate (836 s): dev 19 passed, rehearsal 19 passed. Two warnings: `site.legal.imprint` and `site.legal.privacy` are not set |
| the same gate at 02:15 (earlier tree) | steps 1–6 green; end-to-end: dev 19 passed, rehearsal 15 passed and 1 failed (`🎯️quiz-runs` "one mistake per task in physics": a drag landed in the neighbouring category; no Content-Security-Policy violation in its logs; passed in the final run). Its run directory is kept: `.🧬semio/🎓️teaching/architecture-quiz-e2e/20261002T002714-39256` |
| `bun nx run @teaching/proctor:test --skip-nx-cache` | first run exit 1: the target's 15 s budget killed it while the end-to-end binary was still running (12 of 14 tests done, none failed); second run (03:34) exit 0: 77 unit, 15 conformance, 14 end-to-end. `cargo test -p teaching-proctor` without the budget: the same counts, all passed |

Linux: the image is built and run in Linux containers (that is the Linux verification of the proctor); the site's
`test` and `publish` ran on Linux in the rehearsal container. Not run on Linux: the Docker checks as driven by bun on a
Linux host (they ran from Windows against the Linux engine), and the workflow itself.

## 4. Remaining owner-only steps

1. Put the imprint and privacy URLs into `site.legal` of `🚀️deploy/🔣️.json` (the gate warns until then) and commit.
2. DNS: `quizzes.architektur-und-technologie.de CNAME usalu.github.io`; `proctor.quizzes.architektur-und-technologie.de`
   `A` (and `AAAA` only if the host has IPv6); the `_github-pages-challenge-usalu.quizzes…` `TXT` record GitHub shows.
3. GitHub account → Settings → Pages: add and verify the domain.
4. Repository → Settings → Pages: Source "GitHub Actions", custom domain, later "Enforce HTTPS".
5. Push to `main`, then Actions → "architecture quiz" → Run workflow on `main`.
6. After the first push: make the GHCR package `architecture-quiz-proctor` public.
7. A `linux/amd64` host with Docker Engine ≥ 25, ports 80/443 (tcp) and 443 (udp) open: the four bootstrap lines of the
   runbook.
8. Verify with the table of the runbook; set up the daily backup line and an uptime probe.

## 5. Open issues

- O1. The pinned nightly compiler crashed twice today inside the builder (F2), in two different crates. The second
  attempt in the Dockerfile absorbs it; the cause is in the toolchain (or in this host under load) and is not found.
- O2. The local build context scan takes 20–140 s on this (busy) host: an allowlist with `!**/*.rs` makes BuildKit walk
  the whole repository, host state included. A clean checkout (CI) has nothing to skip. Exceptions without wildcards
  would let BuildKit prune, at the price of an extension blocklist that has to be maintained.
- O3. Closed: `dev`, `dev-site` and `test-e2e` needed the same two generator dependencies as `build` (F1) on a fresh
  clone; reported, and the agent "dev-e2e" has added them. Not rehearsed on a fresh clone: those three targets.
- O4. The whole `🎓️teaching` scope is not taxonomy-clean at 03:34 because of `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity`
  (edge). Earlier the `root-script` grammar rejected `DevScript` of the site's `📜️script.ts` (bisected with
  `deploy_readiness_router_bisect.ts`, reported); that one is gone.
- O9. Two gates of other owners were unstable on this loaded host: the rehearsal spec "one mistake per task in physics"
  failed once in two runs (a drag landed one category off), and `@teaching/proctor:test` exceeded its 15 s budget once in
  two runs. The readiness gate inherits the first: it is only as steady as `test-e2e`.
- O5. No request rate limit at the proxy (stock Caddy has none); the proctor's own limits (edge) are the protection.
- O6. Nothing monitors the stack or copies backups off the host by itself; the runbook gives the commands.
- O7. Host trap: a file loaded by a running bun process cannot be truncated on Windows; bun's `writeFileSync` then leaves
  the old tail in place and python's `open(…, "w")` fails with `Errno 22`. It corrupted two files for three minutes here
  (repaired). Source files are to be changed with the editor tools only.
- O8. Under load `bun nx …` sometimes fails before any task starts ("Plugin Worker … did not receive a load message
  within 10 seconds"); `NX_PLUGIN_NO_TIMEOUTS=true` avoids it. Not a deployment matter.

## 6. Files

Created: `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile.dockerignore`, `…/🚀️deploy/🧬️schema/🔣️.json`.

Updated: `…/🚀️deploy/Dockerfile`, `…/🚀️deploy/compose.yaml`, `…/🚀️deploy/Caddyfile`, `…/🚀️deploy/🔣️.json` (`$schema`),
`…/🚀️deploy/🟦️.ts`, `…/🏗️builder/🌐️vite/🟦️.ts` (release document plugin), `…/🧪️tests/🧪️deploy/🟦️.ts`,
`…/📦️packages/🟦️typescript/📜️script.ts` (`deploy-check`, `checkQuizImage`/`checkQuizStack` signatures),
`…/📦️packages/🟦️typescript/📋️project.json` (`deploy-check`, generator dependencies), `…/README.md` (section "Deploy" and
three rows of the path table), `.github/workflows/architecture-quiz.yml`, `.dockerignore` (header), `package.json`
(`check:teaching:architecture-quiz:deploy`), `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` (one row),
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (contract `dockerfile-dockerignore`).

Written by me, now owned by "domain"/"edge" (they built on it): `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs`
(`ready` → `health`, `backup`, `restore`), `…/🗄️storage/🦀️.rs` (`Database::snapshot`, `inspect`, `adopt`),
`…/🎚️config/🦀️.rs` (`data_directory`, `probe_address`), and their tests.

Ticket folder (inputs kept): `deploy_readiness_ws_stop.ts`, `deploy_readiness_ws_hold.ts`, `deploy_readiness_rustc_crash.sh`,
`deploy_readiness_csp_probe.ts`, `deploy_readiness_router_bisect.ts`, `deploy_readiness_linux_site_job.sh`,
`deploy_readiness_runbook_rehearsal.sh`, `deploy_readiness_clean_tree_build.ts`, `deploy_readiness_gate_when_stable.sh`,
`deploy_readiness_splice_readme.ts`, `deploy_readiness_docstring_emojis.ts`, `🗒️deploy-notes-for-domain-and-edge.md`.
Output under `🗑️generated/deploy/` deleted.

Docker afterwards: `ghcr.io/usalu/architecture-quiz-proctor:latest` (the deliverable: image `06ae71e455a3`, 31.5 MB,
built 03:02 in the final gate run from the tree described above, revision label `48d881aa…-dirty`) and the bases it
needs (`rust:1-bookworm`, `caddy` 2.11.4 and distroless by digest). Removed again: every container, volume and network
of the checks, the tags `deploy-readiness-*`, `sha-48d881aa7ab5`, and the test tool images `rhysd/actionlint` and
`node:24.15.0-bookworm` with the rehearsal volume. Left as they were: `semio/architecture-quiz:latest` (obsolete, from
2026-09-28), the image and volumes of the devcontainer. Docker Desktop is left running (stopping it recreates the stale
socket of F3). On disk under `%LOCALAPPDATA%`: `Docker\run-stale-20261001` and `docker-secrets-engine-stale-20261001`,
safe to delete.
