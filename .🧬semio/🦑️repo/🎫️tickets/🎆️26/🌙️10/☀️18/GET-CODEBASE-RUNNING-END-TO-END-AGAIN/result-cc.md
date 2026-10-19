# PROGRESS (cc, latest first)
- GREEN: framework lib 34, 35; root lib 63-67, 69, 71, 73, 75; root tests 64, 66; teaching 147-148; ui carve: 300 (20 framework crates incl. new ui-tui-*/ui-wgpu-* lib), cstep priority framework 330.
- RUNNING: 331 (cstep root: 10 crates, long compile).
- FAILING / BLOCKED: 301-317 (ui carve tests + consumers, not yet run), 68/76 (test-companion privacy, executor `tco`), 36 and 85, 149 (re-check after ui carve), 70/72/74.
- QUEUED (generated, `cc-tools/batches.json`): 302-317 ui consumers, 332-365 cstep plugin-workspace crates (--lib), 366 job `original-close-context` test, then bim (cg3 section 7), lfix §4/lfix2, 171 stale locks (law4), epw workspace, pdf law test, BIM_BLESS fixtures, result-lex/c6fin/c6inf/c6store/c6db/c6plug/fnsplit/plugfix/hdr3/glob/wgpu2 crate lists, 86-146 and 150-172 of the original plan.
- Infra fixes this round: lease image filled by macOS `.fseventsd` (64 MiB) -> `no_log` written at attach (`cleanup/🎛️command`) and manually on all mounted images; runner waits for foreign `bun repo cargo` before each launch.

# Result: cc (COMPILE critical-path executor)

## Phase 1: launcher (DONE, green)
`bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-geometry-primitives --lib --message-format=short` exits 0 (rustup toolchain nightly-2026-07-20 installed into the APFS tools image, crates downloaded, `Finished prototype` in 1.4 s). Note: a bare call without `--manifest-path` targets the ROOT workspace.

Root causes fixed (in order):
1. `rustup/📥️installation/🟦️.ts` `acquireArchive`: the cache probe used the tracked `hashRustupFileV1`; the expected ENOENT failed a tracked operation and `track()` (`invocation/process/🟦️.ts`) closes the whole invocation on ANY tracked failure, even a caught one, so the next download died with "Original process admission closed". Fix: untracked `lstat` first, hash only when present. (Observation, not changed: `settle()` rethrows the secondary "admission closed" error instead of `failures[0]`, which hides root causes.)
2. Stale legacy `.🧬semio/🦑️repo/⚡️cache/rust/fixed` (image.json without mountDirectory, fixed.dmg already deleted but still attached as /dev/disk4): detached and removed; the cleanup command then requires the parent `⚡️cache/rust` to exist (non-recursive mkdir). Zero-touch fix in `budget/📥️opening/🟦️.ts` `openRepositoryCachePartitionV1`: `mkdirSync(dirname(declaration.directory),{recursive:true})` before opening a partition (the `rust` partition lives outside the `.cache/b` ancestors the parents-schema materializes). Not exercised on a clean tree yet.
3. Tools image `.cache/b/t` was already APFS (coordinator): no ENOTSUP.
4. `wgpu/📦️packages/🦀️rust/Cargo.toml` path dep used `🧿️test-support`; the directory (and all 18 other manifests) use `🧯️test-support`. Fixed the one manifest.
5. Framework workspace "member of the wrong workspace": 8 recently carved crates in the framework workspace depend on root-workspace crates (os-kernel, plugin-*). Moved them to the root workspace (members entry moved from `🧰️framework/Cargo.toml` to `Cargo.toml`, `package.workspace` key re-pointed to the repo root): original-snapshot-owners, transient-preparation-law, typed-input-law (framework/🔨️modules/🧪️test), sqlite-snapshot testing/💰️backing, plugin tests surface-fixture, surface-carriers, app-typed-command-full-operation laws, plugin testing declared-verb-verdicts. `cc-tools/wscheck.mjs <ws>` (bun) lists members with path deps outside the workspace: 0 violators for framework now. The compile plan must treat these 8 as ROOT crates.
6. Reverted a temporary `[DEBUG]` log in `invocation/process/🟦️.ts` (file byte-identical to original).

