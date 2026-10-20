# Result: x-proctor (proctor as a cached build product)

Executor x-proctor, started 2026-10-20 ~15:20. Status: STAGED SET COMPLETE 16:58; live code and tests done; real cold build PENDING the peer's GO (quiet tree). Handed to x-integrate. Paths relative to the repo root unless noted. `T` = this ticket folder.

## Design

1. `@teaching/proctor:build` produces, per mode, `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/dist/build/<mode>/receipt.json` and the executable at `.../dist/build/<mode>/<sha256>/proctor[.exe]` (content-addressed by the SHA-256 of the executable bytes). The receipt (schema `🧾️product/🧬️schema/🔣️.json`) names `executable` (relative to the mode directory), `sha256`, `bytes`, `platform`, `architecture`, `mode`, and `sources` (every in-repo file Cargo's dep-info says was read).
2. The build runs through the issued Cargo launcher of the shared store: `launchRepositoryCargoV1` with `readRepositoryCargoLaunchV1(["--store=shared","--configuration=<mode>","--","build","--locked","--package","teaching-proctor","--bin","proctor","--message-format=json-render-diagnostics"])` in the `🎓️teaching` workspace. That is the code `bun repo cargo` itself runs (root `🏗️build/📜️script.ts:128-130`), invoked in-process under the build task's own invocation (precedent: `🏭️inventory`). Output is captured (16 MiB / 65536 lines), the executable path is read from Cargo's `compiler-artifact` JSON, and the copy plus receipt are written inside the launcher's `consume` callback while the store is still retained.
3. Audit finding 2 (raw `cargo` spawn at `🏗️native-build/🟦️.ts:854,860`): that is the `buildCargoArtifacts` path, which spawns `cargo` with the store environment already issued by `withRepositoryCompilerStoreV1`; custody holds (the root `.cargo/config.toml` rustc ingress still gates it), but it is not the `bun repo cargo --store=shared` route the brief demands. The new product build does not use it. The old in-process `buildProctor` stays only behind the transitional switch below and is deleted in the cleanup step.
4. One shared resolver, `acquireProctor(repoRoot, invocation, env, signal, requestedMode, {rebuild})` in `🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts`, is used by dev, check, test-e2e, docker-image-build (course), the proctor operator verbs (`runProctor`, `runDevelopmentProctor`) and the capacity gate. It reads the receipt for the original build mode, verifies platform, executable existence, size and SHA-256, and returns a `ProctorArtifact` (same interface as before: `launch` runs a private copy, `withExecutable` streams verified bytes, `close`). A missing or stale product throws `ProctorProductError` with one English line then one German line naming the command to run (`bun repo [--tool=bazel] run @teaching/proctor:build [--configuration=release]`); nothing is rebuilt. Only the dev stack's rebuild-on-Rust-edit passes `{rebuild:true}`, which runs `buildProctorProduct` explicitly.
5. Transitional switch (freeze): `PROCTOR_ACQUISITION_DEFAULT = "build"` keeps the current Nx route working (in-process `buildProctor`) until the staged manifests declare `dependsOn: @teaching/proctor:build`. `SEMIO_PROCTOR_ACQUISITION=product` selects the product path per run. The staged `apply.py` flips the default to `"product"`; the cleanup step then deletes the `build` branch and `buildProctor`.
6. Inputs (audit finding 1): the manifest cannot read the closure, so `proctorSourceGlobs(roots, data)` derives the globs from `proctorSourceDirectories` (21 owner roots from the Cargo manifests, dev edges excluded): per root `**/*.rs` and `**/Cargo.toml`, plus `🎓️teaching/Cargo.toml|Cargo.lock`, plus the exact data files Cargo reads (`🧾️product/📥️inputs/🔣️.json`: value schema `.ts/.graphql/.proto/.json`, retained-clone wire schemas), minus `target`, `dist`, `node_modules`. The staged manifest stores them as the named input `proctorRust`. The build fails closed (`uncovered`, EN/DE) when Cargo's own dep-info names an in-repo source that the declared inputs do not cover, so a stale cache hit cannot arise from an undeclared source.
7. Cache key: `sharedGlobals` (already in `cargoSources`) carries platform+arch, bun version and `SEMIO_BUILD_MODE`, so mode and platform are in the key. Outputs: both mode directories (only the built one exists in a run).
8. `check` (proctor and course): staged to depend on `@teaching/proctor:build` and to read `{ "dependentTasksOutputFiles": "**/receipt.json", "transitive": false }`; the receipt carries the executable SHA-256, so a changed framework Rust source changes the check key through the product, and the whole-repo proctor glob and root Cargo globs go away.

## Files (live)

New:
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts` (module), `🧬️schema/🔣️.json`, `💬️messages/🔣️.json` (EN/DE), `📥️inputs/🔣️.json` (data files), `🧪️tests/🧫️fixtures/🔣️.json` (language-agnostic fixture)
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts` (laws; oracles Ajv, @noble/hashes, picomatch)

Edited:
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`: removed `proctorBuildLaunch`/`REPOSITORY_BUILD_ENTRY` (dead twin of the launcher route) and `runProctor`/`runDevelopmentProctor` (moved to the product module so they use the resolver)
- `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts`: `build` = `buildProctorProduct`; imports
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts`, `🚀️deploy/🐳️docker-image/🟦️.ts`, `🎭️e2e/🟦️.ts`, `🧱️stack/🟦️.ts`, `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts`: `buildProctor` -> `acquireProctor`
- `🧪️tests/🧱️local-stack/🟦️.ts`: the launcher-route law now tests `proctorCargoArguments` against the launcher's own parser
- `🧪️tests/🎚️config/🟦️.ts`: includes the new test file

## Staged (frozen files)

`T/x-proctor-tools/staged/`: `apply.py` (idempotent, structure-anchored; `--root`, `--check`), `inputs.ts` (derives the globs from the Cargo manifests; apply.py calls it, nothing hand-listed), `check.sh` (applies twice on a scratch copy, asserts the second run is a no-op, parses the JSON, runs the manifest laws against the copy; `--live` also asserts the checkout is in the target state).
What `apply.py` changes:
- proctor `📋️project.json`: new named input `proctorRust` (53 entries: 21 owner roots x (`**/*.rs`, `**/Cargo.toml`), `🎓️teaching/Cargo.toml|Cargo.lock`, 6 exact data files, 3 exclusions); `cargoSources` = cargo, proctorRust, sharedGlobals, script, bootstrap `*.ts`/`*.json` (`^cargo` and the root `Cargo.lock` are gone: the teaching workspace has its own lock); `build.outputs` = `dist/build/prototype`, `dist/build/release`; `dev, check, rebuild, health, backup, restore, erase, prune, capacity` get `dependsOn: ["build"]`; `check.inputs` = script, sharedGlobals, `{dependentTasksOutputFiles:"**/receipt.json",transitive:false}`, the test env vars, the course tree, `PROCTOR_COURSE`.
- course `📋️project.json`: `dev, check, test-e2e, docker-image-build, deploy-check` get `dependsOn @teaching/proctor:build`; `check.inputs` loses the whole-repo proctor glob and the four root Cargo/toolchain globs and gains the receipt input (cached `check` now keys on the exact proctor bytes via the receipt SHA-256).
- `🧾️product/🟦️.ts`: `PROCTOR_ACQUISITION_DEFAULT` -> `"product"`.

## Tests and results

Language-agnostic fixture `🧾️product/🧪️tests/🧫️fixtures/🔣️.json` (receipts, dep-info texts, Cargo JSON, globs, resolution cases) + schema `🧬️schema/🔣️.json`; laws in `🧪️tests/🧾️proctor-product/🟦️.ts` (registered in the course vitest config). Written first: the first run failed with `Cannot find module …/🧾️product/🟦️.ts`.
- `bun test ./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts`: 26 pass, 3 skip (the 3 manifest laws skip until the manifest declares `proctorRust`).
- `bash x-proctor-tools/staged/check.sh`: OK; same file against the applied scratch copy: 29 pass, 0 skip (closure equals declared inputs; build outputs; dependsOn of 8 verbs; receipt input of check).
- Oracles: Ajv (receipt, messages, inputs schema vs the repo validator), `@noble/hashes` (SHA-256 of the executable), `picomatch` (declared globs vs covered/uncovered files), the launcher's own `readRepositoryCargoLaunchV1` (argv admission). Opt-in live oracle, active once a product exists and the manifest is applied: every source in the real `receipt.sources` (taken from Cargo's dep-info) must be covered by the declared inputs.
- Existing suites: `🏗️proctor-build` 20 pass (SEMIO_TICKET_DIR set); `🧱️local-stack` 40 pass, 2 fail (`a dev server whose proctor goes away while it proxies a socket`, Vite/Bun socket proxy tests, no code of mine in that path); `tsc -p tsconfig.json --noEmit` of the course project: no error in any file touched here (one unrelated error in `🏗️build/📥️caller/📤️child/🟦️.ts` by the peer).

