# Compile Runbook: First Real Compile Window

Date: 2026-10-16 (audit at 18:33 CEST). Ticket folder F = `🪓️monolith-split`.
Basis: read-only audit of the 85 `RESULT-*.md` files modified on 2026-10-16, plus `📓️COMPILE-WINDOW-CHECKLIST.md`, `📓️WAVE5-BRIEF.md`, `RESULT-COMPILE-PASS-1.md`, `RESULT-COMPILE-PASS-2.md`, `RESULT-INFRA-UNBLOCK-2.md` and `RESULT-INFRA-UNBLOCK-3.md`. Nothing was compiled, edited or committed by this audit. Section references below are to those RESULT files.

## 0. State at audit time

- Last real compile: `RESULT-COMPILE-PASS-2.md` section 2. Framework batch 0 (60 crates) exited 0 at 05:08 to 05:11. Batches 1 and 2 exited 101 at 05:11 to 05:28 with 4 failing crates (fixed in source, not yet re-verified). Batch 3 never compiled. Before that, `RESULT-COMPILE-PASS-1.md` (2026-10-15) compiled 129 of 137 framework crates natively.
- Launcher: `RESULT-INFRA-UNBLOCK-2.md` (05:04) says the launcher reached cargo. `RESULT-INFRA-UNBLOCK-3.md` (15:15) says it does not: `LAUNCHER REACHES CARGO: no`.
- Two hard blockers, both refusing before rustc: the image receipt lacks `mountDirectory`, and the image is 32,124,928 B short of the 4 GiB initial-headroom gate.
- Unverified edits waiting for their first compiler verdict: the fixes in `RESULT-COMPILE-PASS-2.md` section 3, the two fixes in `RESULT-COMPILE-PASS-1.md` section 3, and every flip, carve and split in the 85 results (all static evidence only).

## 1. Command rules (apply to every phase)

- **PREFIX.** Every launcher call below is `PREFIX <cargo subcommand and arguments>`, where:

  ```
  PREFIX = cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --
  ```

  This is the canonical form from `WAVE5-BRIEF.md` rule 2. Do not use the `bun ./…/📜️script.ts repo cargo` form that `RESULT-PLUGIN-INTEGRATION-1.md` section 8 quotes.
- **Two workspaces.** `🧰️framework/Cargo.toml` is its own workspace with its own lock. A framework crate checked with `-p` from the root has unresolved dev-dependencies (`RESULT-COMPILE-PASS-1.md` section 1.1). Framework members therefore always get `--manifest-path 🧰️framework/Cargo.toml` after `--`. Plugin, hub and teaching crates get their own `--manifest-path`. Root crates get none.
- **Order inside each phase.** `--lib` first, then `--tests`, then `--target wasm32-unknown-unknown` and/or `wasm32-wasip2`, then `test` runs. Always `--keep-going --message-format=short`.
- **No `--all-targets`.** `RESULT-COMPILE-PASS-1.md` section 1.3: the source gate refuses a crate whose dev-dependency `semio-framework-replication` is oversized. Use `--lib` and `--tests` as two calls.
- **One launcher call at a time.** Each call takes the fleet mutex (section 2.3) and releases it at once. Batch a whole workspace into one call. Never run two calls in parallel.
- **Exit codes.** Exit 101 means rustc reported errors. Exit 1 with a launcher message (`Insufficient physical compiler initial headroom`, `… required`, `… issuance …`) means no compile happened. Split the two in the log before attributing anything.
- **Attribution.** Fix errors in crates the executor owns or that its flip exposed. Report others as "foreign errors" in the RESULT file (`WAVE5-BRIEF.md` rule 2).
- **Logs.** Under `🗑️generated/` in F, with a prefix. Delete tool output at ticket close (AGENTS.md).

## 2. Phase 0: Prerequisites

### 2.1 Blockers and owners

| # | Blocker | Evidence | Owner | Resolution | Clear when |
| --- | --- | --- | --- | --- | --- |
| B1 | Image receipt `⚡️cache/rust/fixed/image.json` has no `mountDirectory` claim. Every launcher call refuses in `inspectPersistentReceiver`. | `RESULT-INFRA-UNBLOCK-3.md` sections 1, 5.1 | Peer ticket `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES` (Native storage contract, creator and retire path) and Root | Either (a) the peer lands the creator and retire path that writes a conforming receipt, and Root authorises the sanctioned retire; or (b) the peer provides a sanctioned receipt-republish operation. Do not hand-edit `image.json` (the value would be a guess). | `bun iu3-tools/validate-receipt.ts` run from `🧰️framework` (read-only) prints no `/mountDirectory is required`. |
| B2 | Headroom: `⚡️cache/rust/fixed/m` has 4,262,842,368 B available against 4,294,967,296 B required (15:15). | `RESULT-INFRA-UNBLOCK-3.md` section 5.2; `RESULT-COMPILE-PASS-1.md` section 5; `RESULT-COMPILE-PASS-2.md` section 6 | Root | The sanctioned historical retirement: `⚖️cargo-retirement-historical-preview`, then `retire`. At 15:15 the retirement preview was still blocked by B1. `RESULT-COMPILE-PASS-2.md` section 6 records the retirement failing with `Live Original resource control issuance required`. Nothing may be deleted or moved in the image, and the gate may not be bypassed. | The probe in 2.2 no longer prints `Insufficient physical compiler initial headroom`. |

Secondary launcher refusals recorded after 15:00. None is a compile result. They should disappear once B1 and B2 clear, but each needs one dry probe to confirm:

| Message | Where it was recorded | Meaning |
| --- | --- | --- |
| `Live Original resource control issuance required` | `RESULT-PLUGIN-WP5.md`, `RESULT-PLUGIN-WP6.md`, `RESULT-PLUGIN-WP1.md`, `RESULT-OS-KERNEL-WP2-3.md` | Compiler-ingress wrapper (same family as B2). |
| `Original queue claim path identity required` | `RESULT-PLUGIN-WP3.md`, `RESULT-RENDERER-WP4.md` | Lease code. Owned by the launcher peers. |
| `Exact canonical persistent Native publication required` | `RESULT-INFRA-UNBLOCK-3.md`, `RESULT-OS-KERNEL-WP8.md`, `RESULT-SHARED-FILES-3.md` section 6 | Same family as B1. |
| `Cargo authored profiles belong to canonical two-intent consumption` | `RESULT-PLUGIN-WP2.md`, `RESULT-RENDERER-INTEGRATION-1.md` section 6.6 | Launcher profile check. `RESULT-INFRA-UNBLOCK-2.md` says it was fixed at 05:04, so check for recurrence. |
| `SQLiteError: database or disk is full` | `RESULT-RENDERER-INTEGRATION-2.md` line 89 | Lease database. Retry after a pause. |
| `Original selected owner registry changed` | `RESULT-RENDERER-INTEGRATION-1.md` section 6.6 | A peer edited `📋️build-owners.json` during the run. Wait 60 s and retry. |
| `NX Original daemon child exited before acquisition` | `RESULT-OS-KERNEL-WP0.md`, `RESULT-EPHEMERAL-SINGLE-OWNER.md` | Nx route, not cargo. |

### 2.2 Probe (first call after B1 and B2 clear)

Take the mutex (2.3), then run:

`PREFIX check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml --message-format=short`

This is the probe that `RESULT-INFRA-UNBLOCK-3.md` section 1 reproduced. Expected: exit 0 with rustc output. Any launcher message means B1 or B2 is still open. Stop there.

### 2.3 Fleet compile mutex

Source: `WAVE5-BRIEF.md` "Compile admission is single-slot". Let G be F/`🗑️generated`. At audit time `G/cargo-mutex.lock` does not exist (free).

- Acquire: `mkdir "$G/cargo-mutex.lock"` (atomic). If it fails, `sleep 60` and retry. On success, write `<owner> <pid> <date>` to `cargo-mutex.lock/owner`.
- Hold for one launcher call only, then release: `rm -f "$G/cargo-mutex.lock/owner" && rmdir "$G/cargo-mutex.lock"`. Release on failure too.
- Stale: an owner file older than 75 minutes. Remove it and note that in the RESULT file.
- `Insufficient physical compiler initial headroom`: release, wait 10 minutes, retry.
- `cannot update the lock file … --locked`: run the `update --workspace --offline` for that workspace first (2.4).
- `okeng-tools/waitcc.py` and `okeng-tools/cc.py` take the mutex themselves (`RESULT-OS-KERNEL-ENGINE.md` line 40).

### 2.4 Lock refresh (checklist step 1, mandatory first)

Refresh each workspace with `PREFIX update --workspace --offline [--manifest-path <ws>/Cargo.toml]`, one mutex hold per call. Accept cargo's result (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 102). Afterwards, `PREFIX metadata --locked --offline --format-version=1 --no-deps --manifest-path <ws>/Cargo.toml` must be a no-op.

