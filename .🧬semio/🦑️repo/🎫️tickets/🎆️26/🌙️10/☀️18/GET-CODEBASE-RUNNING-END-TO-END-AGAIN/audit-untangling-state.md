# Audit: Untangling State (Clean-Architecture Layering and Monolith Split)

- Date: 2026-10-18. Auditor: Haiku 5.5, read-only, foreground.
- Owning ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (folder `🪓️monolith-split` = F).
- Sources: `F/COMPILE-DEFECT-LEDGER.md` (last written 2026-10-17 23:40), `F/MONOLITH-CENSUS-5.md` (2026-10-17 22:23), `F/RESULT-*.md` (newest read: CENSUS5-A/B/C/E, BASELINE-DEFECTS-11/12, CRATE-HEADROOM, AUDIT-WAVE7-CRATE-GATE-2, COMPILE-PASS-1/2, INFRA-UNBLOCK-3, COMPILE-ADMISSION-DIAG), `F/📓️FLEET.md` (tail to 2026-10-18 00:2x), `F/📓️COMPILE-RUNBOOK.md`, `F/📓️COMPILE-WINDOW-CHECKLIST.md`, and four memory notes.
- No file was edited, no modifying git command ran, no cargo, nx or `bun repo` command ran. The only write is this report.
- Caveat: nothing in the ticket has been compiled since 2026-10-16 05:28. All Rust evidence after that is static (rustfmt parse, scoped oracles, grep and census scripts).

## 1. Target and enforcement

**Target rule.** Each crate closure must stay inside the source-boundary gate: 4,000 active lines (blank and comment lines counted), 32 files, 262,144 bytes, with `include_str!`/`include_bytes!` inputs counted. Per-crate `rust-source-boundary` metadata can only tighten it. Source: `⚡️caching/🔣️policy.json` `storage.sourceBoundary` (memory note `project-source-boundary-gate-and-static-rust-oracle`; I could not re-open the file, the repo-wide search timed out).

**Layering rules.** Framework, OS, plugin, hub and renderer layers, with the dependency-direction and cruiser gates in `REPO-GATES-7` (cruiser 9,237 violations, down from the 10,593 HR2 baseline). No `ui/os/db` layer violations are recorded as open.

**Enforcement, in order of strength:**
1. Compiler ingress: `.cargo/config.toml` `[build] rustc` is set to the peer's ingress script, which refuses raw cargo. Compile is allowed only through `bun repo cargo --store=shared --configuration=prototype -- <subcommand>` (memory `project-cargo-compiler-ingress-gate`). Today the launcher refuses before cargo starts (section 3).
2. Crate source gate: the reader `readRustSourceClosureV1` refuses over-budget closures. Measured without cargo by `hdr-census.ts`, `cgc8-census.ts` and `c5*-tools/cgc-census.ts`.
3. File-level census: `MONOLITH-CENSUS-5` (per file over 1,000 lines), plus `lib-wiring-budget` ratchet and `new crates --check-applied` layout gate.
4. Split verification: `modsplit.py lossless`, `vmod.py`, `defscan.py`, `ambig.py`, plus a patched `--emit=metadata` differential (memory `project-pack-carve-union-oracle-and-typeck-blind-spot`). This oracle is name-resolution only; privacy, trait-scope and coherence need separate scans.

## 2. Done and remaining

### Done (static or partly verified)

- Crate gate: `CRATE-HEADROOM` (2026-10-17 20:0x) brought 122 of 241 crates at or above 0.90 under it. Census 2 (cgc8, 2026-10-17 14:10) measured 2,866 crates: oversized lib 58 to 8, test-only 43 to 1, reader-refused 10 to 0.
- Latest full census (`CENSUS5-A`, 2026-10-17 23:4x, about 3,068 crates): one oversized lib, `semio-s-artifact-bim-model` (41x, owner r9a). Every crate in the plugin tt/tti/tr/tri/rt group is inside the gate, worst at 0.902 (`semio-framework-plugin-tests-15`, 26 of 32 files).
- Monolith packages of `MONOLITH-CENSUS-5` section 4:
  - Packages 1, 2, 3: DONE static (`CENSUS5-A`, 16 plugin files to 119 children).
  - Package 4: DONE static (`CENSUS5-B`, 6 files to children; pinned by oracle and vmod checks).
  - Package 5 and 6: not started (listed as pending in `CENSUS5-B`).
  - Packages 7, 8, 9: IN PROGRESS (`CENSUS5-C`, nothing edited yet at its last record).
  - Packages 10, 11, 12: no result file; not started.
  - Packages 13 (`plugin/🧬️schema/📜️.wit`, 1,662 lines) and 14 (architect `🔺️diff/🟦️.ts`, 1,012 lines): IN PROGRESS (`CENSUS5-E`, investigating; no edit recorded).