## Measured times

Cold and warm build: NOT RUN. Three attempts (15:45-15:58) died at `bun repo` boot on infrastructure faults that are not in this change (orphan persistent child; then link count 2 on boot sources caused by the x-inputs hardlink mirror, three times at different files, the third ~124 s in; then the 16:03 kernel panic). Since "bun repo boots" (16:28) the tree has been quiet-only, so the single permitted cold run (`NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run @teaching/proctor:build`, foreground, timeout 600000, log to `🗑️generated/x-proctor/`) waits for the peer's GO. Expectations from the existing store: a previous proctor (37.8 MB) and units of the `teaching-proctor*` crates exist in the shared build dir, the store has about 4 GiB free, ingress is about 27 s per launcher call.

## Apply order and verification

1. (any time, already live) code and tests above; with `PROCTOR_ACQUISITION_DEFAULT = "build"` the Nx route is unchanged for course targets. `bun repo run @teaching/proctor:build` already produces the product (receipt) with the live manifest, just uncached.
2. When the freeze lifts: `python3 T/x-proctor-tools/staged/apply.py` (manifests + flips the switch), then `bash T/x-proctor-tools/staged/check.sh --live`.
3. `bun repo run @teaching/proctor:build`, then `bun test ./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts` (the dep-info coverage law now runs on the real receipt; it names the first undeclared source, or add the file to `🧾️product/📥️inputs/🔣️.json` and re-run apply.py).
4. Verify: `bun repo --tool=bazel run @teaching/proctor:build` twice (second is a hit), edit a file under `🧰️framework/🔨️modules/⏳️async/` and see a miss; `bun repo --tool=bazel run @teaching/architecture-course:check` consumes the receipt.
5. Cleanup (after round 2): delete `buildProctor`, `PROCTOR_ACQUISITION_*` and the `"build"` branch of `acquireProctor`.