Every workspace whose lock was edited by hand or is stale:

| Lock | Workspace (manifest) | Hand edits and staleness recorded in | Refresh |
| --- | --- | --- | --- |
| L1 root | repo root `Cargo.toml` | `RESULT-DUPLICATE-MOUNTS-2.md` (180 locks, see L7); `RESULT-SINGLE-MOUNT-AND-GATE.md` hand-off 2 (abi, canonical-json, refusal packages); `RESULT-RENDERER-TEST-DEPS.md` line 50 (31 packages patched) and section 7.3 (40 stale); `RESULT-DIRECTION-LEFTOVERS.md` line 141 (two member packages); `RESULT-OS-KERNEL-WP8.md` line 83 (hand-derived blocks for 16 crates and the kernel) | `PREFIX update --workspace --offline` |
| L2 framework | `🧰️framework/Cargo.toml` | `RESULT-COMPILE-PASS-2.md` section 6 (hand-added `pack-record-retained -> replication-wire-core` edge); `RESULT-TOOL-TEST-COMPANIONS.md` section 3 (hand-written entries for `tool-run`, `tool-machine` and two `*-tests` packages); `RESULT-SHARED-FILES-3.md` (helper blocks removed) | `PREFIX update --workspace --offline --manifest-path 🧰️framework/Cargo.toml` |
| L3 hub | `🌎️hub/Cargo.toml` | `RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 137 (seven edges on `semio-hub-directory` added by hand); `RESULT-W4-HUB.md` section 9 (57 new path packages, `--locked` fails); `RESULT-DIRECTION-LEFTOVERS.md` section 3.2 | `PREFIX update --workspace --offline --manifest-path 🌎️hub/Cargo.toml` |
| L4 space | `✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml`; `🪐️space` artifact workspace (manifest to confirm); `🪐️space/🏭️bridge`, `🪐️space/🗿️artifacts/🏠️home`, `🪐️space/🗿️artifacts/🪐️space` (lock list in `RESULT-CARGO-PLUGIN-DIRECTION.md` line 46) | `RESULT-CARGO-PLUGIN-DIRECTION.md` section 2 (core update exit 0 at 06:xx, peers changed it since); `RESULT-PLUGIN-WAVE0-GENERATORS.md` line 95 (`🪐️space/Cargo.lock` stale) | `update --workspace --offline --manifest-path …` for each |
| L5 procedural | `✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml`, `…/🏭️bridge`, `…/🗿️artifacts/🌀️generation2d/Cargo.toml`, `…/🗿️artifacts/🧊️generation3d/Cargo.toml`, `✏️s/🧑‍💻dev/🧩️composition/Cargo.toml`, `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.toml` | `RESULT-DIRECTION-LEFTOVERS.md` lines 141 and 179 (seven locks with the core crates) | per manifest |
| L6 stdio | `✏️s/🔌️plugins/🗄️stdio/Cargo.toml` | `RESULT-CARGO-PLUGIN-DIRECTION.md` section 3.1 (role change, no lock change planned); line 82 (stdio check queued) | per manifest |
| L7 plugin locks in bulk | every workspace whose `Cargo.lock` is in these lists: `dm2-tools/touched.txt` (180 `Cargo.lock` entries), `sf3-tools/touched.txt` (73), `smg-tools/touched.txt` (48), `🗑️generated/pch-lock-touched.txt` (30, plus 11 `block` locks in `RESULT-PLUGIN-CRATE-HYGIENE.md` section 5), `🗑️generated/opd-locks-a.log` (178 os-pack locks, `RESULT-OS-PACK-DEDUPE.md` line 60), `okwp23-tools/touched.txt` | `RESULT-DUPLICATE-MOUNTS-2.md` line 170; `RESULT-SHARED-FILES-3.md` line 125; `RESULT-SINGLE-MOUNT-AND-GATE.md` line 115 (154 locks lack the plugin edge, 136 lack `reactor-context`); `RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 102 (179 locks); `RESULT-PLUGIN-CRATE-HYGIENE.md` section 5 | Derive each lock's workspace root (nearest `[workspace]`), then one `update --workspace --offline --manifest-path <root>/Cargo.toml` per root |
| L8 layout | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml` | Named by checklist step 1. `RESULT-RUST-CROSS-COUPLINGS.md` line 44 says its three lock refreshes are not started. | `update --workspace --offline --manifest-path …` |
| L9 forms | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml` | `RESULT-DUPLICATE-MOUNTS-2.md` line 153 | same |
| L10 teaching | `🎓️teaching/Cargo.toml` | Named by checklist step 1. `RESULT-PRODUCT-FILE-SPLITS.md` line 111 says "only if a peer added crates". **Membership conflict**: `RESULT-REPO-HUB-RUST-FILE-SPLITS.md` line 116 puts `teaching-proctor` in the root workspace. Decide the owner before refreshing (section 5). | `update --workspace --offline --manifest-path 🎓️teaching/Cargo.toml` after the decision |
| L11 `✏️s` | `✏️s/Cargo.toml` (`semio-s-artifact-stdio-contract-testing`, `semio-s-spatial-kernel-semio-session`) and `✏️s/🧑‍💻dev/🧩️composition/Cargo.toml` | `RESULT-SMALL-FIXES-1.md` line 101; `RESULT-RUST-FILE-SPLITS-8.md` lines 146 and 149 | per manifest |

**Bun lockfiles (not cargo).** Six scopes were transplanted by hand: root, `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`, `🎓️teaching` (`RESULT-PACKAGE-SPECIFIER-CODEMOD.md` section 7; `RESULT-PACKAGE-SPECIFIER-CODEMOD-2.md` is still in progress at 17:50). Root and area `bun.lock` files were also edited in `RESULT-HYGIENE-RULES-2.md` line 98. **Do not run `bun install --lockfile-only`** (it rewrites about 24,898 lines in bun's text format, `RESULT-PACKAGE-SPECIFIER-CODEMOD.md` section 7). The check is `bun install --frozen-lockfile --dry-run` per scope; `RESULT-HYGIENE-RULES-2.md` line 92 reports it passing for the six scopes.

The registry-crate download step (checklist step 1) follows the route in `RESULT-INFRA-UNBLOCK.md`. This audit did not re-read that file.

## 3. Phase 1: Framework leaves

Scope: framework workspace. Every call ends `--manifest-path 🧰️framework/Cargo.toml`.

1a. Lock: `PREFIX update --workspace --offline --manifest-path 🧰️framework/Cargo.toml`. (`RESULT-TOOL-TEST-COMPANIONS.md` line 107; `RESULT-COMPILE-PASS-1.md` section 1.2.)

1b. Verify the unverified fixes first:
- `PREFIX check -p semio-framework-canvas-assets -p semio-framework-ui-wgpu-device -p semio-framework-replication-interaction -p semio-framework-replication-mutation-messages -p semio-framework-pack-format-catalog -p semio-framework-replication -p semio-framework-pack-record-retained --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`
  Verifies: `RESULT-COMPILE-PASS-1.md` section 3 (two fixes); `RESULT-COMPILE-PASS-2.md` section 3 (the fixes listed there).
- `PREFIX check -p semio-framework-replication --lib --features deflate --manifest-path 🧰️framework/Cargo.toml`. Verifies: the `deflate` gate restored on the facade re-exports (`RESULT-COMPILE-PASS-2.md` section 3).
- `PREFIX check -p semio-framework-canvas-assets --lib --target wasm32-wasip2 --manifest-path 🧰️framework/Cargo.toml`. Verifies: the `dead_code` expectation on the wasip2 arm (`RESULT-COMPILE-PASS-1.md` section 3, item 1).
- Foreign, not in this group: `semio-framework-ui-contract-presentation` (E0425 at `🔢️number-format/🦀️.rs:50`, `RESULT-COMPILE-PASS-2.md` section 7). Report it; it blocks its dependents.

1c. Framework lib pass, all 240 members in `🗑️generated/cp2-worklist-fw.txt`, in batches of 60:
- `PREFIX check -p <batch 0 names> --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (batch 0 already exited 0 at 05:11; re-run because fixes landed after it).
- `PREFIX check -p <batches 1 to 3 names> --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (one call, as in `RESULT-COMPILE-PASS-2.md` section 2; batch files `🗑️generated/cp2-fw-batch-1` to `-3`).
Verifies: `RESULT-COMPILE-PASS-1.md` section 2 (129 of 137) and `RESULT-COMPILE-PASS-2.md` sections 2 and 3.

1d. Framework lib and tests, by family:
- `PREFIX check -p semio-framework-number -p semio-framework-async -p semio-framework-dsl --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-FRAMEWORK-RUST-FILE-SPLITS.md` line 103)
- `PREFIX check -p semio-framework-pack-record -p semio-framework-pack-format-catalog --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-PACK-QUIZ-FILE-SPLITS.md` line 136)
- `PREFIX check -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints -p semio-framework-value-derive --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-SCHEMA-VALUE-MATH-SPLITS.md` line 129)
- `PREFIX check -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (line 130)
- `PREFIX check -p semio-framework --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (line 132)
- `PREFIX check -p semio-framework-tool-run -p semio-framework-tool-machine -p semio-framework-machine -p semio-framework-graph-dsl -p semio-framework-graph-engine -p semio-framework-graph-layout-run --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-TOOL-GRAPH-MACHINE-SPLITS.md` line 156), then `… -p semio-framework-graph-layout-run --features testing` for that crate and `--tests` for all six (line 159)
- `PREFIX check -p semio-framework-tool-run-tests -p semio-framework-tool-machine-tests --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-TOOL-TEST-COMPANIONS.md` line 109)
- `PREFIX check -p semio-framework-pack-record-retained -p semio-framework-pack --lib --tests --keep-going --manifest-path 🧰️framework/Cargo.toml` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 120)
- `PREFIX check -p semio-framework-trace-memory-testing --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-DEBUG-AND-ALIAS-CLEANUP.md` line 171)
- `PREFIX check -p semio-framework-io-sqlite-snapshot -p semio-framework-io-sqlite-snapshot-tests --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-OS-TEST-COMPANIONS.md` line 83)
- `PREFIX check -p semio-framework-ui-host -p semio-framework-ui-backend-webgpu -p semio-framework-abi --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`, then the same with `--tests` (`RESULT-SINGLE-MOUNT-AND-GATE.md` line 97)
- The framework list at `RESULT-DUPLICATE-MOUNTS-2.md` line 147 (40 names, verbatim there): `PREFIX check -p <names> --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`
- The framework companion list at `RESULT-FRAMEWORK-TEST-COMPANIONS.md` line 72 (22 names): `PREFIX check --offline --tests -p <names> --manifest-path 🧰️framework/Cargo.toml`. **Note:** that RESULT recorded no `--manifest-path`; the names are framework members, so the flag is added here.

