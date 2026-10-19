# Result course-img: course proctor image build on this host (macOS arm64, Docker Desktop)

## Outcome
- The runtime-only image recipe builds, starts, passes its HEALTHCHECK and drains on `docker stop` on this host as `linux/amd64`, **with a stand-in proctor**.
- The real Rust proctor does not compile on the current tree: the framework crates it depends on are in a half-finished refactor. No real release image was produced.

## Contract found (supersedes the brief)
The codex course ticket (`..../☀️19/PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT`, `deploy-contract.md`, `deployment-packaging.md`) removed the compiler stage and `Dockerfile.cargo.toml`. `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Dockerfile` is now one pinned distroless `cc-debian12:nonroot` stage. It `COPY`s a prebuilt `proctor`, the course JSON, `deploy/` and `data/`, and runs `proctor check` as a JSON-form `RUN`. Such an ELF (x64 or arm64) is the build input.
- The input comes from `buildCourseImage` (`🚀️deploy/🐳️docker-image/🟦️.ts`). It requires `process.platform === "linux" && arch === "x64"` and the canonical native compiler (`buildProctor` release), and streams a tar on stdin to `docker build -`.
- So `docker build .` from a checkout is not possible; the context is a finite tar from `courseImageContext`. `.cargo` is excluded twice, by `Dockerfile.dockerignore` and by the tar roster (only 12 entries).
- `Dockerfile.cargo.toml` no longer exists; the compiler-ingress rule is met by never having Cargo in the image path.

## What I ran
1. Docker Desktop: stopped once mid-session (16:02, nothing of mine killed it; the backend was shut down). I restarted it with `open -a Docker` and it came back. Docker Desktop cannot bind-mount `~/Documents` (`mkdir /host_mnt/Users/ueli/Documents: operation not permitted`, macOS file-access permission). All Docker use therefore avoids host bind mounts: sources go in through `tar | docker run -i ... tar -x` into volumes.
2. Real proctor build attempt in a container (`rust:1-bookworm`, `nightly-2026-07-20` per `rust-toolchain.toml`, `cargo build --release --locked -p teaching-proctor --bin proctor`, repo `.cargo/config.toml` not present, volumes only). Result: failed, details below.
3. Image build with a stand-in ELF (`C/🗑️generated/course-img/standin.rs`, std-only Rust, built `linux/amd64`, answers `check <catalog>`, `health` (GET /instance, same as the real CLI) and `serve`, handles SIGTERM, writes into `PROCTOR_DATA`).
   - Context: `bun C/🗑️generated/course-img/context.ts x64 <elf> <tar>` calls the repo's real `courseImageFile`, `courseImageContent`, `courseImageExecutable` and `courseImageContext` (`📦️image/🟦️.ts`) with the same source roster as `buildCourseImage`. Output: 12 entries, 5,109,760 bytes (Dockerfile, .dockerignore, proctor, 6 course/quiz JSON, compose.yaml, Caddyfile, data/).
   - Build, same argv shape as `buildCourseImage`: `docker build --progress=plain --platform linux/amd64 --build-arg VERSION/REVISION/CREATED --file Dockerfile --tag ghcr.io/usalu/architecture-course-proctor:cimg-verify - < context.tar`. Exit 0 in 8.9 s. Step 1/7 pulled the pinned digest (index digest, amd64 resolved), all 7 steps ran, and `RUN proctor check` printed "5 referenced quiz files present". Docker Desktop on Apple Silicon runs amd64 with Rosetta, so emulation cost was negligible (a `rustc` build of the 100-line stand-in amd64 took a few seconds).
   - Image facts: `linux/amd64`, 10.47 MB, `User=65532:65532`, HEALTHCHECK `["CMD","/usr/local/bin/proctor","health"]` (15 s interval, 5 s timeout, 5 min start period, 2 s start interval), OCI labels set. `docker export` contains only `usr/local/bin/proctor` and `srv/course/{content,deploy}` (no `.cargo`, no sources).
   - Smoke: `docker run -d --read-only --cap-drop ALL --security-opt no-new-privileges:true -p 18791:8791 -v <vol>:/srv/course/data`. Health went `starting` then `healthy`. `curl http://127.0.0.1:18791/instance` returned `200`. The process ran as uid 65532 and wrote into the data volume. `docker stop` took 0.28 s with exit code 0. `docker compose -f compose.yaml config -q` with `PROCTOR_TAG=cimg-verify` exited 0.
   - Cleanup: container, image, volumes (`cimg-*`) removed; build tars and ELFs deleted.

## Fixes made (root causes found by the real proctor build)
1. `🎓️teaching/Cargo.lock` was stale against the manifests, so `--locked` refused to build: 8 framework path crates were missing (`course-presence`, `pack-json-float`, `replication-mutation-retained`, `server-presence`, `value-edit`, `value-persistent`, `value-registry`, `value-schema`), later also `value-derive-expansion`. I regenerated it in a container with `cargo fetch` (only additions; no registry version changed; +190/-14 lines vs HEAD). Backup of the pre-change lock: `C/🗑️generated/course-img/Cargo.lock.orig`.
2. `🧰️framework/🔨️modules/🎒️pack/🔤️json/🎯️float-writer/🦀️.rs`: `format_f64` called the removed `write_float`; now `let _ = write_float_to(value, &mut out);` (infallible into a `String`).
3. `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml`: added `semio-framework-value-derive-expansion = { workspace = true }`; the crate's `🦀️.rs` already did `use semio_framework_value_derive_expansion as component`.

## Remaining Rust blockers (not fixed; they belong to the in-flight framework refactors, edits stopped at 14:55-15:15)
- `semio-framework-value-retirement` (`🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️packages/🦀️rust/../../🦀️.rs:658`): `impl RetirementCursor for PagedListRetirement` lacks the new required method `next_demand` (trait at line 41).
- `semio-framework-value-derive-expansion` (`.../✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs:546,550`): `controlled_semantic_admission` is not visible (`controlled_output` module item is private).
- More errors may follow downstream; cargo stopped at the first failing crates. After fixing these, rerun the proctor build (`bun repo cargo ...` or the container recipe) and then run the TS `docker-image-build` unchanged.

## Still needs the Linux x64 runner or `bun repo run`
- The real `proctor` ELF through `buildProctor` (authenticated release, native compiler admission: Debian 12 x64, CAP_SYS_ADMIN, loop devices, 72 GiB ext4 backing). The preflight refuses macOS.
- `docker-image-build` itself (`bun repo run ...:docker-image-build`) is platform-gated to linux/x64; I reproduced only its tar + `docker build` step.
- `docker-image-check` (CORS preflight, learner/presence round trip, body limit, WAL fold): needs the real proctor, so not run.
- `docker-stack-check` and `docker-stack-bundle` (Caddy + proctor under compose): not run; only `compose config -q` passed.
- Image publish (`docker-image-publish`): needs a clean commit, a registry login and the checks above; not run.
- Release browser E2E, canonical typecheck and the dependency sync: `bun repo run`; not run (sync in progress elsewhere).

## Not run
Vitest deploy contract suite (root `node_modules` has no vitest), `bun repo run` anything, host cargo.