- Census 4 packages: all done (`FLEET` 20:3x, wgpu files stay peer-held).
- Ledger (`COMPILE-DEFECT-LEDGER.md`): bd7 to bd11 record 12 + 2 partly + 2 gone + 8 + 13 + 9 + 4 + 13 + 10 rows fixed statically. O-02 (raster) and O-11 (vitest gate) closed.
- Layering: 2D compute re-point (`2D-COMPUTE-REPOINT`, 00:0x): 437 sites, 407 files, 239 manifests and 117 locks re-pointed; rust-source-direction census 486 to 8.
- Plugin TS suites: 71 files / 177 failing tests down to 8 files / 13 failing (bd11).

### Remaining, with evidence

| Area | Open item | Evidence | Owner |
|---|---|---|---|
| Crate gate | `semio-s-artifact-bim-model` lib 41x | CENSUS5-A | r9a |
| Crate gate | `process3d-host` 1.084; `job` 0.991; `3d-brep-boolean` 0.988; 155 crates still at or above 0.90 (105 of them exactly 29/32 files) | CRATE-HEADROOM | hdr / bd10 |
| Crate gate | `semio-framework-plugin` test-only 16x (pi2) | CGC8 (14:10); later not re-measured | pi2 |
| Monolith | Package 5, 6 pending | CENSUS5-B | plugin split |
| Monolith | Packages 7 to 9 in progress, 10 to 12 not started, 13 and 14 in progress | CENSUS5-C/E | CENSUS5 |
| Monolith | Peer-held (172 files, 325k lines), parent (20), scope hints (6), wgpu-held (22 FREE, 31.6k lines) | CENSUS5 section 5 | peers |
| Monolith | `poll_kernel_turn` is one 962-line function; cannot be cut without rewrite | CENSUS5-A; `bd12` item 3 pending | plugin owner |
| Ledger O-03 | `process3d-host` retained-laws integration target rewritten as a lib unit test, not compiled | bd10-09 | process3d owner |
| Ledger O-04 | `stdio/pdf` jpg lib root 600 lines (ratchet breach) | bd7-13 | r5a |
| Ledger O-01 | Pack `source.sha256` pins (30 over 9 files) and TS `PackError` API on an unmounted draft | CENSUS4-PKG-35 | pack owner |
| Ledger O-05 | Plugin close-budget and frontier source anchors: 43 assertions name removed or renamed entrances; `advance_original_document_archive_close` has no production caller | bd9/bd10 | plugin owner |
| Ledger O-07 | Architect and equation window fixtures were never authored; three readers of deleted Rust files | bd9-08 said fixed; O-07 still lists the readers | schema-first author, plugin owner |
| Ledger O-08 | Consumers of moved directory fixtures outside Rust not scanned | scope limit | next defect slice |
| Ledger O-10 | Misplaced fixtures in value, pack, actor, process, ui | not examined | — |
| Ledger O-11 | Vitest ownership gate: bd10 marked done (7/7), but library `workspace-contract` (49 fail), flow wasm unit file (11 fail) remain | bd10/bd11 | library owner |
| Ledger O-13 | Plugin TS: 8 files / 13 tests still failing (mounted-owner receipts, registry runtime, component modes, storybook, semantic pairs, plugin identity, 2 units) | bd11 | plugin owner |
| Ledger O-09 | Launcher refuses (`Exact canonical persistent Native publication required`) | INFRA-UNBLOCK-3, COMPILE-ADMISSION-DIAG | coordinator |
| bd12 | In progress: `runtime_lifecycle_grant` removal (ruling: take grant from owner), dead `pump_runtime_live_cooperative_turn`, `poll_kernel_turn` decomposition, empty generated `plugins.json`, 12 stale descriptor packs, ~110 consumer Cargo.lock files still naming `semio-framework-2d` | `RESULT-BASELINE-DEFECTS-12` (started, items 1 and 2 only) | bd12 |
| Half-split | `time-travel-turns` reads 4 private fields of `TimeTravelStoreState` (real E0616) | CENSUS5-A; routed to ohf | ohf |
| Half-split | `semio_framework_2d::compute` facade alias `pub use semio_framework_2d_compute as compute;` has no consumers left | FLEET 00:0x | mwu |
| Half-split | Dup-mount carve: 180 Cargo.lock files hand-edited; plugin-host carve crates missing from every lock; offline lock refresh is the first step of any compile | DUPLICATE-MOUNTS-2; COMPILE-WINDOW-CHECKLIST item 1 | compile window |
| Half-split | Descriptor-packs and runtime-registry generation waiting on cargo | FLEET 00:1x; bd11 | wasm producers |
| Half-split | `semio-s-artifact-flow-flow` lib 4.1x: no slice owned it in the 14:10 census; CRATE-HEADROOM later fixed sibling crates but the flow root is still listed as not fixed | CGC8, CRATE-HEADROOM | unowned |