1e. Framework wasm:
- `PREFIX check -p semio-framework-number -p semio-framework-async --lib --target wasm32-wasip2 --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`, and the same with `--target wasm32-unknown-unknown` (`RESULT-FRAMEWORK-RUST-FILE-SPLITS.md` line 109)
- `PREFIX check -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --lib --target wasm32-unknown-unknown --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml` (`RESULT-SCHEMA-VALUE-MATH-SPLITS.md` line 131)
- `PREFIX check -p semio-framework-tool-run -p semio-framework-tool-machine --lib --target wasm32-wasip2 --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`, and `--target wasm32-unknown-unknown` (`RESULT-TOOL-TEST-COMPANIONS.md` line 110)
- `PREFIX check -p semio-framework-pack-record -p semio-framework-pack-format-catalog --lib --target wasm32-unknown-unknown …` and `--target wasm32-wasip2` (`RESULT-PACK-QUIZ-FILE-SPLITS.md` line 141)
- `PREFIX check -p semio-framework-ui-wgpu-device -p semio-framework-ui-wgpu-draw-execution -p semio-framework-ui-wgpu-mesh -p semio-framework-ui-wgpu-pipelines -p semio-framework-ui-wgpu-raster --lib --target wasm32-unknown-unknown …` (`RESULT-COMPILE-PASS-1.md` section 5(c) and the ui-wgpu packages of section 1.2)

1f. Framework tests:
- With `SVM_CORPUS_OUT=<F>/🗑️generated/svm-derive-corpus` added to the PREFIX environment: `PREFIX test -p semio-framework-value-derive --manifest-path 🧰️framework/Cargo.toml --lib svm_split_expansion -- --nocapture` (`RESULT-SCHEMA-VALUE-MATH-SPLITS.md` line 92)
- `PREFIX test -p semio-framework --lib exports_typescript_bindings --manifest-path 🧰️framework/Cargo.toml` (line 133)
- `PREFIX test -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --manifest-path 🧰️framework/Cargo.toml` (line 134), and `PREFIX test -p semio-framework-value-derive --manifest-path 🧰️framework/Cargo.toml` (line 135)
- `PREFIX test -p semio-framework-tool-run-tests -p semio-framework-tool-machine-tests --lib --manifest-path 🧰️framework/Cargo.toml` (`RESULT-TOOL-TEST-COMPANIONS.md` line 111)
- `PREFIX test -p semio-framework-pack-record-retained --lib retained_value_retirement_demands --manifest-path 🧰️framework/Cargo.toml`, then `… --lib` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 120)
- `PREFIX test -p semio-framework-pack --test pack_record_schema_laws --test pack_schema_hash --manifest-path 🧰️framework/Cargo.toml` (`RESULT-OS-PACK-DEDUPE.md` line 88)
- `PREFIX test --offline -p <each framework companion> -- --test-threads=1` (`RESULT-FRAMEWORK-TEST-COMPANIONS.md` line 73). Then compare `cargo test -- --list` per companion with the name multisets in the log that RESULT names (line 77).

## 4. Phase 2: OS kernel and store (root workspace)

2a. Lock: `PREFIX update --workspace --offline` (root). (`RESULT-RENDERER-TEST-DEPS.md` line 155.)

2b. Kernel engine, 18 crates plus `space-schema`:
- From F: `python3 okeng-tools/waitcc.py lib1 okeng-tools/b2.crates`. It runs `update --workspace --offline`, then one batched `check -p … --lib --keep-going` under the mutex (`RESULT-OS-KERNEL-ENGINE.md` line 106).
- Then the same crate set with `--tests` (line 106, "lib, then `--tests`").
Verifies: `RESULT-OS-KERNEL-ENGINE.md` section 4 (the 18 crates were never compiled) and line 98.

2c. SQLite host, seven crates: `PREFIX check -p <the 7 crates of RESULT-OS-KERNEL-SQLITE-HOST.md section 6> --lib --tests --keep-going --message-format=short`. The names are in that section's table and in `okwp23-tools/touched.txt`; this audit did not restate them. Names that appear in that RESULT include `semio-framework-os-store-sync-native`, `semio-framework-os-services` and `semio-framework-os-channel`. Verifies: `RESULT-OS-KERNEL-SQLITE-HOST.md` line 76 (refused three times, never compiled).

2d. Kernel WP0, WP2-3 and WP8:
- WP2-3, 14 crates (`RESULT-OS-KERNEL-WP2-3.md` line 7; `semio-framework-` prefix added here, matching the package names in the root cargo lines): `os-store-child`, `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-canonical-json`, `os-store-one-item-publication`, `os-store-operation-wire`, `os-store-snapshot-clone`, `os-store-one-item-batch`, `os-store-space-schema`, `os-store-retirement`, `os-store-owned-schema-fields`, `os-store-owned-schema-decode`. Command: `PREFIX check -p <14 names> --lib --tests --keep-going --message-format=short`. Verifies: the 1077 = 1077 test count is static; this is the first real check.
- WP0, new crates: `semio-framework-os-list-delta` and the rest of the 12 new crates (`okwp0-tools/touched.txt`). Run `--lib --tests`, then `--target wasm32-unknown-unknown` and `--target wasm32-wasip2` (`RESULT-OS-KERNEL-WP0.md` line 127).
- WP8: `PREFIX check -p semio-framework-os-kernel -p semio-framework-os-store-testing-doubles -p semio-framework-os-store-testing-fixtures --lib --tests --keep-going --message-format=short`, then `cargo test --lib --no-run` for A, B and the 14 test crates in the body of `RESULT-OS-KERNEL-WP8.md` (line 128: "expect only privacy and import findings"). Verifies: that RESULT, section 11.

2e. Store and pack root batch (`RESULT-OS-PACK-DEDUPE.md` line 86): `PREFIX check -p semio-framework-os-pack -p semio-framework-os-kernel -p semio-framework-os-mcp -p semio-framework-os-store-codec-factory -p semio-framework-os-store-text-io -p semio-framework-os-store-durable-group -p semio-framework-os-store-durable-group-wire -p semio-framework-os-store-tests-09 -p semio-framework-os-store-test-support -p semio-framework-plugin-decl-surface -p semio-framework-os-dsl-tests --lib --tests --keep-going --message-format=short`. This is the same set the PACK-DEDUPE result lists as its root step 1.

## 5. Phase 3: OS pack, run, vcs, io, services and MCP (root workspace)

