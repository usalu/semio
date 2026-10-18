# Result quizdock: proctor image build and dev-stack proctor build

## Changed
- `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile.cargo.toml` (new): builder-local Cargo config, installed as `$CARGO_HOME/config.toml` (`/usr/local/cargo/config.toml`). Same values as `.cargo/config.toml` for what the proctor build needs: `[build] rustflags`, `[unstable]` (embed-metadata, fine-grain-locking, build-dir-new-layout, checksum-freshness), the linux-gnu LLD `rustflags`, `[profile.release]`. No `rustc`, `rustc-wrapper`, `target-dir` (`CARGO_TARGET_DIR=/out` stays) or `[env]`. `build-dir = "/cache/build"`.
- `Dockerfile.dockerignore`: `!.cargo` removed; `.cargo` and `**/.cargo` excluded after the admissions; `Dockerfile.cargo.toml` admitted.
- `Dockerfile`: `COPY Dockerfile.cargo.toml /usr/local/cargo/config.toml` after the toolchain layer and before `COPY . .`; cache mount moved from the dead `/src/.🧬semio/…/cargo/build` to `/cache/build` (the old path never matched the config's build-dir); `test ! -e .cargo &&` guard before the build; `--release --locked` kept; comments updated.
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`: new `REPOSITORY_BUILD_ENTRY` and `proctorBuildLaunch(repoRoot, env, profile)`; `buildProctor` now spawns `bun <repo build script> repo cargo --store=shared --configuration=prototype|release -- build --locked --package teaching-proctor --bin proctor --message-format=json-render-diagnostics` with cwd `🎓️teaching` and `SEMIO_BUILD_MODE` set to the same configuration (the launcher refuses a mismatch). dev -> prototype, release -> release. Covers `dev`, `check`, `rebuild`, `health`, `backup`, capacity (`buildProctor(..., "release")`), the quiz local stack and the e2e gate (all call `buildProctor`).
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧪️deploy/🟦️.ts`: removed `!.cargo` from the admitted list, added admission of `Dockerfile.cargo.toml` and `.cargo`/`**/.cargo` to the exclusions; new test "compiles the proctor image under its own Cargo configuration, never behind the repository's launcher-only compiler gate" (parses both configs with third-party `@iarna/toml`; builder config equals the repository's on the kept keys and has no rustc/wrapper/target-dir/env; COPY order; cache mount equals build-dir; guard; no RUSTC/bun/SEMIO_ in the builder stage).
- `🧪️tests/🧱️local-stack/🟦️.ts`: new `describe("the build of the proctor")` for `proctorBuildLaunch` (launcher argv, cwd, mode env, release variant, entry file exists).
- `🎓️teaching/🏛️architecture/❓️quiz/README.md`: deploy row mentions `Dockerfile.cargo.toml`.

## Verified
- `bun build <file> --no-bundle` transpiles the three edited TS files.
- `bun` run of `proctorBuildLaunch` against the real repo: entry exists, argv as above, cwd teaching, env mode prototype.
- `Bun.TOML.parse` check: builder config equals the repository config on build/unstable/profile.release/linux target keys (all true).
- `docker info`: client present, daemon not running (`~/.docker/run/docker.sock` missing). Not touched; no image build attempted.
- `bun test`/vitest not run: root `node_modules` holds no packages (no vitest, ajv, yaml, @iarna/toml). No cargo/bun repo builds run.

## Open
- Unverified at runtime: image build under the new config (needs Docker daemon), and that a nested `bun <script> repo cargo` inside an outer `bun repo run` is accepted by the launcher (precedent: `schedulerBinaries` in native-build tests spawns it the same way; `bun repo` entry itself reported broken).
- Stream-mode launcher forwards cargo's stdout (JSON messages) to its own stdout; `proctorExecutable` parsing relies on that.
- `build-dir` in the builder config relies on the pinned nightly honoring `build.build-dir` (stable since 1.91).
- Ticket not opened/closed (repo MCP unavailable in this session).
