# Blockers on `activate-prototype`

Logs: `🗑️generated/activate-5.log` to `activate-10.log` (activate-10 is the first run that reaches task execution).

## 1. `Original SDK binary link group required` (activate-4, activate-5)

- Symptom: run dies about 80 s in, right after "[nx] Validating current owner custody", in the nx tooling provisioning.
- Root cause (state): in `.cache/b/t/m/sdk/e921066e...` all 40 `.wasm`/`.node` names had `nlink=1` and an inode different from their sealed object in `.cache/b/t/m/o/<sha256>`. Content was identical (same sha256), but the hard-link group had been severed on Nov 12 at about 17:16 (only ctimes of binaries and objects changed; regular files were untouched), so something rewrote the files with a new inode and preserved mtime.
- Root cause (code): `ensureNxPristineSdkV1` treated a present pristine SDK as verify-or-die. A severed link group could never be healed, so a host in this state was stuck forever.
- Fix: `repairNxSdkBinaryLinksV1` (new, `...tools/📦️sdk/🧊️binary/📥️publication/🟦️.ts`) rebinds a name to its sealed object only when the bytes equal the sealed manifest (sha256 and size), through the existing atomic link+rename path of `publishNxSdkBinaryV1` (new binding `repair`, restricted to `sdk/` and requiring the expected manifest row). `ensureNxPristineSdkV1` (`...📦️sdk/📥️source/🧊️pristine/🟦️.ts`) runs it before verification. Byte differences are still refused.
- Test: `proveNxSdkBinaryPublicationV1` (`...🧊️binary/🧪️tests/📥️runtime/🟦️.ts`) severs a link group, repairs it, checks idempotence and checks refusal of changed bytes. It needs the repo test harness; not run here.
- Verified: after the fix the sdk files have `nlink=3` and share the object inode, and the run continues.

## 2. `NX namedInputs definitions cannot start with ^` (activate-5)