3a. Run, vcs, io and neural-model (`RESULT-OS-RUN-VCS-IO-SPLITS.md` section 8):
- Lib: `PREFIX check -p semio-framework-os-run -p semio-framework-os-vcs -p semio-framework-os-pack -p semio-framework-os-io -p semio-framework-os-kernel-neural-engine-model --lib --keep-going --message-format=short`
- Tests: the same with `--tests`
- Wasm: the lib call with `--target wasm32-unknown-unknown`
- Dependents, `--lib --tests`: the list at `RESULT-OS-RUN-VCS-IO-SPLITS.md` line 170 (verbatim there)
- Tests: `PREFIX test -p semio-framework-os-pack --lib --keep-going`, once more with `--features record-codec-laws`; then `PREFIX test -p semio-framework-os-vcs --lib`, `… -p semio-framework-os-io --lib`, `PREFIX test -p semio-framework-os-kernel-neural-engine-model --lib --features testkit`, `PREFIX test -p semio-framework-os-run --lib`.

3b. Pack consumers, `--lib --tests` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 117): `PREFIX check -p semio-framework-os-pack -p semio-framework-os-pack-tools -p semio-framework-os-kernel -p semio-framework-os-flow-mesh -p semio-framework-artifact-flow-flow-mutation-laws -p semio-framework-os-store-engine-core -p semio-framework-os-store-command -p semio-framework-os-store-sync-tests-1 -p semio-framework-os-store-text-io -p semio-framework-os-store-testing-fixtures -p semio-framework-os-store-tests-05 -p semio-framework-os-store-tests-09 -p semio-framework-os-store-space-schema -p semio-framework-os-store-document -p semio-framework-os-command-tests -p semio-framework-plugin-host-test-component -p semio-framework-plugin-host-test-component-refusal -p semio-framework-os-dsl -p semio-framework-os-dsl-tests -p semio-framework-print --lib --tests --keep-going --message-format=short`
- Wasm: `PREFIX check -p semio-framework-os-pack --lib --target wasm32-unknown-unknown …` and `--target wasm32-wasip2`.
- Tests: `PREFIX test -p semio-framework-os-pack-tools --lib`; `… -p semio-framework-os-store-tests-05 --lib`; `… -p semio-framework-os-store-tests-09 --lib`; `… -p semio-framework-os-store-sync-tests-1 --lib`; `… -p semio-framework-os-dsl-tests --lib`; `… -p semio-framework-os-store-testing-fixtures --lib`; `… -p semio-framework-os-store-durable-group --lib`.
- Expect the new `facade_stored_content_hash_is_stable_across_two_encodes` and `bridge_document_and_record_body_bytes_equal_the_framework_owner_bytes` to run (line 119).

3c. OS services and MCP (`RESULT-OS-SERVICES-MCP-SPLITS.md` lines 118 to 127):
- `PREFIX check -p semio-framework-os-services -p semio-framework-os-mcp-http-transport -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib --keep-going --message-format=short`
- Wasm: `PREFIX check -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib --target wasm32-unknown-unknown --keep-going --message-format=short`
- Dependents: `PREFIX check -p semio-framework-os-mcp -p semio-framework-os-mcp-workspace -p semio-framework-os-mcp-routing -p semio-framework-os-mcp-bridge -p semio-framework-os-mcp-consent -p semio-framework-os-mcp-server -p semio-framework-os-mcp-shell-channel -p semio-framework-os-mcp-tools-gateway -p semio-framework-os-mcp-hub-pair -p semio-framework-os-directory-client -p semio-framework-os-store-sync-native -p semio-framework-os-kernel -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-imports -p semio-framework-plugin-host-shard --lib --keep-going --message-format=short`, with `--features sync` added for `semio-framework-os-directory-client` (line 127).
- Tests: `PREFIX test -p semio-framework-os-services --lib` and `PREFIX test -p semio-framework-os-mcp-http-transport -p semio-framework-os-mcp-hub-remote -p semio-framework-os-mcp-bridge-codec --lib`.

