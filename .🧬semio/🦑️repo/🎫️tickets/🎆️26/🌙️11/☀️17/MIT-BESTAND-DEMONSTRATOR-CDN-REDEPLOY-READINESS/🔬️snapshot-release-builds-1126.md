# Immutable source snapshots for release builds (2026-11-26)

## What already existed

- `🔨️modules/📁️filesystem/🧾️observation/🔁️churn/📸️snapshot`: `observeStableTreeV1`, an observation primitive (consume each file once, two listings must agree, settle window, retry by churn policy). It never materialised anything.
- `SEMIO_MACHINE_STATE_DIRECTORY` (`🏃️process/🏗️build/🏠️machine-state`): relocates `.cache` independent of the checkout; the checkout and the machine state are already separable.
- `codeRoot` vs `repositoryRoot` in the native caller authority.
- `CargoSourceWitnessV1` (native-build `🧾️receipt/📎️source-witness`): bounded content store of admitted bodies, used only to attribute a consumed checksum; Cargo and rustc still read the live tree. `🔬️descriptor-immutable-source.md` already listed the missing mirror.
- Failure sources confirmed in code: `🧩️component-pass/🔁️failure-churn` (lock changed -> ConcurrentEdit "build"), `📥️source/👁️drift` (watcher compares live files to admitted digests, kills the child), `⚙️configuration` (`requireLoadedCargoProfilePolicyV1`, "changed since load; restart required", exit 75).
- Provenance: `sourceTreeDigest` = digest of `(path relative to workspace root, sha256)` rows of consumed sources. It is root-independent, so a relocated identical tree gives the same value.

## What I built (new, tested)

`🧰️framework/🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/` (schema-first `🧬️schema/🔣️.json`, EN/DE texts):

