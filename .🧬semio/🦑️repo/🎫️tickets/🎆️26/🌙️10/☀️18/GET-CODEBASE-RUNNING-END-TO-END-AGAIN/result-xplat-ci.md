# Result xplat-ci: build-system CI (G21, WP-G2)

Date 2026-10-19. No push, no CI run, no `bun repo` build step, no git-modifying command, no sub-agent. `architecture-course.yml` untouched.

## Changes

- New `.github/workflows/build-system.yml` (only file touched).

Triggers: `pull_request`, `push` to `main`, `workflow_dispatch`. `permissions: contents: read`. Concurrency per PR number or ref, cancel-in-progress for pull requests only. No `sudo`, no `nx`/`bazel`/`cargo` call, no installer: every build step is `bun repo ...`.

Both jobs use a matrix `ubuntu-latest` (X64), `macos-latest` (ARM64), `windows-latest` (X64), `fail-fast: false`.

### Job `smoke` (60 min)
1. Windows only, before checkout: `git config --system core.longpaths true`, `core.autocrlf false`, registry `LongPathsEnabled=1` (tracked paths reach 314 bytes).
2. `actions/checkout` (pinned SHA, `persist-credentials: false`), `actions/setup-node` 24.15.0 (the repo `engines.node`).
3. Pin check: `package.json` `packageManager` must equal `bun@$BUN_VERSION`, `engines.node` must equal `$NODE_VERSION`, runner arch must equal the arch of the pinned Bun asset.
4. Bun 1.4.2 installed from the official release zip, SHA-256 verified against hashes pinned in the workflow (unix: curl + `sha256sum`/`shasum`; Windows: `Invoke-WebRequest` + `Get-FileHash`), then `bun --version` must equal the pin. Hashes read read-only from the release `SHASUMS256.txt`; the linux-x64-baseline and linux-aarch64 rows equal those already pinned in `.devcontainer/Dockerfile`. Used assets: `bun-linux-x64` 36368fae..., `bun-darwin-aarch64` 90987a3a..., `bun-windows-x64` ce4c1749....
5. `bun repo --help`, `bun repo preflight`, `bun repo tools setup all`.
6. Cache keys computed once (a step output, so restore and save use the same key; recomputing after install would hash a different tree), `actions/cache/restore` of `.cache/b/d/m/bun-cache` (key `bun-store-<os>-<arch>-<hash of bun.lock, */bun.lock, package.json>`, prefix fallback) AFTER `tools setup all` (the path lives in the dependencies partition, which setup attaches), then `bun repo dependencies sync --all`.
7. Cache proof, one loop for `nx` then `bazel` with `PROOF_TARGET=@teaching/architecture-course:test` (cacheable, no dependsOn, the target `C/nx-cache-tools/verify-nx-cache.ts` already uses): `cache clear`; `cache status` (entries, bytes, parsed from the printed `CacheStatus` JSON by an inline `bun -e`); run 1 must grow entries and bytes; run 2 must leave entries unchanged and be recognized as a hit. Hit rule: nx = Nx output `read the output from the cache` / `[local cache]` / `[existing outputs match the cache]` (the same markers as `verify-nx-cache.ts`); bazel = `disk cache hit`/`remote cache hit` in the log, or run 1 spawned a process (`N linux-sandbox|darwin-sandbox|processwrapper-sandbox|windows-sandbox|local|worker`) while run 2 spawned none and printed `Build completed successfully`.
8. `actions/cache/save` of the Bun store, only on a non-PR run on the default branch and only after a restore miss.

### Job `products` (90 min, `needs: smoke`)
Same setup (long paths, checkout, node, pin check, Bun, preflight, `tools setup all`), restores the Bun store and the Bazel disk cache (`.cache/b/p/m/bazel/actions`, key `bazel-actions-<os>-<arch>-<hash of MODULE.bazel, .bazelversion, .bazelrc, bun.lock>-<sha>`, prefix fallback), `bun repo dependencies sync --all`, then `bun repo --tool=nx run @teaching/architecture-course:test` and `bun repo --tool=bazel run @teaching/architecture-course:test`. Saves the Bazel disk cache on default-branch pushes only.

### Action pins (full SHA, verified with `gh api repos/<owner>/<action>/git/ref/tags/<tag>`, gh authenticated)
- `actions/checkout` v7.0.1 `3d3c42e5aac5ba805825da76410c181273ba90b1` (latest v7.0.1; same pin as the course workflow)
- `actions/setup-node` v7.0.0 `820762786026740c76f36085b0efc47a31fe5020` (same pin as the course workflow; v7.1.0 exists, not adopted to keep both workflows identical)
- `actions/cache` v6.1.0 `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` (latest, `restore` and `save` sub-actions)