3d. OS test companions (`RESULT-OS-TEST-COMPANIONS.md` line 80): `PREFIX check --lib --tests --keep-going --message-format=short -p semio-framework-os-services -p semio-framework-os-services-tests -p semio-framework-os-directory -p semio-framework-os-directory-tests-1 -p semio-framework-os-directory-tests-2 -p semio-framework-os-directory-client -p semio-framework-os-directory-client-tests -p semio-framework-os-command -p semio-framework-os-command-tests -p semio-framework-os-history-record-decode -p semio-framework-os-history-record-decode-tests -p semio-framework-os-channel -p semio-framework-os-channel-tests -p semio-framework-os-config -p semio-framework-os-config-tests-1 -p semio-framework-os-config-tests-2 -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-kernel-dsl-derive-expand -p semio-framework-os-kernel-dsl-derive-tests`
- Then `PREFIX test -p <each companion> --lib`, and the list-count gate in line 83 (sum of `cargo test -p <companion> --lib -- --list` per crate equals the counts in that RESULT's section 4).

3e. OS flow contract facade (`RESULT-DIRECTION-LEFTOVERS.md` line 176): `PREFIX check -p semio-framework-os-flow-contract -p semio-framework-os-playbook-contract --lib --keep-going --message-format=short`.

## 6. Phase 4: Plugin host (root workspace)

4a. Plugin-host carve, 15 packages (`RESULT-PLUGIN-HOST-CARVE.md` line 161; never compiled, section 7):
- `PREFIX check -p semio-framework-plugin-host-wit -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-operation -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-imports -p semio-framework-plugin-host-guest-runtime -p semio-framework-plugin-host-wake -p semio-framework-plugin-host-post-turn-relay -p semio-framework-plugin-host-routers -p semio-framework-plugin-host-process-transport -p semio-framework-plugin-host-shard -p semio-framework-plugin-host-shard-laws -p semio-framework-plugin-host-guest-laws -p semio-framework-plugin-host-shard-executor -p semio-framework-plugin-host --lib --keep-going --message-format=short`
- Then the same with `--tests`.

4b. Guest contract, refusal component, space schema (`RESULT-SINGLE-MOUNT-AND-GATE.md` lines 96 and 97):
- `PREFIX check -p semio-framework-plugin-guest-contract -p semio-framework-plugin-host-test-component-refusal -p semio-framework-os-store-space-schema -p semio-framework-os-store-testing-fixtures -p semio-framework-plugin-reactor-context --lib --keep-going --message-format=short`
- `--tests` for `semio-framework-plugin-guest-contract` and `semio-framework-os-store-space-schema`.

4c. Plugin-host dependents, root (`RESULT-DUPLICATE-MOUNTS-2.md` line 146): `PREFIX check -p semio-framework-os-history -p semio-framework-os-history-record-decode -p semio-framework-os-history-testing-retirement -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-mcp -p semio-framework-os-mcp-foundation -p semio-framework-os-mcp-plugin-runtime -p semio-framework-os-pack -p semio-framework-os-renderer-wgpu -p semio-framework-os-run -p semio-framework-os-store-sync -p semio-framework-plugin-host -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-wake -p semio-framework-plugin-interpreter -p semio-framework-plugin-interpreter-runtime -p semio-repo-test-host --lib --tests --keep-going --message-format=short`. Expect the foreign failures in that RESULT's section 6 first.

4d. Mixed batch (`RESULT-MISSING-TEST-INPUTS.md` line 94; run here because it needs the kernel and the plugin host, and it also contains renderer crates): `PREFIX check -p semio-framework-os-kernel -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-spr-laws-tests -p semio-framework-plugin -p semio-framework-plugin-host-test-component -p semio-framework-plugin-reactor-lifecycle -p semio-framework-ui-runtime-reconcile -p semio-framework-ui-runtime-reconcile-primitives --lib --tests --keep-going --message-format=short`. This is the compile the RESULT says proves hand-off F, since `sqlite_snapshot_tests` has not compiled since 2026-10-06.

4e. Plugin-host singles. These WP1 to WP4 crates have a recorded command but no manifest path, so root is assumed. Confirm against the manifest:
- `PREFIX check -p semio-framework-plugin-identity --lib` (`RESULT-PLUGIN-WP1.md` line 113)
- `PREFIX check -p semio-framework-plugin-decl-views --lib --tests` (`RESULT-PLUGIN-WP2.md` line 103)
- `PREFIX check -p semio-framework-plugin-test-support --lib` (`RESULT-PLUGIN-WP3.md` line 9)
- `PREFIX check -p semio-framework-plugin-reactor-tasks --lib` (`RESULT-PLUGIN-WP4.md` line 141)

## 7. Phase 5: Plugin tier and the pi1 flips (root)

5a. Flipped crates: `PREFIX check -p <each key of pi1-tools/flipped.json> --keep-going --message-format=short --lib`, then the same with `--tests`, with `--features artifact-app-testing`, and with `--target wasm32-wasip2` (`RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 2; the command form there is replaced by the canonical PREFIX). Verifies: all 24 manifest changes, the `artifact-app-testing` forwarding, and the wasm-gated bodies of `reactor-wit-bridge` (item 3).

5b. Flip order is bottom-up by rank (`WAVE5-BRIEF.md` rule 3). `tool-run` (rank 41) and `time-travel` (rank 42) cannot flip until `time-travel-turns` and `tool-run-turns` (ranks 53 and 54) land, because their roots mount `instance_turns` (`RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 1).

5c. Include paths: about 20 `include_str!` literals with 8 or 9 `../` steps point into moved directories. They break the test build independently of the flips. Fix them before 5a's `--tests` (`RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 4).

5d. WP7 tiers 2 and 3 (`RESULT-PLUGIN-WP7.md` section 7 and 8.1) must not run before pi1. No WP7 crate name is needed to start, because the flips come first.

## 8. Phase 6: Renderer (root workspace)

6a. Renderer test-dependency batch, about 45 crates (`RESULT-RENDERER-TEST-DEPS.md` line 156), `--lib --tests --keep-going --message-format=short`: `-p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-renderer-wgpu-shell-tests-windows-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-4 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-6 -p semio-framework-os-renderer-wgpu-shell-tests-input-3 -p semio-framework-os-renderer-wgpu-media-slots -p semio-framework-os-renderer-wgpu-scene-canvas2d -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-ui-engine -p semio-framework-os-renderer-wgpu-agent-elements -p semio-framework-os-renderer-wgpu-dock -p semio-framework-os-renderer-wgpu-shell-tests-docs-3 -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-render -p semio-framework-os-renderer-camera -p semio-framework-os-renderer-wgpu-shell-tests-chrome-3 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-1 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-5 -p semio-framework-os-renderer-wgpu-shell-tests-input-1 -p semio-framework-os-renderer-wgpu-shell-tests-input-2 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-scene-ink-tests -p semio-framework-os-renderer-wgpu-engine-attach-tests -p semio-framework-os-renderer-wgpu-ui-engine-tests -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests`. This needs the lock refresh from 2a. Verifies: `RESULT-RENDERER-TEST-DEPS.md` section 3 and section 7.

6b. Renderer core:
- `PREFIX check -p semio-framework-os-renderer-wgpu --lib --keep-going --message-format=short`, then `--tests` (`RESULT-SMALL-FIXES-1.md` line 100). Verifies: the "needle drift" law (line 61).
- `PREFIX check -p semio-framework-os-renderer-wgpu-engine-registry --lib --tests` (`RESULT-RENDERER-WP2.md` line 83; that RESULT used `--all-targets`, replaced here per section 1).
- `PREFIX check -p semio-framework-os-renderer-wgpu-asset-authority --lib` (`RESULT-RENDERER-INTEGRATION-1.md` section 6.6, abbreviated there as `…-asset-authority`; confirm the package name).
- `PREFIX check -p semio-framework-os-renderer-wgpu-shell-async --lib` (`RESULT-RENDERER-INTEGRATION-2.md` line 89) and `PREFIX check -p semio-framework-os-renderer-wgpu-shell-vocab --lib` (`RESULT-RENDERER-WP4.md` line 140).
- Shell library batch, 31 crates: `ri2-tools/cargo_batch.sh` (`RESULT-RENDERER-INTEGRATION-2.md` line 90), run through the mutex.

6c. Renderer wasm: no RESULT records a renderer wasm command. Run `--lib --target wasm32-unknown-unknown` for `ui-engine-dump` (named in `RESULT-RENDERER-INTEGRATION-1.md` line 109; its wasm `dump*` exports are noted in section 7 item 5). Confirm the package name first.

6d. Renderer tests: `PREFIX test -p <each renderer test crate in 6a> --lib`.

## 9. Phase 7: Plugin workspaces (one call per workspace, `--manifest-path <ws>/Cargo.toml`)

Order: kernel-level plugins first (stdio, space, procedural), then the artifact plugins, then the workspaces that depend on them.

7a. Stdio: `PREFIX check -p <the 12 contract crates of RESULT-CARGO-PLUGIN-DIRECTION.md section 3.1> --lib --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml` (line 82). Then `PREFIX check -p semio-s-artifact-stdio-contract-testing --lib --keep-going --message-format=short --manifest-path ✏️s/Cargo.toml` (`RESULT-SMALL-FIXES-1.md` line 101).

7b. Space core: `PREFIX check -p semio-s-space-core --lib --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml` (`RESULT-CARGO-PLUGIN-DIRECTION.md` line 90, after the L4 refresh).

7c. Space artifact crates `semio-s-artifact-space-space-schema`, `-diff`, `-mutations`, `-editor` (`RESULT-PLUGIN-WAVE0-GENERATORS.md` line 64). Their manifest path is not recorded; confirm it. The lock is stale per line 95.

7d. Procedural core:
- `PREFIX check -p semio-s-procedural-core-semantic-ui -p semio-s-procedural-core-generation-sqlite --lib --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml`, then `… -p semio-s-procedural-core-generation-sqlite-test-support --tests …` (`RESULT-DIRECTION-LEFTOVERS.md` lines 177 to 178)
- `PREFIX check -p semio-s-procedural-core-generation-sqlite-native --lib --keep-going --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml` (`RESULT-SHARED-FILES-3.md` line 124)
- `PREFIX check --lib --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml`, and the same with `…/🗿️artifacts/🧊️generation3d/Cargo.toml` (`RESULT-DIRECTION-LEFTOVERS.md` line 178)

7e. Energy:
- `PREFIX check -p semio-s-plugin-energy-engine-sim -p semio-s-plugin-energy-engine-sim-restore -p semio-s-plugin-energy-engine-bestest --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml` (`RESULT-PLUGIN-FILE-SPLITS-B.md` line 124), then `test -p semio-s-plugin-energy-engine-sim` (line 137)
- `PREFIX check -p semio-s-artifact-energy-model-session -p semio-s-artifact-energy-model-editor-edit -p semio-s-artifact-energy-model-viewer -p semio-s-artifact-energy-model-mutations --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml` (merged from `RESULT-PLUGIN-GATES-GREEN.md` line 119, `RESULT-DUPLICATE-MOUNTS-2.md` line 156, `RESULT-WIRING-BUDGET-ROOTS.md` line 137 and `RESULT-PLUGIN-FILE-SPLITS-B.md` line 132)

7f. Norm and BIM (table rows at `RESULT-PLUGIN-FILE-SPLITS-A.md` lines 189 to 193):
- `PREFIX check -p semio-s-artifact-norm-vdi3805-inferences -p semio-s-artifact-norm-vdi3805 -p semio-s-artifact-norm-vdi3805-editor -p semio-s-artifact-norm-vdi3805-viewer --lib --tests --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml`
- `PREFIX check -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-inferences -p semio-s-artifact-norm-en1993 -p semio-s-artifact-norm-en1993-editor -p semio-s-artifact-norm-en1993-viewer --lib --tests --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml` (includes the `compliance_tests` mount)
- `PREFIX check -p semio-s-artifact-norm-en1999-checks -p semio-s-artifact-norm-en1999-inferences -p semio-s-artifact-norm-en1999 -p semio-s-artifact-norm-en1999-editor -p semio-s-artifact-norm-en1999-viewer --lib --tests --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml`
- `PREFIX check -p semio-s-artifact-bim-model --lib --tests --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml`

7g. FEM: `PREFIX check -p semio-s-artifact-fem-3d-editor-visual-job -p semio-s-artifact-fem-3d-editor-session -p semio-s-artifact-fem-3d-editor-model-window -p semio-s-artifact-fem-3d-viewer --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml` (`RESULT-PLUGIN-FILE-SPLITS-B.md` line 128)

7h. Process3d (`RESULT-PLUGIN-FILE-SPLITS-A.md` table row 1; `RESULT-DUPLICATE-MOUNTS-2.md` line 150):
- `PREFIX check -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-process-process3d-modes -p semio-s-artifact-process-process3d-panels -p semio-s-artifact-process-process3d-wasm --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml`
- `--lib --target wasm32-wasip2` for `…-host` and `…-wasm`.

7i. Flow: `PREFIX check -p semio-s-artifact-flow-flow --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 121).

7j. Writer and mathematical equation:
- `PREFIX check -p semio-s-artifact-writer-writer --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml` (`RESULT-DUPLICATE-MOUNTS-2.md` line 149)
- `PREFIX check -p semio-s-artifact-mathematical-equation --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml` (`RESULT-OS-PACK-DEDUPE.md` line 89; the manifest path is the directory's `Cargo.toml`)

7k. Lowpoly: `PREFIX check -p semio-s-artifact-lowpoly-lowpoly-commands -p semio-s-artifact-lowpoly-lowpoly-editor -p semio-s-artifact-lowpoly-lowpoly-modes -p semio-s-artifact-lowpoly-lowpoly-mut-geometry -p semio-s-artifact-lowpoly-lowpoly-mut-laws -p semio-s-artifact-lowpoly-lowpoly-mut-objects -p semio-s-artifact-lowpoly-lowpoly-mut-paint -p semio-s-artifact-lowpoly-lowpoly-options -p semio-s-artifact-lowpoly-lowpoly-panels --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml` (`RESULT-DUPLICATE-MOUNTS-2.md` line 151)

7l. Reasoning wires: `PREFIX check -p semio-s-artifact-reasoning-wires --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml` (line 152)

7m. Forms: `PREFIX check -p semio-s-artifact-forms-forms --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml` (line 153)

7n. CAD: `PREFIX check -p semio-s-artifact-cad-cad -p semio-s-artifact-cad-cad-panels -p semio-s-artifact-cad-cad-transform --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml` (line 154)

7o. Remodel: `PREFIX check -p semio-s-artifact-remodel-remodeling-mut-calib -p semio-s-artifact-remodel-remodeling-mut-media -p semio-s-artifact-remodel-remodeling-mut-params -p semio-s-artifact-remodel-remodeling-mut-results -p semio-s-artifact-remodel-remodeling-mut-vectors --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml` (`RESULT-DUPLICATE-MOUNTS-2.md` line 155; the same set in `RESULT-WIRING-BUDGET-ROOTS.md` line 138)

7p. Layout and canvas-asset plugins (checklist steps 8 and 9): layout (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml`) and the canvas-asset plugins dag, sequence, flow, forms, reasoning, space and writer. No RESULT records their crate names, so derive them from each manifest's members, then run `--lib --tests`. Flow, forms, reasoning, space and writer are covered by 7i, 7m, 7l, 7c and 7j.