- `captureCheckoutSnapshotV1`: clones (COW via `COPYFILE_FICLONE`, plain copy elsewhere) and hashes every file once under `observeStableTreeV1`, so the result is one stable tree state; names `<state>/s/<12 hex of digest>` (short root, max 120 chars, refused otherwise); files read-only; `node_modules` kept as links; `.git`, `.cache`, `.nx`, tickets, `target`, `🗑️generated` excluded; unchanged tree reuses the directory; concurrent captures race by atomic rename; progress, cancellation, stray-file sweep after retried observations.
- `checkoutRowsTextV1` / `checkoutRowsDigestV1`: canonical language-agnostic digest (UTF-8 byte order of paths).
- `verifyCheckoutSnapshotV1`, `checkoutSnapshotProvesV1` (proves a receipt's `[path, sha256]` consumed sources equal the snapshot rows), `pruneCheckoutSnapshotsV1`, `checkoutSnapshotEnvironmentV1`, `checkoutSnapshotLiveTextV1` (maps snapshot paths in diagnostics back to live paths).
- CLI: `repo 🧰️framework/🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/📜️script.ts capture|verify ROOT|prune`.

## Tests (9, all pass via `repo test-file`)

Fixture `🧪️tests/🧫️fixtures/🔣️.json` (tree plus expected rows/digest/id, produced by Python hashlib). Cross-checks: independent Python derivation from the on-disk snapshot; real `cargo` as oracle. Key test: a tiny crate (build.rs, `mod`, `include_str!`) is built by real cargo inside the snapshot while a build-script handshake lets the test rewrite the live sources mid-build; the binary equals the pre-edit oracle, all consumed dep-info sources (4) are proved equal to snapshot rows, snapshot verifies unchanged. Also: edit/delete during capture, cancellation, root length, pruning, links/environment, EN/DE.

Real-tree measurement: 170,175 files, 1.79 GiB, 80 s, 2 observations (one live edit absorbed), verify 74 s. Snapshot removed afterwards.

## Wiring (done, 2026-11-26 second pass): `sourceRoot` separate from `repositoryRoot`

Whole-pipeline re-rooting is impossible (the Go launcher binds dependency generations to one root), so the process stays on the live root and only the files Cargo and rustc read move.

- Authority: optional `sourceRoot` in the three caller schemas (`invocation/🧬️schema/🔣️.json`, `📥️native-caller.json`, `🏗️build/🧬️schema/📥️caller.json`); `readNativeCallerAuthorityV1` returns `sourceRoot` (default `repositoryRoot`).
- `📚️library/🏃️process/🦀️cargo/📥️dependencies`: `RepositoryCargoInputsV1.sourceRoot`; the `.cargo/config.toml` read and lock handling use it; a lockfile inside a snapshot is never refreshed in place (stale lock raises a ConcurrentEdit `build`, which takes a new snapshot).
- `⚙️configuration` (Cargo config/profile policy/workspace selection): `sourceRootOfV1(request)` reads the snapshot only for manifests inside it; host tools (scheduler) stay on the live tree. This fixed the regression `Cargo workspace path escapes repository` (half-threaded first version).
- `🧩️component-pass` driver: `cwd` is the source root, new `originCwd` is the live root (scheduler only); provenance roots follow `cwd`, so receipts name workspace-relative paths and `sourceTreeDigest` is the digest of what was compiled.
- `driver-stages`: `runRepositoryComponentDriverV1` is now a wrapper. Release builds capture a checkout snapshot by default (no flag), unless the authority already names a `sourceRoot`; pass manifests are rebased, stage targets map back to live paths (committed descriptor pairs, publication, caches, leases stay live). Wrapped in the `build` churn retry. Prototype builds are unchanged.
- Pruning: `retained` 3, minimum age 5 h (a build lasts at most 4 h), total byte budget 8 GiB, run after every capture; snapshots are COW clones where the filesystem allows.

Tests: snapshot module 9/9; `🧩️component-pass/🧪️tests/📸️snapshot` (real driver, live source rewritten while Cargo runs, linked wasm contains the snapshot marker) passed in the `native-component-real` suite (registered there); `⚙️configuration/🧪️tests/🧭️source-root` 1/1; other agent's `hold-and-wait` 5/5; configuration `steps` and `profile-recheck` pass.

Validation run: `JOBS=1 native-queue.sh flow-extension-math` (snapshot is the default). Attempts so far: snapshot captured by the real pipeline (170,19x files, 1.79 GiB, 50-68 s); attempt 1 hit the `escapes repository` bug (fixed), later attempts were delayed by (a) the descriptor emitter rebuild on the live tree (emitter is a live job, stale on every attempt because the tree changes), (b) a broken live compile of `semio-framework-manifest-arg-schema` mid-edit by another agent, (c) the exclusive product lane held by other queue items. The run is still queued; its log is `🗑️generated/browser-flow-extension-math.log`, queue log `🗑️generated/snapshot-validation-queue.log`, receipt lands in `🗑️generated/confirmed/flow-extension-math.json`.

## Host and ancillary tools (third pass)

Every Rust compile of a release `components describe` run now reads the snapshot:

- `withRepositoryCargoArtifactPlanV1` (all artifact producers) takes `sourceRoot`: policy cwd and config, manifest, lane crates and `repositoryCargoCompilerInputsV1` follow it; owner preparation is skipped inside a snapshot; the root is validated by `requireCheckoutSnapshotRootV1`. The provider request type `CargoArtifactBuildRequestV1` carries it.
- Descriptor emitter: `buildDescriptorEmitterV1` and `ensureFreshDescriptorEmitterV1` take `sourceRoot`; freshness (`🌡️freshness`) maps recorded absolute paths (live origin or any earlier snapshot) onto the current source root (`descriptorEmitterRecordedPathV1`), so an emitter built from one generation is fresh exactly when the files of the next generation are equal, never falsely fresh.
- Native transpiler: `ensureNativeTranspileV1(..., sourceRoot)`.
- Scheduler crate: `nativeBuildSchedulerSelectionV1` rebases its manifest/`--manifest-path` onto the source root.
- Recipe inputs: the ancestor walk of `cargoArtifactTargetRecipeV1` stops at a snapshot root, otherwise the live repository root (an ancestor of `.cache/s/<id>`) leaked its constantly edited `Cargo.toml`/`Cargo.lock` into the recipe and caused endless ConcurrentEdit retries.

Tests: freshness 2/2, scheduler selection 1/1, `checkoutSnapshotRelativeV1`; typecheck clean for all touched files (one unrelated error in another agent's `reclaim.test.ts`).

Open: the flow-extension-math validation run is queued behind host-capacity/product-lane custody held by the other queue (`sourcing`); no receipt yet, so `checkoutSnapshotProvesV1` has not been run on it.

## Disk safety and emitter receipts (fourth pass)

- Retention (`pruneCheckoutSnapshotsV1`): the age rule is gone. A snapshot is protected only by a live holder (`<state>/s/.holders/<id>/<pid>`, written by every capture and step reuse; dead pids are swept) or by `keep`. Unprotected snapshots go oldest first while there are more than 3, their recorded bytes exceed 8 GiB, or free space is below 20 GiB (`minimumFreeBytes`). Dead captures' stage dirs (`.t<pid>-...`) are removed. Pruning runs before and after every capture.
- Clone: measured on this host, Bun `copyFile(COPYFILE_FICLONE)` clones on APFS (a 800 MB file costs ~0 blocks; a whole capture ~0.44 GB of real disk for 1.8 GB apparent, which is per-file metadata, not data). The test `capturing a large file does not grow the disk where the filesystem clones` asserts < 64 MiB growth for a 256 MiB file (skipped where `COPYFILE_FICLONE_FORCE` is unsupported). Divergence from live edits and leaked stage dirs of killed captures were the larger costs; both are bounded now.
- Closure: capture defaults to the Rust-bearing roots (`closureRootEntries`: framework, hub, plugins, Cargo/toolchain/bun config; `node_modules` stay links). Per-file filtering was rejected: `include_str!`/build scripts read arbitrary files.
- Retry reuse: `SEMIO_SOURCE_SNAPSHOT_STEP` (set by `native-queue.sh` per step, forgotten when the step returns) makes the first capture of a process reuse the step's snapshot; in-process `build` retries capture fresh.
- Emitter receipts: `readCompletedCargoRetainedIndexV1` accepts an origin that is a snapshot of the workspace (`checkoutSnapshotOfV1`); `withDescriptorEmitterTaskV1` compares the recorded manifest after rebasing snapshot paths; `descriptorEmitterRecordedPathV1` checks the snapshot form BEFORE the inside-root test (a snapshot lies below `<root>/.cache`, so the old order silently treated its files as non-sources).
- Evidence: the flow-extension-math run compiled and published the descriptor emitter from the snapshot (579 units, package ids `path+file:///…/.cache/s/<id>/…`). The guest pass then raised a build-layer ConcurrentEdit naming the snapshot's `🧮️math/Cargo.toml` (hash unchanged); cause not yet found.

## Cache reuse across roots and the retry (fifth pass)

- Unit-cache keys were already portable (`portableKeyV1` tokenises the cwd and the workspace root), but the rustc wrapper tokenised only the LIVE workspace (`SEMIO_UNIT_CACHE_WORKSPACE_ROOT`), so every path below `<state>/s/<id>` stayed absolute and keyed each snapshot separately. The wrapper now takes the snapshot root of its cwd/manifest dir (`checkoutSnapshotRootOfV1`, `🧭️place`, no file access) as the workspace root of the portable roots; the live and snapshot builds of an unchanged crate share one key. Test `🧪️tests/📸️snapshot-roots`: equal keys in two snapshots and live; with one changed crate only it and the crates consuming its artifact change. Fingerprints/target dirs: all snapshots share the lane build dir; a cargo fingerprint miss only calls the wrapper, which restores from the unit cache.
- Measured: after the key fix the emitter build compiled 180 of 595 units from a new snapshot (415 restored), against 579 of 579 before. Entries written before the fix are keyed per snapshot and miss once.
- The retry that named the first snapshot: the in-process retry cannot work. After the first failure the Original admission is CLOSED and every later operation under the same control rejects with that first failure ("Original process admission closed - first failure: ..."), so attempts 2 and 3 re-threw attempt 1's error, path included, while each also captured another 1.7 GB snapshot. The in-process retry is removed; a source-related failure (ConcurrentEdit, lockfile) forgets the step snapshot and the QUEUE's next process captures anew.
- The failure itself was a live `✏️s` lockfile that did not match its manifests: inside a snapshot the lock is never refreshed in place, so the stale-lock branch now refreshes the LIVE lockfile (`ensureRepositoryCargoLockV1`, path mapped back from the snapshot) and then fails, so the next capture carries a current lock.
- `--config` storage arguments are rebuilt per request by manifest (snapshot or live), fixing `Cargo configuration belongs to original repository admission` for requests whose manifest is outside the snapshot.
- Open: the validation run waits for compiler image headroom (14.4 GiB expected, 16.4 GiB free, 9.1 GiB held by a running command); no receipt yet.