## Verification

- `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 .github/workflows/build-system.yml` (GOPATH in the session scratchpad): exit 0, no findings; the same binary rejects a deliberately broken workflow (exit 1), so the check is live. `shellcheck` and `pwsh` are not installed here, so the embedded bash/PowerShell bodies were not linted by actionlint.
- YAML parsed identically by `Bun.YAML.parse` and the `yaml` package (jobs `smoke`, `products`; triggers as intended).
- Control-flow test of the proof step with a stubbed `bun` (extracted from the parsed YAML, no repo build): hit run exit 0 for both tools with correct entries/bytes arithmetic; a stub whose second run is not a hit exits 1 with the `::error::` message. The real `CACHE_STATUS_JS` was run against `[script]`-decorated sample output (parses `3 9`). Scratch files removed.
- `bun test ".../📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts"`: 14 pass, 7 fail. All 7 are pre-existing and unrelated (stale `❓️quiz` paths after the course rename, MCP staging env, and the `ciDependencies` law that opens `.github/workflows/architecture-quiz.yml`, which no longer exists). The one assertion that touches CI (no `bun nx` in `architecture-course.yml`) passes. No other repo test reads `.github/workflows` except the course deploy test, which reads only `architecture-course.yml`. NOT re-run: that course deploy test (not affected by an added file).
- NOT run: the workflow itself (by instruction), `bun repo preflight`, `tools setup all`, `cache clear|status`, any Windows or Linux step.

## Open items and dependencies (the first CI run is the real test)

1. `bun repo preflight` (no scope) is the coordinator's contract. Today `readBuildPreflightScopeV1` requires `--scope <owner>`, the code is Linux-only and requires CAP_SYS_ADMIN and loop devices. Green only after `xplat-linux`/`xplat-boot` make preflight scope-less, platform-neutral and unprivileged.
2. Unprivileged Linux backend (directory-backed partitions) from `xplat-linux`, and a Windows VHD path without failing, are prerequisites for `tools setup all` on those runners; Windows Bazel additionally refuses non-ASCII ancestry (the checkout path `D:\a\semio\semio` is ASCII) and emoji source paths are unverified (G10).
3. `cache clear` and `cache status` for Bazel must exist (spec in `result-nx-bound.md` section 3, "Spec for the Bazel side"); `--tool=bazel cache ...` currently throws until bzl-act lands it.
4. The Bazel hit rule is a calibrated guess from Bazel's console summary (`N processes: ... disk cache hit`, or no spawned process on a warm server). The adapter's actual output was not observed. If the first run fails on it, adjust the `bazel)` branch of `is_hit`; the log tail is in the step output. The Nx rule is taken from `verify-nx-cache.ts`.
5. Restoring files into the owned partitions (`bun-cache`, `bazel/actions`) bypasses their budget accounting (`dependencies.json`, `products.json`); whether the stores tolerate externally restored content-addressed files is unverified. Failure mode is a failing run, never a stale hit (both stores are content-addressed). Cache restore is soft (a miss is fine). Persisting mounted images instead of their contents was rejected (multi-GB sparse images).
6. Cache eviction: the Bazel disk-cache bound is up to ~9.8 GiB (`bazelDiskCacheMaximumBytesV1`) against GitHub's 10 GiB per repository; with only the course target it stays far smaller. Revisit when more products join `products`.
7. The `ciDependencies` law in `build-surfaces` (fixture `🧪️tests/🧫️fixtures/🔧️build-surfaces/🔣️.json`, schema, test) still names the retired `architecture-quiz.yml` and expects a step named `Install` plus `oven-sh/setup-bun`; owned by the course ticket (26/10/19/PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT). The Bun install here is a checksum-verified manual step on purpose (`setup-bun` does not verify a pinned hash), so extending that law to this workflow needs a decision. `dependabot.yml` already watches `/` for `github-actions`; the three action pins are covered, the pinned Bun hashes need a manual bump with `BUN_VERSION` (guarded by the pin check against `package.json`).
8. Not covered (out of the decision): devcontainer build, the native bootstrap scripts (`🐚️.sh`, `🔵️.ps1`), the other three products (`proctor`, `semio-tech-play`, `mit-bestand-demonstrator`) and Playwright. They can be added as further matrix rows of `products` once their targets are green.
9. Runner cost: 6 jobs per PR (3 smoke, 3 products), macOS and Windows minutes billed at higher multipliers; `products` waits for `smoke`, so a red smoke saves the long jobs.