- Root cause: `♻️mit-bestand/🧺️demonstrator/📋️project.json` defined `namedInputs.activationSources` containing `"^production"`. Nx forbids `^` references inside a named input definition; it is only valid in a target `inputs` list.
- Fix: removed `"^production"` from `activationSources` and added `"^production"` after `"activationSources"` in the `inputs` of the three targets that use it (prepare-prototype, prepare-release, activate-prototype). Semantics (hash the dependencies' production inputs) are preserved.

## 3. `Unauthorized` after "Successfully ran ... 80 skipped" (activate-6)

- Root cause: not a code defect. The invoking shell exports `SEMIO_REMOTE_CACHE_ENDPOINT=http://192.168.178.70:6090` (+ `SEMIO_REMOTE_CACHE_ACCESS=read-write`) but no `SEMIO_REMOTE_CACHE_TOKEN`. The fleet cache answers 401 `www-authenticate: Bearer` (confirmed with curl) and Nx aborts the whole run on the first 401, skipping all tasks.
- Handling: runs 7 to 10 unset the three `SEMIO_REMOTE_CACHE_*` variables (remote cache off). A host that wants the fleet cache must export `SEMIO_REMOTE_CACHE_TOKEN`. I did not invent a token.

## 4. `Funded Nx graph cache capacity exhausted` (activate-7)

- Symptom: `nx:graph-publication` waited on `nx:graph-capacity` for the full `capacityWaitMilliseconds` (900 s) and then failed.
- Cause: the graph pool image is 1.5 GiB with `maximumPoolBytes` 896 MiB and a namespace is about 450 MB (project-graph 347 MB). Two namespaces never fit, so the previous namespace (`3b6c.../c838...`, created by the previous run) has to be evicted before a new one can be published. The previous namespace was still held by the process of the waiting run and by leftover processes of the killed run, so eviction was refused until the holders were gone.
- State: with no holders left (run 10) the eviction succeeded at once (instrumented with temporary `[DEBUG]` lines in `...💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`, since removed; the file is byte-identical to HEAD).
- Open: the self-deadlock risk (a run holding a shared lease on the namespace it needs to evict) is not fixed in code; the pool is sized for about one namespace of the current size.

## 5. Stale dependency generation receipt (`provider changed ... node_modules/@semio-tech/course`)

Harmless. It is an informational `[DATA]` line emitted while the receipt is rechecked; the run re-verifies and continues (it appears in every run, also those that succeed past it). `node_modules/@semio-tech/course` is a symlink to the workspace package `🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript`, whose directory identity changes whenever another agent edits that folder in the shared worktree. The first line (`env:BUN_INSTALL_CACHE_DIR` configuration change) is also expected after the tooling store changed.

## 6. `E0599 has_failure` in `semio-plugin-decl-app` (activate-10)

- Root cause: committed source defect. `OriginalConstructionAdmission::has_failure` was renamed to `failure_untaken` in `🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs`; every other caller was updated, `...🔌️plugin/📜️declaration/🧩️app/🛬️lifted/🎬️media-export/🦀️.rs` was not (HEAD has the same mismatch). It broke `semio-plugin-decl-app` and with it every plugin component crate and the puzzle wasm-pack build.
- Fix: both call sites in media-export now use `failure_untaken()`.
- Verified: in activate-12 the plugin component builds report 0 failed units (about 70 s each with warm caches).

## 7. `E0308 expected &mut dyn NativeSendAllocationLoan, found StageAllocation` in `semio-s-artifact-stdio-svg-snapshot` (activate-12)

- Root cause: committed source defect (API drift). `SqliteSnapshotControl::allocation_stage` now hands the closure a `StageAllocation`; the csv, tsv and print sibling crates call `allocation.encode(..)` / `allocation.decode(..)`, the SVG snapshot crate still called `NativeEncodeControl::new_forwarded(.., allocation)` / `NativeDecodeControl::new_forwarded(..)`.
- Fix: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` uses `allocation.encode(remaining,&mut callback)` (twice) and `allocation.decode(remaining,&mut observer)`. Not yet compiled: the run that would have shown it was stopped (see Status).

## 8. `Original mutable compiler input changed` (activate-12)

Not a defect to fix in the repo. Other agents edit sources in the shared worktree while a multi-hour component build runs, and the native build refuses a unit whose mutable input changed between capture and use (`semio-hub-cad-component describe/model/browser/complete`). Such a refusal repeats until a build window without foreign edits to the compiled crates; rerunning is the remedy.

## 9. Cold graph owner phase after a service restart (activate-12, activate-13)

After `repo service host stop/start` the next plan re-reads all 1746-1916 owners at about 0.15 owner/s (2 to 3 hours; activate-12: 7938 s). Avoid restarting the service; it was needed only because activate-11 hit `The graph service could not answer (timeout)`. Each later source edit in the framework also made the next plan repeat the owner phase (activate-13 was at 51 of 1916 owners after 5 minutes when stopped).

## Status

`activate-prototype` has NOT exited 0. Last full attempt: `🗑️generated/activate-12.log` (ran about 5.8 h, got past the SDK, namedInputs, remote-cache, graph-capacity and plugin-decl-app blockers and was compiling the plugin components and the svg snapshot crate when blocker 7 and blocker 8 showed up; stopped by me). `activate-13.log` was started with all fixes and stopped during the cold owner phase to free the 16 GB host. Rerun with:

`env -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE repo run @semio-tech/mit-bestand-demonstrator:activate-prototype`

## 10. `framework-surface-rs:wasm` ENOENT on `🕸️bindings/framework_surface.d.ts` (activate-12)

- Root cause: the wasm build itself succeeded. The compiler output is now an in-memory captured package (`ProducedWasmPackageV1`, files live in the compiler store), never written to `<rs>/🕸️bindings`. `WasmScript` in `🗺️surface/📦️packages/🦀️rust/📜️script.ts` still ran `🧪️tests/🕸️browser/🟦️.ts` afterwards, and that test read `../../📦️packages/🦀️rust/🕸️bindings` (no producer writes it any more). Editor and node-graph have no such post-step.
- Fix: the test now runs as the `consume` callback via `withProducedWasmPackageTaskV1`, receiving the captured `name -> path` map in env `SEMIO_SURFACE_WASM_PACKAGE`; the test resolves `framework_surface.d.ts/.js/_bg.wasm` from it. Files: `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🔨️modules/🗺️surface/🧪️tests/🕸️browser/🟦️.ts`. Not yet executed.

## 11. `channel-version-descriptors-check`: pin 25, 71 committed descriptors, 70 stale (activate-12)

- Source: `🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:29-35`, `staleChannelDescribeOutputsV1` in `../🟦️.ts:77`. A committed `*/🧩️component/🔣️.json` is stale when `executionProtocol.appChannelVersion !== pin` (`📌️pin/🔣️.json`, 25).
- Why 70 at once: the pin was bumped 24 to 25 (commit 724) without re-describing; 70 descriptors still say 24, only `🔌️plugin/🖥️host/🧪️testing/🧩️component` says 25. Descriptors are 1 MB generated artifacts of `workspace:components describe`, so hand-editing the number would fake them.
- Closure: the check ran inside the activate-12 plan (task `@semio-tech/framework-os-dev:channel-version-descriptors-check`), a dependency of `@semio-tech/s-dev-composition:generate` (`✏️s/🧑‍💻dev/📋️project.json:226`); it failed the nx run (exit 1), so it blocks activate unless that generate is not required. No source change made. After the run finishes: `sh 🩹patches/describe-stale-descriptors.sh` (70 `repo run workspace:components describe <id> --profile prototype`), then rerun activate.

## 12. `puzzle-plugin:wasm` failure (activate-12)

Only the E0308 in `semio-s-artifact-stdio-svg-snapshot` (three call sites, lines 32, 49, 54 of the svg `🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`); `wasm-pack` reported `cargo build failed`. Source now has no `new_forwarded` left, so item 7's fix covers all three.
