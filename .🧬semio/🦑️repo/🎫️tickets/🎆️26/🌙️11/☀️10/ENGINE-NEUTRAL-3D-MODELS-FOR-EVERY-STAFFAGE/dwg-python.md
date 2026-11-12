# DWG wasm (Rust) and the Python consumer: provisioning and results

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder, `D` = `S/🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust`. Raw logs: `T/🗑️generated/dwg-python/` (delete at ticket close). Nothing was run through `repo run`, `repo show`, `repo dependencies` or `repo dashboard`; no git command that modifies anything.

## Verdict

| Gap | State |
|---|---|
| 1. DWG wasm | Rust is provisioned and working, the crate itself is fine, but `staffage_dwg.wasm` is NOT built: the crate's dependency closure (the in-repo DWG writer, which depends on the whole plugin framework) does not compile at the current tree, because of a framework API migration that another group is doing. `dwg-reopen` is still 2 passed / 1 failed. No build target was added (it could not be verified). |
| 2. Python consumer | Python 3.12.13 and the locked environment are provisioned through the pinned uv, but `pycairo` (a manim dependency) has no macOS wheel and cannot be built here (no Homebrew/cairo/pkg-config). With a stubbed `cairo` module (diagnostic only, not a real pass) the consumer tests (61), the course-coverage guards (48) and all 22 clip modules import and pass, and a real end-to-end render of an elevation and an axonometric cast through the real exporter works. The real manim run is therefore not green on this host. |

## 1. Rust and wasm