## Phase 2
- Framework lock refreshed ONLINE: `update --workspace --manifest-path 🧰️framework/Cargo.toml` (22 packages added). `--offline` fails: `zip` (deflate dev-dependency) is not in the offline index cache. Lock backups: `🗑️generated/cc-*-Cargo.lock.before`.
- Root lock refreshed online (`update --workspace`); offline not possible (index cache).
- Launcher/tooling fixes found by batches: planning-capture cap 16 MiB -> shared `cargoPlanningOutputBytesV1` (128 MiB) in `native-build/🕸️units/📥️capture` used by launcher, relay, descriptor selection, process (root `cargo metadata` is 17.2 MB); regenerated stale `🗂️schema-document-index.json` (hand-patched entries pointed outside their roots; regenerated through `mutationSchemaDocumentIndexProjection`).
- Source gate: `semio-framework-ui` (all-features closure) is refused ("extent or boundary exceeded"); 257 of 549 root members depend on it, so every batch skips them (`cc-tools/refused.txt`, `🗑️generated/cc-bNNN.skipped`). Needs the ui carve (c6ui reports it still above 12,000 lines).

## Batches (cc-tools/run.mjs, `--parallelism=4`, `--keep-going`)
| idx | scope | result |
| --- | --- | --- |
| 34 | framework 1 (lib) | green |
| 35 | framework 2 (lib) | green |
| 36 | framework 3 | blocked: peer adds new framework members mid-run (3d-brep-construction-unit-tests `testkit` feature; `ui/tests/targets-wgpu-prepared-laws` manifest) |
| 63 | root 1 (lib) | green after the fixes below (2 crates skipped behind `semio-framework-ui`) |

Source fixes (63): dsl-record-derive `ContainerAttrs` pub(super); companions `extern crate` cfg(test) (32 files); neural model hash import; pack-format `EncodedSegment`/`DecodedSegment` pub(super); ui-contract-builder `BuiltChildRetire*` pub(super); ui-contract accessibility serde import + `is_default`; neural registry/model: `OperatorRetirement` + `RetireOwned for Box<dyn Operator>` + exported `original_retirement_cursor!` moved to the model crate (orphan rule); store-ephemeral missing imports + deps (io-control, os-store-read); store-replay `ReplayProgress` retire impl moved next to the type, `CursorRevisionRecord` fields pub; space-schema lifted snapshot trait import; engine `exhausted` pub; decode-pages `slot_address()`; `TryFrom<OpsSupersededInput> for SupersededInput` moved to os-store-command; `IoFidelityClass` inherent impl moved to its defining crate (codec-registry); `BindingKey` pub; envelope owns `DslField for OwnerRef` + helpers (new dsl-record dep); `StoreMintWitness` path; engine-core/live explicit lifetimes (9 methods); `MemberGroupPreparation` duplicate removed (single lifted type); `MemberHistoryDictionaryAdmissionError`, `MemberFactorySelectionRejected` pub; `CommandDecodeError` fault impl and `HistoryColumn` retire impl moved to their defining crates; durable-group `value_field!` impls moved to the wire crate; initialization genesis factory bounds; directed presentation uses `semio_framework_graph_dsl::Wire*`; flow-document os-command dep; flow-copy `FlowHostComponents` owner + demand conversion.

## Progress snapshot (second pass)
Runner: `cc-tools/run.mjs <idx>` / `cc-tools/rb.sh <idx>` (batches from `compile-plan.md`, `--parallelism=4`, auto `update --workspace` on stale lock, retry on transient `System image plist required` / `Bounded unique parent kernel roster required` / `topology changed`, drops `-p` crates whose closure contains a source-gate-refused crate). Logs: `🗑️generated/cc-bNNN.{log,errs,skipped,done}`.