## 3. Compile state as last recorded

- **Last real compile:** 2026-10-16 05:08 to 05:28 (`RESULT-COMPILE-PASS-2.md` section 2). Framework `--lib` batch 0 (60 crates) exit 0, 0 errors. Batches 1 and 2 exit 101: 4 failing crates, then 5 errors, all fixed in source and not re-verified. Batch 3 failed at the launcher (ENFILE, file-table overflow). Before that, 2026-10-15 `COMPILE-PASS-1` compiled 129 of 137 framework crates natively; 2 failed, both fixed, not re-verified.
- **Error classes from the last real compile:** E0433 unresolved crate (dependency only in `[dev-dependencies]`: `replication-interaction`); E0432 unresolved import (moved path, value facade `factory`, `#[cfg(feature = "deflate")]` gate lost on re-export, `replication-wire-core` moved); E0425 unresolved name (`RetainedPackSymbolSpan` not imported in `pack-format-catalog`); E0277 trait bound (same file); foreign E0425 in `ui-contract-presentation` (peer-owned).
- **Launcher state (latest records):** 2026-10-16 15:15 `LAUNCHER REACHES CARGO: no` (`INFRA-UNBLOCK-3`): the image receipt `⚡️cache/rust/fixed/image.json` lacks the required `mountDirectory`, which the peer's `ImageReceipt` schema now requires. 2026-10-17 13:35 to 14:21 (`COMPILE-ADMISSION-DIAG`): the launcher reaches the receipt check in about 6 s and stops with `Pending Native image publication must settle before ordinary admission` (two interrupted `.publication/intent.json`). Physical headroom was short by about 32 MB against the 4 GiB gate at 07:30 on 10-16.
- **Compile admissions:** `grep "COMPILE ADMITTED" 📓️FLEET.md` returns no line. Every BASELINE-DEFECTS and CENSUS slice states "static evidence only".
- **Ledger date:** the defect ledger's last rewrite is 2026-10-17 23:40 (bd11), with bd12 started at 2026-10-18 00:xx.

Conclusion: the workspace is not known to compile as of 2026-10-18. The last green evidence is framework batch 0 on 2026-10-16. Everything after that is unverified.

## 4. Workspace members vs manifests on disk

- Root `Cargo.toml`: 539 literal `members`, 235 `exclude`, 0 globs. All 539 member paths exist with a `Cargo.toml`.
- `🧰️framework/Cargo.toml` (own workspace with its own lock): 357 literal `members`, 6 `exclude`, 0 globs. All 357 paths exist. The 2026-10-16 runbook still quotes 284 members; this is stale.
- Manifests tracked by git, excluding `.🧬semio`: 3,212 `Cargo.toml` (`git ls-files`). Untracked non-ignored manifests: none in the list check, though one `git ls-files --others` call errored with `daemon terminated` and that count is unreliable.
- Census context: about 287 `[workspace]` roots (CGC8), about 2,866 to 3,068 crates with `[package]` in the census, and 144 manifest directories with no `[package]`. Root plus framework members total 896. The rest are members of the hub, plugin, teaching and other standalone workspaces, which are not in the root or framework lists.
- Consistency: no dangling member path. Count differences are explained by nested workspaces, non-package manifests and crates that belong to other workspaces. Exact reconciliation needs the census script output (`cgc8-manifests.txt`), which I did not re-run.

## 5. Prioritized work packages

Each package is sized for one executor and has a disjoint file set. Packages P0 to P1 are gates; the rest can run in parallel.

**P0 (unblock compile, not an executor task):**
- P0-a. Peer-owned launcher receipt: regenerate `⚡️cache/rust/fixed/image.json` with `mountDirectory` through the peer's sanctioned publication path, and clear the two interrupted `.publication/intent.json`. Owner: coordinator / MODULAR-MONOREPO peer. Do not bypass the gate or delete cache.
- P0-b. Free physical headroom through the peer's sanctioned retirement (not manual deletion).