7q. `✏️s` root and dev composition:
- `PREFIX check -p semio-s-spatial-kernel-semio-session --lib --tests --keep-going --message-format=short --manifest-path ✏️s/Cargo.toml`, and `--lib --target wasm32-unknown-unknown` (`RESULT-RUST-FILE-SPLITS-8.md` lines 146 and 147)
- `PREFIX check -p semio-s-composition-laws --tests --keep-going --message-format=short --manifest-path ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml` (line 149)

## 10. Phase 8: Repo tooling and teaching

8a. Repo crate roots (`RESULT-REPO-CRATE-ROOT-SPLITS.md` lines 91 and 94): `PREFIX check -p semio-framework-repo-tree -p semio-framework-repo-test-runner -p semio-framework-repo-hooks -p semio-framework-repo-mcp -p semio-framework-repo-providers -p semio-framework-repo-languages --lib --keep-going --message-format=short`. Then `--tests`, then once with the direct dependents `-p semio-framework-repo-cli -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli-tree-source-and-search -p semio-framework-repo-cli-event-stream -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-tickets -p semio-framework-repo-graphql-fs -p semio-framework-repo-statutes -p semio-framework-repo-codebase -p semio-framework-repo-move`.

8b. Repo statutes and events (`RESULT-RUST-FILE-SPLITS-8.md` line 144): `PREFIX check -p semio-framework-repo-statutes -p semio-framework-repo-events -p semio-framework-repo-metrics -p semio-framework-repo-move -p semio-framework-repo-cli-verbs --lib --tests --keep-going --message-format=short`

8c. Repo model, tickets, server, CLI (`RESULT-REPO-HUB-RUST-FILE-SPLITS.md` line 116): `PREFIX check -p semio-framework-repo-model -p semio-framework-repo-tickets -p semio-framework-server -p semio-framework-repo-cli -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-graphql-fs -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-dashboard-inventory -p teaching-proctor --lib --keep-going --message-format=short`, then `--tests`. Tests: `PREFIX test -p semio-framework-repo-model -p semio-framework-repo-tickets -p semio-framework-server --lib`, and `PREFIX test -p semio-framework-server --test wire --test closed_ports`. **The `teaching-proctor` entry conflicts with 8d; see section 5.**

8d. Pets and quiz (root):
- `PREFIX check -p semio-framework-pets-contract -p semio-framework-pets-clearance --lib --tests --keep-going --message-format=short` (`RESULT-PRODUCT-FILE-SPLITS.md` line 108); then `PREFIX test -p semio-framework-pets-contract -p semio-framework-pets-clearance --lib`
- `PREFIX check -p semio-framework-quiz-schema --lib --tests --keep-going --message-format=short`, and `PREFIX check -p semio-framework-quiz-schema --lib --features testkit --keep-going --message-format=short` (`RESULT-PACK-QUIZ-FILE-SPLITS.md` lines 137 and 138)
- Quiz-schema wasm: `--lib --target wasm32-unknown-unknown` and `--target wasm32-wasip2` (line 141)

8e. Teaching (`RESULT-PRODUCT-FILE-SPLITS.md` lines 109 and 111):
- `PREFIX check --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --tests --keep-going --message-format=short`
- `PREFIX test --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --test end_to_end`

## 11. Phase 9: Hub (`🌎️hub/Cargo.toml`)

9a. Lock: `PREFIX update --workspace --offline --manifest-path 🌎️hub/Cargo.toml` (`RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 145). This must come before every hub call below.

9b. Hub directory (`RESULT-HUB-DIRECTORY-CAPABILITIES.md` lines 146 to 148):
- Port, backends, dispatch, service: `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory -p semio-hub-directory-contract -p semio-hub-directory-sqlite -p semio-hub-directory-postgres -p semio-hub-directory-neo4j -p semio-hub-directory-dispatch -p semio-hub-directory-service --lib --keep-going --message-format=short`, plus the feature lane `--features semio-hub-directory-postgres/testkit,semio-hub-directory-neo4j/testkit`
- Consumers: `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub -p semio-hub-document-routes -p semio-hub-session-routes -p semio-hub-route-models -p semio-hub-ledgers -p semio-hub-inference-runtime -p semio-hub-inference-catalog -p semio-hub-stores -p semio-hub-artifact-creation-service -p semio-hub-local-bootstrap -p semio-hub-lag-rebootstrap --lib --keep-going --message-format=short`
- Tests: `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory -p semio-hub-directory-laws -p semio-hub-directory-backend-laws -p semio-hub-route-testkit -p semio-hub-directory-route-laws -p semio-hub-presence-route-laws -p semio-hub-session-route-laws -p semio-hub-boot-laws -p semio-hub-open-plan-laws -p semio-hub-hostile-input-laws --tests --keep-going --message-format=short`

9c. Hub repo split (`RESULT-REPO-HUB-RUST-FILE-SPLITS.md` lines 117 and 118):
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory-contract -p semio-hub-directory-neo4j-rows -p semio-hub-directory-sqlite -p semio-hub-directory-postgres -p semio-hub-directory-neo4j -p semio-hub-testing -p semio-hub-directory-dispatch -p semio-hub-directory-service --lib --keep-going --message-format=short`, plus the feature lane above
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-directory-laws -p semio-hub-directory-backend-laws -p semio-hub-directory-route-laws --tests --keep-going --message-format=short`

9d. Hub gis, vcs, stdio, trusted catalog:
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-gis -p semio-hub-vcs -p semio-hub-stdio --lib --tests --keep-going --message-format=short` (`RESULT-OS-PACK-DEDUPE.md` line 96)
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-stdio --lib --tests --keep-going --message-format=short` (`RESULT-RUST-CROSS-COUPLINGS.md` line 88; includes the shipped-fleet fixture)
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-trusted-catalog-laws --tests --keep-going --message-format=short` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 122)
- `PREFIX check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-artifact-creation-service --lib --tests` (`RESULT-TAXONOMY-DEBT.md` line 134)

9e. Hub state and whole workspace (`RESULT-W4-HUB.md` lines 122 and 126):
- `PREFIX check --offline -p semio-hub-state --manifest-path 🌎️hub/Cargo.toml`
- `PREFIX check --workspace --all-features --manifest-path 🌎️hub/Cargo.toml`
- Then `PREFIX test --manifest-path 🌎️hub/Cargo.toml -p <each hub laws crate>`, and the bin and lib tests the RESULT names (`cargo test --bin os-hub`, `cargo test --lib`).

9f. Hub flow wasm tests, named only as `semio-…-wasm` in `RESULT-TAXONOMY-DEBT.md` line 134. Confirm the names from the manifest.

## 12. Phase 10: Generators and runtime checks (no cargo)

Run after Phase 9 is green. Use the launch.json entry for each check where one exists (AGENTS.md: devs run through launch.json). Section 13 gives the status of each open checklist item.