| idx | scope | result |
| --- | --- | --- |
| 34, 35 | framework lib 1, 2 | green |
| 36 | framework lib 3 | blocked by peer edits (see below) |
| 63-67, 69, 71, 73, 75 | root lib batches | green (crates behind `semio-framework-ui` skipped) |
| 64, 66, 70?, 72? | root test batches | 64, 66 green; 68, 76: test-companion crates (see below) |
| 77, 79, 81, 83 | root lib batches | fully skipped (every crate depends on `semio-framework-ui`) |
| 85 | stdio plugin workspace lib | framework+plugin crates compile up to `semio-framework-interaction`, `-manifest-address`, `-kernel`: `ui_wgpu::wgpu` missing because `ui/🎯️targets/🧊️wgpu/🦀️.rs` is absent from the tree right now (peer c6ui move in progress) |
| 147, 148 | teaching lib + tests | green |
| 149 | hub lib | blocked: peer is rewriting `os-mcp-capabilities/Cargo.toml` (ui -> ui-declaration) |

Extra launcher/tooling fixes since the first report: `descriptors` lsof parser accepts macOS lsof `f` rows and empty `n` rows; `ui_contract` time-travel `with_editor_dialogs` roster; plugin-ownership `pack` dependency and `progress()` option; os-pack optional `semio-framework-dsl` dependency behind `record-codec-laws`; vcs `testkit` feature carries the serde impls of `HistoryPageStack`; mcp bridge types `pub(crate)`; plugin-host test fixtures use the new kernel paths (`store::serialization`, `semio_framework_os_kernel::pack_rt`); workflow-run/flow-document `os-command` dependency; flow-vcs-laws `test-fixtures` dependency; testing-doubles `one-item-publication` testkit dependency and `fixture_canonical_json!` invocation moved next to its macro; `management-sync`, `repo-cli` verbs (`RepoContext` trait imports); actor `ScheduledActor` pub(super); sync `acknowledged_edited_head` exported.

## Open (for the coordinator)
1. `semio-framework-ui` fails the source gate (all-features closure above 12,000 active lines): 257 of 549 root members and every renderer/plugin-host batch (77-84) cannot compile until it is carved. c6ui reports it as pre-existing.
2. The framework workspace manifest is churned by peers (new members with broken manifests: `3d-brep-construction-unit-tests` wants a missing `testkit` feature, `ui/tests/targets-wgpu-prepared-laws` failed to load). Batches 36-62 and 153-163 need a quiet window.
3. Test-companion crates (`--tests`): the carve put tests into sibling crates that touch crate-private internals of the crate under test (mcp-bridge session types, store `SnapshotReadLease`/`OwnedSchemaRecordCursor` fields, `LaneItemReceipt` imports, `test_support`, os-services `NativeIoJob` trait methods, vcs `test_allocation`). Root batches 68, 72?, 74?, 76 fail with ~170 + 193 errors of that kind; fix = widen those items to `pub` (or move the tests back) per crate. Logs: `🗑️generated/cc-b068.log`, `cc-b076.log`.
4. Zero-touch gaps: a fresh clone has no `.🧬semio/🦑️repo/⚡️cache/rust` parent (fixed in `budget/📥️opening`, untested on a clean tree); the framework lock needs the crates.io index online once (`update --workspace --offline` fails for `zip`).
5. Concurrent `bun repo` runs by other executors attach disposable images that make the launcher fail with transient roster errors; the runner retries.

## Stop state (13:25)
Blocked, not finished. After the last green batches (63-67, 69, 71, 73, 75, 147, 148) every further call fails on peer churn in the ui/mcp area: `os-mcp-capabilities` manifest rewrite (ui -> ui-declaration), `semio-framework-ui-wgpu-layout` path dependency of the renderer shell-async crate not yet present, `ui/🎯️targets/🧊️wgpu/🦀️.rs` missing. Not started: batches 36-62 (framework, unstable manifest), 70/72/74 (root tests, same churn), 86-146 and 150-172 (plugin workspaces, hub, wasm/feature lanes). Resume with `cc-tools/rb.sh <idx>` (idx = line in `cc-tools/batches.json`, same order as `compile-plan.md`); the runner needs no arguments beyond the index and keeps `cc-tools/refused.txt`.
No `[DEBUG]` logs remain (the one temporary log in `invocation/process/🟦️.ts` was reverted, file byte-identical). Disk free 238 GB. Temporary tooling: `cc-tools/` (wscheck.mjs, run.mjs, rb.sh, graph.mjs, genindex.mjs, crate-errs.mjs, privscan.py); logs `🗑️generated/cc-*`.
