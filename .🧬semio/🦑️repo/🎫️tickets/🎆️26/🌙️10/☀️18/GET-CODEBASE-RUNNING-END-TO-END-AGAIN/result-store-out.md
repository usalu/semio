# result-store-out: restorable receipts for store-published outputs (2026-10-19)

Executor `store-out`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`. No cargo, no `bun repo`, no activation.

## 1. Design

Problem: deterministic targets publish into leased content-addressed stores (compiler store `published/<hash>/<mode>`, products store: Vite `out`, wasm slots, TS artifact output) so their declared `outputs` were empty, bogus (`{projectRoot}/dist/build`, never written) or pointed into the store. A cache hit then reported success while the artifact could be missing.

Protocol (schema-first, tool-neutral, `B/🧾️store-receipt/`):
- **Receipt** = the only restorable output: `{projectRoot}/.🧾️store/{target}.json` (schema `🧬️schema/🔣️.json`; deliberately NOT under `dist`, which is a symlink into the store for TS artifacts). It names `entries` (workspace-relative store path, kind file|directory, bytes, files, sha256; directory digest = sha256 of sorted `path\nsize\nfileDigest\n` lines) and `links` (symlink path -> target, used by the TS artifact `dist` / `📤️dist/<mode>` bindings). Git-ignored via `**/.🧾️store/`.
- **Handler writes it** after publication, from the central publishers (no per-script edits): `withRepositoryArtifactDestinationV1` (`L/⚡️caching/📦️artifacts/🟦️.ts`: Vite sites, play pages, fonts, wasm packages, wgpu wasm, coordinator, dotnet), `buildRepositoryCargoArtifacts` (`L/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`: the compiler store `published` directory of every direct Cargo `build`, including the 105 role:artifact crates), `bindTypeScriptPackageArtifactV1` (TS artifact output + both links). Writes merge by path under a cross-process lock, atomically.
- **Adapters issue the receipt**: nx executor (`B/../nx-plugin/📤️arguments/🟨️.mjs`, both executors) and Bazel `executeTask` call `issueStoreReceiptV1`: a task that declares the receipt output starts from no receipt and gets `SEMIO_TASK_RECEIPT`; every other task has it unset (a nested task never records into its parent's receipt). Handlers do nothing without it.
- **Miss on missing store**: `withStoreReceiptsV1` (`B/🧾️store-receipt/🟦️.ts`) wraps the nx and Bazel branches of `B/📜️script.ts` for `run`/`run-many`. After the run it verifies the receipt of every selected task (closure via `selectedBuildPrerequisitesV1`; digest mode): state valid | invalid (absent/kind/size/digest/link) | unwritten | malformed. Anything but valid is treated as a miss: that one task re-executes alone and uncached (nx: `run <task> --skip-nx-cache --excludeTaskDependencies`, which also replaces the cache entry so the next hit is good; Bazel: `skipCache` + `excludeDependencies` request, in-process), then is re-verified, else the invocation fails closed. A failed first run is retried once after repairing. A task that declares a receipt but whose handler wrote none is an error (contract), not silently accepted.
- Nx admission: the repair launch is the single extra child selection `isStoreRepairSelectionV1` admits in `B/📥️caller/🟦️.ts` (exact `run <project:target[:cfg]> --skip-nx-cache --excludeTaskDependencies [-- args]`).

## 2. Applied

- 129 manifests (cargo-direct `build`, `fonts`, wgpu `wasm`, coordinator, dotnet, praesentation, hub-admin, demonstrator `build`, play `build`) + 79 TS artifact `build` + 9 wasm-slot `wasm` = 217 targets now declare the receipt as `outputs` (codemod `C/store-out-tools/apply.ts`, `apply2.ts`; span edits, JSON re-parsed). Cache flags unchanged in manifests. Pins updated together: demonstrator runtime test, cache-contracts dotnet, package-inventory (`storeReceiptOutputV1("build")`), wasm-wrapper test.
- Inference plugin (`B/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`): TS artifact `build` and wasm-slot producers no longer forced off with store-link/slot outputs; they get the receipt output and decision `deterministic/pure-function-of-inputs` when authored cacheable (else `managed-output`). `targetBuildModes` now adds the `SEMIO_BUILD_MODE` input to the cacheable artifact build. Plugin-cache corpus + test updated.
- Not receipted on purpose: `repo-*-rs` and other `rustBuildIntent` builds (the selected graph replaces `build` by a noop aggregate over per-unit `compile` tasks with real outputs), `mcp-rs:build-release` (custom project-dir `output`, binary-gate pins `dist/build-release`), `publish` targets (real `dist/publish`), course `build`/`publish` (pinned `cache:false` by the deploy test), `check`/`test`/`lint` (verdict only, nothing downstream reads store content).
- Playground `prepare-*` and `build-*-react*` stay `driver-owned`/uncached by decision: they are orchestrators (`metadata.semio.execution.kind = orchestration`; the release-pipeline contract requires them uncached) whose real work runs in child tasks that are cached individually (`activate-*-react` already keys off prepared bytes). Nested tasks deliberately do not inherit the parent's receipt, so an orchestrator cannot aggregate one; flipping them would skip the children on a hit. The receipt+restore protocol covers the cacheable children instead.

## 3. Verification

| Command | Result |
|---|---|
| `bun test ./B/🧾️store-receipt/🧪️tests/🟦️.ts` (fixture-driven: digests vs system `shasum`, schema vs Ajv strict + runtime guard, 14 verification + 5 link cases, 10 runner scenarios, declaration rule, repair admission, lock/merge, repo law: every declared receipt belongs to its own target and to a handler reaching a recording publisher, 217 targets) | 14 pass / 0 fail |
| `bun C/bzl-act-tools/wrap.ts ./B/🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts` | 8 pass |
| same wrapper: `L/🧪️tests/🧱️wasm-package-wrappers`, `L/⚡️caching/🧪️tests/⚡️cache-contracts` | 8 pass, 1 pass |
| `bun test ./B/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts` | 9 pass, 2 skip |
| `bun build --target=bun` of `B/📜️script.ts`, native-build, binding, artifacts, store-receipt | ok |
| NOT run | any nx/Bazel task, real publication (cargo forbidden), nx-plugin arguments suite (needs `bun repo` storage env: "Original bounded Nx namespace required"), `B/🧪️tests/🕸️graph` (fails at import: `nativeLinuxLoopMountV1` removed by xplat-linux), demonstrator runtime test (`41 !== 35` baseline) |

## 4. Open items

1. Runtime proof: first `bun repo run @teaching/proctor:build` twice, then delete the compiler `published/<hash>` slot and run again (expect one `[store-receipt] re-executing` line). Also `--tool=bazel`.
2. Bazel: the repair run is uncached, so the stale disk-cache entry is not overwritten and a later hit repairs again (correct, slower); healing needs the adapter to upload while skipping reads.
3. Play `build` records every publication of its pipeline children (they inherit its receipt, up to 4096 entries); first run should confirm the volume.
4. Hook failures fail the task by design (a receipt that cannot be written is a broken contract). Symlinks inside published directories are refused.
5. Course `build` stays pinned uncached; its Vite store output can adopt the receipt together with the deploy-test pin.
6. Modes alternate in one receipt file per target (prototype vs release); alternating runs re-execute once per switch.