10a. Generators, in order:
1. Layout descriptors: `bun repo run workspace:components -- iterate layout`, then `bun repo run @semio-tech/plugin-registry:generate`, then regenerate the distribution `🧾️manifest.json` (checklist step 4).
2. Canvas plugins (dag, flow, forms, reasoning, sequence, space, writer, layout): regenerate descriptors and rebuild components (checklist step 8; the addendum is in `RESULT-W4-CANVAS.md`, not in today's set).
3. Frame-worker bundles: `generate-wgpu` (`RESULT-INTERPRETER-KERNEL-TS.md` section 7 item 2).
4. Schema catalog: `bun ./📜️script.ts schema generate` in the script directory that `RESULT-RENDERER-HOSTS-UI-2.md` section 6 names.
5. GraphQL types: `bun repo run @semio-tech/repo-vscode:generate-graphql-types`, then `…:test-graphql-types` (`RESULT-REPO-TEST-VSCODE-SPLIT.md` line 224).
6. Schema index: `bun repo run @semio-tech/dsl-derive-rs:check-schema-index` (timed out in `RESULT-SMALL-CROSS-EDGES.md` line 74).
7. Workspaces: `bun repo run @semio-tech/repo-lib:workspaces-check --skip-cache` (timed out in `RESULT-PACKAGE-SPECIFIER-CODEMOD.md` line 159; PSC-2 is still in progress).
8. Styling tokens: `bun repo run @semio-tech/ui-styling-tokens:check-generated --skip-cache` (failed with `BUILD_AUTHORITY_CHANGED` in `RESULT-UI-BARREL-IMPORTS.md` line 139; rerun).

10b. Gates named in the RESULTs (each needs its own launch entry or Nx target):
- `test-sdk-surface` (`RESULT-SDK-BOUNDARY.md` line 168), `test-source-text` (`RESULT-SDK-BOUNDARY-2.md` line 155), `test-module-package-surface` (`RESULT-PACKAGE-SPECIFIER-CODEMOD.md` line 123), `test-pack-single-owner-source` (`RESULT-OS-PACK-RECORD-CODEC-DEDUPE.md` line 123)
- `verify taxonomy report`: no `directory-kind-unresolved` finding for folders created since 10-15 (checklist step 6; `RESULT-TAXONOMY-SWEEP-2.md` and `RESULT-SDK-BOUNDARY-2.md` line 137 list the unresolved folders)
- `verify dependencies write-baseline`, only if a new manifest enters the ratchet (checklist step 2; the ratchet was clean at 303)
- `bun install --frozen-lockfile --dry-run` per Bun scope (section 2.4)

10c. TS and Nx suites that were never run:
- `@semio-tech/framework-renderer-react:test -- long --run ../../../../🧪️tests/<suite>/🟦️.ts` for the engine-contract suites (`RESULT-RENDERER-TS-TESTS-SPLIT.md` line 23)
- `@semio-tech/framework-os:test -- quick ../../🔨️modules/🏪️store/👷️worker/🟦️.ts` (launch `⚖️test-worker-law-suites🖥️os🟦️`, `RESULT-MISPLACED-E2E-TESTS.md` line 87)
- `@semio-tech/gis-gismap-js:inference-service-receiving-check` (`RESULT-MISPLACED-E2E-TESTS.md` line 25)
- `@semio-tech/pets-rs:test quick|exhaustive` and `@semio-tech/quiz-schema-rs:test --configuration=prototype --parallel=1 -- --lib --offline` (`RESULT-PRODUCT-FILE-SPLITS.md` line 111; `RESULT-PACK-QUIZ-FILE-SPLITS.md` line 143)
- Browser bundle: `runtime-check`, `pending-host-close-check`, `closed-browser-component-factory-check` (`RESULT-OS-TS-CYCLES.md` line 161)
- `@semio-tech/repo-test:test` and `@semio-tech/ui-react:test`: blocked in Nx (`RESULT-TS-FILE-SPLITS-8.md` line 68; `RESULT-UI-BARREL-IMPORTS.md` line 42). The direct `node node_modules/vitest/vitest.mjs run --config <cfg>` commands in those RESULTs are the fallback.
- `@semio-tech/framework-transient:test-contract --skip-cache`: Nx daemon failure (`RESULT-EPHEMERAL-SINGLE-OWNER.md` line 113). The direct `bun test` in that RESULT is the evidence.

## 13. Post-compile runtime checks still open (from the checklist and the RESULTs)

| Item | Status in the RESULTs | Command or entry | Source |
| --- | --- | --- | --- |
| React boot: `🪢️ElementHosts` registers all 14 scene hosts, no `Cannot access … before initialization` | Not run (browser or GPU boot). The launch entry is not named. | os dev React boot from launch.json | `RESULT-INTERPRETER-KERNEL-TS.md` section 3 and section 7 item 7 |
| Live time-travel e2e | Not run. The run command is `@semio-tech/framework-os-dev:time-travel`, not `puzzle-js:time-travel`. | `bun repo run @semio-tech/framework-os-dev:time-travel -- --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts --serve http://127.0.0.1:6012/ --renderer react`, against a puzzle2d dev serve. Launch entries `⚖️gate⏪️time-travel⚛️react` and `⚖️gate⏪️time-travel🧊️wgpu`. The `--universal` variant is the other route. | `RESULT-SDK-BOUNDARY-2.md` section 3 and line 67; `RESULT-MISPLACED-E2E-TESTS.md` line 129 is superseded (`TimeTravelScript` removed) |
| `wgpu-frame-worker` bundle regeneration | Not run. The bundles embed kernel code and already drifted (six `package-integration` failures). | `generate-wgpu` | `RESULT-INTERPRETER-KERNEL-TS.md` section 7 item 2 |
| OS dev and play boot with wgpu and React renderers; headless layout open; `[TRACE]` font installs; icons render | Not run | launch.json entries | checklist step 7 |
| Canvas plugins: icons and labels render in a plugin guest | Not run | regenerate and rebuild (10a.2), then the guest check | checklist step 8 (`RESULT-W4-CANVAS.md` addendum) |
| Layout: production staging for its 15 text fonts; package-asset staging in the release orchestrator | Open | checklist step 9 | checklist |
| React renderer activation delivers declared package assets | Open | checklist step 5 | `RESULT-W4-UI-RENDER.md` addendum 3 |
| Double mounts: `ui-host` and `ui-backend-webgpu` no longer `#[path]`-mount the abi file | Done statically. Needs the Phase 1 compile and a grep. | Phase 1 (1d) | checklist step 10; `RESULT-SINGLE-MOUNT-AND-GATE.md` section 1 |
| Taxonomy registration of new folders | Open; the unresolved list is in the RESULT | `verify taxonomy report` | checklist step 6; `RESULT-SDK-BOUNDARY-2.md` line 137 |
| Hub shipped-fleet fixture | Not run | 9d | `RESULT-RUST-CROSS-COUPLINGS.md` section 6 |

## 14. Predicted failure classes by family

Source: the residual-risk sections and the "predicted" lines of the 85 RESULTs. "Foreign" marks errors in another owner's files.

| Family | Predicted class | Where predicted | First exposing step | Fix rule |
| --- | --- | --- | --- | --- |
| Hub directory consumers | Trait method not in scope (E0599). About two dozen consumer files call directory methods with no trait import. | `RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 84 and section 8 | 9b, 9d | Add the trait import to the consumer. The `scope_check.py` scan is not rustc. |
| Hub directory coherence | Orphan or coherence error on the blanket `impl<T: SpaceDirectory + AgentDelegationDirectory> PrincipalAuthority for T` | `RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 131 | 9b | Restructure the impl. |
| Tool-run, tool-machine, machine, graph | Private field or method (E0616, E0624) across children. Proc-macro derive expansions (`RetireOwned`, `ToValue`, `FromValue`, `statechart!`) unseen. | `RESULT-TOOL-GRAPH-MACHINE-SPLITS.md` line 173; `RESULT-TOOL-TEST-COMPANIONS.md` line 116 | 1d, 1f | `pub(super)` at the definition, never a qualified path (`RESULT-OS-RUN-VCS-IO-SPLITS.md` line 171; `RESULT-REPO-CRATE-ROOT-SPLITS.md` line 81). |
| Schema, value and math | Missing `pub(super)` on fields and methods (validator `Scope`, `Frag`, `NfaNode`, processors `counts`, `mu`). Missing trait import in a child (`LogitsProcessor`). Ambiguous glob re-exports. | `RESULT-SCHEMA-VALUE-MATH-SPLITS.md` line 138 | 1d, 1f | As above; for trait imports add `use super::{…}` to the child. |
| Glob chains | E0659 ambiguity from `pub use x::*` next to `use super::*` | `RESULT-PLUGIN-WP3.md` line 122; `RESULT-PLUGIN-WP5.md` line 85; `RESULT-TOOL-GRAPH-MACHINE-SPLITS.md` line 173 | 4d, 5a, 7 | Explicit imports. |
| Plugin-host carve | Missing `pub` on a field or method that only another crate now uses. Missing `testkit` forward on a dev edge. Trait import lost in a moved file. Coherence conflict from split `impl` blocks. | `RESULT-PLUGIN-HOST-CARVE.md` line 139 | 4a, 4b | Widen to `pub` or `pub(super)` at the definition. Forward the feature. |
| Renderer WP2 and WP4 | Missed `pub` on an external-typed field. Trait scoping on external types. `unused import` flood. | `RESULT-RENDERER-WP2.md` line 95; `RESULT-RENDERER-INTEGRATION-2.md` line 95 | 6a, 6b | Widen; remove unused imports (warnings only). |
| Renderer WP5 bridges (borrowck) | `self.as_x().m(args)` creates the receiver before the arguments, so an argument borrowing `self` conflicts where `self.m(args)` used two-phase borrows. Boxed futures borrow the bundle; callers must await in the same statement. | `RESULT-RENDERER-WP5.md` lines 19 and 81 | 6a, 6d | Hoist the argument into a block (the generator already hoists one). |
| Plugin WP7 bundles (borrowck, typeck, privacy) | `&mut *self.c` reborrows next to bridged calls. About 55 hoisted arguments (5 engine, about 50 WP6). The async `dispatch_typed` now takes the bundle by value (`mut self`). `MountedOwnerPolicyV1` is Copy. `PhantomData<fn() -> X>`. The `use<..>` capture list. | `RESULT-PLUGIN-WP7.md` line 167 | 5d (after pi1) | Hoist or reborrow explicitly. |
| Plugin WP4, WP6 and PI1 generics | Trait solving of the `ClosePeers` generics. Method-level `A`/`M` inference. Explicit `&mut self.lower` next to arguments. | `RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 3; `RESULT-PLUGIN-WP4.md` line 195; `RESULT-PLUGIN-WP6.md` line 89 | 5a | Fix at the call site. |
| OS kernel engine and WP8 | Borrow splits in composer use-case methods (plan 13.1). Field and method privacy beyond the `widen` pass. Trait scope. | `RESULT-OS-KERNEL-ENGINE.md` line 106; `RESULT-OS-KERNEL-WP8.md` line 128 | 2b, 2d | Split borrows at the composer. |
| OS store and kernel (static) | Unresolved import left by the carve. Eleven `semio_framework_os_store_ephemeral::` imports were fixed statically, and four manifests lost `os-store-ephemeral`. | `RESULT-OS-KERNEL-WP2-3.md` line 81 | 1c, 2d | Add the dependency or import. |
| Feature-gated re-exports | `deflate` gate lost on facade `pub use` (fixed, unverified). `testkit`, `sync` (directory client), `artifact-app-testing`, `record-codec-laws`, hub postgres and neo4j `testkit` lanes. | `RESULT-COMPILE-PASS-2.md` section 3; `RESULT-OS-SERVICES-MCP-SPLITS.md` line 127; `RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 2; `RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 146 | 1b, 3c, 5a, 9b | Add the feature forward in the consumer. |
| Cross-carve facade misses (E0432, E0425) | `factory` path (`semio_framework_value::factory`); `RetainedPackSymbolSpan` import; `RetainedWireByteSource` moved to `replication-wire-core`. Foreign: `UI_NUMBER_PRECISION_MAX` in `ui-contract-presentation` blocks its dependents. | `RESULT-COMPILE-PASS-2.md` sections 3 and 7 | 1b, 1c | Fix the import; foreign goes to the ui-contract owner. |
| Wasm-gated code | `cfg(wasm32)` and wasip2 bodies never compiled natively. `wgpu 29` `BufferView` has no lifetime parameter. The `estimate_only` arm exists only on wasm32 and p2. `dead_code` expected on wasip2. | `RESULT-COMPILE-PASS-1.md` section 3; `RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 3 | 1b, 1e, 5a | Use the target-specific check. |
| Lock entries | `--locked` refuses (`cannot update the lock file`). Missing edges (154 locks lack the plugin edge, 136 lack `reactor-context`). Hand-written entries (`tool-run`, `tool-machine`). Plugin-host carve crates absent from every lock. 40 stale renderer packages. | `RESULT-COMPILE-PASS-1.md` section 1.2; `RESULT-SINGLE-MOUNT-AND-GATE.md` line 115; `RESULT-DUPLICATE-MOUNTS-2.md` lines 163 and 67; `RESULT-TOOL-TEST-COMPANIONS.md` line 66; `RESULT-RENDERER-TEST-DEPS.md` section 7.3 | 2.4, every phase's first call | Refresh the lock first (2.4). |
| Include and `#[path]` literals | About 20 `include_str!` with 8 or 9 `../` into moved directories. `#[path]` inside an included file resolves against the included file's folder. | `RESULT-PLUGIN-INTEGRATION-1.md` section 8 item 4; `RESULT-PLUGIN-WP3.md` line 122 | 5c | Re-point the literal. |
| Source gate (not a compiler error) | `--all-targets` refused for any crate with the oversized dev-dependency `semio-framework-replication`. | `RESULT-COMPILE-PASS-1.md` sections 1.3 and 4 | any `--all-targets` call | Use `--lib` and `--tests` separately. |
| Stub-only oracle blind spots | Method calls on stub-typed receivers are invisible. Privacy is checked by token heuristics. The oracle cannot see trait-method resolution, coherence, or derive output. | `RESULT-HUB-DIRECTORY-CAPABILITIES.md` line 131; `RESULT-PRIVACY-ORACLE-SWEEP.md` line 11 | all Rust phases | Trust the compiler, not the oracle. |
| Warnings | `unused_imports`, `unused_qualifications`, `private_interfaces`. Warnings only. | `RESULT-COMPILE-PASS-1.md` section 2; `RESULT-FRAMEWORK-RUST-FILE-SPLITS.md` line 109 | all | No action unless a lint is denied. |

## 15. Discrepancies and decisions to make before the window

1. **Teaching membership.** `RESULT-PRODUCT-FILE-SPLITS.md` lines 109 and 111 use `🎓️teaching/Cargo.toml` as its manifest. `RESULT-REPO-HUB-RUST-FILE-SPLITS.md` line 116 puts `teaching-proctor` in the root workspace. The lock refresh (L10) and Phase 8c need one answer.
2. **Command form.** `RESULT-PLUGIN-INTEGRATION-1.md` section 8 quotes `bun ./…/📜️script.ts repo cargo`. `WAVE5-BRIEF.md` requires `bun repo cargo` under the PREFIX. This runbook uses the PREFIX.
3. **Time-travel run command.** `RESULT-MISPLACED-E2E-TESTS.md` line 129 gives `@semio-tech/puzzle-js:time-travel`. `RESULT-SDK-BOUNDARY-2.md` (later, 11:26) removes that target and gives the framework command. Use the later one.
4. **Framework companion list.** `RESULT-FRAMEWORK-TEST-COMPANIONS.md` line 72 has no `--manifest-path`. This runbook adds it (1d).
5. **Abbreviated crate names.** Several RESULTs abbreviate names with `…` or describe them by count: the seven SQLite-host crates, the 12 WP0 crates, the 14 WP8 test crates, the 12 stdio contract crates, the RUN-VCS dependents, `…-asset-authority`, and the layout, dag and sequence crates. The name lists are in the named files. This runbook does not guess them.
6. **Freshness.** `RESULT-PACKAGE-SPECIFIER-CODEMOD-2.md` (17:50, in progress) and `RESULT-PLUGIN-GATES-GREEN.md` (18:27) may still move lock and manifest files. Re-read both before the window.
7. **B1 owner choice.** Option (a) or (b) in section 2.1 is Root's decision, not this runbook's.

## 16. Counts

- Phases: Phase 0 (prerequisites, section 2), then Phases 1 to 10. Phases 1 to 9 are launcher phases (sections 3 to 11). Phase 10 (section 12) is generators and runtime checks with no cargo.
- Launcher invocations (lines that run `PREFIX check|test|update|metadata` or `waitcc.py`): 113. Some lines also carry a second invocation in prose ("then the same with `--tests`"), so the true count is slightly higher.
- Phase 10 also lists about 20 TS, Nx and generator commands in sections 12 and 13. Those are not counted above.
- Verification of this audit: `/usr/bin/grep` only; no compile, no git write, no source or RESULT edit.

## 17. Coordinator resolutions (18:4x)

1. Teaching membership: `teaching-proctor` (`🛂️proctor/📦️packages/🦀️rust`) is a member of the standalone workspace `🎓️teaching/Cargo.toml` (`[workspace] members`). The root `Cargo.toml` does not list it. Use `--manifest-path 🎓️teaching/Cargo.toml` for L10 and Phase 8c.
2. Command form: use the PREFIX `bun repo cargo …` form throughout, as the runbook does.
3. Time-travel: use the later command from RESULT-SDK-BOUNDARY-2.md, the framework `@semio-tech/framework-os-dev:time-travel -- --scenario <module>`.
4. Abbreviated crate names: take them from the named RESULT files at window time. Do not guess.