## Pending after GO (x-integrate or this executor)

1. `python3 T/x-proctor-tools/staged/apply.py` then `bash T/x-proctor-tools/staged/check.sh --live` (check.sh was re-run after the reboot: OK, 29 pass on the applied copy).
2. The one cold build, then the opt-in dep-info coverage law (reads the real `receipt.sources`; adjust `📥️inputs` if it names an undeclared file; the build itself already fails closed with the EN/DE `uncovered` error). Record cold/warm seconds here.
3. Bazel proof: `bun repo --tool=bazel run @teaching/proctor:build` twice (hit), an edit under `🧰️framework/🔨️modules/⏳️async/` (miss), then `@teaching/architecture-course:check` consuming the receipt.
4. Cleanup step in the final series (delete `buildProctor`, `PROCTOR_ACQUISITION_*`).
Known unknown to verify on that run: relative dep-info paths are resolved against `🎓️teaching` (matches the earlier real `.d` files: members relative, path dependencies absolute); `filenames` of the new-layout build directory give the `.d` beside the artifact (confirmed on existing `build/<pkg>/<hash>/out/`).

## Not done / open

- Cold and warm timings, and the data-file list verified against a real dep-info (the six files in `📥️inputs` come from existing `.d` files of earlier builds; the coverage check fails closed if the first real build finds more).
- `test` inputs: the dev-closure of the three teaching members adds no further in-repo crate (checked), so `cargoTests` is covered by the same `proctorRust`.
- `--proctor <relative path>` of test-e2e still resolves against the package directory (audit P2-7), not touched.

## Backend edits

None. No generic backend, manifest or freeze-listed file was edited.
