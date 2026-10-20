# Status 2026-10-20: Get Codebase Running End To End Again

Read-only consolidation by `status-1020` (auditor). No builds, no `bun repo` runs, no tests, no git commands. Only this file was written. Snapshot about 10:35 CEST, 2026-10-20. Successor of `status-1019.md`.

## 0. Inputs, clocks, legend

- Read: `brief-1019.md`, `HANDOVER-1020.md`, `status-1019.md`, and `coordination.md` from "12:40 (10/19, new session)" to the last entry (08:00). Every `result-*.md` and `audit-*.md` in this ticket with a 10/19 or 10/20 mtime: 142 results and 26 audits. Each file was read for title, headings and status or verdict lines. The files that decide status were read in full or near-full: section 0.1.
- 0.1 Read in full or near-full: `coordination.md` (12:40 to 08:00), `HANDOVER-1020.md`, `status-1019.md`, `audit-staged-1020.md`, `audit-verify-1020a.md`, `audit-census10-1020.md` (headline), `audit-verify-1019h.md`, `audit-verify-1019d.md`, `audit-verify-1019e.md`, `audit-verify-1019f.md`, `audit-verify-1019i.md`, `audit-verify-1019b.md` (verdict tables), `result-stage-fix.md` (verdict table), `result-tst7.md`, `result-tst6.md`, `result-bzl-real.md`, `result-bzl-breadth.md` (headline table), `result-law-drift.md` (summary and open items), `result-exports-stage.md` (verdict), `result-tax4.md`, `result-tax5.md`, `result-vit3.md`, `result-tsc5.md`, `result-lay17.md` (headline tables).
- 0.2 Live checks, read-only, 10:32: `bun repo show projects` is still running (pid 88833, elapsed 3 h 56 min, 93 % CPU on one core). `🗑️generated/chain/chain9.log` is 0 bytes. The live Bazel adapter `🏗️build/📜️script.ts` line 409 still writes `.bazelignore`, so bzl-real defect 1 is not applied. The live `🏗️build/🧾️store-receipt/🟨️.mjs` defines `declaredStoreReceiptV1` but is not identical to `C/bzl-real-tools/staged/store-receipt.mjs`. The `semio` MCP server reports CONNECTION_CLOSED in this session.
- Paths: `C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `B` = `L/🏗️build/🔧️tools/🏗️bazel`. `SS` = `✏️s`.
- Legend. VERIFIED-RUN: a command ran and its outcome is recorded (report or coordinator log, named). STATIC-ONLY: reading, schema or Ajv validation, lint, parse, census or static checker. STAGED: a patch exists under `C/*-tools/` and is not in the live tree. OPEN: unfinished or known defect. BLOCKED: waits on a named item. A status is never upgraded from its evidence.
- Gating state: WRITE PAUSE declared 06:35 holds until `show projects` and `tools setup nx|bazel` are green. The dependency sync is green (05:43).

## 1. Status by goal area

### 1.1 Tool-neutral layer (dispatch, `--tool`, selection, operation catalogue, registry)

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Neutral operations and selection unit parity | VERIFIED-RUN (unit) | `result-cli-fix.md`: `bun test ./…/⚙️operations/🧪️tests/🟦️.ts` 10 pass; `…/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🧪️tests/🟦️.ts` 7 pass; `…/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts -t "exec plans"` 1 pass | none for unit level |
| CLI route for `cache status|clear`, `affected --dry-run`, `exec --dry-run` | OPEN; never run through `bun repo` | `result-bzl-ops.md` "Not run"; `result-nx-bound.md` §5 | run after `show projects` and setup (section 4, step 4) |
| Per-command help (EN/DE from catalogue) | STATIC-ONLY | `result-cli-fix.md` | `bun repo <cmd> --help` after build |
| Dispatch selects loop image on Linux | OPEN | `result-docs-build.md` open item 1; `audit-verify-1019d.md` xplat-linux FAIL | route to directory backend (section 1.5) |

### 1.2 nx implementation

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| nx SDK boundary and probe | VERIFIED-RUN (unit, 10/19) | `result-nx-bound.md`: `bun test ./NX/🔌️sdk/🧪️tests/🟦️.ts` 4 pass; `…/NX/🔁️probe/🧪️tests/🟦️.ts` 3 pass | none |
| nx task cache demonstrated (artifacts populate, second run hits) | OPEN; never run | `verify-nx-cache.ts` not run; `tools setup nx` not run (`result-deps4`, coordination 17:30) | `C/nx-cache-tools/verify-nx-cache.ts` after show and setup (step 5) |
| Assembled nx inputs (`namedInputs.nativeSources`, `reads`, `testEnv`, `cargoToolchain`, native explicit inputs) | STATIC-ONLY | `result-cache-s.md`, `result-cache-fw.md`, `result-cache-rest.md`; `result-inputs-closure.md` (applied 05:22 before lock, not observed) | graph load to observe the assembled inputs |
| nx.json:91 | VERIFIED-RUN (read) | `result-nx-bound.md` §1: the rust-partition contract, not a dead path | none |
| `bun repo run` in `package.json` (course, fem) | DECIDED | AGENTS amended by user (`fd14da55f18`, 01:24); coordination 06:00 | fixtures that pin the old AGENTS text need updating |

### 1.3 Bazel implementation (incl. real-run evidence)

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Real Bazel 8.4.2 runs, production entry | OPEN; never run through `bun repo --tool=bazel run` | `result-bzl-real.md` "Harness": `runBazelTasksV1` blocked by the storage lease held by the sync; all evidence from a scratch host | production run (section 4, step 6) |
| Real Bazel runs, scratch host (`bun C/bzl-real-tools/go.sh <label> <tasks>`) | VERIFIED-RUN (scratch harness) | `result-bzl-real.md` evidence table: T1, T2, T3 cold 28.4 s, 4 processes, 3 local; warm 1.2 s, 100 % action-cache hits; touch of declared T2 input re-runs only T2 (15.9 s), dependent T1 is a hit; touch of declared T1 input re-runs only T1 (22.1 s); revert 1.3 s, disk-cache hit; `bazel clean` plus rebuild 1.7 s, 3 disk-cache hits | confirm with production entry; sandboxing not exercised (every spawn `runner=local`) |
| Undeclared-input gap | VERIFIED-RUN (scratch, measured) | `result-bzl-real.md` Q7, Q8: touching an undeclared import gives no rebuild; T1 106 of 111 modules and T2 162 of 169 modules are undeclared | `result-inputs-closure.md` staged fix applied 05:22; re-measure in Bazel (STATIC-ONLY until then) |
| `.bazelignore` disables all caching | VERIFIED-RUN (scratch); live defect OPEN | `result-bzl-real.md` defect 1 (fix: stop writing `.bazelignore`, query `//:all`). Live `B/📜️script.ts:409` still writes it (grep, 10:32) | apply the fix after review; `audit-verify-1020a.md`: the staged bzl-real set is NOT SAFE TO APPLY AS STAGED |
| Emoji target names: 108 of 8,021 targets die `Closed store receipt target required` | VERIFIED-RUN (scratch); fix STAGED | `result-bzl-real.md` defect 2; `C/bzl-real-tools/staged/store-receipt.mjs` 6 pass. Live `🏗️build/🧾️store-receipt/🟨️.mjs` differs from the staged copy | live fix not verified (no run); reconcile staged copy with live file |
| All task rules generated; `bazel build --nobuild //...` | VERIFIED-RUN (scratch workspace) | `result-bzl-breadth.md`: 8,010 task rules plus 859 filegroups = 8,869 targets; `bazel build --nobuild //...` analysed 8,869 targets, 0 errors, 27 s. `bazel query //...` fails (infinite symlink expansion) while the workspace lives inside the checkout | removing `.bazelignore` requires query `//:all`; affected and exec parity depend on it |
| Adapter fail-closed on missing `cache` | VERIFIED-RUN (read) | `audit-verify-1019b.md` confirms `B/📜️script.ts:178` | 624 targets still have no explicit `cache` key (status-1019 row 3) |
| darwin sandbox (double-encoded output base) | VERIFIED-RUN (scratch) | `result-bzl-sandbox.md`; `bun test ./B/💾️storage/🧪️tests/🟦️.ts` 7 pass, 2 skip (`audit-verify-1019c`) | Linux and Windows not tested |
| Disk cache, GC bound, remote switch | VERIFIED-RUN (flags, scratch GC); bound and remote OPEN | `result-bzl-cache.md` F1 to F9 | GC bound on the real 9,830 MiB partition; remote cache against a server; `cache clear` from CLI |
| Contract suite and adapter unit suite | VERIFIED-RUN (unit); runtime phases OPEN | `result-bzl-suite.md`, `audit-verify-1019f.md` PASS with doc gap (12 contract digests recompute ok); `bun C/bzl-act-tools/wrap.ts ./B/🧪️tests/🟦️.ts` 70 pass, 11 skip (10/19) | `bun repo test bazel` aggregate; runtime phases cache, nested, uncached, birth, services, refusals, cancellation, configuration, offline |
| Zero-touch `tools setup bazel` | STATIC-ONLY | `result-bzl-zt.md` "NOT RUN" | run after show (step 1), and it opens the 24 GiB products image |
| Dry runs and cold and warm runs for the 8 requested targets | OPEN | `audit-bazel-state-1019.md` WP6 | step 6 |

### 1.4 Cache coverage and receipts

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Manifest validity (framework 259, `SS` 325, rest 123) | STATIC-ONLY (schema and Ajv, 0 errors) | `result-cache-fw.md`, `result-cache-s.md`, `result-cache-rest.md` | none |
| Deterministic but cache-off: fw 794 to 50, `SS` 307 to 9, rest 77 to 24 | STATIC-ONLY (census) | the three cache reports (10/19) | observe in graph load |
| Stale cache hit in `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📋️project.json` lines 20-28 and 37-45 | OPEN | `audit-verify-1019b.md`, cache-rest ISSUES; no later report found | fix and re-check |
| Store receipts (217 targets; `🏗️build/🧾️store-receipt`) | VERIFIED-RUN (unit 14 pass); runtime repair path OPEN | `result-store-out.md`; store-receipt unit tests 14 pass (`status-1019.md` row 9) | `@teaching/proctor:build` twice, delete compiler slot, rebuild; expect one `[store-receipt] re-executing` (BLOCKED on cargo) |
| Plugin cache decisions (`metadata.semio.cache`) | OPEN (report stale) | `audit-verify-1019c.md` §3: `result-plugin-cache.md` sections 0, 2, 3 say artifact build and wasm stay off; live code replays them through store-receipt | reconcile the report with the receipt design |
| Generation receipt for the dependency walk (walkperf) | APPLIED 05:05; soundness OPEN | `audit-verify-1019h.md` §5.1: `implementationFiles` covers 10 files of a ~156-file walk closure; NOT sound as staged | fence repair before trusting the receipt (section 4, step 3) |

### 1.5 Cross-platform: macOS, Linux, Windows, devcontainer, CI

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| macOS: Bazel pieces (sandbox, disk cache) | VERIFIED-RUN (scratch) | rows 1.3 above | fresh-clone bootstrap never run; Xcode CLT is manual |
| macOS: devcontainer image build | VERIFIED-RUN | `result-xplat-boot.md`: `perl -e 'alarm 590; exec @ARGV' -- docker build …` exit 0; `docker run` shows `en_US.utf8` | none |
| Linux: unprivileged directory partitions | OPEN; `audit-verify-1019d.md` FAIL (2 defects) | `result-xplat-linux.md` (directory suite 12 of 12 pass); verify: capacity accounting can be bypassed, one tracked test still red on the deleted loop observer | fix both defects; dispatch directory backend |
| Linux: devcontainer | OPEN | `.devcontainer/docker-compose.yml` `privileged` removed (`audit-verify-1019d.md` PASS, partial); `devcontainer up` and postCreate never run | section 4, step 15 |
| Windows: directory partitions with journaled swap | STATIC-ONLY (journal test VERIFIED-RUN on macOS) | `result-xplat-win2.md`; `audit-verify-1019f.md`: journal test 10 pass alone, four swap states covered; VHD and elevation deleted (coordination 20:00) | no Windows execution; `pwsh` not installed on this Mac (`result-xplat-boot.md`) |
| Windows: `NAT/🔵️.ps1` parse, FFI junction swap | BLOCKED (environment) | `result-xplat-boot.md` "Not run" | Windows host with pwsh, elevated shell |
| CI `.github/workflows/build-system.yml` | STATIC-ONLY (lint VERIFIED-RUN) | `result-xplat-ci.md`: actionlint v1.7.7 exit 0; action SHAs and Bun SHA-256 pins match upstream (`audit-verify-1019b.md`) | first run needs a push (decision D14) |
| Zero-touch closure law | VERIFIED-RUN | `result-zt-law.md`; `audit-verify-1019e.md`: `bun test` 22 pass | `bun repo --help` with `node_modules` hidden not run |

### 1.6 Dependency sync performance

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| `dependencies sync --all` green | VERIFIED-RUN (coordinator log; no report file) | coordination 05:43: "SYNC GREEN in 31 min"; generation `70825028305173c7f252`, including the exports-stage manifest batch. Earlier green at 22:00 (generation `9e8ff8823758…`) | none for the sync itself |
| Sync speed: scan 302 s to 48-64 s, est. full sync 2.8 h to 30-40 min | VERIFIED-RUN (microbench, deps5) | `result-deps5.md` §1, `bench/scan.bench.test.ts` at load 31-52; measured 31 min end to end (05:43) | none |
| deps5 applied live | APPLIED 05:15 | `result-deps5b.md`; `apply.sh apply --rebase` (no sync running) | fence test relaxed in deps5b |
| lease-fair applied 05:05 | APPLIED; overlay-verified only | `result-lease-fair.md` §5 (scratch overlay); `audit-staged-1020.md` item: no sync guard, 3 `[DEBUG]` lines in the staged set | run `test-resource-lease-fairness` through `bun repo run`; check `[DEBUG]` |
| walkperf applied 05:05 | APPLIED; soundness OPEN | `audit-verify-1019h.md` §5.1 | repair fence coverage (~156 files) |
| Reader starvation by a peer `bun repo run` holding `native-fixed-image` for ~3 h | OPEN (owner decision on writer preference) | coordination 02:50; `result-lease-fair.md` §1 | decision D20 |
| Sync attempts: 50 (7000 s), 51 (rc 142), 60 (rc 1, peer edit mid-walk), 31 (provider-instance defect) | OPEN as history; defects closed by the green sync | coordination 20:45, 04:50 | keep the write pause during any sync (about 40 min with deps5) |
| `show projects` after the green sync | OPEN; BLOCKED | pid 88833 at 10:32: 3 h 56 min, 93 % CPU, `chain9.log` 0 bytes; cold baseline 7 to 9 min (HANDOVER) | section 4, step 0 |

### 1.7 Rust compile, crate gate, resolution sweeps

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Compiled crates since 10/16 | VERIFIED-RUN (few crates) | `result-cc2.md` probe 13:30: `cargo check -p semio-framework-hash --lib` GREEN; coordination 18:40: `cargo check` green ×5 (roster); `result-value-fix.md`: value retained-map lib 30 to 0 errors | the list of crates that have not been compiled (below) |
| Value family (`semio-framework-value*`) | OPEN; BLOCKED | `result-value-fix.md` §5: all other crates not compiled; proctor check not run; `audit-verify-1019h.md` | cargo window, value family first, in groups of up to 8 |
| Facade re-exports removed (`semio-framework-value`) | STATIC-ONLY (resolution, 2,417 refs, 40 of 40 sample) | `result-value-repoint.md` (1,025 files); `audit-verify-1019h.md` §1 | compile; doc section 1 stale; "49 workspace deps" not reproduced |
| Rust resolution sweeps | STATIC-ONLY | `result-rs-sweep.md` 11,501 to 1,622 defects; `result-rs-sweep2.md` fw and hub 1,621 to 528; `result-rs-sweep4.md` 433 to 148 (checker s1 to s4); `audit-verify-1019g.md` FAIL on visibility widenings and `_cold` renames (partly corrected by rs-sweep2 §0) | compile; re-check visibility and rename edits |
| Crate source gate (census10, working tree, 02:15) | STATIC-ONLY (census) | `audit-census10-1020.md` §1-2: 3,468 targets; prod 0 over 1.00, 0 refused; test 1 over (`semio-framework-actor` 1.073); 4 prod and 135 test in the 0.90 to 1.00 band | `result-fixups6.md` §1: actor test cfg 1.073 to 0.781 (applied, not compiled); re-run census |
| Crate splits (rg-core, rg-plug, rg-fem3d, rg-rest, rg-test, rg-refused, split-s, split-fw) | STATIC-ONLY; cargo not run to completion | `audit-verify-1019d.md`, `audit-verify-1019e.md`, `audit-verify-1019f.md` (rg-test PASS, rg-rest PASS with risk: 3 `os` crates not in `🧰️framework/Cargo.toml`) | compile in the cargo window |
| Workspace resolution (291 `[workspace]` manifests) | VERIFIED-RUN for 31 (10/19); lock refresh OPEN | `result-ws-resolve.md`; `result-lockref.md`: 1 of 291 verified (21:40, blocked by sync lease, now lifted); `lockref-tools/all.sh` detached | run `lockref-tools/all.sh` detached (step 8) |
| `cargo` admission (pressure) | APPLIED 05:05; post-apply verify failure OPEN | `result-cargo-pressure.md`: admission recheck throttle 15 s, 12.3 s to 3.0 s per unit (bench); coordination 05:05 "cargo-pressure admission read" failure | fix the admission read and re-run |
| Path budget (repo-relative 256 UTF-16 units; components 255 bytes) | STATIC-ONLY (pathcheck10, 186,361 entries) | `audit-census10-1020.md` §1: 0 paths over; 0 components over 255 UTF-8 bytes | none |

### 1.8 Files and functions untangling

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Hand-authored files over 1,000 lines | STATIC-ONLY (census) | census8 (10/19 HEAD): 17 files; `audit-census10-1020.md` §1: 1 (working tree; 3 generated) | the 1 new file |
| Functions over 600 lines (outermost) | STATIC-ONLY (census) | `audit-census10-1020.md` §1: 4 (from 13 at 10/19); 279 over 300 (from 297) | the 4 remaining |
| Nine functions over 600 restructured | VERIFIED-RUN (type differential, reachability, rustfmt parse) and partly vitest | `result-fnsplit3.md`: `T2/tsdiff.cjs` 0 diagnostics orig and new for host-effects, shell-actions, backbone-worker, gis worker, plugin-module-loader, two-client; `T/reach.ts` all children reachable; `T/vt.sh` ShellHost differences 0 | Rust resolution oracle and compile |
| Split of 8 `SS` files and 8 framework files (+1 duplicate deleted) | VERIFIED-RUN for femu and fem3 (line-multiset oracle), NOT VERIFIABLE for pz5e | `audit-verify-1019d.md` split-s PASS with exception | pz5e tree moved after the split |
| Comment strip in bodies (`comments-a` 9,511 lines; `comments-b` 10,232 rows) | APPLIED live; restored docstrings VERIFIED (22 of 22 declarations) | `result-comments-a.md`, `result-comments-b.md`, `result-fixups5.md`; `audit-verify-1020a.md` (docstrings) | decision D7 (`// 🚫️async:` notes) |
| Broken anchor test pairs A, B, C after comment strip | OPEN (fix claimed) | `audit-verify-1019i.md` (3 broken pairs); `result-fixups5.md` anchor tests (not re-verified) | run the three tests |
| Emoji-led docstrings (5,027) | STATIC-ONLY | `result-emoji-docs.md` (4 frozen library files touched by mistake, comment-only) | none |

### 1.9 Layering

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Per-area dependency-cruiser re-measure | STATIC-ONLY (cruise counts not re-run) | `result-lay13.md`: per area 3,179 to 2,126; `result-lay16.md` static evaluator `lay16-tools/scan.ts` | re-measure with the cruiser when load allows |
| Renderer boundary (`elements` export allow-list) | VERIFIED-RUN (read, confirmed) | `result-lay14.md`; `audit-verify-1019h.md` §4: allow-list is exactly 102 `elements` export targets | none |
| Cycles: 27 to 1 bundle; no-core-path 0 | VERIFIED-RUN (vitest actor-transport 179 of 179 after the cycle fixes; 176 of 179 at the end of the report) | `result-lay15.md`; coordination 20:00 | re-cruise |
| `no-cross-package-relative` 1,062 to 22 | STATIC-ONLY (after exports-stage batch and importer rewrite) | `result-exports-stage.md` §0; batch applied 05:05, sync green 05:43 | re-cruise; 3 blocked export entries (energy-js, framework oracle) |
| Rule deltas `plugins-framework-sdk-only` 17 to 0; `renderer-hosts-only-ui` 41 to 0; `plugin-no-extension-or-artifact` 16 to 4 | STAGED (`lay17-tools`), predicted in scratch mirror | `result-lay17.md` §1, `apply-delta.ts` | apply after the next lock and sync |
| Cycle-break direct parts (92 files) | APPLIED before the freeze | `result-cycle-break.md` §3.1 | none |
| Cycle-break manifest delta (471 ops, including `package.json`) | STAGED | `result-cycle-break.md`; `C/cycle-break-delta.json` | lock and sync under pause |

### 1.10 Taxonomy

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Implementation findings 58,013 to 151 | STATIC-ONLY (whole-tree validation, 10/19) | `result-tax.md` | none for this count |
| Router law 496 to 157; leaf errors 151 to 66 | STATIC-ONLY | `result-tax2.md` | `audit-verify-1019f.md` FAIL on tax2: at least six dangling path references (F2) |
| No-cross-technology 431 to 33; ui-foundation 2 | STATIC-ONLY | `result-tax2b.md` | 33 edges |
| Path policy, 820 empty directories removed, `SS` findings 22,393 to 1,944 | APPLIED (apply-1..4 at 01:40); 820 dirs verified clean | `result-tax3.md`; `audit-verify-1019h.md` §3 | one consumer still uses removed `maxPathBytes` (silent check loss); 182 vs 74 moves unreconciled; 18 vs 21 catalogs |
| `SS` findings 1,946 to 1,528 live (1,119 after staged patch) | PARTLY APPLIED; staged part STAGED | `result-tax4.md` §0 and §2 | apply `C/tax4-tools/staged` |
| Leaf case tables as fixture coverage source | STAGED (`tax5-tools/staged`), not applied | `result-tax5.md` §0: 66 `🔣️.cases.json` produce `semantic-stem-unresolved` on the live tree until applied | apply; re-run taxonomy validation |
| `verify taxonomy report` and whole-tree `enforce` | OPEN; not run | `result-tax.md`, `result-tax2b.md` "Not run" | run after show and setup |

### 1.11 TypeScript, vitest, tsc and Go tests with the tst7 baseline

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Baseline bun suites (tst7, authoritative census) | VERIFIED-RUN (census); classification NOT reliable | `result-tst7.md`: 2,145 suites, 1,764 green, 381 red; 18,595 pass, 1,485 fail; run as `bun test --timeout 120000 ./<file>`. `audit-verify-1020a.md`: about 10 of 33 "real-defect" rows are not source defects; 11 envelope-gated suites hide ReferenceError or ENOENT | re-label the classes before using them (section 4, step 18) |
| bun suites after real install (tst6) | VERIFIED-RUN | `result-tst6.md`: 2,140 suites, 1,756 green; 18,647 pass, 1,456 fail; repo 337 of 576, framework 534 of 624, OS 322 of 345, `SS` 542 of 570 | 1,456 failing tests; deps gaps in `tst6-tools/gaps-*.json`; `pack json` owner decision |
| law-drift (117 suites) | VERIFIED-RUN (run 6); partly APPLIED | `result-law-drift.md`: 54 green live; 22 of 24 staged patches applied; `cc-project` and `cc-test` skipped (frozen `project.json`). Handover lists `jsonattr.ts --apply` and `ALLOW_PROJECT=1 apply.sh` as queued | confirm state with dry run; owners per the report's open items 2 |
| lib-defects (17 files, 30 edits) | STAGED, not applied | `result-lib-defects.md`; `C/lib-defects-tools/` | apply after the lock and sync window |
| vitest, 60 configs | VERIFIED-RUN (42 green after fixes; 18 red) | `result-vit3.md`: course GREEN 16 / 552 (as reported); teaching architecture course 2 failed, 11 passed; OS dev 5 failed | 18 red configs |
| tsc, 30 project configs | VERIFIED-RUN (22 of 30 green); root 1,908 to 455 | `result-tsc5.md`; coordination 00:30 | 8 red configs; 48 errors inside frozen `L` (repo); staged `tsc-stage` applied 05:05 |
| Go modules | VERIFIED-RUN | coordination 19:00: 32 of 32 modules green; 20:00: exhaustive green ×32; `result-gofix7.md`: repo-mcp 26 PASS, `go vet` exit 0; `result-go-all2.md` PASS with flake (5 s timeout under load) | none beyond flake |
| Playwright e2e enumeration (course) | VERIFIED-RUN (list only) | `result-e2e-recheck.md`: `--list` 76 tests in 16 files | run the tests (browsers missing, decision D15) |
| Known defect suites | OPEN | `result-tst7.md` real-defect class: 33 suites, 24 in `L`; `result-tst5.md`, `result-tst6.md` | owner work after decisions D1 to D5 |

### 1.12 MCP servers (`semio`, `repo`)

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| `semio` gateway handshake | VERIFIED-RUN (test and driver with PATH shim) | `result-semio-mcp.md`: `bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🚪️gateway/🧪️tests/🟦️.ts` 14 pass; `bun semio-mcp-tools/drive.ts semio …` answered initialize in about 1.5 s (was timeout) | real `semio-os-mcp` binary never started; `semio` tools empty until `@semio-tech/framework-os-mcp-rs:build` (cargo) |
| `semio` Rust side | OPEN; not compiled | `result-semio-mcp.md`, `result-fixups.md`: `CallToolResult::wire_structured_content` parsed only | cargo window |
| `repo` MCP Go module | VERIFIED-RUN | `result-gofix7.md`: 26 PASS; `go vet` exit 0 | none |
| `repo` MCP binary and session | OPEN | status-1019 row 21: the cached binary is the Oct-18 build | `bun repo run repo-mcp:build`, restart session, `drive.ts repo` (step 12) |
| Unit test `…/🔌️mcp/🧪️tests/🔬️unit/🟦️.ts` | VERIFIED-RUN (22 pass; status-1019 re-run) | status-1019 row 21 | none |
| Session note | OBSERVED | `semio` MCP reported CONNECTION_CLOSED to this session | restart the session after the build |

### 1.13 Command registry and CLI help

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Registry reaches every executable command (G2 to G5) | VERIFIED-RUN (partial) | `result-cmdreg.md`: `bun test ./R/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts` 9 pass, 1 fail (fixture row `:workflow status`); `bun test ./R/🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts` with stale binary override 6 pass, 5 fail; course dashboard 4 pass; play fresh-build vitest 10 pass | fix the fixture row; rebuild dashboard; rerun registry oracle without override |
| `semio commands --check`, `semio run --dry-run` | OPEN; not run | `result-cmdreg.md` "Not run" | step 13 |
| `bun repo --help` prints | VERIFIED-RUN (10/19) | `result-docs-build.md` | re-run after build |
| Build-owner registry check | VERIFIED-RUN | `result-owners-gate.md`; `verify build-owners` passed (coordination 03:25; `result-own-crash.md`) | `verify build-owners sync` is OPEN: it removes rows with `nativeRoots` and rows whose paths are absent (`audit-verify-1019c.md` §4, data-loss risk) |
| Preflight scope | OPEN | `result-owners.md`: `bun repo preflight --scope @teaching/architecture-course` not run | step 14 |
| Launch surface (`.vscode`) | DECIDED (AGENTS amended); fixtures OPEN | coordination 06:00; `.vscode` deleted in `bd9e4e02bd0` (status-1019 row 23) | update fixtures pinning `.vscode` and the old AGENTS text |

### 1.14 Documentation

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Build chapter (EN and DE), command-registry chapter, Bazel "verified and not yet verified" table | STATIC-ONLY (statements tagged) | `result-docs-build.md`, `result-docs-build2.md`; `audit-verify-1019e.md`: docs-build2 mostly holds, 2 to 3 statements unconfirmed | Linux rows after dispatch switch |
| Build docs against call sites | STATIC-ONLY (56 statements checked against call sites) | `result-docs-fix.md`; `audit-verify-1019i.md`: docs-products 10 of 10 PASS | none |
| Doc-law tests | OPEN (status-1019 numbers, not re-run) | owner-projection-policy 33 pass, 2 fail; testing-readme-coordinates 11 pass, 3 fail; build-surfaces 15 pass, 6 fail | re-pin `🧪️test/README.md`; remove `[DEBUG]` in `🏗️build/🧭️invocation/🧪️tests/🟦️.ts` |
| Course and proctor READMEs (EN, DE) | STATIC-ONLY | `result-docs-products.md` | none |

### 1.15 Products

| Item | Status | Evidence (report; command) | Remaining work |
|---|---|---|---|
| Course `@teaching/architecture-course` | BLOCKED (cargo for proctor; Playwright; activation) | `result-e2e-prep.md` §2: `bun build` of router, e2e gate, stack, deploy, learner 0 errors; `result-e2e-recheck.md`: Vite resolves (runner and native, build and serve) and `--list` 76 tests in 16 files; `result-course-img.md`: runtime-only image built and healthy on amd64 with stand-in proctor | proctor cargo build; `run-e2e.sh course --topology dev --warm-proctor`; `docker-image-check`, `docker-stack-check` not run; `site.legal` empty (decision D16) |
| Proctor `@teaching/proctor` | STATIC-ONLY (manifest, receipt, cache flags); BLOCKED on cargo | `result-bzl-man.md`, `result-store-out.md` | receipt repair run (row 1.4), cargo build |
| Play `@semio-tech/semio-tech-play` (:6033) | BLOCKED (cargo; activation not run) | `result-e2e-recheck.md` F1 FIXED (Vite `configLoader: "native"`, VERIFIED-RUN as config load, build and serve); `audit-activation-compile-1019.md`: 845-crate closure; `run-e2e.sh play --check` exit 4 (browsers missing) | cargo window; `activate-prototype` ~3 h cold; `serve` smoke on 6033; browsers (D15) |
| Demonstrator `@semio-tech/mit-bestand-demonstrator` (:6029) | BLOCKED (cargo; activation not run); shell boot fix STATIC-ONLY | `result-tsc5.md` §demonstrator shell; `audit-verify-1019h.md` §2 confirmed statically, not booted; `result-fixups.md` §4 test 1 pass (41 vs 35 fixed); `result-e2e-prep.md` 35 of 38 selectors found | `activate-prototype` ~41 min cold; smallest e2e (8 panes) goes first |
| Playwright browsers (chromium 1248, headless shell, ffmpeg 1013, about 300 MB) | OPEN; needs explicit go-ahead (D15) | `result-e2e-prep.md` F4 | decision D15 |

## 2. Applied-versus-staged ledger

| Patch set or change | Location | State | Evidence | Remaining |
|---|---|---|---|---|
| tax3 `apply-1` to `apply-4` (mutation fixture, package admission, path budget, path-emoji statutes) | `C/tax3-tools/` | APPLIED 01:40 | coordination 01:40; apply-4 36 of 37 (1 timeout under load) | `maxPathBytes` consumer; 182 vs 74 reconciliation |
| own-crash, own-crash2 | `C/own-crash-tools/`, `C/own-crash2-tools/` | APPLIED 03:25, 05:05 | coordination 03:25, 05:05; `result-own-crash.md`; `audit-verify-1020a.md` 22 of 22 | type widened to `| undefined` (design concern) |
| lease-fair | `C/lease-fair-tools/` | APPLIED 05:05 (in the 10-set apply-all) | `result-lease-fair.md`; `result-stage-fix.md` READY | run through `bun repo run`; `[DEBUG]` lines and sync guard per `audit-staged-1020.md` |
| tax-stage | `C/tax-stage-tools/` | APPLIED 05:05 | `result-tax-stage.md`; `audit-verify-1020a.md` anchors 121 of 121; manifest 167 ops (`audit-verify-1020a.md`; `audit-staged-1020.md` counted 168, `result-stage-fix.md` 101) | conflicts with bzl-real and exports-stage; compat shims kept (`audit-staged-1020.md`) |
| tsc-stage | `C/tsc-stage-tools/` | APPLIED 05:05 | `result-tsc-stage.md` (22 files); `result-tsc5.md` | 8 configs still red |
| walkperf (and walkperf2 per coordinator 03:00) | `C/walkperf-tools/` | APPLIED 05:05 (walkperf); walkperf2 not in the 05:05 list; `audit-staged-1020.md` (03:01) found no walkperf2 directory | `audit-verify-1019h.md` §5 NOT sound; `audit-verify-1020a.md` | fence repair; confirm walkperf2 state |
| bzl-stage | `C/bzl-stage-tools/` | APPLIED 05:05 | `result-stage-fix.md`: "provisional, no result report" | no `result-bzl-stage.md` exists; confirm content |
| cargo-pressure | `C/cargo-pressure-tools/` | APPLIED 05:05; throttle 15 s signed off 04:25 | `result-cargo-pressure.md`; coordination 04:25 | post-apply admission read failure (coordination 05:05) |
| fixups6 (10-set apply) | `C/fixups6-tools/` | APPLIED 05:05 | `result-fixups6.md` | graph test split (its §2, 1,055 lines, 104 registrations) still STAGED (handover item 3); `--features typegen` may be undefined (`audit-verify-1020a.md`) |
| env-pass2 | `C/env-pass2-tools/` | APPLIED 05:05 | `result-env-pass2.md`; `result-post-apply.md` | post-apply idempotence failure; `apply.py` `--dry-run` fixed in stage-fix |
| exports-stage (manifest batch) | `C/exports-stage-tools/` | APPLIED 05:05; lock and sync green 05:43 | `result-exports-stage.md` (506 default entries, 3 blocked); coordination 05:05, 05:43 | 12 `/.json` keys flagged before apply (`audit-verify-1020a.md`); fix status not confirmed by any report |
| deps5, deps5b | `C/deps5-tools/` | APPLIED 05:15 (live via rebase) | `result-deps5.md`, `result-deps5b.md` | none beyond sync evidence |
| inputs-closure | `C/inputs-closure-tools/` | APPLIED 05:22 before lock | coordination 05:22; `result-inputs-closure.md` (not run) | suspected cause of the `show projects` slowdown (HANDOVER); revert script not confirmed |
| lift-fix repair (98 lifted scripts) | `C/lift-fix-tools/repair.ts` | APPLIED live 06:45 | `result-lift-fix.md`: 24 test children and 74 routers repaired | `scope` suite still has 4 reds (`root-drain` admission) |
| law-drift patches (24) | `C/law-drift-tools/staged/` | 22 APPLIED; `cc-project`, `cc-test` skipped (frozen `project.json`) | `result-law-drift.md` | handover lists `jsonattr.ts --apply` and `ALLOW_PROJECT=1 apply.sh` as queued; confirm |
| coordinator live fixes | live tree | APPLIED | coordination 05:05 (root script build), 06:05 (forms `{workspaceRoot}`, `home-io-surface-check` cache false) | none |
| Direct live edits by executors (not staged): comments-a, comments-b, emoji-docs, value-repoint, fnsplit2 and fnsplit3, rs-sweep to rs-sweep4, rg-* splits, split-s and split-fw, fixups to fixups5, docs-*, cmdreg, cli-fix, e2e-recheck, owners, rules-fix, law-sweep, law-sweep2, tst3 to tst5, vit2, vit3, lay12 to lay16, tsc4 and tsc5 | live tree | APPLIED | per report | see section 6 |
| lib-defects | `C/lib-defects-tools/staged/patches/lib-defects.json` (17 files, 30 edits) | STAGED | `result-lib-defects.md`; `apply.sh check` | apply after lock and sync window |
| cycle-break manifest delta (471 ops) | `C/cycle-break-delta.json` | STAGED | `result-cycle-break.md` | lock and sync under pause |
| lay16 staged items | `C/lay16-tools/staged/` | STAGED | `result-lay16.md` §5 | apply; re-cruise |
| lay17 delta | `C/lay17-tools/` (`apply-delta.ts`) | STAGED | `result-lay17.md` §1 and §4 | apply after lock and sync |
| tax4 staged part | `C/tax4-tools/staged/` | STAGED | `result-tax4.md` §0 (1,528 to 1,119) | apply |
| tax5 leaf case tables | `C/tax5-tools/staged/` | STAGED | `result-tax5.md` | apply; then taxonomy validation |
| profile-apply (`[profile.*]` out of `.cargo/config.toml`, A067) | `C/lift-fix-tools/profile-apply.py --write` | STAGED; ruling reversed 06:45 (inline profiles in `Cargo.toml`) | `result-lift-fix.md` "ruling NOT applied" | run after PAUSE LIFTED; `cargo metadata` once after |
| bzl-real set (`.bazelignore` removal, store-receipt, adapter, scratch host) | `C/bzl-real-tools/` (3 staged files) | STAGED, NOT SAFE TO APPLY AS STAGED | `audit-verify-1020a.md`: hard-coded `/Users/ueli` path (fixed per stage-fix §8, not re-verified); stale docstring; patch target renamed by tax-stage | re-stage against live; apply after review |
| law-sweep2 | `C/law-sweep2-tools/` | none staged; edits live | `audit-staged-1020.md` NOT READY (no staged content) | none |
| walkperf repair (fence) | none | NOT STAGED | `audit-verify-1019h.md` §5.1 | design and stage |

## 3. Open user decisions (deduplicated)

Status column: RESOLVED means the user or a coordinator ruling has closed it in the reports. Only OPEN items need an answer.

| ID | Decision | Status | Sources |
|---|---|---|---|
| D1 | Launch surface: restore `.vscode` or retire it (repo dashboard replaces it) | RESOLVED by user AGENTS amendment (`fd14da55f18`, coordination 06:00); fixtures still pin the old text | status-1019 §2.1; coordination 06:00 |
| D2 | AGENTS conflicts: `package.json` must call nx; "devs use launch.json and never use the CLI"; `📜️scripts.ts` typo; search tool ban | RESOLVED by user edit 01:24 (coordination 06:00); `protectedInstructions` fixture must change with it | status-1019 §2.2; coordination 06:00 |
| D3 | Permanent scripts outside `📜️script.ts` (bootstrap `.sh` and `.ps1`, hooks, `.mjs`, Python oracles) | RESOLVED: AGENTS rule removed (06:00) | status-1019 §2.3 |
| D4 | Runtime external dependencies versus "MUST NOT create runtime dependencies" (46 entries in 11 framework packages; root `next`, `pg-boss`, `sharp`, `esbuild`, `@napi-rs/canvas`) | OPEN | status-1019 §2.4 |
| D5 | Compatibility and shim surfaces (`preview2-shim-patches`, `host-shim-source`, `export * as mainWindow`, `.sty` compat definition, `polynomial` alias); `tax-stage` keeps compat shims | OPEN | status-1019 §2.5; `audit-staged-1020.md` |
| D6 | `[DEBUG]` evidence prints (90 removed by rules-fix; about 982 test log lines remain; `[DEBUG]` progress line in `🏃️process/📜️script.ts`) | OPEN | status-1019 §2.6; `result-law-drift.md` open item 4 |
| D7 | Comments: `// 🚫️async:` exemption notes (1,642 in comments-a, about 1,762 in comments-b) have no reader; keep as a schema field or docstring, or drop; Rust field docs and `// SAFETY:` status | OPEN | status-1019 §2.7; `audit-verify-1019i.md` |
| D8 | Repo hygiene removals (agents cannot run `git rm`): 34 hash-named lockfile copies, tracked binary `🖥️server/🎛️coordinator/server`, `.orig` and `.DS_Store`, `micro_msg.txt`, `extension-host-build-7a5OFB/`, ticket copies of tracked code, 58 tracked generated `jcoprobe` files | OPEN | status-1019 §2.8; HANDOVER |
| D9 | Jcoprobe generated bundles: keep as fixtures or regenerate (merge with D8) | OPEN | status-1019 §2.9; HANDOVER |
| D10 | `repo:noop` on `👴️leutwiler … build`; command form `bun <path>/📜️script.ts` | OPEN (minor) | status-1019 §2.10 |
| D11 | Devcontainer `privileged: true` | IMPLEMENTED (removed; `audit-verify-1019d.md`); confirm the replacement (directory partitions) | status-1019 §2.11; coordination 14:45 |
| D12 | Linux privilege model: unprivileged directory backend chosen 14:45, not the sudoers helper | DECIDED by coordinator; user to confirm; Linux verify FAIL (`audit-verify-1019d.md`) | status-1019 §2.12; coordination 14:45 |
| D13 | Windows: early refusal versus self-elevation; now directory partitions with a journaled swap, VHD deleted | DECIDED by coordinator 16:45 and 20:00; user to confirm | status-1019 §2.13; coordination 16:45, 20:00 |
| D14 | CI: new `build-system.yml` matrix (ubuntu, macos, windows; PR trigger; cost); approval of first push | OPEN (user approval) | status-1019 §2.14; `result-xplat-ci.md` item 9 |
| D15 | Playwright browser download, about 300 MB (chromium 1248, headless shell 1248, ffmpeg 1013) | OPEN (explicit go-ahead required) | status-1019 §2.15; `result-e2e-prep.md` F4 |
| D16 | Product decisions: course `site.legal` empty; bun pin 1.4.2 vs local 1.3.14; Linux x64 runner for `ARCHITECTURE_COURSE_RUNNER`; play `release-verify` has no dependency on `build`; demonstrator `build` output string | OPEN | status-1019 §2.16; `result-bzl-man.md` items 3, 6 |
| D17 | Bazel cache sizing (40 % of the products budget, 9,830 MiB) and remote cache (off by default) | OPEN | status-1019 §2.17; `result-bzl-cache.md` |
| D18 | Ticket 19 (codex) overlap: restored course launch entries and `.vscode` assertion | OPEN (coordinate with ticket owner) | status-1019 §2.18 |
| D19 | `// 🚫️async:` and comment-strip scope: restore 35 docstrings done; decide the register (same as D7) | OPEN (merged into D7) | `audit-verify-1019i.md` |
| D20 | Raw `acquireResourceLease` writer preference versus reader fairness | OPEN (owner decision; staged predecessor-only reaping) | coordination 02:50; `result-lease-fair.md` §7 |
| D21 | Cargo profiles: coordinator ruling (06:45) keeps profiles inline in `Cargo.toml` and reverses the earlier `.cargo/config.toml` ruling | RULED by coordinator; confirm | `result-lift-fix.md` |
| D22 | Draw bundles with extra `↩️inverse` facet: contract versus data | OPEN | HANDOVER open user decisions; status-1019 |
| D23 | Tree-sitter-rust grammar bump (0.20 cannot parse `async |x|`) | OPEN (deps owner) | `result-law-drift.md` harness finding 6 |
| D24 | `pack json` absence copies the store closure; native-build imports outside the four neutral roots | OPEN (owner decision) | `result-tst6.md` |
| D25 | `geometry-service` fixture `{}` has never had content (commit 251fa724c4a) | OPEN (fixture must be produced) | `result-rust-mounts.md` (d) |

## 4. Ordered remaining runtime steps with time estimates

Estimates are marked: "measured" from a report or log, or "estimate" with the basis. None were measured by this audit. Run each step in the foreground, one at a time. Cargo steps only inside the serialized cargo window.

| Step | Action | Time estimate | Basis |
|---|---|---|---|
| 0 | Diagnose `bun repo show projects` (pid 88833, 3 h 56 min, 93 % CPU). Sample it. The suspect is the inputs-closure import walk applied 05:22. If confirmed, revert that patch (check for a revert script first) and restart `show projects`. | 15 to 30 min to decide and restart; the restarted show takes 7 to 9 min cold, or up to 60 min more if the cause is elsewhere | measured baseline (HANDOVER); estimate for diagnosis |
| 1 | `bun repo tools setup nx`, then `bun repo tools setup bazel`. Setup opens the 24 GiB products image. | 15 to 30 min | estimate (setup never run; image size from status-1019 row 6) |
| 2 | Lift the WRITE PAUSE once steps 0 and 1 are green. Apply the queued live edits: `profile-apply.py --write` (then one `cargo metadata`, detached); `law-drift` `jsonattr.ts --apply` and `ALLOW_PROJECT=1 apply.sh` (dry run first). | minutes each; `cargo metadata` may take 5 to 10 min under load | estimate |
| 3 | Repair the walkperf fence (`audit-verify-1019h.md` §5.1) and stage it. Then apply the manifest deltas: `cycle-break-delta.json` (471 ops), lay16 and lay17 items. Then `bun repo dependencies lock --all` and `sync --all` under a total write pause. | fence repair: 1 to 2 h (estimate); lock plus sync: 45 to 60 min including pause (sync measured 31 min at 05:43) | measured sync; estimate for lock and fence |
| 4 | Neutral dry runs for both tools: `bun repo --tool=<nx\|bazel> affected --targets=test --dry-run`; `exec --projects=<project> --dry-run -- <cmd>`; `bun repo --tool=bazel run @teaching/architecture-course:test --dry-run`; `cache status` for both. | 5 to 10 min per graph load; 24 s with `NX_FORCE_REUSE_CACHED_GRAPH=true` (project memory note) | estimate; memory note for the nx graph |
| 5 | `C/nx-cache-tools/verify-nx-cache.ts` (`@teaching/architecture-course:test`): cold, then second run must hit. | 30 to 60 min | estimate (course test runtime not measured) |
| 6 | Fix the live Bazel adapter first: apply the reviewed `.bazelignore` removal and the emoji store-receipt fix (re-stage against live; `audit-verify-1020a.md`). Then the production run `bun repo --tool=bazel run @teaching/architecture-course:test` cold, then warm; `cache status` (`activity.builds >= 1`); `cache clear`; regrow. | fix and re-stage: 30 to 60 min; cold run 30 min to 2 h, warm seconds (estimate; scratch cold was 28 s for three small tasks and 120 s for the actor-network task) | measured scratch; estimate for course |
| 7 | Store-receipt repair: `@teaching/proctor:build` twice, delete compiler `published/<hash>` slot, build again. Expect one `[store-receipt] re-executing`. | after cargo; not measured | BLOCKED on cargo |
| 8 | Cargo window, serialized (`cc2` as the only cargo owner): value family first (groups of up to 8, `result-value-fix.md` §5), then proctor (`🎓️teaching/Cargo.toml`), then today's splits (lists in `result-rg-*.md`, `split-*.md`, `rs-sweep*.md`, `tco2`, `rg-test*`), then `lockref-tools/all.sh` detached (288 of 291 workspaces unverified). | 4 to 8 h (estimate; one `cargo check` ran 37 min under load per `result-rg-test.md`; workspace resolves hit the 550 s alarm) | estimate |
| 9 | Activations: `@semio-tech/mit-bestand-demonstrator:activate-prototype` (about 41 min cold), then `@semio-tech/semio-tech-play:activate-prototype` (about 3 h cold), then `serve` smoke on 6029 and 6033. | 41 min and 3 h, then minutes | estimate from `audit-activation-compile-1019.md` |
| 10 | Browsers, after D15 go-ahead: `bun repo run workspace:deps-browsers` (about 300 MB). | 5 to 15 min, depends on the link (not measured) | estimate |
| 11 | E2E: `e2e-tools/run-e2e.sh demonstrator --check`, then `run-e2e.sh demonstrator` (8 panes, first), then `play`, then `course --topology dev --warm-proctor` (needs proctor cargo). | demonstrator 15 to 30 min; others not measured | estimate |
| 12 | MCP: `bun repo run repo-mcp:build`, restart session, `bun semio-mcp-tools/drive.ts repo …` (`ticket_close` on a missing ticket). After `framework-os-mcp-rs:build` (cargo, step 8): `drive.ts semio` tools/list must be non-empty. | minutes; the semio build is inside the cargo window | estimate |
| 13 | Dashboard: rebuild; `semio commands --check`; `semio run --dry-run tool:workspace/build-exec --parameter projects=workspace --parameter dry-run=true`; `semio run --dry-run tool:workspace/build-cache-status --parameter build-tool=bazel`; registry oracle `bun test ./R/🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts` without the stale-binary override. | 10 to 20 min (estimate) | estimate |
| 14 | Build owners: `bun ./📜️script.ts verify build-owners` (check only). Run `sync` only after the `nativeRoots` safety fix. Then `bun repo preflight --scope @teaching/architecture-course`. | minutes | estimate |
| 15 | Linux: fix the two `audit-verify-1019d.md` defects; dispatch the directory backend; unprivileged `bun repo tools setup`; scope-less `bun repo preflight`; `devcontainer up` with postCreate; native `workspace:setup` as non-root. | 1 to 2 h plus image build (estimate; `docker build` used a 590 s alarm in `result-xplat-boot.md`) | estimate |
| 16 | Windows: `pwsh` parse of `NAT/🔵️.ps1`; `bun repo tools setup bazel` from an elevated shell with an ASCII checkout; non-ASCII refusal; FFI junction swap test. | BLOCKED: no Windows host or `pwsh` on this Mac | environment |
| 17 | CI: after D14 and a push, the `build-system.yml` PR run (nx and bazel hit proofs; products matrix). | not measured | estimate |
| 18 | Red suites: re-label the tst7 classes (`audit-verify-1020a.md`), then the 70 red suites by owner after D1 to D5; tsc remaining 8 configs; vitest remaining 18 configs. | several hours (estimate) | estimate |

## 5. Changes since status-1019 and stale claims

- Dependency sync: BLOCKED in status-1019, now VERIFIED-RUN green 05:43 (generation `70825028305173c7f252`). `show projects` is the new blocker.
- Bazel: "never executed" in status-1019 is superseded. Scratch-host real runs now exist (`result-bzl-real.md`) and `bazel build --nobuild //...` on the generated scratch workspace is verified. The production `--tool=bazel run` remains NOT RUN.
- Linux: directory-backed partitions implemented (`result-xplat-linux.md`), but `audit-verify-1019d.md` says FAIL (2 defects). `privileged` removed.
- Windows: VHD and elevation removed; directory partitions with a journaled swap (`result-xplat-win2.md`), static only.
- AGENTS amended by the user (01:24 commit, coordination 06:00): launch.json, nx and `📜️script.ts`-only rules removed; search-tool ban removed. Fixtures still pin the old text.
- `.vscode` retirement and `package.json` nx form: decided by the AGENTS edit (see D1, D2).
- Compat facade re-exports removed in `semio-framework-value` (1,025 files; `result-value-repoint.md`). The verify step (`audit-verify-1019h.md` §1) confirms resolution statically.
- Crate gate: test-cfg over-gate crates from 8 at 10/19 to 1 (`audit-census10-1020.md`).
- `bun test` suites: 1,764 of 2,145 green (tst7, 02:55); 1,756 of 2,140 after real install (tst6). These are counts; the failure classes are not reliable (`audit-verify-1020a.md`).
- Stale in status-1019 and still stale: `result-plugin-cache.md` sections 0, 2, 3; "nx.json:91 dead path" stays refuted; "Docker daemon not running" stays superseded.
- Stale in HANDOVER: "law-drift queued" conflicts with `result-law-drift.md` (22 of 24 applied). Treat as partly applied until a dry run confirms.
- `status-1019` said walkperf staged. It was applied 05:05 and is not sound (`audit-verify-1019h.md`).

## 6. Audit defects not closed by any later report

No later report was found that closes these. Each needs a re-check or a fix before the next apply.

- Walkperf fence covers 10 of ~156 files (`audit-verify-1019h.md` §5.1), applied 05:05.
- Live Bazel adapter still writes `.bazelignore`; the caching defect is live (`B/📜️script.ts:409`).
- Bazel emoji-target crash: staged fix not verified live (`result-bzl-real.md` defect 2).
- Undeclared import closure: staged in `inputs-closure` (applied 05:22); not re-measured in Bazel.
- Linux: capacity accounting bypass and a tracked red test on the deleted loop observer (`audit-verify-1019d.md`).
- `verify build-owners sync` removes `nativeRoots` rows and rows with absent paths (`audit-verify-1019c.md` §4): do not run until fixed.
- tax2: at least six dangling path references (`audit-verify-1019f.md` F2). tax3: consumer still uses removed `maxPathBytes` (`audit-verify-1019h.md`).
- rs-sweep: visibility widenings and `_cold` renames wrong (`audit-verify-1019g.md`; partly corrected by rs-sweep2 §0).
- Comment strip: three broken anchor test pairs A, B, C (`audit-verify-1019i.md`; fixups5 claims a repair, not re-verified).
- `exports-stage` 12 `/.json` export keys (`audit-verify-1020a.md`); applied 05:05, no fix report found.
- `fixups6` actor typegen exporter passes `--features typegen`, which the actor package does not define (`audit-verify-1020a.md`).
- `vcs-bound` law: 112 pass and 5 fail, not 117 and 0 (`audit-verify-1019e.md`).
- `cache-rest` stale cache hit in `stdio` `project.json` (`audit-verify-1019b.md`).
- `deps3` leftover `…/rust/pkg` workspace lines in four `package.json` files (`audit-verify-1019b.md` item 2).
- `result-plugin-cache.md` stale (`audit-verify-1019c.md` §3).
- `cargo-pressure` post-apply admission read failure (coordination 05:05).
- tst7 classification unreliable (`audit-verify-1020a.md`): re-label before using the 381 red suites by class.