**P1 (restore a green compile, after P0):**
- P1-1. Re-run framework `--lib` batches 1 to 3 and the 5 fixes from `COMPILE-PASS-2` section 3 (files: `replication-interaction`, `replication-mutation-messages`, `pack-format-catalog`, `replication` facade, `pack-record-retained`). Command per `COMPILE-RUNBOOK` section 1. Log to `F/🗑️generated/`.
- P1-2. Offline lock refresh of root, framework, hub, layout, forms and teaching (`COMPILE-WINDOW-CHECKLIST` item 1), touching only `Cargo.lock` files. Must precede any test run.
- P1-3. Test pass (`--tests`) for the framework batch after P1-1, and the 4 known breaks in `COMPILE-WINDOW-CHECKLIST` item 3.

**P2 (crate gate and monolith split, parallel):**
- P2-1. `semio-s-artifact-bim-model` (lib 41x, 1,320 files): owner r9a. Not for an executor until r9a is done.
- P2-2. `process3d-host` 1.084 and `job` 0.991: one package each; file set limited to those crate roots and their children.
- P2-3. Census 5 package 5 (6 plugin files, 8,920 lines): `🖥️host/🧵️shard/🧪️tests/🔬️unit`, `🗃️document`, `🧠️interpreter/🏃️runtime`, `🧠️interpreter`, `🧪️tests/🔬️app-app-builder`. Use CENSUS5-B brief and its specs.
- P2-4. Census 5 package 6 (5 plugin files, 8,883 lines): `🖥️host/⚡️effects`, `🎭️guest-runtime`, `🧪️tests/🔬️guest-cold-relay`, `🧭️routers`, `🧵️shard`.
- P2-5. Census 5 package 10 and 12 (`value/retained-clone/contract` + `geometry/engine`; `hub/boot/unit`): not yet started; each is 2 files or fewer, one executor each.
- P2-6. Census 5 package 11 (2 plugin TS files, 2,614 lines): `browser-bundle/materialization`, `registry/catalog-verification`.
- P2-7. Finish CENSUS5-C packages 7 to 9 (layout editor engine; plugin framework; directory client and dsl derive). Edits are not started, so one executor per package.
- P2-8. Finish CENSUS5-E packages 13 and 14: `plugin/🧬️schema/📜️.wit` (1,662 lines, consumers read as text) and architect `🔺️diff/🟦️.ts` (1,012 lines, generated parser block, check the owner before editing).

**P3 (defect ledger, parallel with P2):**
- P3-1. bd12 items 3 to 5 (`poll_kernel_turn` decomposition, generator for `plugins.json`, 12 stale descriptor packs) plus the dead-function removal in items 1 and 2. Plugin owner set only.
- P3-2. O-05: classify and re-anchor the 43 absent close-budget assertions. First verify `advance_original_document_archive_close` is intended dead before touching anchors.
- P3-3. O-13: 8 plugin TS files / 13 tests. Split by file.
- P3-4. O-01: the pack producer-authority draft (30 sha pins, 3 TS tests). Do not re-pin blindly.
- P3-5. O-07 readers of deleted Rust files (3 files), and author the architect and equation window fixtures schema-first.
- P3-6. `time-travel-turns` E0616 private fields (ohf/pi2 owner) and the `semio_framework_2d::compute` facade alias removal (mwu).

**P4 (cleanup after compile):**
- P4-1. Close O-03 (process3d retained laws) by compile, then O-04 jpg root (r5a).
- P4-2. Re-measure crate gate (`cgc-census.ts`) and file census (`mc5-report.py`) after P1 is green, and update `MONOLITH-CENSUS-5`.
- P4-3. Run the 0.90-band crates (155) through `new crates --check-applied`.

## 6. Notes

- Memory and ledger disagree on one point: FLEET 20:2x said "Census over 3,032 crates: 0 oversized". The later CENSUS5-A (23:4x) found one oversized crate (bim-model, r9a). Treat bim-model as the current record.
- The runbook's framework member count (284) is stale; the current count is 357.
- A `bun repo` or cargo command must not be run by any executor until P0-a is closed, because each launcher call will fail at its own start.
- Scratch files: while checking member paths I wrote a short scratch list to `/tmp/x` (outside the repo, no deletion). It can be removed by the owner.