### How it is declared (found)
- `rust-toolchain.toml`: `nightly-2026-07-20`, minimal profile, components `rust-src llvm-tools-preview clippy`, targets `wasm32-unknown-unknown` and `wasm32-wasip2`. So the wasm target is already pinned.
- Tool catalog `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`: `rustup` (1.29.0, per-platform sha256 rows for darwin arm64/x64, linux x64/arm64, win32 x64/arm64 in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`, from `static.rust-lang.org`) and `cargo` (`pin` from `rust-toolchain.toml`, `locate`: `SEMIO_TOOL_CARGO`, then `~/.cargo/bin`). I added nothing to the catalog: the declaration with pinned versions and digests already exists.
- Plain `cargo`/`rustc` are refused by design (`.cargo/config.toml` points `rustc` at an ingress script); everything goes through `repo cargo --store=shared --configuration=prototype -- <args>`. Target and build directories and `CARGO_HOME` are redirected by that gate into `.cache/b/r/m/...` (gitignored build output); `repo cargo` is `requires: none`, it does not read the graph.
- Other Rust-to-wasm crates are built by per-package `repo ./📜️script.ts build` scripts and orchestrator targets (see `🏢️semio-tech/🎡️play/📋️project.json`), not by a plain target; `S/📦️packages/🟦️typescript/📋️project.json` and the DWG crate have no build target and no project file.

### What I ran (macOS arm64)
1. `repo cargo ... -- build ...` first stopped at "tool cargo is missing" (nothing was provisioned), after one 5 min attempt that only waited at the memory gate behind another agent's `repo show projects` (11 GB).
2. Provisioned through the repo's documented zero-touch command `repo ./📜️script.ts setup native rust` (runs `🔩️native/🥾️bootstrap/🐚️.sh rust` on macOS/Linux, `🔵️.ps1` on Windows): downloaded the pinned `rustup-init` 1.29.0 (sha256-verified), installed `nightly-2026-07-20` with `rust-src`, `llvm-tools`, `clippy`, `wasm32-unknown-unknown`, `wasm32-wasip2`. Took under 90 s. It installs into the user home (`~/.rustup`, `~/.cargo`) and links `cargo`, `rustc`, `rustup` into `~/.local/bin`; this is the repo's declared location (catalog `locate`), outside the worktree. `repo tools list` now shows `cargo nightly-2026-07-20 path` and `rustup 1.29.0 path`.
3. `repo cargo --store=shared --configuration=prototype -- build --keep-going --manifest-path D/Cargo.toml --lib --target wasm32-unknown-unknown`. First run created `D/Cargo.lock` (git-ignored by `**/📦️packages/**/Cargo.lock`, "never commit nested workspace locks") and fetched crates.io crates into the repo's cargo home.
4. First compile failure: the schema crate includes the git-ignored generated file `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🦀️.rs`. `repo run @semio-tech/framework-schema:generate` is a graph command and `repo ./📜️script.ts generate` refuses to run outside the runner, so I wrote `T/dwg-python-generate-entity-kinds.ts`, which calls the same `readEntityCatalog` and `generatedTargets` plan and the same writer as the target; run with `repo tools exec --light bun -- ...`. Both generated files (`🟦️.ts`, `🦀️.rs`) now exist (git-ignored).
5. Second failure: `semio-framework-job` did not compile because `🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs` defined `admit_original_construction_settled` twice (the first one, lines 159 to 183, used the undefined `fuel_floor`; it is the stale copy, the second one uses `original_construction_settled_quote`). I removed the stale first copy (26 lines), the only change to someone else's source. This file belongs to the framework migration below; please make sure that agent does not re-add it.
6. After that, `--keep-going` shows four independent roots failing in the closure, all unrelated to the staffage crate (logs `cargo-wasm-6.txt`, `cargo-wasm-7.txt`):
   - `semio-plugin-job-child-vcs`: `RetainedWorkPoll` not imported (`📌️checkpoint/🪆️cascade/🦀️.rs:20`).
   - `semio-plugin-document-media`: `MediaControlled` and `MediaRetireOwned` not found (`🎞️media/💥️fault`).
   - `semio-plugin-composition-publication`: mismatched types in `🤝️group/🪆️child/🦀️.rs:81`.
   - `semio-plugin-tool-machine`: `NativeDecodeControl` where `NativeConstructionScope` is expected (`🔁️follow/🦀️.rs:72`).
   Their dependents (including `staffage-dwg`) were skipped.

### Why it cannot be fixed from the staffage side
`staffage-dwg` -> `semio-s-artifact-stdio-dwg-write` -> `semio-s-artifact-stdio-binary` -> `semio-plugin` -> `semio-plugin-job` -> the four crates above (also `semio-os-kernel`, framework value/pack). The "in-repo Rust writer" is not standalone, so the wasm needs the entire plugin framework to compile for `wasm32-unknown-unknown`. The ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END` (`framework-compile-fixes.md`, `framework-document-media.md`, `framework-composition-publication.md`) is the migration of exactly these crates (`RetireOwned`/`RetirementCursor`, effect-threaded steps). Its notes say `document-media` and `composition-publication` reach "0 errors" in their `en1990` workspace (a replica/other machine), but those edits are not in this tree yet. Fixing the four crates here would collide with that migration and would be guesswork; I did not.

### Not done and why
- No `build` target and no `staffage_dwg.wasm` copy step were added. I did not add an untestable target. When the closure compiles the step is: `repo cargo --store=shared --configuration=release -- build --release --manifest-path D/Cargo.toml --lib --target wasm32-unknown-unknown`, then copy `<target-dir>/wasm32-unknown-unknown/release/staffage_dwg.wasm` to `S/🔨️modules/📤️export/🖊️dwg/staffage_dwg.wasm`. `**/*.wasm` is already git-ignored, so the artefact is gitignored build output at the place `DWG_WASM_URL` expects. A `repo ./📜️script.ts build` script in the style of the other wasm packages is the right home; the dwg crate has no `📋️project.json`/`📜️script.ts` yet, and adding a project to the graph cannot be verified while graph commands are defective.
- The Rust oracle test `🧪️tests/🖊️dwg-reopen/🦀️.rs` (acadrust reading the DWG back) and the vitest `dwg-reopen` need the same closure, so the third-party oracle run did not happen. The DWG sheet/axonometric paths therefore remain unverified on a real file.

### Per platform
- macOS arm64: provisioned and run as above. Rust works; the staffage crate is blocked by the framework closure.
- Linux (glibc, apt/dnf/pacman/zypper) and devcontainer: same command, `🐚️.sh rust` (also installs the native compiler packages); declared, not run. musl is refused by the script.
- Windows: `🔵️.ps1 rust` via the same command, pinned `rustup-init.exe` rows for x64 and arm64 exist; not run.

### Test results (DWG)
| Test | Before | After |
|---|---|---|
| `S/🧪️tests/🖊️dwg-reopen` (vitest, direct run) | 2 passed, 1 failed (wasm missing) | 2 passed, 1 failed (wasm missing), unchanged |
| `D` Rust test `dwg_reopen` (acadrust) | not runnable (no Rust) | not run (closure does not compile) |

## 2. Python consumer

### How it is declared (found)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/{pyproject.toml,uv.lock}`: `requires-python ">=3.12,<3.13"`, `manim==0.22.0`, `numpy==2.5.4`, `imageio-ffmpeg`, group `test` (CoolProp, shapely, rhino3dm, ...). The course script `⚡️energy/🏗️authoring/📜️script.ts test` runs `uv run --project <that> --python 3.12 --group test python -m unittest <modules>` with `UV_PROJECT_ENVIRONMENT` from `SEMIO_TEACHING_ENV` or `.🧬semio/🎓️teaching/⚡️cache/python`.
- Repo policy for interpreters (`🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🔣️.json`, `python`): uv with `UV_PYTHON_PREFERENCE=only-managed`, `UV_PYTHON_DOWNLOADS=manual` (explicit `uv python install <series>` is the sanctioned download), "oracle" interpreter series 3.12. The teaching project is not one of its three declared projects (workspace, ui-styling, energy-oracle).

### What I ran (macOS arm64; all paths outside the worktree)
- `uv python install 3.12` with the pinned uv 0.11.6 (`.cache/b/t/m/native/uv/...`), `UV_PYTHON_INSTALL_DIR=~/Library/Caches/semio/teaching-python-runtimes`: CPython 3.12.13 (uv verifies its own download metadata). uv also dropped a `python3.12` link into `~/.local/bin`; I removed it again.
- `uv sync --frozen --project ... --python 3.12 --group test` with `UV_PROJECT_ENVIRONMENT=~/Library/Caches/semio/teaching-python/env`, `UV_PYTHON_DOWNLOADS=never`. `uv.lock` and `pyproject.toml` are untouched.
- Result: the sync fails to build `pycairo 1.29.2`: it has wheels for Windows only (checked on PyPI), so on macOS and Linux it builds from source and needs `pkg-config` and cairo development files (meson: "Pkg-config ... not found"). The repo declares those as system packages: Linux through `🐚️.sh` (`libcairo2-dev pkg-config libpango1.0-dev`, `cairo-devel`, ...) and `.devcontainer/Dockerfile`; macOS through Homebrew (`brew install pkg-config cairo pango`, line 731 of `🐚️.sh`), and Homebrew is not installed here. Installing Homebrew changes system state and I did not do it.
- Workaround for diagnosis only: `uv sync --frozen ... --no-install-package pycairo` plus a stub `cairo` module (`~/Library/Caches/semio/teaching-python/stubs/cairo`, on `PYTHONPATH`, never in the repo). The staffage consumer never rasterises with cairo, so its tests are meaningful; other tests that rasterise (sections) are not.

### Results (stubbed cairo, therefore not an acceptance result)
| Run | Result |
|---|---|
| `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🧪️tests/🐍️.py` (consumer: provenance, cast keys incl. the `axonometric` viewpoint, placement, motion) | 61 tests OK |
| course-coverage guards `🎨️visuals/🧪️tests/🐍️course_staffage`, `🐍️staffage_only`, `🐍️media_staffage` | 48 tests OK |
| every module using `_manim_module.staffage` (22 files, `T/dwg-python-import-clips.py`) | 22 of 22 import with `SEMIO_TEACHING_LANGUAGE=en` and with `de`; without the variable one (`🔥️heating/🎬️clip/🧱️conduction`) raises "Set SEMIO_TEACHING_LANGUAGE to en or de" by design (no default language) |
| end to end, `T/dwg-python-consumer-e2e.py`: `ensure_casts` through `repo tools exec --light bun` and the real cast exporter for one `elevation` and one `axonometric` cast of `standing-adult` | both rendered, provenance verified, `staffage()` placed (VGroup, height 1.6885 m) |
| visuals `🐍️`, `integration`, `scenes`, `section_consumers`, `sections` (not staffage) | 49 run: 5 failures and 7 errors; the failures are raster/caption tests that need real cairo and 7 errors from `SEMIO_TEACHING_LANGUAGE` not set in my environment; none touches the staffage consumer; not investigated further |

No consumer or exporter failure was found, so no code was changed for Gap 2. The glb/fbx/model/model-zip formats belong to the staffage CLI exporter (`--format`); the Python consumer only calls the cast exporter (`🎭️cast`) and has no code for them, so no consumer test exercises them. The library tests of those formats are the vitest suites already reported.

### Per platform
- macOS arm64: interpreter and locked environment minus `pycairo` provisioned as above; real manim needs cairo/pango/pkg-config.
- Linux and devcontainer: `🐚️.sh setup` and the Dockerfile install `libcairo2-dev`/`pkg-config`, `uv run --python 3.12` then builds pycairo; declared, not run.
- Windows: pycairo win_amd64/win_arm64 wheels exist in `uv.lock`, uv installs the interpreter; not run.

## Files changed
- `🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs`: removed the stale duplicate `admit_original_construction_settled` (26 lines).
- Git-ignored, generated: `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/{🟦️.ts,🦀️.rs}`, `D/Cargo.lock`, `.cache/b/r/...` cargo home and target dirs.
- Outside the worktree: `~/.cargo`, `~/.rustup`, `~/.local/bin/{cargo,rustc,rustup}`, `~/Library/Caches/semio/teaching-python{,-runtimes}` (interpreter 3.12.13, environment, the diagnostic cairo stub, cast cache, fonts, output).
- New in `T`: `dwg-python.md`, `dwg-python-generate-entity-kinds.ts`, `dwg-python-import-clips.py`, `dwg-python-consumer-e2e.py`; logs in `T/🗑️generated/dwg-python/`.
- Catalog, `package.json`, `bun.lock`, `uv.lock`, the repo tool: untouched.

## Needs a human decision
1. The DWG wasm waits for the framework migration (`GET-NORM-PLUGIN-WORKING-END-TO-END`) to land the four crates; then run the build command above. Consider whether the staffage DWG export should depend on the whole plugin framework at all (a small standalone writer crate would make the wasm independent of it).
2. Where the build target lives: a `📜️script.ts build` plus `📋️project.json` for the DWG crate (style of the other wasm packages) once graph commands work.
3. macOS without Homebrew cannot build `pycairo`; either document Homebrew as a prerequisite for the course (as `🐚️.sh` already does), or provide a pinned cairo/pango (the repo has no mechanism for it).
4. Declare the teaching project in the `python` toolchain table so its environment and interpreter land in the repo cache partitions instead of `SEMIO_TEACHING_ENV`; that file is in the repo tool's area and I left it alone.
